"""Shared Cargo.toml parsing helpers used by every Stage 1 / pre-Stage-A pass."""

from __future__ import annotations

import re
import tomllib
from pathlib import Path


def read_pkg_name(cargo_toml: Path) -> str | None:
    """Return the `[package].name` value, or None if not found."""
    if not cargo_toml.exists():
        return None
    section = None
    for line in cargo_toml.read_text().splitlines():
        s = line.strip()
        if s.startswith("[") and s.endswith("]"):
            section = s[1:-1]
            continue
        if section == "package" and (m := re.match(r'\s*name\s*=\s*"([^"]+)"', line)):
            return m.group(1)
    return None


def read_crate_name(crate_dir: Path) -> str:
    """Return the Rust crate name (importable identifier) for `crate_dir`.

    Prefers `[lib].name` (set explicitly when the crate publishes a
    different lib name than the package, e.g. `package = "libcsv_raw"`
    + `[lib] name = "libcsv"`); falls back to `[package].name`. Used
    everywhere we need the name that goes on the LHS of `use <crate>::…`
    or in a `crate-type` rewrite.
    """
    cargo = tomllib.loads((crate_dir / "Cargo.toml").read_text())
    return cargo.get("lib", {}).get("name") or cargo["package"]["name"]


def bin_paths(cargo_toml: Path) -> set[Path]:
    """Return the absolute paths listed under `[[bin]] path = "..."`."""
    if not cargo_toml.exists():
        return set()
    out: set[Path] = set()
    in_bin = False
    for line in cargo_toml.read_text().splitlines():
        s = line.strip()
        if s == "[[bin]]":
            in_bin = True
        elif s.startswith("["):
            in_bin = False
        elif in_bin and (m := re.match(r'path\s*=\s*"([^"]+)"', s)):
            out.add((cargo_toml.parent / m.group(1)).resolve())
    return out


def use_line_for(file: Path, dest_module: str,
                 bins: set[Path], crate: str | None) -> str:
    """Return the right `use ...;` line for `file`:
      - lib file → `pub use crate::src::<dest>::*;`
      - bin file → `use ::<crate>::src::<dest>::*;`  (cross-crate)
    Falls back to relative `pub use super::<dest>::*` if `crate` is None.

    The lib form uses absolute `crate::src::<dest>` path rather than
    `super::<dest>` so it works at any depth — both top-level
    `src/<TU>.rs` and nested `src/<sub>/<TU>.rs` (multi-component
    projects like optipng).

    Lib files use `pub use` (not bare `use`) because c2rust harnesses
    sometimes reference types via their ORIGINAL module path
    (e.g. heman's `heman_raw::src::src::ops::heman_image`). Without
    re-export, the dedup pass moves the type to `c_types`, and the
    original module's bare `use` keeps it private at that path → harness
    fails with E0603 "private type alias". `pub use` preserves the
    original public visibility at the original path. Bins don't need
    `pub` (nothing imports a bin's modules).
    """
    if file.resolve() in bins and crate:
        return f"use ::{crate}::src::{dest_module}::*;"
    if crate:
        return f"pub use crate::src::{dest_module}::*;"
    return f"pub use super::{dest_module}::*;"


def insert_use_after_inner_attrs(text: str, use_line: str) -> str:
    """Insert `use_line` AFTER any leading `#![...]` inner attributes.

    Inner attributes must be the first non-comment items in a file; if we
    prepend a `use` statement before them Rust errors with E0658
    "inner attribute not permitted here". This helper scans past the
    leading `#![...]` block (and contiguous comments) and inserts there.
    No-op if `use_line` is already present anywhere in the text.
    """
    if use_line in text:
        return text
    lines = text.splitlines(keepends=True)
    insert_at = 0
    for i, line in enumerate(lines):
        s = line.lstrip()
        if s.startswith("#!["):
            insert_at = i + 1
        elif s and not s.startswith("//"):
            break
    lines.insert(insert_at, use_line + "\n")
    return "".join(lines)
