"""script_c2rust — end-to-end pipeline driver.

Wires the four pipeline stages (per perf_tree_design.md §2):

    C project           →  c2rust transpile           →  <out>/0_raw/
    <out>/0_raw/        →  cleanup (dedup+norm)       →  <out>/1_cleaned/
    <out>/1_cleaned/    →  Stage A (sig-level lift)   →  <out>/2_stage_a/{crate,harness}/
    <out>/1_cleaned/    →  characterize (§2.5)        →  characterization/<NAME>.{json,jsonl}

Default chain runs the first three (c2rust → cleanup → stage_a) and stops.
`--from profile` is a standalone path (does NOT chain into stage_a).

Paths come from Config/paths.conf:
    SOURCE_PROJECT_BASE   — root of C sources (per project: <base>/<NAME>/)
    C2RUST_OUTPUT_BASE    — root of transpile artifacts (per project:
                            <base>/<NAME>/{0_raw,1_cleaned,2_stage_a,…})

CLI:
    python -m main --project bzip2-1.0.8
        [--manifest PATH]            (default: <out>/workloads/pipeline.toml)
        [--from c2rust|cleanup|stage_a|profile]   (default: c2rust)
        [--binary NAME]              (for stage0: TU whose main() to promote)
        [--skip-oracle]              (for stage1: skip post-stage oracle)
        [--output PATH]              (for profile: HotspotProfile JSON)
        [--skip-tma|--skip-cpi|--skip-ic|--skip-build|--skip-loop-invariant]

`--from` lets you resume mid-pipeline:
    --from c2rust    →  stage0 + stage1 + stage_a (full chain)
    --from cleanup   →  stage1 + stage_a (assume 0_raw exists)
    --from stage_a   →  stage_a only (assume 1_cleaned exists)
    --from perf_opt  →  materializes <out>/3_perf_opt/ from 2_stage_a, runs
                        hot_probe; with --run-agent additionally runs the
                        LLM optimization loop over the hot fns.
    --from profile   →  profile only (standalone — no stage_a)
"""

from __future__ import annotations

import argparse
import hashlib
import logging
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

from Config.paths import get_path
from profiling.pipeline import characterize_project, write_outputs
from stages.stage0_c2rust import C2RustOpts, Stage0C2Rust
from stages.stage1_cleanup import Stage1Cleanup
from stages.cleanup.pre_stage_a import prep_for_stage_a
# Reuse bench_pipeline's working-copy infrastructure helpers. Underscore
# prefix is Python convention, not access control.
from perf_opt.bench_pipeline import (
    _ensure_fair_build,
    _ensure_harness_compat,
    _git_init,
    _rewire_path_dep,
    _rsync_copy,
)
from perf_opt.stage_a.runner import run_stage_a
from perf_opt.agent_perf_opt.driver import run_perf_opt

logger = logging.getLogger("main")


def _resolve_paths(name: str) -> dict:
    """Resolve all per-project paths from `paths.conf`. NAME is the project
    folder name under SOURCE_PROJECT_BASE / C2RUST_OUTPUT_BASE."""
    src_base = Path(get_path("SOURCE_PROJECT_BASE") or "")
    out_base = Path(get_path("C2RUST_OUTPUT_BASE") or "")
    if not src_base.exists():
        raise RuntimeError(f"SOURCE_PROJECT_BASE not found: {src_base} "
                           f"(check Config/paths.conf)")
    if not out_base.exists():
        out_base.mkdir(parents=True, exist_ok=True)

    out_proj = out_base / name
    return {
        "c_project":   src_base / name,
        "out_root":    out_proj,
        "raw":         out_proj / "0_raw",
        "cleaned":     out_proj / "1_cleaned",
        "stage_a":     out_proj / "2_stage_a",
        "perf_opt":    out_proj / "3_perf_opt",
        "manifest_default": out_proj / "workloads" / "pipeline.toml",
    }


# ---------------------------------------------------------------------------
# W1 oracle baseline check — used by both _stage1 and _stage_a to verify
# that the on-disk crate state (after cleanup / after prep) still passes
# the project's W1 oracle (stdout sha256 == oracle.stdout_hash).
# ---------------------------------------------------------------------------

