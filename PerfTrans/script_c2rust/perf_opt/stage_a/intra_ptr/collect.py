"""Stage A intra_ptr — COLLECT (tree-sitter half of the lift context).

Read-only scan over the c2rust Rust output. For each fn parameter whose
type is a raw pointer, derive — purely from how the BODY uses it — the
fields the design doc (§4 / §4.1) marks as "tree-sitter sourced":

  · count            Array (body uses `p.offset`/`p.add`) vs Scalar
  · body_mutates     does the body write through `p` (`*p.. = `, `+= `, …)
  · use_sites        every place `p` is used, classified
  · length_source    for Array ptrs: the enclosing loop bound, or UNKNOWN
  · cursor_reassigned is `p` itself reassigned (`p = p.offset(..)`)
  · passed_to_calls  callee names `p` is handed to as a raw arg

NO SVF here. The SA-only fields (same_object / caller_arg_id / cross-callee
mutation) are merged later by plan.py. This module alone produces the bulk
of the lift context package and needs no sa_engine output to run — which is
exactly why the safe floor (Kind A + Immutable) can be built tree-sitter
only (design §4.1).

This is a P1 deliverable: it COLLECTS and is meant to be dumped to JSON for
human review. It rewrites nothing.
"""

from __future__ import annotations

import re as _re
from dataclasses import dataclass, field, asdict
from pathlib import Path

from tree_sitter import Language, Node, Parser
import tree_sitter_rust

_LANG = Language(tree_sitter_rust.language())
_PARSER = Parser(_LANG)


# ─────────────────────────────────────────────────────────────────
# Tiny tree-sitter helpers (kept local — mirrors analyzer/e2 style so
# this module is self-contained and unit-testable).
# ─────────────────────────────────────────────────────────────────

def _txt(src: bytes, n: Node) -> str:
    return src[n.start_byte:n.end_byte].decode("utf-8", errors="replace")


def _child(n: Node, kind: str) -> Node | None:
    for c in n.children:
        if c.type == kind:
            return c
    return None


def _strip_cast(expr: str) -> str:
    """`i as isize` / `i1 as usize` → `i` / `i1`. Leaves complex exprs as-is."""
    s = expr.strip()
    low = s
    for tail in (" as isize", " as usize", " as i32", " as u32", " as i64",
                 " as u64", " as c_int", " as c_uint", " as Int32", " as UInt32"):
        if low.endswith(tail):
            return low[: -len(tail)].strip()
    return s


# ─────────────────────────────────────────────────────────────────
# Data model
# ─────────────────────────────────────────────────────────────────

@dataclass
class PtrUse:
    kind: str               # offset_read|offset_write|deref_read|deref_write|
                            # field_read|field_write|is_null|pass_to_call|reassign|other
    snippet: str
    index_expr: str | None = None     # for offset_* uses (post-cast-strip)
    callee: str | None = None         # for pass_to_call


@dataclass
class LengthSource:
    kind: str               # loop_bound | len_param | UNKNOWN
    expr: str | None = None


@dataclass
class PtrParamFacts:
    fn_name: str
    fn_file: str
    anchor: str                      # param name
    current_type: str                # "*mut UChar"
    inner_type: str | None           # "UChar"
    ptr_depth: int                   # 1 = `*mut T`, 2 = `*mut *mut T` (PLAN
                                     # may skip depth>=2 per §2.4.3 double-ptr)
    decl_mut: bool                   # *mut vs *const (declared)
    count: str                       # Array | Scalar
    body_mutates: bool               # body writes through p
    cursor_reassigned: bool
    length_source: LengthSource
    # Function-level facts (same for every param of the fn) — used by the
    # airtight (no-SVF) PLAN gate to prove no aliasing write can happen:
    fn_n_ptr_params: int = 0       # >1 → a sibling could alias (need SVF)
    fn_has_local_ptr: bool = False # `let q: *..` / interior pointer extraction
    fn_makes_call: bool = False    # a free-fn call → callee could mutate
    # Tier 3a: a cast-from-param LOCAL (`let bzf = b as *mut bzFile`). The
    # anchor is the local name; its SA fact is INHERITED from `source_param`
    # (the local reinterprets the same object — Andersen preserves points-to
    # across the cast, so b's ownership / mutability / same_object transfer).
    is_local: bool = False
    source_param: str | None = None
    use_sites: list[PtrUse] = field(default_factory=list)
    passed_to_calls: list[str] = field(default_factory=list)

    def to_json(self) -> dict:
        d = asdict(self)
        return d


