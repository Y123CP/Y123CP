"""Shared W2 measurement primitives.

The single low-level "run a harness binary under perf" layer, used by BOTH:

  * `verify.w2`      — the W2 performance gate (perf stat -r N, precise wall).
  * `perf_opt.hot_probe` — hotspot locate / characterize (perf record, self-time;
                    added here when hot_probe lands, sharing this scaffolding).

Two perf modes answer two different questions and are NOT interchangeable:

  * `perf_stat`  — COUNTING. Precise whole-program task-clock + counters over
                   `repeats` runs → a low-cv mean. This is what the W2 gate
                   needs to resolve 0.3-1% deltas.
  * `perf_record` (later) — SAMPLING. Per-function self-time distribution over
                   ONE long run. This is what hot_probe needs to find hotspots.

Measurement discipline (host-calibrated, see comments):
  * pin to an isolated core (`taskset -c 16`; host has isolcpus=16,34);
  * `perf stat -x , -r 30` + CSV parse (no regex on free-form text);
  * `warmup` throwaway runs before the measured batch.

Harness CLI (from harness_gen): `harness <op> <input-file> [iters]`. `iters`
loops the op in-process; `autotune_iters` sizes it so one run is ~TARGET_WALL s.
"""

from __future__ import annotations

import csv as _csv
import io as _io
import logging
import os
import subprocess
import tempfile
import time
from dataclasses import dataclass, field
from pathlib import Path

logger = logging.getLogger("verify.measure")

# ── config (host-calibrated defaults; override via env / kwargs) ─────────────
# Default pin core 16: this host isolates CPUs 16,34 via
# `isolcpus=16,34 nohz_full=16,34 rcu_nocbs=16,34`. Pinning there avoids SMT
# cache contention, scheduler-tick noise, and RCU storms. `--pin-cpu 16`
# measured cv 0.6% → 0.002% (40× noise compression — single biggest ROI).
# On a host without isolcpus, set PERF_STAT_PIN_CPU=-1 to disable pinning.
_DEFAULT_PIN_CPU = 16
REPEATS = 30          # `perf stat -r N`. At -r 10 stddev ±0.15-0.25% masked
                      # 0.5-1% lifts; at -r 30 stddev ±0.06-0.08%.
WARMUP = 3            # throwaway runs before the measured batch.
TARGET_WALL = 1.5     # autotune target: one measured run ≈ this many seconds.
RUN_TIMEOUT = 600     # per perf invocation (s).
# A fixed repeat count is a fixed cost only if every op costs the same. It does
# not: `REPEATS + WARMUP` runs of a 21.7s op need 716s, so RUN_TIMEOUT killed
# the measurement — silently, as a crash-shaped TimeoutExpired — for any op
# slower than 600/33 ≈ 18s. That is not a property of one project; it is the
# arithmetic. So spend a TIME budget instead of a run count, and let the first
# warmup run (which happens anyway) price it.
MEASURE_BUDGET_S = 240   # target wall time for one op's measured batch.
MIN_REPEATS = 5          # never go below this: fewer runs cannot resolve a
                         # sub-percent lift no matter how steady the op is.

# perf-stat counters. task-clock = wall; the rest cross-validate the wall Δ.
_PERF_EVENTS = ("task-clock,instructions,cycles,L1-dcache-loads,"
                "L1-dcache-load-misses,LLC-load-misses,branches,branch-misses")


def resolve_pin_cpu(pin_cpu: int | None) -> int:
    """None → env `PERF_STAT_PIN_CPU` or default 16. A negative value disables
    pinning (returned as-is; callers treat <0 as 'no taskset')."""
    if pin_cpu is not None:
        return pin_cpu
    env = os.environ.get("PERF_STAT_PIN_CPU")
    if env is not None:
        try:
            return int(env)
        except ValueError:
            pass
    return _DEFAULT_PIN_CPU


def _pin_prefix(pin_cpu: int) -> list[str]:
    return ["taskset", "-c", str(pin_cpu)] if pin_cpu >= 0 else []


def check_isolation(pin_cpu: int | None = None) -> None:
    """Soft preflight: warn (do not abort) if the pin core is not an isolated
    CPU — pinning to a non-isolated core has no benefit and may even regress if
    another process shares it. Verify with `cat /sys/.../cpu/isolated`."""
    cpu = resolve_pin_cpu(pin_cpu)
    if cpu < 0:
        return
    try:
        isolated = Path("/sys/devices/system/cpu/isolated").read_text().strip()
    except OSError:
        isolated = ""
    iso_set: set[int] = set()
    for part in isolated.split(","):
        part = part.strip()
        if "-" in part:
            a, b = part.split("-", 1)
            iso_set.update(range(int(a), int(b) + 1))
        elif part.isdigit():
            iso_set.add(int(part))
    if cpu not in iso_set:
        logger.warning(
            "pin CPU %d is NOT in the host's isolated set (%r) — W2 numbers "
            "will be noisier; set PERF_STAT_PIN_CPU=-1 to disable pinning, or "
            "pin to an isolcpus core.", cpu, isolated or "none")


