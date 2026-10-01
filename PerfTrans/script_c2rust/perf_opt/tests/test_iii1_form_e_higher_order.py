"""A callback handed to libc is still an indirect call — just not here.

Forms A/B/C/D all look for an indirect call made *inside* the analysed
function. `qsort(base, n, size, Some(cmp))` makes none. The indirect call
happens inside libc, once per element comparison, and the comparator can
never be inlined. The CST shape shares nothing with the other four, so all
four missed it — on every project, not just one.

Measured, replacing a single such call with the native Rust sort (same
ordering key, `Node.weight` ascending):

    tree_codegen              1.4439s -> 1.0003s   -30.72%
    lz77_blocksplit_analysis  1.3531s -> 1.2513s   - 7.52%
    W1 golden replay          1500 / 1500, zero mismatches

That operation's profile had `msort_with_tmp` plus the comparator at 30.5%
of self time — all of it C ABI dispatch that Rust inlines away. It was the
largest single improvement found in the crate, and it sat in an operation
the search had never profiled.

The detector is keyed on the callee name plus a function pointer among the
arguments; `qsort` is how c2rust renders every C `qsort`, so this is a
property of the translator, not of any one codebase.
"""

from __future__ import annotations

import pathlib

import pytest
from tree_sitter import Language, Parser
import tree_sitter_rust

from perf_opt.hot_probe.class_III import iii1_callback as m
from perf_opt.hot_probe.class_III.config import HIGHER_ORDER_LIBC_FNS
from perf_opt.hot_probe.class_III.cst_utils import FnCstEntry

_P = Parser(Language(tree_sitter_rust.language()))


def _scan(src: str):
    root = _P.parse(src.encode()).root_node
    out = []
    stack = [root]
    while stack:
        n = stack.pop()
        if n.type == "function_item":
            name = n.child_by_field_name("name")
            entry = FnCstEntry(key="k", file=pathlib.Path("src/x.rs"), node=n,
                               name=name.text.decode() if name else "?",
                               qualified_path="k")
            r = m.RuleIIIOneHits()
            m._scan_form_e(entry, r)
            out.extend(r.hits)
        stack.extend(n.children)
    return out


# ───────────────────────────── the shape that was being missed

QSORT = '''
unsafe fn f(leaves: *mut Node, numsymbols: c_int) {
    qsort(
        leaves as *mut c_void,
        numsymbols as size_t,
        ::core::mem::size_of::<Node>() as size_t,
        Some(LeafComparator as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
    );
}
'''


def test_a_comparator_handed_to_qsort_is_detected() -> None:
    hits = _scan(QSORT)
    assert len(hits) == 1
    assert hits[0].callee_name == "qsort"
    assert hits[0].form == "E"


def test_the_hit_carries_a_usable_location() -> None:
    h = _scan(QSORT)[0]
    assert h.line > 0 and h.col > 0
    assert str(h.file).endswith(".rs")


@pytest.mark.parametrize("callee", sorted(HIGHER_ORDER_LIBC_FNS))
def test_every_whitelisted_higher_order_fn_is_scanned(callee) -> None:
    src = (f'unsafe fn f(p: *mut c_void) {{ {callee}(p, 4 as size_t, 8 as size_t, '
           f'Some(cmp as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int)); }}')
    hits = _scan(src)
    assert len(hits) == 1 and hits[0].callee_name == callee


# ───────────────────────────── it must not fire on ordinary calls

def test_a_libc_call_without_a_function_pointer_is_not_a_dispatch_site() -> None:
    """`bsearch` on a key with no comparator argument is not this pattern —
    and neither is any other call that merely shares a name."""
    src = 'unsafe fn f(p: *mut c_void) { qsort(p, 4 as size_t, 8 as size_t, None); }'
    assert _scan(src) == []


def test_an_unrelated_call_is_ignored() -> None:
    src = ('unsafe fn f(p: *mut c_void) { memcpy(p, p, 8 as size_t); '
           'my_own_sort(p, Some(cmp as unsafe extern "C" fn())); }')
    assert _scan(src) == []


def test_a_plain_function_without_calls_yields_nothing() -> None:
    assert _scan("fn f() { let x = 1; }") == []


# ───────────────────────────── the other four forms are untouched

def test_form_e_does_not_disturb_the_existing_forms() -> None:
    """Phase 4 runs after A/B/C/D and only appends; a function with both an
    in-body indirect call and a qsort must report both."""
    src = '''
unsafe fn f(cb: Option<unsafe extern "C" fn(u32) -> u32>, p: *mut c_void) {
    let _ = cb.expect("non-null function pointer")(1);
    qsort(p, 4 as size_t, 8 as size_t,
          Some(cmp as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int));
}
'''
    forms = {h.form for h in _scan(src)}
    assert forms == {"E"}, "the standalone scanner sees only form E"


