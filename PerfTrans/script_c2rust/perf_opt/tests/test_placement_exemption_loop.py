"""The placement exemption, judged by the whole sampling loop, not one helper.

The first version of the exemption lived inline in `_decision` and its tests
called `_decision` directly. They passed. The real run still rejected the
candidate, because three other places applied the per-op ceiling on their own:

  * `_decision`'s `all_safe`, which an exempted op could never satisfy, so the
    step gate could only ever answer "continue";
  * the scheduler, which kept extending the exempted op 10 -> 20 -> 40;
  * the max-samples fallback in `compare_paired`, which compared ci_low to the
    flat ceiling and rejected at 40 on the numbers `_decision` had cleared at
    10 and 20.

Measured on lz4 (C11 goto-dispatch split), identical at every checkpoint:
streaming_decode wall +2.5%, insns -1.22%, aggregate -1.94%. Exempted at 10,
exempted at 20, `per_op_regress` at 40.

A fourth, the five-pair early-reject screen, applied the raw ceiling too.
And `_cumulative_op_regress` in gates.py was next in line after all of them.

So these tests drive `compare_paired` end to end with a fake backend that
returns instruction counts, and the cumulative gate on the same numbers.
"""

from __future__ import annotations

import itertools
from pathlib import Path
from types import SimpleNamespace

from perf_opt.agent_perf_opt.gates import PerformanceSession
from perf_opt.agent_perf_opt.measurement import (
    OpComparison,
    WallMs,
    compare_paired,
)

_BASE_INSNS = 1_000_000_000


class _Backend:
    """Per-(binary, op) timings carrying instruction counts; a list cycles."""

    def __init__(self, table: dict) -> None:
        self.table = {
            key: itertools.cycle(v) if isinstance(v, list) else itertools.repeat(v)
            for key, v in table.items()
        }
        self.measure_calls: list[tuple[str, str]] = []

    def prepare(self, binary, op, assets, target_wall):
        return Path(f"{op}.bin"), 100

    def measure_once(self, binary, op, input_path, iters, pin_cpu):
        key = (Path(binary).name, op)
        self.measure_calls.append(key)
        return next(self.table[key])


def _table(spec: dict[str, tuple[float, float]]) -> dict:
    """spec: op -> (wall delta %, insns delta %) of candidate vs parent."""
    table = {}
    for op, (wall, insns) in spec.items():
        table[("parent", op)] = WallMs(100.0, _BASE_INSNS)
        table[("candidate", op)] = WallMs(
            100.0 * (1 + wall / 100), int(_BASE_INSNS * (1 + insns / 100)))
    return table


def _run(tmp_path: Path, spec, **kw):
    backend = _Backend(_table(spec))
    result = compare_paired(
        backend, tmp_path / "parent", tmp_path / "candidate",
        ops=tuple(spec), assets=None, **kw,
    )
    return result, backend


# The lz4 shape: one op slower on fewer instructions, the rest clearly faster.
_LZ4 = {
    "streaming_decode": (+2.5, -1.22),
    "decompress_usingdict": (-6.65, -4.32),
    "extdict_decompress": (-4.79, -4.52),
}


def test_the_loop_accepts_what_the_helper_exempted(tmp_path: Path) -> None:
    result, _ = _run(tmp_path, _LZ4)
    assert result.accepted, result.reason
    assert result.reason == "accepted"


def test_an_exempted_op_is_not_extended_to_the_maximum(tmp_path: Path) -> None:
    _, backend = _run(tmp_path, _LZ4)
    calls = backend.measure_calls.count(("candidate", "streaming_decode"))
    assert calls < 20, f"exempted op was still extended: {calls} candidate runs"


def test_the_max_samples_branch_honours_the_exemption(tmp_path: Path) -> None:
    """Drive the loop to its last checkpoint with the exempted op present.

    With the lz4 ops alone `_decision` accepts at checkpoint 10 and the
    fallback that rejected lz4 is never reached. A second op whose CI still
    straddles the ceiling at 40 pairs keeps `_decision` at "continue", so the
    verdict has to come from the max-samples branch — and the scheduler has
    to keep extending the straddling op without dragging the exempted one
    along.
    """
    table = _table(_LZ4)
    table[("parent", "noisy")] = WallMs(100.0, _BASE_INSNS)
    # Alternating -2.0% / +3.2%: mean +0.6%, CI straddles 1.0% even at 40.
    table[("candidate", "noisy")] = [
        WallMs(98.0, _BASE_INSNS), WallMs(103.2, _BASE_INSNS)]
    backend = _Backend(table)
    result = compare_paired(
        backend, tmp_path / "parent", tmp_path / "candidate",
        ops=tuple(_LZ4) + ("noisy",), assets=None,
    )
    noisy = result.per_op["noisy"]
    assert noisy.ci_low_pct < 1.0 < noisy.ci_high_pct   # precondition holds
    assert result.max_checkpoint_reached >= 40         # really the last one
    assert result.accepted, result.reason
    assert backend.measure_calls.count(("candidate", "streaming_decode")) < 20


