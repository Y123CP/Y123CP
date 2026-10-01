"""Fail-closed resolution and validation for source-region replacement."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import NoReturn, TypeGuard

from tree_sitter import Node

from ...regions.cst import (
    attached_function_spans,
    classify_node,
    exact_named_nodes,
    get_parser,
    named_function_items,
    same_node,
    walk,
)
from perf_opt.agent_perf_opt.rewrite_applier import find_index_derived_slice_views

from ..resolver import ResolutionRejected, resolve_project_path, sha256_bytes
from ..types import (
    ChangeOperation,
    ChangeSetStatus,
    ConcreteEdit,
    OperationResolution,
    RegionKind,
    RegionRef,
    ReplaceSourceRegion,
    ValidationResult,
)


_FORBIDDEN_REPLACEMENT_TYPES = {
    "mod_item",
    "impl_item",
    "trait_item",
    "foreign_mod_item",
    "extern_crate_declaration",
    "function_signature_item",
    "macro_definition",
}
_COMMENT_TYPES = frozenset({"line_comment", "block_comment"})
_POST_COUNTED_TYPES = _FORBIDDEN_REPLACEMENT_TYPES | {"function_item"}


@dataclass(frozen=True)
class ResolvedRegionRef:
    """One RegionRef proven against one immutable source snapshot."""

    path: Path
    source: bytes
    root: Node
    function: Node
    body: Node
    nodes: tuple[Node, ...]
    parent: Node
    declaration_span: tuple[int, int]
    target_name: str
    start_line: int
    end_line: int


def _error_within_span(root, lo: int, hi: int) -> bool:
    """True if any ERROR/MISSING node intersects the byte range [lo, hi).

    c2rust output contains idioms tree-sitter-rust mis-parses (e.g.
    ``x as size_t <= y`` — the ``<`` after a type is read as generic args),
    leaving stray ERROR nodes elsewhere in the file. A whole-file
    ``root.has_error`` check would then reject every region edit. We instead
    tolerate errors outside the region under consideration and reject only when
    the region we read/wrote itself intersects one. The cargo build gate
    backstops any residual malformed edit.
    """
    stack = [root]
    while stack:
        node = stack.pop()
        if node.end_byte <= lo or node.start_byte >= hi:
            continue  # disjoint from the region
        if node.type == "ERROR" or node.is_missing:
            return True
        stack.extend(node.children)
    return False


def resolve_region_ref(crate: Path, region: RegionRef) -> ResolvedRegionRef:
    """Resolve and fully prove a RegionRef without constructing an edit."""

    if not isinstance(region, RegionRef):
        raise TypeError("region must be a RegionRef")
    if (
        region.target_function.file_hint is not None
        and region.target_function.file_hint != region.relative_path
    ):
        _reject_stale("target file_hint does not match region path")
    try:
        path = resolve_project_path(crate, region.relative_path)
    except (OSError, TypeError, ValueError) as exc:
        _reject_stale(f"invalid region path: {exc}")
    if not path.is_file():
        _reject_stale(f"region file is not a regular file: {region.relative_path}")
    try:
        source = path.read_bytes()
    except OSError as exc:
        _reject_stale(f"cannot read region file: {exc}")
    try:
        source.decode("utf-8")
    except UnicodeDecodeError as exc:
        _reject_stale(f"region file is not valid UTF-8: {exc}")

    start = region.start_byte
    end = region.end_byte
    if not (0 <= start < end <= len(source)):
        _reject_stale("region byte range is outside the current file")
    try:
        root = get_parser().parse(source).root_node
    except Exception as exc:
        if isinstance(exc, MemoryError):
            raise
        raise ResolutionRejected(
            ChangeSetStatus.ABSTAINED_UNPROVEN,
            f"Rust parser could not prove the current file: {exc}",
        ) from exc
    if _error_within_span(root, region.start_byte, region.end_byte):
        raise ResolutionRejected(
            ChangeSetStatus.ABSTAINED_UNPROVEN,
            "region span contains CST ERROR or missing nodes",
        )

    target_name = region.target_function.qualified_name.rsplit("::", 1)[-1]
    containing = [
        node
        for node in named_function_items(root, source, target_name)
        if node.start_byte <= start and end <= node.end_byte
    ]
    if len(containing) != 1:
        _reject_stale(
            f"function identity at region is ambiguous: found {len(containing)}"
        )
    function = containing[0]
    body = function.child_by_field_name("body")
    if body is None:
        raise ResolutionRejected(
            ChangeSetStatus.ABSTAINED_UNPROVEN,
            "target function has no provable body",
        )
    declaration_matches = [
        span
        for span in attached_function_spans(function)
        if sha256_bytes(source[span[0] : span[1]])
        == region.target_function.declaration_hash
    ]
    if len(declaration_matches) != 1:
        _reject_stale(
            "function declaration hash did not match exactly one safe span"
        )

    actual_region_hash = sha256_bytes(source[start:end])
    if actual_region_hash != region.expected_region_hash:
        _reject_stale(
            "region hash is stale: "
            f"expected {region.expected_region_hash}, got {actual_region_hash}"
        )

    nodes = _resolve_region_nodes(function, region.region_kind, start, end)
    if nodes is None:
        _reject_stale("region is not the declared exact CST shape")
    parent = nodes[0].parent
    if parent is None or any(node.parent != parent for node in nodes):
        _reject_stale("region nodes do not share one CST parent")
    if parent.type != region.parent_kind:
        _reject_stale(
            f"region parent changed: expected {region.parent_kind}, got {parent.type}"
        )

    start_line = source[:start].count(b"\n") + 1
    end_line = source[: end - 1].count(b"\n") + 1
    if any(line < start_line or line > end_line for line in region.anchor_lines):
        _reject_stale(
            f"anchor line is outside current region lines {start_line}-{end_line}"
        )
    return ResolvedRegionRef(
        path=path,
        source=source,
        root=root,
        function=function,
        body=body,
        nodes=nodes,
        parent=parent,
        declaration_span=declaration_matches[0],
        target_name=target_name,
        start_line=start_line,
        end_line=end_line,
    )


class ReplaceSourceRegionHandler:
    """Stateless handler for one already-proven ``RegionRef``."""

    def resolve(
        self, operation: ChangeOperation, crate: Path
    ) -> OperationResolution:
        if type(operation) is not ReplaceSourceRegion:
            raise TypeError("ReplaceSourceRegionHandler requires ReplaceSourceRegion")
        region = operation.region
        resolved = resolve_region_ref(crate, region)
        source = resolved.source
        start = region.start_byte
        end = region.end_byte
        edit = ConcreteEdit(
            edit_id=f"edit-{operation.operation_id}",
            operation_id=operation.operation_id,
            relative_path=region.relative_path,
            start_byte=start,
            end_byte=end,
            before_hash=sha256_bytes(source),
            replacement_text=operation.replacement_region_source,
        )
        facts = {
            "function_name": resolved.target_name,
            "function_span": [
                resolved.function.start_byte,
                resolved.function.end_byte,
            ],
            "declaration_span": list(resolved.declaration_span),
            "function_parent_fingerprint": _function_parent_fingerprint(
                resolved.function
            ),
            "region_kind": region.region_kind.value,
            "parent_kind": resolved.parent.type,
            "original_region_node_types": [
                node.type for node in resolved.nodes
            ],
            "original_region_macro_hashes": _macro_hashes_in_span(
                resolved.function,
                source,
                start,
                end,
            ),
            "prefix_sha256": sha256_bytes(source[:start]),
            "suffix_sha256": sha256_bytes(source[end:]),
            "original_file_length": len(source),
            "original_region_length": end - start,
            "region_line_bounds": [resolved.start_line, resolved.end_line],
            "original_item_counts": _item_counts(resolved.root),
        }
        return OperationResolution(operation.operation_id, (edit,), facts)

    def pre_validate(
        self,
        operation: ChangeOperation,
        resolution: OperationResolution,
        crate: Path,
    ) -> ValidationResult:
        mismatch = _validate_protocol(operation, resolution)
        if mismatch is not None:
            return mismatch
        assert isinstance(operation, ReplaceSourceRegion)
        try:
            operation.replacement_region_source.encode("utf-8")
        except UnicodeEncodeError as exc:
            return ValidationResult(False, "replace_region_utf8", str(exc))
        shape = validate_region_replacement_source(
            operation.region.region_kind,
            operation.replacement_region_source,
        )
        if not shape.ok:
            return shape
        macro_validation = _validate_replacement_macros(
            operation,
            resolution,
            post=False,
        )
        return macro_validation or shape

    def post_validate(
        self,
        operation: ChangeOperation,
        resolution: OperationResolution,
        crate: Path,
    ) -> ValidationResult:
        mismatch = _validate_protocol(operation, resolution)
        if mismatch is not None:
            return mismatch
        assert isinstance(operation, ReplaceSourceRegion)
        edit = resolution.edits[0]
        try:
            replacement = operation.replacement_region_source.encode("utf-8")
        except UnicodeEncodeError as exc:
            return ValidationResult(False, "replace_region_post_utf8", str(exc))
        try:
            path = resolve_project_path(crate, edit.relative_path)
            if not path.is_file():
                return ValidationResult(
                    False,
                    "replace_region_post_path",
                    "edited path is not a regular file",
                )
            source = path.read_bytes()
            source.decode("utf-8")
        except (OSError, TypeError, UnicodeDecodeError, ValueError) as exc:
            return ValidationResult(False, "replace_region_post_path", str(exc))

        replacement_end = edit.start_byte + len(replacement)
        if not (0 <= edit.start_byte <= replacement_end <= len(source)):
            return ValidationResult(
                False, "replace_region_post_span", "replacement span is out of bounds"
            )
        if source[edit.start_byte : replacement_end] != replacement:
            return ValidationResult(
                False,
                "replace_region_post_image",
                "target bytes do not equal the planned replacement",
            )
        identity = _validated_function_identity_facts(
            operation,
            resolution,
            edit,
        )
        if isinstance(identity, ValidationResult):
            return identity
        (
            original_function_span,
            original_declaration_span,
            original_region_length,
            original_file_length,
            function_parent_fingerprint,
        ) = identity
        if sha256_bytes(source[: edit.start_byte]) != resolution.facts.get(
            "prefix_sha256"
        ):
            return ValidationResult(
                False, "replace_region_post_prefix", "bytes before region changed"
            )
        if sha256_bytes(source[replacement_end:]) != resolution.facts.get(
            "suffix_sha256"
        ):
            return ValidationResult(
                False, "replace_region_post_suffix", "bytes after region changed"
            )
        expected_length = (
            original_file_length - original_region_length + len(replacement)
        )
        if expected_length != len(source):
            return ValidationResult(
                False,
                "replace_region_post_length",
                f"expected file length {expected_length}, got {len(source)}",
            )

        try:
            root = get_parser().parse(source).root_node
        except Exception as exc:
            if isinstance(exc, MemoryError):
                raise
            return ValidationResult(False, "replace_region_post_parse", str(exc))
        if _error_within_span(root, edit.start_byte, replacement_end):
            return ValidationResult(
                False,
                "replace_region_post_parse",
                "edited region contains CST ERROR or missing nodes",
            )
        macro_validation = _validate_replacement_macros(
            operation,
            resolution,
            post=True,
        )
        if macro_validation is not None:
            return macro_validation
        target_name = operation.region.target_function.qualified_name.rsplit("::", 1)[-1]
        delta = len(replacement) - original_region_length
        expected_function_span = (
            original_function_span[0],
            original_function_span[1] + delta,
        )
        expected_declaration_span = (
            original_declaration_span[0],
            original_declaration_span[1] + delta,
        )
        functions = [
            function
            for function in named_function_items(root, source, target_name)
            if (function.start_byte, function.end_byte) == expected_function_span
            and function.start_byte <= edit.start_byte
            and replacement_end <= function.end_byte
            and _function_parent_fingerprint(function)
            == function_parent_fingerprint
            and expected_declaration_span in attached_function_spans(function)
        ]
        if len(functions) != 1:
            return ValidationResult(
                False,
                "replace_region_post_function",
                "expected exactly one target function at its structural post span, "
                f"found {len(functions)}",
            )
        original_counts = resolution.facts.get("original_item_counts")
        if not isinstance(original_counts, dict) or any(
            not isinstance(original_counts.get(node_type), int)
            or isinstance(original_counts.get(node_type), bool)
            for node_type in _POST_COUNTED_TYPES
        ):
            return ValidationResult(
                False, "replace_region_post_facts", "missing original item counts"
            )
        current_counts = _item_counts(root)
        if any(
            current_counts[node_type] > original_counts.get(node_type, 0)
            for node_type in _POST_COUNTED_TYPES
        ):
            return ValidationResult(
                False,
                "replace_region_post_forbidden_item",
                "replacement introduced a function, module, impl, trait, or extern item",
            )

        if operation.replacement_region_source.strip():
            shape = validate_region_replacement_source(
                operation.region.region_kind,
                operation.replacement_region_source,
            )
            if not shape.ok:
                return ValidationResult(
                    False,
                    f"replace_region_post_{shape.code.removeprefix('replace_region_')}",
                    shape.detail,
                )
        elif operation.region.region_kind not in {
            RegionKind.STATEMENT,
            RegionKind.NODE_SEQUENCE,
        }:
            return ValidationResult(
                False,
                "replace_region_post_empty_not_allowed",
                "empty replacement is only valid for complete statements",
            )
        from perf_opt.agent_perf_opt.rewrite_applier import (
            describe_added_bounds_checks,
    find_added_bounds_checks_in_crate,
            find_introduced_scans_in_crate,
        )

        # A rewrite must not buy safety the original code never paid for.
        # Checked HERE, in post-validation, because this is the first moment
        # the applied source exists on disk: the predicate reads the working
        # tree's diff, and every earlier hook runs before `applier.apply`.
        #
        # Measured on one crate's hottest inner loop: two runs produced
        # rewrites of the SAME lines under the SAME rules, differing only in
        # this one expression. Same start commit, same session, CV < 0.1%:
        # `get_unchecked` gave -8.44%; `slice[i]` gave +0.01%; swapping just
        # that expression back recovered -7.55%.
        #
        # It cannot be left to the gates. W2 reads +0.01% as "no regression"
        # and commits it, and the committed region then blocks any better
        # rewrite of those lines for the rest of the run.
        added = find_added_bounds_checks_in_crate(crate)
        if added:
            return ValidationResult(
                False,
                "added_bounds_check_in_loop",
                describe_added_bounds_checks(added),
            )

        # A finding, not a verdict. An introduced length scan is a real cost —
        # measured +7.818% on one crate's `precompute_bonus`, where the
        # original's own loop already walked to the NUL — but it is not always
        # a net loss: replayed over 128 committed rewrites, four had one and
        # every one of them had measured a win, because what the rewrite did
        # with the resulting view paid for the extra traversal. Rejecting here
        # would have blocked those four. So record it and let W2 judge; if W2
        # rejects, `agent` has a diagnosis to hand back instead of a stopwatch
        # reading. Candidates that pass W2 never reach that path, so this
        # cannot cost a winner.
        scans = find_introduced_scans_in_crate(crate)
        if scans:
            return ValidationResult(
                True,
                "introduced_scan",
                "rewrite builds a view whose length must be scanned for, "
                "which the original did not do: " + ", ".join(scans[:3]),
            )
        return ValidationResult(True, "replace_region_post")


def _reject_stale(detail: str) -> NoReturn:
    raise ResolutionRejected(ChangeSetStatus.REJECTED_STALE, detail)


def _function_parent_fingerprint(function: Node) -> list[dict[str, object]]:
    """Describe the named-child path from the file root to one function."""

    reversed_path: list[dict[str, object]] = []
    current = function
    while current.parent is not None:
        parent = current.parent
        index = next(
            (
                index
                for index, child in enumerate(parent.named_children)
                if same_node(child, current)
            ),
            None,
        )
        if index is None:
            return []
        reversed_path.append({"type": current.type, "index": index})
        current = parent
    reversed_path.append({"type": current.type, "index": 0})
    reversed_path.reverse()
    return reversed_path


def _validated_function_identity_facts(
    operation: ReplaceSourceRegion,
    resolution: OperationResolution,
    edit: ConcreteEdit,
) -> (
    tuple[
        tuple[int, int],
        tuple[int, int],
        int,
        int,
        list[dict[str, object]],
    ]
    | ValidationResult
):
    function_span = _plain_span(resolution.facts.get("function_span"))
    declaration_span = _plain_span(resolution.facts.get("declaration_span"))
    original_region_length = resolution.facts.get("original_region_length")
    original_file_length = resolution.facts.get("original_file_length")
    fingerprint = resolution.facts.get("function_parent_fingerprint")
    target_name = operation.region.target_function.qualified_name.rsplit("::", 1)[-1]
    if (
        function_span is None
        or declaration_span is None
        or not _is_plain_int(original_region_length)
        or original_region_length <= 0
        or original_region_length != edit.end_byte - edit.start_byte
        or not _is_plain_int(original_file_length)
        or original_file_length < function_span[1]
        or not _valid_parent_fingerprint(fingerprint)
        or resolution.facts.get("function_name") != target_name
        or declaration_span[0] > function_span[0]
        or declaration_span[1] != function_span[1]
        or not (
            function_span[0] <= edit.start_byte
            and edit.end_byte <= function_span[1]
        )
    ):
        return ValidationResult(
            False,
            "replace_region_post_facts",
            "function identity facts are missing, malformed, or inconsistent",
        )
    return (
        function_span,
        declaration_span,
        original_region_length,
        original_file_length,
        fingerprint,
    )


def _macro_hashes_in_span(
    root: Node,
    source: bytes,
    start: int,
    end: int,
) -> list[str]:
    return sorted(
        sha256_bytes(source[node.start_byte : node.end_byte])
        for node in walk(root)
        if node.type == "macro_invocation"
        and start <= node.start_byte
        and node.end_byte <= end
    )


def _validate_replacement_macros(
    operation: ReplaceSourceRegion,
    resolution: OperationResolution,
    *,
    post: bool,
) -> ValidationResult | None:
    original = resolution.facts.get("original_region_macro_hashes")
    facts_code = "replace_region_post_facts" if post else "replace_region_macro_facts"
    changed_code = (
        "replace_region_post_macro_changed" if post else "replace_region_macro_changed"
    )
    if not _valid_sha256_multiset(original):
        return ValidationResult(
            False,
            facts_code,
            "original region macro hashes are missing or malformed",
        )
    replacement_hashes = _replacement_macro_hashes(
        operation.region.region_kind,
        operation.replacement_region_source,
    )
    if isinstance(replacement_hashes, ValidationResult):
        return ValidationResult(
            False,
            f"replace_region_{'post_' if post else ''}macro_parse",
            replacement_hashes.detail,
        )
    if replacement_hashes != original:
        return ValidationResult(
            False,
            changed_code,
            "replacement must preserve the exact macro invocation multiset",
        )
    return None


def _first_cst_error(root, wrapped: bytes, prefix_len: int) -> str:
    """Name the first place the replacement stopped parsing.

    `has_error` propagates up from any descendant, so the root says only that
    something is wrong somewhere. What comes back to the model without this is
    "replacement wrapper has CST errors" and nothing else — it cannot tell a
    stray brace from a half-written expression, and the reply it sends next is
    a guess. Offsets are reported against the replacement, not the wrapper, so
    they line up with the text the model actually wrote.
    """
    stack = [root]
    while stack:
        node = stack.pop()
        if node.type == "ERROR" or node.is_missing:
            offset = max(0, node.start_byte - prefix_len)
            excerpt = wrapped[node.start_byte:node.end_byte][:80]
            shape = "missing token" if node.is_missing else "unparsed"
            return (f"{shape} at byte {offset} of the replacement: "
                    f"{excerpt.decode('utf-8', 'replace')!r}")
        stack.extend(reversed(node.children))
    return "parse error not localised to a node"


def _replacement_macro_hashes(
    kind: RegionKind,
    replacement: str,
) -> list[str] | ValidationResult:
    stripped = replacement.strip()
    if not stripped:
        return []
    try:
        fragment = stripped.encode("utf-8")
    except UnicodeEncodeError as exc:
        return ValidationResult(False, "replace_region_utf8", str(exc))
    prefix, suffix = _replacement_wrapper(kind)
    wrapped = prefix + fragment + suffix
    try:
        root = get_parser().parse(wrapped).root_node
    except Exception as exc:
        if isinstance(exc, MemoryError):
            raise
        return ValidationResult(False, "replace_region_parse", str(exc))
    if root.has_error or root.is_missing:
        return ValidationResult(
            False,
            "replace_region_parse",
            _first_cst_error(root, wrapped, len(prefix)),
        )
    start = len(prefix)
    end = start + len(fragment)
    return _macro_hashes_in_span(root, wrapped, start, end)


def _valid_sha256_multiset(value: object) -> TypeGuard[list[str]]:
    return (
        isinstance(value, list)
        and value == sorted(value)
        and all(
            isinstance(item, str)
            and len(item) == 64
            and all(char in "0123456789abcdef" for char in item)
            for item in value
        )
    )


def _plain_span(value: object) -> tuple[int, int] | None:
    if (
        not isinstance(value, list)
        or len(value) != 2
        or not all(_is_plain_int(item) for item in value)
        or value[0] < 0
        or value[0] >= value[1]
    ):
        return None
    return value[0], value[1]


def _valid_parent_fingerprint(
    value: object,
) -> TypeGuard[list[dict[str, object]]]:
    return (
        isinstance(value, list)
        and bool(value)
        and all(
            isinstance(entry, dict)
            and set(entry) == {"type", "index"}
            and isinstance(entry["type"], str)
            and bool(entry["type"])
            and _is_plain_int(entry["index"])
            and entry["index"] >= 0
            for entry in value
        )
    )


def _is_plain_int(value: object) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def _resolve_region_nodes(
    function: Node, kind: RegionKind, start: int, end: int
) -> tuple[Node, ...] | None:
    if kind is not RegionKind.NODE_SEQUENCE:
        matches = [
            node
            for node in exact_named_nodes(function, start, end)
            if classify_node(node) is kind
        ]
        return (matches[0],) if len(matches) == 1 else None

    candidates: list[tuple[Node, ...]] = []
    for parent in walk(function):
        children = parent.named_children
        for first_index, child in enumerate(children):
            if child.start_byte != start:
                continue
            sequence: list[Node] = []
            for sibling in children[first_index:]:
                if sibling.start_byte < start:
                    continue
                sequence.append(sibling)
                if sibling.end_byte >= end:
                    break
            if (
                len(sequence) >= 2
                and sequence[-1].end_byte == end
                and all(classify_node(node) is RegionKind.STATEMENT for node in sequence)
            ):
                candidates.append(tuple(sequence))
    return candidates[0] if len(candidates) == 1 else None


def _validate_protocol(
    operation: ChangeOperation, resolution: OperationResolution
) -> ValidationResult | None:
    if type(operation) is not ReplaceSourceRegion:
        return ValidationResult(
            False,
            "replace_region_resolution_mismatch",
            "handler requires ReplaceSourceRegion",
        )
    if resolution.operation_id != operation.operation_id or len(resolution.edits) != 1:
        return ValidationResult(
            False,
            "replace_region_resolution_mismatch",
            "resolution must contain exactly one edit for the operation",
        )
    edit = resolution.edits[0]
    region = operation.region
    if (
        edit.operation_id != operation.operation_id
        or edit.edit_id != f"edit-{operation.operation_id}"
        or edit.relative_path != region.relative_path
        or edit.start_byte != region.start_byte
        or edit.end_byte != region.end_byte
        or edit.replacement_text != operation.replacement_region_source
        or resolution.facts.get("region_kind") != region.region_kind.value
        or resolution.facts.get("parent_kind") != region.parent_kind
    ):
        return ValidationResult(
            False,
            "replace_region_resolution_mismatch",
            "resolution edit or facts do not match the operation",
        )
    return None


def validate_region_replacement_source(
    kind: RegionKind, replacement: str
) -> ValidationResult:
    """Validate a replacement fragment with the handler's canonical wrapper."""
    if not isinstance(kind, RegionKind):
        return ValidationResult(
            False,
            "replace_region_invalid_kind",
            "kind must be a RegionKind",
        )
    stripped = replacement.strip()
    if not stripped:
        if kind in {RegionKind.STATEMENT, RegionKind.NODE_SEQUENCE}:
            return ValidationResult(True, "replace_region_shape")
        return ValidationResult(
            False,
            "replace_region_empty_not_allowed",
            "empty replacement is only valid for complete statements",
        )
    try:
        fragment = stripped.encode("utf-8")
    except UnicodeEncodeError as exc:
        return ValidationResult(False, "replace_region_utf8", str(exc))

    prefix, suffix = _replacement_wrapper(kind)
    wrapped = prefix + fragment + suffix
    try:
        root = get_parser().parse(wrapped).root_node
    except Exception as exc:
        if isinstance(exc, MemoryError):
            raise
        return ValidationResult(False, "replace_region_parse", str(exc))
    if root.has_error or root.is_missing:
        return ValidationResult(
            False, "replace_region_parse",
            _first_cst_error(root, wrapped, len(prefix)),
        )

    nodes = tuple(walk(root))
    if sum(node.type == "function_item" for node in nodes) != 1 or any(
        node.type in _FORBIDDEN_REPLACEMENT_TYPES for node in nodes
    ):
        return ValidationResult(
            False,
            "replace_region_forbidden_item",
            "replacement contains a function, module, impl, trait, or extern item",
        )

    start = len(prefix)
    end = start + len(fragment)
    if kind is not RegionKind.NODE_SEQUENCE and kind is not RegionKind.STATEMENT:
        start, end = _trim_edge_comments(root, wrapped, start, end)
    exact = exact_named_nodes(root, start, end)
    if kind is RegionKind.EXPRESSION:
        valid = [node for node in exact if classify_node(node) is RegionKind.EXPRESSION]
        ok = len(valid) == 1
    elif kind is RegionKind.BLOCK:
        valid = [node for node in exact if node.type == "block"]
        ok = len(valid) == 1
    elif kind is RegionKind.LOOP:
        valid = [
            node
            for node in exact
            if node.type in {"loop_expression", "while_expression", "for_expression"}
        ]
        ok = len(valid) == 1
    elif kind is RegionKind.MATCH_ARM:
        valid = [node for node in exact if node.type == "match_arm"]
        ok = len(valid) == 1
    else:
        wrappers = [node for node in nodes if node.type == "function_item"]
        body = wrappers[0].child_by_field_name("body")
        spanned = (
            [
                node
                for node in body.named_children
                if start <= node.start_byte and node.end_byte <= end
            ]
            if body is not None
            else []
        )
        # A comment is a NAMED node but not a statement, so demanding that every
        # named child classify as STATEMENT rejects any rewrite that explains
        # itself between two statements. That is not a rare style: the cards
        # REQUIRE a `// SAFETY:` justification above each `unsafe` block, so the
        # gate punished the model for following its own instructions. Measured:
        # one such rejection cost a compression crate the single commit worth
        # -8.15% on its main operation, because the model happened to put the
        # comment at statement level rather than inside a call's argument list.
        #
        # Comments are still part of the replacement — they must be spliced in
        # with it — so they stay in the span/coverage check and are excluded
        # only from the "is this a statement" question.
        statements = [n for n in spanned if n.type not in _COMMENT_TYPES]
        minimum = 1 if kind is RegionKind.STATEMENT else 0
        ok = (
            len(statements) >= minimum
            and bool(spanned)
            and spanned[0].start_byte == start
            and spanned[-1].end_byte == end
            and all(
                classify_node(node) is RegionKind.STATEMENT for node in statements
            )
        )
    if not ok:
        return ValidationResult(
            False,
            "replace_region_shape",
            f"replacement is not a complete {kind.value} shape",
        )
    # Same refusal as the whole-function path. A fragment can hold the
    # subscript without the `let` that made the view; that is a miss, never a
    # false refusal.
    derived = find_index_derived_slice_views(wrapped.decode("utf-8", "replace"))
    if derived:
        return ValidationResult(
            False, "index_derived_slice_view", " | ".join(derived)[:2000])
    return ValidationResult(True, "replace_region_shape")



