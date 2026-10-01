"""Parallel Stage A e1 strip — 3-phase implementation.

Replaces the original sequential `while True: ... bisect ...` loop in
`runner.py` for the cargo-check / cargo-build / W1 gates.

Phases:

  1. **Parallel single-candidate trial (cargo check)** — N workers each
     try one candidate in isolation; classify pass/fail. Bad candidates
     are frozen immediately.

  2. **Combine-verify on main** — apply ALL individually-safe candidates
     together on the main crate; run cargo check. If it passes, the set
     is the final cargo-check-safe working set. If it fails, fall back
     to a sequential bisect on this (smaller) set to remove the
     interaction-bad candidates.

  3. **Cargo build + W1 (sequential, on main)** — once the cargo-check
     gate is satisfied, run the remaining gates on the combined
     working set, falling back to the original sequential bisect on
     either failure. These gates are slower per check (cargo build
     ≈ 10× cargo check; W1 actually runs the binary) and most lifts
     pass them once cargo check passes, so parallelizing isn't worth
     the worktree maintenance cost.

Speedup vs sequential bisect: at N=4 workers on libxml2 (~250 candidates,
~100 fail individually), Phase 1 wall-clock is roughly
`ceil(250 / 4) × cargo_check_incremental ≈ 63 × 30 s ≈ 32 min`, versus
sequential's `~100 freeze cycles × 3 min ≈ 5 h`. Phase 2 adds one
combine check (~30 s) plus, in the worst case, a sequential bisect on
the smaller post-Phase-1 set (much faster than original).

Disk cost: N worktree copies under /tmp (~N × cleaned_crate_size).
"""

from __future__ import annotations

import concurrent.futures as _cf
import logging
from pathlib import Path
from typing import Callable

from perf_opt.stage_a.analyzer import AnalysisReport, Candidate
from perf_opt.stage_a.transformer import Edit, TransformPlan, apply_plan, build_plan
from perf_opt.verify.cargo import GateResult, Verifier
from perf_opt.stage_a.worktree_pool import WorktreePool

logger = logging.getLogger(__name__)


# ─────────────────────────────────────────────────────────────────
# Plan path translation (main_crate → worker_crate)
# ─────────────────────────────────────────────────────────────────

def _translate_path(p: Path, main_dir: Path, worker_dir: Path) -> Path:
    """Map a path rooted under `main_dir` onto its counterpart under
    `worker_dir`. Paths not under main_dir are returned unchanged
    (e.g. workload_input lives in the project's workloads/ dir, not
    in main_dir)."""
    try:
        rel = p.resolve().relative_to(main_dir.resolve())
        return worker_dir / rel
    except ValueError:
        return p


def _translate_plan(plan: TransformPlan,
                    main_dir: Path, worker_dir: Path) -> TransformPlan:
    """Return a fresh `TransformPlan` whose Edit / use_imports_added
    paths point at `worker_dir` instead of `main_dir`. Required because
    `apply_plan` writes to `Edit.file` directly; without translation
    we'd mutate main_dir while the worker has the wrong state."""
    new_edits = [
        Edit(
            file=_translate_path(e.file, main_dir, worker_dir),
            start_byte=e.start_byte,
            end_byte=e.end_byte,
            replacement=e.replacement,
            purpose=e.purpose,
        )
        for e in plan.edits
    ]
    new_uses = {
        _translate_path(k, main_dir, worker_dir): list(v)
        for k, v in plan.use_imports_added.items()
    }
    new_extern_drops = {
        _translate_path(k, main_dir, worker_dir): list(v)
        for k, v in plan.extern_decls_dropped.items()
    }
    return TransformPlan(
        edits=new_edits,
        inline_added_for=list(plan.inline_added_for),
        extern_decls_dropped=new_extern_drops,
        use_imports_added=new_uses,
    )


# ─────────────────────────────────────────────────────────────────
# Phase 1: parallel single-candidate cargo check
# ─────────────────────────────────────────────────────────────────

def _classify_candidate_alone(
    cand: Candidate, *,
    report: AnalysisReport,
    main_crate: Path,
    pool: WorktreePool,
) -> tuple[Candidate, bool, str]:
    """Run cargo check with ONLY this candidate lifted, in a worker
    worktree. Returns (candidate, ok, detail_on_fail)."""
    with pool.acquire() as (crate, harness):
        plan = build_plan(report, lifted=[cand])
        plan_local = _translate_plan(plan, main_crate, crate)
        apply_plan(plan_local)
        verifier = Verifier(crate, binary_dir=(harness if harness else crate))
        # A slow check under N-worker CPU contention must freeze THIS
        # candidate, never kill the whole run (brotli 2026-07-29: one
        # 180s TimeoutExpired aborted an 807-candidate Phase 1). 600s
        # accommodates large crates checked 12-way in parallel.
        import subprocess as _sp
        try:
            res = verifier.cargo_check(timeout_s=600)
        except _sp.TimeoutExpired:
            return cand, False, "cargo check timeout (600s) — frozen"
        return cand, res.ok, (res.detail[:200] if not res.ok else "")