# ─────────────────────────────────────────────────────────────────
# Param scan — pointer params only
# ─────────────────────────────────────────────────────────────────

@dataclass
class _PtrParam:
    name: str
    type_text: str
    inner: str | None
    decl_mut: bool


def _scan_ptr_params(src: bytes, fn_node: Node) -> list[_PtrParam]:
    params_node = _child(fn_node, "parameters")
    if params_node is None:
        return []
    out: list[_PtrParam] = []
    for child in params_node.named_children:
        if child.type != "parameter":
            continue
        ident = next((c for c in child.named_children
                      if c.type == "identifier"), None)
        if ident is None:
            continue
        name = _txt(src, ident)
        # type slot = first named child that is neither the mut binding
        # modifier nor the param identifier itself
        type_node = None
        saw_ident = False
        for c in child.named_children:
            if c.type == "mutable_specifier":
                continue
            if c.type == "identifier" and not saw_ident:
                saw_ident = True
                continue
            type_node = c
            break
        if type_node is None or type_node.type != "pointer_type":
            continue
        decl_mut = any(c.type == "mutable_specifier" for c in type_node.children)
        inner = (type_node.named_children[-1]
                 if type_node.named_children else None)
        out.append(_PtrParam(
            name=name, type_text=_txt(src, type_node),
            inner=_txt(src, inner) if inner is not None else None,
            decl_mut=decl_mut,
        ))
    return out


# ─────────────────────────────────────────────────────────────────
# Use-site classification + write-context
# ─────────────────────────────────────────────────────────────────

def _is_write_context(node: Node) -> bool:
    """Is `node` (a deref / field / index place expr) the target of a write?

    Climbs the place-expression chain — `field_expression`,
    `index_expression`, `parenthesized_expression` — while `node` stays on
    the VALUE (base) side, until it reaches an assignment LHS or a `&mut`.
    This catches nested lvalues like `(*p).field[i] = x` / `(*p).a.b = x`
    that a one-level parent check misclassifies as reads (and would then
    wrongly mark the pointer Immutable). `cargo check` would still reject a
    bad `&T` lift, but mis-detection pollutes the plan, so we get it right."""
    cur = node
    p = cur.parent
    while p is not None:
        if p.type in ("assignment_expression", "compound_assignment_expr"):
            named = [c for c in p.children if c.is_named]
            return bool(named) and named[0].start_byte == cur.start_byte
        if p.type == "reference_expression":
            return any(c.type == "mutable_specifier" for c in p.children)
        if p.type in ("field_expression", "index_expression"):
            named = [c for c in p.children if c.is_named]
            if named and named[0].start_byte == cur.start_byte:
                cur, p = p, p.parent          # node is the base → keep climbing
                continue
            return False                       # node is the index/field → read
        if p.type == "parenthesized_expression":
            cur, p = p, p.parent
            continue
        return False
    return False


def _enclosing_loop_bound(idn: Node, index_var: str) -> str | None:
    """Walk up from an offset-use to the nearest loop whose counter is
    `index_var`, return its upper bound expr. Recognizes:
        while <index_var> < N        → N
        while <index_var> <= N       → N
        for  <index_var> in 0..N     → N
        for  <index_var> in 0..=N    → N
    Returns None if no such loop / non-trivial counter."""
    cur = idn.parent
    while cur is not None:
        if cur.type == "while_expression":
            cond = cur.child_by_field_name("condition")
            bound = _bound_from_lt_condition(cond, index_var)
            if bound is not None:
                return bound
        elif cur.type == "for_expression":
            bound = _bound_from_for(cur, index_var)
            if bound is not None:
                return bound
        cur = cur.parent
    return None


