"""Workload manifest parsing + reproducible wall-clock runner.

Per DESIGN.md v2.2 §5.4.0, every benchmark has a TOML manifest pinning
the binary, args, input, warmup count, measure count, and stability
requirements (CV / min runtime). This module:

  1. parses the manifest
  2. runs the binary `warmup_runs + measure_runs` times
  3. records wall-clock per run (process timing, not CPU)
  4. computes mean / stddev / CV and decides if the result is stable
  5. (optionally) verifies stdout/stderr against a reference

Other counter-collection (perf stat, perf record, asm) lives in sibling
modules and reads the manifest to know what to invoke.

Manifest schema (§5.4.0):

    [workload]
    name             = "bzip2-compress-large"
    binary           = "bzip2"                # binary name (target/release/<binary>) or absolute path
    args             = ["-c", "-9", "$INPUT"] # $INPUT is substituted from input field
    input            = "inputs/large.tar"     # path resolved relative to manifest dir
    warmup_runs      = 3
    measure_runs     = 10
    timeout_s        = 120                    # per-run wall-clock cap

    [stability]
    required_cv      = 0.05
    min_runtime_ms   = 100

    [oracle]                                  # optional — used by trace_diff later
    reference_output = "reference_output/large.bz2"
    trace_diff_fn    = ["BZ2_bzCompress"]

Standard-input is not piped in v1 — workloads read from CLI-passed
files. stdin redirection is a v1.1 add-on if any benchmark needs it.
"""

from __future__ import annotations

import logging
import secrets
import statistics
import subprocess
import time
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

from profiling.evidence import WorkloadStability

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Manifest data classes
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class Workload:
    """One workload manifest, fully resolved (paths absolute, $INPUT substituted).

    v1.1 added [profile] section.
    v1.2 added inline oracle fields:
      stdout_hash:    expected sha256 hex of stdout — direct hash check
      exit_code:      expected exit code (default 0)
      stderr_pattern: optional regex that stderr must match
      input_sha256:   expected sha256 of input file (loader verifies)
    Per `docs/workloadGenerate.md §2`, these enable single-manifest oracle
    without external reference_output files. Old `reference_output` is
    retained for back-compat; `stdout_hash` takes precedence when both set.
    """
    manifest_path:    Path
    name:             str
    binary:           str           # absolute path or bare exec name
    args:             list[str]     # already-substituted (no $INPUT placeholder)
    input_path:       Path | None
    warmup_runs:      int
    measure_runs:     int
    timeout_s:        int
    required_cv:      float
    min_runtime_ms:   float
    # [stability].regression_threshold_pct — perf_opt's W2 gate (impl §0.5):
    # delta_pct vs baseline > this %  ⇒  W2 fail → atom rollback.
    # NOT the same as required_cv (which judges single-measurement stability).
    regression_threshold_pct: float   = 3.0
    # [oracle] inline (v1.2)
    stdout_hash:      str | None    = None      # sha256 hex; None ⇒ no hash check
    exit_code:        int           = 0
    stderr_pattern:   str | None    = None      # regex; None ⇒ no stderr check
    # [oracle] legacy (kept for back-compat)
    reference_output: Path | None   = None
    trace_diff_fn:    list[str]     = field(default_factory=list)
    # Which output stream to hash for the trace_diff oracle.
    trace_diff_stream: str          = "stdout"   # "stdout" | "stderr"
    # [workload] input integrity (v1.2)
    input_sha256:     str | None    = None      # expected sha256 of input file
    # [workload] library-benchmark harness (v1.3) — when the staged Rust
    # project is a LIBRARY (no real workload-driver bin, e.g. libcsv),
    # `harness_dir` points to a separate Cargo project under workloads/
    # that depends on the staged lib and produces a binary that drives
    # the workload. _resolve_binary checks <harness_dir>/target/release/
    # before the staged project's target/.
    harness_dir:      Path | None   = None
    # [workload] wrapper script (v1.3) — for roundtrip / multi-step
    # tests (e.g. bzip2 compress|decompress|diff). When set, invocation
    # becomes `<wrapper> <binary_path> <args...>` instead of `<binary>
    # <args...>`. Wrapper is responsible for calling binary as needed.
    wrapper:          Path | None   = None
    # [profile] — v1.1
    call_graph_mode:  str           = "dwarf"
    frequency:        int           = 99
    # [profile].iterations — v2.5: the workload's hot-loop iteration count,
    # if known. Feeds HotspotProfile.ic_axis.insns_per_iter (§2.5.4); when
    # absent, insns_per_iter is left None (honestly "unknown").
    iterations:       int | None    = None
    # [experiment] — v1.2; only populated for W3 manifests
    versions:         list[str]     = field(default_factory=list)
    c_compile_flags:  str           = ""
    rust_compile_flags: str         = ""
    # name → input file path (absolute); each entry is one held-out input
    # to sweep at experiment time. Empty for W1/W2.
    experiment_inputs: dict[str, Path] = field(default_factory=dict)


