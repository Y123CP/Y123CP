""                                                                            

                                                                      
                                                                    
                                                              
                                                                     
                                                                    
                                                                       
                                                                
                                             

                                                          

                                                                 
                                                                  
                                
                                                                 
                                                                    
                                                                 
                                                            
                                                                   
                                                                    
                                                                     
                                                                 
                                                                   

                                                               
                                                                     
                                                                     
                                                                    
                                                                 

      

                    
                             
                                                 
                                                     
                                                                  

                            
                                                         
                                                            
                                            
                                                                              
                                                   
     
                           
                                                                       
                                                      
                                                                             
                                                                                             
                                                                                                           

                    
                                                                   
                                                                          
                                                         
                                        
   

from __future__ import annotations

import argparse
import logging
import os
import re
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
import tomllib
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path

# This file lives at `<repo>/script_c2rust/perf_compare.py`.
# Make `<repo>/script_c2rust/` importable so `perf_opt`, `profiling`,
# `utils` resolve when invoked as either `python -m perf_compare`
# (already on sys.path) or `python script_c2rust/perf_compare.py`
# (needs the parent dir injected).
_REPO_SCRIPTS = Path(__file__).resolve().parent
if str(_REPO_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_REPO_SCRIPTS))

# Default report directory: `<repo_root>/report/<label>_perf_compare.md`.
# `_REPO_SCRIPTS` is `<repo_root>/script_c2rust/`, so its parent is the
# repo root.
_DEFAULT_REPORT_DIR = _REPO_SCRIPTS.parent / "report"

# Intentionally do NOT import perf_opt.bench_pipeline — it transitively
# pulls in Stage A's tree-sitter dependency chain, which this diagnostic
# script does not need. We inline `_rewire_path_dep` below instead.
from perf_opt.fair_build import _audit_c_reference, audit as fair_audit  # noqa: E402
from profiling.workload import load_manifest  # noqa: E402
from utils.toolchain import (  # noqa: E402
    require_c_compiler_version,
    require_rustc_version,
    required_c2rust_version,
    required_c_compiler_version,
    required_rust_toolchain,
    required_rustc_minor,
    write_rust_toolchain,
)

logger = logging.getLogger("perf_compare")


# ───────────────────────────────────────────────────────────────────
# Tunable thresholds for measurement-consistency check (NOT static
# environment requirements). This script targets RELATIVE fairness
# between C / c2rust_raw / our-lift on a shared server, so we do not
# require governor=performance or an idle host. What we DO require is
# that the host stays consistent ACROSS the three versions' measurement
# windows — otherwise a load spike during one version invalidates the
# relative comparison.
# ───────────────────────────────────────────────────────────────────

# Max acceptable difference in 1-min load average across the three
# measurement windows. If load was 3.0 when C was measured but 5.5
# when ours was measured, drift = 2.5 → exceeds threshold → warn.
# Default 2.0 tolerates moderate background activity on a shared box;
# only a real spike (someone starting a chromium build) trips it.
LOAD_DRIFT_WARN_THRESHOLD    = 2.0


# ───────────────────────────────────────────────────────────────────
# Inlined helpers (copied verbatim from perf_opt/bench_pipeline.py to
# avoid the tree-sitter dependency chain). If bench_pipeline updates
# these, mirror the change here.
# ───────────────────────────────────────────────────────────────────

_PATH_DEP_RE = re.compile(
    r'^([ \t]*)([A-Za-z_][A-Za-z_0-9-]*)\s*=\s*\{([^}]*)\}',
    re.MULTILINE,
)

_PROFILE_RELEASE_BLOCK_RE = re.compile(
    r'(?ms)^\[profile\.release\]\s*\n(?:(?!^\[).)*'
)

_PROFILE_BLOCK = """
# fair-build profile — auto-injected by script_c2rust/perf_compare.py.
[profile.release]
opt-level     = 3
lto           = "fat"
codegen-units = 1
panic         = "abort"
debug         = false
"""

_PROFILE_KEYS_CANON: list[tuple[str, str]] = [
    ("opt-level",     "3"),
    ("lto",           '"fat"'),
    ("codegen-units", "1"),
    ("panic",         '"abort"'),
    ("debug",         "false"),
]

_CARGO_CONFIG = """[build]
rustflags = ["-Ctarget-cpu=native"]
"""


def _read_crate_name(crate_dir: Path) -> str:
    cargo = tomllib.loads((crate_dir / "Cargo.toml").read_text())
    return cargo.get("lib", {}).get("name") or cargo["package"]["name"]


def _ensure_fair_build(crate_dir: Path) -> None:
    """Idempotently top up `Cargo.toml` + `.cargo/config.toml` so a fresh
    rsync of an arbitrary stage output becomes a valid fair-build target.
    Mirrors `perf_opt/bench_pipeline.py:_ensure_fair_build`."""
    write_rust_toolchain(crate_dir)
    cargo = crate_dir / "Cargo.toml"
    text = cargo.read_text()
    m = _PROFILE_RELEASE_BLOCK_RE.search(text)
    if m is None:
        text = text.rstrip() + "\n" + _PROFILE_BLOCK
        cargo.write_text(text)
    else:
        block_text = m.group(0)
        missing: list[str] = []
        for key, val in _PROFILE_KEYS_CANON:
            if not re.search(
                rf"(?m)^\s*{re.escape(key)}\s*=", block_text,
            ):
                missing.append(f"{key:<13} = {val}")
        if missing:
            new_block = block_text.rstrip() + "\n" + "\n".join(missing) + "\n"
            text = text[:m.start()] + new_block + text[m.end():]
            cargo.write_text(text)
    cargo_cfg_dir = crate_dir / ".cargo"
    cargo_cfg_dir.mkdir(exist_ok=True)
    cargo_cfg = cargo_cfg_dir / "config.toml"
    if cargo_cfg.exists():
        cfg_text = cargo_cfg.read_text()
        if "-Ctarget-cpu=native" not in cfg_text:
            if re.search(r'(?m)^\s*rustflags\s*=', cfg_text):
                cargo_cfg.write_text(re.sub(
                    r'(?m)^(\s*rustflags\s*=\s*).*$',
                    r'\1["-Ctarget-cpu=native"]', cfg_text,
                ))
            else:
                cargo_cfg.write_text(cfg_text.rstrip() + "\n\n" + _CARGO_CONFIG)
    else:
        cargo_cfg.write_text(_CARGO_CONFIG)


_HARNESS_USE_RE = re.compile(
    r'use\s+([A-Za-z_]\w*)\s*::\s*src\s*::\s*([A-Za-z_]\w*)\s*::',
)


