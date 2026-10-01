"""A compile error is the one failure region mode refused to repair.

Region mode carries a repair turn for post-validation findings, for W1, for
W2 findings and for untested subsumption — every class that leaves a definite
signal. It had none for the class the compiler diagnoses by file and line,
and `_should_decompose` does not cover it either: that path only splits
bundles W2 rejected.

Measured, zopfli 2026-08-27. `EncodeTree` (7.73% self) converted a variable
to a slice under III④ and left four `.offset()` calls behind:

    error[E0599]: no method named `offset` found for reference `&[u32]`
       --> src/deflate.rs:240:25     (and 242, 253, 255)

The candidate ended there. Region mode is where the hottest functions go —
that run routed its five hottest through it.

The second half of the story is what the retry would have been shown. Cargo
writes far more lines for a warning than for an error, and a warning's `-->`
locations look exactly like an error's to a line-wise filter. The old filter
kept `"error" in ln.lower() or "-->" in ln` and then took the last 40 lines:
on that same build, 15 location lines survived and the 5 error headings did
not. Selecting diagnostics by block fixes what the model gets told.
"""

from __future__ import annotations

import inspect
import re

import pytest

from perf_opt.agent_perf_opt import agent
from perf_opt.agent_perf_opt.agent import _truncate_stderr
from perf_opt.agent_perf_opt.rewrite_applier import cargo_error_blocks
from perf_opt.agent_perf_opt.planners.llm_region import build_repair_note
from perf_opt.agent_perf_opt.state import RewriteAttempt

# The shape cargo actually emits: a warning whose location lines outnumber
# the error's, and the error that stopped the build.
CARGO_STDERR = """\
warning: unused import: `foo`
 --> src/a.rs:3:5
  |
3 | use foo;
  |     ^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `n`
 --> src/a.rs:9:9
  |
9 |     let n = 1;
  |         ^

error[E0599]: no method named `offset` found for reference `&[u32]`
   --> src/deflate.rs:240:25
    |
240 |     let x = arr.offset(1);
    |                 ^^^^^^

error: could not compile `zopfli_cleaned` (lib) due to 1 previous error; 2 warnings emitted
"""


# ────────────────────────────── selecting diagnostics by block

def test_the_error_survives_and_the_warnings_do_not() -> None:
    out = cargo_error_blocks(CARGO_STDERR)
    assert "E0599" in out
    assert "unused import" not in out
    assert "unused variable" not in out


def test_an_error_keeps_its_own_location_and_excerpt() -> None:
    """The `-->` line is what tells the model WHERE. Dropping the whole
    block to keep the heading would trade one mutilation for another."""
    out = cargo_error_blocks(CARGO_STDERR)
    assert "src/deflate.rs:240:25" in out
    assert "arr.offset(1)" in out


def test_a_warning_location_is_not_kept() -> None:
    out = cargo_error_blocks(CARGO_STDERR)
    assert "src/a.rs:3:5" not in out
    assert "src/a.rs:9:9" not in out


def test_every_error_heading_survives() -> None:
    """The real case had four identical headings at four different lines.
    Keeping one and dropping three leaves the retry three errors short."""
    many = "\n\n".join(
        f"error[E0599]: no method named `offset` found\n"
        f"   --> src/deflate.rs:{line}:25"
        for line in (240, 242, 253, 255)
    )
    out = cargo_error_blocks("warning: x\n --> src/a.rs:1:1\n\n" + many)
    for line in (240, 242, 253, 255):
        assert f"src/deflate.rs:{line}:25" in out
    assert out.count("E0599") == 4


def test_no_error_block_means_no_opinion() -> None:
    """Warnings only, or output that is not cargo's, returns "" so the
    caller keeps its own fallback rather than being handed an empty string
    where a diagnosis belongs."""
    assert cargo_error_blocks("warning: x\n --> a.rs:1:1") == ""
    assert cargo_error_blocks("") == ""
    assert cargo_error_blocks("w1 replay: record 42 mismatch") == ""


@pytest.mark.parametrize("heading", [
    "error: could not compile `x`",
    "error[E0277]: mismatched types",
    "error[E0599]: no method named `offset`",
])
def test_heading_forms_cargo_uses(heading) -> None:
    assert cargo_error_blocks(heading + "\n   --> a.rs:1:1")


def test_an_indented_error_word_does_not_open_a_block() -> None:
    """`error` inside a source excerpt or a note is not a new diagnostic.
    Matching it would re-open a block the warning owns."""
    out = cargo_error_blocks(
        "warning: unused\n"
        " --> src/a.rs:1:1\n"
        "  = note: error recovery is not a diagnostic\n"
    )
    assert out == ""


# ────────────────────────────── what the retry prompt is built from

def test_truncate_selects_blocks_before_trimming() -> None:
    got = _truncate_stderr(CARGO_STDERR)
    assert "E0599" in got
    assert "unused import" not in got


def test_truncate_leaves_non_cargo_text_alone() -> None:
    """A W1 replay reason comes through the same helper."""
    reason = "w1 replay: record 42 stdout mismatch at byte 17"
    assert _truncate_stderr(reason) == reason


def test_truncate_still_bounds_a_pathological_error() -> None:
    huge = "error[E0599]: x\n" + "\n".join(
        f"   --> src/a.rs:{i}:1" for i in range(2000))
    got = _truncate_stderr(huge)
    assert len(got) < len(huge)
    # The heading survives and the cut is declared. Keeping the heading is the
    # point of trimming forwards rather than backwards: it names the error.
    assert got.startswith("error[E0599]: x")
    assert "truncated" in got


