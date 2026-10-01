"""Stage base class — every pass plugs into this contract.

Contract:
    input_path  → output_path (full Cargo project, isolated copy)

Each stage does:
    1. Copy input_path → output_path (idempotent: rmtree + cp -r)
    2. Initialize git inside output_path → "stage_start" commit (rollback anchor)
    3. Subclass `_apply(output_path)` — performs the rewrites
    4. cargo build --release  (hard gate; stage fails if build breaks)
    5. trace_diff vs input_path  (if requires_oracle and oracle config given)
    6. microbench vs input_path  (if requires_microbench; deferred until B/C/E)
    7. On any gate failure: git reset --hard stage_start (state rolled back)
       Returns StageResult with verdict + diagnostics.

Subclasses set:
    name                 = "stage1_cleanup"
    requires_oracle      = True | False
    requires_microbench  = True | False
"""

from __future__ import annotations

import logging
import os
import re
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from utils.git_manage import GitManager

logger = logging.getLogger(__name__)


@dataclass
class StageResult:
    name: str
    ok: bool
    output_path: Path
    skipped: bool = False
    error: str = ""
    notes: list[str] = field(default_factory=list)

    def __str__(self) -> str:
        tag = "SKIP" if self.skipped else ("PASS" if self.ok else "FAIL")
        return f"[{tag}] {self.name} → {self.output_path}{(' :: ' + self.error) if self.error else ''}"


@dataclass
class OracleConfig:
    """Trace-diff oracle settings; None to disable."""
    c_project_path: Path           # absolute path to C source project (with compile_commands.json)
    target_funcs: list[str]        # functions to instrument and compare


