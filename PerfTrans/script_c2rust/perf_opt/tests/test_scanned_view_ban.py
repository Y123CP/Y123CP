"""A view that costs a scan to exist is not free abstraction.

`CStr::from_ptr(p).to_bytes()` reads the whole string to find the NUL before
it hands back a slice. Written at the top of a function, that scan is paid on
every call — and the loop it feeds usually stops after a few bytes.

Measured: the hottest function of one crate (99.85% of its operation's self
time) parses a short range expression from a C string. A rewrite opened with
`CStr::from_ptr(...).to_bytes_with_nul()` and measured **+41.12%** — the
single worst regression across every run of this pipeline. The rewrite was
correct, idiomatic, and W1-clean; it just added a full string scan to a
function that used to stop at the first few characters.

The rule cards cannot own this: the rewrite was booked under C3 with III④ in
the header, and any rule that turns a raw pointer into a safe view can reach
for it. So it lives in the contract every rewriting prompt carries.
"""

from __future__ import annotations

import re

from perf_opt.agent_perf_opt import prompt_builder as pb


def _contracts() -> list[str]:
    """Every system prompt that asks the model for rewritten code."""
    return [t for t in (pb.SYSTEM_PROMPT, pb.SYSTEM_PLAN,
                        *(v for k, v in vars(pb).items()
                          if k.startswith("SYSTEM") and isinstance(v, str)))
            if "```rust" in t or "rewritten fn" in t]


def test_every_rewriting_contract_carries_the_ban() -> None:
    contracts = _contracts()
    assert contracts, "no rewriting contract found — did the prompts move?"
    for text in contracts:
        assert "SCANNED" in text, text[:200]


def test_the_ban_names_the_concrete_api() -> None:
    """`CStr::from_ptr` is the form that actually appeared; naming it costs
    one line and makes the rule recognisable rather than abstract."""
    for text in _contracts():
        assert "CStr::from_ptr" in text


def test_the_ban_says_why_not_just_what() -> None:
    """A prohibition without its reason gets rationalised away — the model
    reads "this looks safe and clean" and ships it."""
    for text in _contracts():
        assert "EVERY call" in text


def test_the_contracts_stay_short() -> None:
    """Prompt budget is a real constraint: a truncated API list once cost a
    run 72% of its inventory. One clause, three lines."""
    for text in _contracts():
        clause = [l for l in text.split("\n") if "SCANNED" in l]
        assert len(clause) == 1
