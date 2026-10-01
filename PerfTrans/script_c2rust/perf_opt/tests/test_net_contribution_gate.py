"""A commit has to move the crate forward, not merely fail to break it.

The step gate asks "is this candidate locally harmless", against the parent,
over the ops `locate` happened to file the candidate's function under. The
total gate asks "did anything crate-wide fall off a cliff", against pristine,
over every op. Neither asks whether keeping the candidate is worth the relink
it costs — and committing anything relinks the whole image, moving ops the
edit never touched.

Measured, from the changesets on disk:

  http-parser  `http_message_needs_eof`, C3, step gate -0.168%
               crate went -68.907% -> -67.291%, a net LOSS of 1.615%
               `parse_url_and_meta`: -15.427% -> -5.716% at insns +0.036%

That op is 95.2% library and still improved against pristine, so
`_cumulative_op_regress` never fired; the aggregate moved 1.6 points on a
-68.9% base, so the total gate's aggregate never fired; and the step gate was
measuring a different op entirely. Three gates, no objection, 1.6 points gone.

The step gate could not have caught it. Its op list is `hf.per_op`'s keys, and
`locate` writes that from sampled self time with NO floor — a single perf
sample, 0.05%, decides whether an op is watched. Two runs of lil, identical
sources: five functions lost an op that way, libqrencode two. `lil_append_val`
was watched over three ops in one run and two in the next, and the op that
dropped out went from -0.083% to +3.633%.

So the last gate must not depend on op coverage at all. This one reads a
single number that is already paid for.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.gates import PerformanceSession


def _session(tmp_path: Path, **kw) -> PerformanceSession:
    binary = tmp_path / "harness"
    binary.write_bytes(b"\x7fELF-stub")
    binary.chmod(0o755)
    return PerformanceSession(
        pristine_bin=binary, opt_dir=tmp_path, all_ops=("a", "b"), **kw)


@pytest.fixture()
def totals(monkeypatch):
    """Drive the total gate's aggregate; the step gate always passes."""
    box: dict[str, float] = {"total": -9.0, "step": -9.0}

    class _Result:
        def __init__(self, aggregate: float) -> None:
            self.accepted = True
            self.reason = "safe"
            self.aggregate_mean_pct = aggregate
            self.aggregate_ci_low_pct = aggregate - 0.2
            self.aggregate_ci_high_pct = aggregate + 0.2
            self.checkpoint = 10
            self.unmeasurable_ops: tuple = ()
            self.per_op: dict = {}

    def fake(self, base_binary, candidate_binary, ops, **kwargs):
        kind = "total" if Path(base_binary) == self.pristine_bin else "step"
        return _Result(box[kind])

    monkeypatch.setattr(PerformanceSession, "_compare", fake, raising=True)
    return box


def _judge(session: PerformanceSession, tmp_path: Path):
    return session.gate_ops(tmp_path / "harness", ["a"])[0]


# ───────────────────────────────────────────── the gate itself

def test_a_candidate_that_earns_its_place_is_kept(tmp_path, totals):
    session = _session(tmp_path)
    session.parent_generation = 1
    totals["total"] = -5.0                    # 5 points better than pristine

    assert _judge(session, tmp_path).ok


def test_a_candidate_that_pays_for_nothing_is_rejected(tmp_path, totals):
    """0.05% of crate-wide movement does not buy a fresh layout draw."""
    session = _session(tmp_path)
    session.parent_generation = 1
    totals["total"] = -0.05

    verdict = _judge(session, tmp_path)
    assert not verdict.ok
    assert verdict.reason == "insufficient_net_gain"


def test_a_candidate_that_moves_the_crate_backward_is_rejected(tmp_path, totals):
    """The http-parser case: still far better than pristine, yet a net loss."""
    session = _session(tmp_path)
    session.parent_generation = 1
    session._parent_total_pct = -68.907
    totals["total"] = -67.291                 # net +1.616%

    verdict = _judge(session, tmp_path)
    assert not verdict.ok
    assert verdict.reason == "insufficient_net_gain"


def test_the_verdict_still_carries_both_measurements(tmp_path, totals):
    """A rejection nobody can attribute costs a re-measurement to read."""
    session = _session(tmp_path)
    session.parent_generation = 1
    totals["total"], totals["step"] = -0.05, -3.0

    gates_seen = {m["gate"] for m
                  in _judge(session, tmp_path).detail["measurements"]}
    assert gates_seen == {"step", "total"}


# ───────────────────────────────────────────── the baseline it compares to

def test_the_baseline_only_moves_on_promote(tmp_path, totals):
    """A rejected candidate must not become the next one's reference."""
    session = _session(tmp_path)
    session.parent_generation = 1
    totals["total"] = -0.1
    _judge(session, tmp_path)

    assert session._parent_total_pct == 0.0


