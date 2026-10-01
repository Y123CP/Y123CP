import json
import logging
from pathlib import Path
from types import SimpleNamespace

import pytest

from perf_opt.agent_perf_opt import measurement
from perf_opt.agent_perf_opt.measurement import compare_paired
from perf_opt.agent_perf_opt.gates import PerformanceSession
from perf_opt.agent_perf_opt.config import load_agent_config
from perf_opt.agent_perf_opt import driver


class FakeBackend:
    def __init__(self, values, *, missing=(), failures=()):
        self.values = {key: iter(seq) for key, seq in values.items()}
        self.missing = set(missing)
        self.failures = set(failures)
        self.prepare_calls = []
        self.measure_calls = []

    def prepare(self, binary, op, assets, target_wall):
        self.prepare_calls.append(op)
        if op in self.missing:
            return None
        return Path(f"{op}.bin"), 100

    def measure_once(self, binary, op, input_path, iters, pin_cpu):
        key = (Path(binary).name, op)
        self.measure_calls.append(key)
        if key in self.failures:
            raise RuntimeError(f"forced measurement failure: {key}")
        return next(self.values[key])


def _constant_values(parent, candidate, ops=("op",), count=41):
    values = {}
    for op in ops:
        values[("parent", op)] = [float(parent[op])] * count
        values[("candidate", op)] = [float(candidate[op])] * count
    return values


def _comparison(op: str, deltas: list[float]):
    samples = tuple(
        measurement.PairSample(
            pair_index=index,
            parent_ms=100.0,
            candidate_ms=100.0 * (1.0 + delta / 100.0),
        )
        for index, delta in enumerate(deltas)
    )
    mean, ci_low, ci_high = measurement._mean_ci(deltas)
    return measurement.OpComparison(
        op=op,
        samples=samples,
        mean_delta_pct=mean,
        ci_low_pct=ci_low,
        ci_high_pct=ci_high,
    )


def test_weighted_aggregate_supports_unequal_sample_counts() -> None:
    comparisons = {
        "stable": _comparison("stable", [-2.0] * 10),
        "noisy": _comparison("noisy", [-1.0, 1.0] * 10),
    }

    summary = measurement._weighted_aggregate_ci(
        comparisons, {"stable": 0.75, "noisy": 0.25}
    )

    assert summary.mean_pct == pytest.approx(-1.5)
    assert summary.ci_low_pct < summary.mean_pct < summary.ci_high_pct
    assert summary.method == "weighted_unequal_n_t"


def test_high_risk_regression_rejects_before_other_ops_prepare(
    tmp_path: Path,
) -> None:
    backend = FakeBackend(
        _constant_values(
            {"risky": 100.0, "ordinary": 100.0},
            {"risky": 107.0, "ordinary": 99.0},
            ops=("risky", "ordinary"),
            count=11,
        )
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("risky", "ordinary"),
        high_risk_ops=("risky",),
        assets=object(),
        checkpoints=(10, 20, 40),
    )

    assert not result.accepted
    assert result.reason == "per_op_regress"
    assert result.per_op["risky"].sample_count == 5
    assert backend.prepare_calls == ["risky"]


def test_five_pair_gain_cannot_accept_before_full_scope(tmp_path: Path) -> None:
    backend = FakeBackend(
        _constant_values(
            {"risky": 100.0, "ordinary": 100.0},
            {"risky": 95.0, "ordinary": 99.0},
            ops=("risky", "ordinary"),
            count=11,
        )
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("risky", "ordinary"),
        high_risk_ops=("risky",),
        assets=object(),
        checkpoints=(10, 20, 40),
    )

    assert result.accepted
    assert result.per_op["risky"].sample_count >= 10
    assert result.per_op["ordinary"].sample_count >= 10
    assert backend.prepare_calls == ["risky", "ordinary"]


