"""`perf annotate` wrapper — locates the hot *region* inside a hot fn.

perf_tree_design.md §2.5.6 step 2 + step 5: once `perf report` has named a
hot fn, `perf annotate` gives per-instruction sample weights *inside* that
fn. From those weights we extract:

  - hot_region        — the hottest contiguous instruction window (a loop /
                        block), reported as an address range
  - hot_region_share  — that window's share of the fn's self-time

The same machinery, run against an `instructions:u` perf.data, yields the
hot region's instruction-sample share for the IC axis (§2.5.4).

This module only parses `perf annotate --stdio`; address→source-line and
mnemonic classification live in `asm_tells.py` (which owns objdump).

Failure handling mirrors the sibling perf wrappers: any failure returns
`ok=False` with a reason — the pipeline degrades, never aborts.
"""

from __future__ import annotations

import logging
import re
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Data shapes
# ---------------------------------------------------------------------------

@dataclass
class AnnotatedInsn:
    """One disassembled instruction with its sample weight."""
    addr: int            # instruction address
    pct:  float          # sample % (local to the fn, "percent: local period")
    text: str            # raw instruction text from perf annotate


@dataclass
class AnnotateResult:
    ok:        bool
    insns:     list[AnnotatedInsn] = field(default_factory=list)
    symbol:    str = ""
    error:     str = ""

    def __bool__(self) -> bool:
        return self.ok

    @property
    def total_pct(self) -> float:
        """Summed sample % over the fn — ~100 for local-period, the fn's
        global share for global-period."""
        return sum(i.pct for i in self.insns)


@dataclass
class HotRegion:
    """The hottest contiguous instruction window inside a fn."""
    lo_addr:    int                       # first instruction address
    hi_addr:    int                       # last instruction address
    region_pct: float                     # summed sample % inside the window
    fn_pct:     float                     # summed sample % over the whole fn
    insns:      list[AnnotatedInsn] = field(default_factory=list)

    @property
    def share(self) -> float:
        """Region's share of the fn's self-time (0.0–1.0)."""
        return (self.region_pct / self.fn_pct) if self.fn_pct > 0 else 0.0

    @property
    def asm_range(self) -> str:
        """`0x..-0x..` — the form stored in HotRegion.asm_range (§2.5.1)."""
        return f"0x{self.lo_addr:x}-0x{self.hi_addr:x}"


# ---------------------------------------------------------------------------
# perf annotate invocation
# ---------------------------------------------------------------------------

def _perf_available() -> bool:
    if shutil.which("perf") is None:
        return False
    try:
        subprocess.run(["perf", "--version"], capture_output=True, timeout=5, check=True)
        return True
    except (subprocess.SubprocessError, OSError):
        return False


def run_perf_annotate(perf_data: Path, symbol: str,
                      *, percent_type: str = "local",
                      timeout_s: int = 180) -> AnnotateResult:
    """Run `perf annotate --stdio` for one symbol; return its instructions.

    Args:
      perf_data    : path to a perf.data (cycles or instructions:u)
      symbol       : symbol name exactly as `perf report` printed it
      percent_type : "local"  — % within the fn (default; sums to ~100)
                     "global" — % of all samples in the run
      timeout_s    : subprocess cap

    The `--symbol=` filter restricts annotation to the one fn; when perf
    still emits sibling blocks we keep only the block whose header names
    our symbol (by bare last-`::`-segment match)."""
    perf_data = Path(perf_data).resolve()
    if not perf_data.is_file():
        return AnnotateResult(ok=False, symbol=symbol,
                              error=f"perf.data not found: {perf_data}")
    if not _perf_available():
        return AnnotateResult(ok=False, symbol=symbol,
                              error="perf binary not on PATH")
    if percent_type not in ("local", "global"):
        return AnnotateResult(ok=False, symbol=symbol,
                              error=f"invalid percent_type={percent_type!r}")

    # `--percent-type {local,global}-period` controls whether the Percent
    # column is fn-local or run-global. `--stdio` keeps output plain text.
    cmd = [
        "perf", "annotate", "--stdio",
        "-i", str(perf_data),
        "--percent-type", f"{percent_type}-period",
        f"--symbol={symbol}",
    ]
    logger.info(f"[annotate] perf annotate {perf_data.name} "
                f"--percent-type={percent_type}-period --symbol={symbol!r}")

    try:
        proc = subprocess.run(cmd, capture_output=True, text=True,
                              errors="replace", timeout=timeout_s)
    except subprocess.TimeoutExpired:
        return AnnotateResult(ok=False, symbol=symbol,
                              error=f"perf annotate timed out after {timeout_s}s")
    except OSError as e:
        return AnnotateResult(ok=False, symbol=symbol,
                              error=f"perf annotate failed to launch: {e}")

    if proc.returncode != 0:
        return AnnotateResult(ok=False, symbol=symbol,
                              error=f"perf annotate exit={proc.returncode}: "
                                    f"{(proc.stderr or '').strip()[:200]}")

    insns = parse_annotate(proc.stdout, symbol)
    if not insns:
        return AnnotateResult(ok=False, symbol=symbol,
                              error="no annotated instructions parsed "
                                    "(symbol not in perf.data, or stripped build)")
    return AnnotateResult(ok=True, insns=insns, symbol=symbol)


