from dataclasses import FrozenInstanceError

import pytest

from perf_opt.agent_perf_opt.changeset.registry import HandlerRegistry
from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ProposedChangeSet,
    ReplaceFunctionBody,
    SymbolKind,
    SymbolRef,
    TriggerContext,
)


def _operation(op_id: str = "op-1") -> ReplaceFunctionBody:
    return ReplaceFunctionBody(
        operation_id=op_id,
        rule_id="C1",
        evidence_hit_ids=("hit-1",),
        target=SymbolRef(
            qualified_name="crate::xxhash::round32",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint="src/xxhash.rs",
            declaration_hash="a" * 64,
        ),
        replacement_function_source="fn round32() {}\n",
    )


def test_proposal_is_frozen_and_rejects_duplicate_operation_ids() -> None:
    trigger = TriggerContext(
        rule_id="C1",
        candidate_id="cand-1",
        hot_function="round32",
        hit_ids=("hit-1",),
        evidence_artifacts=(),
    )
    with pytest.raises(ValueError, match="duplicate operation_id"):
        ProposedChangeSet(
            changeset_id="cs-1",
            base_head="head",
            trigger=trigger,
            operations=(_operation(), _operation()),
            impact_scope=ImpactScope.LOCAL_FUNCTION,
        )

    proposal = ProposedChangeSet(
        changeset_id="cs-2",
        base_head="head",
        trigger=trigger,
        operations=(_operation(),),
        impact_scope=ImpactScope.LOCAL_FUNCTION,
    )
    with pytest.raises(FrozenInstanceError):
        proposal.base_head = "other"  # type: ignore[misc]


def test_proposal_rejects_empty_operation_list() -> None:
    trigger = TriggerContext(
        rule_id="C1",
        candidate_id="cand-1",
        hot_function="round32",
        hit_ids=("hit-1",),
        evidence_artifacts=(),
    )
    with pytest.raises(ValueError, match="at least one operation"):
        ProposedChangeSet(
            changeset_id="cs-empty",
            base_head="head",
            trigger=trigger,
            operations=(),
            impact_scope=ImpactScope.LOCAL_FUNCTION,
        )


def test_registry_rejects_unregistered_and_duplicate_handlers() -> None:
    registry = HandlerRegistry()
    operation = _operation()

    with pytest.raises(KeyError, match="ReplaceFunctionBody"):
        registry.handler_for(operation)

    handler = object()
    registry.register(ReplaceFunctionBody, handler)
    assert registry.handler_for(operation) is handler

    with pytest.raises(ValueError, match="already registered"):
        registry.register(ReplaceFunctionBody, object())
