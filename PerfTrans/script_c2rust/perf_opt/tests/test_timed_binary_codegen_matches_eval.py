"""The timed binary must be built with the codegen the evaluation reports.

`.cargo/config.toml` resolves from the directory cargo is INVOKED in and never
follows a path dependency, so the harness's config alone decides the codegen of
the whole binary — the library crate's own `-Ctarget-cpu=native` is never read.

Before 2026-09-07 the harness carried only `--remap-path-prefix`, so every
perf_opt decision was made on an x86-64-baseline build while the paper reports
`target-cpu=native` numbers. On fzy the same rewrite measured -22.3% under the
baseline and +0.3% under native on the SAME harness and input: the gate was
accepting changes worth nothing in the reported configuration.
"""
from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))

from harness_gen.build_env import (CANONICAL_CODEGEN_FLAGS,  # noqa: E402
                                   ensure_canonical_codegen,
                                   ensure_stable_debug_paths)
from perf_opt.fair_build import _audit_cargo_config  # noqa: E402


def _flags(harness: Path) -> list[str]:
    import tomllib
    cfg = harness / ".cargo" / "config.toml"
    return list(tomllib.loads(cfg.read_text())["build"]["rustflags"])


def _evaluation_rustflags() -> list[str] | None:
    """The rustflags the evaluation actually builds its binaries with."""
    import tomllib
    cfg = (Path(__file__).resolve().parents[3] / "dataset_trans" / "fzy"
           / "validation_workload" / "purebin" / "raw" / ".cargo" / "config.toml")
    if not cfg.is_file():
        return None
    return list(tomllib.loads(cfg.read_text())["build"]["rustflags"])


def test_canonical_flags_match_the_evaluation_build() -> None:
    """The constant and the evaluation's config must be the SAME SET.

    Checked in both directions on purpose. The first version of this test only
    asserted canonical ⊆ evaluation, and the real defect was the other way
    round: the evaluation gained `-Cllvm-args=-align-all-functions=6` while the
    constant did not, so the pipeline kept optimizing unaligned binaries and
    reporting aligned ones. On libopenaptx that showed up as the pipeline
    measuring +0.16% for an artifact the evaluation measured 6 pp slower than
    its own starting point (2026-09-07).
    """
    evaluation = _evaluation_rustflags()
    if evaluation is None:
        pytest.skip("evaluation tree not present")
    missing_here = [f for f in evaluation if f not in CANONICAL_CODEGEN_FLAGS]
    assert not missing_here, (
        f"the evaluation builds with {missing_here} but CANONICAL_CODEGEN_FLAGS "
        f"does not carry it — the pipeline would optimize under a different "
        f"codegen than the one the paper reports")
    missing_there = [f for f in CANONICAL_CODEGEN_FLAGS if f not in evaluation]
    assert not missing_there, (
        f"{missing_there} is in CANONICAL_CODEGEN_FLAGS but not in the "
        f"evaluation's build config — the two have drifted apart")


def test_adds_missing_flags_to_a_bare_harness(tmp_path: Path) -> None:
    assert ensure_canonical_codegen(tmp_path) is True
    assert _flags(tmp_path) == list(CANONICAL_CODEGEN_FLAGS)


def test_is_idempotent(tmp_path: Path) -> None:
    ensure_canonical_codegen(tmp_path)
    assert ensure_canonical_codegen(tmp_path) is False
    assert _flags(tmp_path) == list(CANONICAL_CODEGEN_FLAGS)


def test_preserves_the_remap_flag(tmp_path: Path) -> None:
    """Both flags must survive together — remap keeps layout comparable across
    arms, the codegen flags keep the build comparable with the evaluation."""
    ensure_stable_debug_paths(tmp_path, tmp_path / "3_perf_opt")
    ensure_canonical_codegen(tmp_path)
    flags = _flags(tmp_path)
    assert any(f.startswith("--remap-path-prefix=") for f in flags), flags
    for flag in CANONICAL_CODEGEN_FLAGS:
        assert flag in flags, flags


def test_order_does_not_matter(tmp_path: Path) -> None:
    ensure_canonical_codegen(tmp_path)
    ensure_stable_debug_paths(tmp_path, tmp_path / "3_perf_opt")
    flags = _flags(tmp_path)
    assert any(f.startswith("--remap-path-prefix=") for f in flags), flags
    for flag in CANONICAL_CODEGEN_FLAGS:
        assert flag in flags, flags


def test_fair_build_rejects_a_harness_without_the_codegen_flags(tmp_path: Path) -> None:
    """This is the check that would have caught the original defect."""
    ensure_stable_debug_paths(tmp_path, tmp_path / "3_perf_opt")   # remap only
    errors = _audit_cargo_config(tmp_path, timed=True)
    assert errors and "-Ctarget-cpu=native" in errors[0], errors

    ensure_canonical_codegen(tmp_path)
    assert _audit_cargo_config(tmp_path, timed=True) == []


def test_library_crate_is_still_audited_leniently(tmp_path: Path) -> None:
    """Only the invoked crate needs the vector pin; a library crate carrying
    just `-Ctarget-cpu=native` is correct and must not start failing."""
    (tmp_path / ".cargo").mkdir()
    (tmp_path / ".cargo" / "config.toml").write_text(
        '[build]\nrustflags = ["-Ctarget-cpu=native"]\n', encoding="utf-8")
    assert _audit_cargo_config(tmp_path) == []
    assert _audit_cargo_config(tmp_path, timed=True) != []
