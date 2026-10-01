"""Stage A support helpers for the unified script_c2rust pipeline.

This module used to expose a standalone experiment CLI. The public entrypoint
is now `script_c2rust/main.py`, which names stages explicitly:

  stage_c2rust -> stage_cleanup -> stage_opt_a -> stage_opt_b

The helpers here are still used by stage_opt_a for fair-build setup,
harness path-dep rewiring, duplicate-type unification, and SA report
generation. Do not add a second CLI here; keep orchestration in main.py.
"""

from __future__ import annotations

import logging
import os
import re
import shutil
import subprocess
import time
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

from perf_opt.stage_a.runner import run_stage_a
from stages.cleanup.cargo_utils import read_crate_name
from stages.cleanup.pre_stage_a import prep_for_stage_a
from stages.cleanup.scan import scan_lib_public_fns
from utils.toolchain import (
    require_c_compiler_version,
    require_rustc_version,
    write_rust_toolchain,
)
# crate_ops primitives moved to perf_opt/verify/crate_ops.py (verify is now the
# independent foundation — bench_pipeline depends DOWN on it, not the reverse).
# Re-exported here so existing `from perf_opt.bench_pipeline import _rsync_copy`
# callers (main.py, stage_a lift drivers, …) keep working unchanged.
from perf_opt.verify.crate_ops import (  # noqa: E402,F401
    _CARGO_CONFIG, _HARNESS_USE_RE, _PATH_DEP_RE, _PROFILE_BLOCK,
    _PROFILE_KEYS_CANON, _PROFILE_RELEASE_BLOCK_RE, _ensure_fair_build,
    _ensure_harness_compat, _git_init, _rewire_path_dep, _rsync_copy,
)

logger = logging.getLogger(__name__)

TMP_ROOT = Path("/tmp/perfopt_runs")
STAGE_CHOICES = ("a", "b", "ab")  # legacy _full_run only; no CLI uses this.


def _default_model() -> str:
    """Single source of truth for the LLM model id used by Stage A's E2 v3
    salvage and Stage B's R-schema loop. Reads `DEFAULT_LLM_MODEL` from
    `Config/paths.conf` — change the model there, every LLM call site
    picks it up. We deliberately do NOT expose a `--model` CLI flag here:
    cross-pass model parity is required for ablation, and adding one knob
    per pass led to drift (see refactor 2026-05-24)."""
    from Config.paths import get_path
    m = get_path("DEFAULT_LLM_MODEL")
    if not m:
        raise RuntimeError(
            "DEFAULT_LLM_MODEL missing from Config/paths.conf — set it there"
        )
    return m

# ─────────────────────────────────────────────────────────────────
# Fair-build setup (canonical Cargo.toml profile + .cargo/config.toml).
# Applied idempotently so a fresh `rsync 0_raw` becomes valid in one call.
# ─────────────────────────────────────────────────────────────────





# ─────────────────────────────────────────────────────────────────
# Disposable working dirs from 0_raw + optional harness.
# ─────────────────────────────────────────────────────────────────

@dataclass
class CrateLayout:
    crate_dir: Path                 # ours-* / raw-* lib+bin crate dir
    harness_dir: Path | None        # optional — driving harness, if lib-only
    binary_path: Path               # target/release/<binary>








# ─────────────────────────────────────────────────────────────────
# Harness extern-"C"-block normalization
#
# c2rust translates harness "consumer" code with `extern "C" { fn foo(...);
# ... }` blocks. If Stage A then strips `extern "C"` from the lib's foo,
# the harness extern block becomes a stale linker contract → undefined
# reference. We pre-normalize the harness: every fn declared in an
# `extern "C" {}` block that is also a public lib function gets rewritten
# to a `use <dep>::src::<module>::<fn>;` import, removing the FFI
# contract upfront.
#
# This is GENERIC (no per-project branches): scans the harness's
# `extern "C" {}` blocks, looks up each declared name in the lib's
# `pub (unsafe) extern "C" fn …` set, and rewires those declarations.
# Unknown names (libc, system) stay in the block.
# ─────────────────────────────────────────────────────────────────



