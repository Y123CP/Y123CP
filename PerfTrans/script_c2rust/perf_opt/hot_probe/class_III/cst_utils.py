"""Class III detector — tree-sitter-rust CST shared utilities.

Central home for CST helpers used by all four rule modules. Never fall back
to regex here (SPEC §0 D0). All predicates operate on `tree_sitter.Node`.

Contents (all referenced from SPEC):
  - _parser()                        — lazy tree-sitter-rust parser
  - parse_crate(crate_root)          — walk `.rs` files, return {path: Tree}
  - is_fn_ptr(type_node)             — §1.1 fn-ptr type predicate
  - collect_fn_items(trees)          — §7.1 {FnKey: FnCstEntry}
  - collect_struct_fields(trees)     — §1.1 (iii) {struct → {field → type_node}}
  - collect_extern_c_decls(trees)    — §2.1 Ext (declare-only extern "C" fns)
  - collect_non_extern_base(trees)   — §2.2 project non-extern fn base-name set
  - collect_ref_types(fn_cst, ...)   — §4.3 RefTypes(f) with recursive descent
  - crate_qualified_path(node)       — §7.1 FnKey builder
  - callee_name_and_prefix(fn_node)  — §2.3 / §2.4 D14 three-tier classifier
  - receiver_type_at(node, ...)      — §1.4 receiver-chain type resolution
                                       (depth cap = config.RECEIVER_FIELD_DEPTH_CAP)
  - iter_call_expressions(fn_cst)    — helper: yield every call_expression node
                                       inside a fn body

Not covered here:
  - Rule-specific detection (III①/②/③/④ live in their own modules)
  - I/O (build wrappers, JSON serialization) — handled in scan.py / build_typesize.py
"""

from __future__ import annotations

import logging
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Iterator

from tree_sitter import Language, Node, Parser, Tree
import tree_sitter_rust

from perf_opt.hot_probe.class_III.config import (
    EXTERNAL_CRATE_PREFIXES,
    INTERNAL_CRATE_PREFIXES,
    RECEIVER_FIELD_DEPTH_CAP,
)

logger = logging.getLogger(__name__)

# ---- FnKey / core dataclasses ----------------------------------------------

FnKey = str  # e.g. "crate::mod::foo", "crate::impl<T>::method", or fallback
             # "<file_relpath>::<fn_name>". See SPEC §7.1.


@dataclass(frozen=True)
class FnCstEntry:
    """CST anchor for a fn definition (§7.1). Read-only across passes."""
    key: FnKey
    file: Path
    node: Node                     # the `function_item` node
    name: str                      # base name (`foo`)
    qualified_path: FnKey          # crate-qualified path (equal to `key`)


@dataclass
class StructIndex:
    """Whole-crate {struct → {field → type_node}} map (§1.1 (iii))."""
    by_name: dict[str, dict[str, Node]] = field(default_factory=dict)
    # Optional: track fully-qualified path for D15 shadow detection.
    fq_paths: dict[str, list[str]] = field(default_factory=dict)


@dataclass
class ExternIndex:
    """extern "C" declare-only fn signatures (§2.1). Ext[fn_name] = sig node."""
    decls: dict[str, Node] = field(default_factory=dict)
    # source file for each decl, for pre-check bookkeeping only.
    origin: dict[str, Path] = field(default_factory=dict)


class CalleePrefixKind(Enum):
    """§2.4 three-tier callee classification (D14)."""
    EXTERNAL   = "external"   # e.g. libc::strlen — authoritatively extern
    INTERNAL   = "internal"   # e.g. crate::foo   — authoritatively project-local
    BARE       = "bare"       # e.g. strlen(...)  — needs ambiguous fallback
    UNKNOWN    = "unknown"    # unrecognized scoped prefix (rare)