def _parallel_classify(
    candidates: list[Candidate], *,
    report: AnalysisReport,
    main_crate: Path,
    pool: WorktreePool,
) -> tuple[list[Candidate], list[Candidate]]:
    """For each candidate, trial in parallel. Returns (safe, bad)."""
    n = len(candidates)
    logger.info(
        f"[stage_a/e1.parallel] Phase 1: classifying {n} candidate(s) "
        f"across {pool.n_workers} worker(s)"
    )
    safe: list[Candidate] = []
    bad: list[Candidate] = []
    with _cf.ThreadPoolExecutor(max_workers=pool.n_workers) as ex:
        futures = {
            ex.submit(_classify_candidate_alone, c,
                      report=report, main_crate=main_crate, pool=pool): c
            for c in candidates
        }
        done = 0
        for fut in _cf.as_completed(futures):
            cand, ok, detail = fut.result()
            (safe if ok else bad).append(cand)
            done += 1
            if done % 20 == 0 or done == n:
                logger.info(
                    f"[stage_a/e1.parallel] Phase 1 progress: "
                    f"{done}/{n} ({len(safe)} safe, {len(bad)} bad)"
                )
    # `as_completed` yields in (non-deterministic) completion order, so `safe`
    # is built in a different order every run. The Phase 2 bisect depends on
    # this order → a DIFFERENT set of candidates gets frozen each run (e.g.
    # makeMaps_d frozen in one run, kept in another), which cascades into a
    # non-reproducible final result. Restore the deterministic input order.
    idx = {c.name: i for i, c in enumerate(candidates)}
    safe.sort(key=lambda c: idx[c.name])
    bad.sort(key=lambda c: idx[c.name])
    logger.info(
        f"[stage_a/e1.parallel] Phase 1 done: "
        f"{len(safe)} individually-check-safe, {len(bad)} frozen"
    )
    return safe, bad


# ─────────────────────────────────────────────────────────────────
# Public entry — parallel e1
# ─────────────────────────────────────────────────────────────────

