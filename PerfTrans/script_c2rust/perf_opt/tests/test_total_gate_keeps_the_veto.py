"""The single-op veto relaxation belongs to the step gate ONLY.

`_op_limits` hands `inf` to any op whose library share is below the floor:
an op that barely runs the crate cannot speak for it. The reasoning is
`s·x` — an op that is a fraction `s` library turns a library move of `x`
into a wall-clock move of about `s·x`, so a big move on a small-share op
cannot have come from the library.

That reasoning assumes the ONLY channel from the rewrite to the op is
library CPU time. Code layout is not that channel: inlining and code
growth move the WHOLE binary, whatever any one operation runs.

Measured on lil, the same C7 rewrite with and without the original
function's `#[inline]`. An operation that is 12.9% library moved by
+22.55% and +24.36% in two independent runs that kept the attribute
(CI[22.14, 22.96] and CI[23.66, 25.06], n=10), while the run that dropped
it measured -0.70% on that same operation:

    operation              dropped #[inline]   kept #[inline]
    the one that appends        -37.40%           -48.34%
    a second one                -14.13%            -4.81%
    allocation-heavy             -0.70%           +22.55%  <- 12.9% library
    whole crate                 -13.17%            -7.59%

The step gate asks "is this candidate locally faster"; the relaxation
holds there. The total gate asks "did anything crate-wide fall off a
cliff"; handing it the same limits would erase exactly this class of
cross-operation cost, and the crate would have committed the -7.59%
variant over the -13.17% one.

An earlier note in `gates.py` credited the `inf` relaxation with
unblocking lil's -7.48% rewrite. It could not have — that rejection came
from the total gate too. Hence this test: the asymmetry is load-bearing
and must not be "fixed".
"""

from __future__ import annotations

import inspect
import re

from perf_opt.agent_perf_opt import gates


def _gate_ops_source() -> str:
    return inspect.getsource(gates.PerformanceSession.gate_ops)


def _total_gate_call() -> str:
    """The `self._compare(...)` that measures the whole crate vs pristine."""
    src = _gate_ops_source()
    start = src.index("aggregate_only=True")
    head = src.rindex("self._compare(", 0, start)
    return src[head:src.index(")", start) + 1]


def test_the_step_gate_gets_the_relaxed_limits() -> None:
    assert "per_op_limits=self._op_limits(" in _gate_ops_source()


def test_the_total_gate_does_not_get_them() -> None:
    """Not an oversight. See the module docstring."""
    assert "per_op_limits" not in _total_gate_call(), _total_gate_call()


def test_the_total_gate_is_the_only_aggregate_only_comparison() -> None:
    """If a second one appears, this test is checking the wrong call."""
    assert _gate_ops_source().count("aggregate_only=True") == 1


def test_the_asymmetry_is_explained_where_it_lives() -> None:
    """A bare missing argument reads as a bug and gets 'fixed'."""
    call_site = _gate_ops_source()
    assert "per_op_limits" in call_site, "the total gate needs a note, not silence"
    assert "22.55" in call_site or "inline" in call_site


def test_the_relaxation_still_hands_out_infinity() -> None:
    """The step-gate half of the story, unchanged."""
    src = inspect.getsource(gates.PerformanceSession._op_limits)
    assert "math.inf" in src
    assert "_LIBRARY_SHARE_FLOOR" in src


def test_the_corrected_note_does_not_reclaim_the_lil_number() -> None:
    """The `-7.48%` rejection came from the total gate, which never saw these
    limits. Citing it here would be attributing a result to a mechanism that
    could not have produced it."""
    src = inspect.getsource(gates.PerformanceSession._op_limits)
    claim = re.search(r"Measured on lil.{0,200}?7\.48", src, re.S)
    assert claim is None, "the misattributed evidence is back"
