"""A no-gain bundle must not bury the rule that would have paid.

W2 accepts or rejects a whole changeset, so a bundle's verdict is a verdict on
its SUM. One rule that costs more than it saves therefore discards the ones
that would have paid, and the function ships with no rewrite at all.

Observed on the same function in two runs, with the same rules available:
one run proposed the loop-invariant hoist alone → committed, measurably faster;
the other bundled it with a pointer→slice rewrite whose bounds checks survived
in a masked ring-buffer scan → W2 measured no gain → BOTH were dropped. The
difference in outcome was purely which subset the model happened to bundle.
"""

from __future__ import annotations

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
NO_GAIN = ChangeSetStatus.REJECTED_W2_NO_GAIN.value


def _extra(delta, status=REGRESS):
    return {"terminal_status": status, "w2_delta_pct": delta}


def _decide(delta, *, rules=("III①", "III④", "C3"), status=REGRESS,
            attempt=RewriteAttempt.W2_REGRESS):
    return _should_decompose(attempt, _extra(delta, status), list(rules),
                             CFG, set())


# ───────────────────────────── only rescue what is worth rescuing

def test_an_improving_bundle_vetoed_by_one_op_is_decomposed() -> None:
    """The measured case: aggregate -5.64%, four of five ops safe, vetoed by
    an op that profiles at 0.0 library share. A profitable rule is in there
    being outvoted."""
    assert _decide(-5.64) is True


def test_a_bundle_that_never_improved_is_not_decomposed() -> None:
    """Nothing to rescue, and each retry costs a full build + W1 + W2.
    Measured alongside the case above: +0.17%, split anyway, wasted."""
    assert _decide(+0.17) is False


def test_a_large_improvement_still_qualifies() -> None:
    """An earlier version excluded aggregates below -5% as "suspicious". The
    measured case sits at -5.64%, so that bound excluded precisely the bundle
    it was meant to rescue. There is no reason a bigger improvement should
    disqualify a split."""
    assert _decide(-5.64) is True
    assert _decide(-30.0) is True


def test_a_single_rule_bundle_has_nothing_to_split() -> None:
    assert _decide(-5.64, rules=("C3",)) is False


def test_an_unmeasured_delta_is_not_decomposed() -> None:
    """`None` means no number. Splitting on it would spend three measurements
    on a guess."""
    assert _decide(None) is False


def test_a_committed_candidate_is_never_decomposed() -> None:
    assert _decide(-5.64, attempt=RewriteAttempt.APPLIED_COMMITTED) is False


def test_only_a_w2_regress_rejection_qualifies() -> None:
    """`no_gain` means the aggregate did not improve — same reasoning as
    `+0.17%` above."""
    assert _decide(-5.64, status=NO_GAIN) is False


# ───────────────────────────── the anti-recycle guard

class _Region:
    relative_path = "src/k.rs"
    start_byte = 100
    end_byte = 400


class _Cand:
    region = _Region()


def test_the_same_bundle_is_not_split_twice() -> None:
    """Anchors of a bundle awaiting decomposition stay in `remaining_hits` on
    purpose, so a later extraction can rebuild the same window. Without this
    guard that becomes a cycle."""
    seen = set()
    cand, rules = _Cand(), ["III①", "III④"]
    assert _not_yet_decomposed(cand, rules, seen) is True
    seen.add(_decomposition_key(cand, rules))
    assert _not_yet_decomposed(cand, rules, seen) is False


def test_a_different_rule_set_on_the_same_region_still_qualifies() -> None:
    seen = {_decomposition_key(_Cand(), ["III①", "III④"])}
    assert _not_yet_decomposed(_Cand(), ["C3"], seen) is True


def test_the_key_ignores_rule_order() -> None:
    a = _decomposition_key(_Cand(), ["III①", "C3"])
    b = _decomposition_key(_Cand(), ["C3", "III①"])
    assert a == b


# ───────────────────────────── anchors must survive until the retry runs

def test_anchors_are_retired_only_when_no_decomposition_follows() -> None:
    """The defect this file exists for: anchors were retired unconditionally,
    so a queued retry could never find its hits again."""
    from pathlib import Path
    src = (Path(__file__).resolve().parents[1] / "agent_perf_opt"
           / "agent.py").read_text(encoding="utf-8")
    assert "if not will_decompose:" in src
    idx = src.index("if not will_decompose:")
    tail = src[idx:idx + 200]
    assert "processed_hit_ids.update" in tail


def test_the_two_decisions_share_one_predicate() -> None:
    """Retiring anchors and queueing retries must agree. Three separate gates
    disagreeing on the same question is how the previous defect shipped."""
    from pathlib import Path
    src = (Path(__file__).resolve().parents[1] / "agent_perf_opt"
           / "agent.py").read_text(encoding="utf-8")
    assert src.count("_should_decompose(") == 3                                 