@dataclass(frozen=True)
class CallSite:
    ""                                                                

                                                                          
                                                                       
                                       

                                        
                                                           
                                                                         
                                                                               
                                                                
                                                
                                             
       
    file: str          # crate-relative path
    line: int          # 1-indexed
    col: int           # 1-indexed
    callee_name: str   # base name (`strlen`, `malloc`, or dynamic-callee label)
    marker_seen: bool = False
    form: str = ""                                        
    callback_fn: str = ""
    """Form E only — the name of the function whose pointer is handed over.

    `qsort(base, n, size, Some(Cmp as unsafe extern "C" fn(..) -> c_int))`
    records `Cmp`. The rewrite is a *semantic* one: the card requires reading
    the comparator to prove the ordering key and direction are unchanged, and
    that body lives in another item, outside the edited region. Without the
    name there is no way to fetch it, and the model can only abstain. Empty
    when the pointer is not a plain `ident as fn(..)` cast.
    """
    binding: str = ""
    """Where the callee name was bound: "param", "let", "static", or "".

    Form A/B resolve the callee through the function's scope, and that scope
    folds in module-level statics so that `xmlFree.expect("..")(args)` resolves
    at all. The consequence is that a global allocator hook and a genuine
    callback parameter reach the rule looking identical — both come out
    labelled `B_param`. They are not interchangeable: monomorphising a
    parameter is a local edit, while a `static mut` hook can be reassigned by
    any translation unit, so III① cannot soundly specialise it without proving
    nobody ever does. Measured across the dataset: every one of libxml2's
    static-bound sites was declined by the model for exactly that reason, one
    model call each, while its two parameter-bound sites (`channel`) committed.
    """


# ---- Parser (lazy singleton) ------------------------------------------------

_PARSER: Parser | None = None


def _parser() -> Parser:
    """Return a lazily-initialized tree-sitter-rust parser."""
    global _PARSER
    if _PARSER is None:
        p = Parser()
        p.language = Language(tree_sitter_rust.language())
        _PARSER = p
    return _PARSER


def parse_crate(crate_root: Path) -> dict[Path, Tree]:
    """Walk a c2rust crate's `src/` tree, parse every `.rs` file.

    Returns {relative_path: Tree}. Skips `target/`, hidden dirs, and any
    directory named `tests` / `benches` (c2rust output rarely populates
    these, but we defensively exclude them so foreign harnesses don't
    pollute the FnKey space).
    """
    src_root = crate_root / "src"
    if not src_root.is_dir():
        logger.warning(f"[cst] no src/ at {crate_root} — parse_crate returns empty")
        return {}

    parser = _parser()
    trees: dict[Path, Tree] = {}
    skip_dir_names = {"target", "tests", "benches"}

    for path in src_root.rglob("*.rs"):
        # skip hidden and skip_dir_names anywhere in the path
        rel_parts = path.relative_to(crate_root).parts
        if any(part.startswith(".") or part in skip_dir_names for part in rel_parts):
            continue
        try:
            data = path.read_bytes()
        except OSError as e:
            logger.warning(f"[cst] failed to read {path}: {e}")
            continue
        trees[path.relative_to(crate_root)] = parser.parse(data)

    logger.info(f"[cst] parsed {len(trees)} .rs file(s) under {src_root}")
    return trees


# ---- Type predicates (§1.1) -------------------------------------------------

_TYPE_ALIAS_DEPTH_CAP = 5  # defensive cap against alias cycles (rare)


def is_fn_ptr(
    type_node: Node,
    type_aliases: dict[str, Node] | None = None,
    _alias_depth: int = 0,
) -> bool:
    ""                            

                                                         
                                                                     
                                        
                                                                         
                                                              
                                                                       
                                                                     
                                                                       
                                                   

                                                                
                                                                    
                                                                       
                                                               
                                               

                                                       
       
    if type_node.type == "function_type":
        return True

    if type_node.type == "generic_type":
        outer = type_node.child_by_field_name("type")
        if outer is None:
            return False
        outer_last_seg = outer.text.rsplit(b"::", 1)[-1]
        if outer_last_seg != b"Option":
            return False
        targs = type_node.child_by_field_name("type_arguments")
        if targs is None:
            return False
        for i in range(targs.named_child_count):
            inner = targs.named_child(i)
            if inner is None or inner.type in ("lifetime",):
                continue
            if is_fn_ptr(inner, type_aliases, _alias_depth):
                return True
        return False

    if (type_aliases is not None
            and type_node.type in ("type_identifier", "scoped_type_identifier")
            and _alias_depth < _TYPE_ALIAS_DEPTH_CAP):
        base = _extract_type_basename(type_node)
        if base is not None:
            aliased = type_aliases.get(base)
            if aliased is not None:
                return is_fn_ptr(aliased, type_aliases, _alias_depth + 1)

    return False


# ---- Crate-wide indices -----------------------------------------------------

