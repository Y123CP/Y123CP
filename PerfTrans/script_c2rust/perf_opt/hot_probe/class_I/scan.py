"""Class I scanner — orchestrates C1 / C2 / C3 predicate application.

Consumes:
  * fat-LTO harness .ll   → C1/C2 IR site scan (rules.scan_ir_for_c1_c2)
                          → attribute via inlinedAt chain
                          → !tbaa counts per fn (c3.count_tbaa_per_fn)
  * class_II ScanResult   → C3 remark ①-channel evidence (c3_remark_hits)
                          → apply full P_C3 predicate (c3.apply_c3_predicate)

Emits:
  * ScanResult with 3 per-fn hit dicts (C1, C2, C3) + attribution
    breakdown (self / inherited) + unresolved / stdlib_only tallies

Public entry:
  scan(crate, harness_dir, ir_path=None, class_ii_result=None,
       hot_fns=None, out_dir=None) → ScanResult
"""

from __future__ import annotations

import json
import logging
from dataclasses import dataclass, field
from pathlib import Path

from perf_opt.hot_probe.class_I.attribute import AggregatedHits, aggregate
from perf_opt.hot_probe.class_I.build import build_and_emit_ir
from perf_opt.hot_probe.class_I.c3 import (
    apply_c3_predicate, count_tbaa_per_fn,
)
from perf_opt.hot_probe.class_I.ir_parse import parse_ir_metadata
from perf_opt.hot_probe.class_I.rules import scan_ir_for_c1_c2

logger = logging.getLogger("hot_probe.class_I.scan")


@dataclass
class ScanResult:
    """Class I inventory.

      c1_c2_hits_by_fn   {fn: {C1: n, C2: n}} — flat per-fn counts (self+inherited)
      c1_c2_inheritance  {host_fn: {source_fn: {rule: n}}} — RQ3 structured
      c3_hits            set[fn] — passes full P_C3 (remark + !tbaa=0)
      c3_dropped_tbaa    set[fn] — remark fired but !tbaa > 0 (mechanism warn)
      unresolved         {rule: count} — sites w/ no !dbg attribution
      stdlib_only        {rule: count} — sites whose chain is entirely stdlib
      tbaa_counts        {fn_mangled: count} — raw !tbaa tally
      ir_path            .ll used
      build_ok           False on build failure
      error              build error msg
    """
    c1_c2_hits_by_fn:  dict[str, dict[str, int]]                          = field(default_factory=dict)
    c1_c2_inheritance: dict[str, dict[str, dict[str, int]]]               = field(default_factory=dict)
    c3_hits:           set[str]                                           = field(default_factory=set)
    c3_dropped_tbaa:   set[str]                                           = field(default_factory=set)
    unresolved:        dict[str, int]                                     = field(default_factory=dict)
    stdlib_only:       dict[str, int]                                     = field(default_factory=dict)
    tbaa_counts:       dict[str, int]                                     = field(default_factory=dict)
    ir_path:           Path | None                                        = None
    build_ok:          bool                                               = True
    error:             str                                                = ""
                                                                
                                                             
    sites:             list                                               = field(default_factory=list)  # list[Site]
    tables:            object | None                                     = None  # MetaTables | None


def scan(*, crate: Path, harness_dir: Path,
         ir_path: Path | None = None,
         class_ii_result: object | None = None,
         hot_fns: set[str] | None = None,
         cache_dir: Path | None = None,
         out_dir: Path | None = None) -> ScanResult:
    """Apply C1 / C2 / C3 predicates.

    Args:
      crate            — crate under test
      harness_dir      — harness project
      ir_path          — pre-built `.ll` (if None, builds via build_and_emit_ir)
      class_ii_result  — a `class_II.ScanResult` providing `c3_remark_hits`
                         (channel ①). If None, C3 is skipped (only C1/C2).
      hot_fns          — optional set of fn names satisfying hot(·).
                         `hot(s)` predicate applied AFTER attribution.
      cache_dir        — optional `.ll` cache location
      out_dir          — optional; writes `class_I_hits.json`
    """
    if ir_path is None:
        rr = build_and_emit_ir(harness_dir, cache_dir=cache_dir)
        if not rr.ok:
            return ScanResult(build_ok=False, error=rr.error)
        ir_path = rr.ir_path
    else:
        ir_path = Path(ir_path)
        if not ir_path.is_file():
            return ScanResult(build_ok=False,
                              error=f"provided ir_path missing: {ir_path}")

    # ── IR-side: parse metadata + scan C1/C2 sites ──
    tables = parse_ir_metadata(ir_path)
    sites = scan_ir_for_c1_c2(ir_path)
    agg: AggregatedHits = aggregate(sites, tables)

    # apply hot(s) filter on attributed sites (post-attribution because
    # attribution is what maps site → source fn)
    if hot_fns is not None:
        # attribute both flat and inheritance already; here we prune fn keys
        pruned_flat = {fn: cnts for fn, cnts in agg.hits_by_fn.items()
                       if fn in hot_fns}
        pruned_inh = {host: {src: cnts for src, cnts in srcs.items()
                             if src in hot_fns or host in hot_fns}
                      for host, srcs in agg.inheritance.items()
                      if host in hot_fns}
        agg.hits_by_fn = pruned_flat
        agg.inheritance = {k: v for k, v in pruned_inh.items() if v}

    # ── IR-side: !tbaa counts ──
    tbaa_counts = count_tbaa_per_fn(ir_path)

    # ── C3 predicate (needs class_II's channel ①) ──
    c3_hits: set[str] = set()
    c3_dropped: set[str] = set()
    if class_ii_result is not None:
        c3_remark_hits = getattr(class_ii_result, "c3_remark_hits", {})
        if hot_fns is not None:
            c3_remark_hits = {fn: ev for fn, ev in c3_remark_hits.items()
                              if fn in hot_fns}
        c3_hits, c3_dropped = apply_c3_predicate(c3_remark_hits, tbaa_counts)
    else:
        logger.info("[class_I.scan] no class_II_result — C3 skipped "
                    "(only C1/C2 applied)")

    result = ScanResult(
        c1_c2_hits_by_fn=agg.hits_by_fn,
        c1_c2_inheritance=agg.inheritance,
        c3_hits=c3_hits,
        c3_dropped_tbaa=c3_dropped,
        unresolved=agg.unresolved,
        stdlib_only=agg.stdlib_only,
        tbaa_counts={k: v for k, v in tbaa_counts.items() if v > 0},
        ir_path=ir_path,
        build_ok=True,
        sites=sites,                               
        tables=tables,                                                     
    )

    if out_dir is not None:
        out_dir = Path(out_dir)
        out_dir.mkdir(parents=True, exist_ok=True)
        (out_dir / "class_I_hits.json").write_text(json.dumps({
            "ir_path": str(ir_path),
            "c1_c2_hits_by_fn": result.c1_c2_hits_by_fn,
            "c1_c2_inheritance": result.c1_c2_inheritance,
            "c3_hits": sorted(result.c3_hits),
            "c3_dropped_tbaa": sorted(result.c3_dropped_tbaa),
            "unresolved": result.unresolved,
            "stdlib_only": result.stdlib_only,
            "tbaa_counts_nonzero": result.tbaa_counts,
        }, indent=2))
        logger.info("[class_I.scan] wrote %s", out_dir / "class_I_hits.json")

    return result
