"""Fail-closed extraction of bounded Rust syntax regions."""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from dataclasses import dataclass
from enum import Enum
import hashlib
from pathlib import Path
from typing import TYPE_CHECKING, Any

from tree_sitter import Node

from ..changeset.types import RegionKind, RegionRef, SymbolKind, SymbolRef
from .cst import (
    classify_node as _classify_node,
    get_parser as _get_parser,
    named_function_items as _named_function_items,
    same_node as _same_node,
    valid_function_span_start as _valid_function_span_start,
    walk as _walk,
)
from .model import (
    RuleCapability,
    hit_capability,
    rule_capability,
    stable_hit_id,
)

if TYPE_CHECKING:
    from ..rewrite_applier import EditTarget

# tree-sitter-rust loop forms; used to rank candidate windows by how many times
# their statements can run per call (see RegionExtractor._loop_depth).
_LOOP_NODE_TYPES = frozenset({
    "for_expression", "while_expression", "loop_expression",
})


@dataclass(frozen=True)
class RegionCandidate:
    region: RegionRef
    rule_ids: tuple[str, ...]


class RegionSkipReason(str, Enum):
    DUPLICATE_HIT = "duplicate_hit"
    UNRESOLVED_LINE = "unresolved_line"
    FILE_MISMATCH = "file_mismatch"
    CROSS_FN_REQUIRES_CHANGESET_V2 = "cross_fn_requires_changeset_v2"
    UNSUPPORTED_RULE = "unsupported_rule"
    REGION_UNPROVEN = "region_unproven"
    REGION_OVER_LIMIT_UNPROVEN = "region_over_limit_unproven"
    REGION_NOT_EDITABLE = "region_not_editable"


@dataclass(frozen=True)
class SkippedRegionHit:
    hit_id: str
    rule_id: str
    reason: RegionSkipReason

    def __post_init__(self) -> None:
        if not isinstance(self.reason, RegionSkipReason):
            raise TypeError("reason must be a RegionSkipReason")


@dataclass(frozen=True)
class RegionExtractionResult:
    candidates: tuple[RegionCandidate, ...]
    skipped: tuple[SkippedRegionHit, ...]


@dataclass(frozen=True)
class _PreparedHit:
    ordinal: int
    hit_id: str
    rule_id: str
    line: int
    hit: Mapping[str, Any]
    matches_target: bool


@dataclass(frozen=True)
class _Anchor:
    ordinal: int
    hit_id: str
    rule_id: str
    line: int


@dataclass
class _NodeGroup:
    node: Node
    region_kind: RegionKind
    anchors: list[_Anchor]


_NodeKey = tuple[str, int, int]


@dataclass(frozen=True)
class _CstIndex:
    comment_ranges_by_line: Mapping[int, tuple[tuple[int, int], ...]]
    named_child_indices: Mapping[_NodeKey, int]


@dataclass(frozen=True)
class _SourceContext:
    source: bytes
    relative_path: str
    target_file: Path
    function: Node
    body: Node
    symbol: SymbolRef
    line_starts: tuple[int, ...]
    cst_index: _CstIndex


