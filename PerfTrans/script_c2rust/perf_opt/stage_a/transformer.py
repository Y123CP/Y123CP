"""Stage A Phase B — CST-anchored Transformer.

All edits are computed as `Edit(file, start_byte, end_byte, replacement)`
records up-front, then applied atomically per file (right-to-left so
earlier offsets stay valid). No file is written until the full batch
has been laid out — if anything is malformed, we fail before touching
disk.

Three families of edits drive E1:

  (1) `_e1_lib_side_edits`     — at the definition site:
        * drop `#[no_mangle]` and/or `#[linkage = "..."]` attributes
        * drop the `extern "C"` modifier from function_modifiers
        * insert `#[inline]` above the (now-trimmed) attribute block
          IFF body_line_count > 0 AND < INLINE_HINT_MAX_LINES AND
          no #[inline] / #[cold] already present

  (2) `_e1_extern_block_edits` — every internal `extern "C" {}` block:
        * delete each `fn NAME(...);` declaration whose NAME is lifted
          (CST handles `pub fn`, multi-line params, return types)
        * if the block has no surviving declarations, delete the
          whole block (and trailing newline)

  (3) `_use_import_edits`      — for every caller file that references
        a lifted fn:
        * inject `use <crate_or_self>::src::<module>::NAME;` once,
          right after the last `use` line near the file head
"""

from __future__ import annotations

import logging
import re
from dataclasses import dataclass, field
from pathlib import Path

from perf_opt.stage_a.analyzer import (
    AnalysisReport,
    Candidate,
    ExternBlock,
    _project_rs_files,
)

logger = logging.getLogger(__name__)

INLINE_HINT_MAX_LINES = 500


@dataclass
class Edit:
    """A byte-level edit."""
    file: Path
    start_byte: int
    end_byte: int
    replacement: str
    purpose: str = ""


@dataclass
class TransformPlan:
    edits: list[Edit] = field(default_factory=list)
    inline_added_for: list[str] = field(default_factory=list)  # candidate.name
    extern_decls_dropped: dict[Path, list[str]] = field(default_factory=dict)
    use_imports_added: dict[Path, list[str]] = field(default_factory=dict)


# ─────────────────────────────────────────────────────────────────
# E1 — definition site
# ─────────────────────────────────────────────────────────────────

