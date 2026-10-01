"""Top-level function dedup — Milestone D.

c2rust translates `static inline` C helpers per TU, emitting verbatim
copies of the same function in every `.rs` file that uses them. After
Stage 1's type / struct / const dedup migrates the supporting machinery,
those helper functions stay duplicated:

  · brotli: 7 copies of `BrotliUnalignedRead32`, plus `Read64` / `Write64`
  · bzip2:  2 copies of `myfeof`
  · libzahl: 12 copies of `libzahl_failure` etc.
  · tmux:   5 copies of `bsearch`

This pass picks the cross-TU clones that are safe to migrate, drops one
canonical body into `c_inlined_fns.rs`, and replaces every other copy with
`use crate::src::c_inlined_fns::*;`.

Why we can't extend `dedup_pass` (the type/struct/const generic):

  · `dedup._self_contained` walks the body with a regex that treats every
    identifier as a "free reference needing scope". For function bodies
    that's flagrantly wrong — local variables, loop counters, inner
    bindings get flagged as unresolved and the candidate is dropped.

  · Functions may call other project-internal functions whose path
    changes after migration (e.g. `BZ2_bzReadOpen` lives at
    `crate::src::bzlib::BZ2_bzReadOpen` after stage 1 rewires it). The
    type-aimed `_self_contained` can't tell which calls are safe.

Selection strategy — "leaf-only":

  · A function is migratable iff every `call_expression` inside its body
    targets either a recognised builtin (libc / core / std), a method
    call (resolved via receiver type), OR a name in `scope` (already
    migrated by a prior pass).

  · This conservatively rejects helpers that depend on cross-module
    project state, keeping risk low while still catching the entire
    `BrotliUnaligned*` / `myfeof` / `FastLog2` / `Log2FloorNonZero` /
    `CommandCopyLen` / `bsearch` family — which is where the wins are.

Rejection summary (matches the gating shape of `dedup._select_migrations`):

  · less than 2 files            → not a cross-TU dup
  · divergent body hash          → safe-to-fold normalization disagrees
  · name in conflict_set         → kind-collision elsewhere
  · `#[cfg(...)]` / `#[no_mangle]` / `#[linkage]` attribute → unsafe
  · body uses `self::` / `super::` → relative path breaks on move
  · not a leaf fn (calls non-builtin, non-scope symbol)
"""

from __future__ import annotations

import logging
import re
import subprocess
from pathlib import Path
from typing import Optional

from tree_sitter import Node, Tree

from .ast_helpers import Item, _get_parser, find_top_level_items
from .cargo_utils import bin_paths as _bin_paths, use_line_for
from .dedup import (
    DedupConfig,
    _register_module,
    _splice_out_and_import,
    _write_dest_module,
)

logger = logging.getLogger(__name__)


_DEST_MODULE = "c_inlined_fns"
# Modules we want to import IF they exist. `ffi` carries libc fn decls
# (fgetc / ungetc / memcpy / fprintf / …) — without it, leaf bodies that
# call libc fail to link (e.g. bzip2 myfeof calls fgetc & ungetc).
# We import only modules that actually exist as `src/<m>.rs` to avoid
# unresolved-import errors on projects where a pass didn't create one.
_CANDIDATE_IMPORTS = (
    "c_types", "c_structs", "c_extern_types", "c_consts", "ffi",
)


# ---------------------------------------------------------------------------
# Built-in call recognition
# ---------------------------------------------------------------------------

_BUILTIN_PATH_PREFIXES = (
    "libc::", "core::", "std::",
    "::core::", "::std::", "::libc::",
)

# Path heads that always resolve outside the project (any `<head>::...`
# reference starting with one of these is trusted without further scope
# lookup). `crate` is the canonical project path which the canonical
# fn body's `use crate::src::ffi::*` etc. continues to make available.
_BUILTIN_PATH_HEADS = frozenset({
    "libc", "core", "std", "crate",
})


