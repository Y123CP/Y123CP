"""Performance workload runner for a generated harness.

The fuzz corpus is a FUNCTIONAL oracle — its inputs are small and
coverage-biased, so profiling them measures per-call overhead (malloc /
memset / setup), not the algorithm (see fzy: a 16-byte input profiles as
52% libc memset; a realistic 900-byte input profiles as 93% project
code). Profiling needs a REPRESENTATIVE, LARGE input per op, and enough
in-process repetition (`iters`) to reach steady state.

This module, per op:
  1. picks a perf input — prefers `perf_inputs/<op>.perf.bin` (produced by
     the harness's `gen-perf`, when present), else the largest corpus
     input as a cheap "heaviest" proxy;
  2. auto-tunes `iters` so one measured run is ≈`target_wall` seconds;
  3. measures under perf (instructions, cycles) + perf record to compute
     `self_time_own` = fraction of samples in the harness binary (project
     code) vs libc/libm/ld/kernel;
  4. flags ops with self_time_own < 0.70 as poor perf targets.

Frequency pinning (perf_run.sh) is OFF by default — self_time_own and
instruction count are frequency-independent; enable --pin only for
final reproducible wall-clock numbers.
"""

from __future__ import annotations

import json
import logging
import re
import shutil
import time
from dataclasses import dataclass, field
from pathlib import Path

from .build_env import ensure_debuginfo, has_crate_debuginfo
from .gates import run_cmd, BUILD_TIMEOUT

logger = logging.getLogger(__name__)

PERF_RUN_SH = "/home/anonymous/artifact/PerfTrans/perf_run.sh"
PIN_CORE = "16"
SELF_TIME_GATE = 0.70
# Minimum share of samples that must land in the LIBRARY (i.e. excluding both
# libc/kernel and the harness's own symbols) for an op to be a usable perf
# target. Calibrated on a case where four ops sat at 98-99.9% self_time_own —
# and so passed the gate above — while 72-99.7% of that was the harness's own
# per-iteration digest code, leaving the library at ~0%: perf_opt then had
# nothing to optimize and committed nothing. After the harness was fixed the
# same ops measured 86-96% library, so 50% cleanly separates the two regimes.
# Advisory, not fatal — it is measured under THIS module's autotuned iters, and
# a one-off setup pass over the input (e.g. the first checksum of a large input)
# is amortized differently at a different iteration count. Downstream stages
# autotune their own iters and can legitimately see a healthier split, so treat
# a flag as "go look at the workload shape", never as a hard verdict.
CRATE_SHARE_GATE = 0.50
# Sizing probes measure per-iteration cost by running the op N times. N is not
# free: the probe itself costs N x that cost, so a fixed N is only safe for
# cheap ops. Cap the probe by TIME instead and let N follow from it.
PROBE_BUDGET_S = 20.0
# Upper rung for the probe — enough iterations to amortize process startup for
# ops whose single iteration is too short to time reliably.
PROBE_CEIL = 200
# What a candidate-selection probe aims to spend per candidate.
PROBE_TARGET_S = 0.3
NON_OWN_DSO = re.compile(
    r"lib(c|m|pthread|gcc_s|dl)[-.]|ld-linux|ld-2|\[kernel|\[vdso\]|\[unknown\]"
)
# A `perf report --sort overhead,dso,symbol` row:
#     49.92%  harness      [.] zopfli_raw_harness::op_cache_roundtrip  -      -
# The trailing `-  -` are empty sort columns perf appends. Stopping the symbol
# at the first run of two-plus spaces drops them while keeping the spaces
# INSIDE a symbol (`<T as Trait>::f`), which never come in pairs. Without this
# the captured name was `ZopfliInitCache                         -      -`,
# which then went into the report and into perf_workload.json.
_SYMBOL_ROW_RE = re.compile(
    r"\s*([0-9.]+)%\s+(\S+)\s+\[[.k]\]\s+(.+?)(?:\s{2,}.*)?$")