# An annotated asm line in `perf annotate --stdio`:
#       12.34 :   401140:  mov    (%rsi),%eax
#        0.00 :   401131:  mov    %rsp,%rbp
#             :   401130:  push   %rbp            (no samples → blank percent)
# Source-interleaved lines look like `        :   560   while ... {` — the
# token after `:` is a *decimal* line number NOT followed by `:`, so the
# `<hex>:` requirement below rejects them.
_INSN_RE = re.compile(
    r"^\s*(?P<pct>\d+\.\d+)?\s*:\s+"
    r"(?P<addr>[0-9a-fA-F]+):\s+"
    r"(?P<insn>\S.*?)\s*$"
)
# Symbol block header: `0000000000401130 <BZ2_blockSort>:`
_SYM_HEADER_RE = re.compile(r"^\s*[0-9a-fA-F]+\s+<(?P<name>[^>]+)>:\s*$")


def parse_annotate(stdout: str, symbol: str) -> list[AnnotatedInsn]:
    """Parse `perf annotate --stdio` output → instructions for `symbol`.

    perf may emit more than one `<name>:` block (the `--symbol` filter is a
    substring match). We keep only the block(s) whose header bare name
    equals our symbol's bare name; if no header is seen at all (some perf
    versions omit it under `--symbol`), we accept every parsed insn."""
    bare = symbol.rsplit("::", 1)[-1]
    out: list[AnnotatedInsn] = []
    in_block = True          # accept until the first header tells us otherwise
    saw_header = False

    for line in stdout.splitlines():
        h = _SYM_HEADER_RE.match(line)
        if h:
            saw_header = True
            hname = h.group("name")
            hbare = hname.rsplit("::", 1)[-1]
            in_block = (bare in hname) or (hbare == bare) or (hname in symbol)
            continue
        if not in_block:
            continue
        m = _INSN_RE.match(line)
        if not m:
            continue
        try:
            addr = int(m.group("addr"), 16)
        except ValueError:
            continue
        pct = float(m.group("pct")) if m.group("pct") else 0.0
        out.append(AnnotatedInsn(addr=addr, pct=pct, text=m.group("insn")))

    # If headers existed but our block matched nothing, the symbol filter
    # caught a different fn — report empty so the caller degrades honestly.
    if saw_header and not out:
        logger.warning(f"[annotate] no block matched symbol {symbol!r}")
    out.sort(key=lambda i: i.addr)
    return out


# ---------------------------------------------------------------------------
# Hot region clustering
# ---------------------------------------------------------------------------

def find_hot_region(insns: list[AnnotatedInsn],
                    *, target_coverage: float = 0.80) -> HotRegion | None:
    """Cluster samples → the hottest contiguous instruction window (§2.5.1).

    Strategy: the *minimal-length* contiguous window (by instruction index)
    whose summed sample % reaches `target_coverage` of the fn's total. A hot
    loop is exactly such a tight cluster; a flat fn yields a near-whole-fn
    window with share ≈ 1.0 — both are reported honestly.

    Returns None when there are no samples at all (dead/cold fn)."""
    if not insns:
        return None
    fn_pct = sum(i.pct for i in insns)
    if fn_pct <= 0.0:
        return None

    need = target_coverage * fn_pct
    n = len(insns)

    # Min-window with sum ≥ need — two pointers (all pct ≥ 0 ⇒ monotone).
    best: tuple[int, int, int, float] | None = None   # (length, lo_i, hi_i, sum)
    lo = 0
    run = 0.0
    for hi in range(n):
        run += insns[hi].pct
        while run - insns[lo].pct >= need and lo < hi:
            run -= insns[lo].pct
            lo += 1
        if run >= need:
            length = hi - lo
            cand = (length, lo, hi, run)
            # prefer shorter window; tie → larger sum
            if best is None or (length, -run) < (best[0], -best[3]):
                best = cand

    if best is None:
        # No window reaches target_coverage (shouldn't happen — full window
        # always does) — fall back to the whole fn.
        lo_i, hi_i, region_pct = 0, n - 1, fn_pct
    else:
        _, lo_i, hi_i, region_pct = best

    window = insns[lo_i:hi_i + 1]
    return HotRegion(
        lo_addr    = window[0].addr,
        hi_addr    = window[-1].addr,
        region_pct = region_pct,
        fn_pct     = fn_pct,
        insns      = window,
    )
