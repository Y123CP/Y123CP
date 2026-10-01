"""agent_perf_opt state — git commit/rollback + audit log。

Per D9: git commit-per-rewrite is both the rollback mechanism and the
audit trail. Each accepted rewrite → one commit; each rejected rewrite
→ no commit (working tree reset to HEAD).

Per §8.3: use `git add <file>` NOT `git add .` — the latter would grab
target/ artifacts. commit message format:
    agent: <fn>/<rule>/att<n> — <status> (W2 <delta:+.2f>%)

Audit log written to `<opt_dir>/rewrites.log` (JSONL, one AttemptRecord
per line — appended, never rewritten).
"""

from __future__ import annotations

import json
import logging
import subprocess
from copy import deepcopy
from dataclasses import asdict, dataclass, field
from enum import Enum
from pathlib import Path
from typing import Iterable, Optional

from .reporting import validate_attempt_trace

logger = logging.getLogger(__name__)


# ─── attempt-layer status enum(business view; RewriteStatus in
# rewrite_applier.py is the apply-layer view — see impl_plan §2.6/P2)──────
class RewriteAttempt(Enum):
    APPLIED_COMMITTED = "applied_committed"
    ABSTAINED         = "abstained"
    SYNTAX_ERROR      = "syntax_error"
    W1_FAIL           = "w1_fail"
    W2_REGRESS        = "w2_regress"
    BUDGET_EXCEEDED   = "budget_exceeded"


@dataclass
class AttemptRecord:
    ""                                                 

                                                 
                                                   
       
    fn_name:      str
    rule_id:      str                                                      
    round_no:     int                                                             
    attempt_no:   int                              # 1..cfg.max_attempts_per_fn
    status:       RewriteAttempt
    w1_result:    Optional[str] = None             # "pass" / "mismatch:<op>"
    w2_delta_pct: Optional[float] = None                   
    commit_sha:   Optional[str] = None
    tokens_in:    int = 0                          # tiktoken estimate
    tokens_out:   int = 0                          # tiktoken estimate
    error:        Optional[str] = None
    reason:       Optional[str] = None                             
                                       
    fired_rules:  Optional[list[str]] = None                       
    applied_rules: Optional[list[str]] = None                                  
                                                       
                                                               
    skipped_rules: Optional[list[list[str]]] = None
    plan_json:    Optional[dict] = None                                               
    original_plan_json: Optional[dict] = None                          
    fn_mode:      Optional[str] = None             # "simple"|"complex"|"fallback"
    v1_abstained_cross_fn: Optional[list[dict]] = None                           
                                                                   
    measured_cv:  Optional[float] = None           # W2Verdict.measured_cv(%)
    terminal_status: Optional[str] = None
    attempt_trace: Optional[list[dict]] = None
    anchor_hit_ids: Optional[list[str]] = None     # Region candidate evidence anchors
    # A post-validation rejection's finding: which guard, and the expression it
    # named. Recorded because the gate's own log line does not carry it — the
    # only previous way to answer "what did the guard object to?" was to open
    # `changesets/<id>/result.json` by hand. `extra` is splatted straight into
    # this dataclass, so a key here that is not a field is a TypeError at the
    # region path, which is how these two arrived.
    validation_code:   Optional[str] = None
    validation_detail: Optional[str] = None
    w2_finding_code:   Optional[str] = None
    w2_finding_detail: Optional[str] = None

    def __post_init__(self) -> None:
        self.attempt_trace = deepcopy(self.attempt_trace)
        self.anchor_hit_ids = deepcopy(self.anchor_hit_ids)
        validate_attempt_trace(self.terminal_status, self.attempt_trace)


