import importlib
import json
import logging
import sys
import types
from copy import deepcopy
from pathlib import Path
from types import SimpleNamespace

import pytest

from perf_opt.agent_perf_opt.changeset.types import (
    AppliedChangeSet,
    ChangeSetStatus,
    ReplaceFunctionBody,
    SymbolKind,
    SymbolRef,
)
from perf_opt.agent_perf_opt.state import AttemptRecord, RewriteAttempt, StateManager


def _import_agent(monkeypatch: pytest.MonkeyPatch):
    llm_module = types.ModuleType("utils.llm_client")
    llm_module.LLMClient = object
    caller_module = types.ModuleType("perf_opt.agent_perf_opt.caller_lookup")
    caller_module.CallerLookup = object
    monkeypatch.setitem(sys.modules, "utils.llm_client", llm_module)
    monkeypatch.setitem(
        sys.modules, "perf_opt.agent_perf_opt.caller_lookup", caller_module
    )
    sys.modules.pop("perf_opt.agent_perf_opt.agent", None)
    return importlib.import_module("perf_opt.agent_perf_opt.agent")


def _stub_executor_result(
    agent,
    monkeypatch: pytest.MonkeyPatch,
    terminal_status,
) -> None:
    applied = AppliedChangeSet(
        resolved=None,
        terminal_status=terminal_status,
        commit_sha="commit-sha",
    )

    class FakeApplier:
        def recover_incomplete(self) -> None:
            pass

    class FakeExecutor:
        def __init__(self, **kwargs) -> None:
            pass

        def execute(self, proposal):
            return applied

    monkeypatch.setattr(agent, "ChangeSetApplier", lambda *args: FakeApplier())
    monkeypatch.setattr(agent, "ChangeSetExecutor", FakeExecutor)


@pytest.mark.parametrize(
    ("changeset_status_name", "expected_attempt"),
    [
        ("COMMITTED", RewriteAttempt.APPLIED_COMMITTED),
        ("REJECTED_BUILD", RewriteAttempt.SYNTAX_ERROR),
        ("REJECTED_W1", RewriteAttempt.W1_FAIL),
        ("REJECTED_W2_NO_GAIN", RewriteAttempt.W2_REGRESS),
        ("REJECTED_W2_REGRESS", RewriteAttempt.W2_REGRESS),
        ("REJECTED_UNMEASURABLE", RewriteAttempt.W2_REGRESS),
        ("REJECTED_STALE", RewriteAttempt.ABSTAINED),
        ("REJECTED_CONFLICT", RewriteAttempt.ABSTAINED),
        ("REJECTED_POST_VALIDATION", RewriteAttempt.ABSTAINED),
        ("ABSTAINED_UNPROVEN", RewriteAttempt.ABSTAINED),
        ("FAILED_INTERNAL", RewriteAttempt.SYNTAX_ERROR),
    ],
)
def test_changeset_status_mapping_preserves_compatibility_and_terminal_status(
    monkeypatch: pytest.MonkeyPatch,
    changeset_status_name: str,
    expected_attempt: RewriteAttempt,
) -> None:
    agent = _import_agent(monkeypatch)
    changeset_status = getattr(agent.ChangeSetStatus, changeset_status_name)

    attempt, terminal_status = agent._classify_changeset_terminal_status(
        changeset_status
    )

    assert attempt is expected_attempt
    assert terminal_status == changeset_status.value


