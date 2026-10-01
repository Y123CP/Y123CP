"""II_inl and II_iso: one inlining attribute, planned without a model.

II_inl had never landed a commit in any of twelve projects: its card's rewrite
is an attribute on the CALLEE, and every LLM path locks its reply to the hot
function. Measured on a colour-conversion op, the one attribute the rule asked
for (`#[inline(always)]` on a per-pixel callee) cut the op's cycles by 15% and
brought its instruction count back to the pre-refinement level.

II_iso is its mirror: a hot loop kernel fully inlined into a much larger
function gets `#[inline(never)]`. Measured on a PNG decoder's Adam7 loop, -12%
on the op it dominates. It is nominated from debug info and judged by W2.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.changeset.handlers.set_inline_attr import (
    SetFunctionInlineAttrHandler,
    locate_inline_site,
)
from perf_opt.agent_perf_opt.changeset.resolver import ResolutionRejected
from perf_opt.agent_perf_opt.changeset.types import SetFunctionInlineAttr
from perf_opt.agent_perf_opt.planners.inline_attr import (
    INLINE_HINT_THRESHOLD,
    InlineCallee,
    choose_inline_attribute,
    demangle_last,
    ii_inl_callees,
    inline_remark_facts,
    plan_inline_attr,
    rank_callees,
)
from perf_opt.hot_probe.inline_container import inlined_instances, isolation_candidates


# ───────────────────────────────── remark parsing

_REMARK = ("'_ZN15lodepng_cleaned3src7lodepng12rgba8ToPixel17h1111111111111111E' "
           "not inlined into '_ZN15lodepng_cleaned3src7lodepng15lodepng_convert"
           "17h2222222222222222E' because too costly to inline (cost=685, threshold=225)")


def test_remark_facts_come_from_the_whole_message() -> None:
    facts = inline_remark_facts(_REMARK)
    assert facts["callee"] == "rgba8ToPixel"
    assert facts["cost"] == 685 and facts["threshold"] == 225


def test_a_truncated_snippet_still_names_the_callee() -> None:
    facts = inline_remark_facts(_REMARK[:120])
    assert facts["callee"] == "rgba8ToPixel"
    assert facts["cost"] is None


def test_demangle_keeps_the_last_segment_only() -> None:
    assert demangle_last("_ZN3foo3bar3baz17h0123456789abcdefE") == "baz"
    assert demangle_last("plain_name") == "plain_name"


def test_callees_are_deduplicated_with_their_costliest_site() -> None:
    hits = [
        {"rule": "II_inl", "pattern": "TooCostly", "extra": {"callee": "a", "cost": 100}},
        {"rule": "II_inl", "pattern": "TooCostly", "extra": {"callee": "a", "cost": 400}},
        {"rule": "II_inl", "pattern": "TooCostly", "snippet": _REMARK},
        {"rule": "II_inl", "pattern": "NoDefinition", "extra": {"callee": "b"}},
        {"rule": "II_vec", "pattern": "TooCostly", "extra": {"callee": "c"}},
    ]
    got = {c.name: c for c in ii_inl_callees(hits)}
    assert set(got) == {"a", "rgba8ToPixel"}
    assert got["a"].cost == 400 and got["a"].n_sites == 2


# ───────────────────────────────── attribute choice and ranking

def test_a_hint_is_used_only_when_it_can_work() -> None:
    assert choose_inline_attribute(INLINE_HINT_THRESHOLD) == "inline"
    assert choose_inline_attribute(INLINE_HINT_THRESHOLD + 1) == "inline(always)"
    assert choose_inline_attribute(None) == "inline(always)"


def test_hot_callees_rank_first_and_the_list_is_capped() -> None:
    callees = [InlineCallee("cold_cheap", 9, 1), InlineCallee("hot", 685, 1),
               InlineCallee("warm", 50, 2), InlineCallee("self", 1, 1)]
    ranked = rank_callees(callees, {"hot": 23.9, "warm": 3.6},
                          exclude={"self"}, limit=2)
    assert [c.name for c in ranked] == ["hot", "warm"]


def test_the_plan_is_one_attribute_operation() -> None:
    plan = plan_inline_attr(
        rule_id="II_inl", hot_function="caller", target_fn="callee",
        relative_path="src/lib.rs", line_hint=3, attribute="inline(always)",
        base_head="a" * 40, candidate_id="caller-II_inl-x")
    assert plan.proposal is not None
    (op,) = plan.proposal.operations
    assert isinstance(op, SetFunctionInlineAttr)
    assert (op.fn_name, op.attribute) == ("callee", "inline(always)")
    assert plan.proposal.changeset_id.startswith("ii-inl-")


def test_an_unknown_attribute_is_refused() -> None:
    plan = plan_inline_attr(
        rule_id="II_inl", hot_function="c", target_fn="f", relative_path="src/lib.rs",
        line_hint=1, attribute="cold", base_head="a" * 40, candidate_id="x")
    assert plan.proposal is None


# ───────────────────────────────── the handler edits exactly one attribute

def _crate(tmp_path: Path, src: str) -> Path:
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "lib.rs").write_text(src)
    return tmp_path


def _apply(crate: Path, op: SetFunctionInlineAttr) -> str:
    handler = SetFunctionInlineAttrHandler()
    res = handler.resolve(op, crate)
    assert handler.pre_validate(op, res, crate).ok
    (edit,) = res.edits
    path = crate / edit.relative_path
    data = path.read_bytes()
    path.write_bytes(data[:edit.start_byte] + edit.replacement_text.encode()
                     + data[edit.end_byte:])
    post = handler.post_validate(op, res, crate)
    assert post.ok, post
    return path.read_text()


def _op(fn: str, attr: str, line: int = 1) -> SetFunctionInlineAttr:
    return SetFunctionInlineAttr(operation_id="op", rule_id="II_inl",
                                 evidence_hit_ids=(), relative_path="src/lib.rs",
                                 fn_name=fn, line_hint=line, attribute=attr)


def test_attribute_is_inserted_above_a_bare_item(tmp_path: Path) -> None:
    crate = _crate(tmp_path, "pub mod m {\n    unsafe fn f(x: u8) -> u8 {\n        x\n    }\n}\n")
    out = _apply(crate, _op("f", "inline(always)", 2))
    assert "    #[inline(always)]\n    unsafe fn f(x: u8)" in out
    assert out.count("fn f(") == 1


def test_an_existing_inline_attribute_is_replaced_not_stacked(tmp_path: Path) -> None:
    crate = _crate(tmp_path, "#[inline]\n#[no_mangle]\nfn f() {}\n")
    out = _apply(crate, _op("f", "inline(never)", 3))
    assert "#[inline(never)]" in out and "#[inline]\n" not in out
    assert "#[no_mangle]" in out


def test_the_same_attribute_twice_is_a_conflict(tmp_path: Path) -> None:
    crate = _crate(tmp_path, "#[inline(always)]\nfn f() {}\n")
    with pytest.raises(ResolutionRejected):
        SetFunctionInlineAttrHandler().resolve(_op("f", "inline(always)", 2), crate)


def test_a_missing_function_is_stale(tmp_path: Path) -> None:
    crate = _crate(tmp_path, "fn g() {}\n")
    with pytest.raises(ResolutionRejected):
        SetFunctionInlineAttrHandler().resolve(_op("f", "inline"), crate)


def test_the_link_name_resolves_too(tmp_path: Path) -> None:
    crate = _crate(tmp_path, '#[export_name = "match"]\nfn match_0() {}\n')
    assert locate_inline_site((crate / "src/lib.rs").read_bytes(), "match", 2)


def test_the_operation_validates_its_attribute() -> None:
    with pytest.raises(ValueError):
        _op("f", "cold")


# ───────────────────────────────── II_iso nomination from debug info

_DUMP = """\
0x00000010:   DW_TAG_subprogram
                DW_AT_low_pc	(0x0000000000001000)
                DW_AT_high_pc	(0x0000000000003000)
                DW_AT_linkage_name	("_ZN7mylib_x3src6decode17h1111111111111111E")
                DW_AT_name	("decode")