def collect_fn_items(trees: dict[Path, Tree]) -> dict[FnKey, FnCstEntry]:
    """§7.1 — collect all `function_item` nodes into a FnKey-indexed map.

    Uses `crate_qualified_path()` to build the key. Macro-generated fns
    without a stable ancestor chain fall back to `(file_relpath, fn_name)`
    (see SPEC §7.1).

    NOTE: `function_signature_item` (declare-only fns inside
    `extern "C" { ... }`) are NOT collected here — those go through
    `collect_extern_c_decls`.
    """
    out: dict[FnKey, FnCstEntry] = {}
    dupes: list[FnKey] = []
    for file_relpath, tree in trees.items():
        for fn_node in _iter_nodes_by_type(tree.root_node, "function_item"):
            name_node = fn_node.child_by_field_name("name")
            if name_node is None:
                continue
            base_name = name_node.text.decode(errors="replace")
            key = crate_qualified_path(fn_node, file_relpath)
            if key in out:
                # Duplicate FnKey — very rare (e.g. cfg-gated dupes).
                dupes.append(key)
                # Disambiguate by appending file to keep both entries.
                key = f"{file_relpath}::{key.rsplit('::', 1)[-1]}"
            out[key] = FnCstEntry(
                key=key,
                file=file_relpath,
                node=fn_node,
                name=base_name,
                qualified_path=key,
            )
    if dupes:
        logger.info(f"[cst] collect_fn_items: {len(dupes)} FnKey collision(s) "
                    f"disambiguated by file-relpath fallback")
    logger.info(f"[cst] collected {len(out)} fn item(s)")
    return out


def _iter_nodes_by_type(root: Node, node_type: str):
    """DFS every descendant of `root` whose `.type == node_type`. Yields Node."""
    if root.type == node_type:
        yield root
    for child in root.children:
        yield from _iter_nodes_by_type(child, node_type)


def collect_struct_fields(trees: dict[Path, Tree]) -> StructIndex:
    """§1.1 (iii) — collect struct definitions and their {field: type_node}.

    Used by:
      - §1.4 receiver type chain (field access chain)
      - §4.1 pre-check base-name catalogue for `-Zprint-type-sizes` diff
      - §4.3 type reference gathering

    Base-name shadowing (`mod_a::S` vs `mod_b::S`) is preserved in
    `StructIndex.fq_paths` for §4.1 D15 warn-status detection.
    `StructIndex.by_name` is keyed by BASE NAME — if two structs share
    a base name the later one overwrites (defensively; §4.4 iii4 hit
    detection is conservative under shadowing).
    """
    out = StructIndex()
    for file_relpath, tree in trees.items():
        for s_node in _iter_nodes_by_type(tree.root_node, "struct_item"):
            name_node = s_node.child_by_field_name("name")
            if name_node is None:
                continue
            base_name = name_node.text.decode(errors="replace")

            qpath = crate_qualified_path(s_node, file_relpath)
            out.fq_paths.setdefault(base_name, []).append(qpath)

            body = s_node.child_by_field_name("body")
            fields: dict[str, Node] = {}
            if body is not None:
                for fd in body.children:
                    if fd.type != "field_declaration":
                        continue
                    fname_node = fd.child_by_field_name("name")
                    ftype_node = fd.child_by_field_name("type")
                    if fname_node is None or ftype_node is None:
                        continue
                    fields[fname_node.text.decode(errors="replace")] = ftype_node
            out.by_name[base_name] = fields
    logger.info(f"[cst] collected {len(out.by_name)} struct(s); "
                f"{sum(1 for v in out.fq_paths.values() if len(v) > 1)} shadowed base name(s)")
    return out


def collect_extern_c_decls(trees: dict[Path, Tree]) -> ExternIndex:
    """§2.1 — collect declare-only fns inside `extern "C" { ... }` blocks.

    ABI check: probe `foreign_mod_item.extern_modifier` and its
    inner `string_literal.string_content`. Non-`"C"` ABIs (e.g. `"Rust"`,
    `"system"`, or unlabeled `extern { ... }` which defaults to `"C"` in
    Rust) are treated as follows:
      - unlabeled extern block  → treated AS `"C"` (Rust semantics)
      - explicitly `"C"`         → collected
      - anything else            → skipped
    """
    out = ExternIndex()
    for file_relpath, tree in trees.items():
        for fmi in _iter_nodes_by_type(tree.root_node, "foreign_mod_item"):
            if not _foreign_mod_is_c_abi(fmi):
                continue
            # `body` field on foreign_mod_item is `declaration_list`.
            body = None
            for i in range(fmi.child_count):
                c = fmi.child(i)
                if c.type == "declaration_list":
                    body = c
                    break
            if body is None:
                continue
            for decl in body.children:
                if decl.type != "function_signature_item":
                    continue
                name_node = decl.child_by_field_name("name")
                if name_node is None:
                    continue
                base_name = name_node.text.decode(errors="replace")
                out.decls[base_name] = decl
                out.origin[base_name] = file_relpath
    logger.info(f"[cst] collected {len(out.decls)} extern \"C\" decl-only fn(s)")
    return out


