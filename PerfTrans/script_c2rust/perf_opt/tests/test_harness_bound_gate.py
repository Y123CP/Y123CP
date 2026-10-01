"""The workload shape gate: does an op measure the LIBRARY or the harness?

`self_time_own` only separates this process from libc/kernel. The harness and
the library under test are linked into the same binary, so an op that spends all
its time in the harness's own per-iteration digest code still scores ~100% there
and sails through that gate — while the library it is supposed to exercise sits
at 0%, leaving perf_opt with nothing to optimize.
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from harness_gen import build_workload
from harness_gen.perf_workload import (
    CRATE_SHARE_GATE,
    SELF_TIME_GATE,
    OpPerf,
    harness_crate_name,
)


def test_crate_share_subtracts_the_harness_own_symbols() -> None:
    row = OpPerf(op="x", self_time_own=0.998, harness_share=0.9975)
    assert row.crate_share == pytest.approx(0.0005, abs=1e-4)


def test_crate_share_never_goes_negative() -> None:
    # symbol-level attribution can, in principle, overshoot the DSO-level figure
    row = OpPerf(op="x", self_time_own=0.90, harness_share=0.95)
    assert row.crate_share == 0.0


def test_harness_bound_op_passes_the_old_gate_but_fails_the_new_one() -> None:
    """The exact regression this gate exists for."""
    row = OpPerf(op="digest_heavy", self_time_own=0.998, harness_share=0.9975)
    assert row.self_time_own >= SELF_TIME_GATE      # old gate: clean
    assert row.crate_share < CRATE_SHARE_GATE       # new gate: caught


def test_library_dominated_op_passes_both() -> None:
    row = OpPerf(op="lib_hot", self_time_own=0.94, harness_share=0.05)
    assert row.self_time_own >= SELF_TIME_GATE
    assert row.crate_share >= CRATE_SHARE_GATE


def test_as_dict_exposes_the_shape_fields_for_downstream() -> None:
    row = OpPerf(op="x", self_time_own=0.99, harness_share=0.98,
                 harness_bound=True)
    payload = row.as_dict()
    assert payload["harness_share"] == 0.98
    assert payload["crate_share"] == 0.01
    assert payload["harness_bound"] is True


def test_harness_crate_name_reads_package_name(tmp_path: Path) -> None:
    (tmp_path / "Cargo.toml").write_text(
        "# comment\n"
        "[package]\n"
        'name = "libfoo_raw_harness"\n'
        'version = "0.1.0"\n'
        "\n[lib]\n"
        'name = "not_this_one"\n',
        encoding="utf-8",
    )
    assert harness_crate_name(tmp_path) == "libfoo_raw_harness"


def test_harness_crate_name_missing_manifest_is_not_fatal(tmp_path: Path) -> None:
    assert harness_crate_name(tmp_path) == ""


def _write_workload(tmp_path: Path, rows: list[dict]) -> Path:
    (tmp_path / "perf_workload.json").write_text(
        json.dumps(rows), encoding="utf-8")
    return tmp_path


def test_build_workload_collects_harness_bound_ops(tmp_path: Path) -> None:
    hdir = _write_workload(tmp_path, [
        {"op": "clean", "harness_bound": False, "crate_share": 0.9},
        {"op": "bad", "harness_bound": True, "crate_share": 0.002,
         "harness_share": 0.997,
         "top_functions": [[99.7, "pkg_harness::digest_common"]]},
    ])
    found = build_workload._harness_bound_ops(hdir)
    assert [op for op, _ in found] == ["bad"]


def test_build_workload_survives_missing_or_broken_workload(tmp_path: Path) -> None:
    assert build_workload._harness_bound_ops(tmp_path) == []
    (tmp_path / "perf_workload.json").write_text("{not json", encoding="utf-8")
    assert build_workload._harness_bound_ops(tmp_path) == []


def test_sweep_ladder_reaches_realistic_scale() -> None:
    """gen_perf sometimes emits tens of MB; the ladder must offer a rung in the
    hundreds-of-KB-to-MB range, because that is where library share peaks
    (measured: 128K 56%, 1.4M 57%, but 26M only 14%)."""
    from harness_gen.perf_workload import _SWEEP_SIZES

    assert any(512_000 <= size <= 4_000_000 for size in _SWEEP_SIZES)
    assert list(_SWEEP_SIZES) == sorted(_SWEEP_SIZES)


def test_contract_caps_perf_input_size() -> None:
    """Without an upper bound the model reads "one LARGE realistic input" as
    "as large as possible" and emits tens of MB."""
    from harness_gen.prompts import CONTRACT

    assert "Do NOT emit" in CONTRACT
    assert "tens of megabytes" in CONTRACT


def test_gen_perf_omits_trivial_call_ops() -> None:
    """`work does not scale with input` does not catch a predicate sweep: total
    work DOES grow with input, but each call is a range check that costs no
    more than the loop's own bookkeeping and gets fused with it by the
    optimizer. Measured on libxml2: `chvalid_sweep` profiled at 2% library with
    full debug info and inline-aware attribution — there was no separable
    library time to find."""
    from harness_gen.prompts import CONTRACT

    assert "predicate" in CONTRACT
    assert "does not scale with the input" in CONTRACT
    # and the positive form, so the model has a target to aim at
    assert "ONE library call does a lot of work" in CONTRACT


def test_contract_covers_both_non_ascii_traps() -> None:
    """Generated corpora carry accented text, and Rust's string rules differ
    from C's in two ways that both actually fired: brotli panicked slicing a
    String at a byte index inside 'é', and libxml2 failed to compile on
    `"Caf\\xe9"` (`\\xNN` in a &str is capped at \\x7f)."""
    from harness_gen.prompts import CONTRACT

    assert "out of range hex escape" in CONTRACT
    assert "char boundary" in CONTRACT
    # ...and the fixes, not just the symptoms
    assert "char_indices()" in CONTRACT
    assert r"\u{e9}" in CONTRACT


def test_contract_rule_numbers_stay_sequential() -> None:
    """Inserting a rule must renumber the ones after it — the gates and this
    prompt both refer to rules by number."""
    import re

    from harness_gen.prompts import CONTRACT

    nums = [int(m.group(1)) for m in re.finditer(r"^  (\d+)\.", CONTRACT, re.M)]
    assert nums == list(range(1, len(nums) + 1))


def test_contract_states_the_loop_invariant_rule() -> None:
    """The generator prompt must carry the executable criterion, not just the
    intent — the original wording ("heavy lifting must happen inside the
    library") was true but unenforceable, and the model complied with it while
    still putting whole-input scans in the timed loop."""
    from harness_gen.prompts import CONTRACT

    assert "loop-invariant" in CONTRACT
    assert "harness_bound" in CONTRACT
    # the three concrete prohibitions
    assert "windows()" in CONTRACT
    assert "clone()" in CONTRACT
    assert "compute it ONCE before the loop" in CONTRACT