def _ensure_harness_compat(crate_dir: Path, harness_dir: Path) -> None:
    """Scan harness rs files for `use <dep>::src::<mod>::…` paths whose
    `<mod>` doesn't actually exist in the lib's `src/`. For each missing
    module, inject a thin re-export shim in lib.rs so the harness still
    builds. Mirrors `perf_opt/bench_pipeline.py:_ensure_harness_compat`.

    Critical for measuring `0_raw` against a harness that was written
    for a later stage (where Stage A has split modules); without this
    `use libcsv_safe::src::c_structs::csv_parser;` fails because 0_raw's
    lib.rs only declares `mod libcsv`."""
    if not (crate_dir / "src").is_dir():
        return
    needed: set[str] = set()
    for rs in harness_dir.rglob("*.rs"):
        if "target" in rs.parts:
            continue
        try:
            text = rs.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for m in _HARNESS_USE_RE.finditer(text):
            needed.add(m.group(2))
    lib_rs = crate_dir / "lib.rs"
    if not lib_rs.is_file():
        return
    lib_rs_text = lib_rs.read_text(encoding="utf-8")
    declared: set[str] = set()
    src_block = re.search(
        r'pub\s+mod\s+src\s*\{(.*?)\}', lib_rs_text, flags=re.DOTALL,
    )
    if src_block:
        declared = set(re.findall(
            r'pub\s+mod\s+([A-Za-z_]\w*)\s*;', src_block.group(1)
        ))
    if not declared:
        return
    missing = sorted(needed - declared)
    if not missing:
        return
    main_mod = max(
        declared,
        key=lambda m: (crate_dir / "src" / f"{m}.rs").stat().st_size
                       if (crate_dir / "src" / f"{m}.rs").is_file() else 0,
    )
    # Use absolute `crate::src::<main_mod>::*` rather than `super::<main_mod>`
    # so the shim is resilient to where the regex below ends up
    # injecting it. For c2rust outputs that nest `pub mod src { … pub mod
    # src { … } }` (e.g. heman, whose C source lives under `src/src/`),
    # the `pub\s+mod\s+src\s*\{([^}]*)\}` match grabs the FIRST closing
    # `}` — which can land mid-nest — and `super::ffi` then resolves to
    # an inner mod with no `ffi` child, triggering E0432 at build time.
    # Absolute crate-rooted paths sidestep that geometry entirely.
    shim_lines = [
        f"    pub mod {m} {{ pub use crate::src::{main_mod}::*; }}"
        for m in missing
    ]
    new_text, n = re.subn(
        r'(pub\s+mod\s+src\s*\{)([^}]*)(\})',
        lambda m: m.group(1) + m.group(2).rstrip() + "\n"
                  + "\n".join(shim_lines) + "\n" + m.group(3),
        lib_rs_text, count=1, flags=re.DOTALL,
    )
    if n == 0:
        return  # no pub mod src {} block — leave alone
    lib_rs.write_text(new_text, encoding="utf-8")
    logger.info(
        f"[harness-compat] inject {len(missing)} shim module(s) "
        f"into {lib_rs.name}: {missing} → {main_mod}"
    )


def _rsync_copy(src: Path, dst: Path) -> None:
    """Mirror src into dst, excluding heavy build/state dirs."""
    if dst.exists():
        shutil.rmtree(dst)
    def _ignore(_dir, names):
        return [n for n in names if n in (
            "target", ".git", ".perf_opt", ".cargo"
        )]
    shutil.copytree(src, dst, ignore=_ignore)


def _rewire_path_dep(harness_cargo: Path, lib_dir: Path) -> None:
    """Rewrite every `path = "…"` path-dep in `harness_cargo` to point at
    `lib_dir`. If the dep name differs from `lib_dir`'s actual crate name,
    inject `package = "<crate_name>"` so existing `use <dep_name>::…`
    imports keep resolving without touching the harness's .rs sources.
    Mirrors `perf_opt/bench_pipeline.py:_rewire_path_dep`."""
    text = harness_cargo.read_text()
    crate_name = _read_crate_name(lib_dir)

    def _replace(m: re.Match) -> str:
        indent, dep_name, body = m.group(1), m.group(2), m.group(3)
        if not re.search(r'\bpath\s*=', body):
            return m.group(0)
        new_body = re.sub(
            r'(\bpath\s*=\s*)"[^"]+"',
            f'\\1"{lib_dir}"',
            body,
        )
        if dep_name.replace("-", "_") != crate_name.replace("-", "_"):
            if re.search(r'\bpackage\s*=', new_body):
                new_body = re.sub(
                    r'(\bpackage\s*=\s*)"[^"]+"',
                    f'\\1"{crate_name}"',
                    new_body,
                )
            else:
                new_body = re.sub(
                    r'(\bpath\s*=\s*"[^"]+")',
                    f'\\1, package = "{crate_name}"',
                    new_body,
                    count=1,
                )
        return f"{indent}{dep_name} = {{{new_body}}}"

    text = _PATH_DEP_RE.sub(_replace, text)
    harness_cargo.write_text(text)


# ───────────────────────────────────────────────────────────────────
# Data types
# ───────────────────────────────────────────────────────────────────

@dataclass
class ToolchainInfo:
    clang_bin: str
    clang_version: str
    clang_full_version: str  # first line of `clang-17 --version`
    rustc_minor: str
    rust_toolchain: str
    c2rust_version: str
    host_rustc: str          # `rustc --version` output observed

    @property
    def ok(self) -> bool:
        return True


@dataclass
class AuditSection:
    label: str               # "c2rust_raw" / "ours" / "c_reference"
    errors: list[str]

    @property
    def ok(self) -> bool:
        return not self.errors


@dataclass
class SystemEnvSnapshot:
    """One-shot snapshot of host environment variables that influence
    measurement noise. `governors` / `turbo_disabled` / `loadavg_1min`
    are hard-gated; the rest are recorded for honesty-in-reporting but
    do not block."""
    governors:            list[str]   # one entry per online CPU (deduplicated downstream)
    turbo_disabled:       bool | None # None = neither Intel nor AMD knob found
    turbo_source:         str         # "intel_pstate/no_turbo" / "cpufreq/boost" / "unknown"
    loadavg_1min:         float
    cpu_mhz_observed_min: float
    cpu_mhz_observed_max: float
    cpu_nominal_max_mhz:  float | None
    smt_control:          str | None  # 'on' / 'off' / 'forceoff' / 'notsupported' / None
    isolcpus:             str         # boot-time isolcpus= value; '' = none
    perf_event_paranoid:  int | None
    aslr:                 int | None
    nproc:                int


@dataclass
class Measurement:
    label: str                       # "C" / "c2rust_raw" / "ours"
    binary: Path
    samples_ms: list[float]
    median_ms: float
    cv: float
    min_ms: float
    max_ms: float
    n: int
    build_seconds: float
    binary_size_bytes: int | None = None       # st_size of the built binary
    # Measurement window — used by audit_measurement_consistency to
    # detect cross-version load drift (the only fairness gate left).
    started_at:        datetime | None = None  # ISO timestamp at window start
    ended_at:          datetime | None = None
    load_1min_before:  float | None    = None  # /proc/loadavg before warmup
    load_1min_after:   float | None    = None  # /proc/loadavg after last sample
    notes: list[str] = field(default_factory=list)


@dataclass
class ConsistencyDiagnostic:
    """Post-run drift check across the 3 measurement windows."""
    load_min:        float | None    # min of all per-window load samples
    load_max:        float | None
    load_drift:      float | None    # max - min, or None if any reading missing
    drift_threshold: float
    drift_ok:        bool            # True = comparison is fair (or unknown)
    warnings:        list[str]


@dataclass
class Report:
    project_label: str
    generated_at: str
    manifest_path: Path
    workload_name: str
    workload_args: list[str]
    workload_input: Path | None
    warmup_runs: int
    measure_runs: int
    toolchain: ToolchainInfo
    audits: list[AuditSection]
    env_snapshot: SystemEnvSnapshot
    consistency: ConsistencyDiagnostic
    c_build_cmd: str | None
    c_source_dir: Path | None
    c_project_dir: Path | None
    c2rust_project: Path
    ours_project: Path
    measurements: list[Measurement]
    pin_cpu: int | None = None              # taskset -c N pin; None = no pinning
    raw_harness_dir: Path | None = None     # --raw-harness-dir override for raw measurement
    ours_harness_dir: Path | None = None    # --ours-harness-dir override for ours measurement