# Rust keywords / well-known items that appear at reference sites in fn
# bodies but aren't free identifiers to lookup. `Some/None/Ok/Err` are
# constructors of std types — always available. `self/Self` are already
# rejected by `_unsafe_to_unify_reason`, but we list them here so a
# stray internal reference doesn't trip the gate.
_RUST_KEYWORDS_AND_INTRINSICS = frozenset({
    "self", "Self", "super", "crate",
    "true", "false",
    "Some", "None", "Ok", "Err",
    "as", "if", "else", "match", "let", "mut", "in", "loop", "while",
    "for", "return", "break", "continue", "fn", "ref", "where",
    "move", "async", "await", "yield",
    "_",
})


# Primitive types — Rust intrinsics plus the c2rust `core::ffi::c_*`
# family which is in scope via `use core::ffi::*;` everywhere c2rust
# emits.
_BUILTIN_TYPE_NAMES = frozenset({
    # Rust primitives
    "u8", "u16", "u32", "u64", "u128", "usize",
    "i8", "i16", "i32", "i64", "i128", "isize",
    "f32", "f64",
    "bool", "char", "str", "String",
    # core::ffi
    "c_int", "c_uint", "c_long", "c_ulong", "c_longlong", "c_ulonglong",
    "c_short", "c_ushort", "c_char", "c_uchar", "c_schar",
    "c_void", "c_float", "c_double",
    # Common std collections (rare in c2rust output but cheap to whitelist)
    "Vec", "Box", "Option", "Result",
})


# Library names whose calls don't bind to project-internal symbols. These
# are conservatively bare libc / POSIX helpers that c2rust commonly emits
# at the call sites of inline helpers (memcpy in BrotliUnaligned*, fgetc /
# ungetc in myfeof, fprintf in libzahl_failure, …).
_BUILTIN_FN_NAMES = frozenset({
    # libc memory
    "memcpy", "memset", "memcmp", "memmove", "memchr",
    "malloc", "calloc", "free", "realloc",
    # libc strings
    "strlen", "strcpy", "strncpy", "strcmp", "strncmp",
    "strcat", "strncat", "strdup", "strchr", "strstr",
    # libc stdio
    "fopen", "fclose", "fread", "fwrite", "fflush",
    "fgetc", "ungetc", "fputc", "getchar", "putchar",
    "fprintf", "snprintf", "sprintf", "vfprintf", "vsnprintf",
    "printf", "fputs", "puts", "perror",
    # libc unix
    "open", "close", "read", "write", "lseek",
    "stat", "fstat", "unlink", "utime", "access",
    "exit", "abort", "_exit",
    "getenv", "setenv", "unsetenv",
    "signal", "kill", "alarm",
    "isatty", "fileno",
    # core / std utility
    "transmute", "size_of", "size_of_val", "align_of",
    "drop", "swap", "replace", "take",
    # Rust constructor-like idioms emitted at call position
    "Some", "None", "Ok", "Err",
    # c2rust's `unreachable!` etc. (resolves via std prelude)
    "unreachable", "panic", "assert", "debug_assert",
})


def _is_builtin_call(callee_text: str) -> bool:
    """True iff this call resolves outside the project (libc / core / std
    or a recognised method idiom)."""
    text = callee_text.strip()
    if any(text.startswith(p) for p in _BUILTIN_PATH_PREFIXES):
        return True
    # Method call `<expr>.foo(...)`: callee is a field_expression. We
    # parse the source text crudely — a `.` before the receiver-less
    # name segment, with no `::` in the head, marks a method call. The
    # method's resolution depends on the receiver type, not the module
    # path, so it survives the move.
    if "." in text:
        head = text.split(".", 1)[0]
        if "::" not in head:
            return True
    return False


# ---------------------------------------------------------------------------
# Body normalization
# ---------------------------------------------------------------------------

_LINE_COMMENT_RE  = re.compile(r"//[^\n]*")
_BLOCK_COMMENT_RE = re.compile(r"/\*.*?\*/", re.DOTALL)
_WS_RE            = re.compile(r"\s+")


