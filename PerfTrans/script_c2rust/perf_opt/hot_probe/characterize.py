"""hot_probe characterize phase — enrich each hot function into a full
EvidencePack (5-rule taxonomy): identity + profile counters + TMA +
per-fn C1/C2/C3/II_vec/II_inl hits + filtered raw remarks.

Consumes pre-computed `class_I.ScanResult` + `class_II.ScanResult` from
the driver. Does NOT run its own signature scans — the driver runs one
`class_II.build_and_collect(also_emit_ir=True)` build and passes both
scan results here to avoid double-building.

Attribution split:
  * per-fn (self-time / instructions / cpi) — the deepest-crate-frame sampler
    (`crate_event_shares`), robust to harness-inlining / libc delegation.
  * process-level (tma / branch-miss) — the copied profiling collectors,
    honestly labeled `scope: process`.
  * class_i_hits / class_ii_hits — from ScanResult(s) via `hits_from_scan_results`
  * llvm_opt_remarks — filtered to the fn's source line range.
  * hot_region — the whole function for v1 (precise per-line srcline
    attribution is correct but too slow to run routinely).
  * llvm_ir — optional, off by default.

Unavailable-by-design (never faked): call_count / instructions_per_call
(sampling can't recover call counts reliably).
"""

from __future__ import annotations

import json
import logging
from collections import defaultdict
from pathlib import Path

from harness_gen.inventory import scan_crate
from perf_opt.verify.measure import autotune_iters, pick_input
from perf_opt.hot_probe.locate import _package_name
from perf_opt.hot_probe.profiling import (collect_cpi_counters,
                                          derive_cpi_metrics, run_tma)
from perf_opt.hot_probe.selftime import crate_event_shares
from perf_opt.hot_probe.symbol_source import build_fn_index
from perf_opt.hot_probe.types import (EvidencePack, HotFunction,
                                        hits_from_scan_results)

logger = logging.getLogger("hot_probe.characterize")

CHAR_TARGET_WALL = 2.0


# ── P0 (D13) — agent_perf_opt EvidencePack fields ──────────────────────
#
# Attribution honesty:  each profile metric is either per-fn (attributable
# to the hot function), process-scoped (whole workload), or unavailable
# from sampling. Agents show this next to the numbers so LLMs don't argue
# from a process-scope figure as if it were the fn's own.
_ATTRIBUTION_SCOPE: dict[str, str] = {
    "self_time_ratio":       "function",
    "retired_instructions":  "function",
    "cpi":                   "function",
    "tma":                   "process",
    "branch_miss_rate":      "process",
    "call_count":            "unavailable",
    "instructions_per_call": "unavailable",
}

# Dimensions we consider when picking the dominant TMA bottleneck.
# Excludes `backend_bound` (L1 aggregate of memory+core) — we prefer the
# L2 drill-down keys `memory_bound` / `core_bound` when they exist.
_TMA_CANDIDATE_DIMS = (
    "retiring", "frontend_bound", "bad_speculation",
    "memory_bound", "core_bound",
)


def _extract_signature(rust_source: str) -> str:
    """Signature portion of the fn source (start → first '{' exclusive).

    v1 uses simple substring — c2rust output rarely contains '{' inside
    attributes / doc-comments. If tightening becomes necessary, switch to
    tree-sitter `function_item` signature subtree.
    """
    if not rust_source:
        return ""
    idx = rust_source.find("{")
    sig = rust_source[:idx] if idx >= 0 else rust_source
    return sig.strip()


def _pick_tma_bottleneck(tma: dict) -> str:
    """Return the TMA dimension (from `_TMA_CANDIDATE_DIMS`) with the highest
    numeric value. Empty string when nothing scores.

    Feeds `EvidencePack.tma_bottleneck`, which the prompt reports as
    background. The agent-side routing that used to consume it is gone —
    this helper only reports the raw max, with no gating of its own.
    """
    best_k, best_v = "", -1.0
    for k in _TMA_CANDIDATE_DIMS:
        v = tma.get(k)
        if v is None:
            continue
        try:
            fv = float(v)
        except (TypeError, ValueError):
            continue
        if fv > best_v:
            best_k, best_v = k, fv
    return best_k


def _tma_l2(tma_path: str) -> tuple[float | None, float | None]:
    """Extract the L2 Backend split (Memory_Bound / Core_Bound %) from toplev's
    drill-down path string, e.g. `... > Backend_Bound.Memory_Bound 30.1% > ...`."""
    import re
    def _grab(name: str) -> float | None:
        m = re.search(rf"{name}\s+([0-9.]+)%", tma_path or "")
        return float(m.group(1)) if m else None
    return _grab("Memory_Bound"), _grab("Core_Bound")


def _filter_remarks(remarks: list[dict], file: str,
                    line_start: int, line_end: int) -> list[dict]:
    """Opt-remarks whose source location falls inside the fn's line range."""
    out = []
    base = Path(file).name
    for r in remarks:
        rf = r.get("file", "") or ""
        if not (rf.endswith(file) or Path(rf).name == base):
            continue
        ln = r.get("line")
        if ln is None or not (line_start <= ln <= line_end):
            continue
        out.append({k: r.get(k) for k in ("pass", "status", "line", "message")})
    return out