class RegionExtractor:
    def __init__(
        self,
        max_lines: int = 400,
        max_bytes: int = 48 * 1024,
        max_sibling_gap_lines: int = 3,
        window_target_lines: int = 250,
        window_pad_lines: int = 30,
        max_hits_per_window: int | None = 25,
        min_window_lines: int = 8,
    ) -> None:
        if not _is_plain_int(max_lines) or not _is_plain_int(max_bytes):
            raise TypeError("max_lines and max_bytes must be integers")
        if not _is_plain_int(max_sibling_gap_lines):
            raise TypeError("max_sibling_gap_lines must be an integer")
        if not _is_plain_int(window_target_lines):
            raise TypeError("window_target_lines must be an integer")
        if not _is_plain_int(window_pad_lines):
            raise TypeError("window_pad_lines must be an integer")
        if max_lines <= 0 or max_bytes <= 0:
            raise ValueError("max_lines and max_bytes must be positive")
        if max_sibling_gap_lines < 0:
            raise ValueError("max_sibling_gap_lines must be non-negative")
        if window_target_lines <= 0:
            raise ValueError("window_target_lines must be positive")
        if window_pad_lines < 0:
            raise ValueError("window_pad_lines must be non-negative")
        self.max_lines = max_lines
        self.max_bytes = max_bytes
        self.max_sibling_gap_lines = max_sibling_gap_lines
        # Grow each located hit node into a ~window_target_lines span of
        # COMPLETE sibling statements around it, so a rule that spans several
        # statements (e.g. the c2rust 3-line cursor idiom
        #   `let fresh = p; p = p.offset(1); *fresh = x;`)
        # lands in ONE editable region instead of being cut mid-idiom. The
        # window is snapped to statement boundaries — never a raw line cut —
        # so the replacement stays a valid syntactic unit. The budget counts
        # NON-BLANK lines only (blank lines are free).
        #
        # 200 is chosen to sit just under the 250-line large-fn threshold
        # (agent._LARGE_FN_LINE_THRESHOLD): functions <=250 lines are rewritten
        # whole by the LLM, so ~250 lines is the empirical ceiling the LLM can
        # rewrite in one shot without the collateral-typo failure (F1). A 200
        # non-blank-line window stays under that ceiling while minimizing how
        # much a large fn gets fragmented.
        self.window_target_lines = window_target_lines
        self.window_pad_lines = window_pad_lines
        # Cap on how many hits one window may carry; None disables it.
        #
        # 25 is not a new number: it IS the hit count that sends a function
        # into REGION mode. Matching them is the whole point.
        #
        # REGION mode exists because a function has "too many interacting
        # rewrite sites" — the trigger counts HITS (>25). The split, however,
        # only ever counted LINES, so for a function shorter than the line
        # budget the split was a no-op: 49 hits in a 72-line function produced
        # one 64-line window, i.e. whole-fn rewriting under another name, and
        # a single shot at W2. Splitting by the same quantity that triggered
        # the mode is what makes the two agree.
        self.max_hits_per_window = max_hits_per_window
        # Windows below this are folded into a neighbour instead of becoming
        # their own candidate. 8 sits under the measured cliff (<=10 lines →
        # 71% abandoned) without touching the 11-40 line band that performs
        # best.
        self.min_window_lines = min_window_lines

    def extract(
        self,
        *,
        crate: Path,
        edit_target: EditTarget,
        hits: Sequence[Mapping[str, Any]],
    ) -> RegionExtractionResult:
        target_file = _resolve_target_file(crate, edit_target)
        prepared, skip_entries = self._prepare_hits(
            hits,
            edit_target,
            crate=crate,
            target_file=target_file,
        )
        if not prepared:
            return _result((), skip_entries)

        if target_file is None:
            skip_entries.extend(
                _skip(hit, RegionSkipReason.REGION_UNPROVEN) for hit in prepared
            )
            return _result((), skip_entries)

        matching: list[_PreparedHit] = []
        for hit in prepared:
            if not hit.matches_target:
                skip_entries.append(_skip(hit, RegionSkipReason.FILE_MISMATCH))
            else:
                matching.append(hit)
        if not matching:
            return _result((), skip_entries)

        context = _load_context(crate, edit_target, target_file)
        if context is None:
            skip_entries.extend(
                _skip(hit, RegionSkipReason.REGION_UNPROVEN) for hit in matching
            )
            return _result((), skip_entries)

        # Collect located hits as (statement, block, anchor). Instead of
        # expanding each hit into its OWN ~200-line window (which produced many
        # overlapping candidates that the per-fn budget then dropped — including
        # high-value ones like the LZ match loop), greedily CLUSTER nearby hits
        # within the same block into ONE padded window carrying ALL their rules.
        # This keeps the candidate count small (≈ one per rule-cluster) so the
        # budget no longer squeezes out a high-value rule, and gives the LLM full
        # bidirectional context around every rule.
        located: list[tuple] = []
        for hit in matching:
            loc = _locate_hit_node(context, hit)
            if loc is None:
                skip_entries.append(_skip(hit, RegionSkipReason.REGION_UNPROVEN))
                continue
            node, _kind = loc
            if _region_has_parse_error(node):
                skip_entries.append(_skip(hit, RegionSkipReason.REGION_UNPROVEN))
                continue
            stmt, block = _largest_enclosing_statement(
                node, context.body, context.source, self.window_target_lines
            )
            region_node = stmt if stmt is not None else node
            if _is_bare_expression_fragment(region_node, context.source):
                skip_entries.append(_skip(hit, RegionSkipReason.REGION_NOT_EDITABLE))
                continue
            anchor = _Anchor(hit.ordinal, hit.hit_id, hit.rule_id, hit.line)
            located.append((region_node, block, anchor))

        groups = self._cluster_groups(context, located)

        bounded: list[_NodeGroup] = []
        for group in sorted(groups, key=lambda item: item.node.start_byte):
            if _region_has_parse_error(group.node):
                skip_entries.extend(
                    (a.ordinal, SkippedRegionHit(
                        a.hit_id, a.rule_id, RegionSkipReason.REGION_UNPROVEN))
                    for a in group.anchors
                )
            elif self._fits(
                context.source, group.node.start_byte, group.node.end_byte
            ):
                bounded.append(group)
            else:
                skip_entries.extend(
                    (a.ordinal, SkippedRegionHit(
                        a.hit_id, a.rule_id,
                        RegionSkipReason.REGION_OVER_LIMIT_UNPROVEN))
                    for a in group.anchors
                )

        # Order candidates by how hot they can possibly be, NOT by file offset.
        #
        # This used to sort on `start_byte` descending — an edit-safety order
        # (rewrite later spans first so earlier offsets do not move) that was
        # doubling as a priority order.  It is not needed for safety: a
        # committed region sets `refresh_required` and the whole extraction is
        # redone against fresh bytes.  Used as a priority it is actively harmful
        # on a large function, because the per-fn token budget only funds the
        # first two or three candidates: on a ~4800-line state machine those
        # were the trailing wind-down statements, which run once per call, while
        # the hot loop in the middle of the function was never reached.
        #
        # Loop depth is the signal available for free here (no per-line profile
        # exists — attribution is per function), and it is the right one: a
        # statement outside every loop executes O(1) times per call and cannot
        # be the hot spot, whereas the body of a nested loop almost always is.
        # Anchor count breaks ties — a window several rules fired on carries
        # more to work with — and `start_byte` descending breaks the rest so the
        # order stays deterministic.
        ordered = sorted(
            bounded,
            key=lambda item: (
                -self._loop_depth(context, item.node),
                -len(item.anchors),
                -item.node.start_byte,
            ),
        )
        candidates = tuple(
            self._build_candidate(context, group) for group in ordered
        )
        return _result(candidates, skip_entries)

    @staticmethod
    def _loop_depth(context: _SourceContext, node: Node) -> int:
        """Deepest loop nesting any point of `node` sits at, within the fn body.

        Counts the loops enclosing the node plus the deepest loop nest *inside*
        it, so both "this window lives in the hot loop" and "this window *is*
        the hot loop" score above straight-line code.
        """
        # Degrades to 0 rather than raising on nodes that do not expose the full
        # tree-sitter surface: an unranked candidate still gets ordered, it just
        # falls back on the tie-breakers.
        depth = 0
        cursor = getattr(node, "parent", None)
        while cursor is not None and not _same_node(cursor, context.body):
            if getattr(cursor, "type", None) in _LOOP_NODE_TYPES:
                depth += 1
            cursor = getattr(cursor, "parent", None)

        deepest_inside = 0
        stack = [(node, 0)]
        while stack:
            current, inside = stack.pop()
            if getattr(current, "type", None) in _LOOP_NODE_TYPES:
                inside += 1
                deepest_inside = max(deepest_inside, inside)
            stack.extend(
                (child, inside) for child in getattr(current, "children", ())
            )
        return depth + deepest_inside

    def _prepare_hits(
        self,
        hits: Sequence[Mapping[str, Any]],
        edit_target: EditTarget,
        *,
        crate: Path,
        target_file: Path | None,
    ) -> tuple[list[_PreparedHit], list[tuple[int, SkippedRegionHit]]]:
        prepared: list[_PreparedHit] = []
        skipped: list[tuple[int, SkippedRegionHit]] = []
        seen: set[str] = set()
        raw_function_name = getattr(edit_target, "fn_name", None)
        function_name = (
            raw_function_name
            if isinstance(raw_function_name, str) and raw_function_name
            else None
        )
        canonical_path = (
            _canonical_relative_path(crate, target_file)
            if target_file is not None
            else None
        )
        for ordinal, hit in enumerate(hits):
            matches_target = (
                target_file is not None
                and _hit_matches_file(hit["file"], crate, target_file)
            )
            identity_hit = hit
            if matches_target and canonical_path is not None:
                identity_hit = dict(hit)
                identity_hit["file"] = canonical_path
            hit_id = stable_hit_id(identity_hit, function_name)
            rule_id = hit["rule"]
            line = hit["line"]
            base = _PreparedHit(
                ordinal,
                hit_id,
                rule_id,
                line,
                hit,
                matches_target,
            )
            if hit_id in seen:
                skipped.append(_skip(base, RegionSkipReason.DUPLICATE_HIT))
                continue
            seen.add(hit_id)
            capability = hit_capability(hit)
            if capability is not RuleCapability.REGION_LOCAL:
                reason = {
                    RuleCapability.CROSS_FUNCTION: (
                        RegionSkipReason.CROSS_FN_REQUIRES_CHANGESET_V2
                    ),
                    RuleCapability.UNSUPPORTED: RegionSkipReason.UNSUPPORTED_RULE,
                }[capability]
                skipped.append(_skip(base, reason))
                continue
            if line == 0:
                skipped.append(_skip(base, RegionSkipReason.UNRESOLVED_LINE))
                continue
            prepared.append(base)
        return prepared, skipped

    def _fits(self, source: bytes, start: int, end: int) -> bool:
        region = source[start:end]
        return len(region) <= self.max_bytes and region.count(b"\n") + 1 <= self.max_lines

    def _merge_statement_siblings(
        self,
        context: _SourceContext,
        groups: list[_NodeGroup],
    ) -> list[_NodeGroup]:
        if not groups:
            return []
        merged: list[_NodeGroup] = []
        current: list[_NodeGroup] = [groups[0]]
        for group in groups[1:]:
            previous = current[-1]
            can_join = _are_adjacent_statement_siblings(
                previous,
                group,
                self.max_sibling_gap_lines,
                context.cst_index.named_child_indices,
            )
            if can_join and self._fits(
                context.source,
                current[0].node.start_byte,
                group.node.end_byte,
            ):
                current.append(group)
                continue
            merged.append(_combine_groups(current))
            current = [group]
        merged.append(_combine_groups(current))
        return merged

    def _build_candidate(
        self,
        context: _SourceContext,
        group: _NodeGroup,
    ) -> RegionCandidate:
        anchors = sorted(group.anchors, key=lambda item: (item.line, item.hit_id))
        rule_ids = tuple(
            dict.fromkeys(anchor.rule_id for anchor in anchors)
        )
        parent = group.node.parent
        parent_kind = parent.type if parent is not None else "source_file"
        start = group.node.start_byte
        end = group.node.end_byte
        region = RegionRef(
            target_function=context.symbol,
            relative_path=context.relative_path,
            region_kind=group.region_kind,
            parent_kind=parent_kind,
            start_byte=start,
            end_byte=end,
            expected_region_hash=hashlib.sha256(
                context.source[start:end]
            ).hexdigest(),
            anchor_hit_ids=tuple(anchor.hit_id for anchor in anchors),
            anchor_lines=tuple(anchor.line for anchor in anchors),
        )
        return RegionCandidate(region=region, rule_ids=rule_ids)

    def _cluster_groups(
        self,
        context: _SourceContext,
        located: list[tuple],
    ) -> list[_NodeGroup]:
        """Greedily cluster located hits into padded sibling-sequence windows.

        ``located`` is a list of ``(statement_node, block_node, anchor)``. Hits
        that are sibling statements in the same block are grouped so that each
        cluster's core span (first→last hit) fits under
        ``window_target_lines - 2*window_pad_lines`` non-blank lines, then the
        window is padded by ``window_pad_lines`` on each side (capped at
        ``window_target_lines``). An error-containing sibling is a hard barrier:
        a window never spans or pads across it. Hits without a proven block
        become singleton regions.
        """
        groups: list[_NodeGroup] = []
        by_block: dict[tuple[int, int], tuple] = {}
        for stmt, block, anchor in located:
            if block is None:
                groups.append(_NodeGroup(
                    stmt, _classify_node(stmt) or RegionKind.STATEMENT, [anchor]))
                continue
            sibs = [child for child in block.named_children]
            idx = next(
                (i for i, child in enumerate(sibs)
                 if child.start_byte == stmt.start_byte
                 and child.end_byte == stmt.end_byte),
                None,
            )
            if idx is None:
                groups.append(_NodeGroup(
                    stmt, _classify_node(stmt) or RegionKind.STATEMENT, [anchor]))
                continue
            key = (block.start_byte, block.end_byte)
            entry = by_block.get(key)
            if entry is None:
                by_block[key] = (sibs, [(idx, anchor)])
            else:
                entry[1].append((idx, anchor))

        core_budget = max(1, self.window_target_lines - 2 * self.window_pad_lines)
        for sibs, hits in by_block.values():
            err = [_region_has_parse_error(child) for child in sibs]
            # A multi-node window becomes a NODE_SEQUENCE, and the resolver
            # only accepts a sequence whose members are ALL complete statements
            # (classify STATEMENT). A block's trailing tail expression (e.g.
            # `a + 2` with no semicolon, classify EXPRESSION) must therefore NOT
            # be swallowed into a sequence, or the whole region is rejected
            # downstream. Treat non-statement siblings as hard barriers next to
            # parse errors: cluster/pad never cross them. A hit landing ON such
            # a sibling still yields a single-node region carrying its true
            # classify kind (set below), which the resolver accepts.
            seq_barrier = [
                err[k] or _classify_node(child) is not RegionKind.STATEMENT
                for k, child in enumerate(sibs)
            ]

            def _nb(lo: int, hi: int, _s=sibs) -> int:
                chunk = context.source[_s[lo].start_byte : _s[hi].end_byte]
                return sum(1 for line in chunk.split(b"\n") if line.strip())

            ordered = sorted(hits, key=lambda t: t[0])
            # First pass: build the core [lo, hi] hit-cluster of each window,
            # then pad it. Record BOTH the padded [low, high] and the core
            # [lo, hi] so a later de-overlap pass can trim padding without ever
            # cutting a hit.
            windows: list[list] = []   # [low, high, core_lo, core_hi, anchors]
            i = 0
            while i < len(ordered):
                lo = hi = ordered[i][0]
                anchors = [ordered[i][1]]
                j = i + 1
                while j < len(ordered):
                    nxt = ordered[j][0]
                    if nxt == hi:                     # same sibling → merge anchor
                        anchors.append(ordered[j][1]); j += 1; continue
                    if any(seq_barrier[k] for k in range(hi + 1, nxt + 1)):
                        break                         # error / non-stmt barrier
                    cap = self.max_hits_per_window
                    if (_nb(lo, nxt) <= core_budget
                            and (cap is None or len(anchors) < cap)):
                        hi = nxt
                        anchors.append(ordered[j][1])
                        j += 1
                    else:
                        break
                low, high = self._pad_range(sibs, lo, hi, seq_barrier, _nb)
                windows.append([low, high, lo, hi, anchors])
                i = j
            # Second pass: pad ranges of adjacent windows in the SAME block can
            # collide in the gap between their cores (each side pads ~30 lines
            # inward), producing byte-OVERLAPPING candidate regions. Two
            # overlapping regions poison each other during the sequential
            # apply/rollback loop (the second one's ref goes stale → resolve
            # fails → the whole candidate is dropped). Split the overlap at the
            # gap midpoint, clamped so neither window's core hits are cut.
            for k in range(len(windows) - 1):
                a, b = windows[k], windows[k + 1]
                if a[1] >= b[0]:                       # padded ranges overlap
                    core_hi_a, core_lo_b = a[3], b[2]  # cores never overlap
                    x = (core_hi_a + core_lo_b) // 2
                    x = max(core_hi_a, min(x, core_lo_b - 1))
                    a[1] = min(a[1], x)                # a keeps [.., x]
                    b[0] = max(b[0], x + 1)            # b keeps [x+1, ..]
            # Third pass: a split can leave a runt — the tail of a capped
            # window, or a lone hit stranded past a barrier. Such a window is
            # too small to rewrite (a one-line `k = k + 1` offers nothing to
            # optimise) but still costs a full build + W1 + W2 to find that
            # out, which on a slow crate is tens of minutes. Measured across
            # two crates: windows of <=10 non-blank lines were abandoned 71%
            # of the time, versus 19-20% for larger ones.
            #
            # So fold a runt into the neighbour it is adjacent to, keeping its
            # hits rather than dropping them. Merging is only allowed while
            # the result stays inside the line budget and no barrier sits
            # between the two — the constraints that governed the split.
            if len(windows) > 1:
                merged: list = []
                for win in windows:
                    low, high, clo, chi, anchors = win
                    span = _nb(low, high)
                    cap = self.max_hits_per_window
                    # Only a REMAINDER is folded back: the previous window
                    # stopped because it hit the cap, so this one is what was
                    # left over. Windows that arose naturally — split by a
                    # barrier or by the line budget — are each about their own
                    # region and must stay separate however small they are.
                    is_remainder = (
                        merged and cap is not None and len(merged[-1][4]) >= cap
                    )
                    if is_remainder and span < self.min_window_lines:
                        prev = merged[-1]
                        gap_clear = not any(
                            seq_barrier[k] for k in range(prev[1] + 1, low + 1))
                        if gap_clear and _nb(prev[0], high) <= self.window_target_lines:
                            prev[1] = high            # extend padded range
                            prev[3] = chi             # extend core
                            prev[4].extend(anchors)   # keep every hit
                            continue
                    merged.append(win)
                windows = merged

            for low, high, _clo, _chi, anchors in windows:
                if low == high:
                    node = sibs[low]
                    # Single node → use its TRUE classify kind (STATEMENT /
                    # EXPRESSION / LOOP / MATCH_ARM / BLOCK) so the resolver,
                    # which validates by classify_node, always accepts it.
                    # Hard-coding STATEMENT rejected tail expressions.
                    kind = _classify_node(node) or RegionKind.STATEMENT
                else:
                    node = _SpanNode(sibs[low], sibs[high])
                    kind = RegionKind.NODE_SEQUENCE
                groups.append(_NodeGroup(node, kind, anchors))
        return groups

    def _pad_range(self, sibs, lo, hi, err, nb_fn) -> tuple[int, int]:
        """Grow [lo, hi] outward by ~window_pad_lines non-blank lines each side,
        capped at window_target_lines total; stop at block edges or an
        error-containing sibling (barrier)."""
        pad, cap = self.window_pad_lines, self.window_target_lines
        added = 0
        while hi + 1 < len(sibs) and not err[hi + 1]:
            inc = nb_fn(hi + 1, hi + 1)
            if added + inc > pad or nb_fn(lo, hi + 1) > cap:
                break
            added += inc
            hi += 1
        added = 0
        while lo - 1 >= 0 and not err[lo - 1]:
            inc = nb_fn(lo - 1, lo - 1)
            if added + inc > pad or nb_fn(lo - 1, hi) > cap:
                break
            added += inc
            lo -= 1
        return lo, hi


