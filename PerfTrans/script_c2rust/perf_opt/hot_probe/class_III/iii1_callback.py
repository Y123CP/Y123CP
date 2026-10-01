""                                                     

                                       

                                                                    
                                                                       
                                                                    
                           

           
                                                       

                                                                          
                                            
   

from __future__ import annotations

import logging
from dataclasses import dataclass, field, replace

from tree_sitter import Node

from perf_opt.hot_probe.class_III.config import (
    CALLBACK_MARKER_STRING,
    HIGHER_ORDER_LIBC_FNS,
)
from perf_opt.hot_probe.class_III.cst_utils import (
    CallSite,
    FnCstEntry,
    StructIndex,
    _extract_type_basename,
    _iter_nodes_by_type,
    _unwrap_ptr_or_ref,
    build_local_scope,
    is_fn_ptr,
    iter_call_expressions,
    receiver_type_at,
    resolve_type_alias,
)

logger = logging.getLogger(__name__)

# Forms whose `callee_name` is a struct field, not a name the function scope
# binds. Looking such a name up in the scope answers a different question.
_FIELD_NAMED_FORMS = frozenset({"B_field", "C"})


@dataclass
class RuleIIIOneHits:
    """Per-fn III① detection outcome (§7.2 iii1_* fields)."""
    hits: list[CallSite] = field(default_factory=list)
    deep_receiver: list[CallSite] = field(default_factory=list)
    scrutinee_unresolved: list[CallSite] = field(default_factory=list)
    c1_boundary_unresolved: list[CallSite] = field(default_factory=list)


# ---- Top-level detect -------------------------------------------------------

def detect(
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    type_aliases: dict[str, Node] | None = None,
    static_items: dict[str, Node] | None = None,
) -> RuleIIIOneHits:
    """Run all four form detectors on `fn_entry`, return aggregated hits.

    Dispatch order (all four run independently; a call_expression may only
    ever hit one form because CST shape is unique):
      A — bare identifier callee whose binding is fn-ptr-typed
      B — `x.expect|unwrap(...)(args)` (or in a `let` — the §1.6 use-def
          case B branch handles this)
      C — `receiver.field(args)` where field is fn-ptr-typed
      D — match arm body with pattern-bound identifier callee

    Boundary with C1 (§1.6): a call_expression whose `function` sub-tree
    is `x.expect(msg)` needs the use-def check to decide whether the site
    is III① (unwrap+call) or C1 (unwrap only). Handled inside _detect_form_b_
    for the immediate-call case, and via _walk_use_def_ for the let-bound
    case.
    """
    result = RuleIIIOneHits()
    if type_aliases is None:
        type_aliases = {}
    origins: dict[str, str] = {}
    local_scope = build_local_scope(fn_entry, static_items, origins=origins)
    # Phase 1 — form A/B/C via call iteration.
    for call in iter_call_expressions(fn_entry.node):
        _classify_call(call, fn_entry, struct_index, local_scope, type_aliases, result)
    # Phase 2 — form D via match_expression walk (§1.2 form D + §1.3 mini scope).
    _scan_form_d(fn_entry, struct_index, local_scope, type_aliases, result)
    # Phase 3 — §1.6 case B via let_declaration walk (shallow use-def).
    _scan_let_use_def(fn_entry, struct_index, local_scope, type_aliases, result)
    # Phase 4 — form E: fn-ptr handed to a higher-order libc function.
    _scan_form_e(fn_entry, result)
    _annotate_bindings(result, origins)
    return result