def _bound_from_lt_condition(cond: Node | None, index_var: str) -> str | None:
    """`<index_var> < N` / `<= N` inside a (possibly wrapped) condition."""
    if cond is None:
        return None
    # condition may be wrapped (e.g. `(i < n) as c_int != 0`); search for a
    # binary_expression `<idx> < <bound>` anywhere inside.
    stack = [cond]
    while stack:
        n = stack.pop()
        if n.type == "binary_expression":
            named = [c for c in n.children if c.is_named]
            ops = [c for c in n.children if not c.is_named]
            op = next((_op_text(c) for c in ops if _op_text(c) in ("<", "<=")), None)
            if op and len(named) == 2:
                lhs = _norm(named[0])
                if lhs == index_var:
                    return _node_src(named[1])
        stack.extend(n.children)
    return None


def _bound_from_for(for_node: Node, index_var: str) -> str | None:
    """`for <index_var> in 0..N` / `0..=N`."""
    pat = for_node.child_by_field_name("pattern")
    val = for_node.child_by_field_name("value")
    if pat is None or val is None:
        return None
    if _node_src(pat).strip() != index_var:
        return None
    # value is a range_expression `0..N`
    if val.type == "range_expression":
        named = [c for c in val.children if c.is_named]
        if len(named) == 2 and _node_src(named[0]).strip() in ("0", "0 as usize"):
            return _node_src(named[1])
    return None


# These three are filled per-call (need src); set by _classify.
_SRC: bytes = b""


def _node_src(n: Node) -> str:
    return _txt(_SRC, n)


def _norm(n: Node) -> str:
    return _strip_cast(_txt(_SRC, n))


def _op_text(n: Node) -> str:
    return _txt(_SRC, n)


# ─────────────────────────────────────────────────────────────────
# Tier 3a — cast-from-param LOCAL detection (`let bzf = b as *mut bzFile`)
# ─────────────────────────────────────────────────────────────────

@dataclass
class _LocalCast:
    name: str
    source_param: str
    inner: str | None
    type_text: str       # the cast target `*mut bzFile`
    is_mut: bool


def _scan_cast_from_param_locals(src: bytes, fn_node: Node,
                                 param_names: set[str]) -> list[_LocalCast]:
    """`let <local> = <param> as *mut/const T` — a local that reinterprets a
    pointer param. Returns one entry per local (first decl wins)."""
    body = _child(fn_node, "block")
    if body is None:
        return []
    seen: set[str] = set()
    out: list[_LocalCast] = []
    stack = [body]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type != "let_declaration":
            continue
        pat = n.child_by_field_name("pattern")
        name = None
        if pat is not None and pat.type == "identifier":
            name = _txt(src, pat)
        elif pat is not None and pat.type == "mut_pattern":
            idn = next((c for c in pat.children if c.type == "identifier"), None)
            name = _txt(src, idn) if idn else None
        if not name or name in seen:
            continue
        val = n.child_by_field_name("value")
        if val is None or val.type != "type_cast_expression":
            continue
        named = [c for c in val.children if c.is_named]
        if len(named) < 2:
            continue
        base, cast_ty = named[0], named[-1]
        if cast_ty.type != "pointer_type":
            continue
        if base.type != "identifier" or _txt(src, base) not in param_names:
            continue
        inner = cast_ty.named_children[-1] if cast_ty.named_children else None
        seen.add(name)
        out.append(_LocalCast(
            name=name, source_param=_txt(src, base),
            inner=_txt(src, inner) if inner is not None else None,
            type_text=_txt(src, cast_ty),
            is_mut=any(c.type == "mutable_specifier" for c in cast_ty.children),
        ))
    return out


# ─────────────────────────────────────────────────────────────────
# Per-fn collection
# ─────────────────────────────────────────────────────────────────