def test_flat_instructions_are_exempt_in_the_loop(tmp_path: Path) -> None:
    """Flat instructions, wall inside the cap: placement, so the step gate
    lets the candidate through.

    This assertion is inverted from its first version, which demanded a
    rejection on exactly these numbers. That version was argued from a
    SYNTHETIC op — at the time no measured candidate sat at (+2.5%, -0.05%),
    and the only measured evidence was lz4's -1.22%, which says nothing about
    flat. Three measured candidates later landed there, each one a high-value
    rewrite that a single such op discarded whole:

        fzy      legacy-faad1b82  two ops -15.2% / -20.7%, vetoed by +2.61%
        lodepng  region-546e1d59  one op  -31.2%,          vetoed by +2.01%
        optipng  ii-inl-6f06775e  one op  -25.8%,          vetoed by +2.33%

    Their vetoing ops sat at ±0.03% instructions — numerically inseparable
    from the synthetic case, so no threshold could keep one and drop the
    other. The wall cap is what keeps the exemption narrow (a flat op past it
    still vetoes, below), and the cumulative gate still adds flat steps up:
    see `test_the_two_gates_answer_flat_differently`.
    """
    spec = dict(_LZ4, streaming_decode=(+2.5, -0.05))
    result, _ = _run(tmp_path, spec)
    assert result.accepted, result.reason


def test_flat_instructions_past_the_wall_cap_still_reject(tmp_path: Path) -> None:
    """libxml2 `encoding_suite` shape: flat work, wall far past what layout
    explains. Replayed over twelve projects, dropping the cap would have
    exempted ops at +8% to +32% — real cache/frontend regressions."""
    spec = dict(_LZ4, streaming_decode=(+8.0, -0.05))
    result, _ = _run(tmp_path, spec)
    assert not result.accepted
    assert result.reason == "per_op_regress"


def test_a_wash_aggregate_still_rejects_in_the_loop(tmp_path: Path) -> None:
    spec = {
        "streaming_decode": (+2.5, -1.22),
        "a": (-1.0, -0.5),
        "b": (-1.0, -0.5),
    }
    result, _ = _run(tmp_path, spec)
    assert not result.accepted
    assert result.reason == "per_op_regress"


def test_a_collapse_still_rejects_in_the_loop(tmp_path: Path) -> None:
    spec = dict(_LZ4, streaming_decode=(+7.2, -1.22))
    result, _ = _run(tmp_path, spec)
    assert not result.accepted
    assert result.reason == "per_op_regress"


def test_the_early_screen_defers_on_fewer_instructions(tmp_path: Path) -> None:
    result, _ = _run(tmp_path, _LZ4, high_risk_ops=("streaming_decode",))
    assert result.accepted, result.reason
    # Measured past the five-pair screen: every op made it into the verdict.
    assert set(result.per_op) == set(_LZ4)


def test_the_early_screen_defers_on_flat_instructions(
    tmp_path: Path,
) -> None:
    """The five-pair screen has to ask the step gate's question, not its own.

    It is the site that killed lz4 at checkpoint 40 after the helper had
    cleared it at 10 and 20, so it gets its own case for every widening.
    """
    spec = dict(_LZ4, streaming_decode=(+2.5, -0.05))
    result, _ = _run(tmp_path, spec, high_risk_ops=("streaming_decode",))
    assert result.accepted, result.reason
    # Not killed by the screen: every op made it into the verdict.
    assert set(result.per_op) == set(_LZ4)


# ─────────────────────────────────────────── the cumulative gate, same numbers