def _annotate_bindings(
    result: RuleIIIOneHits, origins: dict[str, str]
) -> None:
    """Stamp each site with where its callee name was bound.

    Done in one pass here rather than threaded through the nine
    `_build_call_site` call sites: the scope that answers the question is
    already built in `detect`, and the answer is a property of the NAME, not
    of the form that found it.

    Only forms whose `callee_name` IS a scope name are stamped. B_field and C
    record a struct field (`settings.on_message_complete` records
    `on_message_complete`), and a field that happens to share a name with some
    module-level static would otherwise be labelled `static` and lose its
    rule. Placeholder names (`<deep receiver>`, `<indirect Option<fn ptr>>`)
    are excluded for the same reason: they are labels, not bindings.
    """
    if not origins:
        return
    for bucket_name in ("hits", "deep_receiver", "scrutinee_unresolved",
                        "c1_boundary_unresolved"):
        bucket = getattr(result, bucket_name)
        for index, site in enumerate(bucket):
            if site.form in _FIELD_NAMED_FORMS or site.callee_name.startswith("<"):
                continue
            origin = origins.get(site.callee_name)
            if origin and not site.binding:
                bucket[index] = replace(site, binding=origin)


def _scan_form_e(fn_entry: FnCstEntry, result: RuleIIIOneHits) -> None:
    """Form E — the callback is not called here, it is HANDED OVER.

    A/B/C/D all look for an indirect call made *inside* this function. But
    `qsort(base, n, size, Some(cmp))` makes none: the indirect call happens
    inside libc, once per comparison, and the comparator can never inline.
    The CST shape is entirely different, so those four forms miss it.

    This is not one project's idiom — it is how c2rust renders every C call
    to `qsort` and friends. Measured on one crate, replacing a single such
    call with the native Rust sort (same ordering key):

        tree_codegen              1.4439s -> 1.0003s   -30.72%
        lz77_blocksplit_analysis  1.3531s -> 1.2513s   - 7.52%

    Its profile showed `msort_with_tmp` plus the comparator at 30.5% of that
    operation's self time — all of it C ABI dispatch that Rust inlines away.
    """
    for call in iter_call_expressions(fn_entry.node):
        callee = call.child_by_field_name("function")
        if callee is None or callee.type != "identifier":
            continue
        name = callee.text.decode("utf-8", "replace")
        if name not in HIGHER_ORDER_LIBC_FNS:
            continue
        args = call.child_by_field_name("arguments")
        if args is None:
            continue
        text = args.text.decode("utf-8", "replace")
        # A function pointer among the arguments is what makes this a
        # dispatch site rather than an ordinary libc call.
        if "extern \"C\" fn" not in text and " as unsafe" not in text:
            continue
        result.hits.append(CallSite(
            file=str(fn_entry.file),
            line=call.start_point[0] + 1,
            col=call.start_point[1] + 1,
            callee_name=name,
            form="E",
            marker_seen=CALLBACK_MARKER_STRING in text,
            callback_fn=_handed_over_fn_name(args),
        ))
    return


def _handed_over_fn_name(args: Node) -> str:
    """Name of the function whose pointer is passed to the higher-order call.

    Shape, not text: a `type_cast_expression` whose value is a bare identifier
    and whose type is a `function_type`. That is exactly what c2rust emits for
    a C function name decaying to a pointer —
    `Cmp as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int` —
    regardless of how the argument is wrapped (`Some(..)`, a `transmute`, or
    nothing at all), because the cast is what performs the decay.

    Returns "" when no such cast exists (a pointer already held in a variable,
    or a genuinely dynamic one) — the caller must treat that as "unknown", not
    as "no callback": the site is still a Form E hit, it just cannot have its
    callee source fetched.
    """
    found = ""
    for cast in _iter_nodes_by_type(args, "type_cast_expression"):
        target = cast.child_by_field_name("type")
        value = cast.child_by_field_name("value")
        if target is None or value is None:
            continue
        if target.type != "function_type" or value.type != "identifier":
            continue
        name = value.text.decode("utf-8", "replace")
        if found and found != name:
            # Two different function pointers in one call (`tsearch`-style
            # compare + free). Naming one of them would send the model a
            # body that does not belong to the rewrite it is asked for.
            return ""
        found = name
    return found


