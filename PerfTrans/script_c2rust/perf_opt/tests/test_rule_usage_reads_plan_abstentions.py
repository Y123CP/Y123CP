"""A rule declined in `plan_json` is still a rule the model decided about.

`rule_usage` tabulates four counts per rule, and `skipped` is the one that
carries evidence about card fit: the model looked at the rule and said why it
did not apply. Two paths record that decision in two places. The direct and
region paths write `skipped_rules`. The PLAN/EXECUTE path writes
`plan_json.abstained_rules` and leaves `skipped_rules` null.

Reading only `skipped_rules` drops the second kind entirely — the rule raises
`fired` and then lands in neither `applied` nor `skipped`, so a reasoned
decline reads as a rule nobody decided anything about, and the rule looks
like a worse fit than the log says it was.

Measured, zopfli 2026-08-27: `BoundaryPM` (37.68% self, the run's second
hottest) declined C3 and III④ with a paragraph each, and neither reached the
tally.
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from tools.rule_usage import _plan_abstentions, collect

ZOPFLI = Path("/home/anonymous/artifact/PerfTrans/dataset_trans/zopfli"
              "/3_perf_opt.killed_20260827/rewrites.log")


def _rec(**kw):
    base = {"fn_name": "f", "fired_rules": [], "applied_rules": [],
            "skipped_rules": None, "plan_json": None,
            "terminal_status": "abstained"}
    base.update(kw)
    return base


def _plan(*rules):
    return {"applied_rules": [],
            "abstained_rules": [{"rule": r, "reason": f"no leverage: {r}"}
                                for r in rules]}


# ────────────────────────────── extracting the abstentions

def test_a_plan_abstention_is_found() -> None:
    got = _plan_abstentions(_rec(plan_json=_plan("C3", "III④")))
    assert [r for r, _ in got] == ["C3", "III④"]
    assert all(reason for _, reason in got)


@pytest.mark.parametrize("plan", [None, {}, {"abstained_rules": None},
                                  {"abstained_rules": []}, "not a dict", 7])
def test_a_missing_or_malformed_plan_yields_nothing(plan) -> None:
    assert _plan_abstentions(_rec(plan_json=plan)) == []


def test_debris_is_not_tabulated_as_a_rule() -> None:
    """Older logs carry prose where a rule id belongs; `_is_rule_id` is the
    same filter `skipped_rules` already applies."""
    plan = {"abstained_rules": [
        {"rule": "C3", "reason": "ok"},
        {"rule": "the loop was already hoisted by the previous rule", "reason": "x"},
        {"rule": "", "reason": "x"},
        "not a dict",
    ]}
    assert [r for r, _ in _plan_abstentions(_rec(plan_json=plan))] == ["C3"]


# ────────────────────────────── folded into the tally

def test_the_boundarypm_shape_now_counts_as_skipped() -> None:
    stats, _ = collect([_rec(fired_rules=["C3", "III④"],
                             applied_rules=None, skipped_rules=None,
                             plan_json=_plan("C3", "III④"))])
    assert stats["C3"].skipped == 1
    assert stats["III④"].skipped == 1
    assert stats["C3"].dropped == 0


def test_a_rule_declined_outright_lands_in_skipped() -> None:
    """Nothing applied, so nothing else can account for it."""
    stats, _ = collect([_rec(fired_rules=["C3", "III④"],
                             plan_json=_plan("C3", "III④"))])
    for rule in ("C3", "III④"):
        st = stats[rule]
        assert st.skipped == 1
        assert st.fired - st.applied - st.skipped - st.dropped == 0


def test_a_rule_counted_once_when_both_fields_carry_it() -> None:
    """The PLAN path fills `plan_json`; `parse_skipped_rules` may also have
    filed the same decline. Counting each would double the skip."""
    stats, _ = collect([_rec(fired_rules=["C3"],
                             skipped_rules=[["C3", "no loop"]],
                             plan_json=_plan("C3"))])
    assert stats["C3"].skipped == 1


def test_a_rule_can_be_applied_and_declined_in_one_attempt() -> None:
    """`applied` and `skipped` are NOT mutually exclusive, so an applied rule
    must not suppress a decline the model actually recorded.

    A rule covers several hits and the model can take some and decline the
    rest, or take the base strategy and decline a sub-strategy. Measured, 74
    of 741 records across 8 projects: `C3.S1` declined beside an applied
    `C3`, `C3(partial:[4,5,6,9,10])`, `III④@21,22`.

    Seeding the dedup set with the applied rules — which this first did —
    makes the plan path drop exactly these, while the `skipped_rules` path
    keeps counting them. Same decline, two answers, depending on which field
    the run happened to write.
    """
    stats, _ = collect([_rec(fired_rules=["C3"], applied_rules=["C3"],
                             plan_json=_plan("C3.S1"),
                             terminal_status="committed")])
    assert stats["C3"].applied == 1
    assert stats["C3"].skipped == 1


def test_the_two_fields_agree_on_an_applied_base() -> None:
    """The same record via `skipped_rules` must tally the same way."""
    from_plan, _ = collect([_rec(fired_rules=["C3"], applied_rules=["C3"],
                                 plan_json=_plan("C3.S1"),
                                 terminal_status="committed")])
    from_skipped, _ = collect([_rec(fired_rules=["C3"], applied_rules=["C3"],
                                    skipped_rules=[["C3.S1", "needs callers"]],
                                    terminal_status="committed")])
    assert from_plan["C3"].skipped == from_skipped["C3"].skipped


def test_silently_dropped_still_counts_as_a_violation() -> None:
    """The backfill must not launder a contract breach into a reasoned skip."""
    stats, _ = collect([_rec(fired_rules=["C3"],
                             skipped_rules=[["C3", "silently_dropped"]])])
    assert stats["C3"].dropped == 1
    assert stats["C3"].skipped == 0


# ────────────────────────────── against the log that motivated it

@pytest.mark.skipif(not ZOPFLI.is_file(), reason="run artifacts not on disk")
def test_the_gap_closes_on_the_measured_log() -> None:
    recs = [json.loads(l) for l in ZOPFLI.read_text().splitlines() if l.strip()]
    stats, _ = collect(recs)
    assert stats["III④"].fired - stats["III④"].applied \
        - stats["III④"].skipped - stats["III④"].dropped == 0


@pytest.mark.skipif(not ZOPFLI.is_file(), reason="run artifacts not on disk")
def test_c3_does_not_balance_and_that_is_correct() -> None:
    """`fired == applied + skipped + dropped` is not an invariant. Do not
    reintroduce it as one.

    `fired` is counted per rule the dispatcher offered — base names only.
    `applied` and `skipped` are counted per decision the model recorded, and
    it decides at sub-strategy and per-hit granularity. `ZopfliSublenToCache`
    applied `C3` (body-local hoist) and declined `C3.S1` (signature lift,
    "would require auditing all callers"): one offer, two decisions, both
    true, and C3 lands one below zero.

    That -1 used to be cancelled by `BoundaryPM`'s uncounted +1, so the
    column added up and looked right for the wrong reason. Counting
    BoundaryPM makes the granularity mismatch visible. It is not a defect,
    and closing it by suppressing one of the two decisions would delete
    evidence about card fit.
    """
    recs = [json.loads(l) for l in ZOPFLI.read_text().splitlines() if l.strip()]
    stats, _ = collect(recs)
    c3 = stats["C3"]
    assert c3.applied == 8 and c3.skipped == 7
    assert c3.fired - c3.applied - c3.skipped - c3.dropped == -1