def _foreign_mod_is_c_abi(fmi_node: Node) -> bool:
    """Check `foreign_mod_item`'s ABI. See collect_extern_c_decls policy."""
    # Find extern_modifier child, then its string_literal's string_content.
    for i in range(fmi_node.child_count):
        c = fmi_node.child(i)
        if c.type == "extern_modifier":
            # extern_modifier: `extern` [string_literal]?
            has_string = False
            for j in range(c.child_count):
                gc = c.child(j)
                if gc.type == "string_literal":
                    has_string = True
                    # Find string_content inside.
                    for k in range(gc.child_count):
                        ggc = gc.child(k)
                        if ggc.type == "string_content":
                            return ggc.text == b"C"
                    return False
            # extern_modifier with no string_literal — unlabeled extern
            # block; Rust treats this as extern "C".
            return not has_string  # True if no string_literal was seen
    # No extern_modifier at all (shouldn't happen for foreign_mod_item, but
    # defensively).
    return False


def collect_non_extern_base(fn_items: dict[FnKey, FnCstEntry]) -> set[str]:
    """§2.2 — set of base names of every project-defined (non-extern) fn.

    Used by §2.4 fallback tier 3 ambiguous check. Just take the base name
    of each FnCstEntry (i.e. the last segment of the qualified path).
    """
    return {entry.name for entry in fn_items.values()}


def collect_static_items(trees: dict[Path, Tree]) -> dict[str, Node]:
    """Collect `static [mut] NAME: T = ...;` items across the crate.

    Returns `{static_name → type_node}`. Used as a fall-back scope for
    `receiver_type_at` — c2rust routinely emits callback tables as
    module-level statics:

        pub static mut xmlFree: Option<unsafe extern "C" fn(...)> = None;

    and then dispatches via
        `xmlFree.expect("non-null function pointer")(args)`.

    Without recognizing static-var identifiers as fn-ptr bindings,
    every such call falls through form-B receiver_type_at → 87% miss on
    libxml2 (implementation gap discovered post-Step-7 audit).
    """
    out: dict[str, Node] = {}
    for _, tree in trees.items():
        for si in _iter_nodes_by_type(tree.root_node, "static_item"):
            name_n = si.child_by_field_name("name")
            type_n = si.child_by_field_name("type")
            if name_n is None or type_n is None:
                continue
            out[name_n.text.decode(errors="replace")] = type_n
    logger.info(f"[cst] collected {len(out)} static item(s)")
    return out


def collect_type_aliases(trees: dict[Path, Tree]) -> dict[str, Node]:
    """Collect `type NAME = <T>;` aliases across the crate.

    Returns {alias_base_name → aliased_type_node}. This exists because
    c2rust output routinely encodes callback shapes as typedef aliases:

        pub type mz_alloc_func = Option<unsafe extern "C" fn(...)>;
        pub struct mz_stream { pub zalloc: mz_alloc_func, ... }

    Without an alias table `is_fn_ptr` would look at the field type
    `mz_alloc_func` (a bare `type_identifier`) and return False — missing
    every III① form B/C hit in the crate.

    Later definitions with the same base name shadow earlier ones
    (very rare in c2rust output).
    """
    out: dict[str, Node] = {}
    for _, tree in trees.items():
        for ti in _iter_nodes_by_type(tree.root_node, "type_item"):
            name_n = ti.child_by_field_name("name")
            type_n = ti.child_by_field_name("type")
            if name_n is None or type_n is None:
                continue
            out[name_n.text.decode(errors="replace")] = type_n
    logger.info(f"[cst] collected {len(out)} type alias(es)")
    return out


# ---- Callee classification (§2.3 / §2.4 D14) --------------------------------

