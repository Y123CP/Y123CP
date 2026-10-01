"""Build-environment preconditions shared by every profiling path.

There are two profiling stacks in this repo — `harness_gen.perf_workload`
(workload shape, at build-workload time) and `perf_opt.hot_probe` (hot function
location, at optimization time). They ask different questions of the same
binary, but they have the SAME preconditions, and those preconditions have been
learned twice and applied once, three times running:

  * a sizing probe must be bounded by time, not by a fixed iteration count;
  * attribution must be able to recover inlined frames;
  * ...

This module holds the preconditions so the next one is fixed in one place.
Deliberately dependency-free: `perf_opt` already imports `harness_gen`, so
anything shared must live here, never the other way round.
"""

from __future__ import annotations

import re
import subprocess
import tomllib
from pathlib import Path

# `debug = 2` (full DWARF, incl. DW_TAG_inlined_subroutine) is what lets
# `perf script --inline` put an inlined library function back on the stack as
# its own frame. Debug info does not change codegen — but it is NOT free of
# wall-clock consequences, and the claim that it is cost this project a night.
#
# DWARF records the ABSOLUTE path the crate was built from. Build the same
# source under two directory names of different length and you get two
# binaries of different SIZE, hence different code layout. Measured on fzy:
# two pristine harnesses, byte-identical semantics (paired insns -0.000%),
# built under `3_perf_opt` and `3_perf_opt_freeform`, differed by 6.182% of
# wall clock on one op (CI [+5.87, +6.49]) — more than twice the effect the
# ablation running on those two trees was trying to measure, and in the
# direction that reversed its verdict.
#
# `ensure_stable_debug_paths` below removes that path from the binary. Call
# it wherever `ensure_debuginfo` is called and the result gets timed.
DEBUG_LEVEL = "2"

# What the build root is rewritten to. Any constant works; it only has to be
# the same for every tree, and not collide with a real path.
REMAP_TARGET = "/perf-build"

# Codegen flags the TIMED binary must carry — identical to what the evaluation
# builds with (`dataset_trans/<proj>/validation_workload/purebin/*/.cargo/
# config.toml`). They go on the HARNESS because cargo resolves
# `.cargo/config.toml` from the directory cargo is INVOKED in: the library
# crate carries `-Ctarget-cpu=native` in its own config, but that file is never
# read when the build runs from the harness, so without this the whole binary —
# library included — was compiled for the x86-64 baseline.
#
# Not cosmetic (fzy, 2026-09-07): a rewrite that trades instruction count for
# code size measured -22.3% under the baseline target and +0.3% under
# `target-cpu=native`, on the SAME harness and the SAME input. Optimizing under
# one codegen and reporting under another lets the gate accept changes that are
# worth nothing in the configuration the paper reports.
#
# The 64-byte function alignment belongs here for the same reason and is the
# LARGER of the two effects: on http-parser, binaries built from byte-identical
# source with only inert padding inserted spanned 93-143 pp of per-operation
# overhead, and the spread collapsed to 0.5-1.1 pp once every function was
# aligned. Leaving it off the harness while the evaluation has it on reproduces
# the exact defect this constant exists to prevent (libopenaptx, 2026-09-07:
# the pipeline measured its two commits at +0.16% while the evaluation measured
# the same artifact 6 pp SLOWER than its own starting point).
CANONICAL_CODEGEN_FLAGS = ("-Ctarget-cpu=native", "-Ctarget-feature=-avx512f",
                           "-Cllvm-args=-align-all-functions=6")

_PROFILE_RE = re.compile(r"^\s*\[profile\.release\]", re.M)
_DEBUG_KEY_RE = re.compile(r"^\s*debug\s*=.*$", re.M)


def ensure_debuginfo(cargo_toml: Path, level: str = DEBUG_LEVEL) -> bool:
    """Force `[profile.release] debug = <level>`. Returns True if it changed.

    Must be applied to the TOP-LEVEL package (the harness): cargo compiles the
    whole dependency graph with the root package's profile, so this is what
    gives the CRATE UNDER TEST its debug info. Setting it on the crate alone
    does nothing.
    """
    text = cargo_toml.read_text(encoding="utf-8")
    if _PROFILE_RE.search(text):
        if _DEBUG_KEY_RE.search(text):
            new = _DEBUG_KEY_RE.sub(f"debug = {level}", text, count=1)
        else:
            new = _PROFILE_RE.sub(lambda m: f"{m.group(0)}\ndebug = {level}",
                                  text, count=1)
    else:
        new = text.rstrip() + f"\n\n[profile.release]\ndebug = {level}\n"
    if new == text:
        return False
    cargo_toml.write_text(new, encoding="utf-8")
    return True


def has_crate_debuginfo(binary: Path, crate_name: str) -> bool | None:
    """Does `binary` carry DWARF for `crate_name`'s own source?

    This is the precondition for inline-aware attribution. Without it,
    `perf script --inline` has nothing to expand, every inlined library
    function stays charged to the harness symbol that absorbed it, and a
    perfectly good op profiles as 100% harness. Measured on libxml2: the
    harness was built `debug = false`, `chvalid.rs` had no compilation unit at
    all, and `xmlIsBaseChar` (fully inlined, no code symbol left) put
    `chvalid_sweep` at 0% library across every candidate input.

    Returns None when it cannot be determined (no readelf, unreadable binary) —
    callers must treat that as "unknown", never as "absent".
    """
    if not binary.is_file() or not crate_name:
        return None
    try:
        proc = subprocess.run(
            ["readelf", "--debug-dump=info", str(binary)],
            capture_output=True, text=True, errors="replace", timeout=600)
    except (OSError, subprocess.SubprocessError):
        return None
    if proc.returncode != 0 or not proc.stdout:
        return None
    # Rust names compilation units by crate; a crate compiled without debug
    # info contributes none. Match the crate name as a DW_AT_name value.
    needle = re.compile(rf"DW_AT_name\s*:.*\b{re.escape(crate_name)}\b")
    return bool(needle.search(proc.stdout))


