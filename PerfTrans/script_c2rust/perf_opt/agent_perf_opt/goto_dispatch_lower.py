"""C11 — narrow c2rust's goto-lowering state constants.

C has `goto`; Rust does not. c2rust lowers a function's gotos into a state
machine: `let mut current_block: u64` holding an opaque 64-bit label, and
`match current_block { <const> => .. }` blocks routing to whatever the goto
targeted. Every comparison against one of those labels needs its own 10-byte
`movabs` to materialise the immediate.

This module renumbers the labels to small dense integers and narrows the
variable's type. **No control flow moves** — the only bytes that change are
integer literals and one type keyword — so the rewrite is value-preserving by
construction: a bijection applied to every read and every write of the same
variable.

Measured 2026-08-29, miniz `tinfl_decompress`: 81 labels, `movabs` 256 -> 6,
the five inflate operations went from +22.42% against C to +7.85% (-14.57pp),
oracle 14/14 and W1 golden 1200/1200 clean.

It does not always pay. LLVM threads the dispatch away in some functions, and
then the `movabs` were never on the hot path to begin with: the same rewrite
left lz4 `LZ4_decompress_generic` and http-parser `http_parser_execute`
bit-identical in instruction count. Nothing in the source distinguishes the two
cases, so the W2 gate decides.

Everything here works on byte ranges taken from the CST. Two hand-written
attempts that selected constants by *width* instead both corrupted the source:
`http_parser_execute` arms a second `match` on `(method << 16) | (index << 8) |
ch` whose labels (`196929`, `1311298`, …) are as wide as real ones, and holds a
`9223372036854775807` (i64::MAX) used in a comparison. Renaming by width broke
3 of its 5 operations. A literal is a label here only if it sits in an
assignment to the state variable or in an arm pattern of a `match` on that same
variable.
"""

from __future__ import annotations

import re
from dataclasses import dataclass

_VAR_RE = re.compile(r"^current_block(?:_\d+)?$")
_WIDE = 1 << 32
# Namespaces are kept far apart so a reader can tell at a glance which variable
# a label belongs to; the gap costs nothing since these are compare immediates.
_NAMESPACE_STRIDE = 1000


class UnsupportedGotoDispatch(Exception):
    """The function does not carry a lowerable dispatch."""


@dataclass(frozen=True)
class NarrowedDispatch:
    """One function's rewritten source plus what the rewrite touched."""
    text: str
    function_source: str
    var_states: dict[str, int]
    literals_rewritten: int
    types_narrowed: int
    wide_before: int

    @property
    def states(self) -> int:
        return sum(self.var_states.values())


def _parse(source: bytes):
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    return _parse_rust_source(source)


def _int_of(node, source: bytes):
    """An `integer_literal`'s value, or None. Handles `1_000`, `0xff`, `7u32`."""
    text = source[node.start_byte:node.end_byte].decode("ascii", "ignore")
    text = text.replace("_", "")
    m = re.match(r"^(0[xXoObB])?([0-9a-fA-F]+)", text)
    if not m:
        return None
    base = {None: 10, "0x": 16, "0X": 16, "0o": 8, "0O": 8, "0b": 2, "0B": 2}
    try:
        return int(m.group(2), base.get(m.group(1), 10))
    except ValueError:
        return None


def _is_state_var(name: bytes) -> bool:
    return bool(_VAR_RE.match(name.decode("ascii", "ignore")))


def _walk(root, lo: int, hi: int):
    """Nodes whose start BYTE falls inside [lo, hi).

    Byte offsets, not line numbers. The caller's line range is a hint recorded
    when the run started; by the time a rule executes, earlier commits have
    moved the file under it. A stale window collects part of one function and
    part of its neighbour — measured on a compression crate, that narrowed a
    declaration while leaving assignments beyond the window untouched. The
    build caught it there only because the type had been narrowed too; had it
    not been, the result would have compiled and routed the state machine
    wrongly.
    """
    stack = [root]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if lo <= n.start_byte < hi:
            yield n


def _byte_span(source: bytes, fn_start: int, fn_end: int,
               span: tuple[int, int] | None) -> tuple[int, int]:
    """The authoritative byte range: the caller's span when it has one."""
    if span is not None:
        return span
    lines = source.split(b"\n")
    lo = sum(len(l) + 1 for l in lines[:fn_start - 1])
    hi = lo + sum(len(l) + 1 for l in lines[fn_start - 1:fn_end])
    return lo, min(hi, len(source))