def test_changeset_status_mapping_explicitly_covers_every_enum_member(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    agent = _import_agent(monkeypatch)
    compatibility_statuses = {
        agent.ChangeSetStatus.COMMITTED,
        agent.ChangeSetStatus.REJECTED_BUILD,
        agent.ChangeSetStatus.REJECTED_W1,
        agent.ChangeSetStatus.REJECTED_W2_NO_GAIN,
        agent.ChangeSetStatus.REJECTED_W2_REGRESS,
        agent.ChangeSetStatus.REJECTED_UNMEASURABLE,
        agent.ChangeSetStatus.REJECTED_STALE,
        agent.ChangeSetStatus.REJECTED_CONFLICT,
        agent.ChangeSetStatus.REJECTED_POST_VALIDATION,
        agent.ChangeSetStatus.ABSTAINED_UNPROVEN,
        agent.ChangeSetStatus.FAILED_INTERNAL,
    }

    for changeset_status in set(agent.ChangeSetStatus) - compatibility_statuses:
        attempt, terminal_status = agent._classify_changeset_terminal_status(
            changeset_status
        )
        assert attempt is RewriteAttempt.ABSTAINED
        assert terminal_status == changeset_status.value


@pytest.mark.parametrize("unknown_status", ["committed", object(), None])
def test_changeset_status_mapping_rejects_non_enum_values(
    monkeypatch: pytest.MonkeyPatch,
    unknown_status: object,
) -> None:
    agent = _import_agent(monkeypatch)

    with pytest.raises(ValueError, match="ChangeSetStatus"):
        agent._classify_changeset_terminal_status(unknown_status)


_EXECUTOR_TERMINAL_CASES = [
    ("COMMITTED", RewriteAttempt.APPLIED_COMMITTED),
    ("REJECTED_BUILD", RewriteAttempt.SYNTAX_ERROR),
    ("REJECTED_W1", RewriteAttempt.W1_FAIL),
    ("REJECTED_W2_NO_GAIN", RewriteAttempt.W2_REGRESS),
    ("REJECTED_STALE", RewriteAttempt.ABSTAINED),
    ("FAILED_INTERNAL", RewriteAttempt.SYNTAX_ERROR),
]


@pytest.mark.parametrize(
    ("changeset_status_name", "expected_attempt"),
    _EXECUTOR_TERMINAL_CASES,
)
def test_function_executor_propagates_terminal_status_at_call_site(
    tmp_path,
    monkeypatch: pytest.MonkeyPatch,
    changeset_status_name: str,
    expected_attempt: RewriteAttempt,
) -> None:
    agent = _import_agent(monkeypatch)
    changeset_status = getattr(agent.ChangeSetStatus, changeset_status_name)
    _stub_executor_result(agent, monkeypatch, changeset_status)
    operation = ReplaceFunctionBody(
        operation_id="op-1",
        rule_id="C1",
        evidence_hit_ids=(),
        target=SymbolRef(
            qualified_name="hot",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint=None,
            declaration_hash="hash-1",
        ),
        replacement_function_source="pub fn hot() {}",
    )
    monkeypatch.setattr(
        agent,
        "plan_llm_function_change",
        lambda **kwargs: SimpleNamespace(
            proposal=SimpleNamespace(operations=(operation,)),
            abstain_reason=None,
        ),
    )
    state = SimpleNamespace(
        crate=tmp_path / "crate",
        audit_log_path=tmp_path / "audit" / "rewrites.jsonl",
        head_sha=lambda **kwargs: "head-1",
    )
    w2_session = SimpleNamespace(promote_candidate=lambda *args: None)

    attempt, extra = agent._apply_and_gate(
        object(),
        "",
        ["C1"],
        SimpleNamespace(name="hot"),
        tmp_path / "harness",
        tmp_path / "candidate",
        object(),
        object(),
        w2_session,
        state,
        object(),
    )

    assert attempt is expected_attempt
    assert extra["terminal_status"] == changeset_status.value


def test_function_planner_abstain_has_stable_terminal_status(
    tmp_path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    monkeypatch.setattr(
        agent,
        "plan_llm_function_change",
        lambda **kwargs: SimpleNamespace(
            proposal=None, abstain_reason="planner detail may change"
        ),
    )
    state = SimpleNamespace(
        crate=tmp_path / "crate",
        head_sha=lambda **kwargs: "head-1",
    )

    attempt, extra = agent._apply_and_gate(
        object(),
        "",
        ["C1"],
        SimpleNamespace(name="hot"),
        tmp_path / "harness",
        tmp_path / "candidate",
        object(),
        object(),
        object(),
        state,
        object(),
    )

    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    assert attempt is RewriteAttempt.ABSTAINED
    assert extra["reason"] == "planner detail may change"
    assert extra["terminal_status"] == reporting.PLANNER_ABSTAINED


def test_attempt_record_accepts_legacy_fields_only() -> None:
    record = AttemptRecord(
        fn_name="hot_fn",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
    )

    assert record.terminal_status is None
    assert record.attempt_trace is None


_RESULT_COUNT_FIELDS = (
    "committed_attempts",
    "abstained_count",
    "regressed_count",
    "syntax_failed_count",
    "w1_failed_count",
    "rejected_build_count",
    "w2_no_gain_count",
    "w2_regress_count",
    "unmeasurable_count",
)


@pytest.mark.parametrize(
    ("terminal_status", "status", "expected_counts"),
    [
        (ChangeSetStatus.COMMITTED.value, RewriteAttempt.APPLIED_COMMITTED,
         {"committed_attempts": 1}),
        (ChangeSetStatus.ABSTAINED_UNPROVEN.value, RewriteAttempt.ABSTAINED,
         {"abstained_count": 1}),
        (ChangeSetStatus.REJECTED_BUILD.value, RewriteAttempt.SYNTAX_ERROR,
         {"syntax_failed_count": 1, "rejected_build_count": 1}),
        (ChangeSetStatus.REJECTED_W1.value, RewriteAttempt.W1_FAIL,
         {"w1_failed_count": 1}),
        (ChangeSetStatus.REJECTED_W2_NO_GAIN.value, RewriteAttempt.W2_REGRESS,
         {"w2_no_gain_count": 1}),
        (ChangeSetStatus.REJECTED_W2_REGRESS.value, RewriteAttempt.W2_REGRESS,
         {"regressed_count": 1, "w2_regress_count": 1}),
        (ChangeSetStatus.REJECTED_UNMEASURABLE.value, RewriteAttempt.W2_REGRESS,
         {"unmeasurable_count": 1}),
        (ChangeSetStatus.FAILED_INTERNAL.value, RewriteAttempt.SYNTAX_ERROR,
         {"syntax_failed_count": 1}),
    ],
)
def test_agent_result_uses_exact_terminal_status_for_precise_counts(
    monkeypatch: pytest.MonkeyPatch,
    terminal_status: str,
    status: RewriteAttempt,
    expected_counts: dict[str, int],
) -> None:
    agent = _import_agent(monkeypatch)
    result = agent.AgentResult()
    record = AttemptRecord(
        fn_name="hot",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=status,
        w2_delta_pct=-2.0 if status is RewriteAttempt.APPLIED_COMMITTED else None,
        commit_sha=(
            "commit-sha" if status is RewriteAttempt.APPLIED_COMMITTED else None
        ),
        # These deliberately misleading fields prove aggregation does not parse text.
        error="rejected_w2_regress rejected_build",
        reason="rejected_w2_regress rejected_build",
        terminal_status=terminal_status,
    )

    result.add(record)

    assert result.total_attempts == 1
    assert result.per_fn_summary == {"hot": [record]}
    for field_name in _RESULT_COUNT_FIELDS:
        assert getattr(result, field_name) == expected_counts.get(field_name, 0)
    assert result.marginal_results == (
        [{
            "fn_name": "hot",
            "rule_id": "C1",
            "delta_pct": -2.0,
            "commit_sha": "commit-sha",
        }]
        if terminal_status == ChangeSetStatus.COMMITTED.value
        else []
    )


@pytest.mark.parametrize(
    ("terminal_status", "status", "expected_field"),
    [
        (None, RewriteAttempt.APPLIED_COMMITTED, "committed_attempts"),
        (None, RewriteAttempt.ABSTAINED, "abstained_count"),
        (None, RewriteAttempt.W2_REGRESS, "regressed_count"),
        (None, RewriteAttempt.SYNTAX_ERROR, "syntax_failed_count"),
        (None, RewriteAttempt.W1_FAIL, "w1_failed_count"),
        (None, RewriteAttempt.BUDGET_EXCEEDED, None),
        (RewriteAttempt.W2_REGRESS.value,
         RewriteAttempt.W2_REGRESS, "regressed_count"),
    ],
)
def test_agent_result_legacy_records_keep_coarse_fallback_without_exact_counts(
    monkeypatch: pytest.MonkeyPatch,
    terminal_status: str | None,
    status: RewriteAttempt,
    expected_field: str | None,
) -> None:
    agent = _import_agent(monkeypatch)
    result = agent.AgentResult()
    record = AttemptRecord(
        fn_name="legacy",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=status,
        terminal_status=terminal_status,
    )

    result.add(record)

    for field_name in _RESULT_COUNT_FIELDS:
        expected = 1 if field_name == expected_field else 0
        assert getattr(result, field_name) == expected
    assert result.total_attempts == 1
    assert result.per_fn_summary == {"legacy": [record]}


@pytest.mark.parametrize(
    "terminal_status",
    ["rejected_w2_regres", "unknown_terminal_status"],
)
def test_agent_result_rejects_unknown_terminal_before_any_aggregation(
    monkeypatch: pytest.MonkeyPatch,
    terminal_status: str,
) -> None:
    agent = _import_agent(monkeypatch)
    result = agent.AgentResult()
    record = AttemptRecord(
        fn_name="hot",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.W2_REGRESS,
        tokens_in=11,
        tokens_out=7,
        terminal_status=terminal_status,
    )

    with pytest.raises(ValueError, match="terminal_status"):
        result.add(record)

    assert result.total_attempts == 0
    assert result.llm_tokens_used_estimated == 0
    assert result.per_fn_summary == {}
    assert result.per_rule_gain == {}
    assert result.marginal_results == []
    for field_name in _RESULT_COUNT_FIELDS:
        assert getattr(result, field_name) == 0


def test_attempt_record_accepts_valid_trace() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [
        reporting.make_trace_entry(
            attempt_no=1,
            phase="direct",
            rewrite_status=RewriteAttempt.ABSTAINED,
            terminal_status="planner_abstained",
        )
    ]

    record = AttemptRecord(
        fn_name="hot_fn",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
        terminal_status="planner_abstained",
        attempt_trace=trace,
    )

    assert record.terminal_status == "planner_abstained"
    assert record.attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "direct",
            "rewrite_status": "abstained",
            "terminal_status": "planner_abstained",
            "error": None,
        }
    ]
    assert json.loads(json.dumps(trace)) == trace