_RUSTFLAGS_LIST_RE = re.compile(r"(?ms)^[ \t]*rustflags[ \t]*=[ \t]*\[.*?\]")
_BUILD_HEADER_RE = re.compile(r"(?m)^\[build\][ \t]*$")
_REMAP_ANY_RE = re.compile(r"^--remap-path-prefix=")


def _write_rustflags(cfg: Path, flags: list[str]) -> None:
    """Render `flags` into `cfg`'s `[build] rustflags`, creating what's missing."""
    text = cfg.read_text(encoding="utf-8") if cfg.is_file() else ""
    rendered = ", ".join(f'"{f}"' for f in flags)
    if _RUSTFLAGS_LIST_RE.search(text):
        text = _RUSTFLAGS_LIST_RE.sub(f"rustflags = [{rendered}]", text, count=1)
    elif _BUILD_HEADER_RE.search(text):
        text = _BUILD_HEADER_RE.sub(f"[build]\nrustflags = [{rendered}]", text, count=1)
    else:
        head = text.rstrip() + "\n\n" if text.strip() else ""
        text = f"{head}[build]\nrustflags = [{rendered}]\n"
    cfg.parent.mkdir(parents=True, exist_ok=True)
    cfg.write_text(text, encoding="utf-8")


def _existing_rustflags(cfg: Path) -> list[str]:
    text = cfg.read_text(encoding="utf-8") if cfg.is_file() else ""
    try:
        data = tomllib.loads(text) if text.strip() else {}
    except tomllib.TOMLDecodeError:
        data = {}
    flags = data.get("build", {}).get("rustflags", [])
    return flags.split() if isinstance(flags, str) else list(flags)


def ensure_canonical_codegen(harness_dir: Path) -> bool:
    """Put the evaluation's codegen flags on the crate cargo is invoked from.

    Returns True if `<harness_dir>/.cargo/config.toml` changed.

    The harness holds the only `.cargo/config.toml` cargo reads for this build
    (it resolves from the invocation directory, never through a path
    dependency), so the library's own `-Ctarget-cpu=native` does nothing here.
    Without this the timed binary targets the x86-64 baseline while the
    evaluation targets this CPU — two different codegens, in which the same
    rewrite measured -22.3% and +0.3% respectively (see
    CANONICAL_CODEGEN_FLAGS).

    Existing flags are preserved; only missing canonical ones are appended.
    """
    cfg = harness_dir / ".cargo" / "config.toml"
    existing = _existing_rustflags(cfg)
    missing = [f for f in CANONICAL_CODEGEN_FLAGS if f not in existing]
    if not missing:
        return False
    _write_rustflags(cfg, existing + missing)
    return True


def ensure_stable_debug_paths(harness_dir: Path, build_root: Path) -> bool:
    """Make the built binary independent of WHERE it was built.

    Writes `--remap-path-prefix=<build_root>=<REMAP_TARGET>` into
    `<harness_dir>/.cargo/config.toml`. Returns True if the file changed.

    Why this is not optional wherever the output gets timed: see DEBUG_LEVEL
    above. Two trees that differ only in directory name produce binaries that
    differ in size, and a size difference is a layout difference. The ablation
    arm's tree is `3_perf_opt_freeform` against the full arm's `3_perf_opt` —
    a guaranteed length mismatch on EVERY project, with a per-project
    magnitude nobody can predict without measuring it.

    `--remap-path-prefix` is a debug-info flag. It rewrites a string in DWARF
    and touches no codegen decision, so it cannot move real performance — the
    only thing it removes is the accident.

    It adds no codegen flag itself — `ensure_canonical_codegen` does that and
    is called alongside it. The two are kept apart because this one is provably
    performance-neutral (DWARF only) while that one is not: adding codegen
    flags invalidates every measurement taken before them.

    Placement: cargo resolves `.cargo/config.toml` by walking up from the
    directory cargo was INVOKED in, never through a path-dependency. The
    harness is where `cargo build` runs, so the harness is where this goes;
    putting it on the crate would silently do nothing.
    """
    flag = f"--remap-path-prefix={build_root}={REMAP_TARGET}"
    cfg_dir = harness_dir / ".cargo"
    cfg = cfg_dir / "config.toml"
    text = cfg.read_text(encoding="utf-8") if cfg.is_file() else ""

    try:
        data = tomllib.loads(text) if text.strip() else {}
    except tomllib.TOMLDecodeError:
        data = {}
    existing = data.get("build", {}).get("rustflags", [])
    if isinstance(existing, str):
        existing = existing.split()
    if flag in existing:
        return False

    # A remap for a DIFFERENT root is stale — a tree that was moved or copied.
    # Leaving it would map nothing and the real path would stay in the binary.
    flags = [f for f in existing if not _REMAP_ANY_RE.match(f)] + [flag]
    rendered = ", ".join(f'"{f}"' for f in flags)

    if _RUSTFLAGS_LIST_RE.search(text):
        text = _RUSTFLAGS_LIST_RE.sub(f"rustflags = [{rendered}]", text, count=1)
    elif _BUILD_HEADER_RE.search(text):
        text = _BUILD_HEADER_RE.sub(f"[build]\nrustflags = [{rendered}]",
                                    text, count=1)
    else:
        head = text.rstrip() + "\n\n" if text.strip() else ""
        text = f"{head}[build]\nrustflags = [{rendered}]\n"

    cfg_dir.mkdir(parents=True, exist_ok=True)
    cfg.write_text(text, encoding="utf-8")
    return True
