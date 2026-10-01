""                                                                       

                                                                         
                                                                          
                 

                                                                     
                                                                        
                                                                          
                                                                           
                                                                 

                                                                   
                                           

                                                                                
                                                                                   

                                                        
   

from __future__ import annotations

import logging
import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

from profiling.evidence import AsmTell, HotSubregion, RetiringMix
from profiling.perf_annotate import AnnotatedInsn

logger = logging.getLogger(__name__)


# ---------------------------------------------------------------------------
# Instruction decoding helpers
# ---------------------------------------------------------------------------

def _mnemonic(text: str) -> str:
    """First token of a disassembly line, lowercased (the opcode)."""
    t = text.strip()
    return t.split()[0].lower() if t else ""


def _operands(text: str) -> str:
    """Everything after the mnemonic (before any `# comment`)."""
    t = text.strip()
    parts = t.split(None, 1)
    ops = parts[1] if len(parts) > 1 else ""
    # perf annotate appends `# 6e7b0 <name>` comments — drop them.
    return ops.split("#", 1)[0].strip()


def _split_operands(ops: str) -> list[str]:
    """Split a comma-separated operand list, ignoring commas inside `()`."""
    out, depth, cur = [], 0, ""
    for ch in ops:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth = max(0, depth - 1)
        if ch == "," and depth == 0:
            out.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


# Canonicalize a register to its 64-bit name so %eax/%ax/%al ≡ %rax and
# %r8d/%r8w/%r8b ≡ %r8 — needed to track a value across width-changing ops.
_SUBREG = {
    "eax": "rax", "ax": "rax", "al": "rax", "ah": "rax",
    "ebx": "rbx", "bx": "rbx", "bl": "rbx", "bh": "rbx",
    "ecx": "rcx", "cx": "rcx", "cl": "rcx", "ch": "rcx",
    "edx": "rdx", "dx": "rdx", "dl": "rdx", "dh": "rdx",
    "esi": "rsi", "si": "rsi", "sil": "rsi",
    "edi": "rdi", "di": "rdi", "dil": "rdi",
    "ebp": "rbp", "bp": "rbp", "bpl": "rbp",
    "esp": "rsp", "sp": "rsp", "spl": "rsp",
}


def _canon_reg(reg: str) -> str:
    """`%eax` / `%r8d` → `rax` / `r8`. Non-registers pass through stripped."""
    r = reg.strip().lstrip("%").lower()
    if r in _SUBREG:
        return _SUBREG[r]
    m = re.fullmatch(r"r(\d{1,2})[dwb]?", r)
    if m:
        return f"r{m.group(1)}"
    return r


# A memory operand: `disp(base,index,scale)` / `(base)` / `disp(%rip)`.
_MEM_OPERAND = re.compile(r"-?\w*\([^)]*\)")
# Index register inside a scaled memory operand: `(base,%index,scale)`.
_INDEX_REG = re.compile(r"\([^,()]*,\s*(%\w+)\s*,\s*[0-9]+\s*\)")

_BRANCH_MNEMONICS = {
    "jmp", "jmpq", "je", "jne", "jz", "jnz", "ja", "jae", "jb", "jbe",
    "jg", "jge", "jl", "jle", "js", "jns", "jo", "jno", "jp", "jnp",
    "jc", "jnc", "call", "callq", "ret", "retq", "loop", "loopne",
}
_CALL_MNEMONICS = {"call", "callq"}
_LOAD_MNEMONICS = {"mov", "movq", "movl", "movw", "movb",
                   "movzbl", "movzbw", "movzbq", "movzwl", "movzwq",
                   "movsbl", "movsbw", "movsbq", "movswl", "movswq", "movslq",
                   "movss", "movsd", "movups", "movupd", "movaps", "movdqu"}
# `(base,index,scale)` syntax that does NOT dereference memory: `lea`
# computes an address, multi-byte `nop` pads — neither is a gather.
_NON_DEREF = {"lea", "leaq", "nop", "nopl", "nopw", "nopq"}

# Step B Bug 3 — byte-assembly detection state-machine mnemonics.
# True u32::from_be_bytes pattern is `movzbl × 4 + (shl × 3 + or × 3)`. The
# detector below requires at least 3 byte-loads from the SAME base register
# with disps within an 8-byte window AND at least 1 shift/or in the same
# run, then emits the entire run as one cluster. The old `≥3 mnemonic run`
# heuristic fired on stray `movzbl 0x610(%rsp),%ebx` (a stack reload).
_BYTE_LOAD_MNEMONICS = {"movzbl", "movzbw", "movzbq", "movzwl", "movzwq"}
_BYTE_COMBINE_MNEMONICS = {
    "shl", "shlq", "shll", "shlb", "shlw",
    "shr", "shrq", "shrl", "shrb", "shrw",
    "sal", "salq", "sall",
    "or",  "orq",  "orl",  "orb",  "orw",
}

