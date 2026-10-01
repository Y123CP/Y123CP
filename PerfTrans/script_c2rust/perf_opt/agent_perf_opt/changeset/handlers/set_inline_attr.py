"""Set the `#[inline…]` attribute of one function definition (II_inl / II_iso).

The whole edit is one attribute line: an existing `#[inline…]` attribute on the
item is replaced, otherwise one is inserted directly above the item. Body,
signature and every other attribute are untouched, so the build gate and W1
are the semantic checks it needs, and W2 decides whether the codegen change
pays.

Located by name at resolve time rather than by planned byte offsets: earlier
commits in the same run shift offsets, and an attribute is valid wherever the
item currently sits. `post_validate` re-reads the file and checks the outcome
directly — exactly one inline attribute, the requested one — instead of
comparing absolute offsets, which a sibling edit in the same file would move.
"""

from __future__ import annotations

import re
from pathlib import Path

from perf_opt.hot_probe.symbol_source import _exported_symbol, _iter_function_items

from ..resolver import ResolutionRejected, resolve_project_path, sha256_bytes
from ..types import (
    ChangeSetStatus,
    ConcreteEdit,
    OperationResolution,
    SetFunctionInlineAttr,
    ValidationResult,
)

_INLINE_ATTR_RE = re.compile(rb"^#\[\s*inline\b")


def _parse(data: bytes):
    from tree_sitter import Language, Parser
    import tree_sitter_rust
    return Parser(Language(tree_sitter_rust.language())).parse(data)


def _squash(text: bytes | str) -> str:
    if isinstance(text, bytes):
        text = text.decode("utf-8", "replace")
    return re.sub(r"\s+", "", text)


def locate_inline_site(data: bytes, fn_name: str, line_hint: int):
    """The definition named `fn_name` nearest `line_hint`.

    Returns ``(fn_node, name_node, inline_attr_nodes, n_definitions)`` or None.
    `fn_name` may be the source name or the `#[export_name]` link name, the
    same two spellings `_locate_fn_node` accepts.
    """
    tree = _parse(data)
    candidates = []
    for node in _iter_function_items(tree.root_node):
        name_node = node.child_by_field_name("name")
        if name_node is None:
            continue
        if (name_node.text.decode("utf-8", "replace") != fn_name
                and _exported_symbol(node, data) != fn_name):
            continue
        candidates.append((abs(node.start_point[0] + 1 - line_hint), node, name_node))
    if not candidates:
        return None
    candidates.sort(key=lambda t: t[0])
    _, node, name_node = candidates[0]
    inline_attrs = []
    prev = node.prev_sibling
    while prev is not None and prev.type in (
        "attribute_item", "line_comment", "block_comment",
    ):
        if (prev.type == "attribute_item"
                and _INLINE_ATTR_RE.match(data[prev.start_byte:prev.end_byte])):
            inline_attrs.append(prev)
        prev = prev.prev_sibling
    return node, name_node, inline_attrs, len(candidates)


class SetFunctionInlineAttrHandler:
    def resolve(self, operation: SetFunctionInlineAttr, crate: Path) -> OperationResolution:
        path = resolve_project_path(crate, operation.relative_path)
        data = path.read_bytes()
        site = locate_inline_site(data, operation.fn_name, operation.line_hint)
        if site is None:
            raise ResolutionRejected(
                ChangeSetStatus.REJECTED_STALE,
                f"function {operation.fn_name} not found in {operation.relative_path}",
            )
        node, name_node, inline_attrs, n_defs = site
        desired = f"#[{operation.attribute}]"
        if len(inline_attrs) > 1:
            raise ResolutionRejected(
                ChangeSetStatus.REJECTED_CONFLICT,
                f"{operation.fn_name} carries {len(inline_attrs)} inline attributes",
            )
        if inline_attrs:
            existing = inline_attrs[0]
            if _squash(data[existing.start_byte:existing.end_byte]) == _squash(desired):
                raise ResolutionRejected(
                    ChangeSetStatus.REJECTED_CONFLICT,
                    f"{operation.fn_name} already carries {desired}",
                )
            start, end, text = existing.start_byte, existing.end_byte, desired
        else:
            # Replace the item's head (visibility/qualifiers through the name)
            # with the attribute plus that same head, keeping its indentation.
            # A non-empty span is required by the edit model; the head is the
            # smallest span that pins the insertion to this very item.
            line_start = data.rfind(b"\n", 0, node.start_byte) + 1
            indent = data[line_start:node.start_byte]
            indent_text = indent.decode("utf-8", "replace") if not indent.strip() else ""
            start, end = node.start_byte, name_node.end_byte
            text = desired + "\n" + indent_text + data[start:end].decode("utf-8", "replace")
        edit = ConcreteEdit(
            edit_id=f"edit-{operation.operation_id}",
            operation_id=operation.operation_id,
            relative_path=operation.relative_path,
            start_byte=start,
            end_byte=end,
            before_hash=sha256_bytes(data),
            replacement_text=text,
        )
        return OperationResolution(operation.operation_id, (edit,), facts={
            "relative_path": operation.relative_path,
            "fn_name": operation.fn_name,
            "attribute": desired,
            "replaced_existing": bool(inline_attrs),
            "n_definitions": n_defs,
        })

    def pre_validate(self, operation: SetFunctionInlineAttr, resolution,
                     crate: Path) -> ValidationResult:
        path = resolve_project_path(crate, operation.relative_path)
        try:
            data = path.read_bytes()
        except OSError as exc:
            return ValidationResult(False, "inline_attr_file_unreadable", str(exc))
        if sha256_bytes(data) != resolution.edits[0].before_hash:
            return ValidationResult(False, "inline_attr_file_stale")
        return ValidationResult(True, "inline_attr_target_fresh")

    def post_validate(self, operation: SetFunctionInlineAttr, resolution,
                      crate: Path) -> ValidationResult:
        path = resolve_project_path(crate, operation.relative_path)
        try:
            data = path.read_bytes()
        except OSError as exc:
            return ValidationResult(False, "inline_attr_file_unreadable", str(exc))
        # The insertion adds one line above the item; search around both.
        site = locate_inline_site(data, operation.fn_name, operation.line_hint + 1)
        if site is None:
            return ValidationResult(False, "inline_attr_target_missing")
        _, _, inline_attrs, n_defs = site
        if n_defs != resolution.facts.get("n_definitions"):
            return ValidationResult(False, "inline_attr_definition_count_changed")
        if len(inline_attrs) != 1:
            return ValidationResult(
                False, "inline_attr_not_set",
                f"expected exactly one inline attribute, found {len(inline_attrs)}")
        got = inline_attrs[0]
        if _squash(data[got.start_byte:got.end_byte]) != _squash(resolution.facts["attribute"]):
            return ValidationResult(False, "inline_attr_wrong_value")
        return ValidationResult(True, "inline_attr_set")
