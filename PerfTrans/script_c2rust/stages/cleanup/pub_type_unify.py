"""Cross-module type unification BEFORE Stage A.

Two passes, both operating on c2rust's per-TU translation output:

  · `unify_duplicate_pub_types`: same `pub struct/union/enum X { … }`
    declared in N modules with N distinct nominal types → one canonical
    decl + `pub use crate::<canon>::X;` re-exports in the others.

  · `unify_opaque_foreign_types`: same `extern "C" { pub type X; }`
    opaque foreign type declared per-module → same canonical re-export
    treatment; concrete `pub struct X { … }` (if any) wins over opaque
    forward decls.

Both run inside `prep_for_stage_a` because Stage A's E1 strip of
`extern "C"` exposes the nominal-distinctness — without unify, every
cross-module fn call across the duplicated type hits E0308.
"""

from __future__ import annotations

import logging
import tomllib
from pathlib import Path

import tree_sitter_rust
from tree_sitter import Language, Parser

from .decl_compare import normalize_decl_for_compare

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Shared Cargo.toml [[bin]] + [package].name discovery
# ---------------------------------------------------------------------------

def _collect_bin_paths(crate_dir: Path) -> set[Path]:
    """Absolute paths of every file Cargo treats as a binary:
    explicit `[[bin]] path = "…"` entries plus the conventional
    `src/main.rs`. Bin files participate as REWRITE TARGETS but are
    never CANONICAL SOURCES (a `pub use crate::<bin>::X` doesn't
    resolve from the lib root)."""
    bin_paths: set[Path] = set()
    cargo = crate_dir / "Cargo.toml"
    if cargo.is_file():
        try:
            data = tomllib.loads(cargo.read_text(encoding="utf-8"))
            bins = data.get("bin", [])
            if isinstance(bins, list):
                for entry in bins:
                    p = entry.get("path") if isinstance(entry, dict) else None
                    if p:
                        bin_paths.add((crate_dir / p).resolve())
        except (OSError, tomllib.TOMLDecodeError):
            pass
    bin_paths.add((crate_dir / "src" / "main.rs").resolve())
    return bin_paths


def _read_lib_pkg_name(crate_dir: Path) -> str:
    """Resolve the lib crate's package name for bin-side `pub use`
    prefixes (bin is a separate crate; `crate::` doesn't reach lib mods).
    Returns `"crate"` as a fallback if Cargo.toml is unreadable."""
    cargo = crate_dir / "Cargo.toml"
    try:
        if cargo.is_file():
            data = tomllib.loads(cargo.read_text(encoding="utf-8"))
            name_field = data.get("package", {}).get("name")
            if isinstance(name_field, str) and name_field:
                # Cargo crate names use `-` but Rust paths use `_`.
                return name_field.replace("-", "_")
    except (OSError, tomllib.TOMLDecodeError):
        pass
    return "crate"


def _enumerate_modules(
    crate_dir: Path,
    bin_paths: set[Path],
) -> list[tuple[str, Path, bool]]:
    """Walk `crate_dir/src/**/*.rs` and tag each file as (mod_path,
    file, is_bin). Bin sources are tagged so callers can exclude them
    from canonical selection while still letting them participate as
    rewrite targets."""
    out: list[tuple[str, Path, bool]] = []
    src_dir = crate_dir / "src"
    for f in src_dir.rglob("*.rs"):
        if "target" in f.parts:
            continue
        is_bin = (
            f.resolve() in bin_paths
            or (f.parent.name == "bin" and f.parent.parent.name == "src")
        )
        rel = f.relative_to(crate_dir).with_suffix("")
        mod_path = "::".join(rel.parts)
        out.append((mod_path, f, is_bin))
    return out


# ---------------------------------------------------------------------------
# Pass 1: unify duplicate `pub struct / union / enum`
# ---------------------------------------------------------------------------

