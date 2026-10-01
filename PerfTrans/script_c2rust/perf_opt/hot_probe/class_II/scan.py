""                                                                      
                                                                      

                                                                 

                                                          
                                                   
                                                    

                                                  
                                                                           
                                                                          

                                                                      
                                                                   
                                                  

                                                                             
                                                                    
                                                        
   

from __future__ import annotations

import json
import logging
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path

from perf_opt.hot_probe.class_II.build import build_and_collect
from perf_opt.hot_probe.class_II.residual_calls import (
    ResidualCallCounts, count_residual_calls,
)
from perf_opt.hot_probe.class_II.rules import classify
from perf_opt.hot_probe.symbol_source import FnIndex, build_fn_index

logger = logging.getLogger("hot_probe.class_II.scan")


@dataclass
class RemarkEvidence:
    """Per-fn breakdown of matched remarks: rule_id → {reason: count}."""
    counts: dict[str, dict[str, int]] = field(default_factory=dict)   # {rule_id: {reason: n}}
    samples: list[dict] = field(default_factory=list)                 # ≤5 raw remarks

    def add(self, rule_id: str, reason: str, sample: dict) -> None:
        self.counts.setdefault(rule_id, {})[reason] = \
            self.counts.get(rule_id, {}).get(reason, 0) + 1
        if len(self.samples) < 5:
            self.samples.append(
                {k: sample.get(k) for k in ("pass", "status", "line", "message")})


@dataclass
class ScanResult:
    """Class II inventory.

      c3_remark_hits    {fn: RemarkEvidence}    — C3 ① channel (needs class_I to
                                                  finalize via !tbaa gate)
      ii_vec_hits       {fn: RemarkEvidence}    — passes P_II_vec (remark channel;
                                                  no IR gate for II_vec)
      ii_inl_candidates {fn: RemarkEvidence}    — passes remark filter for II_inl;
                                                  needs `residual_calls` gate before
                                                  becoming ii_inl_hits
      ii_inl_hits       {fn: RemarkEvidence}    — passes full P_II_inl (remark + IR)
      residual_calls    ResidualCallCounts | None  — populated if ir_path given
      n_remarks         total parsed remarks
      n_classified      remarks that mapped to some rule
      build_ok
    """
    c3_remark_hits:    dict[str, RemarkEvidence] = field(default_factory=dict)
    ii_vec_hits:       dict[str, RemarkEvidence] = field(default_factory=dict)
    ii_inl_candidates: dict[str, RemarkEvidence] = field(default_factory=dict)
    ii_inl_hits:       dict[str, RemarkEvidence] = field(default_factory=dict)
    residual_calls:    ResidualCallCounts | None = None
    n_remarks: int = 0
    n_classified: int = 0
    build_ok: bool = True
    error: str = ""


# ── attribution helpers ─────────────────────────────────────────────────────


def _fn_by_line_index(index: FnIndex) -> dict[str, list[tuple[int, int, str]]]:
    """Build {file: [(line_start, line_end, fn_name), ...]} for range lookup.
    Longer fns first inside each file so nested wins over enclosing."""
    out: dict[str, list[tuple[int, int, str]]] = defaultdict(list)
    for fn in index.names():
        loc = index.resolve(fn)
        if loc is None:
            continue
        f, s, e = loc
        out[f].append((s, e, fn))
    for f, lst in out.items():
        lst.sort(key=lambda t: t[1] - t[0])
    return out


def _resolve_fn(remark: dict, index_map: dict[str, list[tuple[int, int, str]]]
                ) -> str | None:
    """Map a remark's file:line → fn name. `remark['file']` is the source path
    as LLVM saw it — may be absolute or crate-relative; we match by basename
    and (when both are absolute) by exact path."""
    f = remark.get("file", "") or ""
    ln = remark.get("line")
    if not f or ln is None:
        return None
    cands = index_map.get(f)
    if not cands:
        from pathlib import Path as _P
        base = _P(f).name
        for k, lst in index_map.items():
            if _P(k).name == base:
                cands = lst
                break
    if not cands:
        return None
    for s, e, fn in cands:
        if s <= ln <= e:
            return fn
    return None


# ── source-fn → mangled matching for the II_inl residual gate ────────────
#
# residual_calls is keyed on `mangled name` from the `define` line in IR,
# but our remark attribution key is the source fn name. The check now uses
# Rust's `<len><name>` mangling convention (via
# `ResidualCallCounts.has_residual_for_source_fn`) — bare substring
# matching inflates false positives on short names (see method docstring).


# ── main API ───────────────────────────────────────────────────────────────


