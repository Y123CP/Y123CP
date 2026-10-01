"""A verdict that keeps only its aggregate throws away the run's own evidence.

Every candidate pays for a full paired measurement of every op — 40-plus
minutes of exclusive machine time on one crate. What reached disk was a single
number:

    w2_delta_pct = -9.366289494826727

That is enough to decide, and not enough to explain. Two questions come up
about every judged candidate and neither can be answered from it:

  * **A rejection.** `per_op_regress, triggering_op=cache_roundtrip` says
    which op vetoed and not by how much. The threshold is 5%; code layout on
    an untouched op has been measured at up to 4.12%, reproducible across
    independent runs. So "that op collapsed" and "that op drifted just past
    the ceiling" produce identical records, and the function in question held
    0% of that op's time — meaning it could not have collapsed it causally.
    Telling those apart afterwards costs a re-measurement, which costs as much
    as the run did.

  * **A commit.** A bundle of three rules commits at -9.37% aggregate and
    nothing on disk attributes any part of it. Asked what the `qsort` rewrite
    alone was worth, the run cannot say.

The numbers exist at decision time — `PairedComparison.per_op` holds a mean,
a confidence interval, a sample count and a stop reason for every op. They
were simply not written down. So they are, for both gates: the one that
produced the verdict and the one that also ran.
"""

from __future__ import annotations

import ast
import inspect
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt import agent, gates


# ─────────────────────────────── the rows themselves

class _Item:
    def __init__(self, mean, lo, hi, n=10, state="stopped", stop="safe"):
        self.mean_delta_pct = mean
        self.ci_low_pct = lo
        self.ci_high_pct = hi
        self.sample_count = n
        self.state = state
        self.stop_reason = stop


class _Cmp:
    def __init__(self, per_op, *, accepted=True, aggregate=-9.37, reason="accepted"):
        self.per_op = per_op
        self.accepted = accepted
        self.reason = reason
        self.aggregate_mean_pct = aggregate
        self.aggregate_ci_low_pct = aggregate - 0.4
        self.aggregate_ci_high_pct = aggregate + 0.4
        self.checkpoint = 10
        self.unmeasurable_ops = ()


def _session(monkeypatch, step, total=None):
    """A session whose two gate comparisons are the fixtures given."""
    s = gates.PerformanceSession.__new__(gates.PerformanceSession)
    s.per_op_upper_limit_pct = 1.0
    s.bystander_op_upper_limit_pct = 3.0
    s.primary_op_share_pct = 10.0
    s.regress_tolerance_pct = 0.3
    s.catastrophic_regress_pct = 5.0
    s.cumulative_op_share_pct = 50.0
    s.cumulative_op_limit_pct = 2.0
    s.cumulative_op_limit_strict_pct = 1.0
    s.min_total_gain_pct = 0.3
    # These fixtures model ONE candidate, not a commit history, so there is no
    # earlier position to be judged against. `None` is what the session itself
    # carries in that situation and it waives the net-gain gate — which is a
    # different gate, tested in test_net_contribution_gate.py. Pinning a number
    # here would make every assertion below depend on the gate's floor.
    s._parent_total_pct = None
    s._pending_total_pct = None
    s.library_share = {}
    s.parent_bin = Path("/parent")
    s.pristine_bin = Path("/pristine")
    s.parent_generation = 1
    s.all_ops = ["a", "b", "c"]
    calls = []

    def fake_compare(base, cand, ops, **kw):
        calls.append((str(base), tuple(ops)))
        return step if len(calls) == 1 else (total if total is not None else step)

    monkeypatch.setattr(s, "_compare", fake_compare)
    return s, calls


STEP = _Cmp({
    "a": _Item(-30.7, -31.4, -30.0),
    "b": _Item(0.12, -0.31, 0.55),
})


def test_every_measured_op_gets_a_row(monkeypatch) -> None:
    s, _ = _session(monkeypatch, STEP)
    verdict, _ = s.gate_ops(Path("/cand"), ["a", "b"], op_weights={"a": 90.0})
    ops = verdict.detail["measurements"][0]["ops"]
    assert [row["op"] for row in ops] == ["a", "b"]