def _arm_literals(match_node):
    """`integer_literal` nodes in this match's arm PATTERNS, `A | B` included.

    Arm patterns only. An integer in an arm's body is ordinary program data;
    rewriting it would change what the program computes.
    """
    body = match_node.child_by_field_name("body")
    if body is None:
        return
    for arm in body.children:
        if arm.type != "match_arm":
            continue
        pat = arm.child_by_field_name("pattern")
        if pat is None:
            continue
        stack = [pat]
        while stack:
            n = stack.pop()
            if n.type == "integer_literal":
                yield n
                continue
            stack.extend(n.children)


def collect(source: bytes, lo: int, hi: int):
    """Every label site in one function, grouped by state variable.

    Returns (declared_vars, sites) where `sites` maps a variable name to the
    list of (node, value) pairs that must be renumbered together — assignments
    and arm patterns alike. Declaration order is preserved so numbering is
    deterministic across runs.
    """
    tree = _parse(source)
    declared: list[bytes] = []
    decls: dict[bytes, object] = {}
    sites: dict[bytes, list] = {}

    for n in _walk(tree.root_node, lo, hi):
        if n.type == "let_declaration":
            pat = n.child_by_field_name("pattern")
            if pat is not None and pat.type == "identifier":
                name = source[pat.start_byte:pat.end_byte]
                if _is_state_var(name) and name not in declared:
                    declared.append(name)
                    decls[name] = n
            continue
        if n.type == "assignment_expression":
            lhs = n.child_by_field_name("left")
            rhs = n.child_by_field_name("right")
            if (lhs is not None and rhs is not None
                    and lhs.type == "identifier"
                    and rhs.type == "integer_literal"):
                name = source[lhs.start_byte:lhs.end_byte]
                if _is_state_var(name):
                    v = _int_of(rhs, source)
                    if v is not None:
                        sites.setdefault(name, []).append((rhs, v))
            continue
        if n.type == "match_expression":
            val = n.child_by_field_name("value")
            if val is None or val.type != "identifier":
                continue
            name = source[val.start_byte:val.end_byte]
            if not _is_state_var(name):
                continue
            for lit in _arm_literals(n):
                v = _int_of(lit, source)
                if v is not None:
                    sites.setdefault(name, []).append((lit, v))
    return declared, decls, sites