def _w1_run(workload: dict, oracle: dict, *, build_dir: Path,
            input_root: Path) -> tuple[str, str]:
    """cargo build --release from `build_dir`, then run
    `target/release/<workload.binary>` with `workload.args` (substituting
    `$INPUT` from `input_root/workload.input`), and compare stdout
    sha256 + exit code against `oracle`.

    If `workload.wrapper` is set, invoke it as `wrapper <binary> <args>`
    and sha256 the wrapper's stdout (not the binary's). Used by optipng
    / tmux where the binary writes output to a file and the wrapper cats
    that file to stdout so byte-equivalence can be hashed.

    Also substitutes `$RAND` → random hex in args (lets tmux give each
    invocation a unique socket; see tmux/workloads/pipeline.toml).

    Returns (status, detail) where status ∈ {"pass", "fail", "skip"}.
    `skip` covers: missing/placeholder oracle, missing workload binary
    name, or missing manifest — anything that means we cannot make a
    verdict. Callers should treat skip as "not validated" rather than
    "validated OK".
    """
    expected_sha = str(oracle.get("stdout_hash", ""))
    if not expected_sha or expected_sha.startswith("PLACEHOLDER"):
        return "skip", "oracle.stdout_hash missing/placeholder"
    binary = workload.get("binary")
    if not binary:
        return "skip", "workload.binary not set"

    # Build.
    bproc = subprocess.run(
        ["cargo", "build", "--release"], cwd=str(build_dir),
        capture_output=True, text=True, timeout=600,
    )
    if bproc.returncode != 0:
        tail = "\n".join((bproc.stderr or "").splitlines()[-15:])[:500]
        return "fail", f"cargo build FAIL: {tail}"

    bin_path = build_dir / "target" / "release" / binary
    if not bin_path.is_file():
        return "fail", f"binary missing after build: {bin_path}"

    # Resolve $INPUT + $RAND.
    import secrets
    raw_input = workload.get("input", "")
    workload_input = (input_root / raw_input).resolve() if raw_input else None
    rand_tok = secrets.token_hex(4)
    cmd_args = [
        (str(workload_input) if a == "$INPUT" else a).replace("$RAND", rand_tok)
        for a in workload.get("args", [])
    ]

    # Resolve optional wrapper.
    wrapper_rel = workload.get("wrapper", "")
    wrapper_path: Path | None = None
    if wrapper_rel:
        wp = (input_root / wrapper_rel).resolve()
        if not wp.is_file():
            return "fail", f"workload.wrapper not found: {wp}"
        wrapper_path = wp

    # Run (wrapper invocation = wrapper <binary> <args>; bare = <binary> <args>).
    if wrapper_path is not None:
        run_cmd = [str(wrapper_path), str(bin_path), *cmd_args]
    else:
        run_cmd = [str(bin_path), *cmd_args]
    try:
        rproc = subprocess.run(
            run_cmd,
            capture_output=True,
            timeout=int(workload.get("timeout_s", 180)),
        )
    except subprocess.TimeoutExpired:
        return "fail", "workload timeout"

    expected_exit = int(oracle.get("exit_code", 0))
    if rproc.returncode != expected_exit:
        return "fail", f"exit {rproc.returncode} ≠ {expected_exit}"

    digest = hashlib.sha256(rproc.stdout).hexdigest()
    if digest != expected_sha:
        return "fail", (
            f"stdout sha256 {digest[:16]}… ≠ oracle {expected_sha[:16]}…"
        )
    return "pass", "stdout hash + exit code match oracle"