# ───────────────────────────────────────────────────────────────────
# Toolchain audit
# ───────────────────────────────────────────────────────────────────

def _read_rustc_version(cwd: Path) -> str:
    """Capture `rustc --version` as observed by cargo in this crate's cwd."""
    try:
        proc = subprocess.run(
            ["rustc", "--version"], cwd=str(cwd),
            capture_output=True, text=True, timeout=30,
        )
    except OSError as e:
        return f"<rustc launch failed: {e}>"
    return (proc.stdout or proc.stderr or "").strip()


def audit_host_clang() -> tuple[Path, str]:
    """Hard-gate clang version. Returns (binary_path, first_line_of_version).
    Raises RuntimeError on mismatch."""
    p = require_c_compiler_version()
    try:
        proc = subprocess.run(
            [str(p), "--version"], capture_output=True, text=True, timeout=10,
        )
        first_line = (proc.stdout or "").splitlines()[0] if proc.stdout else str(p)
    except (OSError, subprocess.TimeoutExpired):
        first_line = f"<{p} --version failed>"
    return p, first_line


def audit_staged_rustc(staged: list[Path]) -> str:
    """Hard-gate rustc version against each staged crate's rust-toolchain.toml
    (which `_ensure_fair_build` wrote). Returns the observed rustc --version
    string (same for all crates since they all pin the same toolchain)."""
    for s in staged:
        require_rustc_version(s)
    return _read_rustc_version(staged[0])


# ───────────────────────────────────────────────────────────────────
# System environment snapshot + hard gate
# ───────────────────────────────────────────────────────────────────

def _read_text(p: Path | str) -> str | None:
    try:
        return Path(p).read_text().strip()
    except (OSError, FileNotFoundError):
        return None


def _read_int(p: Path | str) -> int | None:
    txt = _read_text(p)
    if txt is None:
        return None
    try:
        return int(txt)
    except ValueError:
        return None


