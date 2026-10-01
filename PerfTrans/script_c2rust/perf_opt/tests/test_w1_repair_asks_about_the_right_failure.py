"""A W1 failure is not one thing, and the repair turn must not treat it as one.

Over the corpus, W1 rejected 41 candidates. 22 of them died on a signal —
16 segfaults, 6 aborts — against 19 that ran to completion and printed the
wrong bytes. The repair note asked all 41 the same four questions, and those
four are about byte equivalence: hand-derived constants, endianness, the
tail of a widened loop, off-by-one.

None of that applies to a rewrite that segfaulted, and the model behaved
accordingly: handed the byte-equivalence checklist for `exit -11`, it worked
through the list, found nothing wrong (correctly), and returned its previous
reply byte for byte — 1634 chars in, 1634 chars out, in two independent runs
of the same crate.
"""

from __future__ import annotations

import subprocess
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.planners.llm_region import (
    classify_w1_failure,
    quote_previous,
    w1_repair_note,
)


# ───────────────────────────────────────────────── the four detail shapes

@pytest.mark.parametrize("detail,kind", [
    ("op:abc123: exit -11 ≠ 0", "crash"),
    ("op:abc123: exit -4 ≠ 0", "crash"),
    ("op:abc123: exit 139 ≠ 0", "crash"),
    ("op:abc123: exit 101 ≠ 0", "crash"),
    ("op:abc123: exit -6 ≠ 0", "abort"),
    ("op:abc123: exit 134 ≠ 0", "abort"),
    ("op:abc123: sha256 deadbeef1234… ≠ cafebabe5678…", "mismatch"),
    ("op:abc123: W1 timeout", "timeout"),
    ("op:abc123: missing /path/to/binary", "mismatch"),
])
def test_each_detail_shape_is_classified(detail: str, kind: str) -> None:
    assert classify_w1_failure(detail) == kind


def test_an_exit_code_that_is_not_a_number_does_not_raise() -> None:
    assert classify_w1_failure("op: exit ??? ≠ 0") == "mismatch"


def test_the_stderr_suffix_does_not_change_the_classification() -> None:
    """`Verifier.w1` now appends the failed run's stderr, on its own lines.
    A panic message mentioning "sha256" must not turn a crash into a
    mismatch, so only the first line decides."""
    detail = ("op:abc: exit -11 ≠ 0\n"
              "stderr: thread 'main' panicked: sha256 mismatch in helper")
    assert classify_w1_failure(detail) == "crash"


# ─────────────────────────────────────────── the note follows the finding

_BYTE_EQUIVALENCE_MARKERS = ("Byte order", "to_le_bytes", "256 values")
# one marker per checklist item, so dropping an item fails here
_MEMORY_SAFETY_MARKERS = ("early return and guard", "from_raw_parts",
                          "realloc", "malloc_usable_size")


def test_a_crash_is_asked_about_memory_not_about_endianness() -> None:
    note = w1_repair_note("op:abc: exit -11 ≠ 0", "fn f() {}")
    assert "CRASHED" in note
    for marker in _MEMORY_SAFETY_MARKERS:
        assert marker in note, marker
    for marker in _BYTE_EQUIVALENCE_MARKERS:
        assert marker not in note, marker


def test_a_mismatch_is_still_asked_about_byte_equivalence() -> None:
    note = w1_repair_note("op:abc: sha256 aaa… ≠ bbb…", "fn f() {}")
    assert "changed the program's output" in note
    for marker in _BYTE_EQUIVALENCE_MARKERS:
        assert marker in note, marker


def test_an_abort_is_told_the_damage_came_earlier() -> None:
    """SIGABRT is glibc finding its own bookkeeping wrecked, which means the
    offending write already happened somewhere upstream — a different thing
    to look for than the faulting access a segfault reports."""
    note = w1_repair_note("op:abc: exit -6 ≠ 0", "fn f() {}")
    assert "abort" in note
    assert "allocator" in note
    # still gets the shared memory-safety checklist
    assert "from_raw_parts" in note


def test_a_timeout_is_asked_about_termination() -> None:
    note = w1_repair_note("op:abc: W1 timeout", "fn f() {}")
    assert "never finished" in note
    assert "terminate" in note
    for marker in _BYTE_EQUIVALENCE_MARKERS:
        assert marker not in note, marker


@pytest.mark.parametrize("detail", [
    "op:abc: exit -11 ≠ 0",
    "op:abc: exit -6 ≠ 0",
    "op:abc: sha256 aaa… ≠ bbb…",
    "op:abc: W1 timeout",
])
def test_every_shape_keeps_the_output_contract_and_the_reply(detail) -> None:
    reply = "// Applied rules: [C10]\nfn f() {}"
    note = w1_repair_note(detail, reply)
    assert "Applied rules:" in note and "Skipped rules:" in note
    assert reply in note
    assert "do not start over with a different one" in note


