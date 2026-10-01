"""Paired parent/candidate performance measurement.

The statistical decision code depends only on ``MeasurementBackend``.  Real
``perf stat`` execution is isolated in ``PerfStatBackend`` so tests and future
remote runners can provide deterministic backends.
"""

from __future__ import annotations

import math
import logging
from dataclasses import dataclass, field
from pathlib import Path
from statistics import fmean, stdev, variance
from typing import Any, Mapping, Protocol, Sequence

from perf_opt.verify.measure import autotune_iters, measure_harness, pick_input


logger = logging.getLogger(__name__)



class WallMs(float):
    """Task-clock in milliseconds, carrying the run's instruction count.

    `MeasurementBackend.measure_once` is typed as returning a float and every
    caller treats it as one. Widening that to a tuple would touch the
    protocol, both production backends, and every stub in the test suite for
    the sake of one extra counter. A float subclass is the same number to all
    of them, and the counter rides along for the one place that reads it.

    A backend that returns a plain float still satisfies the protocol; the
    instruction delta simply comes out `None` and nothing downstream is
    surprised — which is exactly what the test stubs do.
    """

    __slots__ = ("instructions",)

    def __new__(cls, value: float, instructions: int = 0) -> "WallMs":
        obj = super().__new__(cls, value)
        obj.instructions = int(instructions)
        return obj


@dataclass(frozen=True)
class PairSample:
    pair_index: int
    parent_ms: float
    candidate_ms: float
    parent_insns: int = 0
    candidate_insns: int = 0

    @property
    def delta_pct(self) -> float:
        if self.parent_ms <= 0:
            raise ValueError("parent measurement must be positive")
        return (self.candidate_ms - self.parent_ms) / self.parent_ms * 100.0

    @property
    def insns_delta_pct(self) -> float | None:
        """Instruction-count delta, or None when the backend did not report one.

        Retired instructions answer a question wall time cannot: whether the
        candidate does MORE WORK, or the same work in a differently laid out
        binary. Committing anything relinks the whole image, and the resulting
        i-cache and branch-predictor placement moves operations the edit never
        touched. Measured on binaries the gate itself had used: optipng's
        `png_palette_roundtrip` read +3.16% wall at -0.99% instructions, and
        libxml2's `buf_strings_chars` +6.04% wall at -0.06% instructions —
        both pure placement. The same probe on a genuine win, libxml2's
        `html_entity_encode`, read -34.8% wall at -26.2% instructions.

        Recorded, not judged. No gate reads this.
        """
        if self.parent_insns <= 0 or self.candidate_insns <= 0:
            return None
        return ((self.candidate_insns - self.parent_insns)
                / self.parent_insns * 100.0)


@dataclass(frozen=True)
class OpComparison:
    op: str
    samples: tuple[PairSample, ...]
    mean_delta_pct: float
    ci_low_pct: float
    ci_high_pct: float
    insns_delta_pct: float | None = None
    sample_count: int = 0
    priority: int = 0
    state: str = "active"
    stop_reason: str | None = None


@dataclass(frozen=True)
class AggregateSummary:
    mean_pct: float
    ci_low_pct: float
    ci_high_pct: float
    standard_error_pct: float
    effective_df: float
    method: str = "weighted_unequal_n_t"


@dataclass(frozen=True)
class PairedComparison:
    accepted: bool
    reason: str
    per_op: Mapping[str, OpComparison] = field(default_factory=dict)
    aggregate_samples_pct: tuple[float, ...] = ()
    aggregate_mean_pct: float | None = None
    aggregate_ci_low_pct: float | None = None
    aggregate_ci_high_pct: float | None = None
    checkpoint: int = 0
    unmeasurable_ops: tuple[str, ...] = ()
    aggregate_method: str | None = None
    high_risk_ops: tuple[str, ...] = ()
    max_checkpoint_reached: int = 0
    measurement_errors: tuple[Mapping[str, Any], ...] = ()


class MeasurementBackend(Protocol):
    def prepare(
        self,
        binary: Path,
        op: str,
        assets: Any,
        target_wall: float,
    ) -> tuple[Path, int] | None: ...

    def measure_once(
        self,
        binary: Path,
        op: str,
        input_path: Path,
        iters: int,
        pin_cpu: int | None,
    ) -> float: ...


class PerfStatBackend:
    """Use the repository's existing harness/perf measurement primitives."""

    def __init__(self, *, pin_cpu: int | None = None) -> None:
        self.pin_cpu = pin_cpu

    def prepare(
        self,
        binary: Path,
        op: str,
        assets: Any,
        target_wall: float,
    ) -> tuple[Path, int] | None:
        input_path = pick_input(Path(assets.harness_src), op)
        if input_path is None:
            return None
        iters = autotune_iters(
            Path(binary),
            op,
            input_path,
            target_wall=target_wall,
            pin_cpu=self.pin_cpu,
        )
        if iters <= 0:
            return None
        return input_path, iters

    def measure_once(
        self,
        binary: Path,
        op: str,
        input_path: Path,
        iters: int,
        pin_cpu: int | None,
    ) -> float:
        measurement = measure_harness(
            Path(binary),
            op,
            input_path,
            iters,
            repeats=1,
            warmup=0,
            pin_cpu=pin_cpu,
            label=f"paired[{Path(binary).name}:{op}]",
        )
        return WallMs(measurement.task_clock_ms, measurement.instructions)


