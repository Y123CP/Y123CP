"""W1' differential functional gate.

Given a harness build (any version: baseline, or an optimizer's candidate
rewrite of the crate under test) and a golden.jsonl, replay every corpus
input through the harness and assert its stdout sha256 matches golden.

This is the gate the optimization loop calls after every rewrite:

    ok, report = replay(harness_dir, candidate_bin, golden_path)
    if not ok: reject the rewrite (report.mismatches lists (op, input))

`regen_golden` re-derives golden.jsonl from a (new) baseline binary over
the EXISTING corpus — used to roll the oracle forward when a rewrite is
accepted, so the corpus stays fixed while expected outputs advance.
"""

from __future__ import annotations

import json
import logging
from dataclasses import dataclass, field
from pathlib import Path

from .corpus import sha256_text
from .gates import run_cmd, RUN_TIMEOUT

logger = logging.getLogger(__name__)


@dataclass
class ReplayReport:
    total: int = 0
    matched: int = 0
    mismatches: list[dict] = field(default_factory=list)   # {op,input,expected,got}
    errors: list[dict] = field(default_factory=list)       # {op,input,rc,stderr}

    @property
    def ok(self) -> bool:
        return not self.mismatches and not self.errors and self.total > 0

    def as_dict(self) -> dict:
        return {
            "ok": self.ok, "total": self.total, "matched": self.matched,
            "mismatches": self.mismatches[:50], "errors": self.errors[:50],
            "mismatch_count": len(self.mismatches),
            "error_count": len(self.errors),
        }


def load_golden(golden_path: Path) -> list[dict]:
    rows = []
    for ln in golden_path.read_text(encoding="utf-8").splitlines():
        ln = ln.strip()
        if ln:
            rows.append(json.loads(ln))
    return rows


def replay(harness_dir: Path, binary: Path, golden_path: Path,
           iters: int = 1) -> ReplayReport:
    rep = ReplayReport()
    for row in load_golden(golden_path):
        rep.total += 1
        inp = harness_dir / row["input_path"]
        cmd = [str(binary), row["op"], str(inp)]
        if iters != 1:
            cmd.append(str(iters))
        rc, out, err = run_cmd(cmd, harness_dir, RUN_TIMEOUT)
        if rc != 0:
            rep.errors.append({"op": row["op"], "input": row["input_path"],
                               "rc": rc, "stderr": err[-500:]})
            continue
        got = sha256_text(out)
        expected = row["stdout_sha256"] if iters == 1 else None
        if iters == 1 and got != expected:
            rep.mismatches.append({"op": row["op"], "input": row["input_path"],
                                   "expected": expected, "got": got})
        else:
            rep.matched += 1
    return rep


def regen_golden(harness_dir: Path, baseline_bin: Path,
                 golden_path: Path) -> int:
    ""                                                                     
                                                                          
                                                                         

                                                                      
                                                        
                                                      
       
    from .corpus import _run_twice_bytes, sha256_bytes
    rows = load_golden(golden_path)
    out_rows, dropped = [], 0
    for row in rows:
        inp = harness_dir / row["input_path"]
        ok, out = _run_twice_bytes(baseline_bin, row["op"], inp, harness_dir)
        if not ok:
            dropped += 1
            logger.warning("[regen] dropping %s (%s): new baseline unstable",
                           row["input_path"], row["op"])
            continue
        row["stdout_sha256"] = sha256_bytes(out)
        out_rows.append(row)
    golden_path.write_text("".join(json.dumps(r) + "\n" for r in out_rows),
                           encoding="utf-8")
    if dropped:
        logger.warning("[regen] dropped %d/%d inputs", dropped, len(rows))
    return len(out_rows)
