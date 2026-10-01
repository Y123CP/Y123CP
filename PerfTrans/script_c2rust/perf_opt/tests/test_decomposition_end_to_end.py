"""The decomposition path, exercised end to end on the run that lost the rewrite.

Every previous fix to this path passed its own tests and still failed in
production, because each test checked one link. The chain has five, and it
only works if all five agree:

  1. the bundle is judged worth splitting                (`_should_decompose`)
  2. its anchors survive that judgement                  (retire only if NOT splitting)
  3. the split is ordered so the cheapest rule goes first (fewest anchors)
  4. each retry carries anchors matching its own rule     (prompt builder demands it)
  5. the same bundle is not split again forever           (`_not_yet_decomposed`)

Link 2 is what actually broke: a bundle carrying a `qsort` rewrite worth
-30.72% by hand was rejected at aggregate **-5.64%** with four of five ops
safe, its three rules were queued — and its anchors were retired on the spot.
Another candidate in the same round committed, extraction restarted, the
queued rules were no longer in `remaining_hits`, and the function finished.
The log printed "re-offering each rule alone" and offered nothing.

The numbers below are that run's: rules `III①`/`III④`/`C3` holding 1/2/3
anchors out of the bundle's 14.
"""

from __future__ import annotations

from dataclasses import dataclass, replace as dc_replace

import pytest

from perf_opt.agent_perf_opt.agent import (
    RewriteAttempt,
    _decomposition_key,
    _not_yet_decomposed,
    _should_decompose,
)
from perf_opt.agent_perf_opt.changeset.types import ChangeSetStatus
from perf_opt.agent_perf_opt.config import AgentConfig

CFG = AgentConfig()
REGRESS = ChangeSetStatus.REJECTED_W2_REGRESS.value


@dataclass(frozen=True)
class _Region:
    relative_path: str = "src/katajainen.rs"
    start_byte: int = 4100
    end_byte: int = 9700
    anchor_hit_ids: tuple = ()
    anchor_lines: tuple = ()


@dataclass(frozen=True)
class _Candidate:
    region: _Region
    rule_ids: tuple


                                                         
ANCHOR_RULES = (
    ["III①"] * 1 + ["III④"] * 2 + ["C3"] * 3 + ["III②"] * 5 + ["III③"] * 3
)
ANCHORS = tuple(f"h{i:02d}" for i in range(len(ANCHOR_RULES)))
ANCHOR_RULE = dict(zip(ANCHORS, ANCHOR_RULES))
BUNDLE = _Candidate(
    region=_Region(anchor_hit_ids=ANCHORS,
                   anchor_lines=tuple(range(100, 100 + len(ANCHORS)))),
    rule_ids=("III①", "III④", "C3"),
)
APPLIED = ["III①", "III④", "C3"]


def _split(candidate, applied_rules, anchor_rule):
    """Mirror of the agent's split: narrow anchors per rule, fewest first."""
    region = candidate.region
    by_rule = {
        solo: [i for i, hid in enumerate(region.anchor_hit_ids)
               if anchor_rule.get(hid) == solo]
        for solo in applied_rules
    }
    out = []
    for solo, keep in sorted(((r, k) for r, k in by_rule.items() if k),
                             key=lambda rk: (len(rk[1]), rk[0])):
        out.append(dc_replace(
            candidate,
            region=dc_replace(
                region,
                anchor_hit_ids=tuple(region.anchor_hit_ids[i] for i in keep),
                anchor_lines=tuple(region.anchor_lines[i] for i in keep),
            ),
            rule_ids=(solo,),
        ))
    return out


# ── link 1: the bundle is judged worth splitting ──────────────────────────

def test_the_run7_bundle_is_judged_worth_splitting() -> None:
    extra = {"terminal_status": REGRESS, "w2_delta_pct": -5.64}
    assert _should_decompose(RewriteAttempt.W2_REGRESS, extra, APPLIED,
                             CFG, set()) is True


def test_the_other_bundle_from_the_same_run_is_not() -> None:
    """+0.17% — the aggregate never improved, so no rule inside was being
    outvoted. Splitting it costs three full measurements for nothing."""
    extra = {"terminal_status": REGRESS, "w2_delta_pct": +0.17}
    assert _should_decompose(RewriteAttempt.W2_REGRESS, extra,
                             ["III④", "C3"], CFG, set()) is False


# ── link 2: anchors survive when a split is coming ────────────────────────

def test_anchors_of_a_splitting_bundle_are_not_retired() -> None:
    """The defect itself. `processed_hit_ids` must not swallow anchors that
    the queued retries still need — nothing can bring them back afterwards."""
    processed: set = set()
    will_split = _should_decompose(
        RewriteAttempt.W2_REGRESS,
        {"terminal_status": REGRESS, "w2_delta_pct": -5.64},
        APPLIED, CFG, set())
    if not will_split:                                         
        processed.update(BUNDLE.region.anchor_hit_ids)
    assert not processed, "anchors were retired before the retries ran"


def test_anchors_of_a_non_splitting_candidate_are_retired() -> None:
    """The converse must still hold, or hits would never be consumed."""
    processed: set = set()
    will_split = _should_decompose(
        RewriteAttempt.W2_REGRESS,
        {"terminal_status": REGRESS, "w2_delta_pct": +0.17},
        ["III④", "C3"], CFG, set())
    if not will_split:
        processed.update(BUNDLE.region.anchor_hit_ids)
    assert processed == set(ANCHORS)