@pytest.mark.parametrize(
    "override",
    [
        {"attempt_no": 0},
        {"attempt_no": True},
        {"phase": "unknown"},
        {"phase": []},
        {"rewrite_status": ""},
        {"rewrite_status": object()},
        {"rewrite_status": SimpleNamespace(value=7)},
        {"terminal_status": ""},
        {"terminal_status": object()},
        {"error": 7},
    ],
)
def test_make_trace_entry_rejects_invalid_data(override: dict) -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    arguments = {
        "attempt_no": 1,
        "phase": "direct",
        "rewrite_status": "abstained",
        "terminal_status": "planner_abstained",
        "error": None,
    }
    arguments.update(override)

    with pytest.raises(ValueError):
        reporting.make_trace_entry(**arguments)


def test_attempt_record_rejects_trace_terminal_status_mismatch() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [
        reporting.make_trace_entry(
            attempt_no=1,
            phase="plan",
            rewrite_status="abstained",
            terminal_status="planner_abstained",
        )
    ]

    with pytest.raises(ValueError, match="terminal_status"):
        AttemptRecord(
            fn_name="hot_fn",
            rule_id="C1",
            round_no=1,
            attempt_no=1,
            status=RewriteAttempt.ABSTAINED,
            terminal_status="plan_parse_fail",
            attempt_trace=trace,
        )


