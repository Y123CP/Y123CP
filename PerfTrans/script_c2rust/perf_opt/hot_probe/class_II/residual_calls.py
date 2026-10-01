""                                                                

                                                              
                                                                           
                                                             

                                                                          
                                                                         
                                                                       
                                                      

                                                        
                                                          

                                                                        
                                                                        
                                                        
   

from __future__ import annotations

import logging
import re
from dataclasses import dataclass
from pathlib import Path

# Single source of truth for the C1 panic-runtime callee set. Importing
# from class_I.rules keeps residual_calls.skip and class_I C1 detection in
# lockstep — a callee that IS C1 must be skipped from residual counting,
# else it double-counts (fires both C1 and II_inl gate).
from perf_opt.hot_probe.class_I.rules import _C1_CALLEE_PATTERNS

logger = logging.getLogger("hot_probe.class_II.residual_calls")


# The `define` line captures the mangled fn name; we track "am I inside a
# define block right now" state through the linear IR scan.
_RE_DEFINE = re.compile(r"^define\s+[^@]*@(?P<mangled>[\w.]+)\s*\(")

# One matched call line — captures the callee spelling. `%<reg>` = indirect,
# `@<name>` = direct.
_RE_CALL_DIRECT = re.compile(
    r"(?:\btail\s+)?(?:call|invoke)\s+[^@]*?@(?P<callee>[\w.]+)\s*\("
)
_RE_CALL_INDIRECT = re.compile(
    r"(?:\btail\s+)?(?:call|invoke)\s+[^%]*?%(?P<reg>[\w.]+)\s*\("
)

# Skip set = LLVM intrinsics + the FULL C1 panic-runtime union (imported).
# Any callee that's a C1 instance is NOT a residual call — C1 handles it
# separately, and counting it here inflates II_inl's residual gate.
_SKIP_CALLEE_PATTERNS: tuple[re.Pattern, ...] = (
    re.compile(r"^llvm\."),
    # C1 patterns from class_I.rules — kept in lockstep by construction:
    *(re.compile(re.escape(p)) for p in _C1_CALLEE_PATTERNS),
)


def _skip_callee(callee: str) -> bool:
    return any(p.search(callee) for p in _SKIP_CALLEE_PATTERNS)


@dataclass
class ResidualCallCounts:
    """{fn_mangled: (direct_count, indirect_count)}. Values > 0 satisfy
    the II_inl residual call gate."""
    direct: dict[str, int]
    indirect: dict[str, int]

    def total(self, fn_mangled: str) -> int:
        return self.direct.get(fn_mangled, 0) + self.indirect.get(fn_mangled, 0)

    def has_residual(self, fn_mangled: str) -> bool:
        return self.total(fn_mangled) > 0

    def has_residual_for_source_fn(self, fn_name: str) -> bool:
        """True iff any mangled key encoding `fn_name` (with Rust's
        `<len><name>` mangling convention) has residual calls.

        Uses **length-prefix matching** (`<len><fn_name>` substring), NOT
        bare-name substring: for short names like `add` (3 chars) or
        `filter` (6 chars), bare substring inflates matches — e.g. `add`
        would match `lodepng_add_itext_size`, `addr2line::render_file`,
        etc. The length-prefix (`3add`, `6filter`) is present in every
        Rust mangled symbol at the exact position of that fn name, so the
        match is precise."""
        needle = f"{len(fn_name)}{fn_name}"
        for m in list(self.direct.keys()) + list(self.indirect.keys()):
            if needle in m and self.has_residual(m):
                return True
        return False


def count_residual_calls(ir_path: Path) -> ResidualCallCounts:
    """One linear pass over the .ll file, tallying residual call sites per
    enclosing `define` fn. See module docstring for the counting rules.
    """
    ir_path = Path(ir_path)
    direct: dict[str, int] = {}
    indirect: dict[str, int] = {}
    current: str = ""

    with ir_path.open("r", encoding="utf-8", errors="replace") as fh:
        for line in fh:
            if line.startswith("define"):
                m = _RE_DEFINE.match(line)
                if m is not None:
                    current = m.group("mangled")
                continue
            if line.startswith("}"):
                current = ""
                continue
            if not current:
                continue

            stripped = line.lstrip()
            if stripped.startswith(";"):
                continue
            if "call " not in line and "invoke " not in line:
                continue

            m = _RE_CALL_DIRECT.search(line)
            if m is not None:
                callee = m.group("callee")
                if not _skip_callee(callee):
                    direct[current] = direct.get(current, 0) + 1
                continue

            m = _RE_CALL_INDIRECT.search(line)
            if m is not None:
                indirect[current] = indirect.get(current, 0) + 1

    logger.info("[residual_calls] %s: %d fn(s) w/ direct residual, "
                "%d fn(s) w/ indirect residual",
                ir_path.name,
                sum(1 for v in direct.values() if v > 0),
                sum(1 for v in indirect.values() if v > 0))
    return ResidualCallCounts(direct=direct, indirect=indirect)