# Disp + base for `disp(%base)` (no index). Used by byte-assembly to track
# (base, disp) of each byte load and verify they share base + cluster disps.
_DISP_BASE_RE = re.compile(r"^(-?(?:0[xX][0-9a-fA-F]+|\d*))\(%(\w+)\)$")
# Base register of a scaled-index mem operand `disp(%base,%idx,scale)`.
_SI_BASE_RE = re.compile(r"-?\w*\(%(\w+)\s*,\s*%\w+\s*,\s*[0-9]+\s*\)")
# Scalar FP arithmetic: ends in `ss`/`sd`, optional AVX `v` — but NOT a move.
_SCALAR_FP_ARITH = re.compile(r"^v?(add|sub|mul|div|min|max|sqrt|cvt|"
                              r"ucomi|comi|rcp|rsqrt)[a-z]*s[sd]$")
# Vector / SIMD: AVX `v...`, packed `...ps`/`...pd`, packed-int `p...`.
_VECTOR_RE = re.compile(r"^(v[a-z].*|[a-z]+p[sd]|p(add|sub|mul|cmp|and|or|xor|"
                        r"shuf|unpck|mov|min|max)[a-z]*)$")
_BOUNDS_SYMS = ("panic_bounds_check", "slice_index", "slice_end_index",
                "slice_start_index", "panic_misaligned")
# Vector FP arithmetic only — packed (ps/pd) add/sub/mul/div/min/max/sqrt
# + the FMA family. Excludes vector moves (vmov*) and integer SIMD (pxor
# etc., which the §2.7.6 enum slots into BITWISE_PATTERN, not VECTOR_FP).
_VECTOR_FP_ARITH = re.compile(
    r"^v?(add|sub|mul|div|min|max|sqrt|cvt|rcp|rsqrt)[a-z]*p[sd]$"
    r"|^vf(n?m(add|sub))[a-z0-9]*p[sd]$"
)


# ---------------------------------------------------------------------------
# asm_tells — discrete pattern scan (§2.5.5)
# ---------------------------------------------------------------------------

# Closed enum from perf_tree_design.md §2.7.6. New tags require updating
# §2.5 AND §2.7 in lockstep — do NOT add ad-hoc tags here.
_PATTERN_AXIS = {
    # call class
    "EXPLICIT_CALL":    "both",
    "INDIRECT_CALL":    "both",
    "ALLOC_CALL":       "IC",
    "PURE_SHORT_CALL":  "IC",       # M1: detection stubbed (needs IR-level analysis)
    # mem class
    "RAW_PTR_OFFSET":   "IC",
    "BYTE_ASSEMBLY":    "IC",
    "BOUNDS_CHECK":     "IC",
    "STRIDED_GATHER":   "CPI",
    # arith class
    "SCALAR_FP":        "IC",
    "VECTOR_FP":        "IC",
    "BITWISE_PATTERN":  "IC",
    # control class
    "TIGHT_LOOP":       "IC",
    "COLD_BLOCK":       "IC",        # implemented in `detect_cold_blocks` (called
                                     # from pipeline.py after scan_asm_tells; gates A2)
}


# COLD_BLOCK detection thresholds (per spec §2.5 / A2 trigger).
# A basic block is cold when its sample share over the fn total is ≤ this,
# AND it's reachable from the hot region via a conditional branch.
_COLD_BLOCK_SHARE_MAX  = 0.01     # 1% of fn cumulative sample weight
_COLD_BLOCK_MIN_INSNS  = 8        # skip trampolines / single-insn padding
# Conditional jumps only — `jmp` (unconditional) is fall-through, doesn't
# count as a hot→cold dispatch; `call` is for callee invocation, not
# intra-fn cold-path routing.
_COND_BRANCH_MNEMONICS = {
    "je", "jne", "jz", "jnz", "ja", "jae", "jb", "jbe",
    "jg", "jge", "jl", "jle", "js", "jns", "jo", "jno",
    "jp", "jnp", "jc", "jnc",
}
# Any of these terminates a basic block in the linear scan.
_BB_TERMINATOR_MNEMONICS = (
    _COND_BRANCH_MNEMONICS
    | {"jmp", "jmpq", "call", "callq", "ret", "retq"}
)

# ALLOC_CALL: callsite target ∈ this set. Matched substring-wise on the
# call's operand (perf annotate prints `# <addr> <symbol>` as suffix).
_ALLOC_SYMS = ("malloc", "calloc", "realloc", "free",
               "__rust_alloc", "__rust_dealloc", "__rust_realloc",
               "Box::new", "Vec::with_capacity", "Vec::new",
               "String::with_capacity", "String::new")

# BITWISE_PATTERN: a single instruction's mnemonic counts toward the 40%
# floor. (Bitwise ops on GPRs in c2rust hash/checksum/encoding code.)
_BITWISE_MNEMONICS = {"shl", "shlq", "shll", "shlb", "shlw",
                       "shr", "shrq", "shrl", "shrb", "shrw",
                       "sar", "sarq", "sarl",
                       "sal", "salq", "sall",
                       "xor", "xorq", "xorl", "xorb", "xorw",
                       "and", "andq", "andl", "andb", "andw",
                       "or",  "orq",  "orl",  "orb",  "orw",
                       "rol", "rolq", "roll", "ror", "rorq", "rorl"}


def _dest_reg(ops_list: list[str]) -> str | None:
    """The destination register of an AT&T insn — the last operand, when
    it is a bare register (`%reg`)."""
    if not ops_list:
        return None
    last = ops_list[-1].strip()
    if last.startswith("%") and "(" not in last:
        return _canon_reg(last)
    return None


