"""One broken operation must not cost the whole harness.

A repair rewrites the ENTIRE lib.rs, so every round spent on a stubborn op
destabilises the ops that already work — and when the global budget runs out,
the run reverts the whole coverage extension, discarding every good op to save
none. That is the most destructive possible response to one bad op.

Measured on a 1603-function crate: 7 of 13 LLM calls went to 4 ops (two of them
repeat offenders, one consuming 3 rounds and changing nothing), and the revert
then threw away 3 working ops — including the only op in four runs that reached
the crate's main parser.
"""

from __future__ import annotations

import pytest

import stat
from pathlib import Path

from harness_gen import gates
from harness_gen.gates import DROPPED_KEY, failing_op, mark_dropped


# ───────────────────────────────────────────── identifying the single culprit

@pytest.mark.parametrize("feedback,expected", [
    ("op `uri_roundtrip` on its seed exited -11.\nstderr:\n…", "uri_roundtrip"),
    ("op `globals_threads_memory` is NON-DETERMINISTIC: two identical …",
     "globals_threads_memory"),
    ("op `html_parse_dump` CRASHES on the perf input gen_perf wrote for it",
     "html_parse_dump"),
    ("op `x` printed nothing; the digest line is required", "x"),
    ("op `y`: seed.1 and seed.2 give an identical digest", "y"),
    ("op `z` with iters=3 exited 1 — the runner must…", "z"),
])
def test_per_op_failures_name_their_op(feedback, expected) -> None:
    assert failing_op(feedback) == expected


@pytest.mark.parametrize("feedback", [
    "error[E0432]: unresolved import `x::y`",
    "`harness gen-perf` exited 101 — gen_perf crashes/hangs",
    "gen_perf produced no `<op>.perf.bin` files.",
    "warning: unused import",
    "",
])
def test_whole_harness_failures_name_no_op(feedback) -> None:
    """A build error or a gen_perf-wide crash is NOT one op's fault; dropping
    an op would not fix it, so these must keep using the normal repair path."""
    assert failing_op(feedback) is None


def test_leading_whitespace_does_not_hide_the_op() -> None:
    assert failing_op("\n  op `a_b9` is NON-DETERMINISTIC") == "a_b9"


def test_an_op_mentioned_later_is_not_taken_as_the_culprit() -> None:
    """Only the message's subject counts — other ops appear in stderr dumps."""
    assert failing_op("error: boom\nop `other` was fine") is None


# ─────────────────────────────────────────────────────── the drop policy

def _spec(*names, adopted=()):
    return {"harness_name": "h",
            "operations": [{"name": n, "adopted": n in adopted} for n in names]}


def _run_policy(failures, spec, max_op_repairs=1, max_repairs=6):
    """Mirror of the agent's loop bookkeeping, over a scripted failure list."""
    op_failures: dict[str, int] = {}
    repairs = 0
    dropped, repaired = [], []
    for feedback in failures:
        culprit = failing_op(feedback)
        entry = next((o for o in spec["operations"]
                      if o["name"] == culprit), None) if culprit else None
        if entry is not None:
            allowance = 0 if entry.get("adopted") else max_op_repairs
            op_failures[culprit] = op_failures.get(culprit, 0) + 1
            if (op_failures[culprit] > allowance
                    and len(spec["operations"]) > 1):
                mark_dropped(spec, culprit)
                dropped.append(culprit)
                continue
        repairs += 1
        if repairs > max_repairs:
            return dropped, repaired, "exhausted"
        repaired.append(culprit or "build")
    return dropped, repaired, "ok"


def test_a_repeat_offender_is_dropped_not_repaired_again() -> None:
    """Realistic sequence: the gate reports it, the repair does not fix it, the
    gate reports it again — and it leaves. Once out of the spec it is out of
    the smoke gate too, so it cannot be reported a third time."""
    spec = _spec("good1", "good2", "bad")
    dropped, repaired, status = _run_policy(
        ["op `bad` on its seed exited -11"] * 2, spec)
    assert dropped == ["bad"]
    assert repaired == ["bad"]          # exactly one retry, then dropped
    assert status == "ok"
    assert [o["name"] for o in spec["operations"]] == ["good1", "good2"]


def test_a_dropped_op_is_never_dropped_twice() -> None:
    """Defensive: a stale message naming an already-removed op must not be
    treated as a second removal."""
    spec = _spec("keep", "bad")
    dropped, _repaired, _status = _run_policy(
        ["op `bad` is NON-DETERMINISTIC"] * 5, spec)
    assert dropped == ["bad"]
    assert [o["name"] for o in spec["operations"]] == ["keep"]


