"""Corpus production — raw candidate inputs per operation.

Two backends behind one interface; both emit raw candidate files that
corpus.curate() then validates against the baseline (crash/nondet
rejected, golden recorded). Nothing here decides functional correctness.

  libfuzzer : generate a fuzz_targets/<op>.rs per fuzzable op, run
              `cargo fuzz run <op>` (sanitizer OFF, ignore crashes,
              per-op time budget), then `cargo fuzz cmin` to minimize.
              Fast (in-process). Requires cargo-fuzz + a nightly that
              builds the pinned crate.
  pymut     : zero-dependency byte-mutation loop over the seeds, keeping
              inputs that run cleanly. Slow but always available; also
              the path for anything libFuzzer can't build.

Which ops are fuzzed: pure in-memory ops only. File-I/O ops (names
matching FILE_IO_HINTS or declared "needs_input": false) are skipped —
their coverage comes from seed replay, not fuzzing.
"""

from __future__ import annotations

import logging
import os
import shutil
from dataclasses import dataclass
from pathlib import Path

from .gates import run_cmd

logger = logging.getLogger(__name__)

FILE_IO_HINTS = ("file", "load", "save", "disk", "path")


@dataclass
class FuzzConfig:
    seconds_per_op: int = 600
    backend: str = "auto"          # auto | libfuzzer | pymut
    work_dir: Path | None = None   # scratch (default /dev/shm/...)
    max_candidates_per_op: int = 5000


def fuzzable_ops(spec: dict) -> list[str]:
    out = []
    for op in spec["operations"]:
        name = op["name"]
        if not op.get("needs_input", True):
            continue
        if any(h in name.lower() for h in FILE_IO_HINTS):
            logger.info("[fuzz] skipping file-I/O op %s", name)
            continue
        out.append(name)
    return out


def _scratch(cfg: FuzzConfig, harness_dir: Path) -> Path:
    if cfg.work_dir:
        d = Path(cfg.work_dir)
    else:
        shm = Path("/dev/shm")
        base = shm if shm.is_dir() and os.access(shm, os.W_OK) else Path("/tmp")
        d = base / f"harness_fuzz_{harness_dir.name}"
    d.mkdir(parents=True, exist_ok=True)
    return d


# ────────────────────────────────────────────────────────────────────
# backend availability
# ────────────────────────────────────────────────────────────────────

def libfuzzer_available() -> bool:
    return shutil.which("cargo-fuzz") is not None


def _probe_libfuzzer_builds(harness_dir: Path, an_op: str) -> tuple[bool, str]:
    """cargo-fuzz needs a nightly that can build the pinned crate with
    sanitizer-coverage. Probe once with a build-only run; on failure the
    caller falls back to pymut."""
    rc, _o, err = run_cmd(
        ["cargo", "fuzz", "check", "-s", "none", an_op], harness_dir, 900)
    if rc == 0:
        return True, ""
    return False, err[-2000:]


# ────────────────────────────────────────────────────────────────────
# libFuzzer backend
# ────────────────────────────────────────────────────────────────────

FUZZ_TARGET_TEMPLATE = """\
#![no_main]
use libfuzzer_sys::fuzz_target;

// Dispatch through the guaranteed ops() registry (no per-op fn-name
// assumptions). One execution, iters=1; the runner folds library errors
// into its digest and must never panic on malformed bytes.
fuzz_target!(|data: &[u8]| {{
    if let Some((_, f)) = {harness_name}::ops().into_iter().find(|(n, _)| *n == "{op}") {{
        let _ = f(data, 1);
    }}
}});
"""

FUZZ_CARGO_TEMPLATE = """\
[package]
name = "{harness_name}-fuzz"
version = "0.0.0"
publish = false
edition = "2021"

[package.metadata]
cargo-fuzz = true

[dependencies]
libfuzzer-sys = "0.4"
{harness_name} = {{ path = ".." }}

[[bin]]
name = "{op}"
path = "fuzz_targets/{op}.rs"
test = false
doc = false

{extra_bins}
"""


def setup_libfuzzer(harness_dir: Path, harness_name: str,
                    ops: list[str]) -> None:
    """Create fuzz/ crate with one target per op; each dispatches through
    the harness lib's ops() registry."""
    fuzz_dir = harness_dir / "fuzz"
    (fuzz_dir / "fuzz_targets").mkdir(parents=True, exist_ok=True)

    for op in ops:
        (fuzz_dir / "fuzz_targets" / f"{op}.rs").write_text(
            FUZZ_TARGET_TEMPLATE.format(harness_name=harness_name, op=op),
            encoding="utf-8")
    extra = "\n".join(
        f'[[bin]]\nname = "{op}"\npath = "fuzz_targets/{op}.rs"\n'
        f'test = false\ndoc = false\n' for op in ops[1:])
    (fuzz_dir / "Cargo.toml").write_text(
        FUZZ_CARGO_TEMPLATE.format(harness_name=harness_name, op=ops[0],
                                   extra_bins=extra),
        encoding="utf-8")


