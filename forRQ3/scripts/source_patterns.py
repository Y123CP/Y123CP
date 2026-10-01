"""Source-level shape detectors for HotspotProfile.evidence.source_patterns.

Populated kinds (Step C — 2026-06-01):
  byte_copy_loop          — `for i in <range> { dst[i] = src[i]; }`
                            (and c2rust raw-ptr form `*dst.offset(i) = *src.offset(i)`)
  byte_compare_loop       — `for i in <range> { if a[i] != b[i] { return … } }`
  byte_fill_loop          — `for i in <range> { dst[i] = <const>; }`
  byte_search_loop        — `for i in <range> { if buf[i] == target { return … } }`
  branch_cascade          — `if k==0 {} else if k==1 {} else …` or
                            `match k { 0=>…, 1=>…, _=>…, }` over a small key
                            (≥3 distinct arms, each arm is a pure value
                            mapping — no side effects)
  loop_invariant_branch   — populated by `proposer.syn_walker` (Step A)

Stubs (Step C+ — left for follow-up; both need analysis beyond pure
syntax tree walking and so don't belong in this module):
  pure_short_call         — needs call-graph + side-effect / purity facts
  state_machine_indexing  — needs scattered `slice[i]` aggregation across
                            `match` arms in the SAME fn body

Also populated here:
  fn_signature_facts.has_option_fn_ptr_param / fn_ptr_param_names —
    by scanning fn signatures for `Option<unsafe extern "C" fn(...)>`
    parameters. The caller-side targets (unique_caller_targets /
    caller_count) require a project-wide callsite walk — Step C does the
    fn-side fields; targets are filled in via `sa_iface` in Step C+.

Output is keyed by **bare fn name** (`containing_fn`), mirroring how
`proposer.syn_walker.LoopInvariantCond` indexes its output, so
`pipeline.py` can slice per-hotspot identically (bare fn ∪ inlined_fns
in the hot region).
"""

from __future__ import annotations

import logging
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterator

import tree_sitter_rust
from tree_sitter import Language, Node, Parser

from profiling.evidence import FnSignatureFacts, SourcePattern

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# tree-sitter setup
# ---------------------------------------------------------------------------

_LANG = Language(tree_sitter_rust.language())


def _make_parser() -> Parser:
    return Parser(_LANG)


def _text(node: Node | None, src: bytes) -> str:
    if node is None:
        return ""
    return src[node.start_byte:node.end_byte].decode("utf-8", errors="replace")


# ---------------------------------------------------------------------------
# Public output
# ---------------------------------------------------------------------------

@dataclass
class SourcePatternScan:
    """Project-wide scan result. Indexed by bare fn name (matches
    syn_walker's `containing_fn`) so pipeline.py can slice per-hotspot.

    Phase H.2: `layout_patterns_by_file` carries per-file layout_candidate
    entries (B1 / B2 — struct-definition-level signals). pipeline.py joins
    these to a hotspot via subregion file matches."""
    project_root:      str
    files_scanned:     int                              = 0
    files_parse_failed: list[str]                       = field(default_factory=list)
    # bare fn name → patterns inside that fn
    patterns_by_fn:    dict[str, list[SourcePattern]]   = field(default_factory=lambda: defaultdict(list))
    # bare fn name → its signature facts (callback presence, etc.)
    facts_by_fn:       dict[str, FnSignatureFacts]      = field(default_factory=dict)
    # file basename → struct-level layout_candidate patterns (Phase H.2)
    layout_patterns_by_file:  dict[str, list[SourcePattern]] = field(default_factory=dict)
    # file basename → file-level hand_written_hashmap patterns (Phase H.3)
    hashmap_patterns_by_file:  dict[str, list[SourcePattern]] = field(default_factory=dict)
    # file basename → file-level checksum_or_hash_kernel patterns from
    # top-level static/const tables (Phase H.3)
    checksum_patterns_by_file: dict[str, list[SourcePattern]] = field(default_factory=dict)


# ---------------------------------------------------------------------------
# AST predicate helpers
# ---------------------------------------------------------------------------

def _walk(node: Node) -> Iterator[Node]:
    """Depth-first walk over `node` (yields self + every descendant)."""
    stack: list[Node] = [node]
    while stack:
        n = stack.pop()
        yield n
        stack.extend(reversed(n.children))


def _strip_trivia(s: str) -> str:
    """Cheap text-normalization for cross-form comparison (drops the noisy
    `as usize` / `as isize` casts c2rust adds to indices)."""
    return (s.replace(" ", "")
             .replace("asusize", "")
             .replace("asisize", ""))


def _index_of(node: Node, src: bytes) -> str | None:
    """If `node` is `<base>[<idx>]` OR `*<base>.offset(<idx>)` /
    `*<base>.add(<idx>)`, return a normalized text of `<idx>`. Else None.

    Handles BOTH idiomatic Rust array indexing AND c2rust raw-pointer
    offset forms (post-Stage-A code may carry both)."""
    if node.type == "index_expression":
        # arr[idx] — tree-sitter-rust uses POSITIONAL named children for
        # `index_expression` (no `index` field name): named_children =
        # [base_expr, index_expr]. Verified empirically.
        nc = node.named_children
        if len(nc) < 2:
            return None
        return _strip_trivia(_text(nc[1], src))

    if node.type == "unary_expression":
        # *base
        op = node.child(0)
        if op is None or op.type != "*":
            return None
        inner = node.named_child(0)
        if inner is None or inner.type != "call_expression":
            return None
        func = inner.child_by_field_name("function")
        if func is None or func.type != "field_expression":
            return None
        method = func.child_by_field_name("field")
        if method is None:
            return None
        if _text(method, src) not in ("offset", "add", "wrapping_offset", "wrapping_add"):
            return None
        args = inner.child_by_field_name("arguments")
        if args is None:
            return None
        arg_parts = [_text(c, src) for c in args.named_children]
        if len(arg_parts) != 1:
            return None
        return _strip_trivia(arg_parts[0])

    return None


def _is_simple_path(node: Node | None) -> bool:
    """True for plain identifiers / field paths / `*p` derefs — anything
    short and side-effect-free."""
    if node is None:
        return False
    if node.type in ("identifier", "field_expression",
                      "scoped_identifier", "self"):
        return True
    if node.type == "unary_expression":
        op = node.child(0)
        if op is not None and op.type == "*":
            return _is_simple_path(node.named_child(0))
    return False


def _is_const_literal(node: Node | None) -> bool:
    """True for integer / char / boolean literals."""
    return node is not None and node.type in (
        "integer_literal", "char_literal", "boolean_literal",
        "float_literal", "negative_literal",
    )


def _enclosing_fn_name(node: Node, src: bytes) -> str:
    """Walk parents until a `function_item` is found; return its name.

    `<top-level>` matches syn_walker's convention if no enclosing fn."""
    cur: Node | None = node
    while cur is not None:
        if cur.type == "function_item":
            name = cur.child_by_field_name("name")
            return _text(name, src) if name else "<anon>"
        cur = cur.parent
    return "<top-level>"


# ---------------------------------------------------------------------------
# Byte-loop family detector
# ---------------------------------------------------------------------------

@dataclass
class _LoopShape:
    """Decomposed loop info used by byte_* detectors."""
    induction_var: str    # the loop variable name (or pointer being incremented)
    body:          Node   # block expression


def _decompose_for_range_loop(loop: Node, src: bytes) -> _LoopShape | None:
    """`for i in <range> { ... }` → (induction_var="i", body)."""
    if loop.type != "for_expression":
        return None
    pat = loop.child_by_field_name("pattern")
    val = loop.child_by_field_name("value")
    body = loop.child_by_field_name("body")
    if pat is None or val is None or body is None:
        return None
    # We accept any range value here (0..n, a..b, slice.iter().enumerate(),
    # etc.); the byte-loop detection cares about body shape, not the bound.
    if pat.type != "identifier":
        return None
    return _LoopShape(induction_var=_text(pat, src), body=body)


def _stmt_list(block: Node) -> list[Node]:
    """All statement-position nodes inside a block_expression."""
    if block.type != "block":
        return []
    return [c for c in block.named_children
            if c.type not in ("line_comment", "block_comment")]


def _unwrap_stmt(s: Node) -> Node | None:
    """Strip `expression_statement` / `unsafe_block { block { stmt } }`
    layers from one statement, returning the innermost expression.

    Returns None when the statement has multiple inner stmts inside an
    `unsafe_block` (= not a single-op pattern)."""
    cur = s
    while True:
        if cur.type == "expression_statement":
            if cur.named_child_count == 0:
                return None
            cur = cur.named_child(0)
            continue
        if cur.type == "unsafe_block":
            inner_block = None
            for c in cur.named_children:
                if c.type == "block":
                    inner_block = c
                    break
            if inner_block is None:
                return None
            inner_stmts = _stmt_list(inner_block)
            if len(inner_stmts) != 1:
                return None
            cur = inner_stmts[0]
            continue
        return cur


def _unwrap_single_stmt_block(block: Node) -> Node | None:
    """If `block` contains exactly one statement, return its innermost
    inner expression (via `_unwrap_stmt`). Else None."""
    stmts = _stmt_list(block)
    if len(stmts) != 1:
        return None
    return _unwrap_stmt(stmts[0])


def _classify_byte_op(inner: Node, induction_var: str, src: bytes
                       ) -> SourcePattern | None:
    """Classify ONE inner expression (already unwrapped of
    `expression_statement` / `unsafe_block`) as one of the byte-loop
    kinds. The caller (for-loop entry / while-loop entry) wraps this with
    the right structural check.

    Tight rules (Step C scope):
      copy:    `dst[i] = src[i];`         / `*dst.offset(i) = *src.offset(i);`
      fill:    `dst[i] = <const|simple>;` / `*dst.offset(i) = <const|simple>;`
      compare: `if a[i] != b[i] { return <const>; }`
      search:  `if buf[i] == <target> { return <something>; }`
    """
    if inner.type == "assignment_expression":
        lhs = inner.child_by_field_name("left")
        rhs = inner.child_by_field_name("right")
        lhs_idx = _index_of(lhs, src) if lhs else None
        if lhs_idx == induction_var:
            rhs_idx = _index_of(rhs, src) if rhs else None
            if rhs_idx == induction_var:
                return SourcePattern(
                    kind="byte_copy_loop",
                    loc="",   # caller fills with file:line
                    details={
                        "dst": _text(lhs, src).split("[", 1)[0].strip(),
                        "src": _text(rhs, src).split("[", 1)[0].strip(),
                        "induction_var": induction_var,
                        "form": "slice_index",   # tree-sitter form
                    },
                    related_rule="C5 replace-with-slice-op",
                    confidence="high",
                )
            # Fill: RHS is a constant or a simple non-indexed expression
            if _is_const_literal(rhs) or _is_simple_path(rhs):
                return SourcePattern(
                    kind="byte_fill_loop",
                    loc="",
                    details={
                        "buffer": _text(lhs, src).split("[", 1)[0].strip(),
                        "value":  _text(rhs, src) if rhs else "",
                        "len":    induction_var,
                        "element_type": "",
                    },
                    related_rule="C5 replace-with-slice-op",
                    confidence="high",
                )

    # ------------------------------------------------------------------
    # Conditional-return forms: compare / search
    # Body is: `if <cond> { return <expr>; }`
    # (with at most ONE other trivial assignment to a counter, but we
    # keep this strict to avoid false positives.)
    # ------------------------------------------------------------------
    if inner.type == "if_expression":
        cond = inner.child_by_field_name("condition")
        cons = inner.child_by_field_name("consequence")
        if cond is not None and cons is not None:
            # The consequence block must contain a single `return …;`
            cons_stmts = _stmt_list(cons)
            if (len(cons_stmts) == 1
                    and cons_stmts[0].type in ("expression_statement",
                                                 "return_expression")):
                ret_expr = cons_stmts[0]
                if ret_expr.type == "expression_statement":
                    ret_expr = ret_expr.named_child(0) or ret_expr
                if ret_expr.type != "return_expression":
                    return None
                # Strip the leading `return` keyword if it's an Expression
                # Compare: condition is binary `!=` with both sides
                #           indexing on induction_var
                # Search:  condition is binary `==` with one side indexing
                #           on induction_var and the other a target value
                cond_inner = cond
                # cond may itself be a parenthesized_expression
                while cond_inner.type == "parenthesized_expression":
                    cond_inner = cond_inner.named_child(0) or cond_inner
                if cond_inner.type == "binary_expression":
                    op_node = cond_inner.child(1)
                    op_text = _text(op_node, src).strip()
                    left  = cond_inner.named_child(0)
                    right = cond_inner.named_child(1)
                    l_idx = _index_of(left, src) if left else None
                    r_idx = _index_of(right, src) if right else None
                    if op_text == "!=" and l_idx == induction_var and r_idx == induction_var:
                        ret_txt = _text(ret_expr, src)
                        mb = ("return_false" if "false" in ret_txt
                              else "return_nonzero" if "1" in ret_txt or "-1" in ret_txt
                              else "break" if "break" in ret_txt
                              else "other")
                        return SourcePattern(
                            kind="byte_compare_loop",
                            loc="",
                            details={
                                "left":  _text(left, src).split("[", 1)[0].strip(),
                                "right": _text(right, src).split("[", 1)[0].strip(),
                                "len":   induction_var,
                                "mismatch_behavior": mb,
                                "success_behavior":  "fallthrough",
                            },
                            related_rule="C5 replace-with-slice-op",
                            confidence="high",
                        )
                    if op_text == "==":
                        # one side indexes the var, the other is the target
                        if l_idx == induction_var and r_idx != induction_var:
                            buf_node, tgt_node = left, right
                        elif r_idx == induction_var and l_idx != induction_var:
                            buf_node, tgt_node = right, left
                        else:
                            return None
                        return SourcePattern(
                            kind="byte_search_loop",
                            loc="",
                            details={
                                "buffer": _text(buf_node, src).split("[", 1)[0].strip(),
                                "needle": _text(tgt_node, src),
                                "return_kind": "index",   # explicit-return form usually returns idx
                            },
                            related_rule="E1-memchr replace-byte-search",
                            confidence="high",
                        )
    return None


