"""A planner abstain must leave a trace.

An abstain happens BEFORE the changeset ledger, so it writes no
`changesets/<id>/` directory, and its reason is carried only in an
`AttemptRecord` — which is never persisted. The run summary can therefore say
`abstain=8` and nothing else about eight attempts.

Measured on brotli: 21 attempts produced 13 changesets, and the missing eight
included `UpdateNodes` at 57% self time — the single biggest lever in the
project. Its LLM output had applied three rules and compiled cleanly when
replayed by hand, yet the run left no record of why it was dropped.
"""

from __future__ import annotations

import logging

import pytest

from perf_opt.agent_perf_opt import agent, reporting


def _abstain(caplog, **kw):
    defaults = dict(fn_name="UpdateNodes", kind="region",
                    rule_ids=["C1", "C3"], reason="region hash drifted")
    defaults.update(kw)
    with caplog.at_level(logging.WARNING, logger=agent.logger.name):
        return agent._planner_abstained(**defaults)


def test_abstain_is_logged_with_fn_rules_and_reason(caplog) -> None:
    _abstain(caplog)
    text = caplog.text
    assert "UpdateNodes" in text
    assert "region" in text
    assert "C1,C3" in text
    assert "region hash drifted" in text


def test_abstain_is_logged_at_warning_not_below(caplog) -> None:
    """INFO would be filtered out of the run log where this matters."""
    _abstain(caplog)
    assert [r.levelno for r in caplog.records] == [logging.WARNING]


def test_payload_is_unchanged_by_the_logging(caplog) -> None:
    """The fix must add visibility, not alter the attempt record."""
    extra = _abstain(caplog)
    assert extra == {"reason": "region hash drifted",
                     "terminal_status": reporting.PLANNER_ABSTAINED}


def test_missing_reason_still_names_the_function(caplog) -> None:
    """A planner that abstains without a reason is exactly the case that used
    to vanish completely; it must still say which fn was dropped."""
    extra = _abstain(caplog, reason=None)
    assert "UpdateNodes" in caplog.text
    assert extra["terminal_status"] == reporting.PLANNER_ABSTAINED


def test_no_rules_does_not_produce_an_empty_field(caplog) -> None:
    _abstain(caplog, rule_ids=[])
    assert "rules=-" in caplog.text


@pytest.mark.parametrize("kind", ["whole-fn", "region", "II_const", "C9",
                                  "cross-fn"])
def test_every_planner_path_is_covered(kind: str, caplog) -> None:
    """All five abstain sites route through here, so each must render."""
    _abstain(caplog, kind=kind)
    assert kind in caplog.text


def test_all_five_call_sites_are_wired(caplog) -> None:
    """Guard against a new planner being added with the old silent return.

    Checked by line position rather than by a literal string: the helper now
    picks between PLANNER_ABSTAINED and REJECTED_FORM, so any test that
    matched one exact spelling would go quietly green the next time that
    line is edited — which is how the sites drift apart in the first place.
    """
    import ast
    import inspect

    src = inspect.getsource(agent)
    assert src.count("_planner_abstained(") >= 6      # helper + 5 call sites

    tree = ast.parse(src)
    helper = next(n for n in tree.body
                  if isinstance(n, ast.FunctionDef)
                  and n.name == "_planner_abstained")
    body = range(helper.lineno, helper.end_lineno + 1)
    outside = [
        i for i, line in enumerate(src.splitlines(), start=1)
        if '"terminal_status"' in line and "PLANNER_ABSTAINED" in line
        and i not in body
    ]
    assert not outside, (
        f"lines {outside} set terminal_status to PLANNER_ABSTAINED outside "
        "_planner_abstained — route them through the helper so the abstain "
        "is logged and a mechanical refusal is not misfiled as a judgment")
