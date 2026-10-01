import hashlib
import json
import subprocess
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.audit import AuditWriter
from perf_opt.agent_perf_opt.changeset.executor import (
    ChangeSetExecutor,
    ChangeSetInfrastructureError,
)
from perf_opt.agent_perf_opt.changeset.registry import HandlerRegistry
from perf_opt.agent_perf_opt.changeset.types import (
    ChangeSetStatus,
    ConcreteEdit,
    GateResultRecord,
    ImpactScope,
    OperationResolution,
    ReplaceFunctionBody,
    ProposedChangeSet,
    SymbolKind,
    SymbolRef,
    TriggerContext,
    ValidationResult,
)
from perf_opt.agent_perf_opt.state import StateManager


class FakeState:
    def __init__(self, *, fail_commit: bool = False) -> None:
        self.commits: list[tuple[tuple[Path, ...], str]] = []
        self.fail_commit = fail_commit

    def head_sha(self, *, full: bool = True) -> str:
        return "head-1"

    def commit_success(self, paths, message: str) -> str:
        resolved = tuple(Path(path) for path in paths)
        self.commits.append((resolved, message))
        if self.fail_commit:
            raise RuntimeError("injected commit failure")
        return "commit-1"


class StaticKeywordHandler:
    def __init__(self, calls: list[str], *, fail_post: bool = False) -> None:
        self.calls = calls
        self.fail_post = fail_post

    def resolve(self, operation, crate: Path) -> OperationResolution:
        self.calls.append("resolve")
        path = crate / "lib.rs"
        data = path.read_bytes()
        start = data.index(b"static")
        edit = ConcreteEdit(
            edit_id="edit-1",
            operation_id=operation.operation_id,
            relative_path="lib.rs",
            start_byte=start,
            end_byte=start + len(b"static"),
            before_hash=hashlib.sha256(data).hexdigest(),
            replacement_text="const",
        )
        return OperationResolution(operation.operation_id, (edit,))

    def pre_validate(self, operation, resolution, crate: Path) -> ValidationResult:
        self.calls.append("pre")
        return ValidationResult(True, "handler_pre")

    def post_validate(self, operation, resolution, crate: Path) -> ValidationResult:
        self.calls.append("post")
        if self.fail_post:
            return ValidationResult(False, "handler_post", "injected post failure")
        assert (crate / "lib.rs").read_text().startswith("const")
        return ValidationResult(True, "handler_post")


def _proposal() -> ProposedChangeSet:
    operation = ReplaceFunctionBody(
        operation_id="op-1",
        rule_id="C1",
        evidence_hit_ids=("hit-1",),
        target=SymbolRef(
            qualified_name="crate::A",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint="lib.rs",
            declaration_hash="evidence-hash",
        ),
        replacement_function_source="const A: u32 = 1;\n",
    )
    return ProposedChangeSet(
        changeset_id="cs-1",
        base_head="head-1",
        trigger=TriggerContext(
            rule_id="C1",
            candidate_id="candidate-1",
            hot_function="hot",
            hit_ids=("hit-1",),
            evidence_artifacts=(),
        ),
        operations=(operation,),
        impact_scope=ImpactScope.LOCAL_FUNCTION,
    )


def _gate(name: str, calls: list[str], *, ok: bool = True, reason: str = ""):
    def run(*args) -> GateResultRecord:
        calls.append(name)
        return GateResultRecord(name, ok, {"reason": reason})

    return run


