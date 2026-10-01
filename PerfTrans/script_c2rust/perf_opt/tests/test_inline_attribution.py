"""Attribution must not be more confident than its preconditions.

Telling "the library is hot" from "the harness is hot" only works if inlined
library functions can be put back on the stack, and that needs DWARF for the
crate under test. Built without it, `perf script --inline` expands nothing,
every absorbed library function stays charged to the harness symbol, and the
profiler returns a symbol-level number wearing a call-graph costume.

Measured on libxml2: the harness was built `debug = false`; `xmlIsBaseChar`
survived only as its lookup tables (no code symbol at all); and
`chvalid_sweep` — whose timed loop contains nothing but library calls —
profiled at 99.1% harness / 0% library on every one of its five candidate
inputs. Six of ten ops were flagged; at most two deserved it.
"""

from __future__ import annotations

import json
import re
from pathlib import Path

import pytest

from harness_gen import build_env, perf_workload as pw
from perf_opt.agent_perf_opt import driver


# ───────────────────────────────────────────── the precondition, in one place

def test_ensure_debuginfo_adds_the_profile_when_absent(tmp_path: Path) -> None:
    f = tmp_path / "Cargo.toml"
    f.write_text('[package]\nname = "h"\n', encoding="utf-8")
    assert build_env.ensure_debuginfo(f) is True
    assert "[profile.release]" in f.read_text()
    assert "debug = 2" in f.read_text()


def test_ensure_debuginfo_overwrites_debug_false(tmp_path: Path) -> None:
    """The exact shape harness_gen's template emits."""
    f = tmp_path / "Cargo.toml"
    f.write_text('[profile.release]\nopt-level = 3\nlto = "fat"\n'
                 "codegen-units = 1\ndebug = false\n", encoding="utf-8")
    assert build_env.ensure_debuginfo(f) is True
    text = f.read_text()
    assert "debug = 2" in text
    assert "debug = false" not in text
    # the fair-build settings must survive untouched
    assert 'lto = "fat"' in text and "opt-level = 3" in text


def test_ensure_debuginfo_adds_the_key_to_an_existing_profile(tmp_path: Path) -> None:
    f = tmp_path / "Cargo.toml"
    f.write_text('[profile.release]\nopt-level = 3\n', encoding="utf-8")
    assert build_env.ensure_debuginfo(f) is True
    assert "debug = 2" in f.read_text()


def test_ensure_debuginfo_is_idempotent(tmp_path: Path) -> None:
    f = tmp_path / "Cargo.toml"
    f.write_text("[profile.release]\ndebug = 2\n", encoding="utf-8")
    assert build_env.ensure_debuginfo(f) is False


def test_build_release_guarantees_the_precondition(monkeypatch, tmp_path: Path) -> None:
    """Profiling builds must not be able to skip it."""
    (tmp_path / "Cargo.toml").write_text(
        '[profile.release]\ndebug = false\n', encoding="utf-8")
    monkeypatch.setattr(pw, "run_cmd", lambda *a, **k: (0, "", ""))
    pw.build_release(tmp_path)
    assert "debug = 2" in (tmp_path / "Cargo.toml").read_text()


# ──────────────────────────────────────── unknown must not be reported as bad

def test_missing_crate_debuginfo_makes_attribution_return_none(monkeypatch) -> None:
    """The regression: it used to return a number it could not justify."""
    monkeypatch.setattr(build_env, "has_crate_debuginfo", lambda *a: False)
    monkeypatch.setattr(pw, "has_crate_debuginfo", lambda *a: False)
    got = pw._harness_share_precise(
        Path("bin"), "op", Path("inp"), 10, Path("hdir"), False,
        "proj_harness", "proj_lib")
    assert got is None


def test_unknown_attribution_is_not_the_same_as_harness_bound() -> None:
    unknown = pw.OpPerf(op="x", self_time_own=0.99, harness_share=0.99,
                        attribution_reliable=False, harness_bound=None)
    measured = pw.OpPerf(op="y", self_time_own=0.99, harness_share=0.99,
                         attribution_reliable=True, harness_bound=True)
    assert unknown.harness_bound is None
    assert measured.harness_bound is True
    assert unknown.as_dict()["attribution_reliable"] is False


