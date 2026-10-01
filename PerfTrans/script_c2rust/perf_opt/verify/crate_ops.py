"""Crate/harness working-copy + fair-build + path-dep-rewire + harness-compat
primitives.

Moved verbatim from perf_opt/bench_pipeline.py so verify/ is the independent
foundation: bench_pipeline / stage_a / stage_b all depend DOWN on this module;
it depends only on the cleanup/toolchain leaf utils (never on stage_a or
bench_pipeline). Behavior is identical to the bench_pipeline originals — this
is a pure relocation.
"""

from __future__ import annotations

import logging
import re
import shutil
import subprocess
from pathlib import Path

from stages.cleanup.cargo_utils import read_crate_name
from utils.toolchain import write_rust_toolchain

logger = logging.getLogger(__name__)


# ─── relocated from bench_pipeline.py (constants + functions, verbatim) ───
_PROFILE_BLOCK = """
# perf_opt fair-build profile — auto-injected by bench_pipeline.py.
[profile.release]
opt-level     = 3
lto           = "fat"
codegen-units = 1
panic         = "abort"
debug         = false
"""

# Canonical (key, literal-value) pairs that fair_build.audit() checks.
# Used when a [profile.release] block already exists and we need to top
# it up with whichever fair-build keys are missing — without clobbering
# values the upstream Cargo.toml already set correctly. Keep in sync
# with perf_opt/fair_build.py:_PROFILE_EXPECT_BIN.
_PROFILE_KEYS_CANON: list[tuple[str, str]] = [
    ("opt-level",     "3"),
    ("lto",           '"fat"'),
    ("codegen-units", "1"),
    ("panic",         '"abort"'),
    ("debug",         "false"),
]

_CARGO_CONFIG = """[build]
rustflags = ["-Ctarget-cpu=native"]
"""

_PROFILE_RELEASE_BLOCK_RE = re.compile(
    # Captures the entire [profile.release] table — from its header to
    # the next `^[` (start of the next TOML table) or EOF. Cargo.toml
    # files from c2rust output don't put `[` inside string values at
    # column 0, so this is safe.
    r'(?ms)^\[profile\.release\]\s*\n(?:(?!^\[).)*'
)
def _ensure_fair_build(crate_dir: Path) -> None:
    """Idempotently top up `Cargo.toml` + `.cargo/config.toml` so a fresh
    rsync 0_raw / 1_cleaned becomes a valid fair-build target.

    Cargo.toml: if `[profile.release]` is missing, append the canonical
    block; if it already exists, append any individual key that's
    missing (so an upstream block that only sets opt-level + lto still
    ends up with panic="abort" + debug=false + codegen-units=1).
    """
    write_rust_toolchain(crate_dir)
    cargo = crate_dir / "Cargo.toml"
    text = cargo.read_text()
    m = _PROFILE_RELEASE_BLOCK_RE.search(text)
    if m is None:
        text = text.rstrip() + "\n" + _PROFILE_BLOCK
        cargo.write_text(text)
    else:
        block_text = m.group(0)
        missing: list[str] = []
        for key, val in _PROFILE_KEYS_CANON:
            # Match the key at the start of any line within the block,
            # ignoring leading whitespace. `re.escape` covers the `-`.
            if not re.search(
                rf"(?m)^\s*{re.escape(key)}\s*=", block_text,
            ):
                missing.append(f"{key:<13} = {val}")
        if missing:
            new_block = block_text.rstrip() + "\n" + "\n".join(missing) + "\n"
            text = text[:m.start()] + new_block + text[m.end():]
            cargo.write_text(text)
    cargo_cfg_dir = crate_dir / ".cargo"
    cargo_cfg_dir.mkdir(exist_ok=True)
    cargo_cfg = cargo_cfg_dir / "config.toml"
    if cargo_cfg.exists():
        cfg_text = cargo_cfg.read_text()
        if "-Ctarget-cpu=native" not in cfg_text:
            # Existing config but missing the flag — append our block.
            # Cargo merges multiple `[build]` sections only at top level,
            # so we drop our `[build]` if one is already present, just
            # rewriting the rustflags line.
            if re.search(r'(?m)^\s*rustflags\s*=', cfg_text):
                cargo_cfg.write_text(re.sub(
                    r'(?m)^(\s*rustflags\s*=\s*).*$',
                    r'\1["-Ctarget-cpu=native"]', cfg_text,
                ))
            else:
                cargo_cfg.write_text(cfg_text.rstrip() + "\n\n" + _CARGO_CONFIG)
    else:
        cargo_cfg.write_text(_CARGO_CONFIG)
