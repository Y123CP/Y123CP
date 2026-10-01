"""A W1 failure is a mechanical finding, and it was the only one with no retry.

W1 replays golden records and compares stdout byte for byte. A failure names
the op and the mismatch — the same kind of definite signal a compile error is,
and unlike a W2 verdict, which is a judgement about speed that no amount of
re-prompting can argue with.

It is also the cheapest failure a candidate can reach: the first disagreeing
record shows up in well under a second, against the minutes W2 spends. Yet it
ended the candidate outright.

Measured: one crate's sub-word `split` rewrite failed W1 because the model
wrote the 64-bit selection mask with its bytes reversed (`0x8040_2010_...`
where the transform needs `0x0102_0408_...`). The rest of the rewrite was
byte-for-byte the one that had measured -7.555% a run earlier. It was
discarded, the function kept its 8-iteration loop, and the crate landed at
-5.127%.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.planners.llm_region import w1_repair_note


# ───────────────────────────────────────────── the note carries the evidence

def _note(reason: str = "compress_dict: stdout sha mismatch at record 3",
          previous: str = "// Applied rules: [C10]\nfn f() {}") -> str:
    return w1_repair_note(reason, previous)


def test_the_note_quotes_the_failure_verbatim() -> None:
    assert "compress_dict: stdout sha mismatch at record 3" in _note()


def test_the_note_quotes_the_rejected_output() -> None:
    """Without it the model writes a NEW rewrite, which is a different and
    unmeasured proposal. The point is to recover the one it already had."""
    assert "// Applied rules: [C10]" in _note()


def test_the_note_says_the_output_changed_not_that_it_was_slow() -> None:
    low = _note().lower()
    assert "output" in low
    assert "byte for byte" in low or "byte-for-byte" in low


def test_the_note_forbids_abandoning_the_optimisation() -> None:
    """A model told only "that was wrong" abstains. The cheapest wrong answer
    must be closed off explicitly."""
    low = _note().lower()
    assert "do not abandon" in low and "start over" in low


def test_the_note_names_the_constant_first() -> None:
    """The measured failure was a hand-derived constant written backwards, and
    it is the failure mode the whole SWAR family shares."""
    low = _note().lower()
    assert "re-derive" in low
    assert "domain" in low, "must ask for an exhaustive check, not a re-read"


def test_the_note_names_endianness_and_the_tail() -> None:
    low = _note().lower()
    assert "to_le_bytes" in low or "endian" in low
    assert "tail" in low or "len % n" in low


def test_the_note_keeps_the_output_contract() -> None:
    note = _note()
    assert "Applied rules:" in note and "Skipped rules:" in note


def test_the_note_is_bounded() -> None:
    """A repair turn must not double the prompt. Prompt bloat has already cost
    one run a rewrite: 750 tokens of extra prose turned a committed iterator
    form into a while-index regression worth 6.7 points."""
    assert len(_note()) < 4000


def test_a_huge_note_is_bounded_in_its_prose_not_in_the_reply() -> None:
    """The bound above is about prose, and stays about prose.

    Quoting the previous reply in part is not a saving — it is what breaks
    the repair. A reply cut mid-expression is not valid Rust, so "fix the
    compile error in this" asks about code whose end the model cannot see;
    measured, one function's three build repairs all died on `unexpected
    closing delimiter: }` after being handed 2000 of its 2300 characters.
    So the reply is quoted whole and only the prose around it is capped.
    """
    reply = "y" * 10_000
    note = w1_repair_note("x" * 10_000, reply)
    assert reply in note
    assert len(note) - len(reply) < 4000


# ───────────────────────────────────────────── the loops are wired to it

def _path_source(name: str) -> str:
    import inspect
    from perf_opt.agent_perf_opt import agent
    return inspect.getsource(getattr(agent, name))


_LLM_PATHS = ("_try_fn_direct", "_try_fn_plan_execute", "_try_large_fn_regions")


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_every_llm_path_offers_a_w1_repair(path) -> None:
    body = _path_source(path)
    assert "w1_repair_note(" in body, path
    assert "RewriteAttempt.W1_FAIL" in body, path


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_the_w1_repair_is_capped_at_one_turn(path) -> None:
    """A second failure means the model does not understand the transform, not
    that it mistyped. Unbounded retries would spend a whole function's budget
    on a rewrite that is not going to become correct."""
    body = _path_source(path)
    if path == "_try_large_fn_regions":
        # the region loop is `for repair_turn in range(2)` — one repair, once
        assert "for repair_turn in range(2)" in body
        assert "repair_turn == 0" in body
    else:
        assert "w1_repairs < 1" in body, path
        assert "w1_repairs += 1" in body, path


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_the_w1_reason_is_carried_out_of_the_gate(path) -> None:
    """`extra["w1_result"]` is where the gate leaves it; the loop must read
    that, not re-run the gate."""
    body = _path_source(path)
    assert 'extra.get("w1_result")' in body, path


def test_the_gate_puts_the_reason_where_the_loops_look() -> None:
    """The producing side. If this key ever moves, every retry above goes
    silently blind rather than failing."""
    import inspect
    from perf_opt.agent_perf_opt import agent
    src = inspect.getsource(agent)
    assert "extra.update(w1_result=reason" in src


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_a_bare_w2_verdict_still_gets_no_retry(path) -> None:
    """The boundary, restated precisely.

    A W2 verdict on its own is a measurement, not a defect report —
    re-prompting on it asks the model to guess its way past a stopwatch and
    spends a build, a W1 and a W2 doing it. The ONLY W2 rejection that earns a
    turn is one that also carries a mechanical finding naming something the
    rewrite introduced, which is a defect report. So every `W2_REGRESS` test
    in these paths must be conjoined with a finding lookup.
    """
    body = _path_source(path)
    marker = "RewriteAttempt.W2_REGRESS"
    at = body.find(marker)
    assert at != -1, f"{path}: no W2_REGRESS test at all"
    while at != -1:
        window = body[max(0, at - 400):at + 400]
        assert "w2_finding" in window or "w2f_repairs" in window, (
            f"{path}: a W2_REGRESS test not gated on a finding")
        at = body.find(marker, at + 1)


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_the_w2_finding_retry_is_capped_at_one_turn(path) -> None:
    body = _path_source(path)
    if path == "_try_large_fn_regions":
        assert "repair_turn == 0" in body
    else:
        assert "w2f_repairs < 1" in body, path
        assert "w2f_repairs += 1" in body, path
