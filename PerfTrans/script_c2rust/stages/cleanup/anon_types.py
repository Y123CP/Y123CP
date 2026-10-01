"""Canonicalize per-TU C2RustUnnamed_<N> names by body hash.

c2rust assigns synthetic `C2RustUnnamed_<N>` names to anonymous C
structs/unions by per-file occurrence order. The same underlying C type
gets DIFFERENT N in different translation units (e.g. tmux's
`ibuf.entry` field type is `C2RustUnnamed_19` in `proc.rs`,
`C2RustUnnamed_8` in `server_client.rs`, `C2RustUnnamed_20` in
`client.rs`, `C2RustUnnamed_0` in `imsg_buffer.rs` — same C anon
struct, four different names).

Without this pass, `pub_type_unify` sees parent struct bodies that look
textually distinct (`pub entry: C2RustUnnamed_19` vs `pub entry:
C2RustUnnamed_8`) and skips them under its body-divergence guard,
leaving each translation unit's `ibuf` (and 14 other tmux types) as a
distinct nominal type — 1245 E0308 errors in tmux ours-build.
"""

from __future__ import annotations

import hashlib
import logging
import re
from pathlib import Path

import tree_sitter_rust
from tree_sitter import Language, Parser

from .decl_compare import normalize_decl_for_compare

logger = logging.getLogger(__name__)


# c2rust uses two synthetic-name shapes inconsistently:
#   `C2RustUnnamed`           ← bare form (often the first anon
#                                struct/union in a TU)
#   `C2RustUnnamed_<digits>`  ← suffixed form (subsequent anons,
#                                or sometimes from the first)
# Observed on tmux: `status.rs` has `pub ev_: C2RustUnnamed,` (bare)
# while `format.rs` has `pub ev_: C2RustUnnamed_<N>` for the same
# underlying C anonymous union. Match BOTH.
#
# ALSO match the canonical form `C2RustUnnamed_h<kind><hash>` that
# earlier iterations produced — without this, the fixpoint loop is
# blind to outer unions whose inner refs got renamed in iter N but
# whose own decl name was already a `_h…` from iter N-1. Iter N+1
# would then never see the matching outer bodies and miss the merge.
_NAME_RE = re.compile(r'^C2RustUnnamed(_(?:\d+|h[stu][0-9a-f]+))?$')


def canonicalize_c2rust_anon_types(crate_dir: Path, max_iters: int = 8) -> None:
    """Iterate `_one_pass` to fixpoint so deeply-nested anonymous-type
    names canonicalize correctly.

    Why iterate: c2rust's per-TU anonymous types nest. A union may
    contain a field whose type is itself a `C2RustUnnamed_<N>` struct.
    On iteration 1, the OUTER union's body hash is computed BEFORE
    its INNER `C2RustUnnamed_<N>` refs have been canonicalized — so
    outer bodies look textually distinct across modules even when
    semantically identical. Iteration N+1 sees the inner refs already
    canonical from iteration N, lets the outer match. Continues until
    a pass produces zero rewrites (fixpoint reached) or the safety
    cap (`max_iters`) kicks in.

    Observed on tmux: `pub union C2RustUnnamed_<X>` in status.rs and
    format.rs both contain `pub ev_next_with_common_timeout:
    C2RustUnnamed_<Y>, pub min_heap_idx: c_int` — same body, but `<Y>`
    differs per file. After 1 iteration the inner `<Y>` canonicalizes;
    iteration 2 sees the outer bodies match and unifies them too.
    """
    for _ in range(max_iters):
        if _one_pass(crate_dir) == 0:
            return