# ---------------------------------------------------------------------------
# Step B (2026-06-01) — helpers for the refined RAW_PTR_OFFSET /
# STRIDED_GATHER / BYTE_ASSEMBLY classifiers
# ---------------------------------------------------------------------------

def _scaled_index_base(op_text: str) -> str | None:
    """Return the canonical base register of a scaled-index mem operand
    `disp(%base,%idx,scale)`, or None if absent."""
    m = _SI_BASE_RE.search(op_text)
    return _canon_reg(f"%{m.group(1)}") if m else None


def _is_stack_base(base_reg: str | None) -> bool:
    """%rsp / %rbp accesses are spilled locals / local arrays, not heap
    pointer chasing. Step B Bug 4 filter — keep RAW_PTR_OFFSET semantics
    tight to actual c2rust raw-pointer loops."""
    return base_reg in ("rsp", "rbp")


def _is_pure_store(mn: str, ops_list: list[str]) -> bool:
    """True iff this insn writes memory and does NOT also read memory.

    Pattern: `mov %src_reg, dst_mem` (or similar mov-class store). Used by
    Step B Bug 2 to keep STRIDED_GATHER on LOADs only — gather is by
    definition an indirect READ; the symmetric write is "scatter", a
    different perf concern."""
    if mn not in _LOAD_MNEMONICS:
        # cmp / test / add / arithmetic with mem always READ their mem operand
        return False
    if len(ops_list) < 2:
        return False
    src, dst = ops_list[0], ops_list[-1]
    src_is_mem = bool(_MEM_OPERAND.search(src))
    dst_is_mem = bool(_MEM_OPERAND.search(dst))
    return dst_is_mem and not src_is_mem


def _parse_disp_base(op_text: str) -> tuple[str, int] | None:
    """Parse `disp(%base)` (no index) — return (canon_base, disp) or None.

    Handles negative + hex displacements. For Step B Bug 3 byte-assembly
    detection, where we track which byte loads share a base register and
    cluster within an 8-byte disp window."""
    m = _DISP_BASE_RE.match(op_text.strip())
    if not m:
        return None
    raw_disp = m.group(1)
    try:
        disp = int(raw_disp, 0) if raw_disp else 0
    except ValueError:
        return None
    base = _canon_reg(f"%{m.group(2)}")
    return (base, disp)


