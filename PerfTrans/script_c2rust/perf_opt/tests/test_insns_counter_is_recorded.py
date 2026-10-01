"""Instruction counts ride along with wall time, and change no verdict.

Wall time cannot distinguish the two ways an operation gets slower. The
candidate may do more work — more instructions retired — or it may do exactly
the same work in a binary the linker laid out differently, which moves i-cache
lines and branch-predictor entries for operations the edit never touched.

Measured on binaries this gate had itself used:

    optipng  png_palette_roundtrip   +3.162% wall   -0.987% insns   placement
    optipng  bitset_and_ratio        +1.789% wall   -0.066% insns   placement
    libxml2  buf_strings_chars       +6.037% wall   -0.055% insns   placement
    libxml2  html_entity_encode     -34.815% wall  -26.183% insns   real win

Without the second column a rejection reads the same in both cases, and the
only way to tell them apart afterwards costs a re-measurement.

The counter used to be recorded and read by nothing. Since 2026-09-13 it has
exactly one job in a verdict, argued in
`test_the_counter_can_only_withhold_a_veto_never_create_one` below: an op
whose instruction count FELL — or, since 2026-09-16, stayed FLAT while its
wall moved less than layout can explain — loses its individual veto on the
step gate. It still cannot create a rejection, and every threshold that can
reject is still tuned against wall time.

The cumulative gate in `gates.py` deliberately keeps the narrower rule (fell,
not flat): flat steps are what creep is made of, and it is the only gate that
adds them up.
"""

from __future__ import annotations

from pathlib import Path

from perf_opt.agent_perf_opt.measurement import (
    OpComparison,
    PairSample,
    WallMs,
    _summarize_ops,
)


def test_wall_ms_is_a_float_everywhere_a_float_was_expected() -> None:
    value = WallMs(1500.5, 11_220_931_236)
    assert isinstance(value, float)
    assert float(value) == 1500.5
    assert value + 1 == 1501.5
    assert value.instructions == 11_220_931_236


def test_a_backend_returning_a_plain_float_still_works() -> None:
    """Every test stub in the suite does this."""
    sample = PairSample(0, 1500.0, 1590.0)
    assert sample.delta_pct == 6.0
    assert sample.insns_delta_pct is None

    summary = _summarize_ops({"op": [sample]})["op"]
    assert summary.mean_delta_pct == 6.0
    assert summary.insns_delta_pct is None


def test_layout_movement_reads_as_wall_without_instructions() -> None:
    """libxml2 `buf_strings_chars`: +6.04% wall on -0.06% instructions."""
    samples = [
        PairSample(i, 1467.6, 1556.2,
                   parent_insns=11_220_931_236,
                   candidate_insns=11_214_762_724)
        for i in range(4)
    ]
    summary = _summarize_ops({"buf_strings_chars": samples})["buf_strings_chars"]
    assert summary.mean_delta_pct > 5.0
    assert abs(summary.insns_delta_pct) < 0.5


def test_a_real_regression_moves_both_numbers() -> None:
    samples = [
        PairSample(i, 1000.0, 1260.0,
                   parent_insns=10_000_000_000,
                   candidate_insns=12_500_000_000)
        for i in range(4)
    ]
    summary = _summarize_ops({"op": samples})["op"]
    assert summary.mean_delta_pct == 26.0
    assert summary.insns_delta_pct == 25.0


