"""`perf stat` wrapper — collects hardware counters per workload run.

Used by `hot_probe.characterize` to enrich per-fn EvidencePack with
microarchitectural signals. Beyond wall-clock, we need to know *why* a
function is slow. `perf stat` gives us:

  - cycles, instructions    → IPC (low IPC ≈ memory-bound)
  - branches, branch-misses → branch predictor pressure
  - cache-references, cache-misses → memory hierarchy

Sibling `perf_runner.py` already wraps `perf record` for hot-fn lists.
This module is the *aggregate counter* counterpart. Both feed
`pipeline.profile_project`.

Failure modes:
  - perf not installed         → return ok=False, error="missing"
  - kernel.perf_event_paranoid → return ok=False, error="paranoid"
    (kernel rejects unprivileged perf access; user must run
    `sudo sysctl kernel.perf_event_paranoid=1` or run as root)
"""

from __future__ import annotations

import logging
import re
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

logger = logging.getLogger(__name__)


# Default counter set — these are universally available across Intel/AMD
# and don't require special permissions beyond paranoid≤1. Keep in sync
# with EvidencePack.perf_counters dict keys.
DEFAULT_EVENTS: tuple[str, ...] = (
    "task-clock",
    "cycles",
    "instructions",
    "branches",
    "branch-misses",
    "cache-references",
    "cache-misses",
)


@dataclass
class PerfStatResult:
    ok:        bool
    counters:  dict[str, int | float] = field(default_factory=dict)
    error:     str = ""

    def __bool__(self) -> bool:
        return self.ok


def _perf_available() -> bool:
    if shutil.which("perf") is None:
        return False
    try:
        subprocess.run(["perf", "--version"], capture_output=True, timeout=5, check=True)
        return True
    except (subprocess.SubprocessError, OSError):
        return False


def run_perf_stat(binary: Path, args: list[str], cwd: Path | None = None,
                  events: tuple[str, ...] = DEFAULT_EVENTS,
                  timeout_s: int = 300) -> PerfStatResult:
    """Run `perf stat -e events -- binary args`; parse the counter table.

    perf stat writes its output to stderr (not stdout — stdout is the
    inferior's stdout). The format is one counter per line, in the
    "Performance counter stats" block.
    """
    from perf_opt.hot_probe.profiling._util import _resolve_rand
    args = _resolve_rand(args)

    binary = Path(binary).resolve()
    if not binary.is_file():
        return PerfStatResult(ok=False, error=f"binary not found: {binary}")
    if not _perf_available():
        return PerfStatResult(ok=False, error="missing")

    cmd = [
        "perf", "stat",
        "-x", ",",                    # CSV output — easier to parse than the human format
        "-e", ",".join(events),
        "--",
        str(binary), *args,
    ]
    logger.info(f"[perf-stat] {' '.join(cmd[:6])} ... (binary args elided)")

    # NOTE: stdout is binary-DEVNULLed because workloads can write binary
    # data (e.g. `bzip2 -c` emits compressed bytes to stdout). Only stderr
    # carries the perf stat CSV table, so we keep it text-decoded.
    try:
        proc = subprocess.run(
            cmd,
            cwd=str(cwd) if cwd else None,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            text=True, errors="replace",
            timeout=timeout_s,
        )
    except subprocess.TimeoutExpired:
        return PerfStatResult(ok=False, error=f"timeout after {timeout_s}s")
    except OSError as e:
        return PerfStatResult(ok=False, error=f"launch failed: {e}")

    if proc.returncode != 0:
        msg = (proc.stderr or "").strip().splitlines()[:3]
        if any("paranoid" in line.lower() or "permission" in line.lower()
               for line in msg):
            return PerfStatResult(ok=False, error="paranoid")
        return PerfStatResult(ok=False,
                              error=f"inferior exit={proc.returncode}: "
                                    f"{' / '.join(msg)[:300]}")

    counters = parse_perf_stat_csv(proc.stderr)
    if not counters:
        return PerfStatResult(ok=False,
                              error=f"empty counter table; stderr head: "
                                    f"{(proc.stderr or '')[:200]!r}")
    return PerfStatResult(ok=True, counters=counters)


# perf stat CSV format (from `perf stat -x ,`):
#   <count>,<unit>,<event>,<run-time-ns>,<run-pct>,<extra-metric>,<metric-unit>
# Example:
#   2034567890,,cycles,1234567890,100.00,,
#   <not counted>,,branches,...      (when a counter is unsupported)
_CSV_FIELDS_MIN = 3   # at minimum we need count + unit + event-name


def parse_perf_stat_csv(stderr_text: str) -> dict[str, int | float]:
    """Parse `perf stat -x ,` stderr → {event_name: count}.

    `<not counted>` and `<not supported>` lines are skipped. Numeric
    counts are returned as int when possible (counter values), float
    when not (task-clock is reported in ms with a decimal).
    """
    out: dict[str, int | float] = {}
    for line in stderr_text.splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        # CSV split — perf does not embed commas inside fields, so a plain
        # split is safe. We only care about the first three fields.
        parts = line.split(",")
        if len(parts) < _CSV_FIELDS_MIN:
            continue
        count_raw, _unit, event = parts[0], parts[1], parts[2]
        if count_raw.startswith("<") or not event:
            continue
        # Strip user-space `:u` / kernel `:k` modifier — these are implied
        # by paranoid level, not part of the event name.
        if event.endswith((":u", ":k", ":h")):
            event = event[:-2]
        try:
            if "." in count_raw:
                out[event] = float(count_raw)
            else:
                out[event] = int(count_raw)
        except ValueError:
            continue
    return out