def characterize(hot_fns: list[HotFunction], *,
                 crate: Path, harness_dir: Path,
                 harness_bin: Path, assets, opt_dir: Path,
                 class_i_result=None,
                 class_ii_result=None,
                 opt_remarks: list | None = None,
                 with_ir: bool = False) -> list[EvidencePack]:
    """Build one EvidencePack per hot function; write opt/evidence/<fn>.json.

    Args:
      hot_fns          — from `locate_hotspots()`; may be already-augmented with
                         class_i_hits / class_ii_hits (driver's `augment_hot_fns`)
                         or fresh — we re-look-up from scan results anyway.
      class_i_result   — from `class_I.scan(...)`. If None, class_i_hits stay {}.
      class_ii_result  — from `class_II.scan(...)`. Also source of the
                         `opt_remarks` list for the raw-remark evidence.
      opt_remarks      — pre-parsed remark list. If None but class_ii_result is
                         given, we use class_ii_result implicitly (its scan
                         already consumed the remarks).
      with_ir          — reserve for future: pipe fn IR text into EvidencePack.
    """
    if not hot_fns:
        return []
    crate_name = _package_name(crate)
    inv = scan_crate(crate)
    exported = set(inv.link_names) | set(inv.fn_names)
    scratch = opt_dir / ".hot_probe"
    scratch.mkdir(parents=True, exist_ok=True)
    index = build_fn_index(crate)
    resolvable = index.names()

    all_remarks = opt_remarks if opt_remarks is not None else []
    logger.info("[characterize] using %d raw remark(s) for per-fn filtering",
                len(all_remarks))

    by_op: dict[str, list[HotFunction]] = defaultdict(list)
    for hf in hot_fns:
        by_op[hf.hottest_op].append(hf)

    packs: list[EvidencePack] = []
    for op, fns in by_op.items():
        inp = pick_input(assets.harness_src, op)
        if inp is None:
            continue
        iters = autotune_iters(harness_bin, op, inp, target_wall=CHAR_TARGET_WALL)
        if iters == 0:
            logger.warning("[characterize] %s skip (errors on input)", op)
            continue
        args = [op, str(inp), str(iters)]

        # per-fn dynamic-work share (deepest crate frame, instructions event)
        insn_share = crate_event_shares(harness_bin, op, inp, iters, scratch,
                                        crate_name, exported, event="instructions:u",
                                        resolvable=resolvable)
        # process-level counters + TMA
        counters, _ = collect_cpi_counters(harness_bin, args)
        metrics = derive_cpi_metrics(counters)
        total_ins = counters.get("instructions")
        total_cyc = counters.get("cycles")
        proc_bmiss = metrics.get("branch_mispredict_rate")   # 0-100 or None
        tma = run_tma(harness_bin, args)
        # Prefer the raw-event L2 split (perf-rawevents path); fall back to
        # toplev's drill-down string only when raw events weren't available.
        if tma.tma_memory_bound is not None or tma.tma_core_bound is not None:
            mem_bound, core_bound = tma.tma_memory_bound, tma.tma_core_bound
        else:
            mem_bound, core_bound = _tma_l2(tma.tma_path)
        tma_d = {
            "scope": "process",
            "retiring": tma.tma_retiring,
            "frontend_bound": tma.tma_frontend_bound,
            "bad_speculation": tma.tma_bad_speculation,
            "backend_bound": tma.tma_backend_bound,
            "memory_bound": mem_bound,   # from toplev L2 drill-down (tma_path)
            "core_bound": core_bound,
            "tma_path": tma.tma_path or None,
            "source": tma.source,
        }
        logger.info("[characterize] %-20s tma(retiring=%s) insn-shares=%d fn(s)",
                    op, tma.tma_retiring, len(insn_share))

        for hf in fns:
            ishare = insn_share.get(hf.name)          # 0-100 or None
            retired = (int(total_ins * ishare / 100)
                       if (total_ins and ishare) else None)
            cpi_fn = None
            if total_cyc and total_ins and ishare:
                cyc_fn = hf.self_pct / 100 * total_cyc
                ins_fn = ishare / 100 * total_ins
                cpi_fn = round(cyc_fn / ins_fn, 3) if ins_fn else None
            src = (index.source(hf.file, hf.line_start, hf.line_end)
                   if hf.file else "")
            remarks_fn = (_filter_remarks(all_remarks, hf.file,
                                          hf.line_start, hf.line_end)
                          if hf.file else [])
            # Rule hits — canonical source is scan results; fall back to
            # whatever was pre-populated on the HotFunction (driver may have
            # augmented already, which is idempotent).
            c_i, c_ii = hits_from_scan_results(
                hf.name, class_i_result, class_ii_result)
            if not c_i and hf.class_i_hits:
                c_i = dict(hf.class_i_hits)
            if not c_ii and hf.class_ii_hits:
                c_ii = dict(hf.class_ii_hits)

            packs.append(EvidencePack(
                symbol=hf.name,
                location=(f"{hf.file}:{hf.line_start}-{hf.line_end}"
                          if hf.file else None),
                workload=op,
                hot_region=(f"{hf.file}:{hf.line_start}-{hf.line_end}"
                            if hf.file else None),   # v1: whole function
                rust_source=src,
                self_time_ratio=round(hf.self_pct / 100, 4),
                retired_instructions=retired,
                cpi=cpi_fn,
                tma=tma_d,
                branch_miss_rate=(round(proc_bmiss / 100, 4)
                                  if proc_bmiss is not None else None),
                class_i_hits=c_i,
                class_ii_hits=c_ii,
                llvm_opt_remarks=remarks_fn,
                llvm_ir=None,            # with_ir hook — off by default
                # ─── P0 (D13) fields for agent_perf_opt ──────────────
                signature=_extract_signature(src),
                attribution_scope=dict(_ATTRIBUTION_SCOPE),
                tma_bottleneck=_pick_tma_bottleneck(tma_d),
            ))

    evdir = opt_dir / "evidence"
    evdir.mkdir(parents=True, exist_ok=True)
    for p in packs:
        (evdir / f"{p.symbol}.json").write_text(
            json.dumps(p.to_dict(), indent=2), encoding="utf-8")
    logger.info("[characterize] %d evidence pack(s) → %s", len(packs), evdir)
    return packs
