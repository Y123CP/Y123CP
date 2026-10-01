"""Stage A intra_ptr — deterministic body rewriter (P2a).

Given a signature-frozen lift plan for ONE pointer param, produce the byte
edits that re-express its body deref/offset uses as a local safe view:

  Scalar  `&T`:    insert  `let p_view: &T = unsafe { &*p };`
                   rewrite `*p` / `(*p).f`  →  `*p_view` / `(*p_view).f`
  Array   `&[T]`:  insert  `let p_view: &[T] =
                              unsafe { core::slice::from_raw_parts(p, (len) as usize) };`
                   rewrite `*p.offset(i)`   →  `p_view[(i) as usize]`

The signature is NOT touched (param stays `*mut T`); the unsafe is confined
to the one view-construction point and every use becomes safe. The rewriter
re-scans the function fresh (it does NOT trust COLLECT's byte offsets, which
go stale after edits) — same discipline as e2_v2_slice / e_array_unchecked.

Correctness is NOT assumed here: the driver (apply.py) gates every rewrite
on cargo check + W1 and rolls back per-fn on failure.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from tree_sitter import Node
from .collect import _PARSER, _txt, _child, _strip_cast


@dataclass
class Edit:
    start: int
    end: int
    text: str


def _view_name(anchor: str) -> str:
    return f"{anchor}_view"


def _find_fn(root: Node, src: bytes, fn_name: str) -> Node | None:
    stack = [root]
    while stack:
        n = stack.pop()
        if n.type == "function_item":
            nm = _child(n, "identifier")
            if nm is not None and _txt(src, nm) == fn_name:
                return n
        stack.extend(n.children)
    return None


def _deref_operand_sites(src: bytes, body: Node, anchor: str) -> list[Node]:
    """Identifier nodes == anchor that are the operand of a `*` deref
    (`*anchor` / `(*anchor).f`). These are the scalar rewrite targets."""
    out: list[Node] = []
    stack = [body]
    while stack:
        n = stack.pop()
        if n.type == "identifier" and _txt(src, n) == anchor:
            p = n.parent
            if p is not None and p.type == "unary_expression" \
                    and _txt(src, p).lstrip().startswith("*"):
                out.append(n)
        stack.extend(n.children)
    return out


def _offset_deref_sites(src: bytes, body: Node,
                        anchor: str) -> list[tuple[Node, str]]:
    """`*anchor.offset(i)` whole-unary nodes + their index expr. Array
    rewrite targets."""
    out: list[tuple[Node, str]] = []
    stack = [body]
    while stack:
        n = stack.pop()
        if n.type == "identifier" and _txt(src, n) == anchor:
            p = n.parent
            # anchor.offset / anchor.add  →  field_expression(value=anchor)
            if p is not None and p.type == "field_expression":
                fld = p.child_by_field_name("field")
                val = p.child_by_field_name("value")
                if (val is not None and val.start_byte == n.start_byte
                        and fld is not None and _txt(src, fld) in ("offset", "add")):
                    call = p.parent
                    if call is not None and call.type == "call_expression":
                        deref = call.parent
                        if deref is not None and deref.type == "unary_expression" \
                                and _txt(src, deref).lstrip().startswith("*"):
                            args = call.child_by_field_name("arguments")
                            named = [c for c in args.children if c.is_named] if args else []
                            idx = _strip_cast(_txt(src, named[0])) if named else ""
                            out.append((deref, idx))
        stack.extend(n.children)
    return out


def build_fn_edits(src: bytes, fn_node: Node, anchor: str,
                   target_view: str, length: str | None) -> list[Edit] | None:
    """Edits for ONE pointer's signature-frozen view lift. Returns None if
    the view name collides with an existing identifier or nothing to do."""
    body = _child(fn_node, "block")
    if body is None:
        return None
    vname = _view_name(anchor)
    # Collision guard: bail if `<anchor>_view` already appears in the fn.
    if vname.encode() in src[fn_node.start_byte:fn_node.end_byte]:
        return None

    edits: list[Edit] = []
    is_mut = target_view.lstrip().startswith("&mut")
    is_array = "[" in target_view

    if is_array:
        if not length:
            return None
        sites = _offset_deref_sites(src, body, anchor)
        if not sites:
            return None
        frp = "from_raw_parts_mut" if is_mut else "from_raw_parts"
        decl = (f"\n    let {vname}: {target_view} = unsafe {{ "
                f"core::slice::{frp}({anchor}, ({length}) as usize) }};")
        for deref_node, idx in sites:
            edits.append(Edit(deref_node.start_byte, deref_node.end_byte,
                              f"{vname}[({idx}) as usize]"))
    else:
        sites = _deref_operand_sites(src, body, anchor)
        if not sites:
            return None
        ref = "&mut *" if is_mut else "&*"
        decl = f"\n    let {vname}: {target_view} = unsafe {{ {ref}{anchor} }};"
        for idn in sites:
            edits.append(Edit(idn.start_byte, idn.end_byte, vname))

    # Insert the view decl right after the body's opening brace.
    brace = body.start_byte            # points at `{`
    edits.append(Edit(brace + 1, brace + 1, decl))
    return edits


def apply_edits(src_text: str, edits: list[Edit]) -> str:
    """Apply byte edits to text, highest-offset first (so earlier offsets
    stay valid). Insertions (start==end) and replacements both handled."""
    for e in sorted(edits, key=lambda x: -x.start):
        src_text = src_text[:e.start] + e.text + src_text[e.end:]
    return src_text


def rewrite_fn_in_file(file_path: Path, fn_name: str, anchor: str,
                       target_view: str, length: str | None) -> bool:
    """Re-scan `file_path`, build + apply the lift edits for `fn_name`'s
    `anchor` pointer. Returns True if the file was modified."""
    src = file_path.read_bytes()
    root = _PARSER.parse(src).root_node
    fn_node = _find_fn(root, src, fn_name)
    if fn_node is None:
        return False
    edits = build_fn_edits(src, fn_node, anchor, target_view, length)
    if not edits:
        return False
    new_text = apply_edits(src.decode("utf-8", errors="replace"), edits)
    file_path.write_text(new_text, encoding="utf-8")
    return True