def read_system_env() -> SystemEnvSnapshot:
    """Capture every release-relevant kernel/CPU knob in one pass.
    Read-only; never modifies the system."""
    import glob

    # CPU governor — one per online CPU.
    governors: list[str] = []
    for gov_path in sorted(glob.glob("/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor")):
        g = _read_text(gov_path)
        if g:
            governors.append(g)

    # Turbo: Intel exposes /sys/.../intel_pstate/no_turbo (1=disabled);
    # AMD exposes /sys/.../cpufreq/boost (0=disabled). At most one is
    # present per machine.
    no_turbo = _read_int("/sys/devices/system/cpu/intel_pstate/no_turbo")
    boost    = _read_int("/sys/devices/system/cpu/cpufreq/boost")
    turbo_disabled: bool | None
    turbo_source: str
    if no_turbo is not None:
        turbo_disabled = (no_turbo == 1)
        turbo_source   = "intel_pstate/no_turbo"
    elif boost is not None:
        turbo_disabled = (boost == 0)
        turbo_source   = "cpufreq/boost"
    else:
        turbo_disabled = None
        turbo_source   = "unknown (neither intel_pstate/no_turbo nor cpufreq/boost)"

    # Load average
    loadavg_txt = _read_text("/proc/loadavg") or "0 0 0"
    loadavg_1min = float(loadavg_txt.split()[0])

    # CPU MHz: parse /proc/cpuinfo "cpu MHz" lines
    cpu_mhz_values: list[float] = []
    cpuinfo = _read_text("/proc/cpuinfo") or ""
    for line in cpuinfo.splitlines():
        if line.startswith("cpu MHz"):
            try:
                cpu_mhz_values.append(float(line.split(":", 1)[1].strip()))
            except (ValueError, IndexError):
                pass

    # Nominal max from /sys/.../cpu0/cpufreq/cpuinfo_max_freq (kHz)
    max_khz = _read_int("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
    cpu_nominal_max_mhz = (max_khz / 1000.0) if max_khz else None

    # Other context: SMT / isolcpus / perf_event_paranoid / ASLR
    smt_control = _read_text("/sys/devices/system/cpu/smt/control")

    cmdline = _read_text("/proc/cmdline") or ""
    isolcpus = ""
    for tok in cmdline.split():
        if tok.startswith("isolcpus="):
            isolcpus = tok.split("=", 1)[1]
            break

    perf_event_paranoid = _read_int("/proc/sys/kernel/perf_event_paranoid")
    aslr                = _read_int("/proc/sys/kernel/randomize_va_space")
    nproc               = len(governors) or os.cpu_count() or 0

    return SystemEnvSnapshot(
        governors            = governors,
        turbo_disabled       = turbo_disabled,
        turbo_source         = turbo_source,
        loadavg_1min         = loadavg_1min,
        cpu_mhz_observed_min = min(cpu_mhz_values) if cpu_mhz_values else 0.0,
        cpu_mhz_observed_max = max(cpu_mhz_values) if cpu_mhz_values else 0.0,
        cpu_nominal_max_mhz  = cpu_nominal_max_mhz,
        smt_control          = smt_control,
        isolcpus             = isolcpus,
        perf_event_paranoid  = perf_event_paranoid,
        aslr                 = aslr,
        nproc                = nproc,
    )


# NOTE: previous versions of this script had `audit_system_env()` that
# hard-aborted on governor != performance / Turbo enabled / load > 0.5.
# That model targets ABSOLUTE number quality (paper-grade, cross-machine
# comparable) and is inappropriate for the shared-server, relative-only
# use case this script actually serves. Static env is now SNAPSHOTTED
# into the report (§6) and the only gate is post-run measurement
# consistency: see `audit_measurement_consistency()` below.


# ───────────────────────────────────────────────────────────────────
# Fair-build audits
# ───────────────────────────────────────────────────────────────────

def audit_rust_project(label: str, project_dir: Path,
                       harness_dir: Path | None) -> AuditSection:
    errors = fair_audit(project_dir, harness_dir, strict=False)
    return AuditSection(label=label, errors=list(errors))


def audit_c_reference(manifest_path: Path) -> AuditSection:
    errors = list(_audit_c_reference(manifest_path))
    # Extra check: thin LTO is weaker than Fat LTO. clang `-flto` defaults
    # to Fat/Full LTO, which pairs with Rust's `lto = "fat"`. If the C
    # build_cmd uses `-flto=thin`, the C side gets a weaker optimizer
    # (per-TU summary + parallel restricted), giving Rust an unfair
    # advantage on the cross-TU inlining axis. Reject before measurement.
    try:
        data = tomllib.loads(manifest_path.read_text())
        cmd = (data.get("c_reference") or {}).get("build_cmd", "")
        if re.search(r"(?<!\w)-flto=thin(?!\w)", cmd):
            errors.append(
                f"{manifest_path}: [c_reference].build_cmd uses "
                f"`-flto=thin` (ThinLTO) but Rust profile.release uses "
                f"`lto = \"fat\"` (Full LTO). C would get the weaker "
                f"optimizer and the comparison is unfair. Change to bare "
                f"`-flto` (which is Fat LTO in clang) — or to `-flto=full` "
                f"if you want to be explicit."
            )
    except (OSError, tomllib.TOMLDecodeError):
        pass  # _audit_c_reference already reported parse errors
    return AuditSection(label="c_reference.build_cmd", errors=errors)


def audit_measurement_consistency(
    measurements: list[Measurement],
) -> ConsistencyDiagnostic:
    """The single fairness gate left: across the 3 measurement windows,
    did the host stay consistent? Specifically — did the 1-min load avg
    swing more than LOAD_DRIFT_WARN_THRESHOLD? If yes, one version was
    measured under heavier system contention than the others, and the
    cross-version comparison is unfair (even though each version's own
    cv might still look clean — load contention affects the absolute
    median, which is what gets ratio'd).

    Returns a diagnostic; never raises. Caller is responsible for
    surfacing warnings (logger + report §6)."""
    # Collect every per-window load reading (before + after, per
    # measurement). Skip None readings (e.g. /proc/loadavg unreadable).
    readings: list[float] = []
    for m in measurements:
        for v in (m.load_1min_before, m.load_1min_after):
            if v is not None:
                readings.append(v)

    if len(readings) < 2:
        return ConsistencyDiagnostic(
            load_min        = None,
            load_max        = None,
            load_drift      = None,
            drift_threshold = LOAD_DRIFT_WARN_THRESHOLD,
            drift_ok        = True,  # unknown ⇒ don't false-alarm
            warnings        = ["/proc/loadavg unreadable — drift check skipped"],
        )

    lo, hi = min(readings), max(readings)
    drift = hi - lo
    warnings: list[str] = []
    drift_ok = drift < LOAD_DRIFT_WARN_THRESHOLD
    if not drift_ok:
        # Pinpoint which version saw the spike — most useful info
        # for "which version got the unfair treatment".
        per_version: list[tuple[str, float, float]] = []
        for m in measurements:
            if m.load_1min_before is not None and m.load_1min_after is not None:
                per_version.append(
                    (m.label, m.load_1min_before, m.load_1min_after)
                )
        detail = "; ".join(
            f"{lbl} {lb:.2f}→{la:.2f}" for lbl, lb, la in per_version
        )
        warnings.append(
            f"1-min load average drifted {drift:.2f} across the three "
            f"measurement windows (min={lo:.2f}, max={hi:.2f}, "
            f"threshold={LOAD_DRIFT_WARN_THRESHOLD:.1f}). One version "
            f"was measured under heavier contention than the others — "
            f"the relative comparison is unfair. Per-version load "
            f"(before→after): {detail}. Rerun when the host is more "
            f"stable."
        )

    return ConsistencyDiagnostic(
        load_min        = lo,
        load_max        = hi,
        load_drift      = drift,
        drift_threshold = LOAD_DRIFT_WARN_THRESHOLD,
        drift_ok        = drift_ok,
        warnings        = warnings,
    )


# ───────────────────────────────────────────────────────────────────
# Build phase
# ───────────────────────────────────────────────────────────────────

def _shell(cmd: str, *, cwd: Path | None = None, timeout_s: int = 1200) -> None:
    """Run a shell `build_cmd` line; raise with stderr tail on failure."""
    logger.info(f"[build] $ {cmd}  (cwd={cwd})")
    proc = subprocess.run(
        cmd, shell=True, cwd=str(cwd) if cwd else None,
        capture_output=True, text=True, timeout=timeout_s,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"shell command failed (exit {proc.returncode}):\n  $ {cmd}\n"
            f"cwd: {cwd}\n"
            f"--- stderr tail ---\n{(proc.stderr or '')[-2000:]}\n"
            f"--- stdout tail ---\n{(proc.stdout or '')[-1000:]}"
        )


def build_c_reference(manifest_data: dict) -> tuple[Path, str, Path | None, float]:
    """Run [c_reference].build_cmd. Returns (binary, build_cmd, source_dir, seconds).

    Raises if [c_reference] absent — three-way comparison cannot proceed
    without the C baseline.
    """
    cref = manifest_data.get("c_reference")
    if not isinstance(cref, dict):
        raise RuntimeError(
            "manifest has no [c_reference] section — three-way comparison "
            "needs a C baseline. Add [c_reference] with source_dir, "
            "build_cmd, and binary."
        )
    cmd = cref.get("build_cmd")
    bin_str = cref.get("binary")
    src = cref.get("source_dir")
    if not cmd or not bin_str:
        raise RuntimeError(
            "[c_reference] missing build_cmd or binary in manifest"
        )
    src_dir = Path(src).resolve() if src else None
    if src_dir is not None and not src_dir.is_dir():
        raise RuntimeError(f"[c_reference].source_dir does not exist: {src_dir}")
    binary = Path(bin_str)
    if not binary.is_absolute():
        if src_dir is None:
            raise RuntimeError(
                f"[c_reference].binary is relative ({bin_str!r}) but "
                "[c_reference].source_dir is missing — cannot resolve"
            )
        binary = src_dir / bin_str
    # Best-effort: remove a stale binary so we trust this build's product
    if binary.exists():
        try:
            binary.unlink()
        except OSError:
            pass
    t0 = time.perf_counter()
    _shell(cmd, cwd=src_dir)
    seconds = time.perf_counter() - t0
    if not binary.is_file():
        raise RuntimeError(
            f"[c_reference].build_cmd succeeded but binary not at {binary}"
        )
    return binary, cmd, src_dir, seconds


@dataclass
class StagedCrate:
    label: str
    src_project: Path
    src_harness: Path | None
    staged_crate: Path             # disposable copy of src_project (fair-build'd)
    staged_harness: Path | None    # disposable copy of src_harness (rewired)
    build_cwd: Path                # = staged_harness if any, else staged_crate


def stage_rust_project(label: str, project_dir: Path,
                       harness_dir: Path | None,
                       tmp_root: Path) -> StagedCrate:
    """Mirror the project (and its harness, if any) into the tempdir, top
    up fair-build config on the copies, rewire harness path-dep to the
    staged crate. Original inputs are NEVER modified — `feedback_no_lib
    _edits_for_oracle`."""
    work_root = tmp_root / label
    work_root.mkdir(parents=True, exist_ok=True)

    staged_crate = work_root / "crate"
    _rsync_copy(project_dir, staged_crate)
    _ensure_fair_build(staged_crate)

    staged_harness: Path | None = None
    if harness_dir is not None:
        staged_harness = work_root / "harness"
        _rsync_copy(harness_dir, staged_harness)
        _ensure_fair_build(staged_harness)
        _rewire_path_dep(staged_harness / "Cargo.toml", staged_crate)
        # Inject re-export shim modules for any `use lib::src::<X>::…`
        # path the harness expects but `staged_crate/lib.rs` doesn't
        # declare. Lets a harness written for a later stage (e.g.
        # `2_stage_a` with split modules `c_structs/c_types/…`) compile
        # against `0_raw` which only ships `mod libcsv`.
        _ensure_harness_compat(staged_crate, staged_harness)

    build_cwd = staged_harness or staged_crate
    return StagedCrate(
        label          = label,
        src_project    = project_dir,
        src_harness    = harness_dir,
        staged_crate   = staged_crate,
        staged_harness = staged_harness,
        build_cwd      = build_cwd,
    )


def cargo_build_staged(staged: StagedCrate) -> float:
    """`cargo build --release` against the staged crate (+ harness). Drops
    `RUSTFLAGS` from env so the staged `.cargo/config.toml` rustflags
    actually take effect; env RUSTFLAGS would silently override and
    invalidate the fair-build guarantee."""
    require_rustc_version(staged.build_cwd)
    env = {k: v for k, v in os.environ.items() if k != "RUSTFLAGS"}
    t0 = time.perf_counter()
    logger.info(f"[build] cargo build --release in {staged.build_cwd}")
    proc = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=str(staged.build_cwd), env=env,
        capture_output=True, text=True, timeout=900,
    )
    seconds = time.perf_counter() - t0
    if proc.returncode != 0:
        raise RuntimeError(
            f"cargo build failed in {staged.build_cwd}:\n"
            f"--- stderr tail ---\n{(proc.stderr or '')[-2000:]}"
        )
    return seconds