def test_the_scenario_that_lost_three_working_ops() -> None:
    """Two repeat offenders used to exhaust the budget and revert everything."""
    spec = _spec("a", "b", "c", "uri", "entities")
    failures = ["op `uri` on its seed exited -11",
                "op `uri` on its seed exited -11",
                "op `entities` on its seed exited -6",
                "op `entities` on its seed exited -6"]
    dropped, _repaired, status = _run_policy(failures, spec)
    assert status == "ok", "budget must survive two stubborn ops"
    assert dropped == ["uri", "entities"]
    assert [o["name"] for o in spec["operations"]] == ["a", "b", "c"]


def test_the_scenario_that_lost_six_working_ops() -> None:
    """The real shape of the damage, and the one a same-op counter misses.

    A coverage round adopted nine ops at once; three were broken, each in a
    different way and each ONCE. Under a per-op retry allowance none of them
    was ever a 'repeat offender', so all three took a repair round, the budget
    ran out, and the revert discarded the batch — including the six ops that
    worked and covered subsystems nothing else reached.
    """
    good = ["pattern_regex_unicode", "schema_relaxng_schematron",
            "xinclude_c14n_save", "writer_buf_valid", "encoding_and_strings",
            "html_api_exercise"]
    bad = ["dict_hash_list", "globals_threads_memory", "catalog_io_module"]
    spec = _spec("planned_main", *good, *bad, adopted=tuple(good + bad))
    failures = [
        "op `dict_hash_list` on its seed exited -6",
        "op `globals_threads_memory` is NON-DETERMINISTIC: two identical …",
        "op `catalog_io_module` on its seed exited -11",
    ]
    dropped, repaired, status = _run_policy(failures, spec)
    assert status == "ok"
    assert dropped == bad
    assert repaired == [], "a bonus op must not cost a repair round at all"
    survivors = [o["name"] for o in spec["operations"]]
    assert survivors == ["planned_main"] + good


def test_a_planned_op_still_earns_a_retry() -> None:
    """The plan's ops are the reason the harness exists — a transient failure
    in one must not silently shrink the workload to nothing."""
    spec = _spec("main_parse", "other", adopted=())
    dropped, repaired, _ = _run_policy(
        ["op `main_parse` on its seed exited -11"], spec)
    assert dropped == []
    assert repaired == ["main_parse"]
    dropped, _r, _s = _run_policy(
        ["op `main_parse` on its seed exited -11"] * 2, _spec("main_parse", "x"))
    assert dropped == ["main_parse"], "but not forever"


def test_distinct_ops_each_get_their_own_retry() -> None:
    """Dropping is per-op, so one failure each must not drop anything."""
    spec = _spec("a", "b", "c")
    dropped, repaired, _ = _run_policy(
        ["op `a` is NON-DETERMINISTIC", "op `b` is NON-DETERMINISTIC"], spec)
    assert dropped == []
    assert repaired == ["a", "b"]


def test_build_failures_still_consume_repairs() -> None:
    """They are not attributable to an op; the normal path must still run."""
    spec = _spec("a", "b")
    dropped, repaired, _ = _run_policy(["error[E0308]: mismatched types"] * 3,
                                       spec)
    assert dropped == []
    assert repaired == ["build"] * 3


def test_the_last_op_is_never_dropped() -> None:
    """Dropping down to an empty harness would be worse than failing loudly."""
    spec = _spec("only")
    dropped, _repaired, status = _run_policy(
        ["op `only` on its seed exited -11"] * 8, spec)
    assert dropped == []
    assert status == "exhausted"
    assert [o["name"] for o in spec["operations"]] == ["only"]


# ─────────────────────────── a drop must survive the smoke gate's self-heal
#
# The two mechanisms collided in production. The smoke gate rebuilds the spec
# from the seeds the harness emits, so an op the loop drops — its code still in
# lib.rs, its seed still written — is adopted straight back on the very next
# gate run. Dropping is deliberately free of repair budget, so nothing bounded
# the cycle: one op went drop→re-adopt 249 times in 20 minutes of a live run
# before the process was killed by hand.