def _w1_baseline_stage1(paths: dict, manifest: Path) -> tuple[str, str]:
    """Run W1 oracle on the 1_cleaned output.

    For self-bin projects (workload has no harness_dir), build + run
    directly from 1_cleaned. For harness projects, set up a disposable
    `(crate_clone, harness_clone)` pair under /tmp so we don't mutate
    1_cleaned with `_ensure_harness_compat`'s shim modules. Caller is
    responsible for catching/logging.
    """
    if not manifest.is_file():
        return "skip", "no manifest"
    data = tomllib.loads(manifest.read_text(encoding="utf-8"))
    workload = data.get("workload", {})
    oracle = data.get("oracle", {})

    cleaned = paths["cleaned"]
    input_root = paths["out_root"] / "workloads"
    harness_name = workload.get("harness_dir", "")

    if not harness_name:
        # Self-bin: build + run from 1_cleaned directly.
        return _w1_run(workload, oracle, build_dir=cleaned, input_root=input_root)

    # Harness project: clone both 1_cleaned and the harness into /tmp so
    # `_ensure_harness_compat`'s lib.rs shim mutations don't leak back
    # into 1_cleaned. The Stage A workflow does the same thing for the
    # mutable lift target.
    harness_src = input_root / harness_name
    if not harness_src.is_dir():
        return "fail", f"harness_dir not found: {harness_src}"
    tmp_root = Path("/tmp") / f"stage1_w1_{paths['out_root'].name}"
    if tmp_root.exists():
        shutil.rmtree(tmp_root)
    tmp_root.mkdir(parents=True)
    try:
        tmp_crate = tmp_root / "crate"
        _rsync_copy(cleaned, tmp_crate)
        _ensure_fair_build(tmp_crate)
        tmp_harness = tmp_root / "harness"
        _rsync_copy(harness_src, tmp_harness)
        _ensure_fair_build(tmp_harness)
        _rewire_path_dep(tmp_harness / "Cargo.toml", tmp_crate)
        _ensure_harness_compat(tmp_crate, tmp_harness)
        return _w1_run(workload, oracle, build_dir=tmp_harness,
                       input_root=input_root)
    finally:
        shutil.rmtree(tmp_root, ignore_errors=True)


def _log_w1_verdict(stage_label: str, status: str, detail: str) -> None:
    """Uniform reporting of W1 outcomes. `fail` is logged as WARNING
    (not raised) — the failure is informational; the user can decide
    whether to halt the chain or proceed."""
    if status == "pass":
        logger.info(f"[{stage_label}] W1 baseline: PASS — {detail}")
    elif status == "skip":
        logger.info(f"[{stage_label}] W1 baseline: skip — {detail}")
    else:
        logger.warning(f"[{stage_label}] W1 baseline: FAIL — {detail}")


# ---------------------------------------------------------------------------
# Stage runners
# ---------------------------------------------------------------------------

def _stage0(p: dict, *, binary: str | None) -> None:
    logger.info(f"[stage0] c2rust transpile  {p['c_project']} → {p['raw']}")
    if not p["c_project"].is_dir():
        raise RuntimeError(f"C project not found: {p['c_project']}")
    stage = Stage0C2Rust(args=argparse.Namespace(),
                         opts=C2RustOpts(binary=binary))
    res = stage.run(p["c_project"], p["raw"])
    if not res.ok:
        raise RuntimeError(f"[stage0] failed: {res}")
    logger.info(f"[stage0] OK → {p['raw']}")


def _stage1(p: dict, *, skip_oracle: bool, manifest: Path | None = None) -> None:
    logger.info(f"[stage1] cleanup  {p['raw']} → {p['cleaned']}")
    if not p["raw"].is_dir():
        raise RuntimeError(f"0_raw not found (run stage0 first): {p['raw']}")
    stage = Stage1Cleanup(args=argparse.Namespace(skip_oracle=skip_oracle))
    res = stage.run(p["raw"], p["cleaned"])
    if not res.ok:
        raise RuntimeError(f"[stage1] failed: {res}")
    logger.info(f"[stage1] OK → {p['cleaned']}")

    # W1 oracle baseline: verify 1_cleaned behaves equivalently to the C
    # reference. Currently `cargo check` is the only gate inside Stage 1
    # (via step_rollback), so semantic drift that happens to compile —
    # e.g. a wrong dedup that merges two layout-divergent structs, or an
    # extern decl prune that removes a fn called via fn-pointer — would
    # slip through silently and only surface much later at Stage A's
    # post-lift W1 check (or worse, not at all if the project has 0
    # Stage A candidates, like json_h / libtree).
    if manifest is not None:
        status, detail = _w1_baseline_stage1(p, manifest)
        _log_w1_verdict("stage1", status, detail)


