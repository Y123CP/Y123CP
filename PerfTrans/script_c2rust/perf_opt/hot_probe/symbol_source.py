"""Symbol → source location (file : line-range) + source-text extraction.

perf/self-time gives a base FUNCTION NAME; the evidence pack needs where that
function lives in the crate so the agent can read + rewrite it. We resolve by
scanning the crate's `src/**/*.rs` with tree-sitter for `fn <name>` items and
matching on the base name — NOT via addr2line, which needs debuginfo the fair
(LTO/stripped) build may not carry and whose addresses op_selftime discards.
"""

from __future__ import annotations

import logging
import re
from pathlib import Path

logger = logging.getLogger("hot_probe.symbol_source")

_EXPORT_NAME_RE = re.compile(r'#\[\s*export_name\s*=\s*"([^"]+)"\s*\]')


def _exported_symbol(fn, raw: bytes) -> str | None:
    """The symbol `#[export_name = "..."]` gives this fn, if it carries one.

    perf reports the LINK name, so a fn the source calls one thing and the
    linker calls another is unresolvable under its source name alone and gets
    dropped as if it were macro-generated. c2rust emits exactly this whenever
    the C name is a Rust keyword: `match` becomes `fn match_0` plus
    `#[export_name = "match"]`. Measured on fzy, where `match` is 13-16% of two
    operations and was dropped from every run as "no source location".

    Attributes are PRECEDING SIBLINGS of the function item, not children, and
    there may be several (`#[no_mangle]` then `#[export_name]`), so walk back
    over the contiguous run of them.
    """
    node = fn.prev_named_sibling
    while node is not None and node.type == "attribute_item":
        m = _EXPORT_NAME_RE.match(_slice(raw, node))
        if m:
            return m.group(1)
        node = node.prev_named_sibling
    return None


def _slice(raw: bytes, node) -> str:
    """A node's text. tree-sitter offsets are BYTE offsets, so the slice has to
    happen on the bytes that were parsed — never on a `str` decoded from them.

    Indexing a `str` with a byte offset silently shifts by one position per
    non-ASCII character SEEN SO FAR, and the shift is cumulative, so everything
    after the first such character in the file comes out as garbage. Measured
    on fzy: the changeset writer records applied rules in a comment, and the
    rule names carry `③`/`④` (`// Applied rules: [III④, III③, C3, C1]`, line
    100). Every function defined below that line — `match_row`,
    `match_positions`, `setup_match_struct` — indexed under names like
    `'\n    mut '` and became unresolvable. The crate poisoned its own index by
    being rewritten.
    """
    return raw[node.start_byte:node.end_byte].decode("utf-8", "replace")


def _iter_function_items(node):
    """Yield every `function_item` node in the tree (recursive — c2rust nests
    fns under `pub mod src { pub mod <mod> { ... } }`)."""
    stack = [node]
    while stack:
        n = stack.pop()
        if n.type == "function_item":
            yield n
        stack.extend(n.children)


class FnIndex:
    """base fn name → [(rel_file, line_start, line_end)] over the crate src."""

    def __init__(self, index: dict[str, list[tuple[str, int, int]]],
                 crate: Path) -> None:
        self._index = index
        self._crate = crate

    def names(self) -> set[str]:
        """Every fn name that has a source definition — the source-editable set.
        A hot symbol NOT in here (e.g. a `#[bitfield]`-macro-generated accessor)
        cannot be rewritten; attribution should roll up to an editable caller."""
        return set(self._index)

    def all_sources(self) -> dict[str, str]:
        """{fn_name: source} for every crate fn — same disambiguation as
        `resolve` (largest body wins on collision). Used by the wrapper
        detector's fixed-point pass to catch two-level libc shims (`tty_init`
        → `tty_getwinsz` → `ioctl`)."""
        out: dict[str, str] = {}
        for name in self._index:
            loc = self.resolve(name)
            if loc is not None:
                src = self.source(*loc)
                if src:
                    out[name] = src
        return out

    def aliases_of(self, name: str) -> set[str]:
        """Every OTHER name that resolves to the same definition as `name`.

        A function can be known by two names at once: the one in the source and
        the one the linker exports (`#[export_name]`). Which one reaches a
        given consumer depends on where it came from — perf reports the LINK
        name, while DWARF-based attribution reports the SOURCE name. A set of
        hot function names built from one channel therefore silently drops
        hits produced by the other.

        Measured on fzy: `hot_fn_names` is built from perf and holds `match`;
        the C12 sites attribute through DWARF to `match_0`, and the
        `hot_fns` prune in `class_I.scan` dropped all 26,632 bytes of them.
        """
        loc = self.resolve(name)
        if loc is None:
            return set()
        return {n for n, cands in self._index.items()
                if loc in cands and n != name}

    def resolve(self, name: str) -> tuple[str, int, int] | None:
        """Location for `name`. Unique → it. Ambiguous (same name in several
        modules) → the largest body (the hot one is almost always the real
        definition, not a tiny shim), logged. Missing → None."""
        cands = self._index.get(name)
        if not cands:
            return None
        if len(cands) == 1:
            return cands[0]
        best = max(cands, key=lambda c: c[2] - c[1])
        logger.warning("ambiguous fn %r (%d defs) → picking %s:%d-%d",
                       name, len(cands), best[0], best[1], best[2])
        return best

    def source(self, file: str, line_start: int, line_end: int) -> str:
        """The function's source text (inclusive line range)."""
        try:
            lines = (self._crate / file).read_text(
                encoding="utf-8", errors="replace").splitlines()
        except OSError:
            return ""
        return "\n".join(lines[line_start - 1:line_end])


def build_fn_index(crate: Path) -> FnIndex:
    """tree-sitter scan every crate/src/**/*.rs → FnIndex. Also indexes a
    root-level lib.rs if present (c2rust sometimes emits the whole crate there)."""
    from tree_sitter import Language, Parser
    import tree_sitter_rust
    parser = Parser(Language(tree_sitter_rust.language()))

    index: dict[str, list[tuple[str, int, int]]] = {}
    roots = []
    if (crate / "src").is_dir():
        roots += sorted((crate / "src").rglob("*.rs"))
    if (crate / "lib.rs").is_file():
        roots.append(crate / "lib.rs")

    for rs in roots:
        if "target" in rs.parts:
            continue
        try:
            raw = rs.read_bytes()
        except OSError:
            continue
        tree = parser.parse(raw)
        rel = str(rs.relative_to(crate))
        for fn in _iter_function_items(tree.root_node):
            nn = fn.child_by_field_name("name")
            if nn is None:
                continue
            name = _slice(raw, nn)
            ls = fn.start_point[0] + 1        # tree-sitter rows are 0-indexed
            le = fn.end_point[0] + 1
            index.setdefault(name, []).append((rel, ls, le))
            # Also index the link name when it differs, so a perf frame naming
            # the symbol resolves to the same source range. See
            # `_exported_symbol`.
            link = _exported_symbol(fn, raw)
            if link and link != name:
                index.setdefault(link, []).append((rel, ls, le))
    return FnIndex(index, crate)
