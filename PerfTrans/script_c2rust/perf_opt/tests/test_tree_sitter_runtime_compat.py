from __future__ import annotations

from pathlib import Path

import pytest

from utils.c_parser import CParser
from utils.rust_parser import RustParser


@pytest.mark.parametrize(
    ("declaration", "member_index", "expected"),
    [
        ("struct Record { int first; char *second; };", 1, "second"),
        ("union Value { long integer; double decimal; };", 0, "integer"),
        ("enum State { IDLE, RUNNING, DONE };", 2, "DONE"),
    ],
)
def test_c_parser_reads_struct_union_and_enum_members(
    tmp_path: Path,
    declaration: str,
    member_index: int,
    expected: str,
) -> None:
    parser = CParser(str(tmp_path))

    assert parser.root_path == str(tmp_path)
    assert parser.c_parser_so_path.endswith(
        ("c_parser_new.so", "MAC_c_parser.so")
    )
    assert parser.get_c_member_name_by_index(declaration, member_index) == expected


def test_rust_parser_get_rust_definitions_uses_capsule_runtime(
    tmp_path: Path,
) -> None:
    parser = RustParser(str(tmp_path))
    source = "struct Widget;\nfn compute() {}\n"

    assert parser.root_path == str(tmp_path)
    assert parser.rust_parser_so_path.endswith(
        ("rust_parser.so", "MAC_rust_parser.so")
    )
    assert parser.get_rust_definitions(source) == ["Widget", "compute"]


def test_rust_parser_has_syntax_error_for_valid_and_invalid_code(
    tmp_path: Path,
) -> None:
    parser = RustParser(str(tmp_path))

    assert parser.has_syntax_error("fn valid() {}") is False
    assert parser.has_syntax_error("fn invalid( {") is True


def test_rust_parser_finds_items_with_compatible_return_shape(
    tmp_path: Path,
) -> None:
    parser = RustParser(str(tmp_path))
    source = "use std::fmt;\nstruct Widget;\nfn compute() {}\n"

    assert parser.find_rust_items(source) == [
        {
            "type": "use_declaration",
            "name": "std::fmt",
            "start_line": 1,
            "end_line": 1,
        },
        {
            "type": "struct",
            "name": "Widget",
            "start_line": 2,
            "end_line": 2,
        },
        {
            "type": "function",
            "name": "compute",
            "start_line": 3,
            "end_line": 3,
        },
    ]


def test_rust_parser_extracts_item_content_with_compatible_return_shape(
    tmp_path: Path,
) -> None:
    parser = RustParser(str(tmp_path))
    source = (
        "pub fn compute(value: i32) -> i32 { value + 1 }\n"
        "struct Widget { value: i32 }\n"
    )

    assert parser.extract_item_content(source) == (
        "pub fn compute(value: i32) -> i32\n"
        "struct Widget { value: i32 }"
    )


def test_rust_parser_separates_use_statements_with_compatible_return_shape(
    tmp_path: Path,
) -> None:
    parser = RustParser(str(tmp_path))
    source = "use std::fmt;\nuse std::io;\nfn compute() {}\n"

    use_statements, remaining = parser.separate_use_statements(source)

    assert use_statements == ["use std::fmt;", "use std::io;"]
    assert remaining == "fn compute() {}"


def test_query_captures_flattens_multiple_names_in_source_order() -> None:
    from utils.tree_sitter_runtime import (
        get_c_language,
        get_rust_language,
        new_parser,
        query_captures,
    )

    language = get_rust_language()
    parser = new_parser(language)
    assert get_rust_language() is language
    assert get_c_language() is get_c_language()
    assert parser.language == language
    tree = parser.parse(b"fn beta() {}\nfn alpha() {}\n")
    captures = query_captures(
        language,
        "(function_item name: (identifier) @name) @item",
        tree.root_node,
    )

    assert [(node.text, name) for node, name in captures] == [
        (b"fn beta() {}", "item"),
        (b"beta", "name"),
        (b"fn alpha() {}", "item"),
        (b"alpha", "name"),
    ]