def _classify_byte_loop_from_block(block: Node, induction_var: str, src: bytes
                                     ) -> SourcePattern | None:
    """For-loop entry: body is a single-stmt block (possibly unsafe-wrapped)."""
    inner = _unwrap_single_stmt_block(block)
    if inner is None:
        return None
    return _classify_byte_op(inner, induction_var, src)


def _classify_while_byte_loop(body: Node, induction_var: str, src: bytes
                                ) -> SourcePattern | None:
    """While-loop entry: c2rust translates `for(i=0; i<n; i++)` into
       while <cond_using_i> {
           <body_op_using_i>
           i += <step>;
       }
    Body must have EXACTLY 2 stmts: [op_stmt, increment_stmt]. We've
    already identified `induction_var` from the increment (via
    `_decompose_while_index_loop`); here we classify the op stmt."""
    stmts = _stmt_list(body)
    if len(stmts) != 2:
        return None
    inner = _unwrap_stmt(stmts[0])
    if inner is None:
        return None
    return _classify_byte_op(inner, induction_var, src)


def _is_increment_stmt(stmt: Node, src: bytes) -> str | None:
    """Identify `<id> += <const>` or `<id> = <id>.wrapping_add(<const>)`
    increment statements. Returns the induction var name or None.

    Handles c2rust's two output forms for `i++`:
      (i)  `i += 1;`                    → compound_assignment_expr
      (ii) `i = i.wrapping_add(1);`     → assignment_expression with call

    Both forms must increment by a constant (literal or `<lit> as <type>`)
    to qualify as a recognized loop induction step.
    """
    inner = (stmt.named_child(0)
              if stmt.type == "expression_statement"
                 and stmt.named_child_count > 0
              else stmt)
    if inner is None:
        return None

    # Form (i): compound `<id> += <const>` / `-=`
    if inner.type == "compound_assignment_expr":
        op_node = inner.child_by_field_name("operator")
        op = _text(op_node, src) if op_node is not None else ""
        if op not in ("+=", "-="):
            return None
        lhs = inner.child_by_field_name("left")
        rhs = inner.child_by_field_name("right")
        if lhs is None or lhs.type != "identifier":
            return None
        if not (_is_const_literal(rhs) or
                (rhs is not None and rhs.type == "type_cast_expression"
                    and _is_const_literal(rhs.named_child(0)))):
            return None
        return _text(lhs, src)

    # Form (ii): `<id> = <id>.wrapping_add(<const>)` / wrapping_sub.
    # c2rust often wraps the RHS in outer type-casts:
    #     `i = (i as c_ulong).wrapping_add(1 as c_ulong) as size_t as size_t`
    # so we strip outer type_cast_expression on RHS, and type_cast_expression
    # on the receiver of `.wrapping_add` itself.
    if inner.type == "assignment_expression":
        lhs = inner.child_by_field_name("left")
        rhs = inner.child_by_field_name("right")
        if lhs is None or lhs.type != "identifier" or rhs is None:
            return None
        # Strip outer cast(s): `<expr> as <T1> as <T2>` → `<expr>`.
        while rhs is not None and rhs.type == "type_cast_expression":
            rhs = (rhs.named_child(0)
                    if rhs.named_child_count else None)
        # Strip outer parens.
        if rhs is not None and rhs.type == "parenthesized_expression":
            rhs = rhs.named_child(0) if rhs.named_child_count else None
            while rhs is not None and rhs.type == "type_cast_expression":
                rhs = (rhs.named_child(0)
                        if rhs.named_child_count else None)
        if rhs is None or rhs.type != "call_expression":
            return None
        func = rhs.child_by_field_name("function")
        args = rhs.child_by_field_name("arguments")
        if func is None or func.type != "field_expression" or args is None:
            return None
        method = func.child_by_field_name("field")
        recv = func.child_by_field_name("value")
        if method is None or recv is None:
            return None
        if _text(method, src) not in ("wrapping_add", "wrapping_sub"):
            return None
        # Strip receiver casts/parens: `(i as c_ulong).wrapping_add(...)`.
        while recv is not None and recv.type in ("parenthesized_expression",
                                                   "type_cast_expression"):
            recv = recv.named_child(0) if recv.named_child_count else None
        if recv is None or recv.type != "identifier":
            return None
        if _text(recv, src) != _text(lhs, src):
            return None
        if args.named_child_count == 0:
            return None
        arg0 = args.named_child(0)
        if arg0 is not None and arg0.type == "type_cast_expression":
            arg0 = arg0.named_child(0)
        if arg0 is None or arg0.type != "integer_literal":
            return None
        return _text(lhs, src)

    return None


def _decompose_while_index_loop(while_node: Node, src: bytes) -> _LoopShape | None:
    """Recognize c2rust's `while <cond_using_var> { … var ±= const; }`
    shape, returning (induction_var, body) — analog of
    `_decompose_for_range_loop` for while-loops.

    v2 (2026-06-04): relaxed body shape to scan for ANY increment statement,
    not just the last-of-2. c2rust 0.22+ outputs `let fresh = <var>; <var> =
    <var>.wrapping_add(1); <body using fresh>` (libcsv, brotli, json-c, …)
    where body has many statements — v1's "exactly 2 stmts" check rejected
    all of these.

    Strictness preserved:
      - increment must be `<id> += <const>` OR
                          `<id> = <id>.wrapping_add(<const>)`  (no variable RHS)
      - condition must reference the same `<id>`
    """
    if while_node.type != "while_expression":
        return None
    cond = while_node.child_by_field_name("condition")
    body = while_node.child_by_field_name("body")
    if cond is None or body is None or body.type != "block":
        return None

    # Walk the body for any increment statement. Prefer the FIRST one whose
    # LHS appears in the condition — for nested loops the inner var might
    # increment too and we want the outer var.
    induction_var: str | None = None
    for stmt in _stmt_list(body):
        cand = _is_increment_stmt(stmt, src)
        if cand is None:
            continue
        # Consistency: the candidate id must appear in the loop condition.
        for c in _walk(cond):
            if c.type == "identifier" and _text(c, src) == cand:
                induction_var = cand
                break
        if induction_var is not None:
            break
    if induction_var is None:
        return None

    return _LoopShape(induction_var=induction_var, body=body)


# ---------------------------------------------------------------------------
# branch_cascade detector
# ---------------------------------------------------------------------------

def _detect_branch_cascade(stmt: Node, src: bytes) -> SourcePattern | None:
    """Detect:
       - `if k == c0 { v0 } else if k == c1 { v1 } else if k == c2 { v2 } [else …]`
       - `match k { c0 => v0, c1 => v1, c2 => v2, _ => default }`
    where there are ≥3 distinct constant key values and EVERY arm body is
    a pure value mapping (constant / simple expression) — i.e. the
    docs/stage_b_opt_design.md D3 trigger.

    Returns the cascade as a single SourcePattern (loc filled by caller).
    Side-effecting arms (`return`, `store`, `call`) disqualify the cascade
    so it never gets mis-routed to D3 (D3 procedure assumes pure mapping)."""
    if stmt.type == "if_expression":
        return _detect_chained_if(stmt, src)
    if stmt.type == "match_expression":
        return _detect_match_cascade(stmt, src)
    return None


def _const_or_simple(node: Node | None) -> bool:
    """Arm body is "pure value mapping" if it's a constant or a single
    identifier reference (no side effects, no control flow)."""
    if node is None:
        return False
    if _is_const_literal(node) or _is_simple_path(node):
        return True
    # A block whose only statement is a const/simple value also counts.
    if node.type == "block":
        ss = _stmt_list(node)
        if len(ss) == 1:
            inner = ss[0]
            if inner.type == "expression_statement":
                inner = inner.named_child(0) or inner
            return _const_or_simple(inner)
    return False


def _is_pure_value(node: Node | None) -> bool:
    """Broader than _const_or_simple: also accepts pure cast chains and
    raw byte-string literals — `b"foo\\0" as *const u8 as *const c_char`,
    `2 as c_int`, `CONST_NAME as u32`, etc. Used by D3 broader detector
    (F2/F3 arm bodies) which need to accept c2rust's typical cast-rich
    constants while still rejecting calls and side effects."""
    if node is None:
        return False
    if _is_const_literal(node) or _is_simple_path(node):
        return True
    if node.type in ("string_literal", "raw_string_literal",
                      "byte_string_literal", "raw_byte_string_literal"):
        return True
    if node.type == "parenthesized_expression":
        return _is_pure_value(node.named_child(0) if node.named_child_count > 0 else None)
    if node.type == "type_cast_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_pure_value(inner)
    if node.type == "unary_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_pure_value(inner)
    if node.type == "reference_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_pure_value(inner)
    return False


def _is_cascade_key(node: Node | None) -> bool:
    """Slightly broader than `_is_simple_path`: a cascade key may include
    an `as <T>` cast (c2rust commonly writes `match x as c_uint`) and
    enclosing parens. Still rejects calls / arithmetic — the key must
    be evaluatable exactly once."""
    if node is None:
        return False
    if _is_simple_path(node):
        return True
    if node.type == "parenthesized_expression":
        return _is_cascade_key(node.named_child(0) if node.named_child_count > 0 else None)
    if node.type == "type_cast_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_cascade_key(inner)
    return False


def _classify_arm_form(node: Node | None, src: bytes) -> tuple[str, str] | None:
    """Classify a cascade-arm body into one of 3 form-kinds; return
    `(form_kind, lhs_text_or_empty)` or None on reject.

    Form kinds:
      "F1" — pure value (expression position).  c2rust rarely emits this.
      "F2" — `return <pure_value>` (with or without enclosing block).
      "F3" — `<lhs> = <pure_value>` (assignment, with same lhs across all arms).

    The lhs_text is empty for F1/F2; for F3 it's the lhs text the caller
    must verify is identical across all arms."""
    if node is None:
        return None
    # F1 direct
    if _is_pure_value(node):
        return ("F1", "")
    # Unwrap single-stmt block.
    if node.type == "block":
        ss = _stmt_list(node)
        if len(ss) == 1:
            inner = ss[0]
            if inner.type == "expression_statement":
                expr = inner.named_child(0) if inner.named_child_count > 0 else None
                if expr is None:
                    return None
                return _classify_arm_form(expr, src)
            # Direct return_expression statement (no expression_statement wrapper).
            if inner.type == "return_expression":
                inner_val = inner.named_child(0) if inner.named_child_count > 0 else None
                if inner_val is not None and _is_pure_value(inner_val):
                    return ("F2", "")
                return None
            return _classify_arm_form(inner, src)
        return None
    # F2 — return <pure_value>
    if node.type == "return_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        if inner is not None and _is_pure_value(inner):
            return ("F2", "")
        return None
    # F3 — assignment_expression `<lhs> = <pure_value>`
    if node.type == "assignment_expression":
        lhs = node.child_by_field_name("left")
        rhs = node.child_by_field_name("right")
        if lhs is None or rhs is None:
            return None
        if not _is_pure_value(rhs):
            return None
        return ("F3", _text(lhs, src))
    return None


def _detect_chained_if(if_node: Node, src: bytes) -> SourcePattern | None:
    """Walk `if k==c0 { v0 } else if k==c1 { v1 } else …` cascade.

    Arm bodies are classified into F1 (pure value) / F2 (`return v`) /
    F3 (`<lhs> = v`); all arms must share the SAME form (and same lhs
    for F3). Cases must be literal integer/char values."""
    cases: list[str] = []
    arm_forms: list[tuple[str, str]] = []
    key_text: str | None = None
    has_default = False
    default_form: tuple[str, str] | None = None
    cur: Node | None = if_node
    while cur is not None and cur.type == "if_expression":
        cond = cur.child_by_field_name("condition")
        cons = cur.child_by_field_name("consequence")
        alt  = cur.child_by_field_name("alternative")
        if cond is None or cons is None:
            return None
        # Unwrap parens.
        c_inner = cond
        while c_inner.type == "parenthesized_expression":
            c_inner = c_inner.named_child(0) or c_inner
        if c_inner.type != "binary_expression":
            return None
        op_node = c_inner.child(1)
        if _text(op_node, src).strip() != "==":
            return None
        l = c_inner.named_child(0)
        r = c_inner.named_child(1)
        if l is None or r is None:
            return None
        # Pick: key = the side that is a simple path; case = the other (literal).
        if _is_simple_path(l) and _is_const_literal(r):
            this_key, this_case = _text(l, src), _text(r, src)
        elif _is_simple_path(r) and _is_const_literal(l):
            this_key, this_case = _text(r, src), _text(l, src)
        else:
            return None
        if key_text is None:
            key_text = this_key
        elif this_key != key_text:
            return None    # cascade compares against different keys → not a single dispatch
        f = _classify_arm_form(cons, src)
        if f is None:
            return None
        cases.append(this_case)
        arm_forms.append(f)
        # alt: either another if_expression (continue) or an `else { … }` block (default)
        if alt is None:
            cur = None
        elif alt.type == "else_clause":
            inner_alt = alt.named_child(0)
            if inner_alt is None:
                cur = None
            elif inner_alt.type == "if_expression":
                cur = inner_alt
            else:
                # default block — must also be a pure value mapping (same form)
                df = _classify_arm_form(inner_alt, src)
                if df is None:
                    return None
                default_form = df
                has_default = True
                cur = None
        else:
            cur = None

    if len(set(cases)) < 3:
        return None
    # All arms must share form_kind (and lhs for F3).
    form_kinds = {f[0] for f in arm_forms}
    if default_form is not None:
        form_kinds.add(default_form[0])
    if len(form_kinds) != 1:
        return None
    form_kind = next(iter(form_kinds))
    if form_kind == "F3":
        lhs_set = {f[1] for f in arm_forms if f[1]}
        if default_form is not None:
            lhs_set.add(default_form[1])
        if len(lhs_set) != 1:
            return None
    return SourcePattern(
        kind="branch_cascade",
        loc="",
        details={
            "kind":         "if_chain",
            "key":          key_text or "",
            "domain":       "",
            "case_count":   len(cases),
            "body_kind":    "pure_value_mapping",
            "body_form":    form_kind,
            "cases":        cases,
            "default":      has_default,
            "branch_count": len(cases),
            "shape":        "chained_if",
        },
        related_rule="D3 branch-cascade-to-table",
        confidence="high",
    )