def _classify_call(
    call: Node,
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
    result: RuleIIIOneHits,
) -> None:
    """Dispatch a single `call_expression` to its A/B/C form.

    Form D (match-arm pattern-bound identifier callee) and the §1.6
    boundary `let f = x.expect(...); f(a)` case are Step-5 additions —
    they need the mini scope resolver (§1.3 D16) and the shallow
    use-def chain (§1.6 D9) respectively.
    """
    function_node = call.child_by_field_name("function")
    if function_node is None:
        return

    if function_node.type == "identifier":
        site = _detect_form_a(call, function_node, fn_entry, local_scope, type_aliases)
        if site is not None:
            result.hits.append(site)
            return
        # else: potentially form D (Step 5) or a plain fn call — not our concern

    elif function_node.type in ("call_expression", "parenthesized_expression"):
        site = _detect_form_b(call, function_node, fn_entry, struct_index,
                              local_scope, type_aliases, result)
        if site is not None:
            result.hits.append(site)
            return

    elif function_node.type == "field_expression":
        site = _detect_form_c(call, function_node, fn_entry, struct_index,
                              local_scope, type_aliases, result)
        if site is not None:
            result.hits.append(site)
            return

    # Any other function-sub-tree shape (closures, macros, etc.) — out of scope.


# ---- Form A: bare identifier callee, binding is fn ptr ----------------------

def _detect_form_a(
    call: Node,
    function_node: Node,
    fn_entry: FnCstEntry,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
) -> CallSite | None:
    """§1.2 form A: `call(function: identifier {ID_R}, args)`.

    ID_R is fn-ptr if it resolves via local_scope (§1.1 (i)/(ii)):
      - fn parameter with fn-ptr / Option<fn ptr> type, OR
      - `let ID_R: fn(...) = ...;` with explicit fn-ptr type annotation

    Match-arm pattern-bound identifiers (`Some(f) => f(...)`) are form D,
    handled in Step 5 via the mini scope resolver.

    Returns a CallSite hit, or None if not form A.
    """
    id_name = function_node.text.decode(errors="replace")
    binding_type = local_scope.get(id_name)
    if binding_type is None:
        return None
    if not is_fn_ptr(binding_type, type_aliases):
        return None
    return _build_call_site(call, id_name, fn_entry, marker_seen=False, form="A")


# ---- Form B: unwrap-then-call (`cb.expect(msg)(args)`) ----------------------

def _detect_form_b(
    call: Node,
    function_node: Node,
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
    result: RuleIIIOneHits,
) -> CallSite | None:
    """§1.2 form B: unwrap-then-call over an `Option<fn ptr>`.

    Two CST shapes are equivalent (parenthesized vs bare chained call):
      `(cb.expect(m))(a)`   → function is parenthesized_expression → call_expression
      `cb.expect(m)(a)`     → function is call_expression directly

    Inner call must be a method call on `expect` / `unwrap` /
    `unwrap_unchecked`, and its receiver's type must resolve to
    Option<fn ptr> via receiver_type_at (§1.4).

    §1.5: `marker_seen` records whether the inner call's first arg is a
    string literal matching CALLBACK_MARKER_STRING — observation only,
    NEVER a matching predicate.
    """
    inner = function_node
    if inner.type == "parenthesized_expression":
        inner = _first_named_child(inner)
        if inner is None:
            return None
    if inner.type != "call_expression":
        return None

    inner_fn = inner.child_by_field_name("function")
    if inner_fn is None or inner_fn.type != "field_expression":
        return None
    method_name_node = inner_fn.child_by_field_name("field")
    if method_name_node is None:
        return None
    method = method_name_node.text.decode(errors="replace")
    if method not in ("expect", "unwrap", "unwrap_unchecked"):
        return None

    receiver = inner_fn.child_by_field_name("value")
    if receiver is None:
        return None

    # Fold direct receiver_type_at + Some/Ok constructor unwrap (B2)
    receiver_type = _form_b_receiver_is_option_fn_ptr(
        receiver, fn_entry, struct_index, local_scope, type_aliases,
    )
    if receiver_type is None:
        # If the miss looks like a depth-cap issue on a field chain,
        # bucket it (B1). Otherwise silent.
        if _field_expression_depth(receiver) > _RECEIVER_FIELD_CAP:
            result.deep_receiver.append(
                _build_call_site(call, "<deep receiver>", fn_entry, marker_seen=False)
            )
        return None

    marker_seen = _check_marker_in_call_args(inner)
                                                                   
                                                               
                                                                     
                                                             
                            
    if receiver.type == "field_expression":
        field_ident = receiver.child_by_field_name("field")
        cn = (field_ident.text.decode(errors="replace")
              if field_ident is not None else "<indirect Option<fn ptr>>")
        form_tag = "B_field"
    elif receiver.type == "identifier":
        cn = receiver.text.decode(errors="replace")
        form_tag = "B_param"
    else:
        cn = "<indirect Option<fn ptr>>"
        form_tag = "B"
    return _build_call_site(call, cn, fn_entry, marker_seen=marker_seen,
                            form=form_tag)