def callee_name_and_prefix(function_node: Node) -> tuple[str, CalleePrefixKind]:
    """Given a `call_expression.function` sub-node, return
    (base_name, prefix_kind) per SPEC §2.4 three-tier taxonomy.

    Handles:
      - `identifier`                        → (name, BARE)
      - `scoped_identifier`                 → (last_seg, classify prefix)
      - anything else (field_expression, method call, parenthesized,
        closure, ...)                        → ("", UNKNOWN)

    NOTE: `unsafe_block`-wrapped calls are NOT the callee of the wrapping
    unsafe block — they're independent `call_expression` nodes inside it.
    `iter_call_expressions` reaches them directly; this function is only
    ever called with a real `call_expression.function` sub-node.
    """
    if function_node.type == "identifier":
        return function_node.text.decode(errors="replace"), CalleePrefixKind.BARE
    if function_node.type == "scoped_identifier":
        # @name is the last segment; walk to innermost @path for first segment.
        name_node = function_node.child_by_field_name("name")
        if name_node is None:
            return "", CalleePrefixKind.UNKNOWN
        last_seg = name_node.text.decode(errors="replace")
        # Find first (leftmost) segment. Nested scoped_identifiers put the
        # left-most segment at the deepest .path chain.
        cur = function_node
        while True:
            path = cur.child_by_field_name("path")
            if path is None:
                # No path — this scoped_identifier looks like `::name` (absolute
                # root); treat as UNKNOWN prefix (rare in c2rust output).
                return last_seg, CalleePrefixKind.UNKNOWN
            if path.type == "scoped_identifier":
                cur = path
                continue
            # Leaf path segment. Could be identifier or a keyword like `crate`.
            first_seg = path.text.decode(errors="replace")
            break

        if first_seg in EXTERNAL_CRATE_PREFIXES:
            return last_seg, CalleePrefixKind.EXTERNAL
        if first_seg in INTERNAL_CRATE_PREFIXES:
            return last_seg, CalleePrefixKind.INTERNAL
        return last_seg, CalleePrefixKind.UNKNOWN
    # Everything else — method call, closure call, dynamic call, ...
    return "", CalleePrefixKind.UNKNOWN


# ---- Type-ref collection (§4.3 D11) -----------------------------------------

def collect_ref_types(
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
) -> set[str]:
    """§4.3 — collect all leaf named types referenced from a fn.

    Sources scanned (in fn signature + body):
      - fn params + return type
      - `let_declaration.type_annotation`
      - `struct_expression.name`
      - `type_cast_expression`'s target type (`x as T`)

    Recursively descends into `generic_type` / `array_type` / `tuple_type`
    / `reference_type` / `pointer_type` down to leaf
    `type_identifier` / `scoped_type_identifier`. `function_type` inner
    types are NOT recursed into (SPEC §4.3 last row — outer fn ptr type
    only is registered).

    Returns a set of base type names (last segment of scoped path). Caller
    intersects with `Large ∪ Padded` (both keyed by base name).
    """
    out: set[str] = set()

    # 1. fn signature — params + return type
    params = fn_entry.node.child_by_field_name("parameters")
    if params is not None:
        for i in range(params.child_count):
            c = params.child(i)
            if c.type == "parameter":
                t = c.child_by_field_name("type")
                if t is not None:
                    _collect_named_leafs(t, out)
    ret = fn_entry.node.child_by_field_name("return_type")
    if ret is not None:
        _collect_named_leafs(ret, out)

    # 2. body — let_declaration type annotations + struct_expression names +
    #    as-casts
    body = fn_entry.node.child_by_field_name("body")
    if body is not None:
        for let_node in _iter_nodes_by_type(body, "let_declaration"):
            t = let_node.child_by_field_name("type")
            if t is not None:
                _collect_named_leafs(t, out)
        for se in _iter_nodes_by_type(body, "struct_expression"):
            name_n = se.child_by_field_name("name")
            if name_n is not None:
                _collect_named_leafs(name_n, out)
        for cast in _iter_nodes_by_type(body, "type_cast_expression"):
            t = cast.child_by_field_name("type")
            if t is not None:
                _collect_named_leafs(t, out)
    return out


