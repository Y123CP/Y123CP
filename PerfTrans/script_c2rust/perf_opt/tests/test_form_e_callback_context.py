"""The Form E rewrite needs the comparator, and a region prompt cannot hold it.

Detecting `qsort(base, n, size, Some(Cmp as ...))` is only half the work. Card
§2.E makes step 1 of the rewrite "read the comparator body and identify the
ordering key and direction" — because `sort_unstable_by` is equivalent to
`qsort` only if that key and direction survive. The comparator is a separate
item, and a region prompt shows the edited function and nothing else, so the
premise the card demands is structurally absent from the prompt.

The model got this exactly right, which is what made the gap expensive rather
than merely wrong. Handed the region, the cards, and no comparator, it wrote:

    // Skipped rules: [III①: qsort comparator body is not visible here, so the
    //   ordering cannot be proven equivalent for sort_unstable_by]

and the run recorded "rule did not apply" — indistinguishable, from the
outside, from "the rule does not fit here". The site it declined carried a
hand-measured -30.72% on one operation of that crate, the largest single
improvement found in it.

The fix reuses the mechanism C6 already had (`_delegated_callee_context`):
widen the read-only context with the named callee, keep the editable region
where it was. The only thing missing was the name, so the detector now records
which function's pointer was handed over.
"""

from __future__ import annotations

import pathlib

import pytest
from tree_sitter import Language, Parser
import tree_sitter_rust

from perf_opt.agent_perf_opt import prompt_builder as pb
from perf_opt.hot_probe.class_III import iii1_callback as m
from perf_opt.hot_probe.class_III.cst_utils import CallSite, FnCstEntry

_P = Parser(Language(tree_sitter_rust.language()))


def _args_of(src: str, callee: str):
    root = _P.parse(src.encode()).root_node
    stack = [root]
    while stack:
        node = stack.pop()
        if node.type == "call_expression":
            fn = node.child_by_field_name("function")
            if fn is not None and fn.text.decode() == callee:
                return node.child_by_field_name("arguments")
        stack.extend(node.children)
    raise AssertionError(f"no call to {callee} in fixture")


def _form_e_hits(src: str):
    root = _P.parse(src.encode()).root_node
    out = []
    stack = [root]
    while stack:
        node = stack.pop()
        if node.type == "function_item":
            name = node.child_by_field_name("name")
            entry = FnCstEntry(key="k", file=pathlib.Path("src/x.rs"),
                               node=node,
                               name=name.text.decode() if name else "?",
                               qualified_path="k")
            result = m.RuleIIIOneHits()
            m._scan_form_e(entry, result)
            out.extend(result.hits)
        stack.extend(node.children)
    return out


# ═══════════════════════════════ naming the handed-over function

# Verbatim from the crate where the gap was found — the argument spans five
# lines and is wrapped in `Some(..)`, so nothing about it is convenient to
# match textually.
C2RUST_QSORT = '''
unsafe fn ZopfliLengthLimitedCodeLengths(n: c_int) {
    qsort(
        leaves as *mut c_void,
        numsymbols as size_t,
        ::core::mem::size_of::<Node>() as size_t,
        Some(
            LeafComparator
                as unsafe extern "C" fn(
                    *const c_void,
                    *const c_void,
                ) -> c_int,
        ),
    );
}
'''


def test_the_comparator_is_named_in_the_hit() -> None:
    hits = _form_e_hits(C2RUST_QSORT)
    assert len(hits) == 1
    assert hits[0].form == "E"
    assert hits[0].callback_fn == "LeafComparator"


def test_the_cast_is_what_is_matched_not_the_some_wrapper() -> None:
    """`Some(..)` is one of several ways c2rust spells the argument; the decay
    cast is present in all of them, so that is the anchor."""
    src = ('unsafe fn f() { qsort(b, n, s, Cmp as unsafe extern "C" '
           'fn(*const c_void, *const c_void) -> c_int); }')
    assert m._handed_over_fn_name(_args_of(src, "qsort")) == "Cmp"


def test_bsearch_carries_its_key_comparator_too() -> None:
    src = ('unsafe fn f() { bsearch(k, b, n, s, Some(KeyCmp as unsafe '
           'extern "C" fn(*const c_void, *const c_void) -> c_int)); }')
    assert m._handed_over_fn_name(_args_of(src, "bsearch")) == "KeyCmp"


def test_a_pointer_already_in_a_variable_yields_no_name() -> None:
    """Still a Form E hit — libc still dispatches per element — but there is
    no definition to fetch, so the context stays silent rather than guessing."""
    src = "unsafe fn f() { qsort(b, n, s, cmpptr); }"
    assert m._handed_over_fn_name(_args_of(src, "qsort")) == ""


def test_two_different_callbacks_yield_no_name() -> None:
    """`tsearch(key, root, compare)` plus a free-function variant: naming one
    would hand the model a body belonging to the other half of the call."""
    src = ('unsafe fn f() { tsearch(a, Some(Cmp as unsafe extern "C" '
           'fn(*const c_void, *const c_void) -> c_int), Some(Free as unsafe '
           'extern "C" fn(*const c_void, *const c_void) -> c_int)); }')
    assert m._handed_over_fn_name(_args_of(src, "tsearch")) == ""


def test_the_same_callback_twice_is_not_ambiguous() -> None:
    src = ('unsafe fn f() { tsearch(a, Some(Cmp as unsafe extern "C" '
           'fn(*const c_void, *const c_void) -> c_int), Some(Cmp as unsafe '
           'extern "C" fn(*const c_void, *const c_void) -> c_int)); }')
    assert m._handed_over_fn_name(_args_of(src, "tsearch")) == "Cmp"


