"""c2rust 0.22.1 output compatibility patches.

c2rust 0.22.1 is the LATEST published version on crates.io (no newer
release exists). Two bugs in its output prevent compilation with any
modern Rust toolchain (verified against the project-pinned
nightly-2024-01-15):

  1. **SSE intrinsic typedef**: c2rust emits clang's *internal* alias
     `__m128i_u` (used inside `emmintrin.h` for the unaligned-load
     variant). Rust's `core::arch::x86_64` has no such type — only
     `__m128i`; unaligned loads/stores are expressed via the
     `_mm_loadu_si128` / `_mm_storeu_si128` intrinsics, not a separate
     type. Found in: brotli (SSE2 prefetch / hash routines).

  2. **Multi-byte string-literal initialization**: c2rust mistranslates
     the C idiom

         const unsigned char tbl[N] = "<N-byte string literal>";

     as

         static mut TBL: [c_uchar; N] = unsafe { [
             ::core::mem::transmute::<[u8;N],[c_uchar;N]>(*b"..."),  // ← returns [c_uchar; N]
             0, 0, ..., 0,                                            // ← (N-1) spurious zeros
         ] };

     The transmute already evaluates to the entire `[c_uchar; N]`; the
     surrounding array literal is wrong (mixed element types: `[c_uchar; N]`
     for the transmute, plain `c_uchar` for each `0`). Found in: libxml2
     (`encoding.rs` charset transcoding tables — 14 occurrences).

Both bugs are fixed on c2rust master (commits 8f344b2 / 32b9c0d
"Wrap transmuted string literals in const blocks" and 410c218
"Reduce repetition in simd function handling"), but unreleased.
Rather than build c2rust from source (which would risk regressions for
the 7 projects whose pipeline output is already validated end-to-end),
we patch the output here.

This module is invoked at the start of `stage1_cleanup`, BEFORE the
dedup passes. It is idempotent: if neither bug pattern is present in a
project (e.g. bzip2 / libcsv / lodepng), the passes do nothing.
"""
from __future__ import annotations

import logging
import re
import subprocess
from pathlib import Path

logger = logging.getLogger(__name__)


# Bug 1: __m128i_u → __m128i  (word-boundary match avoids touching e.g. __m128i_unrelated_name)
_M128I_U_RE = re.compile(r'\b__m128i_u\b')


def fix_m128i_u(project_path: Path) -> tuple[int, int]:
    """Replace every `__m128i_u` reference with `__m128i`.

    The two types are semantically identical in modern Rust — `__m128i_u`
    in clang's headers is just `__m128i` with `__aligned__(1)`, which
    Rust's `core::arch::x86_64::__m128i` already supports via the
    `_mm_loadu_*` / `_mm_storeu_*` accessor intrinsics.

    Returns (files_modified, total_replacements).
    """
    src = project_path / "src"
    if not src.is_dir():
        return (0, 0)

    files_modified = total_replacements = 0
    for rs in src.rglob("*.rs"):
        text = rs.read_text()
        new_text, n = _M128I_U_RE.subn('__m128i', text)
        if n:
            rs.write_text(new_text)
            files_modified += 1
            total_replacements += n
    return (files_modified, total_replacements)


# Bug 2: unsafe { [ transmute<[u8;N],[c_uchar;N]>(*b"..."), 0, 0, ... ] }
#        → unsafe { transmute<[u8;N],[c_uchar;N]>(*b"...") }
#
# Anchor on `unsafe { [` ... `] }` so we don't accidentally match any other
# array literal that happens to start with transmute.
_TRANSMUTE_ARRAY_WRAP_RE = re.compile(
    r'unsafe\s*\{\s*\[\s*'                                  # unsafe { [
    r'(::core::mem::transmute::<\s*'                        # capture transmute expr (group 1)
    r'\[u8;\s*\d+\],\s*'
    r'\[::core::ffi::c_uchar;\s*\d+\],?\s*'
    r'>\s*\([^()]*\))'                                      # (*b"…")
    r'\s*,'                                                 # comma after transmute
    r'(?:\s*0,)+'                                           # ≥1 trailing 0,
    r'\s*0?\s*'                                             # optional final 0 (no trailing comma)
    r'\]\s*\}',                                             # ] }
    re.DOTALL,
)