def unify_duplicate_pub_types(crate_dir: Path) -> None:
    """For every type name declared as `pub struct X { … }` / `pub enum X
    { … }` / `pub union X { … }` in more than one lib module, pick ONE
    canonical definition (longest file = richest module) and replace
    every other definition with `pub use super::<canon_mod>::X;`.

    Uses tree-sitter to extract precise CST byte ranges — regex on
    pub-struct bodies breaks on c2rust output (`Option<unsafe extern
    "C" fn(...) -> *mut c_void>` parens and nested brackets confuse a
    naive `[^}]*`).

    Type aliases (`pub type X = ...;`) are intentionally left alone:
    they are usually identical across modules and don't trigger the
    nominal-type-distinct error E1 worries about.
    """
    lang = Language(tree_sitter_rust.language())
    parser = Parser(lang)

    src_dir = crate_dir / "src"
    if not src_dir.is_dir():
        return

    # 2026-05-25: rewrote module enumeration to walk the filesystem
    # instead of regex-parsing lib.rs's `pub mod foo;` file-decls. The
    # old regex missed c2rust outputs with nested `pub mod src { pub
    # mod src { ... } }` block-form (heman). Same fix as
    # `scan_lib_public_fns`. Each .rs file under src/ is a candidate
    # lib module; we skip bin sources by reading Cargo.toml [[bin]].
    bin_paths = _collect_bin_paths(crate_dir)

    # Bin files participate as REWRITE TARGETS but not as canonical
    # sources — they get aliased to lib's canonical via `<pkg_name>::`
    # path. Without this, bin source `pub struct _xmlDoc { … }` stays
    # nominally distinct from lib's same-name struct and every bin↔lib
    # call hits E0308 (observed on libxml2 xmllint.rs: 131 mismatch
    # errors for `_xmlDoc` / `_xmlParserInputBuffer` etc).
    mod_files = _enumerate_modules(crate_dir, bin_paths)
    lib_pkg_name = _read_lib_pkg_name(crate_dir)

    # (name, kind) → list of (mod_path, file, byte_range, body_bytes, is_bin)
    #
    # 2026-05-29: key by (name, kind) to keep `struct X` and `union X` /
    # `enum X` from being lumped together as unify candidates. Each kind
    # gets its own unify decision; cross-kind name clashes (e.g. json-c
    # has `pub struct C2RustUnnamed` in one file + `pub union C2RustUnnamed`
    # in another) won't trigger spurious unify.
    #
    # The fourth tuple field is the FULL declaration bytes (incl. preceding
    # attributes). It feeds the body-equality check below.
    type_decls: dict[tuple[str, str],
                     list[tuple[str, Path, tuple[int, int], bytes, bool]]] = {}
    for (mod, f, is_bin) in mod_files:
        if not f.is_file():
            continue
        src = f.read_bytes()
        tree = parser.parse(src)
        for node in tree.root_node.children:
            if node.type not in ("struct_item", "enum_item", "union_item"):
                continue
            # Check public visibility
            vis = None
            name_node = None
            for c in node.children:
                if c.type == "visibility_modifier":
                    vis = c
                elif c.type == "type_identifier":
                    name_node = c
            if vis is None or name_node is None:
                continue
            name = src[name_node.start_byte:name_node.end_byte].decode("utf-8")
            # Include preceding attribute_item nodes in the range so the
            # replacement nukes #[derive(...)] / #[repr(C)] too.
            start = node.start_byte
            parent = node.parent
            if parent is not None:
                idx = list(parent.children).index(node)
                j = idx - 1
                while j >= 0 and parent.children[j].type == "attribute_item":
                    start = parent.children[j].start_byte
                    j -= 1
            decl_bytes = src[start:node.end_byte]
            type_decls.setdefault((name, node.type), []).append(
                (mod, f, (start, node.end_byte), decl_bytes, is_bin)
            )

    if not type_decls:
        return

    # Collect ALL type-replacement edits across ALL names BEFORE writing
    # any file.  Previously this loop wrote the file once per name, which
    # invalidated byte offsets for every subsequent name that touched the
    # same file (e.g. bzip2 has _IO_FILE × 6, bz_stream × 4, EState × 3
    # all sharing compress.rs; after writing _IO_FILE edits, the byte
    # ranges parsed for bz_stream / EState became stale and the
    # subsequent replace ended up cutting into the middle of a function
    # body — observed as `(*s).bspub use super::bzlib::EState;` on line
    # 79 of compress.rs in the disabled-path run).
    #
    # Fix: aggregate (start, end, replacement) per file across all
    # names, then apply each file's edits exactly once, sorted by
    # descending start so earlier offsets stay valid.
    edits_per_file: dict[Path, list[tuple[int, int, str]]] = {}
    summary_lines: list[str] = []
    for (name, kind), decls in type_decls.items():
        if len(decls) < 2:
            continue
        # Body-equality gate (2026-05-29). Same-name decls might be
        # semantically distinct types — c2rust uses synthetic names like
        # `C2RustUnnamed` / `C2RustUnnamed_0` for ANONYMOUS C structs/unions
        # whose layout differs across translation units. Unifying them
        # would replace one module's local type with a `pub use` of a
        # DIFFERENT type, causing E0560 ("struct X has no field named …")
        # at every cross-module access. Skip such names — they're
        # legitimately distinct per module.
        # Equality criterion: normalize whitespace + strip line comments
        # then compare. Identical-shape decls (typical for cross-module
        # `EState`/`bz_stream` clones) still unify; divergent ones
        # (anonymous `C2RustUnnamed`) get left alone.
        # Body-hash partitioning (Fix A++ 2026-05-29). Previously this
        # was an "all-or-nothing" gate: if ANY two same-name decls had
        # divergent bodies, we'd skip unifying ALL of them. That over-
        # blocks the common c2rust pattern where the SAME name covers
        # multiple distinct anonymous types — e.g. optipng has 5
        # `C2RustUnnamed` unions: two pairs share a body, plus a singleton.
        # Old behavior unified zero; correct behavior unifies each
        # body-identical pair separately so `pngxread::C2RustUnnamed` ≡
        # `ioutil::C2RustUnnamed` (both `{__wch, __wchb}`) and
        # `trees::C2RustUnnamed` ≡ `deflate::C2RustUnnamed` (both
        # `{dad, len}`), without conflating across the two layouts.
        body_groups: dict[bytes, list[tuple[str, Path, tuple[int, int], bytes, bool]]] = {}
        for d in decls:
            body_groups.setdefault(normalize_decl_for_compare(d[3]), []).append(d)
        n_groups = len(body_groups)
        for grp_idx, (_body, group_decls) in enumerate(body_groups.items()):
            if len(group_decls) < 2:
                # Singleton body — nothing to unify in this group.
                continue
            # Canonical selection within the group: prefer LIB files over
            # bin files, then by size (largest = richest module). Bin
            # files are never canonical because `pub use crate::<bin>::X`
            # from lib doesn't resolve.
            decls_sorted = sorted(
                group_decls,
                key=lambda d: (d[4], -d[1].stat().st_size),
            )
            canon_mod, canon_file, _, _, canon_is_bin = decls_sorted[0]
            if canon_is_bin:
                # Pathological: name only exists in bin sources. Skip —
                # no valid `pub use` path would help cross-bin consumers.
                continue
            for mod, f, rng, _, is_bin in decls_sorted[1:]:
                # Path prefix depends on TARGET (file being rewritten):
                #   lib  →  `pub use crate::<canon_mod>::X;`
                #   bin  →  `pub use <pkg_name>::<canon_mod>::X;`
                prefix = lib_pkg_name if is_bin else "crate"
                edits_per_file.setdefault(f, []).append(
                    (rng[0], rng[1],
                     f"pub use {prefix}::{canon_mod}::{name};\n")
                )
            group_label = f"group {grp_idx + 1}/{n_groups}" if n_groups > 1 else ""
            summary_lines.append(
                f"[unify-types] {kind} {name} {group_label}: kept in {canon_mod}, "
                f"aliased from {[d[0] for d in decls_sorted[1:]]}"
            )

    for f, edits in edits_per_file.items():
        edits.sort(key=lambda x: -x[0])
        text = f.read_bytes()
        for s, e, repl in edits:
            text = text[:s] + repl.encode("utf-8") + text[e:]
        f.write_bytes(text)

    for line in summary_lines:
        logger.info(line)


