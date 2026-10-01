"""Deterministic planner for proven-safe II_const declaration targets."""

from __future__ import annotations

import hashlib
from dataclasses import dataclass
from typing import Any

from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    PromoteStaticToConst,
    ProposedChangeSet,
    SymbolKind,
    SymbolRef,
    TriggerContext,
)


@dataclass(frozen=True)
class IIConstPlan:
    proposal: ProposedChangeSet | None
    abstain_reason: str | None = None


def _field(hit: Any, name: str, default=None):
    return hit.get(name, default) if isinstance(hit, dict) else getattr(hit, name, default)


def plan_ii_const(
    *, hot_function: str, hits: list[Any], base_head: str, candidate_id: str
) -> IIConstPlan:
    targets: dict[tuple[str, str], tuple[dict, str]] = {}
    required = {
        "symbol_name", "qualified_name", "symbol_kind", "decl_file",
        "declaration_hash",
    }
    for hit in hits:
        if _field(hit, "rule") != "II_const":
            continue
        extra = _field(hit, "extra", {}) or {}
        if not required.issubset(extra) or extra["symbol_kind"] != "static":
            return IIConstPlan(None, "incomplete_target_evidence")
        hit_id = str(_field(hit, "id", _field(hit, "hit_id", "")))
        targets[(extra["qualified_name"], extra["declaration_hash"])] = (extra, hit_id)
    if not targets:
        return IIConstPlan(None, "no_ii_const_targets")

    operations = []
    hit_ids = []
    for (qualified_name, declaration_hash), (extra, hit_id) in sorted(targets.items()):
        seed = f"{candidate_id}|{qualified_name}|{declaration_hash}"
        operation_id = "promote-" + hashlib.sha256(seed.encode()).hexdigest()[:16]
        evidence = (hit_id,) if hit_id else ()
        hit_ids.extend(evidence)
        operations.append(PromoteStaticToConst(
            operation_id=operation_id,
            rule_id="II_const",
            evidence_hit_ids=evidence,
            target=SymbolRef(
                qualified_name=qualified_name,
                symbol_kind=SymbolKind.STATIC,
                file_hint=extra["decl_file"],
                declaration_hash=declaration_hash,
            ),
        ))
    changeset_seed = candidate_id + "|" + "|".join(op.operation_id for op in operations)
    proposal = ProposedChangeSet(
        changeset_id="ii-const-" + hashlib.sha256(changeset_seed.encode()).hexdigest()[:16],
        base_head=base_head,
        trigger=TriggerContext(
            rule_id="II_const", candidate_id=candidate_id,
            hot_function=hot_function, hit_ids=tuple(hit_ids),
            evidence_artifacts=(),
        ),
        operations=tuple(operations),
        impact_scope=ImpactScope.CRATE_GLOBAL,
    )
    return IIConstPlan(proposal)