def scan_asm_tells(insns: list[AnnotatedInsn],
                    *, subregion_lookup=None) -> list[AsmTell]:
    """Scan hot-region instructions → one AsmTell per pattern that occurs.

    Classifier rules (refined in Step B — see commentary in each block):

    * EXPLICIT_CALL / INDIRECT_CALL fire **only on call insns** — `ret`
      is a return-from-call, not a call, and the retiring_mix already
      buckets ret under `call_ret` (Step A) so a duplicate "call" signal
      via ret leaks 0 information and inflates count.

    * RAW_PTR_OFFSET fires on scaled-index mem operands whose base is
      NOT `%rsp` / `%rbp` — stack-frame addressing is spilled locals,
      not c2rust raw-pointer chasing.

    * STRIDED_GATHER is the subset of RAW_PTR_OFFSET that is a true
      indirect LOAD (`arr[idx[i]]`): the indexed mem operand is a SOURCE
      (not a destination), and the index register was recently mem-loaded.
      Writes to `(base,idx,scale)` are scatters, not gathers.

    * BYTE_ASSEMBLY clusters into a single tell only when ≥3 byte loads
      from `disp(%base)` share the same base + disps within 8 bytes, AND
      ≥1 shift/or appears in the same run. Catches the
      `u32::from_be_bytes`-style idiom; rejects stray stack reloads."""
    hits: dict[str, list[AnnotatedInsn]] = {}

    def add(pattern: str, insn: AnnotatedInsn) -> None:
        hits.setdefault(pattern, []).append(insn)

    mem_tainted: set[str] = set()        # regs currently holding a mem-loaded value

    # Step B Bug 3 — byte-assembly streaming state. A "run" is a sequence
    # of byte loads + shifts/ors broken by anything else (call, branch
    # out, store, alloc, non-byte-load mov, etc.). At each break point we
    # call _commit_byte_run() to decide whether the run qualifies.
    ba_loads:     list[tuple[str, int]]    = []  # (base, disp) of byte loads
    ba_combines:  list[int]                = [0]  # mutable boxed counter
    ba_window:    list[AnnotatedInsn]      = []  # all insns in the active run

    def _commit_byte_run() -> None:
        """If the current byte-assembly run meets the criteria
        (≥3 byte loads from the same base, disps within 8 B, ≥1 shift/or),
        emit the whole run as BYTE_ASSEMBLY; always reset state."""
        try:
            if len(ba_loads) < 3 or ba_combines[0] < 1:
                return
            from collections import Counter
            cnt = Counter(b for b, _ in ba_loads)
            top_base, top_count = cnt.most_common(1)[0]
            if top_count < 3:
                return
            disps = sorted({d for b, d in ba_loads if b == top_base})
            if len(disps) < 3 or (disps[-1] - disps[0]) > 8:
                return
            for w_ins in ba_window:
                add("BYTE_ASSEMBLY", w_ins)
        finally:
            ba_loads.clear()
            ba_combines[0] = 0
            ba_window.clear()

    for ins in insns:
        mn = _mnemonic(ins.text)
        ops = _operands(ins.text)
        if not mn:
            continue
        ops_list = _split_operands(ops)

        # --- calls -------------------------------------------------------
        if mn in _CALL_MNEMONICS:
            star = ops.startswith("*")
            # Operand carries the call target name (perf annotate suffixes
            # `# <addr> <symbol>` to the line) — substring-match for ALLOC.
            is_alloc = any(s in ins.text for s in _ALLOC_SYMS)
            if star and "%rip" not in ops:
                add("INDIRECT_CALL", ins)       # call *%reg / *disp(%reg) — fn pointer
            else:
                add("EXPLICIT_CALL", ins)       # direct, or *(%rip) GOT call
            if is_alloc:
                add("ALLOC_CALL", ins)          # subset signal, fires alongside EXPLICIT/INDIRECT
            mem_tainted.clear()                 # a call clobbers caller-saved regs
            _commit_byte_run()
            continue
        # Step B Bug 1 — `ret` is NOT a call. Used to add EXPLICIT_CALL
        # here, inflated the count by every return insn. Removed.
        if mn in ("ret", "retq"):
            mem_tainted.clear()                 # ret clobbers context too
            _commit_byte_run()
            continue

        # --- bounds check (panic symbol referenced) ----------------------
        if any(s in ins.text for s in _BOUNDS_SYMS):
            add("BOUNDS_CHECK", ins)

        # --- scalar / vector floating-point arithmetic -------------------
        if _SCALAR_FP_ARITH.match(mn):
            add("SCALAR_FP", ins)
        if _VECTOR_FP_ARITH.match(mn):
            add("VECTOR_FP", ins)

        # --- raw-pointer offset access (Bug 2 + Bug 4 refined) ----------
        # Iterate operands to find a scaled-index mem `disp(%base,%idx,N)`.
        # Bug 4: filter out stack-frame bases (%rsp/%rbp) — those are
        # spilled locals, not raw-pointer chasing.
        # Bug 2: STRIDED_GATHER fires only when the scaled-index mem is
        # a SOURCE (read), not a destination (store-style scatter).
        si_op_idx = -1                          # which operand carries it
        for i, op in enumerate(ops_list):
            if _INDEX_REG.search(op):
                si_op_idx = i
                break
        if si_op_idx >= 0 and mn not in _NON_DEREF:
            si_base = _scaled_index_base(ops_list[si_op_idx])
            if si_base and not _is_stack_base(si_base):
                add("RAW_PTR_OFFSET", ins)
                # STRIDED_GATHER subset: indexed mem is read AND index reg
                # was just mem-loaded. AT&T source position is operand 0
                # for two-operand mov/cmp/add — but cmp/test/arithmetic
                # also read mem in pos 1. So fire whenever the insn is
                # NOT a pure store.
                if not _is_pure_store(mn, ops_list):
                    idx_m = _INDEX_REG.search(ops_list[si_op_idx])
                    if idx_m and _canon_reg(idx_m.group(1)) in mem_tainted:
                        add("STRIDED_GATHER", ins)

        # --- byte-assembly idiom (Bug 3 rewritten) -----------------------
        # State machine:
        #   movzbl <disp>(%base) → push (base, disp), insn → ba_window
        #   shl / shr / or / sal — if a run is active, increment combines
        #   anything else (incl. complex-source byte load) — commit the run
        is_byte_load = mn in _BYTE_LOAD_MNEMONICS
        is_combine   = mn in _BYTE_COMBINE_MNEMONICS

        if is_byte_load:
            db = _parse_disp_base(ops_list[0]) if ops_list else None
            if db is not None:
                ba_loads.append(db)
                ba_window.append(ins)
            else:
                # Byte load with non-`disp(%base)` source (e.g. scaled
                # index) — too complex to attribute, break the run.
                _commit_byte_run()
        elif is_combine and ba_window:
            ba_combines[0] += 1
            ba_window.append(ins)
        else:
            # Anything else breaks the current run.
            _commit_byte_run()

        # --- update memory taint -----------------------------------------
        dst = _dest_reg(ops_list)
        if dst is not None:
            src_is_mem = bool(ops_list) and bool(_MEM_OPERAND.search(ops_list[0]))
            if mn in _LOAD_MNEMONICS and src_is_mem:
                mem_tainted.add(dst)            # dst now holds a mem-loaded value
            else:
                mem_tainted.discard(dst)        # dst recomputed — no longer a direct load

    # End-of-region: commit any trailing byte-assembly run.
    _commit_byte_run()

    # --- region-level tags (post-pass) -------------------------------------
    # BITWISE_PATTERN: ≥ 40% of region insns are shift/xor/and/or (per §2.7.6)
    if insns:
        bw = sum(1 for i in insns if _mnemonic(i.text) in _BITWISE_MNEMONICS)
        if bw / len(insns) >= 0.40:
            rep = max((i for i in insns if _mnemonic(i.text) in _BITWISE_MNEMONICS),
                      key=lambda i: i.pct, default=insns[0])
            hits.setdefault("BITWISE_PATTERN", []).append(rep)

    # TIGHT_LOOP: short region (< 20 insns) AND a backward branch within
    # the region (backedge to an earlier addr inside the region range).
    if 0 < len(insns) < 20:
        lo_addr, hi_addr = insns[0].addr, insns[-1].addr
        has_backedge = False
        for i in insns:
            mn = _mnemonic(i.text)
            if mn in _BRANCH_MNEMONICS and mn not in _CALL_MNEMONICS \
                    and mn not in ("ret", "retq"):
                # Branch target: first hex literal in operands.
                m = re.search(r"\b([0-9a-fA-F]{3,})\b", _operands(i.text))
                if m:
                    try:
                        tgt = int(m.group(1), 16)
                        if lo_addr <= tgt < i.addr:
                            has_backedge = True
                            break
                    except ValueError:
                        pass
        if has_backedge:
            rep = max(insns, key=lambda i: i.pct)
            hits.setdefault("TIGHT_LOOP", []).append(rep)

    # PURE_SHORT_CALL / COLD_BLOCK: need IR / per-block sample counts.
    # M1 stub: never emit. Real impl in M2/M3.

    # Step E: when `subregion_lookup` is provided, partition each
    # pattern's hits by subregion (using per-occurrence addresses), so
    # dispatch sees per-(pattern, subregion) tells instead of a single
    # bounding-box aggregate per pattern. Without lookup, behavior is
    # the original: one AsmTell per pattern over the whole region.
    #
    # Phase F: also compute `sample_weight` per tell — the share of the
    # region's total instruction-sample weight that the tell carries
    # (0-100). Lets dispatch rank candidate sites within a subregion by
    # weight, not just by occurrence count (a 1-occurrence hit that took
    # 30% of cycles beats a 10-occurrence hit at 1% each).
    total_region_weight = sum(i.pct for i in insns) if insns else 0.0
    tells: list[AsmTell] = []
    for pattern, group in hits.items():
        if subregion_lookup is None:
            buckets: dict[int, list[AnnotatedInsn]] = {-1: list(group)}
        else:
            buckets = {}
            for ins in group:
                idx = subregion_lookup(ins.addr)
                buckets.setdefault(idx, []).append(ins)
        for sub_idx, bucket in buckets.items():
            rep = max(bucket, key=lambda i: i.pct)
            lo = min(i.addr for i in bucket)
            hi = max(i.addr for i in bucket)
            loc = f"0x{lo:x}" if lo == hi else f"0x{lo:x}-0x{hi:x}"
            bucket_weight = sum(i.pct for i in bucket)
            sample_weight = (bucket_weight / total_region_weight * 100.0
                             if total_region_weight > 0 else 0.0)
            tells.append(AsmTell(
                pattern       = pattern,
                axis          = _PATTERN_AXIS.get(pattern, "both"),
                count         = len(bucket),
                loc           = loc,
                snippet       = rep.text.strip(),
                sample_weight = round(sample_weight, 2),
                subregion_idx = sub_idx,
            ))
    tells.sort(key=lambda t: (-t.sample_weight, -t.count, t.pattern, t.subregion_idx))
    return tells


