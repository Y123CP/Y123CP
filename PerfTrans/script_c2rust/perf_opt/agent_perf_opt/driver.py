""                                

                       

                                                                                      
                                                                            
                                                                                
                                                                           
                                                              
                                                                         
                                                                  
                                                                              
                                                                          
                                                                             
                                                                                  

                             
                                                                             
                                                                                      
                                                                                 

                                        
                                                                              
                                                                                   
                                                                                   
                                                                                  
                                                                         
                                                                               
                                                             

             
                                                            
                                                                              
                                                                             

                                                                              
                                         
   

from __future__ import annotations

import json
import re
import hashlib
import logging
import os
import platform
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

from harness_gen.build_env import (ensure_canonical_codegen, ensure_debuginfo,
                                   ensure_stable_debug_paths)
from perf_opt.verify import WorkloadAssets, autotune_iters, measure_harness
from perf_opt.verify import measure as _measure
from perf_opt.verify.crate_ops import (_ensure_fair_build, _git_init,
                                        _rewire_path_dep, _rsync_copy)

logger = logging.getLogger("perf_opt.driver")

                                                                            
                                                                       
                                                               
                                                
#
                                                               
                                                           
                                           
                                       
                                                          
_INSTRUMENTED_BUILD_TIMEOUT_S = int(
    os.environ.get("PERF_OPT_INSTRUMENTED_BUILD_TIMEOUT", "3600"))

# hot_probe is a component the driver calls; guarded so the initial-baseline
# half of the driver runs standalone before hot_probe lands.
try:
    from perf_opt.hot_probe import characterize, locate_hotspots  # type: ignore
except ImportError:
    locate_hotspots = None
    characterize = None


@dataclass
class PerfOptResult:
    out_root: Path
    opt_dir: Path
    crate: Path
    harness_dir: Path
    harness_bin: Path
    ops: list[str]
    baseline: dict = field(default_factory=dict)   # op -> {wall_ms, insns, cv}
    hotspots: list = field(default_factory=list)
    evidence: list = field(default_factory=list)   # EvidencePack per hot fn
    agent_result: object = None                    # AgentResult if --run-agent, else None


def _log_agent_done(agent_result: object) -> None:
    """Log the final agent outcome categories without collapsing W2 results."""
    total_wall = (
        f"{agent_result.total_wall_gain_pct:+.2f}%"
        if agent_result.total_wall_gain_pct is not None
        else "unmeasured"
    )
    logger.info(
        "[perf_opt] agent done: commit=%d, abstain=%d, form_rejected=%d, "
        "build_rejected=%d, "
        "w1_rejected=%d, w2_no_gain=%d, w2_regress=%d, unmeasurable=%d, "
        "regress_compat=%d, syntax_or_internal=%d; direct total wall=%s",
        agent_result.committed_attempts,
        agent_result.abstained_count,
        agent_result.form_rejected_count,
        agent_result.rejected_build_count,
        agent_result.w1_failed_count,
        agent_result.w2_no_gain_count,
        agent_result.w2_regress_count,
        agent_result.unmeasurable_count,
        agent_result.regressed_count,
        agent_result.syntax_failed_count,
        total_wall,
    )


# ── op enumeration ───────────────────────────────────────────────────────────
def _read_ops(assets: WorkloadAssets) -> list[str]:
    """Ops for this harness, from the most authoritative PER-HARNESS source.

    Order matters: prefer sources that belong to `harness_src` itself over the
    SHARED `harness_src.parent/logs/spec.json`. When a project has more than one
    harness under `workloads/harness_gen/` (e.g. a functional `<proj>_harness`
    plus a `<proj>_raw_harness` from a separate harness_gen run), they share the
    parent `logs/`, so the last writer's spec.json can describe a *different*
    harness. Reading it here silently returns foreign op names that match no
    `perf_inputs/<op>.perf.bin` → 0 perf ops → 0 hot fns. Deriving ops from this
    harness's own corpus / golden.jsonl (golden is guaranteed present — discover
    requires it) avoids that poisoning. The shared parent spec is last resort.
    """
    # 1. This harness's OWN spec.json (richest, if it has one).
    own_spec = assets.harness_src / "logs" / "spec.json"
    if own_spec.is_file():
        ops = [o["name"] for o in json.loads(own_spec.read_text()).get("operations", [])]
        if ops:
            return ops
    # 2. This harness's own corpus subdirs.
    corpus = assets.harness_src / "corpus"
    if corpus.is_dir():
        ops = sorted(p.name for p in corpus.iterdir() if p.is_dir())
        if ops:
            return ops
    # 3. This harness's own golden.jsonl (the W1 contract — always present).
    golden = assets.golden_path
    if golden.is_file():
        ops = sorted({
            json.loads(line)["op"]
            for line in golden.read_text().splitlines()
            if line.strip()
        })
        if ops:
            return ops
    # 4. LAST RESORT: shared parent logs/spec.json (may describe another harness).
    parent_spec = assets.harness_src.parent / "logs" / "spec.json"
    if parent_spec.is_file():
        ops = [o["name"] for o in json.loads(parent_spec.read_text()).get("operations", [])]
        if ops:
            return ops
    return []