def _is_plain_int(value: object) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def _skip(
    hit: _PreparedHit,
    reason: RegionSkipReason,
) -> tuple[int, SkippedRegionHit]:
    return (
        hit.ordinal,
        SkippedRegionHit(hit.hit_id, hit.rule_id, reason),
    )


def _result(
    candidates: tuple[RegionCandidate, ...],
    skipped: list[tuple[int, SkippedRegionHit]],
) -> RegionExtractionResult:
    ordered_skips = tuple(
        item for _, item in sorted(skipped, key=lambda pair: pair[0])
    )
    return RegionExtractionResult(candidates=candidates, skipped=ordered_skips)


def _region_has_parse_error(node) -> bool:
    """True if any ERROR/MISSING node lies within the region's byte span.

    Drops only the individual regions that tree-sitter mis-parsed, while still
    editing the clean regions of a file with a stray parse error elsewhere
    (common in c2rust output). Handles both real tree-sitter nodes and the
    synthetic ``_SpanNode`` (a sibling-sequence span with no children): for the
    latter we scan the parent subtree and keep only errors inside [lo, hi].
    """
    lo, hi = node.start_byte, node.end_byte
    root = node if getattr(node, "children", None) is not None else \
        getattr(node, "parent", None)
    if root is None:
        return False
    stack = [root]
    while stack:
        current = stack.pop()
        if current.start_byte >= hi or current.end_byte <= lo:
            continue  # fully outside the region span
        if (getattr(current, "type", None) == "ERROR"
                or getattr(current, "is_missing", False)):
            if current.start_byte >= lo and current.end_byte <= hi:
                return True
        stack.extend(getattr(current, "children", ()) or ())
    return False