# ---- Form C: field-expression callee, field type is fn ptr ------------------

def _detect_form_c(
    call: Node,
    function_node: Node,
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
    result: RuleIIIOneHits,
) -> CallSite | None:
    """§1.2 form C: `(*p).cb(x)` — direct field-callback call.

    Field itself is fn-ptr-typed (NOT Option<fn ptr> — that requires
    unwrap, hence form B).
    """
    receiver = function_node.child_by_field_name("value")
    field_name_node = function_node.child_by_field_name("field")
    if receiver is None or field_name_node is None:
        return None

    receiver_type = receiver_type_at(receiver, fn_entry, struct_index, local_scope, type_aliases)
    if receiver_type is None:
        # B1: depth-cap → deep_receiver bucket. Note: form C's receiver
        # is `function_node.child_by_field_name("value")` — one field
        # access above sits in `function_node` itself (the field call).
        # We measure receiver depth here (not including function_node's
        # own field access), matching SPEC §1.4's counting on chains
        # like `(*p).a.b.c.cb(x)`.
        if _field_expression_depth(receiver) > _RECEIVER_FIELD_CAP:
            result.deep_receiver.append(
                _build_call_site(call, "<deep receiver>", fn_entry, marker_seen=False)
            )
        return None
    receiver_type = _unwrap_ptr_or_ref(receiver_type) or receiver_type
    sname = _extract_type_basename(receiver_type)
    if sname is None:
        return None
    fields = struct_index.by_name.get(sname)
    if fields is None:
        return None
    field_name = field_name_node.text.decode(errors="replace")
    field_type = fields.get(field_name)
    if field_type is None:
        return None

    # Must be raw fn ptr — Option<fn ptr> field goes through form B (unwrap).
    # Resolve type aliases first: `type mz_alloc_func = Option<fn ptr>` looks
    # like a bare type_identifier on the field, but is really Option-wrapped;
    # after resolution we can compare against `generic_type` cleanly.
    resolved = resolve_type_alias(field_type, type_aliases)
    if resolved.type == "generic_type":
        return None  # Option<fn ptr> alias — belongs to form B
    if not is_fn_ptr(field_type, type_aliases):
        return None

    return _build_call_site(call, field_name, fn_entry, marker_seen=False, form="C")


# ---- Shared helpers ---------------------------------------------------------

def _first_named_child(node: Node) -> Node | None:
    for c in node.children:
        if c.is_named:
            return c
    return None


def _check_marker_in_call_args(inner_call: Node) -> bool:
    """§1.5 — check whether inner_call's first argument is a string literal
    equal to CALLBACK_MARKER_STRING. Observation only, no matching effect.
    """
    args = inner_call.child_by_field_name("arguments")
    if args is None:
        return False
    first_arg = None
    for c in args.children:
        if c.is_named:
            first_arg = c
            break
    if first_arg is None or first_arg.type != "string_literal":
        return False
    # Extract raw string content from string_literal → string_content child.
    for c in first_arg.children:
        if c.type == "string_content":
            return c.text.decode(errors="replace") == CALLBACK_MARKER_STRING
    return False