def resolve_built_binary(binary_dir: Path, binary_name: str) -> Path:
    p = binary_dir / "target" / "release" / binary_name
    if not p.is_file():
        raise RuntimeError(f"expected binary not built: {p}")
    return p


# ───────────────────────────────────────────────────────────────────
# W1 + W2
# ───────────────────────────────────────────────────────────────────

def _one_run(binary: Path, args: list[str], timeout_s: int,
             cwd: Path | None, *, pin_cpu: int | None = None) -> tuple[float, int]:
    """One timed invocation. Returns (elapsed_ms, exit_code).

    Wall-clock is measured by `time.perf_counter_ns()` around the
    `subprocess.run` call, in milliseconds. stdout/stderr are discarded
    (no oracle checks; this tool is performance-only).

    If `pin_cpu` is set, the binary is invoked via `taskset -c <N>`
    so it runs only on CPU N — eliminates cross-CPU cache cold-starts
    and reduces measurement noise. All three versions must be pinned to
    the SAME CPU for fairness; that's the caller's responsibility."""
    from profiling.workload import _resolve_rand
    args = _resolve_rand(args)
    cmd = [str(binary), *args]
    if pin_cpu is not None:
        cmd = ["taskset", "-c", str(pin_cpu), *cmd]
    t0 = time.perf_counter_ns()
    proc = subprocess.run(
        cmd,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        cwd=str(cwd) if cwd else None,
        timeout=timeout_s,
    )
    t1 = time.perf_counter_ns()
    return (t1 - t0) / 1_000_000.0, proc.returncode


def _read_loadavg_1min() -> float | None:
    """Read /proc/loadavg first field. Returns None on failure."""
    txt = _read_text("/proc/loadavg")
    if not txt:
        return None
    try:
        return float(txt.split()[0])
    except (ValueError, IndexError):
        return None


def measure_binary(label: str, binary: Path, args: list[str],
                   warmup: int, runs: int, timeout_s: int,
                   *, cwd: Path | None,
                   build_seconds: float,
                   pin_cpu: int | None = None) -> Measurement:
    """Warmup + measure. Apply trimmed-median (drop max + min when n >= 5)
    and population-stddev / mean as cv, matching `verify.measure_w2`.
    Non-zero-exit samples are skipped as basic sanity (a crashed binary's
    wall-clock is meaningless), but no stdout oracle is enforced.

    Captures `(started_at, load_1min_before)` immediately before warmup
    and `(ended_at, load_1min_after)` after the last measured sample.
    These windows feed `audit_measurement_consistency` to detect host
    drift across the three versions — the actual fairness gate."""
    logger.info(f"[measure] {label}: {warmup} warmup + {runs} measure runs")
    notes: list[str] = []
    started_at = datetime.now(timezone.utc)
    load_before = _read_loadavg_1min()

    for i in range(warmup):
        try:
            _one_run(binary, args, timeout_s, cwd, pin_cpu=pin_cpu)
        except subprocess.TimeoutExpired:
            notes.append(f"warmup {i+1} timed out (continuing)")

    samples: list[float] = []
    for i in range(runs):
        try:
            ms, rc = _one_run(binary, args, timeout_s, cwd, pin_cpu=pin_cpu)
        except subprocess.TimeoutExpired:
            notes.append(f"measure {i+1} timed out, skipped")
            continue
        if rc != 0:
            notes.append(f"measure {i+1} non-zero exit={rc}, skipped")
            continue
        samples.append(ms)

    ended_at = datetime.now(timezone.utc)
    load_after = _read_loadavg_1min()

    if len(samples) < 2:
        raise RuntimeError(
            f"{label}: only {len(samples)} usable samples — too unstable"
        )

    if len(samples) >= 5:
        core = sorted(samples)[1:-1]
    else:
        core = samples
    median = statistics.median(core)
    mean = statistics.fmean(core)
    cv = statistics.pstdev(core) / mean if mean > 0 else float("inf")
    try:
        binary_size = binary.stat().st_size
    except OSError:
        binary_size = None

    return Measurement(
        label             = label,
        binary            = binary,
        samples_ms        = samples,
        median_ms         = median,
        cv                = cv,
        min_ms            = min(samples),
        max_ms            = max(samples),
        n                 = len(samples),
        build_seconds     = build_seconds,
        binary_size_bytes = binary_size,
        started_at        = started_at,
        ended_at          = ended_at,
        load_1min_before  = load_before,
        load_1min_after   = load_after,
        notes             = notes,
    )


# ───────────────────────────────────────────────────────────────────
# Markdown rendering
# ───────────────────────────────────────────────────────────────────

def _fmt_pct(x: float) -> str:
    return f"{x:+.2f}%"


