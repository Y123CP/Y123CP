""                                                                   

                                                                         
                                        

       
                             
                                                                  
                                                                        
                                                                
                                                                      
                                                               
                                 
                                                                         
                                                                           

                                                                     
                                                                      
                                                                    
                                                                              

                                                                   
                                                                          
                                                                          

                                                                         
                                                                         
                                                                        
                                                                    
                                                                       
                                                                      
                                   
   

from __future__ import annotations

import logging
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

from perf_opt.fair_build import audit as fair_audit
from perf_opt.stage_a.analyzer import (
    AnalysisReport,
    Candidate,
    analyze,
    classify,
)
from perf_opt.stage_a.sa_utils import (
    find_sa_report,
    _load_sa_facts,
)
from perf_opt.stage_a.transformer import (
    apply_plan,
    build_plan,
)
from perf_opt.verify.cargo import GateResult, Verifier
from perf_opt.verify import W1Spec, run_w1_suite

logger = logging.getLogger(__name__)


def _append_stage_a_jsonl(project_dir: Path, pass_name: str, fn: str,
                          outcome: str, **extra) -> None:
    """One line per per-fn event to <crate>/.perf_opt/stage_a.jsonl.
    outcome ∈ {keep, reject_check, reject_build, reject_w1, no_candidates,
    no_edits}. Extras: reason:str, detail:str.

    Note: no perf-related outcomes (reject_w2 / aggregate_*) — Stage A is
    a sig-only pass, perf judgement is Stage B's job."""
    try:
        import json as _json
        rec = {"pass": pass_name, "fn": fn, "outcome": outcome, **extra}
        target = project_dir / ".perf_opt" / "stage_a.jsonl"
        target.parent.mkdir(parents=True, exist_ok=True)
        with open(target, "a", encoding="utf-8") as f:
            f.write(_json.dumps(rec, ensure_ascii=False) + "\n")
    except (OSError, TypeError) as e:
        logger.warning(f"[stage_a] jsonl append skipped: {e}")


# ─────────────────────────────────────────────────────────────────
# Result dataclass — sig-level outcomes only
# ─────────────────────────────────────────────────────────────────

@dataclass
class StageARun:
    """Outcome of one Stage A run (Layer 1 == E1 + intra_ptr + deunsafe).

    Fields prune 2026-07-26 (Step 5): removed the record slots for the 7
    deleted pass families (e2 / e2_v2_slice / e3 / e3_v2 / buffer_lift /
    backptr / pr2_tier / struct_param_lift). The remaining fields describe
    the three live passes only."""
    report: AnalysisReport
    # E1: list of candidates that survived strip-extern + cargo + W1.
    applied_candidates: list[Candidate] = field(default_factory=list)
    # Per-gate culprit list (E1 bisect output).
    frozen_by_gate: dict[str, list[str]] = field(default_factory=dict)
    # intra_ptr: signature-frozen body views kept (`fn/param`).
    intra_ptr_lifted: list[str] = field(default_factory=list)
    # de-unsafe: `unsafe fn → fn` markers dropped (fn names).
    deunsafe_fns: list[str] = field(default_factory=list)
    # count of redundant `unsafe { }` blocks stripped (unused_unsafe).
    unsafe_blocks_stripped: int = 0
    # Layer 1 tail sanity: full-corpus W1 replay (every op × every input).
    # per-pass gates use sampled specs (~3-4% coverage); this catches
    # non-sampled-only regressions (brotli 2026-07-16, 46% miss).
    tail_full_replay_ok: bool = True
    tail_full_replay_detail: str = ""
    success: bool = False


# ─────────────────────────────────────────────────────────────────
# Git helpers
# ─────────────────────────────────────────────────────────────────

def _git(args: list[str], cwd: Path, *, check: bool = True,
         capture: bool = False) -> subprocess.CompletedProcess:
    proc = subprocess.run(
        ["git", *args], cwd=str(cwd),
        capture_output=capture, text=True,
    )
    if check and proc.returncode != 0:
        raise RuntimeError(
            f"git {' '.join(args)} failed: {(proc.stderr or '').strip()}"
        )
    return proc


def _snapshot(cwd: Path) -> str:
    return _git(["rev-parse", "HEAD"], cwd, capture=True).stdout.strip()


def _rollback(cwd: Path, sha: str) -> None:
    _git(["reset", "--hard", sha], cwd)
    _git(["clean", "-fd"], cwd)