def derive_metrics(counters: dict[str, int | float]) -> dict[str, float]:
    """Compute IPC, branch-miss-rate, cache-miss-rate from raw counters.

    Returned values are floats; missing inputs yield NaN-style 0.0
    (the consumer should treat 0.0 with a missing source as 'unknown').
    """
    metrics: dict[str, float] = {}
    cyc  = counters.get("cycles", 0)
    ins  = counters.get("instructions", 0)
    br   = counters.get("branches", 0)
    brm  = counters.get("branch-misses", 0)
    cref = counters.get("cache-references", 0)
    cmis = counters.get("cache-misses", 0)

    if cyc and ins:
        metrics["ipc"] = ins / cyc
    if br and brm:
        metrics["branch_miss_rate"] = brm / br
    if cref and cmis:
        metrics["cache_miss_rate"] = cmis / cref
    return metrics


# ===========================================================================
# §2.5.3 — CPI-axis counter groups (perf_tree_design.md §2.5.6 step 4)
# ===========================================================================
#
# The CPI axis wants cache/TLB/branch miss *rates*, not just aggregate
# cache-misses. A modern PMU exposes only 4–8 general counters, so asking
# for all events at once forces perf to time-multiplex and scale — noisy.
# We instead run perf stat in small groups (≤4 events each) so every event
# is counted for the whole run. Each group is one extra workload execution.
#
                                                                    
# (best-effort) — we try an Intel-style event and accept a miss.

# Each tuple is one `perf stat` invocation. Events that the CPU/perf does
# not support come back as `<not supported>` and are simply absent.
CPI_EVENT_GROUPS: tuple[tuple[str, ...], ...] = (
    ("cycles", "instructions", "branches", "branch-misses"),
    ("L1-dcache-loads", "L1-dcache-load-misses", "LLC-loads", "LLC-load-misses"),
    ("dTLB-loads", "dTLB-load-misses"),
    ("l2_rqsts.references", "l2_rqsts.miss"),          # best-effort (Intel)
)


def collect_cpi_counters(binary: Path, args: list[str], cwd: Path | None = None,
                         timeout_s: int = 300) -> tuple[dict[str, int | float], list[str]]:
    """Run the CPI_EVENT_GROUPS; return (merged counters, notes).

    One workload run per group. A group that fails entirely (e.g. perf
    paranoid, or every event unsupported) contributes nothing and adds a
    note — the consumer treats absent counters as 'unknown' (None rate)."""
    merged: dict[str, int | float] = {}
    notes: list[str] = []
    for group in CPI_EVENT_GROUPS:
        res = run_perf_stat(binary=binary, args=args, cwd=cwd,
                            events=group, timeout_s=timeout_s)
        if res.ok:
            merged.update(res.counters)
            missing = [e for e in group if e not in res.counters]
            if missing:
                notes.append(f"perf_stat group {group[0]}..: "
                              f"unsupported events {missing}")
        else:
            notes.append(f"perf_stat group {group[0]}..: {res.error}")
    return merged, notes


def derive_cpi_metrics(counters: dict[str, int | float]) -> dict[str, float | None]:
    """Compute the §2.5.3 CPI-axis rates from merged counters.

    Returns a dict with keys cpi / ipc / l1d_miss_rate / l2_miss_rate /
    llc_miss_rate / dtlb_miss_rate / branch_mispredict_rate. A rate is None
    when its source counters were not collected — never silently 0.0.

    Unit (Step A — 2026-06-01): all miss / mispredict rates emit on the
    **0-100** scale (was 0-1), so they're directly comparable to dispatch
    thresholds in stage_b_opt_design.md (e.g. mispredict > 20%). `cpi`
    and `ipc` are ratios, not percentages — those stay unscaled."""
    def rate_pct(misses: str, refs: str) -> float | None:
        m, r = counters.get(misses), counters.get(refs)
        if m is None or r is None or not r:
            return None
        return (m / r) * 100.0

    cyc = counters.get("cycles")
    ins = counters.get("instructions")
    out: dict[str, float | None] = {
        "cpi": (cyc / ins) if (cyc and ins) else None,
        "ipc": (ins / cyc) if (cyc and ins) else None,
        "l1d_miss_rate":          rate_pct("L1-dcache-load-misses", "L1-dcache-loads"),
        "l2_miss_rate":           rate_pct("l2_rqsts.miss", "l2_rqsts.references"),
        "llc_miss_rate":          rate_pct("LLC-load-misses", "LLC-loads"),
        "dtlb_miss_rate":         rate_pct("dTLB-load-misses", "dTLB-loads"),
        "branch_mispredict_rate": rate_pct("branch-misses", "branches"),
    }
    return out