def test_stable_op_stops_at_ten_while_noisy_op_reaches_later_checkpoint(
    tmp_path: Path, caplog: pytest.LogCaptureFixture,
) -> None:
    caplog.set_level(logging.INFO, logger=measurement.__name__)
    backend = FakeBackend(
        {
            ("parent", "stable"): [100.0] * 41,
            ("candidate", "stable"): [99.0] * 41,
            ("parent", "noisy"): [100.0] * 41,
            ("candidate", "noisy"): [96.0, 104.0] * 21,
        }
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("stable", "noisy"),
        assets=object(),
        checkpoints=(10, 20, 40),
    )

                                                         
                                        
    assert result.accepted
    assert result.per_op["stable"].sample_count == 10
    assert result.per_op["noisy"].sample_count == 40
    assert result.per_op["stable"].stop_reason == "safe"
    assert "op=stable stopped_at=10 reason=safe" in caplog.text


def test_clear_non_regression_accepts_at_first_checkpoint(tmp_path: Path) -> None:
                                                      
                                                 
                                                      
    backend = FakeBackend(
        {
            ("parent", "op"): [100.0] * 21,
            ("candidate", "op"): [98.8, 100.0] * 11,
        }
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("op",),
        assets=object(),
        checkpoints=(10, 20, 40),
    )

    assert result.accepted
    assert result.per_op["op"].sample_count == 10
    assert result.per_op["op"].stop_reason == "safe"


def test_measurement_exception_records_op_side_and_pair(tmp_path: Path) -> None:
    backend = FakeBackend(
        _constant_values({"op": 100.0}, {"op": 99.0}, count=11),
        failures=(("candidate", "op"),),
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("op",),
        assets=object(),
        checkpoints=(10,),
    )

    assert not result.accepted
    assert result.reason == "measurement_error"
    assert result.measurement_errors[0]["op"] == "op"
    assert result.measurement_errors[0]["side"] == "candidate"
    assert result.measurement_errors[0]["pair_index"] == -1


def test_non_positive_measurement_fails_closed(tmp_path: Path) -> None:
    backend = FakeBackend(
        {
            ("parent", "op"): [100.0] * 11,
            ("candidate", "op"): [0.0] + [99.0] * 10,
        }
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("op",),
        assets=object(),
        checkpoints=(10,),
    )

    assert not result.accepted
    assert result.reason == "measurement_error"
    assert "positive finite" in result.measurement_errors[0]["error"]


def test_candidate_is_compared_to_parent_not_pristine(tmp_path: Path) -> None:
    backend = FakeBackend(
        _constant_values({"op": 90.0}, {"op": 94.0})
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("op",),
        assets=object(),
        checkpoints=(10, 20, 40),
        per_op_upper_limit_pct=1.0,
        regress_tolerance_pct=0.3,
        catastrophic_regress_pct=5.0,
    )

    assert not result.accepted
    assert result.reason == "per_op_regress"
    assert result.per_op["op"].mean_delta_pct == pytest.approx(
        4.444444, rel=1e-5
    )


def test_clear_aggregate_gain_is_accepted(tmp_path: Path) -> None:
    backend = FakeBackend(
        _constant_values({"op": 100.0}, {"op": 99.0})
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("op",),
        assets=object(),
        checkpoints=(10, 20, 40),
    )

    assert result.accepted
    assert result.reason == "accepted"
    assert result.aggregate_mean_pct == pytest.approx(-1.0)


def test_one_regressing_op_is_not_hidden_by_other_gain(tmp_path: Path) -> None:
    backend = FakeBackend(
        _constant_values(
            {"fast": 100.0, "regress": 100.0},
            {"fast": 90.0, "regress": 102.0},
            ops=("fast", "regress"),
        )
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("fast", "regress"),
        assets=object(),
        checkpoints=(10, 20, 40),
    )

    assert not result.accepted
    assert result.reason == "per_op_regress"
    assert result.aggregate_mean_pct == pytest.approx(-4.0)