def _ensure_bin_target_for_main(crate_dir: Path,
                                 workload_binary: str | None) -> None:
    """Ensure Cargo.toml declares a [[bin]] target for the workload binary.

    Some c2rust outputs (json_h, libtree) embed `pub fn main()` inside a
    lib-internal module (`src/<driver>.rs`) without declaring [[bin]] in
    Cargo.toml. As a result `cargo build` produces only a .rlib — there
    is no binary for the workload to measure, and `baseline_w2_ms` ends
    up None, which then crashes the Stage A runner.

    Fix: if Cargo.toml has no [[bin]] and we find a `pub fn main()` in
    a lib-internal module, generate `src/bin/<workload_binary>.rs` as a
    thin wrapper calling that inner main via the lib crate's absolute
    path, and append a [[bin]] entry pointing to the wrapper.

    Why a wrapper (not just adding [[bin]] for the driver.rs)?
    Adding [[bin]] path=src/<driver>.rs makes the driver a bin crate
    root. Then its `use crate::src::ffi::*` lookups (relative path)
    fail because the bin's `crate::` no longer points to the lib mod
    tree. A separate wrapper file keeps the driver as a lib-internal
    module (so its relative imports work) and the wrapper uses an
    absolute path `::<crate>::<mod>` to reach the inner main. This is
    the same trick lil/main.rs uses (see lil/0_raw/src/main.rs:10
    `use ::lil_raw;`).

    No-op if:
      * Cargo.toml already has a [[bin]] entry (c2rust did it itself, e.g. lil)
      * No `pub fn main()` found in any src/**/*.rs
      * Cargo.toml not found / malformed

    Generated wrapper path: src/bin/<workload_binary>.rs.
    """
    cargo = crate_dir / "Cargo.toml"
    if not cargo.is_file():
        return
    text = cargo.read_text(encoding="utf-8")

    # Already has a [[bin]] entry? Done.
    if re.search(r"^\s*\[\[bin\]\]", text, re.MULTILINE):
        return

    if not workload_binary:
        logger.warning(
            "[harness-compat] no [[bin]] in Cargo.toml and workload.binary "
            "is unset — cannot auto-generate bin wrapper"
        )
        return

    # Extract lib crate name (used in `use ::<crate>::...`).
    m_name = re.search(
        r'\[lib\][^\[]*?\bname\s*=\s*"([^"]+)"',
        text,
        re.DOTALL,
    )
    if not m_name:
        # Fall back to package.name
        m_name = re.search(
            r'\[package\][^\[]*?\bname\s*=\s*"([^"]+)"',
            text,
            re.DOTALL,
        )
    if not m_name:
        logger.warning(
            "[harness-compat] cannot find crate name in Cargo.toml — "
            "skipping bin wrapper generation"
        )
        return
    crate_name = m_name.group(1)

    # Find the .rs file containing `pub fn main()` and its lib mod path.
    # The mod path is derived from path relative to src/, e.g.
    # `src/json_h_driver.rs` → `src::json_h_driver`,
    # `src/foo/bar.rs` → `src::foo::bar`.
    src_dir = crate_dir / "src"
    if not src_dir.is_dir():
        return
    main_re = re.compile(r"^\s*pub fn main\s*\(", re.MULTILINE)
    inner: tuple[Path, str] | None = None  # (file, mod_path)
    for rs in sorted(src_dir.rglob("*.rs")):
        # Don't pick wrappers we ourselves may have generated previously.
        if rs.relative_to(src_dir).parts[:1] == ("bin",):
            continue
        try:
            txt = rs.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        if main_re.search(txt):
            rel = rs.relative_to(src_dir)
            mod_segs = list(rel.with_suffix("").parts)
            mod_path = "::".join(["src"] + mod_segs)
            inner = (rs, mod_path)
            break
    if inner is None:
        return

    inner_file, mod_path = inner

    # Write wrapper. Use a NEUTRAL stem ("_w2_main") so analyzer's
    # candidate-scan exclude-bin-stems list doesn't accidentally exclude
    # the c2rust driver .rs that shares its name with workload_binary
    # (e.g. json_h: workload binary "json_h_driver" matches c2rust file
    # "src/json_h_driver.rs"). Cargo allows [[bin]].name != path stem.
    bin_dir = crate_dir / "src" / "bin"
    bin_dir.mkdir(parents=True, exist_ok=True)
    wrapper_file = bin_dir / "_w2_main.rs"
    wrapper_text = (
        f"// Auto-generated by bench_pipeline._ensure_bin_target_for_main.\n"
        f"// c2rust placed `pub fn main()` in `{mod_path}` without\n"
        f"// declaring [[bin]] in Cargo.toml. This wrapper forwards to it.\n"
        f"fn main() {{\n"
        f"    {crate_name}::{mod_path}::main();\n"
        f"}}\n"
    )
    wrapper_file.write_text(wrapper_text, encoding="utf-8")

    # Append [[bin]] entry to Cargo.toml. `name` is the workload binary
    # (so cargo produces target/release/<workload_binary>); `path` points
    # to the neutral-stem wrapper to avoid analyzer exclude collision.
    new_text = text.rstrip() + (
        f"\n\n[[bin]]\n"
        f'name = "{workload_binary}"\n'
        f'path = "src/bin/_w2_main.rs"\n'
    )
    cargo.write_text(new_text, encoding="utf-8")
    logger.info(
        f"[harness-compat] generated bin wrapper "
        f"src/bin/_w2_main.rs (name={workload_binary}) → "
        f"{crate_name}::{mod_path}::main "
        f"(c2rust packed main() into lib mod without [[bin]])"
    )






