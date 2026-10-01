""                                                                       

                                                                          
                                                                      
                                                                      
                                        

                                                               

                                                                          
                                                                            
                                                                           
                                                                           
                                                               
                                                                        

                                                                         
                                                                     

                                                                      
                                                                        
                                                
   

from __future__ import annotations

import logging
import os
import re
import shutil
import subprocess
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class TmaResult:
    """One TMA measurement. Percentages are 0–100; None ⇒ not measured."""
    ok:                  bool
    tma_retiring:        float | None = None
    tma_frontend_bound:  float | None = None
    tma_bad_speculation: float | None = None
    tma_backend_bound:   float | None = None
    tma_path:            str          = ""        # dominant-node drill-down (toplev -l3)
    source:              str          = "unavailable"   # toplev|perf-topdown|unavailable
    error:               str          = ""

    def __bool__(self) -> bool:
        return self.ok


# ---------------------------------------------------------------------------
# toplev discovery
# ---------------------------------------------------------------------------

def find_toplev() -> list[str] | None:
    """Locate a runnable toplev; return the command prefix or None."""
    env = os.environ.get("C2RUST_TOPLEV")
    if env and Path(env).is_file():
        return ["python3", env] if env.endswith(".py") else [env]
    for name in ("toplev", "toplev.py"):
        p = shutil.which(name)
        if p:
            return ["python3", p] if p.endswith(".py") else [p]
    home_guess = Path.home() / "pmu-tools" / "toplev.py"
    if home_guess.is_file():
        return ["python3", str(home_guess)]
    return None


def _perf_available() -> bool:
    if shutil.which("perf") is None:
        return False
    try:
        subprocess.run(["perf", "--version"], capture_output=True, timeout=5, check=True)
        return True
    except (subprocess.SubprocessError, OSError):
        return False


# ---------------------------------------------------------------------------
# Public entry
# ---------------------------------------------------------------------------

def run_tma(binary: Path, args: list[str], cwd: Path | None = None,
            *, timeout_s: int = 300) -> TmaResult:
    """Measure TMA L1 for `binary args`, trying tools in priority order."""
    from profiling.workload import _resolve_rand
    args = _resolve_rand(args)

    binary = Path(binary).resolve()
    if not binary.is_file():
        return TmaResult(ok=False, error=f"binary not found: {binary}")

    toplev = find_toplev()
    if toplev is not None:
        res = _run_toplev(toplev, binary, args, cwd, timeout_s)
        if res.ok:
            return res
        logger.info(f"[tma] toplev failed ({res.error}); falling back to perf")

    if _perf_available():
        res = _run_perf_topdown(binary, args, cwd, timeout_s)
        if res.ok:
            return res
        res = _run_perf_metricgroup(binary, args, cwd, timeout_s)
        if res.ok:
            return res

    return TmaResult(ok=False, source="unavailable",
                     error="no TMA tool available (no toplev; perf --topdown "
                           "and -M TopdownL1 both unsupported on this host)")


# ---------------------------------------------------------------------------
# Strategy 1 — toplev
# ---------------------------------------------------------------------------

# toplev `-x ,` CSV row: <obj>,<area>,<value>,<unit>,...  We key on the
# metric/area name; L1 nodes are Retiring / Bad_Speculation / Frontend_Bound
# / Backend_Bound (toplev spells them with underscores or spaces).
_L1_KEYS = {
    "retiring":        "tma_retiring",
    "bad_speculation": "tma_bad_speculation",
    "bad speculation": "tma_bad_speculation",
    "frontend_bound":  "tma_frontend_bound",
    "frontend bound":  "tma_frontend_bound",
    "backend_bound":   "tma_backend_bound",
    "backend bound":   "tma_backend_bound",
}