# ---------------------------------------------------------------------------
# retiring_mix — instruction-type breakdown (§2.5.4)
# ---------------------------------------------------------------------------

# Step A: `branch` covers conditional jumps only; `call_ret` is its own
                                                               
_CALL_RET_MNEMONICS = {"call", "callq", "ret", "retq"}


def _bucket(text: str) -> str:
    """Bucket one instruction into a retiring_mix category.

    Priority: call/ret > branch > (vector/scalar FP arithmetic) > mem > int.
    A data *move* (`mov`/`movsd`/…) is never FP-arithmetic — a
    `movsd (mem),%xmm` is a memory load and buckets as `mem`."""
    mn = _mnemonic(text)
    if not mn:
        return "int"
    if mn in _CALL_RET_MNEMONICS:
        return "call_ret"
    if mn in _BRANCH_MNEMONICS:
        return "branch"
    is_move = mn.startswith("mov") or mn in ("lea", "leaq", "push", "pop")
    if not is_move:
        if _VECTOR_RE.match(mn):
            return "vector_fp"
        if _SCALAR_FP_ARITH.match(mn):
            return "scalar_fp"
    if mn not in ("lea", "leaq", "nop") and _MEM_OPERAND.search(_operands(text)):
        return "mem"
    return "int"


def classify_retiring_mix(insns: list[AnnotatedInsn]) -> RetiringMix:
    """Bucket every instruction, weight by sample %, normalize to ~100.

    Unit (Step A — 2026-06-01): emits on the **0-100** scale (sum ≈ 100),
    matching dispatch thresholds in stage_b_opt_design.md (e.g.
    `int + scalar_fp ≥ 60%`). Was 0-1.

    With no sample weight anywhere (cold region) every instruction is
    weighted uniformly so the mix still reflects the static shape."""
    acc = {"scalar_fp": 0.0, "vector_fp": 0.0, "int": 0.0,
           "branch": 0.0, "mem": 0.0, "call_ret": 0.0}
    if not insns:
        return RetiringMix()

    use_uniform = sum(i.pct for i in insns) <= 0.0
    for ins in insns:
        acc[_bucket(ins.text)] += 1.0 if use_uniform else ins.pct

    denom = sum(acc.values())
    if denom <= 0.0:
        return RetiringMix()
    # `* 100 / denom` so the 6 fractions sum to ~100 (not 1).
    return RetiringMix(
        scalar_fp = acc["scalar_fp"] * 100.0 / denom,
        vector_fp = acc["vector_fp"] * 100.0 / denom,
        int       = acc["int"]       * 100.0 / denom,
        branch    = acc["branch"]    * 100.0 / denom,
        mem       = acc["mem"]       * 100.0 / denom,
        call_ret  = acc["call_ret"]  * 100.0 / denom,
    )


