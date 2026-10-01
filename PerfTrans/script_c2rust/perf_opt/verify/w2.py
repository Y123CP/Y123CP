"""W2 performance gate.

The second verification gate (the first is W1: compile + byte-equivalence, in
`verify.cargo` / `verify.functional`). W2 decides whether a rewrite is a
*performance* win: it measures the harness on the perf workload before and
after, and applies a cv-aware tolerance with anti-drift and (for direct-type
rules) a real-gain requirement.

Wall-clock (task-clock) is the SOLE verdict; the counter deltas (insns / IPC /
cache / branch) are recorded for reporting only — never gated on. A rewrite that
cuts instructions but does not move wall-clock is not a W2 win (calibrated
lesson: counter-only gates false-pass).

Decision (ported from the retired stage_b gate, host-calibrated):

    measured_cv   = max(pre.cv, post.cv)
    effective_tol = min(max(3 × measured_cv, TOL_FLOOR), TOL_CAP)
    delta_pct     = (post.wall - pre.wall) / pre.wall × 100        # <0 = faster

    FAIL if delta_pct > effective_tol                              # regressed
    if require_perf_gain:                                          # direct cards
        FAIL if delta_pct >= -3 × measured_cv                      # must beat 3σ
    if absolute_baseline given:                                    # anti-drift
        FAIL if (post vs absolute) delta > ABS_TOL

`require_perf_gain` maps to the optimization card `type`: `direct` cards pass
True (must show a real speedup); `enabling` cards pass False (must merely not
regress — whether they unlocked anything is judged by the card's unlock_check).

Caching: pre and the absolute baseline do not change between rejected attempts,
so the caller may pass previously-taken `Measurement`s and only `post` is
re-measured each attempt. `w2_gate` returns the Measurements it used so the loop
can cache them (a committed post becomes the next round's pre).
"""

from __future__ import annotations

import logging
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from .measure import Measurement, measure_harness

logger = logging.getLogger("verify.w2")

# Tolerance band (host-calibrated with isolcpus + pin CPU 16 + -r 30, cv
# ~0.06-0.08%, 3σ ~0.20-0.24%):
TOL_FLOOR = 0.3   # floor above 3σ so a true near-zero rewrite counts as "OK".
TOL_CAP = 1.0     # refuse any wall regression > 1.0% regardless of cv.
ABS_TOL = 0.3     # anti-drift: post must not regress > this vs the pristine
                  # absolute baseline (defeats paired-baseline drift inflation).


def _pct_delta(post: float, pre: float) -> Optional[float]:
    return None if pre <= 0 else 100.0 * (post - pre) / pre


@dataclass
class W2Verdict:
    ok: bool
    reason: str
    delta_pct: Optional[float]              # post vs pre wall (<0 = faster)
    effective_tol_pct: float
    measured_cv: float
    drift_pct: Optional[float] = None       # post vs absolute baseline
    # multi-dim Δ (reporting only; NEVER gated on)
    insns_delta_pct: Optional[float] = None
    cycles_delta_pct: Optional[float] = None
    ipc_delta_pct: Optional[float] = None
    l1d_miss_delta_pp: Optional[float] = None
    bmisp_delta_pp: Optional[float] = None
    # the Measurements used — returned so the caller can cache pre/absolute
    pre: Optional[Measurement] = None
    post: Optional[Measurement] = None
    absolute: Optional[Measurement] = None
    detail: dict = field(default_factory=dict)


