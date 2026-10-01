"""Stable reporting helpers for optimization attempts."""

from __future__ import annotations

from copy import deepcopy
from typing import Any, Optional


PLANNER_ABSTAINED = "planner_abstained"
# The model produced a rewrite; a mechanical check refused it before any gate
# could judge whether it was any good — a malformed envelope, a replacement
# whose CST shape does not match the region it replaces, a rule id nobody
# offered. Distinct from PLANNER_ABSTAINED, which is the model's own verdict
# that the rewrite should not happen.
#
# They were one status until now, and the merge misreports the run in the
# direction that matters: `planner_abstained` is the single largest outcome
# (209 of 663 attempts across the corpus), which reads as "the model declined
# a third of the time — the rules must not fit". At least 56 of those are the
# model handing in work that our own checker bounced. One conclusion points at
# the cards, the other at the plumbing.
REJECTED_FORM = "rejected_form"
BUDGET_EXCEEDED = "budget_exceeded"
CROSS_FN_REQUIRES_CHANGESET_V2 = "cross_fn_requires_changeset_v2"
NO_REGION_LOCAL_RULES = "no_region_local_rules"
ALL_REGIONS_UNPROVEN = "all_regions_unproven"
ALL_REGIONS_OVER_BUDGET = "all_regions_over_budget"
ONLY_CROSS_FN_RULES = "only_cross_fn_rules"
# A single region candidate could not be resolved to its declared CST shape
# (stale hash / sequence not reconstructible). Non-fatal: the candidate is
# skipped and the run continues — one un-resolvable window must never crash
# the whole per-fn region pass.
REGION_RESOLVE_FAILED = "region_resolve_failed"
REGION_TERMINAL_STATUSES = frozenset(
    {
        CROSS_FN_REQUIRES_CHANGESET_V2,
        NO_REGION_LOCAL_RULES,
        ALL_REGIONS_UNPROVEN,
        ALL_REGIONS_OVER_BUDGET,
        ONLY_CROSS_FN_RULES,
        # per-candidate skip status (see agent.py region loop); listed here so
        # AttemptResult.add() accepts it as a valid terminal_status. It is NOT a
        # whole-fn terminal state — the loop uses `continue`, not this set, for
        # control flow (the set is validation/reporting only).
        REGION_RESOLVE_FAILED,
    }
)
AGENT_ONLY_TERMINAL_STATUSES = frozenset(
    {
        "plan_parse_fail",
        PLANNER_ABSTAINED,
        REJECTED_FORM,
        "all_rules_abstained_by_dispatch",
        BUDGET_EXCEEDED,
        "pre_abstain",
    }
)
# Failure codes that mean "the reply was refused on its form, so what it
# proposed was never judged". Everything here is upstream of every gate: the
# envelope was malformed, the replacement does not parse, or its CST shape
# does not fit the span it replaces. None of it is a statement about whether
# the rewrite would have helped.
#
# Deliberately NOT here:
#   * `no_applied_rules` — the model read the function and declined every
#     rule. That is exactly the judgment PLANNER_ABSTAINED is for.
#   * `added_bounds_check_in_loop` / `introduced_scan` / `replacement_post_image`
#     — a guard looked at the rewrite's EFFECT and objected. The rewrite was
#     judged; it lost.
#
# `planners.llm_region._CONTRACT_REPAIRABLE` is the subset of these that a
# second turn can plausibly fix. Subset, not equal: a wrong region shape is a
# mechanical refusal that re-asking will not repair. A test pins the
# containment so the two cannot drift apart.
MECHANICAL_REJECTION_CODES = frozenset(
    {
        # envelope / fence contract
        "parse_fail",
        "empty_response",
        "response_fence_contract",
        "metadata_headers_missing_or_not_first",
        "replacement_metadata_header",
        # declared-rule bookkeeping
        "duplicate_declared_rule",
        "candidate_rule_coverage_incomplete",
        "skipped_rule_reason_empty",
        "unknown_declared_rule",
        "rule_declared_in_both",
        # whole-function envelope
        "replacement_function_count",
        "replacement_function_name",
        "replacement_function_ambiguous",
        "replacement_function_span",
        "replacement_extra_source",
        "replacement_parse_error",
        "replace_function_edit_count",
        "target_function_name",
        # region shape — every code `validate_region_replacement_source` can
        # return. They all reach `_abstain` through the same two lines in
        # llm_region.py, so listing one and not its siblings files the same
        # kind of event under two different headings. `replace_region_shape`
        # was the only one here, which read as a deliberate choice but was a
        # gap: a model that hands in a replacement containing an `fn` item, or
        # one whose text does not parse, has done work our checker bounced —
        # not decided the rewrite should not happen. Measured on one corpus
        # run: three region guards fired, and two of them landed in
        # `planner_abstained`, the single largest outcome (197 of 601) and the
        # number used to argue the cards do not fit.
        "replace_region_shape",
        "replace_region_parse",
        "replace_region_forbidden_item",
        "replace_region_empty_not_allowed",
        "replace_region_utf8",
        "replace_region_invalid_kind",
        # A slice view whose length is derived from the index that reads it
        # (`from_raw_parts(p, i + 1)` then `s[i]`): refused on the reply text,
        # before any build. Not contract-repairable — the repair turn says
        # "send the same rewrite again", and this rewrite is what is wrong.
        "index_derived_slice_view",
    }
)