# ---------------------------------------------------------------------------
# Loading
# ---------------------------------------------------------------------------

def load_manifest(manifest_path: Path) -> Workload:
    """Parse a workload manifest TOML; resolve all paths relative to its directory.

    Substitutes `$INPUT` in the args list with the resolved input path.
    Raises ValueError on malformed manifest.
    """
    manifest_path = Path(manifest_path).resolve()
    if not manifest_path.is_file():
        raise FileNotFoundError(f"manifest not found: {manifest_path}")

    raw = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    base = manifest_path.parent

    w = raw.get("workload") or {}
    s = raw.get("stability") or {}
    o = raw.get("oracle") or {}
    p = raw.get("profile") or {}
    e = raw.get("experiment") or {}
    e_inputs = raw.get("experiment", {}).get("inputs") or {}

    for required in ("name", "binary", "args"):
        if required not in w:
            raise ValueError(f"manifest missing [workload].{required}: {manifest_path}")

    cg_mode = str(p.get("call_graph_mode", "dwarf")).lower()
    if cg_mode not in ("dwarf", "fp", "lbr"):
        raise ValueError(f"invalid [profile].call_graph_mode={cg_mode!r}; "
                         f"must be one of dwarf|fp|lbr")

    input_path: Path | None = None
    if "input" in w:
        input_path = (base / w["input"]).resolve()
        if not input_path.exists():
            logger.warning(f"[workload] input path does not exist (yet): {input_path}")

    args_resolved = [
        str(input_path) if (a == "$INPUT" and input_path is not None) else a
        for a in w["args"]
    ]

    ref_out = (base / o["reference_output"]).resolve() if "reference_output" in o else None

    td_stream = str(o.get("trace_diff_stream", "stdout")).lower()
    if td_stream not in ("stdout", "stderr"):
        raise ValueError(f"invalid [oracle].trace_diff_stream={td_stream!r}; "
                         f"must be 'stdout' or 'stderr'")

    # Resolve experiment inputs (W3 sweep)
    exp_inputs: dict[str, Path] = {}
    for k, v in (e_inputs.items() if isinstance(e_inputs, dict) else []):
        if not isinstance(v, str):
            continue
        exp_inputs[k] = (base / v).resolve()

    # v1.3: library-benchmark harness + wrapper
    harness_dir = (base / w["harness_dir"]).resolve() if "harness_dir" in w else None
    wrapper     = (base / w["wrapper"]).resolve() if "wrapper" in w else None

    return Workload(
        manifest_path    = manifest_path,
        name             = w["name"],
        binary           = w["binary"],
        args             = args_resolved,
        input_path       = input_path,
        warmup_runs      = int(w.get("warmup_runs", 3)),
        measure_runs     = int(w.get("measure_runs", 10)),
        timeout_s        = int(w.get("timeout_s", 120)),
        required_cv      = float(s.get("required_cv", 0.05)),
        min_runtime_ms   = float(s.get("min_runtime_ms", 100)),
        regression_threshold_pct = float(s.get("regression_threshold_pct", 3.0)),
        # [oracle] inline (v1.2)
        stdout_hash      = o.get("stdout_hash"),
        exit_code        = int(o.get("exit_code", 0)),
        stderr_pattern   = o.get("stderr_pattern"),
        # [oracle] legacy
        reference_output = ref_out,
        trace_diff_fn    = list(o.get("trace_diff_fn") or []),
        trace_diff_stream= td_stream,
        # [workload] input integrity
        input_sha256     = w.get("input_sha256"),
        # [workload] harness + wrapper (v1.3)
        harness_dir      = harness_dir,
        wrapper          = wrapper,
        # [profile]
        call_graph_mode  = cg_mode,
        frequency        = int(p.get("frequency", 99 if cg_mode == "dwarf" else 999)),
        iterations       = int(p["iterations"]) if "iterations" in p else None,
        # [experiment]
        versions         = list(e.get("versions") or []),
        c_compile_flags  = str(e.get("c_compile_flags", "")),
        rust_compile_flags = str(e.get("rust_compile_flags", "")),
        experiment_inputs = exp_inputs,
    )


