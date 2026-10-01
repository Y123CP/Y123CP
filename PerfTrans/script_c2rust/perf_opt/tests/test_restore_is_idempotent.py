"""Rolling back twice must not kill the run.

Both `restore_and_verify` and `finalize` delete the journal directory once
they succeed. The executor's `except` path calls `restore(applied)` and can be
reached AFTER either of them has already run — a rewrite that is rolled back
and then raises on the way out arrives there a second time.

Re-reading `manifest.json` then raises FileNotFoundError from inside the error
handler, which is not caught anywhere: an ordinary, already-handled rewrite
failure takes the entire pipeline run down with it. Measured on fzy, where it
ended the run at the first function that reached this path.
"""

from __future__ import annotations

import json

import pytest

from perf_opt.agent_perf_opt.changeset.applier import (
    AppliedFiles, ChangeSetApplier,
)


@pytest.fixture()
def applier(tmp_path):
    crate = tmp_path / "crate"
    (crate / "src").mkdir(parents=True)
    (crate / "src" / "a.rs").write_text("fn a() {}\n", encoding="utf-8")
    return ChangeSetApplier(crate=crate,
                            journal_root=tmp_path / ".changeset_txn")


def _applied(applier, changeset_id="cs-1") -> AppliedFiles:
    return AppliedFiles(
        changeset_id=changeset_id,
        journal_dir=applier.journal_root / changeset_id,
        touched_paths=("src/a.rs",),
        before_hashes={},
        after_hashes={},
    )


def test_restore_on_a_vanished_journal_is_a_no_op(applier) -> None:
    applier.restore(_applied(applier))          # must not raise


def test_restore_on_an_empty_journal_dir_is_a_no_op(applier) -> None:
    """`rmtree` may have raced, or a partial apply left the dir behind."""
    a = _applied(applier)
    a.journal_dir.mkdir(parents=True)
    applier.restore(a)


def test_a_mismatched_journal_root_still_raises(applier, tmp_path) -> None:
    """The idempotence must not swallow a genuine programming error."""
    bad = AppliedFiles(
        changeset_id="cs-1",
        journal_dir=tmp_path / "elsewhere" / "cs-1",
        touched_paths=(),
        before_hashes={},
        after_hashes={},
    )
    with pytest.raises(ValueError):
        applier.restore(bad)


def test_a_present_manifest_is_still_honoured(applier) -> None:
    """The real path must be untouched: a live journal still restores."""
    import hashlib
    crate = applier.crate
    target = crate / "src" / "a.rs"
    original = target.read_bytes()
    before = hashlib.sha256(original).hexdigest()

    a = _applied(applier)
    a.journal_dir.mkdir(parents=True)
    (a.journal_dir / "backup0").write_bytes(original)
    (a.journal_dir / "manifest.json").write_text(json.dumps({
        "changeset_id": "cs-1",
        "entries": [{
            "relative_path": "src/a.rs",
            "backup": "backup0",
            "mode": 0o644,
            "before_hash": before,
            "after_hash": hashlib.sha256(b"// clobbered\n").hexdigest(),
        }],
    }), encoding="utf-8")

    target.write_bytes(b"// clobbered\n")
    applier.restore(a)
    assert target.read_bytes() == original