@dataclass
class OpPerf:
    op: str
    input_path: str = ""
    input_bytes: int = 0
    input_source: str = ""      # "gen-perf" | "corpus-largest"
    iters: int = 0
    wall_s: float = 0.0
    instructions: int = 0
    cycles: int = 0
    self_time_own: float = 0.0
    harness_share: float = 0.0  # of total samples, in the harness's OWN code
    top: list = field(default_factory=list)   # [(pct, symbol, dso)]
    ok: bool = False
    note: str = ""
    # True = the library is measurably NOT hot here. False = it is.
    # None = could not be determined — the binary carried no DWARF for the
    # crate under test, so inlined library work is indistinguishable from
    # harness work. A tri-state matters because downstream excludes ops on
    # this flag, and "unknown" must never be read as "bad".
    harness_bound: bool | None = False
    attribution_reliable: bool = True

    @property
    def crate_share(self) -> float:
        """Share of samples in the library under test.

        `self_time_own` only separates this process from libc/kernel — but the
        harness and the library are linked into the SAME binary, so a workload
        that spends all its time in the harness's own digest/verification code
        still scores ~100% there. Subtracting the harness's own symbols is what
        actually answers "is this op measuring the library?".
        """
        return max(0.0, self.self_time_own - self.harness_share)

    def as_dict(self) -> dict:
        return {
            "op": self.op, "input_path": self.input_path,
            "input_bytes": self.input_bytes, "input_source": self.input_source,
            "iters": self.iters, "wall_s": round(self.wall_s, 4),
            "instructions": self.instructions, "cycles": self.cycles,
            "cpi": round(self.cycles / self.instructions, 3) if self.instructions else 0,
            "self_time_own": round(self.self_time_own, 3),
            "harness_share": round(self.harness_share, 3),
            "crate_share": round(self.crate_share, 3),
            "harness_bound": self.harness_bound,
            "attribution_reliable": self.attribution_reliable,
            "top_functions": self.top[:8], "ok": self.ok, "note": self.note,
        }


def symbol_owner(symbol: str, harness_crate: str, dso: str = "") -> str:
    ""                                                                     

                                                                            
                                                                         
                                                                               
                                                                             
                                                                              
                                                               

                                                                       
                                                                            
                                                                         
                                                                       
                                                                            
                                                                           

                                                                           
                                                                          
                                                                             
                                                                  
       
    if not symbol or symbol.startswith("0x"):
        return ""                                # unresolved address
    if dso and NON_OWN_DSO.search(dso):
        return "外部"                            # libc / kernel / vdso
    if harness_crate and f"{harness_crate}::" in symbol:
        # `in`, not `startswith`: trait-impl symbols read as
        # `<pkg::Ty as Trait>::method` and are harness code too.
        return "harness"
    return "library"


