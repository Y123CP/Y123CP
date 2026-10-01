"""A reply that fails on FORM has not had its rewrite judged — ask again.

The region planner rejects a reply for two very different kinds of reason.
One is about the rewrite: the model declined every rule, or the replacement
is not the shape the region needs. The other is about the envelope: the two
declaration header lines are malformed, the fence has prose around it, a rule
id is misspelled. In the second kind the rewrite inside was never looked at,
and throwing it away throws away work that may be correct and valuable.

It usually is. Two consecutive runs of one crate lost the same rewrite this
way. The model had applied three rules, including the `qsort` →
`sort_unstable_by` conversion hand-measured at -30.72% on one operation, and
the reply was discarded because a skip reason quoted `[*mut Node; 2]`. Nothing
retried, the region's anchors were retired, and the run finished without ever
seeing it again — recorded, from the outside, as "the rule did not apply".

Widening that one bracket (see test_rule_header_contract) fixes that instance.
It does not close the class: the next unanticipated bit of punctuation lands
in exactly the same place. So a form failure now gets one corrective turn,
carrying the previous reply and the specific violation, bounded by the same
per-function token budget.
"""

from __future__ import annotations

import hashlib

import pytest

from perf_opt.agent_perf_opt.changeset import (
    RegionKind,
    RegionRef,
    SymbolKind,
    SymbolRef,
)
from perf_opt.agent_perf_opt.planners.llm_region import (
    contract_repair_note,
    is_contract_repairable,
    plan_region_with_contract_repair,
)


def _region() -> RegionRef:
    return RegionRef(
        target_function=SymbolRef(
            qualified_name="crate::target",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint="src/lib.rs",
            declaration_hash=hashlib.sha256(b"fn-a").hexdigest(),
        ),
        relative_path="src/lib.rs",
        region_kind=RegionKind.STATEMENT,
        parent_kind="block",
        start_byte=10,
        end_byte=20,
        expected_region_hash=hashlib.sha256(b"a").hexdigest(),
        anchor_hit_ids=(hashlib.sha256(b"hit-a").hexdigest(),),
        anchor_lines=(2,),
    )


GOOD = ("```rust\n"
        "// Applied rules: [C1]\n"
        "// Skipped rules: [C2: no leverage]\n"
        "let value = 2;\n"
        "```")
# Headers absent — the failure that cost two runs, reduced.
MALFORMED = "```rust\nlet value = 2;\n```"
# The model declining every rule: a judgment, not a form error.
DECLINED = ("```rust\n"
            "// Applied rules: []\n"
            "// Skipped rules: [C1: no hot loop; C2: single use]\n"
            "let value = 2;\n"
            "```")


class _Chat:
    """Returns each scripted reply in turn and records what it was asked."""

    def __init__(self, *replies: str) -> None:
        self.replies = list(replies)
        self.prompts: list[str] = []

    def __call__(self, system: str, user: str) -> str:
        self.prompts.append(user)
        return self.replies[min(len(self.prompts) - 1, len(self.replies) - 1)]


def _run(chat, *, budget: int = 10 ** 6):
    return plan_region_with_contract_repair(
        chat=chat,
        system_prompt="SYS",
        user_prompt="USER",
        candidate_id_for=lambda reply: "cand-" + hashlib.sha256(
            reply.encode()).hexdigest()[:8],
        token_estimate=lambda text: len(text),
        budget_remaining=budget,
        region=_region(),
        rule_ids=("C1", "C2"),
        base_head="head-a",
    )


# ─────────────────────────────────── which failures are worth re-asking

@pytest.mark.parametrize("reason", [
    "metadata_headers_missing_or_not_first",
    "replacement_metadata_header",
    "response_fence_contract",
    "parse_fail",
    "empty_response",
    "duplicate_declared_rule",
    "candidate_rule_coverage_incomplete",
    "skipped_rule_reason_empty",
    "unknown_declared_rule:['C9']",
    "rule_declared_in_both:['C1']",
])
def test_envelope_failures_are_repairable(reason) -> None:
    assert is_contract_repairable(reason)