def fix_transmute_array_wrap(project_path: Path) -> tuple[int, int]:
    """Strip the spurious `[ … , 0, 0, … ]` wrapper around c2rust 0.22.1's
    transmute-from-byte-string output.

    The transmute itself is correct — it converts the `[u8; N]` byte-string
    literal into `[c_uchar; N]`. c2rust then wrongly wraps that result in
    another `[ … ]` array literal whose other elements are individual
    `0_u8` padding tokens, which would only type-check if `c_uchar`-array
    could appear interchangeably with `c_uchar` in the same array literal.

    Returns (files_modified, total_replacements).
    """
    src = project_path / "src"
    if not src.is_dir():
        return (0, 0)

    files_modified = total_replacements = 0
    for rs in src.rglob("*.rs"):
        text = rs.read_text()
        new_text, n = _TRANSMUTE_ARRAY_WRAP_RE.subn(
            lambda m: 'unsafe { ' + m.group(1) + ' }', text)
        if n:
            rs.write_text(new_text)
            files_modified += 1
            total_replacements += n
    return (files_modified, total_replacements)


# Bug 3: missing `as size_t` cast in `while <i:size_t> < (*<s>).<field>[<idx>] {`
#        where `<field>: [uint64_t; N]` (or other non-size_t fixed-size type).
#
# c2rust 0.22.1 usually emits the explicit `as size_t` cast at field-deref
# read sites (line 4227 in brotli/dec/decode.rs is correct). But at a small
# number of `while`-condition sites, the cast is missing — the C source's
# implicit size_t↔uint64_t coercion is dropped on translation.
#
# Observed sites (all in brotli):
#   - dec/decode.rs:2373  while i < (*s).num_block_types[0 ...] {
# Generalized pattern: `while <ident> < (*<ident>).<field>[<...>]` immediately
# followed by `{`. We append `as size_t` before the `{` so usize <-> u64
# comparison type-checks. Safe even when the field IS size_t — `x as size_t`
# is a no-op cast.
_WHILE_FIELD_DEREF_MISSING_CAST_RE = re.compile(
    r'(while\s+\w+\s+<\s+\(\*\w+\)\.\w+\[[^\[\]]+\])(\s*)\{'
)


def fix_while_field_deref_cast(project_path: Path) -> tuple[int, int]:
    """Insert missing `as size_t` cast in `while <i> < (*<s>).<field>[<idx>]`.

    This is the well-known c2rust 0.22.1 oversight where a while-loop bound
    field-read drops the size_t cast that's present at most other read sites.
    See module docstring for the bug taxonomy.

    Returns (files_modified, total_replacements).
    """
    src = project_path / "src"
    if not src.is_dir():
        return (0, 0)

    files_modified = total_replacements = 0
    for rs in src.rglob("*.rs"):
        text = rs.read_text()
        new_text, n = _WHILE_FIELD_DEREF_MISSING_CAST_RE.subn(
            lambda m: f'{m.group(1)} as size_t{m.group(2)}{{', text)
        if n:
            rs.write_text(new_text)
            files_modified += 1
            total_replacements += n
    return (files_modified, total_replacements)