# ---------------------------------------------------------------------------
# Running
# ---------------------------------------------------------------------------

@dataclass
class RunOutcome:
    """One measure-run result. Wall-clock in ms; output kept for oracle check."""
    elapsed_ms:  float
    returncode:  int
    stdout:      bytes
    stderr:      bytes


def _resolve_rand(args: list[str]) -> list[str]:
    """Per-invocation substitution of $RAND -> 12-hex random string.

    Unlike $INPUT (resolved once at manifest load to a stable absolute
    path), $RAND must be FRESH every subprocess call so workloads that
    embed it in shared resource names — e.g. tmux's socket `-L bench_$RAND`
    — get a unique resource per run. Without this, repeated launches
    (warmup 3 + measure 10 = 13) collide on socket / tempfile names and
    every other run fails with a torn-down state from the prior run.

    Substitution is a no-op when no arg contains the placeholder, so
    the cost is one short scan per call when unused.
    """
    if not any("$RAND" in a for a in args):
        return list(args)
    rand = secrets.token_hex(6)
    return [a.replace("$RAND", rand) for a in args]


def _resolve_binary(binary: str, project_path: Path,
                    *, harness_dir: Path | None = None) -> Path:
    """Convert a manifest binary spec into an absolute Path.

    Resolution order:
      1. If `binary` is absolute, use as-is
      2. If `harness_dir` set, look in `<harness_dir>/target/release/<binary>`
      3. Look in `<project>/target/release/<binary>`
      4. Look in PATH (shutil.which) — for system binaries
    """
    p = Path(binary)
    if p.is_absolute():
        return p
    if harness_dir is not None:
        harness_release = harness_dir / "target" / "release" / binary
        if harness_release.is_file():
            return harness_release
    cargo_release = project_path / "target" / "release" / binary
    if cargo_release.is_file():
        return cargo_release
    # last resort: search PATH
    import shutil
    found = shutil.which(binary)
    if found:
        return Path(found)
    raise FileNotFoundError(
        f"binary {binary!r} not found at {harness_dir}/target/release "
        f"or {cargo_release} or on PATH"
    )


