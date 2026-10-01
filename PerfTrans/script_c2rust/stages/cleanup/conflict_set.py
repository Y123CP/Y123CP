"""Global conflict-set detection for Stage 1 dedup.

The Stage 1 dedup passes (extern types, type aliases, structs, unions,
enums, constants, extern fns, extern statics) each scan the project
INDEPENDENTLY for items of their own kind. They detect divergent bodies
WITHIN a kind (e.g., two `pub struct z_stream_s` definitions with
different fields), but they cannot see CROSS-KIND collisions:

  - `internal_state` declared as `extern "C" { pub type internal_state; }`
    in some files (opaque forward decl) and as `pub struct internal_state
    { ... }` in others (zlib's forward-declare-then-define pattern under
    multi-component projects like optipng).

  - `C2RustUnnamed` emerging as `pub type C2RustUnnamed = c_uint` in some
    TUs and `pub union C2RustUnnamed { ... }` in others (c2rust auto-names
    anonymous types per-TU; the same placeholder name represents different
    types across TUs).

If either pass migrates such a name to a shared module, the OTHER pass's
files that get the wildcard import will see two definitions of the same
name (one local, one re-exported) — leading to thousands of compile
errors. The mitigation: precompute names that have multiple distinct
(kind, body) tuples anywhere in the project, then have every dedup pass
filter its candidates against this set.

This catches:

  1. **Cross-kind**       — same name, different kinds (e.g., extern type
                            vs struct, type alias vs union)
  2. **Same-kind divergent** — same name, same kind, different bodies
                              (also caught by `_select_migrations` already,
                              but listed here for completeness)
  3. **Cascade**          — a type alias `pub type B = A` where `A` is in
                            the conflict set is rejected by `_self_contained`
                            (RHS not in scope), without needing explicit
                            tracking here.
"""

from __future__ import annotations

import logging
import re
from pathlib import Path

from .ast_helpers import find_top_level_items

logger = logging.getLogger(__name__)


# All top-level item kinds we de-duplicate. Must match tree-sitter node types.
_TOP_LEVEL_KINDS = (
    "type_item",     # pub type X = ...;
    "struct_item",   # pub struct X { ... } / pub struct X(...); / pub struct X;
    "union_item",    # pub union X { ... }
    "enum_item",     # pub enum X { ... }
    "const_item",    # pub const X: T = ...;
    "static_item",   # pub static [mut] X: T = ...;
    "function_item", # pub fn X(...) { ... }
)


# Match `extern "C" { ... }` blocks at top level (mirrors extern_dedup._EXTERN_BLOCK_RE).
_EXTERN_BLOCK_RE = re.compile(
    r'(?ms)^([ \t]*)extern\s+"C"\s*\{(.*?)\n[ \t]*\}'
)


def _split_extern_body(body: str) -> list[str]:
    """Split block body into top-level `;`-terminated declarations."""
    items, depth, last = [], 0, 0
    for i, ch in enumerate(body):
        if ch in "({[":
            depth += 1
        elif ch in ")}]":
            depth -= 1
        elif ch == ";" and depth == 0:
            items.append(body[last:i + 1])
            last = i + 1
    return items


def _classify_extern_decl(decl: str) -> tuple[str, str] | None:
    """Return (extern_kind, name) for a single decl inside `extern "C" { }`,
    or None if the line is empty / unparseable."""
    s = decl.strip()
    if not s:
        return None
    if m := re.match(r"pub\s+type\s+(\w+)\s*;\s*$", s):
        return ("extern_type", m.group(1))
    if m := re.match(r"(?:pub\s+)?fn\s+(\w+)", s):
        return ("extern_fn", m.group(1))
    if m := re.match(r"(?:pub\s+)?static\s+(?:mut\s+)?(\w+)", s):
        return ("extern_static", m.group(1))
    return None


def _normalize_kind(kind: str) -> str:
    """Collapse kinds that represent the same C-ABI entity into one
    "kind family", so a function's definition + its FFI declaration in
    callers don't trip the cross-kind conflict check.

      function_item  (`pub unsafe extern "C" fn X { body }`)
      extern_fn      (`extern "C" { pub fn X(...); }` in caller)
        → both refer to the same C symbol `X` — kind-family "fn"

      static_item    (`pub static [mut] X: T = ...;`)
      extern_static  (`extern "C" { pub static [mut] X: T; }` in caller)
        → both refer to the same C symbol `X` — kind-family "static"

    All other kinds (type_item / struct_item / union_item / enum_item /
    const_item / extern_type) keep their distinct identity, since their
    definitions ARE genuinely different things at the type/value level.
    """
    if kind in ("function_item", "extern_fn"):
        return "fn"
    if kind in ("static_item", "extern_static"):
        return "static"
    return kind


def compute_conflict_set(src_dir: Path) -> set[str]:
    """Return names that have multiple kind-families anywhere under `src_dir`
    (recursive). Items in this set MUST NOT be migrated by any Stage 1 pass
    — doing so would create wildcard-re-export collisions.

    Detects (and only detects) **cross-kind** collisions:
      - `extern "C" { pub type X; }`  +  `pub struct X { ... }`
        (zlib's `internal_state` forward-declared opaque vs full struct)
      - `pub type X = c_uint;`  +  `pub union X { ... }`
        (c2rust's `C2RustUnnamed` reused as anonymous type per-TU)
      - `pub fn X(...) { ... }`  +  `pub struct X { ... }`
        (rare but possible naming collision)

    Does NOT flag same-kind collisions (e.g., 2 distinct
    `pub struct z_stream_s { ... }` bodies under conditional compilation):
    those are already filtered by `_select_migrations` via its
    `len(bodies) > 1` divergent-body check.

    Does NOT flag definition+declaration pairs (`pub fn X` defined in one
    TU + `extern "C" { fn X; }` declared in callers): kind normalization
    treats these as the same kind-family.

    Generated shared modules (c_types.rs etc.) are excluded so prior-run
    migrations don't self-collide.
    """
    if not src_dir.is_dir():
        return set()

    generated = {"c_types.rs", "c_consts.rs", "c_structs.rs",
                 "ffi.rs", "c_extern_types.rs"}

    # name -> set of normalized kind-families
    seen: dict[str, set[str]] = {}

    for rs in sorted(src_dir.rglob("*.rs")):
        if rs.name in generated:
            continue

        # 1. Top-level items via tree-sitter (covers type/struct/union/enum/const/static/fn).
        try:
            for item in find_top_level_items(rs, _TOP_LEVEL_KINDS):
                seen.setdefault(item.name, set()).add(_normalize_kind(item.kind))
        except Exception as e:
            logger.warning(f"[conflict_set] tree-sitter scan failed on {rs}: {e}")

        # 2. extern "C" { ... } block items via regex.
        try:
            text = rs.read_text()
        except Exception:
            continue
        for m in _EXTERN_BLOCK_RE.finditer(text):
            for decl in _split_extern_body(m.group(2)):
                cls = _classify_extern_decl(decl)
                if cls is None:
                    continue
                kind, name = cls
                seen.setdefault(name, set()).add(_normalize_kind(kind))

    conflicts = {name for name, kinds in seen.items() if len(kinds) > 1}
    if conflicts:
        logger.info(f"[conflict_set] {len(conflicts)} name(s) with cross-kind "
                    f"definitions — excluding from all dedup passes")
        for name in sorted(conflicts):
            logger.debug(f"[conflict_set]   {name!r}: kinds={sorted(seen[name])}")
    else:
        logger.info("[conflict_set] no cross-kind collisions detected")
    return conflicts