# ---------------------------------------------------------------------------
# addr2line — address → source mapping (DWARF, design §2.5: no Rust parsing)
# ---------------------------------------------------------------------------

@dataclass
class SrcSpan:
    """A source-line range mapped from an address range."""
    file:    str
    line_lo: int
    line_hi: int

    def as_lines(self) -> str:
        """`L-L'` form for HotRegion.src_lines (§2.5.1)."""
        if self.line_lo == self.line_hi:
            return str(self.line_lo)
        return f"{self.line_lo}-{self.line_hi}"


@dataclass
class RegionSrc:
    """A hot region's resolved source location."""
    span:        SrcSpan | None
    inlined:     bool          # True ⇒ hot instructions include inlined callee code
    inlined_fns: list[str]     # bare names of all fns appearing in the region's
                               # inline stacks (outermost = hot symbol;
                               # inner frames = LTO-inlined callees)


# addr2line `-a` echoes each address as `0x...`, delimiting per-addr blocks;
# `-f -i -C` prints, per inline frame, a (demangled) function line followed
# by a `file:line` line — innermost frame first, outermost (the real
# containing fn) last.
_A2L_ADDR = re.compile(r"^0x[0-9a-fA-F]+\s*$")
_A2L_SRC  = re.compile(r"^(?P<file>.+):(?P<line>\d+)(?:\s+\(discriminator \d+\))?\s*$")


def _run_addr2line(binary: Path, addrs: list[int],
                   *, timeout_s: int = 90
                   ) -> list[list[tuple[str, str, int]]]:
    """Resolve addresses → per-addr inline stack of `(func, file, line)`
    frames, innermost first, outermost (the real containing fn) last.

    A non-inlined address yields a 1-frame stack; an inlined one yields
    the full stack. `func` is demangled; empty when unresolved."""
    if not addrs:
        return []
    binary = Path(binary).resolve()
    if not binary.is_file():
        return [[] for _ in addrs]
    stdin = "\n".join(f"0x{a:x}" for a in addrs) + "\n"
    try:
        proc = subprocess.run(
            ["addr2line", "-a", "-f", "-i", "-C", "-e", str(binary)],
            input=stdin, capture_output=True, text=True,
            errors="replace", timeout=timeout_s)
    except (subprocess.TimeoutExpired, OSError) as e:
        logger.warning(f"[asm_tells] addr2line failed: {e}")
        return [[] for _ in addrs]
    if proc.returncode != 0:
        logger.warning(f"[asm_tells] addr2line exit={proc.returncode}")
        return [[] for _ in addrs]

    # Each block (delimited by `0x...`) carries alternating (func, file:line)
    # pairs — one pair per inline frame. We walk a small state machine.
    stacks: list[list[tuple[str, str, int]]] = []
    cur: list[tuple[str, str, int]] | None = None
    state = "addr"
    cur_func = ""
    for line in proc.stdout.splitlines():
        s = line.strip()
        if _A2L_ADDR.match(s):
            if cur is not None:
                stacks.append(cur)
            cur, state, cur_func = [], "func", ""
            continue
        if not s or cur is None:
            continue
        if state == "func":
            cur_func = "" if s == "??" else s
            state = "src"
            continue
        # state == "src"
        m = _A2L_SRC.match(s)
        if m:
            f, ln = m.group("file"), int(m.group("line"))
            if f and f != "??" and ln > 0:
                cur.append((cur_func, Path(f).name, ln))
        state = "func"   # next frame (if any) starts with another func line
    if cur is not None:
        stacks.append(cur)
    while len(stacks) < len(addrs):
        stacks.append([])
    return stacks[:len(addrs)]


def _bare_fn(func: str) -> str:
    """Last `::` segment of a demangled fn name; collapses generic suffix
    `<...>` if present so `mainSort<T>` → `mainSort`."""
    if not func:
        return ""
    name = func.rsplit("::", 1)[-1]
    if "<" in name:
        name = name.split("<", 1)[0]
    return name


def fn_location(binary: Path, entry_addr: int) -> tuple[str, int] | None:
    """A hot fn's definition `(file, line)` from its entry address.

    Uses the *outermost* inline frame — the real containing function —
    so a fn whose very first statement is an inlined call (common after
    LTO for tiny fns) still resolves to its own definition site, not to
    the inlined callee (verified: csv_count::cb_field entry → main.rs:55,
    not core/atomic.rs)."""
    stacks = _run_addr2line(binary, [entry_addr])
    if stacks and stacks[0]:
        _, f, ln = stacks[0][-1]      # outermost = the real fn
        return (f, ln)
    return None


