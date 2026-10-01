"""A repaired rejection must still appear in the record.

A repair turn overwrites `status` and `extra` and loops again. The REGION path
then wrote ONE trace entry with `attempt_no` hard-coded to 1, so whatever the
first turn was judged to be vanished behind whatever the second turn became.

Measured, lodepng `filter` on 2026-08-26:

    02:06:07  region C3,III④ — 94-line rewrite, build ok
    02:06:21  post_validation added_bounds_check_in_loop — one repair turn
    02:06:36  region-repair — a different rewrite
    02:16:51  commit 2b05935                              +0.130%

`rewrites.log` recorded the commit and nothing else. The guard rejection — the
event worth counting, and the one that turned a 94-line rewrite into a +0.130%
one — could only be recovered from `changesets/*/result.json`. Across that run
every one of the 27 REGION attempts carried exactly one trace entry, while the
COMPLEX path, which appends per turn, carried two where two turns happened.

The same run put `terminal_status: null` on both `pre_abstain_short_fn`
records, dropping them out of any tally keyed on that field.
"""

from __future__ import annotations

import inspect

import pytest

from perf_opt.agent_perf_opt import agent, reporting
from perf_opt.agent_perf_opt.state import AttemptRecord, RewriteAttempt


def _source(name: str) -> str:
    return inspect.getsource(getattr(agent, name))


# ────────────────────────── the record layer can carry a repaired rejection

def test_two_region_turns_survive_the_record_layer() -> None:
    """The shape `filter` should have written: refused, then committed."""
    trace = [
        reporting.make_trace_entry(
            1, "region", RewriteAttempt.ABSTAINED,
            "rejected_post_validation", "added_bounds_check_in_loop"),
        reporting.make_trace_entry(
            2, "region", RewriteAttempt.APPLIED_COMMITTED, "committed", None),
    ]
    fields = reporting.with_attempt_trace({"validation_code": "x"}, trace)
    rec = AttemptRecord(
        fn_name="filter", rule_id="C3,III④", round_no=1, attempt_no=1,
        status=RewriteAttempt.APPLIED_COMMITTED, fn_mode="region", **fields)
    assert rec.terminal_status == "committed"
    assert [e["terminal_status"] for e in rec.attempt_trace] == [
        "rejected_post_validation", "committed"]


def test_the_first_turns_verdict_is_not_overwritten() -> None:
    """What the guard said has to stay readable after the repair succeeds."""
    trace = [
        reporting.make_trace_entry(
            1, "region", RewriteAttempt.ABSTAINED,
            "rejected_post_validation", "added_bounds_check_in_loop"),
        reporting.make_trace_entry(
            2, "region", RewriteAttempt.APPLIED_COMMITTED, "committed", None),
    ]
    fields = reporting.with_attempt_trace({}, trace)
    assert fields["attempt_trace"][0]["error"] == "added_bounds_check_in_loop"


# ────────────────────────────────── the region loop appends, once per turn

def test_the_region_loop_appends_a_trace_entry_per_turn() -> None:
    body = _source("_try_large_fn_regions")
    assert "trace.append(" in body, "region loop still writes one fixed entry"


def test_the_region_trace_numbers_itself_from_its_own_length() -> None:
    """`attempt_no` hard-coded to 1 was how the second turn overwrote the
    first without tripping the contiguity check."""
    body = _source("_try_large_fn_regions")
    appended = body[body.index("trace.append("):]
    assert "len(trace) + 1" in appended[:200], appended[:200]


def test_the_entry_is_appended_before_the_repair_continues() -> None:
    """Appended after the `continue`, the refused turn would never be
    recorded at all — which is the bug, rewritten."""
    body = _source("_try_large_fn_regions")
    assert body.index("trace.append(") < body.index(
        "if is_post_validation_repairable(pv_code):")


def test_the_region_loop_no_longer_builds_a_fixed_single_entry() -> None:
    body = _source("_try_large_fn_regions")
    assert "trace = [reporting.make_trace_entry(" not in body


# ──────────────────────────────────── a pre-abstain is a terminal outcome

def test_a_pre_abstain_declares_its_terminal_status() -> None:
    src = _source("run_agent") if hasattr(agent, "run_agent") else inspect.getsource(agent)
    marker = 'rule_id="pre_abstain_short_fn"'
    assert marker in src
    tail = src[src.index(marker):]
    # Window, not an exact offset: the record between the marker and the trace
    # carries comments, and a comment must not be able to fail this test.
    assert "with_attempt_trace" in tail[:1200], tail[:1200]
    assert '"pre_abstain"' in tail[:1200], tail[:1200]


def test_pre_abstain_is_a_status_the_reporter_accepts() -> None:
    assert "pre_abstain" in reporting.AGENT_ONLY_TERMINAL_STATUSES


def test_a_pre_abstain_record_round_trips() -> None:
    """It reaches `rewrites.log` through `AttemptRecord(**fields)` like any
    other outcome, so the kwargs have to fit the dataclass."""
    fields = reporting.with_attempt_trace({}, [
        reporting.make_trace_entry(
            1, "direct", RewriteAttempt.ABSTAINED, "pre_abstain", None)])
    rec = AttemptRecord(
        fn_name="getHash", rule_id="pre_abstain_short_fn", round_no=1,
        attempt_no=1, status=RewriteAttempt.ABSTAINED,
        reason="short_no_leverage: short fn", **fields)
    assert rec.terminal_status == "pre_abstain"
    assert rec.attempt_trace[0]["rewrite_status"] == "abstained"


@pytest.mark.parametrize("field", ["terminal_status", "attempt_trace"])
def test_the_null_columns_that_started_this_are_populated(field) -> None:
    fields = reporting.with_attempt_trace({}, [
        reporting.make_trace_entry(
            1, "direct", RewriteAttempt.ABSTAINED, "pre_abstain", None)])
    assert fields.get(field) is not None
