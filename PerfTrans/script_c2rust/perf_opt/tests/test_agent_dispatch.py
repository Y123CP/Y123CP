import importlib
import sys
import types
from copy import deepcopy
from pathlib import Path
from types import SimpleNamespace

import pytest

from perf_opt.agent_perf_opt.changeset.handlers.replace_function import (
    ReplaceFunctionHandler,
)
from perf_opt.agent_perf_opt.changeset.types import ReplaceFunctionBody
from perf_opt.agent_perf_opt.planners.llm_function import (
    plan_llm_function_change,
)
from perf_opt.agent_perf_opt.rewrite_applier import EditTarget
from perf_opt.agent_perf_opt.state import AttemptRecord, RewriteAttempt


def _target(crate: Path) -> EditTarget:
    source = crate / "src" / "lib.rs"
    source.parent.mkdir(parents=True)
    content = b"pub fn hot(x: i32) -> i32 { x }\n"
    source.write_bytes(content)
    return EditTarget(source, "hot", (0, len(content) - 1))


def test_llm_adapter_creates_single_typed_operation(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)
    response = """Here is the rewrite:
```rust
// Applied rules: [C1]
pub fn hot(x: i32) -> i32 { x + 1 }
```
"""

    planned = plan_llm_function_change(
        response=response,
        edit_target=edit_target,
        rule_ids=("C1",),
        hit_ids=("hit-1",),
        base_head="head-1",
        candidate_id="hot-1",
        crate=crate,
    )

    assert planned.abstain_reason is None
    assert planned.proposal is not None
    assert len(planned.proposal.operations) == 1
    operation = planned.proposal.operations[0]
    assert isinstance(operation, ReplaceFunctionBody)
    assert operation.rule_id == "C1"
    assert operation.target.qualified_name == "crate::hot"
    assert operation.evidence_hit_ids == ("hit-1",)


def test_llm_adapter_preserves_explicit_abstain(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)

    planned = plan_llm_function_change(
        response="abstain: proof is insufficient",
        edit_target=edit_target,
        rule_ids=("C1",),
        hit_ids=(),
        base_head="head-1",
        candidate_id="hot-1",
        crate=crate,
    )

    assert planned.proposal is None
    assert planned.abstain_reason == "proof is insufficient"


def test_llm_adapter_rejects_extra_top_level_function(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)
    response = """```rust
pub fn hot(x: i32) -> i32 { x + 1 }
pub fn surprise() {}
```"""

    planned = plan_llm_function_change(
        response=response,
        edit_target=edit_target,
        rule_ids=("C1",),
        hit_ids=(),
        base_head="head-1",
        candidate_id="hot-1",
        crate=crate,
    )

    assert planned.proposal is None
    # The reason leads with the CODE and keeps the detail behind it. `detail or
    # code` used to drop the code whenever a detail existed, which left callers
    # a free-text string they could not classify — so a repairable envelope
    # failure looked exactly like the model's own judgment, and the retry loop
    # had to treat both as final.
    assert planned.abstain_reason.startswith("replacement_function_count")
    assert "replacement_must_contain_exactly_one_function" in planned.abstain_reason

    from perf_opt.agent_perf_opt.planners.llm_region import is_contract_repairable
    assert is_contract_repairable(planned.abstain_reason)


def test_replace_function_handler_resolves_only_original_span(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)
    planned = plan_llm_function_change(
        response="""```rust
pub fn hot(x: i32) -> i32 { x + 1 }
```""",
        edit_target=edit_target,
        rule_ids=("C1",),
        hit_ids=("hit-1",),
        base_head="head-1",
        candidate_id="hot-1",
        crate=crate,
    )
    assert planned.proposal is not None
    operation = planned.proposal.operations[0]
    assert isinstance(operation, ReplaceFunctionBody)
    handler = ReplaceFunctionHandler(edit_target)

    resolution = handler.resolve(operation, crate)

    assert len(resolution.edits) == 1
    edit = resolution.edits[0]
    assert edit.relative_path == "src/lib.rs"
    assert (edit.start_byte, edit.end_byte) == edit_target.span
    assert edit.replacement_text == operation.replacement_function_source
    assert handler.pre_validate(operation, resolution, crate).ok


