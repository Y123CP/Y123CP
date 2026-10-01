""                                                    

                                                   
                                           
            
   

from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.caller_lookup import CallerLookup
from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.resolver import (
    ResolutionRejected, sha256_bytes,
)
from perf_opt.agent_perf_opt.changeset.handlers.replace_call_site import (
    ReplaceCallSiteHandler,
)
from perf_opt.agent_perf_opt.changeset.types import ReplaceCallSite


def _make_crate(tmp_path: Path, lib_src: str) -> Path:
    (tmp_path / "src").mkdir(parents=True, exist_ok=True)
    (tmp_path / "src" / "lib.rs").write_text(lib_src, encoding="utf-8")
    return tmp_path


def _op(src: bytes, site, replacement: str, oid: str) -> ReplaceCallSite:
    span = src[site.start_byte:site.end_byte]
    return ReplaceCallSite(
        operation_id=oid,
        rule_id="III①",
        evidence_hit_ids=(),
        relative_path=site.file,
        start_byte=site.start_byte,
        end_byte=site.end_byte,
        expected_span_hash=sha256_bytes(span),
        replacement_text=replacement,
    )


def test_two_call_sites_same_file_replace_atomically(tmp_path: Path) -> None:
    lib = (
        "fn target(a: i32, b: i32) -> i32 { a + b }\n"
        "fn caller_one() -> i32 { target(1, 2) }\n"
        "fn caller_two() -> i32 { target(3, 4) }\n"
    )
    crate = _make_crate(tmp_path, lib)
    src = (crate / "src" / "lib.rs").read_bytes()

    sites = CallerLookup(crate).callers_of("target")
    assert len(sites) == 2

    handler = ReplaceCallSiteHandler()
                                    
    ops = [_op(src, s, "target2(9)", f"op-{i}") for i, s in enumerate(sites)]
    edits = tuple(handler.resolve(op, crate).edits[0] for op in ops)

    composed = ChangeSetApplier(crate, tmp_path / ".txn").compose(edits)
    out = composed["src/lib.rs"].decode()

                         
    assert "target(1, 2)" not in out
    assert "target(3, 4)" not in out
    assert out.count("target2(9)") == 2
                               
    assert "fn target(a: i32, b: i32) -> i32 { a + b }" in out


def test_stale_span_is_rejected(tmp_path: Path) -> None:
    lib = (
        "fn target(x: i32) -> i32 { x }\n"
        "fn caller() -> i32 { target(7) }\n"
    )
    crate = _make_crate(tmp_path, lib)
    src = (crate / "src" / "lib.rs").read_bytes()
    site = CallerLookup(crate).callers_of("target")[0]

    handler = ReplaceCallSiteHandler()
                                                  
    bad = ReplaceCallSite(
        operation_id="op-stale", rule_id="III①", evidence_hit_ids=(),
        relative_path=site.file, start_byte=site.start_byte, end_byte=site.end_byte,
        expected_span_hash="deadbeef" * 8, replacement_text="target(0)",
    )
    with pytest.raises(ResolutionRejected):
        handler.resolve(bad, crate)
    assert not handler.pre_validate(bad, None, crate).ok


def test_fresh_span_passes_pre_validate(tmp_path: Path) -> None:
    lib = (
        "fn target(x: i32) -> i32 { x }\n"
        "fn caller() -> i32 { target(7) }\n"
    )
    crate = _make_crate(tmp_path, lib)
    src = (crate / "src" / "lib.rs").read_bytes()
    site = CallerLookup(crate).callers_of("target")[0]
    op = _op(src, site, "target(8)", "op-fresh")
    assert handler_ok(op, crate)


def handler_ok(op, crate) -> bool:
    h = ReplaceCallSiteHandler()
    res = h.resolve(op, crate)
    return h.pre_validate(op, res, crate).ok and h.post_validate(op, res, crate).ok
