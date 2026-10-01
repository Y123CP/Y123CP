"""An operation that barely runs the library cannot speak for it.

The gates give every perf op a single-op veto: one op regressing past its
ceiling sinks the candidate. That is right for an op the crate under test
actually runs, and wrong for one whose time is the harness's own — there, what
moved is allocator behaviour and code layout, not the rewrite.

So an op below a library-share floor keeps its place in the aggregate (its
time is real time) but loses the veto. Everything then rests on measuring that
share correctly, and there is exactly one way to get it wrong.

**It must come from an inline-expanded profile.** LTO absorbs library
functions into the harness's own op function and leaves them no symbol at all,
so `perf report --sort symbol` charges every one of their samples to the
harness. Profiled both ways on the same binary, same input, same iteration
count, one operation came out:

    symbol-level (`perf report --sort dso,symbol`)        0.0% library
    inline-expanded (`--call-graph dwarf` + `--inline`)   49.1% library

64% of its samples sat in one harness symbol the library had been inlined
into. Believing the first number strips a half-library operation of its veto
on the grounds that the crate does not run in it. The same artefact, on
another crate: an operation whose timed loop contains nothing but library
calls profiled at 99.1% harness / 0% library (see test_inline_attribution).

`locate` already profiles the right way and writes the result to
`hotspots.json`. The gates read that, and nothing else.
"""

from __future__ import annotations

import json

import pytest

from perf_opt.agent_perf_opt.config import AgentConfig
from perf_opt.agent_perf_opt.gates import (
    _LIBRARY_SHARE_FLOOR,
    _measured_library_share,
    W2Session,
)


class _Session:
    bystander_op_upper_limit_pct = AgentConfig.w2_bystander_op_upper_limit_pct
    primary_op_share_pct = AgentConfig.w2_primary_op_share_pct
    catastrophic_regress_pct = AgentConfig.w2_catastrophic_regress_pct
    library_share: dict = {}
    _op_limits = W2Session._op_limits


OPS = ["cache_roundtrip", "zlib_compress", "raw_deflate"]
WEIGHTS = {"cache_roundtrip": 20.0, "zlib_compress": 35.0, "raw_deflate": 35.0}


# ───────────────────────────── the regression

def test_an_op_that_never_reaches_the_library_loses_its_veto() -> None:
    """The measured case: 0.0 library share, yet holding a full veto."""
    s = _Session()
    s.library_share = {"cache_roundtrip": 0.0}
    limits = s._op_limits(OPS, WEIGHTS)
    assert limits["cache_roundtrip"] >= AgentConfig.w2_catastrophic_regress_pct


def test_a_library_dominated_op_keeps_its_veto() -> None:
    """Control: the ops that do measure the library must be unaffected."""
    s = _Session()
    s.library_share = {"cache_roundtrip": 0.0, "zlib_compress": 0.99,
                       "raw_deflate": 0.99}
    limits = s._op_limits(OPS, WEIGHTS) or {}
    assert "zlib_compress" not in limits
    assert "raw_deflate" not in limits


@pytest.mark.parametrize("share", [0.0, 0.02, 0.09])
def test_shares_below_the_floor_are_released(share) -> None:
    s = _Session()
    s.library_share = {"cache_roundtrip": share}
    limits = s._op_limits(OPS, WEIGHTS)
    assert limits["cache_roundtrip"] >= AgentConfig.w2_catastrophic_regress_pct


# Expressed relative to the floor, not as a copy of its value: the previous
# literal `0.10` silently encoded the floor of the day and broke the moment it
# moved, reporting a floor change as a failure of the code under test.
@pytest.mark.parametrize("share", [_LIBRARY_SHARE_FLOOR, 0.5, 0.918, 1.0])
def test_shares_at_or_above_the_floor_are_untouched(share) -> None:
    s = _Session()
    s.library_share = {"zlib_compress": share}
    limits = s._op_limits(OPS, WEIGHTS) or {}
    assert "zlib_compress" not in limits


def test_an_unmeasured_op_keeps_full_standing() -> None:
    """No measurement is not evidence of a low share — absence must not
    silently strip an op of its veto."""
    s = _Session()
    s.library_share = {}
    limits = s._op_limits(OPS, WEIGHTS) or {}
    assert "zlib_compress" not in limits


# ───────────────────────────── reading the measured shares

def _hotspots(tmp_path, functions) -> None:
    (tmp_path / "hotspots.json").write_text(
        json.dumps({"hot_functions": functions}), encoding="utf-8")


def test_shares_come_from_the_inline_expanded_profile(tmp_path) -> None:
    """`hotspots.json` is `locate`'s output — the one profile in the pipeline
    that puts inlined library frames back on the stack."""
    _hotspots(tmp_path, [
        {"name": "ZopfliInitCache", "per_op": {"cache_roundtrip": 20.1,
                                               "zlib_compress": 0.4}},
        {"name": "ZopfliFindLongestMatch", "per_op": {"cache_roundtrip": 29.0,
                                                      "zlib_compress": 95.9}},
    ])
    got = _measured_library_share(tmp_path)
    assert got["cache_roundtrip"] == pytest.approx(0.491)
    assert got["zlib_compress"] == pytest.approx(0.963)