def _load_context(
    crate: Path,
    edit_target: EditTarget,
    target_file: Path,
) -> _SourceContext | None:
    try:
        crate_root = Path(crate).resolve(strict=True)
        if not crate_root.is_dir():
            return None
        target_file = Path(target_file).resolve(strict=True)
        relative_path = target_file.relative_to(crate_root).as_posix()
        if not target_file.is_file():
            return None
        source = target_file.read_bytes()
        start, end = edit_target.span
        if (
            not _is_plain_int(start)
            or not _is_plain_int(end)
            or start < 0
            or start >= end
            or end > len(source)
            or not isinstance(edit_target.fn_name, str)
            or not edit_target.fn_name
        ):
            return None
    except (AttributeError, OSError, TypeError, ValueError):
        return None

    try:
        root = _get_parser().parse(source).root_node
    except Exception as error:
        if isinstance(error, MemoryError):
            raise
        return None
    # NOTE: do NOT reject on `root.has_error`. c2rust output contains idioms
    # tree-sitter-rust mis-parses (e.g. `x as size_t <= y` — the `<` after a
    # type is read as generic-args), leaving a stray ERROR node somewhere in a
    # 10k+ line file. A parse error in an unrelated statement must not block
    # editing every function in the file. Per-region soundness is enforced
    # later: a candidate region containing an ERROR/MISSING node is dropped
    # (see `_region_has_parse_error` in `extract`).
    if root.is_missing:
        return None
    functions = [
        node
        for node in _named_function_items(root, source, edit_target.fn_name)
        if node.start_byte >= start
        and node.end_byte <= end
    ]
    if len(functions) != 1:
        return None
    function = functions[0]
    if function.end_byte != end or not _valid_function_span_start(function, start):
        return None
    body = function.child_by_field_name("body")
    # `body.has_error` is likewise tolerated — a mis-parsed statement elsewhere
    # in the body must not block clean regions; per-region gating handles it.
    if body is None or body.type != "block" or body.is_missing:
        return None
    declaration_hash = hashlib.sha256(source[start:end]).hexdigest()
    line_starts = _build_line_starts(source)
    cst_index = _build_cst_index(function)
    symbol = SymbolRef(
        qualified_name=edit_target.fn_name,
        symbol_kind=SymbolKind.FUNCTION,
        file_hint=relative_path,
        declaration_hash=declaration_hash,
    )
    return _SourceContext(
        source=source,
        relative_path=relative_path,
        target_file=target_file,
        function=function,
        body=body,
        symbol=symbol,
        line_starts=line_starts,
        cst_index=cst_index,
    )