def test_truncate_keeps_the_early_errors_and_counts_the_rest() -> None:
    """A build fails with more diagnostics than the budget holds.

    What the model must not receive is a subset formatted as if it were
    everything — measured on a libxml2 retry, 22 errors reported and 5
    delivered with no notice, so the retry fixed those 5 and the next build
    failed on the 17 it had never been shown.
    """
    blocks = [
        f"error[E0308]: mismatched types\n"
        f"    --> src/a.rs:{i}:9\n"
        f"     |\n"
        f"     = help: {'x' * 300}"
        for i in range(40)
    ]
    tally = "error: could not compile `c` (lib) due to 40 previous errors"
    got = _truncate_stderr("\n".join(blocks + [tally]))

    assert got.startswith("error[E0308]")          # earliest errors kept
    assert tally in got                            # cargo's own count kept
    omitted = re.search(r"\[(\d+) more error diagnostic", got)
    assert omitted, "an excerpt must say how much it left out"
    kept = got.count("error[E0308]")
    assert kept + int(omitted.group(1)) == 40      # nothing vanishes silently


def test_re_excerpting_does_not_understate_what_was_dropped() -> None:
    """cargo_check excerpts once and the retry path excerpts again.

    The second pass must carry the first pass's count forward. Left as an
    ordinary block, the notice sorted last among the droppable ones and the
    second trim replaced "30 omitted" with its own "1 omitted".
    """
    blocks = [
        f"error[E0308]: mismatched types\n"
        f"    --> src/a.rs:{i}:9\n"
        f"     = help: {'x' * 300}"
        for i in range(60)
    ]
    tally = "error: could not compile `c` (lib) due to 60 previous errors"
    once = _truncate_stderr("\n".join(blocks + [tally]))
    twice = _truncate_stderr(once)
    thrice = _truncate_stderr(twice)

    assert twice == thrice, "excerpting is idempotent once it fits"
    for text in (once, twice):
        omitted = int(re.search(r"\[(\d+) more error diagnostic",
                                text).group(1))
        assert text.count("error[E0308]") + omitted == 60


def test_the_note_carries_the_error_and_the_previous_reply() -> None:
    note = build_repair_note("error[E0599]: no method named `offset`",
                             "let x = arr.offset(1);")
    assert "E0599" in note
    assert "arr.offset(1)" in note
    # It must not read as permission to propose something else.
    assert "do not" in note.lower()


def test_the_note_names_the_cause_the_measurement_found() -> None:
    """Four of one run's five build failures were a leftover pointer call on
    a binding the rewrite had just turned into a slice."""
    note = build_repair_note("error", "prev")
    assert ".offset()" in note or ".add()" in note


# ────────────────────────────── wired into region mode

def _region_src() -> str:
    return inspect.getsource(agent._try_large_fn_regions)


def test_region_mode_repairs_a_build_failure() -> None:
    src = _region_src()
    assert "RewriteAttempt.SYNTAX_ERROR" in src, (
        "region mode ends the candidate on a compile error")
    assert "build_repair_note(" in src


def test_the_build_branch_is_inside_the_repair_turn_guard() -> None:
    """Outside the `repair_turn == 0` guard it would retry forever; after
    the loop it would never run at all."""
    src = _region_src()
    guard = src.index("if repair_turn == 0")
    branch = src.index("RewriteAttempt.SYNTAX_ERROR")
    closing = src.index("            break\n", guard)
    assert guard < branch < closing


def test_the_repair_prefers_the_unabridged_stderr() -> None:
    """`extra` carries both: `error` is already trimmed, `cargo_stderr_full`
    is not. Repairing from the trimmed copy discards diagnostics twice."""
    src = _region_src()
    tail = src[src.index("RewriteAttempt.SYNTAX_ERROR"):][:600]
    assert "cargo_stderr_full" in tail
    assert tail.index("cargo_stderr_full") < tail.index('"error"')


def test_the_repair_also_offers_name_corrections() -> None:
    """The direct path pairs every build retry with this; an unresolved name
    is the one error replaying the stderr cannot fix."""
    tail = _region_src()
    tail = tail[tail.index("RewriteAttempt.SYNTAX_ERROR"):][:600]
    assert "name_correction_note(" in tail


def test_a_build_failure_matches_no_other_repair_condition() -> None:
    """The four pre-existing branches read fields that are only populated on
    their own status, which is why the failure fell through to `break`."""
    assert RewriteAttempt.SYNTAX_ERROR is not RewriteAttempt.W1_FAIL
    assert RewriteAttempt.SYNTAX_ERROR is not RewriteAttempt.W2_REGRESS
    assert RewriteAttempt.SYNTAX_ERROR is not RewriteAttempt.ABSTAINED
    src = _region_src()
    for guarded, status in (
        ("pv_code = ", "RewriteAttempt.ABSTAINED"),
        ("w1_reason = ", "RewriteAttempt.W1_FAIL"),
        ("w2_finding = ", "RewriteAttempt.W2_REGRESS"),
    ):
        line = src[src.index(guarded):][:220]
        assert status in line, guarded


def test_decompose_does_not_cover_a_build_failure() -> None:
    """The other second chance in region mode is bundle decomposition, and
    its first test excludes anything that is not a W2 rejection."""
    src = inspect.getsource(agent._should_decompose)
    assert "REJECTED_W2_REGRESS" in src