class StageBase:
    name: str = "stage_base"
    requires_oracle: bool = False
    requires_microbench: bool = False
    # When False, a failing `cargo build` at gate time logs a warning but
    # PRESERVES the output_path (no rollback to stage_start). This is for
    # stages whose output is genuinely diagnostic / under iterative
    # development — partial successful work is more useful than a wiped
    # directory. Default stays True (strict) for safety.
    strict_cargo_gate: bool = True

    def __init__(self, args, oracle: Optional[OracleConfig] = None):
        self.args = args
        self.oracle = oracle
        self.git = GitManager()

    # --------------------------------------------------------------------
    # Public entry
    # --------------------------------------------------------------------

    def run(self, input_path: Path, output_path: Path) -> StageResult:
        input_path = Path(input_path).resolve()
        output_path = Path(output_path).resolve()
        logger.info(f"[{self.name}] {input_path} → {output_path}")

        if not input_path.exists():
            return StageResult(self.name, ok=False, output_path=output_path,
                               error=f"input does not exist: {input_path}")

        # Safety guard: when --keep-existing is set, skip the stage entirely
        # if `output_path` already holds a Cargo project that builds. Avoids
        # blowing away prior work after an interrupted long pipeline run
        # (e.g. token exhaustion midway through Stage 2). Stage 2 has its
        # own resume mechanism inside its `run()` override; this guard
        # protects Stage 0 / Stage 1 / future stages.
        if getattr(self.args, "keep_existing", False) \
                and output_path.exists() \
                and (output_path / "Cargo.toml").exists():
            build_err = self._cargo_build(output_path)
            if not build_err:
                logger.info(f"[{self.name}] --keep-existing: output already builds; "
                            f"skipping stage")
                return StageResult(self.name, ok=True, skipped=True,
                                   output_path=output_path,
                                   notes=["kept existing build"])
            logger.warning(f"[{self.name}] --keep-existing: output exists but does NOT "
                           f"build; falling through to regenerate. Build tail:\n{build_err}")

        # 1. Materialize output_path as a fresh copy of input_path.
        self._sync_dir(input_path, output_path)

        # 1b. Re-namespace the Cargo package to `<project>_<stage_suffix>` so
        # each stage's crate name encodes which pass produced it.
        self._patch_cargo_pkg_name(output_path / "Cargo.toml",
                                   self.derive_pkg_name(output_path))

        # 2. Git anchor inside output_path so we can roll back on gate failure.
        self.git.ensure_git_repo(str(output_path))
        anchor = self.git.git_save_state(str(output_path), f"{self.name}: stage_start")

        # 3. Subclass-specific rewrites.
        try:
            self._apply(output_path)
        except Exception as e:
            logger.exception(f"[{self.name}] _apply raised")
            self._rollback(output_path, anchor)
            return StageResult(self.name, ok=False, output_path=output_path,
                               error=f"_apply raised: {e}")

        # 4. cargo build  — gate (strict by default; subclasses can opt
        # into a soft gate that preserves the output for inspection).
        build_err = self._cargo_build(output_path)
        if build_err:
            if self.strict_cargo_gate:
                self._rollback(output_path, anchor)
                return StageResult(self.name, ok=False, output_path=output_path,
                                   error=f"cargo build failed: {build_err}")
            # Soft gate: keep the partial work in `output_path`, mark the
            # stage as failed in the result, but do NOT roll back. The
            # next stage won't run, but humans can inspect what landed.
            logger.warning(f"[{self.name}] cargo build failed (SOFT gate; "
                           f"output preserved). Tail:\n{build_err}")
            self.git.git_save_state(str(output_path),
                                    f"{self.name}: stage_end (cargo failing)")
            return StageResult(self.name, ok=False, output_path=output_path,
                               error=f"cargo build failed (soft gate): "
                                     f"{build_err}",
                               notes=["soft gate: output preserved for inspection"])

        # 5. trace_diff oracle (optional per stage).
        if self.requires_oracle and self.oracle is not None and not getattr(self.args, "skip_oracle", False):
            ok, msg = self._run_trace_diff(input_path, output_path)
            if not ok:
                self._rollback(output_path, anchor)
                return StageResult(self.name, ok=False, output_path=output_path,
                                   error=f"trace_diff failed: {msg}")

        # 6. microbench (deferred — Pass B/C/E will enable this).
        # TODO: implement once Pass B kicks in; gated on requires_microbench.

        self.git.git_save_state(str(output_path), f"{self.name}: stage_end")
        return StageResult(self.name, ok=True, output_path=output_path)

    # --------------------------------------------------------------------
    # Subclass hook
    # --------------------------------------------------------------------

    def _apply(self, project_path: Path) -> None:
        raise NotImplementedError(f"{type(self).__name__}._apply not implemented")

    # --------------------------------------------------------------------
    # Helpers
    # --------------------------------------------------------------------

    @staticmethod
    def _sync_dir(src: Path, dst: Path) -> None:
        if dst.exists():
            shutil.rmtree(dst)
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(src, dst, symlinks=False, ignore=shutil.ignore_patterns(".git", "target"))

    @staticmethod
    def _cargo_build(project_path: Path) -> str:
        if not (project_path / "Cargo.toml").exists():
            return ""  # not a cargo project (e.g. before stage 0); skip build gate
        res = subprocess.run(
            ["cargo", "build", "--release"],
            cwd=str(project_path), capture_output=True, text=True, timeout=600,
        )
        if res.returncode != 0:
            tail = "\n".join((res.stderr or "").splitlines()[-30:])
            return tail
        return ""

    def _rollback(self, output_path: Path, anchor: Optional[str]) -> None:
        if anchor:
            self.git.git_restore_state(str(output_path), anchor)

    # --------------------------------------------------------------------
    # Cargo package-name helpers (also used by Stage 0 standalone)
    # --------------------------------------------------------------------

    @staticmethod
    def sanitize_pkg_name(name: str) -> str:
        """Coerce arbitrary string to a valid Cargo package name.

        Cargo: ASCII letters/digits/`_`/`-`, must start with a letter.
        `bzip2-1.0.8` → `bzip2_1_0_8`; `0_raw` → `pkg_0_raw`.
        """
        s = re.sub(r'[^a-z0-9_]', '_', name.lower())
        s = re.sub(r'_+', '_', s).strip('_')
        if not s or not s[0].isalpha():
            s = 'pkg_' + s
        return s

    @classmethod
    def derive_pkg_name(cls, stage_output_path: Path) -> str:
        """`<project_dir>/<N_stage_suffix>` → `<project>_<stage_suffix>`.

        Examples:
          `sortbench/0_raw/`        → `sortbench_raw`
          `bzip2_1_0_8/2_safe/`     → `bzip2_1_0_8_safe`
          `lz4_1_9_4/3_idiom_b/`    → `lz4_1_9_4_idiom_b`
        """
        project = cls.sanitize_pkg_name(stage_output_path.parent.name)
        suffix  = re.sub(r'^\d+_', '', stage_output_path.name)  # strip leading "N_"
        suffix  = cls.sanitize_pkg_name(suffix)
        return f"{project}_{suffix}"

    @staticmethod
    def _patch_cargo_pkg_name(cargo_toml: Path, pkg_name: str) -> None:
        """Rewrite `name = "..."` under [package] and [lib] to `pkg_name`.
        Also fixes any `use ::<old_pkg>` references in src/*.rs so binaries
        promoted by Stage 0 keep linking when the package is renamed by a
        later stage. No-op if Cargo.toml is missing.
        """
        if not cargo_toml.exists():
            return
        text = cargo_toml.read_text()
        out, section, old_pkg = [], None, None
        for line in text.splitlines():
            stripped = line.strip()
            if stripped.startswith("[") and stripped.endswith("]"):
                section = stripped[1:-1]
            if section == "package":
                if m := re.match(r'^\s*name\s*=\s*"([^"]*)"', line):
                    old_pkg = m.group(1)
            if section in ("package", "lib") and re.match(r'^\s*name\s*=\s*"', line):
                line = re.sub(r'"[^"]*"', f'"{pkg_name}"', line, count=1)
            out.append(line)
        cargo_toml.write_text("\n".join(out) + ("\n" if text.endswith("\n") else ""))

        if old_pkg and old_pkg != pkg_name:
            src_dir = cargo_toml.parent / "src"
            if src_dir.is_dir():
                pat = re.compile(rf'\buse\s+::\s*{re.escape(old_pkg)}\b')
                # rglob: also catches nested-main wrappers Stage 0 generates
                # under src/bin/<name>.rs (multi-component projects like
                # optipng). Top-level src/*.rs is already covered.
                for rs in src_dir.rglob("*.rs"):
                    txt = rs.read_text()
                    new = pat.sub(f"use ::{pkg_name}", txt)
                    if new != txt:
                        rs.write_text(new)

    def _run_trace_diff(self, input_path: Path, output_path: Path) -> tuple[bool, str]:
        """Hook for trace_diff oracle. Wired to oracle/run_differential_test.sh.

        For now this is a stub returning (True, '') so the pipeline doesn't
        block on oracle wiring during Stage 0/1 bring-up. Will be filled in
        once Stage 2 (Pass A) is being validated.
        """
        # TODO: invoke oracle/run_differential_test.sh with self.oracle.c_project_path
        # and self.oracle.target_funcs, then parse trace_diff.py exit code.
        return True, "trace_diff oracle not yet wired (TODO)"