def _normalize_fn_body(body: str) -> str:
    """Whitespace + comment normalize, plus c2rust-specific folds that
    span how the same fn can be emitted slightly differently per TU.

    These folds are conservative — they collapse pairs that are clearly
    syntactic equivalents at the c2rust output layer, not semantic
    rewrites:

      `(-9 as c_int)`    ≡ `-9`
      `0 as c_int`       ≡ `0`     (at end of `as` chain)
      `-(9 as c_int)`    ≡ `-9`
    """
    text = _BLOCK_COMMENT_RE.sub(" ", body)
    text = _LINE_COMMENT_RE.sub("", text)
    text = _WS_RE.sub(" ", text).strip()
    # `(-N as c_int)` → `-N`
    text = re.sub(r"\(\s*(-?\d+)\s+as\s+c_int\s*\)", r"\1", text)
    # `N as c_int` not followed by another `as` → `N`
    text = re.sub(r"(?<![\w])(-?\d+)\s+as\s+c_int(?!\s*as)", r"\1", text)
    # `- ( N )` → `-N`
    text = re.sub(r"-\s*\(\s*(\d+)\s*\)", r"-\1", text)
    return text


# ---------------------------------------------------------------------------
# Safety filter — what NOT to migrate even when body-equal
# ---------------------------------------------------------------------------

_CFG_RE       = re.compile(r"#\s*\[\s*cfg\s*\(")
_NO_MANGLE_RE = re.compile(r"#\s*\[\s*no_mangle")
_LINKAGE_RE   = re.compile(r"#\s*\[\s*linkage\s*=")
_REL_PATH_RE  = re.compile(r"\b(self|super)::")


def _unsafe_to_unify_reason(body: str) -> Optional[str]:
    """Return a string reason iff this fn must NOT be unified. None = OK."""
    if _CFG_RE.search(body):
        return "#[cfg(...)] attribute"
    if _NO_MANGLE_RE.search(body):
        return "#[no_mangle] attribute"
    if _LINKAGE_RE.search(body):
        return "#[linkage = ...] attribute"
    if _REL_PATH_RE.search(body):
        return "body uses self:: or super:: path"
    return None


# ---------------------------------------------------------------------------
# Leaf-fn check
# ---------------------------------------------------------------------------

def _walk(node: Node):
    yield node
    for c in node.children:
        yield from _walk(c)


def _idents_in_pattern(pat_node: Node, src: bytes) -> set[str]:
    """Recursively collect every `identifier` text under a pattern node.
    Used to harvest names introduced by a fn parameter, let binding,
    for-loop pattern, match-arm pattern, closure parameter, etc."""
    out: set[str] = set()
    for n in _walk(pat_node):
        if n.type == "identifier":
            out.add(src[n.start_byte:n.end_byte].decode("utf-8", errors="replace"))
    return out


def _collect_fn_locals(fn_node: Node, src: bytes) -> set[str]:
    """Names introduced inside the fn — parameters, let bindings,
    for / while-let / if-let / match patterns, closure parameters.

    These never need to resolve against `scope` because they're bound by
    the fn body itself.
    """
    locals_: set[str] = set()
    # Function parameters.
    params = fn_node.child_by_field_name("parameters")
    if params is not None:
        for p in params.children:
            if p.type == "parameter":
                pat = p.child_by_field_name("pattern")
                if pat is not None:
                    locals_.update(_idents_in_pattern(pat, src))
    # Walk body for inner binding sites.
    body = fn_node.child_by_field_name("body")
    if body is None:
        return locals_
    for n in _walk(body):
        if n.type == "let_declaration":
            pat = n.child_by_field_name("pattern")
            if pat is not None:
                locals_.update(_idents_in_pattern(pat, src))
        elif n.type == "for_expression":
            pat = n.child_by_field_name("pattern")
            if pat is not None:
                locals_.update(_idents_in_pattern(pat, src))
        elif n.type == "closure_expression":
            cparams = n.child_by_field_name("parameters")
            if cparams is not None:
                for p in cparams.children:
                    if p.type == "parameter":
                        pat = p.child_by_field_name("pattern")
                        if pat is not None:
                            locals_.update(_idents_in_pattern(pat, src))
                    elif p.type == "identifier":
                        locals_.add(src[p.start_byte:p.end_byte].decode(
                            "utf-8", errors="replace",
                        ))
        elif n.type == "match_arm":
            pat = n.child_by_field_name("pattern")
            if pat is not None:
                locals_.update(_idents_in_pattern(pat, src))
        # Nested item decls inside fn body — rare but real (c2rust
        # occasionally emits local `pub const FOO: T = ...;`). Treat
        # them as locals so the body's own use of FOO doesn't fail
        # the scope gate.
        elif n.type in ("const_item", "static_item", "type_item",
                         "struct_item", "enum_item", "union_item",
                         "function_item"):
            name_node = n.child_by_field_name("name")
            if name_node is not None:
                locals_.add(src[name_node.start_byte:name_node.end_byte].decode(
                    "utf-8", errors="replace",
                ))
    return locals_


