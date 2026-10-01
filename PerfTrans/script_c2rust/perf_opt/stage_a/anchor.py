"""Stage A shared — view construction-point rule (design local_lift §5).

A `&*p` / `from_raw_parts(p, …)` view ASSERTS p's validity at the
construction point. Constructing earlier than the original code's first
deref is a hidden re-derive (the original may only deref conditionally),
which breaks the re-express principle. This module computes the earliest
byte position that is NOT earlier than that assertion:

  1. early-return null guard   → right after the guard `if` statement
  2. guard-block (all derefs inside) → top of the protected block
  3. no guard, all derefs reachable from one block → after the decl if the
     decl's block contains every deref; otherwise immediately before the
     statement containing the FIRST deref, provided every other deref sits
     at-or-after that statement within the same or an inner block
  4. anything else (derefs split across exclusive branches, guard that
     doesn't cover, …) → None — the caller must SKIP

Approximation is block-containment on the tree-sitter CST (no dominator
tree). When in doubt it returns None; a skip is always sound.
"""

from __future__ import annotations

from tree_sitter import Node


def _stmt_of(node_start: int, block: Node) -> Node | None:
    """The direct child statement of `block` that contains byte `node_start`."""
    for c in block.named_children:
        if c.start_byte <= node_start < c.end_byte:
            return c
    return None


def _block_containing(root: Node, pos: int, kind: str = "block") -> Node | None:
    """Innermost `block` node containing byte `pos`."""
    best: Node | None = None
    stack = [root]
    while stack:
        n = stack.pop()
        if n.start_byte <= pos < n.end_byte:
            if n.type == kind:
                if best is None or n.start_byte >= best.start_byte:
                    best = n
            stack.extend(n.children)
    return best


def compute_view_anchor(body: Node, decl_end: int,
                        deref_sites: list[tuple[int, int]],
                        guard_kind: str = "none",
                        guard_protected: tuple[int, int] | None = None,
                        uses_all_in_guard: bool = False) -> int | None:
    """Byte offset where the view `let` may be inserted, or None (skip).

    `body` is the function's block node; `decl_end` the end byte of the
    local's own `let` (the view must come after it — it names the local)."""
    if not deref_sites:
        return None
    first = min(s for s, _ in deref_sites)

    # rule 1 & 2 — null-guard shapes (guard implies validity assertion).
    # early-return allows PARTIAL coverage: the anchor is the guard end, and
    # the caller redirects only derefs at/after it (derefs before the guard
    # sit inside their own tiny is_null blocks — macro-expansion shape — and
    # stay raw, which is always sound).
    if guard_kind == "early-return" and guard_protected is not None:
        pos = max(guard_protected[0], decl_end)
        return pos if any(s >= pos for s, _ in deref_sites) else None
    if guard_kind == "guard-block" and uses_all_in_guard \
            and guard_protected is not None:
        pos = guard_protected[0] + 1          # just inside the `{`
        return pos if decl_end <= pos <= first else None
    if guard_kind not in ("none",):
        return None                            # guard exists but doesn't cover

    # rule 3 — no guard: insertion point must be in a block that contains
    # EVERY deref (so the view is in scope and on every path to each use),
    # and at/after the decl, and at/before the first deref's statement.
    decl_block = _block_containing(body, decl_end - 1) or body
    if all(decl_block.start_byte <= s and e <= decl_block.end_byte
           for s, e in deref_sites):
        # candidate A: right after the decl statement, if the decl's block
        # sees every deref
        first_stmt = _stmt_of(first, decl_block)
        if first_stmt is not None and first_stmt.start_byte >= decl_end:
            return decl_end
        # decl and first deref share the block but the deref's statement
        # starts before the decl ends (same statement?) — unsafe, bail
        return None

    # candidate B: hoist to the first deref's statement inside ITS block,
    # requiring all other derefs to be located at-or-after that statement
    # and inside that block (same-or-inner scope ⇒ view in scope & ordered).
    fblock = _block_containing(body, first)
    if fblock is None or fblock.start_byte < decl_end - 1:
        # first deref's block opened before the decl finished → the decl is
        # visible there only if it's an OUTER block, which the containment
        # check above already rejected — bail conservatively
        return None
    stmt = _stmt_of(first, fblock)
    if stmt is None:
        return None
    ok = all(fblock.start_byte <= s and e <= fblock.end_byte
             and s >= stmt.start_byte for s, e in deref_sites)
    return stmt.start_byte if ok else None