def test_attempt_record_defensively_copies_attempt_trace() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [
        reporting.make_trace_entry(
            attempt_no=1,
            phase="direct",
            rewrite_status="abstained",
            terminal_status="planner_abstained",
        )
    ]
    record = AttemptRecord(
        fn_name="hot_fn",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
        terminal_status="planner_abstained",
        attempt_trace=trace,
    )

    trace[0]["terminal_status"] = "tampered"
    trace.append(_trace_entry(attempt_no=2))

    assert record.attempt_trace is not trace
    assert record.attempt_trace == [{
        "attempt_no": 1,
        "phase": "direct",
        "rewrite_status": "abstained",
        "terminal_status": "planner_abstained",
        "error": None,
    }]


def test_attempt_record_defensively_copies_anchor_hit_ids() -> None:
    anchors = ["hit-1"]
    record = AttemptRecord(
        fn_name="hot_fn",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
        anchor_hit_ids=anchors,
    )

    anchors[0] = "tampered"
    anchors.append("hit-2")

    assert record.anchor_hit_ids is not anchors
    assert record.anchor_hit_ids == ["hit-1"]


def test_state_log_revalidates_mutated_attempt_trace(tmp_path) -> None:
    record = AttemptRecord(
        fn_name="hot_fn",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
        terminal_status="planner_abstained",
        attempt_trace=[_trace_entry()],
    )
    assert record.attempt_trace is not None
    record.attempt_trace[0]["attempt_no"] = 2
    audit_log = tmp_path / "audit" / "rewrites.jsonl"
    state = StateManager(tmp_path / "crate", audit_log)

    with pytest.raises(ValueError, match="attempt_no"):
        state.log(record)

    assert not audit_log.exists()


def test_state_log_writes_terminal_status_and_attempt_trace_as_jsonl(
    tmp_path: Path,
) -> None:
    trace = [{
        "attempt_no": 1,
        "phase": "typed",
        "rewrite_status": "syntax_error",
        "terminal_status": "rejected_build",
        "error": "cargo failed: nested detail",
    }]
    record = AttemptRecord(
        fn_name="hot_fn",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.SYNTAX_ERROR,
        terminal_status="rejected_build",
        attempt_trace=trace,
    )
    audit_log = tmp_path / "audit" / "rewrites.jsonl"
    state = StateManager(tmp_path / "crate", audit_log)

    state.log(record)

    payload = json.loads(audit_log.read_text(encoding="utf-8"))
    assert payload["status"] == "syntax_error"
    assert payload["terminal_status"] == "rejected_build"
    assert payload["attempt_trace"] == trace


