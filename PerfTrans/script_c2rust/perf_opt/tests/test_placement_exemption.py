"""An op that did the SAME work and only moved is placement, not a regression.

The per-op veto exists to stop a candidate that makes one workload slower.
Its placement exemption only recognised an op that retired measurably FEWER
instructions, so the plainest case — instructions flat, wall moved — kept its
veto and discarded the candidate whole.

Measured cost, one such op per candidate, each with instructions flat to
±0.03% (numbers from the runs' own changeset records):

    fzy      legacy-faad1b82   two ops at -15.2% / -20.7%, vetoed by +2.61%
    lodepng  region-546e1d59   one op  at -31.2%,          vetoed by +2.01%
    optipng  ii-inl-6f06775e   one op  at -25.8%,          vetoed by +2.33%

The wall cap keeps the exemption narrow. Replayed over every rejected
candidate in twelve projects, "instructions flat" alone would also exempt ops
at +8%..+32% wall — real regressions (cache, frontend). With the cap, the
replay exempts 22 candidates, worst exempted op +2.81%, and 181 rejections
stand.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.measurement import (
    _PLACEMENT_INSNS_DROP_PCT,
    _PLACEMENT_INSNS_FLAT_PCT,
    _PLACEMENT_WALL_NOISE_PCT,
    _per_op_vetoes,
    is_placement_noise,
)


# ───────────────────────────────── the predicate

@pytest.mark.parametrize("wall,insns", [
    (2.61, -0.01),     # fzy match_primitives
    (2.01, 0.01),      # lodepng chunk_ops
    (2.33, -0.03),     # optipng png_write_read_roundtrip
    (2.50, -1.22),     # lz4 streaming_decode — the original "did less work"
    (12.0, -5.0),      # far over the cap, but measurably less work
])
def test_placement_is_recognised(wall: float, insns: float) -> None:
    assert is_placement_noise(wall, insns)


@pytest.mark.parametrize("wall,insns", [
    (31.58, -0.03),    # libxml2 encoding_suite — flat work, far past the cap
    (22.22, -0.07),    # lil expr_embedded_builtins
    (10.63, -0.28),    # lodepng file_codec: -0.28% is below the flat band and
                       # above the drop floor... it IS less work, so this one
                       # is only excluded once the drop floor is missed
    (2.00, 0.50),      # same wall as the fzy case but real added work
    (2.00, None),      # no counter at all — conservative direction
])
def test_a_real_regression_keeps_its_veto(wall: float, insns: float | None) -> None:
    if insns is not None and insns <= _PLACEMENT_INSNS_DROP_PCT:
        pytest.skip("covered by the original drop floor")
    assert not is_placement_noise(wall, insns)


def test_the_band_and_cap_are_the_stated_ones() -> None:
    assert is_placement_noise(_PLACEMENT_WALL_NOISE_PCT, _PLACEMENT_INSNS_FLAT_PCT)
    assert not is_placement_noise(_PLACEMENT_WALL_NOISE_PCT + 0.01,
                                  _PLACEMENT_INSNS_FLAT_PCT)
    assert not is_placement_noise(1.0, _PLACEMENT_INSNS_FLAT_PCT + 0.01)


# ───────────────────────────────── the veto site uses it

class _Cmp:
    def __init__(self, wall: float, insns: float | None, ci_low: float | None = None):
        self.mean_delta_pct = wall
        self.insns_delta_pct = insns
        self.ci_low_pct = wall if ci_low is None else ci_low


class _Agg:
    def __init__(self, ci_high: float):
        self.ci_high_pct = ci_high


def _vetoes(per_op, agg_ci_high=-5.0, ceiling=1.0, catastrophic=20.0):
    return _per_op_vetoes(
        per_op, _Agg(agg_ci_high), upper_limit_pct=ceiling,
        per_op_limits=None, catastrophic_regress_pct=catastrophic)


def test_the_fzy_candidate_is_no_longer_vetoed() -> None:
    vetoing, exempted = _vetoes({
        "choices_add_search": _Cmp(-15.19, -1.93),
        "choices_fread_search": _Cmp(-20.70, -3.21),
        "match_primitives": _Cmp(2.61, -0.01),
    })
    assert vetoing == []
    assert "match_primitives" in exempted


def test_a_flat_op_far_over_the_cap_still_vetoes() -> None:
    vetoing, exempted = _vetoes({
        "html_entity_encode": _Cmp(-24.33, -36.86),
        "encoding_suite": _Cmp(31.58, -0.03),
    })
    assert vetoing == ["encoding_suite"] and not exempted


def test_no_exemption_without_an_aggregate_gain() -> None:
    vetoing, exempted = _vetoes({"op": _Cmp(2.61, -0.01)}, agg_ci_high=+0.5)
    assert vetoing == ["op"] and not exempted


def test_no_exemption_for_a_collapse() -> None:
    vetoing, exempted = _vetoes({"op": _Cmp(2.61, -0.01)}, catastrophic=2.0)
    assert vetoing == ["op"] and not exempted


def test_an_op_within_its_ceiling_is_not_even_considered() -> None:
    vetoing, exempted = _vetoes({"op": _Cmp(0.5, +5.0)})
    assert vetoing == [] and not exempted


# ─────────────────────── every veto site asks the same question

def test_each_gate_asks_its_own_question_explicitly() -> None:
    """Both STEP veto sites share one predicate; the cumulative gate uses the
    strict one, on purpose.

    `_per_op_vetoes` and the sampling loop's early-reject screen judge a
    candidate against its parent and must agree — they disagreed once by
    accident (lz4: exempted at checkpoints 10 and 20, rejected by the
    max-samples branch at 40). The cumulative gate judges the crate against
    pristine and answers flat differently; pin that split so neither drifts
    back into sharing a predicate with the other side.
    """
    import inspect
    from perf_opt.agent_perf_opt import gates, measurement
    assert inspect.getsource(measurement._per_op_vetoes).count("is_placement_noise") >= 1
    loop = inspect.getsource(measurement.compare_paired)
    assert "is_placement_noise" in loop
    # Match the CALL, not the prose. The comments at that gate name the step
    # gate's predicate on purpose, to say why this one does not use it — so a
    # bare substring search reads its own explanation as a violation.
    cumulative = inspect.getsource(gates.PerformanceSession)
    assert "did_measurably_less_work(" in cumulative
    assert "is_placement_noise(" not in cumulative
