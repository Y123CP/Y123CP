"""Generic top-level item dedup pass.

For a given category of top-level item (type alias, constant, ...):
  1. collect every instance from src/*.rs
  2. eligibility: appears in ≥2 distinct files AND identical bodies AND
     RHS only references fully-qualified paths, Rust builtins, or
     identifiers already brought into scope by a prior pass
  3. write canonical body to src/<dest_module>.rs
  4. splice instances out of source files; ensure each gets
     `use super::<dest_module>::*;`
  5. register `pub mod <dest_module>;` in lib.rs

Idempotent: re-running on already-deduped output is a no-op.

Passes are chained: after type-alias dedup migrates `Bool` to `c_types`,
the constant pass receives `{'Bool', ...}` in `scope`, allowing
`pub const True: Bool = ...;` to migrate (with c_consts importing c_types).
"""

from __future__ import annotations

import logging
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from .ast_helpers import Item, find_top_level_items
from .cargo_utils import (
    bin_paths as _bin_paths, insert_use_after_inner_attrs, use_line_for,
)

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Eligibility filter: "is this body self-contained?"
# ---------------------------------------------------------------------------

# Identifiers that are safe regardless of context:
# Rust primitives, prelude types, and keywords.
_BENIGN: frozenset[str] = frozenset({
    # primitives
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128",
    "isize", "usize", "f32", "f64", "bool", "char", "str",
    # prelude
    "Option", "Result", "Box", "Vec", "String", "Cow",
    "Some", "None", "Ok", "Err", "true", "false",
    # keywords
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "fn", "for", "if", "impl", "in", "let",
    "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "Self",
    "self", "static", "struct", "super", "trait", "type", "unsafe",
    "use", "where", "while", "yield", "union", "box",
})

# Identifier reference: a complete word (\b-anchored), not preceded by `:` or
# word char (so `c_int` inside `::core::ffi::c_int` is excluded), and NOT
# immediately followed by `:` (excludes struct-field names like `pub next_in:`).
# The trailing \b prevents regex backtracking from matching identifier prefixes.
_IDENT  = re.compile(r"(?<![:\w])([A-Za-z_]\w*)\b(?!\s*:)")
_STRING = re.compile(r'"[^"]*"')
# Strip leading attributes + `pub <kind> <name>` so the symbol's own name
# and its declaring keywords don't show up in the identifier scan.
_LHS    = re.compile(
    r"^(?:\s*#\[[^\]]*\]\s*)*\s*pub\s+(?:type|const|struct|union|enum)\s+\w+\s*[:{]?"
)


def _self_contained(body: str, scope: frozenset[str], own_name: str = "") -> bool:
    """Body's free identifiers (those not preceded by `::`) all belong to
    one of: Rust builtins, names in `scope`, or the item's own name (for
    self-referential types like `*mut _IO_FILE` inside `_IO_FILE`)."""
    body = _LHS.sub("", body)
    body = _STRING.sub('""', body)
    permitted = _BENIGN | scope
    if own_name:
        permitted = permitted | {own_name}
    return all(m.group(1) in permitted for m in _IDENT.finditer(body))


# ---------------------------------------------------------------------------
# Pass config & entry
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class DedupConfig:
    item_kinds:  tuple[str, ...]    # tree-sitter node kinds, e.g. ("type_item",)
    dest_module: str                # filename stem, e.g. "c_types"
    label:       str                # for logs, e.g. "type aliases"
    imports:     tuple[str, ...] = ()  # `use super::<m>::*;` for each in the dest


def dedup_pass(project_path: Path, cfg: DedupConfig,
               scope: Optional[set[str]] = None,
               include_bins: bool = False,
               crate_name: Optional[str] = None,
               conflicts: Optional[set[str]] = None) -> int:
    """Run one dedup pass. Returns count of symbols migrated. Mutates
    `scope` by adding migrated names (so chained passes can reference them).

    `include_bins` extends the candidate set to `[[bin]]` files; binary
    files use a cross-crate `use ::<crate>::src::<dest>::*;` import
    instead of `use super::<dest>::*;` since they are independent crate
    roots. Caller must pass `crate_name` (the lib crate name) when
    `include_bins=True`.

    `conflicts`: names with cross-kind / cross-body collisions anywhere in
    the project (precomputed by `cleanup/conflict_set.compute_conflict_set`).
    Items whose name is in this set are NEVER migrated, regardless of
    same-body / ≥2-file rules. Required for safe dedup on multi-component
    projects (e.g., optipng) where c2rust emits divergent translations
    of identically-named items across TUs.
    """
    src_dir = project_path / "src"
    lib_rs  = project_path / "lib.rs"
    cargo   = project_path / "Cargo.toml"
    if not src_dir.is_dir() or not lib_rs.exists():
        logger.warning(f"[{cfg.label}] no src/ or lib.rs — skip")
        return 0

    dest_file = src_dir / f"{cfg.dest_module}.rs"
    scope = scope if scope is not None else set()
    bins  = _bin_paths(cargo)

    items: list[Item] = []
    # rglob: nested layouts (multi-component projects like optipng have
    # `src/<sub>/<TU>.rs`) need recursive scan; flat layouts are unaffected
    # since their .rs files all live at top level. Cross-kind / divergent-
    # body collisions exposed by deep scans are filtered via `conflicts`.
    for rs in sorted(src_dir.rglob("*.rs")):
        if rs == dest_file:
            continue
        if not include_bins and rs.resolve() in bins:
            continue
        items.extend(find_top_level_items(rs, cfg.item_kinds))

    migrate = _select_migrations(items, frozenset(scope),
                                  frozenset(conflicts or set()))
    if not migrate:
        logger.info(f"[{cfg.label}] nothing to dedup")
        return 0

    affected: dict[Path, list[Item]] = {}
    for it in items:
        if it.name in migrate:
            affected.setdefault(it.file, []).append(it)

    _write_dest_module(dest_file, migrate, cfg)
    for f, removed in affected.items():
        _splice_out_and_import(
            f, removed,
            use_line_for(f, cfg.dest_module, bins, crate_name),
        )
    _register_module(lib_rs, cfg.dest_module)

    scope.update(migrate.keys())
    logger.info(f"[{cfg.label}] migrated {len(migrate)} symbol(s) "
                f"across {len(affected)} file(s)")
    return len(migrate)


