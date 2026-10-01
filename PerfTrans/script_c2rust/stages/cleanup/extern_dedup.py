"""Milestone C — dedup items inside `extern "C" { ... }` blocks.

c2rust replicates libc/opaque-type declarations in EVERY .rs file that
references them. Splitting them into shared modules has two payoffs:

  1. **c_extern_types.rs** holds opaque types like `_IO_marker`, `_IO_codecvt`,
     `_IO_wide_data`. Once these are out of per-file scope, the struct-dedup
     pass can finally migrate `_IO_FILE` (whose fields reference them).

  2. **ffi.rs** holds libc fns (`malloc`, `free`, `fopen`, `fprintf`, ...)
     and global statics (`stdin`, `stdout`, `stderr`). 8+ files used to
     redeclare the same ~30 libc fns each — this kills hundreds of lines.

Migration policy:
  - Items present in ≥2 files with identical body → migrate (both libc
    and project-internal cross-TU decls qualify).
  - Items present in only 1 file: migrate iff name is in the curated
    libc/POSIX set (`_LIBC_FNS` / `_LIBC_STATICS`). This catches single-bin
    libc decls (e.g. `bzip2.rs` having ~30 file/syscall decls used only
    by the binary entry point) so the FFI boundary lives in one place.
  - Project-internal cross-TU decls present only in 1 file (e.g. a binary
    referencing a single lib symbol via `extern "C"`) stay put — they're
    not FFI in the foreign sense, and Stage 2 should re-route them to
    proper Rust imports later.
"""

from __future__ import annotations

import logging
import re
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger(__name__)


# Match `extern "C" { ... }` block at top level. group(1) = leading indent,
# group(2) = block body (excluding the wrapping braces). Brace-balance is
# checked at parse time — c2rust output never nests `{}` inside extern.
_EXTERN_BLOCK_RE = re.compile(
    r'(?ms)^([ \t]*)extern\s+"C"\s*\{(.*?)\n[ \t]*\}'
)


@dataclass(frozen=True)
class ExternItem:
    kind: str        # "type" | "fn" | "static"
    name: str
    body: str        # full declaration text incl. its leading indent
    file: Path