def _low_crate_share_cause(row: "OpPerf", hot: str, harness_crate: str,
                           hot_dso: str = "") -> str:
    """Why the library is not hot here, and what to do about it.

    One threshold, several different diseases — and most of them are NOT "the
    harness does too much work", so telling every one of them to hoist
    loop-invariant digest work sends the reader after a fix that cannot exist.
    Observed on one libxml2 run: an op at 2% harness whose time was all in the
    kernel, an op at 5% harness whose hottest symbol was library code, and an
    op at 99% "harness" that was really the library inlined into it.

    The hottest symbol's OWNER is the primary clue, and it is checked before
    the self-time threshold: an op can be under the threshold and still be
    running library code for every in-process cycle, and "most time is outside
    this process" alone would hide that. Reported on optipng: an op whose
    hottest symbol was `crc32_z` — the crate's own vendored zlib — was
    described as libc/kernel time.
    """
    if not row.attribution_reliable:
        return ("attribution is UNRELIABLE — the binary has no DWARF for the"
                " crate under test, so library functions inlined into the"
                " harness are charged to the harness and this share is a lower"
                " bound, not a measurement. Rebuild with"
                " [profile.release] debug = 2 and re-profile before believing"
                " it.")
    owner = symbol_owner(hot, harness_crate, hot_dso)
    if owner == "library":
        outside = 1.0 - row.self_time_own
        return (f"the hottest symbol ({hot}) is LIBRARY code, so the op does"
                f" exercise the library; the shortfall is the"
                f" {outside:.0%} spent outside this process (libc/allocator/"
                f"kernel) around it. Usually a larger or denser input fixes"
                f" this.")
    if owner == "外部":
        return (f"the hottest symbol ({hot}) is libc/kernel — a syscall- or"
                " allocator-bound op does not become a perf target by editing"
                " the harness. Either give it an input that makes the library"
                " work dominate, or accept it as functional-only.")
    if owner == "":
        return ("the hottest symbol did not resolve to a name (no symbols for"
                " that DSO). Re-profile with the crate's debug info present"
                " before drawing a conclusion from this share.")
    if row.self_time_own < SELF_TIME_GATE:
        return ("most time is OUTSIDE this process (libc/kernel) — a"
                " syscall- or allocator-bound op does not become a perf target"
                " by editing the harness. Either give it an input that makes"
                " the library work dominate, or accept it as functional-only.")
    return ("this op measures the harness, not the library. Move"
            " loop-invariant digest/verification work out of the timed loop,"
            " and never re-implement a library function inside the harness."
            " If the harness work is already hoisted, the remaining cost is"
            " the ONE-OFF setup pass over the input: a very large perf input"
            " keeps iters low, so that pass never amortizes — shrink the input"
            " or raise iters.")


def _wrap(cmd: list[str], pin: bool) -> list[str]:
    inner = ["taskset", "-c", PIN_CORE] + cmd
    return ["bash", PERF_RUN_SH] + inner if pin else inner


def build_release(hdir: Path) -> Path:
    # Attribution below has to tell "the library is hot" from "the harness is
    # hot", and small library functions get inlined INTO the harness at
    # opt-level 3 + fat LTO, keeping no symbol of their own. Recovering them
    # needs DWARF for the crate under test, which cargo only emits when the
    # TOP-LEVEL package (this harness) asks for it.
    ensure_debuginfo(hdir / "Cargo.toml")
    rc, _o, err = run_cmd(["cargo", "build", "--release", "--bin", "harness"],
                          hdir, BUILD_TIMEOUT)
    if rc != 0:
        raise RuntimeError(f"release build failed:\n{err[-2000:]}")
    return hdir / "target" / "release" / "harness"


def try_gen_perf(harness_bin: Path, hdir: Path) -> dict[str, Path]:
    """Run `harness gen-perf <dir>` if the harness supports it. Returns
    {op -> perf input path}. Empty dict if unsupported."""
    outdir = hdir / "perf_inputs"
    if outdir.exists():
        shutil.rmtree(outdir)
    outdir.mkdir(parents=True)
    rc, out, _err = run_cmd([str(harness_bin), "gen-perf", str(outdir)], hdir, 300)
    found: dict[str, Path] = {}
    # Clean run: use the `perf <op> <path>` lines gen_perf printed.
    for line in out.splitlines():
        # `perf <op> <path>`
        parts = line.split()
        if len(parts) == 3 and parts[0] == "perf":
            p = Path(parts[2])
            if p.exists() and p.stat().st_size > 0:
                found[parts[1]] = p
    # Robustness: gen_perf may CRASH partway (e.g. a mis-sized buffer in one
    # op's encode step) AFTER already writing valid inputs for the earlier ops.
    # A non-zero exit prints no lines, but the files on disk are good — salvage
    # every `<op>.perf.bin` actually written so one broken op doesn't discard
    # the whole big/realistic perf-input set to corpus fallback. The perf gate
    # (gates.gate_perf) still surfaces the crash at generation time.
    if rc != 0:
        for pf in outdir.glob("*.perf.bin"):
            if pf.stat().st_size > 0:
                found.setdefault(pf.name[: -len(".perf.bin")], pf)
    return found