def _is_clean(cwd: Path) -> bool:
    return not _git(["status", "--porcelain"], cwd, capture=True).stdout.strip()


# ─────────────────────────────────────────────────────────────────
# Bisect — linear scan splitting in halves (used by E1)
# ─────────────────────────────────────────────────────────────────

def _bisect_one_bad(report: AnalysisReport, subset: list[Candidate],
                    project_dir: Path, snapshot: str,
                    gate_fn) -> int:
    """Bisect `subset` to find the index of one Candidate that fails the
    gate. Precondition: applying `subset` fails the gate."""
    lo, hi = 0, len(subset)
    while hi - lo > 1:
        mid = (lo + hi) // 2
        _rollback(project_dir, snapshot)
        plan = build_plan(report, lifted=subset[lo:mid])
        apply_plan(plan)
        result = gate_fn()
        if result.ok:
            lo = mid
        else:
            hi = mid
    return lo


# ─────────────────────────────────────────────────────────────────
# Top-level entry
# ─────────────────────────────────────────────────────────────────

def run_stage_a(project_dir: Path, *,
                w1_specs: list[W1Spec],
                w1_specs_full: list[W1Spec] | None = None,
                classify_mode: str = "conservative",
                hot_allowlist: set[str] | None = None,
                binary_dir: Path | None = None,
                # Defaults set 2026-07-29 → FULL param-lift (E1 + intra_ptr
                # + deunsafe + idiomatize) is the Stage A default. The 2026-07-27
                # "boundary-cleanup only" default was based on a +3.67% churn
                # measurement that the purebin fair bench OVERTURNED (it carried
                # the crate-type build confound): on libopenaptx intra_ptr vs
                # pure-E1 is NEUTRAL (median −0.9pp, 0/6 ops slower) while
                # unsafe fn drops 35→8; on lz4 stage_a vs cleaned is neutral
                # (median −0.89%) with unsafe fn 246→150. The residual cost is
                # E1's extern-C removal itself (~+3.3% on libopenaptx), which is
                # the price of de-unsafe (deunsafe skips extern fns, so keeping
                # extern "C" would zero the unsafe-fn reduction — verified
                # 2026-07-29). See memory: project_lz4_purebin_2026_07_28.
                # Flags kept for per-project opt-out.
                enable_intra_ptr: bool = True,
                enable_deunsafe: bool = True,
                enable_idiomatize: bool = True,
                ) -> StageARun:
    ""                                                           

                  
                                                                      
                                                                                
                                                                        
                                                                           
                                                                          
                                             
                                                                         
                                                                          

                                                                        
                                                                         
                                                          

                                                                 
                                                                        
                                                                       
                                                                 
                                                      

                                                                     
                                                                    
                                                                        
                                                                         
       
    project_dir = project_dir.resolve()
    binary_dir = binary_dir.resolve() if binary_dir else project_dir

    # Phase 0 — fair-build audit
    errs = fair_audit(project_dir, None, strict=False)
    if errs:
        raise RuntimeError(f"fair-build audit failed: {errs}")

    # Phase A — analyze + classify
    report = analyze(project_dir)
    classify(report, mode=classify_mode, hot_allowlist=hot_allowlist)
    run = StageARun(report=report)
    if not report.safe_to_strip:
        logger.info("[stage_a] no candidates to strip; nothing to do")
        return run

    if not _is_clean(project_dir):
        raise RuntimeError(f"{project_dir} working tree dirty; commit first")
    snapshot = _snapshot(project_dir)
    logger.info(f"[stage_a] snapshot HEAD = {snapshot[:12]}")

    verifier = Verifier(project_dir, binary_dir=binary_dir)
    working_set = list(report.safe_to_strip)

    def _gate_w1() -> GateResult:
        # Sampled W1 specs (per op, ~8 inputs) — per-fn per-rewrite fast gate.
        # First failure wins → per-fn rollback reverts the lift that broke a
        # path. Caller (main._stage_a) already baseline-filtered these.
        ok, detail = run_w1_suite(verifier, w1_specs)
        return GateResult(gate="w1", ok=ok, detail=detail)

    def _gate_w1_full() -> GateResult:
        # FULL corpus replay (every golden input across every op) — used ONCE
        # as Layer 1's tail sanity check after all passes converge. Sampled
        # `_gate_w1` covers only ~3-4% of inputs; a lift that breaks a
        # non-sampled input (see brotli 2026-07-16, 46% miss) would pass the
        # per-fn gate but corrupt the deliverable. First-failure-wins keeps
        # the cost bounded to `slowest_op × 1` when clean. Falls back to
        # sampled if the caller didn't supply a full set.
        ok, detail = run_w1_suite(verifier, w1_specs_full or w1_specs)
        return GateResult(gate="w1-full", ok=ok, detail=detail)

    # ── E1: extern-strip with parallel single-candidate trial ──
    #
    # 2026-05-30 — replaced the original sequential bisect loop with a
    # 3-phase parallel implementation (see `parallel_bisect.py`). The
    # original loop was O(n_bad × log(N) × cargo_check) sequential —
    # on libxml2 (~250 candidates, ~100 fail) that is 3-5 hours.
    # The parallel version is roughly `ceil(N / n_workers) × 30 s` for
    # Phase 1 + a single combined check + (rare) sequential bisect on a
    # smaller filtered set.
    #
    # Falls back to the original sequential loop when `n_workers <= 1`
    # (kept around for debugging / single-core hosts).
    from perf_opt.stage_a.parallel_bisect import parallel_e1_strip
    # Worker count: each cargo check uses ~3 cores via rustc's internal
    # parallelism, so on this 36-core host n_workers=12 keeps the box
    # warm without thrashing. For smaller hosts, tune down — disk +
    # memory cost is N × cleaned_crate_size (~250 MB for libxml2).
    import os as _os
    n_workers = max(1, min(12, (_os.cpu_count() or 4) // 3))
    final_working_set, frozen = parallel_e1_strip(
        report=report,
        candidates=working_set,
        main_crate=project_dir,
        main_binary_dir=binary_dir,
        snapshot=snapshot,
        verifier_main=verifier,
        gate_w1_fn=_gate_w1,
        rollback_fn=_rollback,
        bisect_fn=_bisect_one_bad,
        n_workers=n_workers,
    )
    working_set = final_working_set
    for gate_name, names in frozen.items():
        run.frozen_by_gate.setdefault(gate_name, []).extend(names)
    if not working_set:
        logger.warning("[stage_a/e1] all candidates frozen; nothing applied")
        return run

    logger.info(
        f"[stage_a/e1] {len(working_set)} fn(s) lifted; "
        f"{sum(len(v) for v in run.frozen_by_gate.values())} frozen by gate"
    )

    # ── intra_ptr: body-scoped reborrow (safety lift) ──
    # For each raw-ptr param with a read-only, single-depth, non-escaping
    # body, hoist one `let p_view = unsafe { &*p };` (or
    # `slice::from_raw_parts`) at fn entry and rewrite every deref/offset
    # use to go through the view. Signature preserved (fn stays `unsafe fn`,
    # per FFI-boundary contract). Per-fn cargo check + W1 gate.
    # 2026-07-04: LLM half retired (memory: project_perf_opt_pipeline);
    # deterministic path only.
    if enable_intra_ptr:
        _run_intra_ptr_pass(project_dir, verifier, _gate_w1, run)

    # ── de-unsafe: drop `unsafe fn` where the contract is safe (no raw-ptr
    # params, after E2 lifted them to refs). Runs LAST so it sees the final
    # signatures. Per-fn cargo check + build + W1 (all workloads) gate.
    if enable_deunsafe:
        _run_deunsafe_pass(project_dir, verifier, _gate_w1, run)
        # Strip `unsafe { }` blocks rustc flags as unnecessary (the view
        # rewrites + de-unsafe wraps leave many redundant ones, e.g.
        # `unsafe { &mut *s_view }` — a safe reborrow). Removing the `unsafe`
        # keyword is a RUNTIME NO-OP (compile-time only) so W1 is preserved;
        # rustc guarantees the flagged blocks are unnecessary so it still
        # compiles. Re-build for downstream + a single W1 sanity check.
        from perf_opt.stage_a.deunsafe import (
            strip_unnecessary_unsafe, simplify_raw_roundtrip)
        # First collapse `unsafe { &mut *&raw mut x }` → `&mut x` (E2 callsite
        # wrapper's pointless address-then-reborrow round-trip — a whole unsafe
        # block per site, ≈38 on bzip2, behaviour-identical). Then strip the
        # `unused_unsafe` blocks rustc flags (safe reborrows etc.). Both are
        # compile-time-only edits → W1 preserved; one rebuild + W1 sanity check.
        n_simpl = simplify_raw_roundtrip(binary_dir)
        n_stripped = strip_unnecessary_unsafe(binary_dir)
        if n_simpl or n_stripped:
            run.unsafe_blocks_stripped = n_stripped
            verifier.cargo_build()
            g = _gate_w1()
            logger.info(f"[stage_a/deunsafe] simplified {n_simpl} raw round-trip(s)"
                        f" + stripped {n_stripped} unused unsafe block(s) "
                        f"(W1 {'ok' if g.ok else 'FAIL'})")

    # idiomatize is now INDEPENDENT of deunsafe (2026-07-27) — kept ON in the
    # default boundary-cleanup profile. It only touches ref vars the ref-lift
    # produced; when intra_ptr is OFF there are none, so it is a no-op (n_idm=0)
    # — harmless. Split out so it follows intra_ptr when that is re-enabled.
    if enable_idiomatize:
        # ── idiomatize: collapse the `(*x).field → x.field` auto-derefs the
        # ref-lift exposes (E2/intra_ptr make `s` a reference but the rewriters
        # keep the `(*s)` form). DETERMINISTIC, ref-vars-only (never a raw-ptr
        # deref), value-EXACT (dereffing a reference is the no-op the compiler
        # inserts anyway). Replaces `cargo clippy --fix`, which was fragile:
        # broke libcsv's transmute (E0133), structurally skipped heman's
        # overlapping derefs (1264 left), and could leave partial broken edits.
        # The deterministic pass does what clippy couldn't — heman 1263 → 211
        # complexity — and cannot break the build (cargo-check gated, rollback).
        from perf_opt.stage_a.deunsafe import idiomatize_auto_deref
        idm_snap = {p: p.read_bytes() for p in project_dir.rglob("*.rs")
                    if "target" not in p.parts}
        n_idm = idiomatize_auto_deref(project_dir)
        if n_idm:
            ok = verifier.cargo_check().ok
            if ok:
                verifier.cargo_build()
                ok = _gate_w1().ok
            if ok:
                logger.info(f"[stage_a/idiomatize] {n_idm} (*x)→x auto-deref(s) "
                            f"(W1 ok)")
            else:
                for p, b in idm_snap.items():
                    p.write_bytes(b)
                verifier.cargo_build()
                logger.info("[stage_a/idiomatize] broke check/W1 — rolled back")

    # ── Layer 1 tail sanity: full corpus W1 replay ──
    # Per-pass gates use sampled W1 specs (~3-4% of corpus). Run the full
    # replay once at the end to catch a lift that passes sampled but breaks
    # a non-sampled input (see brotli 2026-07-16, 46% miss). Failure is
    # logged but NOT fatal — Layer 1 still returns; the caller inspects the
    # jsonl trail (or bisects git snapshots) to pinpoint the offender. No
    # per-lift blame is attributable at this point since we no longer own
    # a per-lift snapshot chain.
    g_full = _gate_w1_full()
    n_full = len(w1_specs_full or w1_specs)
    run.tail_full_replay_ok = g_full.ok
    run.tail_full_replay_detail = g_full.detail or ""
    if g_full.ok:
        logger.info(f"[stage_a] tail W1 full replay: PASS ({n_full} spec(s))")
    else:
        logger.warning(
            f"[stage_a] tail W1 full replay: FAIL ({n_full} spec(s)) — "
            f"{g_full.detail}"
        )

    run.applied_candidates = list(working_set)
    run.success = True
    return run


# ─────────────────────────────────────────────────────────────────
# Per-pass helpers — each is sig-only, gated on cargo check + W1
# ─────────────────────────────────────────────────────────────────

def _run_intra_ptr_pass(project_dir: Path, verifier: Verifier,
                        gate_w1, run: StageARun) -> None:
    """intra_ptr: signature-frozen body view recovery (body-scoped reborrow).

    For each raw-ptr fn parameter that passes 7 gates (depth==1 ∧ not
    mutated ∧ read-only ∧ non-escape ∧ typed inner ∧ not passed to fns ∧
    length resolvable for arrays), hoist `let p_view = unsafe { &*p };`
    (or `slice::from_raw_parts`) at fn entry and rewrite every body deref /
    offset to go through the safe view. Signature preserved; the fn remains
    `unsafe fn` as an FFI-boundary contract.

    Deterministic rewriter only. The LLM half (Tier 2 null-guarded /
    Tier 3a cast-locals) was retired 2026-07-04 per memory
    project_perf_opt_pipeline; only the safe floor (Kind A + Immutable +
    primitive-inner &mut) is planned. SVF facts via sa_utils.find_sa_report;
    airtight structural fallback when absent (fewer covered fn).

    Per fn: rewrite → cargo check → cargo build → W1 (sampled per-op);
    any failure → byte rollback of touched files. See
    `docs/stage_a_safety_lift_design.md`."""
    from perf_opt.stage_a.borrow_hoist import hoist_conflicting_args
    from perf_opt.stage_a.intra_ptr.plan import plan_crate
    from perf_opt.stage_a.intra_ptr.rewrite import rewrite_fn_in_file, _view_name

    sa_report = find_sa_report(project_dir)
    # allow_null_guarded: still False — the deterministic rewriter cannot
    # build a conditional view inside a null guard (needs the retired LLM half).
    #
    # allow_mutable_struct (struct `&mut` param-lift): DEFAULT ON since
    # 2026-07-29 (STAGE_A_MUT_STRUCT=0 to opt out). The 2026-07-27 default-OFF
    # rested on a "+3.67% pure churn" measurement that the purebin fair bench
    # OVERTURNED (crate-type build confound): on libopenaptx the full lift is
    # NEUTRAL vs pure-E1 (median −0.9pp, 0/6 ops slower) and it is what
    # unlocks deunsafe's unsafe-fn reduction (35→8 / 246→150). Soundness
    # caveat stands: struct &mut interior-back-alias (`s.field.ptr == s`) is
    # not statically proven — plan.py lets the byte-exact W1 gate judge, so
    # W1 must cover the mutating paths (all 12 projects gate on golden.jsonl).
    # See memory: project_lz4_purebin_2026_07_28.
    import os as _os
    _allow_mut_struct = _os.environ.get("STAGE_A_MUT_STRUCT", "1") == "1"
    cp = plan_crate(project_dir, sa_report=sa_report,
                    allow_null_guarded=False,
                    allow_mutable_struct=_allow_mut_struct)
    if not cp.units:
        logger.info("[stage_a/intra_ptr] no eligible body-view candidates")
        return
    logger.info(
        f"[stage_a/intra_ptr] {sum(len(u.pointers) for u in cp.units)} "
        f"view(s) on {len(cp.units)} fn(s) "
        f"(SVF={'yes' if sa_report else 'airtight'})"
    )
    kept = 0
    for u in cp.units:
        fpath = project_dir / u.fn_file
        snap = fpath.read_bytes()
        any_edit = any(
            rewrite_fn_in_file(fpath, u.fn_name, p.anchor,
                               p.target_view, p.length)
            for p in u.pointers)
        if any_edit:
            # Resolve `&mut` view borrow conflicts (E0503): a call passing
            # `&mut *view` alongside a sibling arg that reads the view —
            # legal as raw ptrs, rejected as `&mut`. Hoist reading args
            # into temps before the call. See borrow_hoist.py.
            hoist_conflicting_args(
                fpath, u.fn_name,
                [_view_name(p.anchor) for p in u.pointers])
        if not any_edit or not verifier.cargo_check().ok:
            fpath.write_bytes(snap)
            verifier.cargo_build()
            continue
        g_build = verifier.cargo_build()   # cargo check already passed
        g_w1 = gate_w1() if g_build.ok else g_build
        if g_build.ok and g_w1.ok:
            kept += 1
            for p in u.pointers:
                run.intra_ptr_lifted.append(f"{u.fn_name}/{p.anchor}")
            logger.info(f"[stage_a/intra_ptr] {u.fn_name}: keep "
                        f"{[p.anchor for p in u.pointers]}")
        else:
            fpath.write_bytes(snap)
            verifier.cargo_build()
            _append_stage_a_jsonl(project_dir, "intra_ptr", u.fn_name, "reject",
                                  build_ok=g_build.ok, w1_ok=g_w1.ok,
                                  detail=((g_build.detail if not g_build.ok
                                           else g_w1.detail) or "")[:300])
            logger.info(f"[stage_a/intra_ptr] {u.fn_name}: reject "
                        f"(build={g_build.ok} w1={g_w1.ok}) — rollback")
    logger.info(f"[stage_a/intra_ptr] {kept}/{len(cp.units)} fn(s) body-lifted "
                f"({len(run.intra_ptr_lifted)} view(s))")


def _run_deunsafe_pass(project_dir: Path, verifier: Verifier,
                       gate_w1, run: StageARun) -> None:
    """Drop `unsafe fn` → `fn` for safe-contract functions (no raw-ptr params).

    Per fn: try dropping the `unsafe` keyword alone (PURE win when the body is
    already safe — E2 lifted its params to refs); if cargo check fails (body
    still has unsafe ops), drop + wrap the body in `unsafe { }`. Then cargo
    build + W1 (all workloads). Per-fn byte rollback. See deunsafe.py for the
    soundness argument (no raw-ptr param ⇒ safe contract)."""
    from perf_opt.stage_a.deunsafe import (
        find_candidates, drop_unsafe_kw, wrap_body_unsafe)
    cands = find_candidates(project_dir)
    if not cands:
        logger.info("[stage_a/deunsafe] no safe-contract unsafe-fn candidates")
        return
    logger.info(f"[stage_a/deunsafe] {len(cands)} candidate(s) "
                f"(unsafe fn, no raw-ptr params, non-extern)")

    # Fast path: drop `unsafe fn` AND wrap each body in `unsafe { }` for ALL
    # candidates, then gate ONCE. Wrapping a body in `unsafe { }` is always
    # valid Rust, so this compiles whenever the keyword drops are legal —
    # turning N per-candidate builds into ONE (optipng: 362 cands × ~11s of
    # build+W1 → a single build). The strip pass that runs right after de-unsafe
    # removes the wraps that turn out unnecessary (pure functions whose body was
    # already safe), so the pure/wrapped split is identical to the per-fn loop,
    # just far faster. Fall back to the careful per-fn loop only if the batch
    # fails to compile or breaks W1 (rare — a drop that's not actually legal).
    files = {project_dir / rel for _, rel in cands}
    snaps = {f: f.read_bytes() for f in files}
    for fn_name, rel in cands:
        fpath = project_dir / rel
        if drop_unsafe_kw(fpath, fn_name):
            wrap_body_unsafe(fpath, fn_name)
    if verifier.cargo_check().ok:
        g_build = verifier.cargo_build()
        g_w1 = gate_w1() if g_build.ok else g_build
        if g_build.ok and g_w1.ok:
            run.deunsafe_fns.extend(fn for fn, _ in cands)
            logger.info(f"[stage_a/deunsafe] {len(cands)}/{len(cands)} unsafe-fn "
                        f"dropped (batch drop+wrap; strip trims pure wraps)")
            return
    for f, b in snaps.items():                  # batch failed → revert + per-fn
        f.write_bytes(b)
    verifier.cargo_build()
    logger.info("[stage_a/deunsafe] batch did not pass — per-fn fallback")

    pure = wrapped = 0
    for fn_name, rel in cands:
        fpath = project_dir / rel
        snap = fpath.read_bytes()
        if not drop_unsafe_kw(fpath, fn_name):
            continue
        method = None
        if verifier.cargo_check().ok:            # body already safe → pure win
            method = "drop"
        else:                                    # body needs an unsafe block
            fpath.write_bytes(snap)
            drop_unsafe_kw(fpath, fn_name)
            wrap_body_unsafe(fpath, fn_name)
            if verifier.cargo_check().ok:
                method = "wrap"
            else:
                fpath.write_bytes(snap)
                verifier.cargo_build()
                continue
        g_build = verifier.cargo_build()
        g_w1 = gate_w1() if g_build.ok else g_build
        if g_build.ok and g_w1.ok:
            pure += method == "drop"
            wrapped += method == "wrap"
            run.deunsafe_fns.append(fn_name)
            logger.info(f"[stage_a/deunsafe] {fn_name}: keep ({method})")
        else:
            fpath.write_bytes(snap)
            verifier.cargo_build()
            logger.info(f"[stage_a/deunsafe] {fn_name}: reject "
                        f"(build={g_build.ok} w1={g_w1.ok})")
    logger.info(f"[stage_a/deunsafe] {pure} pure + {wrapped} wrapped = "
                f"{pure + wrapped}/{len(cands)} unsafe-fn dropped")
