"""F9 strips rules the model declared but did not execute. When it strips
*every* one of them the remainder is an empty list, and an empty list is
falsy — so `executed or declared` hands the declaration back and the strip
silently undoes itself.

That is the one case that matters most. A rewrite where some rules landed
still carries the ones that did; a rewrite where none landed is a rewrite
that did nothing the rules describe, and it is exactly then that the record
claimed otherwise. Observed on a fixed-length sub-word loop: the model
declared C10, wrote eight hand-unrolled raw-pointer stores instead of the
word-wide store, F9 saw no fingerprint and stripped it — and the log still
read `applied=['C10']`, next to `skipped=[C10: no_fingerprint(F9 stripped)]`.
"""
from __future__ import annotations

from pathlib import Path
from types import SimpleNamespace

import pytest

from test_attempt_reporting import (  # type: ignore[import-not-found]
    RewriteAttempt,
    _import_agent,
    _run_direct_trace,
)


def test_stripping_every_rule_leaves_applied_empty(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """`parse_applied_rules` is patched to return ["C1"]; F9 strips it. The
    record must say nothing was applied, not repeat the declaration."""
    agent = _import_agent(monkeypatch)

    record = _run_direct_trace(
        agent,
        tmp_path,
        monkeypatch,
        [
            (
                RewriteAttempt.APPLIED_COMMITTED,
                {
                    "terminal_status": "committed",
                    "commit_sha": "abc123",
                    "executed_applied_rules": [],
                    "stripped_rules": ["C1"],
                },
            ),
        ],
    )

    assert record.applied_rules == [], (
        "F9 stripped every declared rule, so the record must not name any as "
        f"applied — got {record.applied_rules}"
    )
    # the record normalises pairs to lists on the way through
    assert [list(p) for p in (record.skipped_rules or [])] == [
        ["C1", "no_fingerprint(F9 stripped)"]
    ], f"the stripped rule must appear in the skip pool — got {record.skipped_rules}"


def test_a_partial_strip_keeps_what_executed(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """The non-empty case must keep working: what survived the strip is what
    the record reports."""
    agent = _import_agent(monkeypatch)
    monkeypatch.setattr(agent, "parse_applied_rules", lambda response: ["C1", "C10"])

    record = _run_direct_trace(
        agent,
        tmp_path,
        monkeypatch,
        [
            (
                RewriteAttempt.APPLIED_COMMITTED,
                {
                    "terminal_status": "committed",
                    "commit_sha": "abc123",
                    "executed_applied_rules": ["C1"],
                    "stripped_rules": ["C10"],
                },
            ),
        ],
    )

    assert record.applied_rules == ["C1"]


def test_no_strip_at_all_still_falls_back_to_the_declaration(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """When the key is absent — no F9 pass ran — the declaration is still the
    source. Absent and empty must not be conflated in either direction."""
    agent = _import_agent(monkeypatch)

    record = _run_direct_trace(
        agent,
        tmp_path,
        monkeypatch,
        [
            (
                RewriteAttempt.APPLIED_COMMITTED,
                {"terminal_status": "committed", "commit_sha": "abc123"},
            ),
        ],
    )

    assert record.applied_rules == ["C1"]