def _trim_edge_comments(
    root: Node, data: bytes, start: int, end: int
) -> tuple[int, int]:
    """Shrink ``[start, end)`` past any comment sitting at either edge.

    The single-node kinds ask for a node whose bytes are EXACTLY the
    replacement. A leading or trailing comment breaks that equality even
    though the replacement is perfectly well-formed — and the cards require a
    `// SAFETY:` line above every `unsafe`, so the model writing one is
    compliance, not noise. The comment still travels with the replacement when
    it is spliced in; it is only excluded from "which node is this".
    """
    comments = [node for node in walk(root) if node.type in _COMMENT_TYPES]
    moved = True
    while moved and start < end:
        moved = False
        while start < end and data[start : start + 1].isspace():
            start += 1
        for node in comments:
            if node.start_byte == start and node.end_byte <= end:
                start, moved = node.end_byte, True
                break
        while end > start and data[end - 1 : end].isspace():
            end -= 1
        for node in comments:
            if node.end_byte == end and node.start_byte >= start:
                end, moved = node.start_byte, True
                break
    return start, end


def _replacement_wrapper(kind: RegionKind) -> tuple[bytes, bytes]:
    # The suffix opens with a newline so a replacement ending in a `//` comment
    # cannot swallow the closing delimiter — `… // note}` comments out the `}`
    # and the wrapper fails to parse, which surfaces as "wrapper has CST
    # errors" and looks like the model produced broken Rust when it did not.
    if kind is RegionKind.EXPRESSION:
        return b"fn __region_wrapper(){let __value = (", b"\n);}"
    if kind is RegionKind.MATCH_ARM:
        return b"fn __region_wrapper(__x:i32)->i32{match __x{", b"\n}}"
    return b"fn __region_wrapper(){", b"\n}"


def _item_counts(root: Node) -> dict[str, int]:
    counts = {node_type: 0 for node_type in sorted(_POST_COUNTED_TYPES)}
    for node in walk(root):
        if node.type in counts:
            counts[node.type] += 1
    return counts
