"""Detect thin wrappers around libc / system calls.

A "wrapper" is a crate function whose body does no real algorithmic work — the
sampled self-time is really spent inside a libc / syscall callee that no rewrite
card can touch (a 3-line `strpbrk` shim, a `vfprintf` wrapper, an
`open/fcntl/close` bundle, ...). Reporting these as hot functions gives the
agent candidates it cannot act on, and — worse — pushes their editable callers
off the top of the hot list.

Detection is source-based (from the evidence pack's rust_source) and
deterministic:

  A function is a wrapper iff its body
    * contains NO loop (while / for / loop) — a loop is real work, and
    * every function call in the body targets a libc / syscall symbol in the
      whitelist below (calls into other crate functions disqualify it).

The wrapper's `wrapper_target` is the whitelisted symbol accounting for the
most calls in the body (usually the sole one). If the body has zero calls at
all, it is NOT a wrapper (a pure arithmetic 4-liner is a valid D2 candidate,
not a shim).
"""

from __future__ import annotations

import re
from pathlib import Path

# Libc / syscall / c2rust-runtime symbols whose callers do no work of their
# own — any call to one of these is a delegation, not real work. Kept broad
# but not exhaustive; extend when a real-world hot wrapper leaks through.
_LIBC_WHITELIST: frozenset[str] = frozenset({
    # stdio / printf family
    "printf", "fprintf", "sprintf", "snprintf", "vprintf", "vfprintf",
    "vsprintf", "vsnprintf", "puts", "fputs", "putc", "fputc", "putchar",
    "getc", "fgetc", "getchar", "gets", "fgets",
    "fopen", "fclose", "fdopen", "freopen", "fread", "fwrite", "fflush",
    "fseek", "ftell", "rewind", "setvbuf", "setbuf", "clearerr", "feof",
    "ferror", "fileno",
    # unistd / io / fcntl
    "open", "close", "read", "write", "pread", "pwrite", "dup", "dup2",
    "pipe", "fcntl", "ioctl", "lseek", "unlink", "rename", "access",
    "stat", "fstat", "lstat", "isatty", "getcwd", "chdir",
    # mem/str
    "malloc", "calloc", "realloc", "free", "aligned_alloc", "posix_memalign",
    "memcpy", "memmove", "memset", "memcmp", "memchr", "memrchr",
    "strcpy", "strncpy", "strcat", "strncat", "strcmp", "strncmp",
    "strcasecmp", "strncasecmp", "strchr", "strrchr", "strstr", "strpbrk",
    "strspn", "strcspn", "strlen", "strnlen", "strdup", "strndup",
    "strtok", "strtok_r", "strerror",
    # ctype
    "toupper", "tolower", "isalpha", "isdigit", "isalnum", "isspace",
    "isprint", "isupper", "islower", "ispunct", "iscntrl", "isxdigit",
    "__ctype_toupper_loc", "__ctype_tolower_loc", "__ctype_b_loc",
    # numeric conv
    "atoi", "atol", "atoll", "atof", "strtol", "strtoll", "strtoul",
    "strtoull", "strtod", "strtof",
    # tty / termios
    "tcgetattr", "tcsetattr", "tcflush", "tcdrain", "tcflow",
    "cfmakeraw", "cfsetispeed", "cfsetospeed",
    # signal / process / time
    "signal", "sigaction", "kill",
    "getpid", "getppid", "sleep", "usleep", "nanosleep",
    "time", "clock", "gettimeofday", "clock_gettime",
    # search / sort
    "qsort", "bsearch",
    # errno
    "__errno_location",
})

# Error/termination noise: allowed anywhere in a wrapper body but NEVER chosen
# as the delegation target. A fn that `open()`s then `exit()`s on failure is an
# open-wrapper; picking `exit` as the target would misrepresent where the time
# actually went. Keep this separate from `_LIBC_WHITELIST` (targeting rules).
_ERROR_TERMINATORS: frozenset[str] = frozenset({
    "exit", "_exit", "abort", "raise",
    "perror", "assert_fail", "__assert_fail",
})

# Rust `while` / `for` / `loop` / `while let` / `for … in …` keywords.
# c2rust-generated code uses all four forms plus `unsafe { while … }`.
_LOOP_RE = re.compile(r"\b(while|for|loop)\b")