def _detect_match_cascade(match_node: Node, src: bytes) -> SourcePattern | None:
    """Walk `match k { c0 => v0, c1 => v1, _ => default }`.

    Arms classified into F1/F2/F3; all arms must share form_kind (and
    lhs for F3). See `_detect_chained_if` doc for form definitions."""
    scrutinee = match_node.child_by_field_name("value")
    if scrutinee is None or not _is_cascade_key(scrutinee):
        return None
    arms_block = None
    for c in match_node.named_children:
        if c.type == "match_block":
            arms_block = c
            break
    if arms_block is None:
        return None
    cases: list[str] = []
    arm_forms: list[tuple[str, str]] = []
    has_default = False
    default_form: tuple[str, str] | None = None
    for arm in arms_block.named_children:
        if arm.type != "match_arm":
            continue
        # `arm.named_child(0)` is unsafe when an arm has 0 named children
        # (e.g. `_ => …` — `_` is an unnamed token). Use the field +
        # explicit count check.
        pat_node = arm.child_by_field_name("pattern")
        body_node = arm.child_by_field_name("value")
        if pat_node is None or body_node is None:
            return None
        # match_pattern wraps the actual pattern node — descend one level
        # ONLY if there IS one. `_` wildcard has no named children;
        # match_pattern's raw text is just `_`.
        if pat_node.type == "match_pattern":
            if pat_node.named_child_count > 0:
                inner = pat_node.named_child(0)
                if inner is not None:
                    pat_node = inner
            else:
                # No named child — only raw `_` token. Detect via text.
                if _text(pat_node, src).strip() == "_":
                    df = _classify_arm_form(body_node, src)
                    if df is None:
                        return None
                    default_form = df
                    has_default = True
                    continue
                return None
        pat_text = _text(pat_node, src).strip()
        if pat_text == "_":
            df = _classify_arm_form(body_node, src)
            if df is None:
                return None
            default_form = df
            has_default = True
            continue
        if not _is_const_literal(pat_node):
            return None
        f = _classify_arm_form(body_node, src)
        if f is None:
            return None
        cases.append(pat_text)
        arm_forms.append(f)
    if len(set(cases)) < 3:
        return None
    # All arms must share form_kind (and lhs for F3).
    form_kinds = {f[0] for f in arm_forms}
    if default_form is not None:
        form_kinds.add(default_form[0])
    if len(form_kinds) != 1:
        return None
    form_kind = next(iter(form_kinds))
    if form_kind == "F3":
        lhs_set = {f[1] for f in arm_forms if f[1]}
        if default_form is not None:
            lhs_set.add(default_form[1])
        if len(lhs_set) != 1:
            return None
    return SourcePattern(
        kind="branch_cascade",
        loc="",
        details={
            "kind":         "match",
            "key":          _text(scrutinee, src),
            "domain":       "",
            "case_count":   len(cases),
            "body_kind":    "pure_value_mapping",
            "body_form":    form_kind,
            "cases":        cases,
            "default":      has_default,
            "branch_count": len(cases),
            "shape":        "match",
        },
        related_rule="D3 branch-cascade-to-table",
        confidence="high",
    )


# ===========================================================================
# Phase H.1 detectors — raw_pointer_loop / libc_mem_calls /
# loop_alloc_free / const_size_alloc
# ===========================================================================

# Known allocator call patterns (function text). Maps callee text → spec
# `alloc` enum value per stage_b_opt_design.md HotspotProfile schema.
_ALLOC_PATTERNS: dict[str, str] = {
    "Vec::new":               "Vec::new",
    "Vec::with_capacity":     "Vec::with_capacity",
    "Vec::from":              "Vec::with_capacity",
    "Box::new":               "Box::new",
    "Box::from":              "Box::new",
    "String::new":            "Vec::new",
    "String::with_capacity":  "Vec::with_capacity",
    "String::from":           "Vec::with_capacity",
    "malloc":                 "malloc",
    "calloc":                 "calloc",
    "realloc":                "malloc",
    "__rust_alloc":           "__rust_alloc",
    "__rust_realloc":         "__rust_alloc",
    "libc::malloc":           "malloc",
    "libc::calloc":           "calloc",
    "libc::realloc":          "malloc",
}

# Known libc-mem call sites. Maps callee → (related_rule, normalized callee).
_LIBC_MEM_FNS: dict[str, str] = {
    "libc::memcpy":            "C5-libc-memcpy",
    "libc::memmove":           "C5-libc-memcpy",
    "libc::memcmp":            "C5-libc-memcmp",
    "libc::memset":            "C5-libc-memset",
    "memcpy":                  "C5-libc-memcpy",
    "memmove":                 "C5-libc-memcpy",
    "memcmp":                  "C5-libc-memcmp",
    "memset":                  "C5-libc-memset",
    "ptr::copy_nonoverlapping":     "C5-libc-memcpy",
    "ptr::copy":                    "C5-libc-memcpy",
    "ptr::write_bytes":             "C5-libc-memset",
    "core::ptr::copy_nonoverlapping": "C5-libc-memcpy",
    "core::ptr::copy":               "C5-libc-memcpy",
    "core::ptr::write_bytes":        "C5-libc-memset",
    "std::ptr::copy_nonoverlapping": "C5-libc-memcpy",
    "std::ptr::copy":                "C5-libc-memcpy",
    "std::ptr::write_bytes":        "C5-libc-memset",
}


def _allocator_label(canonical: str) -> str:
    """Map canonical alloc kind → spec `allocator` enum
    (`malloc | Box | Vec | other`) for const_size_alloc.details."""
    if canonical in ("malloc", "calloc", "realloc", "__rust_alloc", "__rust_realloc"):
        return "malloc"
    if canonical.startswith("Box"):
        return "Box"
    if canonical.startswith("Vec") or canonical.startswith("String"):
        return "Vec"
    return "other"


def _is_alloc_call(node: Node, src: bytes) -> str | None:
    """If `node` is a call_expression OR `vec![...]` macro matching a known
    alloc pattern, return the canonical allocator name; else None."""
    if node.type == "macro_invocation":
        macro = node.child_by_field_name("macro")
        if macro is None:
            return None
        if _text(macro, src) == "vec":
            return "Vec::with_capacity"  # vec![…] always allocates
        return None
    if node.type != "call_expression":
        return None
    func = node.child_by_field_name("function")
    if func is None:
        return None
    return _ALLOC_PATTERNS.get(_text(func, src).strip())


def _walk_ptr_offset_uses(body: Node, src: bytes
                           ) -> list[dict]:
    """Walk `body` for `*<base>.offset(<idx>)` / `*<base>.add(<idx>)`
    derefs (post-Stage-A code is still riddled with these). Return per-use
    info `{ptr, form, access, idx, line}` for raw_pointer_loop aggregation."""
    uses: list[dict] = []
    for n in _walk(body):
        if n.type != "unary_expression":
            continue
        op = n.child(0)
        if op is None or op.type != "*":
            continue
        if n.named_child_count == 0:
            continue
        inner = n.named_child(0)
        if inner is None or inner.type != "call_expression":
            continue
        func = inner.child_by_field_name("function")
        if func is None or func.type != "field_expression":
            continue
        method_node = func.child_by_field_name("field")
        if method_node is None:
            continue
        method = _text(method_node, src)
        if method not in ("offset", "add", "wrapping_offset", "wrapping_add"):
            continue
        base_node = func.child_by_field_name("value")
        if base_node is None:
            continue
        args = inner.child_by_field_name("arguments")
        idx_text = ""
        if args is not None and args.named_child_count > 0:
            idx_text = _text(args.named_child(0), src)
        # access: LHS of assignment_expression / compound_assignment_expr → "write"
        access = "read"
        par = n.parent
        if par is not None and par.type in ("assignment_expression",
                                              "compound_assignment_expr"):
            left = par.child_by_field_name("left")
            if left is not None and left.start_byte == n.start_byte:
                access = "write"
        uses.append({
            "ptr":    _text(base_node, src),
            "form":   ("add" if method in ("add", "wrapping_add") else "offset"),
            "access": access,
            "idx":    idx_text.strip(),
            "line":   n.start_point[0] + 1,
        })
    return uses


def _aggregate_raw_pointer_loop(loop_node: Node, loop_body: Node,
                                  induction_var: str, len_expr: str,
                                  src: bytes, file_path: str
                                  ) -> SourcePattern | None:
    """Aggregate all `*p.offset(i)` / `*p.add(i)` uses inside one loop into
    a single raw_pointer_loop pattern per spec."""
    uses = _walk_ptr_offset_uses(loop_body, src)
    if not uses:
        return None
    ptrs    = sorted({u["ptr"] for u in uses})
    forms   = {u["form"] for u in uses}
    accesses = {u["access"] for u in uses}
    if accesses == {"read"}:
        access = "read"
    elif accesses == {"write"}:
        access = "write"
    else:
        access = "read_write"
    form = "add" if forms == {"add"} else "offset"
    line = loop_node.start_point[0] + 1
    return SourcePattern(
        kind="raw_pointer_loop",
        loc=f"{file_path}:{line}",
        details={
            "loop_var":  induction_var,
            "ptrs":      ptrs,
            "len":       len_expr,
            "access":    access,
            "form":      form,
            "use_count": len(uses),
        },
        related_rule="C1 loop-to-iterator",
        confidence=("high" if len(ptrs) <= 2 and induction_var else "medium"),
    )


def _detect_libc_mem_calls(body: Node, src: bytes, file_path: str
                            ) -> list[SourcePattern]:
    """Find every libc::memcpy / memcmp / memset / ptr::copy* / write_bytes
    callsite anywhere in `body`. One SourcePattern per callsite."""
    out: list[SourcePattern] = []
    for n in _walk(body):
        if n.type != "call_expression":
            continue
        func = n.child_by_field_name("function")
        if func is None:
            continue
        callee = _text(func, src).strip()
        rule = _LIBC_MEM_FNS.get(callee)
        if rule is None:
            continue
        args_node = n.child_by_field_name("arguments")
        arg_texts: list[str] = []
        if args_node is not None:
            arg_texts = [_text(c, src) for c in args_node.named_children]
        line = n.start_point[0] + 1
        out.append(SourcePattern(
            kind="libc_mem_calls",
            loc=f"{file_path}:{line}",
            details={"callee": callee, "args": arg_texts},
            related_rule=rule,
            confidence="high",
        ))
    return out


def _detect_loop_alloc_free(loop_node: Node, src: bytes, file_path: str
                              ) -> list[SourcePattern]:
    """Per spec — one SourcePattern per alloc callsite found inside a loop body.
    `free` is conservatively `"implicit"` for Rust-side allocators (drop at
    scope exit) and `"free"` when paired with `libc::free`. Caller invokes
    this once per for_/while_/loop_expression."""
    body = loop_node.child_by_field_name("body")
    if body is None:
        return []
    out: list[SourcePattern] = []
    loop_line = loop_node.start_point[0] + 1
    # Quick scan for a paired `libc::free` in the same loop body, so we can
    # label `free` field accurately when present.
    has_libc_free = False
    for n in _walk(body):
        if n.type == "call_expression":
            fn = n.child_by_field_name("function")
            if fn is not None and _text(fn, src).strip() in ("libc::free", "free"):
                has_libc_free = True
                break

    for n in _walk(body):
        kind = _is_alloc_call(n, src)
        if kind is None:
            continue
        line = n.start_point[0] + 1
        out.append(SourcePattern(
            kind="loop_alloc_free",
            loc=f"{file_path}:{line}",
            details={
                "loop_loc":        f"{file_path}:{loop_line}",
                "alloc":           kind,
                "free":            ("free" if has_libc_free else "implicit"),
                "inside_hot_loop": True,
            },
            related_rule="B4 hoist-loop-alloc",
            confidence="medium",
        ))
    return out


def _detect_const_size_alloc(body: Node, src: bytes, file_path: str
                              ) -> list[SourcePattern]:
    """Find alloc calls whose first arg is a constant literal (compile-time
    known size). Vec::with_capacity(64) / malloc(128) / Box::new(SmallType)
    etc. Skips `vec![]` (size depends on tokens, harder to pin a constant)."""
    out: list[SourcePattern] = []
    for n in _walk(body):
        if n.type == "macro_invocation":
            continue
        kind = _is_alloc_call(n, src)
        if kind is None:
            continue
        args = n.child_by_field_name("arguments")
        if args is None or args.named_child_count == 0:
            continue   # Vec::new() / Box::new() with no size arg
        first = args.named_child(0)
        # Accept literal OR `<lit> as <ty>` cast (c2rust common).
        if _is_const_literal(first):
            size_text = _text(first, src)
        elif (first.type == "type_cast_expression"
                and first.named_child_count > 0
                and _is_const_literal(first.named_child(0))):
            size_text = _text(first, src)
        else:
            continue
        line = n.start_point[0] + 1
        out.append(SourcePattern(
            kind="const_size_alloc",
            loc=f"{file_path}:{line}",
            details={
                "size":      size_text,
                "allocator": _allocator_label(kind),
            },
            related_rule="B3 stackify-alloc",
            confidence="high",
        ))
    return out


# ===========================================================================
# Phase H.3 detectors — pure_short_call / hand_written_hashmap /
# checksum_or_hash_kernel
# ===========================================================================

# --- pure_short_call ------------------------------------------------------

# Param types accepted as "small arg domain" (C6 trigger requires bounded
# domain so we can table-ize). c2rust outputs c_char/c_int/c_uint frequently
# — we accept c_char and small unsigned/signed bytes; reject general c_int
# since its domain is huge.
_SMALL_ARG_DOMAIN_TYPES = {
    "u8":           "u8",
    "i8":           "i8",
    "bool":         "bool",
    "u16":          "u16",
    "i16":          "i16",
    "c_char":       "c_char",
    "libc::c_char": "c_char",
    "char":         "char",
}


