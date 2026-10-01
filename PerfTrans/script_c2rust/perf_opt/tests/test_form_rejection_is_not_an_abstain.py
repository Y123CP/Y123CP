"""A refused reply and a declined rewrite are two different outcomes.

`planner_abstained` reads as "the model looked at this function and decided
the rewrite should not happen". It was also, until now, what a reply got when
our own checker bounced it — a malformed envelope, a replacement whose CST
shape does not fit the span, a rule id nobody offered. In those cases the
model handed in work and nothing ever judged it.

Merged, the two misreport the run in the direction that matters. Across the
corpus `planner_abstained` is the single largest outcome (209 of 663
attempts), which reads as "the model declines a third of the time — the rules
must not fit". At least 56 of those were our checker refusing the reply. One
reading sends you to the cards, the other to the plumbing.

The root cause of the merge was upstream: planners wrote
`result.detail or result.code`, and a detail almost always exists, so the
code — the only classifiable half — was dropped. `replacement_function_count`
is listed as contract-repairable, yet its detail masked the code, so the
repair turn it was entitled to never fired.
"""

from __future__ import annotations

import importlib

import pytest

from perf_opt.agent_perf_opt import reporting
from perf_opt.agent_perf_opt.changeset.types import ValidationResult


# ─────────────────────────────────────── a reason must keep its code

def test_a_validation_failure_leads_with_its_code() -> None:
    v = ValidationResult(False, "replace_region_shape",
                         "replacement is not a complete node_sequence shape")
    assert v.as_reason().startswith("replace_region_shape")
    assert "node_sequence" in v.as_reason(), "the detail must survive too"


def test_a_failure_with_no_detail_is_just_its_code() -> None:
    assert ValidationResult(False, "parse_fail").as_reason() == "parse_fail"


def test_the_detail_no_longer_masks_the_code() -> None:
    """The exact shape that defeated the repair table: a detail that is
    itself a plausible-looking code."""
    v = ValidationResult(False, "replacement_function_count",
                         "replacement_must_contain_exactly_one_function")
    assert reporting.reason_code(v.as_reason()) == "replacement_function_count"
    assert reporting.is_mechanical_rejection(v.as_reason())


def test_every_planner_leads_with_the_code(monkeypatch) -> None:
    """`detail or code` was fixed once, in one planner, and left standing in
    three others. Pin all of them so the fourth escape does not happen."""
    import inspect
    import re

    # `<x>.detail or <y>.code` specifically — `detail or None` is a different
    # idiom (empty string to None) and is fine.
    pattern = re.compile(r"\.detail\s+or\s+[\w.]*\bcode\b")

    for mod in ("planners.llm_region", "planners.llm_cross_fn",
                "planners.llm_function", "changeset.executor"):
        src = inspect.getsource(
            importlib.import_module(f"perf_opt.agent_perf_opt.{mod}"))
        # Code only — the comments explaining why this pattern is wrong
        # naturally quote it.
        offenders = [
            line.strip() for line in src.splitlines()
            if pattern.search(line) and not line.lstrip().startswith("#")
        ]
        assert not offenders, (
            f"{mod} still builds a reason as `detail or code`, which drops "
            f"the code whenever a detail exists — use as_reason(): {offenders}")


# ──────────────────────────────────────────────── the classification

@pytest.mark.parametrize("reason", [
    "parse_fail",
    "empty_response",
    "metadata_headers_missing_or_not_first",
    "replace_region_shape: replacement is not a complete node_sequence shape",
    "replacement_function_count: replacement_must_contain_exactly_one_function",
    "unknown_declared_rule:['C99']",
    "rule_declared_in_both:['C3']",
])
def test_a_refused_reply_is_classified_as_such(reason: str) -> None:
    assert reporting.is_mechanical_rejection(reason)


@pytest.mark.parametrize("reason", [
    # The model read the function and declined every rule. That IS a judgment.
    "no_applied_rules",
    # A guard looked at the rewrite's effect and objected — it was judged.
    "added_bounds_check_in_loop",
    "introduced_scan",
    # The model's own prose.
    "III④ requires rewriting the raw-pointer cursor into a slice cursor",
    "llm_abstain:target uses a struct-field callback",
])
def test_a_real_judgment_is_left_alone(reason: str) -> None:
    assert not reporting.is_mechanical_rejection(reason)


def test_an_absent_reason_is_not_a_refusal() -> None:
    assert not reporting.is_mechanical_rejection(None)
    assert not reporting.is_mechanical_rejection("")


