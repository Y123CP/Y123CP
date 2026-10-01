"""An op that mostly times the harness must not veto a faster crate.

`test_total_gate_single_op_collapse` added the collapse check that stops a
rewrite from wrecking an op no gate was watching. This is the other half: the
check has to skip the ops the caller already disarmed, or it re-arms them at
exactly the threshold they were exempted from.

Measured on lil. Two rewrites of `lil_append_char` made the crate **7.48% and
7.67% FASTER** and were both recorded as `w2_regress`. The previous run had
committed the same function at -15.0%. The whole run ended 0 commit against
that run's -11.55%. The veto came from `expr_embedded_builtins`, an op whose
library share is 12.5% — 87% of what it times is harness and allocator.

Two separate defects produced that:

  1. `_op_limits` handed a disarmed op `catastrophic_regress_pct`, and the
     collapse check reads that same constant — so "loses its veto" left it
     vetoing at the unchanged threshold.
  2. `_measured_library_share` summed only `hot_functions`, dropping the
     `dropped_wrappers` entries. On lil that reads 0.2% instead of 12.5%,
     because the op's two hottest crate functions both delegate to libc and
     live on the dropped list. Under-reading a share does not preserve a veto
     — it falls below the floor and removes one.
"""

from __future__ import annotations

import json
import math

from perf_opt.agent_perf_opt.gates import (
    _LIBRARY_SHARE_FLOOR,
    _measured_library_share,
)
from perf_opt.agent_perf_opt.measurement import _limit_for


class _Op:
    def __init__(self, low, high=None):
        self.ci_low_pct = low
        self.ci_high_pct = high if high is not None else low


# ───────────────────────────── the exemption must actually exempt

def test_a_disarmed_op_is_skipped_by_the_collapse_check() -> None:
    """`inf` from `_op_limits` has to survive into the collapse threshold."""
    limits = {"harness_bound_op": math.inf}
    assert _limit_for("harness_bound_op", 5.0, limits) == math.inf
    # 12% — the shape that legitimately collapses a real op — still does not
    # clear `inf`, which is the whole point of disarming.
    assert not _Op(12.0).ci_low_pct > _limit_for("harness_bound_op", 5.0, limits)


def test_an_armed_op_still_collapses_at_the_catastrophic_threshold() -> None:
    """The disarm must not leak into ops that kept their standing."""
    limits = {"other_op": math.inf}
    assert _limit_for("real_op", 5.0, limits) == 5.0
    assert _Op(12.0).ci_low_pct > _limit_for("real_op", 5.0, limits)


def test_the_catastrophic_constant_is_not_what_disarms_an_op() -> None:
    """Regression: handing out `catastrophic_regress_pct` as the exemption is
    indistinguishable from no exemption, because the collapse check compares
    against that very number."""
    as_before = {"op": 5.0}
    assert _limit_for("op", 5.0, as_before) == 5.0, \
        "5.0 as an 'exemption' leaves the op vetoing at 5.0 — not an exemption"


# ───────────────────────────── the share must be read completely

def _hotspots(tmp_path, hot, dropped):
    (tmp_path / "hotspots.json").write_text(json.dumps({
        "crate": "c", "tau": 3.0, "ops": ["op_a"],
        "hot_functions": hot, "dropped_wrappers": dropped,
    }), encoding="utf-8")
    return tmp_path


def test_dropped_wrappers_count_toward_the_library_share(tmp_path) -> None:
    """lil's actual numbers: 0.2% from hot_functions, 12.3% from the wrappers.

    `locate` measures the same op at 13% with a full profile, so the sum is
    the honest reading and `hot_functions` alone is not.
    """
    d = _hotspots(
        tmp_path,
        hot=[{"name": "hm_hash", "per_op": {"op_a": 0.2}}],
        dropped=[{"name": "lil_clone_value", "per_op": {"op_a": 7.5},
                  "extern_wrapper": True, "wrapper_target": "memcpy"},
                 {"name": "alloc_value_len", "per_op": {"op_a": 4.8},
                  "extern_wrapper": True, "wrapper_target": "memcpy"}],
    )
    share = _measured_library_share(d)
    assert share["op_a"] == pytest_approx(0.125), share
    # and the reading that ignored the wrappers would have been 65x smaller
    assert share["op_a"] > 0.002 * 50


def test_a_missing_dropped_wrappers_key_is_not_an_error(tmp_path) -> None:
    (tmp_path / "hotspots.json").write_text(json.dumps({
        "hot_functions": [{"name": "f", "per_op": {"op_a": 90.0}}],
    }), encoding="utf-8")
    assert _measured_library_share(tmp_path)["op_a"] == pytest_approx(0.90)


# ───────────────────────────── the floor separates the two populations

def test_the_floor_sits_between_the_measured_populations() -> None:
    """Not a knob turned until a project passed.

    Harness-bound (must lose the veto): lil 12.5%, fzy 16.9% / 17.2%.
    Library-bound (must keep it): zopfli's cache_roundtrip 49.8% — the op the
    collapse check was built for — through 96.6%.
    """
    must_lose = (0.125, 0.169, 0.172)
    must_keep = (0.498, 0.802, 0.871, 0.935, 0.962, 0.963, 0.966)
    for s in must_lose:
        assert s < _LIBRARY_SHARE_FLOOR, s
    for s in must_keep:
        assert s >= _LIBRARY_SHARE_FLOOR, s
    # the gap the floor sits in, so a later edit cannot quietly narrow it
    assert max(must_lose) < _LIBRARY_SHARE_FLOOR <= min(must_keep)


def pytest_approx(v):
    import pytest
    return pytest.approx(v, rel=1e-6)
