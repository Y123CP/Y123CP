"""A build that ran out of clock is not a rewrite that failed to compile.

`cargo_check` reports both through one `(ok, stderr)` channel, so a timeout
arrived at the agent wearing a compile error's clothes. What followed was the
full syntax-error repair path: a turn spent telling the model to "fix the
compile error", with `cargo build timeout after 120s` pasted in where the
stderr goes.

Measured, lodepng `update_adler32` on 2026-08-26:

    01:29:50  [build] timeout after 120s
    01:29:51  rejected_build at gate=build: cargo build timeout after 120s
    01:29:51  COMPLEX execute syntax_error attempt 1/3, retrying with stderr
    ...
    01:32:10  rejected_w2_regress   +7.947%      <- the run's worst regression

The transcript for that turn carries the prompt verbatim:

    ## Previous attempt failed (syntax_error / cargo build)
    cargo stderr:
    ```
    cargo build timeout after 120s
    ```

The model had nothing to fix, so it edited working code. Two things follow
from that: the build is retried once before the timeout is believed at all
(the second build of that identical source succeeded), and a timeout that
survives the retry ends the attempt instead of buying an LLM turn.
"""

from __future__ import annotations

import subprocess
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.rewrite_applier import (
    BUILD_TIMEOUT_PREFIX,
    cargo_check,
    is_build_timeout,
)


# ────────────────────────────────────── telling the two failures apart

def test_a_timeout_is_recognised() -> None:
    assert is_build_timeout(f"{BUILD_TIMEOUT_PREFIX} after 120s, twice")


def test_a_real_compile_error_is_not_a_timeout() -> None:
    assert not is_build_timeout(
        "error[E0425]: cannot find value `linebytes` in this scope")


@pytest.mark.parametrize("stderr", [None, "", "   "])
def test_nothing_to_read_is_not_a_timeout(stderr) -> None:
    assert not is_build_timeout(stderr)


def test_a_compile_error_merely_mentioning_the_word_is_not_a_timeout() -> None:
    """The mark leads the string; a `timeout` identifier in someone's source
    must not be mistaken for one."""
    assert not is_build_timeout(
        "error[E0599]: no method named `cargo build timeout` found")


# ──────────────────────────────────────── the rebuild before believing it

class _Clock:
    """`subprocess.run` that times out for the first `n` calls."""

    def __init__(self, timeouts: int, returncode: int = 0) -> None:
        self.timeouts = timeouts
        self.returncode = returncode
        self.calls = 0

    def __call__(self, *args, **kwargs):
        self.calls += 1
        if self.calls <= self.timeouts:
            raise subprocess.TimeoutExpired(cmd="cargo", timeout=120)
        return subprocess.CompletedProcess(
            args=args[0] if args else [], returncode=self.returncode,
            stdout="", stderr="" if self.returncode == 0 else "error[E0308]")


def test_one_timeout_is_rebuilt_and_the_rewrite_survives(monkeypatch, tmp_path) -> None:
    """The case that actually happened: identical source, second build fine."""
    clock = _Clock(timeouts=1)
    monkeypatch.setattr(subprocess, "run", clock)
    ok, stderr = cargo_check(tmp_path, timeout_s=120)
    assert clock.calls == 2
    assert ok and stderr == ""


def test_two_timeouts_report_the_clock_not_the_compiler(monkeypatch, tmp_path) -> None:
    clock = _Clock(timeouts=2)
    monkeypatch.setattr(subprocess, "run", clock)
    ok, stderr = cargo_check(tmp_path, timeout_s=120)
    assert clock.calls == 2, "the rebuild is one extra build, not a loop"
    assert not ok and is_build_timeout(stderr)


def test_a_real_compile_error_is_never_rebuilt(monkeypatch, tmp_path) -> None:
    """The retry exists for the clock. A rewrite that does not compile must
    not cost a second build to learn that twice."""
    clock = _Clock(timeouts=0, returncode=101)
    monkeypatch.setattr(subprocess, "run", clock)
    ok, stderr = cargo_check(tmp_path, timeout_s=120)
    assert clock.calls == 1
    assert not ok and not is_build_timeout(stderr)


# ─────────────────────────── neither retry path spends a turn on the clock

@pytest.mark.parametrize("marker", [
    # the direct / SIMPLE loop
    "and contract_reason is None and pv_code is None",
    # the COMPLEX PLAN/EXECUTE loop
    "and pv_code is None\n                and not w1_reason",
])
def test_both_retry_loops_exempt_a_timeout(marker) -> None:
    """Both loops break on `SYNTAX_ERROR` when the build merely timed out.

    Asserted on the source because the condition is inline in a loop that
    needs an LLM to reach. If either loop is rewritten, this fails and asks
    for the exemption to be carried across rather than dropped.
    """
    source = (Path(__file__).resolve().parents[1]
              / "agent_perf_opt" / "agent.py").read_text(encoding="utf-8")
    assert marker in source, "retry loop moved — recheck the timeout exemption"
    before = source.split(marker)[0]
    guard = before.rsplit("build_timed_out = ", 1)
    assert len(guard) == 2, "no timeout exemption ahead of this retry loop"
    assert "is_build_timeout" in guard[1]
    assert "status != RewriteAttempt.SYNTAX_ERROR or build_timed_out" in guard[1]