def test_confident_aggregate_gain_accepts_despite_persistently_uncertain_op(
    tmp_path: Path,
) -> None:
    ""                                           

                                                         
                                                             
                                                  
       
       
    backend = FakeBackend(
        {
            ("parent", "gain"): [100.0] * 41,
            ("candidate", "gain"): [95.0] * 41,
            ("parent", "noisy"): [100.0] * 41,
            ("candidate", "noisy"): [96.0, 104.0] * 21,
        }
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("gain", "noisy"),
        assets=object(),
        checkpoints=(10, 20, 40),
    )

    assert result.accepted
    assert result.reason == "accepted"
    assert result.aggregate_ci_high_pct < -0.3                   
    assert result.per_op["noisy"].ci_low_pct <= 1.0                   
    assert result.per_op["noisy"].ci_high_pct > 1.0                
    assert result.per_op["noisy"].sample_count == 40           


def test_missing_expected_op_is_unmeasurable(tmp_path: Path) -> None:
    backend = FakeBackend({}, missing=("missing",))

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("missing",),
        assets=object(),
    )

    assert not result.accepted
    assert result.reason == "unmeasurable"
    assert result.unmeasurable_ops == ("missing",)


def test_direct_comparison_does_not_sum_historical_marginal_deltas(
    tmp_path: Path,
) -> None:
    backend = FakeBackend(
        _constant_values({"op": 100.0}, {"op": 92.0})
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("op",),
        assets=object(),
        checkpoints=(10,),
    )

    assert result.aggregate_mean_pct == pytest.approx(-8.0)
    assert result.aggregate_mean_pct != pytest.approx(-4.0 + -5.0)


def test_pair_order_alternates_ab_then_ba(tmp_path: Path) -> None:
    backend = FakeBackend(
        _constant_values({"op": 100.0}, {"op": 100.0}, count=11)
    )

    result = compare_paired(
        backend,
        tmp_path / "parent",
        tmp_path / "candidate",
        ops=("op",),
        assets=object(),
        checkpoints=(10,),
    )

                                           
    assert result.reason == "accepted"
    expected = [("parent", "op"), ("candidate", "op")]
    for pair_index in range(10):
        expected.extend(
            [("parent", "op"), ("candidate", "op")]
            if pair_index % 2 == 0
            else [("candidate", "op"), ("parent", "op")]
        )
    assert backend.measure_calls == expected


def test_checkpoint_plan_cannot_lower_minimum_coverage_below_ten(
    tmp_path: Path,
) -> None:
    backend = FakeBackend(
        _constant_values({"op": 100.0}, {"op": 99.0}, count=7)
    )

    with pytest.raises(ValueError, match="at least 10"):
        compare_paired(
            backend,
            tmp_path / "parent",
            tmp_path / "candidate",
            ops=("op",),
            assets=object(),
            checkpoints=(6,),
        )


def test_aggregate_regress_beyond_tolerance_rejected(tmp_path: Path) -> None:
                                                          
    backend = FakeBackend(_constant_values({"op": 100.0}, {"op": 100.6}))
    result = compare_paired(
        backend, tmp_path / "parent", tmp_path / "candidate",
        ops=("op",), assets=object(), checkpoints=(10, 20, 40),
    )
    assert not result.accepted
    assert result.reason == "no_gain"


def test_aggregate_only_skips_per_op_early_reject(tmp_path: Path) -> None:
                                                              
                                                           
                                                      
                                                  
    normal = compare_paired(
        FakeBackend(_constant_values(
            {"a": 100.0, "b": 100.0}, {"a": 95.0, "b": 102.0}, ops=("a", "b"))),
        tmp_path / "parent", tmp_path / "candidate",
        ops=("a", "b"), assets=object(), checkpoints=(10, 20, 40),
    )
    assert not normal.accepted
    assert normal.reason == "per_op_regress"

    agg = compare_paired(
        FakeBackend(_constant_values(
            {"a": 100.0, "b": 100.0}, {"a": 95.0, "b": 102.0}, ops=("a", "b"))),
        tmp_path / "parent", tmp_path / "candidate",
        ops=("a", "b"), assets=object(), checkpoints=(10, 20, 40),
        aggregate_only=True,
    )
    assert agg.accepted
    assert agg.reason == "accepted"