def test_a_row_carries_magnitude_interval_and_sample_count(monkeypatch) -> None:
    """The interval is what separates "collapsed" from "drifted"; the count is
    what says whether the measurement had converged when it stopped."""
    s, _ = _session(monkeypatch, STEP)
    verdict, _ = s.gate_ops(Path("/cand"), ["a", "b"], op_weights={"a": 90.0})
    row = verdict.detail["measurements"][0]["ops"][0]
    assert row["mean_delta_pct"] == pytest.approx(-30.7)
    assert row["ci_low_pct"] == pytest.approx(-31.4)
    assert row["ci_high_pct"] == pytest.approx(-30.0)
    assert row["samples"] == 10
    assert row["stop_reason"] == "safe"


def test_the_aggregate_is_recorded_beside_the_rows(monkeypatch) -> None:
    s, _ = _session(monkeypatch, STEP)
    verdict, _ = s.gate_ops(Path("/cand"), ["a", "b"], op_weights={"a": 90.0})
    block = verdict.detail["measurements"][0]
    assert block["aggregate_pct"] == pytest.approx(-9.37)
    assert block["accepted"] is True


# ─────────────────────────────── both gates, not just the deciding one

def test_a_candidate_passing_both_gates_records_both(monkeypatch) -> None:
    """The step gate produces the verdict, so the total gate's numbers would
    otherwise be paid for and discarded."""
    total = _Cmp({"a": _Item(-28.0, -29.0, -27.0),
                  "c": _Item(1.2, 0.4, 2.0)}, aggregate=-8.1)
    s, _ = _session(monkeypatch, STEP, total)
    verdict, _ = s.gate_ops(Path("/cand"), ["a", "b"], op_weights={"a": 90.0})
    blocks = {b["gate"]: b for b in verdict.detail["measurements"]}
    assert set(blocks) == {"step", "total"}
    assert [r["op"] for r in blocks["total"]["ops"]] == ["a", "c"]


def test_each_block_names_what_it_was_measured_against(monkeypatch) -> None:
    """Step is vs the parent commit, total is vs the pristine base. A delta
    without its reference is not interpretable."""
    total = _Cmp({"c": _Item(1.2, 0.4, 2.0)}, aggregate=-8.1)
    s, _ = _session(monkeypatch, STEP, total)
    verdict, _ = s.gate_ops(Path("/cand"), ["a", "b"], op_weights={"a": 90.0})
    blocks = {b["gate"]: b for b in verdict.detail["measurements"]}
    assert blocks["step"]["reference"] == "parent"
    assert blocks["total"]["reference"] == "pristine"


def test_a_total_gate_rejection_records_the_step_gate_too(monkeypatch) -> None:
    """The rejection case is the one that most needs both: the step gate
    passing is exactly the evidence that the vetoing op is not the candidate's
    own."""
    total = _Cmp({"cache": _Item(6.1, 5.2, 7.0)}, accepted=False,
                 aggregate=0.081, reason="per_op_regress")
    s, _ = _session(monkeypatch, STEP, total)
    verdict, _ = s.gate_ops(Path("/cand"), ["a", "b"], op_weights={"a": 90.0})
    assert verdict.ok is False
    blocks = {b["gate"]: b for b in verdict.detail["measurements"]}
    assert set(blocks) == {"step", "total"}
    assert blocks["total"]["ops"][0]["ci_low_pct"] == pytest.approx(5.2)
    assert blocks["step"]["accepted"] is True


def test_a_step_gate_rejection_records_what_it_had(monkeypatch) -> None:
    """The total gate never ran, so there is one block and it says so."""
    step = _Cmp({"a": _Item(3.0, 2.1, 3.9)}, accepted=False,
                aggregate=3.0, reason="per_op_regress")
    s, _ = _session(monkeypatch, step)
    verdict, _ = s.gate_ops(Path("/cand"), ["a"], op_weights={"a": 90.0})
    assert [b["gate"] for b in verdict.detail["measurements"]] == ["step"]


# ─────────────────────────────── it reaches disk from every record point