def largest_corpus_input(hdir: Path, op: str) -> Path | None:
    d = hdir / "corpus" / op
    if not d.is_dir():
        return None
    files = [p for p in d.glob("*.bin") if p.is_file()]
    return max(files, key=lambda p: p.stat().st_size) if files else None


def _corpus_candidates(hdir: Path, op: str, k: int = 3) -> list[Path]:
    """Top-k largest corpus inputs — bigger usually exercises more work,
    but the best one is picked empirically by self_time (algorithms with an
    internal size cap, e.g. fzy's MATCH_MAX_LEN, prefer a mid-size input, so
    we never trust size alone)."""
    d = hdir / "corpus" / op
    if not d.is_dir():
        return []
    files = sorted((p for p in d.glob("*.bin") if p.is_file()),
                   key=lambda p: p.stat().st_size, reverse=True)
    return files[:k]


# Truncation ladder for candidate inputs. The 1 MiB rung matters: gen_perf
# sometimes emits a tens-of-MB input, and at that size autotune can only afford
# a few thousand iterations, so any one-off per-run setup (the harness's first
# pass over the input) never amortizes — and the working set stops fitting in
# cache, so the profile drifts toward memory bandwidth. Measured on a streaming
# parser, library share by input size: 2K 55%, 16K 55%, 128K 56%, 1.4M 57%,
# 4M 53%, 26M **14%**. Everything up to a few MB is equivalent; only the huge
# original collapses. 1 MiB keeps a realistic-scale rung near the L3 boundary.
_SWEEP_SIZES = (2_048, 16_384, 131_072, 1_048_576)   # bytes


def _size_variants(inp: Path, tmpdir: Path, op: str) -> list[Path]:
    """Head-truncated variants of a candidate input, so algorithms with an
    internal size cap (e.g. fzy's MATCH_MAX_LEN=1024, where a huge input just
    makes libc strlen dominate) can be measured at a size where the algorithm
    itself dominates — found empirically, without knowing the cap. A truncation
    that corrupts a structured input simply profiles poorly (or hits the
    harness's error-as-data path) and loses candidate selection, so this is
    safe for any format."""
    try:
        data = inp.read_bytes()
    except OSError:
        return []
    tmpdir.mkdir(parents=True, exist_ok=True)
    out = []
    for sz in _SWEEP_SIZES:
        if len(data) > sz:
            p = tmpdir / f"{op}.trunc{sz}.bin"
            p.write_bytes(data[:sz])
            out.append(p)
    return out


def _time_once(harness_bin: Path, op: str, inp: Path, iters: int,
               hdir: Path, pin: bool) -> float:
    cmd = _wrap([str(harness_bin), op, str(inp), str(iters)], pin)
    t0 = time.monotonic()
    rc, _o, _e = run_cmd(cmd, hdir, 300)
    dt = time.monotonic() - t0
    return dt if rc == 0 else -1.0


