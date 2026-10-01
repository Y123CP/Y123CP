""                                                                        

                                                                          
                                                                           
                                                                            
                                                             

                                                                          
                                                    
                                                    
                                                    

                                                                          
                                                                    

                                         

                                                                          
                                                                 
                                                                       
                                                                              
                                                                         

                                                                            
                                                                             
   

from __future__ import annotations

from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.gates import PerformanceSession


class _RecordingBackend:
    """Captures which ops each comparison was asked to measure."""

    def __init__(self) -> None:
        self.calls: list[tuple[str, tuple[str, ...]]] = []


def _session(tmp_path: Path, all_ops=(), **kw) -> PerformanceSession:
    binary = tmp_path / "harness"
    binary.write_bytes(b"\x7fELF-stub")
    binary.chmod(0o755)
    return PerformanceSession(
        pristine_bin=binary, opt_dir=tmp_path, all_ops=all_ops, **kw)


@pytest.fixture()
def spy(monkeypatch):
    """Intercept `_compare`, recording (base_kind, ops) per call."""
    seen: list[tuple[str, tuple[str, ...]]] = []

    class _Result:
        # Mirrors `PairedComparison`'s full shape, not just the fields this
        # test reads: the gate now also records the descriptive ones, and a
        # fake that models half the type stops catching real breakage.
        accepted = True
        reason = "safe"
        aggregate_mean_pct = -0.5
        aggregate_ci_low_pct = -0.5 - 0.2
        aggregate_ci_high_pct = -0.5 + 0.2
        checkpoint = 10
        unmeasurable_ops: tuple = ()
        per_op: dict = {}

    def fake(self, base_binary, candidate_binary, ops, **kwargs):
        kind = "pristine" if Path(base_binary) == self.pristine_bin else "parent"
        seen.append((kind, tuple(ops)))
        return _Result()

    monkeypatch.setattr(PerformanceSession, "_compare", fake, raising=True)
    return seen


# ───────────────────────────────────────────── scope

def test_total_gate_measures_every_op_not_just_the_candidate_s(tmp_path, spy):
    session = _session(tmp_path, all_ops=("a", "b", "c", "d"))
    session.parent_generation = 1
    session.gate_ops(tmp_path / "harness", ["a"])

    step = [ops for kind, ops in spy if kind == "parent"]
    total = [ops for kind, ops in spy if kind == "pristine"]
    assert step == [("a",)], "step gate stays scoped to the candidate"
    assert total == [("a", "b", "c", "d")], "total gate covers the crate"


def test_the_first_commit_is_not_exempt(tmp_path, spy):
    """It used to be skipped when parent == pristine. That reasoning assumed
    both gates measure the same ops; they no longer do. On optipng the very
    first commit was the one that left an op 13% slower."""
    session = _session(tmp_path, all_ops=("a", "b", "c"))
    assert session.parent_generation == 0
    session.gate_ops(tmp_path / "harness", ["a"])

    assert [ops for kind, ops in spy if kind == "pristine"] == [("a", "b", "c")]


def test_no_redundant_total_when_scopes_coincide(tmp_path, spy):
    """First commit AND the candidate already covers every op: the two gates
    would measure the same thing, so the second is pure cost."""
    session = _session(tmp_path, all_ops=("a", "b"))
    session.gate_ops(tmp_path / "harness", ["a", "b"])

    assert [kind for kind, _ in spy] == ["parent"]


def test_falls_back_to_candidate_ops_when_the_full_set_is_unknown(tmp_path, spy):
    """A caller that never supplied `all_ops` keeps the old behaviour rather
    than silently measuring nothing."""
    session = _session(tmp_path, all_ops=())
    session.parent_generation = 1
    session.gate_ops(tmp_path / "harness", ["a"])

    assert [ops for kind, ops in spy if kind == "pristine"] == [("a",)]


# ───────────────────────────────────────────── the op set itself

def test_all_ops_is_deduplicated_and_ordered(tmp_path):
    session = _session(tmp_path, all_ops=("b", "a", "b", "c", "a"))
    assert session.all_ops == ("b", "a", "c")


def test_all_ops_defaults_to_empty(tmp_path):
    assert _session(tmp_path).all_ops == ()


# ───────────────────────────────────────────── ordering

def test_the_step_gate_runs_first_and_short_circuits(tmp_path, monkeypatch):
    """A candidate that regresses its own ops must not cost a full-crate
    measurement — the cheap, decisive check comes first."""
    seen: list[str] = []

    class _Rejected:
        # Mirrors `PairedComparison`'s full shape, not just the fields this
        # test reads: the gate now also records the descriptive ones, and a
        # fake that models half the type stops catching real breakage.
        accepted = False
        reason = "per_op_regress"
        aggregate_mean_pct = 4.0
        aggregate_ci_low_pct = 4.0 - 0.2
        aggregate_ci_high_pct = 4.0 + 0.2
        checkpoint = 10
        unmeasurable_ops: tuple = ()
        per_op: dict = {}

    def fake(self, base_binary, candidate_binary, ops, **kwargs):
        seen.append("pristine" if Path(base_binary) == self.pristine_bin
                    else "parent")
        return _Rejected()

    monkeypatch.setattr(PerformanceSession, "_compare", fake, raising=True)
    session = _session(tmp_path, all_ops=("a", "b", "c"))
    session.parent_generation = 1
    verdict, _ = session.gate_ops(tmp_path / "harness", ["a"])

    assert seen == ["parent"], "total gate must not run after a step rejection"
    assert not verdict.ok