def _executor(
    tmp_path: Path,
    *,
    failed_gate: str | None = None,
    fail_parent: bool = False,
    fail_commit: bool = False,
):
    crate = tmp_path / "crate"
    crate.mkdir()
    source = crate / "lib.rs"
    source.write_text("static A: u32 = 1;\n")
    candidate = tmp_path / "candidate-bin"
    candidate.write_bytes(b"candidate")
    calls: list[str] = []
    handler = StaticKeywordHandler(calls, fail_post=failed_gate == "post")
    registry = HandlerRegistry()
    registry.register(ReplaceFunctionBody, handler)
    state = FakeState(fail_commit=fail_commit)
    promotions: list[tuple[Path, str]] = []

    def promote(path: Path, commit_sha: str) -> None:
        calls.append("parent")
        if fail_parent:
            raise OSError("injected parent promotion failure")
        promotions.append((path, commit_sha))

    executor = ChangeSetExecutor(
        crate=crate,
        registry=registry,
        applier=ChangeSetApplier(crate, tmp_path / "journal"),
        state=state,
        audit=AuditWriter(tmp_path / "audit"),
        candidate_binary=candidate,
        build_gate=_gate("build", calls, ok=failed_gate != "build"),
        w1_gate=_gate("w1", calls, ok=failed_gate != "w1"),
        w2_gate=_gate(
            "w2",
            calls,
            ok=failed_gate not in {"w2_no_gain", "w2_regress", "w2_unmeasurable"},
            reason={
                "w2_no_gain": "no_gain",
                "w2_regress": "per_op_regress",
                "w2_unmeasurable": "unmeasurable",
            }.get(failed_gate, "accepted"),
        ),
        parent_promoter=promote,
    )
    return executor, source, state, promotions, calls, tmp_path / "audit"


@pytest.mark.parametrize(
    ("failed_gate", "expected_status"),
    [
        ("build", ChangeSetStatus.REJECTED_BUILD),
        ("post", ChangeSetStatus.REJECTED_POST_VALIDATION),
        ("w1", ChangeSetStatus.REJECTED_W1),
        ("w2_no_gain", ChangeSetStatus.REJECTED_W2_NO_GAIN),
        ("w2_regress", ChangeSetStatus.REJECTED_W2_REGRESS),
        ("w2_unmeasurable", ChangeSetStatus.REJECTED_UNMEASURABLE),
    ],
)
def test_executor_restores_before_commit_failures(
    tmp_path: Path, failed_gate: str, expected_status: ChangeSetStatus
) -> None:
    executor, source, state, promotions, _, _ = _executor(
        tmp_path, failed_gate=failed_gate
    )

    result = executor.execute(_proposal())

    assert result.terminal_status is expected_status
    assert source.read_text() == "static A: u32 = 1;\n"
    assert state.commits == []
    assert promotions == []
    assert result.rollback_verified


def test_executor_success_order_commits_then_promotes_parent(tmp_path: Path) -> None:
    executor, source, state, promotions, calls, audit = _executor(tmp_path)

    result = executor.execute(_proposal())

    assert result.terminal_status is ChangeSetStatus.COMMITTED
    assert result.commit_sha == "commit-1"
    assert source.read_text() == "const A: u32 = 1;\n"
    assert len(state.commits) == 1
    assert promotions == [(tmp_path / "candidate-bin", "commit-1")]
    assert calls == ["resolve", "pre", "build", "post", "w1", "w2", "parent"]
    persisted = json.loads((audit / "changesets" / "cs-1" / "result.json").read_text())
    assert persisted["terminal_status"] == "committed"
    assert persisted["applied_rules"] == ["C1"]
    operation = persisted["resolved"]["proposal"]["operations"][0]
    resolution = persisted["resolved"]["resolutions"][0]
    assert operation["evidence_hit_ids"] == ["hit-1"]
    assert resolution["operation_id"] == operation["operation_id"]
    assert [edit["operation_id"] for edit in resolution["edits"]] == [
        operation["operation_id"]
    ]
    assert persisted["touched_paths"] == ["lib.rs"]


def test_commit_failure_restores_source(tmp_path: Path) -> None:
    executor, source, state, promotions, _, _ = _executor(
        tmp_path, fail_commit=True
    )

    result = executor.execute(_proposal())

    assert result.terminal_status is ChangeSetStatus.FAILED_INTERNAL
    assert source.read_text() == "static A: u32 = 1;\n"
    assert len(state.commits) == 1
    assert promotions == []
    assert result.rollback_verified