def _try_purable(fn_node: Node, src: bytes) -> dict | None:
    """Pass-1 helper for C6 pure_short_call. Try to classify this fn as
    "small + pure + small arg domain". Returns metadata dict, else None.

    Heuristic gates (conservative):
      * 1-2 parameters; first param type in `_SMALL_ARG_DOMAIN_TYPES`
      * body block has ≤ 5 named statements
      * body has NO disqualifying patterns (assignments, unsafe_block,
        macro_invocation other than allowed pure ones, calls to known
        side-effecting libs, mid-fn returns)
      * `body_kind` derived from return type: `bool` → pure_predicate,
        otherwise → pure_mapping"""
    name_node = fn_node.child_by_field_name("name")
    params    = fn_node.child_by_field_name("parameters")
    body      = fn_node.child_by_field_name("body")
    if name_node is None or params is None or body is None:
        return None
    if body.type != "block":
        return None

    param_count = 0
    arg_domain  = ""
    for p in params.named_children:
        if p.type != "parameter":
            continue
        param_count += 1
        if param_count == 1:
            ptype = p.child_by_field_name("type")
            if ptype is not None:
                ptxt = _text(ptype, src).strip()
                arg_domain = _SMALL_ARG_DOMAIN_TYPES.get(ptxt, "")
                # also accept `<libc>::c_char` forms
                if not arg_domain and "c_char" in ptxt:
                    arg_domain = "c_char"
    if param_count == 0 or param_count > 2 or not arg_domain:
        return None

    stmts = _stmt_list(body)
    if len(stmts) > 5:
        return None

    if _has_disqualifying_for_pure(body, src):
        return None

    body_kind = "pure_mapping"
    return_type_node = fn_node.child_by_field_name("return_type")
    if return_type_node is not None:
        rtxt = _text(return_type_node, src)
        if "bool" in rtxt or rtxt.strip() == "bool":
            body_kind = "pure_predicate"

    return {
        "arg_domain":  arg_domain,
        "body_kind":   body_kind,
        "stmt_count":  len(stmts),
        "param_count": param_count,
    }


def _has_disqualifying_for_pure(body: Node, src: bytes) -> bool:
    """True if `body` contains any pattern that disqualifies pure-fn
    classification: writes, unsafe, macros, libc/io calls, mid-fn returns."""
    # Identify the LAST statement node so a tail `return …;` is allowed.
    last_stmt = None
    for c in body.named_children:
        if c.type not in ("line_comment", "block_comment"):
            last_stmt = c
    for n in _walk(body):
        t = n.type
        if t in ("assignment_expression", "compound_assignment_expr"):
            return True
        if t == "unsafe_block":
            return True
        if t == "macro_invocation":
            macro = n.child_by_field_name("macro")
            if macro is not None and _text(macro, src) not in (
                    "matches", "min", "max", "debug_assert"):
                return True
        if t == "call_expression":
            func = n.child_by_field_name("function")
            if func is None:
                continue
            fname = _text(func, src)
            if any(x in fname for x in (
                    "libc::", "malloc", "free", "calloc", "realloc",
                    "alloc::", "write", "read", "panic", "println",
                    "eprintln", "print!", "format!", "fprintf",
                    "ptr::copy", "ptr::write")):
                return True
        if t == "return_expression":
            # Allow a tail-position `return …;` (parent = expression_statement
            # which IS last_stmt). Reject all other returns.
            par = n.parent
            if par is None:
                return True
            if par.type == "expression_statement" and par is last_stmt:
                continue
            return True
    return False


def _detect_pure_short_call(body: Node, src: bytes, file_path: str,
                              purable_fns: dict[str, dict]
                              ) -> list[SourcePattern]:
    """Walk body for call_expressions whose callee (last `::`/`.` segment)
    matches a purable fn. One SourcePattern per (callsite, line)."""
    out: list[SourcePattern] = []
    seen: set[tuple] = set()
    for n in _walk(body):
        if n.type != "call_expression":
            continue
        func = n.child_by_field_name("function")
        if func is None:
            continue
        callee_text = _text(func, src).strip()
        bare = callee_text.rsplit("::", 1)[-1].rsplit(".", 1)[-1]
        info = purable_fns.get(bare)
        if info is None:
            continue
        line = n.start_point[0] + 1
        key = (bare, line)
        if key in seen:
            continue
        seen.add(key)
        out.append(SourcePattern(
            kind="pure_short_call",
            loc=f"{file_path}:{line}",
            details={
                "callee":      bare,
                "arg_domain":  info["arg_domain"],
                "body_kind":   info["body_kind"],
                "params":      info["param_count"],
                "stmt_count":  info["stmt_count"],
            },
            related_rule="C6 predicate-to-table",
            confidence=("high" if info["body_kind"] == "pure_predicate"
                        else "medium"),
        ))
    return out


# --- hand_written_hashmap -------------------------------------------------

# Patterns recognized in fn names (case-insensitive substring match).
_HASHMAP_API_PATTERNS = (
    "hm_", "hmap_", "ht_", "hashmap_", "hashtable_",
    "map_set", "map_get", "map_lookup", "map_remove", "map_insert",
    "dict_set", "dict_get", "dict_remove", "dict_insert",
    "table_set", "table_get", "table_lookup",
)
# Field names indicative of hashmap internals.
_HASHMAP_STRUCT_FIELDS = ("buckets", "entries", "bins", "slots", "hash_table",
                           "hashtable", "hash_buckets", "table",
                           "n_buckets", "nbuckets", "bucket_count",
                           "items", "elements")
# Struct name hints.
_HASHMAP_STRUCT_NAME_HINTS = ("hashmap", "hash_map", "hashtable", "hash_table",
                                "_map_t", "_dict_t", "_ht_t", "hash_t")


def _scan_hashmap_file(root: Node, src: bytes, file_path: str
                         ) -> SourcePattern | None:
    """File-level hashmap evidence collector. Returns ONE pattern per file
    when BOTH a hashmap-like struct AND ≥2 hashmap-like API fns are present.
    Lower confidence (medium) since name-based heuristics can false-positive.
    """
    api_fns: list[str] = []
    matched_struct = ""
    matched_line = 0
    matched_fields: list[str] = []

    for n in _walk(root):
        if n.type == "function_item":
            name_node = n.child_by_field_name("name")
            if name_node is None:
                continue
            fname = _text(name_node, src)
            fname_l = fname.lower()
            if any(p in fname_l for p in _HASHMAP_API_PATTERNS):
                api_fns.append(fname)
        elif n.type == "struct_item":
            name_node = n.child_by_field_name("name")
            if name_node is None:
                continue
            sname = _text(name_node, src)
            sname_l = sname.lower()
            has_hint = any(h in sname_l for h in _HASHMAP_STRUCT_NAME_HINTS)
            body = n.child_by_field_name("body")
            field_ev: list[str] = []
            if body is not None:
                for fd in body.named_children:
                    if fd.type != "field_declaration":
                        continue
                    fname_node = fd.child_by_field_name("name")
                    if fname_node is None:
                        continue
                    fnm = _text(fname_node, src).lower()
                    if fnm in _HASHMAP_STRUCT_FIELDS:
                        field_ev.append(fnm)
            if has_hint or len(field_ev) >= 2:
                if not matched_struct:
                    matched_struct = sname
                    matched_line = n.start_point[0] + 1
                matched_fields.extend(field_ev)

    if len(api_fns) < 2 or not matched_struct:
        return None

    return SourcePattern(
        kind="hand_written_hashmap",
        loc=f"{file_path}:{matched_line}",
        details={
            "api_functions":   sorted(set(api_fns))[:10],
            "data_structures": sorted(set(matched_fields))[:10],
            "struct_name":     matched_struct,
        },
        related_rule="E1-hashmap replace-hashmap",
        confidence="medium",
    )


# --- checksum_or_hash_kernel ----------------------------------------------

# Map known constant (as decimal int or hex string variants) → checksum kind.
# CRC32 polynomials and Adler32 modulus are very high-confidence.
# MD5/SHA initial state and xxHash primes are medium-confidence.
_HASH_CONSTANTS_INT: dict[int, tuple[str, str, str]] = {
    # (kind, related_rule, confidence)
    0xEDB88320: ("crc32",        "E1-crc32 replace-crc32",          "high"),
    0x04C11DB7: ("crc32",        "E1-crc32 replace-crc32",          "high"),
    0x82F63B78: ("crc32",        "E1-crc32 replace-crc32",          "high"),  # CRC32C
    65521:      ("adler32",      "E1-adler replace-adler32",        "high"),
    # MD5 initial state
    0x67452301: ("generic_hash", "E1-generic library replacement",  "medium"),
    0xEFCDAB89: ("generic_hash", "E1-generic library replacement",  "medium"),
    0x98BADCFE: ("generic_hash", "E1-generic library replacement",  "medium"),
    0x10325476: ("generic_hash", "E1-generic library replacement",  "medium"),
    # SHA-1/256 H constants (just a few)
    0x6A09E667: ("generic_hash", "E1-generic library replacement",  "medium"),
    0xBB67AE85: ("generic_hash", "E1-generic library replacement",  "medium"),
    # xxHash primes
    0x9E3779B1: ("generic_hash", "E1-generic library replacement",  "medium"),
    0x85EBCA77: ("generic_hash", "E1-generic library replacement",  "medium"),
    0xC2B2AE3D: ("generic_hash", "E1-generic library replacement",  "medium"),
    # FNV offset basis (32-bit)
    0x811C9DC5: ("generic_hash", "E1-generic library replacement",  "medium"),
}


def _parse_int_literal(text: str) -> int | None:
    """Parse a Rust integer literal — handles `0x`/`0o`/`0b` prefixes,
    `_` digit separators, and optional `u32`/`i32` suffix."""
    s = text.strip().replace("_", "")
    for suf in ("u128","i128","u64","i64","u32","i32","u16","i16",
                  "u8","i8","usize","isize"):
        if s.endswith(suf):
            s = s[:-len(suf)]
            break
    try:
        if s.startswith(("0x", "0X")):
            return int(s, 16)
        if s.startswith(("0o", "0O")):
            return int(s, 8)
        if s.startswith(("0b", "0B")):
            return int(s, 2)
        return int(s)
    except ValueError:
        return None


def _detect_file_static_checksum(root: Node, src: bytes, file_path: str
                                    ) -> list[SourcePattern]:
    """File-level scan for top-level `static`/`const` items containing
    known checksum/hash constants. Catches bzip2's `BZ2_crc32Table`
    (256-entry table with the CRC32 forward polynomial `0x04C11DB7` as
    one of its elements) — those constants live in data tables, not in
    any fn body, so the per-fn scan misses them."""
    out: list[SourcePattern] = []
    for n in _walk(root):
        if n.type not in ("static_item", "const_item"):
            continue
        name_node = n.child_by_field_name("name")
        if name_node is None:
            continue
        by_kind: dict[str, dict] = {}
        for c in _walk(n):
            if c.type != "integer_literal":
                continue
            v = _parse_int_literal(_text(c, src))
            if v is None:
                continue
            info = _HASH_CONSTANTS_INT.get(v)
            if info is None:
                continue
            kind, rule, conf = info
            entry = by_kind.setdefault(
                kind, {"evidence": [], "rule": rule, "conf": conf})
            lit_txt = _text(c, src)
            if lit_txt not in entry["evidence"]:
                entry["evidence"].append(lit_txt)
        if not by_kind:
            continue
        line = n.start_point[0] + 1
        for kind, info in by_kind.items():
            out.append(SourcePattern(
                kind="checksum_or_hash_kernel",
                loc=f"{file_path}:{line}",
                details={
                    "kind":        kind,
                    "evidence":    info["evidence"],
                    "container":   "static_table",
                    "static_name": _text(name_node, src),
                },
                related_rule=info["rule"],
                confidence=info["conf"],
            ))
    return out


def _detect_longest_match_canonical(fn_node: Node, src: bytes,
                                      file_path: str
                                      ) -> Optional[SourcePattern]:
    """E1-bytecmp trigger — recognize the zlib LZ77 longest_match canonical
    shape via a 4-anchor structural fingerprint.

    Match criteria (all must hold):
      A1. Bare fn name == "longest_match"
      A2. Signature contains `*mut deflate_state` and `IPos` and `-> uInt`
      A3. Body reads ≥ 4 of the canonical deflate_state fields
          (max_chain_length, strstart, prev_length, nice_match, prev,
           window, w_size, w_mask, lookahead)
      A4. Body has ≥ 4 occurrences of `*scan ... == *match_0`-style byte
          equality compares (indicates 8-way unroll)

    All four required → emit SourcePattern. False-positive cost: fn must
    literally be named `longest_match`, which alone rules out >99.9% of
    fns. The 4 anchors together yield near-zero FP across zlib forks.
    """
    name_node = fn_node.child_by_field_name("name")
    if name_node is None:
        return None
    fn_name = _text(name_node, src).strip()
    if fn_name != "longest_match":
        return None

    # A2: signature substrings (skip full regex — fn_node guarantees `fn` shape)
    params = fn_node.child_by_field_name("parameters")
    ret    = fn_node.child_by_field_name("return_type")
    sig_text = ""
    if params is not None:
        sig_text += _text(params, src)
    if ret is not None:
        sig_text += " " + _text(ret, src)
    if ("*mut deflate_state" not in sig_text
            or "IPos" not in sig_text
            or "uInt" not in sig_text):
        return None

    body = fn_node.child_by_field_name("body")
    if body is None:
        return None
    body_text = _text(body, src)

    # A3: deflate_state field reads
    canonical_fields = (
        "max_chain_length", "strstart", "prev_length", "nice_match",
        "prev", "window", "w_size", "w_mask", "lookahead",
    )
    fields_seen = sum(1 for f in canonical_fields if f"(*s).{f}" in body_text)
    if fields_seen < 4:
        return None

    # A4: byte-cmp pattern count — match `*scan ... == *match_0`
    # Counts the 8-way unrolled byte equality compares; 4+ indicates unroll.
    byte_cmp_count = body_text.count("*scan as c_int == *match_0 as c_int") \
        + body_text.count("*scan == *match_0")
    if byte_cmp_count < 4:
        return None

    fn_line = fn_node.start_point[0] + 1
    return SourcePattern(
        kind="longest_match_canonical",
        loc=f"{file_path}:{fn_line}",
        details={
            "fn_name":       fn_name,
            "fields_seen":   fields_seen,
            "byte_cmp_count": byte_cmp_count,
            "fn_start_line": fn_line,
            "fn_end_line":   fn_node.end_point[0] + 1,
        },
        related_rule="E1-bytecmp longest_match AVX2 substitution",
        confidence="high",
    )