def test_crate_under_test_name_resolves_a_renamed_dependency(tmp_path: Path) -> None:
    """`brotli_raw = { path = "...", package = "brotli_cleaned" }` — DWARF
    carries the PACKAGE name, not the dependency key."""
    (tmp_path / "Cargo.toml").write_text(
        '[package]\nname = "brotli_raw_harness"\n\n[dependencies]\n'
        'brotli_raw = { path = "/x/crate", package = "brotli_cleaned" }\n',
        encoding="utf-8")
    assert pw.crate_under_test_name(tmp_path) == "brotli_cleaned"


def test_crate_under_test_name_falls_back_to_the_key(tmp_path: Path) -> None:
    (tmp_path / "Cargo.toml").write_text(
        '[dependencies]\nlibxml2_raw = { path = "/x/crate" }\n', encoding="utf-8")
    assert pw.crate_under_test_name(tmp_path) == "libxml2_raw"


def test_has_crate_debuginfo_reports_unknown_not_absent(monkeypatch) -> None:
    """No readelf / unreadable binary must not read as 'no debug info'."""
    assert build_env.has_crate_debuginfo(Path("/nope"), "c") is None
    assert build_env.has_crate_debuginfo(Path("/nope"), "") is None


# ─────────────────────────────────────── a flag this weak must not drop an op

def _assets(tmp_path: Path, ops: list[str], rows: list[dict]):
    from perf_opt.verify import WorkloadAssets
    h = tmp_path / "p_raw_harness"
    (h / "perf_inputs").mkdir(parents=True)
    for op in ops:
        (h / "perf_inputs" / f"{op}.perf.bin").write_bytes(b"x")
    (h / "golden.jsonl").write_text("", encoding="utf-8")
    (h / "perf_workload.json").write_text(json.dumps(rows), encoding="utf-8")
    return WorkloadAssets(project="p", harness_src=h,
                          golden_path=h / "golden.jsonl")


@pytest.mark.parametrize("flag", [True, None, False])
def test_flagged_ops_are_still_profiled(tmp_path: Path, flag) -> None:
    """hot_probe profiles exactly this list, so dropping an op here means any
    function hot only in that op is never found. The flag is not good enough
    evidence to pay that."""
    ops = ["good", "flagged"]
    assets = _assets(tmp_path, ops, [
        {"op": "good", "harness_bound": False},
        {"op": "flagged", "harness_bound": flag},
    ])
    assert driver._perf_ops(assets, ops) == ops


def test_ops_without_a_perf_input_are_still_functional_only(tmp_path: Path) -> None:
    assets = _assets(tmp_path, ["has"], [{"op": "has", "harness_bound": False}])
    assert driver._perf_ops(assets, ["has", "missing"]) == ["has"]


class TestLowShareDiagnosis:
    """A low `crate_share` has four causes and only one is fixable in the
    harness. Telling all four to "hoist loop-invariant digest work" sends three
    of them after a fix that cannot exist — measured in one batch: two ops with
    harness at 0% and libc's allocator on top, and one whose hottest symbol was
    library code."""

    @staticmethod
    def _hint(**row):
        from harness_gen.build_workload import _low_share_hint
        row.setdefault("attribution_reliable", True)
        row.setdefault("harness_share", 0.0)
        return _low_share_hint(row, row.pop("top", ""))

    def test_unreliable_attribution_is_named_first(self):
        h = self._hint(attribution_reliable=False, harness_share=0.99,
                       top="pkg_harness::op_x")
        assert "debug = 2" in h

    def test_harness_self_cost_gets_the_hoisting_advice(self):
        h = self._hint(harness_share=0.67, top="pkg_harness::op_x")
        assert "循环不变" in h

    @pytest.mark.parametrize("sym", ["_int_free", "_int_malloc",
                                     "__memmove_avx_unaligned_erms"])
    def test_libc_bound_is_not_blamed_on_the_harness(self, sym):
        h = self._hint(harness_share=0.0, top=sym)
        assert "进程外" in h
        assert "循环不变" not in h

    def test_library_hot_symbol_is_reported_as_such(self):
        h = self._hint(harness_share=0.0,
                       top="libxml2_raw::src::HTMLparser::htmlParseContentInternal")
        assert "库代码" in h
        assert "循环不变" not in h

    def test_a_harness_symbol_is_not_mistaken_for_library_code(self):
        h = self._hint(harness_share=0.30, top="pkg_raw_harness::op_y")
        assert "循环不变" in h

    def test_trailing_padding_in_the_symbol_is_ignored(self):
        """perf output pads symbol columns; the classifier must not see it."""
        h = self._hint(harness_share=0.0, top="_int_free            ")
        assert "进程外" in h
        assert "_int_free 属" in h          # exactly one space, not the padding
        assert "_int_free  " not in h

    def test_no_cause_identified_defers_to_hot_probe(self):
        h = self._hint(harness_share=0.02, top="")
        assert "hot_probe" in h