def test_state_log_writes_legacy_record_with_null_reporting_fields(
    tmp_path: Path,
) -> None:
    record = AttemptRecord(
        fn_name="legacy_fn",
        rule_id="C1",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
    )
    audit_log = tmp_path / "audit" / "rewrites.jsonl"
    state = StateManager(tmp_path / "crate", audit_log)

    state.log(record)

    payload = json.loads(audit_log.read_text(encoding="utf-8"))
    assert payload["status"] == "abstained"
    assert payload["terminal_status"] is None
    assert payload["attempt_trace"] is None


def test_agent_only_terminal_statuses_are_stable() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")

    assert reporting.AGENT_ONLY_TERMINAL_STATUSES == frozenset(
        {
            "plan_parse_fail",
            "planner_abstained",
            # Our checker refusing the reply, kept apart from the model
            # declining the rewrite — see reporting.REJECTED_FORM.
            "rejected_form",
            "all_rules_abstained_by_dispatch",
            "budget_exceeded",
            "pre_abstain",
        }
    )


def test_validate_attempt_trace_requires_error_key() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [{
        "attempt_no": 1,
        "phase": "direct",
        "rewrite_status": "abstained",
        "terminal_status": "planner_abstained",
    }]

    with pytest.raises(ValueError, match="error"):
        reporting.validate_attempt_trace("planner_abstained", trace)


def _trace_entry(**overrides: object) -> dict:
    entry = {
        "attempt_no": 1,
        "phase": "direct",
        "rewrite_status": "abstained",
        "terminal_status": "planner_abstained",
        "error": None,
    }
    entry.update(overrides)
    return entry


def test_validate_attempt_trace_accepts_contiguous_attempts() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [
        _trace_entry(attempt_no=attempt_no)
        for attempt_no in [1, 2, 3]
    ]

    reporting.validate_attempt_trace("planner_abstained", trace)


@pytest.mark.parametrize(
    "attempt_numbers",
    [[2], [1, 1], [2, 1], [1, 3], [1, 3, 2], [1, 2, 4]],
)
def test_validate_attempt_trace_requires_contiguous_attempt_sequence(
    attempt_numbers: list[int],
) -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [
        _trace_entry(attempt_no=attempt_no)
        for attempt_no in attempt_numbers
    ]

    with pytest.raises(ValueError, match="attempt_no.*contiguous"):
        reporting.validate_attempt_trace("planner_abstained", trace)


def test_with_attempt_trace_is_pure_and_replaces_report_fields() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [
        reporting.make_trace_entry(
            1, "direct", "syntax_error", "rejected_build", "cargo failed"
        ),
        reporting.make_trace_entry(
            2, "direct", "applied_committed", "committed"
        ),
    ]
    extra = {
        "terminal_status": "committed",
        "commit_sha": "abc123",
    }
    extra_snapshot = deepcopy(extra)
    trace_snapshot = deepcopy(trace)

    fields = reporting.with_attempt_trace(extra, trace)

    assert extra == extra_snapshot
    assert trace == trace_snapshot
    assert fields == {
        "terminal_status": "committed",
        "commit_sha": "abc123",
        "attempt_trace": trace,
    }
    assert fields["attempt_trace"] is not trace


def test_with_attempt_trace_rejects_terminal_status_conflict() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [reporting.make_trace_entry(
        1, "direct", "applied_committed", "committed"
    )]

    with pytest.raises(ValueError, match="terminal_status conflicts"):
        reporting.with_attempt_trace(
            {"terminal_status": "rejected_build"}, trace
        )


def test_with_attempt_trace_rejects_attempt_trace_conflict() -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")
    trace = [reporting.make_trace_entry(
        1, "direct", "applied_committed", "committed"
    )]
    conflicting_trace = [reporting.make_trace_entry(
        1, "direct", "syntax_error", "rejected_build", "build failed"
    )]

    with pytest.raises(ValueError, match="attempt_trace conflicts"):
        reporting.with_attempt_trace(
            {"attempt_trace": conflicting_trace}, trace
        )


def _run_direct_trace(
    agent,
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    outcomes: list[tuple[RewriteAttempt, dict]],
):
    monkeypatch.setattr(
        agent, "build_multi_card_prompt", lambda *args, **kwargs: ("sys", "user")
    )
    outcome_iter = iter(outcomes)
    monkeypatch.setattr(agent, "_apply_and_gate", lambda *args: next(outcome_iter))
    monkeypatch.setattr(agent, "parse_applied_rules", lambda response: ["C1"])
    monkeypatch.setattr(agent, "parse_skipped_rules", lambda response: [])
    responses = iter(["response"] * len(outcomes))
    return agent._try_fn_direct(
        SimpleNamespace(name="hot"),
        object(),
        SimpleNamespace(file=tmp_path / "lib.rs"),
        ["C1"],
        [{"rule": "C1"}],
        llm=SimpleNamespace(chat=lambda *args, **kwargs: next(responses)),
        state=object(),
        w2_session=object(),
        verifier=object(),
        assets=object(),
        harness_dir=tmp_path / "harness",
        crate=tmp_path / "crate",
        cfg=SimpleNamespace(max_attempts_per_fn=len(outcomes)),
    )