def test_replace_function_handler_rejects_stale_target(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)
    planned = plan_llm_function_change(
        response="""```rust
pub fn hot(x: i32) -> i32 { x + 1 }
```""",
        edit_target=edit_target,
        rule_ids=("C1",),
        hit_ids=(),
        base_head="head-1",
        candidate_id="hot-1",
        crate=crate,
    )
    assert planned.proposal is not None
    operation = planned.proposal.operations[0]
    assert isinstance(operation, ReplaceFunctionBody)
    edit_target.file.write_text("pub fn hot(x: i32) -> i32 { x + 2 }\n")

    handler = ReplaceFunctionHandler(edit_target)
    try:
        handler.resolve(operation, crate)
    except ValueError as exc:
        assert "stale function target" in str(exc)
    else:
        raise AssertionError("stale target was accepted")


def test_replace_function_post_validation_rejects_duplicate_target(
    tmp_path: Path,
) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)
    planned = plan_llm_function_change(
        response="""```rust
pub fn hot(x: i32) -> i32 { x + 1 }
```""",
        edit_target=edit_target,
        rule_ids=("C1",),
        hit_ids=(),
        base_head="head-1",
        candidate_id="hot-1",
        crate=crate,
    )
    assert planned.proposal is not None
    operation = planned.proposal.operations[0]
    assert isinstance(operation, ReplaceFunctionBody)
    handler = ReplaceFunctionHandler(edit_target)
    resolution = handler.resolve(operation, crate)
    edit = resolution.edits[0]
    source = edit_target.file.read_bytes()
    replacement = operation.replacement_function_source.encode()
    edit_target.file.write_bytes(
        source[: edit.start_byte]
        + replacement
        + source[edit.end_byte :]
        + b"\npub fn hot() {}\n"
    )

    validation = handler.post_validate(operation, resolution, crate)

    assert not validation.ok
    assert validation.code == "replacement_function_ambiguous"


def test_apply_and_gate_uses_changeset_executor_not_legacy_apply(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    llm_module = types.ModuleType("utils.llm_client")
    llm_module.LLMClient = object
    caller_module = types.ModuleType("perf_opt.agent_perf_opt.caller_lookup")
    caller_module.CallerLookup = object
    monkeypatch.setitem(sys.modules, "utils.llm_client", llm_module)
    monkeypatch.setitem(
        sys.modules, "perf_opt.agent_perf_opt.caller_lookup", caller_module
    )
    sys.modules.pop("perf_opt.agent_perf_opt.agent", None)
    agent = importlib.import_module("perf_opt.agent_perf_opt.agent")

    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)
    harness = tmp_path / "harness"
    post_bin = harness / "target" / "release" / "harness"
    post_bin.parent.mkdir(parents=True)
    post_bin.write_bytes(b"candidate")

    class State:
        def __init__(self) -> None:
            self.crate = crate
            self.audit_log_path = tmp_path / "rewrites.log"

        def head_sha(self, *, full: bool = True) -> str:
            return "head-1"

        def commit_success(self, paths, message: str) -> str:
            return "commit-1"

    class W2:
        def __init__(self) -> None:
            self.promoted = []
            self.high_risk_ops = []

        def gate_ops(
            self,
            post_bin,
            ops,
            op_weights=None,
            high_risk_ops=None,
        ):
            self.high_risk_ops.append(list(high_risk_ops or ()))
            verdict = SimpleNamespace(
                ok=True,
                reason="accepted",
                delta_pct=-1.0,
                measured_cv=0.1,
            )
            return verdict, "op"

        def promote_candidate(self, post_bin, commit_sha):
            self.promoted.append((post_bin, commit_sha))

    def legacy_apply_must_not_run(*args, **kwargs):
        raise AssertionError("legacy apply_rewrite was called")

    monkeypatch.setattr(agent, "apply_rewrite", legacy_apply_must_not_run)
    monkeypatch.setattr(agent, "cargo_check", lambda *args, **kwargs: (True, ""), raising=False)
    w1_samples = []

    def full_w1(*args, **kwargs):
        w1_samples.append(kwargs["sample_per_op"])
        return SimpleNamespace(passed=True, reason="")

    monkeypatch.setattr(agent, "w1_gate", full_w1)
    monkeypatch.setattr(agent, "_w2_ops_for_fn", lambda *args: ["op"])
    cfg = SimpleNamespace(
        cargo_build_timeout_s=120,
        w1_sample_per_op=99,
        w2_scope="all_ops",
    )
    hf = SimpleNamespace(
        name="hot", hottest_op="op", per_op={"op": 1.0}
    )
    response = """```rust
// Applied rules: [C1]
pub fn hot(x: i32) -> i32 { x + 1 }
```"""

    w2 = W2()
    status, extra = agent._apply_and_gate(
        edit_target,
        response,
        ["C1"],
        hf,
        harness,
        post_bin,
        object(),
        object(),
        w2,
        State(),
        cfg,
    )

    assert status is agent.RewriteAttempt.APPLIED_COMMITTED
    assert extra["commit_sha"] == "commit-1"
    assert w2.promoted == [(post_bin, "commit-1")]
    assert w2.high_risk_ops == [["op"]]
    assert w1_samples == [0]
    assert edit_target.file.read_text().startswith("// Applied rules: [C1]")