def _per_iter_cost(harness_bin: Path, op: str, inp: Path, hdir: Path,
                   pin: bool) -> float:
    """Seconds per iteration, measured with a probe that stays cheap.

    A fixed N-iteration probe costs N x the per-iteration cost, so it is only
    safe when that cost is small. It is not always small: a compressor over a
    1 MiB input runs ~3 s per iteration, where N=200 needs ~600 s and blows
    `_time_once`'s 300 s cap — the op then looks like it FAILED and is dropped
    from the workload entirely, and an op that lands just under the cap keeps a
    several-hundred-second wall that no downstream A/B loop can afford.

    So size the probe by time: one iteration first, then re-measure at the
    largest rung that fits `PROBE_BUDGET_S`. Having both points, take the SLOPE
    rather than the average — that cancels the fixed process-startup term,
    which a single averaged probe folds into the per-iteration cost and which
    for a microsecond-scale op dominates it (4 ms of startup over 200 iters of
    a 10 us op reads as 30 us/iter, so the op is then measured for a third of
    the intended wall). Returns -1.0 only when the op genuinely errors.
    """
    t1 = _time_once(harness_bin, op, inp, 1, hdir, pin)
    if t1 <= 0:
        return -1.0
    probe = max(1, min(PROBE_CEIL, int(PROBE_BUDGET_S / t1)))
    if probe <= 1:
        return t1
    t = _time_once(harness_bin, op, inp, probe, hdir, pin)
    if t <= 0:
        return -1.0
    slope = (t - t1) / (probe - 1)
    # Noise (or a cached first run) can put the two points out of order; the
    # average is the safe fallback because it can only over-estimate.
    return slope if slope > 0 else t / probe


def _probe_iters(harness_bin: Path, op: str, inp: Path, hdir: Path,
                 pin: bool) -> int:
    """Small iters count for a cheap (~0.3s) self_time probe of a candidate."""
    per_iter = _per_iter_cost(harness_bin, op, inp, hdir, pin)
    if per_iter <= 0:
        return 1
    return max(1, min(int(PROBE_TARGET_S / per_iter), 2_000_000))


def _autotune_iters(harness_bin: Path, op: str, inp: Path, hdir: Path,
                    target_wall: float, pin: bool) -> tuple[int, float]:
    """`iters` such that one measured run takes ≈`target_wall` seconds.

    No lower bound on the count: the invariant that matters is the WALL, and a
    floor of N iterations violates it in the expensive direction — an op at
    1.4 s per iteration pinned to 200 iterations measures for 283 s. When one
    iteration already exceeds `target_wall`, one iteration is the right answer.
    """
    per_iter = _per_iter_cost(harness_bin, op, inp, hdir, pin)
    if per_iter <= 0:
        return 0, -1.0
    iters = int(target_wall / per_iter)
    return max(1, min(iters, 100_000_000)), per_iter


def _perf_stat(harness_bin: Path, op: str, inp: Path, iters: int,
               hdir: Path, pin: bool) -> tuple[int, int, float]:
    cmd = _wrap(["perf", "stat", "-e", "instructions,cycles",
                 str(harness_bin), op, str(inp), str(iters)], pin)
    t0 = time.monotonic()
    rc, _o, err = run_cmd(cmd, hdir, 600)
    wall = time.monotonic() - t0
    ic = cyc = 0
    for line in err.splitlines():
        s = line.replace(",", "").strip()
        m = re.match(r"([0-9]+)\s+instructions", s)
        if m:
            ic = int(m.group(1))
        m = re.match(r"([0-9]+)\s+cycles", s)
        if m:
            cyc = int(m.group(1))
    return ic, cyc, wall


def harness_crate_name(hdir: Path) -> str:
    """`[package] name` of the harness crate — the prefix its own symbols carry."""
    try:
        text = (hdir / "Cargo.toml").read_text(encoding="utf-8", errors="replace")
    except OSError:
        return ""
    in_package = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("["):
            in_package = stripped == "[package]"
            continue
        if in_package and stripped.startswith("name"):
            _, _, value = stripped.partition("=")
            return value.strip().strip('"').strip("'")
    return ""


_PATH_DEP_RE = re.compile(
    r"^\s*(?P<key>[A-Za-z0-9_-]+)\s*=\s*\{[^}]*\bpath\s*=", re.M)
_DEP_PACKAGE_RE = re.compile(r"\bpackage\s*=\s*[\"']([^\"']+)[\"']")