def test_direct_retry_records_one_candidate_with_complete_trace(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)

    record = _run_direct_trace(
        agent,
        tmp_path,
        monkeypatch,
        [
            (
                RewriteAttempt.SYNTAX_ERROR,
                {
                    "terminal_status": "rejected_build",
                    "error": "short build error",
                    "cargo_stderr_full": "full build error",
                },
            ),
            (
                RewriteAttempt.APPLIED_COMMITTED,
                {
                    "terminal_status": "committed",
                    "commit_sha": "abc123",
                    "executed_applied_rules": ["C1"],
                },
            ),
        ],
    )

    assert record.terminal_status == "committed"
    assert record.attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "direct",
            "rewrite_status": "syntax_error",
            "terminal_status": "rejected_build",
            "error": "short build error",
        },
        {
            "attempt_no": 2,
            "phase": "direct",
            "rewrite_status": "applied_committed",
            "terminal_status": "committed",
            "error": None,
        },
    ]


def test_direct_non_retry_records_single_trace_entry(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)

    record = _run_direct_trace(
        agent,
        tmp_path,
        monkeypatch,
        [
            (
                RewriteAttempt.W2_REGRESS,
                {"terminal_status": "rejected_w2_no_gain", "error": "no gain"},
            )
        ],
    )

    assert record.terminal_status == "rejected_w2_no_gain"
    assert record.attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "direct",
            "rewrite_status": "w2_regress",
            "terminal_status": "rejected_w2_no_gain",
            "error": "no gain",
        }
    ]


def test_direct_trace_requires_structured_terminal_status(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)

    with pytest.raises(KeyError, match="terminal_status"):
        _run_direct_trace(
            agent,
            tmp_path,
            monkeypatch,
            [(RewriteAttempt.ABSTAINED, {"reason": "planner detail"})],
        )


def _run_plan_trace(
    agent,
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    plan_result,
    execute_outcomes: list[tuple[RewriteAttempt, dict]] | None = None,
):
    monkeypatch.setattr(
        agent, "build_plan_prompt", lambda *args, **kwargs: ("plan-sys", "plan-user")
    )
    monkeypatch.setattr(agent, "parse_plan_json", lambda response: plan_result)
    responses = ["plan-response"]
    if execute_outcomes:
        monkeypatch.setattr(
            agent,
            "build_execute_prompt",
            lambda *args, **kwargs: ("exec-sys", "exec-user"),
        )
        outcomes = iter(execute_outcomes)
        monkeypatch.setattr(agent, "_apply_and_gate", lambda *args: next(outcomes))
        monkeypatch.setattr(agent, "parse_applied_rules", lambda response: ["C1"])
        monkeypatch.setattr(agent, "parse_skipped_rules", lambda response: [])
        responses.extend(["execute-response"] * len(execute_outcomes))
    response_iter = iter(responses)
    return agent._try_fn_plan_execute(
        SimpleNamespace(name="hot"),
        object(),
        SimpleNamespace(file=tmp_path / "lib.rs"),
        ["C1"],
        [{"rule": "C1"}],
        llm=SimpleNamespace(chat=lambda *args, **kwargs: next(response_iter)),
        state=object(),
        w2_session=object(),
        verifier=object(),
        assets=object(),
        harness_dir=tmp_path / "harness",
        crate=tmp_path / "crate",
        cfg=SimpleNamespace(
            max_attempts_per_fn=(len(execute_outcomes) if execute_outcomes else 1)
        ),
        caller_lookup=SimpleNamespace(callers_of=lambda fn_name: []),
    )


def test_plan_parse_failure_records_plan_trace(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)

    record = _run_plan_trace(
        agent, tmp_path, monkeypatch, (None, "invalid plan json")
    )

    assert record.terminal_status == "plan_parse_fail"
    assert record.attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "plan",
            "rewrite_status": "abstained",
            "terminal_status": "plan_parse_fail",
            "error": "invalid plan json",
        }
    ]