@pytest.mark.parametrize("reason", [
    "no_applied_rules",                       # the model declined — a judgment
    "not a complete node_sequence shape",     # the rewrite BODY is wrong
    "invalid_region",
    "the loop is not hot enough to justify a rewrite",   # free-text abstain
    "",
    None,
])
def test_judgments_and_body_failures_are_not_repairable(reason) -> None:
    """Re-asking here is either pestering the model about a decision it made,
    or telling it to "send the same thing again" when the thing is wrong."""
    assert not is_contract_repairable(reason)


# ─────────────────────────────────── the repair turn

def test_a_malformed_reply_gets_one_more_turn() -> None:
    chat = _Chat(MALFORMED, GOOD)
    attempt = _run(chat)
    assert len(chat.prompts) == 2
    assert attempt.plan.proposal is not None
    assert attempt.repaired_from == "metadata_headers_missing_or_not_first"


def test_a_good_reply_costs_exactly_one_turn() -> None:
    chat = _Chat(GOOD)
    attempt = _run(chat)
    assert len(chat.prompts) == 1
    assert attempt.repaired_from is None


def test_a_declined_reply_is_not_re_asked() -> None:
    chat = _Chat(DECLINED)
    attempt = _run(chat)
    assert len(chat.prompts) == 1
    assert attempt.plan.proposal is None
    assert attempt.plan.abstain_reason == "no_applied_rules"


def test_the_repair_is_asked_exactly_once() -> None:
    """Bounded, not a loop — a model that keeps failing must not burn the
    function's whole budget on one region."""
    chat = _Chat(MALFORMED, MALFORMED, GOOD)
    attempt = _run(chat)
    assert len(chat.prompts) == 2
    assert attempt.plan.proposal is None


def test_the_second_failure_is_the_one_reported() -> None:
    chat = _Chat(MALFORMED, DECLINED)
    attempt = _run(chat)
    assert attempt.plan.abstain_reason == "no_applied_rules"
    assert attempt.repaired_from == "metadata_headers_missing_or_not_first"


# ─────────────────────────────────── what the repair turn says

def test_the_repair_prompt_carries_the_previous_reply() -> None:
    """Without it the model writes a NEW rewrite — a different proposal, and
    the one worth recovering is gone anyway."""
    chat = _Chat(MALFORMED, GOOD)
    _run(chat)
    assert MALFORMED in chat.prompts[1]


def test_the_repair_prompt_names_the_violation() -> None:
    chat = _Chat(MALFORMED, GOOD)
    _run(chat)
    assert "metadata_headers_missing_or_not_first" in chat.prompts[1]


def test_the_repair_prompt_keeps_the_original_task() -> None:
    """It is the same region and the same cards; only a note is appended."""
    chat = _Chat(MALFORMED, GOOD)
    _run(chat)
    assert chat.prompts[1].startswith("USER")


def test_the_repair_prompt_says_not_to_re_derive() -> None:
    note = contract_repair_note("parse_fail", ("C1",), "prev")
    assert "same rewrite again" in note
    assert "Do not re-derive" in note


def test_the_repair_prompt_lists_the_exact_candidate_ids() -> None:
    """`unknown_declared_rule` is usually a transcription slip; quoting the
    ids back is what makes the second turn able to fix it."""
    note = contract_repair_note("unknown_declared_rule:['C9']", ("C1", "C2"),
                                "prev")
    assert "['C1', 'C2']" in note


def test_the_repair_prompt_warns_about_the_punctuation_that_caused_this() -> None:
    note = contract_repair_note("metadata_headers_missing_or_not_first",
                                ("C1",), "prev")
    assert "backticks" in note
    assert "single line" in note


# ─────────────────────────────────── the budget still binds