def run_once(binary: Path, args: list[str], timeout_s: int,
             cwd: Path | None = None,
             wrapper: Path | None = None) -> RunOutcome:
    """Execute `binary args` once, return wall-clock + outputs.

    `wrapper`, when set, runs `[wrapper, binary, ...args]` instead of
    `[binary, ...args]`. Used by the W1 oracle path for workloads whose
    output goes to a FILE (not stdout) — the wrapper script is responsible
    for running the binary and writing a stable signature (e.g.
    sha256 of the output file) to stdout, which the oracle then hashes
    via `workload.stdout_hash`. W2 perf measurement should NOT pass
    `wrapper` — perf stat needs to measure the raw binary, not the
    wrapper's overhead."""
    args = _resolve_rand(args)
    cmd = ([str(wrapper), str(binary), *args] if wrapper is not None
           else [str(binary), *args])
    t0 = time.perf_counter_ns()
    proc = subprocess.run(
        cmd,
        capture_output=True,
        cwd=str(cwd) if cwd else None,
        timeout=timeout_s,
    )
    t1 = time.perf_counter_ns()
    return RunOutcome(
        elapsed_ms = (t1 - t0) / 1_000_000.0,
        returncode = proc.returncode,
        stdout     = proc.stdout,
        stderr     = proc.stderr,
    )


def run_workload(workload: Workload, project_path: Path) -> tuple[WorkloadStability, list[RunOutcome]]:
    """Run a workload `warmup_runs + measure_runs` times; return stability + measure outcomes.

    Warmup outcomes are discarded for stability stats but kept on disk for debug.
    A non-zero return code in *any* run aborts (returns stable=False with
    stub stats). This keeps the pipeline honest — broken workloads never
    masquerade as fast.
    """
    binary = _resolve_binary(workload.binary, project_path,
                             harness_dir=workload.harness_dir)
    if not binary.is_file():
        raise FileNotFoundError(f"resolved binary not a file: {binary}")

    logger.info(f"[workload] {workload.name}: binary={binary} "
                f"args={workload.args} warmup={workload.warmup_runs} "
                f"measure={workload.measure_runs}")

    # warmup
    for i in range(workload.warmup_runs):
        out = run_once(binary, workload.args, workload.timeout_s, cwd=project_path)
        if out.returncode != 0:
            return _stub_stability(0), []
        logger.debug(f"[workload]  warmup {i + 1}/{workload.warmup_runs}: "
                     f"{out.elapsed_ms:.2f} ms")

    # measurement
    measures: list[RunOutcome] = []
    for i in range(workload.measure_runs):
        out = run_once(binary, workload.args, workload.timeout_s, cwd=project_path)
        if out.returncode != 0:
            logger.error(f"[workload]  run {i + 1} exited {out.returncode}; "
                         f"stderr={out.stderr[:200]!r}")
            return _stub_stability(workload.measure_runs), measures
        measures.append(out)

    samples = [o.elapsed_ms for o in measures]
    mean = statistics.fmean(samples)
    stddev = statistics.pstdev(samples) if len(samples) > 1 else 0.0
    cv = (stddev / mean) if mean > 0 else 0.0

    stability = WorkloadStability(
        runs       = len(samples),
        mean_ms    = mean,
        stddev_ms  = stddev,
        cv         = cv,
        min_ms     = min(samples),
        max_ms     = max(samples),
        stable     = (cv <= workload.required_cv) and (mean >= workload.min_runtime_ms),
    )

    if not stability.stable:
        if cv > workload.required_cv:
            logger.warning(f"[workload]  CV={cv:.4f} > required {workload.required_cv} "
                           f"— measurement is noisy")
        if mean < workload.min_runtime_ms:
            logger.warning(f"[workload]  mean={mean:.2f}ms < min_runtime_ms="
                           f"{workload.min_runtime_ms}ms — too short to trust")
    else:
        logger.info(f"[workload]  STABLE: mean={mean:.2f}ms cv={cv:.4f} "
                    f"({len(samples)} runs)")

    return stability, measures


def _stub_stability(runs: int) -> WorkloadStability:
    return WorkloadStability(
        runs=runs, mean_ms=0.0, stddev_ms=0.0, cv=0.0,
        min_ms=0.0, max_ms=0.0, stable=False,
    )