# Bug 4: missing system-library link directives in build.rs
#
# c2rust generates the `extern "C" { fn foo(...); }` declarations but has
# no idea which system libraries the C project linked against. The default
# c2rust-emitted build.rs is just an empty `fn main()` per cfg. For projects
# that need libraries OUTSIDE libc (libevent, libssl, …), `cargo build`
# fails at LINK time with "undefined reference to …" — even though the
# Rust source itself type-checks fine.
#
# Per-project library list. Keys match the directory name under
# `dataset_trans_process/<key>/` (which is also `project_path.parent.name`
# when `project_path` points at a stage's output directory like 1_cleaned).
#
# Add new projects here only when `cargo build` reports linker errors
# for symbols that are in a known system library.
_PROJECT_LINK_LIBS: dict[str, tuple[str, ...]] = {
    # tmux's autoconf LIBS line (see dataset_source/tmux/config.log):
    #   LIBS='-lutil -ltinfo -levent_core -lm -lresolv'
    # Without these every event_*/evbuffer_* (libevent), forkpty (libutil),
    # cur_term/setupterm/tigetstr (libtinfo), __b64_pton/__b64_ntop
    # (libresolv) shows up as an undefined reference at link.
    "tmux": ("util", "tinfo", "event_core", "m", "resolv"),
}


def inject_link_libs(project_path: Path) -> int:
    """Inject `println!("cargo:rustc-link-lib=<lib>");` into build.rs for
    projects whose C source links against non-libc system libraries.

    The project key is the parent directory name of `project_path` (so
    `…/tmux/1_cleaned` → key `tmux`). Returns the number of lib directives
    added (0 if no libs configured or all already present — idempotent).
    """
    proj_key = project_path.parent.name
    libs = _PROJECT_LINK_LIBS.get(proj_key, ())
    if not libs:
        return 0

    build_rs = project_path / "build.rs"
    if not build_rs.exists():
        return 0

    text = build_rs.read_text()
    new_directives: list[str] = []
    for lib in libs:
        directive = f'    println!("cargo:rustc-link-lib={lib}");'
        if directive in text:
            continue
        new_directives.append(directive)
    if not new_directives:
        return 0

    # Insert into the FIRST `fn main() { ... }` body (the `cfg(unix, not macos)`
    # arm — our target). The closing `}` for that fn is identified as the
    # first `}` after the `fn main()` token, which we replace with our
    # directives + `}`.
    pattern = re.compile(r'(#\[cfg\(all\(unix.*?\)\)\]\s*fn\s+main\s*\(\)\s*\{[^}]*?)(\n\})', re.DOTALL)
    if not pattern.search(text):
        return 0
    inserted = '\n' + '\n'.join(new_directives)
    new_text = pattern.sub(lambda m: m.group(1) + inserted + m.group(2), text, count=1)
    build_rs.write_text(new_text)
    return len(new_directives)


