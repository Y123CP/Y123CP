"""Pre-Stage-A `extern "C" {}` block normalization.

c2rust translates consumer code (harness, self-bin) with `extern "C" {
fn foo(...); ... }` blocks. If Stage A then strips `extern "C"` from
the lib's `foo`, the consumer's extern block becomes a stale linker
contract → undefined reference. We pre-normalize: every fn / opaque
type declared in an `extern "C" {}` block that is ALSO a public lib
fn / type gets rewritten to a `use <crate>::<mod>::<name>;` import,
removing the FFI contract upfront. Unknown names (libc, system) stay
in the block.

Two variants — same skeleton, different `<crate>` path:

  · `normalize_harness_extern_blocks` — harness is in a SIBLING crate
    that depends on lib via path-dep; uses the dep's alias name
    (resolved from harness Cargo.toml's `[dependencies]` table).

  · `normalize_self_bin_extern_blocks` — bin is in the SAME crate as
    lib (e.g. lil's 0_raw bundles `src/main.rs` as `[[bin]]`); uses
    the lib's own crate name.

Plus `strip_staticlib_from_crate_type` — removes `"staticlib"` /
`"cdylib"` from `[lib].crate-type` so the analyzer no longer treats
the crate as "ships to C consumers" once the FFI contract is gone.
"""

from __future__ import annotations

import logging
import re
import tomllib
from pathlib import Path

from .cargo_utils import read_crate_name
from .scan import scan_lib_public_fns, scan_lib_public_types

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Shared regexes
# ---------------------------------------------------------------------------

# Match a top-level `extern "C" { ... }` block (single-level body — c2rust
# never nests `{}` inside extern blocks, so a non-greedy `[^}]*` suffices).
_EXTERN_BLOCK_C = re.compile(
    r'(?P<lead>[ \t]*)extern\s+"C"\s*\{(?P<body>[^}]*)\}',
    re.MULTILINE | re.DOTALL,
)

# Match `(pub) fn NAME(` inside an extern block. The `pubmod` group eats
# the optional `pub` so a re-declaration with explicit `pub fn …` is
# still recognized.
_BLOCK_FN_DECL = re.compile(
    r'(?P<pre>[ \t]*)(?P<pubmod>pub\s+)?fn\s+(?P<name>[A-Za-z_]\w*)\s*\(',
)

# Match `pub type NAME;` inside an extern "C" block — c2rust's idiom
# for opaque extern types.
_BLOCK_TYPE_DECL = re.compile(
    r'[ \t]*pub\s+type\s+(?P<name>[A-Za-z_]\w*)\s*;',
)


# ---------------------------------------------------------------------------
# Cargo dep-name resolution (harness only)
# ---------------------------------------------------------------------------

def _harness_dep_name(harness_cargo: Path, target_crate_name: str) -> str:
    """Return the harness's dep alias for the lib crate (might be the
    same as crate name or might be a rename via `package = "..."`)."""
    data = tomllib.loads(harness_cargo.read_text())
    deps = data.get("dependencies", {})
    for name, spec in deps.items():
        if isinstance(spec, dict):
            pkg = spec.get("package")
            if pkg == target_crate_name or name == target_crate_name:
                return name
    return target_crate_name


# ---------------------------------------------------------------------------
# Pass 1: harness extern-block normalization
# ---------------------------------------------------------------------------

