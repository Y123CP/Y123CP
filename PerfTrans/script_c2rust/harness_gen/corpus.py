"""Corpus curation + golden-output oracle (backend-agnostic).

Whatever produced the raw candidate inputs (libFuzzer, the pymut
fallback, an imported OSS-Fuzz corpus), curation is identical:

  for each candidate input, for its operation:
      run  `harness <op> <input>`  TWICE on the baseline binary
      keep it iff  both runs exit 0  AND  stdout is byte-identical
                   (deterministic) — record stdout_sha256 as golden
  then dedup by (op, stdout_sha256) and cap per-op count.

Output layout under the harness dir:

  corpus/<op>/<sha8>.bin        curated inputs, named by input sha256
  golden.jsonl                  one row per corpus input:
                                {op, input_sha256, input_path, stdout_sha256}

The golden clause is what makes this a functional oracle: replay.py can
later run ANY harness build over corpus/ and assert its stdout matches
these stdout_sha256 values. The baseline that defines golden is a CLI
arg, so it can be rolled forward as the optimizer accepts new versions.
"""

from __future__ import annotations

import hashlib
import json
import logging
import shutil
from dataclasses import dataclass, field
from pathlib import Path

from .gates import run_cmd, RUN_TIMEOUT

logger = logging.getLogger(__name__)


def sha256_bytes(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


def sha256_text(s: str) -> str:
    ""                                                          
                                                                    
                                                          
                                                    
                                  
    return sha256_bytes(s.encode("utf-8", errors="replace"))


def sha256_bytes(b: bytes) -> str:
    ""                                                        
                                                            
                         
    return hashlib.sha256(b).hexdigest()


@dataclass
class CorpusStats:
    per_op_kept: dict[str, int] = field(default_factory=dict)
    rejected_crash: int = 0
    rejected_nondet: int = 0
    rejected_dup: int = 0
    total_candidates: int = 0

    def as_dict(self) -> dict:
        return {
            "per_op_kept": self.per_op_kept,
            "kept_total": sum(self.per_op_kept.values()),
            "rejected_crash": self.rejected_crash,
            "rejected_nondet": self.rejected_nondet,
            "rejected_dup": self.rejected_dup,
            "total_candidates": self.total_candidates,
        }


def _run_twice(binary: Path, op: str, inp: Path, harness_dir: Path
               ) -> tuple[bool, str]:
    ""                                                              
                                                               
    rc1, out1, _ = run_cmd([str(binary), op, str(inp)], harness_dir, RUN_TIMEOUT)
    if rc1 != 0:
        return False, ""
    rc2, out2, _ = run_cmd([str(binary), op, str(inp)], harness_dir, RUN_TIMEOUT)
    if rc2 != 0 or out1 != out2:
        return False, ""
    return True, out1


def _run_twice_bytes(binary: Path, op: str, inp: Path, harness_dir: Path
                      ) -> tuple[bool, bytes]:
    ""                                               
                                                                  
    import subprocess
    def _run_one() -> tuple[int, bytes]:
        try:
            p = subprocess.run([str(binary), op, str(inp)],
                                cwd=str(harness_dir), timeout=RUN_TIMEOUT,
                                capture_output=True)
            return p.returncode, p.stdout
        except subprocess.TimeoutExpired:
            return 124, b""
    rc1, out1 = _run_one()
    if rc1 != 0:
        return False, b""
    rc2, out2 = _run_one()
    if rc2 != 0 or out1 != out2:
        return False, b""
    return True, out1


def curate(harness_dir: Path, baseline_bin: Path,
           candidates: dict[str, list[Path]], per_op_cap: int = 300
           ) -> tuple[list[dict], CorpusStats]:
    """candidates: {op -> [raw input files]}. Returns (golden_rows, stats)
    and writes corpus/ + golden.jsonl under harness_dir."""
    corpus_root = harness_dir / "corpus"
    if corpus_root.exists():
        shutil.rmtree(corpus_root)
    corpus_root.mkdir(parents=True)

    stats = CorpusStats()
    golden: list[dict] = []

    for op, files in candidates.items():
        op_dir = corpus_root / op
        op_dir.mkdir(exist_ok=True)
        seen_output: set[str] = set()
        seen_input: set[str] = set()
        kept = 0
        # Deterministic order → reproducible corpus selection.
        for inp in sorted(files, key=lambda p: p.name):
            if kept >= per_op_cap:
                break
            stats.total_candidates += 1
            try:
                raw = inp.read_bytes()
            except OSError:
                continue
            in_sha = sha256_bytes(raw)
            if in_sha in seen_input:
                stats.rejected_dup += 1
                continue

            ok, out = _run_twice(baseline_bin, op, inp, harness_dir)
            if not ok:
                # crash / nonzero / nondeterministic — no stable golden
                stats.rejected_crash += 1
                continue
            out_sha = sha256_text(out)
            if out_sha in seen_output:
                stats.rejected_dup += 1
                continue

            seen_input.add(in_sha)
            seen_output.add(out_sha)
            dst = op_dir / f"{in_sha[:8]}.bin"
            dst.write_bytes(raw)
            golden.append({
                "op": op,
                "input_sha256": in_sha,
                "input_path": str(dst.relative_to(harness_dir)),
                "stdout_sha256": out_sha,
            })
            kept += 1
        stats.per_op_kept[op] = kept
        logger.info("[corpus] op %-16s kept %d", op, kept)

    (harness_dir / "golden.jsonl").write_text(
        "".join(json.dumps(r) + "\n" for r in golden), encoding="utf-8")
    (harness_dir / "corpus_stats.json").write_text(
        json.dumps(stats.as_dict(), indent=2), encoding="utf-8")
    return golden, stats


def seed_candidates(harness_dir: Path, ops: list[str]) -> dict[str, list[Path]]:
    """The gen-seeds outputs are always valid candidates — fold them in so a
    corpus is never empty even if fuzzing produced nothing new."""
    seeds_dir = harness_dir / "seeds"
    out: dict[str, list[Path]] = {op: [] for op in ops}
    if not seeds_dir.exists():
        return out
    for op in ops:
        for idx in (1, 2):
            p = seeds_dir / f"{op}.{idx}.bin"
            if p.exists():
                out[op].append(p)
    return out