0x00000020:     DW_TAG_inlined_subroutine
                  DW_AT_abstract_origin	(0x00000099 "_ZN7mylib_x3src6kernel17h2222222222222222E")
                  DW_AT_low_pc	(0x0000000000001100)
                  DW_AT_high_pc	(0x0000000000001500)

0x00000030:     DW_TAG_inlined_subroutine
                  DW_AT_abstract_origin	(0x00000098 "_ZN7mylib_x3src6helper17h3333333333333333E")
                  DW_AT_low_pc	(0x0000000000001600)
                  DW_AT_high_pc	(0x0000000000001610)

0x00000040:     NULL

0x00000050:   DW_TAG_subprogram
                DW_AT_low_pc	(0x0000000000004000)
                DW_AT_high_pc	(0x0000000000004100)
                DW_AT_linkage_name	("_ZN7mylib_x3src5owned17h4444444444444444E")
                DW_AT_name	("owned")

0x00000060:   NULL
"""


def _hot(name, fn_type="algorithm_hot", reason="40 lines, 2 compute loops", pct=10.0):
    return {"name": name, "fn_type": fn_type, "fn_type_reason": reason,
            "self_pct": pct, "file": "src/lib.rs", "line_start": 1}


def test_a_kernel_inlined_into_a_large_library_function_is_nominated() -> None:
    inst, own = inlined_instances(_DUMP.splitlines(), {"kernel", "helper", "owned"})
    found = isolation_candidates(inst, own, [_hot("kernel")], "mylib_x")
    assert [c["fn"] for c in found] == ["kernel"]
    assert found[0]["container"] == "decode"
    assert found[0]["container_bytes"] == 0x2000 and found[0]["inlined_bytes"] == 0x400


def test_leaves_owned_copies_and_non_library_containers_are_not() -> None:
    inst, own = inlined_instances(_DUMP.splitlines(), {"kernel", "helper", "owned"})
    assert not isolation_candidates(inst, own, [_hot("helper", "other", "3 lines")], "mylib_x")
    assert not isolation_candidates(inst, own, [_hot("owned")], "mylib_x")
    assert not isolation_candidates(inst, own, [_hot("kernel")], "another_crate")


def test_a_container_not_much_larger_is_not_nominated() -> None:
    inst, own = inlined_instances(_DUMP.splitlines(), {"kernel"})
    assert not isolation_candidates(inst, own, [_hot("kernel")], "mylib_x", min_ratio=9.0)


def test_nominations_are_capped_hottest_first() -> None:
    inst = {n: [("decode", "_ZN7mylib_x3src6decode17h1E", 10000, 200)] for n in "abcd"}
    hot = [_hot(n, pct=p) for n, p in zip("abcd", (5.0, 30.0, 10.0, 20.0))]
    found = isolation_candidates(inst, {}, hot, "mylib_x", cap=3)
    assert [c["fn"] for c in found] == ["b", "d", "c"]


def test_a_small_container_or_a_fragment_is_not_nominated() -> None:
    """Register pressure is a property of LARGE functions acting on a loop
    body of real size; a 30-byte fragment in a 1.3 KB function is neither."""
    small_container = {"k": [("f", "_ZN7mylib_x1f17h1E", 1300, 300)]}
    fragment = {"k": [("f", "_ZN7mylib_x1f17h1E", 20000, 30)]}
    assert not isolation_candidates(small_container, {}, [_hot("k")], "mylib_x")
    assert not isolation_candidates(fragment, {}, [_hot("k")], "mylib_x")


def test_instances_in_one_container_are_summed() -> None:
    """A kernel inlined at several call sites, or split into several ranges,
    is that much code in the container — each piece alone can look tiny."""
    link = "_ZN7mylib_x1f17h1E"
    split = {"k": [("f", link, 20000, 90), ("f", link, 20000, 90)]}
    found = isolation_candidates(split, {}, [_hot("k")], "mylib_x")
    assert found and found[0]["inlined_bytes"] == 180


def test_cold_callees_are_not_attempted() -> None:
    """Most too-costly remarks in a hot function are for cold calls — error
    reporters, one-shot setup. Only a callee with its own self time in the
    profile has its call on the hot path."""
    callees = [InlineCallee("xmlErrMemory", 35, 1), InlineCallee("hot", 685, 1)]
    ranked = rank_callees(callees, {"hot": 12.0}, limit=2)
    assert [c.name for c in ranked] == ["hot"]
    everything = rank_callees(callees, {"hot": 12.0}, limit=2, hot_only=False)
    assert {c.name for c in everything} == {"hot", "xmlErrMemory"}
