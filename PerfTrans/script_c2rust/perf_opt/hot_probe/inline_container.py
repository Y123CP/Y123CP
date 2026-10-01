"""II_iso detector: hot loop kernels fully inlined into a much larger function.

A loop kernel inlined into a big container is scheduled and register-allocated
together with all of it. Measured twice on the same mechanism: a PNG codec's
Adam7 deinterlace loop inlined into its ~9 KB decoder ran 12% faster once given
a function of its own (`#[inline(never)]`), and an LZ4-HC match finder inlined
into a 40 KB driver spilled registers in its chain-walk loop. Neither case is
visible in the source; both are visible in the binary's debug info, which
records every inlined instance and the concrete function it landed in.

This module only NOMINATES. Isolation can also cost what inlining bought
(constant propagation into the loop), so W2 decides per candidate. The
criteria keep the nominations few:

  * the function is a loop kernel — `fn_type == "algorithm_hot"` with at least
    one compute loop (small leaf helpers are meant to be inlined);
  * it has no out-of-line copy at all (fully inlined);
  * its largest container is a LIBRARY function (not harness/driver code) at
    least `min_ratio` times the size of the inlined instance;
  * at most `cap` per project, hottest first.
"""

from __future__ import annotations

import logging
import re
import shutil
import subprocess
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable

logger = logging.getLogger(__name__)

_TAG_RE = re.compile(r"^0x[0-9a-f]+:(\s+)(DW_TAG_\w+|NULL)")
_RANGE_RE = re.compile(r"\[(0x[0-9a-f]+), (0x[0-9a-f]+)\)")
_LOOPS_RE = re.compile(r"(\d+) compute loops?")


def _dwarfdump() -> str | None:
    for name in ("llvm-dwarfdump-17", "llvm-dwarfdump"):
        path = shutil.which(name)
        if path:
            return path
    candidate = Path("/usr/lib/llvm-17/bin/llvm-dwarfdump")
    return str(candidate) if candidate.exists() else None


def _hf(hf: Any, name: str, default=None):
    return hf.get(name, default) if isinstance(hf, dict) else getattr(hf, name, default)


def inlined_instances(lines: Iterable[str], targets: set[str]):
    """Walk an `llvm-dwarfdump --debug-info` listing.

    Returns ``(instances, own)``: ``instances[fn] = [(container_name,
    container_linkage, container_bytes, inlined_bytes), ...]`` for each target
    inlined anywhere, and ``own[fn] = bytes`` for each target that also has a
    concrete out-of-line definition.
    """
    stack: list[tuple[int, dict]] = []
    instances: dict[str, list[tuple[str, str, int, int]]] = defaultdict(list)
    own: dict[str, int] = {}
    matchers = {t: re.compile(r"\d" + re.escape(t) + r"17h[0-9a-f]{16}E$") for t in targets}

    def container():
        for _, info in reversed(stack):
            if info["tag"] == "DW_TAG_subprogram" and info["size"]:
                return info
        return None

    def finish(info):
        if info is None:
            return
        if info["tag"] == "DW_TAG_inlined_subroutine" and info["origin"]:
            origin = info["origin"]
            for t, rx in matchers.items():
                if origin == t or rx.search(origin):
                    c = info["container"]
                    if c is not None:
                        instances[t].append((c["name"] or "?", c["link"] or "",
                                             c["size"], info["size"]))
                    break
        elif (info["tag"] == "DW_TAG_subprogram" and info["name"] in targets
              and info["size"]):
            own[info["name"]] = info["size"]

    cur = None
    in_ranges = False
    for line in lines:
        m = _TAG_RE.match(line)
        if m:
            finish(cur)
            in_ranges = False
            indent, tag = len(m.group(1)), m.group(2)
            while stack and stack[-1][0] >= indent:
                stack.pop()
            if tag == "NULL":
                cur = None
                continue
            cur = {"tag": tag, "low": None, "size": 0, "name": None, "link": "",
                   "origin": None, "container": container()}
            stack.append((indent, cur))
            continue
        if cur is None:
            continue
        mm = re.search(r"DW_AT_low_pc\s+\((0x[0-9a-f]+)\)", line)
        if mm:
            cur["low"] = int(mm.group(1), 16)
        mm = re.search(r"DW_AT_high_pc\s+\((0x[0-9a-f]+)\)", line)
        if mm:
            high, low = int(mm.group(1), 16), cur["low"] or 0
            cur["size"] = high - low if high > low else high
        if "DW_AT_ranges" in line:
            in_ranges = True
        if in_ranges:
            for a, b in _RANGE_RE.findall(line):
                cur["size"] += int(b, 16) - int(a, 16)
            if line.rstrip().endswith(")") and "DW_AT_ranges" not in line:
                in_ranges = False
        mm = re.search(r'DW_AT_name\s+\("([^"]+)"\)', line)
        if mm:
            cur["name"] = mm.group(1)
        mm = re.search(r'DW_AT_linkage_name\s+\("([^"]+)"\)', line)
        if mm:
            cur["link"] = mm.group(1)
        mm = re.search(r'DW_AT_abstract_origin\s+\(0x[0-9a-f]+ "([^"]+)"\)', line)
        if mm:
            cur["origin"] = mm.group(1)
    finish(cur)
    return instances, own


