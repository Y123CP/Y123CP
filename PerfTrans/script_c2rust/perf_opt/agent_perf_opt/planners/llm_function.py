"""Adapt a legacy fenced LLM function rewrite into a typed ChangeSet."""

from __future__ import annotations

import hashlib
from dataclasses import dataclass
from pathlib import Path

from perf_opt.agent_perf_opt.changeset.handlers.replace_function import (
    validate_single_function_source,
)
from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ProposedChangeSet,
    ReplaceFunctionBody,
    SymbolKind,
    SymbolRef,
    TriggerContext,
)
from perf_opt.agent_perf_opt.rewrite_applier import (
    EditTarget,
    parse_applied_rules,
    parse_llm_response,
)


@dataclass(frozen=True)
class LLMFunctionPlan:
    proposal: ProposedChangeSet | None
    abstain_reason: str | None = None


def plan_llm_function_change(
    *,
    response: str,
    edit_target: EditTarget,
    rule_ids: tuple[str, ...],
    hit_ids: tuple[str, ...],
    base_head: str,
    candidate_id: str,
    crate: Path,
) -> LLMFunctionPlan:
    code, abstain_reason = parse_llm_response(response)
    if abstain_reason is not None or code is None:
        return LLMFunctionPlan(None, abstain_reason or "parse_fail")

    validation = validate_single_function_source(code, edit_target.fn_name)
    if not validation.ok:
        # Lead with the CODE — see ValidationResult.as_reason for why
        # `detail or code` silently defeats every downstream classifier.
        return LLMFunctionPlan(None, validation.as_reason())

    target_path = edit_target.file.resolve()
    relative_path = str(target_path.relative_to(crate.resolve()))
    data = target_path.read_bytes()
    start, end = edit_target.span
    if not (0 <= start < end <= len(data)):
        return LLMFunctionPlan(None, "function_target_span_out_of_bounds")
    span_hash = hashlib.sha256(data[start:end]).hexdigest()

    declared = tuple(
        rule for rule in parse_applied_rules(code) if rule in set(rule_ids)
    )
    effective_rules = declared or rule_ids
    rule_id = ",".join(effective_rules) if effective_rules else "legacy_function"
    operation_seed = f"{candidate_id}|{relative_path}|{span_hash}|{rule_id}"
    operation_id = "replace-" + hashlib.sha256(operation_seed.encode()).hexdigest()[:16]
    changeset_seed = f"{candidate_id}|{operation_id}|{hashlib.sha256(code.encode()).hexdigest()}"
    changeset_id = "legacy-" + hashlib.sha256(changeset_seed.encode()).hexdigest()[:16]

    operation = ReplaceFunctionBody(
        operation_id=operation_id,
        rule_id=rule_id,
        evidence_hit_ids=hit_ids,
        target=SymbolRef(
            qualified_name=f"crate::{edit_target.fn_name}",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint=relative_path,
            declaration_hash=span_hash,
        ),
        replacement_function_source=code,
    )
    trigger = TriggerContext(
        rule_id=rule_id,
        candidate_id=candidate_id,
        hot_function=edit_target.fn_name,
        hit_ids=hit_ids,
        evidence_artifacts=(),
    )
    proposal = ProposedChangeSet(
        changeset_id=changeset_id,
        base_head=base_head,
        trigger=trigger,
        operations=(operation,),
        impact_scope=ImpactScope.LOCAL_FUNCTION,
    )
    return LLMFunctionPlan(proposal)