def crate_under_test_name(hdir: Path) -> str:
    """Package name of the crate this harness measures.

    Needed to ask whether the binary carries DWARF for it. It is NOT the
    dependency's key in `[dependencies]`: the harness renames it, so
    `brotli_raw = { path = "...", package = "brotli_cleaned" }` must resolve to
    `brotli_cleaned` — the name cargo and DWARF actually use.
    """
    try:
        text = (hdir / "Cargo.toml").read_text(encoding="utf-8", errors="replace")
    except OSError:
        return ""
    for line in text.splitlines():
        m = _PATH_DEP_RE.match(line)
        if not m:
            continue
        renamed = _DEP_PACKAGE_RE.search(line)
        return renamed.group(1) if renamed else m.group("key")
    return ""


def _perf_selftime(harness_bin: Path, op: str, inp: Path, iters: int,
                   hdir: Path, pin: bool) -> tuple[float, float, list]:
    """Returns (self_time_own, harness_share, top_symbols).

    `harness_share` is the fraction of ALL samples landing in symbols belonging
    to the harness crate itself — the part of `self_time_own` that is scaffolding
    rather than library under test.
    """
    data = hdir / f".perf_{op}.data"
    rec = _wrap(["perf", "record", "-o", str(data), "--",
                 str(harness_bin), op, str(inp), str(iters)], pin)
    rc, _o, err = run_cmd(rec, hdir, 600)
    if rc != 0 or not data.exists():
        return -1.0, 0.0, []
    # DSO-level breakdown → self_time_own = 1 - (libc/libm/ld/kernel)
    rc, out, _e = run_cmd(["perf", "report", "-i", str(data), "--stdio",
                           "-g", "none", "--sort", "dso"], hdir, 300)
    own = 0.0
    for line in out.splitlines():
        m = re.match(r"\s*([0-9.]+)%\s+(.+?)\s*$", line)
        if not m or line.lstrip().startswith("#"):
            continue
        pct = float(m.group(1)) / 100.0
        dso = m.group(2).strip()
        if not NON_OWN_DSO.search(dso):
            own += pct
    # symbol-level breakdown: keep the top few for reporting, and accumulate the
    # harness's OWN share over EVERY symbol (not just the top ones, or a workload
    # split across many small harness helpers would look clean).
    # DSO in the sort key, because the symbol name alone cannot say whether
    # `memcpy` came from glibc or from the crate's own vendored copy.
    rc, out, _e = run_cmd(["perf", "report", "-i", str(data), "--stdio",
                           "-g", "none", "--sort", "overhead,dso,symbol"],
                          hdir, 300)
    crate = harness_crate_name(hdir)
    top: list = []
    harness_share = 0.0
    for line in out.splitlines():
        m = _SYMBOL_ROW_RE.match(line)
        if not m:
            continue
        pct, dso, symbol = float(m.group(1)), m.group(2), m.group(3).strip()
        if symbol_owner(symbol, crate, dso) == "harness":
            harness_share += pct / 100.0
        if len(top) < 10:
            top.append((pct, symbol[:60], dso))
    data.unlink(missing_ok=True)
    return own, harness_share, top


# `perf script` output: a sample header carrying the period, then one indented
# frame per line (leaf first).  Same shapes hot_probe/selftime.py parses.
_SAMPLE_HEADER_RE = re.compile(r"^\S.*:\s+(\d+)\s+\S+:\s*$")
_STACK_FRAME_RE = re.compile(r"^\s+[0-9a-fA-F]+\s+(.*?)\s+\(([^)]*)\)\s*$")


