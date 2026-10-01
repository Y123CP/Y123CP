""                                                                

                                                                      
                                                                       
                                                           

                                                                         
                                                                        

                                                                     
                                                                    
                                                                  
                                                                         
                                                                      
                                                                 
                                                                      
                                                               
                                                                 

                                                                        
                                                                   
                                                                      

                                                                    
                                                                        
                                                        

    
                                   
                                               
                                                                
                                                  
   

from __future__ import annotations

import argparse
import json
import logging
from pathlib import Path

from profiling.asm_tells import (
    classify_retiring_mix, derive_hot_subregions, fn_location, region_src_span,
    scan_asm_tells,
)
from profiling.evidence import (
    Anchor, CharacterizationReport, CpiAxis, Evidence, HotRegion, HotspotProfile,
    IcAxis, LoopInvariantCandidate, OptRemarkTell, Routing, RetiringMix,
    SourcePattern, derive_regime, utc_now_iso,
)
from profiling.opt_remarks_collector import collect_opt_remarks
from profiling.perf_annotate import find_hot_region, run_perf_annotate
from profiling.perf_runner import run_perf
from profiling.perf_stat import collect_cpi_counters, derive_cpi_metrics
from profiling.tma import run_tma
from profiling.workload import _resolve_binary, load_manifest, run_workload

# Cap on opt-remarks kept per hotspot — remarks are noisy even after the
                                                 
_MAX_REMARKS_PER_FN = 30
# A loop's vectorize remark is attributed to its *header* line, which sits
# a few lines above the sampled loop *body* — so the hot-region span is
# widened by this margin when attributing remarks (verified on libcsv:
# csv_parse's remark is at libcsv.rs:343, its hot region starts at 344).
_REMARK_LINE_MARGIN = 16

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Orchestrator
# ---------------------------------------------------------------------------