def test_the_stderr_the_gate_captured_reaches_the_model() -> None:
    """The point of capturing it: an abort names the violation, and a panic
    names the file, the line, and the index."""
    detail = ("op:abc: exit -6 ≠ 0\n"
              "stderr: free(): invalid pointer")
    assert "free(): invalid pointer" in w1_repair_note(detail, "fn f() {}")


# ─────────────────────────────────────────────────── quoting the reply

def test_a_short_reply_is_quoted_whole_with_no_warning() -> None:
    reply = "fn f() { 1 }"
    quoted = quote_previous(reply)
    assert reply in quoted
    assert "cut off" not in quoted


def test_a_reply_past_the_runaway_guard_says_so() -> None:
    """If it ever is cut, the model is told — the failure mode being fixed
    here is a model that cannot tell truncated code from code it wrote."""
    quoted = quote_previous("y" * 30_000)
    assert "cut off" in quoted
    assert "last line is incomplete" in quoted


def test_the_guard_is_far_above_any_reply_seen() -> None:
    """The longest reply ever quoted across the corpus was 11554 chars. The
    cap that used to sit here was 2000."""
    from perf_opt.agent_perf_opt.planners import llm_region
    assert llm_region._PREVIOUS_REPLY_CHARS >= 20_000


# ──────────────────────────────────────── no repair note truncates a reply

_NOTE_SOURCES = (
    Path(__file__).resolve().parents[1] / "agent_perf_opt" / "planners" / "llm_region.py",
    Path(__file__).resolve().parents[1] / "agent_perf_opt" / "agent.py",
)


def test_no_repair_note_slices_the_previous_reply_by_hand() -> None:
    """One `[:N]` on a previous reply is how this bug got in, in six places.
    `quote_previous` is the only place allowed to bound one, and
    `subsumption_repair_note` is the documented exception — it tells the
    model to start over from the original, so its quote is an excerpt on
    purpose."""
    import re
    offenders = []
    for path in _NOTE_SOURCES:
        src = path.read_text(encoding="utf-8")
        for m in re.finditer(r"(prev_output|previous)\[:\s*([\w]+)\s*\]", src):
            if m.group(2) in ("_PREVIOUS_REPLY_CHARS",):
                continue
            line = src[:m.start()].count("\n") + 1
            offenders.append(f"{path.name}:{line} {m.group(0)}")
    assert offenders == [
        f"llm_region.py:{_subsumption_excerpt_line()} previous[:1200]"
    ], offenders


def _subsumption_excerpt_line() -> int:
    src = _NOTE_SOURCES[0].read_text(encoding="utf-8")
    for i, line in enumerate(src.splitlines(), 1):
        if "previous[:1200]" in line:
            return i
    raise AssertionError("the documented exception is gone — update this test")


# ────────────────────────────────── the gate has to capture it in the first place

def _fake_run(returncode: int, stdout: bytes, stderr: bytes):
    class _Proc:
        pass
    proc = _Proc()
    proc.returncode, proc.stdout, proc.stderr = returncode, stdout, stderr
    return proc


def _w1_detail(monkeypatch, tmp_path, *, returncode: int, stderr: bytes) -> str:
    """Run `Verifier.w1` against a stubbed subprocess and return its detail."""
    from perf_opt.verify import cargo as cargo_mod

    binary = tmp_path / "target" / "release" / "prog"
    binary.parent.mkdir(parents=True)
    binary.write_bytes(b"")
    monkeypatch.setattr(
        cargo_mod.subprocess, "run",
        lambda *a, **k: _fake_run(returncode, b"", stderr))
    verifier = cargo_mod.Verifier(tmp_path)
    result = verifier.w1(binary="prog", args=[], input_path=tmp_path / "in",
                         expected_sha256="0" * 64)
    assert not result.ok
    return result.detail


def test_a_crashing_run_carries_its_stderr_into_the_detail(monkeypatch, tmp_path):
    """This is the whole point of the capture: `exit -6` alone says a
    violation happened, and glibc already said which one."""
    detail = _w1_detail(monkeypatch, tmp_path, returncode=-6,
                        stderr=b"free(): invalid pointer\n")
    assert "exit -6" in detail
    assert "free(): invalid pointer" in detail
    assert classify_w1_failure(detail) == "abort"