def _e1_lib_side_edits(cand: Candidate, file_src: bytes,
                        plan: TransformPlan) -> None:
    # Drop the no_mangle / linkage attribute lines (each is already
    # range-extended to include trailing newline).
    if cand.attr_no_mangle_range:
        plan.edits.append(Edit(
            file=cand.file,
            start_byte=cand.attr_no_mangle_range[0],
            end_byte=cand.attr_no_mangle_range[1],
            replacement="",
            purpose=f"E1 drop #[no_mangle] on {cand.name}",
        ))
    if cand.attr_linkage_range:
        plan.edits.append(Edit(
            file=cand.file,
            start_byte=cand.attr_linkage_range[0],
            end_byte=cand.attr_linkage_range[1],
            replacement="",
            purpose=f"E1 drop #[linkage] on {cand.name}",
        ))
    # Drop the `extern "C"` modifier including the trailing space.
    plan.edits.append(Edit(
        file=cand.file,
        start_byte=cand.extern_modifier_range[0],
        end_byte=cand.extern_modifier_range[1],
        replacement="",
        purpose=f"E1 drop extern \"C\" modifier on {cand.name}",
    ))
    # Promote to `pub` if the fn wasn't already public. c2rust translates
    # internal helpers as `unsafe extern "C" fn foo(...)` (no `pub`),
    # relying on extern "C" + #[no_mangle] for cross-TU linker resolve.
    # Once we strip extern "C", any binary_fix-generated `use lib::module
    # ::foo` in [[bin]] sources requires `pub` for `use` resolution.
    #
    # NOTE (2026-07-27): tried making this OFF to fix numeric-kernel wall-clock
    # regression (libopenaptx decode +3.9%). In ISOLATION (pure E1, no other
    # pass) keeping helpers private is neutral (−0.34%) while promoting to pub
    # costs +3.8% (external linkage stops LLVM folding the hot call tree). BUT
    # in the FULL pipeline (intra_ptr + deunsafe + idiomatize) the opposite
    # holds: private is +5.22% vs pub +3.88% — pub interacts with the body
    # passes and lands on a better global-inlining local optimum. The slowdown
    # is a chaotic {visibility × body-pass × LLVM-global-opt} interaction, not
    # a single knob; flipping pub alone made libopenaptx WORSE, so this stays
    # ON. The real fix is project-level opt-out for no-noalias-benefit numeric
    # kernels (skip the body-lift passes entirely). See memory:
    # project_stage_a_slowdown_attribution_2026_07_27.
    import os as _os
    # OFF by default (2026-07-27 boundary-cleanup): pub-promotion gives the
    # helper external linkage and breaks LLVM's inline-folding (measured
    # +3.87% on libopenaptx). Cross-module / [[bin]] users of a stripped
    # helper instead freeze at the per-fn cargo-check gate (kept extern "C",
    # safe). Set STAGE_A_PROMOTE_PUB=1 to restore the old always-promote.
    if not cand.has_pub and _os.environ.get("STAGE_A_PROMOTE_PUB") == "1":
        plan.edits.append(Edit(
            file=cand.file,
            start_byte=cand.fn_item_start,
            end_byte=cand.fn_item_start,
            replacement="pub ",
            purpose=f"E1 promote {cand.name} to pub (no extern \"C\" "
                    f"linker fallback after strip)",
        ))
    # Add #[inline] when eligible. Insertion point: just before the
    # attribute block (the no_mangle attr's start_byte if present, else
    # right before the function_item itself).
    #
    # **Internal fn safeguard (aggressive mode)**:
    # aggressive classify_mode lifts internal `extern "C" fn` (no
    # #[no_mangle], no #[linkage] — c2rust's translation of C `static`
    # helpers). For these, do NOT auto-attach #[inline] — they may be
    # cold paths (e.g. libcsv's csv_increase_buffer is a realloc-on-
    # full helper called from csv_parse's edge path). Forcing inline
    # bloats hot fns' icache footprint without any noalias-side win
    # (which is the whole point of lifting). Empirically (libcsv 2026-
    # 05-22): aggressive-with-inline retreats ours_post_A from 394 ms
    # to 441 ms (+12%) because csv_increase_buffer's panic / realloc
    # / memcpy body spills into csv_parse's hot region. Leaving the
    # decision to LLVM's cost-model preserves the noalias unlock that
    # aggressive mode is designed for, without forcing the cold
    # bloat. Public API fns (with #[no_mangle] / #[linkage]) still
    # get the hint — they're the boundary between FFI callers and
    # the lib, and inline-at-boundary is consistently beneficial.
    is_public_abi_export = (
        cand.attr_no_mangle_range is not None
        or cand.attr_linkage_range is not None
    )
    eligible_for_inline = (
        is_public_abi_export
        and not cand.attr_inline_present
        and not cand.attr_cold_present
        and 0 < cand.body_line_count < INLINE_HINT_MAX_LINES
    )
    if eligible_for_inline:
        insert_at = cand.item_range[0]
        if cand.attr_no_mangle_range:
            insert_at = min(insert_at, cand.attr_no_mangle_range[0])
        if cand.attr_linkage_range:
            insert_at = min(insert_at, cand.attr_linkage_range[0])
        # Preserve indentation: back-walk to start-of-line.
        ip = insert_at
        while ip > 0 and file_src[ip-1:ip] not in (b"\n", b"\r"):
            ip -= 1
        indent = file_src[ip:insert_at].decode("utf-8", errors="replace")
        plan.edits.append(Edit(
            file=cand.file,
            start_byte=insert_at,
            end_byte=insert_at,
            replacement=f"{indent}#[inline]\n" if not indent.strip() else "#[inline]\n",
            purpose=f"E1 add #[inline] for {cand.name}",
        ))
        plan.inline_added_for.append(cand.name)


# ─────────────────────────────────────────────────────────────────
# Extern-block declaration deletes + block-collapse
# ─────────────────────────────────────────────────────────────────

def _e1_extern_block_edits(blk: ExternBlock, lifted_names: set[str],
                            file_src: bytes, plan: TransformPlan) -> None:
    surviving = [d for d in blk.decls if d.name not in lifted_names]
    to_drop = [d for d in blk.decls if d.name in lifted_names]
    if not to_drop:
        return
    plan.extern_decls_dropped.setdefault(blk.file, []).extend(
        d.name for d in to_drop
    )
    if not surviving:
        # Drop the whole foreign_mod_item, plus trailing newline.
        s, e = blk.block_range
        if e < len(file_src) and file_src[e:e+1] == b"\n":
            e += 1
        plan.edits.append(Edit(
            file=blk.file, start_byte=s, end_byte=e, replacement="",
            purpose=f"E1 drop empty extern \"C\" {{}} block",
        ))
    else:
        # Drop just the matched declaration spans.
        for d in to_drop:
            plan.edits.append(Edit(
                file=blk.file, start_byte=d.range[0], end_byte=d.range[1],
                replacement="",
                purpose=f"E1 drop extern decl {d.name}",
            ))


# ─────────────────────────────────────────────────────────────────
# `use` injection for callers
# ─────────────────────────────────────────────────────────────────

def _module_path_for_target_from(report: AnalysisReport, target: Path,
                                  caller: Path) -> str:
    """Caller-relative path. lib-internal files use `crate::…`; [[bin]]
    files use `::<crate_name>::…`."""
    rel_t = target.resolve().relative_to(report.project_dir.resolve())
    parts = list(rel_t.with_suffix("").parts)
    target_path = "" if parts == ["lib"] else "::" + "::".join(parts)
    rel_c = caller.resolve().relative_to(report.project_dir.resolve())
    if rel_c.stem in report.bin_file_stems:
        return f"::{report.crate_name}{target_path}"
    return f"crate{target_path}"