def narrow(source: bytes, fn_start: int, fn_end: int,
           span: tuple[int, int] | None = None) -> NarrowedDispatch:
    """Renumber one function's goto labels; return the rewritten whole source.

    `span` is the byte range the caller intends to splice — the changeset
    handler works on byte spans that include the function's preceding
    attributes, and a renumbered label is shorter than the one it replaces, so
    the end offset moves. When given, `function_source` is that span rewritten;
    otherwise it is the [fn_start, fn_end] line range.

    Raises `UnsupportedGotoDispatch` when there is nothing to gain — fewer than
    two labels for a variable (a flag, not a dispatch), or no label wide enough
    to have cost a `movabs` in the first place.
    """
    lo_b, hi_b = _byte_span(source, fn_start, fn_end, span)
    declared, decls, sites = collect(source, lo_b, hi_b)
    if not declared:
        raise UnsupportedGotoDispatch("no c2rust state variable in range")
    # Declaration order, for the same reason the labels follow source order:
    # the CST walk is depth-first over a LIFO stack, so without this the first
    # variable in the file could be handed the second namespace.
    declared.sort(key=lambda v: decls[v].start_byte)

    edits: list[tuple[int, int, bytes]] = []
    var_states: dict[str, int] = {}
    wide_before = 0
    types_narrowed = 0

    for idx, var in enumerate(declared):
        # Source order, so label 0 is the first one the reader meets and a
        # diff can be checked by eye. The CST walk is depth-first over a LIFO
        # stack, which is deterministic but not source order; sorting here is
        # what makes the numbering both reviewable and reproducible.
        pairs = sorted(sites.get(var, []), key=lambda pv: pv[0].start_byte)
        order: list[int] = []
        for _node, value in pairs:
            if value not in order:
                order.append(value)
        if len(order) < 2:
            continue
        wide = sum(1 for v in order if v >= _WIDE)
        if wide < 2:
            # Comparing against small labels already uses an imm32; renumbering
            # would change bytes without removing a single `movabs`.
            continue
        wide_before += wide
        base = idx * _NAMESPACE_STRIDE
        mapping = {v: base + i for i, v in enumerate(order)}
        for node, value in pairs:
            edits.append((node.start_byte, node.end_byte,
                          str(mapping[value]).encode("ascii")))
        var_states[var.decode()] = len(order)

        # Narrow the carrier only when every new label fits. The type is only
        # ever compared and assigned, never stored or passed, so its width is
        # free to change.
        if base + len(order) - 1 < _WIDE:
            decl = decls.get(var)
            ty = decl.child_by_field_name("type") if decl is not None else None
            if ty is not None and source[ty.start_byte:ty.end_byte] == b"u64":
                edits.append((ty.start_byte, ty.end_byte, b"u32"))
                types_narrowed += 1

    if not var_states:
        raise UnsupportedGotoDispatch(
            "no state variable with >= 2 labels of which >= 2 need a movabs")

    # De-duplicate defensively: a literal reached through two paths must not be
    # spliced twice. Apply right-to-left so earlier offsets stay valid.
    seen: set[int] = set()
    unique: list[tuple[int, int, bytes]] = []
    for start, end, text in edits:
        if start in seen:
            continue
        seen.add(start)
        unique.append((start, end, text))
    unique.sort(key=lambda e: e[0], reverse=True)

    out = bytearray(source)
    for start, end, text in unique:
        out[start:end] = text
    rewritten = bytes(out).decode("utf-8")
    if span is not None:
        lo, hi = span
        piece = bytearray(source[lo:hi])
        for start, end, text in unique:
            if start < lo or end > hi:
                # An edit outside the span the caller will splice would be lost
                # silently, leaving one half of a bijection applied.
                raise UnsupportedGotoDispatch(
                    "label site lies outside the requested span")
            piece[start - lo:end - lo] = text
        fn_src = bytes(piece).decode("utf-8")
    else:
        piece = bytearray(source[lo_b:hi_b])
        for start, end, text in unique:
            piece[start - lo_b:end - lo_b] = text
        fn_src = bytes(piece).decode("utf-8")
    return NarrowedDispatch(
        text=rewritten,
        function_source=fn_src,
        var_states=var_states,
        literals_rewritten=len(unique) - types_narrowed,
        types_narrowed=types_narrowed,
        wide_before=wide_before,
    )


# ── Splitting a merged loop ────────────────────────────────────────────────────
#
# The other cost the same lowering carries. When a dispatch is the first
# statement of a loop body, the loop's own back edge re-runs it every
# iteration: the translator merged what were separate C loops into one
# `loop { match .. }`, and the hot arm's `continue` now has to route through the
# comparison chain to get back to where it already was.
#
# Giving the hot arm its own loop takes the dispatch off that edge:
#
#     loop {                            'outer: loop {
#         match v {                         if v == L {
#             L => { BODY }        ->           'fast: loop { BODY break 'fast; }
#             _ => { REST }                 } else { REST }
#         }
#         TAIL                              TAIL
#     }                                 }
#
# The trailing `break 'fast;` is what makes this safe without reasoning about
# which states can be live at the exit: it reproduces the original edge — fall
# off the end of the arm, continue past the dispatch — exactly.
#
# Keeping the `if` INSIDE the outer loop is deliberate. Re-entering the arm from
# elsewhere still works, because the condition is re-evaluated at the top of
# every outer iteration; hoisting the fast loop above the outer one instead
# would need a proof that nothing else ever assigns the hot label.
#
# The hazard is `break` and `continue`. Inside the arm they targeted the merged
# loop; once the arm is a loop of its own they would target that instead. Only
# the ones at the state machine's own nesting level may be relabelled — an arm
# body may contain its own `while`, and a `break` in there belongs to the
# `while`. Relabelling it is a silent semantic change that a replay corpus need
# not reach.
_MAX_ARMS_FOR_SPLIT = 2


@dataclass(frozen=True)
class SplitLoop:
    text: str
    function_source: str
    label: int
    relabelled_breaks: int
    relabelled_continues: int


def _loop_is_unlabelled(loop_node) -> bool:
    return not any(c.type == "label" for c in loop_node.children)


def _statement_of(node):
    """Walk out of `expression_statement` wrappers to the statement node."""
    stmt = node
    while stmt.parent is not None and stmt.parent.type == "expression_statement":
        stmt = stmt.parent
    return stmt