def _build_call_site(
    call: Node,
    callee_name: str,
    fn_entry: FnCstEntry,
    marker_seen: bool,
    form: str = "",
) -> CallSite:
    row, col = call.start_point
    return CallSite(
        file=str(fn_entry.file),
        line=row + 1,
        col=col + 1,
        callee_name=callee_name,
        marker_seen=marker_seen,
        form=form,
    )


# ============================================================================
# Form D — match-arm pattern-bound identifier callee (§1.2 form D)
# Implements §1.3 mini scope resolver over each match_expression's arms.
# ============================================================================

def _scan_form_d(
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
    result: RuleIIIOneHits,
) -> None:
    """§1.2 form D + §1.3 D16 mini scope resolver.

    For each `match_expression`:
      1. Enumerate arms; for each arm collect `Some(name)` pattern bindings
         (SPEC §1.3 step a).
      2. If the arm body contains a `call_expression(function=identifier{f})`
         where `f` is one of the Some-bound names → resolve scrutinee type
         (SPEC §1.3 step b table).
      3. If scrutinee resolves to `Option<fn ptr>` (raw or via typedef) →
         form D hit.
      4. Bucket routing per §1.3 D16:
         - scrutinee is `call_expression` → no hit, no audit (SPEC row 4)
         - scrutinee is other complex form → `scrutinee_unresolved` bucket
    """
    for match_expr in _iter_nodes_by_type(fn_entry.node, "match_expression"):
        scrutinee = match_expr.child_by_field_name("value")
        body = match_expr.child_by_field_name("body")
        if scrutinee is None or body is None:
            continue

        for arm in body.children:
            if arm.type != "match_arm":
                continue
            pattern = arm.child_by_field_name("pattern")
            arm_body = arm.child_by_field_name("value")
            if pattern is None or arm_body is None:
                continue

            some_bound = _collect_some_bindings(pattern)
            if not some_bound:
                continue

            for call in iter_call_expressions(arm_body):
                call_fn = call.child_by_field_name("function")
                if call_fn is None or call_fn.type != "identifier":
                    continue
                id_name = call_fn.text.decode(errors="replace")
                if id_name not in some_bound:
                    continue
                # A Some-bound identifier IS being called. Resolve scrutinee.
                s_type = receiver_type_at(scrutinee, fn_entry, struct_index, local_scope, type_aliases)
                if s_type is None:
                    # SPEC §1.3 row 4: call_expression scrutinee → silent
                    # SPEC §1.3 row 5: other complex → scrutinee_unresolved
                    if scrutinee.type != "call_expression":
                        result.scrutinee_unresolved.append(
                            _build_call_site(call, id_name, fn_entry, marker_seen=False)
                        )
                    continue
                if not is_fn_ptr(s_type, type_aliases):
                    continue
                resolved = resolve_type_alias(s_type, type_aliases)
                # Some(f) requires the scrutinee to actually be Option<...>.
                if resolved.type != "generic_type":
                    continue
                result.hits.append(
                    _build_call_site(call, id_name, fn_entry, marker_seen=False, form="D")
                )


def _collect_some_bindings(match_pattern: Node) -> set[str]:
    """§1.3 step a — from a `match_pattern` subtree, collect the identifiers
    declared by `Some(name)` tuple-struct patterns.

    Returns the set of bound identifier names (empty for None/other arms).
    """
    out: set[str] = set()
    for pat_node in match_pattern.children:
        if pat_node.type != "tuple_struct_pattern":
            continue
        variant = pat_node.child_by_field_name("type")
        if variant is None or variant.text != b"Some":
            continue
        for c in pat_node.children:
            if c.type == "identifier":
                out.add(c.text.decode(errors="replace"))
    return out


# ============================================================================
# §1.6 case B — shallow use-def III①↔C1 boundary for `let f = x.expect(...);`
# ============================================================================