def parallel_e1_strip(
    *,
    report: AnalysisReport,
    candidates: list[Candidate],
    main_crate: Path,
    main_binary_dir: Path,
    snapshot: str,
    verifier_main: Verifier,
    gate_w1_fn: Callable[[], GateResult],
    rollback_fn: Callable[[Path, str], None],
    bisect_fn: Callable,    # original _bisect_one_bad signature
    n_workers: int = 4,
    tmp_root_base: Path = Path("/tmp/stage_a_workers"),
) -> tuple[list[Candidate], dict[str, list[str]]]:
    """Parallel version of runner.py's e1 strip loop.

    Returns (final_safe_set, frozen_by_gate). The main crate is left in
    the lifted state (matching the original loop's post-break contract,
    so e2/e3 passes see the lifts already applied).

    Phase 1: parallel single-candidate cargo check on N workers.
    Phase 2: combine on main, verify cargo check (sequential bisect
             fallback if the combined set fails).
    Phase 3: cargo build + W1 gates sequential on main, original bisect
             on either failure (these gates fail rarely once cargo
             check passes, so the cost is amortized away).
    """
    frozen: dict[str, list[str]] = {}
    # Worker copies live on a PERSISTENT FS next to the crate, NOT /tmp. On this
    # host /tmp is per-session-private + flaky for detached (setsid) runs — git
    # object writes fail ENOENT there, which crashes the worktree pool. The
    # project dir (dataset_trans/<proj>) is durable and local to 37.2.
    # tmp_root_base is kept for API compat but the persistent path wins unless a
    # caller explicitly overrides to something non-/tmp.
    if tmp_root_base == Path("/tmp/stage_a_workers"):
        tmp_root = main_crate.parent.parent / ".stage_a_workers"
    else:
        tmp_root = tmp_root_base / main_crate.parent.name
    main_harness = main_binary_dir if main_binary_dir != main_crate else None

    pool = WorktreePool(
        main_crate=main_crate, main_harness=main_harness,
        n_workers=n_workers, tmp_root=tmp_root,
    )
    try:
        # ── Phase 1 — parallel cargo check ──
        safe1, bad1 = _parallel_classify(
            candidates, report=report, main_crate=main_crate, pool=pool,
        )
        if bad1:
            frozen["check"] = [c.name for c in bad1]
            for c in bad1:
                logger.warning(f"[stage_a/e1.parallel]   frozen by check (Phase 1): {c.name}")
        if not safe1:
            return [], frozen
    finally:
        # Free worker disk before Phase 2 — workers no longer needed.
        pool.cleanup()

    # ── Phase 2 — combine-verify cargo check on main ──
    logger.info(
        f"[stage_a/e1.parallel] Phase 2: combine-verify {len(safe1)} on main"
    )
    rollback_fn(main_crate, snapshot)
    plan = build_plan(report, lifted=safe1)
    apply_plan(plan)
    g_check = verifier_main.cargo_check()

    working_set: list[Candidate]
    if g_check.ok:
        working_set = list(safe1)
        logger.info("[stage_a/e1.parallel] Phase 2: combined cargo check OK")
    else:
        logger.warning(
            "[stage_a/e1.parallel] Phase 2: combined cargo check FAIL — "
            "sequential bisect on Phase-1-safe set"
        )
        working_set = list(safe1)
        while True:
            rollback_fn(main_crate, snapshot)
            if not working_set:
                break
            plan = build_plan(report, lifted=working_set)
            apply_plan(plan)
            res = verifier_main.cargo_check()
            if res.ok:
                break
            bad_idx = bisect_fn(report, working_set, main_crate,
                                snapshot, verifier_main.cargo_check)
            bad = working_set.pop(bad_idx)
            frozen.setdefault("check", []).append(bad.name)
            logger.warning(
                f"[stage_a/e1.parallel]   frozen by check (Phase 2 bisect): {bad.name}"
            )

    # ── Phase 2b — recover false-positive check-freezes ──
    # The Phase 2 bisect freezes a candidate per combined-check failure, but
    # the candidate it blames can be INNOCENT (the failure came from another
    # candidate; the bisect just isolated a different one). Re-add each
    # check-frozen + individually-safe candidate to the SETTLED working set and
    # re-check; if it still passes, the freeze was a false-positive → recover.
    # Makes the frozen set depend only on real incompatibility, not bisect path.
    safe1_names = {c.name for c in safe1}
    recovered: list[str] = []
    for name in list(frozen.get("check", [])):
        if name not in safe1_names:
            continue                       # genuinely individually-bad (Phase 1)
        cand = next((c for c in safe1 if c.name == name), None)
        if cand is None:
            continue
        rollback_fn(main_crate, snapshot)
        trial = working_set + [cand]
        apply_plan(build_plan(report, lifted=trial))
        if verifier_main.cargo_check().ok:
            working_set = trial
            recovered.append(name)
    if recovered:
        frozen["check"] = [n for n in frozen["check"] if n not in recovered]
        if not frozen["check"]:
            del frozen["check"]
        working_set.sort(key=lambda c: {x.name: i for i, x in
                                        enumerate(safe1)}.get(c.name, 0))
        logger.info(f"[stage_a/e1.parallel] Phase 2b recovered "
                    f"{len(recovered)} false-positive freeze(s): {recovered}")

    # ── Phase 3 — cargo build + W1 sequential on main ──
    while True:
        rollback_fn(main_crate, snapshot)
        if not working_set:
            return [], frozen
        plan = build_plan(report, lifted=working_set)
        apply_plan(plan)

        g_check = verifier_main.cargo_check()
        if not g_check.ok:
            # Unexpected after Phase 2 settled — bisect and continue.
            logger.warning(
                "[stage_a/e1.parallel] Phase 3: post-Phase-2 cargo check "
                "regressed; sequential bisect"
            )
            bad_idx = bisect_fn(report, working_set, main_crate,
                                snapshot, verifier_main.cargo_check)
            bad = working_set.pop(bad_idx)
            frozen.setdefault("check", []).append(bad.name)
            continue

        g_build = verifier_main.cargo_build()
        if not g_build.ok:
            logger.warning("[stage_a/e1.parallel] Phase 3: cargo build FAIL — bisect")
            bad_idx = bisect_fn(report, working_set, main_crate,
                                snapshot, verifier_main.cargo_build)
            bad = working_set.pop(bad_idx)
            frozen.setdefault("build", []).append(bad.name)
            logger.warning(f"[stage_a/e1.parallel]   frozen by build: {bad.name}")
            continue

        g_w1 = gate_w1_fn()
        if not g_w1.ok:
            logger.warning(
                f"[stage_a/e1.parallel] Phase 3: W1 FAIL: {g_w1.detail} — bisect"
            )
            bad_idx = bisect_fn(report, working_set, main_crate,
                                snapshot, gate_w1_fn)
            bad = working_set.pop(bad_idx)
            frozen.setdefault("w1", []).append(bad.name)
            logger.warning(f"[stage_a/e1.parallel]   frozen by W1: {bad.name}")
            continue

        # All gates pass.
        return working_set, frozen