_USE_LINE_RE = re.compile(rb'^[ \t]*use\s', re.MULTILINE)


def _use_import_edits(report: AnalysisReport, lifted: list[Candidate],
                       plan: TransformPlan, file_srcs: dict[Path, bytes]) -> None:
    """For every caller file of a lifted fn, ensure a `use` import is
    present. We aggregate per-file then emit ONE insertion edit per file
    after the last existing `use` line."""
    name_to_target = {c.name: c.file for c in lifted}
    # caller_file -> ordered list of fn names needed
    per_caller: dict[Path, list[str]] = {}
    for cand in lifted:
        callers = report.cross_file_call_sites.get(cand.name, set())
        for cf in callers:
            per_caller.setdefault(cf, []).append(cand.name)

    for cf, names in per_caller.items():
        src = file_srcs.get(cf)
        if src is None:
            src = cf.read_bytes()
            file_srcs[cf] = src
        # Filter out names already imported.
        seen: list[str] = []
        for name in sorted(set(names)):
            target = name_to_target[name]
            mod = _module_path_for_target_from(report, target, cf)
            stmt = f"use {mod}::{name};"
            if stmt.encode() in src or f"::{name};".encode() in src:
                continue
            seen.append(stmt)
        if not seen:
            continue
        # Insertion = end of last existing `use ...` line in the first
        # ~6000 bytes (covers c2rust prelude).
        head = src[:6000]
        last = None
        for m in _USE_LINE_RE.finditer(head):
            # Find end-of-line for this use statement.
            nl = src.find(b"\n", m.start())
            if nl < 0:
                nl = len(src)
            last = nl + 1
        insert_at = last if last is not None else 0
        plan.edits.append(Edit(
            file=cf, start_byte=insert_at, end_byte=insert_at,
            replacement="".join(s + "\n" for s in seen),
            purpose=f"E1 inject use for {len(seen)} cross-file callers",
        ))
        plan.use_imports_added.setdefault(cf, []).extend(seen)


# ─────────────────────────────────────────────────────────────────
# Top-level — build the full TransformPlan
# ─────────────────────────────────────────────────────────────────

def build_plan(report: AnalysisReport,
               lifted: list[Candidate] | None = None) -> TransformPlan:
    if lifted is None:
        lifted = report.safe_to_strip
    plan = TransformPlan()
    file_srcs: dict[Path, bytes] = {}

    # Cache file bytes for every file we'll touch.
    for c in lifted:
        if c.file not in file_srcs:
            file_srcs[c.file] = c.file.read_bytes()
    for blk in report.internal_extern_blocks:
        if blk.file not in file_srcs:
            file_srcs[blk.file] = blk.file.read_bytes()

    # 1. Definition-site edits.
    for cand in lifted:
        _e1_lib_side_edits(cand, file_srcs[cand.file], plan)

    # 2. Extern-block edits.
    lifted_names = {c.name for c in lifted}
    for blk in report.internal_extern_blocks:
        _e1_extern_block_edits(blk, lifted_names, file_srcs[blk.file], plan)

    # 3. Cross-file use-import edits.
    _use_import_edits(report, lifted, plan, file_srcs)

    return plan


# ─────────────────────────────────────────────────────────────────
# Apply — per-file right-to-left
# ─────────────────────────────────────────────────────────────────

def apply_plan(plan: TransformPlan) -> dict[Path, int]:
    """Apply every Edit. Returns {file: num_edits_applied} for reporting.

    Atomicity: we group by file, sort edits right-to-left within a file,
    splice into the file's bytes, then write the file. If two edits
    overlap we raise — overlap means the plan was malformed and we must
    not write anything for that file."""
    by_file: dict[Path, list[Edit]] = {}
    for e in plan.edits:
        by_file.setdefault(e.file, []).append(e)

    counts: dict[Path, int] = {}
    for f, edits in by_file.items():
        # Sort by start_byte descending so later edits apply first.
        edits_sorted = sorted(edits, key=lambda e: -e.start_byte)
        # Overlap check.
        for i in range(len(edits_sorted) - 1):
            a = edits_sorted[i]
            b = edits_sorted[i+1]
            # a starts later or at same as b. They overlap if a.start < b.end.
            if a.start_byte < b.end_byte and not (
                a.start_byte == a.end_byte == b.start_byte == b.end_byte
            ):
                # Pure-insertion at same point is fine.
                if not (a.start_byte == a.end_byte
                        or b.start_byte == b.end_byte):
                    raise RuntimeError(
                        f"overlapping edits in {f}: {a} vs {b}"
                    )
        src = f.read_bytes()
        for e in edits_sorted:
            src = src[:e.start_byte] + e.replacement.encode() + src[e.end_byte:]
        f.write_bytes(src)
        counts[f] = len(edits)
    return counts
