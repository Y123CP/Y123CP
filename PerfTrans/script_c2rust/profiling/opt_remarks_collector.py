"""Collect LLVM optimization remarks during a release build.

perf_tree_design.md §2.5.6 step 7: the evidence pool's `opt_remarks` come
from the compiler self-reporting which loops vectorized / failed to.

Toolchain reality (verified on the staged projects' rustc 1.70 nightly):
`-Cremark=...` emits *nothing* — the remark flag is effectively a no-op
on that rustc. The LLVM-native flag DOES work:

    -Cllvm-args=-pass-remarks-missed=loop-vectorize   → "loop not vectorized"
    -Cllvm-args=-pass-remarks=loop-vectorize          → "vectorized loop"

emitted to stderr as:

    remark: src/huffman.rs:159:19: loop not vectorized

So we drive LLVM directly via `-Cllvm-args` and parse that line form.
The legacy `file:line:col: remark: msg` form is still accepted as a
fallback for other toolchains.
"""

from __future__ import annotations

import logging
import os
import re
import subprocess
from dataclasses import asdict, dataclass, field
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class OptRemarksReport:
    ok:                 bool
    remarks:            list[dict] = field(default_factory=list)
    # {"file","line","col","pass","status","message"} — status ∈ {missed,passed}
    vectorize_hits:     int = 0
    vectorize_failures: list[dict] = field(default_factory=list)  # legacy: {file,line,reason}
    inline_hits:        int = 0
    inline_failures:    int = 0
    raw_output:         Path | None = None
    error:              str = ""

    def to_dict(self) -> dict:
        d = asdict(self)
        d["raw_output"] = str(self.raw_output) if self.raw_output else None
        return d


# LLVM remark flags driven through rustc. loop-vectorize is the signal that
# matters for the §5.1/§5.2 perf story (c2rust raw-ptr loops fail to
# vectorize); each is a separate `-Cllvm-args=`.
_REMARK_LLVM_ARGS = (
    "-Cllvm-args=-pass-remarks-missed=loop-vectorize",
    "-Cllvm-args=-pass-remarks=loop-vectorize",
)

# rustc 1.70 form:  `remark: src/foo.rs:159:19: loop not vectorized`
_REMARK_NEW = re.compile(
    r"^remark:\s*(?P<file>[^:]+):(?P<line>\d+):(?P<col>\d+):\s*(?P<msg>.+?)\s*$"
)
# legacy form:      `src/foo.rs:159:19: remark: loop not vectorized`
_REMARK_OLD = re.compile(
    r"^(?P<file>[^:]+):(?P<line>\d+):(?P<col>\d+):\s*remark:\s*(?P<msg>.+?)\s*$"
)


def _classify(msg: str) -> tuple[str, str]:
    """Map a remark message → (pass, status). status ∈ {missed, passed}."""
    m = msg.lower()
    if "loop not vectorized" in m:
        return "loop-vectorize", "missed"
    if "vectorized loop" in m or m.startswith("vectorized"):
        return "loop-vectorize", "passed"
    if "vectoriz" in m:
        # e.g. "the cost-model indicates that vectorization is not beneficial"
        return "loop-vectorize", "missed"
    if "not inlined" in m or "will not be inlined" in m:
        return "inline", "missed"
    if "inlined into" in m:
        return "inline", "passed"
    return "other", "missed"


def parse_opt_remarks(stderr_text: str) -> list[dict]:
    """Parse a build's stderr → list of structured remark dicts."""
    out: list[dict] = []
    for line in stderr_text.splitlines():
        line = line.strip()
        m = _REMARK_NEW.match(line) or _REMARK_OLD.match(line)
        if not m:
            continue
        passname, status = _classify(m.group("msg"))
        out.append({
            "file":    m.group("file"),
            "line":    int(m.group("line")),
            "col":     int(m.group("col")),
            "pass":    passname,
            "status":  status,
            "message": m.group("msg"),
        })
    return out


def collect_opt_remarks(project_path: Path,
                        cargo_args: list[str] | None = None,
                        timeout: int = 600,
                        extra_rustflags: str = "") -> OptRemarksReport:
    """Run `cargo build --release` with LLVM remark flags; parse the output.

    Args:
      project_path    : absolute path to a cargo project
      cargo_args      : extra args to pass to cargo (e.g. ["--bin", "name"])
      timeout         : seconds before giving up on the build
      extra_rustflags : extra flags appended to RUSTFLAGS — the §2.5
                        pipeline passes "-Cdebuginfo=1" so this single
                        build also produces the line-numbered binary used
                        for perf annotate / objdump (design §2.5.5).

    Returns OptRemarksReport with `remarks` (structured) populated. A build
    is allowed to fail (cached output may still be usable); remarks parsed
    from whatever stderr was produced are returned regardless.
    """
    project_path = Path(project_path).resolve()
    cargo_args = list(cargo_args or [])

    # Pass remark + extra flags via RUSTFLAGS so they reach every crate.
    env = os.environ.copy()
    env["RUSTFLAGS"] = " ".join(filter(None, (
        env.get("RUSTFLAGS", ""),
        *_REMARK_LLVM_ARGS,
        extra_rustflags,
    ))).strip()
    cmd = ["cargo", "build", "--release", *cargo_args]

    logger.info(f"[opt-remarks] cargo build with RUSTFLAGS={env['RUSTFLAGS']!r}")

    try:
        proc = subprocess.run(cmd, cwd=str(project_path), env=env,
                              capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return OptRemarksReport(ok=False, error=f"cargo build timed out after {timeout}s")
    except OSError as e:
        return OptRemarksReport(ok=False, error=f"cargo build failed to launch: {e}")

    remarks = parse_opt_remarks(proc.stderr or "")
    vect_fail = [{"file": r["file"], "line": r["line"], "reason": r["message"]}
                 for r in remarks if r["pass"] == "loop-vectorize"
                 and r["status"] == "missed"]
    vect_hit = sum(1 for r in remarks
                   if r["pass"] == "loop-vectorize" and r["status"] == "passed")

    return OptRemarksReport(
        ok                 = (proc.returncode == 0),
        remarks            = remarks,
        vectorize_hits     = vect_hit,
        vectorize_failures = vect_fail,
        error              = "" if proc.returncode == 0 else
                             f"cargo build exit={proc.returncode}",
    )