def characterize_project(project_path: Path, manifest_path: Path, *,
                         top_n_hot: int = 10,
                         skip_threshold_pct: float = 3.0,
                         hot_region_coverage: float = 0.80,
                         frequency: int = 999,
                         perf_data_dir: Path | None = None,
                         skip_build: bool = False,
                         skip_tma: bool = False,
                         skip_cpi: bool = False,
                         skip_ic: bool = False,
                         skip_remarks: bool = False,
                         skip_loop_invariant: bool = False) -> CharacterizationReport:
    """Run §2.5.6's 7-step extraction; return a CharacterizationReport.

    Args:
      project_path        : the staged Rust cargo project (its `src/` is the
                            source of truth for fn spans)
      manifest_path       : workload manifest TOML (see workloads/*.toml)
      top_n_hot           : keep at most this many hot fns
      skip_threshold_pct  : self_time_pct gate — fns below are skipped (§2.5.1)
      hot_region_coverage : sample-coverage target for hot-region clustering
      frequency           : perf record sampling frequency (Hz)
      skip_*              : short-circuit a phase (graceful degradation)
    """
    project_path = Path(project_path).resolve()
    workload = load_manifest(manifest_path)
    # For a library benchmark the workload-driving binary lives in a
    # separate harness crate; build whichever crate owns the binary.
    build_dir = workload.harness_dir if workload.harness_dir else project_path
    notes: list[str] = []

    # --- Step 0: one release build with debuginfo (+ opt-remarks) --------
    remarks: list[dict] = []
    if skip_build:
        notes.append("build: skipped by flag — binary may lack debuginfo / be stale")
    else:
        # LLVM emits opt-remarks only for crates it actually (re)compiles —
        # a cached staged crate stays silent. Touch its sources so the
        # remark build always recompiles it (design §2.5.6 step 7).
        touched = _touch_sources(project_path)
        logger.info(f"[characterize] step 0: build {build_dir.name} "
                    f"(--release, RUSTFLAGS+=-Cdebuginfo=1 + LLVM remark flags; "
                    f"touched {touched} src file(s))")
        orr = collect_opt_remarks(build_dir, extra_rustflags="-Cdebuginfo=1")
        remarks = orr.remarks
        if orr.ok:
            logger.info(f"[characterize] build ok; {len(remarks)} opt-remark(s)")
        else:
            notes.append(f"build (debuginfo+remarks): {orr.error} "
                         f"— continuing on existing binary")
        if not remarks:
            notes.append("opt_remarks: build emitted no LLVM remarks "
                         "(toolchain may not support -pass-remarks)")

    # --- resolve the binary (must exist after the build) -----------------
    try:
        binary = _resolve_binary(workload.binary, project_path,
                                 harness_dir=workload.harness_dir)
    except FileNotFoundError as e:
        notes.append(f"FATAL: {e}")
        return CharacterizationReport(
            project_path=str(project_path), workload_name=workload.name,
            workload_args=list(workload.args), binary=str(workload.binary),
            scanned_at=utc_now_iso(), stability=_empty_stability(),
            hotspots=[], skipped=[], notes=notes)

    # --- Step 1a: wall-clock stability -----------------------------------
    stability, _ = run_workload(workload, project_path)
    if not stability.stable:
        notes.append(f"unstable measurement: cv={stability.cv:.4f} "
                     f"mean={stability.mean_ms:.2f}ms — TMA/counter noise likely")

    # --- Step 1b: hot fns via perf record (cycles) -----------------------
    cyc = run_perf(binary=binary, args=workload.args, cwd=project_path,
                   output_dir=perf_data_dir, frequency=frequency,
                   call_graph_mode="fp", timeout=workload.timeout_s)
    hot_raw: list[dict] = []
    if cyc.ok:
        hot_raw = cyc.hot_fns
    else:
        notes.append(f"perf record (cycles): {cyc.error}")
        logger.warning(f"[characterize] perf record failed: {cyc.error}")

    kept = [h for h in hot_raw if h["cpu_pct"] >= skip_threshold_pct][:top_n_hot]
    skipped = [{"name": h["name"], "self_time_pct": h["cpu_pct"]}
               for h in hot_raw if h["cpu_pct"] < skip_threshold_pct][:20]
    logger.info(f"[characterize] {len(kept)} hot fn(s) ≥ {skip_threshold_pct}%, "
                f"{len(hot_raw) - len(kept)} below threshold")

    # --- Step 3: TMA L1 routing (process-level, shared) ------------------
    tma = None
    if skip_tma:
        notes.append("tma: skipped by flag")
    else:
        tma = run_tma(binary, workload.args, cwd=project_path,
                      timeout_s=workload.timeout_s)
        if tma.ok:
            logger.info(f"[characterize] TMA via {tma.source}: "
                        f"retiring={tma.tma_retiring}")
        else:
            notes.append(f"tma: {tma.error}")

    # --- Step 4: CPI-axis counters (process-level, shared) ---------------
    cpi_metrics: dict = {}
    cpi_counters: dict = {}
    if skip_cpi:
        notes.append("cpi counters: skipped by flag")
    else:
        cpi_counters, cpi_notes = collect_cpi_counters(
            binary, workload.args, cwd=project_path, timeout_s=workload.timeout_s)
        notes.extend(cpi_notes)
        cpi_metrics = derive_cpi_metrics(cpi_counters)

    # --- Step 5: instruction-sample perf.data (per-fn IC source) ---------
    instr = None
    instr_self: dict[str, float] = {}
    if skip_ic:
        notes.append("ic axis (instructions): skipped by flag")
    else:
        instr = run_perf(binary=binary, args=workload.args, cwd=project_path,
                         output_dir=perf_data_dir, frequency=frequency,
                         call_graph_mode="fp", timeout=workload.timeout_s,
                         event="instructions:u")
        if instr.ok:
            instr_self = {h["name"]: h["cpu_pct"] for h in instr.hot_fns}
        else:
            notes.append(f"perf record (instructions:u): {instr.error}")

    # --- Step 8: loop-invariant cond candidates ---------------------------
    # Step A change: LICM now folds into each hot fn's
    # `evidence.source_patterns` (kind="loop_invariant_branch") instead of
    # riding as a top-level sidecar on CharacterizationReport. The
    # project-wide scan still runs project-wide (syn_walker — §2.5 forbids
                                                                          
    # index by `containing_fn` and let _characterize_one attach the slice.
    licm_by_fn: dict[str, list[LoopInvariantCandidate]] = {}
    if skip_loop_invariant:
        notes.append("loop_invariant: skipped by flag")
    else:
        try:
            from proposer.syn_walker import list_loop_invariant_conds
            li_out = list_loop_invariant_conds(project_path)
            total = 0
            for c in li_out.items:
                cand = LoopInvariantCandidate(
                    file          = c.file,
                    line          = c.line,
                    loop_kind     = c.loop_kind,
                    loop_line     = c.loop_line,
                    containing_fn = c.containing_fn,
                    cond_snippet  = c.cond_snippet,
                    reads         = list(c.reads),
                )
                licm_by_fn.setdefault(cand.containing_fn, []).append(cand)
                total += 1
            logger.info(f"[characterize] loop_invariant: "
                        f"{total} candidate(s) across {len(licm_by_fn)} fn(s)")
        except Exception as e:
            notes.append(f"loop_invariant: {e}")
            logger.warning(f"[characterize] loop_invariant failed: {e}")

    # --- Step 9: source-level shape detectors (Step C) -------------------
    # byte_{copy,compare,fill,search}_loop / branch_cascade / Option-callback
    # signature facts. Indexed by bare fn name like LICM; _characterize_one
    # slices per-hotspot using the same (bare ∪ inlined_fns) intersection.
    sp_scan = None
    try:
        from profiling.source_patterns import scan_source_patterns
        sp_scan = scan_source_patterns(project_path)
    except Exception as e:
        notes.append(f"source_patterns: {e}")
        logger.warning(f"[characterize] source_patterns failed: {e}")

    # --- per hot fn: assemble the 5-zone HotspotProfile ------------------
    hotspots: list[HotspotProfile] = []
    for h in kept:
        hotspots.append(_characterize_one(
            h, workload=workload, binary=binary,
            cyc=cyc, instr=instr, instr_self=instr_self,
            tma=tma, cpi_metrics=cpi_metrics, cpi_counters=cpi_counters,
            remarks=remarks, hot_region_coverage=hot_region_coverage,
            licm_by_fn=licm_by_fn,
            sp_scan=sp_scan))

    return CharacterizationReport(
        project_path              = str(project_path),
        workload_name             = workload.name,
        workload_args             = list(workload.args),
        binary                    = str(binary),
        scanned_at                = utc_now_iso(),
        stability                 = stability,
        hotspots                  = hotspots,
        skipped                   = skipped,
        notes                     = notes,
    )