def test_repairable_codes_are_a_subset_of_refusals() -> None:
    """Every reason a repair turn can act on is, by definition, a reply that
    was refused on its form. The reverse does not hold — a wrong region shape
    is a refusal that re-asking will not fix — so this is containment, not
    equality, and it is pinned because the two tables live in different
    modules."""
    from perf_opt.agent_perf_opt.planners.llm_region import _CONTRACT_REPAIRABLE

    assert set(_CONTRACT_REPAIRABLE) <= reporting.MECHANICAL_REJECTION_CODES


# ─────────────────────────────────────────── what the agent records

def _record(monkeypatch, reason: str):
    agent = importlib.import_module("perf_opt.agent_perf_opt.agent")
    return agent._planner_abstained("hot_fn", "region", ["C3"], reason)


def test_a_refused_reply_is_recorded_as_rejected_form(monkeypatch) -> None:
    extra = _record(monkeypatch, "replace_region_shape: not a complete shape")
    assert extra["terminal_status"] == reporting.REJECTED_FORM


def test_a_declined_rewrite_is_still_recorded_as_an_abstain(monkeypatch) -> None:
    extra = _record(monkeypatch, "no_applied_rules")
    assert extra["terminal_status"] == reporting.PLANNER_ABSTAINED


def test_the_two_are_counted_separately(monkeypatch) -> None:
    agent = importlib.import_module("perf_opt.agent_perf_opt.agent")
    result = agent.AgentResult()
    for reason in ("no_applied_rules",
                   "replace_region_shape: not a complete shape",
                   "parse_fail"):
        extra = agent._planner_abstained("f", "region", ["C3"], reason)
        result.add(agent.AttemptRecord(
            fn_name="f", rule_id="C3", round_no=1, attempt_no=1,
            status=agent.RewriteAttempt.ABSTAINED,
            terminal_status=extra["terminal_status"],
            reason=extra["reason"]))
    assert result.abstained_count == 1, "only the real judgment is an abstain"
    assert result.form_rejected_count == 2
    assert result.total_attempts == 3


# ──────────────────────── one checker's codes must not split across headings

def _codes_returned_by(func_name: str) -> set[str]:
    """Every `replace_region_*` literal inside one function's body.

    Read from source rather than listed by hand: a hand-written list is a
    snapshot that stops matching the day someone adds a return, which is
    exactly how the gap below was introduced.
    """
    import pathlib
    import re

    from perf_opt.agent_perf_opt.changeset.handlers import replace_region

    src = pathlib.Path(replace_region.__file__).read_text()
    start = src.find(f"def {func_name}")
    assert start != -1, f"{func_name} not found — did it get renamed?"
    end = src.find("\ndef ", start + 10)
    body = src[start:end if end != -1 else len(src)]
    return set(re.findall(r'"(replace_region_\w+)"', body))


def test_every_region_replacement_code_is_a_form_rejection() -> None:
    """All six reach `_abstain` through the same two lines in llm_region.py.

    ```
    shape = validate_region_replacement_source(region.region_kind, replacement)
    if not shape.ok:
        return _abstain(shape.as_reason())
    ```

    One checker, one call site, one meaning: the model handed in a
    replacement our own parser refused. Filing one of them as
    `rejected_form` and its siblings as `planner_abstained` describes the
    same event two ways, and the second way is the one that gets counted
    against the cards.
    """
    codes = _codes_returned_by("validate_region_replacement_source")
    assert len(codes) >= 5, f"expected the full family, got {sorted(codes)}"
    missing = sorted(c for c in codes
                     if not reporting.is_mechanical_rejection(f"{c}: detail"))
    assert not missing, (
        f"{missing} still read as the model declining the rewrite. They come "
        f"from the same checker as replace_region_shape — add them to "
        f"MECHANICAL_REJECTION_CODES."
    )


def test_a_forbidden_item_is_not_the_model_declining(monkeypatch) -> None:
    """The concrete case: a replacement carrying an `fn` item.

    Observed on one crate's 2214-line function — the model tried to lift a
    helper out of the region, which the contract forbids in its first line.
    That is a refused reply, not a judgment about whether the rules fit.
    """
    extra = _record(
        monkeypatch,
        "replace_region_forbidden_item: replacement contains a function, "
        "module, impl, trait, or extern item",
    )
    assert extra["terminal_status"] == reporting.REJECTED_FORM


def test_a_genuine_abstain_is_untouched_by_the_widening() -> None:
    """Widening the set must not swallow the reasons it was meant to isolate."""
    for reason in ("no_applied_rules", "abstained_unproven",
                   "all_regions_unproven", "no_length_source: not provable"):
        assert not reporting.is_mechanical_rejection(reason), reason
