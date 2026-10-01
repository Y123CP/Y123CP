import hashlib
import os
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.types import ConcreteEdit


def _hash(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _keyword_edit(
    crate: Path,
    relative_path: str,
    occurrence: int,
    operation_id: str,
) -> ConcreteEdit:
    data = (crate / relative_path).read_bytes()
    positions = []
    cursor = 0
    while True:
        position = data.find(b"static", cursor)
        if position < 0:
            break
        positions.append(position)
        cursor = position + 1
    start = positions[occurrence]
    return ConcreteEdit(
        edit_id=f"edit-{operation_id}",
        operation_id=operation_id,
        relative_path=relative_path,
        start_byte=start,
        end_byte=start + len(b"static"),
        before_hash=_hash(data),
        replacement_text="const",
    )


def test_compose_applies_same_file_edits_from_high_to_low_offset(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    source = crate / "lib.rs"
    source.write_text("static A: u32 = 1;\nstatic B: u32 = 2;\n")
    edits = (
        _keyword_edit(crate, "lib.rs", 0, "op-a"),
        _keyword_edit(crate, "lib.rs", 1, "op-b"),
    )

    applier = ChangeSetApplier(crate, tmp_path / "journal")
    composed = applier.compose(edits)

    assert composed["lib.rs"] == b"const A: u32 = 1;\nconst B: u32 = 2;\n"
    assert source.read_text() == "static A: u32 = 1;\nstatic B: u32 = 2;\n"


def test_apply_and_restore_round_trip_across_files(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    (crate / "a.rs").write_text("static A: u32 = 1;\n")
    (crate / "b.rs").write_text("static B: u32 = 2;\n")
    edits = (
        _keyword_edit(crate, "a.rs", 0, "op-a"),
        _keyword_edit(crate, "b.rs", 0, "op-b"),
    )
    applier = ChangeSetApplier(crate, tmp_path / "journal")

    applied = applier.apply("cs-round-trip", edits)
    assert (crate / "a.rs").read_text().startswith("const A")
    assert (crate / "b.rs").read_text().startswith("const B")

    applier.restore(applied)
    assert (crate / "a.rs").read_text().startswith("static A")
    assert (crate / "b.rs").read_text().startswith("static B")
    assert not applied.journal_dir.exists()


def test_cross_file_apply_restores_all_files_when_second_replace_fails(
    tmp_path: Path,
) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    (crate / "a.rs").write_text("static A: u32 = 1;\n")
    (crate / "b.rs").write_text("static B: u32 = 2;\n")
    edits = (
        _keyword_edit(crate, "a.rs", 0, "op-a"),
        _keyword_edit(crate, "b.rs", 0, "op-b"),
    )
    calls = 0

    def fail_second(source: Path, destination: Path) -> None:
        nonlocal calls
        calls += 1
        if calls == 2:
            raise OSError("injected second replace failure")
        os.replace(source, destination)

    applier = ChangeSetApplier(
        crate, tmp_path / "journal", replace_file=fail_second
    )
    with pytest.raises(OSError, match="injected second replace failure"):
        applier.apply("cs-fail", edits)

    assert (crate / "a.rs").read_text() == "static A: u32 = 1;\n"
    assert (crate / "b.rs").read_text() == "static B: u32 = 2;\n"
    assert not (tmp_path / "journal" / "cs-fail").exists()


def test_recover_incomplete_restores_applied_transaction(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    source = crate / "lib.rs"
    source.write_text("static A: u32 = 1;\n")
    edit = _keyword_edit(crate, "lib.rs", 0, "op-a")
    journal = tmp_path / "journal"

    first_process = ChangeSetApplier(crate, journal)
    first_process.apply("cs-crash", (edit,))
    assert source.read_text().startswith("const A")

    restarted_process = ChangeSetApplier(crate, journal)
    recovered = restarted_process.recover_incomplete()

    assert recovered == ["cs-crash"]
    assert source.read_text() == "static A: u32 = 1;\n"
    assert not (journal / "cs-crash").exists()


def test_finalize_keeps_applied_files_and_removes_journal(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    crate.mkdir()
    source = crate / "lib.rs"
    source.write_text("static A: u32 = 1;\n")
    edit = _keyword_edit(crate, "lib.rs", 0, "op-a")
    journal = tmp_path / "journal"
    applier = ChangeSetApplier(crate, journal)

    applied = applier.apply("cs-commit", (edit,))
    applier.finalize(applied)

    assert source.read_text() == "const A: u32 = 1;\n"
    assert not applied.journal_dir.exists()
    assert applier.recover_incomplete() == []