# Two-sided Student-t 95% critical values at the checkpoints used by the
# optimizer.  Unknown sample sizes use a standard high-df approximation.
_T_CRITICAL_95 = {
    1: 12.706205,
    2: 4.302653,
    3: 3.182446,
    4: 2.776445,
    5: 2.570582,
    6: 2.446912,
    7: 2.364624,
    8: 2.306004,
    9: 2.262157,
    19: 2.093024,
    39: 2.022691,
}


def _t_critical_95(df: int) -> float:
    if df <= 0:
        return math.inf
    if df in _T_CRITICAL_95:
        return _T_CRITICAL_95[df]
    # Cornish-Fisher expansion for t_(0.975, df), accurate enough for
    # sequential gate checkpoints not listed above and conservative at small n.
    z = 1.959963984540054
    inverse_df = 1.0 / df
    return (
        z
        + (z**3 + z) * inverse_df / 4.0
        + (5 * z**5 + 16 * z**3 + 3 * z) * inverse_df**2 / 96.0
    )


def _mean_ci(values: Sequence[float]) -> tuple[float, float, float]:
    if not values:
        raise ValueError("cannot summarize an empty sample")
    mean = fmean(values)
    if len(values) == 1:
        return mean, -math.inf, math.inf
    sample_stdev = stdev(values)
    if sample_stdev == 0:
        return mean, mean, mean
    margin = _t_critical_95(len(values) - 1) * sample_stdev / math.sqrt(len(values))
    return mean, mean - margin, mean + margin


def _summarize_ops(
    samples: Mapping[str, list[PairSample]],
) -> dict[str, OpComparison]:
    comparisons: dict[str, OpComparison] = {}
    for op, op_samples in samples.items():
        deltas = [sample.delta_pct for sample in op_samples]
        mean, ci_low, ci_high = _mean_ci(deltas)
        insns = [d for d in (s.insns_delta_pct for s in op_samples)
                 if d is not None]
        comparisons[op] = OpComparison(
            op=op,
            samples=tuple(op_samples),
            mean_delta_pct=mean,
            ci_low_pct=ci_low,
            ci_high_pct=ci_high,
            insns_delta_pct=(sum(insns) / len(insns)) if insns else None,
            sample_count=len(op_samples),
        )
    return comparisons


def _normalized_weights(
    ops: tuple[str, ...], weights: Mapping[str, float] | None
) -> dict[str, float]:
    if weights is None:
        return {op: 1.0 / len(ops) for op in ops}
    selected = {op: float(weights.get(op, 0.0)) for op in ops}
    if any(weight < 0 for weight in selected.values()):
        raise ValueError("workload weights must be non-negative")
    total = sum(selected.values())
    if total <= 0:
        raise ValueError("workload weights must have a positive total")
    return {op: weight / total for op, weight in selected.items()}


def _weighted_aggregate_ci(
    per_op: Mapping[str, OpComparison],
    weights: Mapping[str, float],
) -> AggregateSummary:
    """Combine independent per-op paired deltas with unequal sample counts."""
    if not per_op:
        raise ValueError("cannot aggregate an empty workload set")
    ops = tuple(per_op)
    normalized = _normalized_weights(ops, weights)
    mean = sum(normalized[op] * per_op[op].mean_delta_pct for op in ops)

    variance_terms: list[tuple[float, int]] = []
    for op in ops:
        deltas = [sample.delta_pct for sample in per_op[op].samples]
        if not deltas:
            raise ValueError(f"workload {op!r} has no samples")
        sample_variance = variance(deltas) if len(deltas) > 1 else 0.0
        term = normalized[op] ** 2 * sample_variance / len(deltas)
        variance_terms.append((term, len(deltas)))

    standard_error_sq = sum(term for term, _ in variance_terms)
    if standard_error_sq == 0.0:
        return AggregateSummary(mean, mean, mean, 0.0, math.inf)

    denominator = sum(
        term**2 / (sample_count - 1)
        for term, sample_count in variance_terms
        if term > 0.0 and sample_count > 1
    )
    effective_df = (
        standard_error_sq**2 / denominator if denominator > 0.0 else math.inf
    )
    standard_error = math.sqrt(standard_error_sq)
    critical = (
        1.959963984540054
        if math.isinf(effective_df)
        else _t_critical_95(max(1, int(math.floor(effective_df))))
    )
    margin = critical * standard_error
    return AggregateSummary(
        mean_pct=mean,
        ci_low_pct=mean - margin,
        ci_high_pct=mean + margin,
        standard_error_pct=standard_error,
        effective_df=effective_df,
    )


def _aggregate_state(
    summary: AggregateSummary, regress_tolerance_pct: float
) -> str:
    ""                                   

                                             
                                                         
                                                             
                                                       
                                    
                                                              
                                                     
       
    tol = regress_tolerance_pct
    if summary.ci_high_pct < tol:
        return "pass"
    if summary.ci_low_pct >= tol:
        return "regressed"
    return "uncertain"