def test_total_gate_rejects_cumulative_regress_vs_base(tmp_path: Path) -> None:
                                                         
                                                           
    # candidate=105。
    pristine = tmp_path / "initial"
    candidate = tmp_path / "candidate"
    pristine.write_bytes(b"pristine")
    candidate.write_bytes(b"candidate")
    backend = FakeBackend(
        {
            ("pristine-harness", "op"): [100.0] * 200,
            ("parent-harness", "op"): [105.0] * 200,
            ("candidate", "op"): [105.0] * 200,
        }
    )
    session = PerformanceSession(
        pristine_bin=pristine, assets=object(),
        opt_dir=tmp_path / "opt", backend=backend, checkpoints=(10, 20, 40),
    )
    session.parent_generation = 1                                

    verdict, _ = session.gate_ops(candidate, ["op"])

    assert not verdict.ok
    assert verdict.reason == "total_regress"


def test_total_gate_passes_within_base_tolerance(tmp_path: Path) -> None:
                                                                        
    pristine = tmp_path / "initial"
    candidate = tmp_path / "candidate"
    pristine.write_bytes(b"pristine")
    candidate.write_bytes(b"candidate")
    backend = FakeBackend(
        {
            ("pristine-harness", "op"): [100.0] * 200,
            ("parent-harness", "op"): [100.0] * 200,
            ("candidate", "op"): [100.0] * 200,
        }
    )
    session = PerformanceSession(
        pristine_bin=pristine, assets=object(),
        opt_dir=tmp_path / "opt", backend=backend, checkpoints=(10, 20, 40),
    )
    session.parent_generation = 1
    # This case is about the TOTAL gate's tolerance. A candidate that measures
    # exactly neutral also earns nothing, which the net-gain gate rejects on
    # purpose (see test_net_contribution_gate.py); waiving it here keeps this
    # assertion about the gate it names.
    session._parent_total_pct = None

    verdict, _ = session.gate_ops(candidate, ["op"])

    assert verdict.ok


def test_session_promotes_candidate_only_after_commit(tmp_path: Path) -> None:
    pristine = tmp_path / "initial"
    candidate = tmp_path / "candidate"
    pristine.write_bytes(b"pristine")
    candidate.write_bytes(b"candidate")
    backend = FakeBackend(
        {
            ("parent-harness", "op"): [100.0] * 11,
            ("candidate", "op"): [99.0] * 11,
        }
    )
    session = PerformanceSession(
        pristine_bin=pristine,
        assets=object(),
        opt_dir=tmp_path / "opt",
        backend=backend,
        checkpoints=(10,),
    )

    decision = session.compare_candidate(candidate, ("op",))

    assert decision.accepted
    assert session.parent_generation == 0
    assert session.parent_bin.read_bytes() == b"pristine"
    session.promote_candidate(candidate, "abc123")
    assert session.parent_generation == 1
    assert session.parent_commit == "abc123"
    assert session.parent_bin.read_bytes() == b"candidate"


def test_final_total_is_direct_pristine_comparison_not_sum(tmp_path: Path) -> None:
    pristine = tmp_path / "initial"
    final = tmp_path / "final"
    pristine.write_bytes(b"pristine")
    final.write_bytes(b"final")
    backend = FakeBackend(
        {
            ("pristine-harness", "op"): [100.0] * 11,
            ("final", "op"): [92.0] * 11,
        }
    )
    session = PerformanceSession(
        pristine_bin=pristine,
        assets=object(),
        opt_dir=tmp_path / "opt",
        backend=backend,
        checkpoints=(10,),
    )

    report = session.measure_final(final, ("op",))

    assert report.aggregate_mean_pct == pytest.approx(-8.0)
    assert report.aggregate_mean_pct != pytest.approx(-4.0 + -5.0)
    assert (tmp_path / "opt" / "final_measurement.json").is_file()