def test_agent_result_does_not_sum_marginal_w2_percentages(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    llm_module = types.ModuleType("utils.llm_client")
    llm_module.LLMClient = object
    caller_module = types.ModuleType("perf_opt.agent_perf_opt.caller_lookup")
    caller_module.CallerLookup = object
    monkeypatch.setitem(sys.modules, "utils.llm_client", llm_module)
    monkeypatch.setitem(
        sys.modules, "perf_opt.agent_perf_opt.caller_lookup", caller_module
    )
    sys.modules.pop("perf_opt.agent_perf_opt.agent", None)
    agent = importlib.import_module("perf_opt.agent_perf_opt.agent")
    result = agent.AgentResult()

    for index, delta in enumerate((-4.0, -5.0), start=1):
        result.add(
            AttemptRecord(
                fn_name=f"hot_{index}",
                rule_id="C1",
                round_no=1,
                attempt_no=1,
                status=RewriteAttempt.APPLIED_COMMITTED,
                w2_delta_pct=delta,
                commit_sha=f"commit-{index}",
            )
        )

    assert result.total_wall_gain_pct is None
    assert [item["delta_pct"] for item in result.marginal_results] == [-4.0, -5.0]
    result.final_total_gain_pct = -8.0
    assert result.total_wall_gain_pct == -8.0
    assert result.total_wall_gain_pct != -9.0


def test_pristine_preflight_always_runs_full_w1(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    llm_module = types.ModuleType("utils.llm_client")
    llm_module.LLMClient = object
    caller_module = types.ModuleType("perf_opt.agent_perf_opt.caller_lookup")
    caller_module.CallerLookup = object
    monkeypatch.setitem(sys.modules, "utils.llm_client", llm_module)
    monkeypatch.setitem(
        sys.modules, "perf_opt.agent_perf_opt.caller_lookup", caller_module
    )
    sys.modules.pop("perf_opt.agent_perf_opt.agent", None)
    agent = importlib.import_module("perf_opt.agent_perf_opt.agent")
    calls = []
    monkeypatch.setattr(
        agent,
        "w1_gate",
        lambda *args, **kwargs: (
            calls.append(kwargs["sample_per_op"])
            or SimpleNamespace(passed=True, reason="")
        ),
    )

    agent._run_pristine_w1_preflight(object(), object())

    assert calls == [0]
    monkeypatch.setattr(
        agent,
        "w1_gate",
        lambda *args, **kwargs: SimpleNamespace(
            passed=False, reason="golden mismatch"
        ),
    )
    with pytest.raises(agent.AgentError, match="pristine full W1 preflight failed"):
        agent._run_pristine_w1_preflight(object(), object())


def test_finalize_w2_uses_skip_record_without_measuring(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    llm_module = types.ModuleType("utils.llm_client")
    llm_module.LLMClient = object
    caller_module = types.ModuleType("perf_opt.agent_perf_opt.caller_lookup")
    caller_module.CallerLookup = object
    monkeypatch.setitem(sys.modules, "utils.llm_client", llm_module)
    monkeypatch.setitem(
        sys.modules, "perf_opt.agent_perf_opt.caller_lookup", caller_module
    )
    sys.modules.pop("perf_opt.agent_perf_opt.agent", None)
    agent = importlib.import_module("perf_opt.agent_perf_opt.agent")
    skipped = {
        "status": "skipped",
        "reason": "no_accepted_commits",
        "committed_attempts": 0,
        "binaries_identical": True,
    }

    class W2:
        parent_bin = Path("parent")

        def maybe_skip_final(self, committed_attempts):
            assert committed_attempts == 0
            return skipped

        def measure_final(self, final_binary, ops):
            raise AssertionError("final measurement must be skipped")

    result = agent.AgentResult()

    agent._finalize_w2(result, W2(), ["op"])

    assert result.final_measurement == skipped
    assert result.final_total_gain_pct is None


def _load_agent_for_plan_tests(monkeypatch: pytest.MonkeyPatch):
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


def _run_plan_execute(
    agent,
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    plan: dict,
    *,
    execute_must_not_run: bool = False,
    caller_kinds: list[str] | None = None,
):
    crate = tmp_path / "crate"
    crate.mkdir()
    edit_target = _target(crate)
    captured = {}

    monkeypatch.setattr(agent, "build_plan_prompt", lambda *args, **kwargs: ("plan-sys", "plan-user"))
    monkeypatch.setattr(
        agent,
        "parse_plan_json",
        lambda response: (deepcopy(plan), None),
    )

    if execute_must_not_run:
        def build_execute_must_not_run(*args, **kwargs):
            raise AssertionError("EXECUTE must not run when dispatcher filters all rules")

        monkeypatch.setattr(agent, "build_execute_prompt", build_execute_must_not_run)
        monkeypatch.setattr(
            agent,
            "_apply_and_gate",
            lambda *args, **kwargs: (_ for _ in ()).throw(
                AssertionError("gate must not run when dispatcher filters all rules")
            ),
        )
        responses = iter(["plan-response"])
    else:
        def capture_execute_prompt(
            hf, ep, target, applied_rules, plan_json, hits, cfg, **kwargs
        ):
            captured["applied_rules"] = applied_rules
            captured["plan"] = deepcopy(plan_json)
            return "exec-sys", "exec-user"

        monkeypatch.setattr(agent, "build_execute_prompt", capture_execute_prompt)
        monkeypatch.setattr(
            agent,
            "_apply_and_gate",
            lambda *args, **kwargs: (
                agent.RewriteAttempt.ABSTAINED,
                {"terminal_status": "abstained"},
            ),
        )
        responses = iter([
            "plan-response",
            "// Applied rules: [III③]\npub fn hot(x: i32) -> i32 { x }",
        ])

    llm = SimpleNamespace(chat=lambda *args, **kwargs: next(responses))
    if caller_kinds is None:
        caller_kinds = ["rust_same_crate"] * 4
    callers = [SimpleNamespace(kind=kind) for kind in caller_kinds]
    caller_lookup = SimpleNamespace(callers_of=lambda fn_name: callers)

    result = agent._try_fn_plan_execute(
        SimpleNamespace(name="hot"),
        object(),
        edit_target,
        ["III①", "III③"],
        [{"id": 0}, {"id": 1}],
        llm=llm,
        state=object(),
        w2_session=object(),
        verifier=object(),
        assets=object(),
        harness_dir=tmp_path / "harness",
        crate=crate,
        cfg=SimpleNamespace(max_attempts_per_fn=1),
        caller_lookup=caller_lookup,
    )
    return result, captured


def test_plan_execute_uses_dispatcher_filtered_plan(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _load_agent_for_plan_tests(monkeypatch)
    original = {
        "applied_rules": [
            {"rule": "III①", "target_hits": [0], "needs_cross_fn": True},
            {"rule": "III③", "target_hits": [1], "needs_cross_fn": False},
        ],
        "abstained_rules": [],
    }

    result, captured = _run_plan_execute(agent, tmp_path, monkeypatch, original)

    assert captured["applied_rules"] == ["III③"]
    assert [item["rule"] for item in captured["plan"]["applied_rules"]] == [
        "III③"
    ]
    rejected = captured["plan"]["abstained_rules"]
    assert [item["rule"] for item in rejected] == ["III①"]
    assert rejected[0]["target_hits"] == [0]
    assert rejected[0]["caller_count"] == 4
    assert rejected[0]["caller_kinds"] == ["rust_same_crate"]
    assert result.plan_json == captured["plan"]
    assert result.original_plan_json == original


def test_all_plan_rules_filtered_records_both_plans_without_execute(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _load_agent_for_plan_tests(monkeypatch)
    original = {
        "applied_rules": [
            {"rule": "III①", "target_hits": [0], "needs_cross_fn": True},
        ],
        "abstained_rules": [],
    }

    result, _ = _run_plan_execute(
        agent,
        tmp_path,
        monkeypatch,
        original,
        execute_must_not_run=True,
    )

    assert result.status is agent.RewriteAttempt.ABSTAINED
    assert result.reason == "all_rules_abstained_by_dispatch"
    assert result.plan_json["applied_rules"] == []
    assert result.plan_json["abstained_rules"][0]["rule"] == "III①"
    assert result.original_plan_json == original


def test_plan_execute_freezes_original_before_f6_target_normalization(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _load_agent_for_plan_tests(monkeypatch)
    original = {
        "applied_rules": [
            {
                "rule": "III③",
                "target_hits": [0, 99, "not-an-index"],
                "needs_cross_fn": False,
            },
        ],
        "abstained_rules": [],
    }

    result, captured = _run_plan_execute(agent, tmp_path, monkeypatch, original)

    assert result.original_plan_json == original
    assert result.original_plan_json["applied_rules"][0]["target_hits"] == [
        0,
        99,
        "not-an-index",
    ]
    assert captured["plan"]["applied_rules"][0]["target_hits"] == [0]
    assert result.plan_json == captured["plan"]


def test_f6_abstain_appears_only_in_effective_plan(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _load_agent_for_plan_tests(monkeypatch)
    original = {
        "applied_rules": [
            {
                "rule": "III③",
                "target_hits": ["not-an-index"],
                "needs_cross_fn": False,
            },
        ],
        "abstained_rules": [],
    }

    result, _ = _run_plan_execute(
        agent,
        tmp_path,
        monkeypatch,
        original,
        execute_must_not_run=True,
    )

    assert result.original_plan_json == original
    assert result.plan_json["applied_rules"] == []
    assert result.plan_json["abstained_rules"][0]["rule"] == "III③"
    assert result.plan_json["abstained_rules"][0]["reason"].startswith("F6:")


def test_cross_fn_boundary_abstain_preserves_rule_and_caller_evidence(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _load_agent_for_plan_tests(monkeypatch)
    original = {
        "applied_rules": [
            {
                "rule": "III①",
                "target_hits": [0],
                "needs_cross_fn": True,
                "strategy": "change signature",
            },
        ],
        "abstained_rules": [],
    }

    result, _ = _run_plan_execute(
        agent,
        tmp_path,
        monkeypatch,
        original,
        execute_must_not_run=True,
        caller_kinds=["extern_c_facing", "cross_crate"],
    )

    rejected = result.plan_json["abstained_rules"][0]
    assert rejected["rule"] == "III①"
    assert rejected["target_hits"] == [0]
    assert rejected["needs_cross_fn"] is True
    assert rejected["strategy"] == "change signature"
    assert rejected["reason"] == "cross_fn_boundary"
    assert rejected["caller_count"] == 2
    assert rejected["caller_kinds"] == ["cross_crate", "extern_c_facing"]
