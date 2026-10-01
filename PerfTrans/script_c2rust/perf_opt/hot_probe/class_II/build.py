"""Class II build wrapper — one `cargo build --release` on the harness that
serves both the remark scan (this module) and the Class I IR scan.

Two things this build MUST get right, both learned the hard way.

1. `-C remark=all` alone never reports loop-vectorize under LTO.
   Measured on nightly-2024-01-15 / LLVM 17, same two-crate source, only
   `lto` varied:

       lto = "fat"   → 0 loop-vectorize remarks
       lto = "thin"  → 0
       lto = false   → 3

   A single-crate build with fat LTO *does* get them, so the killer is the
   combination "code lives in a path-dependency crate" + "LTO on" — which is
   exactly every harness we build, and fat LTO is non-negotiable (it is what
   matches C's `-flto`). Consequence: II_vec fired on 0 of 12 projects.

   The fix is the `-Cllvm-args=-pass-remarks-*` channel, which keeps working
   under LTO (1451 remarks on the repro where `remark=all` gave 0). NOTE the
   older attempt at this in `profiling.opt_remarks._REMARK_LLVM_ARGS` asked
   only for `-pass-remarks-missed`, whose message is the bare header
   "loop not vectorized" with no reason — the classifier drops those, which
   is why it looked like the channel was dead. Every actionable reason
   arrives on `-pass-remarks-analysis`. Both are required.

2. Setting `RUSTFLAGS` in the environment silently discards
   `.cargo/config.toml`'s `[build] rustflags` — cargo does not merge them.
   Measured, with `-Ctarget-cpu=native` pinned in the config file:

       RUSTFLAGS unset            → 422 ymm instructions in the binary
       RUSTFLAGS="-C debuginfo=1" → 0

   So the previous version of this file, which set RUSTFLAGS directly, built
   the *analysis* artifact at x86-64 baseline while the *measured* artifact
   was `target-cpu=native` — the same detection/measurement codegen split
   that docs/perf_opt/2026-09-07-code-layout-and-codegen-parity.md was
   written about, surviving in a second place. lz4's scanned .ll carried
   `"target-cpu"="x86-64"` against a measured binary with 3480 ymm.

   The config file stays authoritative: we read its rustflags and forward
   them, because cargo would otherwise drop them, then assert on the emitted
   artifact that the canonical flags actually took (config says nothing about
   what the compiler did — see the `#[used]`-eaten-by-LTO lesson).
"""

from __future__ import annotations

import logging
import os
import subprocess
from pathlib import Path

from perf_opt.hot_probe.profiling.opt_remarks import (
    OptRemarksReport, parse_opt_remarks,
)

logger = logging.getLogger("hot_probe.class_II.build")

# The vectorizer's remarks survive LTO only on this channel, and only if the
# `analysis` half is asked for too — `missed` alone carries no reason and the
# classifier discards it. See this module's docstring for the measurements.
_VEC_REMARK_ARGS = (
    "-Cllvm-args=-pass-remarks-missed=loop-vectorize",
    "-Cllvm-args=-pass-remarks-analysis=loop-vectorize",
)


def _config_rustflags(harness_dir: Path) -> list[str]:
    """`[build] rustflags` from the harness's own `.cargo/config.toml`.

    Forwarded verbatim into RUSTFLAGS because setting that variable makes
    cargo ignore the file — the file remains the source of truth, we only
    carry it across.
    """
    cfg = harness_dir / ".cargo" / "config.toml"
    if not cfg.is_file():
        return []
    try:
        import tomllib
        flags = tomllib.loads(cfg.read_text(encoding="utf-8")) \
            .get("build", {}).get("rustflags", [])
    except Exception as e:                      # malformed config: say so, do
        logger.warning("[class_II.build] cannot read %s: %s", cfg, e)
        return []                               # not silently build differently
    # `--remap-path-prefix` stays behind. It changes no machine code — codegen
    # is the only thing this build has to share with the measured one — but it
    # rewrites every source path in debug info and remarks: the crate's
    # absolute path becomes `/perf-build/crate/...`. The consumers that map a
    # remark back to a function match on the crate-relative path, and
    # `merged_hits._rel_to_crate` cannot relativize a remapped one, so the
    # remark is dropped as unattributable.
    #
    # Forwarding it did exactly that on lz4: C3 remarks reached fn_hits.json
    # for 13-16 functions before, and for 0 after — every C3 candidate
    # silently gone, including the hash-chain rewrite that had committed at
    # -12.45% on the HC ops. The loop-vectorize channel prints relative paths
    # and kept working, which is what hid the loss.
    out: list[str] = []
    skip_next = False
    for f in (str(x) for x in flags):
        if skip_next:                           # value of a split
            skip_next = False                   # `--remap-path-prefix FROM=TO`
            continue
        if f == "--remap-path-prefix":
            skip_next = True
            continue
        if f.startswith("--remap-path-prefix="):
            continue
        out.append(f)
    return out