def _harness_share_precise(harness_bin: Path, op: str, inp: Path, iters: int,
                           hdir: Path, pin: bool, marker: str,
                           crate_name: str = "") -> float | None:
    """Harness self-share attributed to the DEEPEST non-system frame.

    Flat symbol attribution is not good enough here: when a library function is
    inlined into a harness function, its samples are charged to the harness
    symbol, which makes a perfectly good workload look harness-bound.  Sampling
    with a call graph and expanding inline frames puts the library function back
    on the stack as its own frame, so "who is really running" can be read off
    the innermost frame that is neither libc nor the kernel.

    Returns None when the profile could not be taken, so the caller can fall
    back to the flat estimate rather than inventing a number.

    That contract has a precondition this function used to skip: expanding
    inline frames requires DWARF for the crate under test. Without it `--inline`
    silently expands nothing, every absorbed library function stays charged to
    the harness, and this returns a symbol-level number wearing a call-graph
    costume — which is worse than returning nothing, because the caller trusts
    it. Measured on libxml2: 99.1% "harness", 0% library, on an op whose timed
    loop contains nothing but library calls.
    """
    if not marker:
        return None
    if crate_name and has_crate_debuginfo(harness_bin, crate_name) is False:
        logger.warning(
            "[perf] %s: no DWARF for `%s` in the harness binary — inline-aware "
            "attribution is impossible, reporting library share as UNKNOWN "
            "(build the harness with [profile.release] debug = 2)",
            op, crate_name)
        return None
    data = hdir / f".perf_cg_{op}.data"
    rec = _wrap(["perf", "record", "--call-graph", "dwarf", "-F", "999",
                 "-o", str(data), "--",
                 str(harness_bin), op, str(inp), str(iters)], pin)
    rc, _o, _e = run_cmd(rec, hdir, 900)
    if rc != 0 or not data.exists():
        data.unlink(missing_ok=True)
        return None
    rc, out, _e = run_cmd(["perf", "script", "-i", str(data), "--inline"],
                          hdir, 900)
    data.unlink(missing_ok=True)
    if rc != 0 or not out:
        return None

    total = harness = 0.0
    period = 0.0
    frames: list[tuple[str, str]] = []

    def flush() -> None:
        nonlocal total, harness
        if not frames:
            return
        total += period
        for symbol, dso in frames:                  # leaf → root
            if NON_OWN_DSO.search(dso) or NON_OWN_DSO.search(symbol):
                continue                            # libc/kernel: keep descending
            if marker in symbol:
                harness += period
            return                                  # deepest own frame decides

    for line in out.splitlines():
        header = _SAMPLE_HEADER_RE.match(line)
        if header:
            flush()
            frames, period = [], float(header.group(1))
            continue
        frame = _STACK_FRAME_RE.match(line)
        if frame:
            frames.append((frame.group(1).strip(), frame.group(2).strip()))
    flush()

    if total <= 0:
        return None
    return harness / total