def collect_fn(src: bytes, fn_node: Node, fn_file: str) -> list[PtrParamFacts]:
    global _SRC
    _SRC = src
    name_node = _child(fn_node, "identifier")
    if name_node is None:
        return []
    fn_name = _txt(src, name_node)
    body = _child(fn_node, "block")
    if body is None:
        return []
    ptr_params = _scan_ptr_params(src, fn_node)
    local_casts = _scan_cast_from_param_locals(
        src, fn_node, {p.name for p in ptr_params})
    if not ptr_params and not local_casts:
        return []
    # candidate anchors = pointer params + cast-from-param locals (Tier 3a)
    all_names = [p.name for p in ptr_params] + [lc.name for lc in local_casts]
    by_name = set(all_names)

    # Gather all identifier uses inside the body, bucketed by anchor name.
    uses_by_param: dict[str, list[PtrUse]] = {n: [] for n in all_names}
    reassigned: set[str] = set()
    index_vars_by_param: dict[str, list[tuple[Node, str]]] = {
        n: [] for n in all_names
    }
    calls_by_param: dict[str, list[str]] = {n: [] for n in all_names}

    fn_has_local_ptr = False
    fn_makes_call = False
    stack = [body]
    while stack:
        n = stack.pop()
        if n.type == "identifier":
            nm = _txt(src, n)
            if nm in by_name:
                use = _classify_use(src, n, nm, index_vars_by_param,
                                    calls_by_param)
                if use is not None:
                    if use.kind == "reassign":
                        reassigned.add(nm)
                    uses_by_param[nm].append(use)
        elif n.type == "let_declaration":
            ty = n.child_by_field_name("type")
            if ty is not None and ty.type == "pointer_type":
                fn_has_local_ptr = True
        elif n.type == "call_expression":
            # A *free-function* call (`foo(...)`) — not a method like
            # `p.offset(i)` / `p.is_null()` (those are field_expression fns).
            fnf = n.child_by_field_name("function")
            if fnf is not None and fnf.type in ("identifier", "scoped_identifier"):
                fn_makes_call = True
        stack.extend(n.children)

    out: list[PtrParamFacts] = []
    for pp in ptr_params:
        uses = uses_by_param[pp.name]
        is_array = any(u.kind.startswith("offset_") for u in uses)
        mutates = any(u.kind.endswith("_write") for u in uses)
        length = _resolve_length(pp, index_vars_by_param[pp.name],
                                 is_array, by_name)
        out.append(PtrParamFacts(
            fn_name=fn_name, fn_file=fn_file,
            anchor=pp.name, current_type=pp.type_text, inner_type=pp.inner,
            ptr_depth=pp.type_text.count("*"),
            decl_mut=pp.decl_mut,
            count="Array" if is_array else "Scalar",
            body_mutates=mutates,
            cursor_reassigned=pp.name in reassigned,
            length_source=length,
            fn_n_ptr_params=len(ptr_params),
            fn_has_local_ptr=fn_has_local_ptr,
            fn_makes_call=fn_makes_call,
            use_sites=uses,
            passed_to_calls=sorted(set(calls_by_param[pp.name])),
        ))
    # Tier 3a: cast-from-param locals (anchor = local; SA fact inherited from
    # source_param in plan.py).
    for lc in local_casts:
        uses = uses_by_param[lc.name]
        is_array = any(u.kind.startswith("offset_") for u in uses)
        mutates = any(u.kind.endswith("_write") for u in uses)
        length = _resolve_length(None, index_vars_by_param[lc.name], is_array, by_name)
        out.append(PtrParamFacts(
            fn_name=fn_name, fn_file=fn_file,
            anchor=lc.name, current_type=lc.type_text, inner_type=lc.inner,
            ptr_depth=lc.type_text.count("*"), decl_mut=lc.is_mut,
            count="Array" if is_array else "Scalar",
            body_mutates=mutates,
            cursor_reassigned=lc.name in reassigned,
            length_source=length,
            fn_n_ptr_params=len(ptr_params),
            fn_has_local_ptr=fn_has_local_ptr,
            fn_makes_call=fn_makes_call,
            is_local=True, source_param=lc.source_param,
            use_sites=uses,
            passed_to_calls=sorted(set(calls_by_param[lc.name])),
        ))
    return out


