"""Common path, content identity, and concrete-edit checks."""

from __future__ import annotations

import hashlib
from collections import defaultdict
from pathlib import Path

from .types import ChangeSetStatus, ConcreteEdit, ValidationResult


class ResolutionRejected(ValueError):
    def __init__(self, status: ChangeSetStatus, detail: str) -> None:
        super().__init__(detail)
        self.status = status


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def resolve_project_path(crate: Path, relative_path: str) -> Path:
    relative = Path(relative_path)
    if relative.is_absolute():
        raise ValueError(f"project path must be relative: {relative_path}")

    crate_root = crate.resolve()
    resolved = (crate_root / relative).resolve()
    try:
        resolved.relative_to(crate_root)
    except ValueError as exc:
        raise ValueError(f"project path resolves outside crate: {relative_path}") from exc
    return resolved


def validate_before_hash(crate: Path, edit: ConcreteEdit) -> ValidationResult:
    try:
        path = resolve_project_path(crate, edit.relative_path)
    except ValueError as exc:
        return ValidationResult(False, "invalid_edit_path", str(exc))
    try:
        current = path.read_bytes()
    except OSError as exc:
        return ValidationResult(False, "missing_edit_file", str(exc))

    current_hash = sha256_bytes(current)
    if current_hash != edit.before_hash:
        return ValidationResult(
            False,
            "stale_before_hash",
            f"{edit.relative_path}: expected {edit.before_hash}, got {current_hash}",
        )
    return ValidationResult(True, "before_hash")


def validate_edit_ranges(
    crate: Path, edits: tuple[ConcreteEdit, ...]
) -> ValidationResult:
    edit_ids = [edit.edit_id for edit in edits]
    if len(edit_ids) != len(set(edit_ids)):
        return ValidationResult(False, "duplicate_edit_id", str(edit_ids))

    by_path: dict[str, list[ConcreteEdit]] = defaultdict(list)
    for edit in edits:
        by_path[edit.relative_path].append(edit)

    for relative_path, file_edits in sorted(by_path.items()):
        try:
            path = resolve_project_path(crate, relative_path)
        except ValueError as exc:
            return ValidationResult(False, "invalid_edit_path", str(exc))
        try:
            data = path.read_bytes()
        except OSError as exc:
            return ValidationResult(False, "missing_edit_file", str(exc))

        current_hash = sha256_bytes(data)
        for edit in file_edits:
            if edit.before_hash != current_hash:
                return ValidationResult(
                    False,
                    "stale_before_hash",
                    f"{relative_path}: expected {edit.before_hash}, got {current_hash}",
                )
            if not (0 <= edit.start_byte <= edit.end_byte <= len(data)):
                return ValidationResult(
                    False,
                    "edit_out_of_bounds",
                    f"{edit.edit_id}: [{edit.start_byte}, {edit.end_byte}) "
                    f"for {len(data)} bytes",
                )

        ordered = sorted(file_edits, key=lambda edit: (edit.start_byte, edit.end_byte))
        for previous, current in zip(ordered, ordered[1:]):
            if current.start_byte < previous.end_byte:
                return ValidationResult(
                    False,
                    "overlapping_edits",
                    f"{previous.edit_id} overlaps {current.edit_id} in {relative_path}",
                )

    return ValidationResult(True, "edit_ranges")