def test_the_counter_can_only_withhold_a_veto_never_create_one() -> None:
    """The one place a verdict reads the instruction delta, and its direction.

    Replaces `test_the_counter_is_not_part_of_any_verdict`. That test asked
    for this change to be argued here, so:

    The per-op ceiling stops a rewrite from buying its aggregate win by
    wrecking one operation. It reads wall time, and wall time rises for two
    unrelated reasons — the rewrite made that op do more work, or the commit
    relinked the binary and the op's hot loop landed somewhere worse. The
    second is a lottery every accepted commit re-runs for every other op, and
    it is not the rewrite's fault.

    Measured on lz4 (C11 goto-dispatch split; 16 ops in the step gate, 15 of
    them faster, aggregate -1.936% with CI [-1.978, -1.895]):

        streaming_decode      wall +2.454%   insns -1.2303%   <- sole veto
        decompress_usingdict  wall -6.653%   insns -4.3227%
        extdict_decompress    wall -4.793%   insns -4.5188%
        compress_decompress   wall -3.271%   insns -5.6342%

    The vetoing op retired 1.2% FEWER instructions and still took longer.
    Reproduced independently with perf (r=5): insns 3988.2M -> 3940.1M,
    cycles 2273.8M -> 2313.1M, DSB uops 2164M -> 1910M with MITE taking over
    (uop-cache share 45.6% -> 40.6%), L1i tag misses lower, branch misses
    unchanged. Placement, not work — and it vetoed fifteen real wins.

    So the delta may WITHHOLD a veto. It may never cast one: an op with no
    counter, or with instructions up, is judged exactly as before. That
    asymmetry is the property this test pins, because it is what keeps every
    rejection threshold tuned against wall time.
    """
    from perf_opt.agent_perf_opt.measurement import AggregateSummary, _decision

    def op(name, wall, insns):
        return OpComparison(
            op=name, samples=(), mean_delta_pct=wall,
            ci_low_pct=wall - 0.2, ci_high_pct=wall + 0.2,
            insns_delta_pct=insns, sample_count=10,
        )

    gain = AggregateSummary(-1.936, -1.978, -1.895, 0.02, 100.0)
    wash = AggregateSummary(-0.02, -0.06, +0.02, 0.02, 100.0)
    limits = dict(regress_tolerance_pct=0.3, upper_limit_pct=1.0)

    over = {"slow": op("slow", +2.454, -1.2303), "fast": op("fast", -6.653, -4.32)}
    assert _decision(over, gain, **limits) != "per_op_regress"

    # Flat instructions inside the wall cap are placement too: the op did the
    # same work and only moved. Three measured candidates turned on this —
    # see `test_placement_exemption_loop.test_flat_instructions_are_exempt`.
    flat = {"slow": op("slow", +2.454, -0.05)}
    assert _decision(flat, gain, **limits) != "per_op_regress"

    # …and only there. Each of these still rejects.
    for name, per_op, agg in (
        ("instructions up", {"slow": op("slow", +2.454, +1.5)}, gain),
        ("flat but far past the wall cap", {"slow": op("slow", +8.0, -0.05)}, gain),
        ("no counter at all", {"slow": op("slow", +2.454, None)}, gain),
        ("aggregate is a wash", dict(over), wash),
        ("op collapsed", {"slow": op("slow", +7.2, -1.23)}, gain),
    ):
        assert _decision(per_op, agg, **limits) == "per_op_regress", name

    # The withholding is never a licence: strip the counter from every op and
    # the verdict may only get stricter, never looser.
    stripped = {k: op(v.op, v.mean_delta_pct, None) for k, v in over.items()}
    assert not (
        _decision(stripped, gain, **limits) != "per_op_regress"
        and _decision(over, gain, **limits) == "per_op_regress"
    )


def test_the_counter_survives_the_state_annotation_rebuild() -> None:
    """The annotation pass rebuilds each comparison field by field.

    Anything it does not name explicitly reverts to its default. The delta was
    computed correctly and reached the serializer as `None` for exactly this
    reason, with the samples it comes from still attached to the same object —
    a shape that unit-testing `_summarize_ops` alone cannot catch.
    """
    from perf_opt.agent_perf_opt.measurement import _annotate_comparisons

    samples = [
        PairSample(i, 1000.0, 1060.0,
                   parent_insns=10_000_000_000,
                   candidate_insns=10_000_500_000)
        for i in range(4)
    ]
    summarized = _summarize_ops({"op": samples})
    assert summarized["op"].insns_delta_pct is not None

    annotated = _annotate_comparisons(
        summarized, {"op": 0}, {}, 1.0,
    )
    assert annotated["op"].insns_delta_pct == summarized["op"].insns_delta_pct
    assert annotated["op"].state is not None


def test_op_comparison_defaults_keep_old_constructions_valid() -> None:
    """Positional construction elsewhere must not shift onto the new field."""
    comparison = OpComparison(
        op="op", samples=(), mean_delta_pct=1.0,
        ci_low_pct=0.5, ci_high_pct=1.5,
    )
    assert comparison.insns_delta_pct is None
    assert comparison.sample_count == 0
    assert comparison.priority == 0