def _detect_checksum_kernel(fn_node: Node, src: bytes, file_path: str
                              ) -> list[SourcePattern]:
    """Scan fn body for known checksum / hash constants. Group hits by
    kind — one SourcePattern per (fn, kind).

    Also surfaces "256-entry static table" as evidence (a common
    fingerprint of CRC/lookup-driven implementations)."""
    body = fn_node.child_by_field_name("body")
    if body is None:
        return []
    by_kind: dict[str, dict] = {}   # kind → {"evidence":[…], "rule":…, "conf":…}

    for n in _walk(body):
        if n.type != "integer_literal":
            continue
        v = _parse_int_literal(_text(n, src))
        if v is None:
            continue
        info = _HASH_CONSTANTS_INT.get(v)
        if info is None:
            continue
        kind, rule, conf = info
        entry = by_kind.setdefault(kind, {"evidence": [], "rule": rule, "conf": conf})
        lit_txt = _text(n, src)
        if lit_txt not in entry["evidence"]:
            entry["evidence"].append(lit_txt)

    if not by_kind:
        return []

    out: list[SourcePattern] = []
    line = fn_node.start_point[0] + 1
    for kind, info in by_kind.items():
        out.append(SourcePattern(
            kind="checksum_or_hash_kernel",
            loc=f"{file_path}:{line}",
            details={
                "kind":     kind,
                "evidence": info["evidence"],
            },
            related_rule=info["rule"],
            confidence=info["conf"],
        ))
    return out


# ===========================================================================
# Phase H.2 detectors — branchless_candidate /
# scattered_state_machine_indexing / memory_access_pattern / layout_candidate
# ===========================================================================

# --- branchless_candidate -------------------------------------------------

# Allow-list of binary operators considered "cheap pure" for D2 branchless.
_CHEAP_BIN_OPS = {
    "+", "-", "*", "/", "%",
    "&", "|", "^", "<<", ">>",
    "==", "!=", "<", "<=", ">", ">=",
    "&&", "||",
}


def _is_cheap_value_expr(node: Node | None, src: bytes, depth: int = 4) -> bool:
    """True iff `node` is a "cheap, pure" expression — constants, simple
    paths, arithmetic / comparison on cheap parts. Used by D2 trigger to
    confirm both branches can be unconditionally evaluated without
    side effects, calls, or possible panics.

    Rejects: call_expression, index_expression (could panic), method
    calls, return / break / continue, assignment, macro invocation."""
    if node is None or depth <= 0:
        return False
    t = node.type
    if t in ("integer_literal", "float_literal", "char_literal",
              "boolean_literal", "negative_literal", "string_literal",
              "raw_string_literal", "byte_string_literal",
              "identifier", "self", "scoped_identifier", "field_expression"):
        return True
    if t == "binary_expression":
        op = node.child(1)
        if op is None or _text(op, src).strip() not in _CHEAP_BIN_OPS:
            return False
        l = node.named_child(0) if node.named_child_count > 0 else None
        r = node.named_child(1) if node.named_child_count > 1 else None
        return (_is_cheap_value_expr(l, src, depth - 1)
                and _is_cheap_value_expr(r, src, depth - 1))
    if t == "unary_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_cheap_value_expr(inner, src, depth - 1)
    if t == "parenthesized_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_cheap_value_expr(inner, src, depth - 1)
    if t == "type_cast_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_cheap_value_expr(inner, src, depth - 1)
    if t == "reference_expression":
        inner = node.named_child(0) if node.named_child_count > 0 else None
        return _is_cheap_value_expr(inner, src, depth - 1)
    return False


def _detect_branchless_candidate(if_node: Node, src: bytes, file_path: str
                                   ) -> SourcePattern | None:
    """`if c { e1 } else { e2 }` where both arms are cheap-value-yielding
    and equivalent forms can be lowered to branchless ops by LLVM (or
    explicit cmov / table lookup). Returns one SourcePattern per qualifying
    if. Skips if-else-if cascades (those route to branch_cascade)."""
    if if_node.type != "if_expression":
        return None
    cond = if_node.child_by_field_name("condition")
    cons = if_node.child_by_field_name("consequence")
    alt  = if_node.child_by_field_name("alternative")
    if cond is None or cons is None or alt is None:
        return None
    if cons.type != "block":
        return None
    alt_inner = alt
    if alt.type == "else_clause":
        if alt.named_child_count == 0:
            return None
        alt_inner = alt.named_child(0)
    # Skip cascades: `else if` chains route to branch_cascade.
    if alt_inner.type == "if_expression":
        return None
    if alt_inner.type != "block":
        return None

    cons_inner = _unwrap_single_stmt_block(cons)
    alt_expr   = _unwrap_single_stmt_block(alt_inner)
    if cons_inner is None or alt_expr is None:
        return None

    # Case 1: both arms assign to the same LHS (pure_value_assignment).
    if (cons_inner.type == "assignment_expression"
            and alt_expr.type == "assignment_expression"):
        l1 = cons_inner.child_by_field_name("left")
        l2 = alt_expr.child_by_field_name("left")
        r1 = cons_inner.child_by_field_name("right")
        r2 = alt_expr.child_by_field_name("right")
        if not (l1 and l2 and r1 and r2):
            return None
        if _text(l1, src) != _text(l2, src):
            return None
        if not (_is_cheap_value_expr(r1, src) and _is_cheap_value_expr(r2, src)):
            return None
        body_kind = "pure_value_assignment"
        then_text = _text(r1, src)
        else_text = _text(r2, src)
    # Case 2: bare values — `if c { a } else { b }` as an expression.
    elif _is_cheap_value_expr(cons_inner, src) and _is_cheap_value_expr(alt_expr, src):
        then_text = _text(cons_inner, src)
        else_text = _text(alt_expr, src)
        # Heuristic minmax detection: `if a OP b { a } else { b }`.
        body_kind = "select"
        cond_inner = cond
        while cond_inner.type == "parenthesized_expression":
            cond_inner = cond_inner.named_child(0) or cond_inner
        if cond_inner.type == "binary_expression":
            op_text = _text(cond_inner.child(1), src).strip()
            if op_text in ("<", "<=", ">", ">="):
                cl = cond_inner.named_child(0)
                cr = cond_inner.named_child(1)
                if cl is not None and cr is not None:
                    cl_t, cr_t = _text(cl, src), _text(cr, src)
                    if {then_text, else_text} == {cl_t, cr_t}:
                        body_kind = "minmax"
    else:
        return None

    line = if_node.start_point[0] + 1
    return SourcePattern(
        kind="branchless_candidate",
        loc=f"{file_path}:{line}",
        details={
            "condition": _text(cond, src),
            "then_expr": then_text,
            "else_expr": else_text,
            "body_kind": body_kind,
        },
        related_rule="D2 branch-to-branchless",
        confidence=("high" if body_kind in ("pure_value_assignment", "minmax")
                    else "medium"),
    )


# --- scattered_state_machine_indexing -------------------------------------

def _walk_index_accesses(body: Node, src: bytes) -> list[str]:
    """Collect every indexing-like access inside `body`:
        array indexing  `expr[i]`
        raw-ptr deref   `*expr.offset(i)` / `*expr.add(i)`
    Returns short text of each occurrence."""
    out: list[str] = []
    for n in _walk(body):
        if n.type == "index_expression":
            out.append(_text(n, src).split("\n")[0][:80])
            continue
        if n.type == "unary_expression":
            op = n.child(0)
            if op is None or op.type != "*":
                continue
            inner = n.named_child(0) if n.named_child_count > 0 else None
            if inner is None or inner.type != "call_expression":
                continue
            func = inner.child_by_field_name("function")
            if func is None or func.type != "field_expression":
                continue
            method_node = func.child_by_field_name("field")
            if method_node is None:
                continue
            if _text(method_node, src) in ("offset", "add", "wrapping_offset",
                                            "wrapping_add"):
                out.append(_text(n, src).split("\n")[0][:80])
    return out


def _detect_scattered_state_machine_indexing(node: Node, src: bytes,
                                                file_path: str
                                                ) -> SourcePattern | None:
    """Match expression or chained-if with ≥3 scattered index accesses across
    ≥2 arms — the C7 trigger. Differs from branch_cascade (D3) which is
    pure value mapping; this one has indexing in arm bodies."""
    if node.type == "match_expression":
        scrutinee = node.child_by_field_name("value")
        if scrutinee is None:
            return None
        arms_block = None
        for c in node.named_children:
            if c.type == "match_block":
                arms_block = c
                break
        if arms_block is None:
            return None
        idx_sites: list[str] = []
        arm_count = 0
        arms_with_idx = 0
        for arm in arms_block.named_children:
            if arm.type != "match_arm":
                continue
            arm_count += 1
            arm_body = arm.child_by_field_name("value")
            if arm_body is None:
                continue
            accesses = _walk_index_accesses(arm_body, src)
            if accesses:
                arms_with_idx += 1
                idx_sites.extend(accesses)
        if len(idx_sites) < 3 or arms_with_idx < 2 or arm_count < 3:
            return None
        line = node.start_point[0] + 1
        unique_sites = list(dict.fromkeys(idx_sites))[:10]
        return SourcePattern(
            kind="scattered_state_machine_indexing",
            loc=f"{file_path}:{line}",
            details={
                "state_var":     _text(scrutinee, src),
                "index_sites":   unique_sites,
                "structure":     "match",
                "arm_count":     arm_count,
                "arms_with_idx": arms_with_idx,
            },
            related_rule="C7 unchecked-index-elim",
            confidence="medium",
        )
    if node.type == "if_expression":
        idx_sites_if: list[str] = []
        arms_with_idx = 0
        arm_count = 0
        state_var = ""
        cur: Node | None = node
        while cur is not None and cur.type == "if_expression":
            arm_count += 1
            cons = cur.child_by_field_name("consequence")
            if cons is not None:
                accesses = _walk_index_accesses(cons, src)
                if accesses:
                    arms_with_idx += 1
                    idx_sites_if.extend(accesses)
            if not state_var:
                cnd = cur.child_by_field_name("condition")
                if cnd is not None:
                    cnd_inner = cnd
                    while cnd_inner.type == "parenthesized_expression":
                        cnd_inner = cnd_inner.named_child(0) or cnd_inner
                    if cnd_inner.type == "binary_expression":
                        l = cnd_inner.named_child(0)
                        if l is not None:
                            state_var = _text(l, src)
            alt = cur.child_by_field_name("alternative")
            if alt is None:
                cur = None
            elif alt.type == "else_clause":
                inner = alt.named_child(0) if alt.named_child_count > 0 else None
                if inner is not None and inner.type == "if_expression":
                    cur = inner
                else:
                    arm_count += 1
                    if inner is not None:
                        accesses = _walk_index_accesses(inner, src)
                        if accesses:
                            arms_with_idx += 1
                            idx_sites_if.extend(accesses)
                    cur = None
            else:
                cur = None
        if arm_count < 3 or arms_with_idx < 2 or len(idx_sites_if) < 3:
            return None
        line = node.start_point[0] + 1
        unique_sites = list(dict.fromkeys(idx_sites_if))[:10]
        return SourcePattern(
            kind="scattered_state_machine_indexing",
            loc=f"{file_path}:{line}",
            details={
                "state_var":     state_var,
                "index_sites":   unique_sites,
                "structure":     "chained_if",
                "arm_count":     arm_count,
                "arms_with_idx": arms_with_idx,
            },
            related_rule="C7 unchecked-index-elim",
            confidence="medium",
        )
    return None


# --- memory_access_pattern ------------------------------------------------

import re as _re   # local-scope alias to avoid shadowing module-level `re`


def _classify_bound_expr(node: Node | None, src: bytes) -> tuple[str, int | None]:
    """Strip `as <type>` casts and parens; return (bound_kind, bound_value).

      const_lit    — bare integer literal RHS (e.g. `while i < 16`)
      dynamic_var  — single identifier RHS  (e.g. `while i < nGroups`)
      unknown      — anything else (binary expr, call, indexed, …)

    `bound_value` is only filled for const_lit; dynamic_var leaves it None
    rather than guessing across-scope const resolution (avoids false claims
    of small bound for project-wide constants)."""
    while True:
        if node is None:
            return ("unknown", None)
        if node.type == "type_cast_expression":
            inner = node.named_child(0)
            if inner is None:
                return ("unknown", None)
            node = inner
            continue
        if node.type == "parenthesized_expression":
            inner = node.named_child(0)
            if inner is None:
                return ("unknown", None)
            node = inner
            continue
        break
    if node.type == "integer_literal":
        try:
            return ("const_lit", int(_text(node, src).replace("_", ""), 0))
        except (ValueError, TypeError):
            return ("const_lit", None)
    if node.type == "identifier":
        return ("dynamic_var", None)
    return ("unknown", None)


def _expr_contains_ident(node: Node | None, name: str, src: bytes) -> bool:
    if node is None:
        return False
    for n in _walk(node):
        if n.type == "identifier" and _text(n, src) == name:
            return True
    return False


