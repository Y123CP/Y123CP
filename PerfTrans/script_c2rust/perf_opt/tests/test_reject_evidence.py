"""A rejection must record WHY, not just that it happened.

Measured on brotli: the hottest function in the project (UpdateNodes, 57% self
time) produced a rewrite that failed to compile, and the audit trail recorded

    {"changeset_id": "...", "status": "rejected_build", "detail": {}}

The compiler output existed — `build_callback` puts it in the gate record — and
was then dropped on the floor. Diagnosing the single most valuable failure of a
multi-hour run required re-running it.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.changeset.executor import (
    _REJECT_EXCERPT_LINES,
    _failure_excerpt,
)
from perf_opt.agent_perf_opt.changeset.types import GateResultRecord

CARGO_ERR = (
    "error[E0502]: cannot borrow `*s` as mutable because it is also borrowed\n"
    "   --> src/enc/backward_references_hq.rs:512:9\n"
    "    |\n"
    "512 |         nodes.offset(pos as isize),\n"
    "    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n"
)


def test_build_failure_keeps_the_compiler_error() -> None:
    gates = [
        GateResultRecord("build", False, {"stderr": CARGO_ERR}),
    ]
    detail = _failure_excerpt(gates)
    assert detail["gate"] == "build"
    assert "E0502" in detail["excerpt"]


def test_excerpt_starts_at_the_first_error_line() -> None:
    """The head of compiler output is the informative part."""
    detail = _failure_excerpt([GateResultRecord("build", False,
                                                {"stderr": CARGO_ERR})])
    assert detail["excerpt"].splitlines()[0].startswith("error[E0502]")


def test_excerpt_is_bounded_and_says_how_much_it_dropped() -> None:
    """cargo output can run to megabytes; the ledger must stay readable."""
    big = "\n".join(f"error: line {i}" for i in range(500))
    detail = _failure_excerpt([GateResultRecord("build", False,
                                                {"stderr": big})])
    assert len(detail["excerpt"].splitlines()) == _REJECT_EXCERPT_LINES
    assert detail["truncated_lines"] == 500 - _REJECT_EXCERPT_LINES


def test_blank_lines_do_not_eat_the_excerpt_budget() -> None:
    padded = "\n\n\n" + CARGO_ERR
    detail = _failure_excerpt([GateResultRecord("build", False,
                                                {"stderr": padded})])
    assert detail["excerpt"].splitlines()[0].startswith("error[E0502]")


@pytest.mark.parametrize("key", ["stderr", "reason", "detail", "message"])
def test_every_gate_kind_surfaces_its_own_evidence_field(key: str) -> None:
    """w1 reports `reason`, build reports `stderr`; both must be captured."""
    detail = _failure_excerpt([GateResultRecord("w1", False, {key: "boom"})])
    assert detail["excerpt"] == "boom"


def test_the_failing_gate_is_the_one_reported() -> None:
    """Earlier gates passed; the last failure is what rejected the changeset."""
    gates = [
        GateResultRecord("build", True, {"stderr": ""}),
        GateResultRecord("w1", True, {"reason": "pass"}),
        GateResultRecord("w2", False, {"reason": "per_op_regress on decode"}),
    ]
    detail = _failure_excerpt(gates)
    assert detail["gate"] == "w2"
    assert "per_op_regress" in detail["excerpt"]


def test_gate_that_failed_without_text_still_names_itself() -> None:
    detail = _failure_excerpt([GateResultRecord("build", False, {})])
    assert detail == {"gate": "build"}


def test_empty_string_is_not_mistaken_for_evidence() -> None:
    detail = _failure_excerpt([GateResultRecord("build", False,
                                                {"stderr": "   \n\n"})])
    assert detail == {"gate": "build"}


def test_no_failing_gate_yields_no_detail() -> None:
    """Rejections that never ran a gate (e.g. resolve-time) add nothing."""
    assert _failure_excerpt([]) == {}
    assert _failure_excerpt([GateResultRecord("build", True, {})]) == {}
