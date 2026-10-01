"""Milestone C-3 — drop extern items whose name is not referenced anywhere
else in the file.

After Milestone C extern_dedup runs, each lib file's `extern "C" { ... }`
block contains only file-specific declarations. Some of those are inherited
from the union of the original C `#include`s and not actually used in this
TU. We catch them by tokenizing the rest of the file and pruning extern
items whose name appears 0 times outside their own declaration.

Conservatively scoped: ONLY prunes extern items, not top-level public items
(those may be referenced cross-module via `use super::*` and our local
search would miss it).
"""

from __future__ import annotations

import logging
import re
from pathlib import Path

from .extern_dedup import (
    _EXTERN_BLOCK_RE, _candidate_files, _classify, _split_extern_body,
)

logger = logging.getLogger(__name__)


def prune_unused_externs(project_path: Path) -> int:
    """Returns the total number of extern decls pruned across all lib files."""
    total = 0
    files_modified = 0
    for f in _candidate_files(project_path):
        n = _prune_file(f)
        if n > 0:
            files_modified += 1
            total += n
    if total:
        logger.info(f"[extern prune] removed {total} unused decl(s) "
                    f"across {files_modified} file(s)")
    else:
        logger.info("[extern prune] nothing to prune")
    return total


def _prune_file(file: Path) -> int:
    text = file.read_text()
    matches = list(_EXTERN_BLOCK_RE.finditer(text))
    if not matches:
        return 0

    # Build a (decl_name -> set of byte ranges occupied by THAT decl) map. We
    # need this because each candidate type/fn must be checked against a
    # search corpus that masks ONLY its own declaration text — other decls
    # in the same extern block (including fn signatures that reference the
    # type) MUST stay visible. Previous version blanked every extern block
    # entirely, which caused valid `pub type X;` to be pruned away when its
    # only user was a sibling `fn foo() -> *mut X;` in the same block —
    # leaving the fn signature dangling and the lib uncompilable. Observed
    # on tmux's `options_entry` / `tmuxproc` (opaque types referenced only
    # by extern fn signatures in the same block).
    decl_ranges: dict[str, list[tuple[int, int]]] = {}
    for m in matches:
        body = m.group(2)
        body_offset = m.start(2)
        cursor = 0
        for decl in _split_extern_body(body):
            decl_start = body_offset + cursor
            decl_end = decl_start + len(decl)
            cursor += len(decl)
            ci = _classify(decl, file)
            if ci is None:
                continue
            decl_ranges.setdefault(ci.name, []).append((decl_start, decl_end))

    def _is_referenced(name: str) -> bool:
        """True iff `name` appears anywhere in the file OUTSIDE its own
        declaration byte-ranges. A re-declaration of the same name in
        another extern block (which dedup may have left behind) counts as
        a "use" too — that's fine, the duplicate decl preserves the type."""
        my_ranges = decl_ranges.get(name, [])
        corpus = list(text)
        for s, e in my_ranges:
            for i in range(s, e):
                corpus[i] = " "
        search_text = "".join(corpus)
        return bool(re.search(rf"\b{re.escape(name)}\b", search_text))

    pruned = 0
    new_text = text
    for m in reversed(matches):
        body = m.group(2)
        kept_decls = []
        for decl in _split_extern_body(body):
            ci = _classify(decl, file)
            if ci is None:
                kept_decls.append(decl)  # unparsable line — keep
                continue
            if _is_referenced(ci.name):
                kept_decls.append(decl)
            else:
                pruned += 1

        new_body = "".join(kept_decls)
        if not new_body.strip():
            end = m.end() + (1 if m.end() < len(new_text) and new_text[m.end()] == "\n" else 0)
            new_text = new_text[:m.start()] + new_text[end:]
        else:
            indent = m.group(1)
            new_text = (new_text[:m.start()]
                        + f'{indent}extern "C" {{{new_body}\n{indent}}}'
                        + new_text[m.end():])

    new_text = re.sub(r"\n{3,}", "\n\n", new_text)
    if new_text != text:
        file.write_text(new_text)
    return pruned
