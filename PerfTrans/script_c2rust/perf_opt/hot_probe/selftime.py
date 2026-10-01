"""Deepest-crate-frame self-time attribution.

Symbol-level self-time (`perf report --sort symbol`) fails on two common
shapes: (a) a crate kernel small enough that LTO inlines it into the harness op
wrapper — its time lands on a harness symbol; (b) a crate function that
delegates to libc (e.g. `strcasechr` → `strpbrk`) — its time lands on a libc
symbol. Both hide the crate function that is actually the optimization target.

Instead we sample WITH a call graph (`--call-graph dwarf`) and expand inline
frames (`--inline`), then attribute each sample to the DEEPEST crate frame on
its stack:

    op_fuzzy_match(harness) → has_match(crate,inlined) → strcasechr(crate,inlined) → strpbrk(libc)
                                                          ^^^^^^^^^^ deepest crate frame ⇒ gets the sample

This yields per-crate-function self-time that is robust to harness-inlining and
libc delegation, while still separating crate→crate calls (the deepest crate
frame, not the entry). Requires the binary to carry debug info (the driver
builds the perf-opt copy with `debug = 2`; this does not change codegen, so W2
timing is unaffected). For a big non-inlined kernel the deepest crate frame IS
the kernel symbol, so this is a strict superset of symbol self-time.
"""

from __future__ import annotations

import logging
import re
import subprocess
from pathlib import Path

from harness_gen.hotness_probe import own_base
from perf_opt.verify.measure import resolve_pin_cpu

logger = logging.getLogger("hot_probe.selftime")

RECORD_TIMEOUT = 600
# perf-script header: `<comm> <pid> <time>: <period> <event>:`
_HEADER_RE = re.compile(r"^\S.*:\s+(\d+)\s+\S+:\s*$")
# frame: `\t<hexip> <symbol>[+0xoff] (<dso>)` — capture the symbol.
_FRAME_RE = re.compile(r"^\s+[0-9a-fA-F]+\s+(.*?)\s+\([^)]*\)\s*$")


def _fold_deepest_crate(script_out: str, crate_name: str, exported: set[str],
                        resolvable: set[str] | None = None) -> dict[str, float]:
    """Parse `perf script --inline` (one stack per SAMPLE, leaf first) and
    attribute each sample's period to its deepest crate frame. Returns
    {crate_fn: self%} — % of total sampled time (all periods).

    `resolvable` (the source-editable fn names): when given, prefer the deepest
    crate frame that HAS a source definition — a macro-generated accessor (e.g.
    a `#[bitfield]` getter, no source `fn`) is skipped so its time rolls up to
    the editable caller it was inlined into. Falls back to the deepest crate
    frame if the stack has no editable crate frame (never drops the sample)."""
    fn_period: dict[str, float] = {}
    total = 0.0
    frames: list[str] = []          # current sample, leaf first
    period = 0.0

    def _flush() -> None:
        nonlocal total
        if not frames:
            return
        total += period
        fallback: str | None = None
        for sym in frames:          # leaf → root
            base = own_base(sym, crate_name, exported)
            if base is None:
                continue
            if fallback is None:
                fallback = base       # deepest crate frame (editable or not)
            if resolvable is None or base in resolvable:
                fn_period[base] = fn_period.get(base, 0.0) + period
                return
        if fallback is not None:      # no editable crate frame → deepest crate frame
            fn_period[fallback] = fn_period.get(fallback, 0.0) + period

    for line in script_out.splitlines():
        h = _HEADER_RE.match(line)
        if h:
            _flush()
            frames, period = [], float(h.group(1))
            continue
        f = _FRAME_RE.match(line)
        if f:
            sym = re.sub(r"\+0x[0-9a-fA-F]+$", "", f.group(1).strip())
            frames.append(sym)
    _flush()

    if total <= 0:
        return {}
    return {fn: 100.0 * p / total for fn, p in fn_period.items()}


def crate_event_shares(harness_bin: Path, op: str, inp: Path, iters: int,
                       scratch: Path, crate_name: str, exported: set[str], *,
                       event: str = "cycles",
                       resolvable: set[str] | None = None,
                       pin_cpu: int | None = None) -> dict[str, float]:
    """`perf record -e <event> --call-graph dwarf` the harness on one op, then
    fold each SAMPLE to its deepest crate frame (inline-expanded). Returns
    {crate_fn: share%} — the fn's % of total `event` (cycles → self-time%,
    instructions:u → dynamic-work share, …). A sample whose whole stack is
    non-crate (libc/harness/std/kernel) contributes to the denominator only."""
    cpu = resolve_pin_cpu(pin_cpu)
    pin = ["taskset", "-c", str(cpu)] if cpu >= 0 else []
    data = scratch / f".cg_{op}.data"

    rec = subprocess.run(
        [*pin, "perf", "record", "-e", event, "--call-graph", "dwarf",
         "-F", "999", "-o", str(data), "--",
         str(harness_bin), op, str(inp), str(iters)],
        cwd=str(scratch), stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        timeout=RECORD_TIMEOUT)
    if rec.returncode != 0 or not data.exists():
        return {}
    script = subprocess.run(
        ["perf", "script", "-i", str(data), "--inline"],
        cwd=str(scratch), capture_output=True, text=True, timeout=RECORD_TIMEOUT)
    data.unlink(missing_ok=True)
    return _fold_deepest_crate(script.stdout, crate_name, exported, resolvable)


def crate_selftime(harness_bin: Path, op: str, inp: Path, iters: int,
                   scratch: Path, crate_name: str, exported: set[str], *,
                   resolvable: set[str] | None = None,
                   pin_cpu: int | None = None) -> dict[str, float]:
    """Per-crate-function self-time% (cycles). Thin wrapper over
    `crate_event_shares` with event=cycles."""
    return crate_event_shares(harness_bin, op, inp, iters, scratch,
                              crate_name, exported, event="cycles",
                              resolvable=resolvable, pin_cpu=pin_cpu)