def profile_harness(hdir: Path, ops: list[str], target_wall: float = 1.5,
                    pin: bool = False) -> list[OpPerf]:
    harness_bin = build_release(hdir)
    # Read once: both the attribution marker and the diagnostics need it, and
    # it comes from this harness's manifest — never from a name we assumed.
    harness_crate = harness_crate_name(hdir)
    perf_inputs = try_gen_perf(harness_bin, hdir)
    if perf_inputs:
        logger.info("[perf] gen-perf produced inputs for: %s",
                    sorted(perf_inputs))
    else:
        logger.info("[perf] harness has no gen-perf; falling back to "
                    "largest corpus input per op")

    results: list[OpPerf] = []
    for op in ops:
        r = OpPerf(op=op)
        # candidate inputs: gen-perf output + top-k largest corpus inputs.
        # Pick empirically by self_time_own — "largest" is not always best
        # (algorithms with an internal size cap prefer a mid-size input).
        candidates: list[tuple[str, Path]] = []
        sweep_dir = hdir / ".perf_sweep"
        if op in perf_inputs:
            gp = perf_inputs[op]
            candidates.append(("gen-perf", gp))
            # size sweep: truncated variants let capped-size algorithms be
            # measured where the algorithm (not libc strlen/memset) dominates
            for v in _size_variants(gp, sweep_dir, op):
                candidates.append(("gen-perf/trunc", v))
        for c in _corpus_candidates(hdir, op):
            candidates.append(("corpus", c))
        if not candidates:
            r.note = "no perf input (no gen-perf, no corpus)"
            results.append(r)
            continue

        best = None  # (crate_share, source, path)
        for src, cand in candidates:
            st, hs, _top = _perf_selftime(
                harness_bin, op, cand,
                _probe_iters(harness_bin, op, cand, hdir, pin),
                hdir, pin)
            # Rank candidates by LIBRARY share, not by in-process share: the
            # latter cannot tell "the library is hot" from "the harness's own
            # digest code is hot", and would happily pick an input that only
            # exercises the scaffolding.
            score = max(0.0, st - hs) if st >= 0 else st
            if best is None or score > best[0]:
                best = (score, src, cand)
        _self0, r.input_source, inp = best
        r.input_path = str(inp)
        r.input_bytes = inp.stat().st_size

        iters, per_iter = _autotune_iters(harness_bin, op, inp, hdir,
                                          target_wall, pin)
        if iters == 0:
            r.note = "op errored on perf input"
            results.append(r)
            continue
        r.iters = iters

        r.instructions, r.cycles, r.wall_s = _perf_stat(
            harness_bin, op, inp, iters, hdir, pin)
        r.self_time_own, r.harness_share, r.top = _perf_selftime(
            harness_bin, op, inp, iters, hdir, pin)
        # Refine the flat estimate with call-graph + inline attribution, which
        # is the only way to tell an inlined library function from the harness
        # function it was inlined into. Falls back to the flat number if the
        # call-graph profile cannot be taken.
        precise = _harness_share_precise(
            harness_bin, op, inp, iters, hdir, pin, harness_crate,
            crate_under_test_name(hdir))
        if precise is not None:
            r.harness_share = precise
        else:
            # Flat symbol attribution cannot see through inlining, so any
            # library share it reports is a lower bound, not a measurement.
            r.attribution_reliable = False
        r.ok = r.self_time_own >= SELF_TIME_GATE
        if not r.ok and r.self_time_own >= 0:
            r.note = (f"self_time_own {r.self_time_own:.0%} < {SELF_TIME_GATE:.0%}"
                      f" — dominated by libc/setup; not a clean perf target"
                      f" (input {r.input_bytes}B may be too small)")
        # Independent of the libc/kernel gate: is the LIBRARY actually hot? An op
        # can sit at 99% self_time_own and still measure nothing but the harness's
        # own per-iteration digest work.
        if r.self_time_own >= 0 and r.crate_share < CRATE_SHARE_GATE:
            # Index, not tuple-unpack: rows carry a DSO now, and a
            # perf_workload.json written before that does not.
            first = r.top[0] if r.top else ()
            hot = first[1] if len(first) > 1 else ""
            hot_dso = first[2] if len(first) > 2 else ""
            r.harness_bound = True if r.attribution_reliable else None
            detail = (f"crate_share {r.crate_share:.0%} < {CRATE_SHARE_GATE:.0%}"
                      f" (harness itself {r.harness_share:.0%}"
                      f"{f'; hottest symbol {hot}' if hot else ''})"
                      f" at iters={r.iters}, input {r.input_bytes}B — "
                      + _low_crate_share_cause(r, hot, harness_crate,
                                              hot_dso))
            r.note = f"{r.note}; {detail}" if r.note else detail
        verdict = ("UNKNOWN-ATTRIBUTION" if r.harness_bound is None
                   else "HARNESS-BOUND" if r.harness_bound
                   else ("OK" if r.ok else "LOW"))
        logger.info(
            "[perf] %-22s iters=%d wall=%.2fs IC=%.2e self=%.0f%% crate=%.0f%% %s",
            op, iters, r.wall_s, r.instructions, 100 * r.self_time_own,
            100 * r.crate_share, verdict)
        if r.harness_bound is not False:
            logger.warning("[perf] ⚠ %s %s: %s", op, verdict, detail)
        results.append(r)
    return results
