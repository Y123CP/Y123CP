"""Milestone C-4 — bring binary TUs in line with the lib's shared modules.

Stage 0 promotes every TU containing `pub fn main()` to a `[[bin]]` target,
which is its OWN crate root and cannot reach `super::c_*::*`. The earlier
dedup passes therefore skip binaries entirely. This pass closes that gap:

  1. Read what each shared module (c_types, c_structs, c_consts,
     c_extern_types, ffi) exports.
  2. For each binary file, remove any top-level item or extern decl whose
     name appears in a shared module.
  3. Add `use ::<lib_crate>::src::<mod>::*;` for every module that
     contributed at least one removed item.

Removed items get their canonical definitions from the lib crate at
compile time, eliminating per-binary duplication.
"""

from __future__ import annotations

import logging
import re
from pathlib import Path

from .ast_helpers import find_top_level_items
from .extern_dedup import (
    _EXTERN_BLOCK_RE, _classify, _split_extern_body,
)

logger = logging.getLogger(__name__)

_SHARED_MODULES = ("c_types", "c_structs", "c_consts", "c_extern_types", "ffi")


def cleanup_binaries(project_path: Path) -> int:
    """Returns the number of binary files modified."""
    cargo = project_path / "Cargo.toml"
    if not cargo.exists():
        return 0
    bins = _bin_paths(cargo)
    if not bins:
        return 0
    crate = _read_pkg_name(cargo)
    if not crate:
        logger.warning("[bin cleanup] could not read crate name from Cargo.toml")
        return 0
    available = _collect_exports(project_path / "src")
    if not available:
        return 0

    affected = 0
    for b in sorted(bins):
        if b.exists() and _cleanup_one(b, crate, available):
            affected += 1
    if affected:
        logger.info(f"[bin cleanup] modified {affected} binary file(s)")
    return affected


# ---------------------------------------------------------------------------
# Cargo.toml helpers
# ---------------------------------------------------------------------------

def _read_pkg_name(cargo_toml: Path) -> str | None:
    section = None
    for line in cargo_toml.read_text().splitlines():
        s = line.strip()
        if s.startswith("[") and s.endswith("]"):
            section = s[1:-1]
            continue
        if section == "package" and (m := re.match(r'\s*name\s*=\s*"([^"]+)"', line)):
            return m.group(1)
    return None


def _bin_paths(cargo_toml: Path) -> set[Path]:
    out, in_bin = set(), False
    for line in cargo_toml.read_text().splitlines():
        s = line.strip()
        if s == "[[bin]]":
            in_bin = True
        elif s.startswith("["):
            in_bin = False
        elif in_bin and (m := re.match(r'path\s*=\s*"([^"]+)"', s)):
            out.add((cargo_toml.parent / m.group(1)).resolve())
    return out


# ---------------------------------------------------------------------------
# Shared-module export collection
# ---------------------------------------------------------------------------

# Top-level pub items (outside extern blocks).
_PUB_TOP_RE = re.compile(
    r"(?m)^\s*pub\s+(?:type|const|fn|static(?:\s+mut)?|struct|union|enum)\s+(\w+)"
)


def _collect_exports(src_dir: Path) -> dict[str, set[str]]:
    """Module name → set of identifier names it exports."""
    out: dict[str, set[str]] = {}
    for mod in _SHARED_MODULES:
        path = src_dir / f"{mod}.rs"
        if not path.exists():
            continue
        text = path.read_text()
        names: set[str] = set()
        for m in _PUB_TOP_RE.finditer(text):
            names.add(m.group(1))
        for em in _EXTERN_BLOCK_RE.finditer(text):
            for decl in _split_extern_body(em.group(2)):
                if ci := _classify(decl, path):
                    names.add(ci.name)
        if names:
            out[mod] = names
    return out


# ---------------------------------------------------------------------------
# Per-binary rewriter
# ---------------------------------------------------------------------------

def _cleanup_one(bin_path: Path, crate: str, available: dict[str, set[str]]) -> bool:
    text = bin_path.read_text()
    name_to_module: dict[str, str] = {}
    for mod, names in available.items():
        for n in names:
            name_to_module.setdefault(n, mod)

    used_mods: set[str] = set()
    new_text = text

    # 1. Strip top-level type/const/struct items already in shared modules.
    top_items = find_top_level_items(
        bin_path, ("type_item", "const_item", "struct_item")
    )
    strip_ranges = [(it.start_byte, it.end_byte, name_to_module[it.name])
                    for it in top_items if it.name in name_to_module]
    for start, end, mod in sorted(strip_ranges, key=lambda x: -x[0]):
        new_text = new_text[:start] + new_text[end:]
        used_mods.add(mod)

    # 2. Strip extern block items already in shared modules.
    for m in reversed(list(_EXTERN_BLOCK_RE.finditer(new_text))):
        kept = []
        for decl in _split_extern_body(m.group(2)):
            ci = _classify(decl, bin_path)
            if ci and ci.name in name_to_module:
                used_mods.add(name_to_module[ci.name])
                continue
            kept.append(decl)
        new_body = "".join(kept)
        if not new_body.strip():
            end = m.end() + (1 if m.end() < len(new_text) and new_text[m.end()] == "\n" else 0)
            new_text = new_text[:m.start()] + new_text[end:]
        else:
            indent = m.group(1)
            new_text = (new_text[:m.start()]
                        + f'{indent}extern "C" {{{new_body}\n{indent}}}'
                        + new_text[m.end():])

    if not used_mods:
        return False

    # 3. Insert `use ::<crate>::src::<mod>::*;` after the inner attrs.
    new_text = _insert_imports(new_text, crate, used_mods)
    new_text = re.sub(r"\n{3,}", "\n\n", new_text)
    bin_path.write_text(new_text)
    return True


def _insert_imports(text: str, crate: str, mods: set[str]) -> str:
    """Inject use statements after `#![...]` inner attrs and any existing
    `use ::<crate>;` lines (Stage 0 may have added one). Skip lines that
    already contain the same `use` text."""
    lines = text.splitlines(keepends=True)
    insert_at = 0
    for i, line in enumerate(lines):
        s = line.strip()
        if s.startswith("#!["):
            insert_at = i + 1
        elif s and not s.startswith("//"):
            break
    while insert_at < len(lines) and lines[insert_at].strip().startswith(("#[", "use ")):
        insert_at += 1

    new_lines = []
    existing = "".join(lines)
    for mod in sorted(mods):
        ul = f"use ::{crate}::src::{mod}::*;"
        if ul not in existing:
            new_lines.append(ul + "\n")

    if not new_lines:
        return text
    lines[insert_at:insert_at] = new_lines
    return "".join(lines)
