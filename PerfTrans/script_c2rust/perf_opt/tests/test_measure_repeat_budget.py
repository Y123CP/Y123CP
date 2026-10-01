"""A fixed repeat count is a fixed cost only if every op costs the same.

`perf stat -r 30` plus 3 warmups is 33 runs. Against a 600s timeout that caps
the per-run cost at ~18s — and the timeout surfaces as `TimeoutExpired`, which
looks exactly like a crash. A compression op measured at 21.7s per run needs
716s, so the very first baseline measurement of that project died, taking the
whole optimization run with it after 16 minutes.

Nothing about that is project-specific; it is arithmetic on REPEATS, WARMUP and
RUN_TIMEOUT. The same shape has now appeared three times in this pipeline (a
hardcoded probe iteration count, a hardcoded perf-input sizing rung, and this),
always with the same signature: a slow operation looking like a broken one.

So: spend a time budget, priced by the first warmup run, which happens anyway.
"""

from __future__ import annotations

import pytest

from perf_opt.verify import measure


def _plan(single_s: float, repeats: int = measure.REPEATS):
    """Mirror of the sizing decision inside `perf_stat`."""
    affordable = int(measure.MEASURE_BUDGET_S / single_s)
    adjusted = max(measure.MIN_REPEATS, min(repeats, affordable))
    timeout = max(measure.RUN_TIMEOUT, int((adjusted + 1) * single_s * 2) + 60)
    return adjusted, timeout


# ───────────────────────────────────────── fast ops must not change behaviour

@pytest.mark.parametrize("single_s", [0.2, 1.0, 1.5, 1.48, 2.0])
def test_fast_ops_keep_the_full_repeat_count(single_s) -> None:
    """The 30-repeat figure buys ±0.06-0.08% stddev, which is what resolves a
    sub-percent lift. Ops that can afford it must keep it."""
    repeats, _ = _plan(single_s)
    assert repeats == measure.REPEATS


def test_a_fast_op_keeps_the_default_timeout(single_s=1.5) -> None:
    _, timeout = _plan(single_s)
    assert timeout == measure.RUN_TIMEOUT


# ───────────────────────────────────────── slow ops must fit, not die

def test_the_op_that_killed_the_run_now_fits() -> None:
    """21.7s per run: 33 runs = 716s > 600s timeout."""
    repeats, timeout = _plan(21.7)
    assert repeats < measure.REPEATS
    assert (repeats + measure.WARMUP) * 21.7 < timeout


@pytest.mark.parametrize("single_s", [10.0, 21.7, 45.0, 90.0, 200.0])
def test_no_op_is_scheduled_past_its_own_timeout(single_s) -> None:
    """The property that was violated: predicted batch cost must stay under
    the deadline for ANY per-run cost, not just the ones seen so far."""
    repeats, timeout = _plan(single_s)
    predicted = (repeats + measure.WARMUP) * single_s
    assert predicted < timeout, (
        f"{single_s}s/run: {repeats} repeats = {predicted:.0f}s vs {timeout}s")


def test_repeats_never_fall_below_the_precision_floor() -> None:
    """Cheaper than the alternative of reporting a number nobody can trust:
    a handful of runs cannot resolve a sub-percent difference."""
    for single_s in (100.0, 500.0, 5_000.0):
        repeats, _ = _plan(single_s)
        assert repeats >= measure.MIN_REPEATS


def test_a_very_slow_op_raises_the_timeout_rather_than_thinning_forever() -> None:
    """Once at the floor, the only remaining lever is the deadline."""
    _, timeout = _plan(90.0)
    assert timeout > measure.RUN_TIMEOUT


# ───────────────────────────────────────── monotonicity

def test_slower_ops_never_get_more_repeats() -> None:
    prev = None
    for single_s in (0.5, 1.5, 5.0, 21.7, 60.0, 200.0):
        repeats, _ = _plan(single_s)
        if prev is not None:
            assert repeats <= prev
        prev = repeats


def test_the_budget_and_floor_are_declared_not_inlined() -> None:
    """They are tuning knobs; a future incident should be able to move them
    in one place."""
    assert measure.MEASURE_BUDGET_S > 0
    assert measure.MIN_REPEATS > 0
    assert measure.MIN_REPEATS < measure.REPEATS