def apply_rustc_machine_applicable_fixes(
    project_path: Path,
    *,
    max_iters: int = 4,
    allowed_codes: tuple[str, ...] = ("E0308", "E0277", "E0605", "E0382", "E0061"),
) -> int:
    """Run `cargo check --message-format=json` and apply every
    MachineApplicable suggestion rustc emits for the diagnostics whose
    error code is in `allowed_codes`. Iterate to fixpoint (or
    `max_iters` rounds, whichever comes first). Returns total edits
    applied across all iterations.

    Background: c2rust 0.22.1 sometimes drops the explicit integer
    casts that C performs implicitly (e.g. brotli's
    `BrotliWriteBits(max_bits, symbols[i as usize])` passes `usize`
    where `BrotliWriteBits` declares `bits: uint64_t`). rustc not only
    detects these — it also emits a `MachineApplicable` suggestion to
    fix each one (typically `.try_into().unwrap()` or `as <type>`).
    Applying those suggestions verbatim is exactly what `cargo fix`
    does for warnings, just lifted to errors.

    We restrict to a known-safe subset of error codes to keep the
    blast radius small: E0308 (mismatched types), E0277 (trait bound
    not satisfied), E0605 (non-primitive cast), E0382 (use of moved
    value), E0061 (wrong arg count). All commonly emit MachineApplicable
    suggestions and rarely false-positive in practice.
    """
    import json as _json

    total_edits = 0
    for iteration in range(max_iters):
        proc = subprocess.run(
            ["cargo", "check", "--release", "--message-format=json"],
            cwd=str(project_path), capture_output=True, text=True, timeout=600,
        )
        # cargo emits one JSON object per line on stdout; we only care
        # about `reason == "compiler-message"`. Errors that didn't come
        # with MachineApplicable suggestions are skipped silently.
        # (file, byte_start, byte_end, replacement)
        edits: list[tuple[Path, int, int, str]] = []

        def _collect(node, parent_code: str | None):
            """Walk a diagnostic + its children for MachineApplicable
            replacements. rustc nests its suggestions inside `children`
            (one child per suggestion); the actual span/replacement is
            on the child's `spans[]`."""
            code = ((node.get("code") or {}).get("code")) or parent_code
            for span in node.get("spans") or []:
                if span.get("suggestion_applicability") != "MachineApplicable":
                    continue
                repl = span.get("suggested_replacement")
                if repl is None:
                    continue
                if code not in allowed_codes:
                    # rustc may attach a suggestion under a non-allowed
                    # error code (e.g. as a hint chain). Skip.
                    continue
                fname = span.get("file_name")
                if not fname:
                    continue
                fpath = (project_path / fname).resolve()
                if not fpath.is_file():
                    continue
                edits.append((fpath, int(span["byte_start"]),
                              int(span["byte_end"]), repl))
            for child in node.get("children") or []:
                _collect(child, code)

        for line in (proc.stdout or "").splitlines():
            line = line.strip()
            if not line.startswith("{"):
                continue
            try:
                msg = _json.loads(line)
            except _json.JSONDecodeError:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            diag = msg.get("message")
            if not diag:
                continue
            _collect(diag, None)

        if not edits:
            break

        # Dedup edits — rustc occasionally emits the same suggestion
        # via multiple children paths (same file, same byte range).
        seen: set[tuple[Path, int, int]] = set()
        uniq: list[tuple[Path, int, int, str]] = []
        for e in edits:
            key = (e[0], e[1], e[2])
            if key in seen:
                continue
            seen.add(key)
            uniq.append(e)

        # Apply per-file, right-to-left so earlier offsets stay valid.
        by_file: dict[Path, list[tuple[int, int, str]]] = {}
        for f, s, e, r in uniq:
            by_file.setdefault(f, []).append((s, e, r))
        for f, batch in by_file.items():
            batch.sort(key=lambda x: -x[0])
            src = f.read_bytes()
            for s, e, r in batch:
                if s < 0 or e > len(src) or s > e:
                    continue
                src = src[:s] + r.encode("utf-8") + src[e:]
            f.write_bytes(src)

        total_edits += len(uniq)
        logger.info(
            f"[c2rust-compat] rustc-machine-applicable iter {iteration + 1}: "
            f"applied {len(uniq)} suggestion(s) across {len(by_file)} file(s)"
        )

    return total_edits


def apply_all(project_path: Path) -> None:
    """Apply every c2rust 0.22.1 compatibility patch to the project.

    Logs one line per non-trivial patch. Idempotent — re-running on
    already-patched output is a no-op.
    """
    f, n = fix_m128i_u(project_path)
    if n:
        logger.info(f"[c2rust-compat] __m128i_u → __m128i: {n} replacements in {f} file(s)")
    f, n = fix_transmute_array_wrap(project_path)
    if n:
        logger.info(f"[c2rust-compat] transmute array-wrap fix: {n} sites in {f} file(s)")
    f, n = fix_while_field_deref_cast(project_path)
    if n:
        logger.info(f"[c2rust-compat] while-loop missing `as size_t` cast: {n} sites in {f} file(s)")
    n = inject_link_libs(project_path)
    if n:
        logger.info(f"[c2rust-compat] injected {n} link-lib directive(s) into build.rs")
    # Finally — pick up anything rustc itself can autofix. This catches
    # c2rust's missing implicit-int-cast pattern (brotli E0308) and
    # similar one-off output bugs without us hand-coding each
    # pattern. Fixpoint-iterated; safe for re-runs (no-op when there
    # are no MachineApplicable suggestions left).
    n = apply_rustc_machine_applicable_fixes(project_path)
    if n:
        logger.info(f"[c2rust-compat] rustc-machine-applicable: {n} total edit(s) applied")