def _scan_let_use_def(
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
    result: RuleIIIOneHits,
) -> None:
    """§1.6 D9 case B — `let f = x.expect(...); ... f(args);`.

    For each `let_declaration` whose rhs is `x.expect|unwrap|unwrap_unchecked(...)`
    with receiver typed `Option<fn ptr>`:
      - scan the enclosing block for `f(args)` at SAME scope depth →
        III① hit (with marker_seen if inner expect used the c2rust marker
        string)
      - scan the enclosing block for `f(args)` at NESTED depth (inside
        if/loop/inner block) → `c1_boundary_unresolved` bucket
        (SPEC §1.6 explicit)
      - no `f(args)` found → silent (this is C1 territory, not III①)

    Case A (`x.expect(msg)(args)` — immediate call) is already covered
    by form B inside `_classify_call`.
    Case C (other consumers of the unwrapped value) is silent = C1.
    """
    for let_node in _iter_nodes_by_type(fn_entry.node, "let_declaration"):
        pat = let_node.child_by_field_name("pattern")
        val = let_node.child_by_field_name("value")
        if pat is None or val is None or pat.type != "identifier":
            continue
        if val.type != "call_expression":
            continue
        val_fn = val.child_by_field_name("function")
        if val_fn is None or val_fn.type != "field_expression":
            continue
        method_n = val_fn.child_by_field_name("field")
        if method_n is None:
            continue
        method = method_n.text.decode(errors="replace")
        if method not in ("expect", "unwrap", "unwrap_unchecked"):
            continue
        receiver = val_fn.child_by_field_name("value")
        if receiver is None:
            continue
        receiver_type = receiver_type_at(receiver, fn_entry, struct_index, local_scope, type_aliases)
        if receiver_type is None or not is_fn_ptr(receiver_type, type_aliases):
            continue

        # Now find the enclosing `block` and scan for `f(args)`.
        f_name = pat.text.decode(errors="replace")
        enclosing = _enclosing_block(let_node)
        if enclosing is None:
            continue
        marker_seen = _check_marker_in_call_args(val)

        for call in iter_call_expressions(enclosing):
            # Skip the let's own expect call — it lives inside the same block.
            if _is_ancestor(let_node, call):
                continue
            call_fn = call.child_by_field_name("function")
            if call_fn is None or call_fn.type != "identifier":
                continue
            if call_fn.text.decode(errors="replace") != f_name:
                continue

            if _in_same_scope(call, enclosing):
                result.hits.append(
                    _build_call_site(call, f_name, fn_entry, marker_seen=marker_seen, form="A")
                )
            else:
                # nested inside if/loop/closure — can't decide III① vs C1
                result.c1_boundary_unresolved.append(
                    _build_call_site(call, f_name, fn_entry, marker_seen=marker_seen, form="A")
                )


def _enclosing_block(node: Node) -> Node | None:
    """Return the nearest ancestor whose type is `block`."""
    cur = node.parent
    while cur is not None:
        if cur.type == "block":
            return cur
        cur = cur.parent
    return None


def _in_same_scope(call: Node, enclosing_block: Node) -> bool:
    """True iff `call` is at direct-statement scope inside `enclosing_block`,
    NOT nested inside a further block / closure / if-arm / loop body.

    NOTE: tree-sitter Python binding creates a fresh Node wrapper on every
    `.parent` / `.child(...)` access — so identity via `is` fails even when
    two references point to the same underlying tree node. Compare by
    `.id` (the underlying node ID is stable within a Tree).
    """
    target_id = enclosing_block.id
    cur = call.parent
    while cur is not None and cur.id != target_id:
        if cur.type in ("block", "closure_expression",
                        "if_expression", "match_expression",
                        "loop_expression", "while_expression",
                        "for_expression"):
            return False
        cur = cur.parent
    return cur is not None and cur.id == target_id


def _is_ancestor(anc: Node, node: Node) -> bool:
    """True iff `anc` is an ancestor of (or is) `node`.
    Comparison by `.id` — see `_in_same_scope` note.
    """
    target_id = anc.id
    cur: Node | None = node
    while cur is not None:
        if cur.id == target_id:
            return True
        cur = cur.parent
    return False


