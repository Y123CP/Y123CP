"""A targeted run (--only-fns) reuses what it can prove it may reuse.

A full lz4 run takes ~7 hours. When a change touches a few functions, the
targeted run re-runs detection as a full run would and hands only those
functions to the agent. Everything it borrows from a previous run is checked
first — exists, read, validated — and the one thing it cannot validate, the
baseline, it declines to borrow at all.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
from types import SimpleNamespace

import pytest

from perf_opt.agent_perf_opt import driver

_OPS = ["decompress_usingdict", "streaming_decode", "xxhash_digest"]


def _hotspots(dirpath: Path, ops=_OPS, names=("LZ4_decompress_generic",)) -> Path:
    dirpath.mkdir(parents=True, exist_ok=True)
    f = dirpath / "hotspots.json"
    f.write_text(json.dumps({
        "ops": list(ops),
        "hot_functions": [{"name": n, "self_pct": 30.0, "hottest_op": ops[0]}
                          for n in names],
        "sub_tau_profile": {"LZ4_compress_fast": {ops[0]: 0.17}},
    }))
    return f


def _inputs(dirpath: Path) -> Path:
    dirpath.mkdir(parents=True, exist_ok=True)
    for op in _OPS:
        (dirpath / f"{op}.perf.bin").write_bytes(b"x")
    return dirpath


def _age(path: Path, seconds: float) -> None:
    t = path.stat().st_mtime - seconds
    os.utime(path, (t, t))


# ───────────────────────────────── hotspots: exists → read → validate → use


def test_a_missing_hotspots_file_is_refused(tmp_path: Path) -> None:
    with pytest.raises(RuntimeError, match="不存在"):
        driver._load_reusable_hotspots(
            tmp_path / "prev" / "hotspots.json", _OPS, _inputs(tmp_path / "in"))


def test_hotspots_from_another_op_set_is_refused(tmp_path: Path) -> None:
    inputs = _inputs(tmp_path / "in")
    src = _hotspots(tmp_path / "prev", ops=_OPS[:2] + ["frame_misc"])
    with pytest.raises(RuntimeError, match="op 集合"):
        driver._load_reusable_hotspots(src, _OPS, inputs)


def test_hotspots_older_than_the_perf_inputs_is_refused(tmp_path: Path) -> None:
    """The lz4 case: inputs regenerated after the hot list was profiled."""
    src = _hotspots(tmp_path / "prev")
    _age(src, 3600)
    inputs = _inputs(tmp_path / "in")
    with pytest.raises(RuntimeError, match="比当前 perf 输入旧"):
        driver._load_reusable_hotspots(src, _OPS, inputs)


def test_valid_hotspots_come_back_whole(tmp_path: Path) -> None:
    inputs = _inputs(tmp_path / "in")
    for b in inputs.glob("*.perf.bin"):
        _age(b, 3600)
    src = _hotspots(tmp_path / "prev")
    data = driver._load_reusable_hotspots(src, list(reversed(_OPS)), inputs)
    assert data["hot_functions"][0]["name"] == "LZ4_decompress_generic"
    # fixed-cost admission reads this back from the new run's tree
    assert "sub_tau_profile" in data


# ──────────────────────────────────────────────────────── selecting functions


class _Index:
    def aliases_of(self, name):
        return {"LZ4_decompress_generic": ["lz4_decompress_generic_link"]}.get(
            name, [])


def _hf(*names):
    return [SimpleNamespace(name=n) for n in names]


def test_selection_keeps_hot_list_order_and_accepts_aliases() -> None:
    hot = _hf("A", "LZ4_decompress_generic", "B", "C")
    picked = driver._select_only_fns(
        hot, ["C", "lz4_decompress_generic_link"], _Index())
    assert [h.name for h in picked] == ["LZ4_decompress_generic", "C"]


def test_an_unknown_function_is_refused_not_skipped() -> None:
    with pytest.raises(RuntimeError, match="不在本轮的热函数集合里"):
        driver._select_only_fns(_hf("A", "B"), ["A", "Nope"], _Index())


# ─────────────────────────────────────────────── where it writes, what it skips


def test_a_targeted_run_never_writes_the_full_arm_directory(tmp_path: Path) -> None:
    out = driver._opt_dir_for(tmp_path, "full", ["X"])
    assert out == tmp_path / driver.TARGETED_DIRNAME
    assert out != tmp_path / "3_perf_opt"
    assert driver._opt_dir_for(tmp_path, "full", None) == tmp_path / "3_perf_opt"


def test_targeted_with_an_ablation_arm_is_refused_before_anything_runs(
    tmp_path: Path,
) -> None:
    with pytest.raises(RuntimeError, match="full-arm"):
        driver.run_perf_opt(tmp_path, only_fns=["X"], arm="freeform")
    assert not any(tmp_path.iterdir())


def test_the_skipped_baseline_says_so_and_carries_no_numbers(tmp_path: Path) -> None:
    b = driver._skipped_baseline(tmp_path, "abc123")
    on_disk = json.loads((tmp_path / "baseline.json").read_text())
    assert b == on_disk
    assert on_disk["skipped"] is True and on_disk["measurements"] == {}


def test_the_run_records_that_it_was_targeted(tmp_path: Path) -> None:
    driver._write_targeted_record(tmp_path, ["C"], _hf("C"), tmp_path / "prev")
    rec = json.loads((tmp_path / "targeted.json").read_text())
    assert rec["kind"].startswith("targeted run")
    assert rec["selected_in_order"] == ["C"]