def render_markdown(report: Report) -> str:
    L: list[str] = []
    L.append(f"# Three-way performance report — {report.project_label}")
    L.append("")
    L.append(f"_Generated: {report.generated_at}_")
    L.append("")
    L.append("## 1. Inputs")
    L.append("")
    L.append("| field | value |")
    L.append("|---|---|")
    L.append(f"| workload manifest | `{report.manifest_path}` |")
    L.append(f"| workload name | `{report.workload_name}` |")
    L.append(f"| workload args | `{' '.join(report.workload_args)}` |")
    if report.workload_input:
        L.append(f"| workload input | `{report.workload_input}` |")
    L.append(f"| warmup / measure runs | {report.warmup_runs} / {report.measure_runs} |")
    pin_str = (f"`taskset -c {report.pin_cpu}`"
               if report.pin_cpu is not None
               else "(none — kernel scheduler picks CPU each run)")
    L.append(f"| CPU pinning | {pin_str} |")
    if report.raw_harness_dir is not None:
        L.append(f"| raw harness override | `{report.raw_harness_dir}` "
                 f"(c2rust_raw uses this; ours uses manifest's `harness_dir`) |")
    if report.ours_harness_dir is not None:
        L.append(f"| ours harness override | `{report.ours_harness_dir}` "
                 f"(ours uses this; c2rust_raw uses manifest's `harness_dir`) |")
    if report.c_project_dir:
        L.append(f"| C project (metadata) | `{report.c_project_dir}` |")
    L.append(f"| C source dir (build_cmd cwd) | "
             f"`{report.c_source_dir or '(none)'}` |")
    L.append(f"| c2rust raw project | `{report.c2rust_project}` |")
    L.append(f"| ours project | `{report.ours_project}` |")
    L.append("")
    L.append("## 2. Toolchain (hard-gated)")
    L.append("")
    t = report.toolchain
    L.append("| component | required | observed |")
    L.append("|---|---|---|")
    L.append(f"| C compiler | clang {t.clang_version} | `{t.clang_full_version}` ✓ |")
    L.append(f"| rustc | {t.rustc_minor}.x ({t.rust_toolchain}) | `{t.host_rustc}` ✓ |")
    L.append(f"| c2rust (translator) | {t.c2rust_version} | (pinned; not invoked here — only consumed via 0_raw) |")
    L.append("")
    L.append("## 3. Fair-build audit (§2.8)")
    L.append("")
    for a in report.audits:
        if a.ok:
            L.append(f"- **{a.label}** — ✓ pass")
        else:
            L.append(f"- **{a.label}** — ✗ FAIL ({len(a.errors)} error(s)):")
            for e in a.errors:
                L.append(f"    - {e}")
    L.append("")
    if report.c_build_cmd:
        L.append("**C reference build_cmd**:")
        L.append("")
        L.append(f"```\n{report.c_build_cmd}\n```")
        L.append("")
    L.append("## 4. Performance (wall-clock, trimmed median over n samples)")
    L.append("")
    L.append("**Metric**: per-run wall-clock measured by "
             "`time.perf_counter_ns()` around `subprocess.run` "
             "(milliseconds). Each version runs `warmup_runs` times "
             "first (discarded), then `measure_runs` timed samples. "
             "**Trimmed median** = drop the single max and single min "
             "sample, take median of the remaining; **cv** = "
             "population-stddev / mean over the same trimmed core, "
             "indicating measurement stability (lower = tighter).")
    L.append("")
    by_label = {m.label: m for m in report.measurements}
    c_med = by_label["C"].median_ms if "C" in by_label else None
    raw_med = by_label["c2rust_raw"].median_ms if "c2rust_raw" in by_label else None

    L.append("| version | median ms | cv | min | max | n | vs C | vs c2rust_raw |")
    L.append("|---|---:|---:|---:|---:|---:|---:|---:|")
    for m in report.measurements:
        vs_c = _fmt_pct((m.median_ms - c_med) / c_med * 100) if c_med else "—"
        vs_raw = _fmt_pct((m.median_ms - raw_med) / raw_med * 100) if raw_med else "—"
        L.append(
            f"| {m.label} | {m.median_ms:.2f} | {m.cv*100:.3f}% | "
            f"{m.min_ms:.2f} | {m.max_ms:.2f} | {m.n} | {vs_c} | {vs_raw} |"
        )
    L.append("")
    L.append("Interpretation:")
    L.append("- **vs C** — `+x%` means slower than C by x%; `0` = same as C.")
    L.append("- **vs c2rust_raw** — `+x%` means slower than raw; "
             "negative = perf-opt improved over raw (= our lift gain).")
    if c_med and raw_med and "ours" in by_label:
        ours_med = by_label["ours"].median_ms
        lift_vs_raw = (raw_med - ours_med) / raw_med * 100.0
        gap_to_c = (ours_med - c_med) / c_med * 100.0
        L.append("")
        L.append("**Headline numbers**:")
        L.append(f"- perf-opt lift over c2rust_raw: **{lift_vs_raw:+.2f}%** "
                 f"(positive = faster than raw)")
        L.append(f"- ours vs C gap: **{gap_to_c:+.2f}%** "
                 f"(closer to 0 = closer to C)")
        # Only report "translation cost recovered" when the C↔raw gap is
        # > 1% of C — otherwise (raw − C) is within measurement noise
        # and the ratio is meaningless (we hit -159137% in early runs
        # where raw − C = 0.03 ms).
        raw_overhead_pct = (raw_med - c_med) / c_med * 100.0
        if raw_overhead_pct > 1.0:
            closed = ((raw_med - ours_med) / (raw_med - c_med)) * 100.0
            L.append(f"- c2rust translation cost recovered: **{closed:.1f}%** "
                     f"of (raw − C, which is {raw_overhead_pct:+.2f}%)")
        else:
            L.append(f"- c2rust translation cost recovered: _n/a_ — "
                     f"c2rust_raw is only {raw_overhead_pct:+.2f}% slower "
                     f"than C, within measurement noise (no meaningful gap to recover)")
    L.append("")
    L.append("## 5. Raw samples")
    L.append("")
    for m in report.measurements:
        L.append(f"### {m.label}")
        L.append("")
        L.append(f"- binary: `{m.binary}`")
        if m.binary_size_bytes is not None:
            size_kb = m.binary_size_bytes / 1024.0
            L.append(f"- binary size: {m.binary_size_bytes:,} bytes "
                     f"({size_kb:.1f} KiB)")
        L.append(f"- build time: {m.build_seconds:.2f}s")
        if m.notes:
            L.append(f"- notes: {'; '.join(m.notes)}")
        L.append(f"- samples (ms): "
                 f"{', '.join(f'{x:.2f}' for x in m.samples_ms)}")
        L.append("")

    # ───── §6 Measurement environment & consistency ─────
    snap = report.env_snapshot
    cons = report.consistency
    L.append("## 6. Measurement environment & consistency")
    L.append("")
    L.append("This script targets **relative** fairness on a shared host: "
             "it does not require `governor=performance` or an idle "
             "machine. Instead, after the three versions are measured, "
             "it checks that the host stayed **consistent** across all "
             "three measurement windows — if load drifted, the relative "
             "comparison is unfair regardless of how clean each version's "
             "individual cv looks.")
    L.append("")

    # ── §6.1 static snapshot ──
    L.append("### 6.1 Static environment snapshot (recorded, not gated)")
    L.append("")
    L.append("| variable | value |")
    L.append("|---|---|")
    govs_uniq = sorted(set(snap.governors))
    govs_str = govs_uniq[0] if len(govs_uniq) == 1 else f"mixed: {govs_uniq}"
    L.append(f"| CPU governor (×{snap.nproc} CPUs) | `{govs_str}` |")
    if snap.turbo_disabled is True:
        turbo_val = "disabled"
    elif snap.turbo_disabled is False:
        turbo_val = "ENABLED"
    else:
        turbo_val = "unknown"
    L.append(f"| Turbo / Boost (via {snap.turbo_source}) | `{turbo_val}` |")
    L.append(f"| Observed CPU MHz (min/max across cores) | "
             f"`{snap.cpu_mhz_observed_min:.0f} / "
             f"{snap.cpu_mhz_observed_max:.0f}` MHz |")
    nominal = (f"{snap.cpu_nominal_max_mhz:.0f} MHz"
               if snap.cpu_nominal_max_mhz else "(unknown)")
    L.append(f"| Nominal CPU max | `{nominal}` |")
    L.append(f"| SMT / Hyper-Threading | `{snap.smt_control or '(unknown)'}` |")
    L.append(f"| isolcpus (boot param) | `{snap.isolcpus or '(none)'}` |")
    L.append(f"| perf_event_paranoid | "
             f"`{snap.perf_event_paranoid if snap.perf_event_paranoid is not None else '(unknown)'}` |")
    L.append(f"| ASLR (randomize_va_space) | "
             f"`{snap.aslr if snap.aslr is not None else '(unknown)'}` |")
    L.append("")
    L.append("These are reported for honest reproducibility — readers "
             "who want absolute-quality numbers (cross-machine, "
             "cross-paper) should rerun under `governor=performance` "
             "+ Turbo disabled + isolated cores. For the relative-only "
             "claims in this report, the static snapshot is informational.")
    L.append("")

    # ── §6.2 per-window timeline ──
    L.append("### 6.2 Per-version measurement windows")
    L.append("")
    L.append("| version | started (UTC) | duration | load 1-min (before → after) |")
    L.append("|---|---|---|---|")
    for m in report.measurements:
        start = (m.started_at.strftime("%H:%M:%S")
                 if m.started_at else "—")
        if m.started_at and m.ended_at:
            dur = (m.ended_at - m.started_at).total_seconds()
            dur_str = f"{dur:.1f}s"
        else:
            dur_str = "—"
        lb = f"{m.load_1min_before:.2f}" if m.load_1min_before is not None else "—"
        la = f"{m.load_1min_after:.2f}"  if m.load_1min_after  is not None else "—"
        L.append(f"| {m.label} | {start} | {dur_str} | {lb} → {la} |")
    L.append("")

    # ── §6.3 consistency verdict ──
    L.append("### 6.3 Cross-version consistency check (the fairness gate)")
    L.append("")
    if cons.load_drift is None:
        L.append("- **Status**: ⚠ load drift could not be computed "
                 "(/proc/loadavg unreadable). Treat the comparison as "
                 "unverified.")
    elif cons.drift_ok:
        L.append(f"- **Status**: ✓ **fair** — load drift "
                 f"`{cons.load_drift:.2f}` is below threshold "
                 f"`{cons.drift_threshold:.1f}`. The three versions saw "
                 f"a comparable host state; relative numbers are trustworthy.")
        L.append(f"  - load 1-min: min `{cons.load_min:.2f}`, "
                 f"max `{cons.load_max:.2f}` across all window endpoints")
    else:
        L.append(f"- **Status**: ✗ **UNFAIR** — load drift "
                 f"`{cons.load_drift:.2f}` exceeds threshold "
                 f"`{cons.drift_threshold:.1f}`. One version was "
                 f"measured under heavier host contention than the others; "
                 f"the relative comparison in §4 is not trustworthy. "
                 f"Rerun when the host is more stable.")
        L.append(f"  - load 1-min: min `{cons.load_min:.2f}`, "
                 f"max `{cons.load_max:.2f}` across all window endpoints")
        for w in cons.warnings:
            L.append(f"  - {w}")
    L.append("")

    return "\n".join(L) + "\n"