def test_the_whitelist_is_translator_level_not_project_level() -> None:
    """Every entry is a C standard-library higher-order function, so the
    detector generalises to any c2rust output rather than one codebase."""
    assert "qsort" in HIGHER_ORDER_LIBC_FNS
    assert "bsearch" in HIGHER_ORDER_LIBC_FNS
    for name in HIGHER_ORDER_LIBC_FNS:
        assert name.islower() and "_" not in name.strip("_") or name in {
            "qsort_r"}, name


# ───────────────────────────── the card must teach the rewrite

def _card() -> str:
    from pathlib import Path
    return (Path(__file__).resolve().parents[1] / "agent_perf_opt"
            / "Optimization_Card" / "III1_callback_monomorph.md").read_text(
                encoding="utf-8")


def test_the_card_documents_form_e() -> None:
    """Detection without instruction produces a prompt that asks for an
    unspecified rewrite."""
    card = _card()
    assert "Form E" in card
    assert "qsort" in card and "sort_unstable_by" in card


def test_the_card_requires_proving_the_order_is_unchanged() -> None:
    """This rewrite reorders data. Getting the direction wrong is a silent
    correctness bug that only some inputs expose."""
    card = _card()
    assert "ascending" in card and "descending" in card
    assert "then_with" in card                      # multi-key comparators


def test_the_card_warns_about_stability() -> None:
    """`qsort` is unstable; `sort_by` allocates. Picking the wrong one is
    either a semantic change or a needless allocation."""
    card = _card()
    assert "not stable" in card or "not** stable" in card
    assert "sort_unstable_by" in card


def test_the_card_covers_the_subtraction_overflow_trap() -> None:
    """`a.key - b.key` as a comparator is only a total order while the
    subtraction cannot overflow — reproducing it verbatim in Rust would be
    wrong for full-range keys."""
    assert "overflow" in _card()


def test_the_card_states_when_to_abstain() -> None:
    card = _card()
    assert "Abstain" in card
    assert "qsort_r" in card       # captured-context comparators


# ───────────────────────────── routing: the same rule, two kinds of work

from perf_opt.agent_perf_opt.regions.model import (          # noqa: E402
    RuleCapability,
    hit_capability,
    rule_capability,
    rules_region_local_for,
)

_HIT_E = {"rule": "III①", "file": "src/k.rs", "line": 289, "col": 5,
          "pattern": "higher-order-libc",
          "extra": {"callee_name": "qsort", "form": "E"}}
_HIT_B = {"rule": "III①", "file": "src/s.rs", "line": 363, "col": 9,
          "pattern": "param-callback",
          "extra": {"callee_name": "costmodel", "form": "B_param"}}


def test_form_e_routes_region_local_while_the_rule_stays_cross_function() -> None:
    """A/B/C/D rewrite the signature and every call site; form E replaces one
    call inside the body. Routing by rule id alone sends form E to a signature
    rewrite it does not need, and the region path then discards it."""
    assert rule_capability("III①") is RuleCapability.CROSS_FUNCTION
    assert hit_capability(_HIT_E) is RuleCapability.REGION_LOCAL
    assert hit_capability(_HIT_B) is RuleCapability.CROSS_FUNCTION


def test_the_region_prompt_gate_admits_form_e() -> None:
    """The gate that actually fired in production:

        ERROR only region-local rules may enter a region prompt: ['III①']

    The detector found the site, the router sent it down the region path, and
    this check — reading rule ids, not hits — threw it away.
    """
    assert rules_region_local_for(("III①",), [_HIT_E]) == []


def test_the_region_prompt_gate_still_rejects_the_cross_function_forms() -> None:
    assert rules_region_local_for(("III①",), [_HIT_B]) == ["III①"]


def test_a_mixed_candidate_is_admitted_when_every_hit_is_local() -> None:
    c3 = {"rule": "C3", "file": "src/x.rs", "line": 1, "col": 1, "extra": {}}
    assert rules_region_local_for(("C3", "III①"), [c3, _HIT_E]) == []


def test_without_hits_the_gate_falls_back_to_the_conservative_answer() -> None:
    """No hits to consult must not become a free pass."""
    assert rules_region_local_for(("III①",), []) == ["III①"]
    assert rules_region_local_for(("III①",), None) == ["III①"]


def test_every_gate_on_the_path_agrees() -> None:
    """Router, prompt builder and planner must reach the same verdict — the
    first run carrying form E failed because they disagreed."""
    from pathlib import Path
    root = Path(__file__).resolve().parents[1]
    pb = (root / "agent_perf_opt" / "prompt_builder.py").read_text(encoding="utf-8")
    pl = (root / "agent_perf_opt" / "planners" / "llm_region.py").read_text(encoding="utf-8")
    ex = (root / "agent_perf_opt" / "regions" / "extractor.py").read_text(encoding="utf-8")
    assert "rules_region_local_for(candidates, hits)" in pb
    assert "rules_region_local_for(rule_ids, hits)" in pl
    assert "hit_capability(hit)" in ex