def _resolve_target_file(crate: Path, edit_target: EditTarget) -> Path | None:
    try:
        crate_root = Path(crate).resolve(strict=True)
        if not crate_root.is_dir():
            return None
        requested = Path(edit_target.file)
        target_file = (
            requested if requested.is_absolute() else crate_root / requested
        ).resolve(strict=True)
        target_file.relative_to(crate_root)
        if not target_file.is_file():
            return None
        return target_file
    except (AttributeError, OSError, TypeError, ValueError):
        return None


def _canonical_relative_path(crate: Path, target_file: Path) -> str | None:
    try:
        crate_root = Path(crate).resolve(strict=True)
        return Path(target_file).resolve(strict=True).relative_to(crate_root).as_posix()
    except (OSError, TypeError, ValueError):
        return None


def _hit_matches_file(hit_file: object, crate: Path, target_file: Path) -> bool:
    try:
        crate_root = Path(crate).resolve(strict=True)
        candidate = Path(hit_file)
        resolved = (
            candidate if candidate.is_absolute() else crate_root / candidate
        ).resolve(strict=False)
        return resolved == target_file
    except (OSError, TypeError, ValueError):
        return False


def _locate_hit_node(
    context: _SourceContext,
    hit: _PreparedHit,
) -> tuple[Node, RegionKind] | None:
    line_range = _byte_range_for_line(
        context.source,
        context.line_starts,
        hit.line,
    )
    if line_range is None:
        return None
    line_start, line_end = line_range
    if line_start >= line_end:
        return None
    col = hit.hit.get("col", 0)
    if col == 0:
        position = next(
            (
                offset
                for offset in range(line_start, line_end)
                if context.source[offset] not in b" \t\r\n"
            ),
            None,
        )
        if position is None:
            return None
        initial = _named_node_at(context.function, position)
        if initial is None or _is_comment_node(initial):
            return None
    else:
        line_length = line_end - line_start
        if col > line_length:
            return None
        position = line_start + col - 1
        if context.source[position] & 0xC0 == 0x80:
            return None
        exact = _named_node_at(context.function, position)
        if exact is not None and _is_comment_node(exact):
            return None
        if context.source[position] in b" \t\r\n":
            segment = _comment_free_segment(
                position,
                line_start,
                line_end,
                context.cst_index.comment_ranges_by_line.get(hit.line - 1, ()),
            )
            if segment is None:
                return None
            initial = _nearest_named_syntax_node(
                context.function,
                context.source,
                position,
                segment[0],
                segment[1],
            )
        else:
            initial = exact
    if initial is None:
        return None
    if not (
        context.body.start_byte <= initial.start_byte
        and initial.end_byte <= context.body.end_byte
    ):
        return None
    node = initial
    while node is not None and node.type != "function_item":
        if (
            context.body.start_byte <= node.start_byte
            and node.end_byte <= context.body.end_byte
            and _node_is_proven(node, context.function)
        ):
            region_kind = _classify_node(node)
            if region_kind is not None:
                return node, region_kind
        if _same_node(node, context.body):
            break
        node = node.parent
    return None