# ───────────────────────────────────────────────────────────────────
# Main driver
# ───────────────────────────────────────────────────────────────────

def run_report(c2rust_project: Path, ours_project: Path,
               manifest_path: Path, out_path: Path,
               *, c_project: Path | None,
               project_label: str,
               keep_build_dir: bool,
               pin_cpu: int | None = None,
               raw_harness_dir: Path | None = None,
               ours_harness_dir: Path | None = None) -> Report:
    workload = load_manifest(manifest_path)
    manifest_data = tomllib.loads(manifest_path.read_text(encoding="utf-8"))

    # ───── host clang gate (rustc gate runs later on the staged copies,
    #       because _ensure_fair_build writes rust-toolchain.toml there) ─
    clang_path, clang_full = audit_host_clang()

    # ───── stage two disposable copies, auto-fair-build them ─────
    measurements: list[Measurement] = []
    tmp_root = Path(tempfile.mkdtemp(prefix="perf_compare_"))
    logger.info(f"[setup] staging dir: {tmp_root}")
    try:
        logger.info("[stage] copying c2rust_raw + harness, applying fair-build")
        # raw uses --raw-harness-dir if provided (lib projects whose
        # harness API drifted after Stage A E3 callback monomorphize
        # need a separate fn-ptr harness to compile against c2rust raw);
        # falls back to the manifest's harness_dir otherwise (binary
        # projects, or libs whose harness still uses fn-ptr).
        raw_harness = raw_harness_dir or workload.harness_dir
        if raw_harness_dir is not None:
            logger.info(f"[stage] c2rust_raw using --raw-harness-dir: "
                        f"{raw_harness_dir}")
        raw_staged = stage_rust_project(
            "c2rust_raw", c2rust_project, raw_harness, tmp_root,
        )
        logger.info("[stage] copying ours + harness, applying fair-build")
        # ours uses --ours-harness-dir if provided (Stage A's E3 callback
        # monomorphization REWRITES the harness's `Some(fn)` callsites
        # into `move |…| unsafe { fn(…) }` closure bridges to match the
        # lifted `impl Fn` signature; the bridged version lives under
        # `2_stage_a/harness/` and MUST be used to build against the
        # lifted lib). Falls back to the manifest's harness_dir for
        # projects whose Stage A did not change the lib's public sig.
        ours_harness = ours_harness_dir or workload.harness_dir
        if ours_harness_dir is not None:
            logger.info(f"[stage] ours using --ours-harness-dir: "
                        f"{ours_harness_dir}")
        ours_staged = stage_rust_project(
            "ours", ours_project, ours_harness, tmp_root,
        )

        # ───── rustc gate (on staged copies; their toolchain.toml was
        #       written by _ensure_fair_build) ─────
        host_rustc = audit_staged_rustc([
            raw_staged.build_cwd, ours_staged.build_cwd,
        ])
        toolchain = ToolchainInfo(
            clang_bin          = str(clang_path),
            clang_version      = required_c_compiler_version(),
            clang_full_version = clang_full,
            rustc_minor        = required_rustc_minor(),
            rust_toolchain     = required_rust_toolchain(),
            c2rust_version     = required_c2rust_version(),
            host_rustc         = host_rustc,
        )

        # ───── snapshot static system env (no abort — recorded for the
        # report so readers know what host state this run saw). The
        # only fairness gate is post-run measurement-consistency: see
        # audit_measurement_consistency() after the three measurements. ─
        env_snapshot = read_system_env()
        if env_snapshot.governors and set(env_snapshot.governors) != {"performance"}:
            logger.info(
                f"[env] CPU governor = {sorted(set(env_snapshot.governors))} "
                f"(not 'performance') — OK for relative comparison on shared "
                f"hosts, just be aware the absolute ms numbers reflect that."
            )

        # ───── audit the STAGED copies (not source) ─────
        audits: list[AuditSection] = []
        audits.append(audit_rust_project(
            "c2rust_raw (staged)", raw_staged.staged_crate,
            raw_staged.staged_harness,
        ))
        audits.append(audit_rust_project(
            "ours (staged)", ours_staged.staged_crate,
            ours_staged.staged_harness,
        ))
        audits.append(audit_c_reference(manifest_path))
        failed = [a for a in audits if not a.ok]
        if failed:
            for a in failed:
                for e in a.errors:
                    logger.error(f"[audit:{a.label}] {e}")
            raise RuntimeError(
                "fair-build audit FAILED on the staged copies — this means "
                "_ensure_fair_build couldn't normalize a config (likely a "
                "weird upstream Cargo.toml). Fix the errors above; the "
                "originals are untouched."
            )

        # ───── build all three ─────
        logger.info("[1/3] building C reference")
        c_binary, c_cmd, c_src, c_secs = build_c_reference(manifest_data)
        m_c = measure_binary(
            "C", c_binary, workload.args,
            warmup=workload.warmup_runs, runs=workload.measure_runs,
            timeout_s=workload.timeout_s, cwd=None,
            build_seconds=c_secs, pin_cpu=pin_cpu,
        )
        measurements.append(m_c)

        logger.info("[2/3] building c2rust_raw")
        raw_secs = cargo_build_staged(raw_staged)
        raw_binary = resolve_built_binary(raw_staged.build_cwd, workload.binary)
        m_raw = measure_binary(
            "c2rust_raw", raw_binary, workload.args,
            warmup=workload.warmup_runs, runs=workload.measure_runs,
            timeout_s=workload.timeout_s, cwd=None,
            build_seconds=raw_secs, pin_cpu=pin_cpu,
        )
        measurements.append(m_raw)

        logger.info("[3/3] building ours")
        ours_secs = cargo_build_staged(ours_staged)
        ours_binary = resolve_built_binary(ours_staged.build_cwd, workload.binary)
        m_ours = measure_binary(
            "ours", ours_binary, workload.args,
            warmup=workload.warmup_runs, runs=workload.measure_runs,
            timeout_s=workload.timeout_s, cwd=None,
            build_seconds=ours_secs, pin_cpu=pin_cpu,
        )
        measurements.append(m_ours)

        # ───── post-run consistency gate: did the host stay
        #       consistent across the three windows? ─────
        consistency = audit_measurement_consistency(measurements)
        for w in consistency.warnings:
            logger.warning(f"[consistency] {w}")
    finally:
        if keep_build_dir:
            logger.info(f"[cleanup] kept staging dir: {tmp_root}")
        else:
            shutil.rmtree(tmp_root, ignore_errors=True)

    report = Report(
        project_label   = project_label,
        generated_at    = datetime.now(timezone.utc).strftime(
            "%Y-%m-%d %H:%M:%S UTC"
        ),
        manifest_path   = manifest_path,
        workload_name   = workload.name,
        workload_args   = list(workload.args),
        workload_input  = workload.input_path,
        warmup_runs     = workload.warmup_runs,
        measure_runs    = workload.measure_runs,
        toolchain       = toolchain,
        audits          = audits,
        env_snapshot    = env_snapshot,
        consistency     = consistency,
        c_build_cmd     = c_cmd,
        c_source_dir    = c_src,
        c_project_dir   = c_project,
        c2rust_project  = c2rust_project,
        ours_project    = ours_project,
        measurements    = measurements,
        pin_cpu         = pin_cpu,
        raw_harness_dir = raw_harness_dir,
        ours_harness_dir = ours_harness_dir,
    )
    md = render_markdown(report)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(md, encoding="utf-8")
    logger.info(f"[done] report -> {out_path}")
    return report