def _extract_loop_bound(loop_node: Node, induction_var: str, src: bytes
                          ) -> tuple[str, int | None]:
    """For a `for` or `while` loop, return the trip-count bound as
    (bound_kind, bound_value) — see `_classify_bound_expr`.

    Recognised shapes:
      for <var> in <start>..<end>     → classify <end>
      for <var> in <start>..=<end>    → classify <end> (+1 if int)
      while <var> <op> <expr>         → classify <expr>
      while <expr> <op> <var>         → classify <expr> (swapped sides)

    Anything else → ("unknown", None). The induction_var arg is used to
    pick the non-var side of the comparison even when the loop var is
    wrapped in a cast (e.g. `(i as usize) < n`)."""
    if loop_node.type == "for_expression":
        val = loop_node.child_by_field_name("value")
        if val is None:
            return ("unknown", None)
        if val.type == "range_expression":
            children = list(val.named_children)
            if not children:
                return ("unknown", None)
            end_node = children[-1]
            kind, value = _classify_bound_expr(end_node, src)
            # `0..=N` is one trip count larger; only adjust if literal.
            for c in val.children:
                if c.type == "..=" and kind == "const_lit" and value is not None:
                    value = value + 1
                    break
            return (kind, value)
        return ("unknown", None)

    if loop_node.type != "while_expression":
        return ("unknown", None)
    cond = loop_node.child_by_field_name("condition")
    if cond is None:
        return ("unknown", None)
    # Walk into the condition to find a single comparison binary_expr.
    # We extract the trip-count UPPER bound only; backward (`i >= 0`,
    # `i > 0`) loops have init point unknown without inspecting the init
    # statement, so we report unknown for them rather than silently
    # treating the lower bound as a trip count (would mis-filter
    # blocksort.rs:1091's `while i >= 0` over a multi-K-element ftab).
    for n in _walk(cond):
        if n.type != "binary_expression":
            continue
        op_node = n.child_by_field_name("operator")
        op_text = _text(op_node, src) if op_node else ""
        if op_text not in ("<", "<=", ">", ">="):
            continue
        lhs = n.child_by_field_name("left")
        rhs = n.child_by_field_name("right")
        lhs_has = _expr_contains_ident(lhs, induction_var, src)
        rhs_has = _expr_contains_ident(rhs, induction_var, src)
        if lhs_has == rhs_has:    # both or neither — skip ambiguous shape
            continue
        # `i < X` / `i <= X`:                 induction LHS, upper bound = X
        # `X > i` / `X >= i`  (≡ `i < X`):    induction RHS, upper bound = X
        # `i > X` / `i >= X`:                 induction LHS, lower bound = X
        # `X < i` / `X <= i`  (≡ `i > X`):    induction RHS, lower bound = X
        if lhs_has:
            bound_side = rhs
            is_upper   = op_text in ("<", "<=")
        else:
            bound_side = lhs
            is_upper   = op_text in (">", ">=")
        if not is_upper:
            return ("unknown", None)
        return _classify_bound_expr(bound_side, src)
    return ("unknown", None)


def _detect_memory_access_pattern(loop_node: Node, loop_body: Node,
                                    induction_var: str, src: bytes,
                                    file_path: str
                                    ) -> SourcePattern | None:
    """Classify the dominant memory access pattern in a loop body for
    B5 insert-prefetch routing.

    Categories per spec:
      sequential     — `arr[i]` / `*p.offset(i)` with i = induction var, stride=1
      strided        — `arr[i * K]` / `arr[K * i]`, stride=K
      indexed        — `arr[idx[i]]` (gather)
      pointer_chasing — `node = node.next` form (currently best-effort, hard to
                          detect via tree-sitter alone — heuristic only)
      unknown        — everything else"""
    if loop_body is None or not induction_var:
        return None

    bases: list[str] = []
    idx_texts: list[str] = []
    for n in _walk(loop_body):
        if n.type == "index_expression":
            if n.named_child_count < 2:
                continue
            base = _text(n.named_children[0], src)
            idx  = _text(n.named_children[1], src)
            bases.append(base)
            idx_texts.append(idx)
        elif n.type == "unary_expression":
            op = n.child(0)
            if op is None or op.type != "*":
                continue
            inner = n.named_child(0) if n.named_child_count > 0 else None
            if inner is None or inner.type != "call_expression":
                continue
            func = inner.child_by_field_name("function")
            if func is None or func.type != "field_expression":
                continue
            method_node = func.child_by_field_name("field")
            if method_node is None or _text(method_node, src) not in (
                    "offset", "add", "wrapping_offset", "wrapping_add"):
                continue
            base_node = func.child_by_field_name("value")
            args = inner.child_by_field_name("arguments")
            if base_node is None or args is None or args.named_child_count == 0:
                continue
            bases.append(_text(base_node, src))
            idx_texts.append(_text(args.named_child(0), src))

    if not idx_texts:
        return None

    induction = induction_var.replace(" ", "")
    norm_idxs = [_strip_trivia(i) for i in idx_texts]

    # sequential: every index equals induction var
    if all(i == induction for i in norm_idxs):
        kind = "sequential"
        stride = "1"
        predictable = True
    else:
        # strided: idx = `i * K` or `K * i` or `i + K`
        strided_K = None
        for i in norm_idxs:
            m = _re.match(rf"^{_re.escape(induction)}\*(\d+)$", i)
            if m is None:
                m = _re.match(rf"^(\d+)\*{_re.escape(induction)}$", i)
            if m is not None:
                strided_K = m.group(1)
                break
        # indexed (gather): idx contains another indexing
        indexed = any(("[" in t and "]" in t) or ".offset(" in t or ".add(" in t
                      for t in idx_texts)
        if indexed:
            kind = "indexed"
            stride = "unknown"
            predictable = False
        elif strided_K is not None:
            kind = "strided"
            stride = strided_K
            predictable = True
        else:
            kind = "unknown"
            stride = "unknown"
            predictable = False

    from collections import Counter
    dom_base = Counter(bases).most_common(1)[0][0] if bases else ""

    # Iteration-shape signals for B5 to filter short / trivial loops
    # without paying a full build+W2 cycle (each B5 site = ~70s on bzip2).
    bound_kind, bound_value = _extract_loop_bound(loop_node, induction_var, src)
    body_lines = loop_body.end_point[0] - loop_body.start_point[0] + 1

    line = loop_node.start_point[0] + 1
    return SourcePattern(
        kind="memory_access_pattern",
        loc=f"{file_path}:{line}",
        details={
            "kind":        kind,
            "base":        dom_base,
            "index":       induction_var,
            "stride":      stride,
            "predictable": predictable,
            "bound_kind":  bound_kind,     # const_lit / dynamic_var / unknown
            "bound_value": bound_value,    # int (only for const_lit) else None
            "body_lines":  body_lines,     # ≈ static body size proxy
        },
        related_rule="B5 insert-prefetch",
        confidence=("high" if kind == "sequential"
                    else "medium" if kind in ("strided", "indexed")
                    else "low"),
    )


# --- layout_candidate -----------------------------------------------------

# Wide int field types that B2 narrow-int-field would consider.
_WIDE_INT_TYPES = (
    "i32", "u32", "i64", "u64", "isize", "usize",
    "c_int", "c_long", "c_uint", "c_ulong",
    "libc::c_int", "libc::c_long", "libc::c_uint", "libc::c_ulong",
)


# --- callback_devirt ------------------------------------------------------
# D1 callback-devirtualization detector. Two phases:
#
# (1) Project-wide scan via `_scan_callback_field_targets_global`:
#     For every `<expr>.<field> = <RHS>` or struct-literal `<T> { <field>: <RHS> }`
#     in any *.rs under the crate, classify the RHS as:
#         - None                  → ok (no target added)
#         - Some(<literal_ident>) → ok, target_fn = ident
#         - Some(<literal_ident> as <T>) → same, cast strips off
#         - anything else (variable RHS, conditional Some(if), etc.) → BLOCK
#     The "BLOCK" marker poisons the field for the whole crate — once any
#     non-literal RHS is observed, that field can NEVER be devirtualized.
#     This conservative rule rejects setter functions of the shape
#         `pub fn set_cb(p, f) { (*p).cb = f; }`
#     automatically — RHS `f` is a variable, hence BLOCK.
#
# (2) Per-fn detector via `_detect_callback_devirt`:
#     Walk the fn body for call expressions of shape
#         `<expr>.<field>.expect("non-null function pointer")(<args>)`
#     For each, look up the field in the global map. If the field has
#     exactly ONE target_fn AND no BLOCK → emit a SourcePattern at the
#     callsite, recording (receiver_text, field, target_fn).
#
# Emits one SourcePattern per fn (not per callsite) — multiple callsites
# in the same fn batch into one record's `details.callsites` list. D1's
# rule.apply() then sends the whole fn to the LLM in one prompt.


def _classify_callback_rhs(rhs: Node | None, src: bytes
                             ) -> tuple[str, str | None]:
    """Classify a struct/field-assignment RHS. Returns (kind, target_fn).
      ("none",    None)    — `None` literal (initial / clear)
      ("some",    "f")     — `Some(f)` or `Some(f as <Type>)` (single target)
      ("blocked", None)    — anything else (variable / conditional / table /
                             struct field / expression). This kind poisons
                             the field — any later detection on that field
                             must degrade.
    """
    if rhs is None:
        return ("blocked", None)
    # Unwrap parentheses if any.
    if rhs.type == "parenthesized_expression":
        inner = rhs.named_child(0) if rhs.named_child_count else None
        return _classify_callback_rhs(inner, src)
    # Strip outer cast: c2rust wraps `Some(f as <fn_ty>) as Option<...>`.
    if rhs.type == "type_cast_expression":
        inner = rhs.named_child(0) if rhs.named_child_count else None
        return _classify_callback_rhs(inner, src)
    # `None`
    if rhs.type == "identifier" and _text(rhs, src) == "None":
        return ("none", None)
    if rhs.type == "scoped_identifier":
        # e.g. core::option::Option::None — rare in c2rust but handle.
        if _text(rhs, src).endswith("::None"):
            return ("none", None)
    # `Some(<inner>)` — call_expression with function identifier "Some".
    if rhs.type == "call_expression":
        func = rhs.child_by_field_name("function")
        args = rhs.child_by_field_name("arguments")
        if (func is not None and args is not None
                and func.type == "identifier"
                and _text(func, src) == "Some"
                and args.named_child_count == 1):
            inner = args.named_child(0)
            # Strip cast(s): `<f> as <T>` → <f>. May nest (rare).
            while inner is not None and inner.type == "type_cast_expression":
                inner = (inner.named_child(0)
                          if inner.named_child_count else None)
            if inner is not None and inner.type == "parenthesized_expression":
                inner = inner.named_child(0) if inner.named_child_count else None
                while inner is not None and inner.type == "type_cast_expression":
                    inner = (inner.named_child(0)
                              if inner.named_child_count else None)
            if inner is not None and inner.type == "identifier":
                return ("some", _text(inner, src))
            # Else: Some(expr) with non-identifier inner → BLOCK
    return ("blocked", None)


def _scan_callback_field_targets_global(
    parsed: list,
) -> dict[str, tuple[str | None, bool]]:
    """Walk every parsed file; for every callback field assignment, update
    a global field-target map.

    Returns: field_name → (single_target_fn_or_None, blocked).
      - single_target_fn: the one fn assigned, or None if only None / no
        Some seen.
      - blocked: True if multiple distinct targets seen OR any RHS
        classified as "blocked".

    `parsed` is the same list of (path, src_bytes, tree) the main scanner
    built in Phase 0.
    """
    # During scan: field_name → (set_of_targets, blocked_flag)
    accum: dict[str, tuple[set[str], bool]] = {}

    def _record(field: str, kind: str, target: str | None) -> None:
        cur_targets, cur_blocked = accum.get(field, (set(), False))
        if kind == "blocked":
            accum[field] = (cur_targets, True)
        elif kind == "some" and target is not None:
            cur_targets.add(target)
            accum[field] = (cur_targets, cur_blocked)
        # "none" doesn't change anything.

    for (_rs, src, tree) in parsed:
        for n in _walk(tree.root_node):
            # Case A: <expr>.<field> = <RHS>
            if n.type == "assignment_expression":
                lhs = n.child_by_field_name("left")
                rhs = n.child_by_field_name("right")
                if (lhs is not None and lhs.type == "field_expression"
                        and rhs is not None):
                    field_node = lhs.child_by_field_name("field")
                    if field_node is not None:
                        field_name = _text(field_node, src)
                        kind, target = _classify_callback_rhs(rhs, src)
                        # Only record if the field MIGHT be a callback
                        # (any kind moves the field into our tracked set).
                        # "none" assignments don't move anything but we
                        # still want to know the field exists — drop, OK.
                        if kind != "none" or field_name in accum:
                            _record(field_name, kind, target)
            # Case B: struct literal `<T> { ..., <field>: <RHS>, ... }`
            elif n.type == "struct_expression":
                # Find every field_initializer child.
                for c in _walk(n):
                    if c.type == "field_initializer":
                        name_node = c.child_by_field_name("name")
                        val_node = c.child_by_field_name("value")
                        if name_node is None or val_node is None:
                            continue
                        field_name = _text(name_node, src)
                        kind, target = _classify_callback_rhs(val_node, src)
                        if kind != "none" or field_name in accum:
                            _record(field_name, kind, target)

    # Finalize: collapse set → single_target, decide blocked.
    out: dict[str, tuple[str | None, bool]] = {}
    for field, (targets, blocked) in accum.items():
        if blocked:
            out[field] = (None, True)
            continue
        if len(targets) == 0:
            # Only None assignments observed; field is initialized to None
            # but never set to Some(f). Devirtualization wouldn't have a
            # target — degrade.
            out[field] = (None, True)
        elif len(targets) == 1:
            out[field] = (next(iter(targets)), False)
        else:
            out[field] = (None, True)   # multi-target
    return out