# ---------------------------------------------------------------------------
# Step E — attribute each evidence tell to its enclosing hot subregion
# ---------------------------------------------------------------------------

def _parse_addr_range(s: str) -> tuple[int, int] | None:
    """`0x1234` or `0x1234-0x5678` → (lo, hi). None on parse failure."""
    if not s:
        return None
    parts = s.split("-", 1)
    try:
        lo = int(parts[0], 16)
        hi = int(parts[1], 16) if len(parts) > 1 else lo
        return (lo, hi)
    except (ValueError, IndexError):
        return None


def _parse_src_loc(loc: str) -> tuple[str, int] | None:
    """`<anything>/file.rs:LINE` → (basename, LINE). None on parse failure."""
    if not loc or ":" not in loc:
        return None
    file_part, _, line_part = loc.rpartition(":")
    try:
        line = int(line_part)
    except ValueError:
        return None
    basename = file_part.rsplit("/", 1)[-1]
    return (basename, line)


def _parse_src_range(s: str) -> tuple[str, int, int] | None:
    """`file.rs:LO-HI` or `file.rs:LINE` → (file, lo, hi). None on parse failure."""
    if not s or ":" not in s:
        return None
    file_part, _, rng = s.partition(":")
    if "-" in rng:
        lo_s, _, hi_s = rng.partition("-")
    else:
        lo_s = hi_s = rng
    try:
        return (file_part, int(lo_s), int(hi_s))
    except ValueError:
        return None


def _attribute_by_addr(tell_lo: int, tell_hi: int, subregions) -> int:
    """Find subregion whose asm_range overlaps [tell_lo, tell_hi] most.
    Ties broken by highest `self_share`. -1 = no overlap.

    For diffuse tells (range covers most of the parent), prefer the
    subregion with greatest absolute overlap — this naturally picks the
    most-dominant subregion when a tell straddles multiple."""
    best_idx = -1
    best_overlap = 0
    best_share = -1.0
    for i, s in enumerate(subregions):
        rng = _parse_addr_range(s.asm_range)
        if rng is None:
            continue
        sub_lo, sub_hi = rng
        ov_lo = max(tell_lo, sub_lo)
        ov_hi = min(tell_hi, sub_hi)
        if ov_hi < ov_lo:
            continue
        overlap = ov_hi - ov_lo + 1
        if (overlap > best_overlap
                or (overlap == best_overlap and s.self_share > best_share)):
            best_overlap = overlap
            best_share = s.self_share
            best_idx = i
    return best_idx


def _attribute_by_src(file: str, line: int, subregions) -> int:
    """Find subregion whose (file, src_line range) contains (file, line).

    Prefer the MOST SPECIFIC match (smallest line range) — when nested
    inlined fns share lines, the innermost subregion has the tighter
    range. Tie-break by self_share. -1 = no match."""
    best_idx = -1
    best_size = float("inf")
    best_share = -1.0
    for i, s in enumerate(subregions):
        rng = _parse_src_range(s.src_lines)
        if rng is None:
            continue
        sub_file, sub_lo, sub_hi = rng
        if sub_file != file:
            continue
        if not (sub_lo <= line <= sub_hi):
            continue
        size = sub_hi - sub_lo + 1
        if (size < best_size
                or (size == best_size and s.self_share > best_share)):
            best_size = size
            best_share = s.self_share
            best_idx = i
    return best_idx