def test_the_repair_is_skipped_when_it_would_not_fit() -> None:
    chat = _Chat(MALFORMED, GOOD)
    attempt = _run(chat, budget=len("SYS") + len("USER") + len(MALFORMED))
    assert len(chat.prompts) == 1
    assert attempt.plan.proposal is None
    assert attempt.repaired_from is None


def test_the_repair_turn_is_counted_in_the_token_totals() -> None:
    """The caller adds these to the per-function budget; undercounting them
    would let one region overrun it."""
    one = _run(_Chat(GOOD))
    two = _run(_Chat(MALFORMED, GOOD))
    assert two.tokens_in > one.tokens_in
    assert two.tokens_out > 0


# ─────────────────────────────────── the live path uses it

def test_the_region_loop_goes_through_the_repair_driver() -> None:
    """Guard against the fix landing on a path the run does not take — that
    has happened before, with 891 tests still green."""
    import inspect
    from perf_opt.agent_perf_opt import agent

    src = inspect.getsource(agent)
    assert src.count("plan_region_with_contract_repair(") == 1


def test_the_repair_turn_is_labelled_in_the_transcript() -> None:
    """Two calls land in one transcript; without a distinct label the second
    is indistinguishable from a fresh candidate when reading a run back."""
    import inspect
    from perf_opt.agent_perf_opt import agent

    assert '"region-repair"' in inspect.getsource(agent)


# ═══════════════════════════ the whole-function path

def test_the_whole_fn_envelope_codes_are_repairable() -> None:
    """These name what is wrong with the REPLY, not with the rewrite: one
    function too many, a renamed function, source trailing the function. The
    body inside may be exactly right — on one crate a correct rewrite of the
    hottest function (99.75% self time in its op) was discarded for nesting a
    helper, and the planner never came back to it."""
    for code in ("replacement_function_count", "replacement_function_name",
                 "replacement_extra_source"):
        assert is_contract_repairable(code)
        assert is_contract_repairable(f"{code}: some detail from the validator")


def test_a_broken_body_is_still_not_repairable() -> None:
    """`replacement_parse_error` is about the rewrite itself. "Send the same
    thing again, reshaped" is the wrong instruction for code that does not
    parse, so it stays out — the same line the region path draws."""
    assert not is_contract_repairable("replacement_parse_error")
    assert not is_contract_repairable("replacement_parse_error: unexpected `;`")


def test_the_code_survives_the_detail() -> None:
    """The classification depends on the code being readable. It was being
    dropped: `validation.detail or validation.code` returned free text
    whenever a detail existed, which is precisely the informative cases."""
    from perf_opt.agent_perf_opt.planners.llm_region import _reason_code
    assert _reason_code("replacement_function_count: found 2") == \
        "replacement_function_count"
    assert _reason_code("parse_fail") == "parse_fail"


def test_the_whole_fn_loop_lets_contract_abstains_retry() -> None:
    """The loop existed all along and admitted only `syntax_error`; a contract
    abstain fell out of it on the first turn."""
    import inspect
    from perf_opt.agent_perf_opt import agent

    src = inspect.getsource(agent)
    block = src[src.index("prev_contract_reason: str | None = None"):]
    block = block[:block.index("prev_output = response") + 40]
    assert "is_contract_repairable(contract_reason)" in block
    assert "RewriteAttempt.ABSTAINED" in block
    assert "contract_reason is None" in block


def test_the_whole_fn_repair_turn_uses_the_shared_note() -> None:
    """One wording, one set of rules, both paths — a second hand-written note
    would drift from the first."""
    import inspect
    from perf_opt.agent_perf_opt import agent

    src = inspect.getsource(agent)
    assert "contract_repair_note(" in src
    assert "direct_repair" in src


def test_a_semantic_abstain_still_ends_the_whole_fn_loop() -> None:
    """The guard on the relaxation: the model declining is a decision, and
    re-asking is pestering it."""
    assert not is_contract_repairable("no_applied_rules")
    assert not is_contract_repairable("the loop is too short to be worth it")
