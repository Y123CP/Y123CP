"""The analysis build must keep the paths its remarks are matched by.

`class_II.build` forwards the harness's `.cargo/config.toml` rustflags so the
analysis artifact has the measured binary's codegen. It first forwarded all of
them, `--remap-path-prefix` included. That flag changes no code; it rewrites
source paths in debug info and remarks, turning the crate's absolute path into
`/perf-build/crate/src/...`. `merged_hits.iter_class_ii` maps each remark back
to a function on the crate-relative path, cannot relativize a remapped one, and
drops the remark.

Measured on lz4: C3 reached fn_hits.json for 13-16 functions before the change
and for 0 after, run after run, while class_I still listed 12-16 C3 functions.
Nothing failed loudly — the loop-vectorize channel prints relative paths and
kept filling fn_hits with II_vec hits.
"""

from __future__ import annotations

from pathlib import Path

from perf_opt.hot_probe.class_II.build import _config_rustflags
from perf_opt.hot_probe.merged_hits import iter_class_ii

_CODEGEN = [
    "-Ctarget-cpu=native",
    "-Ctarget-feature=-avx512f",
    "-Cllvm-args=-align-all-functions=6",
]


def _harness(tmp_path: Path, flags: list[str]) -> Path:
    (tmp_path / ".cargo").mkdir()
    quoted = ", ".join(f'"{f}"' for f in flags)
    (tmp_path / ".cargo" / "config.toml").write_text(
        f"[build]\nrustflags = [{quoted}]\n")
    return tmp_path


def test_codegen_is_forwarded_and_remap_is_not(tmp_path: Path) -> None:
    h = _harness(tmp_path, [
        "--remap-path-prefix=/work/lz4/3_perf_opt=/perf-build", *_CODEGEN])
    assert _config_rustflags(h) == _CODEGEN


def test_the_split_spelling_of_remap_is_dropped_too(tmp_path: Path) -> None:
    h = _harness(tmp_path, [
        "--remap-path-prefix", "/work/lz4/3_perf_opt=/perf-build", *_CODEGEN])
    assert _config_rustflags(h) == _CODEGEN


class _Index:
    """The one function a remark has to be attributed to."""

    def names(self):
        return ["LZ4HC_InsertAndGetWiderMatch"]

    def resolve(self, name):
        return ("src/lz4hc.rs", 1315, 1679)


def _c3_remark(path: str) -> dict:
    return {"file": path, "line": 1345, "col": 7, "pass": "gvn",
            "status": "missed",
            "message": "load of type i32 not eliminated because it is "
                       "clobbered by call"}


def test_a_c3_remark_on_the_unremapped_path_reaches_fn_hits(
    tmp_path: Path,
) -> None:
    crate = tmp_path / "crate"
    (crate / "src").mkdir(parents=True)
    out = iter_class_ii(
        None, [_c3_remark(str(crate / "src" / "lz4hc.rs"))], _Index(),
        crate_root=crate, c3_pass_fns={"LZ4HC_InsertAndGetWiderMatch"})
    assert [h.rule for h in out["LZ4HC_InsertAndGetWiderMatch"]] == ["C3"]


def test_losing_every_c3_remark_is_reported(tmp_path: Path, caplog) -> None:
    """The silent version of this bug lasted three runs. It must be loud."""
    import logging
    from types import SimpleNamespace

    from perf_opt.hot_probe.merged_hits import merge_hits

    crate = tmp_path / "crate"
    (crate / "src").mkdir(parents=True)
    ci = SimpleNamespace(c3_hits={"LZ4HC_InsertAndGetWiderMatch"},
                         sites=[], tables=None)
    caplog.set_level(logging.WARNING)
    merge_hits(ci, None, None, [_c3_remark("/perf-build/crate/src/lz4hc.rs")],
               _Index(), crate_root=crate)
    assert any("0 个 C3 进入" in r.getMessage() for r in caplog.records)


def test_no_report_when_c3_arrives(tmp_path: Path, caplog) -> None:
    import logging
    from types import SimpleNamespace

    from perf_opt.hot_probe.merged_hits import merge_hits

    crate = tmp_path / "crate"
    (crate / "src").mkdir(parents=True)
    ci = SimpleNamespace(c3_hits={"LZ4HC_InsertAndGetWiderMatch"},
                         sites=[], tables=None)
    caplog.set_level(logging.WARNING)
    merge_hits(ci, None, None, [_c3_remark(str(crate / "src" / "lz4hc.rs"))],
               _Index(), crate_root=crate)
    assert not any("0 个 C3 进入" in r.getMessage() for r in caplog.records)


def test_losing_every_ii_inl_remark_is_reported_too(tmp_path: Path, caplog) -> None:
    """II_inl rode the same channel and fell from 28 functions to 0 with C3."""
    import logging
    from types import SimpleNamespace

    from perf_opt.hot_probe.merged_hits import merge_hits

    crate = tmp_path / "crate"
    (crate / "src").mkdir(parents=True)
    cii = SimpleNamespace(ii_inl_hits={}, ii_vec_hits={},
                          ii_inl_candidates={"LZ4HC_InsertAndGetWiderMatch": 1})
    remark = {"file": "/perf-build/crate/src/lz4hc.rs", "line": 1345, "col": 7,
              "pass": "inline", "status": "missed",
              "message": "'LZ4HC_countBack' not inlined into "
                         "'LZ4HC_InsertAndGetWiderMatch' because too costly "
                         "to inline (cost=400, threshold=325)"}
    caplog.set_level(logging.WARNING)
    merge_hits(None, cii, None, [remark], _Index(), crate_root=crate)
    assert any("0 个 II_inl 进入" in r.getMessage() for r in caplog.records)
