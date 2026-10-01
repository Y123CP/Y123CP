"""The type definitions a rewrite needs to see, and the names it got wrong.

A c2rust crate keeps its struct definitions far from the functions that walk
them, and a function's own text names only the outermost one. `hm_destroy`
takes a `*mut hashmap_t` and indexes `hm.cell[i].e[j].k`; the element type of
`e` is three definitions away and appears nowhere in the function. To slice
that walk the model has to name that type, and with no definition in front of
it, it invents a plausible one.

Measured: `hashmap_entry_t`, where the crate says `hashentry_t`. Four
consecutive runs of the same crate, three LLM turns and three builds burned
each time, because the compile error says the name does not exist without
saying which name does.

Two things live here:

  * `build_type_context` — the transitive closure of type definitions a
    function's source reaches, for the prompt.
  * `name_correction_note` — for a build that already failed on an unresolved
    name: the closest names the crate actually defines.
"""

from __future__ import annotations

import difflib
import functools
import re
from pathlib import Path
from typing import Iterable, NamedTuple, Optional

# `pub type X =` / `pub struct X {` / `pub struct X(` / `pub union X {` /
# `pub enum X {`. c2rust emits everything at module top level with `pub`, and
# wraps composites in `#[derive]` / `#[repr(C)]` attributes on preceding lines.
_DEF_RE = re.compile(
    r"^pub\s+(?P<kind>type|struct|union|enum)\s+(?P<name>[A-Za-z_]\w*)",
    re.MULTILINE,
)
_IDENT_RE = re.compile(r"\b[A-Za-z_]\w*\b")

# Scalar spellings a type alias can bottom out in. An alias chain that ends in
# one of these carries nothing the model does not already know — `size_t =
# usize` is noise, `hashmap_t = _hashmap_t` is the whole point.
_SCALARS = frozenset({
    "u8", "u16", "u32", "u64", "u128", "usize",
    "i8", "i16", "i32", "i64", "i128", "isize",
    "f32", "f64", "bool", "char", "str",
    "c_char", "c_schar", "c_uchar", "c_short", "c_ushort", "c_int", "c_uint",
    "c_long", "c_ulong", "c_longlong", "c_ulonglong", "c_float", "c_double",
    "c_void",
})


class TypeDef(NamedTuple):
    name: str
    kind: str          # "type" | "struct" | "union" | "enum"
    text: str          # the definition as written, attributes included
    refs: tuple[str, ...]   # identifiers its body mentions


def _attribute_lines_before(text: str, start: int) -> int:
    """Walk back over the `#[...]` lines attached to a definition."""
    pos = start
    while pos > 0:
        prev_line_end = text.rfind("\n", 0, pos - 1)
        line_start = prev_line_end + 1
        line = text[line_start:pos - 1].strip()
        if not line.startswith("#["):
            break
        pos = line_start
    return pos


def _definition_span(text: str, match: re.Match) -> tuple[int, int]:
    """Byte span of one definition: attributes through `}` or `;`."""
    start = _attribute_lines_before(text, match.start())
    brace = text.find("{", match.end())
    semi = text.find(";", match.end())
    # A brace-bodied composite ends at its matching close; everything else
    # (alias, tuple struct) ends at the first semicolon.
    if brace >= 0 and (semi < 0 or brace < semi):
        depth, j = 0, brace
        while j < len(text):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    return start, j + 1
            j += 1
        return start, len(text)
    if semi >= 0:
        return start, semi + 1
    return start, match.end()


def _crate_sources(crate_root: Path) -> list[Path]:
    files: list[Path] = []
    src = crate_root / "src"
    if src.is_dir():
        files += sorted(src.rglob("*.rs"))
    for extra in ("lib.rs", "main.rs"):
        p = crate_root / extra
        if p.is_file():
            files.append(p)
    return files


@functools.lru_cache(maxsize=4)
def _index(crate_root: Path) -> dict[str, TypeDef]:
    """name → definition, over the whole crate.

    Cached per crate. Safe because no rule in the set rewrites a type
    definition — rewrites replace function bodies, signatures, and regions,
    and the one class of change that would touch a struct (field retyping,
    AoS→SoA) is deliberately out of scope, W1 cannot verify it. If that ever
    changes, this cache has to go with it; `test_the_index_is_cached_per_crate`
    is where that assumption is written down.
    """
    out: dict[str, TypeDef] = {}
    for path in _crate_sources(crate_root):
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for m in _DEF_RE.finditer(text):
            name = m.group("name")
            if name in out:
                continue
            lo, hi = _definition_span(text, m)
            body = text[lo:hi]
            refs = tuple(dict.fromkeys(
                t for t in _IDENT_RE.findall(body[m.start() - lo:])
                if t != name
            ))
            out[name] = TypeDef(name, m.group("kind"), body, refs)
    return out