# ---------------------------------------------------------------------------
# Selection
# ---------------------------------------------------------------------------

def _select_migrations(items: list[Item], scope: frozenset[str],
                       conflicts: frozenset[str] = frozenset()) -> dict[str, Item]:
    """name → canonical Item.

    Two stages:
    1. Pre-filter: identical body across ≥2 distinct files.
    2. Fixpoint: iteratively accept candidates whose free identifiers are
       resolved by (BENIGN ∪ scope ∪ already-accepted candidates ∪ own name).
       Each iteration may unlock peers (e.g. `EState` referencing `bz_stream`
       once `bz_stream` is accepted).

    `conflicts`: names with cross-kind or cross-body collisions anywhere in
    the project; never migrated. See cleanup/conflict_set.py.
    """
    by_name: dict[str, list[Item]] = {}
    for it in items:
        by_name.setdefault(it.name, []).append(it)

    candidates: dict[str, Item] = {}
    for name, insts in by_name.items():
        if name in conflicts:
            logger.debug(f"  [skip] {name!r} in conflict set — has cross-kind "
                         f"or cross-body collision elsewhere in project")
            continue
        # Milestone D safety gate (2026-06-19): never migrate `static mut`.
        # c2rust emits one mutable static per TU; merging them into one
        # shared cell would change observable semantics (writes in one TU
        # become visible to another). Allows `static` (immutable).
        if insts[0].kind == "static_item" and _is_static_mut(insts[0].body):
            logger.debug(f"  [skip] {name!r} is `static mut` — semantic "
                          f"change risk if unified")
            continue
        files  = {i.file for i in insts}
        bodies = {_norm(i.body) for i in insts}
        if len(files) < 2:
            continue
        if len(bodies) > 1:
            logger.info(f"  [skip] '{name}' has {len(bodies)} divergent bodies "
                        f"across {len(files)} files")
            continue
        candidates[name] = insts[0]

    accepted: dict[str, Item] = {}
    pending = dict(candidates)
    while pending:
        permitted = scope | set(accepted.keys())
        promoted = [
            n for n, it in pending.items()
            if _self_contained(it.body, frozenset(permitted), own_name=n)
        ]
        if not promoted:
            break
        for n in promoted:
            accepted[n] = pending.pop(n)
    for n in pending:
        logger.debug(f"  [skip] '{n}' depends on non-migrated symbols")
    return accepted


_STATIC_MUT_RE = re.compile(r"\bstatic\s+mut\b")


def _is_static_mut(body: str) -> bool:
    """True iff the item body declares `static mut ...`. We strip
    attributes / `pub` / leading whitespace before checking so the regex
    works the same whether the body starts with `#[no_mangle]` lines or
    a bare `pub static mut`."""
    return _STATIC_MUT_RE.search(body) is not None


_LINE_COMMENT_RE  = re.compile(r"//[^\n]*")
_BLOCK_COMMENT_RE = re.compile(r"/\*.*?\*/", re.DOTALL)
_WS_RE            = re.compile(r"\s+")
# `(N as c_int)` / `(-N as c_int)` → `N` / `-N`
_PAREN_INT_AS_CINT_RE = re.compile(r"\(\s*(-?\d+)\s+as\s+c_(?:int|uint|short|long|ushort|ulong|uchar|schar|char)\s*\)")
# `N as c_int` not followed by another `as` → `N`
_INT_AS_CINT_RE = re.compile(r"(?<![\w])(-?\d+)\s+as\s+c_(?:int|uint|short|long|ushort|ulong|uchar|schar|char)(?!\s*as)")
# `- ( N )` → `-N`
_NEG_PAREN_RE = re.compile(r"-\s*\(\s*(\d+)\s*\)")
# `(N)` where N is a bare integer literal → `N`
_PAREN_INT_RE = re.compile(r"\(\s*(-?\d+)\s*\)")