def _build_subregion_addr_lookup(subregions):
    """Build a callable `addr → subregion_idx` for per-instruction
    attribution inside `scan_asm_tells`. Uses linear scan over the
    subregion list (typically ≤ 10 subregions per hot fn). Ties resolved
    by highest self_share (matches `_attribute_by_addr` semantics).
    Returns None when there are no subregions (caller uses default lookup-less
    path, all tells get subregion_idx = -1)."""
    if not subregions:
        return None
    ranges = []
    for i, s in enumerate(subregions):
        rng = _parse_addr_range(s.asm_range)
        if rng is not None:
            ranges.append((rng[0], rng[1], s.self_share, i))
    if not ranges:
        return None
    def _lookup(addr: int) -> int:
        best_idx = -1
        best_share = -1.0
        for lo, hi, share, idx in ranges:
            if lo <= addr <= hi and share > best_share:
                best_share = share
                best_idx = idx
        return best_idx
    return _lookup


def _attribute_src_evidence_to_subregions(evidence, subregions) -> None:
    """Set `subregion_idx` on every opt_remark + source_pattern based on
    its `loc=file:line`. asm_tells are attributed inside `scan_asm_tells`
    via the addr lookup (per-occurrence, not bounding-box). No-op when
    there are no subregions.

    Phase F: `evidence.source_patterns` is a kind-keyed dict; iterate all
    lists across kinds. fn_signature_patterns get attribution too."""
    if not subregions:
        return
    for r in evidence.opt_remarks:
        sl = _parse_src_loc(r.loc)
        if sl is not None:
            r.subregion_idx = _attribute_by_src(sl[0], sl[1], subregions)
    for kind_list in evidence.source_patterns.values():
        for sp in kind_list:
            sl = _parse_src_loc(sp.loc)
            if sl is not None:
                sp.subregion_idx = _attribute_by_src(sl[0], sl[1], subregions)