# ─────────────────────────────────────────────────────────────────
# Build + W2 measurement
# ─────────────────────────────────────────────────────────────────

@dataclass
class Measurement:
    label: str
    binary: Path
    task_clock_ms: float
    instructions: int
    l1_loads: int
    cv_pct: float


def _cargo_build(crate_dir: Path, *, timeout_s: int = 600) -> None:
    require_rustc_version(crate_dir)
    proc = subprocess.run(
        ["cargo", "build", "--release"], cwd=str(crate_dir),
        capture_output=True, text=True, timeout=timeout_s,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"cargo build failed in {crate_dir}:\n{(proc.stderr or '')[-2000:]}"
        )


def _perf_stat(binary: Path, args: list[str], workload_input: Path,
                *, repeats: int = 30, warmup: int = 3,
                label: str = "",
                pin_cpu: int | None = None) -> Measurement:
    ""                                       

                                                                   
                                                                  
                                                                  
                                                                  

                                                                        
                                                                             
                                                                
                                                               
                                           

                                                                
                                                                     
               
                                                                    
                                                         

                                                                   
                                                                     
       
    import csv as _csv
    import io as _io
    import os
    cmd_args = [
        str(workload_input) if a == "$INPUT" else a for a in args
    ]

    # Resolve CPU pin.
    # 2026-06-18: default pin moved 2 → 16 because the host now isolates
    # CPUs 16,34 via `isolcpus=16,34 nohz_full=16,34 rcu_nocbs=16,34`
    # at the kernel cmdline. CPU 16 and its SMT sibling 34 are off-limits
    # to the regular scheduler, so workload measurements there avoid:
    #   * SMT cache contention with concurrently-running user processes
    #     (the prior pin=2 hit this — sibling CPU 20 was hosting Claude),
    #   * scheduler-tick noise (nohz_full disables 1 kHz timer on 16,34),
    #   * RCU callback storms (rcu_nocbs offloads RCU to other CPUs).
    # Verify with: cat /sys/devices/system/cpu/isolated → "16,34".
    # On a host without isolcpus configured, set PERF_STAT_PIN_CPU=-1
    # to disable pinning; otherwise pinning to non-isolated CPU 16 has
    # no benefit (and may regress if some other process happens to
    # share that core).
    if pin_cpu is None:
        env_cpu = os.environ.get("PERF_STAT_PIN_CPU")
        if env_cpu is not None:
            try:
                pin_cpu = int(env_cpu)
            except ValueError:
                pin_cpu = 16
        else:
            pin_cpu = 16
    pin_prefix: list[str] = []
    if pin_cpu is not None and pin_cpu >= 0:
        pin_prefix = ["taskset", "-c", str(pin_cpu)]

    for _ in range(warmup):
        subprocess.run([*pin_prefix, str(binary), *cmd_args],
                       stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL)
    # Use `perf stat -o <file>` so perf output (CSV via -x) goes to its
    # own file, not stderr. Binary stdout/stderr both redirect to
    # DEVNULL — without this split, benches that print progress to
    # stderr (optipng "Trying: zc = 9...") mangle the CSV stream
    # ("new-line character seen in unquoted field").
    import tempfile as _tempfile, os as _os
    perf_outfile = _tempfile.mktemp(suffix=".perfstat")
    try:
        subprocess.run(
            [*pin_prefix,
             "perf", "stat", "-x", ",", "-r", str(repeats),
             "-o", perf_outfile,
             "-e",
             "task-clock,instructions,cycles,L1-dcache-loads,"
             "L1-dcache-load-misses,LLC-load-misses,branches,branch-misses",
             str(binary), *cmd_args],
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        with open(perf_outfile, encoding="utf-8", errors="replace") as _f:
            out = _f.read()
    finally:
        try:
            _os.unlink(perf_outfile)
        except FileNotFoundError:
            pass
    extra: dict[str, float] = {}
    tc = insn = l1 = None
    cv = 0.0
    for row in _csv.reader(_io.StringIO(out)):
        if len(row) < 3:
            continue
        event = row[2].strip()
        try:
            val = float(row[0].replace(",", ""))
        except (ValueError, IndexError):
            continue
        if event == "task-clock":
            tc = val
            if len(row) > 3 and row[3].strip():
                try:
                    cv = float(row[3].strip().rstrip("%"))
                except ValueError:
                    pass
        elif event == "instructions":
            insn = int(val)
        elif event == "L1-dcache-loads":
            l1 = int(val)
        else:
            # cycles, L1-dcache-load-misses, LLC-load-misses, branch-misses
            extra[event] = val
    if tc is None:
        raise RuntimeError(f"could not parse perf stat output: {out[-500:]}")
    m = Measurement(
        label=label, binary=binary, task_clock_ms=tc,
        instructions=insn or 0, l1_loads=l1 or 0, cv_pct=cv,
    )
    # Attach extra counters via dict-like setattr (Measurement is a
    # dataclass; we add an `extra` attr if present without breaking
    # existing callers).
    try:
        m.__dict__["extra"] = extra
    except Exception:
        pass
    return m


# ─────────────────────────────────────────────────────────────────
# Pipeline orchestration
# ─────────────────────────────────────────────────────────────────

def _prepare_working_copy(project_dir: Path, label: str,
                           workload: dict, ts: str,
                           enable_unify: bool = True) -> CrateLayout:
    """`label`:  "raw" for the c2rust_raw baseline (no pipeline edits),
                "ours" for the pipeline-edited copy.

    `enable_unify`:  if True (default), runs the full pre-Stage-A cleanup
                pipeline (see `stages.cleanup.pre_stage_a.prep_for_stage_a`).
                MUST be False for the "raw" label — c2rust_raw is supposed
                to be the verbatim c2rust output for fair-baseline
                comparison; running prep on raw would conflate our
                method's impact with the baseline.  Pipeline calls
                pass enable_unify=True for ours, False for raw.
    """
    proj_name = project_dir.parent.name
    work_root = TMP_ROOT / proj_name / f"{label}-{ts}"
    work_root.mkdir(parents=True, exist_ok=True)
    crate_dst = work_root / "crate"
    _rsync_copy(project_dir, crate_dst)
    _ensure_fair_build(crate_dst)

    harness_dst: Path | None = None
    if workload.get("harness_dir"):
        # Harness lives at <project>/workloads/<harness_dir>
        harness_src = project_dir.parent / "workloads" / workload["harness_dir"]
        if not harness_src.is_dir():
            raise RuntimeError(f"harness dir not found: {harness_src}")
        harness_dst = work_root / "harness"
        _rsync_copy(harness_src, harness_dst)
        _ensure_fair_build(harness_dst)
        _rewire_path_dep(harness_dst / "Cargo.toml", crate_dst)
        # Inject shim modules so any module path the harness expects
        # but lib doesn't ship (e.g. `c_structs` when 0_raw only has
        # `libcsv.rs`) resolves via re-export.
        _ensure_harness_compat(crate_dst, harness_dst)
        # Run the pre-Stage-A cleanup pipeline if this is the "ours"
        # copy.  c2rust translates per-file, so e.g. bzip2's
        # `pub struct EState` ends up duplicated across 3 modules with
        # identical layout; Stage A E1's `extern "C"` strip then
        # triggers E0308 mismatched-types on every cross-module call
        # (BZ2_blockSort / BZ2_compressBlock / sendMTFValues /
        # handle_compress = 84% of bzip2's hot self_time).  The prep
        # pipeline folds those duplicates into shared canonicals first.
        if enable_unify:
            prep_for_stage_a(crate_dst, harness_dst)
        # Init git on the lib crate (Stage A only mutates the lib).
        _git_init(crate_dst, "1_cleaned")
        build_crate = harness_dst
    else:
        # Binary lives in the lib crate itself (e.g. bzip2 self-contained
        # bundles bzip2.rs as a [[bin]]).  The pre-Stage-A pipeline still
        # runs, but with the self-bin variant: same canonicalize + unify
        # pair, then the self-bin extern-block normalizer (rewrites
        # `extern "C" { fn / pub type }` in the bin source into Rust
        # `use <crate>::<mod>::…` imports), then `strip_staticlib`.
        if enable_unify:
            prep_for_stage_a(crate_dst, harness_dir=None)
            # 2026-05-28: ensure cargo can produce the W2 workload binary
            # even when c2rust packed `pub fn main()` into a lib-internal
            # module without declaring [[bin]]. Without this, cargo
            # produces only a lib (.rlib), workload measure fails to
            # locate the binary, and baseline_w2_ms = None crashes the
            # runner. See _ensure_bin_target_for_main for details.
            _ensure_bin_target_for_main(crate_dst, workload.get("binary"))
        _git_init(crate_dst, "1_cleaned")
        build_crate = crate_dst

    binary = build_crate / "target" / "release" / workload["binary"]
    return CrateLayout(
        crate_dir=crate_dst,
        harness_dir=harness_dst,
        binary_path=binary,
    )


def _build_layout(layout: CrateLayout) -> None:
    target = layout.harness_dir or layout.crate_dir
    _cargo_build(target)


def _ensure_sa_report(c_ref_cfg: dict, lib_crate_dir: Path) -> None:
    """Mandatory prereq: ensure SVF func_analysis_report.json exists for
    the project before Stage A runs. SA facts unlock E2 (ptr→ref),
    E2 v2 (Array ptr→slice), and E3 (callback monomorphize) passes —
    without them, those passes silently no-op.

    Zero per-project toml config. Knowledge flows from the existing
    `[c_reference].source_dir` + the existing c2rust lib mod tree:

      1. Read lib_crate_dir's `pub mod` structure → set of canonical
         lib source basenames (e.g. {lodepng}, {blocksort, bzlib, ...},
         {ops, distance, aabb2, ...}).
      2. rglob `source_dir` for `*.c` / `*.cpp` files whose stem
         matches a lib mod basename. This auto-excludes drivers
         (e.g. `lodepng_roundtrip_c.c`, `libcsv_exhaustive_driver.c`)
         because their stems aren't lib modules.
      3. Flatten matched sources into `/tmp/sa_auto_<proj>/`, renaming
         `.cpp` → `.c` (we feed to clang as plain C; c2rust-source-
         compatible .cpp files are C-style).
      4. Recursively flatten transitive headers via `#include "..."`
         (quoted) AND `#include <...>` (angle) — any header whose
         basename is findable under `source_dir`. This handles
         heman's `<heman.h>` (in include/), kazmath subdir headers, etc.
      5. Auto-stub external system-like headers when absent (currently
         just `omp.h`; extend `_KNOWN_HEADER_STUBS` as needed).
      6. Auto-define `<X>_NO_COMPILE_CPP=1` if any matched .cpp has
         `#ifdef <X>_COMPILE_CPP` — bypasses C++-only sections
         (lodepng's std::cout block).
      7. Run sa_engine/run.sh with auto-built SA_CFLAGS_EXTRA.
      8. Copy `func_analysis_report.json` back to
         `<source_dir>/svf_analysis_output/`.

    Idempotent: if the report already exists, returns immediately.
    """
    source_dir = Path(c_ref_cfg["source_dir"]).resolve()
    sa_out_dir = source_dir / "svf_analysis_output"
    sa_report = sa_out_dir / "func_analysis_report.json"
    if sa_report.is_file():
        # Idempotent ONLY on a VALID report. run.sh preserves *.json across
        # reruns, so a truncated/empty/partial report (e.g. an SVF timeout that
        # still wrote a skeleton) would otherwise be cached forever and silently
        # keep E2/E3 no-op'ing — the exact gap this whole hook closes. Validate
        # that it parses and carries at least one fn's facts before trusting it.
        try:
            from perf_opt.stage_a.sa_utils import _load_sa_facts
            n_fns = len(_load_sa_facts(sa_report))
        except Exception:
            n_fns = 0
        if n_fns > 0:
            logger.info(
                f"[sa-engine] SA report present + valid "
                f"({n_fns} fn(s)) at "
                f"{sa_report.relative_to(source_dir.parent)} — skip auto-gen"
            )
            return
        logger.warning(
            f"[sa-engine] SA report at "
            f"{sa_report.relative_to(source_dir.parent)} is empty/invalid "
            f"(0 fn(s)) — regenerating"
        )

    logger.info(
        f"[sa-engine] SA report missing — auto-generating "
        f"(prereq for E2/E2v2/E3 passes)"
    )

    # Step 1: lib mod basenames from c2rust output mod tree.
    lib_fns = scan_lib_public_fns(lib_crate_dir)
    if not lib_fns:
        raise RuntimeError(
            f"[sa-engine] scan_lib_public_fns returned empty for "
            f"{lib_crate_dir}; cannot derive SA source set"
        )
    lib_mod_basenames: set[str] = {p.split("::")[-1] for p in lib_fns.values()}

    # Step 2: match C/C++ sources in source_dir by stem.
    matched: list[Path] = []
    for ext in ("*.c", "*.cpp"):
        for c in source_dir.rglob(ext):
            if "svf_analysis_output" in c.parts:
                continue
            if c.stem in lib_mod_basenames:
                matched.append(c)
    if not matched:
        raise RuntimeError(
            f"[sa-engine] no C/C++ source in {source_dir} matches lib "
            f"modules {sorted(lib_mod_basenames)}; cannot generate SA report"
        )
    logger.info(
        f"[sa-engine] matched {len(matched)} lib source(s): "
        f"{[m.name for m in matched]}"
    )

    # Step 3: prepare flat work dir.
    proj_name = source_dir.name
    work_dir = Path(f"/tmp/sa_auto_{proj_name}_{int(time.time())}")
    work_dir.mkdir(parents=True, exist_ok=True)
    extra_defines: list[str] = []
    _CPP_GUARD_RE = re.compile(r'#ifdef\s+([A-Z_][A-Z0-9_]*_COMPILE_CPP)\b')
    for src in matched:
        flat = work_dir / (src.stem + ".c")
        data = src.read_bytes()
        if src.suffix == ".cpp":
            text = data.decode("utf-8", errors="replace")
            for m in _CPP_GUARD_RE.finditer(text):
                guard = m.group(1)
                no_guard = guard.replace("_COMPILE_CPP", "_NO_COMPILE_CPP")
                d = f"-D{no_guard}=1"
                if d not in extra_defines:
                    extra_defines.append(d)
        flat.write_bytes(data)

    # Step 4 + 5: recursively flatten transitive headers (quoted + angle).
    # Subpath-aware: if a header is included as `<kazmath/vec3.h>` (angle
    # with subpath), put it at work_dir/kazmath/vec3.h so `-I work_dir`
    # resolves it. ALSO put at work_dir/vec3.h so peer files in the same
    # directory (kazmath/*.c) using `"vec3.h"` (no subpath) resolve too.
    # `flattened` is keyed by the include-path-as-written for the angle
    # form; basename suffices for quoted includes.
    _INCLUDE_RE = re.compile(r'#include\s+(["<])([^">]+)[">]')
    flattened_paths: set[Path] = set()   # actual files written under work_dir
    scan_queue: list[Path] = list(work_dir.glob("*.c"))
    while scan_queue:
        cur = scan_queue.pop()
        try:
            text = cur.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for m in _INCLUDE_RE.finditer(text):
            inc_path = m.group(2)          # e.g. "kazmath/vec3.h"
            basename = Path(inc_path).name  # "vec3.h"
            # Find any file matching basename anywhere in source_dir.
            candidates = sorted(source_dir.rglob(basename))
            if not candidates:
                continue
            src_bytes = candidates[0].read_bytes()
            # Always place at work_dir/<basename> so peer-quoted includes
            # in the same dir (the flat work_dir) work.
            top_dst = work_dir / basename
            if top_dst not in flattened_paths:
                top_dst.write_bytes(src_bytes)
                flattened_paths.add(top_dst)
                scan_queue.append(top_dst)
            # If included as `<sub/path/foo.h>`, ALSO place at that
            # subpath so the angle-include with subpath resolves.
            if Path(inc_path).parent != Path("."):
                sub_dst = work_dir / inc_path
                if sub_dst not in flattened_paths:
                    sub_dst.parent.mkdir(parents=True, exist_ok=True)
                    sub_dst.write_bytes(src_bytes)
                    flattened_paths.add(sub_dst)

    # Step 6: stub external headers we can't find but recognize.
    _KNOWN_HEADER_STUBS: dict[str, str] = {
        "omp.h": (
            "#ifndef OMP_SHIM_H\n#define OMP_SHIM_H\n"
            "static inline int omp_get_max_threads(void){return 1;}\n"
            "static inline int omp_get_num_threads(void){return 1;}\n"
            "static inline int omp_get_thread_num(void){return 0;}\n"
            "#endif\n"
        ),
    }
    for cur in list(work_dir.rglob("*.c")) + list(work_dir.rglob("*.h")):
        try:
            text = cur.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for m in _INCLUDE_RE.finditer(text):
            basename = Path(m.group(2)).name
            top_dst = work_dir / basename
            if top_dst in flattened_paths:
                continue
            if basename in _KNOWN_HEADER_STUBS:
                top_dst.write_text(_KNOWN_HEADER_STUBS[basename])
                flattened_paths.add(top_dst)
                logger.info(f"[sa-engine] auto-stubbed <{basename}>")

    logger.info(
        f"[sa-engine] flat work dir: {work_dir} "
        f"({len(list(work_dir.glob('*.c')))} sources, "
        f"{len(flattened_paths)} header file(s) materialized, "
        f"defines: {extra_defines or 'none'})"
    )

    # Step 7: run sa_engine.
    sa_engine_dir = Path(__file__).parent.parent / "sa_engine"
    if not (sa_engine_dir / "run.sh").is_file():
        raise RuntimeError(
            f"[sa-engine] sa_engine/run.sh not found at {sa_engine_dir}"
        )
    sa_cflags = " ".join(
        [f"-I {work_dir}"]
        + extra_defines
        + ["-Wno-implicit-function-declaration"]
    )
    env = os.environ.copy()
    env["SA_CFLAGS_EXTRA"] = sa_cflags
    env.setdefault("SA_TIMEOUT_SEC", "1800")
    logger.info(
        f"[sa-engine] invoking run.sh (SA_CFLAGS_EXTRA={sa_cflags!r})"
    )
    proc = subprocess.run(
        ["bash", str(sa_engine_dir / "run.sh"), str(work_dir)],
        capture_output=True, text=True, env=env, cwd=str(sa_engine_dir),
    )
    if proc.returncode != 0:
        # Log last chunk of stdout + stderr so the user sees the failure.
        for tag, payload in (("stdout", proc.stdout), ("stderr", proc.stderr)):
            for line in (payload or "").splitlines()[-30:]:
                logger.error(f"[sa-engine.{tag}] {line}")
        raise RuntimeError(
            f"[sa-engine] auto-gen failed (rc={proc.returncode}); inspect "
            f"{work_dir} for the flat C sources used"
        )

    # Step 8: copy result back to <source_dir>/svf_analysis_output/.
    produced = work_dir / "svf_analysis_output" / "func_analysis_report.json"
    if not produced.is_file():
        raise RuntimeError(
            f"[sa-engine] sa_engine exited 0 but expected output missing: "
            f"{produced}"
        )
    sa_out_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy(produced, sa_report)
    logger.info(
        f"[sa-engine] ✓ SA report generated → "
        f"{sa_report.relative_to(source_dir.parent)}"
    )


def _build_c_reference(c_ref_cfg: dict) -> Path:
    """Build the C reference binary per `[c_reference]` toml block.
    Returns the absolute binary path. Raises if build fails."""
    require_c_compiler_version()
    src_dir = Path(c_ref_cfg["source_dir"]).resolve()
    build_cmd = c_ref_cfg["build_cmd"]
    binary = c_ref_cfg["binary"]
    if not src_dir.is_dir():
        raise RuntimeError(f"C reference source_dir not found: {src_dir}")
    logger.info(f"[c-ref] building in {src_dir}")
    proc = subprocess.run(
        ["bash", "-c", build_cmd],
        cwd=str(src_dir),
        capture_output=True, text=True, timeout=180,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"C reference build failed:\n{(proc.stderr or '')[-1500:]}"
        )
    bin_path = Path(binary)
    if not bin_path.is_absolute():
        bin_path = src_dir / binary
    if not bin_path.is_file():
        raise RuntimeError(f"C reference binary missing after build: {bin_path}")
    return bin_path


# ─────────────────────────────────────────────────────────────────
# Stage B — invoke perf_opt.orchestrator.run_loop on ours workdir.
# Methodologically Stage A and Stage B are independent passes; we keep
# them gated by the `--stage` CLI knob so ablation can isolate each
# stage's contribution.
# ─────────────────────────────────────────────────────────────────

def _materialize_stage_b_manifest(
    original_manifest: Path, ours_workdir_root: Path,
    ours_harness_dir: Path | None, ours_input_path: Path,
) -> Path:
    """Write a manifest mirror beside the ours workdir, rewriting `input`
    and `harness_dir` to ABSOLUTE paths into the disposable copy so Stage B
    runs against the SAME crate/harness Stage A measured.

    Stage B will resolve other manifest fields against the new manifest's
    parent dir; since we now use absolute paths for everything path-like,
    that resolution is a no-op."""
    raw = tomllib.loads(original_manifest.read_text(encoding="utf-8"))
    w = raw.setdefault("workload", {})
    # Rewrite paths.
    w["input"] = str(ours_input_path)
    if ours_harness_dir is not None:
        w["harness_dir"] = str(ours_harness_dir)
    # Wrapper script — resolve against the ORIGINAL manifest's dir (where
    # the script actually lives) and store as absolute. Without this, the
    # mirror's load_manifest resolves `wrapper = "wrapper_pipe.sh"` against
    # `<project>/3_stage_b/` and the orchestrator's W1 fails with
    # FileNotFoundError. Discovered 2026-06-04 wiring optipng's W1 oracle.
    if "wrapper" in w:
        w["wrapper"] = str((original_manifest.parent / w["wrapper"]).resolve())
    # Substitute $INPUT now too (load_manifest does this internally but
    # downstream tooling reading the manifest text directly is happier
    # with already-resolved args).
    if "args" in w:
        w["args"] = [
            str(ours_input_path) if a == "$INPUT" else a for a in w["args"]
        ]
    # We don't need [c_reference] for Stage B; drop it to avoid confusing
    # any tooling that re-reads the mirrored manifest.
    raw.pop("c_reference", None)

    # Serialize back to TOML by hand — stdlib has no tomllib writer.
    # We write a tiny "good enough" emitter that handles flat tables +
    # one [oracle.X] inline if present.
    def _emit(d: dict) -> str:
        out: list[str] = []
        for section, body in d.items():
            if not isinstance(body, dict):
                continue
            out.append(f"\n[{section}]")
            for k, v in body.items():
                if isinstance(v, list):
                    rendered = "[" + ", ".join(_render(x) for x in v) + "]"
                elif isinstance(v, dict):
                    # Nested table — emit as [<section>.<k>]
                    out.append(f"\n[{section}.{k}]")
                    for kk, vv in v.items():
                        out.append(f"{kk} = {_render(vv)}")
                    continue
                else:
                    rendered = _render(v)
                out.append(f"{k} = {rendered}")
        return "\n".join(out).strip() + "\n"

    def _render(v) -> str:
        if isinstance(v, str):
            return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
        if isinstance(v, bool):
            return "true" if v else "false"
        if isinstance(v, (int, float)):
            return str(v)
        if v is None:
            return '""'
        return '"' + str(v) + '"'

    mirror = ours_workdir_root / "pipeline_resolved.toml"
    mirror.write_text(_emit(raw), encoding="utf-8")
    return mirror


# NOTE: removed old Stage B harness (`StageBResult` / `_run_stage_b` /
# `_full_run`) on cleanup. Stage B now invokes
# `perf_opt.stage_b.characterize.stage_b_characterize` directly from
# `script_c2rust/main.py --from stage_b`; this file only carries the
# workdir-materialization helpers re-used by Stage A and Stage B.