def _low_library_share_ops(harness_src: Path) -> dict[str, str]:
    """{op -> verdict} for ops the workload builder could not confirm as
    library-dominated. Verdict is "harness_bound" (measured) or "unknown"
    (attribution was not reliable enough to say).

    Diagnostic only — see `_perf_ops` for why this must not exclude anything.
    """
    try:
        rows = json.loads((harness_src / "perf_workload.json")
                          .read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return {}
    if not isinstance(rows, list):
        return {}
    out: dict[str, str] = {}
    for row in rows:
        if not isinstance(row, dict) or not row.get("op"):
            continue
        flag = row.get("harness_bound")
        if flag is True:
            out[row["op"]] = "harness_bound"
        elif flag is None and "harness_bound" in row:
            out[row["op"]] = "unknown"
    return out


def _perf_ops(assets: WorkloadAssets, ops: list[str]) -> list[str]:
    """Ops with a perf input. Ops without one are functional-only.

    Deliberately does NOT drop ops the workload builder flagged as
    harness-bound, for three reasons found by measurement:

    1. They cannot cast the vote that would justify dropping them. W2 scopes
       each candidate to `hf.per_op` — the ops the rewritten function actually
       appears in — so an op with no rewritable hot function is never part of
       any candidate's gate in the first place.
    2. Dropping one is not free: this list is also what `hot_probe` profiles,
       so an op removed here can never contribute a hot function, and any
       function hot ONLY in that op is lost for the whole run.
    3. The flag is not reliable enough to spend that. It is a symbol-level
       measurement, and small library functions are inlined into the harness at
       opt-level 3 + fat LTO, which charges their samples to the harness. On
       libxml2 six of ten ops were flagged and at most two deserved it: one had
       its hottest symbol IN the library, and three were library work absorbed
       into a harness symbol.

    So report it and let the run decide on real hot functions instead.
    """
    with_input = [op for op in ops
                  if _measure.pick_input(assets.harness_src, op) is not None]
    flagged = _low_library_share_ops(assets.harness_src)
    noted = {op: v for op, v in flagged.items() if op in with_input}
    if noted:
        logger.warning(
            "[perf_opt] workload builder could not confirm library dominance "
            "for: %s — kept as perf targets (hot_probe decides on real hot "
            "functions); if one yields no hot function, that is the answer",
            ", ".join(f"{op}({v})" for op, v in sorted(noted.items())))
    return with_input


# ── materialize working copy ─────────────────────────────────────────────────
def _materialize(stage_a: Path, opt_dir: Path) -> tuple[Path, Path]:
    """Copy 2_stage_a/{crate,harness} → 3_perf_opt/{crate,harness}, rewire the harness
    path-dep to 3_perf_opt/crate, fair-build + git-init the crate. Returns
    (3_perf_opt/crate, 3_perf_opt/harness)."""
    if opt_dir.exists():
        shutil.rmtree(opt_dir)
    opt_dir.mkdir(parents=True)

    crate_src, harness_src = stage_a / "crate", stage_a / "harness"
    if not (crate_src / "Cargo.toml").is_file():
        raise RuntimeError(f"2_stage_a/crate not found: {crate_src}")
    if not (harness_src / "Cargo.toml").is_file():
        raise RuntimeError(
            f"2_stage_a/harness not found: {harness_src} — the perf stage needs "
            f"stage_a's boundary-adapted harness (the workloads harness calls "
            f"the un-lifted raw-ptr signatures and won't build vs 2_stage_a).")

    crate = opt_dir / "crate"
    harness = opt_dir / "harness"
    _rsync_copy(crate_src, crate, extra_ignore=("target", "target-optfind"))
    _rsync_copy(harness_src, harness, extra_ignore=("target",))

    # Rewire the harness path-dep from 2_stage_a/crate → 3_perf_opt/crate.
    _rewire_path_dep(harness / "Cargo.toml", crate)
    _ensure_fair_build(crate)
    # Build with debug info so hot_probe's inline-frame self-time attribution
    # works (recovers crate functions inlined into the harness / delegating to
    # libc). Cargo compiles the whole graph with the TOP-LEVEL (harness)
    # profile, so this must go on the harness. debug info is metadata — codegen
    # and W2 wall-clock are unaffected (only the binary grows).
    ensure_debuginfo(harness / "Cargo.toml")
    # ...and then take the build path back OUT of the binary. `debug = 2`
    # writes the absolute build directory into DWARF, so a tree named
    # `3_perf_opt_freeform` yields a bigger binary — and a different code
    # layout — than the same source under `3_perf_opt`. Measured on fzy at
    # 6.182% of wall clock between two semantically identical pristine
    # harnesses. Both arms must remap to the same constant or their numbers
    # are not on one scale. See harness_gen.build_env.DEBUG_LEVEL.
    ensure_stable_debug_paths(harness, opt_dir)
    # Build the timed binary with the SAME codegen the evaluation uses.
    # `.cargo/config.toml` resolves from the invocation directory, so the
    # library crate's own `-Ctarget-cpu=native` never applies here — without
    # this the whole binary was built for the x86-64 baseline while the paper
    # reports `target-cpu=native` numbers. On fzy the same rewrite measured
    # -22.3% under the baseline and +0.3% under native, so the gate was
    # accepting changes worth nothing in the reported configuration.
    ensure_canonical_codegen(harness)
    _git_init(crate, "2_stage_a")
    return crate, harness


def _build_harness(harness_dir: Path) -> Path:
    """cargo build --release the harness in place → its `harness` binary."""
    proc = subprocess.run(["cargo", "build", "--release"],
                          cwd=str(harness_dir), capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(
            f"harness build failed in {harness_dir}:\n{(proc.stderr or '')[-1500:]}")
    binp = harness_dir / "target" / "release" / "harness"
    if not binp.is_file():
        raise RuntimeError(f"harness built but binary missing: {binp}")
    return binp


# ── initial W2 baseline ──────────────────────────────────────────────────────
def _probe_op_error(harness_bin: Path, op: str, inp: Path) -> str:
    """One run capturing why an op is unmeasurable (non-zero exit): the panic
    line for a Rust bounds/overflow abort, else the first stderr line, else the
    exit code. Perf inputs that overflow a fixed C-array bound (a large gen_perf
    input the small corpus never reached) or don't match the op's expected
    format surface here — a workload-data issue, not a rewrite target."""
    proc = subprocess.run([str(harness_bin), op, str(inp), "1"],
                          capture_output=True, text=True)
    err = proc.stderr or ""
    for line in err.splitlines():
        if "panicked at" in line or "index out of bounds" in line \
                or "overflow" in line:
            return f"exit={proc.returncode}: {line.strip()[:160]}"
    first = next((ln for ln in err.splitlines() if ln.strip()), "")
    return f"exit={proc.returncode}: {first.strip()[:160]}" if first \
        else f"exit={proc.returncode} (no stderr)"




def _harness_crate_name(harness_src: Path) -> str:
    """The harness crate's own name, so its symbols can be told apart."""
    try:
        text = (harness_src / "Cargo.toml").read_text(encoding="utf-8")
    except OSError:
        return ""
    m = re.search(r'^\s*name\s*=\s*"([^"]+)"', text, re.M)
    return m.group(1) if m else ""


def _initial_baseline(harness_bin: Path, assets: WorkloadAssets,
                      perf_ops: list[str], opt_dir: Path,
                      *, target_wall: float | None = None,
                      source_head: str = "") -> dict:
    """Measure the pristine working copy on every perf op — the pre-optimization
    performance state AND the anti-drift `absolute` anchor. Written to
    opt/baseline.json. MUST run under the frequency lock (perf_run.sh)."""
    tw = target_wall or _measure.TARGET_WALL
    measurements: dict = {}
    for op in perf_ops:
        inp = _measure.pick_input(assets.harness_src, op)
        iters = autotune_iters(harness_bin, op, inp, target_wall=tw)
        if iters == 0:
            logger.warning("[baseline] %s SKIP — %s", op,
                           _probe_op_error(harness_bin, op, inp))
            continue
        m = measure_harness(harness_bin, op, inp, iters, label=op)
        # No `crate_share` here on purpose. Deciding how much of an op is the
        # crate under test needs inline expansion (`--call-graph dwarf` +
        # `perf script --inline`); LTO absorbs library functions into the
        # harness's op function and leaves them no symbol, so a symbol-level
        # profile reports 0% library for code that is half library. `locate`
        # already measures it the right way and writes it to `hotspots.json`,
        # which is what the gates read — see `gates._measured_library_share`.
        measurements[op] = {
            "input": str(inp), "iters": iters,
            "wall_ms": m.task_clock_ms, "instructions": m.instructions,
            "cv_pct": m.cv_pct,
        }
        logger.info("[baseline] %-22s %8.1f ms  insns=%s  cv=%.3f%%  (iters=%d)",
                    op, m.task_clock_ms, f"{m.instructions:,}", m.cv_pct, iters)
    baseline = {
        "schema_version": 2,
        "identity": {
            "source_head": source_head,
            "harness_sha256": hashlib.sha256(harness_bin.read_bytes()).hexdigest(),
            "ops_sha256": hashlib.sha256(
                json.dumps(perf_ops, separators=(",", ":")).encode()
            ).hexdigest(),
            "environment": {
                "system": platform.system(),
                "release": platform.release(),
                "machine": platform.machine(),
            },
        },
        "measurements": measurements,
    }
    (opt_dir / "baseline.json").write_text(json.dumps(baseline, indent=2))
    return baseline


# ── orchestration ────────────────────────────────────────────────────────────
def _warn_if_workload_unrefined(harness_src: Path, perf_inputs_dir: Path,
                                proj: str) -> None:
    ""                                               

                                                                            
                                                         
                                                            
                                                                  
                                  
                                                               
                                           
       
    manifest = harness_src / "workload_manifest.json"
    ref_mtime = 0.0
    if perf_inputs_dir.is_dir():
        for b in perf_inputs_dir.glob("*.perf.bin"):
            ref_mtime = max(ref_mtime, b.stat().st_mtime)
    if not manifest.exists():
        logger.warning(
            "[perf_opt] %s: NO workload_manifest.json — 当前 workload 可能"
            "跳过了 perf_refine(热度精修)。冷门 kernel 可能未进 workload。"
            "建议用 `python -m harness_gen.build_workload --harness %s` 重建。",
            proj, harness_src)
        return
    try:
        m = json.loads(manifest.read_text(encoding="utf-8"))
        built_at = float(m.get("built_at", 0.0))
    except Exception:
        built_at = manifest.stat().st_mtime
    if ref_mtime > 0 and built_at + 5 < ref_mtime:
        logger.warning(
            "[perf_opt] %s: workload_manifest.json 比 perf_inputs 旧"
            "(manifest=%s < perf.bin)——perf_inputs 疑似 refine 之后又被单独"
            "重产,当前 workload 未必经 refine。建议 build_workload 重建。",
            proj, m.get("built_at_iso", "?"))
    else:
        logger.info("[perf_opt] %s: workload refine OK (verdict=%s)",
                    proj, m.get("steps", {}).get("perf_refine", {}).get("verdict", "?"))


# ── targeted runs (--only-fns) ──────────────────────────────────────────────
#
# A full lz4 run is ~7 hours, most of it W2 measurement across ~20 hot
# functions. When a change touches only some of them — a detector fix that
# restores one rule, a gate fix that matters to one candidate — re-running all
# twenty to see those few is the cost that made iteration slow. A targeted run
# executes the whole detection stage exactly as a full run does (so its
# fn_hits.json is directly comparable) and then hands the agent only the named
# functions.
#
# It never writes the full arm's directory. `_materialize` rmtree's its
# target, so a targeted run pointed at 3_perf_opt would delete the full arm's
# results outright — and a partial result left there could later be read as a
# complete one. It gets its own tree, and a record saying what it was.

TARGETED_DIRNAME = "3_perf_opt_targeted"


def _opt_dir_for(out_root: Path, arm: str,
                 only_fns: "list[str] | None") -> Path:
    if only_fns:
        return out_root / TARGETED_DIRNAME
    return out_root / ("3_perf_opt" if arm == "full" else f"3_perf_opt_{arm}")


def _load_reusable_hotspots(src: Path, perf_ops: list[str],
                            perf_inputs_dir: Path) -> dict:
    """Read a previous run's hotspots.json, or refuse with the reason.

    Checked before use, in this order: the file exists and has hot functions;
    it was profiled over the same ops; it is newer than every perf input. The
    last one is the check that matters: lz4's perf inputs were replaced on
    2026-09-12 (random bytes → text-like corpus) and the hot function set
    moved with them — a hot list from before that date names functions the
    current workload does not heat.
    """
    if not src.is_file():
        raise RuntimeError(
            f"[perf_opt] --reuse-from: {src} 不存在 —— 没有可复用的热点表。"
            f"去掉 --reuse-from 让本轮自己 locate,或指向一次完整跑的输出目录。")
    data = json.loads(src.read_text(encoding="utf-8"))
    if not data.get("hot_functions"):
        raise RuntimeError(f"[perf_opt] --reuse-from: {src} 里没有 hot_functions")
    theirs, ours = set(data.get("ops") or ()), set(perf_ops)
    if theirs != ours:
        raise RuntimeError(
            f"[perf_opt] --reuse-from: {src} 的 op 集合与本轮不同 "
            f"(只在它那边: {sorted(theirs - ours)[:5]},只在本轮: "
            f"{sorted(ours - theirs)[:5]}) —— 热点是在另一套 workload 上测的")
    newest_input = max(
        (b.stat().st_mtime for b in perf_inputs_dir.glob("*.perf.bin")),
        default=0.0)
    if src.stat().st_mtime < newest_input:
        raise RuntimeError(
            f"[perf_opt] --reuse-from: {src} 比当前 perf 输入旧 —— 输入在它之后"
            f"被重新生成过,热函数集合已不对应当前 workload")
    return data


def _select_only_fns(hotspots: list, only_fns: list[str], fn_index) -> list:
    """The requested functions, in hot-list order. An unknown name is an error.

    Matched by name or by any alias `fn_index` knows (link vs source names
    differ under `#[export_name]`). Silently skipping a name the caller typed
    would report a run over fewer functions than were asked for.
    """
    want = list(dict.fromkeys(only_fns))
    picked: list = []
    found: set[str] = set()
    for hf in hotspots:
        names = {hf.name}
        if fn_index is not None:
            names |= set(fn_index.aliases_of(hf.name))
        hit = [w for w in want if w in names]
        if hit:
            picked.append(hf)
            found.update(hit)
    missing = [w for w in want if w not in found]
    if missing:
        raise RuntimeError(
            f"[perf_opt] --only-fns: {missing} 不在本轮的热函数集合里"
            f"(含 C12 准入的)。可选: {[hf.name for hf in hotspots]}")
    return picked


def _skipped_baseline(opt_dir: Path, source_head: str) -> dict:
    """What a targeted run writes instead of measuring the baseline.

    Nothing reads baseline.json — every W2 verdict re-measures pristine
    against the candidate in pairs — so skipping it changes no decision and
    saves ~20 minutes on lz4. It is not copied from the previous run either:
    that file's identity cannot be checked here. The harness hash changes with
    the build directory, and ops_sha256 covers op names, not input contents,
    so a baseline taken on the pre-2026-09-12 inputs would still match. An
    explicit "skipped" is better than numbers of unknown provenance.
    """
    baseline = {
        "schema_version": 2,
        "skipped": True,
        "reason": "targeted run (--only-fns): no consumer reads baseline.json;"
                  " W2 re-measures pristine against each candidate in pairs",
        "identity": {"source_head": source_head},
        "measurements": {},
    }
    (opt_dir / "baseline.json").write_text(json.dumps(baseline, indent=2))
    return baseline


def _write_targeted_record(opt_dir: Path, only_fns: list[str],
                           selected: list, reuse_from) -> None:
    import datetime as _dt
    (opt_dir / "targeted.json").write_text(json.dumps({
        "kind": "targeted run — NOT a full perf_opt result",
        "only_fns": list(only_fns),
        "selected_in_order": [hf.name for hf in selected],
        "hotspots_reused_from": str(reuse_from) if reuse_from else None,
        "baseline": "skipped",
        "started_at": _dt.datetime.now().isoformat(timespec="seconds"),
    }, indent=2, ensure_ascii=False), encoding="utf-8")


def run_perf_opt(out_root: Path, *, tau: float = 3.0, top_n: int = 15,
                 target_wall: float | None = None,
                 run_agent: bool = False,
                 arm: str = "full",
                 only_fns: "list[str] | None" = None,
                 reuse_from: "Path | None" = None) -> PerfOptResult:
    """Perf-optimization stage entry (called by main.py `_perf_opt`).

    Runs the initial logic — materialize → build → initial W2 baseline →
    hot_probe. The agent rewrite loop is a later phase; when hot_probe is not
    yet importable this stops after the baseline.
    """
    if only_fns and arm != "full":
        raise RuntimeError(
            "[perf_opt] --only-fns is a full-arm tool; an ablation arm must "
            "share the full arm's whole target set, not a subset of it")
    out_root = Path(out_root)
    stage_a = out_root / "2_stage_a"
    # The ablation arm writes to its own tree. `_materialize` rmtree's this
    # directory, so sharing it with the full arm would destroy the very
    # results the ablation is being compared against. A targeted run gets its
    # own tree for the same reason (see `_opt_dir_for`).
    opt_dir = _opt_dir_for(out_root, arm, only_fns)
    if not stage_a.is_dir():
        raise RuntimeError(f"2_stage_a not found (run stage_a first): {stage_a}")

    assets = WorkloadAssets.discover(out_root)
    if assets is None:
        raise RuntimeError(
            f"no harness_gen golden harness under {out_root/'workloads'}")

    ops = _read_ops(assets)
    perf_ops = _perf_ops(assets, ops)
    logger.info("[perf_opt] %s: %d ops (%d with perf inputs)",
                out_root.name, len(ops), len(perf_ops))

    # Loud consistency check: if the harness ships perf inputs but NONE match
    # the op list, the op names are wrong (e.g. a foreign shared spec.json) —
    # this silently yields 0 hot fns downstream, so fail fast instead.
    perf_inputs_dir = assets.harness_src / "perf_inputs"
    n_perf_bin = (
        len(list(perf_inputs_dir.glob("*.perf.bin")))
        if perf_inputs_dir.is_dir() else 0
    )

                                                              
                                                                    
                                                            
                                                            
    _warn_if_workload_unrefined(assets.harness_src, perf_inputs_dir, out_root.name)
    if n_perf_bin > 0 and len(perf_ops) == 0:
        raise RuntimeError(
            f"[perf_opt] {out_root.name}: {n_perf_bin} perf_inputs/*.perf.bin "
            f"exist but NONE match the {len(ops)} op names {ops[:8]}… — the op "
            f"list is wrong (likely a foreign shared logs/spec.json). Fix "
            f"_read_ops source or the stray spec.json before re-running; a run "
            f"here would find 0 hot functions."
        )

    # 1-2. materialize + build
    logger.info("[perf_opt] materialize 2_stage_a → %s", opt_dir)
    crate, harness_dir = _materialize(stage_a, opt_dir)
    harness_bin = _build_harness(harness_dir)
    logger.info("[perf_opt] absolute harness built: %s", harness_bin)

    # 3. initial W2 baseline (pre-opt state + anti-drift anchor)
    _measure.check_isolation()
    import os as _os
    if _os.environ.get("PERF_RUN_ENV") != "1":
        logger.warning(
            "[perf_opt] ⚠️ 未在 perf_run.sh 环境下(PERF_RUN_ENV 未设)——"
            "baseline.json 会带 governor/turbo/L3 contention 噪声,\n"
            "  agent W2 gate 将拿它做 anti-drift 锚点。--run-agent 会硬 abort;\n"
            "  只跑 detection(不加 --run-agent)可无视。"
            "  正确起法:bash /home/anonymous/artifact/PerfTrans/perf_run.sh "
            "<venv>/python -m main ...")
    source_head = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=crate, text=True
    ).strip()
    if only_fns:
        logger.info("[perf_opt] targeted run: baseline SKIPPED "
                    "(nothing reads it; see _skipped_baseline)")
        baseline = _skipped_baseline(opt_dir, source_head)
    else:
        logger.info("[perf_opt] initial W2 baseline over %d perf op(s) ...",
                    len(perf_ops))
        baseline = _initial_baseline(harness_bin, assets, perf_ops, opt_dir,
                                     target_wall=target_wall,
                                     source_head=source_head)

    result = PerfOptResult(
        out_root=out_root, opt_dir=opt_dir, crate=crate,
        harness_dir=harness_dir, harness_bin=harness_bin, ops=perf_ops,
        baseline=baseline)

    # 4. hot_probe → evidence packs (5-rule taxonomy)
    if locate_hotspots is None:
        logger.warning("[perf_opt] hot_probe not yet implemented — stopping "
                       "after initial baseline (opt/baseline.json written)")
        return result

    # 4a. locate: pure workload sampling → hot fn list
    #
    # An ablation arm does NOT locate. It reads the full arm's hot function
    # set verbatim, because the two arms are only comparable if they are
    # given the same targets. Sampling does not reproduce a set: profiled
    # twice on one unchanged crate with one unchanged workload, libxml2
    # produced 33 shared functions plus 5 seen only in one run and 6 only in
    # the other, and `perf --call-graph dwarf` inline attribution moved 17
    # points of self time between two of them (`UTF8ToHtml` 25.9%→8.9%,
    # `htmlEncodeEntities` 26.6%→45.5%). A gap of that size between arms
    # would be read as the ablated layer's effect when it is sampling noise.
    shared_hotspots = out_root / "3_perf_opt" / "hotspots.json"
    if reuse_from is not None:
        _src = Path(reuse_from) / "hotspots.json"
        _shared = _load_reusable_hotspots(_src, perf_ops, perf_inputs_dir)
        from perf_opt.hot_probe.types import HotFunction as _HotFunction
        result.hotspots = [_HotFunction.from_dict(x)
                           for x in _shared.get("hot_functions") or []]
        # Written into this run's tree verbatim — sub_tau_profile included,
        # which fixed-cost admission reads back from opt_dir below.
        (opt_dir / "hotspots.json").write_text(
            json.dumps(_shared, indent=2), encoding="utf-8")
        logger.info("[perf_opt] REUSING %d hot function(s) from %s "
                    "(checked: exists / same ops / newer than perf inputs)",
                    len(result.hotspots), _src)
    elif arm != "full":
        if not shared_hotspots.is_file():
            raise RuntimeError(
                f"[perf_opt] arm={arm} needs the full arm's hot function set, "
                f"but {shared_hotspots} does not exist. Run the full arm first "
                f"(--from perf_opt --run-agent); an ablation without the arm it "
                f"is compared against measures nothing."
            )
        from perf_opt.hot_probe.types import HotFunction as _HotFunction
        _shared = json.loads(shared_hotspots.read_text(encoding="utf-8"))
        result.hotspots = [_HotFunction.from_dict(x)
                           for x in _shared.get("hot_functions") or []]
        logger.info(
            "[perf_opt] arm=%s: REUSING the full arm's %d hot function(s) "
            "from %s (not re-locating — the arms must share targets)",
            arm, len(result.hotspots), shared_hotspots)
        (opt_dir / "hotspots.json").write_text(
            json.dumps(_shared, indent=2), encoding="utf-8")
    else:
        logger.info(
            "[perf_opt] hot_probe: locating hotspots (tau=%.1f%%, top_n=%d) ...",
            tau, top_n)
        result.hotspots = locate_hotspots(
            crate=crate, harness_bin=harness_bin, assets=assets,
            ops=perf_ops, tau=tau, top_n=top_n, opt_dir=opt_dir)
        logger.info("[perf_opt] %d hot function(s) located", len(result.hotspots))
    # Built from perf, so these are LINK names. Scanners that attribute
    # through DWARF report SOURCE names instead, and the two differ whenever
    # `#[export_name]` is in play — c2rust emits it for every C identifier
    # that is a Rust keyword. Carry both, or a scanner's hits get pruned as
    # "not hot" against a set that names the same function differently.
    from perf_opt.hot_probe.symbol_source import build_fn_index
    fn_index = build_fn_index(crate)
    hotspot_fn_names = {hf.name for hf in result.hotspots}
    hot_fn_names = set(hotspot_fn_names)
    for hf in result.hotspots:
        hot_fn_names |= fn_index.aliases_of(hf.name)

    # 4b. ONE instrumented build serves both class_II remark scan and
    # class_I IR scan (residual gate + tbaa) — same artifact, two consumers.
    from perf_opt.hot_probe.class_II import (build_and_collect,
                                              scan as scan_class_ii)
    from perf_opt.hot_probe.class_I import scan as scan_class_i
    from perf_opt.hot_probe.class_I.build import _pick_harness_ll

    logger.info("[perf_opt] instrumented build (remark + emit-ir, "
                "timeout=%ds) ...", _INSTRUMENTED_BUILD_TIMEOUT_S)
    rr = build_and_collect(harness_dir, also_emit_ir=True,
                           timeout=_INSTRUMENTED_BUILD_TIMEOUT_S)
    if not rr.ok:
                                                    
                                                        
                                                    
                                          
        logger.error("[perf_opt] instrumented build FAILED: %s\n"
                     "  → Class I + Class II rule scans SKIPPED. Every C1/C2/"
                     "C3-remark signal will be empty for reasons that have "
                     "nothing to do with the code. Detector-coverage numbers "
                     "from this run are NOT valid.\n"
                     "  → raise the budget with "
                     "PERF_OPT_INSTRUMENTED_BUILD_TIMEOUT=<seconds> "
                     "(current: %ds) and re-run.",
                     rr.error[:200], _INSTRUMENTED_BUILD_TIMEOUT_S)
        (opt_dir / "rule_scan_status.json").write_text(json.dumps({
            "class_i": "skipped",
            "class_ii": "skipped",
            "class_iii": "ran",
            "reason": rr.error[:500],
            "instrumented_build_timeout_s": _INSTRUMENTED_BUILD_TIMEOUT_S,
            "warning": "Class I/II hits are empty because the scan did not "
                       "run — NOT because no pattern matched. Do not use "
                       "this run for detector-coverage statistics.",
        }, indent=2), encoding="utf-8")
        opt_remarks = []
        ir_path = None
        class_ii_result = None
        class_i_result = None
    else:
        opt_remarks = rr.remarks
        ir_path = _pick_harness_ll(harness_dir / "target", "harness")
        logger.info("[perf_opt] instrumented build: %d remark(s), ir=%s",
                    len(opt_remarks), ir_path.name if ir_path else "none")

        # 4c. class_II scan (produces c3 remark hits, ii_vec hits, ii_inl hits)
        logger.info("[perf_opt] class_II scan on %d hot fn(s) ...",
                    len(hot_fn_names))
        class_ii_result = scan_class_ii(
            crate=crate, harness_dir=harness_dir,
            opt_remarks=opt_remarks, ir_path=ir_path,
            hot_fns=hot_fn_names, out_dir=opt_dir)

        # 4d. class_I scan (uses class_ii_result for C3 remark channel)
        logger.info("[perf_opt] class_I scan on %d hot fn(s) ...",
                    len(hot_fn_names))
        class_i_result = scan_class_i(
            crate=crate, harness_dir=harness_dir,
            ir_path=ir_path, class_ii_result=class_ii_result,
            hot_fns=hot_fn_names, out_dir=opt_dir)

    # 4d1. Fixed-cost admission — give C12 a predicate it can pass.
    #
    # Everything above filtered on hot(s) = self-time >= tau, which is blind to
    # a per-call fixed cost: an oversized zero-init sits in `*_init` / `*_reset`
    # and never clears tau, yet it is what dominates on small inputs (lz4: raw
    # is +0.04% vs C on a Silesia-sized input and +23.94% on a 48 KB one).
    # Across the 12-project set that blind spot swallowed 287 of 296
    # project-side C12 sites. Admitting them here, tightly capped, is cheaper
    # and far more honest than lowering tau — see hot_probe.fixed_cost_admit.
    if class_i_result is not None:
        from perf_opt.hot_probe.fixed_cost_admit import admit_fixed_cost_fns
        try:
            _sub_tau = json.loads(
                (opt_dir / "hotspots.json").read_text(encoding="utf-8")
            ).get("sub_tau_profile") or {}
        except Exception:
            _sub_tau = {}
        _extra, _extra_hits = admit_fixed_cost_fns(
            result.hotspots, class_i_result=class_i_result,
            sub_tau_profile=_sub_tau, fn_index=fn_index)
        if _extra:
            result.hotspots.extend(_extra)
            for _hf in _extra:
                hotspot_fn_names.add(_hf.name)
                hot_fn_names.add(_hf.name)
                hot_fn_names |= fn_index.aliases_of(_hf.name)
            # The scan pruned its per-fn hits against the pre-admission hot set,
            # so re-attach the evidence that justified admitting these.
            for _fn, _h in _extra_hits.items():
                class_i_result.c1_c2_hits_by_fn.setdefault(_fn, {}).update(_h)
            logger.info("[perf_opt] fixed-cost admission: hot set %d → %d",
                        len(result.hotspots) - len(_extra), len(result.hotspots))

    # 4d2. class_III scan (translation-product layer, independent of I/II).
    # Runs even when the instrumented build failed above — class_III's
    # channel is source CST, no build/IR required.
    from perf_opt.hot_probe.class_III import scan as scan_class_iii
    logger.info("[perf_opt] class_III scan on %d hot fn(s) ...",
                len(hot_fn_names))
    class_iii_result = scan_class_iii(
        crate=crate,
        hot_fns=hot_fn_names,     # auto-detect base-name mode (§7 dual-mode)
        out_dir=opt_dir,
    )

    # 4e. augment hot_fns with 5-rule hits + rewrite hotspots.json
    from perf_opt.hot_probe.types import augment_hot_fns
    from perf_opt.hot_probe.locate import rewrite_hotspots_with_hits
    augment_hot_fns(result.hotspots, class_i_result, class_ii_result)

                                                                         
                                                                        
                                   
    from perf_opt.hot_probe.fn_type import classify_fn_type_from_file
    for hf in result.hotspots:
        if hf.file and hf.line_start and hf.line_end:
            r = classify_fn_type_from_file(crate, hf.file,
                                            hf.line_start, hf.line_end)
            if r:
                hf.fn_type, hf.fn_type_reason = r
    _fn_type_counts: dict[str, int] = {}
    for hf in result.hotspots:
        _fn_type_counts[hf.fn_type or "unclassified"] = \
            _fn_type_counts.get(hf.fn_type or "unclassified", 0) + 1
    logger.info("[perf_opt] fn_type distribution: %s",
                dict(sorted(_fn_type_counts.items(), key=lambda x: -x[1])))

    rewrite_hotspots_with_hits(opt_dir, result.hotspots)

                                                                
    from perf_opt.hot_probe.merged_hits import merge_hits, write_fn_hits
    merged = merge_hits(class_i_result, class_ii_result, class_iii_result,
                        opt_remarks, fn_index, crate_root=crate,
                        hot_fn_names=hot_fn_names,
                        canonical_fn_names=hotspot_fn_names)
    # II_iso: hot loop kernels fully inlined into a much larger LIBRARY
    # function, read from the harness build's debug info. Nominations only —
    # W2 decides each one. A failure here costs the rule, never the run.
    try:
        import tomllib
        from perf_opt.hot_probe.inline_container import find_isolation_candidates
        from perf_opt.hot_probe.merged_hits import FnHits, Hit
        _manifest = tomllib.loads((crate / "Cargo.toml").read_text(encoding="utf-8"))
        _lib_crate = ((_manifest.get("lib") or {}).get("name")
                      or (_manifest.get("package") or {}).get("name") or "").replace("-", "_")
        for _cand in find_isolation_candidates(harness_bin, result.hotspots, _lib_crate):
            _key = next((k for k in merged if k.split("::")[-1] == _cand["fn"]),
                        _cand["fn"])
            _fh = merged.get(_key) or FnHits(
                fn=_key, file=_cand["file"],
                line_range=(_cand["line"], _cand["line"]))
            _fh.hits.append(Hit(
                rule="II_iso", pattern="inlined-into-larger-container",
                file=_cand["file"], line=_cand["line"], col=0,
                snippet=(f"{_cand['fn']} fully inlined into {_cand['container']} "
                         f"({_cand['container_bytes']} B container, "
                         f"{_cand['inlined_bytes']} B inlined)"),
                extra={k: _cand[k] for k in (
                    "container", "container_bytes", "inlined_bytes", "self_pct")},
            ))
            merged[_key] = _fh
    except Exception as exc:  # noqa: BLE001 — detector is best-effort
        logger.warning("[perf_opt] II_iso detection skipped: %s", exc)
    write_fn_hits(merged, opt_dir / "fn_hits.json")
    logger.info("[perf_opt] fn_hits.json written: %d fn(s) with hits",
                len(merged))

    # 4e3. Targeted run: detection above ran over the whole hot set, exactly as
    # a full run would, so fn_hits.json stays comparable. Only the functions
    # asked for go on to characterize and the agent.
    if only_fns:
        _before = len(result.hotspots)
        result.hotspots = _select_only_fns(result.hotspots, only_fns, fn_index)
        _write_targeted_record(opt_dir, only_fns, result.hotspots, reuse_from)
        logger.info("[perf_opt] targeted run: %d of %d hot fn(s) selected: %s",
                    len(result.hotspots), _before,
                    [hf.name for hf in result.hotspots])

    # 4f. characterize → full evidence packs (one per hot fn)
    logger.info("[perf_opt] characterizing %d hot fn(s) ...",
                len(result.hotspots))
    result.evidence = characterize(
        result.hotspots, crate=crate, harness_dir=harness_dir,
        harness_bin=harness_bin, assets=assets, opt_dir=opt_dir,
        class_i_result=class_i_result, class_ii_result=class_ii_result,
        opt_remarks=opt_remarks)
    logger.info("[perf_opt] %d evidence pack(s) written", len(result.evidence))

    # 5. agent rewrite loop (optional — opt-in via --run-agent)
    if run_agent:
        from perf_opt.agent_perf_opt.config import load_agent_config
        cfg = load_agent_config({})
        if arm == "freeform":
            # Ablation: the rule layer is removed. Detection still ran above
            # (hot_probe + the class scans), which costs little next to the
            # W2 measurements and buys the comparison a useful column —
            # which functions the FULL arm would have skipped for want of a
            # rule hit are visible in this run's own fn_hits.json.
            from perf_opt.agent_perf_opt.ablation_freeform import (
                optimize_hot_fns_freeform as _optimize,
            )
            logger.info("[perf_opt] --run-agent [ABLATION arm=freeform]: "
                        "hot fn → LLM → gates, no cards/regions")
        else:
            from perf_opt.agent_perf_opt.agent import (
                optimize_hot_fns as _optimize,
            )
            logger.info("[perf_opt] --run-agent: kicking off optimize_hot_fns")
        result.agent_result = _optimize(
            result.hotspots,
            evidence_dir=opt_dir / "evidence",
            harness_dir=harness_dir, crate=crate, assets=assets, cfg=cfg,
            opt_dir=opt_dir, perf_ops=result.ops,
        )
        _log_agent_done(result.agent_result)
    return result


if __name__ == "__main__":  # dev smoke; main.py --from perf_opt is the real entry
    import argparse
    import sys
    logging.basicConfig(level=logging.INFO,
                        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
                        datefmt="%H:%M:%S")
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-root", required=True,
                    help="dataset_trans/<proj> directory")
    ap.add_argument("--tau", type=float, default=3.0)
    ap.add_argument("--top-n", type=int, default=0)   # 0 = no cap (tau floor only)
    a = ap.parse_args()
    run_perf_opt(Path(a.out_root), tau=a.tau, top_n=a.top_n)
    sys.exit(0)