# ── placement-only regression: wall up while the machine did strictly less ──
#
# The per-op ceiling exists to stop a rewrite from paying for its aggregate
# win by wrecking one operation. It reads wall time only, and wall time on
# one op moves for two quite different reasons: the rewrite made that op do
# more work, or the rewrite relinked the binary and the op's hot loop landed
# somewhere less friendly. Only the first is the rewrite's fault; the second
# is a lottery that any accepted commit re-runs for every other op.
#
# The instruction count separates them. Measured on lz4 (C11 goto-dispatch
# split, 16 ops in the step gate, 15 of them faster, aggregate -1.936% with
# CI [-1.978, -1.895]):
#
#     streaming_decode      wall +2.454%   insns -1.2303%   <- the sole veto
#     decompress_usingdict  wall -6.653%   insns -4.3227%
#     extdict_decompress    wall -4.793%   insns -4.5188%
#     compress_decompress   wall -3.271%   insns -5.6342%
#
# The vetoing op executed 1.2% FEWER instructions and still took longer.
# Independently reproduced with perf (r=5): insns 3988.2M -> 3940.1M,
# cycles 2273.8M -> 2313.1M, and the reason is the uop cache — DSB uops
# 2164M -> 1910M with MITE picking up the slack (DSB share 45.6% -> 40.6%),
# while L1i tag misses got BETTER and branch misses did not move. Code
# placement, not work. One such op vetoed a change that made 15 others faster.
#
# So an op that did less work loses its individual veto — it is still
# measured, still logged, and still counted in the aggregate that has to
# come out ahead. It does NOT get a pass on collapse: the catastrophic
# ceiling below stays armed for every op.
_PLACEMENT_INSNS_DROP_PCT = -0.25   # "did measurably less work" floor
_PLACEMENT_MIN_AGG_GAIN_PCT = -0.10  # aggregate must be a confident gain
# The drop floor above only recognises placement when the op did measurably
# LESS work — which leaves out the plainest case of all: the op did exactly
# the same work (instructions flat) and only its wall moved. That is what
# placement looks like when the rewrite did not touch the op at all, and the
# floor rejected it for being "not less".
#
# Measured cost, three candidates rejected by one such op each, with the
# op's instruction count flat to ±0.03%:
#   fzy      legacy-faad1b82  two ops -15.2% / -20.7%, vetoed by +2.61%
#   lodepng  region-546e1d59  one op  -31.2%,          vetoed by +2.01%
#   optipng  ii-inl-6f06775e  one op  -25.8%,          vetoed by +2.33%
#
# The wall cap is what keeps this narrow. Replayed over every rejected
# candidate in twelve projects, "instructions flat" ALONE would also have
# exempted ops at +8% to +32% wall (libxml2 encoding_suite +31.6%, lil
# expr_embedded_builtins +22.2%, lodepng file_codec +10.6%) — regressions
# that are real (cache, frontend), not placement. With the cap at 3% the
# replay exempts 22 candidates, the worst exempted op sits at +2.81%, and
# 181 rejections stand.
_PLACEMENT_INSNS_FLAT_PCT = 0.10    # "did the same work" band, either side
_PLACEMENT_WALL_NOISE_PCT = 3.0     # and moved no more than layout can explain


def did_measurably_less_work(insns_pct: float | None) -> bool:
    """True when the op retired measurably FEWER instructions.

    The strict half of the placement question, kept as its own predicate
    because the two gates must answer differently and once disagreed by
    accident (lz4: exempted at checkpoints 10 and 20, rejected at 40).

    The step gate judges one candidate against its parent, where a flat count
    is the plainest evidence the rewrite never touched that op — so it also
    exempts flat, via `is_placement_noise`. The cumulative gate judges the
    whole crate against pristine across every commit so far, and the creep it
    exists to catch IS a run of flat-to-slightly-up steps, each one small
    enough to clear the step gate. Exempting flat there would disarm it.
    """
    return insns_pct is not None and insns_pct <= _PLACEMENT_INSNS_DROP_PCT


def is_placement_noise(wall_pct: float | None, insns_pct: float | None) -> bool:
    """True when an op's wall regression is attributable to code placement.

    Either the op did measurably less work (the original floor), or it did
    the same work and moved by no more than layout accounts for. Callers must
    still require an aggregate gain and a non-catastrophic op — this answers
    one question only, and answers it identically at every STEP veto site.
    The cumulative gate deliberately asks the stricter question instead; see
    `did_measurably_less_work`.
    """
    if insns_pct is None:
        return False
    if did_measurably_less_work(insns_pct):
        return True
    return (abs(insns_pct) <= _PLACEMENT_INSNS_FLAT_PCT
            and wall_pct is not None
            and wall_pct <= _PLACEMENT_WALL_NOISE_PCT)


