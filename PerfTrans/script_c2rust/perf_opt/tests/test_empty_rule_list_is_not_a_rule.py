"""`none` is not a rule id, however the model spells the empty list.

Measured, lodepng `lodepng_compute_color_stats` on 2026-08-26 — a 159-line
rewrite lost across two turns to the same misunderstanding:

    03:54:32  region        // Skipped rules: none
              -> metadata_headers_missing_or_not_first
    03:55:05  region-repair // Skipped rules: [none]
              -> unknown_declared_rule:['none']  -> rejected_form

Three things had to line up for that:

  * The REGION contract never said how to write an empty list. The whole-fn
    contract does ("Skipped rules use `[]` if none"); the region one, which
    is what large functions route through, did not.
  * The repair note said the line must read ``// Skipped rules: [...]``, so
    the model put its `none` inside the brackets — doing exactly as asked,
    into a second failure.
  * The repair budget is one turn. The first form error spent it, and
    `unknown_declared_rule` — repairable in principle — never got a turn.

For scale: across the same run the model wrote a correct `[]` ten times. This
is the contract's gap showing up occasionally, not a model that cannot count.
"""

from __future__ import annotations

import inspect

import pytest

from perf_opt.agent_perf_opt.rewrite_applier import (
    parse_applied_rules,
    parse_skipped_rules,
)


# ─────────────────────────────────────── the parser reconciles the spelling

@pytest.mark.parametrize("spelling", [
    "none", "None", "NONE", "-", "--", "n/a", "N/A", "na",
    "nil", "null", "empty", "nothing", "no rules", "(none)", "none.",
])
def test_an_empty_skipped_list_however_it_is_spelled(spelling) -> None:
    assert parse_skipped_rules(f"// Skipped rules: [{spelling}]") == []


@pytest.mark.parametrize("spelling", ["none", "N/A", "-", "nothing"])
def test_an_empty_applied_list_however_it_is_spelled(spelling) -> None:
    """`[none]` in the applied line means the same thing `[]` does: nothing
    was applied. It must reach `no_applied_rules`, not invent a rule."""
    assert parse_applied_rules(f"// Applied rules: [{spelling}]") == []


def test_the_exact_reply_that_lost_the_rewrite() -> None:
    code = ("// Applied rules: [C3]\n"
            "// Skipped rules: [none]\n"
            "let mut r_1: c_uchar = 0 as c_uchar;\n")
    assert parse_applied_rules(code) == ["C3"]
    assert parse_skipped_rules(code) == []


# ───────────────────────────────────── a real rule is still a real rule

def test_a_real_skip_entry_survives() -> None:
    got = parse_skipped_rules("// Skipped rules: [C6: no loop-invariant dispatch]")
    assert got == [("C6", "no loop-invariant dispatch")]


def test_a_rule_whose_reason_mentions_none_survives() -> None:
    """The sentinel is the WHOLE list, never a word inside a reason."""
    got = parse_skipped_rules(
        "// Skipped rules: [C3: none of the loads here are invariant]")
    assert got == [("C3", "none of the loads here are invariant")]


@pytest.mark.parametrize("rules", ["C3", "C3, III④", "C1, C3, C6"])
def test_ordinary_applied_lists_are_untouched(rules) -> None:
    expected = [r.strip() for r in rules.split(",")]
    assert parse_applied_rules(f"// Applied rules: [{rules}]") == expected


def test_a_rule_id_is_not_swallowed_by_a_prefix_match() -> None:
    """`nan`-like ids must not be read as the `na` sentinel."""
    assert parse_applied_rules("// Applied rules: [nan_guard]") == ["nan_guard"]


# ──────────────────────────────────────── both prompts now say how

def test_the_region_contract_says_to_write_an_empty_list_as_brackets() -> None:
    from perf_opt.agent_perf_opt import prompt_builder
    src = inspect.getsource(prompt_builder)
    marker = "// Skipped rules: [<rule>: <non-empty reason>; ...]"
    assert marker in src
    tail = src[src.index(marker):src.index(marker) + 700]
    assert "`[]`" in tail, tail
    assert "none" in tail, "the failure mode must be named, not just the fix"


def test_the_repair_note_says_it_too() -> None:
    """The repair is where the model was actively steered wrong."""
    from perf_opt.agent_perf_opt.planners import llm_region
    note = llm_region.contract_repair_note(
        "metadata_headers_missing_or_not_first", ("C3",), "```rust\n...\n```")
    assert "`[]`" in note
    assert "none" in note


def test_the_repair_note_still_carries_the_previous_reply() -> None:
    """Guard against the added sentence displacing what the repair is for."""
    note = None
    from perf_opt.agent_perf_opt.planners import llm_region
    note = llm_region.contract_repair_note(
        "metadata_headers_missing_or_not_first", ("C3",), "PREVIOUS_REPLY_BODY")
    assert "PREVIOUS_REPLY_BODY" in note
    assert "['C3']" in note