def test_plan_dispatch_abstain_records_single_plan_trace(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    plan = {
        "applied_rules": [
            {"rule": "C1", "target_hits": [0], "needs_cross_fn": True}
        ],
        "abstained_rules": [],
    }
    caller = SimpleNamespace(kind="rust_same_crate")
    monkeypatch.setattr(
        agent, "build_plan_prompt", lambda *args, **kwargs: ("plan-sys", "plan-user")
    )
    monkeypatch.setattr(agent, "parse_plan_json", lambda response: (plan, None))

    record = agent._try_fn_plan_execute(
        SimpleNamespace(name="hot"),
        object(),
        SimpleNamespace(file=tmp_path / "lib.rs"),
        ["C1"],
        [{"rule": "C1"}],
        llm=SimpleNamespace(chat=lambda *args, **kwargs: "plan-response"),
        state=object(),
        w2_session=object(),
        verifier=object(),
        assets=object(),
        harness_dir=tmp_path / "harness",
        crate=tmp_path / "crate",
        cfg=SimpleNamespace(max_attempts_per_fn=1),
        caller_lookup=SimpleNamespace(callers_of=lambda fn_name: [caller]),
    )

    assert record.terminal_status == "all_rules_abstained_by_dispatch"
    assert record.attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "plan",
            "rewrite_status": "abstained",
            "terminal_status": "all_rules_abstained_by_dispatch",
            "error": None,
        }
    ]


