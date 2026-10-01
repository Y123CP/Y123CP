"""Sizing `iters` must not charge process startup to the first iteration.

Every probe run pays a cost unrelated to `iters`: process start, paging in the
harness, reading the input. `autotune_iters` divided one run's wall time by its
iteration count, which is correct only while that cost is negligible — an
assumption the ×8 growth loop happened to arrange, since it kept growing until
a run reached 0.3 s.

It stops growing the moment a run clears 0.3 s. When the FIRST probe already
clears it, the loop exits at `probe=1` and the entire run becomes the
per-iteration estimate. Measured on libxml2 with a cold page cache: four
operations were sized at 1-4 iterations against the 5000-200000 they need,
running for about 1 ms instead of 1.5 s, cv degrading from 0.07% to 1.2%. Those
operations still carry a veto in the total gate, so the noise decides
candidates.

Warm, the same binaries put the fixed cost at 4 ms and the naive per-iteration
figure 19x to 582x high — the identical error, sitting just under the threshold
that makes it visible.
"""

from __future__ import annotations

import pytest

from perf_opt.verify import measure


def _fake_clock(monkeypatch, *, fixed_s: float, per_iter_s: float):
    """Time a run as `fixed_s + iters * per_iter_s`, and count the probes."""
    calls: list[int] = []

    def timer(binary, op, input_path, iters, *, pin_cpu=None, timeout=120.0):
        calls.append(iters)
        return fixed_s + iters * per_iter_s

    monkeypatch.setattr(measure, "_time_once", timer)
    return calls


def test_a_large_fixed_cost_no_longer_collapses_iters(monkeypatch) -> None:
    """The libxml2 shape: 0.5 s of startup, 0.216 ms of actual work per iter."""
    _fake_clock(monkeypatch, fixed_s=0.5, per_iter_s=0.000216)
    iters = measure.autotune_iters(
        object(), "entities_tree_valid", object(), target_wall=1.5,
    )
    # Dividing the first probe straight through gave 1.5 / 0.500216 = 2.
    assert iters == pytest.approx(1.5 / 0.000216, rel=0.02)
    assert iters > 6000


def test_a_small_fixed_cost_gives_the_same_answer(monkeypatch) -> None:
    """Warm cache, where the old code already worked. It must not shift."""
    _fake_clock(monkeypatch, fixed_s=0.004, per_iter_s=0.000216)
    iters = measure.autotune_iters(object(), "op", object(), target_wall=1.5)
    assert iters == pytest.approx(1.5 / 0.000216, rel=0.02)


def test_a_very_light_iteration_is_sized_correctly(monkeypatch) -> None:
    """`globals_threads_memory`: 7.5 us per iteration, 582x naive overstatement."""
    _fake_clock(monkeypatch, fixed_s=0.0043, per_iter_s=0.0000075)
    iters = measure.autotune_iters(object(), "op", object(), target_wall=1.5)
    assert iters == pytest.approx(200_000, rel=0.05)


def test_a_slow_workload_still_resolves_small_and_early(monkeypatch) -> None:
    """Seconds per iteration must not make the probe loop run away.

    The old loop detected this at probe=1. The differenced one needs a second
    point, so it must reach the answer on the very next probe rather than
    growing into a timeout.
    """
    calls = _fake_clock(monkeypatch, fixed_s=0.01, per_iter_s=3.0)
    iters = measure.autotune_iters(object(), "op", object(), target_wall=1.5)
    assert iters == 1
    assert max(calls) <= 2, f"probed too far for a 3 s/iter workload: {calls}"


def test_the_warmup_probe_is_discarded(monkeypatch) -> None:
    """A cold first execution must not enter the estimate.

    Without the discard, `fixed` is the cold run and the later probes are warm,
    so the difference comes out negative or absurdly small.
    """
    seen: list[int] = []
    first = {"done": False}

    def timer(binary, op, input_path, iters, *, pin_cpu=None, timeout=120.0):
        seen.append(iters)
        if not first["done"]:
            first["done"] = True
            return 0.5          # cold: page-in, dominated by fixed cost
        return 0.004 + iters * 0.000216

    monkeypatch.setattr(measure, "_time_once", timer)
    iters = measure.autotune_iters(object(), "op", object(), target_wall=1.5)
    assert seen[0] == 1 and seen[1] == 1, "the first probe is warm-up, not data"
    assert iters == pytest.approx(1.5 / 0.000216, rel=0.02)


def test_a_failing_probe_still_returns_zero(monkeypatch) -> None:
    monkeypatch.setattr(
        measure, "_time_once",
        lambda *a, **k: -1.0,
    )
    assert measure.autotune_iters(object(), "op", object()) == 0
