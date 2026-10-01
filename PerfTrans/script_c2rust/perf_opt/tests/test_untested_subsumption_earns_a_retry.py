"""A rule dropped as "subsumed by X" is a free assertion that costs a rule.

Measured on one crate's hottest function — `match_row`, 69% of its operation,
a six-array DP row. The model declared `[III④]` and skipped C3 with the reason
"subsumed by III④". Four builds, output-identical, n=21, interleaved:

    C3 alone (hoist the loop-invariant load)          -4.123%
    the III④ rewrite that supposedly covered it       +3.100%

W2 rejected the bundle at +1.975% and the function shipped untouched. C3 had
fired on it (3 hits) and was never independently tried: it was not in
`applied_rules`, so the region path's decomposition could not see it, and the
COMPLEX path this candidate actually took has no decomposition at all.

The claim is never tested and nothing records that it was made. When the
rewrite that was supposed to subsume it then loses, the assertion has become
evidence against itself — that is the moment to spend one turn on the rule it
discarded.
"""

from __future__ import annotations

import inspect

import pytest

from perf_opt.agent_perf_opt.planners.llm_region import (
    _SUBSUMPTION_WORDS,
    find_untested_subsumption,
    subsumption_repair_note,
)


# ───────────────────────────────────────────── the detector

def test_the_measured_reason_is_recognised() -> None:
    """Verbatim from the transcript that cost -4.123%."""
    got = find_untested_subsumption([
        ("C3", "subsumed by III④"),
        ("C6", "no loop-invariant dispatch is present"),
    ])
    assert got == ("C3", "subsumed by III④")


def test_the_long_form_from_the_other_crate_is_recognised() -> None:
    """optipng, verbatim: the same claim at length."""
    reason = ("Subsumed by III④: once the parser uses a body-local byte "
              "slice/CStr with indexed access and per-iteration local byte "
              "values, the repeated raw-pointer loads disappear without "
              "needing a separate alias-gap hoist.")
    assert find_untested_subsumption([("C3", reason)]) == ("C3", reason)


@pytest.mark.parametrize("word", _SUBSUMPTION_WORDS)
def test_every_listed_phrasing_is_recognised(word) -> None:
    assert find_untested_subsumption([("C3", f"it is {word} the other rule")])


def test_a_substantive_reason_is_not_a_subsumption() -> None:
    """The point is not to distrust every skip. A reason that states a
    property of the CODE is the model doing its job."""
    for reason in [
        "the function is short and contains no loop",
        "hits are two standalone pointer writes, not a loop/cursor walk",
        "ownership escapes this function; RAII would ripple to callers",
        "no loop-invariant dispatch is present",
    ]:
        assert find_untested_subsumption([("C3", reason)]) is None, reason


def test_the_claim_must_lead_not_trail() -> None:
    """A reason that argues about the code and only then remarks on coverage
    is the model doing its job.

    Verbatim from three committed rewrites in three projects. Re-offering the
    rule against any of these buys an abstain and costs a build, a W1 and a W2.
    """
    for reason in [
        # zopfli
        "All reported cursor-index-deref sites are single/few scalar accesses "
        "in a branchy recursive function body, not a forward loop cursor to "
        "amortize slice construction. Rewriting to slice views here would add "
        "from_raw_parts/bounds machinery for one-off accesses. The profitable "
        "part of these sites is their repeated reloading, which is already "
        "covered by the hoist.",
        # lodepng
        "Abstain for hits [7,8]: these are not a forward cursor walk over a "
        "stable buffer but repeated access to the dynamically changing last "
        "byte of a reallocatable vector. size changes in the loop and resize "
        "may relocate the backing storage, so there is no stable slice region "
        "to establish once. A naive rewrite is largely subsumed by that.",
        # libxml2
        "Single cursor-deref hit [11] is not in a loop, and slice-ifying the "
        "destination would require proving stronger no-alias guarantees than "
        "the current memmove semantics imply. A local-hoist + ptr::copy "
        "rewrite captures the main benefit and subsumes this isolated write.",
    ]:
        assert find_untested_subsumption([("III④", reason)]) is None, reason[:60]


