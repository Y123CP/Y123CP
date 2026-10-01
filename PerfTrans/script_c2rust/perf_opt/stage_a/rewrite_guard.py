"""Structural guard for whole-function LLM rewrites.

A pointer/slice lift is supposed to touch ONLY the target anchor's uses. In
practice an LLM reproducing the whole function can silently alter code far
from the anchor — the zopfli 2026-07-20 miscompute: lifting the local
`hhashval` pointer, the model also rewrote an UNRELATED sibling access
`*(*h).hashval2.offset(p)` into `hhashval[p]` (wrong hash array). Signature
was unchanged, it compiled, and it matched stdout on the 8-sampled inner W1
gate — the error only surfaced on 2/300 inputs in the final full replay,
which then hard-failed the whole run.

The invariant enforced here, per single-pointer rewrite:

    Every field/method access reached through an expression that does NOT
    mention the anchor being lifted must be preserved (same set before and
    after).

The anchor's OWN accesses are expected to change (`*p.offset(i)` → `p[i]`,
`p.is_null()` dropped, etc.), so any access whose base expression mentions the
anchor identifier is excluded. What remains is code the lift had no business
touching; if such an access appears or vanishes, the model over-reached and the
rewrite is rejected (treated like a candidate the gate declined — no apply, no
rollback). zopfli: lifting `hhashval`, the access `(*h).hashval2 . offset` (base
`(*h).hashval2`, no `hhashval`) vanishes → rejected.

Method-call receivers ARE included (a struct-field-only check would miss zopfli:
`(*h).hashval2` still appears elsewhere via the head2-switch slice build, so as
a plain field it is unchanged — the tell is that the `.offset` READ of it is
gone). Comparison is per-candidate (one pointer at a time), matching how the pr2
/ intra_ptr / buffer_lift drivers call the LLM, so already-lifted sibling
pointers are identical on both sides and never enter the delta.

Set semantics (not multiset): a lift may read the anchor's source field more or
fewer times; the field still appears on both sides, so only true appear/vanish
is caught. Over-rejection is the safe direction — a false reject just forgoes
one lift; the final full replay remains the correctness backstop.
"""

from __future__ import annotations

import re

from .intra_ptr.collect import _PARSER, _txt


def _walk(n):
    st = [n]
    while st:
        x = st.pop()
        st.extend(x.children)
        yield x


# `(* inner )` wrapping the WHOLE base — c2rust's raw-deref form. Once a pointer
# is lifted to a reference, `(*p).field` idiomatically becomes `p.field` (Rust
# auto-deref), so the two must compare EQUAL or every ref-lift trips the guard
# (fzy 2026-07-20: `(*w).job` → `w.job` false-rejected valid pr2 lifts).
_DEREF_WRAP = re.compile(r"^\(\s*\*\s*(.+?)\s*\)$")


def _norm_base(base: str) -> str:
    """Collapse a full outer raw-deref wrapper: `(*w)` → `w`, `(*(*s).x)` →
    `(*s).x`. A base only PARTLY wrapped (`(*h).hashval2` — the `(*h)` is not
    the whole string) is left as-is, so a genuine sibling-field read still
    registers (zopfli)."""
    m = _DEREF_WRAP.match(base)
    return m.group(1) if m else base


def access_set(fn_text: str) -> set[tuple[str, str]]:
    """{(base_expr_normalized, field_or_method_name)} for every `expr.name`
    (struct-field access AND method-call receiver — both are `field_expression`
    in tree-sitter-rust). Base is deref-normalized so `(*p).f` == `p.f`.
    Tuple-index accesses (`.0`) are skipped."""
    src = fn_text.encode()
    root = _PARSER.parse(src).root_node
    out: set[tuple[str, str]] = set()
    for n in _walk(root):
        if n.type != "field_expression":
            continue
        val = n.child_by_field_name("value")
        fld = n.child_by_field_name("field")
        if val is None or fld is None or fld.type != "field_identifier":
            continue
        base = _norm_base(" ".join(_txt(src, val).split()))
        out.add((base, _txt(src, fld)))
    return out


def foreign_access_delta(orig_text: str, new_text: str, anchor: str
                         ) -> tuple[set[tuple[str, str]], set[tuple[str, str]]]:
    """(removed, added) accesses between orig and new whose base expression does
    NOT mention `anchor` (the pointer being lifted). Non-empty => the rewrite
    changed an access outside the lift target — a structural over-reach.

    An unparsable `new_text` is already caught by the signature guard upstream;
    on any parse error here we return empty sets (defer to the other gates)
    rather than false-reject.
    """
    try:
        o = access_set(orig_text)
        n = access_set(new_text)
    except Exception:
        return set(), set()
    if anchor:
        word = re.compile(r"\b" + re.escape(anchor) + r"\b")
        o = {(b, f) for (b, f) in o if not word.search(b)}
        n = {(b, f) for (b, f) in n if not word.search(b)}
    return o - n, n - o