def _is_bare_expression_fragment(node: Node, source: bytes) -> bool:
    """Is this region an expression fragment with no room to rewrite in?

    A region is the ONLY text the model may replace. Replacing a fragment can
    only produce another expression: there is nowhere to bind a local, nowhere
    to reorder, nowhere to put a slice view. C3's hoist and III④'s bounded view
    both need statement-level space, so a one-line expression region can only
    ever come back as an abstention. Measured on libxml2, two of them did —
    `(*(*ctxt).input)` for C3 and `*cur.offset(1 as c_int as isize)` for III④,
    16 and 32 bytes, one model call each, both declined for exactly this
    reason. Skipping them costs nothing that was reachable and saves the call.

    Two things separate these from the expressions worth editing. A block's
    tail expression — the `x` in `fn f() -> i32 { x }` — is a direct child of
    the block, so a rewrite can put statements before it and still return the
    value; it is small but not cramped. And an `unsafe { .. }` body, a
    multi-arm `if`, a `match` all classify as EXPRESSION while carrying whole
    statements inside them, so size rules them out too. What is left after both
    tests is the case with genuinely nowhere to write: a short expression
    nested inside a larger one.
    """
    if _classify_node(node) is not RegionKind.EXPRESSION:
        return False
    parent = node.parent
    if parent is not None and parent.type == "block":
        return False
    text = source[node.start_byte : node.end_byte]
    return sum(1 for line in text.split(b"\n") if line.strip()) < 2


