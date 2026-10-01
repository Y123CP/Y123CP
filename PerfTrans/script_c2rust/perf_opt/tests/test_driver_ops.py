"""Regression tests for perf_opt op resolution (_read_ops / _perf_ops).

Guards the foreign-shared-spec poisoning bug: when a project has two harnesses
under workloads/harness_gen/, they share the parent logs/spec.json, and reading
it returns op names that match no perf_inputs → 0 hot fns downstream.
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.driver import _read_ops, _perf_ops, run_perf_opt
from perf_opt.verify import WorkloadAssets


def _make_project(tmp_path: Path, *, own_ops: list[str],
                  foreign_spec_ops: list[str] | None,
                  perf_bins: list[str]) -> Path:
    """Build a project whose real harness has `own_ops` (corpus + golden +
    perf_inputs) while the SHARED parent logs/spec.json lists `foreign_spec_ops`.
    """
    proj = tmp_path / "proj"
    hg = proj / "workloads" / "harness_gen"
    harness = hg / "proj_harness"
    (harness / "corpus").mkdir(parents=True)
    (harness / "perf_inputs").mkdir(parents=True)
    for op in own_ops:
        (harness / "corpus" / op).mkdir()
    for op in perf_bins:
        (harness / "perf_inputs" / f"{op}.perf.bin").write_bytes(b"\0" * 16)
    (harness / "golden.jsonl").write_text(
        "\n".join(json.dumps({"op": op, "stdout_sha256": "x"}) for op in own_ops)
    )
    if foreign_spec_ops is not None:
        (hg / "logs").mkdir(parents=True)
        (hg / "logs" / "spec.json").write_text(json.dumps(
            {"operations": [{"name": op} for op in foreign_spec_ops]}
        ))
    return proj


def test_read_ops_ignores_foreign_shared_spec(tmp_path: Path) -> None:
    own = ["compress_decompress", "frame_round_trip", "xxhash_digest"]
    proj = _make_project(
        tmp_path,
        own_ops=own,
        foreign_spec_ops=["lz4_block_roundtrip", "xxhash_streaming"],
        perf_bins=own,
    )
    assets = WorkloadAssets.discover(proj)
    ops = _read_ops(assets)
    assert sorted(ops) == sorted(own)          # own corpus wins, not the foreign spec
    assert len(_perf_ops(assets, ops)) == len(own)


def test_read_ops_prefers_own_spec_then_corpus(tmp_path: Path) -> None:
    own = ["op_a", "op_b"]
    proj = _make_project(tmp_path, own_ops=own, foreign_spec_ops=None, perf_bins=own)
    # give the harness its OWN spec.json (richest source)
    harness = proj / "workloads" / "harness_gen" / "proj_harness"
    (harness / "logs").mkdir(parents=True)
    (harness / "logs" / "spec.json").write_text(json.dumps(
        {"operations": [{"name": "op_a"}, {"name": "op_b"}]}
    ))
    assets = WorkloadAssets.discover(proj)
    assert sorted(_read_ops(assets)) == own


def test_run_perf_opt_fails_loud_on_op_perfbin_mismatch(tmp_path: Path) -> None:
    # perf_inputs exist but the (foreign) op names match none of them.
    proj = _make_project(
        tmp_path,
        own_ops=[],                                   # no corpus/golden ops
        foreign_spec_ops=["ghost_op"],
        perf_bins=["real_op"],
    )
    # 2_stage_a must exist for run_perf_opt to reach the check.
    (proj / "2_stage_a").mkdir(parents=True)
    with pytest.raises(RuntimeError, match="match the"):
        run_perf_opt(proj)
