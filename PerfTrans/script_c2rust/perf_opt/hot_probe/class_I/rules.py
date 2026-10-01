""                                                                       
                                                                           
                   

                               

                                                         
                                                                  
                                                               
                                                                         

                                                           
                                                            
                                                              

                                                                     
                                                                      
                                                                    
                                                         
                                                                    
                                                                      
                                   

                                                                      
                                                                      
                                                              
                                                                 
                                                                     
                                                             
                                                                       
                                                                    
                                 

                                                                    
                                                                       
   

from __future__ import annotations

import logging
import re
from pathlib import Path

from perf_opt.hot_probe.class_I.attribute import Site

logger = logging.getLogger("hot_probe.class_I.rules")


# ── callee-name matchers (v1) ────────────────────────────────────────────────
#
# We match on CALLEE NAME substring (mangled symbols), extracted from
# `call ... @<callee>(` lines in the IR. Substring matches are stable across
# rustc nightlies (`_ZN..panic_bounds_check17h..E` — `panic_bounds_check`
# is the substring; hash suffix changes).

# ── C1 callee union (same-principle members; see module docstring) ──────────
#
# Every member satisfies (i) Rust-semantics-obligated, (ii) 'cond-branch +
# cold panic block' IR shape, (iii) 'prove precondition → _unchecked'
# rewrite family. Grouped by syntactic source, not by empirical/extension
# tier — the rule's ≥3-hot-fn admission is inherited by every member.
# Order: most-frequent first (short-circuits earlier for perf).
_C1_CALLEE_PATTERNS: tuple[str, ...] = (
    # ── index / slice-range checks (source: arr[i], slice[a..b]) ──
    "panic_bounds_check",             # arr[i] out of bounds
    "slice_start_index_len_fail",     # slice[a..] with a > len
    "slice_end_index_len_fail",       # slice[..b] with b > len
    "slice_index_order_fail",         # slice[a..b] with a > b
    "slice_end_index_overflow_fail",  # arithmetic overflow in slice bounds
    "slice_error_fail",               # str slice bounds (str::slice_error_fail / _rt)
    # ── Option/Result state checks (source: .expect / .unwrap) ──
    "expect_failed",                  # Option::expect / Result::expect
    "unwrap_failed",                  # Option::unwrap / Result::unwrap
    # ── arithmetic-semantics checks (source: /, %, RefCell::borrow) ──
    "panic_const_div_by_zero",        # a / b with b proven zero
    "panic_already_borrowed",         # RefCell borrow-state
)

# Explicitly EXCLUDED — these fail one of the three admission conditions:
#   * generic panic runtime symbols catch cleanup/fmt/unwind/assert paths
#     that have no `_unchecked` rewrite family (fails (iii))
#   * assert_failed is user-code intent, not a c2rust artifact (fails (i))
                                                    
_C1_EXCLUDED_PATTERNS: tuple[str, ...] = (
    "panic_in_cleanup",               # cleanup path, not user code
    "panic_fmt",                      # panic! formatting
    "panic_cannot_unwind",            # unwind machinery
    "panic_nounwind",                 # nounwind machinery
    "assert_failed",                  # generic assert (would over-match)
    "__rust_panic_cleanup",           # runtime cleanup helper
)


def _matches_C1(callee: str) -> bool:
    ""                                                  
                                                                    
                                                                               
    if any(bad in callee for bad in _C1_EXCLUDED_PATTERNS):
        return False
    return any(good in callee for good in _C1_CALLEE_PATTERNS)


def _matches_C2(callee: str) -> bool:
    ""             
                                                                                
       
    return (callee.startswith("llvm.fptosi.sat")
            or callee.startswith("llvm.fptoui.sat"))


# ── IR line patterns ────────────────────────────────────────────────────────

_RE_CALL_LINE = re.compile(
    r"(?:\btail\s+)?(?:call|invoke)\s+[^@]*?@(?P<callee>[\w.]+)\s*\("
)
_RE_DBG_ID = re.compile(r"!dbg\s+!(?P<id>\d+)")
_RE_DEFINE = re.compile(r"^define\s+[^@]*@(?P<mangled>[\w.]+)\s*\(")

