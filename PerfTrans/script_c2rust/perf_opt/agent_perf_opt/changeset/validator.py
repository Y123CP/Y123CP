"""Validation shared by all typed ChangeSet operations."""

from __future__ import annotations

from collections import Counter
from pathlib import Path

from .resolver import resolve_project_path
from .types import (
    ConcreteEdit,
    ReplaceSourceRegion,
    ResolvedChangeSet,
    ValidationResult,
)


def validate_operation_edit_bijection(
    operation_ids: tuple[str, ...],
    edits: tuple[ConcreteEdit, ...],
) -> ValidationResult:
    mapped = [edit.operation_id for edit in edits]
    known = set(operation_ids)
    counts = Counter(mapped)
    unknown = sorted(set(mapped) - known)
    missing = sorted(known - set(mapped))
    duplicate = sorted(
        operation_id
        for operation_id, count in counts.items()
        if operation_id in known and count != 1
    )
    if unknown or missing or duplicate:
        return ValidationResult(
            False,
            "operation_edit_bijection",
            f"unknown={unknown}, missing={missing}, duplicate={duplicate}",
        )
    return ValidationResult(True, "operation_edit_bijection")


def validate_region_file_isolation(
    crate: Path,
    resolved: ResolvedChangeSet,
) -> ValidationResult:
    """Require a region edit to be the only edit targeting its source file."""
    canonical_edits: list[tuple[ConcreteEdit, Path | None]] = []
    for edit in resolved.edits:
        try:
            canonical = resolve_project_path(crate, edit.relative_path)
        except (OSError, RuntimeError, ValueError):
            canonical = None
        canonical_edits.append((edit, canonical))

    failures: list[str] = []
    for operation in resolved.proposal.operations:
        if not isinstance(operation, ReplaceSourceRegion):
            continue
        path = operation.region.relative_path
        matching_resolutions = [
            resolution
            for resolution in resolved.resolutions
            if resolution.operation_id == operation.operation_id
        ]
        own_edits = (
            matching_resolutions[0].edits
            if len(matching_resolutions) == 1
            else ()
        )
        try:
            canonical_path = resolve_project_path(crate, path)
        except (OSError, RuntimeError, ValueError):
            canonical_path = None
        path_edits = [
            edit
            for edit, canonical in canonical_edits
            if canonical_path is not None and canonical == canonical_path
        ]
        if (
            len(matching_resolutions) != 1
            or canonical_path is None
            or len(own_edits) != 1
            or own_edits[0].operation_id != operation.operation_id
            or not any(
                edit == own_edits[0] and canonical == canonical_path
                for edit, canonical in canonical_edits
            )
            or len(path_edits) != 1
            or path_edits[0] != own_edits[0]
        ):
            failures.append(f"{operation.operation_id}:{path}")

    if failures:
        return ValidationResult(
            False,
            "region_file_not_isolated",
            "region source file has another or mismatched edit: "
            + ", ".join(sorted(failures)),
        )
    return ValidationResult(True, "region_file_isolated")