def region_src_span(binary: Path, addrs: list[int]) -> RegionSrc:
    """Map hot-region instruction addresses → their dominant source span
    PLUS the bare names of every fn appearing in the inline stacks.

    Uses the *innermost* frame for the line span (where the hot code
    physically is). `inlined` flags a meaningful share of inline-depth>1
    samples (a true signal). `inlined_fns` lists every fn (outermost +
    inlined callees) seen across the region's stacks — feeds per-hotspot
    LICM slicing so BZ2_blockSort surfaces the LTO-inlined mainSort's
    candidates instead of showing 0."""
    stacks = _run_addr2line(binary, addrs)
    inner = [st[0] for st in stacks if st]
    if not inner:
        return RegionSrc(span=None, inlined=False, inlined_fns=[])
    by_file: dict[str, list[int]] = {}
    for _func, f, ln in inner:
        by_file.setdefault(f, []).append(ln)
    file = max(by_file.items(), key=lambda kv: len(kv[1]))[0]
    lines = by_file[file]

    inlined_cnt = sum(1 for st in stacks if len(st) > 1)
    resolved = sum(1 for st in stacks if st)
    inlined = resolved > 0 and inlined_cnt >= 0.30 * resolved

    fns: set[str] = set()
    for st in stacks:
        for func, _f, _ln in st:
            b = _bare_fn(func)
            if b:
                fns.add(b)

    return RegionSrc(
        span        = SrcSpan(file=file, line_lo=min(lines), line_hi=max(lines)),
        inlined     = inlined,
        inlined_fns = sorted(fns),
    )


# ---------------------------------------------------------------------------
# Step D — hot_subregions: split a diffuse hot region into tight clusters
# by innermost DWARF inline frame
# ---------------------------------------------------------------------------

def derive_hot_subregions(binary: Path, insns: list[AnnotatedInsn],
                            hot_fn_bare: str,
                            *, min_share_pct: float = 1.0,
                            ) -> list[HotSubregion]:
    """Decompose a hot region into sub-clusters by innermost inline frame.

    Each instruction's innermost frame (deepest `(func, file, line)` in
    the addr2line inline stack) marks the physical source location of
    the code that ran — when a hot fn like bzip2's `handle_compress`
    pulls in 14 LTO-inlined callees (mainSort / mainQSort3 / mainGtU
    etc.), each callee produces its own subregion. Without this split
    the parent `hot_region.src_lines` covers 700+ lines of disparate
    code that dispatch can't site to a tight loop.

    For each unique innermost (fn-name, file) we emit a subregion:
      * src_lines = "<file>:L-L'"   (subregion may live in a different
                                      file than its parent — keep the
                                      file in the string so dispatch /
                                      LLM can navigate directly)
      * asm_range = "0x..-0x.."     (addresses of instructions in this
                                      subregion, within the parent's range)
      * self_share = % of parent's sample weight (0-100, Step A unit policy)
      * inlined_fn = bare callee name (`""` when the subregion is the
                                       outer hot fn's own code, not an
                                       inlined callee)

    Subregions with `self_share < min_share_pct` are dropped (noise). Output
    sorted by self_share desc. Returns [] if addr2line couldn't resolve
    any addresses (e.g. binary stripped)."""
    if not insns:
        return []
    addrs = [i.addr for i in insns]
    stacks = _run_addr2line(binary, addrs)
    if not any(stacks):
        return []

    total_weight = sum(i.pct for i in insns)
    use_uniform = total_weight <= 0.0
    if use_uniform:
        total_weight = float(len(insns))

    @dataclass
    class _SubAcc:
        inlined_fn: str
        file:       str
        lines:      list[int] = field(default_factory=list)
        addrs:      list[int] = field(default_factory=list)
        weight:     float     = 0.0

    groups: dict[tuple[str, str], _SubAcc] = {}
    for ins, st in zip(insns, stacks):
        if not st:
            continue
        innermost_func, innermost_file, innermost_line = st[0]
        bare = _bare_fn(innermost_func)
        if not bare or not innermost_file or innermost_file == "??":
            continue
        key = (bare, innermost_file)
        if key not in groups:
            groups[key] = _SubAcc(
                inlined_fn = ("" if bare == hot_fn_bare else bare),
                file       = innermost_file,
            )
        g = groups[key]
        g.lines.append(innermost_line)
        g.addrs.append(ins.addr)
        g.weight += (1.0 if use_uniform else ins.pct)

    # Emit one HotSubregion per group. The outer fn (`inlined_fn == ""`)
    # is ALWAYS emitted regardless of share — that way evidence tells in
    # the outer fn's own code never become "unattributed". The
    # `min_share_pct` noise floor applies only to inlined callees (which
    # add clutter when LLVM inlines a thousand tiny intrinsics).
    out: list[HotSubregion] = []
    for g in groups.values():
        if not g.lines:
            continue
        share = (g.weight / total_weight) * 100.0
        is_outer = (g.inlined_fn == "")
        if not is_outer and share < min_share_pct:
            continue
        line_lo, line_hi = min(g.lines), max(g.lines)
        addr_lo, addr_hi = min(g.addrs), max(g.addrs)
        src_lines = (f"{g.file}:{line_lo}" if line_lo == line_hi
                     else f"{g.file}:{line_lo}-{line_hi}")
        asm_range = (f"0x{addr_lo:x}" if addr_lo == addr_hi
                     else f"0x{addr_lo:x}-0x{addr_hi:x}")
        out.append(HotSubregion(
            src_lines  = src_lines,
            asm_range  = asm_range,
            self_share = round(share, 2),
            inlined_fn = g.inlined_fn,
        ))
    out.sort(key=lambda s: -s.self_share)
    return out