def _fake_harness(tmp_path: Path, ops) -> Path:
    """A stand-in binary that only implements `gen-seeds`.

    Enough to drive the self-heal, which is the subject here; the per-op checks
    that follow it are free to fail. What matters is what the gate did to the
    spec on its way there.
    """
    hdir = tmp_path / "harness"
    bin_path = hdir / "target" / "debug" / "harness"
    bin_path.parent.mkdir(parents=True)
    emit = "\n".join(
        f'printf a > "$2/{op}.1.bin"; printf bb > "$2/{op}.2.bin"' for op in ops)
    bin_path.write_text(f'#!/bin/sh\nif [ "$1" = gen-seeds ]; then\n{emit}\n'
                        f'exit 0\nfi\nexit 1\n', encoding="utf-8")
    bin_path.chmod(bin_path.stat().st_mode | stat.S_IEXEC | stat.S_IXGRP)
    return hdir


def _names(spec):
    return [o["name"] for o in spec["operations"]]


def test_the_smoke_gate_adopts_ops_it_finds_seeds_for(tmp_path) -> None:
    """Precondition for the regression below — this behaviour is wanted: a
    coverage round adds ops to lib.rs without re-emitting the spec."""
    hdir = _fake_harness(tmp_path, ["planned", "bonus"])
    spec = {"harness_name": "h", "operations": [{"name": "planned"}]}
    gates.gate_smoke(hdir, spec)
    assert _names(spec) == ["planned", "bonus"]


def test_a_dropped_op_is_not_adopted_back(tmp_path) -> None:
    hdir = _fake_harness(tmp_path, ["planned", "bonus"])
    spec = {"harness_name": "h",
            "operations": [{"name": "planned"}, {"name": "bonus"}]}
    mark_dropped(spec, "bonus")
    gates.gate_smoke(hdir, spec)
    assert _names(spec) == ["planned"]


def test_re_adoption_does_not_come_back_on_a_later_round(tmp_path) -> None:
    """The loop re-gates after every drop, so once is not enough."""
    hdir = _fake_harness(tmp_path, ["planned", "bonus"])
    spec = {"harness_name": "h",
            "operations": [{"name": "planned"}, {"name": "bonus"}]}
    mark_dropped(spec, "bonus")
    for _ in range(5):
        gates.gate_smoke(hdir, spec)
        assert _names(spec) == ["planned"]


def test_dropping_is_recorded_in_the_spec_not_out_of_band(tmp_path) -> None:
    """The spec travels to the perf loop and to perf_refine, which run the
    same gate; a caller-side set would not protect those."""
    spec = {"harness_name": "h",
            "operations": [{"name": "a"}, {"name": "b"}]}
    mark_dropped(spec, "b")
    assert spec[DROPPED_KEY] == ["b"]
    import json
    assert json.loads(json.dumps(spec))[DROPPED_KEY] == ["b"]


def test_marking_twice_records_once(tmp_path) -> None:
    spec = {"harness_name": "h", "operations": [{"name": "a"}, {"name": "b"}]}
    mark_dropped(spec, "b")
    mark_dropped(spec, "b")
    assert spec[DROPPED_KEY] == ["b"]


def test_a_deliberately_redesigned_op_is_not_blocked(tmp_path) -> None:
    """A later coverage round may write the op afresh. The block is only on
    SILENT re-adoption from a seed file — an op listed in the spec is honoured,
    or the harness could never recover from one bad generation."""
    hdir = _fake_harness(tmp_path, ["planned", "bonus"])
    spec = {"harness_name": "h", "operations": [{"name": "planned"}]}
    mark_dropped(spec, "bonus")
    spec["operations"].append({"name": "bonus", "needs_input": True})
    gates.gate_smoke(hdir, spec)
    assert _names(spec) == ["planned", "bonus"]
    assert sum(1 for n in _names(spec) if n == "bonus") == 1


def test_the_drop_loop_terminates(tmp_path) -> None:
    """The termination argument, exercised: every free drop removes one op from
    a finite set, and the only path that puts one back is now closed."""
    ops = ["good", "bad1", "bad2", "bad3"]
    hdir = _fake_harness(tmp_path, ops)
    spec = {"harness_name": "h", "operations": [{"name": n} for n in ops]}
    rounds = 0
    while rounds < 50:
        rounds += 1
        gates.gate_smoke(hdir, spec)
        stuck = [n for n in _names(spec) if n.startswith("bad")]
        if not stuck:
            break
        if len(spec["operations"]) > 1:
            mark_dropped(spec, stuck[0])
    assert _names(spec) == ["good"]
    assert rounds <= len(ops), f"took {rounds} rounds for {len(ops)} ops"