def _rsync_copy(src: Path, dst: Path, *, extra_ignore=()) -> None:
    """Mirror src into dst, excluding target/ and .git/ (we'll init a
    fresh git in dst). `extra_ignore` names — plus any `*.profraw` — are
    also skipped, so harness copies can drop coverage/build junk at copy
    time instead of copy-then-delete (copying tens of thousands of junk
    files is slow and, on a flaky FS, risks a partial copy). Done via
    Python (shutil) to avoid rsync dep."""
    if dst.exists():
        shutil.rmtree(dst)
    skip = {"target", ".git", ".perf_opt", ".cargo", *extra_ignore}
    def _ignore(d, names):
        return [n for n in names if n in skip or n.endswith(".profraw")]
    shutil.copytree(src, dst, ignore=_ignore)


def _git_init(crate_dir: Path, source_label: str = "1_cleaned") -> None:
    """Init a fresh git repo on `crate_dir` so a Stage can snapshot + rollback.

    Adds a `.gitignore` excluding `target/` and `.perf_opt/` so subsequent
    cargo builds (needed for the pre-Stage W2 baseline) don't make the
    working tree dirty — Stages refuse to run on a dirty tree, and we'd
    rather not couple the pipeline to `git status` fragility.

    `source_label`: caller-supplied tag for the initial commit message so
    later git-log readers know what stage the snapshot was taken from.
    - Stage A (input = 1_cleaned)   → "1_cleaned"  (default)
    - Stage A worktree pool worker  → "1_cleaned worker copy"
    - Stage B (input = 2_stage_a)   → "2_stage_a"
    Historically the message was hardcoded "snapshot from 0_raw" which
    silently misled Stage B consumers (audited 2026-06-03).
    """
    if (crate_dir / ".git").exists():
        return
    (crate_dir / ".gitignore").write_text(
        "target/\n.perf_opt/\n"
    )
    subprocess.run(["git", "init", "-q"], cwd=crate_dir, check=True)
    subprocess.run(
        ["git", "-c", "user.email=perf-opt@local",
         "-c", "user.name=perf-opt",
         "add", "-A"], cwd=crate_dir, check=True,
    )
    subprocess.run(
        ["git", "-c", "user.email=perf-opt@local",
         "-c", "user.name=perf-opt",
         "commit", "-q", "-m", f"snapshot from {source_label}"],
        cwd=crate_dir, check=True,
    )
_PATH_DEP_RE = re.compile(
    r'^([ \t]*)([A-Za-z_][A-Za-z_0-9-]*)\s*=\s*\{([^}]*)\}',
    re.MULTILINE,
)
# ─────────────────────────────────────────────────────────────────
# Harness-vs-stage compatibility shim
#
# c2rust pipeline stages may split / merge modules differently — 0_raw
# typically has a single `src/libcsv.rs` while a later stage may split
# into `src/{libcsv, c_structs, c_types, ffi}.rs`. If the harness was
# written against a specific stage's module layout, its `use lib::src::
# c_structs::Foo;` won't resolve against an earlier stage that has only
# `src/libcsv.rs`. We injects re-export shim modules so the harness can
# import its expected paths regardless of how lib has factored itself.
#
# This is a GENERIC build-compat step (akin to fair-build setup), NOT a
# performance hack and NOT project-specific — it activates only when the
# harness references a module the lib doesn't actually own, and it
# re-exports the lib's REAL module contents.
# ─────────────────────────────────────────────────────────────────

_HARNESS_USE_RE = re.compile(
    r'use\s+([A-Za-z_]\w*)\s*::\s*src\s*::\s*([A-Za-z_]\w*)\s*::',
)




