"""The build directory must not reach the binary.

`debug = 2` is required for inline attribution, and DWARF records the
absolute path the crate was built from. That path is not inert: two trees
differing only in directory NAME produce binaries differing in SIZE, and a
size difference is a code-layout difference.

Measured on fzy — two pristine harnesses, identical source, identical
toolchain, paired `insns -0.000%`, built under `3_perf_opt` and
`3_perf_opt_freeform`:

    match_primitives   wall +6.182%   CI [+5.870, +6.494]

That is larger than most rewrites this pipeline commits, and it lands
exactly between an ablation's two arms, whose directory names differ by
construction. The ablation verdict it produced was backwards.

`ensure_stable_debug_paths` maps the build root to a constant so both arms
emit the same bytes.
"""

from __future__ import annotations

import tomllib
from pathlib import Path

import pytest

from harness_gen.build_env import REMAP_TARGET, ensure_stable_debug_paths


def _flags(harness: Path) -> list[str]:
    cfg = harness / ".cargo" / "config.toml"
    return tomllib.loads(cfg.read_text(encoding="utf-8"))["build"]["rustflags"]


def test_it_creates_the_config_when_absent(tmp_path: Path) -> None:
    h = tmp_path / "harness"
    h.mkdir()
    assert ensure_stable_debug_paths(h, tmp_path / "3_perf_opt") is True
    assert _flags(h) == [f"--remap-path-prefix={tmp_path / '3_perf_opt'}={REMAP_TARGET}"]


def test_it_is_idempotent(tmp_path: Path) -> None:
    h = tmp_path / "harness"
    h.mkdir()
    root = tmp_path / "3_perf_opt"
    assert ensure_stable_debug_paths(h, root) is True
    assert ensure_stable_debug_paths(h, root) is False
    assert len(_flags(h)) == 1


def test_the_two_arms_end_up_with_the_same_target(tmp_path: Path) -> None:
    """The whole point: different roots, one destination string."""
    full, abl = tmp_path / "full", tmp_path / "abl"
    full.mkdir(); abl.mkdir()
    ensure_stable_debug_paths(full, tmp_path / "3_perf_opt")
    ensure_stable_debug_paths(abl, tmp_path / "3_perf_opt_freeform")
    assert _flags(full)[0].endswith(f"={REMAP_TARGET}")
    assert _flags(abl)[0].endswith(f"={REMAP_TARGET}")
    assert _flags(full)[0] != _flags(abl)[0]      # sources differ...
    # ...and both erase to the same string, which is what makes the binaries
    # comparable. The remapped-to half is all the binary keeps.


def test_an_existing_rustflag_survives(tmp_path: Path) -> None:
    """`-Ctarget-cpu=native` is a codegen flag — dropping it would silently
    change machine code and invalidate every measurement taken so far."""
    h = tmp_path / "harness"
    (h / ".cargo").mkdir(parents=True)
    (h / ".cargo" / "config.toml").write_text(
        '[build]\nrustflags = ["-Ctarget-cpu=native"]\n', encoding="utf-8")
    ensure_stable_debug_paths(h, tmp_path / "root")
    assert "-Ctarget-cpu=native" in _flags(h)
    assert len(_flags(h)) == 2


def test_a_stale_remap_for_another_root_is_replaced(tmp_path: Path) -> None:
    """A copied or moved tree carries a remap that now maps nothing; leaving
    it would let the real path back into the binary."""
    h = tmp_path / "harness"
    (h / ".cargo").mkdir(parents=True)
    (h / ".cargo" / "config.toml").write_text(
        f'[build]\nrustflags = ["--remap-path-prefix=/old/tree={REMAP_TARGET}"]\n',
        encoding="utf-8")
    ensure_stable_debug_paths(h, tmp_path / "new")
    flags = _flags(h)
    assert len(flags) == 1
    assert "/old/tree" not in flags[0]
    assert str(tmp_path / "new") in flags[0]


def test_it_adds_no_codegen_flag(tmp_path: Path) -> None:
    """Only debug metadata. Anything that moves machine code would make this
    fix indistinguishable from the bug it removes."""
    h = tmp_path / "harness"
    h.mkdir()
    ensure_stable_debug_paths(h, tmp_path / "root")
    for f in _flags(h):
        assert f.startswith("--remap-path-prefix="), f
        assert "-C" not in f and "target-cpu" not in f


def test_other_config_sections_are_preserved(tmp_path: Path) -> None:
    h = tmp_path / "harness"
    (h / ".cargo").mkdir(parents=True)
    (h / ".cargo" / "config.toml").write_text(
        '[net]\nretry = 3\n\n[build]\nrustflags = ["-Ctarget-cpu=native"]\n',
        encoding="utf-8")
    ensure_stable_debug_paths(h, tmp_path / "root")
    data = tomllib.loads((h / ".cargo" / "config.toml").read_text(encoding="utf-8"))
    assert data["net"]["retry"] == 3
    assert len(data["build"]["rustflags"]) == 2


def test_a_build_section_without_rustflags_gets_one(tmp_path: Path) -> None:
    h = tmp_path / "harness"
    (h / ".cargo").mkdir(parents=True)
    (h / ".cargo" / "config.toml").write_text(
        '[build]\ntarget-dir = "t"\n', encoding="utf-8")
    ensure_stable_debug_paths(h, tmp_path / "root")
    data = tomllib.loads((h / ".cargo" / "config.toml").read_text(encoding="utf-8"))
    assert data["build"]["target-dir"] == "t"
    assert len(data["build"]["rustflags"]) == 1


def test_the_driver_calls_it_on_the_harness_not_the_crate(tmp_path: Path) -> None:
    """cargo walks up from the INVOCATION directory, never through a
    path-dependency. `cargo build` runs in the harness, so a config placed on
    the crate is read by nothing."""
    src = Path("perf_opt/agent_perf_opt/driver.py").read_text(encoding="utf-8")
    assert "ensure_stable_debug_paths(harness, opt_dir)" in src
    assert "ensure_stable_debug_paths(crate" not in src
