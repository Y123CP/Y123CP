"""Per-step rollback helper for cleanup pipelines.

Used by:
  * `stages/stage1_cleanup.py`        — Stage 1 multi-pass cleanup
  * `perf_opt/bench_pipeline.py`      — pre-Stage-A unify/normalize/strip

Each cleanup step is wrapped in an atomic snapshot/verify/restore loop:

    for each (name, fn) in steps:
        snapshot all .rs file bytes + Cargo.toml bytes
        try fn(crate_dir)
        run `cargo check --release`; count `error[Exxxx]` lines
        if post > pre:
            restore snapshot
            log "{name}: +{delta} errors → rolled back"
        else:
            keep edits, baseline := post
            log "{name}: kept ({pre} → {post})"

A step that raises an exception is also rolled back automatically.

This addresses two real failure modes seen in the audit:

  1. `Stage1Cleanup`'s strict-cargo-gate is all-or-nothing — a bad pass
     wipes ALL preceding good work (e.g. brotli: `apply_c2rust_compat_patches`
     fixes `__m128i_u` correctly, but a downstream int-conv issue trips
     the final gate and the whole `_apply` rolls back to stage_start →
     the m128i fix is lost.)

  2. `_prepare_working_copy`'s passes have no gate at all — a pass that
     introduces new errors (e.g. tmux's `_unify_duplicate_pub_types`
     creating `pub use crate::environ::environ` that clashes with an
     `extern { static mut environ }` → 2 E0255) carries the new errors
     into Stage A with no visibility into which pass introduced them.

Caveat: this gate only catches cargo-detectable regressions. Semantic
divergence that compiles is NOT caught — for that, the project's W1
oracle (when configured) is the only ground truth.
"""

from __future__ import annotations

import logging
import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable

logger = logging.getLogger(__name__)


# Match `error[E0xxx]` anywhere on the line — `--message-format=short`
# prefixes each error with `<file>:<line>:<col>:`, so a `^error[...]`
# anchor would miss every diagnostic. The bracketed-code form is
# specific enough to avoid false positives in help / suggestion text.
_ERROR_LINE_RE = re.compile(r"\berror\[E\d+\]")


@dataclass
class StepResult:
    name: str
    status: str             # "kept" | "rolled_back" | "raised"
    pre_errors: int
    post_errors: int
    delta: int
    note: str = ""


@dataclass
class StepRollbackReport:
    baseline_errors: int
    final_errors: int
    steps: list[StepResult] = field(default_factory=list)

    def kept(self) -> list[StepResult]:
        return [s for s in self.steps if s.status == "kept"]

    def rolled_back(self) -> list[StepResult]:
        return [s for s in self.steps if s.status != "kept"]


def _snapshot_files(dirs: list[Path]) -> dict[Path, bytes]:
    """Snapshot .rs + Cargo.toml/lock bytes across one or more dirs.
    Multi-dir support is required when a step modifies BOTH the lib
    crate AND a sibling harness/binary crate
    (e.g. `_normalize_harness_extern_blocks(crate_dst, harness_dst)`
    injects shims into crate's lib.rs while rewriting harness .rs
    files; rolling back only crate would leave harness inconsistent)."""
    snap: dict[Path, bytes] = {}
    for d in dirs:
        for f in d.rglob("*.rs"):
            if "target" in f.parts:
                continue
            try:
                snap[f] = f.read_bytes()
            except OSError:
                pass
        for cargo_name in ("Cargo.toml", "Cargo.lock"):
            cargo = d / cargo_name
            if cargo.is_file():
                snap[cargo] = cargo.read_bytes()
    return snap


def _restore_snapshot(dirs: list[Path], snap: dict[Path, bytes]) -> None:
    """Restore file content + delete any .rs that the step created."""
    existing = set(snap.keys())
    # Restore content of files that existed pre-step.
    for f, content in snap.items():
        try:
            f.write_bytes(content)
        except OSError as e:
            logger.warning(f"[step-rollback] restore failed for {f}: {e}")
    # Delete any .rs files NOT in the snapshot — the step created them.
    for d in dirs:
        for f in d.rglob("*.rs"):
            if "target" in f.parts:
                continue
            if f not in existing:
                try:
                    f.unlink()
                except OSError as e:
                    logger.warning(f"[step-rollback] could not remove {f}: {e}")