def _classify_use(src: bytes, idn: Node, pname: str,
                  index_vars_by_param: dict[str, list],
                  calls_by_param: dict[str, list]) -> PtrUse | None:
    """Classify one occurrence of identifier `pname`."""
    p = idn.parent
    if p is None:
        return None

    # Param declaration itself (inside `parameters`) — skip.
    anc = p
    while anc is not None:
        if anc.type == "parameters":
            return None
        if anc.type == "block":
            break
        anc = anc.parent

    # Binding site of a `let <name> = …` (a cast-from-param local's own decl) —
    # skip so it isn't counted as an "other" (escape) use.
    if p.type == "mut_pattern":
        return None
    if p.type == "let_declaration":
        patn = p.child_by_field_name("pattern")
        if patn is not None and patn.start_byte == idn.start_byte:
            return None

    # `p = ...` (cursor reassignment): idn is LHS identifier of assignment.
    if p.type in ("assignment_expression", "compound_assignment_expr"):
        named = [c for c in p.children if c.is_named]
        if named and named[0].start_byte == idn.start_byte:
            return PtrUse(kind="reassign", snippet=_txt(src, p)[:80])

    # `p.offset(i)` / `p.add(i)` / `p.is_null()` — idn is value of a
    # field_expression whose field is the method.
    if p.type == "field_expression":
        fld = p.child_by_field_name("field")
        val = p.child_by_field_name("value")
        if val is not None and val.start_byte == idn.start_byte and fld is not None:
            fname = _txt(src, fld)
            call = p.parent  # call_expression `p.offset(i)`
            if fname in ("offset", "add") and call is not None and \
                    call.type == "call_expression":
                args = call.child_by_field_name("arguments")
                idx_raw = _inner_args_text(src, args)
                idx = _strip_cast(idx_raw)
                # is this offset deref'd & written? walk up to unary `*`
                deref = _enclosing_deref(call)
                write = deref is not None and _is_write_context(deref)
                index_vars_by_param[pname].append((idn, idx))
                return PtrUse(
                    kind="offset_write" if write else "offset_read",
                    snippet=_txt(src, deref if deref is not None else call)[:80],
                    index_expr=idx,
                )
            if fname == "is_null":
                return PtrUse(kind="is_null", snippet=_txt(src, call or p)[:80])
            # `(*p).field` handled via the deref branch below; a bare
            # `p.<field>` on a raw ptr is unusual → other.
            return PtrUse(kind="other", snippet=_txt(src, p)[:80])

    # `*p` — idn is operand of a unary `*`.
    if p.type == "unary_expression" and _txt(src, p).lstrip().startswith("*"):
        write = _is_write_context(p)
        # distinguish `(*p).field` (field access) from bare `*p`
        gp = p.parent
        is_field = False
        if gp is not None and gp.type == "parenthesized_expression":
            ggp = gp.parent
            if ggp is not None and ggp.type == "field_expression":
                is_field = True
                fw = _is_write_context(ggp)
                return PtrUse(
                    kind="field_write" if fw else "field_read",
                    snippet=_txt(src, ggp)[:80],
                )
        if not is_field:
            return PtrUse(
                kind="deref_write" if write else "deref_read",
                snippet=_txt(src, p)[:80],
            )

    # passed as a call argument: idn directly under `arguments`
    if p.type == "arguments":
        call = p.parent
        callee = None
        if call is not None and call.type == "call_expression":
            fn = call.child_by_field_name("function")
            if fn is not None:
                callee = _txt(src, fn)
        if callee:
            calls_by_param[pname].append(callee)
        return PtrUse(kind="pass_to_call", snippet=_txt(src, idn), callee=callee)

    return PtrUse(kind="other", snippet=_txt(src, p)[:80])


def _enclosing_deref(call_node: Node) -> Node | None:
    """For a `p.offset(i)` call, return the enclosing `*<...>` unary node if
    the offset result is immediately dereferenced (`*p.offset(i)`)."""
    par = call_node.parent
    if par is not None and par.type == "unary_expression" \
            and _txt(_SRC, par).lstrip().startswith("*"):
        return par
    return None


