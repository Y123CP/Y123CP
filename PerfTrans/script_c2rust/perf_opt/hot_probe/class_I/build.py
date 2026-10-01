""                                                                      
                                                                   

                                                                 
                                                                  
                                                                      
                                                              
                                                                        
                                     

                                                                       
                                                                     
                                                                    
                                                             
   

from __future__ import annotations

import logging
import os
import shutil
import subprocess
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger("hot_probe.class_I.build")


@dataclass
class BuildResult:
    ok: bool
    ir_path: Path | None = None            # the fat-LTO harness .ll
    error: str = ""


def _pick_harness_ll(target_dir: Path, harness_pkg: str) -> Path | None:
    ""                                           
                                                                     
                                                                  
                    
    deps = target_dir / "release" / "deps"
    if not deps.is_dir():
        return None
    # `<harness_pkg>-<hash>.ll` — match by prefix; `harness_pkg` may contain
    # `-` so we look at all `*.ll` and filter by hyphen-hash tail pattern.
    cands: list[Path] = []
    for p in deps.glob("*.ll"):
        stem = p.stem                 # `harness-ab649b78b8c95656`
        if "-" not in stem:
            continue
        base, _, tail = stem.rpartition("-")
        if base == harness_pkg and len(tail) >= 8:
            cands.append(p)
    if not cands:
        return None
    return max(cands, key=lambda p: p.stat().st_mtime)


def build_and_emit_ir(harness_dir: Path, *,
                      harness_pkg: str = "harness",
                      cache_dir: Path | None = None,
                      timeout: int = 600,
                      force_rebuild: bool = False,
                      also_emit_remarks: bool = False,
                      ) -> BuildResult:
    """One release build on the harness with `--emit=llvm-ir`; returns the
    fat-LTO harness `.ll`. Optional cache; optional shared remark emission
    (so a single build serves Class I and Class II).

    Args:
      harness_dir       : harness project (cargo package) — its fat-LTO +
                          cgunits=1 release profile is a precondition
      harness_pkg       : the harness package's cargo package name (used
                          to disambiguate `target/release/deps/*.ll`);
                          default "harness" (dataset_trans convention)
      cache_dir         : if given, cache the .ll there
      timeout           : seconds for the build
      force_rebuild     : ignore any cache and force a fresh build
      also_emit_remarks : add `-C remark=all` so the same build's stderr
                          carries Class II's remark stream too

    Returns:
      `BuildResult` with `ir_path` on success.
    """
    harness_dir = Path(harness_dir).resolve()
    cache_dir = Path(cache_dir).resolve() if cache_dir is not None else None
    cache_ir: Path | None = cache_dir / "harness.ll" if cache_dir else None

    # ── cache hit? ──
    if not force_rebuild and cache_ir is not None and cache_ir.is_file():
        harness_bin = harness_dir / "target" / "release" / "harness"
        if not harness_bin.is_file() or (
                cache_ir.stat().st_mtime >= harness_bin.stat().st_mtime):
            logger.info("[class_I.build] cache hit: %s", cache_ir)
            return BuildResult(ok=True, ir_path=cache_ir)

    # ── build ──
    flags: list[str] = [
        "-C", "debuginfo=1",
        "--emit=llvm-ir",
    ]
    if also_emit_remarks:
        flags[:0] = ["-C", "remark=all"]

    env = os.environ.copy()
    env["RUSTFLAGS"] = " ".join(filter(None, (env.get("RUSTFLAGS", ""),
                                              *flags))).strip()

    logger.info("[class_I.build] cargo build --release on %s "
                "with RUSTFLAGS=%r", harness_dir, env["RUSTFLAGS"])

    try:
        proc = subprocess.run(
            ["cargo", "build", "--release"],
            cwd=str(harness_dir), env=env,
            capture_output=True, text=True, timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        return BuildResult(ok=False, error=f"cargo build timed out after {timeout}s")
    except OSError as e:
        return BuildResult(ok=False, error=f"cargo build failed to launch: {e}")

    if proc.returncode != 0:
        return BuildResult(
            ok=False,
            error=f"cargo exit={proc.returncode}\n"
                  f"{(proc.stderr or '')[-2000:]}")

    ir = _pick_harness_ll(harness_dir / "target", harness_pkg)
    if ir is None:
        return BuildResult(
            ok=False,
            error=f"no `{harness_pkg}-*.ll` in {harness_dir}/target/release/deps")

    # ── copy to cache ──
    if cache_ir is not None:
        cache_ir.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ir, cache_ir)
        logger.info("[class_I.build] cached IR → %s (%.1f MB)",
                    cache_ir, cache_ir.stat().st_size / 1024 / 1024)
        ir = cache_ir
    else:
        logger.info("[class_I.build] IR at %s (%.1f MB)",
                    ir, ir.stat().st_size / 1024 / 1024)

    result = BuildResult(ok=True, ir_path=ir)
    if also_emit_remarks:
        # Callers wanting the remark stream can re-parse; we don't
        # buffer stderr through Class I. Class II's own build wrapper is
        # the ergonomic path for Class II consumption.
        pass
    return result