def test_the_exit_code_stays_on_the_first_line(monkeypatch, tmp_path):
    """`gates.w1_gate` splits the detail on its first colon to recover the op
    name, and `classify_w1_failure` reads only the first line. A multi-line
    stderr must not disturb either."""
    detail = _w1_detail(monkeypatch, tmp_path, returncode=-11,
                        stderr=b"line one\nline two\nline three\n")
    assert detail.splitlines()[0] == "exit -11 ≠ 0"
    assert classify_w1_failure(detail) == "crash"


def test_a_huge_stderr_is_tailed_not_headed(monkeypatch, tmp_path):
    """A panic prints last, after whatever the workload logged on the way."""
    noise = b"log line\n" * 5000
    detail = _w1_detail(monkeypatch, tmp_path, returncode=101,
                        stderr=noise + b"thread 'main' panicked: index 9 of 4")
    assert "panicked: index 9 of 4" in detail
    assert len(detail) < 1000


def test_a_silent_crash_still_produces_the_old_detail(monkeypatch, tmp_path):
    """No stderr, no suffix — a segfault usually writes nothing itself."""
    detail = _w1_detail(monkeypatch, tmp_path, returncode=-11, stderr=b"")
    assert detail == "exit -11 ≠ 0"

# ────────────────────────────────────────── the prose stays inside its budget

@pytest.mark.parametrize("detail,max_chars", [
    ("op: exit -11 ≠ 0", 1800),
    ("op: exit -6 ≠ 0", 2100),
    ("op: sha256 a… ≠ b…", 1400),
    ("op: W1 timeout", 1100),
])
def test_each_checklist_stays_inside_its_budget(detail, max_chars) -> None:
    """Splitting one checklist into four is an invitation to write four long
    ones. The ceiling exists because prompt bloat has a measured price: 750
    tokens of extra prose once turned a committed iterator form into a
    while-index regression worth 6.7 points. The crash list earns more room
    than the mismatch list — it replaced four questions that did not apply —
    but not unlimited room, and the numbers here are ~15% above what each
    currently costs, so a real addition has to be argued for rather than
    slipped in.
    """
    assert len(w1_repair_note(detail, "")) < max_chars


def test_the_crash_list_is_not_the_longest_thing_in_the_prompt() -> None:
    """Sanity floor: the note is a nudge appended to a prompt that already
    carries the function, its hits, and its cards."""
    assert len(w1_repair_note("op: exit -11 ≠ 0", "")) < 2000


# ────────────────── the guard question comes first, and it comes first for a reason

def test_the_first_question_is_about_the_originals_guards() -> None:
    """Measured, on the rewrite this whole checklist came from.

    The original function opened with `if v.is_null() || (*v).l == 0 { return
    1 }`. The rewrite dropped it, and `(*v).d` is null exactly when `l == 0`
    (the crate's own allocator sets both), so `ptr::copy` read from a null
    pointer. The prompt showed the guard — it was not truncated, both runs
    carried it — and the model still deleted it. A c2rust body is mostly
    guards; a rewrite that tidies it drops one.

    The first version of this checklist asked four questions about what the
    rewrite had ADDED and none about what it had REMOVED. The model went
    looking for something it had added, found `malloc_usable_size` — the only
    thing matching any question — and abandoned a sound -4.88% optimisation on
    a provenance claim that was false: every assignment to that field in the
    crate comes from `malloc`.
    """
    note = w1_repair_note("op: exit -11 ≠ 0", "fn f() {}")
    body = note[note.index("Check in this order"):]
    guard_pos = body.index("early return and guard")
    probe_pos = body.index("malloc_usable_size")
    assert guard_pos < probe_pos, "the guard question must be asked first"
    assert body.index("1. ") < guard_pos < body.index("2. ")


def test_the_guard_question_is_not_tied_to_one_api() -> None:
    """The old wording hid this question inside `from_raw_parts(p, 0)`. The
    rewrite that needed it used raw pointers and `ptr::copy` — no slice, no
    `from_raw_parts` — so the question never reached it."""
    note = w1_repair_note("op: exit -11 ≠ 0", "fn f() {}")
    first = note[note.index("1. "):note.index("2. ")]
    assert "from_raw_parts" not in first
    assert "null" in first and "zero-length" in first


def test_the_probe_question_does_not_offer_an_exit() -> None:
    """It used to end 'If the rewrite needs a capacity, track one' — read as
    'this approach is unsound, take another'. It now demands the provenance
    actually be traced before the optimisation is dropped."""
    note = w1_repair_note("op: exit -11 ≠ 0", "fn f() {}")
    fourth = note[note.index("4. "):]
    assert "Only if none of those explain it" in fourth
    assert "trace where the pointer is assigned" in fourth
    assert "Do not abandon the optimisation over a provenance worry" in fourth
