"""Stage A Phase C — Verifier.

Each gate runs cargo / the workload and reports `GateResult(ok, detail, …)`.

The LIVE Stage A pipeline (`runner.py:run_stage_a`) is **3-gate** — Stage A
is a sig-only pass, so its correctness gates are cargo check + build + W1.
Performance (W2) judgement belongs to Stage B / perf_compare, NOT here
(see `docs/perf_tree_design.md` §2.4.3 and `runner.py`'s module docstring).

  Gate 1  cargo check (release profile, no link) — fastest signal.
                                                   Catches: name resolution,
                                                   type errors, missing
                                                   `use` imports after
                                                   transform.
  Gate 2  cargo build --release                   — full LTO + link.
                                                   Catches: symbol clashes,
                                                   linker errors specific
                                                   to release optimization.
  Gate 3  W1 oracle (stdout sha256 + exit_code)   — functional equivalence
                                                   to c2rust_raw under the
                                                   pipeline workload.

There is no W2 perf gate here. Performance judgement happens at Stage B /
perf_compare. (A `w2()` method + the FATR driver that was its sole caller
were retired 2026-06-25; see `docs/stage_a_safety_lift_design.md` §8.)
"""

from __future__ import annotations

import hashlib
import logging
import subprocess
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger(__name__)


_STDERR_TAIL_CHARS = 600


def _stderr_tail(raw: bytes | None) -> str:
    """The last of a failed run's stderr, as a suffix for a W1 detail.

    A W1 detail used to be the exit code alone, which names the symptom and
    nothing else. It is the only thing a repair turn gets to work from, and
    for the two commonest failures it is not enough on its own: an abort is
    glibc reporting a specific heap violation ("free(): invalid pointer"),
    and exit 101 is a Rust panic that names the file, the line, and the
    index that was out of bounds. Both write it here and it was discarded.

    Tail, not head: a panic's message is the last thing printed, after
    whatever the workload logged on its way there.
    """
    if not raw:
        return ""
    text = raw.decode("utf-8", errors="replace").strip()
    if not text:
        return ""
    if len(text) > _STDERR_TAIL_CHARS:
        text = "…" + text[-_STDERR_TAIL_CHARS:]
    return f"\nstderr: {text}"


@dataclass
class GateResult:
    gate: str       # "check" / "build" / "w1"
    ok: bool
    detail: str = ""