def _detect_callback_devirt(
    fn_node: Node, src: bytes, file_path: str,
    field_targets: dict[str, tuple[str | None, bool]],
) -> SourcePattern | None:
    """Find every `<expr>.<field>.expect("non-null function pointer")(<args>)`
    in this fn whose <field> is single-target in `field_targets`. Emit one
    SourcePattern aggregating all such callsites, or None if no candidate.
    """
    if not field_targets:
        return None
    body = fn_node.child_by_field_name("body")
    if body is None:
        return None
    name_node = fn_node.child_by_field_name("name")
    fn_name = _text(name_node, src) if name_node else "<anon>"

    callsites: list[dict] = []
    for n in _walk(body):
        # Outer call: <inner>(<args>)
        if n.type != "call_expression":
            continue
        inner = n.child_by_field_name("function")
        if inner is None or inner.type != "call_expression":
            continue
        # Inner: `<R>.expect("non-null function pointer")`
        inner_func = inner.child_by_field_name("function")
        if inner_func is None or inner_func.type != "field_expression":
            continue
        method = inner_func.child_by_field_name("field")
        if method is None or _text(method, src) != "expect":
            continue
        inner_args = inner.child_by_field_name("arguments")
        if (inner_args is None or inner_args.named_child_count != 1):
            continue
        msg_arg = inner_args.named_child(0)
        if msg_arg is None or msg_arg.type != "string_literal":
            continue
        msg_text = _text(msg_arg, src)
        if "non-null function pointer" not in msg_text:
            continue
        # Receiver = inner_func.value
        receiver = inner_func.child_by_field_name("value")
        if receiver is None or receiver.type != "field_expression":
            # v1: only struct-field receivers `<expr>.<field>`.
            continue
        field_node = receiver.child_by_field_name("field")
        if field_node is None:
            continue
        field_name = _text(field_node, src)
        single_target, blocked = field_targets.get(field_name, (None, True))
        if blocked or single_target is None:
            continue
        # Capture the outer call expression's location for the rewrite anchor.
        line = n.start_point[0] + 1
        # Capture the FULL outer call text — LLM uses this as a unique
        # search anchor (multiple identical callsites in the same fn need
        # context to disambiguate; we leave that to LLM via fn-body prompt).
        call_text = _text(n, src)
        callsites.append({
            "line":          line,
            "receiver":      _text(receiver, src),
            "field":         field_name,
            "target_fn":     single_target,
            "call_text":     call_text,
        })
    if not callsites:
        return None
    # Anchor loc at the FN's start line (rule.apply edits the whole fn).
    fn_start = fn_node.start_point[0] + 1
    return SourcePattern(
        kind="callback_devirt",
        loc=f"{file_path}:{fn_start}",
        details={
            "fn_name":   fn_name,
            "fn_start_line": fn_start,
            "fn_end_line":   fn_node.end_point[0] + 1,
            "callsites": callsites,
        },
        related_rule="D1 callback-devirt",
        confidence="high",
    )


# --- manual_byte_unroll ---------------------------------------------------

def _offset_load_info(stmt: Node, src: bytes
                       ) -> tuple[str, str, str] | None:
    """Return (lhs_id, base, idx) iff stmt is `<id> = *<base>.offset(<idx>);`
    style (also `.add` / `wrapping_offset` / `wrapping_add`)."""
    expr = _unwrap_stmt(stmt)
    if expr is None or expr.type != "assignment_expression":
        return None
    lhs = expr.child_by_field_name("left")
    rhs = expr.child_by_field_name("right")
    if lhs is None or lhs.type != "identifier" or rhs is None:
        return None
    if rhs.type != "unary_expression":
        return None
    op = rhs.child(0)
    if op is None or op.type != "*":
        return None
    inner = rhs.named_child(0) if rhs.named_child_count > 0 else None
    if inner is None or inner.type != "call_expression":
        return None
    func = inner.child_by_field_name("function")
    args = inner.child_by_field_name("arguments")
    if func is None or func.type != "field_expression" or args is None:
        return None
    method = func.child_by_field_name("field")
    base   = func.child_by_field_name("value")
    if method is None or base is None or args.named_child_count == 0:
        return None
    if _text(method, src) not in ("offset", "add", "wrapping_offset", "wrapping_add"):
        return None
    arg0 = args.named_child(0)
    if arg0 is not None and arg0.type == "type_cast_expression":
        arg0 = arg0.named_child(0)
    if arg0 is None:
        return None
    return (_text(lhs, src), _text(base, src), _text(arg0, src))


# Manual-unroll cluster shape:
#   - ≥ MIN_LOADS offset-loads in the same block, with
#   - consecutive loads no more than MAX_INTER_GAP stmts apart, and
#   - no for/while inside the spanning range (i.e. truly flat, not a loop)
# Lower bound 4 matches the minimum unroll factor we'd want to bother
# prefetching for; mainGtU's prefix has 24 loads (12 × 2 ptrs).
_MUR_MIN_LOADS     = 4
_MUR_MAX_INTER_GAP = 5


def _enclosing_fn_param_types(node: Node, src: bytes) -> dict[str, str]:
    """Walk up to the enclosing `function_item` and parse its parameters
    into {param_name: type_text}. Empty dict if no fn / no params.

    Used by C2 SWAR to verify a manual-unroll cluster's base pointer is
    `*mut UChar` / `*mut u8` / `*mut c_char` (i.e. byte-typed) before
    proposing a wide-int rewrite. Type info is only available from the
    fn signature — tree-sitter has no name-resolution layer."""
    fn: Node | None = node
    while fn is not None and fn.type != "function_item":
        fn = fn.parent
    if fn is None:
        return {}
    params = fn.child_by_field_name("parameters")
    if params is None:
        return {}
    out: dict[str, str] = {}
    for c in params.named_children:
        if c.type != "parameter":
            continue
        pat = c.child_by_field_name("pattern")
        typ = c.child_by_field_name("type")
        if pat is None or typ is None:
            continue
        # pat can be `mut_pattern` wrapping identifier, or bare identifier.
        if pat.type == "mut_pattern":
            inner = pat.named_child(0) if pat.named_child_count else None
            if inner is None:
                continue
            pat = inner
        if pat.type != "identifier":
            continue
        out[_text(pat, src)] = _text(typ, src)
    return out


def _emit_manual_unroll(group: list[tuple[int, tuple[str, str, str]]],
                         stmts:    list[Node],
                         src:      bytes,
                         file_path: str,
                         out:      list[SourcePattern]) -> None:
    if len(group) < _MUR_MIN_LOADS:
        return
    first_idx = group[0][0]
    last_idx  = group[-1][0]
    # If any spanned stmt itself contains a for/while, the cluster is
    # not "manual unroll" — fall through to the loop-scoped detectors.
    for i in range(first_idx, last_idx + 1):
        for n in _walk(stmts[i]):
            if n.type in ("for_expression", "while_expression", "loop_expression"):
                return
    bases   = sorted({g[1][1] for g in group})
    indices = sorted({g[1][2] for g in group})
    start_line = stmts[first_idx].start_point[0] + 1
    # Extend cluster end past the last offset-load: the trailing
    # if-return + incr stmts of the last unit weren't picked up by the
    # load-only scan. Walk forward only while the next stmt is either:
    #   - an `if_expression` (the unit's early-return compare), or
    #   - an assignment whose LHS is one of the cluster's INDICES (so
    #     `k = (nblock...)` outside the unroll is NOT swallowed).
    idx_set = set(indices)
    last_unit_idx = last_idx
    while last_unit_idx + 1 < len(stmts):
        nxt = stmts[last_unit_idx + 1]
        nxt_inner = _unwrap_stmt(nxt) or nxt
        if nxt_inner.type == "if_expression":
            last_unit_idx += 1
            continue
        if nxt_inner.type in ("compound_assignment_expr",
                               "assignment_expression"):
            lhs = nxt_inner.child_by_field_name("left")
            if lhs is not None and lhs.type == "identifier" \
               and _text(lhs, src) in idx_set:
                last_unit_idx += 1
                continue
        break
    end_line = stmts[last_unit_idx].end_point[0] + 1
    # unit_count K — number of repeated unroll units. Each unit has
    # len(bases) × len(indices) offset-loads; total loads / per-unit
    # loads gives K. For mainGtU prefix: 24 / (1×2) = 12 units.
    loads_per_unit = max(1, len(bases) * len(indices))
    unit_count = len(group) // loads_per_unit
    # pointed_type — type string of bases[0] from fn-arg signature.
    # Empty when bases[0] is a struct field / local var (no fn-arg
    # match). C2 SWAR only applies when this resolves to a byte type.
    param_types = _enclosing_fn_param_types(stmts[first_idx], src)
    pointed_type = param_types.get(bases[0], "") if bases else ""
    out.append(SourcePattern(
        kind="manual_byte_unroll",
        loc=f"{file_path}:{start_line}",
        details={
            "bases":      bases,           # distinct base pointers (≥ 1)
            "indices":    indices,         # distinct induction vars (≥ 1)
            "load_count": len(group),      # total offset-loads in cluster
            "span_stmts": last_idx - first_idx + 1,
            "start_line": start_line,      # 1-indexed start (mirrors loc)
            "end_line":   end_line,        # 1-indexed last line of cluster
            "unit_count": unit_count,      # K — unroll factor (for C2 K/4 split)
            "pointed_type": pointed_type,  # bases[0] fn-arg type text ("" if N/A)
            # B5 manual-unroll path needs a pointer access form; reuse
            # the slice/raw_ptr nomenclature memory_access_pattern uses.
            "access_form": "raw_ptr",
        },
        related_rule="B5 insert-prefetch (manual-unroll variant)",
        confidence="high",
    ))


def _detect_manual_byte_unroll(fn_body: Node, src: bytes, file_path: str
                                  ) -> list[SourcePattern]:
    """Detect manual byte-loop unroll clusters — sequences of repeated
    `<id> = *<base>.offset(<idx>);` loads that c2rust + clang inline
    expansion leaves as flat statement blocks, with no surrounding
    for/while loop for `_detect_memory_access_pattern` to anchor on.

    Concretely the mainGtU prefix (blocksort.rs:458-541) does:
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 != c2 { return c1 > c2 }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        ... × 12

    These never hit `memory_access_pattern` (loop-anchored) so B5 never
    fires there. We emit one SourcePattern per dense load cluster.
    """
    out: list[SourcePattern] = []
    for block in _walk(fn_body):
        if block.type != "block":
            continue
        stmts = _stmt_list(block)
        if len(stmts) < _MUR_MIN_LOADS:
            continue
        # Collect (stmt_index, load_info) for every offset-load.
        loads: list[tuple[int, tuple[str, str, str]]] = []
        for idx, s in enumerate(stmts):
            info = _offset_load_info(s, src)
            if info is not None:
                loads.append((idx, info))
        if len(loads) < _MUR_MIN_LOADS:
            continue
        # Group loads whose stmt-indices are within MAX_INTER_GAP.
        cur: list[tuple[int, tuple[str, str, str]]] = [loads[0]]
        for nxt in loads[1:]:
            if nxt[0] - cur[-1][0] <= _MUR_MAX_INTER_GAP:
                cur.append(nxt)
            else:
                _emit_manual_unroll(cur, stmts, src, file_path, out)
                cur = [nxt]
        _emit_manual_unroll(cur, stmts, src, file_path, out)
    return out


def _detect_layout_candidates(root: Node, src: bytes, file_path: str
                                ) -> list[SourcePattern]:
    """Walk all top-level struct_item nodes; emit B1 / B2 candidates.

    `repr_c_struct` fires when `#[repr(C)]` is present (B1 drop-repr-c).
    `wide_int_fields` fires when ≥2 fields are 32/64-bit ints (B2
    narrow-int-field — note B2 needs value-range proof, so we mark
    confidence=low here)."""
    out: list[SourcePattern] = []
    for n in _walk(root):
        if n.type != "struct_item":
            continue
        name_node = n.child_by_field_name("name")
        if name_node is None:
            continue
        struct_name = _text(name_node, src)

        # Aggregate any attributes — both preceding-sibling and child positions.
        attrs_text = ""
        prev = n.prev_sibling
        depth = 0
        while prev is not None and depth < 6:
            if prev.type in ("attribute_item", "inner_attribute_item"):
                attrs_text += _text(prev, src) + " "
            elif prev.type in ("line_comment", "block_comment"):
                pass
            else:
                break
            prev = prev.prev_sibling
            depth += 1
        for ch in n.children:
            if ch.type in ("attribute_item", "inner_attribute_item"):
                attrs_text += _text(ch, src) + " "
        has_repr_c = ("repr(C)" in attrs_text
                       or "repr (C)" in attrs_text
                       or "repr(C," in attrs_text)

        body = n.child_by_field_name("body")
        wide_fields: list[str] = []
        if body is not None:
            for fd in body.named_children:
                if fd.type != "field_declaration":
                    continue
                ftype = fd.child_by_field_name("type")
                if ftype is None:
                    continue
                ftype_text = _text(ftype, src)
                if any(t in ftype_text for t in _WIDE_INT_TYPES):
                    fname_node = fd.child_by_field_name("name")
                    if fname_node is not None:
                        wide_fields.append(_text(fname_node, src))

        line = n.start_point[0] + 1
        if has_repr_c:
            out.append(SourcePattern(
                kind="layout_candidate",
                loc=f"{file_path}:{line}",
                details={
                    "type":    struct_name,
                    "pattern": "repr_c_struct",
                    "fields":  wide_fields,
                },
                related_rule="B1 drop-repr-c",
                confidence="medium",
            ))
        if len(wide_fields) >= 2:
            out.append(SourcePattern(
                kind="layout_candidate",
                loc=f"{file_path}:{line}",
                details={
                    "type":    struct_name,
                    "pattern": "wide_int_fields",
                    "fields":  wide_fields,
                },
                related_rule="B2 narrow-int-field",
                confidence="low",   # narrowing needs value-range proof
            ))
    return out


# ---------------------------------------------------------------------------
# fn_signature_facts: scan for `Option<unsafe extern "C" fn(...)>` params
# ---------------------------------------------------------------------------

