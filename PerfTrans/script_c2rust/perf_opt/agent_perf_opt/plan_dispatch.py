"""PLAN transformations applied by the deterministic dispatcher."""

from copy import deepcopy
from typing import Any


def build_effective_plan(
    original_plan: dict[str, Any],
    applied_rules: list[dict[str, Any]],
    dispatcher_abstained: list[dict[str, Any]],
) -> dict[str, Any]:
    """Return the PLAN authorized for EXECUTE without mutating inputs."""
    effective = deepcopy(original_plan)
    effective["applied_rules"] = deepcopy(applied_rules)
    effective["abstained_rules"] = [
        *deepcopy(original_plan.get("abstained_rules", [])),
        *deepcopy(dispatcher_abstained),
    ]
    return effective
