"""A rewrite can wreck an op that no gate was watching.

The two gates divided the work and left a hole between them:

  * the **step gate** measures only the ops the rewritten function appears in
    (`hf.per_op`), and
  * the **total gate** measures every op, but judged on the aggregate alone —
    its comment said single-op movement was "the step gate's job".

It is not the step gate's job when the op is not in `hf.per_op`. Measured: a
rewrite of a function whose profile reads {lz77 4.74%, raw_deflate 7.16%,
zlib_compress 7.27%} — no cache operation in it at all — made the cache
operation **12.12% slower**, verified by rebuilding that single commit and
timing it (the other seven commits of the run moved it +0.3% combined).

Neither gate saw it. The step gate never measured that op. In the five-op
equal-weight aggregate the 12% diluted to ~2.4% and was cancelled by -13% on
the two main ops, so the total gate read "pass". The rewrite committed, and
the run's final measurement came back **+9.62%** on that op with
`accepted: false` for the whole run — two main ops at their best-ever -13%,
thrown away.

The fix keeps the original intent (small per-op wobble is absorbed by the
aggregate, so noise cannot veto a good rewrite) and adds only a floor: an op
that collapses is not wobble.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.config import AgentConfig
from perf_opt.agent_perf_opt.measurement import _decision


class _Op:
    def __init__(self, low, high=None):
        self.ci_low_pct = low
        self.ci_high_pct = high if high is not None else low


class _Agg:
    """`_aggregate_state` reads these three."""
    def __init__(self, low, high):
        self.ci_low_pct = low
        self.ci_high_pct = high
        self.mean_delta_pct = (low + high) / 2


def _verdict(per_op, agg_low, agg_high, *, aggregate_only=True, catastrophic=5.0):
    return _decision(
        per_op, _Agg(agg_low, agg_high),
        regress_tolerance_pct=0.3, upper_limit_pct=1.0,
        catastrophic_regress_pct=catastrophic, aggregate_only=aggregate_only)


# ───────────────────────────────── the regression

def test_a_collapsed_op_is_refused_even_when_the_aggregate_passes() -> None:
    """The measured shape: one op at +12%, two main ops at -13%, aggregate
    comfortably negative."""
    per_op = {
        "cache_roundtrip": _Op(12.1),
        "zlib_compress": _Op(-13.2),
        "raw_deflate": _Op(-13.3),
        "lz77_blocksplit_analysis": _Op(-0.8),
        "tree_codegen": _Op(0.5),
    }
    assert _verdict(per_op, -4.0, -3.0) == "per_op_regress"


def test_the_same_shape_without_the_collapse_still_passes() -> None:
    """Control: the fix must not reject the run it was extracted from."""
    per_op = {
        "cache_roundtrip": _Op(-2.0),
        "zlib_compress": _Op(-13.2),
        "raw_deflate": _Op(-13.3),
        "lz77_blocksplit_analysis": _Op(-0.8),
        "tree_codegen": _Op(0.5),
    }
    assert _verdict(per_op, -6.0, -5.0) == "accepted"


# ───────────────────────────────── the original intent is preserved

@pytest.mark.parametrize("wobble", [0.5, 1.5, 3.0, 4.9])
def test_ordinary_per_op_wobble_is_still_absorbed(wobble) -> None:
    """Code layout alone moves untouched ops by 0.79% median, 4.12% worst
    (measured across seven commits that never touched the op's source). The
    total gate must keep tolerating that, or every commit becomes a coin
    flip."""
    per_op = {"a": _Op(wobble), "b": _Op(-5.0)}
    assert _verdict(per_op, -3.0, -2.0) == "accepted"


def test_the_threshold_is_the_catastrophic_one_not_the_per_op_ceiling() -> None:
    """This gate exists to catch collapse, not to re-apply the step gate's
    tolerances — otherwise it would veto on drift the step gate already
    accepted."""
    assert AgentConfig.w2_catastrophic_regress_pct > \
        AgentConfig.w2_per_op_upper_limit_pct
    per_op = {"a": _Op(2.0), "b": _Op(-5.0)}                        
    assert _verdict(per_op, -3.0, -2.0) == "accepted"


# ───────────────────────────────── the step gate is untouched

def test_the_step_gate_still_judges_every_op_it_measures() -> None:
    per_op = {"a": _Op(2.0), "b": _Op(-5.0)}
    assert _verdict(per_op, -3.0, -2.0, aggregate_only=False) == "per_op_regress"


def test_an_aggregate_regression_still_loses_regardless() -> None:
    per_op = {"a": _Op(0.1), "b": _Op(0.2)}
    assert _verdict(per_op, 0.4, 0.9) == "no_gain"


# ───────────────────────────────── the knob is live

def test_the_catastrophic_threshold_actually_drives_the_decision() -> None:
    """It had been accepted by the constructor and immediately `del`-ed; a
    dead knob here would silently restore the hole."""
    per_op = {"a": _Op(6.0), "b": _Op(-9.0)}
    assert _verdict(per_op, -3.0, -2.0, catastrophic=5.0) == "per_op_regress"
    assert _verdict(per_op, -3.0, -2.0, catastrophic=20.0) == "accepted"


def test_compare_paired_no_longer_discards_the_threshold(  ) -> None:
    from pathlib import Path
    src = (Path(__file__).resolve().parents[1] / "agent_perf_opt"
           / "measurement.py").read_text(encoding="utf-8")
    assert "del catastrophic_regress_pct" not in src