def isolation_candidates(instances, own, hot_fns: Iterable[Any], lib_crate: str,
                         *, min_ratio: float = 2.0, cap: int = 3,
                         min_container_bytes: int = 4096,
                         min_inlined_bytes: int = 128) -> list[dict]:
    """Apply the nomination criteria; one record per nominated hot function.

    Sizes are per container: a function inlined into one container at several
    call sites, or split into several address ranges, counts all of it there.
    The two floors keep the rule on its mechanism — register pressure in a
    LARGE function acting on a loop body of real size. Without them a dry run
    over twelve projects nominated 6-35 byte fragments inside 1.3 KB
    containers, which no isolation can help. They are heuristics, not fitted
    values; W2 decides every nomination.
    """
    lib_segment = f"{len(lib_crate)}{lib_crate}" if lib_crate else ""
    out = []
    for hf in hot_fns:
        name = (_hf(hf, "name") or "").split("::")[-1]
        if _hf(hf, "fn_type") != "algorithm_hot":
            continue
        loops = _LOOPS_RE.search(_hf(hf, "fn_type_reason") or "")
        if not loops or int(loops.group(1)) < 1:
            continue
        if name in own or not instances.get(name):
            continue
        per_container: dict[tuple[str, int], int] = defaultdict(int)
        for cname, clink, csize, isize in instances[name]:
            if lib_segment and lib_segment in clink:
                per_container[(cname, csize)] += isize
        if not per_container:
            continue
        (cname, csize), isize = max(per_container.items(), key=lambda kv: kv[0][1])
        if (csize < min_container_bytes or isize < min_inlined_bytes
                or csize < min_ratio * isize):
            continue
        out.append({
            "fn": name, "self_pct": float(_hf(hf, "self_pct") or 0.0),
            "container": cname, "container_bytes": csize, "inlined_bytes": isize,
            "file": _hf(hf, "file") or "", "line": int(_hf(hf, "line_start") or 0),
        })
    out.sort(key=lambda r: -r["self_pct"])
    return out[:cap]


def find_isolation_candidates(binary: Path, hot_fns: list[Any], lib_crate: str,
                              *, min_ratio: float = 2.0, cap: int = 3) -> list[dict]:
    """Run the detector on a debug-info binary. Empty (and logged) when the
    tool or the debug info is unavailable — the rule then simply does not fire."""
    tool = _dwarfdump()
    if tool is None:
        logger.warning("[II_iso] llvm-dwarfdump not found; isolation detector skipped")
        return []
    targets = {(_hf(hf, "name") or "").split("::")[-1] for hf in hot_fns}
    targets.discard("")
    try:
        proc = subprocess.Popen([tool, "--debug-info", str(binary)],
                                stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                                text=True, errors="replace")
        assert proc.stdout is not None
        instances, own = inlined_instances(proc.stdout, targets)
        proc.wait(timeout=600)
    except (OSError, subprocess.SubprocessError) as exc:
        logger.warning("[II_iso] dwarfdump failed on %s: %s", binary, exc)
        return []
    found = isolation_candidates(instances, own, hot_fns, lib_crate,
                                 min_ratio=min_ratio, cap=cap)
    logger.info("[II_iso] %d isolation candidate(s): %s", len(found),
                [(r["fn"], r["container"], r["container_bytes"], r["inlined_bytes"])
                 for r in found])
    return found
