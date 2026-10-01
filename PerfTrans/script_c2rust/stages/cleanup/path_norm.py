"""Path normalization pass: collapse fully-qualified `::core::ffi::c_X`
references to bare `c_X` identifiers under `use core::ffi::*;`.

This is purely cosmetic — it's safe because every supported target maps
`c_int = i32`, `c_void = ()` etc. via the same canonical aliases.

Operates on every .rs file in src/, including:
  - lib modules (blocksort.rs, ...)
  - generated dedup modules (c_types.rs, ...)
  - binary TUs (bzip2.rs, sortbench.rs, ...)

The `use core::ffi::*;` is inserted AFTER any inner attributes
(`#![feature(...)]`) so it's accepted as a valid item.
"""

from __future__ import annotations

import logging
import re
from pathlib import Path

logger = logging.getLogger(__name__)

# Match `::core::ffi::c_X` and capture the c_X identifier.
_FFI_PATH = re.compile(r"::\s*core::\s*ffi::(c_\w+)")
_USE_LINE = "use core::ffi::*;"


def normalize_paths(project_path: Path) -> int:
    """Returns count of files modified."""
    src_dir = project_path / "src"
    if not src_dir.is_dir():
        return 0
    affected = 0
    # rglob: nested layouts need recursive scan; flat layouts are unaffected
    # since their .rs files all live at top level.
    for rs in sorted(src_dir.rglob("*.rs")):
        text = rs.read_text()
        new_text, n = _FFI_PATH.subn(r"\1", text)
        if n == 0:
            continue
        new_text = _ensure_use_after_inner_attrs(new_text, _USE_LINE)
        rs.write_text(new_text)
        affected += 1
        logger.info(f"  [paths] {rs.name}: collapsed {n} occurrence(s)")
    return affected


def _ensure_use_after_inner_attrs(text: str, use_line: str) -> str:
    """Insert `use_line` after inner attrs (`#![...]`); no-op if already present."""
    if use_line in text:
        return text
    lines = text.splitlines(keepends=True)
    insert_at = 0
    for i, line in enumerate(lines):
        s = line.strip()
        if s.startswith("#!["):
            insert_at = i + 1
        elif s and not s.startswith("//"):
            break
    lines.insert(insert_at, use_line + "\n")
    return "".join(lines)