def w2_gate(pre_bin: Path, post_bin: Path, *,
            op: str, input_path: Path, iters: int,
            require_perf_gain: bool = False,
            absolute_baseline_bin: Optional[Path] = None,
            pre_measurement: Optional[Measurement] = None,
            absolute_measurement: Optional[Measurement] = None,
            repeats: Optional[int] = None,
            pin_cpu: Optional[int] = None) -> W2Verdict:
    """Measure `pre_bin` vs `post_bin` on `op`/`input_path` and decide.

    `pre_measurement` / `absolute_measurement`: pass a cached Measurement to skip
    re-measuring an unchanged binary (see module docstring). `post` is always
    measured fresh. `iters` MUST be the same value used for every binary.
    """
    kw = {}
    if repeats is not None:
        kw["repeats"] = repeats
    if pin_cpu is not None:
        kw["pin_cpu"] = pin_cpu

    post = measure_harness(post_bin, op, input_path, iters, label="post", **kw)
    pre = pre_measurement or measure_harness(
        pre_bin, op, input_path, iters, label="pre", **kw)
    absolute = absolute_measurement
    if absolute is None and absolute_baseline_bin is not None:
        absolute = measure_harness(
            absolute_baseline_bin, op, input_path, iters, label="absolute", **kw)

    delta_pct = _pct_delta(post.task_clock_ms, pre.task_clock_ms)
    measured_cv = max(pre.cv_pct or 0.0, post.cv_pct or 0.0)
    effective_tol = min(max(3.0 * measured_cv, TOL_FLOOR), TOL_CAP)

    # multi-dim deltas (reporting only)
    insns_d = _pct_delta(post.instructions, pre.instructions)
    cycles_d = _pct_delta(post.cycles, pre.cycles)
    pre_ipc = pre.instructions / pre.cycles if pre.cycles > 0 else None
    post_ipc = post.instructions / post.cycles if post.cycles > 0 else None
    ipc_d = (_pct_delta(post_ipc, pre_ipc)
             if pre_ipc and post_ipc else None)

    def _rate_pp(pre_num, pre_den, post_num, post_den):
        if pre_den <= 0 or post_den <= 0:
            return None
        return 100.0 * post_num / post_den - 100.0 * pre_num / pre_den

    l1d_pp = _rate_pp(pre.l1d_misses, pre.l1_loads,
                      post.l1d_misses, post.l1_loads)
    bmisp_pp = _rate_pp(pre.branch_misses, pre.branches,
                        post.branch_misses, post.branches)

    drift_pct = (_pct_delta(post.task_clock_ms, absolute.task_clock_ms)
                 if absolute is not None else None)

    detail = {
        "pre_ms": pre.task_clock_ms, "post_ms": post.task_clock_ms,
        "delta_pct": delta_pct, "effective_tol_pct": effective_tol,
        "measured_cv": measured_cv, "drift_pct": drift_pct,
        "insns_delta_pct": insns_d, "cycles_delta_pct": cycles_d,
        "ipc_delta_pct": ipc_d, "l1d_miss_delta_pp": l1d_pp,
        "bmisp_delta_pp": bmisp_pp,
    }

    def _verdict(ok: bool, reason: str) -> W2Verdict:
        return W2Verdict(
            ok=ok, reason=reason, delta_pct=delta_pct,
            effective_tol_pct=effective_tol, measured_cv=measured_cv,
            drift_pct=drift_pct, insns_delta_pct=insns_d,
            cycles_delta_pct=cycles_d, ipc_delta_pct=ipc_d,
            l1d_miss_delta_pp=l1d_pp, bmisp_delta_pp=bmisp_pp,
            pre=pre, post=post, absolute=absolute, detail=detail)

    if delta_pct is None:
        return _verdict(False, "pre measurement had zero wall-clock")

    # 1. regression gate
    if delta_pct > effective_tol:
        return _verdict(False,
                        f"regressed +{delta_pct:.2f}% > tol {effective_tol:.2f}% "
                        f"(max(3×cv={3*measured_cv:.2f}%, floor={TOL_FLOOR}%))")

    # 2. anti-drift vs pristine absolute baseline
    if drift_pct is not None and drift_pct > ABS_TOL:
        return _verdict(False,
                        f"drift +{drift_pct:.2f}% > {ABS_TOL}% vs absolute "
                        f"baseline (paired-baseline drift, not a real win)")

    # 3. real-gain requirement for direct-type cards
    if require_perf_gain and delta_pct >= -3.0 * measured_cv:
        return _verdict(False,
                        f"direct card but no real gain (delta={delta_pct:+.2f}%, "
                        f"need ≤ -{3*measured_cv:.2f}% to clear 3σ)")

    return _verdict(True,
                    f"ok (delta={delta_pct:+.2f}%, tol={effective_tol:.2f}%)")
