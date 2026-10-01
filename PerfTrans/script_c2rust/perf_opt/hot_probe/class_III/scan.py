""                                                         

                                                                    
               

           
                                                      

                                                                          

                          
                                                              
                                                           
                                                             
                                                                     
   

from __future__ import annotations

import dataclasses
import json
import logging
from dataclasses import dataclass, field
from pathlib import Path

from perf_opt.hot_probe.class_III import (
    cst_utils,
    iii1_callback,
    iii2_alloc,
    iii3_mem_ops,
    iii4_raw_ptr_cursor,
)
from perf_opt.hot_probe.class_III.cst_utils import CallSite, FnKey
from perf_opt.hot_probe.class_III.iii4_raw_ptr_cursor import CursorSite

logger = logging.getLogger(__name__)


@dataclass
class ScanResult:
    """SPEC §7.2 return schema — full class_III inventory."""

    # Rule hits (fn-level dicts).
    # iii4 is the raw-ptr cursor detector: source-cause attribution, no
                                                                         
    iii1_hits: dict[FnKey, list[CallSite]] = field(default_factory=dict)
    iii2_hits: dict[FnKey, list[CallSite]] = field(default_factory=dict)
    iii3_hits: dict[FnKey, list[CallSite]] = field(default_factory=dict)
    iii4_hits: dict[FnKey, list[CursorSite]] = field(default_factory=dict)

    # Cross-rule metadata
    libc_extern_set: set[str] = field(default_factory=set)
    custom_mem_op_fns: set[str] = field(default_factory=set)                                             

    # Audit buckets (never used as inputs to hit predicates — pure telemetry)
    iii1_deep_receiver: list[CallSite] = field(default_factory=list)
    iii1_scrutinee_unresolved: list[CallSite] = field(default_factory=list)
    iii1_c1_boundary_unresolved: list[CallSite] = field(default_factory=list)
    iii2_ambiguous_callees: list[CallSite] = field(default_factory=list)
    iii3_ambiguous_callees: list[CallSite] = field(default_factory=list)

    # Housekeeping
    error: str | None = None
    n_fns_seen: int = 0
    n_fns_analyzed: int = 0


