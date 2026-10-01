"""Tree-sitter helpers for locating top-level Rust items.

The dedup passes need precise byte ranges of items so we can splice them out
without touching surrounding source. Tree-sitter handles multi-line items
(e.g. `pub const NULL: *mut c_void = ::core::ptr::null_mut::<...>();`) cleanly,
which is why we don't fall back to regex.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Optional

from tree_sitter import Language, Parser
import tree_sitter_rust

_parser: Optional[Parser] = None


def _get_parser() -> Parser:
    """Build a tree-sitter Rust parser using the modern
    tree-sitter-rust Python binding (>= 0.21 API: Language wraps a
    capsule from the package, Parser() takes no args, language is set
    via .language attribute).

    Old API path (`Language(<so_path>, "rust")` + `Parser().set_language()`)
    broke when tree-sitter Python binding moved to capsule-based languages
    — observed via batch Stage A audit 2026-05-29 (binn / brotli / heman
    / libxml2 / tmux / lodepng all failed Stage 1 cleanup with
    `TypeError: __init__() takes exactly 1 argument (2 given)`)."""
    global _parser
    if _parser is None:
        lang = Language(tree_sitter_rust.language())
        p = Parser(lang)
        _parser = p
    return _parser


@dataclass(frozen=True)
class Item:
    kind: str           # tree-sitter node type, e.g. "type_item" / "const_item"
    name: str           # symbol name
    body: str           # full source text, exactly as it appears in the file
    file: Path
    start_byte: int
    end_byte: int


def find_top_level_items(file: Path, kinds: tuple[str, ...]) -> list[Item]:
    """Return direct children of `source_file` whose node-type is in `kinds`.

    "Top-level" = direct child of the file's root node. c2rust emits all
    type aliases, constants, structs, and `extern "C" { }` blocks at the
    top level, so we never need to descend into nested scopes.

    Outer attributes (`#[derive(...)]`, `#[repr(C)]`) are preceding sibling
    `attribute_item` nodes in tree-sitter's grammar. We extend the item's
    range backwards to include any consecutive attribute siblings so the
    spliced body carries them along.
    """
    parser = _get_parser()
    src = file.read_bytes()
    tree = parser.parse(src)
    children = list(tree.root_node.children)
    items: list[Item] = []
    for i, child in enumerate(children):
        if child.type not in kinds:
            continue
        name_node = child.child_by_field_name("name")
        if name_node is None:
            continue
        start = child.start_byte
        j = i - 1
        while j >= 0 and children[j].type == "attribute_item":
            start = children[j].start_byte
            j -= 1
        items.append(Item(
            kind       = child.type,
            name       = src[name_node.start_byte:name_node.end_byte].decode(),
            body       = src[start:child.end_byte].decode(),
            file       = file,
            start_byte = start,
            end_byte   = child.end_byte,
        ))
    return items