def _bottoms_out_in_a_composite(
    name: str, index: dict[str, TypeDef], _seen: frozenset[str] = frozenset()
) -> bool:
    """True if following this alias eventually reaches a struct/union/enum."""
    defn = index.get(name)
    if defn is None or name in _seen:
        return False
    if defn.kind != "type":
        return True
    seen = _seen | {name}
    return any(
        ref not in _SCALARS and _bottoms_out_in_a_composite(ref, index, seen)
        for ref in defn.refs
    )


def build_type_context(
    crate_root: Path,
    source: str,
    *,
    max_defs: int = 10,
    max_chars: int = 2000,
) -> str:
    """Definitions of the composite types `source` reaches, nearest first.

    Breadth-first from the names the function itself spells, so a struct it
    indexes two fields deep still arrives — that is the case the model cannot
    guess. Scalar aliases are dropped: they are the bulk of a c2rust type
    table and none of them tell the model anything.
    """
    index = _index(crate_root)
    if not index:
        return ""
    mentioned = set(_IDENT_RE.findall(source))
    frontier = [n for n in index if n in mentioned]
    if not frontier:
        return ""
    ordered: list[str] = []
    seen: set[str] = set()
    while frontier and len(ordered) < max_defs:
        nxt: list[str] = []
        for name in frontier:
            if name in seen:
                continue
            seen.add(name)
            defn = index[name]
            if defn.kind == "type" and not _bottoms_out_in_a_composite(name, index):
                continue        # scalar alias — noise
            ordered.append(name)
            if len(ordered) >= max_defs:
                break
            nxt += [r for r in defn.refs
                    if r in index and r not in seen and r not in _SCALARS]
        frontier = nxt
    blocks: list[str] = []
    used = 0
    for name in ordered:
        text = index[name].text.strip()
        if used + len(text) > max_chars:
            break
        blocks.append(text)
        used += len(text)
    if not blocks:
        return ""
    return (
        "## Type definitions reachable from this function (read-only)\n"
        "Nearest first. A field's element type is often several definitions "
        "away and named nowhere in the function above — use these names as "
        "written; do not infer one from a field's role.\n\n"
        "```rust\n" + "\n\n".join(blocks) + "\n```"
    )


# ───────────────────────────────────────── after a build has already failed

# rustc's unresolved-name diagnostics. E0412 type, E0422/E0425 value,
# E0433 path, E0405 trait — all of the form "cannot find X `NAME`".
_UNRESOLVED_RE = re.compile(
    r"error\[E0(?:412|422|425|433|405)\][^`\n]*`([A-Za-z_]\w*)`"
)
_CLOSE_ENOUGH = 0.6


def unresolved_names(stderr: str) -> list[str]:
    """The names rustc could not resolve, in order, without duplicates."""
    return list(dict.fromkeys(_UNRESOLVED_RE.findall(stderr or "")))


def name_correction_note(
    crate_root: Path, stderr: str, *, max_names: int = 4, max_each: int = 3
) -> str:
    """For each unresolved name, the closest names the crate actually defines.

    The compile error says a name does not exist. It never says which one
    does, so a retry that only replays the error gets the same invented name
    back — measured, three times running on one function, across four runs of
    the crate.
    """
    names = unresolved_names(stderr)
    if not names:
        return ""
    index = _index(crate_root)
    if not index:
        return ""
    lines: list[str] = []
    for name in names[:max_names]:
        near = difflib.get_close_matches(
            name, list(index), n=max_each, cutoff=_CLOSE_ENOUGH)
        if not near:
            continue
        shown = ", ".join(f"`{n}`" for n in near)
        lines.append(f"  * `{name}` does not exist. The crate defines: {shown}")
    if not lines:
        return ""
    return (
        "\n\n## Names the crate actually defines\n"
        "The error above says a name is unresolved; here is what is there "
        "instead. Use the spelling as given — do not invent a variant of "
        "it.\n" + "\n".join(lines) + "\n"
    )