# ---------------------------------------------------------------------------
# Pass 2: unify opaque `extern "C" { pub type X; }` foreign types
# ---------------------------------------------------------------------------

def unify_opaque_foreign_types(crate_dir: Path) -> None:
    """Cross-module unify for `extern "C" { pub type X; }` opaque foreign
    types. c2rust emits these per translation unit for C structs whose
    layout is unknown / unused, e.g. `extern { pub type internal_state; }`
    in every zlib module. Each is a DISTINCT nominal type to rustc, so
    `*mut zlib::inflate::internal_state` ≠ `*mut zlib::uncompr::internal_state`
    even though both came from `struct internal_state` in the same C
    header — E0308 at every cross-module fn call.

    Detection (tree-sitter): inside each `foreign_mod_item.declaration_list`,
    find ERROR nodes containing the `type` keyword + an identifier child.
    tree-sitter-rust does NOT have a stable production for `pub type X;`
    (`extern_types` is an unstable rustc feature), so it materializes as
    an ERROR node whose constituents we walk explicitly. The semicolon
    is a sibling `empty_statement` node; we eat it + the trailing newline
    into the delete range.

    Action: for each name appearing in ≥2 modules, pick a canonical
    (largest file = richest module) and replace other modules' `pub type
    X;` decls with empty (delete) + insert a `pub use crate::<canon>::X;`
    immediately before the foreign_mod_item that hosted the deleted decl.
    If the foreign block becomes empty after deletion, it's left as an
    empty `extern "C" {}` — harmless and rustc-accepted.
    """
    lang = Language(tree_sitter_rust.language())
    parser = Parser(lang)

    src_dir = crate_dir / "src"
    if not src_dir.is_dir():
        return

    bin_paths = _collect_bin_paths(crate_dir)
    mod_files = _enumerate_modules(crate_dir, bin_paths)
    lib_pkg_name = _read_lib_pkg_name(crate_dir)

    # name → list of (mod_path, file, delete_range, foreign_mod_start_byte, is_bin)
    # delete_range covers the ERROR node + its sibling ';' + trailing '\n'
    # so the file is left tidy after removal.
    opaque_decls: dict[str, list[tuple[str, Path, tuple[int, int], int, bool]]] = {}
    # name → mod_path where a concrete `pub struct/union/enum X { ... }`
    # of the same name is defined. Only LIB modules can claim
    # canonical-source status; bin modules might happen to host one,
    # but `pub use crate::<bin>::X` is unusual and harness-side bin can't
    # easily be a `crate::` path from lib anyway. Hence the is_bin filter.
    concrete_decls: dict[str, str] = {}
    for (mod, f, is_bin) in mod_files:
        if not f.is_file():
            continue
        src = f.read_bytes()
        tree = parser.parse(src)
        for fmi in tree.root_node.children:
            # Collect concrete pub struct / union / enum decls at top
            # level — these become preferred canonicals.
            if fmi.type in ("struct_item", "enum_item", "union_item"):
                has_pub = any(c.type == "visibility_modifier"
                              and src[c.start_byte:c.end_byte] == b"pub"
                              for c in fmi.children)
                if not has_pub:
                    continue
                name_node = next((c for c in fmi.children
                                  if c.type == "type_identifier"), None)
                if name_node is None:
                    continue
                cname = src[name_node.start_byte:name_node.end_byte].decode(
                    "utf-8", "replace"
                )
                # Only LIB modules can be canonical sources. Bin sources
                # might happen to have struct decls but `crate::<bin>::X`
                # paths are unusual and easily broken.
                if not is_bin:
                    concrete_decls.setdefault(cname, mod)
                continue
            if fmi.type != "foreign_mod_item":
                continue
            dlist = next((c for c in fmi.children
                          if c.type == "declaration_list"), None)
            if dlist is None:
                continue
            # Recursive walk: tree-sitter's error recovery is inconsistent
            # for `pub type X;` inside `extern "C" { … }`. Sometimes the
            # decl materializes as a top-level ERROR sibling of
            # function_signature_item; other times the `pub` is absorbed
            # as a visibility_modifier on the NEXT fn_sig_item and the
            # `type X;` part lands as a NESTED ERROR child of that
            # fn_sig_item (observed on optipng gzwrite.rs and many peers).
            # Walking the whole subtree catches both shapes.
            stack = [dlist]
            while stack:
                node = stack.pop()
                stack.extend(node.children)
                if node.type != "ERROR":
                    continue
                # An ERROR node for `(pub) type X;` typically contains:
                #   `type` keyword + identifier + `;`
                # (visibility, if present, may have been hoisted up to the
                # parent fn_sig_item.) Detect by these two presences.
                if not any(c.type == "type" for c in node.children):
                    continue
                ident = next((c for c in node.children
                              if c.type == "identifier"), None)
                if ident is None:
                    continue
                name = src[ident.start_byte:ident.end_byte].decode(
                    "utf-8", "replace"
                )
                # The semicolon is usually a CHILD of the ERROR node in the
                # nested form, or a sibling `empty_statement` in the
                # top-level form. Cover both by extending `end` past any
                # immediately-following `;` and trailing newline.
                end = node.end_byte
                # Eat sibling empty_statement (top-level form)
                parent = node.parent
                if parent is not None:
                    sibs = list(parent.children)
                    try:
                        i_in_parent = sibs.index(node)
                        if (i_in_parent + 1 < len(sibs)
                                and sibs[i_in_parent + 1].type == "empty_statement"):
                            end = sibs[i_in_parent + 1].end_byte
                    except ValueError:
                        pass
                # Eat trailing newline.
                if end < len(src) and src[end:end + 1] == b"\n":
                    end += 1
                # Eat leading whitespace.
                start = node.start_byte
                # For the NESTED form, the `pub` visibility was hoisted up
                # as a sibling visibility_modifier on the same fn_sig_item.
                # We must delete that too — otherwise we leave a dangling
                # `pub fn vsnprintf(...)` whose pub doesn't bind to anything.
                # Walk up from `node` to find a fn_sig_item or
                # function_signature_item; if the previous sibling is a
                # visibility_modifier whose text is `pub`, include it in
                # the delete range.
                p = node.parent
                if p is not None and p.type == "function_signature_item":
                    sibs = list(p.children)
                    # Find the ERROR's index within fn_sig_item
                    try:
                        i_in_p = sibs.index(node)
                    except ValueError:
                        i_in_p = -1
                    if i_in_p > 0 and sibs[i_in_p - 1].type == "visibility_modifier":
                        vis = sibs[i_in_p - 1]
                        if src[vis.start_byte:vis.end_byte] == b"pub":
                            start = vis.start_byte
                line_start = start
                while line_start > 0 and src[line_start - 1:line_start] in (b" ", b"\t"):
                    line_start -= 1
                opaque_decls.setdefault(name, []).append(
                    (mod, f, (line_start, end), fmi.start_byte, is_bin)
                )

    if not opaque_decls:
        return

    # Group edits per file, then commit.
    edits_per_file: dict[Path, list[tuple[int, int, str]]] = {}
    summary_lines: list[str] = []
    for name, entries in opaque_decls.items():
        # Decide canonical:
        #   · If a concrete `pub struct/union/enum X { ... }` exists in
        #     SOME module, it wins — all opaque `pub type X;` decls
        #     become aliases of the concrete type. Even with just ONE
        #     opaque entry, we still want to alias (otherwise the opaque
        #     stays distinct and breaks cross-fn calls — observed on
        #     optipng zlib::deflate concrete `struct internal_state`
        #     with `wrap` field vs other modules' opaque `internal_state`).
        if name in concrete_decls:
            canon_mod = concrete_decls[name]
            tag = "concrete"
        else:
            if len(entries) < 2:
                continue
            entries_sorted = sorted(entries, key=lambda d: -d[1].stat().st_size)
            canon_mod = entries_sorted[0][0]
            tag = "opaque"
        for mod, f, (s, e), fmi_start, is_bin in entries:
            if mod == canon_mod:
                # Already at the canonical module — leave the opaque
                # decl in place.
                continue
            edits_per_file.setdefault(f, []).append((s, e, ""))
            # Path prefix: bin sources are a SEPARATE crate from the lib
            # — `crate::<mod>` resolves to the bin's own root which lacks
            # the lib's module tree. Bin-side aliases must use the lib's
            # package name (e.g. `libxml2_cleaned::src::catalog::_xmlBuf`)
            # so the import resolves through the package's lib target.
            # Lib-side aliases still use `crate::<mod>` — `crate::` from
            # within the lib IS the lib root.
            path_prefix = lib_pkg_name if is_bin else "crate"
            edits_per_file.setdefault(f, []).append(
                (fmi_start, fmi_start,
                 f"pub use {path_prefix}::{canon_mod}::{name};\n")
            )
        summary_lines.append(
            f"[unify-opaque-types] {name}: kept in {canon_mod} ({tag}), "
            f"aliased from {[e[0] for e in entries if e[0] != canon_mod]}"
        )

    for f, edits in edits_per_file.items():
        # Apply right-to-left so earlier offsets stay valid. Multiple
        # edits at the same start byte (insertion + adjacent delete) are
        # naturally ordered by descending start.
        edits.sort(key=lambda x: (-x[0], x[1] - x[0]))
        text = f.read_bytes()
        for s, e, repl in edits:
            text = text[:s] + repl.encode("utf-8") + text[e:]
        f.write_bytes(text)

    for line in summary_lines:
        logger.info(line)