def _characterize_one(h: dict, *, workload, binary: Path,
                      cyc, instr, instr_self: dict, tma, cpi_metrics: dict,
                      cpi_counters: dict, remarks: list[dict],
                      hot_region_coverage: float,
                      licm_by_fn: dict[str, list[LoopInvariantCandidate]] | None = None,
                      sp_scan = None
                      ) -> HotspotProfile:
    """Build one HotspotProfile for hot fn `h` ({name, cpu_pct})."""
    name = h["name"]
    self_pct = float(h["cpu_pct"])
    fn_notes: list[str] = []

    # ----- Step 2: hot region from cycles annotate -----------------------
    region = None
    region_insns = []
    fn_insns_full: list = []     # whole fn (incl. cold paths) — for COLD_BLOCK
    fn_entry: int | None = None
    if cyc.ok and cyc.raw_data:
        ann = run_perf_annotate(Path(cyc.raw_data), name, percent_type="local")
        if ann.ok:
            fn_entry = ann.insns[0].addr            # insns are addr-sorted
            fn_insns_full = ann.insns
            region = find_hot_region(ann.insns, target_coverage=hot_region_coverage)
            if region is None:
                fn_notes.append("hot_region: fn carried no cycle samples")
            else:
                region_insns = region.insns
        else:
            fn_notes.append(f"hot_region: perf annotate (cycles): {ann.error}")
    else:
        fn_notes.append("hot_region: no cycles perf.data")

    # ----- anchor.function: symbol @ file:line (addr2line / DWARF) -------
    loc: tuple[str, int] | None = None
    if fn_entry is not None:
        loc = fn_location(binary, fn_entry)
        if loc is not None:
            func_str = f"{name} @ {loc[0]}:{loc[1]}"
        else:
            func_str = f"{name} @ <location unresolved>"
            fn_notes.append("function: addr2line could not resolve the entry "
                            "(binary built without -Cdebuginfo?)")
    else:
        func_str = f"{name} @ <location unresolved>"

    # ----- Step 6 (src lines): hot region address → source span ----------
    hot_region = HotRegion()
    region_share = 0.0
    region_span = None
    if region is not None:
        hot_region.asm_range = region.asm_range
        region_share = region.share * 100.0          # → percent
        region_src = region_src_span(binary, [i.addr for i in region.insns])
        region_span = region_src.span
        hot_region.inlined_fns = list(region_src.inlined_fns)
        if region_span is not None:
            hot_region.src_lines = region_span.as_lines()
        else:
            fn_notes.append("hot_region.src_lines: addr2line mapping "
                            "unavailable (no debuginfo)")
        # honesty notes — a diffuse region has no single tight loop; an
        # inlined region's src lines are a callee's, not the fn's own.
        asm_size = region.hi_addr - region.lo_addr
        if asm_size > 4096:
            fn_notes.append(f"hot_region: diffuse — asm range spans "
                            f"~{asm_size // 1024} KB, no single tight loop")
        if region_src.inlined:
            fn_notes.append("hot_region: contains inlined callee code "
                            "— src_lines are the inlined callee's")
        # Step D — sub-cluster the hot region by innermost inline frame so
        # dispatch can pick a tight site even when the parent region spans
        # 700+ lines of mixed inlined callee code.
        hot_fn_bare = name.rsplit("::", 1)[-1] if "::" in name else name
        hot_region.hot_subregions = derive_hot_subregions(
            binary, region.insns, hot_fn_bare,
        )
        if not hot_region.hot_subregions and asm_size > 4096:
            fn_notes.append("hot_region.hot_subregions: empty "
                            "(addr2line unresolved or all clusters below "
                            "5% noise floor)")

    # Phase F: workload becomes {id, cmd} object — `cmd` is the full
    # reproducible invocation so consumers can copy-paste a re-run.
    from profiling.evidence import Workload as _WorkloadAnchor
    workload_cmd = " ".join([str(binary), *workload.args])
    anchor = Anchor(
        workload         = _WorkloadAnchor(id=workload.name, cmd=workload_cmd),
        function         = func_str,
        self_time_pct    = self_pct,
        hot_region       = hot_region,
        hot_region_share = round(region_share, 2),
        # call_count stays None — needs harness-side perf-probe / eBPF to fill.
    )

    # ----- Step 3: routing (process-level TMA, shared) -------------------
    routing = Routing()
    if tma is not None and tma.ok:
        routing.tma_retiring        = tma.tma_retiring
        routing.tma_frontend_bound  = tma.tma_frontend_bound
        routing.tma_bad_speculation = tma.tma_bad_speculation
        routing.tma_backend_bound   = tma.tma_backend_bound
        routing.tma_source          = tma.source
        routing.regime              = derive_regime(tma.tma_retiring)
        fn_notes.append("routing: process-level TMA — function-granularity "
                        "approximation (§2.5.6 note ②)")
    else:
        routing.tma_source = "unavailable"
        routing.regime     = derive_regime(None)      # → unknown
        fn_notes.append("routing: TMA unavailable — regime=unknown (degraded)")

    # ----- Step 4: cpi_axis (process-level counters, shared) -------------
    cpi_axis = CpiAxis(
        cpi                    = cpi_metrics.get("cpi"),
        l1d_miss_rate          = cpi_metrics.get("l1d_miss_rate"),
        l2_miss_rate           = cpi_metrics.get("l2_miss_rate"),
        llc_miss_rate          = cpi_metrics.get("llc_miss_rate"),
        dtlb_miss_rate         = cpi_metrics.get("dtlb_miss_rate"),
        branch_mispredict_rate = cpi_metrics.get("branch_mispredict_rate"),
        tma_path               = (tma.tma_path if (tma is not None and tma.ok) else ""),
    )

    # ----- Step 5: ic_axis (Phase G: 3 spec-named fields) ---------------
    # insns_total_workload  : entire workload's retired insns (perf stat)
    # hot_fn_insn_share     : this hot fn's % share of instruction samples (0-100)
    # hot_fn_insns_est      : estimated retired insns inside this hot fn
    #                         ≈ insns_total_workload * hot_fn_insn_share/100 * local_frac
    #                         (was `insns_total` pre-Phase G)
    ic_axis = IcAxis(ipc=cpi_metrics.get("ipc"))
    ic_axis.retiring_mix = (classify_retiring_mix(region_insns)
                            if region_insns else RetiringMix())
    total_instr = cpi_counters.get("instructions")
    if total_instr:
        ic_axis.insns_total_workload = int(total_instr)
    if instr is not None and instr.ok:
        fn_instr_pct = instr_self.get(name)
        if fn_instr_pct is not None:
            ic_axis.hot_fn_insn_share = float(fn_instr_pct)   # already 0-100
    # `hot_fn_insns_est` is fn-level per spec:
    #     hot_fn_insns_est = insns_total_workload × hot_fn_insn_share / 100
    # NOT region-scoped. The region-scoping (via `local_frac`) is only used
    # to derive per-iter insns for `insns_per_iter`:
    #     hot_loop_insns_est = hot_fn_insns_est × local_frac
    #     insns_per_iter     = hot_loop_insns_est / workload.iterations
    if total_instr and ic_axis.hot_fn_insn_share > 0:
        ic_axis.hot_fn_insns_est = int(
            total_instr * (ic_axis.hot_fn_insn_share / 100.0))

    hot_loop_insns_est = 0
    if (instr is not None and instr.ok and instr.raw_data
            and total_instr and region is not None):
        if ic_axis.hot_fn_insn_share <= 0:
            fn_notes.append("ic_axis.hot_fn_insns_est: fn absent from "
                            "instruction-sample report")
        else:
            ann_i = run_perf_annotate(Path(instr.raw_data), name,
                                      percent_type="local")
            if ann_i.ok:
                lo, hi = region.lo_addr, region.hi_addr
                reg_pct = sum(i.pct for i in ann_i.insns if lo <= i.addr <= hi)
                fn_pct = sum(i.pct for i in ann_i.insns)
                local_frac = (reg_pct / fn_pct) if fn_pct > 0 else 0.0
                hot_loop_insns_est = int(ic_axis.hot_fn_insns_est * local_frac)
            else:
                fn_notes.append(f"ic_axis.insns_per_iter: perf annotate "
                                f"(instructions): {ann_i.error}")
    elif region is None:
        fn_notes.append("ic_axis.insns_per_iter: no hot region to scope")
    elif not (instr is not None and instr.ok):
        fn_notes.append("ic_axis.insns_per_iter: instruction-sample data unavailable")

    if workload.iterations and hot_loop_insns_est:
        ic_axis.insns_per_iter = hot_loop_insns_est / workload.iterations
    elif workload.iterations is None:
        fn_notes.append("ic_axis.insns_per_iter: workload iteration count "
                        "unknown ([profile].iterations unset)")

    # ----- Steps 6/7: evidence pool --------------------------------------
    evidence = Evidence()
    # Step E: pass per-subregion addr lookup so scan_asm_tells emits one
    # AsmTell per (pattern, subregion) instead of one bounding-box tell per
    # pattern. Each tell's `count` then reflects per-subregion occurrences.
    _sub_lookup = _build_subregion_addr_lookup(hot_region.hot_subregions)
    evidence.asm_tells = (scan_asm_tells(region_insns, subregion_lookup=_sub_lookup)
                          if region_insns else [])
    # COLD_BLOCK detection (gates A2 cold-path-outline). Needs the WHOLE
    # fn's annotated insns + the hot region — both already collected above.
    # Cold blocks sit by definition OUTSIDE the hot region, so they carry
    # subregion_idx = -1 (not attributable to any hot_subregion).
    if region_insns and fn_insns_full:
        from profiling.asm_tells import detect_cold_blocks
        evidence.asm_tells.extend(detect_cold_blocks(fn_insns_full, region_insns))
    # opt-remarks filtered to the hot region's source span (§2.5.5).
    if region_span is not None and remarks:
        seen: set[tuple] = set()
        for r in remarks:
            if Path(r["file"]).name != region_span.file:
                continue
            if not (region_span.line_lo - _REMARK_LINE_MARGIN <= r["line"]
                    <= region_span.line_hi + _REMARK_LINE_MARGIN):
                continue
            key = (r["file"], r["line"], r["pass"], r["status"])
            if key in seen:
                continue
            seen.add(key)
            evidence.opt_remarks.append(OptRemarkTell(
                opt_pass = r["pass"],
                status   = r["status"],
                reason   = r["message"],
                loc      = f"{r['file']}:{r['line']}",
            ))
            if len(evidence.opt_remarks) >= _MAX_REMARKS_PER_FN:
                fn_notes.append(f"opt_remarks: capped at {_MAX_REMARKS_PER_FN} "
                                f"(hot region had more)")
                break

    # ----- source_patterns + fn_signature_patterns: Step A + Step C + Phase F
    # Project-wide indexes (LICM via syn_walker, byte-loops/cascades/
    # callbacks via profiling.source_patterns) → per-hotspot slice:
    # (bare fn name) ∪ (LTO-inlined callees in hot region). So
    # handle_compress's hot region picks up patterns from BZ2_blockSort /
    # mainSort etc. that were inlined into it.
    bare_fn = name.rsplit("::", 1)[-1] if "::" in name else name
    fn_set: set[str] = {bare_fn, *(hot_region.inlined_fns or [])}

    # (a) LICM (Step A) — into source_patterns["loop_invariant_branch"]
    # `details["origin_fn"]` records WHICH fn the pattern was detected in
    # (anchor or an LTO-inlined callee). Body-scanning rules (C4/C5-libc/
    # C1/D1) use it to skip patterns whose source line lives outside the
    # current anchor fn body — those can't be rewritten in-place via the
    # anchor's snippet. Without this tag, brotli CBRNH68's 6 LICM hits
    # mix 3 in-fn lines with 3 cross-fn (other CBR* monomorphizations
    # + FindLongestMatchH68); guards fail noisily on the cross-fn ones.
    if licm_by_fn:
        seen: set[tuple] = set()
        for fn in fn_set:
            for cand in licm_by_fn.get(fn, []):
                key = (cand.file, cand.line, cand.cond_snippet)
                if key in seen:
                    continue
                seen.add(key)
                sp = cand.to_source_pattern()
                sp.details = {**sp.details, "origin_fn": fn}
                evidence.source_patterns.setdefault(sp.kind, []).append(sp)

    # (b) Step C: byte_*_loop, branch_cascade — bucket by kind
    if sp_scan is not None:
        seen_sp: set[tuple] = set()
        for fn in fn_set:
            for sp in sp_scan.patterns_by_fn.get(fn, []):
                key = (sp.kind, sp.loc)
                if key in seen_sp:
                    continue
                seen_sp.add(key)
                sp.details = {**sp.details, "origin_fn": fn}
                evidence.source_patterns.setdefault(sp.kind, []).append(sp)

        # (b.2) Phase H.2: layout_candidate — per-file struct-level signals.
        # Attach when ANY subregion's src_file matches the file the candidate
        # was defined in. Also include the hot fn's primary file (from anchor).
        sub_files: set[str] = set()
        if " @ " in anchor.function:
            primary_loc = anchor.function.split(" @ ", 1)[1].split(":", 1)[0]
            sub_files.add(primary_loc)
        for s in hot_region.hot_subregions:
            rng = _parse_src_range(s.src_lines)
            if rng is not None:
                sub_files.add(rng[0])
        seen_layout: set[tuple] = set()
        for f in sub_files:
            for lp in sp_scan.layout_patterns_by_file.get(f, []):
                key = (lp.loc, lp.details.get("pattern"))
                if key not in seen_layout:
                    seen_layout.add(key)
                    evidence.source_patterns["layout_candidate"].append(lp)

        # (b.3) Phase H.3: hand_written_hashmap — per-file evidence;
        # same file-set matching as layout_candidate.
        seen_hwh: set[str] = set()
        for f in sub_files:
            for hp in sp_scan.hashmap_patterns_by_file.get(f, []):
                if hp.loc in seen_hwh:
                    continue
                seen_hwh.add(hp.loc)
                evidence.source_patterns["hand_written_hashmap"].append(hp)

        # (b.4) Phase H.3: checksum_or_hash_kernel from static tables —
        # bzip2's CRC32 polynomial lives in `static BZ2_crc32Table`, not in
        # any fn body; attach by file match like the other per-file sources.
        seen_csum: set[tuple] = set()
        for f in sub_files:
            for cp in sp_scan.checksum_patterns_by_file.get(f, []):
                key = (cp.loc, cp.details.get("kind"))
                if key in seen_csum:
                    continue
                seen_csum.add(key)
                evidence.source_patterns["checksum_or_hash_kernel"].append(cp)

        # (c) Phase F: fn_signature_patterns — one entry per callback param,
        # aggregated over (bare ∪ inlined_fns). LTO inlines csv_parse(cb:
        # Option<...>) into csv_count::main so the relevant callback
        # footprint sits in main's hot region but the signature lives in
        # csv_parse. We collect all params from every fn in the inline set.
        from profiling.evidence import FnSignaturePattern
        seen_params: set[tuple] = set()
        for fn in fn_set:
            f = sp_scan.facts_by_fn.get(fn)
            if f is None:
                continue
            for pname in f.fn_ptr_param_names:
                key = (fn, pname)
                if key in seen_params:
                    continue
                seen_params.add(key)
                evidence.fn_signature_patterns.append(FnSignaturePattern(
                    pattern      = "option_fn_ptr_param",
                    loc          = f"{fn}",   # bare fn name (Step C+ adds file:line)
                    param        = pname,
                    type         = "Option<unsafe extern \"C\" fn(...)>",
                    related_rule = "G1 callback-monomorphize / D1 callback-devirt",
                    confidence   = "high",
                ))

    # ----- Step E: attribute opt_remarks + source_patterns (asm_tells were
    # attributed in-place by scan_asm_tells via subregion_lookup above)
    _attribute_src_evidence_to_subregions(evidence, hot_region.hot_subregions)

    return HotspotProfile(anchor=anchor, routing=routing, cpi_axis=cpi_axis,
                          ic_axis=ic_axis, evidence=evidence, notes=fn_notes)


