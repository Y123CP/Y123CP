"""Stage A — argument hoisting to resolve `&mut` view borrow conflicts (E0503).

After a `*mut T → &mut T` view lift, a call like

    bsW(unsafe { &mut *s_view }, (*s_view).len[(*s_view).selector[i]], ...);
        ^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        mutable borrow of view   read of view in a LATER argument

is rejected by the borrow checker (E0503): the `&mut *s_view` borrow is live
while a sibling argument reads `s_view`. Raw pointers had no borrow checker so
this compiled; `&mut` does not.

Fix: hoist the reading arguments into `let __hb_… = …;` bindings BEFORE the
call statement, so the reads are sequenced ahead of the borrow:

    let __hb_0_1 = (*s_view).len[(*s_view).selector[i]];
    bsW(unsafe { &mut *s_view }, __hb_0_1, ...);

Soundness: the reborrow arg `&mut *view` only CREATES a reference — it runs no
code and has no side effects (the mutation happens inside the callee, after
ALL args are evaluated). So moving the reading args ahead of it cannot change
observable behaviour, even if a read contains a (pure) `.offset()` call. The
reading args keep their relative order. cargo check + W1 still gate the result.
"""
from __future__ import annotations

import re
from pathlib import Path

from .intra_ptr.collect import _PARSER, _txt

# `(unsafe { )? & (mut )? * <view> ( })?` — a reborrow of a view.
_REBORROW = re.compile(
    r"^\s*(?:unsafe\s*\{)?\s*&\s*(?:mut\s+)?\*\s*([A-Za-z_][A-Za-z0-9_]*)\s*\}?\s*$")


def _find_fn(root, src: bytes, name: str):
    st = [root]
    while st:
        n = st.pop()
        st.extend(n.children)
        if n.type == "function_item":
            nm = next((c for c in n.children if c.type == "identifier"), None)
            if nm is not None and _txt(src, nm) == name:
                return n
    return None


def _walk(n):
    st = [n]
    while st:
        x = st.pop()
        st.extend(x.children)
        yield x


def _is_reborrow(src: bytes, arg, view_set: set[str]) -> bool:
    m = _REBORROW.match(_txt(src, arg))
    return bool(m) and m.group(1) in view_set


def _reads_view(src: bytes, arg, view_set: set[str]) -> bool:
    """arg mentions a view name as an identifier read (and is not itself a
    reborrow)."""
    if _is_reborrow(src, arg, view_set):
        return False
    return any(n.type == "identifier" and _txt(src, n) in view_set
               for n in _walk(arg))


def _has_call(arg) -> bool:
    return any(n.type in ("call_expression", "macro_invocation",
                          "await_expression") for n in _walk(arg))


def _enclosing_statement(node):
    """The ancestor whose parent is a `block` (i.e. the statement holding the
    call). None if the call isn't directly inside a block-level statement."""
    cur = node
    while cur.parent is not None:
        if cur.parent.type == "block":
            return cur
        cur = cur.parent
    return None


def _indent_of(src: bytes, byte_off: int) -> str:
    line_start = src.rfind(b"\n", 0, byte_off) + 1
    ws = []
    i = line_start
    while i < byte_off and src[i:i + 1] in (b" ", b"\t"):
        ws.append(src[i:i + 1])
        i += 1
    return b"".join(ws).decode()


def hoist_conflicting_args(file_path: Path, fn_name: str,
                           view_names: list[str]) -> int:
    """Hoist pure view-reading args out of calls that also pass a view
    reborrow. Returns the number of calls rewritten."""
    src = file_path.read_bytes()
    root = _PARSER.parse(src).root_node
    fn = _find_fn(root, src, fn_name)
    if fn is None:
        return 0
    view_set = set(view_names)
    edits: list[tuple[int, int, bytes]] = []   # (start, end, replacement)
    n_calls = 0
    for call in _walk(fn):
        if call.type != "call_expression":
            continue
        args_node = call.child_by_field_name("arguments")
        if args_node is None:
            continue
        args = [a for a in args_node.named_children]
        if not any(_is_reborrow(src, a, view_set) for a in args):
            continue
        reading = [a for a in args if _reads_view(src, a, view_set)]
        if not reading:
            continue
        stmt = _enclosing_statement(call)
        if stmt is None:
            continue
        indent = _indent_of(src, stmt.start_byte)
        lets: list[str] = []
        for i, a in enumerate(reading):
            tmp = f"__hb_{n_calls}_{i}"
            lets.append(f"let {tmp} = {_txt(src, a)};")
            edits.append((a.start_byte, a.end_byte, tmp.encode()))
        prefix = ("\n" + indent).join(lets) + "\n" + indent
        edits.append((stmt.start_byte, stmt.start_byte, prefix.encode()))
        n_calls += 1
    if not edits:
        return 0
    out = bytearray(src)
    for s, e, r in sorted(edits, key=lambda x: x[0], reverse=True):
        out[s:e] = r
    file_path.write_bytes(bytes(out))
    return n_calls