def _norm(text: str) -> str:
    """Normalize a top-level item body so cross-TU clones with cosmetic
    differences fold to the same hash.

    Strips: line + block comments, every whitespace run collapses to a
    single space.

    Folds (Milestone D, 2026-06-19): syntactic equivalents c2rust commonly
    emits inconsistently across TUs at integer literal sites. Each
    equivalence pair below has been observed in the wild:

      `(-9 as c_int)`         ≡ `-9`     (bzip2 BZ_CONFIG_ERROR)
      `-(9 as c_int)`         ≡ `-9`     (same as above, paren outside `-`)
      `9 as c_int`            ≡ `9`      (bzip2 BZ_OK; tmux ARG_MAX)
      `(9)`                   ≡ `9`      (over-parenthesized literal)

    Same rules apply to c_uint / c_short / c_long / c_uchar / c_schar /
    c_ushort / c_ulong / c_char — every primitive aliases c2rust emits.
    A non-numeric `(X as c_int)` is left untouched: the wrapping cast may
    carry semantic information when X is an expression rather than a
    literal.
    """
    text = _BLOCK_COMMENT_RE.sub(" ", text)
    text = _LINE_COMMENT_RE.sub("", text)
    text = _WS_RE.sub(" ", text).strip()
    # Apply integer-literal folds iteratively to a fixed point — once
    # `(-9 as c_int)` collapses to `-9`, an outer wrap can collapse too.
    prev = None
    while prev != text:
        prev = text
        text = _PAREN_INT_AS_CINT_RE.sub(r"\1", text)
        text = _INT_AS_CINT_RE.sub(r"\1", text)
        text = _NEG_PAREN_RE.sub(r"-\1", text)
        text = _PAREN_INT_RE.sub(r"\1", text)
    return text


# ---------------------------------------------------------------------------
# Output writers
# ---------------------------------------------------------------------------

_USE_SUPER_RE = re.compile(r"use\s+super::(\w+)::\*\s*;")


def _write_dest_module(dest: Path, migrate: dict[str, Item], cfg: DedupConfig) -> None:
    """Write or APPEND-MERGE the dest module.

    When `dest` already exists from an earlier pass (e.g. round-1 type
    aliases), this preserves its existing content and merges new items
    on top. Imports are union-merged so a later pass can declare an
    extra dependency (e.g. round-2 type pass adds `c_structs` to import
    `_IO_FILE` for `pub type FILE = _IO_FILE;`).
    """
    existing_imports: set[str] = set()
    existing_items_blob = ""
    if dest.exists():
        existing = dest.read_text()
        existing_imports = set(_USE_SUPER_RE.findall(existing))
        lines = existing.splitlines()
        # Items follow the last `use ...;` line.
        last_use = max((i for i, l in enumerate(lines) if l.lstrip().startswith("use ")),
                       default=-1)
        existing_items_blob = "\n".join(lines[last_use + 1:]).strip()

    all_imports = sorted(existing_imports | set(cfg.imports))
    parts = [
        f"// Auto-generated by script_c2rust Stage 1: shared {cfg.label}.",
        f"// Do not edit by hand — regenerate via the cleanup pass.",
        "",
    ]
    if all_imports:
        parts += [f"use super::{m}::*;" for m in all_imports] + [""]
    if existing_items_blob:
        parts.append(existing_items_blob)
    parts += [it.body for it in sorted(migrate.values(), key=lambda i: i.name)]
    dest.write_text("\n".join(parts) + "\n")


def _splice_out_and_import(file: Path, removed: list[Item], use_line: str) -> None:
    """Remove `removed` items from `file` (by byte range) and add `use_line`
    after any leading `#![...]` inner attributes (binaries have these and
    will reject `use` placed before them)."""
    src = file.read_bytes()
    for it in sorted(removed, key=lambda i: i.start_byte, reverse=True):
        src = src[:it.start_byte] + src[it.end_byte:]
    text = src.decode()
    text = re.sub(r"\n{3,}", "\n\n", text)
    text = insert_use_after_inner_attrs(text, use_line)
    file.write_text(text)


def _register_module(lib_rs: Path, module_name: str) -> None:
    """Add `pub mod <module_name>;` to lib.rs.

    Prefers placement inside the c2rust-typical `pub mod src { ... }` wrapper,
    so the file maps to `src/<module_name>.rs`. Falls back to top-level.
    """
    text = lib_rs.read_text()
    if re.search(rf"\bpub\s+mod\s+{re.escape(module_name)}\s*;", text):
        return  # already registered

    m = re.search(r"pub\s+mod\s+src\s*\{", text)
    if m:
        nl = text.find("\n", m.end())
        pos = nl + 1 if nl != -1 else m.end()
        text = text[:pos] + f"    pub mod {module_name};\n" + text[pos:]
    else:
        text = f"pub mod {module_name};\n" + text
    lib_rs.write_text(text)