def _largest_enclosing_statement(
    node: Node, body: Node, source: bytes, max_lines: int
) -> tuple[Node | None, Node | None]:
    """Return (statement, block) for the LARGEST ancestor statement that is a
    direct child of a block within the function body and whose line span is
    <= max_lines.

    This anchors a deeply-nested hit on the whole enclosing loop/block (up to
    the window limit) instead of on its innermost statement, so all hits inside
    one big loop cluster into ONE window carrying full context — the deflate
    match loop, its hash-chain walk, and their neighbouring de-unsafe sites end
    up in a single editable region rather than fragmented across nesting levels.

    When no block-child statement qualifies, the widest ancestor that is still
    a complete region and still inside the window is returned instead, with no
    block. This is not a rare corner. A hit deep inside a loop whose FIRST
    block-child ancestor already exceeds `max_lines` used to leave the caller
    holding the raw hit node, and the raw hit node is whatever `classify_node`
    accepted on the way up — `field_expression` and `type_cast_expression`
    both qualify. Measured on libxml2: two regions came out as
    `(*(*ctxt).input)` and `*cur.offset(1 as c_int as isize)`, 16 and 32 bytes,
    handed to the model as the only text it was allowed to edit. Both abstained,
    as they had to; there is nothing to optimise in a field access. Returning a
    statement-shaped ancestor gives the rewrite somewhere to happen, and the
    `_fits` check downstream still rejects it if it is too large to send.
    """
    best: tuple[Node, Node] | None = None
    widest: Node | None = None
    current: Node | None = node
    while current is not None and current.parent is not None:
        parent = current.parent
        # `classify_node` first: it is a type test, while the other two walk
        # bytes and subtrees.
        if _classify_node(current) is not None:
            span = source[current.start_byte : current.end_byte].count(b"\n") + 1
            if span <= max_lines and not _region_has_parse_error(current):
                widest = current
        if parent.type == "block" and (
            body.start_byte <= parent.start_byte
            and parent.end_byte <= body.end_byte
        ):
            span = source[current.start_byte : current.end_byte].count(b"\n") + 1
            # Grow only while the enclosing statement stays within the window
            # AND parses cleanly. A statement that swallows a stray ERROR (e.g.
            # the loop containing a c2rust `x as size_t <= y`) must not become
            # the anchor, or the whole window would be dropped downstream.
            if span <= max_lines and not _region_has_parse_error(current):
                best = (current, parent)
            else:
                break
        if _same_node(parent, body):
            break
        current = parent
    if best is not None:
        return best
    return (widest, None) if widest is not None else (None, None)


def _statement_in_block(node: Node, body: Node) -> tuple[Node | None, Node | None]:
    """Walk up from ``node`` to the statement that is a direct child of a
    ``block`` inside the function body. Returns (statement, block) or (None,
    None) if ``node`` is not nested in a proven in-body block."""
    current: Node | None = node
    while current is not None:
        parent = current.parent
        if parent is None:
            return None, None
        if (
            parent.type == "block"
            and body.start_byte <= parent.start_byte
            and parent.end_byte <= body.end_byte
        ):
            return current, parent
        if _same_node(current, body):
            return None, None
        current = parent
    return None, None


def _expand_to_window(
    context: _SourceContext,
    node: Node,
    region_kind: RegionKind,
    target_lines: int,
) -> tuple[Node, RegionKind]:
    """Grow ``node`` to a span of complete sibling statements around it, up to
    ``target_lines`` lines, snapped to statement boundaries. This keeps a
    multi-statement idiom (the c2rust 3-line cursor pattern) in ONE editable
    region instead of a single mid-idiom line. Falls back to ``node`` unchanged
    when it is not a statement nested in a proven block."""
    stmt, block = _statement_in_block(node, context.body)
    if stmt is None or block is None:
        return node, region_kind
    sibs = [child for child in block.named_children]
    index = next(
        (i for i, child in enumerate(sibs) if _same_node(child, stmt)),
        None,
    )
    if index is None or not sibs:
        return node, region_kind

    def _span_lines(lo: int, hi: int) -> int:
        # Count only NON-BLANK lines toward the window budget: blank /
        # whitespace-only lines are free, so the window holds ~target_lines of
        # actual code rather than being padded out by blank lines.
        chunk = context.source[sibs[lo].start_byte : sibs[hi].end_byte]
        return sum(1 for line in chunk.split(b"\n") if line.strip())

    low = high = index
    grew = True
    while grew:
        grew = False
        if high + 1 < len(sibs) and _span_lines(low, high + 1) <= target_lines:
            high += 1
            grew = True
        if low - 1 >= 0 and _span_lines(low - 1, high) <= target_lines:
            low -= 1
            grew = True
    if low == high:
        # A single statement is already a complete, proven region.
        return sibs[index], RegionKind.STATEMENT
    return _SpanNode(sibs[low], sibs[high]), RegionKind.NODE_SEQUENCE


def _build_line_starts(source: bytes) -> tuple[int, ...]:
    starts = [0]
    starts.extend(index + 1 for index, value in enumerate(source) if value == 10)
    return tuple(starts)


def _byte_range_for_line(
    source: bytes,
    starts: tuple[int, ...],
    line: int,
) -> tuple[int, int] | None:
    if line <= 0 or line > len(starts):
        return None
    start = starts[line - 1]
    end = starts[line] - 1 if line < len(starts) else len(source)
    if end > start and source[end - 1 : end] == b"\r":
        end -= 1
    return start, end


def _nearest_named_syntax_node(
    scope: Node,
    source: bytes,
    position: int,
    line_start: int,
    line_end: int,
) -> Node | None:
    maximum = max(position - line_start, line_end - position - 1)
    for distance in range(maximum + 1):
        positions = [position - distance]
        if distance:
            positions.append(position + distance)
        for candidate in positions:
            if candidate < line_start or candidate >= line_end:
                continue
            if source[candidate] in b" \t\r\n":
                continue
            node = _named_node_at(scope, candidate)
            if node is not None and not _is_comment_node(node):
                return node
    return None