def run_libfuzzer(harness_dir: Path, ops: list[str], cfg: FuzzConfig,
                  scratch: Path) -> dict[str, list[Path]]:
    out: dict[str, list[Path]] = {}
    for op in ops:
        art = scratch / "corpus" / op
        art.mkdir(parents=True, exist_ok=True)
        # seed the libFuzzer corpus with gen-seeds outputs
        for idx in (1, 2):
            s = harness_dir / "seeds" / f"{op}.{idx}.bin"
            if s.exists():
                shutil.copy(s, art / f"seed{idx}.bin")
        # `-s none`: sanitizer OFF — c2rust code is UB-laden; ASan would
        # halt constantly and we want inputs, not crash reports.
        cmd = ["cargo", "fuzz", "run", "-s", "none", op, str(art),
               "--", f"-max_total_time={cfg.seconds_per_op}",
               "-ignore_crashes=1", "-timeout=5", "-rss_limit_mb=4096"]
        rc, _o, err = run_cmd(cmd, harness_dir,
                              cfg.seconds_per_op + 300,
                              {"RUSTFLAGS": ""})
        if rc not in (0, 77):   # 77 = libFuzzer found a crash (expected/ok)
            logger.warning("[fuzz] op %s cargo-fuzz rc=%d: %s", op, rc, err[-400:])
        out[op] = sorted(p for p in art.glob("*") if p.is_file())
        logger.info("[fuzz] op %-16s libFuzzer corpus: %d inputs", op, len(out[op]))
    return out


# ────────────────────────────────────────────────────────────────────
# pymut backend — zero-dependency mutation loop
# ────────────────────────────────────────────────────────────────────

def _mutate(data: bytes, rng_state: int) -> tuple[bytes, int]:
    """Deterministic LCG-driven byte mutation (no Math.random needed)."""
    s = (rng_state * 6364136223846793005 + 1442695040888963407) & ((1 << 64) - 1)
    if not data:
        return bytes([s & 0xFF]), s
    b = bytearray(data)
    choice = s % 5
    pos = (s >> 8) % len(b)
    if choice == 0:               # bit flip
        b[pos] ^= 1 << ((s >> 16) % 8)
    elif choice == 1:             # byte set
        b[pos] = (s >> 16) & 0xFF
    elif choice == 2:             # truncate
        b = b[: max(1, pos)]
    elif choice == 3:             # duplicate a chunk (grow)
        b = b + b[: min(len(b), 1 + (pos % 64))]
    else:                         # swap two bytes
        p2 = (s >> 24) % len(b)
        b[pos], b[p2] = b[p2], b[pos]
    return bytes(b), s


def run_pymut(harness_dir: Path, baseline_bin: Path, ops: list[str],
              cfg: FuzzConfig, scratch: Path) -> dict[str, list[Path]]:
    import time
    out: dict[str, list[Path]] = {}
    for op in ops:
        op_out = scratch / "pymut" / op
        if op_out.exists():
            shutil.rmtree(op_out)
        op_out.mkdir(parents=True)
        corpus: list[bytes] = []
        for idx in (1, 2):
            s = harness_dir / "seeds" / f"{op}.{idx}.bin"
            if s.exists():
                corpus.append(s.read_bytes())
        if not corpus:
            corpus = [b"\x00"]
        seen: set[bytes] = set(corpus)
        rng = 0x2545F4914F6CDD1D ^ (hash(op) & 0xFFFFFFFF)
        deadline = time.monotonic() + cfg.seconds_per_op
        tmp = op_out / "_cand.bin"
        n = 0
        while time.monotonic() < deadline and n < cfg.max_candidates_per_op:
            base = corpus[rng % len(corpus)]
            cand, rng = _mutate(base, rng)
            if cand in seen:
                continue
            seen.add(cand)
            tmp.write_bytes(cand)
            rc, _o, _e = run_cmd([str(baseline_bin), op, str(tmp)],
                                 harness_dir, 10)
            if rc == 0:                     # clean exit → keep, may grow corpus
                dst = op_out / f"m{n:06d}.bin"
                tmp.rename(dst)
                corpus.append(cand)
                n += 1
        out[op] = sorted(p for p in op_out.glob("m*.bin"))
        logger.info("[fuzz] op %-16s pymut kept: %d inputs", op, len(out[op]))
    return out


# ────────────────────────────────────────────────────────────────────
# driver
# ────────────────────────────────────────────────────────────────────

def produce_candidates(harness_dir: Path, harness_name: str, spec: dict,
                       baseline_bin: Path, cfg: FuzzConfig
                       ) -> tuple[dict[str, list[Path]], str]:
    """Returns ({op -> [candidate files]}, backend_used)."""
    ops = fuzzable_ops(spec)
    if not ops:
        return {}, "none"
    scratch = _scratch(cfg, harness_dir)

    want = cfg.backend
    if want in ("auto", "libfuzzer") and libfuzzer_available():
        setup_libfuzzer(harness_dir, harness_name, ops)
        ok, err = _probe_libfuzzer_builds(harness_dir, ops[0])
        if ok:
            return run_libfuzzer(harness_dir, ops, cfg, scratch), "libfuzzer"
        logger.warning("[fuzz] libFuzzer cannot build the pinned crate; "
                       "falling back to pymut.\n%s", err)
        if want == "libfuzzer":
            return {}, "libfuzzer-failed"

    return run_pymut(harness_dir, baseline_bin, ops, cfg, scratch), "pymut"
