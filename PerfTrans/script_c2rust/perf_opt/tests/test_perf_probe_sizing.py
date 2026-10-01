"""Sizing probes must be bounded by TIME, not by a fixed iteration count.

`_autotune_iters` used to probe with a hard-coded 200 iterations. That costs
200x the per-iteration time, so for a heavyweight op it blows `_time_once`'s
300 s subprocess cap and the op is reported as "errored on perf input" — a
silent drop, indistinguishable from an op that really crashes. Measured on
brotli: `encode_buffer` at 3.1 s/iter and `shared_dict_roundtrip` at 2.6 s/iter
were both dropped, while `stream_roundtrip` at 1.4 s/iter squeaked under the cap
and kept a 283 s wall that no A/B measurement loop can afford.

The same `_autotune_iters` is called from `hotness_probe.py`, so the defect sat
in the generation loop too.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from harness_gen import perf_workload as pw

# `_time_once` gives up here; a probe that needs longer looks like a failure.
TIME_ONCE_CAP_S = 300.0


def _fake_clock(cost_per_iter: float, *, startup: float = 0.004,
                cap: float = TIME_ONCE_CAP_S):
    """Stand-in for `_time_once`: linear cost, and -1.0 past the subprocess cap
    (which is what a timeout actually looks like to the caller)."""
    calls: list[int] = []

    def _time_once(_bin, _op, _inp, iters, _hdir, _pin):
        calls.append(iters)
        wall = startup + cost_per_iter * iters
        return -1.0 if wall > cap else wall

    return _time_once, calls


def _autotune(monkeypatch, cost_per_iter: float, target_wall: float = 1.5,
              **kw) -> tuple[int, float, list[int]]:
    fake, calls = _fake_clock(cost_per_iter, **kw)
    monkeypatch.setattr(pw, "_time_once", fake)
    iters, per_iter = pw._autotune_iters(
        Path("bin"), "op", Path("inp"), Path("hdir"), target_wall, False)
    return iters, per_iter, calls


# --------------------------------------------------------------- the drop bug

@pytest.mark.parametrize("cost_per_iter, label", [
    (3.1, "brotli encode_buffer"),
    (2.6, "brotli shared_dict_roundtrip"),
])
def test_expensive_op_is_measured_instead_of_dropped(monkeypatch, cost_per_iter,
                                                     label):
    """The regression: 200 x 3.1 s > 300 s cap -> iters=0 -> op dropped."""
    iters, per_iter, _ = _autotune(monkeypatch, cost_per_iter)
    assert iters > 0, f"{label} was dropped from the workload"
    assert per_iter == pytest.approx(cost_per_iter, rel=0.05)


def test_probe_never_exceeds_its_time_budget(monkeypatch):
    """Whatever the op costs, no single probe run may exceed the budget —
    that is the property that keeps it under the subprocess cap."""
    for cost in (1e-6, 1e-3, 0.05, 1.4, 3.1, 25.0):
        _iters, _pi, calls = _autotune(monkeypatch, cost)
        for n in calls:
            assert n * cost <= pw.PROBE_BUDGET_S + cost, (
                f"probe of {n} iters at {cost}s/iter overruns the budget")
        assert pw.PROBE_BUDGET_S < TIME_ONCE_CAP_S


def test_a_genuinely_failing_op_still_reports_zero(monkeypatch):
    """The fix must not paper over a real failure: iters=0 is still the signal
    for `op errored on perf input`."""
    monkeypatch.setattr(pw, "_time_once",
                        lambda *_a, **_k: -1.0)
    iters, per_iter = pw._autotune_iters(
        Path("bin"), "op", Path("inp"), Path("hdir"), 1.5, False)
    assert iters == 0
    assert per_iter == -1.0


# ------------------------------------------------------------ the 283 s wall

def test_no_iteration_floor_pins_an_expensive_op_to_a_huge_wall(monkeypatch):
    """`max(200, ...)` made a 1.4 s/iter op measure for 283 s. The invariant is
    the wall, so one iteration is the correct answer once it alone exceeds it."""
    iters, per_iter, _ = _autotune(monkeypatch, 1.4, target_wall=1.5)
    assert iters == 1
    assert iters * per_iter < 5.0


def test_wall_tracks_target_across_the_whole_cost_range(monkeypatch):
    """Either we hit the target wall, or we are at one iteration because a
    single iteration already overshoots it. Never a multiple of the target."""
    target = 1.5
    for cost in (1e-6, 1e-4, 1e-2, 0.1, 1.4, 3.1):
        iters, per_iter, _ = _autotune(monkeypatch, cost, target_wall=target)
        wall = iters * per_iter
        assert wall <= target * 1.5 or iters == 1, (
            f"{cost}s/iter -> {iters} iters -> {wall:.1f}s wall")


def test_cheap_ops_keep_their_large_iteration_counts(monkeypatch):
    """No regression for the healthy case that already worked."""
    iters, _pi, calls = _autotune(monkeypatch, 1e-5, target_wall=1.5)
    assert iters == pytest.approx(150_000, rel=0.2)
    # startup must still be amortized: the probe climbs to the ceiling rung
    assert max(calls) == pw.PROBE_CEIL


def test_fixed_startup_is_not_charged_to_the_per_iteration_cost(monkeypatch):
    """A single averaged probe folds process startup into per_iter. At 4 ms of
    startup and 10 us of work, that reads as 30 us/iter — a 3x over-estimate,
    so the op gets a third of the intended wall. Two points cancel it."""
    _iters, per_iter, _ = _autotune(monkeypatch, 1e-5, startup=0.004)
    assert per_iter == pytest.approx(1e-5, rel=0.02)


def test_slope_falls_back_to_the_average_when_points_invert(monkeypatch):
    """Noise can make the multi-iteration run look cheaper than the single one;
    a negative slope must not turn into a negative or absurd iteration count."""
    def _noisy(_bin, _op, _inp, iters, _hdir, _pin):
        return 1.0 if iters == 1 else 0.5      # inverted on purpose
    monkeypatch.setattr(pw, "_time_once", _noisy)
    iters, per_iter = pw._autotune_iters(
        Path("bin"), "op", Path("inp"), Path("hdir"), 1.5, False)
    assert per_iter > 0
    assert iters >= 1


def test_iteration_count_stays_bounded(monkeypatch):
    """A near-free op must not produce an unbounded iteration count."""
    iters, _pi, _ = _autotune(monkeypatch, 1e-12, target_wall=1.5)
    assert iters <= 100_000_000


# ------------------------------------------------------ candidate-selection probe

def test_candidate_probe_is_cheap_for_an_expensive_op(monkeypatch):
    """`_probe_iters` runs once PER CANDIDATE input. At the old fixed 50-iter
    probe a 3.1 s/iter op spent 155 s on every candidate — minutes per op, and
    the same cap-overrun one rung further up."""
    fake, calls = _fake_clock(3.1)
    monkeypatch.setattr(pw, "_time_once", fake)
    n = pw._probe_iters(Path("bin"), "op", Path("inp"), Path("hdir"), False)
    assert n == 1
    assert sum(c * 3.1 for c in calls) <= pw.PROBE_BUDGET_S * 2


def test_candidate_probe_still_amortizes_startup_when_cheap(monkeypatch):
    fake, _calls = _fake_clock(1e-5)
    monkeypatch.setattr(pw, "_time_once", fake)
    n = pw._probe_iters(Path("bin"), "op", Path("inp"), Path("hdir"), False)
    assert n == pytest.approx(pw.PROBE_TARGET_S / 1e-5, rel=0.2)


def test_candidate_probe_survives_a_failing_candidate(monkeypatch):
    """A candidate the op rejects must not return an iteration count that then
    gets handed to `perf record`."""
    monkeypatch.setattr(pw, "_time_once", lambda *_a, **_k: -1.0)
    assert pw._probe_iters(
        Path("bin"), "op", Path("inp"), Path("hdir"), False) >= 1