def _stage_a(p: dict, *, manifest: Path, enable_pr2_tier: bool = True) -> None:
    # enable_pr2_tier: CLI back-compat kwarg (--no-pr2-tier). pr2_tier was
    # deleted in Layer 1 cleanup (Step 3, commit c61a3171e) — the flag is
    # silently accepted so existing invocations don't break, and no longer
    # forwarded to run_stage_a.
    _ = enable_pr2_tier
    """Stage A: sig-level inter-procedural semantic refinement.

    Reads workload from `<out>/workloads/harness_gen/<>_harness/` (harness_gen
    golden corpus). Snapshots `1_cleaned` into `2_stage_a/crate/`, stands up a
    thin harness driver against the snapshot, runs the pre-Stage-A cleanup
    chain, then invokes `run_stage_a` with W1 specs from `golden.jsonl`
    (sampled per op for per-fn gating; full corpus for final replay).

    Layout produced:
      <out>/2_stage_a/crate/      ← mutated lib (Stage A target; git-init'd
                                    so Stage A can snapshot/rollback per fn)
      <out>/2_stage_a/harness/    ← thin driver scaffold (Cargo.toml
                                    path-dep → ../crate); corpus + golden
                                    stay in workloads/harness_gen/, read
                                    in place

    2026-07-26: retired legacy TOML manifest path (pipeline.toml /
    correctness*.toml / oracle.stdout_hash) — see verify/workload.py
    docstring. `manifest` kwarg kept for CLI back-compat and ignored.
    Projects without harness_gen assets (e.g. empirical-corpus
    dataset_trans_process/*) cannot use --from stage_a until they
    generate a harness_gen golden.
    """
    import shutil
    from perf_opt.verify import filter_baseline_specs
    from perf_opt.verify.cargo import Verifier
    from perf_opt.verify.functional import (
        WorkloadAssets, build_driver, golden_specs,
    )
    _ = manifest  # legacy CLI kwarg — retired TOML path, ignored

    logger.info(f"[stage_a] {p['cleaned']} → {p['stage_a']}")
    if not p["cleaned"].is_dir():
        raise RuntimeError(f"1_cleaned not found (run stage1 first): {p['cleaned']}")

    assets = WorkloadAssets.discover(p["out_root"])
    if not assets:
        raise RuntimeError(
            f"no harness_gen assets for {p['out_root']} — Stage A needs "
            f"<proj>/workloads/harness_gen/<>_harness/golden.jsonl "
            f"(see verify/workload.py)"
        )

    work_root = p["stage_a"]
    if work_root.exists():
        shutil.rmtree(work_root)
    work_root.mkdir(parents=True)
    crate_dst = work_root / "crate"
    harness_dst = work_root / "harness"

    _rsync_copy(p["cleaned"], crate_dst)
    _ensure_fair_build(crate_dst)
    build_driver(assets, crate_dst, harness_dst)

    # Pre-Stage-A cleanup pipeline (canon anon types / unify dup pub
    # types / unify opaque foreign / normalize extern blocks / strip
    # staticlib). Each step is rollback-gated by step_rollback.
    prep_for_stage_a(crate_dst, harness_dst)

    # Git-init so Stage A's per-fn snapshot/rollback works.
    _git_init(crate_dst, "1_cleaned")

    # W1 specs from golden.jsonl:
    #   inner_specs — sampled per op (fast per-fn W1 gate)
    #   full_specs  — every corpus entry (definitive final replay)
    inner_specs = golden_specs(assets, sample_per_op=8)
    full_specs = golden_specs(assets, sample_per_op=0)

    # Baseline filter — precondition of runner._gate_w1 ("caller already
    # baseline-filtered these specs"). A spec the pristine baseline itself
    # cannot reproduce (stale/tainted oracle, e.g. an op that folds a raw
    # pointer address into its digest → build-sensitive) does NOT indicate
    # a lift regression; gating on it would roll back every lift. Drop it
    # here rather than in-loop. Aborts only if none survive (broken crate
    # copy or wholesale oracle drift).
    baseline_verifier = Verifier(crate_dst, binary_dir=harness_dst)
    build_gate = baseline_verifier.cargo_build()
    if not build_gate.ok:
        raise RuntimeError(
            f"[stage_a] baseline cargo build failed: {build_gate.detail}"
        )
    inner_specs = filter_baseline_specs(
        baseline_verifier, inner_specs, label="stage_a inner"
    )
    full_specs = filter_baseline_specs(
        baseline_verifier, full_specs, label="stage_a full"
    )

    logger.info(
        f"[stage_a] W1 gates: {len(inner_specs)} sampled / "
        f"{len(full_specs)} full spec(s) (post baseline filter)"
    )

    # classify_mode: aggressive (default, back-compat) strips every internal
    # extern "C" fn; conservative strips only #[no_mangle]/#[linkage] boundary
    # exports and leaves internal helpers untouched. Overridable via env for
    # A/B experiments without touching this default (see
    # project_stage_a_slowdown_attribution_2026_07_27: aggressive perturbs
    # LLVM's global inlining on numeric kernels).
    import os as _os
    _classify_mode = _os.environ.get("STAGE_A_CLASSIFY_MODE", "aggressive")
    # Full param-lift is the DEFAULT since 2026-07-29 (purebin retest overturned
    # the 2026-07-27 churn finding — see runner.run_stage_a docs). Set
    # STAGE_A_ENABLE_INTRA_PTR=0 / STAGE_A_ENABLE_DEUNSAFE=0 /
    # STAGE_A_MUT_STRUCT=0 (read inside _run_intra_ptr_pass) to opt out
    # per-project.
    _enable_intra_ptr = _os.environ.get("STAGE_A_ENABLE_INTRA_PTR", "1") == "1"
    _enable_deunsafe = _os.environ.get("STAGE_A_ENABLE_DEUNSAFE", "1") == "1"
    result = run_stage_a(
        crate_dst,
        w1_specs=inner_specs,
        w1_specs_full=full_specs,
        classify_mode=_classify_mode,
        binary_dir=harness_dst,
        enable_intra_ptr=_enable_intra_ptr,
        enable_deunsafe=_enable_deunsafe,
    )
    logger.info(f"[stage_a] classify_mode={_classify_mode}")
    n_lifted = len(result.report.safe_to_strip) - sum(
        len(v) for v in result.frozen_by_gate.values()
    )
    logger.info(
        f"[stage_a] OK  candidates={len(result.report.safe_to_strip)} "
        f"lifted={n_lifted} "
        f"frozen={dict((k, len(v)) for k, v in result.frozen_by_gate.items())} "
        f"→ {crate_dst}"
    )