def normalize_harness_extern_blocks(crate_dir: Path, harness_dir: Path) -> None:
    """Rewrite same-crate `extern "C" {}` declarations in harness sources
    into proper `use` imports — see module docstring above."""
    lib_fns = scan_lib_public_fns(crate_dir)
    if not lib_fns:
        return
    crate_name = read_crate_name(crate_dir)
    dep_name = _harness_dep_name(harness_dir / "Cargo.toml", crate_name)
    touched = 0
    for rs in harness_dir.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        text = rs.read_text(encoding="utf-8", errors="replace")
        new_uses: list[str] = []
        modified = False

        def _per_block(m: re.Match) -> str:
            nonlocal modified
            body = m.group("body")
            lead = m.group("lead")
            kept_decls: list[str] = []
            # Walk declarations
            pos = 0
            for dm in _BLOCK_FN_DECL.finditer(body):
                name = dm.group("name")
                if name in lib_fns:
                    # Determine declaration span (paren-balance + trailing ;)
                    paren_open = dm.end() - 1
                    depth = 0
                    i = paren_open
                    while i < len(body):
                        c = body[i]
                        if c == "(":
                            depth += 1
                        elif c == ")":
                            depth -= 1
                            if depth == 0:
                                i += 1
                                break
                        i += 1
                    sc = body.find(";", i)
                    if sc < 0:
                        kept_decls.append(body[pos:])
                        pos = len(body)
                        break
                    # Skip this declaration entirely.
                    decl_start = dm.start()
                    kept_decls.append(body[pos:decl_start])
                    pos = sc + 1
                    if pos < len(body) and body[pos] == "\n":
                        pos += 1
                    # lib_fns[name] is already a `::`-joined Rust mod path
                    # rooted at the crate (e.g. "src::lil",
                    # "src::src::ops") — see scan_lib_public_fns.
                    new_uses.append(
                        f"use {dep_name}::{lib_fns[name]}::{name};"
                    )
                    modified = True
                else:
                    # Move past this decl without changing.
                    pass
            kept_decls.append(body[pos:])
            new_body = "".join(kept_decls)
            if not new_body.strip():
                return ""  # block fully empty → drop
            return f"{lead}extern \"C\" {{{new_body}}}"

        new_text = _EXTERN_BLOCK_C.sub(_per_block, text)
        if not modified:
            continue
        # Inject use lines right after the last existing `use` line.
        lines = new_text.split("\n")
        last_use = -1
        for i, ln in enumerate(lines[:200]):
            if ln.startswith("use "):
                last_use = i
        insert_at = (last_use + 1) if last_use >= 0 else 0
        # Dedup against existing uses.
        existing_text = new_text
        deduped = [u for u in new_uses if u not in existing_text]
        if deduped:
            lines[insert_at:insert_at] = deduped
            new_text = "\n".join(lines)
        rs.write_text(new_text, encoding="utf-8")
        touched += 1
    if touched:
        logger.info(
            f"[harness-compat] normalized extern \"C\" blocks in "
            f"{touched} harness file(s); {len(set([u for u in new_uses]))} "
            f"new use imports total"
        )


# ---------------------------------------------------------------------------
# Pass 2: self-bin extern-block normalization
# ---------------------------------------------------------------------------

def normalize_self_bin_extern_blocks(crate_dir: Path) -> None:
    """Self-bin variant of `normalize_harness_extern_blocks`.

    When the bin lives in the same crate as the lib (e.g. lil's 0_raw
    bundles `src/main.rs` as a `[[bin]]`), c2rust emits a big
    `extern "C" {}` block at the top of main.rs declaring BOTH lib's
    public fns AND lib's extern-type names. E1 strip on those lib fns
    then triggers ABI mismatch + extern-type nominal divergence at
    every call site. We move both kinds of decl into `use` imports
    rooted at the lib crate itself (the bin auto-links the lib via
    Cargo's [[bin]] + [lib] association).

    Libc / system fns and unrelated types stay in the extern block.

    Found via lil 2026-05-25: 15/90 candidates frozen at check, all
    lil_* public API fns. After this fix, those resolve to lib's
    `pub struct _lil_t` and the extern decl is removed cleanly.
    """
    lib_fns = scan_lib_public_fns(crate_dir)
    lib_types = scan_lib_public_types(crate_dir)
    if not lib_fns and not lib_types:
        return
    crate_name = read_crate_name(crate_dir)

    # Collect bin sources to normalize: [[bin]] paths + src/bin/*.rs.
    bin_paths: list[Path] = []
    cargo = crate_dir / "Cargo.toml"
    if cargo.is_file():
        try:
            data = tomllib.loads(cargo.read_text(encoding="utf-8"))
            bins = data.get("bin", [])
            if isinstance(bins, list):
                for entry in bins:
                    p = entry.get("path") if isinstance(entry, dict) else None
                    if p:
                        bp = (crate_dir / p).resolve()
                        if bp.is_file():
                            bin_paths.append(bp)
        except (OSError, tomllib.TOMLDecodeError):
            pass
    src_bin = crate_dir / "src" / "bin"
    if src_bin.is_dir():
        bin_paths.extend(src_bin.glob("*.rs"))
    main_rs = crate_dir / "src" / "main.rs"
    if main_rs.is_file() and main_rs.resolve() not in [b.resolve() for b in bin_paths]:
        bin_paths.append(main_rs)
    if not bin_paths:
        return

    touched = 0
    use_imports_total = 0
    for rs in bin_paths:
        text = rs.read_text(encoding="utf-8", errors="replace")
        new_uses: list[str] = []
        modified = False

        def _per_block(m: re.Match) -> str:
            nonlocal modified
            body = m.group("body")
            lead = m.group("lead")
            # Build an EXCLUDE-byte-set for fn decls + type decls we want
            # to remove. Then concatenate everything NOT excluded.
            removals: list[tuple[int, int]] = []  # (start, end) inclusive
            # Fn decls — span is from match start to the trailing semicolon
            for dm in _BLOCK_FN_DECL.finditer(body):
                name = dm.group("name")
                if name not in lib_fns:
                    continue
                # Walk through `(...)` then `;`.
                paren_open = dm.end() - 1
                depth = 0
                i = paren_open
                while i < len(body):
                    c = body[i]
                    if c == "(":
                        depth += 1
                    elif c == ")":
                        depth -= 1
                        if depth == 0:
                            i += 1
                            break
                    i += 1
                sc = body.find(";", i)
                if sc < 0:
                    continue
                end = sc + 1
                if end < len(body) and body[end] == "\n":
                    end += 1
                removals.append((dm.start(), end))
                new_uses.append(
                    f"use {crate_name}::{lib_fns[name]}::{name};"
                )
                modified = True
            # Type decls — `pub type X;`
            for dm in _BLOCK_TYPE_DECL.finditer(body):
                name = dm.group("name")
                if name not in lib_types:
                    continue
                end = dm.end()
                if end < len(body) and body[end] == "\n":
                    end += 1
                removals.append((dm.start(), end))
                new_uses.append(
                    f"use {crate_name}::{lib_types[name]}::{name};"
                )
                modified = True
            if not removals:
                return m.group(0)
            removals.sort()
            pieces = []
            pos = 0
            for (s, e) in removals:
                if s > pos:
                    pieces.append(body[pos:s])
                pos = e
            pieces.append(body[pos:])
            new_body = "".join(pieces)
            if not new_body.strip():
                return ""  # block fully empty → drop
            return f"{lead}extern \"C\" {{{new_body}}}"

        new_text = _EXTERN_BLOCK_C.sub(_per_block, text)
        if not modified:
            continue
        lines = new_text.split("\n")
        last_use = -1
        for i, ln in enumerate(lines[:200]):
            if ln.startswith("use "):
                last_use = i
        insert_at = (last_use + 1) if last_use >= 0 else 0
        deduped = [u for u in dict.fromkeys(new_uses) if u not in new_text]
        if deduped:
            lines[insert_at:insert_at] = deduped
            new_text = "\n".join(lines)
        rs.write_text(new_text, encoding="utf-8")
        touched += 1
        use_imports_total += len(deduped)
    if touched:
        logger.info(
            f"[harness-compat] normalized self-bin extern \"C\" blocks in "
            f"{touched} bin file(s); {use_imports_total} new use imports total"
        )


