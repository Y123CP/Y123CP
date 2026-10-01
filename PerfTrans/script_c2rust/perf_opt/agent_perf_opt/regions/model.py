"""Immutable source-region references and region-routing primitives."""

from __future__ import annotations

from collections.abc import Mapping
from enum import Enum
import hashlib
import json
from typing import Any

from ..changeset.types import RegionKind, RegionRef


class RuleCapability(str, Enum):
    REGION_LOCAL = "region_local"
    CROSS_FUNCTION = "cross_function"
    UNSUPPORTED = "unsupported"


_RULE_CAPABILITIES: dict[str, RuleCapability] = {
    "C1": RuleCapability.REGION_LOCAL,
    "C2": RuleCapability.REGION_LOCAL,
    "C3": RuleCapability.REGION_LOCAL,
    "C4": RuleCapability.REGION_LOCAL,
    "C5": RuleCapability.REGION_LOCAL,
    "C6": RuleCapability.REGION_LOCAL,
    "C7": RuleCapability.REGION_LOCAL,
    "C8": RuleCapability.REGION_LOCAL,
    # A fixed-length sub-word loop is entirely inside one loop nest —
    # nothing about folding it reaches past the region it sits in.
    "C10": RuleCapability.REGION_LOCAL,
    "II_vec": RuleCapability.REGION_LOCAL,
    # The declaration and every read of an over-zeroed local are in one body:
    # §2.1 of its card shrinks the init extent in place and hands the rest of
    # the function a slice. §2.2 would change the filler too, and the card
    # sends that case back to §2.1 rather than across a function boundary.
    "C12": RuleCapability.REGION_LOCAL,
    # Its card rules out every cross-function shape itself ("the allocation
    # escapes the function", "alloc and free are in different functions" ->
    # abstain), so what remains is a body-local malloc/free -> Vec rewrite.
    # Unregistered, `rule_capability` answered UNSUPPORTED and the extractor
    # dropped every hit before it could become a candidate: 38 hits on one
    # compression crate, 16 and 9 on two others, all silently discarded while
    # the detector, the card and the reporting layer all knew the rule.
    "III②": RuleCapability.REGION_LOCAL,
    "III③": RuleCapability.REGION_LOCAL,
    "III④": RuleCapability.REGION_LOCAL,
    # II_inl is not routed: its edit is one attribute on the CALLEE, which no
    # LLM path can make (every reply is locked to the hot function). It runs
    # as a deterministic planner instead — planners/inline_attr.py.
    "III①": RuleCapability.CROSS_FUNCTION,
}


def rule_capability(rule_id: str) -> RuleCapability:
    """Return the explicitly declared execution capability for ``rule_id``."""

    if not isinstance(rule_id, str):
        return RuleCapability.UNSUPPORTED
    return _RULE_CAPABILITIES.get(rule_id, RuleCapability.UNSUPPORTED)


def stable_hit_id(
    hit: Mapping[str, Any], function_name: str | None = None
) -> str:
    """Hash the canonical identity tuple for a detector hit."""

    if not isinstance(hit, Mapping):
        raise TypeError("hit must be a mapping")

    file_name = _required_string(hit, "file")
    if function_name is None:
        normalized_function = _required_string(hit, "function")
    else:
        normalized_function = _nonempty_string(function_name, "function_name")
    line = _required_nonnegative_int(hit, "line")
    rule = _required_string(hit, "rule")

    col = hit.get("col", 0)
    if not _is_plain_int(col):
        raise TypeError("col must be an integer")
    if col < 0:
        raise ValueError("col must be non-negative")

    pattern = hit.get("pattern", "")
    snippet = hit.get("snippet", "")
    if not isinstance(pattern, str):
        raise TypeError("pattern must be a string")
    if not isinstance(snippet, str):
        raise TypeError("snippet must be a string")

    identity = (
        file_name,
        normalized_function,
        line,
        col,
        rule,
        pattern,
        snippet,
    )
    canonical = json.dumps(
        identity,
        ensure_ascii=False,
        separators=(",", ":"),
    ).encode("utf-8")
    return hashlib.sha256(canonical).hexdigest()


def _is_plain_int(value: object) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def _nonempty_string(value: object, field_name: str) -> str:
    if not isinstance(value, str):
        raise TypeError(f"{field_name} must be a string")
    if not value:
        raise ValueError(f"{field_name} must not be empty")
    return value


def _required_string(hit: Mapping[str, Any], field_name: str) -> str:
    if field_name not in hit:
        raise KeyError(f"missing required hit field: {field_name}")
    return _nonempty_string(hit[field_name], field_name)


def _required_nonnegative_int(hit: Mapping[str, Any], field_name: str) -> int:
    if field_name not in hit:
        raise KeyError(f"missing required hit field: {field_name}")
    value = hit[field_name]
    if not _is_plain_int(value):
        raise TypeError(f"{field_name} must be an integer")
    if value < 0:
        raise ValueError(f"{field_name} must be non-negative")
    return value


def hit_capability(hit: dict) -> RuleCapability:
    """Routing for one hit, not one rule.

    `III①` is registered CROSS_FUNCTION because forms A/B/C/D monomorphise a
    callback by changing the function's signature and every call site. Form E
    is the same rule but not the same work: it replaces one
    `qsort(..., Some(cmp))` inside the body with a native sort, touching
    nothing outside the function.

    Routing form E by rule id alone sends it to the cross-function path (a
    signature rewrite it does not need) and makes the region path discard it
    as `cross_fn_requires_changeset_v2`. Since any function large enough to
    carry a `qsort` call usually has enough hits to be region-split, that
    combination means such a hit is never attempted at all — measured worth
    -30.72% on one operation when applied by hand.
    """
    rule_id = hit.get("rule", "")
    if rule_id == "III①" and (hit.get("extra") or {}).get("form") == "E":
        return RuleCapability.REGION_LOCAL
    return rule_capability(rule_id)


def rules_region_local_for(rule_ids, hits) -> list[str]:
    """Which of `rule_ids` are NOT region-local for THIS candidate's hits.

    `rule_capability` answers per rule id, which is right for A/B/C/D of
    `III①` (they rewrite the signature and every call site) and wrong for
    form E (it replaces one `qsort(..., Some(cmp))` inside the body). The
    routing already distinguishes them via `hit_capability`; these gates,
    which see rule ids rather than hits, must agree or the candidate is
    rejected the moment it reaches the prompt:

        ERROR only region-local rules may enter a region prompt: ['III①']

    That is exactly what happened on the first run carrying form E — the
    detector found the site, the router sent it down the region path, and
    the prompt builder threw it away.

    A rule counts as region-local here when every hit carrying it in this
    candidate is region-local. With no hits to consult, fall back to the
    per-rule answer.
    """
    hits = list(hits or ())
    out: list[str] = []
    for rule_id in rule_ids:
        owned = [h for h in hits if isinstance(h, dict)
                 and h.get("rule") == rule_id]
        if owned:
            if all(hit_capability(h) is RuleCapability.REGION_LOCAL
                   for h in owned):
                continue
        elif rule_capability(rule_id) is RuleCapability.REGION_LOCAL:
            continue
        out.append(rule_id)
    return out