def _collect_named_leafs(type_node: Node, out: set[str], depth: int = 0) -> None:
    """Recursively walk a type CST and add every base-name leaf into `out`.

    `function_type` inner types are NOT recursed (SPEC §4.3 last row).
    Depth-capped (10) as a defensive stop against pathological recursion —
    e.g. deeply-nested phantomdata generics; typical c2rust nesting is 2-3.
    """
    if depth > 10:
        return
    t = type_node.type

    if t == "type_identifier":
        out.add(type_node.text.decode(errors="replace"))
        return

    if t == "scoped_type_identifier":
        name_n = type_node.child_by_field_name("name")
        if name_n is not None:
            out.add(name_n.text.decode(errors="replace"))
        return

    if t == "generic_type":
        outer = type_node.child_by_field_name("type")
        if outer is not None:
            # outer's last segment as a leaf (Vec / Box / Option itself)
            base = outer.text.rsplit(b"::", 1)[-1].decode(errors="replace")
            out.add(base)
        targs = type_node.child_by_field_name("type_arguments")
        if targs is not None:
            for i in range(targs.named_child_count):
                inner = targs.named_child(i)
                if inner is None or inner.type == "lifetime":
                    continue
                _collect_named_leafs(inner, out, depth + 1)
        return

    if t == "array_type":
        elem = type_node.child_by_field_name("element")
        if elem is not None:
            _collect_named_leafs(elem, out, depth + 1)
        return

    if t == "tuple_type":
        for i in range(type_node.named_child_count):
            c = type_node.named_child(i)
            if c is not None:
                _collect_named_leafs(c, out, depth + 1)
        return

    if t == "reference_type":
        # Skip lifetime children; recurse referent.
        for i in range(type_node.named_child_count):
            c = type_node.named_child(i)
            if c is not None and c.type != "lifetime":
                _collect_named_leafs(c, out, depth + 1)
        return

    if t == "pointer_type":
        # Skip `*`/`mut`/`const` tokens; recurse pointee.
        for i in range(type_node.named_child_count):
            c = type_node.named_child(i)
            if c is not None and c.type not in ("mutable_specifier",):
                _collect_named_leafs(c, out, depth + 1)
        return

    if t == "function_type":
        # SPEC §4.3 last row: don't recurse into fn-ptr inner types; the
        # fn-ptr itself contributes no `type_identifier` leaf.
        return

    # `primitive_type` (i32/u64/etc.) contributes no user type — drop.
    # `never_type` / `unit_type` / `dynamic_type` (dyn Trait) — drop.
    # Anything else — unknown shape; silent skip.


# ---- FnKey builder (§7.1) ---------------------------------------------------

def crate_qualified_path(item_node: Node, file_relpath: Path) -> FnKey:
    """§7.1 — build a crate-qualified path for a `function_item` (or
    `struct_item`) node.

    Walks ancestors: `mod_item` contributes its name; `impl_item`
    contributes `impl<T>` (impl-type text, truncated to first
    type_identifier) so overlapping method names on different impl
    blocks don't collide. Top-level `source_file` gives `crate::` prefix.

    Fallback (macro-generated fn without clean function_item ancestor
    chain, or when name extraction fails):
      `f"{file_relpath}::{fn_name_text_or_unknown}"`
    """
    name_node = item_node.child_by_field_name("name")
    if name_node is None:
        # Malformed / macro-expanded — fall back.
        return f"{file_relpath}::<unknown>"
    my_name = name_node.text.decode(errors="replace")

    segments: list[str] = []
    cur = item_node.parent
    while cur is not None:
        if cur.type == "mod_item":
            n = cur.child_by_field_name("name")
            if n is not None:
                segments.append(n.text.decode(errors="replace"))
            else:
                return f"{file_relpath}::{my_name}"
        elif cur.type == "impl_item":
            # impl block — take the `type` field (the Self type). Handles
            # `impl S {...}` and `impl Trait for S {...}` (both put the
            # Self-type at the `type` field on tree-sitter-rust).
            t = cur.child_by_field_name("type")
            if t is not None:
                # Grab the first type_identifier or scoped_type_identifier
                # text to keep the segment short and comparable.
                impl_seg = t.text.decode(errors="replace").strip()
                segments.append(f"impl<{impl_seg}>")
        elif cur.type == "source_file":
            break
        # Otherwise (function_item, block, ...) — skip; only mod / impl
        # contribute to the path.
        cur = cur.parent

    segments.reverse()
    prefix = "::".join(["crate"] + segments) if segments else "crate"
    return f"{prefix}::{my_name}"


# ---- Receiver-chain type resolution (§1.4 D10/D17) --------------------------