def _dispatch_at_loop_head(root, lo: int, hi: int, source: bytes):
    """The first `loop { match <state_var> { L => .., _ => .. } .. }` in range.

    Returns (loop_node, match_node, arm_hot, arm_rest, label) or None. Anything
    with a shape the rewrite cannot express — a labelled loop, more than two
    arms, an arm whose pattern is not one integer literal, an arm body that is
    not a block — is declined here rather than half-handled later.
    """
    best = None
    for n in _walk(root, lo, hi):
        if n.type != "loop_expression" or not _loop_is_unlabelled(n):
            continue
        body = n.child_by_field_name("body")
        if body is None or body.type != "block":
            continue
        named = [c for c in body.children if c.is_named]
        if not named:
            continue
        first = named[0]
        inner = first
        if inner.type == "expression_statement":
            kids = [c for c in inner.children if c.is_named]
            if not kids:
                continue
            inner = kids[0]
        if inner.type != "match_expression":
            continue
        val = inner.child_by_field_name("value")
        if val is None or val.type != "identifier":
            continue
        if not _is_state_var(source[val.start_byte:val.end_byte]):
            continue
        mb = inner.child_by_field_name("body")
        if mb is None:
            continue
        arms = [c for c in mb.children if c.type == "match_arm"]
        if len(arms) != _MAX_ARMS_FOR_SPLIT:
            continue
        hot, rest = arms
        hot_pat = hot.child_by_field_name("pattern")
        rest_pat = rest.child_by_field_name("pattern")
        if hot_pat is None or rest_pat is None:
            continue
        lits = [c for c in hot_pat.children if c.type == "integer_literal"]
        if len(lits) != 1:
            continue
        if source[rest_pat.start_byte:rest_pat.end_byte].strip() != b"_":
            continue
        hot_body = hot.child_by_field_name("value")
        rest_body = rest.child_by_field_name("value")
        if (hot_body is None or hot_body.type != "block"
                or rest_body is None or rest_body.type != "block"):
            continue
        # A comma between the arms would survive into `if { .. } else`, where it
        # is a syntax error.
        between = source[hot_body.end_byte:rest.start_byte]
        if between.strip():
            continue
        label = _int_of(lits[0], source)
        if label is None:
            continue
        if best is None or n.start_byte < best[0].start_byte:
            best = (n, inner, hot, rest, label)
    return best


def _machine_level_jumps(arm_body):
    """`break`/`continue` in this arm that belong to the state machine's loop.

    Excluded: anything already carrying a label (its target is explicit and
    must not move), and anything inside an inner `loop`/`while`/`for` — that one
    targets the inner loop, and relabelling it would change where it goes.
    """
    out: list = []
    stack = [(c, 0) for c in arm_body.children]
    while stack:
        node, depth = stack.pop()
        if node.type in ("loop_expression", "while_expression", "for_expression"):
            stack.extend((c, depth + 1) for c in node.children)
            continue
        if node.type in ("break_expression", "continue_expression"):
            if depth == 0 and not any(c.type == "label" for c in node.children):
                out.append(node)
            continue
        stack.extend((c, depth) for c in node.children)
    return out


def _preceded_by_hot_assignment(node, source: bytes, var: bytes,
                                label: int) -> bool:
    """The statement right before this jump sets the state variable to `label`.

    Only then may a `continue` be shortened to `continue 'fast` — otherwise the
    original would have gone round the dispatch and possibly landed in the other
    arm, and only `continue 'outer` reproduces that.
    """
    stmt = _statement_of(node)
    parent = stmt.parent
    if parent is None:
        return False
    named = [c for c in parent.children if c.is_named]
    try:
        idx = next(i for i, c in enumerate(named)
                   if c.start_byte == stmt.start_byte)
    except StopIteration:
        return False
    if idx == 0:
        return False
    prev = named[idx - 1]
    text = source[prev.start_byte:prev.end_byte].strip()
    return text == var + b" = " + str(label).encode() + b";"



def _line_indent(source: bytes, byte_off: int) -> bytes:
    """The leading whitespace of the line `byte_off` sits on."""
    line_start = source.rfind(b"\n", 0, byte_off) + 1
    i = line_start
    while i < len(source) and source[i:i + 1] in (b" ", b"\t"):
        i += 1
    return source[line_start:i]


def _line_start(source: bytes, byte_off: int) -> int:
    return source.rfind(b"\n", 0, byte_off) + 1