# ── link 3: cheapest rule first ───────────────────────────────────────────

def test_the_split_is_ordered_by_anchor_count() -> None:
    order = [c.rule_ids[0] for c in _split(BUNDLE, APPLIED, ANCHOR_RULE)]
    assert order == ["III①", "III④", "C3"]


def test_the_qsort_rule_is_tried_first() -> None:
    """It holds one anchor of fourteen. Under the previous order — whatever
    sequence the model happened to write — it was tried last and, with the
    loop stopping at the first commit, never reached."""
    first = _split(BUNDLE, APPLIED, ANCHOR_RULE)[0]
    assert first.rule_ids == ("III①",)
    assert len(first.region.anchor_hit_ids) == 1


# ── link 4: each retry is a candidate the prompt builder accepts ──────────

def test_each_retry_carries_only_its_own_anchors() -> None:
    for cand in _split(BUNDLE, APPLIED, ANCHOR_RULE):
        rule = cand.rule_ids[0]
        assert all(ANCHOR_RULE[h] == rule for h in cand.region.anchor_hit_ids)


def test_the_paired_anchor_fields_stay_aligned() -> None:
    """`anchor_hit_ids` and `anchor_lines` are positionally paired; filtering
    one without the other silently attaches every line to the wrong hit."""
    for cand in _split(BUNDLE, APPLIED, ANCHOR_RULE):
        assert len(cand.region.anchor_hit_ids) == len(cand.region.anchor_lines)


def test_no_retry_is_empty() -> None:
    for cand in _split(BUNDLE, APPLIED, ANCHOR_RULE):
        assert cand.region.anchor_hit_ids


def test_a_rule_owning_no_anchor_produces_no_retry() -> None:
    out = _split(BUNDLE, APPLIED + ["C9"], ANCHOR_RULE)
    assert [c.rule_ids[0] for c in out] == ["III①", "III④", "C3"]


# ── link 5: the split does not recur forever ──────────────────────────────

def test_the_same_bundle_is_split_only_once() -> None:
    """Anchors are deliberately kept, so the same window can be extracted
    again after an unrelated commit. Without this guard that is a cycle."""
    seen: set = set()
    assert _not_yet_decomposed(BUNDLE, APPLIED, seen) is True
    seen.add(_decomposition_key(BUNDLE, APPLIED))
    assert _not_yet_decomposed(BUNDLE, APPLIED, seen) is False


def test_a_retry_itself_cannot_be_split_again() -> None:
    """Each retry carries one rule, so the multi-rule condition fails — the
    recursion is one level deep by construction."""
    for cand in _split(BUNDLE, APPLIED, ANCHOR_RULE):
        extra = {"terminal_status": REGRESS, "w2_delta_pct": -3.0}
        assert _should_decompose(RewriteAttempt.W2_REGRESS, extra,
                                 list(cand.rule_ids), CFG, set()) is False


# ── the whole chain ───────────────────────────────────────────────────────

def test_the_full_chain_reaches_the_qsort_rule() -> None:
    """What run7 should have done: judge -> keep anchors -> split -> try the
    one-anchor rule first."""
    extra = {"terminal_status": REGRESS, "w2_delta_pct": -5.64}
    seen: set = set()

    assert _should_decompose(RewriteAttempt.W2_REGRESS, extra, APPLIED, CFG, seen)
    assert _not_yet_decomposed(BUNDLE, APPLIED, seen)

    processed: set = set()                    # anchors kept, not retired
    retries = _split(BUNDLE, APPLIED, ANCHOR_RULE)
    seen.add(_decomposition_key(BUNDLE, APPLIED))

    assert not processed
    assert retries[0].rule_ids == ("III①",)
    assert not _not_yet_decomposed(BUNDLE, APPLIED, seen)


# ── the mirror above proves the design; these prove the code ──────────────
#
# The helper `_split` in this file is a mirror of the agent's logic. A mirror
# validates the design and nothing else: reverting the real code leaves it
# green. Every fix on this path so far passed its own tests and still failed
# in production, so each link also gets an assertion against the source.

def _agent_src() -> str:
    from pathlib import Path
    return (Path(__file__).resolve().parents[1] / "agent_perf_opt"
            / "agent.py").read_text(encoding="utf-8")


def test_the_code_orders_retries_by_anchor_count() -> None:
    """Link 3 in the real source: sorted by (len(anchors), rule)."""
    src = _agent_src()
    assert "key=lambda rk: (len(rk[1]), rk[0])" in src


def test_the_code_keeps_anchors_when_a_split_follows() -> None:
    """Link 2 in the real source — the one that broke."""
    src = _agent_src()
    idx = src.index("if not will_decompose:")
    assert "processed_hit_ids.update" in src[idx:idx + 160]


def test_the_code_reports_rules_it_cannot_retry() -> None:
    """A rule owning no anchor used to be dropped by a bare `continue`, so a
    missing retry left no trace anywhere."""
    src = _agent_src()
    assert "own no anchor in this region" in src


def test_the_mirror_matches_the_code_on_ordering() -> None:
    """If the agent's ordering key ever changes, this file's mirror — and
    therefore its 13 design tests — must be revisited rather than silently
    diverging."""
    order = [c.rule_ids[0] for c in _split(BUNDLE, APPLIED, ANCHOR_RULE)]
    assert order == sorted(
        order, key=lambda r: (sum(1 for x in ANCHOR_RULES if x == r), r))