def _ensure_harness_compat(crate_dir: Path, harness_dir: Path) -> None:
    """Scan harness rs files for `use <dep>::src::<mod>::…` paths whose
    `<mod>` doesn't actually exist in the lib's `src/`. For each missing
    module, inject a thin re-export shim in lib.rs so the harness still
    builds.

    Uses tree-sitter to locate the outer `pub mod src { ... }` block and
    enumerate its inner mod declarations — a brace-balanced parse is
    required because lib.rs may have nested `pub mod src { ... }` blocks
    (e.g. heman, whose C source is laid out in `src/src/`). A naive
    regex with `[^}]*` or `.*?` matches an inner block and injects the
    shim at the wrong nesting level, producing unresolved `super::ffi`.
    """
    if not (crate_dir / "src").is_dir():
        return
    needed: set[str] = set()
    for rs in harness_dir.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        try:
            text = rs.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for m in _HARNESS_USE_RE.finditer(text):
            needed.add(m.group(2))
    lib_rs = crate_dir / "lib.rs"
    lib_rs_text = lib_rs.read_text(encoding="utf-8")
    # tree-sitter offsets are BYTE offsets, so every slice below runs on the
    # bytes that were parsed. Indexing `lib_rs_text` with them shifts by one
    # position per non-ASCII character earlier in the file, cumulatively, and
    # the mod names come back as garbage — see `symbol_source._slice`.
    lib_rs_bytes = lib_rs_text.encode("utf-8")

    def _txt(node) -> str:
        return lib_rs_bytes[node.start_byte:node.end_byte].decode(
            "utf-8", "replace")

    # Locate the outermost `pub mod src { ... }` mod_item with tree-sitter.
    # We pick the FIRST mod_item at root scope named "src" — c2rust always
    # emits this at the top level of lib.rs.
    from tree_sitter import Language as _Lang, Parser as _P
    import tree_sitter_rust as _tsr
    parser = _P(_Lang(_tsr.language()))
    tree = parser.parse(lib_rs_bytes)

    outer_src: object | None = None
    for ch in tree.root_node.children:
        if ch.type != "mod_item":
            continue
        name_node = ch.child_by_field_name("name")
        if name_node is None:
            continue
        if _txt(name_node) != "src":
            continue
        body_node = ch.child_by_field_name("body")
        if body_node is None or body_node.type != "declaration_list":
            continue
        outer_src = ch
        break
    if outer_src is None:
        return

    # Enumerate every mod NAME declared at outer_src's top level — both
    # `pub mod X;` (semicolon) and `pub mod X { ... }` (inline block).
    # Inline blocks count: a harness `use lib::src::X::…` resolves even
    # if X is an inline submodule.
    declared: set[str] = set()
    for ch in outer_src.child_by_field_name("body").children:
        if ch.type != "mod_item":
            continue
        nn = ch.child_by_field_name("name")
        if nn is None:
            continue
        declared.add(_txt(nn))
    if not declared:
        return
    missing = sorted(needed - declared)
    if not missing:
        return

    # Pick a "main" module to re-export from. Prefer the largest .rs file
    # in src/{main_mod}.rs. Falls back to first declared name if no .rs
    # file is at src/ root (e.g. heman has src/src/ and src/kazmath/
    # subdirs but no src/{name}.rs at root).
    def _size_at(m: str) -> int:
        p = crate_dir / "src" / f"{m}.rs"
        try:
            return p.stat().st_size if p.is_file() else 0
        except OSError:
            return 0
    main_mod = max(declared, key=_size_at)
    if _size_at(main_mod) == 0:
        # No flat src/{name}.rs — fall back to a deterministic pick:
        # smallest alphabetical name. This is best-effort; the shim
        # generally only needs to satisfy the harness's compile-time
        # path resolution since it pulls only public symbols.
        main_mod = sorted(declared)[0]

    shim_lines = [
        f"    pub mod {m} {{ pub use super::{main_mod}::*; }}"
        for m in missing
    ]

    # Inject the shim lines immediately BEFORE outer_src's closing `}`.
    # outer_src.end_byte points one past the closing `}`. We splice the
    # text right before it (keeping the `}` intact).
    insert_pos = outer_src.end_byte - 1
    # Splice on BYTES: `end_byte` is a byte offset, and slicing the decoded
    # `str` with it lands in the wrong place once anything earlier in lib.rs
    # is non-ASCII — here that does not merely mislabel something, it writes
    # the shim into the middle of a token.
    payload = "\n" + "\n".join(shim_lines) + "\n"
    lib_rs.write_bytes(lib_rs_bytes[:insert_pos] + payload.encode("utf-8")
                       + lib_rs_bytes[insert_pos:])
    logger.info(
        f"[harness-compat] inject {len(missing)} shim module(s) "
        f"into {lib_rs.name}: {missing} → {main_mod}"
    )
def _rewire_path_dep(harness_cargo: Path, lib_dir: Path) -> None:
    """Rewire EVERY path-dep in `harness_cargo` whose target name matches
    `lib_dir`'s crate name OR is the sole path-dep, so the harness builds
    against the disposable `lib_dir`. If the harness dep_name differs
    from the actual crate name (e.g. dep name `libcsv_safe` but target
    crate is `libcsv_raw`), inject `package = "<crate_name>"` so cargo
    accepts the rename and the existing `use libcsv_safe::…` source
    keeps resolving without touching .rs files.
    """
    text = harness_cargo.read_text()
    crate_name = read_crate_name(lib_dir)

    def _replace(m: re.Match) -> str:
        indent, dep_name, body = m.group(1), m.group(2), m.group(3)
        # Only touch deps that already declare a `path = ...` field.
        if not re.search(r'\bpath\s*=', body):
            return m.group(0)
        # Update the path target.
        new_body = re.sub(
            r'(\bpath\s*=\s*)"[^"]+"',
            f'\\1"{lib_dir}"',
            body,
        )
        # Inject / fix the package field iff dep_name != actual crate name.
        if dep_name.replace("-", "_") != crate_name.replace("-", "_"):
            if re.search(r'\bpackage\s*=', new_body):
                new_body = re.sub(
                    r'(\bpackage\s*=\s*)"[^"]+"',
                    f'\\1"{crate_name}"',
                    new_body,
                )
            else:
                # Append a `package = "..."` cleanly after the path field.
                new_body = re.sub(
                    r'(\bpath\s*=\s*"[^"]+")',
                    f'\\1, package = "{crate_name}"',
                    new_body,
                    count=1,
                )
        return f"{indent}{dep_name} = {{{new_body}}}"

    text = _PATH_DEP_RE.sub(_replace, text)
    harness_cargo.write_text(text)