def _per_op_vetoes(
    per_op: Mapping[str, OpComparison],
    aggregate: AggregateSummary,
    *,
    upper_limit_pct: float,
    per_op_limits: Mapping[str, float] | None,
    catastrophic_regress_pct: float,
    placement_insns_drop_pct: float = _PLACEMENT_INSNS_DROP_PCT,
    placement_min_agg_gain_pct: float = _PLACEMENT_MIN_AGG_GAIN_PCT,
) -> tuple[list[str], dict[str, str]]:
    """Ops over their per-op ceiling: (the ones that veto, the ones exempted).

    The single definition every per-op veto in this module goes through —
    `_decision`, the max-samples fallback in the sampling loop, and the
    scheduler that picks which ops to extend. The exemption first lived
    inline in `_decision` alone, and the other places kept applying the raw
    ceiling. Measured on lz4, same candidate, same numbers at every
    checkpoint (streaming_decode wall +2.5%, insns -1.22%, aggregate -1.94%):
    exempted at 10 and 20, extended 10->20->40 as "uncertain", then rejected
    by the max-samples branch at 40 as `per_op_regress`. One predicate, called
    from every site, is the only way the sites cannot disagree again.

    The exempted map carries a printable reason per op, for the log.
    """
    agg_is_gain = aggregate.ci_high_pct < placement_min_agg_gain_pct
    vetoing: list[str] = []
    exempted: dict[str, str] = {}
    for op, item in per_op.items():
        if item.ci_low_pct <= _limit_for(op, upper_limit_pct, per_op_limits):
            continue
        # getattr: several gate tests pass a stub comparison object that
        # predates this field. No counter -> no exemption, the conservative
        # direction.
        insns = getattr(item, "insns_delta_pct", None)
        placement = is_placement_noise(
            getattr(item, "mean_delta_pct", None), insns)
        collapse = item.ci_low_pct > _limit_for(
            op, catastrophic_regress_pct, per_op_limits)
        if placement and agg_is_gain and not collapse:
            exempted[op] = (
                f"{op}=wall{item.mean_delta_pct:+.2f}%/insns{insns:+.2f}%")
        else:
            vetoing.append(op)
    return vetoing, exempted


def _log_exemptions(
    exempted: Mapping[str, str], aggregate: AggregateSummary, min_gain: float
) -> None:
    if exempted:
        logger.info(
            "[w2] per-op 门豁免(指令数下降,判为代码摆放而非多做功): %s "
            "— 聚合 %.3f%% ci_high %.3f%% < %.2f%%",
            ", ".join(exempted.values()), aggregate.mean_pct,
            aggregate.ci_high_pct, min_gain)


def _decision(
    per_op: Mapping[str, OpComparison],
    aggregate: AggregateSummary,
    *,
    regress_tolerance_pct: float,
    upper_limit_pct: float,
    per_op_limits: Mapping[str, float] | None = None,
    catastrophic_regress_pct: float = 5.0,
    aggregate_only: bool = False,
    placement_insns_drop_pct: float = _PLACEMENT_INSNS_DROP_PCT,
    placement_min_agg_gain_pct: float = _PLACEMENT_MIN_AGG_GAIN_PCT,
) -> str:
    exempted: dict[str, str] = {}
    if not aggregate_only:
        # The exemption is only safe while the aggregate is *confidently* a
        # gain — ci_high, not the point estimate. If the overall number is a
        # wash there is nothing to weigh the single regression against, and
        # the ceiling does its ordinary job.
        vetoing, exempted = _per_op_vetoes(
            per_op, aggregate,
            upper_limit_pct=upper_limit_pct,
            per_op_limits=per_op_limits,
            catastrophic_regress_pct=catastrophic_regress_pct,
            placement_insns_drop_pct=placement_insns_drop_pct,
            placement_min_agg_gain_pct=placement_min_agg_gain_pct,
        )
        _log_exemptions(exempted, aggregate, placement_min_agg_gain_pct)
        if vetoing:
            return "per_op_regress"
    state = _aggregate_state(aggregate, regress_tolerance_pct)
    if state == "regressed":
        return "no_gain"                                      
    if aggregate_only:
                                              
                          
        #
        # The two gates used to leave a hole between them. The step gate
        # measures only the ops the rewritten function appears in; the total
        # gate measures every op but looked at the aggregate alone. A rewrite
        # that slows down an op it is NOT part of was therefore seen by
        # neither. Measured: one rewrite of a function whose profile lists
        # {lz77 4.7%, raw_deflate 7.2%, zlib_compress 7.3%} — no cache op at
        # all — made the cache operation 12.1% slower. The step gate never
        # measured it; in the five-op equal-weight aggregate it diluted to
        # 2.4% and was cancelled by -13% on the two main ops. It committed,
        # and the run's final measurement came back +9.62% on that op and
        # `accepted: false` for the whole run.
        #
        # So the total gate now also refuses a catastrophic single-op
        # regression. The threshold is deliberately the catastrophic one, not
        # the per-op ceiling: this gate exists to catch collapse, not to
        # re-litigate the tolerances the step gate already applied.
        #
        # It must honour `per_op_limits` like the step gate above does, or the
        # exemption the caller granted is not an exemption at all. An op the
        # caller has disarmed (library share below the floor — what it measures
        # is mostly harness, so its wall time cannot speak for the crate) gets
        # `inf` there, and reading the flat threshold here would re-arm it.
        # Measured: a rewrite of `lil_append_char` that made the crate 7.48%
        # FASTER was logged as `w2_regress`, vetoed by an op whose library
        # share is 12.5% — 87% of what it times is the harness. The previous
        # run had committed the same function at -15.0%.
        collapsed = [
            op for op, item in per_op.items()
            if item.ci_low_pct > _limit_for(
                op, catastrophic_regress_pct, per_op_limits)
        ]
        if collapsed:
            logger.info(
                "[w2] total gate 拒:单 op 崩溃 %s (aggregate=%.3f%%)",
                ", ".join(
                    f"{op}=ci_low{per_op[op].ci_low_pct:+.2f}%"
                    f" mean{getattr(per_op[op], 'mean_delta_pct', float('nan')):+.2f}%"
                    f" > limit"
                    f"{_limit_for(op, catastrophic_regress_pct, per_op_limits):.2f}%"
                    for op in collapsed),
                getattr(aggregate, "mean_pct", float("nan")),
            )
            return "per_op_regress"
        return "accepted" if state == "pass" else "continue"
    # An exempted op is settled, not pending: its wall regression is certain
    # and so is the instruction drop that explains it. Leaving it out of
    # `all_safe` meant the step gate could never accept while it was present
    # — every checkpoint returned "continue" and the loop ran to its maximum.
    all_safe = all(
        item.ci_high_pct <= upper_limit_pct or op in exempted
        for op, item in per_op.items()
    )
    if all_safe and state == "pass":
        return "accepted"
    return "continue"