def test_the_field_defaults_to_empty_for_every_other_rule() -> None:
    """III②/III③ build `CallSite`s too and must not acquire a stray name."""
    assert CallSite(file="f", line=1, col=1, callee_name="malloc").callback_fn == ""


# ═══════════════════════════════ the name reaching the model

def test_merged_hits_propagates_the_name() -> None:
    """Three layers separate the detector from the prompt; the name is only
    useful if it survives all of them."""
    from perf_opt.hot_probe.merged_hits import iter_class_iii

    class _SR:
        iii1_hits = {"crate::x::f": [CallSite(
            file="src/x.rs", line=289, col=5, callee_name="qsort",
            form="E", callback_fn="LeafComparator")]}
        iii2_hits: dict = {}
        iii3_hits: dict = {}
        iii4_hits: dict = {}

    hits = iter_class_iii(_SR())["f"]
    assert hits[0].extra["callback_fn"] == "LeafComparator"


def test_an_empty_name_is_not_written_into_extra() -> None:
    from perf_opt.hot_probe.merged_hits import iter_class_iii

    class _SR:
        iii1_hits = {"crate::x::f": [CallSite(
            file="src/x.rs", line=1, col=1, callee_name="qsort", form="E")]}
        iii2_hits: dict = {}
        iii3_hits: dict = {}
        iii4_hits: dict = {}

    assert "callback_fn" not in iter_class_iii(_SR())["f"][0].extra


def test_the_prompt_may_show_the_name_in_the_anchor_detail() -> None:
    """`extra` is whitelisted before it is rendered — a key absent from the
    whitelist is dropped without a word."""
    import inspect
    src = inspect.getsource(pb.build_region_prompt)
    whitelist = src[src.index('safe_extra = {'):src.index('if safe_extra')]
    assert '"callback_fn"' in whitelist


# ═══════════════════════════════ the widened read-only context

@pytest.fixture()
def crate(tmp_path) -> pathlib.Path:
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "katajainen.rs").write_text(
        'unsafe extern "C" fn LeafComparator(a: *const c_void, b: *const c_void)'
        ' -> c_int {\n'
        '    return (*(a as *const Node)).weight\n'
        '        .wrapping_sub((*(b as *const Node)).weight) as c_int;\n'
        '}\n',
        encoding="utf-8")
    return tmp_path


def _hit(**extra):
    return {"h1": {"rule": "III①", "extra": extra}}


def test_the_comparator_body_is_supplied(crate) -> None:
    out = pb._handed_over_callback_context(
        crate, ["III①", "C3"],
        _hit(form="E", callee_name="qsort", callback_fn="LeafComparator"),
        ["h1"])
    assert "wrapping_sub" in out                      # the ordering key
    assert "LeafComparator" in out


def test_the_context_forbids_editing_the_callback(crate) -> None:
    """Only the region is editable; a rewrite that also deletes the now-unused
    comparator would be rejected as touching source outside the region."""
    out = pb._handed_over_callback_context(
        crate, ["III①"],
        _hit(form="E", callback_fn="LeafComparator"), ["h1"])
    assert "Do NOT edit the callback" in out


def test_the_context_answers_the_abstain_it_exists_to_prevent(crate) -> None:
    out = pb._handed_over_callback_context(
        crate, ["III①"],
        _hit(form="E", callback_fn="LeafComparator"), ["h1"])
    assert "not visible" in out


@pytest.mark.parametrize("candidates, extra", [
    (["C3", "III④"], dict(form="E", callback_fn="LeafComparator")),  # rule absent
    (["III①"], dict(form="B_param", callback_fn="LeafComparator")),  # not form E
    (["III①"], dict(form="E", callback_fn="")),                      # no name
    (["III①"], dict(form="E")),                                      # no key
])
def test_the_scan_is_skipped_when_it_cannot_pay(crate, candidates, extra) -> None:
    """It reads every .rs file in the crate, so it must not run speculatively."""
    assert pb._handed_over_callback_context(
        crate, candidates, _hit(**extra), ["h1"]) == ""


def test_a_name_with_no_definition_in_the_crate_yields_nothing(crate) -> None:
    assert pb._handed_over_callback_context(
        crate, ["III①"], _hit(form="E", callback_fn="NotHere"), ["h1"]) == ""


def test_a_non_form_e_anchor_alongside_does_not_suppress_it(crate) -> None:
    """Regions carry several anchors; the one that matters may not be first."""
    hits = {
        "h0": {"rule": "III③", "extra": {"callee_name": "qsort", "form": ""}},
        "h1": {"rule": "III①", "extra": {"form": "E",
                                         "callback_fn": "LeafComparator"}},
    }
    out = pb._handed_over_callback_context(crate, ["III①", "III③"], hits,
                                           ["h0", "h1"])
    assert "LeafComparator" in out


# ═══════════════════════════════ C6 keeps its own channel

def test_the_c6_widening_is_untouched(crate) -> None:
    """Both blocks are joined into one context slot; adding the second must
    not have stolen the first."""
    hits = {"h1": {"rule": "C6", "extra": {"dispatch_on": "LeafComparator"}}}
    out = pb._delegated_callee_context(crate, ["C6"], hits, ["h1"])
    assert "LeafComparator" in out
    assert "C6" in out


def test_both_widenings_can_appear_together(crate) -> None:
    import inspect
    src = inspect.getsource(pb.build_region_prompt)
    body = src[src.index("callee_ctx = "):src.index("before_text = ")]
    assert "_delegated_callee_context" in body
    assert "_handed_over_callback_context" in body