def _profile(p: dict, *, manifest: Path, output: Path,
             skip_tma: bool, skip_cpi: bool, skip_ic: bool,
             skip_build: bool, skip_loop_invariant: bool) -> None:
    logger.info(f"[profile] characterize  {p['cleaned']} via {manifest} → {output}")
    if not p["cleaned"].is_dir():
        raise RuntimeError(f"1_cleaned not found (run stage1 first): {p['cleaned']}")
    if not manifest.is_file():
        raise RuntimeError(f"workload manifest not found: {manifest}")
    report = characterize_project(
        project_path        = p["cleaned"],
        manifest_path       = manifest,
        skip_build          = skip_build,
        skip_tma            = skip_tma,
        skip_cpi            = skip_cpi,
        skip_ic             = skip_ic,
        skip_loop_invariant = skip_loop_invariant,
    )
    out_json, out_jsonl = write_outputs(report, output)
    logger.info(f"[profile] OK  json={out_json}  jsonl={out_jsonl}")
    # Step A: LICM lives per-hotspot now (evidence.source_patterns), not at top.
    licm_total = sum(
        1 for hs in report.hotspots for sp in hs.evidence.source_patterns
        if sp.kind == "loop_invariant_branch"
    )
    logger.info(f"[profile]   hot_fns={len(report.hotspots)} "
                f"skipped={len(report.skipped)} "
                f"LICM={licm_total}")


