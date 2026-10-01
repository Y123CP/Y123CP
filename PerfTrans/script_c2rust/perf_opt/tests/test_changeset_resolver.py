import hashlib
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.changeset.resolver import (
    resolve_project_path,
    sha256_bytes,
    validate_before_hash,
    validate_edit_ranges,
)
from perf_opt.agent_perf_opt.changeset.types import ConcreteEdit
from perf_opt.agent_perf_opt.changeset.validator import (
    validate_operation_edit_bijection,
)


def _write(crate: Path, relative_path: str, data: bytes) -> str:
    path = crate / relative_path
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    return hashlib.sha256(data).hexdigest()


def _edit(
    *,
    relative_path: str = "src/lib.rs",
    operation_id: str = "op-1",
    edit_id: str = "edit-1",
    start: int = 0,
    end: int = 6,
    before_hash: str,
) -> ConcreteEdit:
    return ConcreteEdit(
        edit_id=edit_id,
        operation_id=operation_id,
        relative_path=relative_path,
        start_byte=start,
        end_byte=end,
        before_hash=before_hash,
        replacement_text="const",
    )


def test_resolve_project_path_rejects_absolute_and_parent_escape(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()

    with pytest.raises(ValueError, match="relative"):
        resolve_project_path(crate, str(tmp_path / "outside.rs"))
    with pytest.raises(ValueError, match="outside crate"):
        resolve_project_path(crate, "../outside.rs")


def test_validate_before_hash_detects_stale_file(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    digest = _write(crate, "src/lib.rs", b"static A: u32 = 1;\n")
    edit = _edit(before_hash=digest)
    assert validate_before_hash(crate, edit).ok

    (crate / "src/lib.rs").write_bytes(b"static A: u32 = 2;\n")
    stale = validate_before_hash(crate, edit)
    assert not stale.ok
    assert stale.code == "stale_before_hash"


def test_validate_edit_ranges_uses_utf8_byte_offsets(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    data = "// 常量\nstatic A: u32 = 1;\n".encode()
    digest = _write(crate, "src/lib.rs", data)
    keyword_start = data.index(b"static")
    edit = _edit(
        start=keyword_start,
        end=keyword_start + len(b"static"),
        before_hash=digest,
    )

    result = validate_edit_ranges(crate, (edit,))
    assert result.ok, result.detail


def test_validate_edit_ranges_rejects_overlap_and_out_of_bounds(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    data = b"static A: u32 = 1;\n"
    digest = _write(crate, "src/lib.rs", data)
    overlap = (
        _edit(start=0, end=6, before_hash=digest),
        _edit(
            operation_id="op-2",
            edit_id="edit-2",
            start=5,
            end=10,
            before_hash=digest,
        ),
    )

    result = validate_edit_ranges(crate, overlap)
    assert not result.ok
    assert result.code == "overlapping_edits"

    out_of_bounds = _edit(start=0, end=len(data) + 1, before_hash=digest)
    result = validate_edit_ranges(crate, (out_of_bounds,))
    assert not result.ok
    assert result.code == "edit_out_of_bounds"


def test_validate_operation_edit_bijection_reports_missing_unknown_and_duplicate(
    tmp_path: Path,
) -> None:
    crate = tmp_path / "crate"
    digest = _write(crate, "src/lib.rs", b"static A: u32 = 1;\n")
    edits = (
        _edit(operation_id="op-1", edit_id="edit-1", before_hash=digest),
        _edit(operation_id="op-1", edit_id="edit-2", before_hash=digest),
        _edit(operation_id="op-extra", edit_id="edit-3", before_hash=digest),
    )

    result = validate_operation_edit_bijection(("op-1", "op-2"), edits)
    assert not result.ok
    assert result.code == "operation_edit_bijection"
    assert "missing=['op-2']" in result.detail
    assert "unknown=['op-extra']" in result.detail
    assert "duplicate=['op-1']" in result.detail


def test_sha256_bytes_matches_hashlib() -> None:
    data = b"declaration bytes"
    assert sha256_bytes(data) == hashlib.sha256(data).hexdigest()