def _one_pass(crate_dir: Path) -> int:
    """One canonicalization pass; returns total rewrites done
    (renames + deletions). Caller loops to fixpoint.

    Pass:
      1. tree-sitter walk every `struct_item` / `union_item` / `type_item`
         whose name matches `C2RustUnnamed[_<suffix>]?$`
      2. Hash the BODY (field list bytes, normalized) — name excluded
         so same-shape variants collide regardless of their N
      3. For each body-hash group with ≥2 occurrences, assign a stable
         canonical name `C2RustUnnamed_h<kind><8-char-md5>`
      4. Per-file: rewrite every `type_identifier` node whose text is in
         that file's rename map to the canonical name; delete duplicate
         decls that collapse to the same canonical in the same file
    """
    lang = Language(tree_sitter_rust.language())
    parser = Parser(lang)

    src_dir = crate_dir / "src"
    if not src_dir.is_dir():
        return 0

    # Step 1+2: collect decls grouped by body hash.
    # Each entry carries enough info to also DELETE the decl byte range
    # later (incl. preceding `#[derive(...)]` / `#[repr(C)]` attrs).
    decls: list[tuple[Path, str, str, int, int]] = []
    #                ↑file ↑name ↑hash ↑decl_full_start ↑decl_full_end
    for f in src_dir.rglob("*.rs"):
        if "target" in f.parts:
            continue
        try:
            src = f.read_bytes()
        except OSError:
            continue
        tree = parser.parse(src)
        for n in tree.root_node.children:
            # c2rust emits 3 shapes of synthetic name:
            #   struct_item  `pub struct C2RustUnnamed_<N> { ... }`
            #   union_item   `pub union  C2RustUnnamed_<N> { ... }`
            #   type_item    `pub type   C2RustUnnamed_<N> = <RHS>;`
            #                 (c2rust lowers C anonymous enums to a
            #                  type alias + a batch of `pub const`s)
            # All three need canonicalization — tmux's `pub type
            # C2RustUnnamed_30 = c_uint` decls were the missing piece
            # blocking 1245→167 → ... E0308 errors.
            if n.type not in ("struct_item", "union_item", "type_item"):
                continue
            name_node = next((c for c in n.children
                              if c.type == "type_identifier"), None)
            if name_node is None:
                continue
            name = src[name_node.start_byte:name_node.end_byte].decode(
                "utf-8", "replace"
            )
            if not _NAME_RE.match(name):
                continue
            # Body bytes = whatever distinguishes one variant from another:
            #   struct/union: the `field_declaration_list` (i.e. `{...}`)
            #   type alias:   everything after the `=` up to (not incl.) `;`
            # Hash JUST this — name is excluded so same-shape variants
            # with different N land in the same bucket.
            if n.type == "type_item":
                rhs = None
                saw_eq = False
                for c in n.children:
                    if c.type == "=":
                        saw_eq = True
                        continue
                    if saw_eq and c.type != ";":
                        rhs = c
                        break
                if rhs is None:
                    continue
                body_bytes = src[rhs.start_byte:rhs.end_byte]
            else:
                body_node = next((c for c in n.children
                                  if c.type == "field_declaration_list"), None)
                if body_node is None:
                    continue
                body_bytes = src[body_node.start_byte:body_node.end_byte]
            body_norm = normalize_decl_for_compare(body_bytes)
            body_hash = hashlib.md5(body_norm).hexdigest()[:8]
            # Tag the hash by kind so a `struct C2RustUnnamed_5` with
            # body `{x: u32}` and a `type C2RustUnnamed_5 = u32` aren't
            # accidentally pooled — different kinds of decl.
            body_hash = f"{n.type[0]}{body_hash}"
            # Compute full decl range incl. preceding attribute_item siblings
            # so a deletion strips `#[derive(...)] #[repr(C)] pub struct …{…}`.
            decl_start = n.start_byte
            parent = n.parent
            if parent is not None:
                sibs = list(parent.children)
                try:
                    idx = sibs.index(n)
                except ValueError:
                    idx = -1
                j = idx - 1
                while j >= 0 and sibs[j].type == "attribute_item":
                    decl_start = sibs[j].start_byte
                    j -= 1
            decl_end = n.end_byte
            # Eat trailing newline so deletion doesn't leave a blank line.
            if decl_end < len(src) and src[decl_end:decl_end + 1] == b"\n":
                decl_end += 1
            decls.append((f, name, body_hash, decl_start, decl_end))

    if not decls:
        return 0

    # Step 3: canonical names per body-hash group of ≥2 occurrences.
    # Singleton hashes don't need renaming (no conflict to resolve).
    hash_count: dict[str, int] = {}
    for _, _, h, _, _ in decls:
        hash_count[h] = hash_count.get(h, 0) + 1
    canonical_names = {
        h: f"C2RustUnnamed_h{h}" for h, n in hash_count.items() if n >= 2
    }
    if not canonical_names:
        return 0

    # Per-file rename map: each file's old_name → canonical.
    # Also: per-file delete map — when a single file has MULTIPLE decls
    # collapsing to the SAME canonical (e.g. libxml2 threads.rs has two
    # local C2RustUnnamed_<a> and C2RustUnnamed_<b> with identical bodies),
    # the rename produces two `pub struct C2RustUnnamed_h…` decls in the
    # same file → E0255 duplicate definition. Solution: keep the FIRST
    # occurrence per (file, canonical) pair, delete the rest's full decl
    # range (incl. attrs + trailing newline).
    file_renames: dict[Path, dict[str, str]] = {}
    file_deletes: dict[Path, list[tuple[int, int]]] = {}
    seen_per_file: dict[tuple[Path, str], bool] = {}
    # Track which canonical(s) a given old_name maps to ACROSS the project.
    # If a name `C2RustUnnamed_3` is declared in 8 modules and all 8 map
    # to the same canonical (same body shape), then refs to `_3` in any
    # OTHER module (like Stage 1's auto-gen `c_consts.rs` / `c_types.rs`
    # that consolidate `pub const`/`pub type` from per-TU sources but
    # don't have local decls) can be safely globally-renamed too. If
    # multiple distinct canonicals → ambiguous, refs in non-decl files
    # left alone (the local `use super::*` decides which one is in scope).
    old_to_canon: dict[str, set[str]] = {}
    for f, old_name, h, decl_start, decl_end in decls:
        # Singleton body-hashes are not renamed (they don't appear in
        # `canonical_names`), but they still "claim" the old_name as
        # their effective canonical — record that fact in old_to_canon
        # so a peer module that DOES rename the same old_name to a
        # different canonical isn't able to silently capture this
        # decl's refs via global_rename. Without this, libxml2's
        # `pub union C2RustUnnamed_1` in xmlschemastypes.rs (singleton
        # union body) was being captured by global_rename routing
        # `C2RustUnnamed_1` to a *type* canonical from another module
        # (E0428: duplicate definition).
        effective_canon = canonical_names.get(h, old_name)
        old_to_canon.setdefault(old_name, set()).add(effective_canon)
        if effective_canon == old_name:
            # Singleton: nothing to rename, nothing to dedup.
            continue
        if old_name != effective_canon:
            file_renames.setdefault(f, {})[old_name] = effective_canon
        key = (f, effective_canon)
        if seen_per_file.get(key):
            # Duplicate-in-file: mark this decl for deletion.
            file_deletes.setdefault(f, []).append((decl_start, decl_end))
        else:
            seen_per_file[key] = True

    # Global-safe rename map: old_name → canonical iff all decls of
    # old_name agree on the same canonical. Used to also rewrite refs
    # in files that have NO local decl (like c_consts.rs).
    global_rename: dict[str, str] = {
        old: next(iter(canons))
        for old, canons in old_to_canon.items()
        if len(canons) == 1
    }

    if not file_renames and not file_deletes and not global_rename:
        return 0

    # Step 4: walk EVERY .rs file under src/ — replace type_identifier
    # nodes per file_renames (when file has its own decls) OR per
    # global_rename (when only refs, no local decl — Stage 1 auto-gen
    # c_consts.rs / c_types.rs hit this case); also apply duplicate-decl
    # deletions for files in file_deletes.
    total_renamed = 0
    total_deleted = 0
    touched_files: set[Path] = set()
    for f in src_dir.rglob("*.rs"):
        if "target" in f.parts:
            continue
        file_rename = file_renames.get(f)
        deletes = file_deletes.get(f, [])
        # Effective rename for this file: prefer file-local map; for
        # entries not in file-local (because no local decl), fall back
        # to global_rename. The local map's canonical is authoritative
        # since the file has its own decl with that body.
        eff: dict[str, str] = dict(global_rename)
        if file_rename:
            eff.update(file_rename)
        if not eff and not deletes:
            continue
        src = f.read_bytes()
        tree = parser.parse(src)
        edits: list[tuple[int, int, str]] = []
        if eff:
            stack = [tree.root_node]
            while stack:
                n = stack.pop()
                stack.extend(n.children)
                if n.type != "type_identifier":
                    continue
                text = src[n.start_byte:n.end_byte].decode("utf-8", "replace")
                if text in eff:
                    edits.append((n.start_byte, n.end_byte, eff[text]))
            total_renamed += len(edits)
        # Add duplicate-decl deletions. Filter rename edits whose byte
        # range falls inside any delete range — those would be redundant
        # (the delete subsumes them).
        if deletes:
            del_ranges = [(s, e) for s, e in deletes]
            edits = [
                (s, e, r) for (s, e, r) in edits
                if not any(ds <= s and e <= de for ds, de in del_ranges)
            ]
            for s, e in del_ranges:
                edits.append((s, e, ""))
            total_deleted += len(del_ranges)
        if not edits:
            continue
        edits.sort(key=lambda x: -x[0])
        text = src
        for s, e, repl in edits:
            text = text[:s] + repl.encode("utf-8") + text[e:]
        f.write_bytes(text)
        touched_files.add(f)

    logger.info(
        f"[canon-c2rust-anon] canonicalized {len(canonical_names)} body-hash group(s); "
        f"global-safe rename names = {len(global_rename)}; "
        f"applied {total_renamed} type_identifier rename(s) and "
        f"{total_deleted} duplicate-decl deletion(s) across "
        f"{len(touched_files)} file(s)"
    )
    return total_renamed + total_deleted