def _w2_record_dicts() -> list[ast.Dict]:
    """Every dict literal built for a `w2` GateResultRecord in the agent."""
    tree = ast.parse(inspect.getsource(agent))
    found = []
    for node in ast.walk(tree):
        if not (isinstance(node, ast.Call)
                and getattr(node.func, "id", "") == "GateResultRecord"):
            continue
        args = list(node.args)
        if not args or not (isinstance(args[0], ast.Constant)
                            and args[0].value == "w2"):
            continue
        payload = next((a for a in args if isinstance(a, ast.Dict)), None)
        if payload is not None:
            found.append(payload)
    return found


def test_the_agent_has_more_than_one_w2_record_point() -> None:
    """Guard the guard: if this ever finds one, the AST walk has drifted and
    the test below stopped covering anything."""
    assert len(_w2_record_dicts()) >= 4


def test_every_w2_record_point_persists_the_per_op_rows() -> None:
    """The failure this prevents is not "it does not work" but "it works on
    the path I looked at". Four separate call sites build this record; a fix
    installed in one of them leaves the other three writing aggregates only,
    and every test still passes.
    """
    missing = [
        i for i, payload in enumerate(_w2_record_dicts())
        if not any(isinstance(k, ast.Constant) and k.value == "measurements"
                   for k in payload.keys)
    ]
    assert not missing, (
        f"{len(missing)} of {len(_w2_record_dicts())} w2 record points drop "
        "the per-op rows")


def test_every_w2_record_point_still_persists_the_aggregate() -> None:
    """Adding the rows must not have displaced what was already recorded."""
    for payload in _w2_record_dicts():
        keys = {k.value for k in payload.keys
                if isinstance(k, ast.Constant)}
        assert {"delta_pct", "triggering_op", "ops"} <= keys


def test_the_rows_survive_json_round_trip(monkeypatch) -> None:
    """`result.json` is written with `json.dump`; a tuple or a dataclass in
    there would raise at the moment of recording, after the measurement was
    already paid for."""
    import json
    total = _Cmp({"c": _Item(1.2, 0.4, 2.0, stop=None)}, aggregate=-8.1)
    s, _ = _session(monkeypatch, STEP, total)
    verdict, _ = s.gate_ops(Path("/cand"), ["a", "b"], op_weights={"a": 90.0})
    assert json.loads(json.dumps(verdict.detail))["measurements"][0]["ops"]


# ─────────────────────────────── recording must not be able to break gating

def test_a_verdict_without_the_field_records_nothing_and_does_not_raise() -> None:
    """This runs while a gate result is being written. An exception here would
    turn a measured, PASSING candidate into a gate failure — the record of the
    measurement destroying its subject. Not hypothetical: reaching straight
    for `.detail` did exactly that to three existing tests, reporting a
    committed rewrite as `syntax_error`."""
    class _Bare:
        ok = True
    assert agent._w2_measurements(_Bare()) == []


@pytest.mark.parametrize("detail", [None, "text", 7, {"measurements": "x"},
                                    {"measurements": None}, {}])
def test_any_unusable_detail_degrades_to_empty(detail) -> None:
    class _V:
        pass
    v = _V()
    v.detail = detail
    assert agent._w2_measurements(v) == []


def test_a_comparison_missing_descriptive_fields_still_yields_a_block(monkeypatch) -> None:
    """Same principle one level down, inside the gate. Only the fields the
    VERDICT is built from are read directly; the ones that merely describe it
    are forgiving, so a partial result object cannot reject a candidate."""
    class _Partial:
        accepted = True
        reason = "safe"
        aggregate_mean_pct = -1.0
        per_op: dict = {}

    s, _ = _session(monkeypatch, _Partial())
    verdict, _ = s.gate_ops(Path("/cand"), ["a"], op_weights={"a": 90.0})
    block = verdict.detail["measurements"][0]
    assert block["accepted"] is True
    assert block["aggregate_ci_low_pct"] is None
    assert block["ops"] == []


def test_an_op_row_missing_a_number_records_none_not_a_crash(monkeypatch) -> None:
    class _PartialItem:
        mean_delta_pct = -3.0

    s, _ = _session(monkeypatch, _Cmp({"a": _PartialItem()}))
    verdict, _ = s.gate_ops(Path("/cand"), ["a"], op_weights={"a": 90.0})
    row = verdict.detail["measurements"][0]["ops"][0]
    assert row["mean_delta_pct"] == pytest.approx(-3.0)
    assert row["ci_low_pct"] is None
    assert row["samples"] == 0
