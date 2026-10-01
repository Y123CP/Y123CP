"""Filesystem scans of a crate's public-API surface.

These are READ-ONLY helpers — they never mutate the crate, only enumerate
what the lib exports. Used by:

  · `pre_stage_a_extern.normalize_*_extern_blocks` to rewrite harness /
    bin `extern "C" { fn foo; }` decls into `use <crate>::<mod>::foo;`
    imports when `foo` is actually a lib-public fn.

  · `perf_opt`'s sa-engine stage to decide which fns are part of the
    crate's public API and therefore worth lifting.

2026-05-25: walks the filesystem rather than regex-parsing `lib.rs`.
The old `pub mod NAME;` regex missed c2rust outputs that nest
`pub mod src { pub mod src { … } }` (e.g. heman: 302 candidates → 0
lift because the scan returned empty); the filesystem walk also picks
up block-form `pub mod kazmath { … }` modules transparently.
"""

from __future__ import annotations

import re
import tomllib
from pathlib import Path


# Match a top-level `pub (unsafe) (extern "C") fn NAME` decl (the trailing
# `[(<]` accepts both `fn foo(` and the rare generic `fn foo<` form).
_PUB_FN_RE = re.compile(
    r'pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+([A-Za-z_]\w*)\s*[(<]',
)

# Match `pub struct/type/union/enum NAME` at line start (so we don't
# pick up nested decls inside fn bodies — c2rust never nests these).
_PUB_STRUCT_RE     = re.compile(r'(?:^|\n)pub\s+struct\s+([A-Za-z_]\w*)')
_PUB_TYPE_ALIAS_RE = re.compile(r'(?:^|\n)pub\s+type\s+([A-Za-z_]\w*)\s*=')
_PUB_UNION_RE      = re.compile(r'(?:^|\n)pub\s+union\s+([A-Za-z_]\w*)')
_PUB_ENUM_RE       = re.compile(r'(?:^|\n)pub\s+enum\s+([A-Za-z_]\w*)')


def _collect_bin_paths(crate_dir: Path) -> set[Path]:
    """Return absolute paths of every file Cargo treats as a binary:
    explicit `[[bin]] path = "…"` entries plus the conventional
    `src/main.rs`. `src/bin/*.rs` is handled separately by the parent
    directory check (`rs.parent.name == "bin"`)."""
    bin_paths: set[Path] = set()
    cargo = crate_dir / "Cargo.toml"
    if cargo.is_file():
        try:
            data = tomllib.loads(cargo.read_text(encoding="utf-8"))
            bins = data.get("bin", [])
            if isinstance(bins, list):
                for entry in bins:
                    p = entry.get("path") if isinstance(entry, dict) else None
                    if p:
                        bin_paths.add((crate_dir / p).resolve())
        except (OSError, tomllib.TOMLDecodeError):
            pass
    bin_paths.add((crate_dir / "src" / "main.rs").resolve())
    return bin_paths


def _is_lib_module(rs: Path, bin_paths: set[Path]) -> bool:
    """True iff `rs` is a regular lib module (not lib.rs itself, not a
    bin, not under src/bin/)."""
    if rs.name == "lib.rs":
        return False
    if rs.resolve() in bin_paths:
        return False
    if rs.parent.name == "bin" and rs.parent.parent.name == "src":
        return False
    return True


def _module_path(crate_dir: Path, rs: Path) -> str:
    """File path → `::`-joined Rust module path rooted at the crate
    (e.g. `src/lil.rs` → `"src::lil"`, `src/src/ops.rs` → `"src::src::ops"`)."""
    rel = rs.relative_to(crate_dir).with_suffix("")
    return "::".join(rel.parts)


def scan_lib_public_fns(crate_dir: Path) -> dict[str, str]:
    """`{fn_name: rust_mod_path}` for every `pub (unsafe) (extern "C") fn`
    defined in any lib module under `crate_dir`. Bin sources and lib.rs
    itself are excluded."""
    bin_paths = _collect_bin_paths(crate_dir)
    out: dict[str, str] = {}
    for rs in crate_dir.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        if not _is_lib_module(rs, bin_paths):
            continue
        mod_path = _module_path(crate_dir, rs)
        text = rs.read_text(encoding="utf-8", errors="replace")
        for m in _PUB_FN_RE.finditer(text):
            out.setdefault(m.group(1), mod_path)
    return out


def scan_lib_public_types(crate_dir: Path) -> dict[str, str]:
    """`{type_name: rust_mod_path}` for every `pub struct/type/union/enum`
    defined in any lib module under `crate_dir`. Same skip rules as
    `scan_lib_public_fns`.

    Used by `normalize_self_bin_extern_blocks`: c2rust outputs
    `extern "C" { pub type _foo; }` in a bin's main.rs aliasing lib's
    concrete `pub struct _foo`. Without unifying, the two `_foo` are
    nominally distinct Rust types and Stage A E1 strip triggers E0308
    mismatched-types at every call site.
    """
    bin_paths = _collect_bin_paths(crate_dir)
    out: dict[str, str] = {}
    for rs in crate_dir.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        if not _is_lib_module(rs, bin_paths):
            continue
        mod_path = _module_path(crate_dir, rs)
        text = rs.read_text(encoding="utf-8", errors="replace")
        for pat in (_PUB_STRUCT_RE, _PUB_TYPE_ALIAS_RE, _PUB_UNION_RE, _PUB_ENUM_RE):
            for m in pat.finditer(text):
                out.setdefault(m.group(1), mod_path)
    return out