def test_zero_commits_equal_hashes_skip_final_measurement(tmp_path: Path) -> None:
    pristine = tmp_path / "initial"
    pristine.write_bytes(b"identical")
    backend = FakeBackend({})
    session = PerformanceSession(
        pristine_bin=pristine,
        assets=object(),
        opt_dir=tmp_path / "opt",
        backend=backend,
        checkpoints=(10,),
    )

    skipped = session.maybe_skip_final(committed_attempts=0)

    assert skipped == {
        "status": "skipped",
        "reason": "no_accepted_commits",
        "committed_attempts": 0,
        "binaries_identical": True,
    }
    assert backend.measure_calls == []
    assert json.loads(
        (tmp_path / "opt" / "final_measurement.json").read_text()
    ) == skipped


def test_zero_commits_unequal_hashes_run_final_measurement(
    tmp_path: Path,
) -> None:
    pristine = tmp_path / "initial"
    pristine.write_bytes(b"pristine")
    session = PerformanceSession(
        pristine_bin=pristine,
        assets=object(),
        opt_dir=tmp_path / "opt",
        backend=FakeBackend({}),
        checkpoints=(10,),
    )
    # Snapshots are frozen 0o555, so the mutation has to unfreeze first — the
    # same thing anything running as this user could do. Read-only stops a
    # stray write; it is _assert_intact's digest that catches a deliberate one.
    session.parent_bin.chmod(0o755)
    session.parent_bin.write_bytes(b"unexpected mutation")

    assert session.maybe_skip_final(committed_attempts=0) is None


def test_accepted_commit_always_runs_final_measurement(tmp_path: Path) -> None:
    pristine = tmp_path / "initial"
    candidate = tmp_path / "candidate"
    pristine.write_bytes(b"pristine")
    candidate.write_bytes(b"candidate")
    session = PerformanceSession(
        pristine_bin=pristine,
        assets=object(),
        opt_dir=tmp_path / "opt",
        backend=FakeBackend({}),
        checkpoints=(10,),
    )
    session.promote_candidate(candidate, "commit-1")

    assert session.maybe_skip_final(committed_attempts=1) is None


def test_pair_checkpoint_env_override_is_integer_tuple(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.setenv("AGENT_W2_PAIR_CHECKPOINTS", "6,12,24")

    cfg = load_agent_config()

    assert cfg.w2_pair_checkpoints == (6, 12, 24)


def test_baseline_schema_carries_identity_and_measurements(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    harness_bin = tmp_path / "harness"
    harness_bin.write_bytes(b"binary")
    input_path = tmp_path / "op.bin"
    input_path.write_bytes(b"input")
    assets = SimpleNamespace(harness_src=tmp_path)
    monkeypatch.setattr(driver._measure, "pick_input", lambda *args: input_path)
    monkeypatch.setattr(driver, "autotune_iters", lambda *args, **kwargs: 7)
    monkeypatch.setattr(
        driver,
        "measure_harness",
        lambda *args, **kwargs: SimpleNamespace(
            task_clock_ms=10.0, instructions=123, cv_pct=0.1
        ),
    )

    baseline = driver._initial_baseline(
        harness_bin,
        assets,
        ["op"],
        tmp_path,
        source_head="head-1",
    )

    assert baseline["schema_version"] == 2
    assert baseline["identity"]["source_head"] == "head-1"
    assert len(baseline["identity"]["harness_sha256"]) == 64
    assert len(baseline["identity"]["ops_sha256"]) == 64
    assert baseline["measurements"]["op"]["wall_ms"] == 10.0
