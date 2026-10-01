"""Both arms must be handed the same hot functions.

An ablation answers "what does the rule layer contribute". That question is
only asked if the two arms optimize the same targets. Locating separately per
arm does not give the same targets: profiled twice on one unchanged crate with
one unchanged workload, libxml2 produced 33 shared hot functions plus 5 seen
only in the first run and 6 only in the second, and `perf --call-graph dwarf`
inline attribution moved 17 points of self time between two of them
(`UTF8ToHtml` 25.9%→8.9%, `htmlEncodeEntities` 26.6%→45.5%, in the same
operation, almost exactly complementary).

A set difference of that size between arms would be attributed to the ablated
layer. So the ablation arm reads the full arm's `hotspots.json` verbatim.
"""

from __future__ import annotations

import json

import pytest

from perf_opt.hot_probe.types import HotFunction


def _fn(name: str, pct: float, **kw) -> HotFunction:
    return HotFunction(
        name=name, self_pct=pct, hottest_op=kw.pop("op", "op1"),
        per_op=kw.pop("per_op", {"op1": pct}), **kw,
    )


def test_round_trip_is_exact() -> None:
    original = _fn(
        "hot", 42.5, file="src/lib.rs", line_start=10, line_end=99,
        extern_wrapper=False, wrapper_target=None,
        class_i_hits={"C1": 2, "C3": True},
        class_ii_hits={"II_vec": {"r": 1}},
        fn_type="algorithm_hot", fn_type_reason="120 lines, 3 loops",
    )
    assert HotFunction.from_dict(original.to_dict()).to_dict() == original.to_dict()


def test_location_is_derived_not_stored_twice() -> None:
    """`to_dict` emits `location`; rebuilding must not choke on it."""
    d = _fn("hot", 1.0, file="src/a.rs", line_start=3, line_end=7).to_dict()
    assert d["location"] == "src/a.rs:3-7"
    back = HotFunction.from_dict(d)
    assert (back.file, back.line_start, back.line_end) == ("src/a.rs", 3, 7)


def test_unresolved_function_survives_the_round_trip() -> None:
    d = _fn("hot", 1.0).to_dict()
    assert d["location"] is None
    back = HotFunction.from_dict(d)
    assert back.file is None and back.line_start is None


def test_mutating_the_rebuilt_hits_does_not_touch_the_source_dict() -> None:
    """The arms must not share mutable state through the reused file."""
    src = _fn("hot", 1.0, class_i_hits={"C1": 1}, per_op={"op1": 1.0}).to_dict()
    back = HotFunction.from_dict(src)
    back.class_i_hits["C1"] = 999
    back.per_op["op1"] = 999.0
    assert src["class_i_hits"]["C1"] == 1
    assert src["per_op"]["op1"] == 1.0


def test_a_real_hotspots_file_rebuilds_identically(tmp_path) -> None:
    payload = {
        "crate": "x",
        "hot_functions": [
            _fn("a", 45.47, file="src/a.rs", line_start=1, line_end=50).to_dict(),
            _fn("b", 8.91, file="src/b.rs", line_start=9, line_end=20,
                extern_wrapper=True, wrapper_target="memcpy").to_dict(),
        ],
    }
    p = tmp_path / "hotspots.json"
    p.write_text(json.dumps(payload), encoding="utf-8")
    loaded = [HotFunction.from_dict(x)
              for x in json.loads(p.read_text())["hot_functions"]]
    assert [f.name for f in loaded] == ["a", "b"]
    assert [f.self_pct for f in loaded] == [45.47, 8.91]
    assert loaded[1].extern_wrapper and loaded[1].wrapper_target == "memcpy"


def test_ablation_refuses_to_run_without_the_full_arm() -> None:
    """No control group, no comparison — fail loudly rather than locate."""
    import inspect

    from perf_opt.agent_perf_opt import driver

    source = inspect.getsource(driver)
    assert 'arm != "full"' in source
    assert "needs the full arm's hot function set" in source
    # And the reuse must sit BEFORE the locate call it replaces.
    assert source.index('arm != "full"') < source.index("locate_hotspots(")


def test_the_freeform_phase_is_a_valid_attempt_phase() -> None:
    """The ablation arm records its attempts under phase "freeform".

    Left out of the whitelist, the arm raised `ValueError` from
    `make_trace_entry` the first time a candidate was rejected — after the
    model call, the build, and a 900/900 W1 replay had all succeeded. The
    run died on bookkeeping, having done the work.
    """
    from perf_opt.agent_perf_opt import reporting
    from perf_opt.agent_perf_opt.state import RewriteAttempt

    assert "freeform" in reporting.ATTEMPT_PHASES
    entry = reporting.make_trace_entry(
        1, "freeform", RewriteAttempt.ABSTAINED, "pre_abstain", None)
    assert entry["phase"] == "freeform"

    rejected = reporting.make_trace_entry(
        1, "freeform", RewriteAttempt.ABSTAINED,
        "rejected_w2_regress", "[op +1.23%] per_op_regress")
    assert rejected["terminal_status"] == "rejected_w2_regress"