def _perf_opt(p: dict, *, tau: float, top_n: int,
              run_agent: bool = False, arm: str = "full",
              only_fns: "list[str] | None" = None,
              reuse_from: "Path | None" = None) -> None:
    """Perf-optimization stage (`--from perf_opt`) — consumes 2_stage_a and
    produces `<out>/3_perf_opt/`:

      3_perf_opt/crate/          — mutable working copy of 2_stage_a/crate
                                   (git-init'd, fair-build, debug=2)
      3_perf_opt/harness/        — 2_stage_a/harness rewired to the copy
      3_perf_opt/baseline.json   — initial per-op W2 measurement (pre-opt anchor)
      3_perf_opt/hotspots.json   — hot functions + dropped extern_wrappers
      3_perf_opt/evidence/*.json — one full evidence pack per hot function

    With --run-agent, additionally runs the LLM-driven per-fn per-rule
    optimization loop after characterize; commits accepted rewrites to git.

    Standalone: assumes 2_stage_a exists (does NOT chain from earlier stages).
    Wall-clock measurements need frequency-locked cores — invoke via
    `bash perf_run.sh <venv>/python -m main ...` on this host.
    """
    logger.info(f"[perf_opt] {p['stage_a']} → {p['perf_opt']}")
    if not p["stage_a"].is_dir():
        raise RuntimeError(
            f"2_stage_a not found (run stage_a first): {p['stage_a']}")
    run_perf_opt(p["out_root"], tau=tau, top_n=top_n,
                 run_agent=run_agent, arm=arm,
                 only_fns=only_fns, reuse_from=reuse_from)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def _cli() -> int:
    p = argparse.ArgumentParser(
        description="C → c2rust → cleanup → Stage A end-to-end driver.",
    )
    p.add_argument("--project", required=True,
                   help="Project name under SOURCE_PROJECT_BASE / "
                        "C2RUST_OUTPUT_BASE (e.g. bzip2-1.0.8)")
    p.add_argument("--manifest", default=None,
                   help="Workload manifest path. Default: "
                        "<out>/workloads/pipeline.toml")
    p.add_argument("--output", default=None,
                   help="[profile] HotspotProfile JSON output. Default: "
                        "characterization/<NAME>.json")
    p.add_argument("--from", dest="start_from",
                   choices=("c2rust", "cleanup", "stage_a",
                            "perf_opt", "profile"),
                   default="c2rust",
                   help="Start at this stage. Default chain: "
                        "c2rust → cleanup → stage_a. `perf_opt` is "
                        "standalone (assume 2_stage_a exists) — materializes "
                        "3_perf_opt/ from 2_stage_a and runs hot_probe (+ "
                        "optional --run-agent for LLM optimization loop). "
                        "`profile` is standalone too.")
    # stage0
    p.add_argument("--binary", default=None,
                   help="[stage0] TU whose main() to promote as Rust bin")
    # stage1
    p.add_argument("--skip-oracle", action="store_true",
                   help="[stage1] skip post-stage oracle check")
    # profile (pass-through to characterize_project)
    p.add_argument("--skip-build",          action="store_true")
    p.add_argument("--skip-tma",            action="store_true")
    p.add_argument("--skip-cpi",            action="store_true")
    p.add_argument("--skip-ic",             action="store_true")
    p.add_argument("--skip-loop-invariant", action="store_true")
    # perf_opt knobs
    p.add_argument("--top-n", type=int, default=0,
                   help="[perf_opt] keep at most this many hot fns; "
                        "0 = no cap (keep every fn above --skip-threshold)")
    p.add_argument("--skip-threshold", type=float, default=3.0,
                   help="[perf_opt] self_time_pct gate (a.k.a. tau);"
                        " fns below this % are dropped.")
    # stage_a knobs
    p.add_argument("--no-pr2-tier", action="store_true",
                   help="[stage_a] disable pr2_tier (PR2 OOPSLA baseline "
                        "replay). Use for perf_opt / production runs — "
                        "pr2_tier's decision logic optimizes for safety "
                        "typing, not perf, and produces observable regressions "
                        "(e.g. slice::from_raw_parts(p, strlen(p)) in loops). "
                        "Keep ON only for empirical study / paper baseline.")
    # agent_perf_opt knobs
    p.add_argument("--ablation-freeform", action="store_true",
                   help="[perf_opt] ABLATION arm: remove the rule layer. "
                        "Each hot fn is handed to the LLM whole — no "
                        "Optimization_Card, no region split, no rule-hit "
                        "gating, no typed rules — and whatever comes back "
                        "goes through the SAME build/W1/W2 gates. Writes to "
                        "3_perf_opt_freeform/ so the full arm's results are "
                        "untouched. Requires --run-agent.")
    p.add_argument("--only-fns", nargs="+", default=None, metavar="FN",
                   help="[perf_opt] targeted run: detection runs over the whole "
                        "hot set as usual, then only these functions go to the "
                        "agent. Writes 3_perf_opt_targeted/ (never 3_perf_opt/), "
                        "skips the baseline measurement. Requires --run-agent.")
    p.add_argument("--reuse-from", default=None, metavar="DIR",
                   help="[perf_opt] reuse DIR/hotspots.json instead of "
                        "re-locating (checked: exists, same ops, newer than the "
                        "perf inputs). DIR is a previous full run's output.")
    p.add_argument("--run-agent", action="store_true",
                   help="[perf_opt] after hot_probe, run LLM-driven "
                        "per-fn per-rule optimization loop (commits accepted "
                        "rewrites to git). Requires DEFAULT_LLM_MODEL in "
                        "Config/paths.conf.")
    args = p.parse_args()

    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
        datefmt="%H:%M:%S",
    )

    paths = _resolve_paths(args.project)
    manifest = Path(args.manifest) if args.manifest else paths["manifest_default"]
    output = Path(args.output) if args.output else \
        Path("characterization") / f"{args.project}.json"

    def _run_profile() -> None:
        _profile(paths, manifest=manifest, output=output,
                 skip_build=args.skip_build,
                 skip_tma=args.skip_tma,
                 skip_cpi=args.skip_cpi,
                 skip_ic=args.skip_ic,
                 skip_loop_invariant=args.skip_loop_invariant)

    # Run the requested span — every stage is its own try/except so a
    # failure stops the chain with a useful message.
    try:
        if args.start_from == "c2rust":
            _stage0(paths, binary=args.binary)
            _stage1(paths, skip_oracle=args.skip_oracle, manifest=manifest)
            _stage_a(paths, manifest=manifest,
                      enable_pr2_tier=not args.no_pr2_tier)
        elif args.start_from == "cleanup":
            _stage1(paths, skip_oracle=args.skip_oracle, manifest=manifest)
            _stage_a(paths, manifest=manifest,
                      enable_pr2_tier=not args.no_pr2_tier)
        elif args.start_from == "stage_a":
            _stage_a(paths, manifest=manifest,
                      enable_pr2_tier=not args.no_pr2_tier)
        elif args.start_from == "perf_opt":
            if args.ablation_freeform and not args.run_agent:
                raise RuntimeError(
                    "--ablation-freeform needs --run-agent: the ablation IS "
                    "the rewrite loop, and without it the run stops after "
                    "hot_probe having changed nothing.")
            if args.only_fns and not args.run_agent:
                raise RuntimeError(
                    "--only-fns needs --run-agent: a targeted run exists to "
                    "hand those functions to the agent.")
            if args.only_fns and args.ablation_freeform:
                raise RuntimeError(
                    "--only-fns cannot be combined with --ablation-freeform: "
                    "the ablation must share the full arm's whole target set.")
            if args.reuse_from and not Path(args.reuse_from).is_dir():
                raise RuntimeError(f"--reuse-from: {args.reuse_from} is not a "
                                   f"directory")
            _perf_opt(paths, tau=args.skip_threshold, top_n=args.top_n,
                       run_agent=args.run_agent,
                       arm="freeform" if args.ablation_freeform else "full",
                       only_fns=args.only_fns,
                       reuse_from=Path(args.reuse_from) if args.reuse_from else None)
        else:    # "profile"
            _run_profile()
    except RuntimeError as e:
        logger.error(str(e))
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(_cli())