# ── C12: oversized eager zero-initialization ───────────────────────────────
#
#   P_C12(s) ≜ hot(s) ∧ optIR(s) ⊇ { llvm.memset.*(p, i8 0, i64 N, ...) }
#              with N a CONSTANT ≥ C12_MIN_BYTES
#
# What it catches. C leaves a large local uninitialized — `struct S s;`,
# `score_t D[MAX];` — and writes only the prefix a runtime length selects.
# c2rust cannot express "uninitialized" and emits a full zero literal
                                                                         
# so it cannot prove the whole buffer is overwritten. The zeroing survives to
# the binary as a memset the C original never runs.
#
# Why the size must be CONSTANT. A non-constant length (`i64 %n`) IS the
# runtime extent the program actually needs — that is the shape we would be
# rewriting TOWARD, not away from. Only a compile-time constant can be
# "the whole array when a prefix would do".
#
# Why the fill byte must be 0. `i8 -1` and friends are a deliberate sentinel
# fill (`vec![usize::MAX; n]`), semantically load-bearing, not translation
# slack.
#
# The floor. Below a few cache lines LLVM widens the store instead of calling
# memset, and there is no call to see; 256 B = 4 lines is where the call
# appears and where the win starts being measurable. It is a reporting floor,
# NOT a claim the buffer is shrinkable — whether only a prefix is read is a
# semantic question left to the rewrite stage, with W1/W2 as the arbiter.
C12_MIN_BYTES = 256
_RE_MEMSET_ZERO_CONST = re.compile(r"\bi8 0,\s*i64 (?P<bytes>\d+)\b")


def _matches_C12(callee: str, line: str) -> int | None:
    """Constant byte count when `line` is an oversized zero memset, else None."""
    if "llvm.memset" not in callee:
        return None
    m = _RE_MEMSET_ZERO_CONST.search(line)
    if m is None:
        return None                      # non-constant size, or fill byte != 0
    n = int(m.group("bytes"))
    return n if n >= C12_MIN_BYTES else None


# ── scanner ─────────────────────────────────────────────────────────────────

def scan_ir_for_c1_c2(ir_path: Path) -> list[Site]:
    """One linear scan over the .ll file, emitting one `Site` per matched
    `call`/`invoke` line for C1 or C2. Attribute via `class_I.attribute`.

    Site.rule_id is "C1" or "C2"; Site.dbg_id is the `!<N>` DILocation
    (0 if the instruction had no !dbg — attribution falls back to the
    enclosing `define`). Site.define_mangled is the enclosing `define`.
    """
    ir_path = Path(ir_path)
    sites: list[Site] = []
    n_lines = 0
    n_call_lines = 0
    n_no_dbg = 0
    current_define = ""

    with ir_path.open("r", encoding="utf-8", errors="replace") as fh:
        for line in fh:
            n_lines += 1
            stripped = line.lstrip()

            if line.startswith("define"):
                m = _RE_DEFINE.match(line)
                if m is not None:
                    current_define = m.group("mangled")
                continue
            if line.startswith("}"):
                current_define = ""
                continue
            if "call " not in line and "invoke " not in line:
                continue
            if stripped.startswith(";"):
                continue

            n_call_lines += 1
            m = _RE_CALL_LINE.search(line)
            if m is None:
                continue
            callee = m.group("callee")

            rid: str | None = None
            detail = ""
            if _matches_C1(callee):
                rid = "C1"
            elif _matches_C2(callee):
                rid = "C2"
            else:
                zeroed = _matches_C12(callee, line)
                if zeroed is not None:
                    rid, detail = "C12", str(zeroed)
            if rid is None:
                continue

            mdbg = _RE_DBG_ID.search(line)
            if mdbg is None:
                n_no_dbg += 1
                sites.append(Site(rule_id=rid, dbg_id=0, callee=callee,
                                  define_mangled=current_define,
                                  detail=detail))
                continue
            sites.append(Site(rule_id=rid, dbg_id=int(mdbg.group("id")),
                              callee=callee, define_mangled=current_define,
                              detail=detail))

    by_rule: dict[str, int] = {"C1": 0, "C2": 0, "C12": 0}
    for s in sites:
        by_rule[s.rule_id] = by_rule.get(s.rule_id, 0) + 1
    logger.info("[class_I.rules] %s: %d lines / %d call-shaped / "
                "%d matches: %s (no-dbg: %d)",
                ir_path.name, n_lines, n_call_lines, len(sites),
                by_rule, n_no_dbg)
    return sites
