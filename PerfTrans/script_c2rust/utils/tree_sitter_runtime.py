"""Shared tree-sitter 0.25 runtime construction and query helpers."""

from functools import lru_cache

from tree_sitter import Language, Node, Parser, Query, QueryCursor
import tree_sitter_c
import tree_sitter_rust


@lru_cache(maxsize=1)
def get_c_language() -> Language:
    return Language(tree_sitter_c.language())


@lru_cache(maxsize=1)
def get_rust_language() -> Language:
    return Language(tree_sitter_rust.language())


def new_parser(language: Language) -> Parser:
    return Parser(language)


def query_captures(
    language: Language,
    source: str,
    node: Node,
) -> list[tuple[Node, str]]:
    captures_by_name = QueryCursor(Query(language, source)).captures(node)
    captures = [
        (captured_node, capture_name)
        for capture_name, captured_nodes in captures_by_name.items()
        for captured_node in captured_nodes
    ]
    return sorted(
        captures,
        key=lambda item: (item[0].start_byte, item[0].end_byte, item[1]),
    )
