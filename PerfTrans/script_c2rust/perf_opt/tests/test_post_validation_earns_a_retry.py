"""A guard rejection is a finding, not a verdict — it earns another turn.

The retry loop existed all along and took two classes: `syntax_error`, and a
contract abstain. A post-validation rejection took neither route. It mapped to
`ABSTAINED` (agent.py's terminal→attempt table), the loop's exit condition
only tested `SYNTAX_ERROR`, and so the attempt ended there — one LLM call and
one full build spent to learn nothing that was ever acted on.

Measured across four runs of the same crate:

  libqrencode  Mask_calcRunLengthH  rejected in 4/4 runs, never once landed
  libqrencode  Mask_calcRunLengthV  rejected in 2/4 (worth -6.353% when it did)
  fzy          precompute_bonus     rejected, run before it committed
  lil          hm_destroy           rejected, run before it committed

The rejection names the offending expression, which is precisely the feedback
a corrective turn needs. That is what makes it the same class as a compile
error and not the same class as "the model chose to abstain".
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.planners.llm_region import (
    is_contract_repairable,
    is_post_validation_repairable,
    post_validation_repair_note,
)


# ───────────────────────────── which findings earn a turn

def test_the_bounds_check_finding_is_repairable() -> None:
    assert is_post_validation_repairable("added_bounds_check_in_loop")


def test_machinery_disagreeing_with_itself_is_not_repairable() -> None:
    """No rewrite by the model fixes these — retrying only burns a turn."""
    for code in ("operation_edit_bijection", "region_file_isolated",
                 "edit_ranges", "replacement_function_span",
                 "replacement_post_image"):
        assert not is_post_validation_repairable(code), code


def test_absent_or_malformed_codes_do_not_earn_a_turn() -> None:
    for code in (None, "", 0, [], {"code": "x"}):
        assert not is_post_validation_repairable(code)


def test_it_is_a_separate_channel_from_contract_repair() -> None:
    """The two repair kinds must not be confused for one another: a guard
    finding is not a malformed reply, and vice versa."""
    assert not is_contract_repairable("added_bounds_check_in_loop")
    assert not is_post_validation_repairable("missing_rule_header")


# ───────────────────────────── what the corrective turn actually says

def _note(detail="rewrite adds a bounds check inside a loop the original "
                 "indexed unchecked: run_length[head as usize] = 1 as c_int;"):
    return post_validation_repair_note(
        "added_bounds_check_in_loop", detail, "fn f() { /* previous */ }")


def test_the_note_carries_the_finding_verbatim() -> None:
    """Without the offending expression the retry is a blind reroll."""
    note = _note()
    assert "run_length[head as usize]" in note
    assert "added_bounds_check_in_loop" in note


def test_the_note_names_the_way_out() -> None:
    note = _note()
    assert "get_unchecked" in note
    assert "SAFETY" in note


def test_the_note_quotes_the_previous_reply() -> None:
    """Same reason `contract_repair_note` does: the point is to correct the
    rewrite already produced, not to solicit a different one."""
    assert "/* previous */" in _note()


def test_the_note_closes_the_constant_size_table_loophole() -> None:
    """The exact reasoning a model used to justify keeping the index.

    Its words: "the nested table lookup can remain as indexing if proven
    constant-size". The table's size is not what makes the check run — the
    index coming from runtime data is.
    """
    note = _note().lower()
    assert "fixed-size lookup table is not an exception" in note


def test_an_unknown_code_still_produces_a_usable_note() -> None:
    """Belt and braces: the whitelist gates which codes reach this, but the
    formatter must not depend on that to avoid producing nonsense."""
    note = post_validation_repair_note("some_other_code", "the finding", "prev")
    assert "some_other_code" in note and "the finding" in note


def test_the_note_keeps_the_output_contract() -> None:
    """A corrective turn that drops the rule headers fails the next parse."""
    note = _note()
    assert "Applied rules:" in note and "Skipped rules:" in note


# ───────────────────────────── the loop is actually wired to it

def _agent_source() -> str:
    import inspect
    from perf_opt.agent_perf_opt import agent
    return inspect.getsource(agent)


def _path_source(name: str) -> str:
    import inspect
    from perf_opt.agent_perf_opt import agent
    return inspect.getsource(getattr(agent, name))


_LLM_PATHS = ("_try_fn_direct", "_try_fn_plan_execute", "_try_large_fn_regions")


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_every_llm_path_consults_the_predicate(path) -> None:
    """direct, COMPLEX and REGION all had the same dead end.

    REGION was the one that kept it longest, and it is the one that matters
    most: a crate whose hot functions are all large routes every candidate
    here. miniz did, and six of its seven guard rejections ended the
    candidate outright.
    """
    body = _path_source(path)
    assert "is_post_validation_repairable(" in body, path
    assert "post_validation_repair_note(" in body, path


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_a_repairable_finding_does_not_end_the_candidate(path) -> None:
    """The finding must reach a `continue`/non-break, not fall off the end."""
    body = _path_source(path)
    head = body[:body.index("is_post_validation_repairable(")]
    tail = body[body.index("post_validation_repair_note("):]
    assert "for " in head, f"{path}: the repair note has no loop to re-enter"
    assert "continue" in tail or "break" in tail, path


def test_the_finding_is_carried_out_of_the_gate() -> None:
    """`extra` must ship the code and detail, or the loop has nothing to
    hand the model."""
    src = _agent_source()
    assert 'extra["validation_code"]' in src
    assert 'extra["validation_detail"]' in src
    assert "if not v.ok" in src, "must select the FAILING validation"


# ───────────────────────────── the finding must survive the record layer

def test_the_finding_survives_being_splatted_into_an_attempt_record() -> None:
    """`extra` is handed to `AttemptRecord(**fields)` verbatim.

    Regression, caught only by running it: adding `validation_code` to `extra`
    without adding the field crashed the REGION path with
    `AttemptRecord.__init__() got an unexpected keyword argument`, 17 minutes
    into a run. The unit tests above all passed — they exercised the predicate
    and the note, and asserted on agent.py's source, but never pushed the dict
    through the layer that actually consumes it.
    """
    from perf_opt.agent_perf_opt import reporting
    from perf_opt.agent_perf_opt.state import AttemptRecord, RewriteAttempt

    extra = {
        "reason": "rejected_post_validation",
        "validation_code": "added_bounds_check_in_loop",
        "validation_detail": "rewrite adds a bounds check inside a loop: x[i]",
        "terminal_status": "rejected_post_validation",
    }
    # Built the way the agent builds it, not hand-rolled — a trace shaped by
    # hand can satisfy this test while the real one fails validation.
    trace = [reporting.make_trace_entry(
        1, "region", RewriteAttempt.ABSTAINED, "rejected_post_validation")]
    fields = reporting.with_attempt_trace(extra, trace)
    record = AttemptRecord(
        fn_name="f", rule_id="III④", round_no=1, attempt_no=1,
        status=RewriteAttempt.ABSTAINED, **fields,
    )
    assert record.validation_code == "added_bounds_check_in_loop"
    assert "x[i]" in record.validation_detail


def test_every_key_the_gate_adds_is_a_record_field_or_is_popped() -> None:
    """The general form of the same bug.

    Anything left in `extra` reaches `AttemptRecord(**fields)`, so a key must
    either BE a field or be popped before it gets there. `cargo_stderr_full`
    takes the second route deliberately — it is retry-loop scratch, too large
    and too transient to record — and the code says so at every pop site.
    Both routes are fine; having neither is the crash this test exists for.
    """
    import dataclasses, inspect, re
    from perf_opt.agent_perf_opt import agent
    from perf_opt.agent_perf_opt.state import AttemptRecord

    fields = {f.name for f in dataclasses.fields(AttemptRecord)}
    src = inspect.getsource(agent)
    popped = set(re.findall(r'extra\.pop\([\'"](\w+)[\'"]', src))
    for key in re.findall(r'extra\[[\'"](\w+)[\'"]\]\s*=', src):
        assert key in fields or key in popped, (
            f"agent puts extra[{key!r}] on the record path, but AttemptRecord "
            f"has no such field and nothing pops it — TypeError at runtime")