def _session(tmp_path: Path) -> PerformanceSession:
    binary = tmp_path / "harness"
    binary.write_bytes(b"\x7fELF-stub")
    binary.chmod(0o755)
    session = PerformanceSession(
        pristine_bin=binary, opt_dir=tmp_path, all_ops=tuple(_LZ4))
    # streaming_decode was 90% library on lz4: the strict 1.0% ceiling.
    session.library_share = {op: 0.90 for op in _LZ4}
    return session


def _total(slow_insns, agg_ci_high=-1.91):
    per_op = {
        op: OpComparison(
            op=op, samples=(), mean_delta_pct=wall,
            ci_low_pct=wall - 0.2, ci_high_pct=wall + 0.2,
            insns_delta_pct=(slow_insns if op == "streaming_decode" else insns),
        )
        for op, (wall, insns) in _LZ4.items()
    }
    return SimpleNamespace(per_op=per_op, aggregate_ci_high_pct=agg_ci_high)


def test_cumulative_gate_exempts_the_same_placement(tmp_path: Path) -> None:
    assert _session(tmp_path)._cumulative_op_regress(_total(-1.22)) is None


def test_cumulative_gate_still_catches_work_creep(tmp_path: Path) -> None:
    session = _session(tmp_path)
    for insns in (+0.8, -0.05, None):
        worst = session._cumulative_op_regress(_total(insns))
        assert worst is not None and worst[0] == "streaming_decode", insns


def test_the_two_gates_answer_flat_differently(tmp_path: Path) -> None:
    """One op, flat instructions: the step gate exempts it, this one does not.

    The asymmetry is the point, not an oversight. Against a single parent,
    flat means the rewrite never touched that op — placement. Against pristine
    across every commit so far, flat is what creep looks like on the way up:
    a run of steps each small enough to clear the step gate, and this is the
    only gate positioned to add them together. So the two call deliberately
    different predicates, and this pins that they keep doing so.
    """
    from perf_opt.agent_perf_opt.measurement import (
        did_measurably_less_work,
        is_placement_noise,
    )

    assert is_placement_noise(+2.5, -0.05)          # step gate: placement
    assert not did_measurably_less_work(-0.05)      # cumulative gate: not
    worst = _session(tmp_path)._cumulative_op_regress(_total(-0.05))
    assert worst is not None and worst[0] == "streaming_decode"

    # The measured case both still agree on.
    assert is_placement_noise(+2.5, -1.22) and did_measurably_less_work(-1.22)
    assert _session(tmp_path)._cumulative_op_regress(_total(-1.22)) is None


def test_cumulative_gate_needs_the_crate_to_be_ahead(tmp_path: Path) -> None:
    worst = _session(tmp_path)._cumulative_op_regress(
        _total(-1.22, agg_ci_high=+0.05))
    assert worst is not None and worst[0] == "streaming_decode"


# ─────────────────── the exemption cannot spend an instruction drop twice
#
# After the C11 split committed, streaming_decode sat at -1.22% instructions
# against pristine, and every later candidate's total gate saw that number.
# Judged against pristine, a later candidate could add up to ~0.95% more work
# to the op and still read "fewer instructions than base" — the cumulative
# gate, whose whole purpose is catching creep across commits, would wave it
# through. Past the first commit the exemption is judged against the op's
# instruction count at the last commit instead.


def test_no_committed_state_judges_against_pristine(tmp_path: Path) -> None:
    session = _session(tmp_path)
    session._committed_op_insns = None
    assert session._cumulative_op_regress(_total(-1.22)) is None


def test_unchanged_since_the_commit_stays_exempt(tmp_path: Path) -> None:
    session = _session(tmp_path)
    session._committed_op_insns = {"streaming_decode": -1.22}
    assert session._cumulative_op_regress(_total(-1.18)) is None


def test_a_banked_drop_cannot_hide_later_work(tmp_path: Path) -> None:
    """-1.22% at the commit, -0.40% now: 0.8% more work, still below base."""
    session = _session(tmp_path)
    session._committed_op_insns = {"streaming_decode": -1.22}
    worst = session._cumulative_op_regress(_total(-0.40))
    assert worst is not None and worst[0] == "streaming_decode"


def test_promote_carries_the_op_instructions_forward(tmp_path: Path) -> None:
    session = _session(tmp_path)
    session._pending_op_insns = {"streaming_decode": -1.22}
    session.promote_candidate(tmp_path / "harness", "8f5489a")
    assert session._committed_op_insns == {"streaming_decode": -1.22}
    assert session._pending_op_insns is None