def test_the_bare_forms_from_four_projects_are_caught() -> None:
    """Swept over every committed skip declaration in the dataset: seven are
    a leading claim, in four different projects, and every one of them is C3
    or C1 being eaten by III④. Not one crate's quirk."""
    for reason in [
        "subsumed by III④",
        "Subsumed by III④'s iterator-based loop rewrite",
        "Likely subsumed by III④: once the loop nest is reborrowed into slices",
        "Subsumed by III④: once the parser uses a body-local byte slice",
        "Subsumed by III④ body-local slice reborrow: the missed GVN loads",
        "Subsumed by III④ + C10",
    ]:
        assert find_untested_subsumption([("C3", reason)]), reason


def test_no_skips_at_all_is_not_a_subsumption() -> None:
    assert find_untested_subsumption([]) is None
    assert find_untested_subsumption(None) is None


def test_the_first_one_wins_deterministically() -> None:
    got = find_untested_subsumption([("C1", "redundant with C3"),
                                     ("C3", "subsumed by III④")])
    assert got[0] == "C1"


# ───────────────────────────────────────────── the note

def _note() -> str:
    return subsumption_repair_note("C3", "subsumed by III④",
                                   "// Applied rules: [III④]\nfn f() {}")


def test_the_note_names_the_discarded_rule() -> None:
    assert "`C3`" in _note()


def test_the_note_quotes_the_claim_it_is_challenging() -> None:
    assert "subsumed by III④" in _note()


def test_the_note_says_apply_it_alone() -> None:
    """Re-offering the same bundle would just reproduce the rejection."""
    low = _note().lower()
    assert "alone" in low
    assert "do not apply the rules from the previous attempt" in low


def test_the_note_forbids_reproducing_the_rejected_rewrite() -> None:
    """Otherwise the model edits its own output instead of the original."""
    low = _note().lower()
    assert "start from the original function" in low


def test_the_note_keeps_the_output_contract() -> None:
    n = _note()
    assert "Applied rules:" in n and "Skipped rules:" in n


def test_the_note_is_bounded() -> None:
    assert len(subsumption_repair_note("C3", "x" * 5000, "y" * 9000)) < 3000


# ───────────────────────────────────────────── the loops are wired to it

_LLM_PATHS = ("_try_fn_direct", "_try_fn_plan_execute", "_try_large_fn_regions")


def _path_source(name: str) -> str:
    from perf_opt.agent_perf_opt import agent
    return inspect.getsource(getattr(agent, name))


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_every_llm_path_offers_the_retry(path) -> None:
    """`match_row` took the COMPLEX path, which had no decomposition at all —
    the region path's queue would never have seen it."""
    body = _path_source(path)
    assert "find_untested_subsumption(" in body, path
    assert "subsumption_repair_note(" in body, path


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_it_reads_the_reply_not_the_applied_set(path) -> None:
    """The discarded rule is in `Skipped rules:`, never in `applied_rules` —
    which is exactly why the existing decomposition could not rescue it."""
    body = _path_source(path)
    at = body.index("find_untested_subsumption(")
    assert "parse_skipped_rules(" in body[at:at + 200], path


@pytest.mark.parametrize("path", _LLM_PATHS)
def test_only_a_w2_rejection_triggers_it(path) -> None:
    """A skip is only suspect once the rewrite that justified it has lost."""
    body = _path_source(path)
    at = body.index("find_untested_subsumption(")
    assert "RewriteAttempt.W2_REGRESS" in body[at:at + 300], path


@pytest.mark.parametrize("path", ("_try_fn_direct", "_try_fn_plan_execute"))
def test_it_shares_the_one_w2_turn_budget(path) -> None:
    """A candidate gets at most ONE W2-triggered turn, whichever kind."""
    body = _path_source(path)
    at = body.index("find_untested_subsumption(")
    window = body[at - 200:at + 300]
    assert "w2f_repairs < 1" in window, path
    assert "not w2_finding" in window, path
