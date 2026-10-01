"""Not every op in a candidate's gate is measuring the candidate.

Committing one edit relinks the whole binary. Functions move, and code that
was never touched lands in different cache lines and different branch-
predictor sets. The effect is not noise — it is a property of the new binary,
so more samples converge on it rather than averaging it out.

Measured on a compression crate. Seven commits, **none of which touched
`cache.rs`**, replayed from git and timed with `perf stat -r 15` under the
isolated measurement environment:

    commit    file the commit changed      run 1     run 2
    53130ab   (base)                       1.448     1.459
    ed6c0eb   squeeze.rs                   1.467     1.463
    3ba6989   squeeze.rs                   1.429     1.433
    6016279   hash.rs                      1.422     1.426
    cc61d79   lz77.rs                      1.483     1.483    <- +4.12%
    df42836   deflate.rs                   1.481     1.470
    545f703   tree.rs (one line)           1.477     1.479
    18f0a62   tree.rs                      1.460     1.459

The two independent runs agree to 0.74%, so a 4.12% move on an operation
whose source nobody edited is real and reproducible. Against a flat 1.0%
per-op ceiling, that rejects the candidate — and the log line is the same one
a genuine regression produces.

So the ceiling is now per-op: the operation the rewritten function actually
lives in keeps the tight one, and the ones it merely appears in get a ceiling
above the layout band. The aggregate gate (where opposite-signed drift
largely cancels) and the final holistic re-measurement are unchanged.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.config import AgentConfig
from perf_opt.agent_perf_opt.gates import W2Session
from perf_opt.agent_perf_opt.measurement import _limit_for


class _Session:
    """The ceiling policy alone, without a measurement backend."""
    bystander_op_upper_limit_pct = AgentConfig.w2_bystander_op_upper_limit_pct
    primary_op_share_pct = AgentConfig.w2_primary_op_share_pct
    catastrophic_regress_pct = AgentConfig.w2_catastrophic_regress_pct
    library_share: dict = {}                            
    _op_limits = W2Session._op_limits


OPS = ["tree_codegen", "cache_roundtrip", "zlib_compress"]


# ─────────────────────────────────────────── who gets which ceiling

def test_the_op_the_function_lives_in_keeps_the_tight_ceiling() -> None:
    weights = {"tree_codegen": 39.69, "cache_roundtrip": 0.4, "zlib_compress": 0.5}
    limits = _Session()._op_limits(OPS, weights)
    assert "tree_codegen" not in limits


def test_ops_the_function_barely_appears_in_are_relaxed() -> None:
    weights = {"tree_codegen": 39.69, "cache_roundtrip": 0.4, "zlib_compress": 0.5}
    limits = _Session()._op_limits(OPS, weights)
    assert limits["cache_roundtrip"] == _Session.bystander_op_upper_limit_pct
    assert limits["zlib_compress"] == _Session.bystander_op_upper_limit_pct


def test_a_function_hot_in_several_ops_stays_tight_in_all_of_them() -> None:
    """Relaxation is about absence of leverage, not about picking a winner."""
    weights = {"tree_codegen": 39.69, "cache_roundtrip": 20.75, "zlib_compress": 26.72}
    assert _Session()._op_limits(OPS, weights) is None


def test_an_op_missing_from_the_weights_is_a_bystander() -> None:
    """No recorded share means the profile never saw the function there."""
    limits = _Session()._op_limits(OPS, {"tree_codegen": 39.69})
    assert set(limits) == {"cache_roundtrip", "zlib_compress"}


# ─────────────────────────────────────────── the default path is untouched

@pytest.mark.parametrize("weights", [None, {}])
def test_no_weights_means_no_relaxation(weights) -> None:
    """Without a profile there is no basis to call anything a bystander, and
    the gate must behave exactly as it did before."""
    assert _Session()._op_limits(OPS, weights) is None


def test_no_overrides_yields_the_default_ceiling() -> None:
    assert _limit_for("any_op", 1.0, None) == 1.0


def test_an_op_without_an_override_keeps_the_default() -> None:
    assert _limit_for("tree_codegen", 1.0, {"cache_roundtrip": 3.0}) == 1.0


# ─────────────────────────────────────────── the ceiling can only loosen

def test_an_override_can_never_tighten_the_ceiling() -> None:
    """A ceiling below the configured one would reject candidates the
    unrelaxed gate accepts — the opposite of the intent, and a silent one."""
    assert _limit_for("op", 1.0, {"op": 0.1}) == 1.0
    assert _limit_for("op", 1.0, {"op": 3.0}) == 3.0


def test_the_relaxed_ceiling_clears_the_measured_drift_band() -> None:
    """0.79% median, 4.12% worst observed. The ceiling must sit above the
    common case; the worst case is left to be rejected, since a 4% move is
    also what a genuinely bad rewrite looks like."""
    assert AgentConfig.w2_bystander_op_upper_limit_pct > 0.79
    assert AgentConfig.w2_bystander_op_upper_limit_pct > \
        AgentConfig.w2_per_op_upper_limit_pct


def test_the_aggregate_tolerance_is_untouched() -> None:
    """Opposite-signed drift largely cancels in the weighted aggregate, so
    that gate had no reason to move."""
    assert AgentConfig.w2_regress_tolerance_pct == 0.3
