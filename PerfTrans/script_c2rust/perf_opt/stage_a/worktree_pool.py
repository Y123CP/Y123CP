"""Worker-worktree pool for parallel Stage A bisect.

Each worker is an independent rsync copy of the main crate (+ harness if
the project has one), with its own git repo. Workers are acquired under
a lock + condition variable; concurrent callers wait when all N workers
are busy. After acquire, the worker is git-reset to its baseline
snapshot, ready for a fresh trial.

The point of N worktrees is **isolation**: each cargo check writes its
own `target/` and mutates its own .rs files. Without isolation we'd
have to serialize.

Disk cost: ~N × crate_size. For libxml2 (~250 MB cleaned), N=4 → ~1 GB
under /tmp. Cleaned up when `cleanup()` is called (typically at the end
of e1 parallel phase).
"""

from __future__ import annotations

import logging
import shutil
import subprocess
import threading
from contextlib import contextmanager
from pathlib import Path

# Re-use bench_pipeline's rsync / git_init / path-dep rewire — these are
# already battle-tested in the main pre-Stage-A pipeline. Underscore
# prefix is Python convention, not access control.
from perf_opt.bench_pipeline import _git_init, _rewire_path_dep, _rsync_copy

logger = logging.getLogger(__name__)


class WorktreePool:
    """N parallel rsync copies of the main project, each with its own
    git baseline snapshot."""

    def __init__(self, main_crate: Path, main_harness: Path | None,
                 n_workers: int, tmp_root: Path) -> None:
        self.main_crate = main_crate.resolve()
        self.main_harness = main_harness.resolve() if main_harness else None
        self.n_workers = n_workers
        self.tmp_root = tmp_root.resolve()
        # (crate_dir, harness_dir|None, baseline_sha)
        self.workers: list[tuple[Path, Path | None, str]] = []
        self._lock = threading.Lock()
        self._cond = threading.Condition(self._lock)
        self._available: list[int] = []
        self._setup()

    # ── setup / teardown ──────────────────────────────────────────────

    def _setup(self) -> None:
        """rsync N copies, git-init each, record baseline SHA."""
        if self.tmp_root.exists():
            shutil.rmtree(self.tmp_root)
        self.tmp_root.mkdir(parents=True)
        for i in range(self.n_workers):
            crate = self.tmp_root / f"worker_{i}_crate"
            _rsync_copy(self.main_crate, crate)
            _git_init(crate, "1_cleaned worker copy")
            sha = subprocess.run(
                ["git", "rev-parse", "HEAD"], cwd=str(crate),
                capture_output=True, text=True, check=True,
            ).stdout.strip()
            harness: Path | None = None
            if self.main_harness is not None:
                harness = self.tmp_root / f"worker_{i}_harness"
                _rsync_copy(self.main_harness, harness)
                # Each worker's harness Cargo.toml path-dep points at
                # ITS OWN crate, not main's. Without this all N harness
                # crates would share main_crate's target/ → races.
                _rewire_path_dep(harness / "Cargo.toml", crate)
            self.workers.append((crate, harness, sha))
            self._available.append(i)
        logger.info(
            f"[worktree-pool] {self.n_workers} worker(s) set up under {self.tmp_root}"
        )

    def cleanup(self) -> None:
        if self.tmp_root.exists():
            shutil.rmtree(self.tmp_root)

    # ── acquire / release ────────────────────────────────────────────

    @contextmanager
    def acquire(self):
        """Block until a worker is free; reset it to baseline; yield
        (crate, harness). Worker returns to the available pool on
        context exit (success OR exception). Tests should not assume
        any particular worker ordering."""
        idx = self._claim()
        try:
            crate, harness, sha = self.workers[idx]
            self._reset(crate, sha)
            yield crate, harness
        finally:
            self._release(idx)

    def _claim(self) -> int:
        with self._cond:
            while not self._available:
                self._cond.wait()
            return self._available.pop()

    def _release(self, idx: int) -> None:
        with self._cond:
            self._available.append(idx)
            self._cond.notify()

    def _reset(self, crate: Path, sha: str) -> None:
        subprocess.run(
            ["git", "reset", "--hard", sha], cwd=str(crate),
            capture_output=True, check=True,
        )
        subprocess.run(
            ["git", "clean", "-fd"], cwd=str(crate),
            capture_output=True, check=True,
        )

    # ── prewarm ─────────────────────────────────────────────────────

    def prewarm(self, verifier_factory) -> None:
        """Run `cargo check` on each worker BEFORE any trial so the
        worker's incremental cache is populated. The first cargo check
        on an empty target/ takes minutes (cold compile); subsequent
        per-trial checks are then seconds (incremental).

        `verifier_factory(crate, harness) → Verifier` builds a Verifier
        pointing at the right binary_dir for this project layout."""
        import concurrent.futures as _cf

        def warm(idx: int) -> None:
            crate, harness, _ = self.workers[idx]
            v = verifier_factory(crate, harness)
            res = v.cargo_check()
            if not res.ok:
                logger.warning(
                    f"[worktree-pool] worker {idx} prewarm: cargo check FAILED "
                    f"(baseline already broken before any lift) — {res.detail[:200]}"
                )

        with _cf.ThreadPoolExecutor(max_workers=self.n_workers) as ex:
            list(ex.map(warm, range(self.n_workers)))
        logger.info(f"[worktree-pool] {self.n_workers} worker(s) prewarmed")
