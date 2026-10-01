"""Wrapper for `perf record` + `perf report` to extract hot functions.

DESIGN.md v2 §7.3 (A): Phase 0 profiling tool stack.

This module shells out to system `perf` (linux-tools). If `perf` is not
available or blocked by `kernel.perf_event_paranoid`, falls back to
returning an empty hot-fn list with a recorded reason — caller can then
optionally try `samply` (TODO) or skip profiling.

Output format: list[dict] with at least `{"name": str, "cpu_pct": float}`.
"""

from __future__ import annotations

import logging
import re
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class PerfResult:
    """One profiling run's result."""
    ok:           bool
    hot_fns:      list[dict] = field(default_factory=list)  # {"name": str, "cpu_pct": float}
    raw_data:     Path | None = None                          # path to perf.data, if recorded
    error:        str  = ""

    def __bool__(self) -> bool:
        return self.ok


def _perf_available() -> bool:
    """Return True if `perf` is on PATH and runnable."""
    if shutil.which("perf") is None:
        return False
    try:
        subprocess.run(["perf", "--version"], check=True, capture_output=True, timeout=5)
        return True
    except (subprocess.SubprocessError, OSError):
        return False


def run_perf(binary: Path, args: list[str], cwd: Path | None = None,
             output_dir: Path | None = None,
             frequency: int = 999,
             call_graph_mode: str = "fp",
             timeout: int = 300,
             event: str | None = None) -> PerfResult:
    """Run `perf record -F <freq> --call-graph <mode> <binary> <args...>` and `perf report`.

    Returns PerfResult with hot_fns sorted by cpu_pct descending.

    `event` selects the sampled PMU event (`perf record -e`). The default
    (None) samples `cycles` → self-time CPU%. Pass `"instructions:u"` for
    the §2.5.4 IC-axis run. Each event writes a distinctly-named perf.data
    so a cycles run and an instructions run do not clobber each other.

    `call_graph_mode` selects the unwinder:
      - "fp"     : frame pointer (default) — fastest but **fails on stripped Rust release**
                   builds; only the leaf frame is captured
      - "dwarf"  : DWARF-based unwinding from .eh_frame — works on stripped builds,
                   slower (~10x larger perf.data), recommend frequency≤99
      - "lbr"    : Intel LBR hardware — fastest, but limited stack depth (16-32),
                   Intel-only

    On any failure (perf not available, paranoid level too high, binary
    crash, parse error), returns ok=False with `error` populated.
    """
    from profiling.workload import _resolve_rand
    args = _resolve_rand(args)

    binary = Path(binary).resolve()
    if not binary.is_file():
        return PerfResult(ok=False, error=f"binary not found: {binary}")

    if not _perf_available():
        return PerfResult(ok=False, error="perf binary not on PATH or not runnable")

    if call_graph_mode not in ("fp", "dwarf", "lbr"):
        return PerfResult(ok=False, error=f"invalid call_graph_mode={call_graph_mode!r}")

    output_dir = Path(output_dir).resolve() if output_dir else Path.cwd() / ".perf_runs"
    output_dir.mkdir(parents=True, exist_ok=True)
    event_tag = (event or "cycles").replace(":", "_").replace("/", "_")
    perf_data = output_dir / f"{binary.name}.{event_tag}.perf.data"

    # `-g` alone uses the default unwinder (fp). For dwarf/lbr we explicitly
    # spell out --call-graph; perf accepts both forms without conflict.
    if call_graph_mode == "fp":
        cg_args = ["-g"]
    else:
        cg_args = ["--call-graph", call_graph_mode]

    event_args = ["-e", event] if event else []
    record_cmd = [
        "perf", "record",
        "-F", str(frequency),
        *cg_args,
        *event_args,
        "-o", str(perf_data),
        "--",
        str(binary), *args,
    ]
    logger.info(f"[perf] record (mode={call_graph_mode}, freq={frequency}): "
                f"{' '.join(record_cmd[:8])} ...")

    # NOTE: workloads can write binary to stdout (e.g. `bzip2 -c`), so we
    # DEVNULL stdout and only capture stderr.
    try:
        rec = subprocess.run(record_cmd, cwd=str(cwd) if cwd else None,
                             stdout=subprocess.DEVNULL,
                             stderr=subprocess.PIPE,
                             text=True, errors="replace",
                             timeout=timeout)
    except subprocess.TimeoutExpired:
        return PerfResult(ok=False, error=f"perf record timed out after {timeout}s")
    except OSError as e:
        return PerfResult(ok=False, error=f"perf record failed to launch: {e}")

    if rec.returncode != 0:
        # Common failure: kernel.perf_event_paranoid > 2 forbids unprivileged perf.
        return PerfResult(ok=False,
                          error=f"perf record exit={rec.returncode}; stderr: "
                                f"{(rec.stderr or '').strip()[:300]}")

    # perf report --stdio gives a textual top-fn list we can parse.
    report_cmd = ["perf", "report", "-i", str(perf_data), "--stdio",
                  "--no-children", "--sort=symbol", "--percent-limit", "0.1"]
    try:
        rep = subprocess.run(report_cmd, capture_output=True, text=True, timeout=60)
    except (subprocess.SubprocessError, OSError) as e:
        return PerfResult(ok=False, raw_data=perf_data,
                          error=f"perf report failed: {e}")

    if rep.returncode != 0:
        return PerfResult(ok=False, raw_data=perf_data,
                          error=f"perf report exit={rep.returncode}")

    hot_fns = parse_perf_report(rep.stdout)
    return PerfResult(ok=True, hot_fns=hot_fns, raw_data=perf_data)


# `perf report --stdio --no-children --sort=symbol` produces, depending
# on perf version, one of:
#   12.34%  binary_name  [.] symbol_name                       (older perf)
#   12.34%  [.] symbol_name                                    (newer perf, no DSO column)
#   12.34%  [.] symbol_name        IPC  [IPC Coverage]         (newer perf w/ IPC append)
# We accept either by making the DSO column optional and the trailing
# IPC columns part of the symbol's right-trim.
_PERF_LINE = re.compile(
    r"^\s*(?P<pct>\d+\.\d+)%\s+"
    r"(?:\S+\s+)?"                 # optional DSO column
    r"\[\.?\]\s+"
    r"(?P<sym>\S(?:.*?\S)?)"       # symbol — first non-space to last non-space
    r"(?:\s{2,}\S.*)?\s*$"         # optional trailing IPC columns (≥2 spaces gap)
)


def parse_perf_report(stdout: str) -> list[dict]:
    """Parse `perf report --stdio` output into list of {"name", "cpu_pct"}."""
    hits: list[dict] = []
    for line in stdout.splitlines():
        if not line or line.startswith("#"):
            continue
        m = _PERF_LINE.match(line)
        if not m:
            continue
        hits.append({
            "name":    m.group("sym"),
            "cpu_pct": float(m.group("pct")),
        })
    # Sort by cpu_pct desc, in case perf didn't already.
    hits.sort(key=lambda h: -h["cpu_pct"])
    return hits