# ============================================================================
# B1 — count receiver field-expression depth (§1.4 D17)
# Used to route None from `receiver_type_at` to the `iii1_deep_receiver`
# audit bucket when the miss is caused by depth-cap, not by unresolved
# binding.
# ============================================================================

_RECEIVER_FIELD_CAP = 3  # mirrors config.RECEIVER_FIELD_DEPTH_CAP


def _field_expression_depth(receiver_node: Node) -> int:
    """Count consecutive `field_expression` nesting starting from
    `receiver_node`, per SPEC §1.4 D17 counting rules:

      - each nested `field_expression` +1
      - `unary_expression(op="*", ...)` deref passes through, no cost
      - `parenthesized_expression` passes through, no cost
    """
    depth = 0
    cur: Node | None = receiver_node
    while cur is not None:
        if cur.type == "field_expression":
            depth += 1
            cur = cur.child_by_field_name("value")
            continue
        if cur.type == "parenthesized_expression":
            cur = _first_named_child(cur)
            continue
        if cur.type == "unary_expression":
            is_deref = False
            operand: Node | None = None
            for c in cur.children:
                if c.type == "*":
                    is_deref = True
                elif c.is_named:
                    operand = c
            if not is_deref:
                break
            cur = operand
            continue
        break
    return depth


# ============================================================================
# B2 — constructor / unwrap unwrap-through type inference (SPEC §1.5 gap)
#
# Handles miniz-style nested form B:
#     Some((*d).cb.expect("...")).expect("...")(args)
#
# The outer form B receiver is `Some(inner_expect)`. To decide whether
# it's Option<fn ptr>, we recurse into the Some's argument and take one
# unwrap-step so `x.expect(...)` on Option<fn ptr> yields fn ptr, then
# wrap conceptually back to Option<fn ptr>.
# ============================================================================

_CONSTRUCTOR_NAMES = frozenset({"Some", "Ok"})
_UNWRAP_METHODS = frozenset({"expect", "unwrap", "unwrap_unchecked"})


def _infer_expression_yields_fn_ptr(
    expr: Node,
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
) -> bool:
    """Return True iff `expr` evaluates to a fn-ptr value (raw fn ptr).

    Recursively handles:
      - identifier / field / deref / paren  → via receiver_type_at, then
        `is_fn_ptr` check on the resolved node (raw fn ptr, not
        Option-wrapped — that's for `_infer_expression_yields_option_fn_ptr`)
      - `x.expect|unwrap|unwrap_unchecked()` — the value is x's type
        Option-unwrapped by one layer

    Used by form B's constructor-call receiver check (see below).
    """
    # Direct path via receiver_type_at
    t = receiver_type_at(expr, fn_entry, struct_index, local_scope, type_aliases)
    if t is not None:
        # If t is fn ptr directly, done (raw fn ptr yielded).
        # Note: Option<fn ptr> also passes is_fn_ptr, but that's not what
        # we want here — we want the *unwrapped* type. Distinguish by
        # checking generic_type wrapping.
        resolved = resolve_type_alias(t, type_aliases)
        if resolved.type == "function_type":
            return True
        return False

    # Unwrap-method call: x.expect/unwrap/unwrap_unchecked()
    if expr.type == "call_expression":
        efn = expr.child_by_field_name("function")
        if efn is not None and efn.type == "field_expression":
            fld = efn.child_by_field_name("field")
            if fld is not None and fld.text.decode(errors="replace") in _UNWRAP_METHODS:
                receiver = efn.child_by_field_name("value")
                if receiver is not None:
                    rt = receiver_type_at(receiver, fn_entry, struct_index, local_scope, type_aliases)
                    if rt is not None and is_fn_ptr(rt, type_aliases):
                        # rt is Option<fn ptr> (or via alias); after
                        # unwrap yields fn ptr → yes, this expr yields fn ptr.
                        resolved = resolve_type_alias(rt, type_aliases)
                        return resolved.type == "generic_type"
    return False