def _empty_stability():
    from profiling.evidence import WorkloadStability
    return WorkloadStability(runs=0, mean_ms=0.0, stddev_ms=0.0, cv=0.0,
                             min_ms=0.0, max_ms=0.0, stable=False)


def _touch_sources(project_path: Path) -> int:
    """Bump the mtime of every `.rs` under `project_path` (skipping
    `target/`) so the next cargo build recompiles the staged crate —
    a precondition for LLVM opt-remarks to be re-emitted. Returns count."""
    n = 0
    for rs in project_path.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        try:
            rs.touch()
            n += 1
        except OSError:
            pass
    return n


# ---------------------------------------------------------------------------
# Output — JSON (full report) + JSONL (one self-contained line per hot fn)
# ---------------------------------------------------------------------------

def write_outputs(report: CharacterizationReport, json_path: Path) -> tuple[Path, Path]:
    """Write `<json_path>` (full report) and a sibling `.jsonl` (one
    HotspotProfile per line).

    Step A: LICM candidates are no longer a top-level field — they're
    already inside each hotspot's `evidence.source_patterns` (kind=
    "loop_invariant_branch"). The JSONL line is now just meta + hotspot."""
    json_path = Path(json_path).resolve()
    out_json = report.write_json(json_path)

    jsonl_path = json_path.with_suffix(".jsonl")
    meta = {
        "project_path":  report.project_path,
        "workload_name": report.workload_name,
        "binary":        report.binary,
        "scanned_at":    report.scanned_at,
    }
    with jsonl_path.open("w", encoding="utf-8") as f:
        for hs in report.hotspots:
            line = {
                **meta,
                "hotspot": hs.to_dict(),
            }
            f.write(json.dumps(line, ensure_ascii=False) + "\n")
    return out_json, jsonl_path


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def _cli() -> None:
    p = argparse.ArgumentParser(
        description="刻画 — produce per-hot-fn HotspotProfile records (§2.5)")
    p.add_argument("--project",  required=True,
                   help="absolute path to the staged Rust cargo project")
    p.add_argument("--manifest", required=True,
                   help="path to the workload manifest TOML")
    p.add_argument("--output",   required=True,
                   help="path to write the CharacterizationReport JSON "
                        "(a sibling .jsonl is written too)")
    p.add_argument("--top-n",          type=int,   default=10)
    p.add_argument("--skip-threshold", type=float, default=3.0,
                   help="self_time_pct gate; fns below are skipped (§2.5.1)")
    p.add_argument("--frequency",      type=int,   default=999,
                   help="perf record sampling frequency in Hz")
    p.add_argument("--skip-build",            action="store_true")
    p.add_argument("--skip-tma",              action="store_true")
    p.add_argument("--skip-cpi",              action="store_true")
    p.add_argument("--skip-ic",               action="store_true")
    p.add_argument("--skip-remarks",          action="store_true")
    p.add_argument("--skip-loop-invariant",   action="store_true")
    args = p.parse_args()

    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
        datefmt="%H:%M:%S",
    )

    report = characterize_project(
        project_path        = Path(args.project),
        manifest_path       = Path(args.manifest),
        top_n_hot           = args.top_n,
        skip_threshold_pct  = args.skip_threshold,
        frequency           = args.frequency,
        skip_build          = args.skip_build,
        skip_tma            = args.skip_tma,
        skip_cpi            = args.skip_cpi,
        skip_ic             = args.skip_ic,
        skip_remarks        = args.skip_remarks,
        skip_loop_invariant = args.skip_loop_invariant,
    )
    out_json, out_jsonl = write_outputs(report, Path(args.output))
    logger.info(f"[characterize] report → {out_json}")
    logger.info(f"[characterize] jsonl  → {out_jsonl}")
    logger.info(f"[characterize]   hot_fns={len(report.hotspots)} "
                f"skipped={len(report.skipped)} notes={len(report.notes)}")
    for hs in report.hotspots:
        logger.info(f"[characterize]   {hs.anchor.function} "
                    f"self={hs.anchor.self_time_pct:.1f}% "
                    f"regime={hs.routing.regime} "
                    f"region={hs.anchor.hot_region.asm_range or '-'}")
    for n in report.notes:
        logger.info(f"[characterize]   note: {n}")


if __name__ == "__main__":
    _cli()