# ---------------------------------------------------------------------------
# COLD_BLOCK detection (gates A2 cold-path-outline)
# ---------------------------------------------------------------------------
#
# Spec §2.5 / A2 trigger: a basic block B inside the hot fn is "cold" when
#   1. B's sample share over the fn total is < 1%
#   2. B is reachable from h's hot region via a CONDITIONAL branch
#   3. B is not the hot region itself
# Per-block sample share is the available approximation of the spec's
                                                                 
# sampling noise but for the 1% threshold the proxy is sound.

def _split_into_basic_blocks(insns: list[AnnotatedInsn]
                                ) -> list[list[AnnotatedInsn]]:
    """Linear basic-block split: a new block opens after every branch /
    call / ret. Doesn't account for branch targets that aren't block
    starts (= we'd over-merge slightly), which is fine for cold-block
    detection — a true cold block is far away from the hot path's
    target set, not nested inside it."""
    blocks: list[list[AnnotatedInsn]] = []
    current: list[AnnotatedInsn] = []
    for ins in insns:
        current.append(ins)
        mn = _mnemonic(ins.text)
        if mn in _BB_TERMINATOR_MNEMONICS:
            blocks.append(current)
            current = []
    if current:
        blocks.append(current)
    return blocks


def _conditional_branch_targets(insns: list[AnnotatedInsn]) -> set[int]:
    """Pull target addresses out of conditional-branch insns in `insns`.
    Targets are the first hex literal in the operand text (perf annotate
    prints either `je 13314` or `je 13314 <symbol+off>` — both parse).
    `jmp` is excluded — unconditional fall-through doesn't count as a
    cold-path dispatch per spec."""
    targets: set[int] = set()
    for ins in insns:
        mn = _mnemonic(ins.text)
        if mn not in _COND_BRANCH_MNEMONICS:
            continue
        m = re.search(r"\b([0-9a-fA-F]{3,})\b", _operands(ins.text))
        if m is None:
            continue
        try:
            targets.add(int(m.group(1), 16))
        except ValueError:
            continue
    return targets


def detect_cold_blocks(fn_insns: list[AnnotatedInsn],
                         region_insns: list[AnnotatedInsn]
                         ) -> list[AsmTell]:
    """Emit one COLD_BLOCK AsmTell per basic block in `fn_insns` that:
      * carries ≤ `_COLD_BLOCK_SHARE_MAX` of the fn's cumulative sample
        weight (1% in spec),
      * holds ≥ `_COLD_BLOCK_MIN_INSNS` instructions (trampoline filter),
      * is reachable from `region_insns` via a CONDITIONAL branch
        whose target lands inside the block,
      * does NOT lie inside the hot region itself.

    Returns asm_tells with `loc = "0x<lo>-0x<hi>"`, `count = len(block)`,
    `sample_weight = share * 100` (0-100 unit policy). Empty list when
    no cold blocks qualify."""
    if not fn_insns or not region_insns:
        return []

    blocks = _split_into_basic_blocks(fn_insns)
    if not blocks:
        return []

    total_pct = sum(i.pct for i in fn_insns)
    if total_pct <= 0:
        return []

    region_lo = min(i.addr for i in region_insns)
    region_hi = max(i.addr for i in region_insns)
    region_targets = _conditional_branch_targets(region_insns)
    if not region_targets:
        return []

    out: list[AsmTell] = []
    for block in blocks:
        if len(block) < _COLD_BLOCK_MIN_INSNS:
            continue
        lo = min(i.addr for i in block)
        hi = max(i.addr for i in block)
        # Skip blocks that overlap the hot region (they ARE the hot path).
        if not (hi < region_lo or lo > region_hi):
            continue
        block_pct = sum(i.pct for i in block)
        share = block_pct / total_pct
        if share > _COLD_BLOCK_SHARE_MAX:
            continue
        if not any(lo <= t <= hi for t in region_targets):
            continue
        rep = max(block, key=lambda i: i.pct) if block else block[0]
        out.append(AsmTell(
            pattern       = "COLD_BLOCK",
            axis          = _PATTERN_AXIS.get("COLD_BLOCK", "IC"),
            count         = len(block),
            loc           = f"0x{lo:x}-0x{hi:x}",
            snippet       = rep.text.strip(),
            sample_weight = round(share * 100.0, 2),
            subregion_idx = -1,    # by definition outside any hot_subregion
        ))
    # Sort by block_size desc — larger cold blocks are higher A2 leverage.
    out.sort(key=lambda t: -t.count)
    return out