def _form_b_receiver_is_option_fn_ptr(
    receiver: Node,
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    local_scope: dict[str, Node],
    type_aliases: dict[str, Node],
) -> Node | None:
    """Return receiver's type CST if it resolves to Option<fn ptr> — either
    via `receiver_type_at` (identifier/field/deref) or via `Some(x)/Ok(x)`
    constructor unwrap (B2 patch). Returns None if not Option<fn ptr>.
    """
    # Direct path
    rt = receiver_type_at(receiver, fn_entry, struct_index, local_scope, type_aliases)
    if rt is not None and is_fn_ptr(rt, type_aliases):
        return rt

    # B2: constructor call `Some(x)` / `Ok(x)` — check if x yields fn ptr.
    # Callee CST forms handled: bare `Ok`, turbofish `Ok::<_,()>`,
    # scoped `Result::Ok`.
    if receiver.type == "call_expression":
        cfn = receiver.child_by_field_name("function")
        cname = ""
        if cfn is not None:
            if cfn.type == "identifier":
                cname = cfn.text.decode(errors="replace")
            elif cfn.type == "generic_function":
                inner = cfn.child_by_field_name("function")
                if inner is not None and inner.type == "identifier":
                    cname = inner.text.decode(errors="replace")
            elif cfn.type == "scoped_identifier":
                name_node = cfn.child_by_field_name("name")
                if name_node is not None:
                    cname = name_node.text.decode(errors="replace")
        if cname in _CONSTRUCTOR_NAMES:
            args = receiver.child_by_field_name("arguments")
            if args is not None:
                for c in args.children:
                    if not c.is_named:
                        continue
                    if _infer_expression_yields_fn_ptr(
                        c, fn_entry, struct_index, local_scope, type_aliases,
                    ):
                        return receiver  # sentinel — caller only checks not-None
                    break  # first arg only
    return None


# ---- Form D: match arm body, callee is pattern-bound (§1.3 mini scope) ------

def _detect_form_d(
    call: Node,
    function_node: Node,
    match_arm: Node,
    scrutinee: Node,
    fn_entry: FnCstEntry,
    struct_index: StructIndex,
    result: RuleIIIOneHits,
) -> CallSite | None:
    """§1.2 form D via §1.3 mini scope resolver.

    Steps:
      1. Build `arm_scope: set[str]` from match_arm.pattern subtree
         (identifier names declared at pattern binding sites).
      2. Resolve scrutinee's type per §1.3 table (identifier / field_expression /
         unary_expression '*' / call_expression / other).
         - call_expression / other → log to result.scrutinee_unresolved,
           return None.
      3. If scrutinee_type is Option<fn ptr> AND pattern is `Some(f)`
         where f ∈ arm_scope → f is fn ptr.
      4. If callee identifier ∈ arm_scope AND is fn-ptr-typed → hit.
    """
    # TODO
    raise NotImplementedError("SEE SPEC §1.2 form D + §1.3 D16")


# ---- §1.6 shallow use-def III① ↔ C1 boundary --------------------------------

def resolve_c1_boundary(
    expect_call: Node,
    fn_entry: FnCstEntry,
    result: RuleIIIOneHits,
) -> str | None:
    """§1.6 D9 — decide III① or C1 for an `x.expect(...)` site.

    Cases (shallow use-def, same block scope only):
      A — enclosing expression is `call_expression` wrapping expect_call
          → returns "III①"
      B — enclosing is `let_declaration` binding a single identifier `f`;
          walk enclosing block for `call_expression(function=identifier{f})`
          → if found returns "III①", else "C1"
      C — other (return value, non-call use)
          → returns "C1"

    Unresolvable case (crosses block scope, e.g. f used inside nested
    if/loop body) logs to result.c1_boundary_unresolved and returns None.
    """
    # TODO
    raise NotImplementedError("SEE SPEC §1.6 D9 case A/B/C")
