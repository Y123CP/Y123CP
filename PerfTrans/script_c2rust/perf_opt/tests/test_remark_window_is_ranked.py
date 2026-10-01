"""The remarks the model sees should be the useful ones, spread across passes.

`_filter_top_remarks` was `remarks[:10]` — the compiler's emission order,
which correlates with nothing. Of 34,760 remarks collected across the corpus
11.3% fit the window; a fifth of what fit was `asm-printer` naming basic
blocks, and 8,568 non-noise `missed` remarks were cut.

The second half of the fix matters more than it looks. Ranking by usefulness
alone lets one chatty pass fill the whole window: by tier only, 139 functions
saw exactly one kind of actionable signal. The model is choosing between
rules — the same signal ten times cannot inform that choice, three different
ones can.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.prompt_builder import (
    _filter_top_remarks, _remark_tier,
)


def _r(pass_name: str, status: str = "missed", line: int = 1, msg: str = "m"):
    return {"pass": pass_name, "status": status, "line": line, "message": msg}


# ────────────────────────────────────────────────────────── what gets dropped

@pytest.mark.parametrize("pass_name", [
    "asm-printer", "size-info", "annotation-remarks",
    "prologepilog", "stack-frame-layout", "TTI",
])
def test_build_describing_passes_never_reach_the_model(pass_name: str) -> None:
    """These report what the build produced, not an optimization declined.
    `asm-printer` alone held 449 of the 3,927 slots the old window spent."""
    assert _remark_tier(_r(pass_name, "analysis")) is None
    out = _filter_top_remarks([_r(pass_name, "analysis")] * 5 + [_r("gvn")])
    assert [r["pass"] for r in out] == ["gvn"]


def test_a_window_of_pure_noise_comes_back_empty() -> None:
    assert _filter_top_remarks([_r("asm-printer", "analysis")] * 30) == []


# ───────────────────────────────────────────────────────────── the ordering

def test_an_actionable_miss_outranks_an_unrelated_one() -> None:
    assert _remark_tier(_r("licm")) < _remark_tier(_r("regalloc"))


def test_a_miss_outranks_a_success() -> None:
    assert _remark_tier(_r("gvn", "missed")) < _remark_tier(_r("gvn", "success"))


def test_the_window_leads_with_actionable_misses() -> None:
    remarks = ([_r("licm", "success")] * 8 + [_r("regalloc", "missed")] * 8
               + [_r("gvn", "missed"), _r("loop-vectorize", "missed")])
    top = _filter_top_remarks(remarks, top_n=2)
    assert {r["pass"] for r in top} == {"gvn", "loop-vectorize"}


# ──────────────────────────────────────── the part that is easy to get wrong

def test_one_chatty_pass_cannot_take_the_whole_window() -> None:
    """A pass that emits 30 misses must not crowd out the other two."""
    remarks = ([_r("gvn")] * 30) + [_r("licm")] + [_r("loop-vectorize")]
    top = _filter_top_remarks(remarks, top_n=10)
    assert {"gvn", "licm", "loop-vectorize"} == {r["pass"] for r in top}
    assert sum(1 for r in top if r["pass"] == "gvn") == 8


def test_rotation_stays_inside_a_tier() -> None:
    """Rotation spreads the budget across passes; it must not promote a
    lower tier past a higher one to do it."""
    remarks = [_r("licm", "missed")] * 4 + [_r("regalloc", "missed")] * 4
    top = _filter_top_remarks(remarks, top_n=4)
    assert [r["pass"] for r in top] == ["licm"] * 4


def test_ties_keep_the_compilers_own_order() -> None:
    """Same build, same window, every run — the ranking must not reshuffle
    remarks it considers equally useful."""
    remarks = [_r("gvn", "missed", line=i) for i in range(5)]
    assert [r["line"] for r in _filter_top_remarks(remarks, top_n=3)] == [0, 1, 2]
    assert _filter_top_remarks(remarks) == _filter_top_remarks(remarks)


# ───────────────────────────────────────────────────────────────── edges

@pytest.mark.parametrize("remarks", [[], None])
def test_no_remarks_is_not_an_error(remarks) -> None:
    assert _filter_top_remarks(remarks or []) == []


def test_fewer_than_the_budget_returns_them_all() -> None:
    assert len(_filter_top_remarks([_r("gvn"), _r("licm")])) == 2


def test_a_malformed_entry_is_skipped_not_raised() -> None:
    """This runs while a prompt is being assembled; a stray entry must cost
    the prompt one remark, not the whole turn."""
    assert _filter_top_remarks(["not-a-dict", _r("gvn"), None]) == [_r("gvn")]


def test_an_unknown_pass_still_gets_through_below_the_named_ones() -> None:
    """The pass tables are a ranking, not a whitelist — a pass nobody has
    classified is worth less than a named miss but more than nothing."""
    out = _filter_top_remarks([_r("some-new-pass", "missed"), _r("gvn", "missed")])
    assert [r["pass"] for r in out] == ["gvn", "some-new-pass"]