def scan(
    crate: Path,
    hot_fns: set[FnKey] | None = None,
    out_dir: Path | None = None,
) -> ScanResult:
    """Run all four III detectors against `crate`.

    Args:
      crate:           c2rust output crate root (contains src/ + Cargo.toml).
      hot_fns:         candidate pool. If None, every fn is a candidate
                       (data will be inflated).

                       DUAL-MODE MATCHING (B2 audit + driver compat):
                         - If every entry contains `::` → treated as
                           qualified FnKeys ("crate::mod::foo"), exact
                           match against `fn_items.keys()`.
                         - Otherwise treated as bare base names
                           ("tdefl_compress"), matched against each fn's
                           last-segment name.
                       This lets driver.py pass the same
                       `{hf.name for hf in hotspots}` set (base names
                       from the perf profiler) that class_I / class_II
                       already accept, without a bespoke qualified-path
                       translation layer.
      out_dir:         if given, write `class_III_hits.json` for driver dedup.

    Sequencing (see SPEC §7):
      1. parse crate  (cst_utils.parse_crate)
      2. build indices  (fn_items, struct_index, extern_index,
                         non_extern_base, type_aliases, custom_mem_op_fns)
      3. filter to hot_fns
      4. iii1/iii2/iii3/iii4 per-fn detection
      5. write JSON audit (out_dir/class_III_hits.json)
    """
    result = ScanResult()

    # ── Phase 1: parse ──────────────────────────────────────────────────────
    trees = cst_utils.parse_crate(crate)
    if not trees:
        logger.warning(f"[scan] no .rs files parsed under {crate}; "
                       f"returning empty ScanResult")
        result.error = f"no src/ found under {crate}"
        if out_dir is not None:
            _write_json(result, out_dir / "class_III_hits.json")
        return result

    # ── Phase 2: build crate-wide indices ───────────────────────────────────
    fn_items = cst_utils.collect_fn_items(trees)
    struct_index = cst_utils.collect_struct_fields(trees)
    extern_index = cst_utils.collect_extern_c_decls(trees)
    non_extern_base = cst_utils.collect_non_extern_base(fn_items)
    # A1 audit: type_aliases is REQUIRED by iii1_callback.detect (typedef
                                                                     
    # skipping this step reverts miniz III① from 122 → 0.
    type_aliases = cst_utils.collect_type_aliases(trees)
    # Post-batch-audit fix: module-level `static xmlFree: Option<fn ptr>`
    # is the dominant callback binding source in libxml2 (2382/2720 miss).
    # Collect once here so form B receiver_type_at resolves them.
    static_items = cst_utils.collect_static_items(trees)
    # III③ body-pattern detection: identify project-defined byte-copy /
    # mem-op wrapper fns (e.g. copy_be32, libzahl_mem*) whose body is a
    # short sequence of `*p.offset(i) = ...` byte load/stores. Recognized
    # once at crate scope, then III③ treats calls to them as mem-op sites.
    result.custom_mem_op_fns = iii3_mem_ops.detect_custom_mem_op_fns(fn_items)

    result.libc_extern_set = set(extern_index.decls.keys())
    result.n_fns_seen = len(fn_items)

    # ── Phase 3: filter to hot_fns (dual-mode: FnKey or base name) ─────────
    if hot_fns is None:
        candidates = fn_items
    else:
        # Auto-detect mode: if every entry contains `::`, treat as FnKeys;
        # otherwise treat as bare base names (class_I/II compatibility).
        is_qualified_mode = all("::" in h for h in hot_fns) if hot_fns else True
        if is_qualified_mode:
            candidates = {k: v for k, v in fn_items.items() if k in hot_fns}
            missing = hot_fns - set(candidates.keys())
        else:
            candidates = {k: v for k, v in fn_items.items() if v.name in hot_fns}
            matched_names = {v.name for v in candidates.values()}
            missing = hot_fns - matched_names
        if missing:
            logger.info(f"[scan] {len(missing)} hot_fns not found "
                        f"(mode={'FnKey' if is_qualified_mode else 'base_name'}); "
                        f"e.g. {sorted(missing)[:3]}")
    result.n_fns_analyzed = len(candidates)

    # ── Phase 4: iii1 / iii2 / iii3 / iii4 per-fn ───────────────────────────
    for key, entry in candidates.items():
        r1 = iii1_callback.detect(entry, struct_index, type_aliases, static_items)
        if r1.hits:
            result.iii1_hits[key] = r1.hits
        result.iii1_deep_receiver.extend(r1.deep_receiver)
        result.iii1_scrutinee_unresolved.extend(r1.scrutinee_unresolved)
        result.iii1_c1_boundary_unresolved.extend(r1.c1_boundary_unresolved)

        r2 = iii2_alloc.detect(entry, non_extern_base)
        if r2.hits:
            result.iii2_hits[key] = r2.hits
        result.iii2_ambiguous_callees.extend(r2.ambiguous_callees)

        r3 = iii3_mem_ops.detect(
            entry, extern_index, non_extern_base,
            custom_mem_op_fns=result.custom_mem_op_fns,
        )
        if r3.hits:
            result.iii3_hits[key] = r3.hits
        result.iii3_ambiguous_callees.extend(r3.ambiguous_callees)

        r4 = iii4_raw_ptr_cursor.detect(entry)
        if r4.hits:
            result.iii4_hits[key] = r4.hits

    # ── Phase 5: JSON audit ────────────────────────────────────────────────
    if out_dir is not None:
        out_dir.mkdir(parents=True, exist_ok=True)
        _write_json(result, out_dir / "class_III_hits.json")

    logger.info(
        f"[scan] {crate.name}: iii1={len(result.iii1_hits)}fn / "
        f"iii2={len(result.iii2_hits)}fn / iii3={len(result.iii3_hits)}fn / "
        f"iii4={len(result.iii4_hits)}fn"
    )
    return result


# ── JSON serialization (C3 audit) ─────────────────────────────────────────

def _write_json(result: ScanResult, path: Path) -> None:
    """Serialize ScanResult to JSON for driver-layer dedup.

    Path / Enum → str, dataclass → dict via `dataclasses.asdict`. Nothing
    stored in ScanResult holds a tree_sitter.Node (verified: CallSite and
    CursorSite carry only str/int/bool primitives), so no Node stripping
    is needed.
    """
    obj = dataclasses.asdict(result)

    if isinstance(obj.get("libc_extern_set"), set):
        obj["libc_extern_set"] = sorted(obj["libc_extern_set"])
    if isinstance(obj.get("custom_mem_op_fns"), set):
        obj["custom_mem_op_fns"] = sorted(obj["custom_mem_op_fns"])
    for k, v in list(obj.items()):
        if isinstance(v, set):
            obj[k] = sorted(v)

    path.write_text(json.dumps(obj, indent=2, default=_json_default))
    logger.info(f"[scan] wrote audit → {path}")


def _json_default(o):
    """Fallback for anything json can't natively serialize."""
    if isinstance(o, Path):
        return str(o)
    if isinstance(o, set):
        return sorted(o)
    if hasattr(o, "value"):     # Enum instances
        return o.value
    raise TypeError(f"un-serializable: {type(o).__name__}")
