from copy import deepcopy
import json

import pytest

from perf_opt.agent_perf_opt.plan_dispatch import build_effective_plan
from perf_opt.agent_perf_opt.prompt_builder import (
    _validate_execute_plan_consistency,
    build_execute_prompt,
)
from perf_opt.agent_perf_opt.state import AttemptRecord, RewriteAttempt, StateManager


def test_build_effective_plan_filters_rules_without_mutating_original() -> None:
    original = {
        "applied_rules": [
            {
                "rule": "III①",
                "target_hits": ["callsite-a"],
                "needs_cross_fn": True,
            },
            {
                "rule": "III③",
                "target_hits": ["loop-a"],
                "needs_cross_fn": False,
            },
        ],
        "abstained_rules": [
            {"rule": "II_vec", "reason": "not applicable"}
        ],
    }
    snapshot = deepcopy(original)
    rejected = [
        {
            "rule": "III①",
            "target_hits": ["callsite-a"],
            "needs_cross_fn": True,
            "reason": "cross-function caller scope unsupported",
            "caller_count": 4,
            "caller_kinds": ["rust_same_crate"],
        }
    ]

    effective = build_effective_plan(
        original,
        applied_rules=[original["applied_rules"][1]],
        dispatcher_abstained=rejected,
    )

    assert original == snapshot
    assert [item["rule"] for item in effective["applied_rules"]] == ["III③"]
    assert [item["rule"] for item in effective["abstained_rules"]] == [
        "II_vec",
        "III①",
    ]
    assert effective["abstained_rules"][1]["target_hits"] == ["callsite-a"]


def test_attempt_record_accepts_original_and_effective_plans() -> None:
    original = {"applied_rules": [{"rule": "III①"}]}
    effective = {"applied_rules": [], "abstained_rules": [{"rule": "III①"}]}

    record = AttemptRecord(
        fn_name="hot",
        rule_id="III①",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
        original_plan_json=original,
        plan_json=effective,
    )

    assert record.original_plan_json == original
    assert record.plan_json == effective


def test_validate_execute_plan_consistency_rejects_rule_mismatch() -> None:
    plan = {"applied_rules": [{"rule": "III③"}]}

    with pytest.raises(ValueError, match="effective PLAN"):
        _validate_execute_plan_consistency(["III①", "III③"], plan)


def test_validate_execute_plan_consistency_requires_exact_order() -> None:
    plan = {
        "applied_rules": [{"rule": "III①"}, {"rule": "III③"}],
    }

    with pytest.raises(ValueError, match="effective PLAN"):
        _validate_execute_plan_consistency(["III③", "III①"], plan)


def test_validate_execute_plan_consistency_accepts_exact_match() -> None:
    plan = {
        "applied_rules": [{"rule": "III①"}, {"rule": "III③"}],
    }

    _validate_execute_plan_consistency(["III①", "III③"], plan)


@pytest.mark.parametrize(
    "applied_rule_ids,effective_plan",
    [
        (["III③"], {"applied_rules": ["III①", {"rule": "III③"}]}),
        (["III③"], {"applied_rules": None}),
        (["III③"], {"applied_rules": [{"rule": ""}]}),
        (["III③"], {"applied_rules": [{"rule": 3}]}),
        ("III③", {"applied_rules": [{"rule": "III③"}]}),
    ],
)
def test_validate_execute_plan_consistency_rejects_malformed_entries(
    applied_rule_ids, effective_plan
) -> None:
    with pytest.raises(ValueError, match="effective PLAN"):
        _validate_execute_plan_consistency(applied_rule_ids, effective_plan)


def test_build_execute_prompt_calls_plan_consistency_validator() -> None:
    with pytest.raises(ValueError, match="effective PLAN"):
        build_execute_prompt(
            object(),
            object(),
            object(),
            ["III①"],
            {"applied_rules": [{"rule": "III③"}]},
            [],
            object(),
        )


def test_state_log_serializes_original_and_effective_plans(tmp_path) -> None:
    original = {"applied_rules": [{"rule": "III①"}]}
    effective = {
        "applied_rules": [],
        "abstained_rules": [{"rule": "III①", "reason": "dispatcher"}],
    }
    audit_log = tmp_path / "audit" / "rewrites.jsonl"
    state = StateManager(tmp_path / "crate", audit_log)
    record = AttemptRecord(
        fn_name="hot",
        rule_id="III①",
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
        original_plan_json=original,
        plan_json=effective,
    )

    state.log(record)

    payload = json.loads(audit_log.read_text(encoding="utf-8"))
    assert payload["original_plan_json"] == original
    assert payload["plan_json"] == effective