def reason_code(reason: str | None) -> str:
    """The bare failure code, with any `: <detail>` explanation stripped."""
    if not isinstance(reason, str):
        return ""
    return reason.split(":", 1)[0].strip()


def is_mechanical_rejection(reason: str | None) -> bool:
    """True when a check refused the reply before anything could judge it.

    Requires reasons to lead with their code (`ValidationResult.as_reason`).
    A planner that emits a bare human-readable detail lands here as False and
    is reported as the model's own judgment — which is the bug this pair of
    helpers exists to end, so keep the two in step.
    """
    return bool(reason) and reason_code(reason) in MECHANICAL_REJECTION_CODES


# The phases an attempt can be in. "freeform" belongs to the ablation arm,
# which has no cards and no regions and therefore none of the other six —
# leaving it out made the arm die on its first rejected candidate, after the
# LLM call, the build and a clean 900/900 W1 replay had all already run.
ATTEMPT_PHASES = frozenset(
    {"direct", "plan", "execute", "typed", "region", "cross_fn", "freeform"}
)
_TRACE_REQUIRED_KEYS = frozenset(
    {"attempt_no", "phase", "rewrite_status", "terminal_status", "error"}
)


def make_trace_entry(
    attempt_no: int,
    phase: str,
    rewrite_status: Any,
    terminal_status: str,
    error: Optional[str] = None,
) -> dict:
    """Build one JSON-serializable attempt trace entry."""
    normalized_status = (
        rewrite_status
        if isinstance(rewrite_status, str)
        else getattr(rewrite_status, "value", None)
    )
    entry = {
        "attempt_no": attempt_no,
        "phase": phase,
        "rewrite_status": normalized_status,
        "terminal_status": terminal_status,
        "error": error,
    }
    _validate_trace_entry(entry, "trace entry")
    return entry


def _validate_trace_entry(entry: Any, label: str) -> int:
    if not isinstance(entry, dict):
        raise ValueError(f"{label} must be a dict")
    missing = _TRACE_REQUIRED_KEYS.difference(entry)
    if missing:
        raise ValueError(f"{label} missing required keys: {sorted(missing)}")

    attempt_no = entry["attempt_no"]
    if (
        not isinstance(attempt_no, int)
        or isinstance(attempt_no, bool)
        or attempt_no <= 0
    ):
        raise ValueError(f"{label}.attempt_no must be a positive integer")

    phase = entry["phase"]
    if not isinstance(phase, str) or phase not in ATTEMPT_PHASES:
        raise ValueError(f"{label}.phase must be one of {sorted(ATTEMPT_PHASES)}")

    for key in ("rewrite_status", "terminal_status"):
        value = entry[key]
        if not isinstance(value, str) or not value:
            raise ValueError(f"{label}.{key} must be a non-empty string")

    error = entry["error"]
    if error is not None and not isinstance(error, str):
        raise ValueError(f"{label}.error must be a string or None")
    return attempt_no


def validate_attempt_trace(
    terminal_status: Optional[str],
    attempt_trace: Optional[list[dict]],
) -> None:
    """Validate an optional, fail-closed attempt trace."""
    if attempt_trace is None:
        return
    if not isinstance(attempt_trace, list) or not attempt_trace:
        raise ValueError("attempt_trace must be a non-empty list or None")
    if not isinstance(terminal_status, str) or not terminal_status:
        raise ValueError("terminal_status must be a non-empty string with a trace")

    for index, entry in enumerate(attempt_trace):
        attempt_no = _validate_trace_entry(entry, f"attempt_trace[{index}]")
        if attempt_no != index + 1:
            raise ValueError(
                "attempt_trace attempt_no values must be contiguous starting at 1"
            )

    if terminal_status != attempt_trace[-1]["terminal_status"]:
        raise ValueError(
            "terminal_status must match the last attempt trace terminal_status"
        )


def with_attempt_trace(extra: dict, attempt_trace: list[dict]) -> dict:
    """Return fresh AttemptRecord kwargs with one validated trace attached."""
    if not isinstance(extra, dict):
        raise ValueError("extra must be a dict")
    fields = deepcopy(extra)
    trace = deepcopy(attempt_trace)
    if not isinstance(trace, list) or not trace:
        raise ValueError("attempt_trace must be a non-empty list")
    if not isinstance(trace[-1], dict):
        raise ValueError("attempt_trace entries must be dicts")

    terminal_status = trace[-1].get("terminal_status")
    existing_terminal = fields.pop("terminal_status", None)
    existing_trace = fields.pop("attempt_trace", None)
    if existing_terminal is not None and existing_terminal != terminal_status:
        raise ValueError("extra terminal_status conflicts with attempt_trace")
    if existing_trace is not None and existing_trace != trace:
        raise ValueError("extra attempt_trace conflicts with attempt_trace")

    validate_attempt_trace(terminal_status, trace)
    fields["terminal_status"] = terminal_status
    fields["attempt_trace"] = trace
    return fields