# ---------------------------------------------------------------------------
# Pass 3: strip `"staticlib"` / `"cdylib"` from `[lib].crate-type`
# ---------------------------------------------------------------------------

def strip_staticlib_from_crate_type(crate_dir: Path) -> None:
    """Remove "staticlib" from `[lib].crate-type` in crate_dir's Cargo.toml.

    Called AFTER `normalize_harness_extern_blocks` so the lib is now
    consumed via Rust path-deps (use imports), not C ABI. The analyzer
    classifier freezes ALL extern "C" fns when crate-type contains
    "staticlib"/"cdylib" AND there's no [[bin]] (treated as "ships to C
    consumers"). For our pipeline that's a false positive — the consumer
    is the workload harness, which uses Rust paths. Stripping staticlib
    here lets analyzer correctly treat the crate as Rust-only.

    Found via heman 2026-05-25: 302 candidates → 0 safe-to-strip because
    heman 0_raw Cargo.toml has `crate-type = ["staticlib", "rlib"]` and
    no [[bin]]; libcsv works because it has a `[[bin]]` (the c2rust test
    bin) — heman lacks one.
    """
    cargo = crate_dir / "Cargo.toml"
    if not cargo.is_file():
        return
    text = cargo.read_text(encoding="utf-8")
    # Match the crate-type line; preserve indentation + quoting style.
    m = re.search(
        r'(?P<lead>\bcrate-type\s*=\s*)(?P<list>\[[^\]]*\])',
        text,
    )
    if m is None:
        return
    listed = m.group("list")
    # Strip "staticlib" / "cdylib" entries.
    new_list = re.sub(
        r'"(?:staticlib|cdylib)"\s*,?\s*',
        '',
        listed,
    )
    # Tidy trailing commas + multi-spaces.
    new_list = re.sub(r',\s*\]', ']', new_list)
    new_list = re.sub(r'\[\s*,', '[', new_list)
    if new_list == listed:
        return
    new_text = text[:m.start("list")] + new_list + text[m.end("list"):]
    cargo.write_text(new_text, encoding="utf-8")
    logger.info(
        f"[harness-compat] stripped staticlib/cdylib from "
        f"{cargo.relative_to(crate_dir.parent)}'s crate-type "
        f"(lib is now Rust-consumed via harness path-dep)"
    )