def _ident_is_definition_site(n: Node) -> bool:
    """True iff `n` is at a binding / declaration site (so its text is
    NOT a free reference to look up). Examples:
      · inside a `parameter`'s pattern         (binding)
      · inside a `let_declaration`'s pattern   (binding)
      · field name of `field_expression`        (resolves via receiver type)
      · field name of `field_initializer`       (struct literal lhs)
      · the name of a nested item               (decl, not ref)
    """
    parent = n.parent
    if parent is None:
        return False
    # Field access: `x.foo` — foo is a method/field name, not a free name.
    if parent.type == "field_expression":
        field = parent.child_by_field_name("field")
        if field is not None and field.start_byte == n.start_byte:
            return True
    # Struct literal lhs: `S { foo: 1 }` — `foo` is the field name.
    if parent.type in ("field_initializer", "shorthand_field_initializer",
                        "field_declaration"):
        return True
    # The name slot of a declarative item.
    if parent.type in ("function_item", "struct_item", "enum_item",
                        "union_item", "type_item", "const_item",
                        "static_item", "mod_item"):
        name = parent.child_by_field_name("name")
        if name is not None and name.start_byte == n.start_byte:
            return True
    # The name slot of an enum variant.
    if parent.type == "enum_variant":
        name = parent.child_by_field_name("name")
        if name is not None and name.start_byte == n.start_byte:
            return True
    return False


def _scoped_path_head(n: Node, src: bytes) -> Optional[str]:
    """If `n` is a `scoped_identifier` / `scoped_type_identifier`, return
    the head segment (e.g. `core::mem::size_of` → `core`). Otherwise None."""
    if n.type not in ("scoped_identifier", "scoped_type_identifier"):
        return None
    path = n.child_by_field_name("path")
    if path is None:
        # Bare `::foo` path — no head, treat as builtin (absolute).
        return ""
    if path.type in ("scoped_identifier", "scoped_type_identifier"):
        # Recurse into the head of the nested path.
        return _scoped_path_head(path, src)
    if path.type in ("identifier", "type_identifier"):
        return src[path.start_byte:path.end_byte].decode("utf-8", errors="replace")
    return None


