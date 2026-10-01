"""Resolve recorded experiment IDs without changing anonymized Git history."""

import json
import subprocess
from pathlib import Path


MAP_PATH = Path(__file__).resolve().parents[1] / "commit_id_map.json"


def commits(run: Path) -> list[tuple[str, str]]:
    """Return accepted rewrites with their original measurement-record IDs."""
    arm = {"3_perf_opt": "PerfTrans", "3_perf_opt_freeform": "PerfTrans_NoRule"}[run.name]
    entries = {
        row["artifact_sha"]: row
        for row in json.loads(MAP_PATH.read_text())
        if row["project"] == run.parent.name and row["arm"] == arm
    }
    output = subprocess.run(
        ["git", "-C", str(run / "crate"), "log", "--reverse", "--format=%H\t%s"],
        capture_output=True, text=True, check=True,
    ).stdout
    result = []
    for line in output.splitlines():
        sha, subject = line.split("\t", 1)
        if not subject.startswith("agent changeset "):
            continue
        entry = entries.get(sha)
        if entry is None or entry["subject"] != subject:
            raise ValueError(f"Unmapped or mismatched experiment commit: {run.parent.name}/{run.name}/{sha}")
        result.append((entry["recorded_sha"], subject))
    return result