def _run_toplev(toplev: list[str], binary: Path, args: list[str],
                cwd: Path | None, timeout_s: int) -> TmaResult:
    """Run `toplev -l3 -v -x , -- binary args`; parse the CSV node table.

    `-v` ("verbose") forces toplev to emit *every* node, including those
    below the default ~5% threshold — otherwise low-but-real categories
    like Retiring at 20% can still be missing because the L1 default
    threshold filters them out (verified: without -v we only got
    Backend_Bound on bzip2, the other 3 L1 nodes were silently dropped)."""
    cmd = [*toplev, "-l3", "-v", "--no-desc", "-x", ",", "--",
           str(binary), *args]
    logger.info(f"[tma] toplev: {' '.join(cmd[:5])} ...")
    try:
        proc = subprocess.run(cmd, cwd=str(cwd) if cwd else None,
                              stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                              text=True, errors="replace", timeout=timeout_s)
    except subprocess.TimeoutExpired:
        return TmaResult(ok=False, source="toplev",
                         error=f"toplev timed out after {timeout_s}s")
    except OSError as e:
        return TmaResult(ok=False, source="toplev",
                         error=f"toplev failed to launch: {e}")

    nodes = _parse_toplev_csv(proc.stderr)
    if not nodes:
        return TmaResult(ok=False, source="toplev",
                         error=f"toplev produced no parseable nodes "
                               f"(exit={proc.returncode})")

    l1 = {field: pct for name, (field, lvl, pct) in _l1_view(nodes).items()}
    if not any(k.startswith("tma_") for k in l1):
        return TmaResult(ok=False, source="toplev",
                         error="toplev gave no L1 nodes")

    return TmaResult(
        ok                  = True,
        source              = "toplev",
        tma_retiring        = l1.get("tma_retiring"),
        tma_frontend_bound  = l1.get("tma_frontend_bound"),
        tma_bad_speculation = l1.get("tma_bad_speculation"),
        tma_backend_bound   = l1.get("tma_backend_bound"),
        tma_path            = _toplev_path(nodes),
    )


# CSV: object,nodename,value,unit,...   value may be `12.3` or `12.3%`.
# toplev emits one row per CPU/core when measuring system-wide (default
# with `-- cmd`), so the same metric name recurs N times — accumulate and
# average instead of letting the last row win.
_TOPLEV_ROW = re.compile(
    r"^[^,]*,\s*(?P<name>[^,]+?)\s*,\s*(?P<val>[\d.]+)\s*%?\s*,\s*(?P<unit>[^,]*)"
)


def _parse_toplev_csv(text: str) -> dict[str, tuple[float, str]]:
    """Parse toplev `-x ,` output → {node_name: (mean_value_pct, unit)}.

    Per-metric values from per-core rows are averaged. Only rows whose
    unit names a percentage are kept (toplev also emits raw-count metric
    rows we don't want)."""
    accum: dict[str, list[float]] = {}
    unit_of: dict[str, str] = {}
    for line in text.splitlines():
        m = _TOPLEV_ROW.match(line.strip())
        if not m:
            continue
        unit = m.group("unit").strip()
        if "%" not in unit and "%" not in line:
            continue
        try:
            val = float(m.group("val"))
        except ValueError:
            continue
        name = m.group("name").strip()
        accum.setdefault(name, []).append(val)
        unit_of[name] = unit
    return {name: (sum(vs) / len(vs), unit_of[name]) for name, vs in accum.items()}


def _l1_view(nodes: dict[str, tuple[float, str]]
             ) -> dict[str, tuple[str, int, float]]:
    """Pick the four L1 TMA nodes out of a toplev node table."""
    out: dict[str, tuple[str, int, float]] = {}
    for name, (val, _unit) in nodes.items():
        key = name.strip().lower().replace("-", "_")
        field = _L1_KEYS.get(key)
        if field and field not in {v[0] for v in out.values()}:
            out[name] = (field, 1, val)
    return out


def _toplev_path(nodes: dict[str, tuple[float, str]]) -> str:
    """`L1 > L1.L2 > L1.L2.L3` drill-down of the dominant TMA branch.

    toplev encodes hierarchy in node names by `.` (e.g.
    `Backend_Bound.Memory_Bound.DRAM_Bound`). We walk: pick the L1 with
    the highest value, then its highest-valued L2 child (names starting
    with `<L1>.`), then the highest-valued L3 child of that L2. A pure
    "top 3 by value" sort silently jumps branches and is misleading."""
    l1_nodes = {n: v for n, (v, _u) in nodes.items() if "." not in n}
    if not l1_nodes:
        return ""
    l1 = max(l1_nodes, key=l1_nodes.get)
    path = [(l1, l1_nodes[l1])]
    cur = l1
    for depth in (1, 2):                       # depth = number of dots wanted
        children = {n: v for n, (v, _u) in nodes.items()
                    if n.startswith(cur + ".") and n.count(".") == depth}
        if not children:
            break
        child = max(children, key=children.get)
        path.append((child, children[child]))
        cur = child
    return " > ".join(f"{n} {v:.1f}%" for n, v in path if v > 0)