@pytest.mark.parametrize("text", ['{"not": "a list"}', "{ broken json", ""])
def test_unreadable_perf_workload_is_not_a_verdict(tmp_path: Path, text) -> None:
    """Absence of evidence must read as 'nothing flagged', never as a flag."""
    from perf_opt.verify import WorkloadAssets
    h = tmp_path / "p_raw_harness"
    (h / "perf_inputs").mkdir(parents=True)
    for op in ("a", "b"):
        (h / "perf_inputs" / f"{op}.perf.bin").write_bytes(b"x")
    (h / "golden.jsonl").write_text("", encoding="utf-8")
    (h / "perf_workload.json").write_text(text, encoding="utf-8")
    assets = WorkloadAssets(project="p", harness_src=h,
                            golden_path=h / "golden.jsonl")
    assert driver._low_library_share_ops(h) == {}
    assert driver._perf_ops(assets, ["a", "b"]) == ["a", "b"]


def test_missing_perf_workload_is_not_a_verdict(tmp_path: Path) -> None:
    assert driver._low_library_share_ops(tmp_path / "nope") == {}


def test_malformed_rows_are_skipped(tmp_path: Path) -> None:
    (tmp_path / "perf_workload.json").write_text(
        json.dumps([None, "junk", {"harness_bound": True},
                    {"op": "b", "harness_bound": True}]), encoding="utf-8")
    assert driver._low_library_share_ops(tmp_path) == {"b": "harness_bound"}


def test_rows_predating_the_field_are_not_flagged(tmp_path: Path) -> None:
    """Older harnesses have no `harness_bound` key at all."""
    (tmp_path / "perf_workload.json").write_text(
        json.dumps([{"op": "a"}, {"op": "b", "iters": 5}]), encoding="utf-8")
    assert driver._low_library_share_ops(tmp_path) == {}


def test_flagged_ops_are_reported_with_their_verdict(tmp_path: Path) -> None:
    ops = ["a", "b", "c"]
    assets = _assets(tmp_path, ops, [
        {"op": "a", "harness_bound": True},
        {"op": "b", "harness_bound": None},
        {"op": "c", "harness_bound": False},
    ])
    found = driver._low_library_share_ops(assets.harness_src)
    assert found == {"a": "harness_bound", "b": "unknown"}


# ───────────────────────────────────────────────── stop learning this twice

def test_debuginfo_precondition_has_exactly_one_implementation() -> None:
    """Third time a lesson landed in perf_opt and not in harness_gen (the
    iteration-count probe floor, then inline attribution). Two copies is the
    defect; assert there is one."""
    import inspect

    from perf_opt.agent_perf_opt import driver as drv

    root = Path(inspect.getfile(drv)).resolve().parents[2]
    # Writing the key back into a manifest is the implementation; naming it in
    # a log message or a docstring is not.
    writes = re.compile(r"write_text\(|\bre\.sub\([^)]*profile\\?\.release")
    authors = []
    for path in root.rglob("*.py"):
        if "/tests/" in str(path) or path.name == "build_env.py":
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        if "profile.release" not in text or "debug = 2" not in text:
            continue
        for block in text.split("\ndef "):
            if "debug = 2" in block and "profile.release" in block \
                    and writes.search(block):
                authors.append(str(path.relative_to(root)))
                break
    assert authors == [], (
        f"these re-implement the debug-info precondition instead of calling "
        f"harness_gen.build_env.ensure_debuginfo: {authors}")