def build_local_scope(
    fn_entry: FnCstEntry,
    static_items: dict[str, Node] | None = None,
    *,
    origins: dict[str, str] | None = None,
) -> dict[str, Node]:
    """Build a fn's simple {identifier → type_node} scope.

    Covers:
      - fn parameters with a simple identifier pattern and explicit type
      - `let name: T = ...;` bindings anywhere in the body (only when the
        pattern is a single identifier AND a type annotation exists —
        SPEC §1.1 (ii))
      - module-level `static [mut] NAME: T = ...;` items (post-Step-7
        audit fix) — used as a fall-back so that references like
        `xmlFree.expect("...")(args)` in libxml2 resolve; local scope
        wins on shadow.

    Deliberately excludes:
      - `let` bindings without type annotation (type inference — out of
        reach without a full type checker)
      - tuple / struct destructuring patterns (rare in c2rust output)
      - closure captures / for-loop variables (not needed for III① forms
        A/B/C)

    Shadowing across nested blocks flattens (later `let` wins). This is
    a known simplification — precise scoping is a Step-5 optional
    refinement if downstream requires it.

    Pass `origins` to also receive `{name -> "param" | "let" | "static"}`.
    The scope alone cannot say which of the three a name came from, and for
    III① that is what decides whether a rewrite is expressible at all — see
    `CallSite.binding`.
    """
    scope: dict[str, Node] = {}

    def _bind(name: str, node: Node, origin: str) -> None:
        scope[name] = node
        if origins is not None:
            origins[name] = origin

    params = fn_entry.node.child_by_field_name("parameters")
    if params is not None:
        for i in range(params.child_count):
            c = params.child(i)
            if c.type != "parameter":
                continue
            pat = c.child_by_field_name("pattern")
            typ = c.child_by_field_name("type")
            if pat is None or typ is None:
                continue
            if pat.type == "identifier":
                _bind(pat.text.decode(errors="replace"), typ, "param")
    body = fn_entry.node.child_by_field_name("body")
    if body is not None:
        for let_node in _iter_nodes_by_type(body, "let_declaration"):
            pat = let_node.child_by_field_name("pattern")
            typ = let_node.child_by_field_name("type")
            if pat is None or typ is None:
                continue
            if pat.type == "identifier":
                _bind(pat.text.decode(errors="replace"), typ, "let")
    # Fold in module-level statics (only names not shadowed by local scope).
    # Fn params + let bindings take precedence — matches Rust name resolution
    # semantics; global statics are the fallback.
    if static_items:
        for name, tnode in static_items.items():
            if name not in scope:
                _bind(name, tnode, "static")
    return scope


def _unwrap_ptr_or_ref(t: Node) -> Node | None:
    """From `pointer_type` / `reference_type` return the pointee/referent
    type CST node. Rust auto-derefs through refs and (post-deref) pointers,
    so `receiver_type_at` chases through one level when resolving field
    access. Returns None if `t` is neither a pointer nor a reference.
    """
    if t.type not in ("pointer_type", "reference_type"):
        return None
    skip = {"*", "&", "const", "mut", "mutable_specifier", "lifetime"}
    for c in t.children:
        if c.is_named and c.type not in skip:
            return c
    return None


def resolve_type_alias(
    type_node: Node,
    type_aliases: dict[str, Node] | None,
    _depth: int = 0,
) -> Node:
    """Follow `type_identifier` / `scoped_type_identifier` through the alias
    table to its ultimate underlying type CST node. Returns `type_node`
    unchanged if it's not an alias or the alias table is None/empty.

    Used to differentiate raw fn ptr vs `Option<fn ptr>` when the field
    type is expressed via a typedef (§1.2 form C vs form B decision).

    Depth-capped at `_TYPE_ALIAS_DEPTH_CAP` against cycles.
    """
    if not type_aliases:
        return type_node
    if _depth >= _TYPE_ALIAS_DEPTH_CAP:
        return type_node
    if type_node.type not in ("type_identifier", "scoped_type_identifier"):
        return type_node
    base = _extract_type_basename(type_node)
    if base is None:
        return type_node
    aliased = type_aliases.get(base)
    if aliased is None:
        return type_node
    return resolve_type_alias(aliased, type_aliases, _depth + 1)


def _extract_type_basename(type_node: Node) -> str | None:
    """Given a leaf-ish type CST, return its base (last-segment) name.

    Handles:
      - `type_identifier`         → its text
      - `scoped_type_identifier`  → last segment (name field)
      - `generic_type`            → outer type's last segment (e.g. Vec, Option)

    Returns None for anything else (primitive, function_type, etc.).
    """
    if type_node.type == "type_identifier":
        return type_node.text.decode(errors="replace")
    if type_node.type == "scoped_type_identifier":
        n = type_node.child_by_field_name("name")
        if n is not None:
            return n.text.decode(errors="replace")
        return None
    if type_node.type == "generic_type":
        outer = type_node.child_by_field_name("type")
        if outer is None:
            return None
        return outer.text.rsplit(b"::", 1)[-1].decode(errors="replace")
    return None


