"""A veto should say how much, not just which.

`[expr_embedded_builtins] total_regress` names the operation that refused a
candidate and stops there. An operation that collapsed and an operation that
drifted a percent on code layout produce the same line, and telling them
apart afterwards costs a re-measurement — as much as the run that produced
it. The magnitude is already measured and already written to
`changesets/<id>/result.json`; it just never reached `rewrites.log`, which is
the file anyone opens first.

Real case this comes from: a candidate recorded `w2_delta_pct: -8.73` — the
total gate's aggregate, i.e. the crate got 8.7% faster — next to
`status: w2_regress`, with nothing on that line explaining that one operation
had gone +23.09% while the step gate, which measures only the operations the
rewritten function appears in, never saw it.
"""

from __future__ import annotations

import importlib

import pytest

agent = importlib.import_module("perf_opt.agent_perf_opt.agent")


class _Verdict:
    def __init__(self, detail) -> None:
        self.detail = detail


def _verdict(op_rows, gate="total"):
    return _Verdict({"measurements": [{"gate": gate, "reference": "pristine",
                                       "ops": op_rows}]})


def test_the_line_carries_the_offending_delta() -> None:
    v = _verdict([{"op": "expr_embedded_builtins", "mean_delta_pct": 23.0906},
                  {"op": "builtin_script_matrix", "mean_delta_pct": -51.7588}])
    line = agent._w2_rejection_line(v, "expr_embedded_builtins", "total_regress")
    assert line == "[expr_embedded_builtins +23.09%] total_regress"


def test_a_drift_and_a_collapse_no_longer_read_alike() -> None:
    drift = agent._w2_rejection_line(
        _verdict([{"op": "cache", "mean_delta_pct": 1.04}]), "cache", "total_regress")
    collapse = agent._w2_rejection_line(
        _verdict([{"op": "cache", "mean_delta_pct": 31.7}]), "cache", "total_regress")
    assert drift != collapse
    assert "+1.04%" in drift and "+31.70%" in collapse


def test_no_triggering_op_leaves_the_reason_alone() -> None:
    assert agent._w2_rejection_line(_verdict([]), "", "no_gain") == "no_gain"


@pytest.mark.parametrize("verdict", [
    _Verdict(None),                                  # a path that never measured
    _Verdict({}),                                    # measured, no rows
    _Verdict({"measurements": "not-a-list"}),        # malformed
    _Verdict({"measurements": [{"ops": [{"op": "other", "mean_delta_pct": 1.0}]}]}),
    _Verdict({"measurements": [{"ops": [{"op": "x", "mean_delta_pct": None}]}]}),
])
def test_a_missing_magnitude_degrades_to_the_old_line(verdict) -> None:
    """This runs while a gate result is being recorded. A verdict that cannot
    supply the number must cost the candidate nothing — the bookkeeping must
    never be what fails the thing it is bookkeeping."""
    assert agent._w2_rejection_line(verdict, "x", "total_regress") == "[x] total_regress"


def test_the_deciding_gates_block_is_read_first() -> None:
    """Both gates' numbers are recorded; the veto came from the one listed
    first, and that is the delta the line must quote."""
    v = _Verdict({"measurements": [
        {"gate": "total", "ops": [{"op": "shared", "mean_delta_pct": 23.1}]},
        {"gate": "step", "ops": [{"op": "shared", "mean_delta_pct": -0.3}]},
    ]})
    assert "+23.10%" in agent._w2_rejection_line(v, "shared", "total_regress")