def _strip_body(rust_source: str) -> str:
    """Return the fn body text (between the outermost { and }). If parsing
    fails, return the whole source — the caller's checks still run over it."""
    open_pos = rust_source.find("{")
    if open_pos < 0:
        return rust_source
    depth = 0
    for i in range(open_pos, len(rust_source)):
        c = rust_source[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return rust_source[open_pos + 1:i]
    return rust_source[open_pos + 1:]


# A call in Rust source appears as `<name>(...)` or `path::to::<name>(...)`
# or `<recv>.<method>(...)`. We take the identifier immediately before the
# opening paren. Method calls (preceded by `.`) don't concern us for the
# whitelist check — receivers like `args.clone()` are arg prep, not work.
_CALL_RE = re.compile(r"(?<![.\w])([A-Za-z_]\w*)\s*\(")

# Rust keywords that look like calls (`if(...)`, `match(...)`) — never real calls.
_KW_NOT_CALL: frozenset[str] = frozenset({
    # control-flow keywords
    "if", "match", "while", "for", "loop", "return", "break", "continue",
    "unsafe", "fn", "let", "mut", "as", "in", "move", "async", "await",
    "yield", "impl", "dyn", "ref", "self", "Self", "true", "false",
    # rustc macros (with or without `!` — `_CALL_RE` sees name+`(`, misses `!`)
    "assert", "assert_eq", "assert_ne", "debug_assert", "panic", "eprintln",
    "println", "print", "eprint", "format", "write", "writeln", "vec",
    # stdlib enum/option constructors — `Some(x)` etc. are NOT function calls
    # into any code we can inspect; treating them as non-libc would wrongly
    # disqualify wrappers that just pack an optional value.
    "Some", "None", "Ok", "Err",
})


def is_extern_wrapper(rust_source: str,
                      known_wrappers: set[str] | None = None
                      ) -> tuple[bool, str | None]:
    """Return (is_wrapper, wrapper_target). See module docstring for the rule.

    * Any loop keyword in the body ⇒ NOT a wrapper (real algorithmic work).
    * Zero calls in the body ⇒ NOT a wrapper (pure arithmetic — a D2 candidate,
      not a shim).
    * All non-method calls target the libc whitelist (or a previously-flagged
      wrapper — `known_wrappers`) ⇒ wrapper. `known_wrappers` lets the caller
      catch two-level shims (`tty_init` → `tty_getwinsz` → `ioctl`) via a
      fixed-point iteration on the hot-fn list.
    * wrapper_target = the LAST libc call in the body (real delegated work
      comes after arg-prep helpers); if there is no libc call — the body only
      delegates to other wrappers — the target is the last such wrapper.
    """
    body = _strip_body(rust_source)
    if _LOOP_RE.search(body):
        return False, None

    call_names = [m.group(1) for m in _CALL_RE.finditer(body)
                  if m.group(1) not in _KW_NOT_CALL]
    if not call_names:
        return False, None

    kw = known_wrappers or set()
    allowed = _LIBC_WHITELIST | _ERROR_TERMINATORS | kw
    non_delegated = [n for n in call_names if n not in allowed]
    if non_delegated:
        return False, None

    # A `realloc` in the body means this function manages a *growable* heap
    # buffer: its hot cost is the reallocation schedule, not opaque libc work.
    # C7 (amortized growth) and III② (→ Vec) can rewrite it, so it is NOT an
    # un-actionable libc shim — keep it as an editable hot fn. Without this,
    # a per-append `realloc(p, len+k)` helper (e.g. an append-char primitive)
    # is dropped as a wrapper and never reaches the growth-amortization rule.
    if "realloc" in call_names:
        return False, None

    # Target ranking (never an error terminator — those don't do work):
    #   1st choice: last real libc call (dominant work in the source order)
    #   2nd choice: last known-wrapper call (delegation through a shim)
    #   fallback:   None (body is only error terminators — theoretical only)
    libc_calls = [n for n in call_names if n in _LIBC_WHITELIST]
    if libc_calls:
        return True, libc_calls[-1]
    wrapper_calls = [n for n in call_names if n in kw]
    if wrapper_calls:
        return True, wrapper_calls[-1]
    return True, None


def mark_wrappers(hot_fns, index) -> int:
    """Set `extern_wrapper` / `wrapper_target` on each HotFunction whose source
    body is a (possibly transitive) libc/syscall wrapper.

    Two-pass fixed-point over ALL crate fns (not just the hot list) so that a
    hot fn delegating to non-hot crate shims is caught too: e.g. tty_init calls
    tty_getwinsz + tty_setnormal (not on the hot list) which each just call
    ioctl / tcsetattr — tty_init IS a wrapper, but only visible once its
    callees are flagged. Iterate until the wrapper set stops growing.

    Returns the count of HOT functions flagged. Fns without a source location
    (unresolved / macro-generated) → unchanged."""
    all_src = index.all_sources()
    wrappers: dict[str, str] = {}       # fn_name → wrapper_target
    while True:
        added = False
        for name, src in all_src.items():
            if name in wrappers:
                continue
            is_w, target = is_extern_wrapper(src, known_wrappers=set(wrappers))
            if is_w:
                wrappers[name] = target
                added = True
        if not added:
            break

    flagged = 0
    for hf in hot_fns:
        if hf.file is None:
            continue
        if hf.name in wrappers:
            hf.extern_wrapper = True
            hf.wrapper_target = wrappers[hf.name]
            flagged += 1
    return flagged