def test_promote_advances_the_baseline(tmp_path, totals):
    session = _session(tmp_path)
    session.parent_generation = 1
    totals["total"] = -5.0
    assert _judge(session, tmp_path).ok

    session.promote_candidate(tmp_path / "harness", "deadbee")
    assert session._parent_total_pct == pytest.approx(-5.0)


def test_a_second_candidate_is_judged_against_the_first(tmp_path, totals):
    """-5.0% then -5.05% is a 0.05-point step, which does not clear the floor.

    The absolute figure is not what is judged: -5.05% against pristine is the
    best the crate has ever measured, and it is still rejected, because what
    it ADDS is nothing.
    """
    session = _session(tmp_path)
    session.parent_generation = 1
    totals["total"] = -5.0
    assert _judge(session, tmp_path).ok
    session.promote_candidate(tmp_path / "harness", "deadbee")

    totals["total"] = -5.05
    assert not _judge(session, tmp_path).ok

    totals["total"] = -5.5                    # 0.5 points clears it
    assert _judge(session, tmp_path).ok


def test_the_first_commit_seeds_the_baseline_from_the_step_gate(tmp_path, totals):
    """Its total gate is skipped as redundant — parent IS pristine and the
    candidate covers every op — so the step aggregate is the crate's position
    and the next candidate still gets something to be judged against."""
    session = _session(tmp_path)
    totals["step"] = -20.0
    assert session.gate_ops(tmp_path / "harness", ["a", "b"])[0].ok

    session.promote_candidate(tmp_path / "harness", "deadbee")
    assert session._parent_total_pct == pytest.approx(-20.0)


def test_a_missing_baseline_waives_the_check(tmp_path, totals):
    """Never guess. Without a reference the question has no answer."""
    session = _session(tmp_path)
    session.parent_generation = 1
    session._parent_total_pct = None
    totals["total"] = -0.001

    assert _judge(session, tmp_path).ok


# ───────────────────────────────────────────── replaying what actually ran

@pytest.mark.parametrize("parent, candidate, kept", [
    # http-parser: C9 lands, then four candidates that pay for nothing.
    (-68.922, -68.911, False),
    (-68.911, -68.907, False),
    (-68.907, -67.291, False),   # the 1.6-point loss, step gate said -0.168%
    (-67.291, -67.257, False),
    # lil: the one that pushed expr_embedded_builtins to +3.633%.
    (-13.755, -13.701, False),
    (-13.335, -13.184, False),
    (-12.877, -13.335, True),
    # miniz and libqrencode: real wins, untouched.
    (-19.977, -22.764, True),
    (-7.551, -12.225, True),
])
def test_replay_of_committed_changesets(tmp_path, totals, parent, candidate, kept):
    session = _session(tmp_path)
    session.parent_generation = 1
    session._parent_total_pct = parent
    totals["total"] = candidate

    assert _judge(session, tmp_path).ok is kept


# ───────────────────────────────────────────── which bucket it lands in

def test_the_rejection_is_filed_as_no_gain_not_regress() -> None:
    """Nothing regressed — both harm gates passed. The candidate just did not
    earn the relink it costs.

    Not cosmetic. A REGRESS verdict asks the agent to split the bundle and
    retry, on the theory that one rule inside is being dragged down by
    another, and that theory is tested with `w2_delta_pct < 0`. A verdict from
    the TOTAL gate carries the crate's position against pristine — -67% on
    http-parser — which is negative whatever the candidate did, so every such
    rejection would buy three more measurements to re-learn the same answer.
    Classification happens in agent.py's w2 callback, which collapses the
    verdict to one of four reason codes BEFORE the executor sees it — so the
    executor cannot do it, and a new verdict reason nobody maps there lands in
    the regress bucket by default. That is what the first http-parser run with
    this gate did: the rejections were correct, and filed as
    `rejected_w2_regress ... per_op_regress`.
    """
    import inspect

    from perf_opt.agent_perf_opt import agent

    src = inspect.getsource(agent)
    mapped = src.count('"insufficient_net_gain" in reason')
    callbacks = src.count("def w2_callback")
    assert mapped >= callbacks, (
        f"{callbacks} w2 callbacks, only {mapped} map the new reason")


def test_the_gate_emits_exactly_that_reason() -> None:
    """The two halves are in different modules; a rename would silently
    reroute the verdict into the regress bucket."""
    import inspect

    from perf_opt.agent_perf_opt.gates import PerformanceSession

    assert 'reason="insufficient_net_gain"' in inspect.getsource(
        PerformanceSession.gate_ops)
