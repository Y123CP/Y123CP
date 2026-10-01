"""A reasoned decline is not the model dropping a rule on the floor.

`_reconcile_fired_rules` files any fired rule that was neither applied nor
skipped as `silently_dropped` — a contract violation. On an abstain the model
DID account for its rules, but in prose, in `extra["reason"]`, one level up
from where that function looks.

Measured, lodepng `advanceBits` on 2026-08-26 — one record, both fields:

    reason : "fn is < 30 lines and has no loop; C3 hoist has no leverage"
    skipped: [["C3", "silently_dropped"]]

The log printed "LLM silently dropped fired rules ['C3'] (contract violation
— no skip reason)" on the line after it printed the reason.

Corpus-wide the mislabel ran 8 of 10 flagged rules, every one a reasoned
decline: lz4 `XXH32_round` ("C3 no-leverage here — fn is <30 lines with no
loop"), libxml2 `xmlFileClose` ("III③ does not apply here—fflush/fclose are
external stdio"). `skipped_rules` feeds the paper's rule-fit numbers, so the
mislabel points "the cards do not fit" at the cards instead of at this gap.

The PLAN/EXECUTE path already folded its `abstained_rules` in. Only the
direct/SIMPLE path lacked it.
"""

from __future__ import annotations

import inspect

import pytest

from perf_opt.agent_perf_opt import agent
from perf_opt.agent_perf_opt.agent import (
    _account_abstain_reason,
    _reconcile_fired_rules,
    _rule_base,
)

REASON = "fn is < 30 lines and has no loop; C3 hoist has no leverage"


def _reconciled(fired, applied, skipped, reason):
    skipped = list(skipped)
    _account_abstain_reason(fired, applied, skipped, reason)
    return dict((r, why) for r, why in _reconcile_fired_rules(fired, applied, skipped))


# ────────────────────────────────── the record that started this

def test_the_advancebits_record_is_no_longer_a_violation() -> None:
    got = _reconciled(["C3"], [], [], REASON)
    assert got == {"C3": REASON}


def test_without_the_reason_it_is_still_a_violation() -> None:
    """An abstain that says nothing IS the contract breach the flag is for."""
    got = _reconciled(["C3"], [], [], None)
    assert got == {"C3": "silently_dropped"}


@pytest.mark.parametrize("reason", ["", "   "])
def test_an_empty_reason_is_not_an_account(reason) -> None:
    got = _reconciled(["C3"], [], [], reason)
    assert got.get("C3") == "silently_dropped" or got.get("C3") == "unspecified"


# ────────────────────────────────── it must not paper over real skips

def test_a_real_skip_reason_is_not_overwritten() -> None:
    got = _reconciled(["C3", "III④"], [], [("C3", "subsumed by III④")], REASON)
    assert got["C3"] == "subsumed by III④"
    assert got["III④"] == REASON


def test_an_applied_rule_is_not_given_a_skip_reason() -> None:
    got = _reconciled(["C3", "C6"], ["C3"], [], REASON)
    assert "C3" not in got
    assert got["C6"] == REASON


def test_sub_strategies_count_as_covering_their_base() -> None:
    """`C3.S2` applied means C3 was accounted for — the same base-name rule
    `_reconcile_fired_rules` already uses, so the backfill cannot disagree."""
    got = _reconciled(["C3"], ["C3.S2"], [], REASON)
    assert "C3" not in got


def test_each_unaccounted_rule_is_named_once() -> None:
    got = _reconciled(["C1", "C3", "III④"], [], [], REASON)
    assert got == {"C1": REASON, "C3": REASON, "III④": REASON}


def test_the_backfill_does_not_invent_rules() -> None:
    _skipped = []
    _account_abstain_reason([], [], _skipped, REASON)
    assert _skipped == []


def test_rule_base_is_shared_not_duplicated() -> None:
    assert _rule_base("C3.S2") == "C3"
    assert _rule_base("III④") == "III④"
    body = inspect.getsource(_reconcile_fired_rules)
    assert "_base = _rule_base" in body, "the two must not drift apart"


# ────────────────────────────────── wired into the direct/SIMPLE path

def test_the_direct_path_backfills_before_reconciling() -> None:
    """Called after `_reconcile_fired_rules`, the backfill would be too late:
    the flag is already set."""
    from perf_opt.agent_perf_opt import agent
    body = inspect.getsource(agent._try_fn_direct)
    assert "_account_abstain_reason(" in body
    assert body.index("_account_abstain_reason(") < body.index(
        "_reconcile_fired_rules(fired_rules, applied, skipped_raw)")


def test_only_an_abstain_gets_the_backfill() -> None:
    """A committed or rejected rewrite that drops a rule is still a
    violation — the reason field there describes something else."""
    from perf_opt.agent_perf_opt import agent
    body = inspect.getsource(agent._try_fn_direct)
    head = body[:body.index("_account_abstain_reason(")]
    assert "RewriteAttempt.ABSTAINED" in head.rsplit("if ", 1)[-1]


# ───────── a rule the run never asked about still needs a destination
#
# Two paths abstain before the model is consulted at all: the short-function
# filter, and a hot function fn_hits has no entry for. Neither produced a
# `skipped` entry, so their fired rules landed in `fired` and nowhere else —
# read back, they are indistinguishable from a rule the model dropped on the
# floor. Measured on libopenaptx: `aptx_bin_search` fired III④ and accounted
# for it nowhere, in both the old run and the new.

def test_short_fn_filter_accounts_for_every_fired_rule() -> None:
    skips = agent._short_fn_skips(["III④", "C3"], "19 lines, 1 hits")
    assert [r for r, _ in skips] == ["III④", "C3"]
    assert all("short_no_leverage" in why for _, why in skips)


def test_short_fn_filter_keeps_an_already_accounted_rule_once() -> None:
    """`_account_abstain_reason` normalises ids, so an annotated variant of a
    rule already listed must not be filed twice."""
    skips = agent._short_fn_skips(["C3", "C3"], "short")
    assert len(skips) == 1


def test_short_fn_filter_on_no_rules_is_empty() -> None:
    assert agent._short_fn_skips([], "short") == []


def test_the_short_fn_record_carries_the_skips() -> None:
    src = inspect.getsource(agent)
    marker = 'rule_id="pre_abstain_short_fn"'
    tail = src[src.index(marker):src.index(marker) + 1200]
    assert "skipped_rules=_short_fn_skips(" in tail, tail


def test_a_hot_fn_without_hits_is_written_to_the_audit_log() -> None:
    """It used to leave nothing behind but a logger line."""
    src = inspect.getsource(agent)
    marker = '"[agent] %s: no fn_hits entry, skip"'
    assert marker in src
    tail = src[src.index(marker):src.index(marker) + 700]
    assert "state.log(AttemptRecord(" in tail, tail
    assert "no_fn_hits_entry" in tail, tail


def test_a_skipped_hot_fn_does_not_consume_candidate_budget() -> None:
    """`result.add` raises `total_attempts`, which is what the loop tests
    against `max_candidates`. A function nothing was attempted on must not
    spend another function's turn."""
    src = inspect.getsource(agent)
    marker = '"[agent] %s: no fn_hits entry, skip"'
    tail = src[src.index(marker):src.index(marker) + 700]
    assert "result.add(" not in tail, tail