# ---------------------------------------------------------------------------
# Strategy 2 — perf stat --topdown
# ---------------------------------------------------------------------------

def _run_perf_topdown(binary: Path, args: list[str], cwd: Path | None,
                      timeout_s: int) -> TmaResult:
    """Run `perf stat --topdown`; parse the 4-category percentage row."""
    cmd = ["perf", "stat", "--topdown", "--", str(binary), *args]
    logger.info(f"[tma] perf stat --topdown ...")
    try:
        proc = subprocess.run(cmd, cwd=str(cwd) if cwd else None,
                              stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                              text=True, errors="replace", timeout=timeout_s)
    except (subprocess.TimeoutExpired, OSError) as e:
        return TmaResult(ok=False, source="perf-topdown",
                         error=f"perf stat --topdown failed: {e}")

    vals = _parse_perf_topdown(proc.stderr)
    if vals is None:
        return TmaResult(ok=False, source="perf-topdown",
                         error="perf --topdown output not parseable "
                               "(likely unsupported on this CPU/perf)")
    ret, badspec, fe, be = vals
    return TmaResult(ok=True, source="perf-topdown",
                     tma_retiring=ret, tma_bad_speculation=badspec,
                     tma_frontend_bound=fe, tma_backend_bound=be)


def _parse_perf_topdown(text: str) -> tuple[float, float, float, float] | None:
    """`perf stat --topdown` prints the 4 categories in a fixed order
    (retiring, bad speculation, frontend bound, backend bound). Find the
    line carrying exactly four `NN.N%` tokens."""
    for line in text.splitlines():
        pcts = re.findall(r"(\d+\.\d+)\s*%", line)
        if len(pcts) == 4:
            r, b, f, k = (float(x) for x in pcts)
            return r, b, f, k
    return None


# ---------------------------------------------------------------------------
# Strategy 3 — perf stat -M TopdownL1
# ---------------------------------------------------------------------------

_TMA_METRIC_RE = re.compile(
    r"(\d+\.\d+)\s*%\s*(tma_)?(?P<name>retiring|frontend_bound|"
    r"bad_speculation|backend_bound)",
    re.IGNORECASE,
)


def _run_perf_metricgroup(binary: Path, args: list[str], cwd: Path | None,
                          timeout_s: int) -> TmaResult:
    """Run `perf stat -M TopdownL1`; parse the tma_* metric lines."""
    cmd = ["perf", "stat", "-M", "TopdownL1", "--", str(binary), *args]
    logger.info(f"[tma] perf stat -M TopdownL1 ...")
    try:
        proc = subprocess.run(cmd, cwd=str(cwd) if cwd else None,
                              stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                              text=True, errors="replace", timeout=timeout_s)
    except (subprocess.TimeoutExpired, OSError) as e:
        return TmaResult(ok=False, source="perf-topdown",
                         error=f"perf stat -M TopdownL1 failed: {e}")

    found: dict[str, float] = {}
    for line in proc.stderr.splitlines():
        m = _TMA_METRIC_RE.search(line)
        if m:
            found[f"tma_{m.group('name').lower()}"] = float(m.group(1))
    if not found:
        return TmaResult(ok=False, source="perf-topdown",
                         error="perf -M TopdownL1 metric group unavailable")
    return TmaResult(ok=True, source="perf-topdown",
                     tma_retiring=found.get("tma_retiring"),
                     tma_frontend_bound=found.get("tma_frontend_bound"),
                     tma_bad_speculation=found.get("tma_bad_speculation"),
                     tma_backend_bound=found.get("tma_backend_bound"))