def _projected_aggregate_ci(
    per_op: Mapping[str, OpComparison],
    weights: Mapping[str, float],
    projected_counts: Mapping[str, int],
) -> AggregateSummary:
    normalized = _normalized_weights(tuple(per_op), weights)
    mean = sum(normalized[op] * per_op[op].mean_delta_pct for op in per_op)
    terms: list[tuple[float, int]] = []
    for op, comparison in per_op.items():
        deltas = [sample.delta_pct for sample in comparison.samples]
        count = projected_counts.get(op, len(deltas))
        sample_variance = variance(deltas) if len(deltas) > 1 else 0.0
        terms.append((normalized[op] ** 2 * sample_variance / count, count))
    standard_error_sq = sum(term for term, _ in terms)
    if standard_error_sq == 0.0:
        return AggregateSummary(mean, mean, mean, 0.0, math.inf)
    denominator = sum(
        term**2 / (count - 1)
        for term, count in terms
        if term > 0.0 and count > 1
    )
    effective_df = (
        standard_error_sq**2 / denominator if denominator > 0.0 else math.inf
    )
    standard_error = math.sqrt(standard_error_sq)
    critical = (
        1.959963984540054
        if math.isinf(effective_df)
        else _t_critical_95(max(1, int(math.floor(effective_df))))
    )
    margin = critical * standard_error
    return AggregateSummary(
        mean, mean - margin, mean + margin, standard_error, effective_df
    )


def _ops_for_next_checkpoint(
    per_op: Mapping[str, OpComparison],
    weights: Mapping[str, float],
    *,
    target_count: int,
    regress_tolerance_pct: float,
    upper_limit_pct: float,
    settled: "frozenset[str]" = frozenset(),
) -> dict[str, str]:
    # `settled` = ops exempted by `_per_op_vetoes`. More pairs cannot change
    # their verdict; on lz4 they only doubled the op's sample count twice.
    selected = {
        op: "per_op_uncertain"
        for op, item in per_op.items()
        if item.ci_high_pct > upper_limit_pct and op not in settled
    }
    aggregate = _weighted_aggregate_ci(per_op, weights)
    if _aggregate_state(aggregate, regress_tolerance_pct) != "uncertain":
        return selected

    normalized = _normalized_weights(tuple(per_op), weights)
    candidates: list[tuple[float, str]] = []
    for op, item in per_op.items():
        if op in selected:
            continue
        deltas = [sample.delta_pct for sample in item.samples]
        sample_variance = variance(deltas) if len(deltas) > 1 else 0.0
        current_count = len(deltas)
        reduction = normalized[op] ** 2 * sample_variance * (
            1.0 / current_count - 1.0 / target_count
        )
        if reduction > 0.0:
            candidates.append((reduction, op))
    candidates.sort(reverse=True)

    projected_counts = {
        op: (target_count if op in selected else len(item.samples))
        for op, item in per_op.items()
    }
    projected = _projected_aggregate_ci(per_op, weights, projected_counts)
    if _aggregate_state(projected, regress_tolerance_pct) != "uncertain":
        return selected

    for _, op in candidates:
        selected[op] = "aggregate_precision"
        projected_counts[op] = target_count
        projected = _projected_aggregate_ci(per_op, weights, projected_counts)
        if _aggregate_state(projected, regress_tolerance_pct) != "uncertain":
            break
    return selected


def _annotate_comparisons(
    per_op: Mapping[str, OpComparison],
    priorities: Mapping[str, int],
    extension_reasons: Mapping[str, str],
    upper_limit_pct: float,
) -> dict[str, OpComparison]:
    annotated: dict[str, OpComparison] = {}
    for op, item in per_op.items():
        if item.ci_low_pct > upper_limit_pct:
            state = "regressed"
            stop_reason = "per_op_regress"
        elif item.ci_high_pct <= upper_limit_pct:
            state = "safe"
            stop_reason = extension_reasons.get(op, "safe")
        else:
            state = "active"
            stop_reason = extension_reasons.get(op, "max_samples")
        annotated[op] = OpComparison(
            op=item.op,
            samples=item.samples,
            mean_delta_pct=item.mean_delta_pct,
            ci_low_pct=item.ci_low_pct,
            ci_high_pct=item.ci_high_pct,
            # Carried explicitly. This rebuilds the comparison field by field
            # rather than copying it, so anything not named here is silently
            # replaced by its default — which is how the instruction delta
            # first arrived at the serializer as `None` while the samples it
            # is computed from were sitting right there in `item.samples`.
            insns_delta_pct=item.insns_delta_pct,
            sample_count=item.sample_count,
            priority=priorities[op],
            state=state,
            stop_reason=stop_reason,
        )
    return annotated



