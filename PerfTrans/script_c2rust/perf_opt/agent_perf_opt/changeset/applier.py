"""Recoverable multi-file application of already validated concrete edits."""

from __future__ import annotations

import json
import logging
import os
import shutil
import stat
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Callable

logger = logging.getLogger("agent_perf_opt.changeset.applier")

from .resolver import (
    resolve_project_path,
    sha256_bytes,
    validate_edit_ranges,
)
from .types import ConcreteEdit


class RollbackVerificationError(RuntimeError):
    pass


@dataclass(frozen=True)
class AppliedFiles:
    changeset_id: str
    journal_dir: Path
    touched_paths: tuple[str, ...]
    before_hashes: dict[str, str]
    after_hashes: dict[str, str]


class ChangeSetApplier:
    def __init__(
        self,
        crate: Path,
        journal_root: Path,
        *,
        replace_file: Callable[[Path, Path], None] | None = None,
    ) -> None:
        self.crate = crate.resolve()
        self.journal_root = journal_root.resolve()
        self._replace_file = replace_file or self._default_replace

    @staticmethod
    def _default_replace(source: Path, destination: Path) -> None:
        os.replace(source, destination)

    @staticmethod
    def _validate_changeset_id(changeset_id: str) -> None:
        if (
            not changeset_id
            or changeset_id in {".", ".."}
            or Path(changeset_id).name != changeset_id
        ):
            raise ValueError(f"unsafe changeset_id: {changeset_id!r}")

    def compose(self, edits: tuple[ConcreteEdit, ...]) -> dict[str, bytes]:
        validation = validate_edit_ranges(self.crate, edits)
        if not validation.ok:
            raise ValueError(f"{validation.code}: {validation.detail}")

        by_path: dict[str, list[ConcreteEdit]] = defaultdict(list)
        for edit in edits:
            by_path[edit.relative_path].append(edit)

        composed: dict[str, bytes] = {}
        for relative_path, file_edits in sorted(by_path.items()):
            path = resolve_project_path(self.crate, relative_path)
            data = path.read_bytes()
            for edit in sorted(
                file_edits,
                key=lambda item: (item.start_byte, item.end_byte),
                reverse=True,
            ):
                replacement = edit.replacement_text.encode("utf-8")
                data = data[: edit.start_byte] + replacement + data[edit.end_byte :]
            composed[relative_path] = data
        return composed

    def apply(
        self, changeset_id: str, edits: tuple[ConcreteEdit, ...]
    ) -> AppliedFiles:
        self._validate_changeset_id(changeset_id)
        outputs = self.compose(edits)
        if not outputs:
            raise ValueError("changeset has no concrete edits")

        self.journal_root.mkdir(parents=True, exist_ok=True)
        journal_dir = self.journal_root / changeset_id
        if journal_dir.exists():
            raise FileExistsError(f"changeset journal already exists: {journal_dir}")
        backup_dir = journal_dir / "backups"
        backup_dir.mkdir(parents=True)

        entries: list[dict[str, object]] = []
        before_hashes: dict[str, str] = {}
        after_hashes: dict[str, str] = {}
        for index, (relative_path, output) in enumerate(sorted(outputs.items())):
            target = resolve_project_path(self.crate, relative_path)
            original = target.read_bytes()
            before_hash = sha256_bytes(original)
            after_hash = sha256_bytes(output)
            backup_relative = f"backups/{index:04d}.bin"
            backup = journal_dir / backup_relative
            self._write_and_sync(backup, original)
            mode = stat.S_IMODE(target.stat().st_mode)
            entries.append(
                {
                    "relative_path": relative_path,
                    "backup": backup_relative,
                    "before_hash": before_hash,
                    "after_hash": after_hash,
                    "mode": mode,
                }
            )
            before_hashes[relative_path] = before_hash
            after_hashes[relative_path] = after_hash

        manifest: dict[str, object] = {
            "schema_version": 1,
            "changeset_id": changeset_id,
            "status": "prepared",
            "entries": entries,
        }
        self._write_manifest(journal_dir, manifest)

        try:
            for entry in entries:
                relative_path = str(entry["relative_path"])
                target = resolve_project_path(self.crate, relative_path)
                self._replace_target(
                    target,
                    outputs[relative_path],
                    int(entry["mode"]),
                    changeset_id,
                )
            manifest["status"] = "applied"
            self._write_manifest(journal_dir, manifest)
        except BaseException:
            self._restore_manifest(journal_dir)
            raise

        return AppliedFiles(
            changeset_id=changeset_id,
            journal_dir=journal_dir,
            touched_paths=tuple(sorted(outputs)),
            before_hashes=before_hashes,
            after_hashes=after_hashes,
        )

    def restore(self, applied: AppliedFiles) -> None:
        if applied.journal_dir != self.journal_root / applied.changeset_id:
            raise ValueError("AppliedFiles belongs to a different journal root")
        if not (applied.journal_dir / "manifest.json").is_file():
            # Restore has to be idempotent. Both `restore_and_verify` and
            # `finalize` delete the journal once they succeed, and the
            # executor's `except` path can be reached AFTER either of them has
            # run — a rewrite that is rolled back and then raises on the way
            # out arrives here a second time. Re-reading the manifest then
            # turns an ordinary, already-handled rewrite failure into an
            # uncaught FileNotFoundError that takes the whole run down with
            # it. A missing journal means the undo already happened, or the
            # commit made it moot; either way there is nothing left to undo.
            logger.info(
                "[changeset] restore %s: journal already gone — nothing to "
                "undo (rolled back or committed earlier)",
                applied.changeset_id)
            return
        self._restore_manifest(applied.journal_dir)

    def finalize(self, applied: AppliedFiles) -> None:
        manifest = self._read_manifest(applied.journal_dir)
        for entry in self._entries(manifest):
            relative_path = str(entry["relative_path"])
            target = resolve_project_path(self.crate, relative_path)
            actual = sha256_bytes(target.read_bytes())
            expected = str(entry["after_hash"])
            if actual != expected:
                raise RollbackVerificationError(
                    f"cannot finalize {relative_path}: expected {expected}, got {actual}"
                )
        manifest["status"] = "committed"
        self._write_manifest(applied.journal_dir, manifest)
        shutil.rmtree(applied.journal_dir)

    def recover_incomplete(self) -> list[str]:
        if not self.journal_root.is_dir():
            return []
        recovered: list[str] = []
        for journal_dir in sorted(
            path for path in self.journal_root.iterdir() if path.is_dir()
        ):
            manifest_path = journal_dir / "manifest.json"
            if not manifest_path.is_file():
                continue
            manifest = self._read_manifest(journal_dir)
            if manifest.get("status") == "committed":
                shutil.rmtree(journal_dir)
                continue
            changeset_id = str(manifest["changeset_id"])
            self._restore_manifest(journal_dir)
            recovered.append(changeset_id)
        return recovered

    @staticmethod
    def _entries(manifest: dict[str, object]) -> list[dict[str, object]]:
        entries = manifest.get("entries")
        if not isinstance(entries, list) or not all(
            isinstance(entry, dict) for entry in entries
        ):
            raise ValueError("invalid changeset manifest entries")
        return entries

    def _restore_manifest(self, journal_dir: Path) -> None:
        manifest = self._read_manifest(journal_dir)
        entries = self._entries(manifest)
        for entry in entries:
            relative_path = str(entry["relative_path"])
            target = resolve_project_path(self.crate, relative_path)
            backup = journal_dir / str(entry["backup"])
            original = backup.read_bytes()
            self._replace_target_direct(
                target,
                original,
                int(entry["mode"]),
                str(manifest["changeset_id"]),
            )

        failures: list[str] = []
        for entry in entries:
            relative_path = str(entry["relative_path"])
            target = resolve_project_path(self.crate, relative_path)
            actual = sha256_bytes(target.read_bytes())
            expected = str(entry["before_hash"])
            if actual != expected:
                failures.append(
                    f"{relative_path}: expected {expected}, got {actual}"
                )
        if failures:
            raise RollbackVerificationError("; ".join(failures))
        shutil.rmtree(journal_dir)

    def _replace_target(
        self, target: Path, data: bytes, mode: int, changeset_id: str
    ) -> None:
        temporary = target.parent / f".{target.name}.{changeset_id}.tmp"
        try:
            self._write_and_sync(temporary, data, mode=mode)
            self._replace_file(temporary, target)
        finally:
            temporary.unlink(missing_ok=True)

    def _replace_target_direct(
        self, target: Path, data: bytes, mode: int, changeset_id: str
    ) -> None:
        temporary = target.parent / f".{target.name}.{changeset_id}.restore.tmp"
        try:
            self._write_and_sync(temporary, data, mode=mode)
            os.replace(temporary, target)
        finally:
            temporary.unlink(missing_ok=True)

    @staticmethod
    def _write_and_sync(path: Path, data: bytes, *, mode: int | None = None) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        if mode is not None:
            path.chmod(mode)

    @staticmethod
    def _read_manifest(journal_dir: Path) -> dict[str, object]:
        data = json.loads((journal_dir / "manifest.json").read_text(encoding="utf-8"))
        if not isinstance(data, dict):
            raise ValueError("changeset manifest must be a JSON object")
        return data

    @staticmethod
    def _write_manifest(journal_dir: Path, manifest: dict[str, object]) -> None:
        temporary = journal_dir / "manifest.json.tmp"
        payload = json.dumps(manifest, ensure_ascii=False, indent=2, sort_keys=True)
        with temporary.open("w", encoding="utf-8") as stream:
            stream.write(payload)
            stream.write("\n")
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, journal_dir / "manifest.json")