class Verifier:
    def __init__(self, project_dir: Path,
                 binary_dir: Path | None = None) -> None:
        """`project_dir` is the lib crate Stage A operates on (where
        `cargo check / build` runs to validate the lift). `binary_dir`
        is the driving crate whose `target/release/<binary>` we measure
        — defaults to `project_dir` (for binary projects), but is the
        harness crate for lib-only projects."""
        self.project_dir = project_dir.resolve()
        self.binary_dir = (binary_dir or project_dir).resolve()

    # ── Gate 1 ───────────────────────────────────────────────────────
    def cargo_check(self, *, timeout_s: int = 180) -> GateResult:
        """Runs `cargo check` from the DRIVING crate (binary_dir). For
        lib-only projects this is the harness; for binary projects it's
        the lib+bin crate itself. Critically, we do NOT run check from
        the lib crate when a separate harness exists, because c2rust
        translations often ship alternate [[bin]] entry points
        (e.g. libcsv's `test_csv.rs` or bzip2's `bzip2recover.rs`) that
        re-declare types and don't depend on lib's `pub use`. Those
        binaries are NOT the workload we measure; checking them would
        produce false-positive frozen culprits."""
        try:
            proc = subprocess.run(
                ["cargo", "check", "--release"], cwd=str(self.binary_dir),
                capture_output=True, text=True, timeout=timeout_s,
            )
        except subprocess.TimeoutExpired:
            # Contention-slow check (parallel workers / shared server) must
            # gate as FAIL, never propagate and kill the whole Stage A run
            # (brotli 2026-07-29: one 180s timeout aborted Phase 1).
            return GateResult(gate="check", ok=False,
                              detail=f"cargo check timeout ({timeout_s}s)")
        ok = proc.returncode == 0
        detail = ""
        if not ok:
            detail = "\n".join(
                ln for ln in (proc.stderr or "").splitlines()
                if "error" in ln.lower()
            )[:1500]
        return GateResult(gate="check", ok=ok, detail=detail)

    # ── Gate 2 ───────────────────────────────────────────────────────
    def cargo_build(self, *, timeout_s: int = 600) -> GateResult:
        """Builds the DRIVING crate (binary_dir) for the W1 CORRECTNESS gate.

        Disables LTO (`lto=fat` + `codegen-units=1` makes every build re-run
        whole-program LTO with no incremental benefit — ~9s on heman; off →
        ~4s and incremental). Stage A's gate only checks STDOUT EQUIVALENCE,
        not performance, so LTO is unnecessary here; the output crate's
        Cargo.toml keeps the fair `lto=fat` profile for Stage B perf, which
        rebuilds from it. SOUND: aliasing is proven by cargo check (the borrow
        checker), not the optimizer; Rust is IEEE-strict (no fast-math), so
        opt-level=3 without LTO yields byte-identical stdout → W1 still holds.
        opt-level kept at 3 (only LTO/codegen-units changed) to stay closest
        to the fair build."""
        import os
        env = {**os.environ,
               "CARGO_PROFILE_RELEASE_LTO": "false",
               "CARGO_PROFILE_RELEASE_CODEGEN_UNITS": "16"}
        proc = subprocess.run(
            ["cargo", "build", "--release"], cwd=str(self.binary_dir),
            capture_output=True, text=True, timeout=timeout_s, env=env,
        )
        ok = proc.returncode == 0
        detail = ""
        if not ok:
            detail = "\n".join(
                ln for ln in (proc.stderr or "").splitlines()
                if "error" in ln.lower() or "undefined" in ln.lower()
            )[:1500]
        return GateResult(gate="build", ok=ok, detail=detail)

    # ── Gate 3 ───────────────────────────────────────────────────────
    def w1(self, *, binary: str, args: list[str], input_path: Path,
           expected_sha256: str, expected_exit: int = 0,
           timeout_s: int = 180,
           wrapper: Path | None = None) -> GateResult:
        """W1 oracle: run binary, sha256 stdout, compare to expected.

        `wrapper` (optional): absolute path to a shell script invoked as
        `wrapper <binary> <args>`. The wrapper's stdout (not the
        binary's) becomes the sha256 input. Used by optipng / tmux /
        similar projects whose binary writes output to a file or socket
        rather than stdout — the wrapper cats that artifact to stdout
        so the oracle sees byte-equivalent comparison material. See
        e.g. dataset_trans_process/optipng-0.7.7/workloads/wrapper_pipe.sh.

        Substitutes per-invocation tokens in args before launch:
          · `$INPUT` → str(input_path)
          · `$RAND`  → random hex (lets tmux give each invocation a
                      unique socket name; see tmux pipeline.toml header).
        """
        bin_path = self.binary_dir / "target" / "release" / binary
        if not bin_path.is_file():
            return GateResult(gate="w1", ok=False, detail=f"missing {bin_path}")
        import secrets
        rand_tok = secrets.token_hex(4)
        substituted = [
            (str(input_path) if a == "$INPUT" else a).replace("$RAND", rand_tok)
            for a in args
        ]
        if wrapper is not None:
            cmd = [str(wrapper), str(bin_path), *substituted]
        else:
            cmd = [str(bin_path), *substituted]
        try:
            proc = subprocess.run(cmd, capture_output=True, timeout=timeout_s)
        except subprocess.TimeoutExpired:
            return GateResult(gate="w1", ok=False, detail="W1 timeout")
        if proc.returncode != expected_exit:
            return GateResult(gate="w1", ok=False,
                              detail=f"exit {proc.returncode} ≠ {expected_exit}"
                                     + _stderr_tail(proc.stderr))
        digest = hashlib.sha256(proc.stdout).hexdigest()
        if digest != expected_sha256:
            return GateResult(gate="w1", ok=False,
                              detail=f"sha256 {digest[:12]}… ≠ {expected_sha256[:12]}…")
        return GateResult(gate="w1", ok=True)