def _inner_args_text(src: bytes, args: Node | None) -> str:
    if args is None:
        return ""
    named = [c for c in args.children if c.is_named]
    if not named:
        return ""
    return _txt(src, named[0])


_CLEAN_LEN_RE = _re.compile(
    r"^\s*[A-Za-z_]\w*(::\w+)*\s*$"            # ident / scoped const  (alphaSize)
    r"|^\s*\d+(\s+as\s+\w+(::\w+)*)?\s*$"      # int literal (maybe `as ty`)
    r"|^\s*[A-Za-z_]\w*(\s+as\s+\w+)?\s*[-+]\s*\d+(\s+as\s+\w+)?\s*$"  # x ± const
)


def _is_clean_length(b: str) -> bool:
    """A bound is usable as a slice length only if it is a simple, stable
    quantity: an identifier / scoped const, an int literal, or `x ± const`.
    Reject anything with `[` (array element, e.g. `copyStart[ss]`), `.`
    (field/method), or `(` (call) — those are per-iteration values or
    derived expressions, NOT a length. This is the gate that kills the
    false positives P1's dump surfaced (2026-06-25)."""
    if any(ch in b for ch in "[].("):
        return False
    return _CLEAN_LEN_RE.match(b) is not None


def _resolve_length(pp: _PtrParam, index_uses: list[tuple[Node, str]],
                    is_array: bool, by_name: dict) -> LengthSource:
    """Conservative length inference. Length sources are loop bounds only
    (struct-field / call lengths intentionally excluded — user decision
    2026-06-25). A length is accepted ONLY IF it is airtight:

      (1) EVERY offset use of the pointer resolves to a loop bound — a
          single unbounded use (e.g. mainGtU's `i1 = i1.wrapping_add(1)`
          probe) makes the whole pointer UNKNOWN, because that use could
          exceed any candidate length;
      (2) all uses agree on the SAME bound N;
      (3) N is a clean length (`_is_clean_length`).

    Otherwise UNKNOWN. Better to defer than to slice to a wrong length and
    risk OOB — correctness is the bottom line."""
    if not is_array:
        return LengthSource(kind="n/a", expr=None)  # scalar: no length needed
    if not index_uses:
        return LengthSource(kind="UNKNOWN", expr=None)
    bounds: set[str] = set()
    for idn, idx in index_uses:
        var = _strip_cast(idx)
        if not var.isidentifier():
            return LengthSource(kind="UNKNOWN", expr=None)   # non-counter index
        b = _enclosing_loop_bound(idn, var)
        if b is None:
            return LengthSource(kind="UNKNOWN", expr=None)   # unbounded use
        b = b.strip()
        if not _is_clean_length(b):
            return LengthSource(kind="UNKNOWN", expr=None)   # e.g. copyStart[ss]
        bounds.add(b)
    if len(bounds) == 1:
        return LengthSource(kind="loop_bound", expr=next(iter(bounds)))
    return LengthSource(kind="UNKNOWN", expr=None)            # conflicting bounds


# ─────────────────────────────────────────────────────────────────
# Crate-level entry
# ─────────────────────────────────────────────────────────────────

def collect_crate(crate_dir: Path,
                  only_fns: set[str] | None = None) -> list[PtrParamFacts]:
    """Scan every .rs file (excluding target/) and collect pointer-param
    facts for every fn (or only `only_fns`)."""
    out: list[PtrParamFacts] = []
    for f in sorted(crate_dir.rglob("*.rs")):
        if "target" in f.parts:
            continue
        src = f.read_bytes()
        tree = _PARSER.parse(src)
        rel = str(f.relative_to(crate_dir))
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            if n.type == "function_item":
                nm = _child(n, "identifier")
                if nm is not None:
                    fn_name = _txt(src, nm)
                    if only_fns is None or fn_name in only_fns:
                        out.extend(collect_fn(src, n, rel))
            stack.extend(n.children)
    return out