class StateManager:
    """git wrapper + audit JSONL writer, one instance per agent run.

    Contract:
      * `snapshot_before_attempt()` — record HEAD sha for rollback.
      * `commit_success(paths, msg)` — stage exactly `paths`, commit, return new sha.
      * `rollback()` — `git reset --hard HEAD` (drops working tree changes,
        keeps prior commits).
      * `log(record)` — append AttemptRecord as JSONL.

    Never uses `git add .`. Always adds specific files (defend against
    target/ / preflight artifacts sneaking in).
    """

    def __init__(self, crate: Path, audit_log_path: Path):
        self.crate = crate.resolve()
        self.audit_log_path = audit_log_path
        self.audit_log_path.parent.mkdir(parents=True, exist_ok=True)
        # Track token accumulator for project_budget_exceeded()
        self._tokens_used = 0

    # ─── git ─────────────────────────────────────────────────────────

    _GIT_TIMEOUT_S = 30       # defensive; real git ops <1s. Guards against
                              # lock file from prior interrupted run.

    def _git(self, *args: str, check: bool = True) -> str:
        """Run `git <args>` in the crate; return stdout stripped. Raise on
        nonzero exit if check=True. 30s timeout guards against a stale
        .git/index.lock from a prior interrupted run."""
        cmd = ["git", "-C", str(self.crate), *args]
        proc = subprocess.run(cmd, capture_output=True, text=True,
                               timeout=self._GIT_TIMEOUT_S)
        if check and proc.returncode != 0:
            raise RuntimeError(
                f"git failed: {' '.join(cmd)}\n"
                f"stderr: {proc.stderr.strip()}"
            )
        return proc.stdout.strip()

    def snapshot_before_attempt(self) -> str:
        """Return current HEAD sha (short). Called before applying a rewrite."""
        return self.head_sha(full=False)

    def head_sha(self, *, full: bool = True) -> str:
        """Return the current HEAD identity."""
        args = ("rev-parse", "HEAD") if full else ("rev-parse", "--short", "HEAD")
        return self._git(*args)

    def _relative_paths(self, paths: Iterable[Path]) -> list[str]:
        relative: list[str] = []
        for raw_path in paths:
            path = Path(raw_path)
            absolute = path.resolve() if path.is_absolute() else (self.crate / path).resolve()
            relative.append(str(absolute.relative_to(self.crate)))
        return sorted(set(relative))

    def unstage_paths(self, paths: Iterable[Path]) -> None:
        rel_paths = self._relative_paths(paths)
        if rel_paths:
            self._git("restore", "--staged", "--", *rel_paths)

    def diff_paths(self) -> tuple[str, ...]:
        changed = self._git("diff", "--name-only", "HEAD", "--")
        return tuple(line for line in changed.splitlines() if line)

    def commit_success(self, paths: Iterable[Path], msg: str) -> str:
        """Stage `paths` (files, not `.`), commit, return the new short sha.

        `paths` are absolute or repo-relative — we resolve + relativize.
        Empty diff after `git add` is a bug caller-side (nothing to
        commit) — raise RuntimeError so it surfaces.
        """
        rel_paths = self._relative_paths(paths)
        if not rel_paths:
            raise RuntimeError("commit_success: no paths to stage")
        already_staged = set(
            line for line in self._git("diff", "--cached", "--name-only").splitlines()
            if line
        )
        if already_staged:
            raise RuntimeError(
                f"commit_success: pre-existing staged paths: {sorted(already_staged)}")
        self._git("add", "--", *rel_paths)
        staged = set(
            line for line in self._git("diff", "--cached", "--name-only").splitlines()
            if line
        )
        expected = set(rel_paths)
        if staged != expected:
            self.unstage_paths(rel_paths)
            raise RuntimeError(
                f"commit_success: staged paths {sorted(staged)} != expected "
                f"{sorted(expected)}")
        self._git("commit", "-m", msg)
        sha = self._git("rev-parse", "--short", "HEAD")
        logger.info("[state] commit %s  %s", sha, msg)
        return sha

    def rollback(self) -> None:
        """`git reset --hard HEAD` — drops uncommitted changes; keeps history.

        Idempotent: calling on a clean working tree is a no-op.
        """
        self._git("reset", "--hard", "HEAD")
        logger.info("[state] rollback → HEAD clean")

    def working_tree_clean(self) -> bool:
        """True iff no uncommitted changes (staged or unstaged)."""
        return self._git("status", "--porcelain") == ""

    # ─── audit log ───────────────────────────────────────────────────

    def log(self, record: AttemptRecord) -> None:
        """Append one JSONL record to audit_log_path. Enum serialized as .value."""
        validate_attempt_trace(record.terminal_status, record.attempt_trace)
        d = asdict(record)
        d["status"] = record.status.value    # Enum → str for JSON
        with self.audit_log_path.open("a", encoding="utf-8") as f:
            f.write(json.dumps(d, ensure_ascii=False) + "\n")
        # Accumulate tokens for budget tracking
        self._tokens_used += record.tokens_in + record.tokens_out

    # ─── budget ──────────────────────────────────────────────────────

    def project_tokens_used(self) -> int:
        return self._tokens_used

    def project_budget_exceeded(self, cfg) -> bool:
        ""                                                       
                                                              
                                                      
           
        return False