def _cli() -> int:
    p = argparse.ArgumentParser(
        prog="perf_compare",
        description="Release-quality C / c2rust_raw / our-lift perf "
                    "comparison. Hard-gates host env (CPU governor, "
                    "Turbo, load average) + toolchain + fair-build "
                    "config before measuring.",
    )
    p.add_argument("--c2rust-project", required=True, type=Path,
                   help="Path to the c2rust raw Rust project (e.g. .../0_raw)")
    p.add_argument("--ours-project", required=True, type=Path,
                   help="Path to the our-lift Rust project. "
                        "Pipeline final output: .../<proj>/2_stage_b/ "
                        "(Stage A + Stage B cumulative); for Stage-A-only "
                        "evaluation pass .../<proj>/2_stage_a/.")
    p.add_argument("--manifest", required=True, type=Path,
                   help="Workload manifest TOML (must include [c_reference])")
    p.add_argument("--out", default=None, type=Path,
                   help=f"Output Markdown report path. "
                        f"Default: {_DEFAULT_REPORT_DIR}/<label>_perf_compare.md "
                        f"where <label> defaults to the ours-project parent dir.")
    p.add_argument("--c-project", default=None, type=Path,
                   help="Optional C source-tree path for report metadata only "
                        "(C build is driven by [c_reference].build_cmd)")
    p.add_argument("--label", default=None,
                   help="Project label used in report title; "
                        "defaults to the ours-project dirname")
    p.add_argument("--keep-build-dir", action="store_true",
                   help="Keep the tempdir with staged harness copies "
                        "(default: delete on exit)")
    p.add_argument("--pin-cpu", type=int, default=None, metavar="N",
                   help="Run every measurement via `taskset -c N` to pin "
                        "all three versions to the same physical CPU. "
                        "Eliminates cross-CPU cache cold-starts and "
                        "shrinks cv (typically 0.6%% → 0.1%%). Pick a "
                        "CPU index nobody else is hammering (see `htop`).")
    p.add_argument("--raw-harness-dir", default=None, type=Path,
                   help="Override harness for the c2rust_raw measurement. "
                        "Needed for lib projects whose harness API drifted "
                        "after Stage A (e.g. libcsv: 2_stage_a uses `impl "
                        "Fn` callbacks while 0_raw still has `Option<fn-ptr>`; "
                        "one harness can't compile against both). Provide a "
                        "fn-ptr-interface harness here for raw; ours keeps "
                        "using manifest's [workload].harness_dir.")
    p.add_argument("--ours-harness-dir", default=None, type=Path,
                   help="Override harness for the `ours` measurement. "
                        "Needed when Stage A's E3 callback monomorphization "
                        "rewrote the harness's `Some(fn)` callsites into "
                        "`move |…| unsafe { fn(…) }` closure bridges to "
                        "match the lifted `impl Fn` lib signature — the "
                        "bridged harness lives under `2_stage_a/harness/` "
                        "and MUST be used against the lifted lib. Default: "
                        "manifest's [workload].harness_dir (a fresh copy).")
    p.add_argument("--verbose", "-v", action="store_true")
    args = p.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
        datefmt="%H:%M:%S",
    )

    c2rust_project = args.c2rust_project.resolve()
    ours_project   = args.ours_project.resolve()
    manifest       = args.manifest.resolve()
    c_project      = args.c_project.resolve() if args.c_project else None
    if not c2rust_project.is_dir():
        logger.error(f"--c2rust-project not a directory: {c2rust_project}")
        return 1
    if not ours_project.is_dir():
        logger.error(f"--ours-project not a directory: {ours_project}")
        return 1
    if not manifest.is_file():
        logger.error(f"--manifest not a file: {manifest}")
        return 1
    project_label = args.label or ours_project.parent.name
    out_path = (args.out.resolve() if args.out
                else _DEFAULT_REPORT_DIR / f"{project_label}_perf_compare.md")

    try:
        raw_harness_dir = (args.raw_harness_dir.resolve()
                           if args.raw_harness_dir else None)
        if raw_harness_dir is not None and not raw_harness_dir.is_dir():
            logger.error(f"--raw-harness-dir not a directory: {raw_harness_dir}")
            return 1
        ours_harness_dir = (args.ours_harness_dir.resolve()
                            if args.ours_harness_dir else None)
        if ours_harness_dir is not None and not ours_harness_dir.is_dir():
            logger.error(f"--ours-harness-dir not a directory: {ours_harness_dir}")
            return 1
        run_report(
            c2rust_project, ours_project, manifest, out_path,
            c_project=c_project,
            project_label=project_label,
            keep_build_dir=args.keep_build_dir,
            pin_cpu=args.pin_cpu,
            raw_harness_dir=raw_harness_dir,
            ours_harness_dir=ours_harness_dir,
        )
    except RuntimeError as e:
        logger.error(str(e))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(_cli())