def test_execute_retry_trace_starts_at_one_without_plan_success_entry(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    plan = {
        "applied_rules": [
            {"rule": "C1", "target_hits": [0], "needs_cross_fn": False}
        ],
        "abstained_rules": [],
    }

    record = _run_plan_trace(
        agent,
        tmp_path,
        monkeypatch,
        (plan, None),
        [
            (
                RewriteAttempt.SYNTAX_ERROR,
                {
                    "terminal_status": "rejected_build",
                    "error": "build failed",
                    "cargo_stderr_full": "build failed full",
                },
            ),
            (
                RewriteAttempt.APPLIED_COMMITTED,
                {
                    "terminal_status": "committed",
                    "commit_sha": "abc123",
                    "executed_applied_rules": ["C1"],
                },
            ),
        ],
    )

    assert record.terminal_status == "committed"
    assert [entry["attempt_no"] for entry in record.attempt_trace] == [1, 2]
    assert [entry["phase"] for entry in record.attempt_trace] == [
        "execute",
        "execute",
    ]
    assert [entry["terminal_status"] for entry in record.attempt_trace] == [
        "rejected_build",
        "committed",
    ]


def test_execute_trace_requires_structured_terminal_status(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    plan = {
        "applied_rules": [
            {"rule": "C1", "target_hits": [0], "needs_cross_fn": False}
        ],
        "abstained_rules": [],
    }

    with pytest.raises(KeyError, match="terminal_status"):
        _run_plan_trace(
            agent,
            tmp_path,
            monkeypatch,
            (plan, None),
            [(RewriteAttempt.ABSTAINED, {"reason": "planner detail"})],
        )


def test_driver_agent_done_log_reports_precise_counts(
    caplog: pytest.LogCaptureFixture,
) -> None:
    driver = importlib.import_module("perf_opt.agent_perf_opt.driver")
    caplog.set_level(logging.INFO, logger=driver.logger.name)
    result = SimpleNamespace(
        committed_attempts=1,
        abstained_count=2,
        form_rejected_count=11,
        regressed_count=9,
        rejected_build_count=3,
        w1_failed_count=4,
        w2_no_gain_count=5,
        w2_regress_count=6,
        unmeasurable_count=7,
        syntax_failed_count=8,
        total_wall_gain_pct=-1.25,
    )

    driver._log_agent_done(result)

    assert (
        "commit=1, abstain=2, form_rejected=11, build_rejected=3, "
        "w1_rejected=4, "
        "w2_no_gain=5, w2_regress=6, unmeasurable=7"
    ) in caplog.text
    assert "regress_compat=9" in caplog.text
    assert "syntax_or_internal=8" in caplog.text
    assert "direct total wall=-1.25%" in caplog.text

    caplog.clear()
    result.total_wall_gain_pct = None
    driver._log_agent_done(result)

    assert "direct total wall=unmeasured" in caplog.text
    assert "unmeasured%" not in caplog.text












@pytest.mark.parametrize(
    ("terminal_status", "attempt_trace"),
    [
        ("planner_abstained", []),
        ("planner_abstained", (_trace_entry(),)),
        ("planner_abstained", [42]),
        ("planner_abstained", [{}]),
        ("planner_abstained", [{
            "phase": "direct",
            "rewrite_status": "abstained",
            "terminal_status": "planner_abstained",
            "error": None,
        }]),
        ("planner_abstained", [{
            "attempt_no": 1,
            "rewrite_status": "abstained",
            "terminal_status": "planner_abstained",
            "error": None,
        }]),
        ("planner_abstained", [{
            "attempt_no": 1,
            "phase": "direct",
            "terminal_status": "planner_abstained",
            "error": None,
        }]),
        ("planner_abstained", [{
            "attempt_no": 1,
            "phase": "direct",
            "rewrite_status": "abstained",
            "error": None,
        }]),
        ("planner_abstained", [_trace_entry(attempt_no=True)]),
        ("planner_abstained", [_trace_entry(attempt_no=0)]),
        ("planner_abstained", [_trace_entry(attempt_no=-1)]),
        ("planner_abstained", [_trace_entry(attempt_no=1.0)]),
        ("planner_abstained", [_trace_entry(attempt_no="1")]),
        ("planner_abstained", [_trace_entry(phase="unknown")]),
        ("planner_abstained", [_trace_entry(phase=[])]),
        ("planner_abstained", [_trace_entry(rewrite_status="")]),
        ("planner_abstained", [_trace_entry(rewrite_status=None)]),
        ("", [_trace_entry(terminal_status="")]),
        (None, [_trace_entry(terminal_status=None)]),
        ("planner_abstained", [_trace_entry(error=7)]),
    ],
)
def test_validate_attempt_trace_rejects_invalid_data(
    terminal_status: object,
    attempt_trace: object,
) -> None:
    reporting = importlib.import_module("perf_opt.agent_perf_opt.reporting")

    with pytest.raises(ValueError):
        reporting.validate_attempt_trace(terminal_status, attempt_trace)


def test_reconcile_fired_rules_does_not_flag_covered_sub_strategies() -> None:
    # Regression: a rule with sub-strategies (C3.S1 sig-lift / C3.S2 local
    # hoist) is FIRED under its base name "C3" in the plan, but EXECUTE reports
    # the sub-names — "C3.S2" applied, "C3.S1" skipped with a reason. That is
    # correct, complete handling of C3; the reconciler must NOT mis-flag "C3"
    # as a silently-dropped contract violation just because the base name is
    # absent from the sub-name sets.
    from perf_opt.agent_perf_opt.agent import _reconcile_fired_rules

    def dropped(res):
        return [r for r, why in res if why == "silently_dropped"]

    covered = _reconcile_fired_rules(
        fired=["C3", "III①", "III②", "III③"],
        applied=["C3.S2", "III②", "III③"],
        skipped=[("III①", "caller churn"), ("C3.S1", "sig lift needs callers")],
    )
    assert dropped(covered) == []

    # A genuinely dropped rule (no sub-strategy applied or skipped) is still
    # caught — the fix must not weaken real contract-violation detection.
    real = _reconcile_fired_rules(fired=["C3", "III②"], applied=["III②"], skipped=[])
    assert dropped(real) == ["C3"]

    # A rule with no sub-strategy that is genuinely dropped is still caught.
    assert dropped(
        _reconcile_fired_rules(fired=["III④"], applied=[], skipped=[])
    ) == ["III④"]


def test_iii4_fingerprint_recognizes_split_at_and_chunks() -> None:
    # Regression (2026-08-13, libopenaptx aptx_reconstructed_differences_update):
    # III④ turns a `*mut T` + `.offset(n)` cursor into two slices via
    # `split_at_mut(n)`. The exec fingerprint used to omit split_at/chunks, so a
    # genuine III④ rewrite was flagged as "declared without fingerprint" and
    # stripped — under-counting III④ in the rule ledger. split_at/chunks/windows
    # only ever appear once a raw pointer has been turned into a slice (raw
    # c2rust output uses `.offset`), so recognizing them cannot mislabel.
    from perf_opt.agent_perf_opt.rewrite_applier import find_undeclared_executions

    split_at_body = (
        "let (rd1, rd2) = reconstructed_differences.split_at_mut(order as usize);\n"
        "rd1[p as usize] = rd2[p as usize];\n"
    )
    assert find_undeclared_executions(["III④"], split_at_body) == []

    chunks_body = "for w in buf.chunks_exact(4) { acc += w[0] as i64; }"
    assert find_undeclared_executions(["III④"], chunks_body) == []

    # A bogus III④ claim that only reborrows `&*p` but keeps `.offset()` cursor
    # access (no slice/iter introduced) must still be flagged as undeclared.
    reborrow_only = (
        "let tables_ref: &aptx_tables = &*tables;\n"
        "qr = *tables_ref.quantize_intervals.offset(idx as isize) / 2;\n"
    )
    assert find_undeclared_executions(["III④"], reborrow_only) == ["III④"]
