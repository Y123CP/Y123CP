"""Durable JSON audit records for ChangeSet state transitions and results."""

from __future__ import annotations

import json
from dataclasses import asdict
from enum import Enum
from pathlib import Path
from typing import Any

from .types import AppliedChangeSet, ChangeSetStatus


def _json_default(value: object) -> object:
    if isinstance(value, Enum):
        return value.value
    if isinstance(value, Path):
        return str(value)
    raise TypeError(f"not JSON serializable: {type(value).__name__}")


class AuditWriter:
    def __init__(self, root: Path) -> None:
        self.root = root.resolve()
        self.changesets_dir = self.root / "changesets"
        self.changesets_dir.mkdir(parents=True, exist_ok=True)
        self.summary_path = self.root / "changesets.jsonl"

    def write_event(
        self,
        changeset_id: str,
        status: ChangeSetStatus,
        detail: dict[str, Any] | None = None,
    ) -> None:
        record = {
            "changeset_id": changeset_id,
            "status": status.value,
            "detail": detail or {},
        }
        directory = self.changesets_dir / changeset_id
        directory.mkdir(parents=True, exist_ok=True)
        self._append_json(directory / "events.jsonl", record)

    def write_result(self, result: AppliedChangeSet) -> None:
        if result.resolved is None:
            raise ValueError("audited result requires a resolved changeset")
        changeset_id = result.resolved.proposal.changeset_id
        payload = asdict(result)
        payload["applied_rules"] = (
            [operation.rule_id for operation in result.resolved.proposal.operations]
            if result.commit_sha is not None else []
        )
        payload["touched_paths"] = sorted({
            edit.relative_path
            for resolution in result.resolved.resolutions
            for edit in resolution.edits
        })
        directory = self.changesets_dir / changeset_id
        directory.mkdir(parents=True, exist_ok=True)
        serialized = json.dumps(
            payload,
            ensure_ascii=False,
            indent=2,
            sort_keys=True,
            default=_json_default,
        )
        (directory / "result.json").write_text(serialized + "\n", encoding="utf-8")
        summary = {
            "changeset_id": changeset_id,
            "rule_id": result.resolved.proposal.trigger.rule_id,
            "terminal_status": result.terminal_status.value,
            "commit_sha": result.commit_sha,
            "rollback_verified": result.rollback_verified,
        }
        self._append_json(self.summary_path, summary)

    def validate_recovery(
        self,
        changeset_id: str,
        commit_sha: str,
        candidate_binary_sha256: str | None = None,
    ) -> dict[str, Any]:
        result_path = self.changesets_dir / changeset_id / "result.json"
        if not result_path.is_file():
            raise ValueError(f"missing audit result for {changeset_id}")
        payload = json.loads(result_path.read_text(encoding="utf-8"))
        if payload.get("commit_sha") != commit_sha:
            raise ValueError(
                f"recovery commit mismatch: {payload.get('commit_sha')} != {commit_sha}"
            )
        if payload.get("terminal_status") != ChangeSetStatus.FAILED_INTERNAL.value:
            raise ValueError(
                f"changeset {changeset_id} is not awaiting recovery: "
                f"{payload.get('terminal_status')}"
            )
        if candidate_binary_sha256 is not None:
            events_path = self.changesets_dir / changeset_id / "events.jsonl"
            events = [
                json.loads(line)
                for line in events_path.read_text(encoding="utf-8").splitlines()
            ]
            commit_events = [
                event for event in events
                if event.get("status") == ChangeSetStatus.COMMIT_SUCCEEDED.value
            ]
            if not commit_events:
                raise ValueError(f"missing commit_succeeded event for {changeset_id}")
            detail = commit_events[-1].get("detail", {})
            if detail.get("commit_sha") != commit_sha:
                raise ValueError("commit_succeeded event does not match recovery commit")
            expected_hash = detail.get("candidate_binary_sha256")
            if expected_hash != candidate_binary_sha256:
                raise ValueError(
                    f"recovery binary hash mismatch: {candidate_binary_sha256} "
                    f"!= {expected_hash}"
                )
        return payload

    def mark_recovered_after_commit(
        self, changeset_id: str, commit_sha: str
    ) -> None:
        payload = self.validate_recovery(changeset_id, commit_sha)
        result_path = self.changesets_dir / changeset_id / "result.json"
        payload["terminal_status"] = ChangeSetStatus.RECOVERED_AFTER_COMMIT.value
        payload["rollback_verified"] = False
        result_path.write_text(
            json.dumps(
                payload, ensure_ascii=False, indent=2, sort_keys=True,
                default=_json_default,
            ) + "\n",
            encoding="utf-8",
        )
        self.write_event(
            changeset_id,
            ChangeSetStatus.RECOVERED_AFTER_COMMIT,
            {"commit_sha": commit_sha},
        )
        self._append_json(self.summary_path, {
            "changeset_id": changeset_id,
            "rule_id": payload["resolved"]["proposal"]["trigger"]["rule_id"],
            "terminal_status": ChangeSetStatus.RECOVERED_AFTER_COMMIT.value,
            "commit_sha": commit_sha,
            "rollback_verified": False,
        })

    @staticmethod
    def _append_json(path: Path, record: dict[str, Any]) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("a", encoding="utf-8") as stream:
            stream.write(
                json.dumps(record, ensure_ascii=False, sort_keys=True, default=_json_default)
            )
            stream.write("\n")