def _limit_for(
    op: str, default_pct: float, overrides: Mapping[str, float] | None
) -> float:
    """The regression ceiling for one op.

    Not every op in a candidate's gate is measuring the candidate. A function
    that holds 40% of one operation may hold 0.5% of another, yet a single
    committed edit relinks the whole binary and moves BOTH. Measured on a
    compression crate: across seven commits, none of which touched `cache.rs`,
    the cache operation moved by up to 4.12% — reproducible to 0.01s across
    independent runs, so not noise that more samples would average away. A
    flat 1.0% ceiling therefore rejects on code layout at least as often as on
    the edit, and the rejection reads identically in the log.

    So the caller may raise the ceiling for the ops a candidate is only a
    bystander in, while the op it actually lives in keeps the tight one.
    """
    if overrides is None:
        return default_pct
    return max(default_pct, overrides.get(op, default_pct))


def compare_paired(
    backend: MeasurementBackend,
    parent_binary: Path,
    candidate_binary: Path,
    *,
    ops: Sequence[str],
    assets: Any,
    checkpoints: Sequence[int] = (10, 20, 40),
    weights: Mapping[str, float] | None = None,
    per_op_upper_limit_pct: float = 1.0,
    per_op_limits: Mapping[str, float] | None = None,
    regress_tolerance_pct: float = 0.3,
    catastrophic_regress_pct: float = 5.0,
    target_wall: float = 2.0,
    pin_cpu: int | None = None,
    high_risk_ops: Sequence[str] = (),
    early_reject_pairs: int = 5,
    minimum_pairs: int = 10,
    aggregate_only: bool = False,
) -> PairedComparison:
    ""                                                                        

                                                           
                                                          
                                      
       
    ordered_ops = tuple(dict.fromkeys(ops))
    if not ordered_ops:
        return PairedComparison(False, "unmeasurable", unmeasurable_ops=())
    # Ops removed from the scope because they cannot be measured at all. Kept
    # so the verdict can state what it does NOT cover — a comparison over 10 of
    # 11 ops is useful, a comparison that silently became 10 of 11 is not.
    unmeasurable_dropped: tuple[str, ...] = ()
    ordered_checkpoints = tuple(sorted(set(checkpoints)))
    if not ordered_checkpoints or ordered_checkpoints[0] < 2:
        raise ValueError("checkpoints must contain sample counts >= 2")
    if minimum_pairs < 10:
        raise ValueError("W2 acceptance requires at least 10 paired samples")
    if early_reject_pairs < 2:
        raise ValueError("early-reject pair count must be >= 2")
    if ordered_checkpoints[-1] < minimum_pairs:
        raise ValueError(
            f"maximum checkpoint must provide at least {minimum_pairs} pairs"
        )

    stage_checkpoints = tuple(
        dict.fromkeys(
            (minimum_pairs,)
            + tuple(cp for cp in ordered_checkpoints if cp > minimum_pairs)
        )
    )
    parent_binary = Path(parent_binary)
    candidate_binary = Path(candidate_binary)
    prepared: dict[str, tuple[Path, int]] = {}
    warmed: set[str] = set()
    samples: dict[str, list[PairSample]] = {op: [] for op in ordered_ops}
    ordered_high_risk = tuple(
        op for op in dict.fromkeys(high_risk_ops) if op in samples
    )
    priority_order = ordered_high_risk + tuple(
        op for op in ordered_ops if op not in ordered_high_risk
    )
    priorities = {op: index for index, op in enumerate(priority_order)}
    extension_reasons: dict[str, str] = {}
    measurement_errors: list[dict[str, Any]] = []
    logged_stops: set[str] = set()

    def error_result() -> PairedComparison:
        return PairedComparison(
            False,
            "measurement_error",
            high_risk_ops=ordered_high_risk,
            measurement_errors=tuple(measurement_errors),
        )

    def prepare_op(op: str) -> str:
        if op not in prepared:
            try:
                prep = backend.prepare(parent_binary, op, assets, target_wall)
            except Exception as exc:  # backend boundary: fail closed with context
                measurement_errors.append({
                    "op": op,
                    "side": "parent",
                    "phase": "prepare",
                    "pair_index": -1,
                    "error": f"{type(exc).__name__}: {exc}",
                })
                return "error"
            if prep is None:
                return "missing"
            prepared[op] = prep
        return "ok"

    def measure_side(
        binary: Path, side: str, op: str, pair_index: int, phase: str,
    ) -> float | None:
        input_path, iters = prepared[op]
        try:
            value = backend.measure_once(binary, op, input_path, iters, pin_cpu)
            checked = float(value)
            if not math.isfinite(checked) or checked <= 0.0:
                raise ValueError(
                    f"measurement must be positive finite, got {checked}")
            # Returned unconverted: `float(value)` would drop a `WallMs`'s
            # instruction count, which is the whole reason it is a subclass.
            return value
        except Exception as exc:  # backend boundary: fail closed with context
            measurement_errors.append({
                "op": op,
                "side": side,
                "phase": phase,
                "pair_index": pair_index,
                "error": f"{type(exc).__name__}: {exc}",
            })
            return None

    def prepare_and_warm(op: str) -> str:
        status = prepare_op(op)
        if status != "ok":
            return status
        if op not in warmed:
            if measure_side(parent_binary, "parent", op, -1, "warmup") is None:
                return "error"
            if measure_side(
                candidate_binary, "candidate", op, -1, "warmup"
            ) is None:
                return "error"
            warmed.add(op)
        return "ok"

    def append_pair(op: str, pair_index: int) -> bool:
        if pair_index % 2 == 0:
            parent_ms = measure_side(
                parent_binary, "parent", op, pair_index, "sample"
            )
            candidate_ms = measure_side(
                candidate_binary, "candidate", op, pair_index, "sample"
            ) if parent_ms is not None else None
        else:
            candidate_ms = measure_side(
                candidate_binary, "candidate", op, pair_index, "sample"
            )
            parent_ms = measure_side(
                parent_binary, "parent", op, pair_index, "sample"
            ) if candidate_ms is not None else None
        if parent_ms is None or candidate_ms is None:
            return False
        samples[op].append(PairSample(
            pair_index, float(parent_ms), float(candidate_ms),
            parent_insns=getattr(parent_ms, "instructions", 0),
            candidate_insns=getattr(candidate_ms, "instructions", 0),
        ))
        return True

    def extend_to(op: str, target_count: int) -> bool:
        while len(samples[op]) < target_count:
            if not append_pair(op, len(samples[op])):
                return False
        return True

    normalized_weights = _normalized_weights(ordered_ops, weights)

    def make_result(
        *, accepted: bool, reason: str, per_op: Mapping[str, OpComparison],
    ) -> PairedComparison:
        aggregate = _weighted_aggregate_ci(per_op, normalized_weights)
        annotated = _annotate_comparisons(
            per_op, priorities, extension_reasons, per_op_upper_limit_pct
        )
        for op, item in annotated.items():
            if op not in logged_stops:
                logger.info(
                    "[w2] op=%s stopped_at=%d reason=%s",
                    op, item.sample_count, item.stop_reason,
                )
                logged_stops.add(op)
        max_count = max(item.sample_count for item in per_op.values())
        return PairedComparison(
            accepted=accepted,
            reason=reason,
            per_op=annotated,
            # Carried on EVERY verdict, not just the failing one: a reader has
            # to be able to see that the scope was narrowed.
            unmeasurable_ops=unmeasurable_dropped,
            aggregate_samples_pct=(),
            aggregate_mean_pct=aggregate.mean_pct,
            aggregate_ci_low_pct=aggregate.ci_low_pct,
            aggregate_ci_high_pct=aggregate.ci_high_pct,
            checkpoint=max_count,
            aggregate_method=aggregate.method,
            high_risk_ops=ordered_high_risk,
            max_checkpoint_reached=max_count,
        )

    # A five-pair screen is deliberately rejection-only and runs lazily in
    # risk order so a clear regression avoids preparing the rest of the scope.
    for op in tuple(ordered_high_risk):
        status = prepare_and_warm(op)
        if status == "error":
            return error_result()
        if status == "missing":
            # Same reasoning as the full-scope pass below: an unmeasurable op
            # is dropped from the scope, never allowed to abort the others.
            logger.warning(
                "[w2] high-risk op `%s` has no usable perf input — dropping it "
                "from the comparison scope; it is NOT covered by this verdict.",
                op)
            unmeasurable_dropped += (op,)
            ordered_high_risk = tuple(o for o in ordered_high_risk if o != op)
            ordered_ops = tuple(o for o in ordered_ops if o != op)
            samples.pop(op, None)
            if not ordered_ops:
                return PairedComparison(
                    False, "unmeasurable",
                    unmeasurable_ops=unmeasurable_dropped,
                )
            continue
        if not extend_to(op, early_reject_pairs):
            return error_result()
        if aggregate_only:
            continue                                                    
        comparison = _summarize_ops({op: samples[op]})[op]
        if comparison.ci_low_pct > _limit_for(
                op, per_op_upper_limit_pct, per_op_limits):
            # The screen sees one op and no aggregate, so it cannot grant the
            # placement exemption — but it must not pre-empt it either. An op
            # that retired fewer instructions is deferred to the full-scope
            # measurement, where `_per_op_vetoes` judges it against the
            # aggregate. Deferral only; nothing is accepted here.
            insns = comparison.insns_delta_pct
            if (is_placement_noise(comparison.mean_delta_pct, insns)
                    and comparison.ci_low_pct <= _limit_for(
                        op, catastrophic_regress_pct, per_op_limits)):
                logger.info(
                    "[w2] early-reject 暂缓:%s wall ci_low %+.2f%% 越限,但 insns "
                    "%+.2f%% —— 交给全量测量按聚合判",
                    op, comparison.ci_low_pct, insns)
                continue
            extension_reasons[op] = "early_regress"
            return make_result(
                accepted=False, reason="per_op_regress", per_op={op: comparison}
            )

    missing: list[str] = []
    for op in ordered_ops:
        if op in prepared:
            continue
        status = prepare_op(op)
        if status == "error":
            return error_result()
        if status == "missing":
            missing.append(op)
    if missing:
        # An op with no usable input cannot be measured — but neither can it be
        # measured by giving up on every OTHER op, which is what aborting here
        # used to do. Measured: one op whose gen_perf input segfaults turned a
        # 21-commit run's final measurement into `per_op={}`, `checkpoint=0`,
        # no aggregate at all — the work was intact in git and simply had no
        # number attached to it. Dropping the op loses only what was already
        # unobservable, and says so.
        survivors = tuple(op for op in ordered_ops if op not in missing)
        if not survivors:
            return PairedComparison(
                False,
                "unmeasurable",
                unmeasurable_ops=tuple(missing),
                high_risk_ops=ordered_high_risk,
            )
        logger.warning(
            "[w2] dropping unmeasurable op(s) from the comparison scope: %s "
            "— no usable perf input; measuring the remaining %d op(s). The "
            "dropped op(s) are NOT covered by this verdict.",
            ", ".join(missing), len(survivors))
        ordered_ops = survivors
        ordered_high_risk = tuple(
            op for op in ordered_high_risk if op not in missing)
        unmeasurable_dropped = tuple(missing)
        # `samples` was keyed off the original scope; an entry left behind with
        # no pairs makes the summary raise on an empty sample.
        for op in missing:
            samples.pop(op, None)

    # One discarded warm-up for each side before the first full-scope pair.
    for op in ordered_ops:
        if prepare_and_warm(op) == "error":
            return error_result()

    for op in ordered_ops:
        if not extend_to(op, minimum_pairs):
            return error_result()

    for checkpoint_index, sample_count in enumerate(stage_checkpoints):
        per_op = _summarize_ops(samples)
        aggregate = _weighted_aggregate_ci(per_op, normalized_weights)
        verdict = _decision(
            per_op,
            aggregate,
            regress_tolerance_pct=regress_tolerance_pct,
            upper_limit_pct=per_op_upper_limit_pct,
            per_op_limits=per_op_limits,
            catastrophic_regress_pct=catastrophic_regress_pct,
            aggregate_only=aggregate_only,
        )
        exempted_now: dict[str, str] = {} if aggregate_only else _per_op_vetoes(
            per_op, aggregate,
            upper_limit_pct=per_op_upper_limit_pct,
            per_op_limits=per_op_limits,
            catastrophic_regress_pct=catastrophic_regress_pct,
        )[1]
        safe_count = sum(
            item.ci_high_pct <= _limit_for(
                op, per_op_upper_limit_pct, per_op_limits)
            for op, item in per_op.items()
        )
        logger.info(
            "[w2] checkpoint=%d safe=%d uncertain=%d verdict=%s exempt=%d",
            sample_count, safe_count,
            len(per_op) - safe_count - len(exempted_now), verdict,
            len(exempted_now),
        )
        if verdict != "continue":
            return make_result(
                accepted=verdict == "accepted", reason=verdict, per_op=per_op
            )
        if checkpoint_index == len(stage_checkpoints) - 1:
                                                          
                                                               
                                                    
                                                 
                                                                            
                                                                
                      
            # Through the same predicate as `_decision`, not a copy of its
            # ceiling test. The copy here compared ci_low to the flat ceiling:
            # it ignored both the placement exemption and `per_op_limits`, so
            # an op `_decision` had already cleared at every earlier
            # checkpoint was rejected here on unchanged numbers.
            if not aggregate_only and _per_op_vetoes(
                per_op, aggregate,
                upper_limit_pct=per_op_upper_limit_pct,
                per_op_limits=per_op_limits,
                catastrophic_regress_pct=catastrophic_regress_pct,
            )[0]:
                return make_result(
                    accepted=False, reason="per_op_regress", per_op=per_op
                )
                                                
                                                                 
                                                       
            if _aggregate_state(aggregate, regress_tolerance_pct) == "pass":
                return make_result(
                    accepted=True, reason="accepted", per_op=per_op
                )
            return make_result(accepted=False, reason="no_gain", per_op=per_op)

        target_count = stage_checkpoints[checkpoint_index + 1]
        selected = _ops_for_next_checkpoint(
            per_op,
            normalized_weights,
            target_count=target_count,
            regress_tolerance_pct=regress_tolerance_pct,
            upper_limit_pct=per_op_upper_limit_pct,
            settled=frozenset(exempted_now),
        )
        if not selected:
            return make_result(accepted=False, reason="no_gain", per_op=per_op)
        for op, item in per_op.items():
            if (
                op not in selected
                and op not in logged_stops
                and item.ci_high_pct <= per_op_upper_limit_pct
            ):
                logger.info(
                    "[w2] op=%s stopped_at=%d reason=safe",
                    op, item.sample_count,
                )
                logged_stops.add(op)
        for op in exempted_now:
            if op not in selected and op not in logged_stops:
                logger.info(
                    "[w2] op=%s stopped_at=%d reason=placement_exempt",
                    op, per_op[op].sample_count,
                )
                logged_stops.add(op)
        for op, reason in selected.items():
            extension_reasons[op] = reason
            logger.info(
                "[w2] op=%s extend=%d->%d reason=%s",
                op, len(samples[op]), target_count, reason,
            )
            if not extend_to(op, target_count):
                return error_result()

    raise AssertionError("adaptive W2 loop did not return a verdict")