def _assert_codegen_parity(rustflags: list[str], ir_path: Path | None) -> None:
    """Warn loudly if the analysis build is not the measured build's codegen.

    Checked twice on purpose: once on what we asked for, once on what the
    compiler emitted. A flag present in the config and absent from the
    artifact is the failure mode that cost us a full 12-project rerun.
    """
    try:
        from harness_gen.build_env import CANONICAL_CODEGEN_FLAGS
    except Exception:
        return
    missing = [f for f in CANONICAL_CODEGEN_FLAGS if f not in rustflags]
    if missing:
        logger.warning("[class_II.build] ✗ analysis build is missing canonical "
                       "codegen flag(s) %s — Class I/II would be reading a "
                       "different binary than W2 measures", missing)
    if ir_path is None or not ir_path.is_file():
        return
    import re
    ir = ir_path.read_text(encoding="utf-8", errors="replace")
    # `-Ctarget-cpu=native` never survives as the string "native": rustc
    # resolves it to the concrete micro-architecture (here, cascadelake), so
    # the test is "some function got a real uarch", not a literal match.
    # A baseline-only IR is the broken state — that is exactly what the
    # pre-fix build produced, x86-64 everywhere against a measured binary
    # full of AVX2. Some x86-64 entries always remain: std ships precompiled.
    cpus = set(re.findall(r'"target-cpu"="([^"]*)"', ir))
    if cpus and cpus <= {"x86-64"}:
        logger.warning("[class_II.build] ✗ emitted IR is baseline-only "
                       "(target-cpu=%s) — the analysis artifact is not the "
                       "codegen W2 measures", sorted(cpus))
        return
    # A feature can appear twice in one attribute string: the uarch's default
    # set lists `+avx512f`, and our explicit `-Ctarget-feature=-avx512f` is
    # appended after it. LLVM takes the LAST occurrence, so a set membership
    # test reads the disabled state as enabled. Walk in order and keep the
    # final sign.
    def _last(tokens: list[str], feat: str) -> str | None:
        sign = None
        for tok in tokens:
            if tok[1:] == feat:
                sign = tok[0]
        return sign

    feats = re.search(r'"target-features"="([^"]*)"', ir)
    toks = (feats.group(1) if feats else "").split(",")
    if _last(toks, "avx2") != "+":
        logger.warning("[class_II.build] ✗ emitted IR does not enable avx2 "
                       "despite target-cpu=%s — check -Ctarget-feature",
                       sorted(cpus - {"x86-64"}))
        return
    if _last(toks, "avx512f") == "+":
        logger.warning("[class_II.build] ✗ emitted IR leaves avx512f enabled "
                       "— the measured build pins 256-bit vectors; analysis "
                       "would see wider ones")
        return
    logger.info("[class_II.build] ✓ emitted IR matches measured codegen "
                "(target-cpu=%s, +avx2, no avx512f)",
                sorted(cpus - {"x86-64"}))


def build_and_collect(harness_dir: Path, *,
                      timeout: int = 600,
                      also_emit_ir: bool = False) -> OptRemarksReport:
    """One release build on the harness with `-C remark=all`; returns the
    parsed `OptRemarksReport`.

    Args:
      harness_dir    : harness project (cargo package with path-dep on the
                       crate under test). Fat-LTO + cgunits=1 in its
                       release profile is a precondition (set by fair-build
                       upstream, not here).
      timeout        : seconds before giving up on the build
      also_emit_ir   : if True, also add `--emit=llvm-ir` so Class I can
                       consume the same build's `.ll` (default: remarks only)

    RUSTFLAGS composition:
      -C remark=all       (all passes emit missed / passed / analysis remarks)
      -C debuginfo=1      (DILocation for attribution; also required by Class I)
      --emit=llvm-ir      (only if also_emit_ir)
    """
    harness_dir = Path(harness_dir).resolve()

    # Order matters only for readability; cargo concatenates. The config-file
    # flags come first so a reader sees the measured build's codegen up front,
    # then what this build adds on top for analysis.
    flags: list[str] = [
        *_config_rustflags(harness_dir),   # carry the file's flags across —
                                           # RUSTFLAGS would otherwise drop them
        "-C", "remark=all",                # pass name + status inline
        *_VEC_REMARK_ARGS,                 # the only channel that survives LTO
        "-C", "debuginfo=1",
    ]
    if also_emit_ir:
        flags.append("--emit=llvm-ir")

    env = os.environ.copy()
    env["RUSTFLAGS"] = " ".join(filter(None, (env.get("RUSTFLAGS", ""),
                                              *flags))).strip()

    logger.info("[class_II.build] cargo build --release on %s "
                "with RUSTFLAGS=%r", harness_dir, env["RUSTFLAGS"])

    try:
        proc = subprocess.run(
            ["cargo", "build", "--release"],
            cwd=str(harness_dir), env=env,
            capture_output=True, text=True, timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        return OptRemarksReport(
            ok=False, error=f"cargo build timed out after {timeout}s")
    except OSError as e:
        return OptRemarksReport(
            ok=False, error=f"cargo build failed to launch: {e}")

    remarks = parse_opt_remarks(proc.stderr or "")
    ok = proc.returncode == 0
    if not ok:
        logger.warning("[class_II.build] cargo returned %d — %d remark(s) "
                       "parsed anyway", proc.returncode, len(remarks))

    n_vec = sum(1 for r in remarks if r.get("pass") == "loop-vectorize")
    logger.info("[class_II.build] %d remark(s), %d from loop-vectorize",
                len(remarks), n_vec)
    if ok and n_vec == 0:
        logger.warning("[class_II.build] ✗ zero loop-vectorize remarks — the "
                       "LTO suppression this module works around has come "
                       "back in some other form; II_vec cannot fire")

    ir = None
    if also_emit_ir:
        from perf_opt.hot_probe.class_I.build import _pick_harness_ll
        # Signature is (target_dir, harness_pkg) — the driver calls it the same
        # way. Deliberately not wrapped in try/except: a swallowed error here
        # silently skips the artifact-level parity check, which is the only
        # half of `_assert_codegen_parity` that can catch "the config asked
        # and the compiler declined".
        ir = _pick_harness_ll(harness_dir / "target", "harness")
    _assert_codegen_parity(flags, ir)

    return OptRemarksReport(
        ok=ok,
        remarks=remarks,
        error="" if ok else f"cargo exit={proc.returncode}",
    )