def _swallow_blank_line(source: bytes, start: int, end: int) -> tuple[int, int]:
    """Widen a deletion to the whole line when nothing else is on it.

    Deleting the match's closing brace otherwise leaves an empty line behind in
    the middle of the function it just rewrote.
    """
    ls = _line_start(source, start)
    if source[ls:start].strip():
        return start, end
    nl = source.find(b"\n", end)
    if nl == -1 or source[end:nl].strip():
        return start, end
    return ls, nl + 1


def split_loop_head(source: bytes, fn_start: int, fn_end: int,
                    span: tuple[int, int] | None = None) -> SplitLoop:
    """Give the hot arm of a loop-head dispatch its own loop.

    Raises `UnsupportedGotoDispatch` when the function carries no dispatch at a
    loop head, or carries one in a shape this rewrite cannot express.
    """
    lo_b, hi_b = _byte_span(source, fn_start, fn_end, span)
    tree = _parse(source)
    found = _dispatch_at_loop_head(tree.root_node, lo_b, hi_b, source)
    if found is None:
        raise UnsupportedGotoDispatch("no two-arm dispatch heads an unlabelled loop")
    loop_node, match_node, arm_hot, arm_rest, label = found

    hot_body = arm_hot.child_by_field_name("value")
    match_block = match_node.child_by_field_name("body")
    var = source[match_node.child_by_field_name("value").start_byte:
                 match_node.child_by_field_name("value").end_byte]

    # A label we introduce must not collide with one already in the arm.
    for n in _walk(hot_body, 0, 1 << 62):
        if n.type == "label":
            name = source[n.start_byte:n.end_byte]
            if name in (b"'fast", b"'outer"):
                raise UnsupportedGotoDispatch("label name already in use")

    # Indentation is taken from the code being replaced, so the rewritten
    # function reads like the rest of the file rather than announcing itself.
    match_indent = _line_indent(source, match_node.start_byte)
    arm_indent = _line_indent(source, arm_hot.start_byte)
    close_indent = _line_indent(source, hot_body.end_byte - 1)

    edits: list[tuple[int, int, bytes]] = []
    edits.append((loop_node.start_byte, loop_node.start_byte, b"'outer: "))
    edits.append((match_node.start_byte, match_block.start_byte + 1,
                  b"if " + var + b" == " + str(label).encode() + b" {"))
    edits.append((arm_hot.start_byte, hot_body.start_byte + 1,
                  b"'fast: loop {"))
    edits.append((hot_body.end_byte - 1, hot_body.end_byte - 1,
                  b"    break 'fast;\n" + close_indent))
    # `}` closes the `if`, so it belongs at the `if`'s indentation, not the
    # arm's.
    rest_body = arm_rest.child_by_field_name("value")
    edits.append((_line_start(source, arm_rest.start_byte),
                  rest_body.start_byte + 1,
                  match_indent + b"} else {"))
    edits.append(_swallow_blank_line(
        source, match_block.end_byte - 1, match_block.end_byte) + (b"",))

    breaks = conts = 0
    for jump in _machine_level_jumps(hot_body):
        if jump.type == "break_expression":
            edits.append((jump.start_byte, jump.end_byte, b"break 'outer"))
            breaks += 1
        else:
            target = (b"continue 'fast"
                      if _preceded_by_hot_assignment(jump, source, var, label)
                      else b"continue 'outer")
            edits.append((jump.start_byte, jump.end_byte, target))
            conts += 1

    edits.sort(key=lambda e: (e[0], e[1]), reverse=True)
    out = bytearray(source)
    for start, end, text in edits:
        out[start:end] = text
    rewritten = bytes(out).decode("utf-8")

    if span is not None:
        lo, hi = span
        for start, end, _t in edits:
            if start < lo or end > hi:
                raise UnsupportedGotoDispatch("edit lies outside the requested span")
        piece = bytearray(source[lo:hi])
        for start, end, text in edits:
            piece[start - lo:end - lo] = text
        fn_src = bytes(piece).decode("utf-8")
    else:
        piece = bytearray(source[lo_b:hi_b])
        for start, end, text in edits:
            piece[start - lo_b:end - lo_b] = text
        fn_src = bytes(piece).decode("utf-8")

    return SplitLoop(text=rewritten, function_source=fn_src, label=label,
                     relabelled_breaks=breaks, relabelled_continues=conts)