def scan(*, crate: Path, harness_dir: Path,
         opt_remarks: list[dict] | None = None,
         ir_path: Path | None = None,
         hot_fns: set[str] | None = None,
         out_dir: Path | None = None) -> ScanResult:
    """Apply Class II predicates over remarks (and IR, for II_inl gate).

    Args:
      crate         — crate under test (source fn index)
      harness_dir   — harness project (build target if opt_remarks is None)
      opt_remarks   — pre-collected list from `class_II.build.build_and_collect`
                      (or another compatible parser). If None, triggers a build.
      ir_path       — optional harness `.ll`. If given, residual call gate
                      applies to II_inl (else all II_inl candidates surface
                      as `ii_inl_candidates` without hit confirmation).
      hot_fns       — optional set of fn names satisfying `hot(·)`. If None,
                      predicate `hot(f)` is treated as True for every fn
                      (deploy mode without perf gating).
      out_dir       — optional; writes `class_II_hits.json`
    """
    if opt_remarks is None:
        rr = build_and_collect(harness_dir,
                               also_emit_ir=ir_path is None)
        if not rr.ok:
            return ScanResult(build_ok=False, error=rr.error)
        opt_remarks = rr.remarks
        # F2 fix (2026-07-23): if we requested emit-ir on this build (i.e.
        # caller gave no ir_path), locate the emitted .ll now so the
        # II_inl residual gate isn't silently skipped later. Without this,
        # the fallback path builds the IR but never uses it.
        if ir_path is None:
            from perf_opt.hot_probe.class_I.build import _pick_harness_ll
            picked = _pick_harness_ll(Path(harness_dir) / "target",
                                      harness_pkg="harness")
            if picked is not None:
                ir_path = picked
                logger.info("[class_II.scan] fallback build emitted IR at %s "
                            "(picked up for II_inl residual gate)",
                            ir_path.name)
            else:
                logger.warning("[class_II.scan] fallback build requested "
                               "emit-ir but no harness-*.ll found — II_inl "
                               "residual gate will be skipped")
    logger.info("[class_II.scan] classifying %d remark(s)", len(opt_remarks))

    index = build_fn_index(crate)
    index_map = _fn_by_line_index(index)

    # Site-level dedup at classification time (rustc's fat-LTO emits some
    # sites twice — pre-link + LTO). Key: (file, line, col, pass, reason).
    seen_sites: set[tuple] = set()

    c3_remark_hits: dict[str, RemarkEvidence] = {}
    ii_vec_hits:    dict[str, RemarkEvidence] = {}
    ii_inl_cands:   dict[str, RemarkEvidence] = {}

    n_classified = 0
    for r in opt_remarks:
        c = classify(r)
        if c.rule_id is None:
            continue
        site = (r.get("file"), r.get("line"), r.get("col"),
                r.get("pass"), c.reason)
        if site in seen_sites:
            continue
        seen_sites.add(site)
        fn = _resolve_fn(r, index_map)
        if fn is None:
            continue
        if hot_fns is not None and fn not in hot_fns:
            continue
        n_classified += 1

        target = {"C3": c3_remark_hits,
                  "II_vec": ii_vec_hits,
                  "II_inl": ii_inl_cands}[c.rule_id]
        target.setdefault(fn, RemarkEvidence()).add(c.rule_id, c.reason, r)

    logger.info("[class_II.scan] classified %d remark(s) → C3=%d fn / "
                "II_vec=%d fn / II_inl_cand=%d fn",
                n_classified, len(c3_remark_hits), len(ii_vec_hits),
                len(ii_inl_cands))

    # ── II_inl residual-call gate ──
    ii_inl_hits: dict[str, RemarkEvidence] = {}
    residual_counts: ResidualCallCounts | None = None
    if ir_path is not None:
        residual_counts = count_residual_calls(Path(ir_path))
        for fn, ev in ii_inl_cands.items():
            if residual_counts.has_residual_for_source_fn(fn):
                ii_inl_hits[fn] = ev
        logger.info("[class_II.scan] P_II_inl gate: %d/%d candidates "
                    "survived residual-call check",
                    len(ii_inl_hits), len(ii_inl_cands))
    else:
        logger.info("[class_II.scan] no ir_path — II_inl candidates NOT gated "
                    "(showing candidates as ii_inl_candidates only)")

    result = ScanResult(
        c3_remark_hits=c3_remark_hits,
        ii_vec_hits=ii_vec_hits,
        ii_inl_candidates=ii_inl_cands,
        ii_inl_hits=ii_inl_hits,
        residual_calls=residual_counts,
        n_remarks=len(opt_remarks),
        n_classified=n_classified,
        build_ok=True,
    )

    if out_dir is not None:
        out_dir = Path(out_dir)
        out_dir.mkdir(parents=True, exist_ok=True)
        (out_dir / "class_II_hits.json").write_text(json.dumps({
            "n_remarks": result.n_remarks,
            "n_classified": result.n_classified,
            "c3_remark_hits": {fn: ev.counts for fn, ev in c3_remark_hits.items()},
            "ii_vec_hits":    {fn: ev.counts for fn, ev in ii_vec_hits.items()},
            "ii_inl_candidates": {fn: ev.counts for fn, ev in ii_inl_cands.items()},
            "ii_inl_hits":    {fn: ev.counts for fn, ev in ii_inl_hits.items()},
        }, indent=2))
        logger.info("[class_II.scan] wrote %s", out_dir / "class_II_hits.json")

    return result