def _is_self_contained_fn(
    fn_node: Node, src: bytes, scope: set[str],
) -> tuple[bool, str]:
    """Every identifier referenced in the fn body must resolve via one of:
      · `_BUILTIN_PATH_HEADS`   (libc / core / std / crate)
      · `_BUILTIN_TYPE_NAMES`   (u32 / c_int / Vec / ...)
      · `_BUILTIN_FN_NAMES`     (memcpy / fprintf / ...)
      · `_RUST_KEYWORDS_AND_INTRINSICS`
      · `scope`                (already migrated by prior pass)
      · fn-local binding (parameter / let / pattern / nested decl)

    Any free identifier outside these is a project-internal reference
    that can't be reached from `c_inlined_fns.rs` — reject.

    Replaces the old call-only `_is_leaf_fn`. Catches the type / const
    refs that previously slipped through (Command, kBrotliBitMask,
    __m128i_u, ...).
    """
    body = fn_node.child_by_field_name("body")
    if body is None:
        return True, ""
    locals_ = _collect_fn_locals(fn_node, src)

    def _ok(head: str) -> bool:
        return (
            head in _BUILTIN_PATH_HEADS
            or head in _BUILTIN_TYPE_NAMES
            or head in _BUILTIN_FN_NAMES
            or head in _RUST_KEYWORDS_AND_INTRINSICS
            or head in scope
            or head in locals_
            or head == ""    # absolute `::path` form
        )

    # Subtrees to scan: signature parts (param types, return type) +
    # the body. Type references inside parameters (e.g. `*const Command`)
    # must resolve at the call site too, so we can't skip them.
    scan_roots: list[Node] = [body]
    params = fn_node.child_by_field_name("parameters")
    if params is not None:
        scan_roots.append(params)
    ret = fn_node.child_by_field_name("return_type")
    if ret is not None:
        scan_roots.append(ret)
    # type_parameters (generics on the fn header) — names declared here
    # ARE bindings (e.g. `fn foo<T>(x: T)` introduces T). Add to locals
    # rather than scanning for references.
    type_params = fn_node.child_by_field_name("type_parameters")
    if type_params is not None:
        for tp in _walk(type_params):
            if tp.type == "type_identifier":
                locals_.add(src[tp.start_byte:tp.end_byte].decode(
                    "utf-8", errors="replace",
                ))

    walk_iter = (n for root in scan_roots for n in _walk(root))
    for n in walk_iter:
        # ----- scoped path -----
        if n.type in ("scoped_identifier", "scoped_type_identifier"):
            # Skip if WE are the path child of an outer scoped path —
            # the outer's head is what counts.
            parent = n.parent
            if parent is not None and parent.type in (
                "scoped_identifier", "scoped_type_identifier"
            ):
                # We're nested; outer iteration handled it.
                continue
            head = _scoped_path_head(n, src)
            if head is None:
                continue
            if not _ok(head):
                return False, f"references path head '{head}'"
            continue
        # ----- bare identifier or type identifier -----
        if n.type not in ("identifier", "type_identifier"):
            continue
        if _ident_is_definition_site(n):
            continue
        # Skip ident that's INSIDE a scoped path (handled by outer iteration).
        parent = n.parent
        if parent is not None and parent.type in (
            "scoped_identifier", "scoped_type_identifier"
        ):
            continue
        # Skip ident in struct-literal field-initializer key, etc.
        if parent is not None and parent.type == "use_declaration":
            continue
        text = src[n.start_byte:n.end_byte].decode("utf-8", errors="replace")
        if not text or not re.match(r"^[A-Za-z_]\w*$", text):
            continue
        if not _ok(text):
            return False, f"references '{text}'"
    return True, ""


# ---------------------------------------------------------------------------
# pub-injection — make the canonical body callable from other modules
# ---------------------------------------------------------------------------

_LEADING_FN_HEADER_RE = re.compile(
    r"^(\s*)((?:unsafe\s+|extern\s+\"[^\"]+\"\s+)*fn\b)"
)


def _ensure_pub(body: str) -> str:
    """If the fn header isn't already public, splice `pub ` before the
    first declarative keyword. Preserves any leading attributes / `unsafe`
    / `extern "ABI"` modifiers.
    """
    lines = body.splitlines(keepends=True)
    for i, line in enumerate(lines):
        stripped = line.lstrip()
        if not stripped or stripped.startswith("//") or stripped.startswith("#["):
            continue
        if re.match(r"\s*pub\b", line):
            return body                          # already public
        m = _LEADING_FN_HEADER_RE.match(line)
        if m is None:
            return body                          # unrecognised — leave alone
        indent, kw = m.group(1), m.group(2)
        lines[i] = f"{indent}pub {kw}{line[m.end():]}"
        return "".join(lines)
    return body