def _extract_fn_signature_facts(fn_node: Node, src: bytes
                                  ) -> FnSignatureFacts | None:
    """Inspect a `function_item` for the c2rust callback param pattern:
       fn name(... cb: Option<unsafe extern "C" fn(args) -> ret>, ...)

    Returns FnSignatureFacts (with the fn-side fields filled —
    has_option_fn_ptr_param + fn_ptr_param_names). Returns None if the
    fn has no Option-callback params at all (skip storing for noise
    reduction). caller_count + unique_caller_targets are left empty for
    Step C+ (require project-wide callsite scan via sa_iface)."""
    if fn_node.type != "function_item":
        return None
    params = fn_node.child_by_field_name("parameters")
    if params is None:
        return None
    matched: list[str] = []
    for p in params.named_children:
        if p.type != "parameter":
            continue
        # Parameter shape: <pattern> : <type>
        ptype = p.child_by_field_name("type")
        pname_node = p.child_by_field_name("pattern")
        if ptype is None or pname_node is None:
            continue
        type_text = _text(ptype, src)
        # Heuristic match — `Option<` + `extern "C" fn`
        if ("Option<" in type_text
                and ('extern "C" fn' in type_text or "extern\"C\"fn" in type_text)):
            matched.append(_text(pname_node, src))
    if not matched:
        return None
    return FnSignatureFacts(
        has_option_fn_ptr_param = True,
        fn_ptr_param_names      = matched,
        unique_caller_targets   = {},   # Step C+ — needs callsite scan
        caller_count            = 0,    # Step C+ — needs callsite scan
    )


# ---------------------------------------------------------------------------
# Per-fn driver
# ---------------------------------------------------------------------------

def _scan_fn(fn_node: Node, src: bytes, file_path: str,
              *, purable_fns: dict[str, dict] | None = None,
              callback_field_targets: dict[str, tuple[str | None, bool]] | None = None,
              ) -> tuple[list[SourcePattern], FnSignatureFacts | None]:
    """Run every detector on one function body. Returns (patterns, facts).

    `purable_fns` is the project-wide pure-fn index built in pass 1, used
    by Phase H.3 pure_short_call. Pass None when running stand-alone.

    `callback_field_targets` is the project-wide D1 callback field map
    (field_name → (single_target_fn, blocked)). Used by
    `_detect_callback_devirt` to validate single-target at callsites."""
    out: list[SourcePattern] = []

    body = fn_node.child_by_field_name("body")
    if body is None:
        return ([], _extract_fn_signature_facts(fn_node, src))

    # byte_loop family — both `for i in <range>` (idiomatic Rust) and
    # c2rust-style `while <cond> { … i ±= step }` forms.
    # Phase H.1: also emit raw_pointer_loop (broader signal) + loop_alloc_free
    # (alloc inside loop) per for/while loop. Per-loop alloc detection runs
    # in the same pass as the byte loop / cascade walk.
    for n in _walk(body):
        if n.type == "for_expression":
            shape = _decompose_for_range_loop(n, src)
            if shape is not None:
                p = _classify_byte_loop_from_block(shape.body, shape.induction_var, src)
                if p is not None:
                    line = n.start_point[0] + 1
                    p.loc = f"{file_path}:{line}"
                    out.append(p)
            # H.1 raw_pointer_loop — runs even when byte_loop didn't match.
            loop_body = n.child_by_field_name("body")
            if loop_body is not None:
                induction_var = ""
                len_expr = ""
                pat = n.child_by_field_name("pattern")
                if pat is not None and pat.type == "identifier":
                    induction_var = _text(pat, src)
                val = n.child_by_field_name("value")
                if val is not None:
                    if val.type == "range_expression":
                        rc = val.named_children
                        if rc:
                            len_expr = _text(rc[-1], src)
                    else:
                        len_expr = _text(val, src)
                rpp = _aggregate_raw_pointer_loop(
                    n, loop_body, induction_var, len_expr, src, file_path)
                if rpp is not None:
                    out.append(rpp)
            # H.1 loop_alloc_free
            out.extend(_detect_loop_alloc_free(n, src, file_path))

        elif n.type == "while_expression":
            shape = _decompose_while_index_loop(n, src)
            if shape is not None:
                p = _classify_while_byte_loop(shape.body, shape.induction_var, src)
                if p is not None:
                    line = n.start_point[0] + 1
                    p.loc = f"{file_path}:{line}"
                    out.append(p)
            # H.1 raw_pointer_loop on while-loops too.
            loop_body = n.child_by_field_name("body")
            if loop_body is not None:
                induction_var = shape.induction_var if shape is not None else ""
                cond = n.child_by_field_name("condition")
                len_expr = _text(cond, src) if cond is not None else ""
                rpp = _aggregate_raw_pointer_loop(
                    n, loop_body, induction_var, len_expr, src, file_path)
                if rpp is not None:
                    out.append(rpp)
            out.extend(_detect_loop_alloc_free(n, src, file_path))

        elif n.type == "loop_expression":
            # `loop { … }` — no induction var, no len; still scan for allocs.
            out.extend(_detect_loop_alloc_free(n, src, file_path))

        elif n.type in ("if_expression", "match_expression"):
            # Skip nested ifs that are children of another if cascade we
            # already entered — only treat the OUTERMOST if/match as the
            # cascade root.
            par = n.parent
            if par is not None and par.type == "else_clause":
                continue
            cascade = _detect_branch_cascade(n, src)
            if cascade is not None:
                line = n.start_point[0] + 1
                cascade.loc = f"{file_path}:{line}"
                out.append(cascade)
            # H.2: scattered_state_machine_indexing on the OUTERMOST
            # match / chained-if (same root as branch_cascade).
            ssm = _detect_scattered_state_machine_indexing(n, src, file_path)
            if ssm is not None:
                out.append(ssm)
            # H.2: branchless_candidate on every if_expression (not just
            # outer). Skip if-as-cascade-leaf nodes (their parent is else_clause).
            if n.type == "if_expression":
                bc = _detect_branchless_candidate(n, src, file_path)
                if bc is not None:
                    out.append(bc)

    # H.2: memory_access_pattern walks each loop ONCE; appended along
    # with raw_pointer_loop / loop_alloc_free above would re-walk; do
    # it here in a single pass instead.
    for n in _walk(body):
        if n.type in ("for_expression", "while_expression"):
            loop_body = n.child_by_field_name("body")
            if loop_body is None:
                continue
            iv = ""
            if n.type == "for_expression":
                pat = n.child_by_field_name("pattern")
                if pat is not None and pat.type == "identifier":
                    iv = _text(pat, src)
            else:
                shape = _decompose_while_index_loop(n, src)
                if shape is not None:
                    iv = shape.induction_var
            map_pat = _detect_memory_access_pattern(
                n, loop_body, iv, src, file_path)
            if map_pat is not None:
                out.append(map_pat)

    # H.1: libc_mem_calls + const_size_alloc walk the whole fn body once.
    out.extend(_detect_libc_mem_calls(body, src, file_path))
    out.extend(_detect_const_size_alloc(body, src, file_path))
    # H.4 (2026-06-03): manual_byte_unroll — flat-statement clusters
    # of *.offset(idx) loads that the loop-anchored detectors above
    # can't see (mainGtU prefix is 12 byte-compare blocks at fn body
    # top level, not inside any for/while).
    out.extend(_detect_manual_byte_unroll(body, src, file_path))

    # H.3: pure_short_call — needs project-wide purable_fns index built in pass 1.
    if purable_fns:
        out.extend(_detect_pure_short_call(body, src, file_path, purable_fns))

    # D1: callback_devirt — needs project-wide field-target map built in pass 1.
    if callback_field_targets:
        cb_dv = _detect_callback_devirt(fn_node, src, file_path,
                                          callback_field_targets)
        if cb_dv is not None:
            out.append(cb_dv)

    # H.3: checksum_or_hash_kernel — per-fn known-constant scan.
    out.extend(_detect_checksum_kernel(fn_node, src, file_path))

    # E1-bytecmp: zlib LZ77 longest_match canonical structural fingerprint.
    # Only fires on fns named `longest_match` to avoid the false-positive
    # cost of structure-scanning every fn in the project.
    lm = _detect_longest_match_canonical(fn_node, src, file_path)
    if lm is not None:
        out.append(lm)

    facts = _extract_fn_signature_facts(fn_node, src)
    return (out, facts)


# ---------------------------------------------------------------------------
# Top-level project walker
# ---------------------------------------------------------------------------

def _is_rust_src(path: Path) -> bool:
    if path.suffix != ".rs":
        return False
    parts = set(path.parts)
    # Skip target/, vendored deps, integration tests we don't care about.
    return not (parts & {"target", ".git", "vendor"})


def scan_source_patterns(project_path: Path) -> SourcePatternScan:
    """Walk `<project>/src/**/*.rs` (and `<project>/lib.rs`); run every
    detector on every fn. Returns the scan keyed by bare fn name.

    Mirrors `proposer.syn_walker.list_loop_invariant_conds` in scope and
    indexing convention so pipeline.py can slice per-hotspot identically."""
    project_path = Path(project_path).resolve()
    parser = _make_parser()

    scan = SourcePatternScan(project_root=str(project_path))

    # Walk every .rs file under the project (mirrors syn_walker's scope).
    rs_files: list[Path] = []
    for sub in ("src", "lib.rs", "main.rs", ".", "bin"):
        root = project_path / sub
        if root.is_file() and root.suffix == ".rs":
            rs_files.append(root)
        elif root.is_dir():
            rs_files.extend(p for p in root.rglob("*.rs") if _is_rust_src(p))
    # Dedup (e.g. <project>/src/lib.rs shows up via both `src/` and `.`).
    rs_files = sorted({p.resolve() for p in rs_files})

    # ----- Phase 0: parse every file (kept in memory for 2-pass scan) ----
    parsed: list[tuple[Path, bytes, Any]] = []
    for rs in rs_files:
        try:
            src = rs.read_bytes()
        except OSError as e:
            scan.files_parse_failed.append(f"{rs}: {e}")
            continue
        try:
            tree = parser.parse(src)
        except Exception as e:
            scan.files_parse_failed.append(f"{rs}: parse error {e}")
            continue
        scan.files_scanned += 1
        parsed.append((rs, src, tree))

    # ----- Phase H.3 pass-1: build project-wide pure-fn index -----------
    # `purable_fns` maps bare fn name → metadata. Used in pass-2 to fire
    # pure_short_call on callsites of these fns. Same-name collisions
    # (different modules) keep the FIRST match — good enough for C6.
    purable_fns: dict[str, dict] = {}
    for (_rs, _src, _tree) in parsed:
        for n in _walk(_tree.root_node):
            if n.type != "function_item":
                continue
            info = _try_purable(n, _src)
            if info is None:
                continue
            nn = n.child_by_field_name("name")
            if nn is None:
                continue
            fname = _text(nn, _src)
            purable_fns.setdefault(fname, info)

    # ----- Phase D1 pass-1: build callback field → target map -----------
    # `callback_field_targets` maps field_name → (single_target_fn, blocked).
    # Used by `_detect_callback_devirt` in pass-2 to validate single-target
    # at each `<expr>.<field>.expect("non-null function pointer")(...)`
    # callsite found in a fn body. blocked=True poisons the field (any
    # non-literal RHS or multi-target assignment seen anywhere).
    try:
        callback_field_targets = _scan_callback_field_targets_global(parsed)
    except Exception as e:
        logger.warning(f"[source_patterns] callback field scan error: {e}")
        callback_field_targets = {}

    # ----- Phase 2 (main): per-file struct/file-level + per-fn detection
    for rs, src, tree in parsed:
        # Phase H.2: layout_candidate scan per file.
        try:
            layout_pats = _detect_layout_candidates(tree.root_node, src, rs.name)
        except Exception as e:
            logger.warning(f"[source_patterns] {rs}: layout scan error {e}")
            layout_pats = []
        if layout_pats:
            scan.layout_patterns_by_file[rs.name] = layout_pats
        # Phase H.3: hand_written_hashmap file-level scan.
        try:
            hwh = _scan_hashmap_file(tree.root_node, src, rs.name)
        except Exception as e:
            logger.warning(f"[source_patterns] {rs}: hashmap scan error {e}")
            hwh = None
        if hwh is not None:
            scan.hashmap_patterns_by_file.setdefault(rs.name, []).append(hwh)
        # Phase H.3: file-level checksum_or_hash_kernel from static tables.
        try:
            file_csum = _detect_file_static_checksum(tree.root_node, src, rs.name)
        except Exception as e:
            logger.warning(f"[source_patterns] {rs}: static-table checksum error {e}")
            file_csum = []
        if file_csum:
            scan.checksum_patterns_by_file.setdefault(rs.name, []).extend(file_csum)
        # Walk every function_item at any nesting depth.
        for n in _walk(tree.root_node):
            if n.type != "function_item":
                continue
            name_node = n.child_by_field_name("name")
            if name_node is None:
                continue
            fn_name = _text(name_node, src)
            try:
                patterns, facts = _scan_fn(n, src, rs.name,
                                             purable_fns=purable_fns,
                                             callback_field_targets=callback_field_targets)
            except Exception as e:
                logger.warning(f"[source_patterns] {rs}:{fn_name}: detector error {e}")
                continue
            if patterns:
                scan.patterns_by_fn[fn_name].extend(patterns)
            if facts is not None:
                # Merge with prior facts (same fn name may occur in
                # multiple modules; carry the broadest evidence).
                prior = scan.facts_by_fn.get(fn_name)
                if prior is None:
                    scan.facts_by_fn[fn_name] = facts
                else:
                    prior.fn_ptr_param_names = sorted({
                        *prior.fn_ptr_param_names, *facts.fn_ptr_param_names
                    })
                    prior.has_option_fn_ptr_param = True

    logger.info(f"[source_patterns] {scan.files_scanned} file(s); "
                f"{sum(len(v) for v in scan.patterns_by_fn.values())} "
                f"pattern(s) across {len(scan.patterns_by_fn)} fn(s); "
                f"{len(scan.facts_by_fn)} fn(s) with callback signature")
    return scan
