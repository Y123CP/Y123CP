"""Shared, iterative Rust CST primitives for extraction and validation."""

from __future__ import annotations

from collections.abc import Iterator
from functools import lru_cache

from tree_sitter import Language, Node, Parser
import tree_sitter_rust

from ..changeset.types import RegionKind


@lru_cache(maxsize=1)
def get_parser() -> Parser:
    """Return the process-wide parser for the locked Rust grammar."""

    return Parser(Language(tree_sitter_rust.language()))


def walk(node: Node) -> Iterator[Node]:
    """Walk a CST without consuming Python recursion depth."""

    stack = [node]
    while stack:
        current = stack.pop()
        yield current
        stack.extend(reversed(current.children))


def same_node(left: Node, right: Node) -> bool:
    return (
        left.type == right.type
        and left.start_byte == right.start_byte
        and left.end_byte == right.end_byte
    )


def function_name(source: bytes, function: Node) -> str | None:
    name = function.child_by_field_name("name")
    if name is None:
        return None
    try:
        return source[name.start_byte : name.end_byte].decode("utf-8")
    except UnicodeDecodeError:
        return None


def named_function_items(root: Node, source: bytes, name: str) -> tuple[Node, ...]:
    """Locate all Rust function items with the exact source-level name."""

    return tuple(
        node
        for node in walk(root)
        if node.type == "function_item" and function_name(source, node) == name
    )


_ATTACHED_FUNCTION_NODE_TYPES = {
    "attribute_item",
    "line_comment",
    "block_comment",
}


def attached_function_spans(function: Node) -> tuple[tuple[int, int], ...]:
    """Enumerate safe declaration spans, from the function through attachments."""

    spans = [(function.start_byte, function.end_byte)]
    parent = function.parent
    if parent is None:
        return tuple(spans)
    siblings = parent.named_children
    index = next(
        (index for index, sibling in enumerate(siblings) if same_node(sibling, function)),
        None,
    )
    if index is None:
        return tuple(spans)
    index -= 1
    while index >= 0 and siblings[index].type in _ATTACHED_FUNCTION_NODE_TYPES:
        sibling = siblings[index]
        spans.append((sibling.start_byte, function.end_byte))
        index -= 1
    return tuple(spans)


def valid_function_span_start(function: Node, span_start: int) -> bool:
    return any(
        start == span_start for start, _ in attached_function_spans(function)
    )


def classify_node(node: Node) -> RegionKind | None:
    """Classify only complete region-safe named Rust nodes."""

    if node.type in {"loop_expression", "while_expression", "for_expression"}:
        return RegionKind.LOOP
    if node.type == "match_arm":
        return RegionKind.MATCH_ARM
    if node.type == "block":
        return RegionKind.BLOCK
    if node.type in {
        "let_declaration",
        "expression_statement",
        "empty_statement",
    }:
        return RegionKind.STATEMENT
    if (
        node.parent is not None
        and node.parent.type == "block"
        and node.type != "function_item"
        and (node.type.endswith("_item") or node.type.endswith("_declaration"))
    ):
        return RegionKind.STATEMENT
    if (
        node.type.endswith("_expression")
        or node.type == "macro_invocation"
        or node.type
        in {"unsafe_block", "async_block", "const_block", "try_block"}
    ):
        return RegionKind.EXPRESSION
    if (
        node.type in {"identifier", "scoped_identifier", "self"}
        or node.type.endswith("_literal")
    ) and _is_value_expression_leaf(node):
        return RegionKind.EXPRESSION
    return None


def exact_named_nodes(root: Node, start: int, end: int) -> tuple[Node, ...]:
    """Return named nodes whose complete byte span is exactly ``[start, end)``."""

    return tuple(
        node
        for node in walk(root)
        if node.is_named and node.start_byte == start and node.end_byte == end
    )


def _is_value_expression_leaf(node: Node) -> bool:
    current = node
    while current.parent is not None:
        parent = current.parent
        field_name = _field_name(current)
        if field_name in {
            "field",
            "label",
            "name",
            "path",
            "pattern",
            "return_type",
            "type",
        }:
            return False
        if field_name == "value":
            return True

        parent_type = parent.type
        if parent_type == "block":
            return True
        if parent_type.endswith("_expression") or parent_type in {
            "async_block",
            "const_block",
            "macro_invocation",
            "try_block",
            "unsafe_block",
        }:
            return True
        if (
            parent_type.endswith("_pattern")
            or parent_type.endswith("_type")
            or parent_type.endswith("_declaration")
            or parent_type.endswith("_item")
            or parent_type
            in {
                "attribute",
                "attribute_item",
                "parameter",
                "parameters",
                "self_parameter",
                "shorthand_field_initializer",
                "token_tree",
                "type_parameters",
                "where_clause",
            }
        ):
            return False
        current = parent
    return False


def _field_name(node: Node) -> str | None:
    parent = node.parent
    if parent is None:
        return None
    for index, child in enumerate(parent.children):
        if same_node(child, node):
            return parent.field_name_for_child(index)
    return None