def _comment_free_segment(
    position: int,
    line_start: int,
    line_end: int,
    comments: Sequence[tuple[int, int]],
) -> tuple[int, int] | None:
    segment_start = line_start
    segment_end = line_end
    for start, end in comments:
        comment_start = max(line_start, start)
        comment_end = min(line_end, end)
        if comment_start <= position < comment_end:
            return None
        if comment_end <= position:
            segment_start = max(segment_start, comment_end)
        elif comment_start > position:
            segment_end = min(segment_end, comment_start)
            break
    if segment_start >= segment_end:
        return None
    return segment_start, segment_end


def _named_node_at(scope: Node, position: int) -> Node | None:
    if position < scope.start_byte or position >= scope.end_byte:
        return None
    node = scope.descendant_for_byte_range(position, position + 1)
    while node is not None and not node.is_named:
        node = node.parent
    return node


def _is_comment_node(node: Node) -> bool:
    current: Node | None = node
    while current is not None:
        if "comment" in current.type:
            return True
        current = current.parent
    return False


def _node_is_proven(node: Node, function: Node) -> bool:
    current: Node | None = node
    while current is not None:
        # Reject only if an ancestor is ITSELF an error/missing node (a broken
        # path re-parents the node under garbage). Do NOT reject on
        # `has_error`: tree-sitter propagates that flag up from ANY descendant,
        # so a stray parse error elsewhere in the body (common in c2rust
        # output, e.g. `x as size_t <= y`) would otherwise mark every node in
        # the function unproven. The node's OWN subtree is validated separately
        # by `_region_has_parse_error`.
        if current.is_missing or current.type == "ERROR":
            return False
        if _same_node(current, function):
            return True
        current = current.parent
    return False


def _kind_priority(kind: RegionKind) -> int:
    return {
        RegionKind.EXPRESSION: 1,
        RegionKind.STATEMENT: 2,
        RegionKind.BLOCK: 3,
        RegionKind.LOOP: 4,
        RegionKind.MATCH_ARM: 5,
        RegionKind.NODE_SEQUENCE: 6,
    }[kind]


def _node_key(node: Node) -> _NodeKey:
    return node.type, node.start_byte, node.end_byte


def _build_cst_index(function: Node) -> _CstIndex:
    comments_by_line: dict[int, list[tuple[int, int]]] = {}
    named_child_indices: dict[_NodeKey, int] = {}
    for node in _walk(function):
        if "comment" in node.type:
            span = (node.start_byte, node.end_byte)
            for row in range(node.start_point.row, node.end_point.row + 1):
                comments_by_line.setdefault(row, []).append(span)
        for index, child in enumerate(node.named_children):
            named_child_indices[_node_key(child)] = index
    return _CstIndex(
        comment_ranges_by_line={
            row: tuple(sorted(spans)) for row, spans in comments_by_line.items()
        },
        named_child_indices=named_child_indices,
    )


def _parent_key(node: Node) -> tuple[str, int, int] | None:
    parent = node.parent
    if parent is None:
        return None
    return parent.type, parent.start_byte, parent.end_byte


def _are_adjacent_statement_siblings(
    left: _NodeGroup,
    right: _NodeGroup,
    max_gap: int,
    named_child_indices: Mapping[_NodeKey, int],
) -> bool:
    if (
        left.region_kind is not RegionKind.STATEMENT
        or right.region_kind is not RegionKind.STATEMENT
        or _parent_key(left.node) != _parent_key(right.node)
    ):
        return False
    left_index = named_child_indices.get(_node_key(left.node))
    right_index = named_child_indices.get(_node_key(right.node))
    if left_index is None or right_index != left_index + 1:
        return False
    line_distance = right.node.start_point.row - left.node.end_point.row
    return line_distance <= max_gap


def _combine_groups(groups: list[_NodeGroup]) -> _NodeGroup:
    if len(groups) == 1:
        return groups[0]
    first = groups[0]
    last = groups[-1]
    sequence_node = _SpanNode(first.node, last.node)
    anchors = [anchor for group in groups for anchor in group.anchors]
    return _NodeGroup(sequence_node, RegionKind.NODE_SEQUENCE, anchors)


class _SpanNode:
    """The exact span of a proven same-parent sibling sequence."""

    def __init__(self, first: Node, last: Node) -> None:
        self.type = "node_sequence"
        self.start_byte = first.start_byte
        self.end_byte = last.end_byte
        self.start_point = first.start_point
        self.end_point = last.end_point
        self.parent = first.parent


def _deduplicate_starts(
    groups: list[_NodeGroup],
    skipped: list[tuple[int, SkippedRegionHit]],
) -> list[_NodeGroup]:
    by_start: dict[int, _NodeGroup] = {}
    for group in groups:
        existing = by_start.get(group.node.start_byte)
        if existing is None:
            by_start[group.node.start_byte] = group
            continue
        if group.node.end_byte < existing.node.end_byte:
            kept, rejected = group, existing
            by_start[group.node.start_byte] = group
        else:
            kept, rejected = existing, group
        kept_ids = {anchor.hit_id for anchor in kept.anchors}
        for anchor in rejected.anchors:
            if anchor.hit_id not in kept_ids:
                skipped.append(
                    (
                        anchor.ordinal,
                        SkippedRegionHit(
                            anchor.hit_id,
                            anchor.rule_id,
                            RegionSkipReason.REGION_UNPROVEN,
                        ),
                    )
                )
    return list(by_start.values())