# ── measurement result ───────────────────────────────────────────────────────
@dataclass
class Measurement:
    """One `perf stat -r N` result for a binary on one workload run.

    task_clock_ms is the wall-clock the W2 gate compares. The counters
    cross-validate the wall Δ (insns / IPC / cache / branch predictability).
    cv_pct is perf's stddev-% on task-clock across the N repeats — the noise
    the tolerance is scaled against.
    """
    label: str
    binary: Path
    task_clock_ms: float
    instructions: int = 0
    cycles: float = 0.0
    l1_loads: int = 0
    l1d_misses: float = 0.0
    llc_misses: float = 0.0
    branches: float = 0.0
    branch_misses: float = 0.0
    cv_pct: float = 0.0
    raw: dict = field(default_factory=dict)


# ── perf stat (counting) ─────────────────────────────────────────────────────
def perf_stat(binary: Path, args: list[str], *,
              repeats: int = REPEATS, warmup: int = WARMUP,
              pin_cpu: int | None = None, label: str = "") -> Measurement:
    """`perf stat -x , -r <repeats>` a binary with `args`, parsed from CSV.

    `args` is the complete argument vector (e.g. `[op, str(input), str(iters)]`).
    stdout/stderr of the binary go to /dev/null; perf's CSV goes to its own
    `-o` file so a bench printing progress to stderr can't corrupt the stream.
    """
    cpu = resolve_pin_cpu(pin_cpu)
    pin = _pin_prefix(cpu)
    argv = [str(a) for a in args]

    # Time the first warmup run: it is the only cost estimate available here,
    # it costs nothing extra, and both the repeat count and the timeout below
    # depend on it.
    single_s = 0.0
    for index in range(warmup):
        started = time.monotonic()
        subprocess.run([*pin, str(binary), *argv],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if index == 0:
            single_s = time.monotonic() - started

    if single_s > 0:
        affordable = int(MEASURE_BUDGET_S / single_s)
        adjusted = max(MIN_REPEATS, min(repeats, affordable))
        if adjusted != repeats:
            logger.info(
                "[measure] %s: %.1fs per run → -r %d (from %d) to stay inside "
                "the %ds budget", label or "op", single_s, adjusted, repeats,
                MEASURE_BUDGET_S)
            repeats = adjusted
    # Headroom over the batch's own predicted cost, never below the floor: a
    # timeout must fire on a hang, not on an op that is merely expensive.
    timeout = max(RUN_TIMEOUT, int((repeats + 1) * single_s * 2) + 60)

    perf_out = tempfile.mktemp(suffix=".perfstat")
    try:
        subprocess.run(
            [*pin, "perf", "stat", "-x", ",", "-r", str(repeats),
             "-o", perf_out, "-e", _PERF_EVENTS, str(binary), *argv],
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            timeout=timeout,
        )
        with open(perf_out, encoding="utf-8", errors="replace") as f:
            out = f.read()
    finally:
        try:
            os.unlink(perf_out)
        except FileNotFoundError:
            pass

    # CSV rows: <value>,<unit>,<event>,<run-time/stddev%>,... For task-clock
    # under -r, column index 3 carries the run-to-run stddev% (verbatim from
    # the calibrated bench_pipeline parse; do not reshuffle indices).
    vals: dict[str, float] = {}
    cv = 0.0
    for row in _csv.reader(_io.StringIO(out)):
        if len(row) < 3:
            continue
        event = row[2].strip()
        try:
            val = float(row[0].replace(",", ""))
        except (ValueError, IndexError):
            continue
        vals[event] = val
        if event == "task-clock" and len(row) > 3 and row[3].strip():
            try:
                cv = float(row[3].strip().rstrip("%"))
            except ValueError:
                pass

    if "task-clock" not in vals:
        raise RuntimeError(f"could not parse perf stat output: {out[-500:]}")

    return Measurement(
        label=label, binary=binary,
        task_clock_ms=vals.get("task-clock", 0.0),
        instructions=int(vals.get("instructions", 0)),
        cycles=vals.get("cycles", 0.0),
        l1_loads=int(vals.get("L1-dcache-loads", 0)),
        l1d_misses=vals.get("L1-dcache-load-misses", 0.0),
        llc_misses=vals.get("LLC-load-misses", 0.0),
        branches=vals.get("branches", 0.0),
        branch_misses=vals.get("branch-misses", 0.0),
        cv_pct=cv, raw=vals,
    )


def measure_harness(binary: Path, op: str, input_path: Path, iters: int, *,
                    repeats: int = REPEATS, warmup: int = WARMUP,
                    pin_cpu: int | None = None,
                    label: str = "") -> Measurement:
    """Convenience over `perf_stat` for the harness CLI
    `harness <op> <input> <iters>`."""
    return perf_stat(binary, [op, str(input_path), str(iters)],
                     repeats=repeats, warmup=warmup, pin_cpu=pin_cpu,
                     label=label or op)


# ── iters autotuning ─────────────────────────────────────────────────────────
def _time_once(binary: Path, op: str, input_path: Path, iters: int, *,
               pin_cpu: int | None = None, timeout: float = RUN_TIMEOUT) -> float:
    """Wall seconds for one `harness <op> <input> <iters>` run; -1 on error OR
    timeout (a timeout must NOT propagate — it just means 'too slow at this
    iters', handled by the caller)."""
    pin = _pin_prefix(resolve_pin_cpu(pin_cpu))
    t0 = time.monotonic()
    try:
        rc = subprocess.run([*pin, str(binary), op, str(input_path), str(iters)],
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                            timeout=timeout).returncode
    except subprocess.TimeoutExpired:
        return -1.0
    dt = time.monotonic() - t0
    return dt if rc == 0 else -1.0


def autotune_iters(binary: Path, op: str, input_path: Path, *,
                   target_wall: float = TARGET_WALL,
                   pin_cpu: int | None = None,
                   probe_timeout: float = 120.0) -> int:
    """Size `iters` so one measured run is ~`target_wall` s. Returns 0 if the op
    errors / times out even at the smallest probe. Same iters MUST be reused for
    pre/post/absolute so the comparison is apples-to-apples.

    Two probes, differenced. Every run pays a cost that has nothing to do with
    `iters` — process start, paging in the harness binary, reading the input —
    and dividing a single run's wall time by its iteration count charges all of
    it to the first iteration.

    Dividing straight through is right only while the fixed cost is negligible
    against the total, which is what the ×8 growth quietly arranged: the loop
    kept going until the run reached 0.3 s, by which point iterations dominated.
    But the growth stops the moment a run clears 0.3 s, and when the FIRST probe
    already clears it the loop exits at `probe=1` with `per_iter` equal to the
    whole run. Measured on libxml2, with a cold page cache: `t(1)` came in at
    roughly 0.5 s and four operations were sized at 1-4 iterations against the
    5000-200000 they need, running 1 ms instead of 1.5 s, with cv degrading from
    0.07% to 1.2%. Warm, on the same binaries, the fixed cost is 4 ms and the
    naive per-iteration figure overstates by 19x to 582x — the same error, just
    below the threshold that exposes it.

    So measure the fixed cost instead of hoping it is small: `t(probe) - t(1)`
    is iteration work alone, and the first run doubles as the warm-up that keeps
    a cold first execution out of the estimate. Slow workloads (seconds per
    iteration) still resolve at the second probe and still get a correctly small
    `iters`; there is NO 200-iter floor (the old floor made a slow op time out at
    600 s repeatedly)."""
    # Discarded: absorbs page-in and any first-execution cost so that neither
    # of the two timed probes carries it.
    if _time_once(binary, op, input_path, 1,
                  pin_cpu=pin_cpu, timeout=probe_timeout) < 0:
        return 0
    fixed = _time_once(binary, op, input_path, 1,
                       pin_cpu=pin_cpu, timeout=probe_timeout)
    if fixed < 0:
        return 0                           # errors / too slow even at one iter
    probe = 2
    while probe < 100_000_000:
        t = _time_once(binary, op, input_path, probe,
                       pin_cpu=pin_cpu, timeout=probe_timeout)
        if t < 0:
            return 0
        span = t - fixed                   # iteration work, fixed cost removed
        if span >= 0.3:
            per_iter = span / (probe - 1)
            if per_iter <= 0:
                return 0
            return max(1, min(int(target_wall / per_iter), 100_000_000))
        probe *= 8
    return 0


# ── workload input selection ─────────────────────────────────────────────────
def pick_input(harness_dir: Path, op: str) -> Path | None:
    """The measurement input for `op`: prefer the large `perf_inputs/<op>.perf.bin`
    (built by the harness's `gen-perf`), else the largest corpus input as a
    cheap 'heaviest' proxy. Never the small correctness corpus inputs directly —
    they profile as per-call libc overhead, not the algorithm."""
    p = harness_dir / "perf_inputs" / f"{op}.perf.bin"
    if p.is_file() and p.stat().st_size > 0:
        return p
    cd = harness_dir / "corpus" / op
    if cd.is_dir():
        files = [f for f in cd.glob("*.bin") if f.is_file()]
        if files:
            return max(files, key=lambda f: f.stat().st_size)
    return None