def receiver_type_at(
    receiver_node: Node,
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node] | None = None,
    type_aliases: dict[str, Node] | None = None,
    depth: int = 0,
) -> Node | None:
    """§1.4 — resolve the type of a receiver expression by CST walk.

    Depth budget (SPEC §1.4 D17):
      - Each nested `field_expression` costs +1 depth (cap =
        RECEIVER_FIELD_DEPTH_CAP)
      - `unary_expression(op="*", ...)` passes through, no depth cost
      - `parenthesized_expression` passes through, no depth cost

    Returns the resolved type CST node, or None if:
      - depth cap exceeded (logged at DEBUG; caller may treat None as
        "deep receiver" when structurally applicable)
      - identifier not found in local scope / struct fields
      - encountered a `call_expression` / other complex form (SPEC §1.3
        table last two rows — out of scope; caller categorizes None as
        `scrutinee_unresolved` when this comes from §1.3 dispatch)

    Auto-deref: when a `field_expression` receiver resolves to a
    `pointer_type` or `reference_type`, one level of unwrap is applied so
    struct field lookup can proceed. This mirrors Rust's implicit deref
    coercion at field access sites.
    """
    if local_scope is None:
        local_scope = build_local_scope(fn_entry)

    n = receiver_node

    if n.type == "parenthesized_expression":
        for c in n.children:
            if c.is_named:
                return receiver_type_at(c, fn_entry, struct_index, local_scope,
                                        type_aliases, depth)
        return None

    if n.type == "unary_expression":
        is_deref = False
        operand: Node | None = None
        for c in n.children:
            if c.type == "*":
                is_deref = True
            elif c.is_named:
                operand = c
        if not is_deref or operand is None:
            return None
        t = receiver_type_at(operand, fn_entry, struct_index, local_scope,
                             type_aliases, depth)
        if t is None:
            return None
        # Resolve alias BEFORE unwrap so `*(mz_streamp)` sees the underlying
        # `*mut mz_stream_s` (c2rust routinely aliases pointer types).
        t = resolve_type_alias(t, type_aliases)
        return _unwrap_ptr_or_ref(t) or t

    if n.type == "field_expression":
        if depth + 1 > RECEIVER_FIELD_DEPTH_CAP:
            logger.debug(f"[cst] receiver_type_at depth cap ({RECEIVER_FIELD_DEPTH_CAP}) "
                         f"exceeded on {n.text[:40]!r}")
            return None
        receiver = n.child_by_field_name("value")
        field_name_node = n.child_by_field_name("field")
        if receiver is None or field_name_node is None:
            return None
        receiver_type = receiver_type_at(
            receiver, fn_entry, struct_index, local_scope, type_aliases, depth + 1,
        )
        if receiver_type is None:
            return None
        # Alias-then-deref for field access: `mz_streamp` alias → `*mut ...`
        # (deref) → struct type. Resolve alias first, then auto-deref one
        # pointer/reference level for Rust's field-access implicit deref.
        receiver_type = resolve_type_alias(receiver_type, type_aliases)
        receiver_type = _unwrap_ptr_or_ref(receiver_type) or receiver_type
        # Resolve alias AGAIN in case the deref target itself is an alias
        # (rare — chained typedef).
        receiver_type = resolve_type_alias(receiver_type, type_aliases)
        sname = _extract_type_basename(receiver_type)
        if sname is None:
            return None
        fields = struct_index.by_name.get(sname)
        if fields is None:
            return None
        return fields.get(field_name_node.text.decode(errors="replace"))

    if n.type == "identifier":
        return local_scope.get(n.text.decode(errors="replace"))

    if n.type == "index_expression":
        # arr[idx] — receiver is the first named child (array-like);
        # index token is bracketed. Under c2rust codegen this appears as
        # `(*pp).read_filter[(filter - 1) as usize]` where read_filter is
        # `[Option<extern "C" fn(...)>; N]` — the callback jump table.
        receiver = None
        for c in n.children:
            if c.is_named:
                receiver = c
                break
        if receiver is None:
            return None
        t = receiver_type_at(receiver, fn_entry, struct_index, local_scope,
                             type_aliases, depth)
        if t is None:
            return None
        t = resolve_type_alias(t, type_aliases)
        # Auto-deref one level (Rust's index sugar derefs); index into an
        # array-of-T yields T.
        t = _unwrap_ptr_or_ref(t) or t
        t = resolve_type_alias(t, type_aliases)
        if t.type == "array_type":
            return t.child_by_field_name("element")
        return None

    # `call_expression`, `closure_expression`, `if_expression`, and other
    # complex forms are out of scope. SPEC §1.3 table last two rows.
    return None


# ---- Iterators --------------------------------------------------------------

def iter_call_expressions(node: Node) -> Iterator[Node]:
    """Yield every `call_expression` descendant of `node`."""
    if node.type == "call_expression":
        yield node
    for child in node.children:
        yield from iter_call_expressions(child)


def iter_field_expressions(node: Node) -> Iterator[Node]:
    """Yield every `field_expression` descendant of `node`."""
    if node.type == "field_expression":
        yield node
    for child in node.children:
        yield from iter_field_expressions(child)