def test_parent_failure_after_commit_never_restores_committed_source(tmp_path: Path) -> None:
    executor, source, state, promotions, _, _ = _executor(
        tmp_path, fail_parent=True
    )

    with pytest.raises(ChangeSetInfrastructureError, match="parent promotion") as exc:
        executor.execute(_proposal())

    assert exc.value.commit_sha == "commit-1"
    assert source.read_text() == "const A: u32 = 1;\n"
    assert len(state.commits) == 1
    assert promotions == []


def test_parent_failure_can_be_recovered_from_committed_candidate(tmp_path: Path) -> None:
    executor, source, state, promotions, calls, audit = _executor(
        tmp_path, fail_parent=True
    )
    with pytest.raises(ChangeSetInfrastructureError) as exc:
        executor.execute(_proposal())
    assert exc.value.commit_sha == "commit-1"

    recovered = []
    executor.parent_promoter = lambda path, sha: recovered.append((path, sha))
    executor.recover_after_commit("cs-1", "commit-1")

    assert recovered == [(tmp_path / "candidate-bin", "commit-1")]
    events = [
        json.loads(line)
        for line in (audit / "changesets" / "cs-1" / "events.jsonl").read_text().splitlines()
    ]
    assert events[-1]["status"] == "recovered_after_commit"
    assert source.read_text() == "const A: u32 = 1;\n"


def test_recovery_validates_commit_before_parent_side_effect(tmp_path: Path) -> None:
    executor, _, _, _, _, _ = _executor(tmp_path, fail_parent=True)
    with pytest.raises(ChangeSetInfrastructureError):
        executor.execute(_proposal())
    recovered = []
    executor.parent_promoter = lambda path, sha: recovered.append((path, sha))

    with pytest.raises(ChangeSetInfrastructureError, match="commit mismatch"):
        executor.recover_after_commit("cs-1", "wrong-commit")

    assert recovered == []


def test_recovery_rejects_candidate_binary_changed_after_commit(tmp_path: Path) -> None:
    executor, _, _, _, _, _ = _executor(tmp_path, fail_parent=True)
    with pytest.raises(ChangeSetInfrastructureError):
        executor.execute(_proposal())
    executor.candidate_binary.write_bytes(b"overwritten-candidate")
    recovered = []
    executor.parent_promoter = lambda path, sha: recovered.append((path, sha))

    with pytest.raises(ChangeSetInfrastructureError, match="binary hash mismatch"):
        executor.recover_after_commit("cs-1", "commit-1")

    assert recovered == []


def test_audit_failure_after_commit_is_infrastructure_error(tmp_path: Path) -> None:
    class FailFirstResultAudit(AuditWriter):
        failed = False

        def write_result(self, result):
            if result.commit_sha is not None and not self.failed:
                self.failed = True
                raise OSError("injected committed result write failure")
            return super().write_result(result)

    executor, source, state, promotions, _, _ = _executor(tmp_path)
    executor.audit = FailFirstResultAudit(tmp_path / "audit-after-commit")

    with pytest.raises(ChangeSetInfrastructureError) as exc:
        executor.execute(_proposal())

    assert exc.value.commit_sha == "commit-1"
    assert source.read_text() == "const A: u32 = 1;\n"
    assert promotions == [(tmp_path / "candidate-bin", "commit-1")]


def _git(repo: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", *args], cwd=repo, text=True, capture_output=True, check=True
    )
    return result.stdout.strip()


def test_state_manager_commits_exact_paths_and_leaves_other_diff(tmp_path: Path) -> None:
    repo = tmp_path / "repo"
    repo.mkdir()
    _git(repo, "init")
    _git(repo, "config", "user.email", "test@example.com")
    _git(repo, "config", "user.name", "Test User")
    a = repo / "a.txt"
    b = repo / "b.txt"
    a.write_text("a0\n")
    b.write_text("b0\n")
    _git(repo, "add", "a.txt", "b.txt")
    _git(repo, "commit", "-m", "initial")
    a.write_text("a1\n")
    b.write_text("b1\n")
    state = StateManager(repo, tmp_path / "audit.jsonl")

    state.commit_success([a], "change a")

    assert _git(repo, "show", "--name-only", "--format=", "HEAD") == "a.txt"
    assert _git(repo, "status", "--short") == "M b.txt"