# ---------------------------------------------------------------------------
# Tree cache — parse each .rs file at most once per pass
# ---------------------------------------------------------------------------

def _parse_file(path: Path) -> tuple[Tree, bytes]:
    parser = _get_parser()
    src = path.read_bytes()
    return parser.parse(src), src


# ---------------------------------------------------------------------------
# Entry point — same signature shape as `dedup_pass`
# ---------------------------------------------------------------------------

def _cargo_check_error_count(project_path: Path) -> int:
    """Run `cargo check --release` and count `error[...]` lines.

    Returns -1 on subprocess failure (treated as unknown — the caller
    should disable self-rollback when this happens, since we can't
    distinguish "broke something" from "infra not available").
    """
    try:
        proc = subprocess.run(
            ["cargo", "check", "--release"], cwd=str(project_path),
            capture_output=True, text=True, timeout=300,
        )
    except (subprocess.SubprocessError, FileNotFoundError):
        return -1
    return sum(1 for ln in (proc.stderr or "").splitlines()
               if re.match(r"error(?:\[E\d+\])?:", ln))


def dedup_top_level_fns(
    project_path: Path,
    scope: set[str],
    include_bins: bool = True,
    crate_name: Optional[str] = None,
    conflicts: Optional[set[str]] = None,
) -> int:
    """Migrate cross-TU fully-identical leaf fns into `c_inlined_fns.rs`.

    Self-rollback: after editing, the pass runs `cargo check --release`.
    If errors grew vs the pre-migration baseline, all edits are undone
    (file restorations + dest module deletion + lib.rs mod un-registration).
    Caller observes count=0 in that case, but the outer dedup_block step
    stays intact — leaf-check imprecision in one pass doesn't poison the
    type/struct/const work that already succeeded.

    Returns the count of migrated fn names. Mutates `scope` by adding
    them so later passes can recognise these names as resolved.
    """
    src_dir = project_path / "src"
    lib_rs  = project_path / "lib.rs"
    cargo   = project_path / "Cargo.toml"
    if not src_dir.is_dir() or not lib_rs.exists():
        logger.warning("[inline fns] no src/ or lib.rs — skip")
        return 0

    dest_file = src_dir / f"{_DEST_MODULE}.rs"
    bins      = _bin_paths(cargo)
    conflicts = conflicts or set()

    # 1. Collect every top-level function across the crate.
    items: list[Item] = []
    files_seen: list[Path] = []
    for rs in sorted(src_dir.rglob("*.rs")):
        if rs == dest_file:
            continue
        if not include_bins and rs.resolve() in bins:
            continue
        files_seen.append(rs)
        items.extend(find_top_level_items(rs, ("function_item",)))

    # 2. Group by name → pick body-equal cross-TU clones.
    by_name: dict[str, list[Item]] = {}
    for it in items:
        by_name.setdefault(it.name, []).append(it)

    # Tree cache so the leaf check only parses each file once.
    tree_cache: dict[Path, tuple[Tree, bytes]] = {}

    migrate: dict[str, Item] = {}
    rejected: dict[str, str] = {}

    for name, insts in by_name.items():
        if name in conflicts:
            rejected[name] = "in conflict set"
            continue
        if len({i.file for i in insts}) < 2:
            continue
        # Body-equal check (with c2rust folds).
        hashes = {_normalize_fn_body(i.body) for i in insts}
        if len(hashes) > 1:
            rejected[name] = f"divergent bodies ({len(hashes)} unique)"
            continue
        canonical = insts[0]

        # Attribute / relative-path safety.
        why = _unsafe_to_unify_reason(canonical.body)
        if why is not None:
            rejected[name] = why
            continue

        # Locate the function_item node in its file.
        if canonical.file not in tree_cache:
            tree_cache[canonical.file] = _parse_file(canonical.file)
        tree, src_bytes = tree_cache[canonical.file]
        fn_node: Optional[Node] = None
        for n in tree.root_node.named_children:
            if n.type == "function_item" and n.end_byte == canonical.end_byte:
                fn_node = n
                break
        if fn_node is None:
            rejected[name] = "could not relocate fn_node in source"
            continue

        # Self-contained check (Phase 2 — Milestone D upgrade): scan
        # every ident in the fn body (not just calls) to make sure
        # nothing references a project-internal name that won't be
        # reachable from c_inlined_fns.rs after the move.
        ok, leaf_reason = _is_self_contained_fn(fn_node, src_bytes, scope)
        if not ok:
            rejected[name] = leaf_reason
            continue

        # Inject `pub` if absent so c_inlined_fns.rs can re-export.
        body_pub = _ensure_pub(canonical.body)
        migrate[name] = Item(
            kind       = canonical.kind,
            name       = canonical.name,
            body       = body_pub,
            file       = canonical.file,
            start_byte = canonical.start_byte,
            end_byte   = canonical.end_byte,
        )

    if rejected:
        for name, reason in list(rejected.items())[:10]:
            logger.debug(f"  [skip] {name!r}: {reason}")
    if not migrate:
        logger.info(
            f"[inline fns] nothing to dedup ({len(rejected)} candidates rejected)"
        )
        return 0

    # 3. Map name → copies to remove from each source file.
    affected: dict[Path, list[Item]] = {}
    for it in items:
        if it.name in migrate:
            affected.setdefault(it.file, []).append(it)

    # 4. Capture baseline cargo-check error count so we can detect (and
    # self-revert) any imprecision in the leaf check.
    baseline_errors = _cargo_check_error_count(project_path)

    # 5. Snapshot every file we're about to touch so we can restore on
    # rollback. Order:
    #   · dest_file (created or appended)
    #   · lib_rs    (mod registration)
    #   · every affected source file (splice + use insert)
    snapshots: dict[Path, Optional[bytes]] = {}
    snapshots[dest_file] = dest_file.read_bytes() if dest_file.exists() else None
    snapshots[lib_rs]    = lib_rs.read_bytes()
    for f in affected:
        snapshots[f] = f.read_bytes()

    # 6. Reuse dedup.py's write / splice / register helpers — same shape
    # as type / struct / const passes, so a future audit/grep treats
    # this consistently.
    live_imports = tuple(
        m for m in _CANDIDATE_IMPORTS
        if (src_dir / f"{m}.rs").is_file()
    )
    cfg = DedupConfig(
        item_kinds  = ("function_item",),
        dest_module = _DEST_MODULE,
        label       = "inline helper fns",
        imports     = live_imports,
    )
    _write_dest_module(dest_file, migrate, cfg)
    for f, removed in affected.items():
        _splice_out_and_import(
            f, removed,
            use_line_for(f, _DEST_MODULE, bins, crate_name),
        )
    _register_module(lib_rs, _DEST_MODULE)

    # 7. Self-rollback gate. If baseline_errors == -1 (cargo not
    # reachable) we skip the gate — better to commit work than block
    # progress on an infra glitch; the outer step_rollback still catches
    # post-hoc gate violations.
    if baseline_errors >= 0:
        post_errors = _cargo_check_error_count(project_path)
        if post_errors > baseline_errors:
            for path, blob in snapshots.items():
                if blob is None:
                    if path.exists():
                        path.unlink()
                else:
                    path.write_bytes(blob)
            logger.warning(
                f"[inline fns] self-rollback: would have migrated "
                f"{len(migrate)} fn(s) but cargo check errors grew "
                f"{baseline_errors} → {post_errors}; reverting all changes "
                f"(leaf check accepted refs the fn body can't resolve "
                f"in c_inlined_fns.rs scope)"
            )
            return 0

    scope.update(migrate.keys())
    logger.info(
        f"[inline fns] migrated {len(migrate)} fn(s) across "
        f"{len(affected)} file(s); rejected {len(rejected)}"
    )
    return len(migrate)


__all__ = ["dedup_top_level_fns"]