def _count_errors(cargo_dir: Path, *, timeout_s: int = 300) -> int:
    """Run `cargo check --release` from `cargo_dir` and return the count
    of `error[Exxxx]` lines. cargo check is faster than build for the
    same error coverage at the type / borrow / linker-decl level."""
    res = subprocess.run(
        ["cargo", "check", "--release", "--message-format=short"],
        cwd=str(cargo_dir),
        capture_output=True, text=True, timeout=timeout_s,
    )
    # cargo emits errors on stderr; short format is one error per line.
    return len(_ERROR_LINE_RE.findall(res.stderr or ""))


def run_with_step_rollback(
    crate_dir: Path,
    steps: list[tuple[str, Callable[[Path], None]]],
    *,
    extra_snapshot_dirs: list[Path] | None = None,
    cargo_check_dir: Path | None = None,
    log_prefix: str = "step-rollback",
    error_count_timeout_s: int = 300,
) -> StepRollbackReport:
    """Run each cleanup `step` in order with per-step rollback.

    Each step is `(name, fn)`; `fn(crate_dir)` is invoked. The function
    is free to do whatever transformation it wants — only the on-disk
    state matters for rollback. Exceptions raised by `fn` also trigger
    rollback (the step's partial writes are still reverted to the
    snapshot, so a half-applied transformation does not leak through).
    """
    crate_dir = crate_dir.resolve()
    snap_dirs = [crate_dir] + [d.resolve() for d in (extra_snapshot_dirs or [])]
    check_dir = (cargo_check_dir or crate_dir).resolve()

    baseline_errors = _count_errors(check_dir, timeout_s=error_count_timeout_s)
    pre = baseline_errors
    logger.info(
        f"[{log_prefix}] baseline cargo check errors: {pre} "
        f"(check_dir={check_dir.name}, snapshot {len(snap_dirs)} dir(s))"
    )

    report = StepRollbackReport(
        baseline_errors=baseline_errors,
        final_errors=baseline_errors,
        steps=[],
    )

    for name, fn in steps:
        snap = _snapshot_files(snap_dirs)
        raised: str | None = None
        try:
            fn(crate_dir)
        except Exception as e:
            raised = f"{type(e).__name__}: {e!s}"[:300]
            logger.exception(f"[{log_prefix}] {name}: exception during apply")

        if raised is not None:
            _restore_snapshot(snap_dirs, snap)
            report.steps.append(StepResult(
                name=name, status="raised",
                pre_errors=pre, post_errors=pre, delta=0,
                note=raised,
            ))
            logger.warning(f"[{log_prefix}] {name}: raised → rolled back")
            continue

        try:
            post = _count_errors(check_dir, timeout_s=error_count_timeout_s)
        except subprocess.TimeoutExpired:
            _restore_snapshot(snap_dirs, snap)
            report.steps.append(StepResult(
                name=name, status="rolled_back",
                pre_errors=pre, post_errors=pre, delta=0,
                note="cargo check timed out",
            ))
            logger.warning(f"[{log_prefix}] {name}: cargo check timeout → rolled back")
            continue

        delta = post - pre
        if delta > 0:
            _restore_snapshot(snap_dirs, snap)
            report.steps.append(StepResult(
                name=name, status="rolled_back",
                pre_errors=pre, post_errors=post, delta=delta,
                note=f"introduced {delta} new errors",
            ))
            logger.warning(
                f"[{log_prefix}] {name}: +{delta} errors "
                f"({pre} → {post}) → rolled back"
            )
        else:
            report.steps.append(StepResult(
                name=name, status="kept",
                pre_errors=pre, post_errors=post, delta=delta,
                note="",
            ))
            logger.info(
                f"[{log_prefix}] {name}: kept "
                f"({pre} → {post}, Δ={delta:+d})"
            )
            pre = post

    report.final_errors = pre
    logger.info(
        f"[{log_prefix}] DONE — baseline={baseline_errors}, "
        f"final={pre}, kept={len(report.kept())}, "
        f"rolled_back={len(report.rolled_back())}"
    )
    return report