def _norm(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


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


def _classify(decl: str, file: Path) -> ExternItem | None:
    stripped = decl.strip()
    if not stripped:
        return None
    if m := re.match(r"pub\s+type\s+(\w+)\s*;\s*$", stripped):
        return ExternItem("type", m.group(1), decl, file)
    if m := re.match(r"(?:pub\s+)?fn\s+(\w+)", stripped):
        return ExternItem("fn", m.group(1), decl, file)
    if m := re.match(r"(?:pub\s+)?static\s+(?:mut\s+)?(\w+)", stripped):
        return ExternItem("static", m.group(1), decl, file)
    return None


from .cargo_utils import (
    bin_paths as _bin_paths, insert_use_after_inner_attrs, use_line_for,
)


_GENERATED = {"c_types.rs", "c_consts.rs", "c_structs.rs",
              "ffi.rs", "c_extern_types.rs"}


# ---------------------------------------------------------------------------
# Curated libc / POSIX / glibc symbol set.
#
# These names are migrated to ffi.rs even when present in only ONE file.
# The list is conservative — only names that are unambiguously libc/POSIX
# in any C codebase. Project-private symbols that happen to share a name
# with a libc fn are not in scope of this set (and would be ambiguous; if
# c2rust emitted them as `extern "C"` decls, they're FFI by intent).
# ---------------------------------------------------------------------------

_LIBC_FNS = frozenset({
    # string.h
    "memcpy", "memmove", "memset", "memcmp", "strcat", "strcmp", "strcpy",
    "strlen", "strncat", "strncpy", "strncmp", "strchr", "strrchr", "strstr",
    "strerror", "strdup", "strndup", "strtok", "strspn", "strcspn", "strpbrk",
    # stdio.h
    "fopen", "freopen", "fclose", "fread", "fwrite", "fprintf", "printf",
    "sprintf", "snprintf", "fflush", "fseek", "ftell", "fputc", "fgetc",
    "fputs", "fgets", "ferror", "feof", "clearerr", "fdopen", "ungetc",
    "perror", "remove", "rename", "rewind", "fileno", "getc", "putc",
    "getchar", "putchar", "puts", "vfprintf", "vsprintf", "vsnprintf",
    "setvbuf", "setbuf", "tmpfile", "tmpnam",
    # stdlib.h
    "malloc", "free", "calloc", "realloc", "exit", "_Exit", "abort",
    "atoi", "atol", "atoll", "atof", "getenv", "setenv", "unsetenv",
    "system", "strtol", "strtod", "strtof", "strtoul", "strtoll", "strtoull",
    "qsort", "bsearch", "abs", "labs", "llabs", "div", "ldiv", "rand", "srand",
    # unistd.h / sys/stat.h
    "open", "close", "read", "write", "lseek", "unlink", "stat", "fstat",
    "lstat", "chmod", "fchmod", "chown", "fchown", "isatty", "access",
    "getcwd", "chdir", "getpid", "getppid", "fork", "execv", "execvp",
    "execve", "pipe", "dup", "dup2", "wait", "waitpid", "sleep", "usleep",
    "umask", "mkdir", "rmdir", "readlink", "symlink",
    # time.h / sys/time.h
    "time", "gmtime", "localtime", "mktime", "ctime", "asctime",
    "strftime", "utime", "clock", "difftime", "gettimeofday", "nanosleep",
    # signal.h
    "signal", "raise", "kill", "sigaction", "sigemptyset", "sigfillset",
    "sigaddset", "sigdelset", "sigprocmask",
    # ctype.h glibc internals
    "__ctype_b_loc", "__ctype_tolower_loc", "__ctype_toupper_loc",
    # errno.h glibc internal
    "__errno_location",
    # glibc stat name-mangled
    "__xstat", "__fxstat", "__lxstat", "__xstat64", "__fxstat64", "__lxstat64",
    "__assert_fail",
    # math.h common
    "sqrt", "sqrtf", "pow", "powf", "log", "logf", "log2", "log10",
    "exp", "expf", "sin", "cos", "tan", "atan", "atan2", "ceil", "floor",
    "fabs", "round", "trunc", "fmod",
    # sys/socket.h
    "socket", "bind", "listen", "accept", "connect", "send", "recv",
    "sendto", "recvfrom", "shutdown", "setsockopt", "getsockopt",
    # locale.h / setjmp.h
    "setlocale", "setjmp", "longjmp",
    # ctype is_* (glibc inlines them via __ctype_b_loc usually, but extern
    # decls also appear in c2rust output)
    "isalpha", "isdigit", "isalnum", "isspace", "isupper", "islower",
    "isxdigit", "iscntrl", "isprint", "ispunct", "isgraph", "tolower",
    "toupper",
})


_LIBC_STATICS = frozenset({
    "stdin", "stdout", "stderr", "errno", "optarg", "optind", "opterr",
    "optopt", "environ", "sys_errlist", "sys_nerr", "tzname", "timezone",
    "daylight",
})


def _is_libc(name: str, kind: str) -> bool:
    """True if `name` is a known libc/POSIX symbol of the given `kind`."""
    if kind == "fn":
        return name in _LIBC_FNS
    if kind == "static":
        return name in _LIBC_STATICS
    return False


# Identifiers that are always resolvable inside ffi.rs (Rust primitives,
# `core::ffi::c_*` re-exports, common std/core syntax). Anything else
# referenced by an extern decl signature must be in the shared `scope`
# set (= migrated to c_types.rs / c_structs.rs / c_extern_types.rs);
# otherwise migrating the decl would break ffi.rs's build.
_RUST_AMBIENT = frozenset({
    # Rust primitives
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64", "bool", "char",
    # core::ffi (imported via `use core::ffi::*;` at top of ffi.rs)
    "c_void", "c_char", "c_uchar", "c_schar", "c_short", "c_ushort",
    "c_int", "c_uint", "c_long", "c_ulong", "c_longlong", "c_ulonglong",
    "c_float", "c_double",
    # Std/core wrappers commonly seen in extern signatures
    "Option", "Self", "fn",
    # Path components in `::core::ffi::X` style refs
    "core", "ffi", "std",
    # Keywords that appear inside type expressions (`*mut T`, `*const T`,
    # `dyn Trait`, `impl Trait`, etc.). These show up as bare identifiers
    # to a regex tokenizer but are syntactic, not types.
    "mut", "const", "dyn", "impl", "where", "extern", "unsafe",
})


def _signature_idents(body: str) -> set[str]:
    """Extract type identifiers referenced in an extern fn/static decl body.

    Strips param names (`__filename:`, `__sig:` etc.) so they don't pollute
    the type ident set. The fn/static name itself (after `fn`/`static`)
    is also stripped. What remains is type-expression identifiers.
    """
    text = body
    # Strip the leading `[pub] fn NAME` or `[pub] static [mut] NAME:` part
    text = re.sub(r"^\s*(?:pub\s+)?(?:unsafe\s+)?fn\s+\w+", "", text, count=1)
    text = re.sub(r"^\s*(?:pub\s+)?static\s+(?:mut\s+)?\w+\s*:", "", text, count=1)
    # Strip param names: ` IDENT:` (param name immediately before colon).
    # Param names are always lowercase or underscore-prefixed in c2rust output.
    text = re.sub(r"\b[a-z_][a-zA-Z0-9_]*\s*:", "", text)
    return set(re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\b", text))


def _signature_resolvable(body: str, scope: set[str]) -> bool:
    """True iff every type identifier in `body` is either a Rust ambient
    name or already migrated to a shared module (in `scope`)."""
    for ident in _signature_idents(body):
        if ident in _RUST_AMBIENT or ident in scope:
            continue
        return False
    return True


def _candidate_files(project: Path, include_bins: bool = False) -> list[Path]:
    """Return all lib .rs files under src/ (recursive), excluding generated
    shared modules. Bin files (registered in Cargo.toml) are excluded by
    default — Stage 0 may have placed nested-main wrappers under src/bin/
    which are bins, not lib modules. Pass `include_bins=True` to include
    them (e.g. for FFI dedup, where bin TUs may also redeclare libc fns).
    """
    src = project / "src"
    cargo = project / "Cargo.toml"
    bins = _bin_paths(cargo)
    return [rs for rs in sorted(src.rglob("*.rs"))
            if rs.name not in _GENERATED
            and (include_bins or rs.resolve() not in bins)]


def _scan(files: list[Path]) -> list[ExternItem]:
    items = []
    for f in files:
        text = f.read_text()
        for m in _EXTERN_BLOCK_RE.finditer(text):
            for decl in _split_extern_body(m.group(2)):
                if ci := _classify(decl, f):
                    items.append(ci)
    return items


def _select(items: list[ExternItem],
            include_single_libc: bool = False,
            scope: set[str] | None = None,
            conflicts: set[str] | None = None) -> dict[str, ExternItem]:
    """Return name → canonical Item for migration.

    Default rule: items appearing in ≥2 files with identical body.
    With `include_single_libc=True`: also migrate single-file items whose
    name is in the curated `_LIBC_FNS` / `_LIBC_STATICS` sets.

    All migration paths additionally require that the decl's signature
    references only types resolvable in ffi.rs (i.e. in `scope` or Rust
    ambient). A fn like `deflate(strm: z_streamp, ...)` appearing in
    22 files would otherwise migrate by the ≥2-file rule, but ffi.rs
    has no `z_streamp` definition (`z_streamp` is a project type alias
    that itself didn't migrate due to a divergent-body chain through
    `z_stream_s`). Skipping such fns leaves them in their origin files,
    which still have the local type alias in scope.

    `conflicts`: names with cross-kind / cross-body collisions anywhere in
    the project (precomputed). Items in this set are NEVER migrated.
    """
    conflicts = conflicts or set()
    by_key: dict[tuple[str, str], list[ExternItem]] = {}
    for it in items:
        by_key.setdefault((it.name, _norm(it.body)), []).append(it)
    out: dict[str, ExternItem] = {}
    for grp in by_key.values():
        first = grp[0]
        if first.name in conflicts:
            logger.debug(f"[extern skip] {first.name!r}: in conflict set "
                         f"(cross-kind or divergent collision elsewhere)")
            continue
        is_multi = len({i.file for i in grp}) >= 2
        is_single_libc = include_single_libc and _is_libc(first.name, first.kind)
        if not (is_multi or is_single_libc):
            continue
        # Resolvability gate (applies to BOTH ≥2-file and single-libc paths).
        if scope is not None and not _signature_resolvable(first.body, scope):
            logger.debug(f"[extern skip] {first.name!r}: signature references "
                         f"unresolved type(s) — leaving in origin file(s)")
            continue
        out[first.name] = first
    return out


def _strip(file: Path, drop_keys: set[tuple[str, str]]) -> bool:
    """Remove items matching `drop_keys` from `file`'s extern blocks.
    Drops blocks that become empty. Returns True iff `file` was modified."""
    text = file.read_text()
    matches = list(_EXTERN_BLOCK_RE.finditer(text))
    if not matches:
        return False
    new_text = text
    for m in reversed(matches):
        body = m.group(2)
        kept = []
        for decl in _split_extern_body(body):
            ci = _classify(decl, file)
            if ci is None or (ci.name, _norm(ci.body)) not in drop_keys:
                kept.append(decl)
        new_body = "".join(kept)
        if not new_body.strip():
            end = m.end() + (1 if m.end() < len(new_text) and new_text[m.end()] == "\n" else 0)
            new_text = new_text[:m.start()] + new_text[end:]
        else:
            indent = m.group(1)
            new_text = new_text[:m.start()] + f'{indent}extern "C" {{{new_body}\n{indent}}}' + new_text[m.end():]
    new_text = re.sub(r"\n{3,}", "\n\n", new_text)
    if new_text != text:
        file.write_text(new_text)
        return True
    return False


def _ensure_use(file: Path, use_line: str) -> None:
    """Insert `use_line` after `#![...]` inner attrs; no-op if already present."""
    text = file.read_text()
    new_text = insert_use_after_inner_attrs(text, use_line)
    if new_text != text:
        file.write_text(new_text)


def _make_pub(decl: str) -> str:
    """Prepend `pub` to fn/static declarations so cross-module imports work."""
    stripped = decl.lstrip()
    indent = decl[:len(decl) - len(stripped)]
    if stripped.startswith("pub "):
        return decl
    return indent + "pub " + stripped


def _register_module(lib_rs: Path, module_name: str) -> None:
    text = lib_rs.read_text()
    if re.search(rf"\bpub\s+mod\s+{re.escape(module_name)}\s*;", text):
        return
    m = re.search(r"pub\s+mod\s+src\s*\{", text)
    if m:
        nl = text.find("\n", m.end())
        pos = nl + 1 if nl != -1 else m.end()
        text = text[:pos] + f"    pub mod {module_name};\n" + text[pos:]
    else:
        text = f"pub mod {module_name};\n" + text
    lib_rs.write_text(text)


# ---------------------------------------------------------------------------
# Public passes
# ---------------------------------------------------------------------------

def dedup_extern_types(project: Path, scope: set[str],
                       include_bins: bool = False,
                       crate_name: str | None = None,
                       conflicts: set[str] | None = None) -> int:
    """Move duplicated `pub type X;` opaque foreign types to c_extern_types.rs.
    With `include_bins=True`, binaries also participate (using cross-crate
    imports). Mutates `scope` so later passes recognize these names.

    `conflicts`: precomputed set of names with cross-kind / cross-body
    collisions anywhere in the project. Items in this set are NEVER
    migrated — covers the case where `internal_state` is declared as
    `extern "C" { pub type internal_state; }` in some files but as
    `pub struct internal_state { ... }` in others (zlib's forward-
    declare-then-define pattern under multi-component projects like
    optipng). See cleanup/conflict_set.py.
    """
    files = _candidate_files(project, include_bins=include_bins)
    items = [it for it in _scan(files) if it.kind == "type"]
    migrate = _select(items, conflicts=conflicts)
    if not migrate:
        logger.info("[extern types] nothing to dedup")
        return 0

    src_dir = project / "src"
    dest = src_dir / "c_extern_types.rs"
    decls = sorted(migrate.values(), key=lambda x: x.name)
    body = "\n".join("    " + d.body.strip() for d in decls)
    dest.write_text(
        "// Auto-generated by script_c2rust Stage 1: shared opaque extern types.\n"
        "// Do not edit by hand — regenerate via the cleanup pass.\n\n"
        'extern "C" {\n'
        f"{body}\n"
        "}\n"
    )

    drop_keys = {(it.name, _norm(it.body)) for it in migrate.values()}
    bins = _bin_paths(project / "Cargo.toml")
    affected = sum(_strip(f, drop_keys) for f in files)
    for f in files:
        if any(it.file == f and (it.name, _norm(it.body)) in drop_keys for it in items):
            _ensure_use(f, use_line_for(f, "c_extern_types", bins, crate_name))
    _register_module(project / "lib.rs", "c_extern_types")
    scope.update(migrate.keys())
    logger.info(f"[extern types] migrated {len(migrate)} symbol(s) across {affected} file(s)")
    return len(migrate)


def dedup_extern_fns(project: Path, scope: set[str],
                     include_bins: bool = False,
                     crate_name: str | None = None,
                     centralize_single_libc: bool = True,
                     conflicts: set[str] | None = None) -> int:
    """Move duplicated extern fn declarations and statics to ffi.rs.

    By default also migrates single-file decls whose name is in the
    curated libc set (e.g. a binary's local `extern "C" { fn open(...); }`
    block) — this gives the FFI boundary a single home. Pass
    `centralize_single_libc=False` to restore the strict ≥2-file rule.

    `conflicts`: precomputed cross-kind / cross-body collision set;
    items in this set are NEVER migrated.

    Items are made `pub` to enable cross-module wildcard imports.
    """
    files = _candidate_files(project, include_bins=include_bins)
    items = [it for it in _scan(files) if it.kind in {"fn", "static"}]
    migrate = _select(items,
                      include_single_libc=centralize_single_libc,
                      scope=scope,
                      conflicts=conflicts)
    if not migrate:
        logger.info("[extern fns] nothing to dedup")
        return 0

    src_dir = project / "src"
    dest = src_dir / "ffi.rs"
    fns     = sorted([i for i in migrate.values() if i.kind == "fn"],     key=lambda x: x.name)
    statics = sorted([i for i in migrate.values() if i.kind == "static"], key=lambda x: x.name)

    parts = [
        "// Auto-generated by script_c2rust Stage 1: shared FFI declarations.",
        "// Do not edit by hand — regenerate via the cleanup pass.",
        "",
        "use core::ffi::*;",
    ]
    for mod in ("c_types", "c_structs", "c_extern_types"):
        if (src_dir / f"{mod}.rs").exists():
            parts.append(f"use super::{mod}::*;")
    parts += ["", 'extern "C" {']
    for s in statics:
        parts.append(_make_pub(s.body.rstrip()))
    for f in fns:
        parts.append(_make_pub(f.body.rstrip()))
    parts.append("}")
    dest.write_text("\n".join(parts) + "\n")

    drop_keys = {(it.name, _norm(it.body)) for it in migrate.values()}
    bins = _bin_paths(project / "Cargo.toml")
    affected = sum(_strip(f, drop_keys) for f in files)
    for f in files:
        if any(it.file == f and (it.name, _norm(it.body)) in drop_keys for it in items):
            _ensure_use(f, use_line_for(f, "ffi", bins, crate_name))
    _register_module(project / "lib.rs", "ffi")
    scope.update(migrate.keys())
    logger.info(f"[extern fns] migrated {len(fns)} fn(s) + {len(statics)} static(s) "
                f"across {affected} file(s)")
    return len(migrate)
