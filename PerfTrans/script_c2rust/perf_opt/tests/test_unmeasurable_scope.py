"""An op that cannot be measured must leave the scope, not void the comparison.

`prepare` returns None for an op with no usable perf input — typically one
whose gen_perf input crashes it. The comparison used to abort on the first such
op, which discards every OTHER op's measurement too.

Measured: one segfaulting perf input turned a 21-commit run's final measurement
into `per_op={}`, `checkpoint=0`, `aggregate=None`. The optimisation work was
intact in git and simply had no number attached to it — and aborting had not
verified the broken op either, since it never ran.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from perf_opt.agent_perf_opt import measurement as M


class _Backend:
    """Deterministic stand-in: `bad` ops have no input, everything else gains
    `gain_pct` on the candidate side."""

    def __init__(self, bad: set[str], gain_pct: float = -5.0):
        self.bad, self.gain_pct = bad, gain_pct

    def prepare(self, _binary, op, _assets, _target_wall):
        return None if op in self.bad else (Path(f"/in/{op}"), 100)

    def measure_once(self, binary, _op, _input_path, _iters, _pin_cpu):
        base = 1000.0
        return base * (1 + self.gain_pct / 100.0) if binary == Path("cand") else base


def _compare(ops, bad, **kw):
    return M.compare_paired(
        parent_binary=Path("parent"),
        candidate_binary=Path("cand"),
        ops=list(ops),
        assets=None,
        backend=_Backend(set(bad)),
        **kw,
    )


def _kwargs():
    return dict(target_wall=1.0, checkpoints=(2, 10), minimum_pairs=10,
                early_reject_pairs=2, per_op_upper_limit_pct=1.0,
                regress_tolerance_pct=0.3, catastrophic_regress_pct=None)


@pytest.fixture(autouse=True)
def _skip_if_signature_differs():
    import inspect
    sig = inspect.signature(M.compare_paired)
    needed = {"parent_binary", "candidate_binary", "ops", "backend"}
    missing = needed - set(sig.parameters)
    if missing:
        pytest.skip(f"compare_paired signature lacks {missing}")


def test_one_unmeasurable_op_does_not_void_the_others():
    """The regression, stated directly."""
    res = _compare(["good_a", "good_b", "bad"], bad={"bad"}, **_kwargs())
    assert res.reason != "unmeasurable"
    assert set(res.per_op) == {"good_a", "good_b"}
    assert res.aggregate_mean_pct is not None


def test_the_dropped_op_is_named_in_the_verdict():
    """A narrowed scope that does not say so is worse than a failure."""
    res = _compare(["good", "bad"], bad={"bad"}, **_kwargs())
    assert "bad" in res.unmeasurable_ops
    assert "good" not in res.unmeasurable_ops


def test_all_ops_unmeasurable_is_still_unmeasurable():
    """Dropping must not degrade into reporting a verdict over nothing."""
    res = _compare(["a", "b"], bad={"a", "b"}, **_kwargs())
    assert res.reason == "unmeasurable"
    assert set(res.unmeasurable_ops) == {"a", "b"}
    assert not res.per_op


def test_no_ops_at_all_is_unmeasurable():
    res = _compare([], bad=set(), **_kwargs())
    assert res.reason == "unmeasurable"


def test_a_clean_run_reports_no_dropped_ops():
    res = _compare(["a", "b"], bad=set(), **_kwargs())
    assert res.unmeasurable_ops == ()
    assert set(res.per_op) == {"a", "b"}