def test_the_symbol_level_baseline_is_not_consulted(tmp_path) -> None:
    """The regression this file exists to prevent, stated directly: a
    `crate_share` written by a symbol-level profile must not reach the gates,
    even when it is sitting right there next to the good number."""
    (tmp_path / "baseline.json").write_text(json.dumps({
        "measurements": {"cache_roundtrip": {"wall_ms": 1494.0,
                                             "crate_share": 0.0}}
    }), encoding="utf-8")
    _hotspots(tmp_path, [
        {"name": "ZopfliInitCache", "per_op": {"cache_roundtrip": 49.1}},
    ])
    assert _measured_library_share(tmp_path)["cache_roundtrip"] > _LIBRARY_SHARE_FLOOR


def test_a_half_library_op_keeps_its_veto(tmp_path) -> None:
    """The whole point, end to end: 49% is nothing like harness-bound."""
    _hotspots(tmp_path, [
        {"name": "ZopfliInitCache", "per_op": {"cache_roundtrip": 49.1}},
    ])
    s = _Session()
    s.library_share = _measured_library_share(tmp_path)
    assert "cache_roundtrip" not in (s._op_limits(
        ["cache_roundtrip"], {"cache_roundtrip": 50.0}) or {})


def test_a_genuinely_harness_bound_op_still_loses_it(tmp_path) -> None:
    """The relaxation must survive the correction — ops that really are the
    harness's own work do exist, and they are what it is for."""
    _hotspots(tmp_path, [
        {"name": "ZopfliInitCache", "per_op": {"probe_roundtrip": 2.5}},
    ])
    s = _Session()
    s.library_share = _measured_library_share(tmp_path)
    assert "probe_roundtrip" in (s._op_limits(
        ["probe_roundtrip"], {"probe_roundtrip": 50.0}) or {})


def test_shares_are_summed_across_hot_functions(tmp_path) -> None:
    _hotspots(tmp_path, [
        {"name": "a", "per_op": {"op": 30.0}},
        {"name": "b", "per_op": {"op": 25.5}},
        {"name": "c", "per_op": {"other": 90.0}},
    ])
    got = _measured_library_share(tmp_path)
    assert got["op"] == pytest.approx(0.555)
    assert got["other"] == pytest.approx(0.900)


def test_the_sum_is_clamped_to_one(tmp_path) -> None:
    """Self-time shares can overshoot slightly on rounding; a share above 1
    would be nonsense to compare against a fraction floor."""
    _hotspots(tmp_path, [
        {"name": "a", "per_op": {"op": 60.0}},
        {"name": "b", "per_op": {"op": 55.0}},
    ])
    assert _measured_library_share(tmp_path)["op"] == 1.0


def test_an_op_absent_from_every_hot_function_is_simply_unknown(tmp_path) -> None:
    """Not zero. An op with no function above the hotness threshold has not
    been shown to be harness-bound — it has not been measured, and absence
    must not strip a veto."""
    _hotspots(tmp_path, [{"name": "a", "per_op": {"other": 90.0}}])
    assert "op" not in _measured_library_share(tmp_path)


def test_hot_functions_only_covers_functions_above_the_threshold(tmp_path) -> None:
    """The sum is therefore a LOWER bound on the crate's share — the safe
    direction for a floor test: it can leave an op its veto, never take one
    away wrongly."""
    _hotspots(tmp_path, [{"name": "a", "per_op": {"op": 9.9}}])
    assert _measured_library_share(tmp_path)["op"] < _LIBRARY_SHARE_FLOOR


@pytest.mark.parametrize("payload", ["", "{", '{"hot_functions": null}',
                                     '{"hot_functions": [1, 2]}',
                                     '{"hot_functions": [{"per_op": "x"}]}'])
def test_an_unusable_hotspots_file_is_not_fatal(tmp_path, payload) -> None:
    (tmp_path / "hotspots.json").write_text(payload, encoding="utf-8")
    assert _measured_library_share(tmp_path) == {}


def test_a_missing_hotspots_file_leaves_every_op_with_full_standing(tmp_path) -> None:
    assert _measured_library_share(tmp_path) == {}
    assert _measured_library_share(None) == {}


def test_the_baseline_no_longer_measures_a_share_at_all(tmp_path) -> None:
    """A symbol-level number sitting in `baseline.json` reads as authoritative
    in the log and invites the next reader to use it. It is gone, along with
    the per-op `perf record` that produced it."""
    import inspect
    from perf_opt.agent_perf_opt import driver
    src = inspect.getsource(driver)
    assert "_measure_crate_share" not in src
    assert "crate_share" not in src.split("def _initial_baseline")[-1][:4000] \
        or "hotspots.json" in src
