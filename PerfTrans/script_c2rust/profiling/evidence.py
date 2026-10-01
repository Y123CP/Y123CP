""                                                           

                                         
                                         
                                                          
                                                     
                                                     
                                                     
                                                       
                                                          
                                                       
                                                    
                                                                 

                                                           
                                                           
                                                                        
                                                                      
                                                                        
                                                                       
                                                              
                                                                
                                                                            
                                                                        
                                                                          
                                                                            
                                                                   
                                                                           
                                                                           
                                                                         
                                          

       
       
                                                                        
                                                                   
                                                                   

                                             
                                                              
                                                                          
                                              
                                                       
                                                                 
                           
                                                                    
                                                                    
                                                                                 
                                                       

                                                                  
                       

from __future__ import annotations

import json
from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class WorkloadStability:
    """Stability indicators for the wall-clock measurement."""
    runs:               int       # number of measure_runs (warmup excluded)
    mean_ms:            float
    stddev_ms:          float
    cv:                 float     # stddev / mean — accept if ≤ required_cv
    min_ms:             float
    max_ms:             float
    stable:             bool      # cv ≤ required_cv AND mean_ms ≥ min_runtime_ms


def utc_now_iso() -> str:
    """ISO-8601 UTC timestamp, second resolution. For EvidencePack.scanned_at."""
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


# ===========================================================================
# §2.5 — HotspotProfile schema
# ===========================================================================
#
# Theory anchor (perf_tree_design.md §2.5): CPU_Time = IC × CPI × T_cycle
# (Iron Law). T_cycle is a measurement control variable; software moves
# only IC and CPI. The record is laid out along those two axes:
#   - routing   : TMA L1 — the CPI-axis spine + top-level router
                                                         
                                                                    
#   - evidence  : compiler-state + source-shape + signature facts, joined
#                 back to the two axes by each item's `axis` tag

# regime enum (§2.5.2 / Phase F) — derived from TMA L1 Retiring share.
# Values are English IDs per stage_b_opt_design.md §HotspotProfile schema.
REGIME_THROUGHPUT = "IC-dominated"    # Retiring ≥ 60% — IC-axis dominates
REGIME_STALL      = "CPI-dominated"   # Retiring ≤ 40% — CPI-axis dominates
REGIME_MIXED      = "mixed"           # in between — both axes
REGIME_UNKNOWN    = "unknown"         # TMA unavailable (degraded)

# Schema version pinned at top-level (Phase F). When schema shape changes
# in a backwards-incompatible way, bump this and write a migration note
# in CHANGELOG / docs/stage_b_opt_design.md.
SCHEMA_VERSION = "hotspot-profile-v1"


def derive_regime(tma_retiring_pct: float | None,
                  *, hi: float = 60.0, lo: float = 40.0) -> str:
    ""                                                                

                                                           
    if tma_retiring_pct is None:
        return REGIME_UNKNOWN
    if tma_retiring_pct >= hi:
        return REGIME_THROUGHPUT
    if tma_retiring_pct <= lo:
        return REGIME_STALL
    return REGIME_MIXED


# ---------------------------------------------------------------------------
# Anchor + hot region
# ---------------------------------------------------------------------------

@dataclass
class HotSubregion:
    """One sample-cluster sub-region inside a hot fn's hot_region.

    A diffuse hot region (e.g. bzip2 handle_compress spans 770 src lines
    with 14 inlined callees) is useless as a single dispatch site — LLM
    can't tell which inlined loop to optimize. Sub-regions split the
    hot region by sample density + DWARF inline boundary so dispatch
    can pick a tight site.

    Populated by Step D (sample clustering + DWARF inline frame walk)."""
    src_lines:   str           = ""    # "L-L'" within the parent hot_region's src
    asm_range:   str           = ""    # "0x..-0x.." within parent's asm_range
    self_share:  float         = 0.0   # this sub-region's share of parent's
                                       # hot-region samples, 0-100
    inlined_fn:  str           = ""    # bare name of the inlined callee that
                                       # owns this sub-region; "" = outer fn

    def to_dict(self) -> dict[str, Any]:
        return {"src_lines": self.src_lines, "asm_range": self.asm_range,
                "self_share": self.self_share, "inlined_fn": self.inlined_fn}

    @classmethod
    def from_dict(cls, d: dict) -> HotSubregion:
        return cls(
            src_lines  = d.get("src_lines", ""),
            asm_range  = d.get("asm_range", ""),
            self_share = float(d.get("self_share", 0.0)),
            inlined_fn = d.get("inlined_fn", ""),
        )


@dataclass
class HotRegion:
    """§2.5.1 — the hottest loop/block inside a hot fn.

    `inlined_fns` (DWARF-derived, design-compliant) lists the bare names
    of every fn whose code is part of this region — outermost = the hot
    symbol itself, inner = LTO-inlined callees. Step D's clustering
    splits diffuse regions into `hot_subregions`."""
    src_lines:      str                = ""    # "L-L'" — source line range of the hot region
    asm_range:      str                = ""    # "0x..-0x.." — instruction address range
    inlined_fns:    list[str]          = field(default_factory=list)
    hot_subregions: list[HotSubregion] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {"src_lines":      self.src_lines,
                "asm_range":      self.asm_range,
                "inlined_fns":    list(self.inlined_fns),
                "hot_subregions": [s.to_dict() for s in self.hot_subregions]}

    @classmethod
    def from_dict(cls, d: dict) -> HotRegion:
        return cls(src_lines=d.get("src_lines", ""),
                   asm_range=d.get("asm_range", ""),
                   inlined_fns=list(d.get("inlined_fns") or []),
                   hot_subregions=[HotSubregion.from_dict(s)
                                    for s in (d.get("hot_subregions") or [])])


@dataclass
class Workload:
    """Phase F — workload identity that drove this characterization.

    `id` is the manifest's `[workload].name`. `cmd` is the full reproducible
    invocation `<binary> <args…>` with $INPUT substituted — quoted so
    consumers can copy-paste it.
    Backwards-compat note: prior `Anchor.workload` was a flat string; from
    Phase F onward it's a nested object per `stage_b_opt_design.md`
    HotspotProfile schema."""
    id:  str = ""    # workload manifest name
    cmd: str = ""    # full reproducible run command

    def to_dict(self) -> dict[str, Any]:
        return {"id": self.id, "cmd": self.cmd}

    @classmethod
    def from_dict(cls, d: Any) -> Workload:
        # Accept legacy flat-string form for backwards-compat reads.
        if isinstance(d, str):
            return cls(id=d, cmd="")
        if not d:
            return cls()
        return cls(id=d.get("id", ""), cmd=d.get("cmd", ""))


@dataclass
class Anchor:
    """§2.5.1 — anchors a hot fn and its hot region. self_time_pct is the
    gate field: < skip_threshold (default 3%) ⇒ the fn is dropped.

    Phase F: `workload` upgraded from string → `Workload` object;
    `call_count` added (optional — None when no harness instrumentation)."""
    workload:         Workload   = field(default_factory=Workload)
    function:         str        = ""     # "sym @ file:L-L'"
    self_time_pct:    float      = 0.0    # self CPU% from perf report --no-children, 0-100
    hot_region:       HotRegion  = field(default_factory=HotRegion)
    hot_region_share: float      = 0.0    # hot region's share of the fn's self-time, 0-100
    call_count:       int | None = None   # runtime call count if harness can supply

    def to_dict(self) -> dict[str, Any]:
        return {
            "workload":         self.workload.to_dict(),
            "function":         self.function,
            "self_time_pct":    self.self_time_pct,
            "hot_region":       self.hot_region.to_dict(),
            "hot_region_share": self.hot_region_share,
            "call_count":       self.call_count,
        }

    @classmethod
    def from_dict(cls, d: dict) -> Anchor:
        return cls(
            workload         = Workload.from_dict(d.get("workload") or {}),
            function         = d.get("function", ""),
            self_time_pct    = float(d.get("self_time_pct", 0.0)),
            hot_region       = HotRegion.from_dict(d.get("hot_region") or {}),
            hot_region_share = float(d.get("hot_region_share", 0.0)),
            call_count       = d.get("call_count"),
        )


# ---------------------------------------------------------------------------
# Routing (TMA L1) + axes
# ---------------------------------------------------------------------------

@dataclass
class Routing:
    """§2.5.2 — TMA L1 (4 slots, 0-100) + derived regime that routes axis fill."""
    tma_retiring:        float | None = None    # 0-100; None ⇒ TMA unavailable
    tma_frontend_bound:  float | None = None    # 0-100
    tma_bad_speculation: float | None = None    # 0-100
    tma_backend_bound:   float | None = None    # 0-100
    regime:              str          = REGIME_UNKNOWN
    tma_source:          str          = "unavailable"  # toplev|perf-topdown|unavailable

    def to_dict(self) -> dict[str, Any]:
        return {
            "tma_retiring":        self.tma_retiring,
            "tma_frontend_bound":  self.tma_frontend_bound,
            "tma_bad_speculation": self.tma_bad_speculation,
            "tma_backend_bound":   self.tma_backend_bound,
            "regime":              self.regime,
            "tma_source":          self.tma_source,
        }

    @classmethod
    def from_dict(cls, d: dict) -> Routing:
        return cls(
            tma_retiring        = d.get("tma_retiring"),
            tma_frontend_bound  = d.get("tma_frontend_bound"),
            tma_bad_speculation = d.get("tma_bad_speculation"),
            tma_backend_bound   = d.get("tma_backend_bound"),
            regime              = d.get("regime", REGIME_UNKNOWN),
            tma_source          = d.get("tma_source", "unavailable"),
        )


@dataclass
class CpiAxis:
    """§2.5.3 — stall axis. Counters are process-level (perf stat); the
    miss rates are misses/references **× 100** so they're directly
    comparable to dispatch thresholds (e.g. mispredict ≥ 20%).
    tma_path is the toplev -l3 drill-down."""
    cpi:                    float | None = None
    tma_path:               str          = ""
    l1d_miss_rate:          float | None = None    # 0-100
    l2_miss_rate:           float | None = None    # 0-100
    llc_miss_rate:          float | None = None    # 0-100
    dtlb_miss_rate:         float | None = None    # 0-100
    branch_mispredict_rate: float | None = None    # 0-100

    def to_dict(self) -> dict[str, Any]:
        return {
            "cpi":                    self.cpi,
            "tma_path":               self.tma_path,
            "l1d_miss_rate":          self.l1d_miss_rate,
            "l2_miss_rate":           self.l2_miss_rate,
            "llc_miss_rate":          self.llc_miss_rate,
            "dtlb_miss_rate":         self.dtlb_miss_rate,
            "branch_mispredict_rate": self.branch_mispredict_rate,
        }

    @classmethod
    def from_dict(cls, d: dict) -> CpiAxis:
        return cls(
            cpi                    = d.get("cpi"),
            tma_path               = d.get("tma_path", ""),
            l1d_miss_rate          = d.get("l1d_miss_rate"),
            l2_miss_rate           = d.get("l2_miss_rate"),
            llc_miss_rate          = d.get("llc_miss_rate"),
            dtlb_miss_rate         = d.get("dtlb_miss_rate"),
            branch_mispredict_rate = d.get("branch_mispredict_rate"),
        )


@dataclass
class RetiringMix:
    ""                                              

                                                                         
                                                                    
                                                                        
                                          
    scalar_fp: float = 0.0    # 0-100
    vector_fp: float = 0.0    # 0-100
    int:       float = 0.0    # noqa: A003 — schema field name per §2.5.4; 0-100
    branch:    float = 0.0    # conditional jumps (jcc / jmp), 0-100 — NOT call/ret
    mem:       float = 0.0    # 0-100
    call_ret:  float = 0.0    # call/callq/ret/retq, 0-100 (NEW Step A)

    def to_dict(self) -> dict[str, Any]:
        return {
            "scalar_fp": self.scalar_fp, "vector_fp": self.vector_fp,
            "int": self.int, "branch": self.branch, "mem": self.mem,
            "call_ret": self.call_ret,
        }

    @classmethod
    def from_dict(cls, d: dict) -> RetiringMix:
        return cls(
            scalar_fp = float(d.get("scalar_fp", 0.0)),
            vector_fp = float(d.get("vector_fp", 0.0)),
            int       = float(d.get("int", 0.0)),
            branch    = float(d.get("branch", 0.0)),
            mem       = float(d.get("mem", 0.0)),
            call_ret  = float(d.get("call_ret", 0.0)),
        )


@dataclass
class IcAxis:
    """§2.5.4 — instruction-count axis. Phase G renames + adds 2 fields
    to match `stage_b_opt_design.md` HotspotProfile schema:

      insns_total_workload  — total retired instructions for the WHOLE
                               workload (`perf stat -e instructions`).
                               Lets dispatch compute "what % of the run
                               does this fn dominate".
      hot_fn_insn_share     — hot fn's share of instruction samples,
                               0-100 (`perf record -e instructions:u`
                               + perf report).
      hot_fn_insns_est      — estimated retired insns inside the hot fn,
                               ≈ `insns_total_workload * hot_fn_insn_share / 100`.
                               (Was previously called `insns_total`;
                               Phase G renames to match spec.)

    `insns_per_iter` is None when the workload's iteration count is
    unknown (`[profile].iterations` unset in the manifest)."""
    insns_total_workload: int          = 0
    hot_fn_insn_share:    float        = 0.0    # 0-100
    hot_fn_insns_est:     int          = 0
    insns_per_iter:       float | None = None
    ipc:                  float | None = None
    retiring_mix:         RetiringMix  = field(default_factory=RetiringMix)

    def to_dict(self) -> dict[str, Any]:
        return {
            "insns_total_workload": self.insns_total_workload,
            "hot_fn_insn_share":    self.hot_fn_insn_share,
            "hot_fn_insns_est":     self.hot_fn_insns_est,
            "insns_per_iter":       self.insns_per_iter,
            "ipc":                  self.ipc,
            "retiring_mix":         self.retiring_mix.to_dict(),
        }

    @classmethod
    def from_dict(cls, d: dict) -> IcAxis:
        # Phase G migration: accept legacy `insns_total` as an alias for
        # `hot_fn_insns_est` so old JSON files keep loading. New writes
        # always use the spec names.
        legacy_insns_total = int(d.get("insns_total", 0))
        return cls(
            insns_total_workload = int(d.get("insns_total_workload", 0)),
            hot_fn_insn_share    = float(d.get("hot_fn_insn_share", 0.0)),
            hot_fn_insns_est     = int(d.get("hot_fn_insns_est", legacy_insns_total)),
            insns_per_iter       = d.get("insns_per_iter"),
            ipc                  = d.get("ipc"),
            retiring_mix         = RetiringMix.from_dict(d.get("retiring_mix") or {}),
        )


# ---------------------------------------------------------------------------
# Evidence pool — three sources joined to the two axes
# ---------------------------------------------------------------------------

@dataclass
class AsmTell:
    """§2.5.5 — one observed asm pattern in the hot region. A fact, not a
    judgement ("optimizable" is decided downstream by rule triggers).

    `subregion_idx` (Step E): index into the enclosing hotspot's
    `anchor.hot_region.hot_subregions` whose `asm_range` covers this tell's
    address. `-1` = not attributable.

    `sample_weight` (Phase F): the share of the hot region's total sample
    weight that this tell carries (0-100). Lets dispatch rank candidate
    sites within a subregion."""
    pattern:       str   = ""      # INDIRECT_CALL|BOUNDS_CHECK|SCALAR_FP|... (extensible)
    axis:          str   = ""      # CPI|IC|both
    count:         int   = 0       # occurrences in the hot region
    loc:           str   = ""      # asm address / mapped source line
    snippet:       str   = ""      # raw asm fragment
    sample_weight: float = 0.0     # 0-100 share of region's total sample weight
    subregion_idx: int   = -1      # → hot_subregions[idx]; -1 = no attribution

    def to_dict(self) -> dict[str, Any]:
        return {"pattern": self.pattern, "axis": self.axis,
                "count": self.count, "loc": self.loc, "snippet": self.snippet,
                "sample_weight": self.sample_weight,
                "subregion_idx": self.subregion_idx}

    @classmethod
    def from_dict(cls, d: dict) -> AsmTell:
        return cls(pattern=d.get("pattern", ""), axis=d.get("axis", ""),
                   count=int(d.get("count", 0)), loc=d.get("loc", ""),
                   snippet=d.get("snippet", ""),
                   sample_weight=float(d.get("sample_weight", 0.0)),
                   subregion_idx=int(d.get("subregion_idx", -1)))


@dataclass
class OptRemarkTell:
    """§2.5.5 — one compiler optimization remark, filtered to the anchor.

    `opt_pass` serializes as JSON key "pass" (Python keyword ⇒ renamed).
    `subregion_idx` (Step E): same semantics as AsmTell — attributed by
    matching the remark's `loc=file:line` against subregion src_lines."""
    opt_pass:      str = ""       # loop-vectorize|inline|licm|...
    status:        str = ""       # missed|passed
    reason:        str = ""       # compiler's free-text reason
    loc:           str = ""       # source location
    subregion_idx: int = -1       # → hot_subregions[idx]; -1 = no attribution

    def to_dict(self) -> dict[str, Any]:
        return {"pass": self.opt_pass, "status": self.status,
                "reason": self.reason, "loc": self.loc,
                "subregion_idx": self.subregion_idx}

    @classmethod
    def from_dict(cls, d: dict) -> OptRemarkTell:
        return cls(opt_pass=d.get("pass", ""), status=d.get("status", ""),
                   reason=d.get("reason", ""), loc=d.get("loc", ""),
                   subregion_idx=int(d.get("subregion_idx", -1)))


@dataclass
class SourcePattern:
    """One source-level shape detected by tree-sitter / syn_walker inside
    a hot fn's hot region. Drives the dispatch picks that asm_tells alone
    can't safely make (§2.6 source-pattern triggers).

    `kind` (closed enum, extensible in lockstep with stage_b_opt_design.md):

        loop_invariant_branch  — `if c` inside a loop, c is loop-invariant
                                 (C4 loop-unswitch trigger)
        byte_copy_loop         — `for i { dst[i] = src[i] }` (C5-loop-copy)
        byte_compare_loop      — `for i { if a[i] != b[i] { ... } }`
                                 (C5-loop-equal)
        byte_fill_loop         — `for i { dst[i] = v }` (C5-loop-fill)
        byte_search_loop       — `while p { if *p == t { return } }`
                                 (E1-memchr trigger)
        branch_cascade         — `if k==0 .. else if k==1 ..` or match
                                 (D3 trigger)
        pure_short_call        — small pure-fn callsite, small arg domain
                                 (C6 trigger)
        state_machine_indexing — slice[i] inside `match s` arms, scattered
                                 (C7 trigger)

    Step A populates `loop_invariant_branch` only (via existing
    `proposer.syn_walker.list_loop_invariant_conds`); Step C adds the
    byte loop family + branch_cascade; Phase H adds the remaining 11 kinds.

    Phase F additions:
      `related_rule`: which dispatch rule consumes this signal (verbatim
                      from stage_b_opt_design.md schema), e.g.
                      `"C1 loop-to-iterator"`, `"C5 replace-with-slice-op"`.
      `confidence`:   `"high" | "medium" | "low"` — detector's confidence
                      in the match (helps dispatch tie-break).

    `subregion_idx` (Step E): same semantics as AsmTell — attributed by
    matching the pattern's `loc=file:line` against subregion src_lines.

    `details`: per-kind structured payload (kind-specific keys per spec)."""
    kind:          str            = ""    # closed enum
    loc:           str            = ""    # "file.rs:120-135"
    details:       dict[str, Any] = field(default_factory=dict)
    related_rule:  str            = ""    # dispatch rule id this serves
    confidence:    str            = "medium"   # high | medium | low
    subregion_idx: int            = -1    # → hot_subregions[idx]; -1 = no attribution
    # Per-kind detail keys (for reference; not enforced here):
    #   loop_invariant_branch  : {loop_kind, loop_line, cond_snippet, reads}
    #   byte_copy_loop         : {src, dst, len}
    #   byte_compare_loop      : {a, b, len, early_return_value}
    #   byte_fill_loop         : {dst, val, len}
    #   byte_search_loop       : {target, buf, direction, return_kind}
    #   branch_cascade         : {key, cases, default, branch_count}
    #   pure_short_call        : {callee, domain, body_ir_insns}
    #   state_machine_indexing : {match_var, sites_count}

    def to_dict(self) -> dict[str, Any]:
        return {"kind": self.kind, "loc": self.loc,
                "details": dict(self.details),
                "related_rule": self.related_rule,
                "confidence": self.confidence,
                "subregion_idx": self.subregion_idx}

    @classmethod
    def from_dict(cls, d: dict) -> SourcePattern:
        return cls(kind=d.get("kind", ""), loc=d.get("loc", ""),
                   details=dict(d.get("details") or {}),
                   related_rule=d.get("related_rule", ""),
                   confidence=d.get("confidence", "medium"),
                   subregion_idx=int(d.get("subregion_idx", -1)))


@dataclass
class FnSignatureFacts:
    """**Internal-only** per-fn callback aggregator. NOT part of the
    serialized HotspotProfile schema (Phase F replaced the JSON-emitted
    `fn_signature_facts` with a list of `FnSignaturePattern` entries).
    Kept here as a typed intermediate that `profiling.source_patterns`
    uses while scanning the project — `pipeline.py` then unrolls it to
    per-param FnSignaturePattern entries at emission time."""
    has_option_fn_ptr_param: bool                       = False
    fn_ptr_param_names:      list[str]                  = field(default_factory=list)
    unique_caller_targets:   dict[str, str | None]      = field(default_factory=dict)
    caller_count:            int                        = 0


@dataclass
class FnSignaturePattern:
    """Phase F (per `stage_b_opt_design.md` HotspotProfile.evidence
    .fn_signature_patterns[]) — ONE entry per callback parameter on the
    enclosing hot fn (or any of its LTO-inlined callees).

    Shape mirrors the spec's per-param entry:
      pattern       — fixed enum: `"option_fn_ptr_param"` (the only
                       Phase F kind; extensible)
      loc           — `file:line` of the fn signature
      param         — param name, e.g. `"cb"`
      type          — full type text, e.g.
                       `"Option<unsafe extern \\"C\\" fn(i32) -> bool>"`
      related_rule  — `"G1 callback-monomorphize / D1 callback-devirt"`
                       (spec verbatim)
      confidence    — `"high" | "medium" | "low"`

    Was previously aggregated as a single `FnSignatureFacts` per hot fn;
    Phase F splits to per-param entries so dispatch can target a specific
    callback param when there are multiple."""
    pattern:      str = "option_fn_ptr_param"
    loc:          str = ""
    param:        str = ""
    type:         str = ""    # noqa: A003 — schema field name
    related_rule: str = "G1 callback-monomorphize / D1 callback-devirt"
    confidence:   str = "medium"

    def to_dict(self) -> dict[str, Any]:
        return {
            "pattern":      self.pattern,
            "loc":          self.loc,
            "param":        self.param,
            "type":         self.type,
            "related_rule": self.related_rule,
            "confidence":   self.confidence,
        }

    @classmethod
    def from_dict(cls, d: dict) -> FnSignaturePattern:
        return cls(
            pattern      = d.get("pattern", "option_fn_ptr_param"),
            loc          = d.get("loc", ""),
            param        = d.get("param", ""),
            type         = d.get("type", ""),
            related_rule = d.get("related_rule", ""),
            confidence   = d.get("confidence", "medium"),
        )


# All 17 source_pattern kinds per `stage_b_opt_design.md` §HotspotProfile.
# Empty arrays are pre-populated so the JSON output shape is stable even
# when a detector is absent / stubbed — dispatch can read
# `source_patterns["pure_short_call"]` without a KeyError check.
SOURCE_PATTERN_KINDS: tuple[str, ...] = (
    "raw_pointer_loop",
    "byte_copy_loop",
    "byte_compare_loop",
    "byte_fill_loop",
    "byte_search_loop",
    "branchless_candidate",
    "branch_cascade",
    "loop_invariant_branch",
    "scattered_state_machine_indexing",
    "libc_mem_calls",
    "pure_short_call",
    "loop_alloc_free",
    "const_size_alloc",
    "memory_access_pattern",
    "manual_byte_unroll",
    "callback_devirt",
    "layout_candidate",
    "hand_written_hashmap",
    "checksum_or_hash_kernel",
    "longest_match_canonical",
)


def _empty_source_patterns() -> dict[str, list[SourcePattern]]:
    """Initialize the kind-keyed source_patterns dict with every spec kind
    mapped to an empty list, so downstream consumers don't have to guard."""
    return {kind: [] for kind in SOURCE_PATTERN_KINDS}


@dataclass
class Evidence:
    """§2.5.5 — the evidence pool. Phase F reshape (per
    `stage_b_opt_design.md` HotspotProfile.evidence):
      * `source_patterns` is a **dict keyed by `kind`** (was flat list).
        Every spec kind is pre-initialized to `[]` — dispatch can read
        `source_patterns["byte_copy_loop"]` etc. unconditionally.
      * `fn_signature_patterns` is a **list of per-param entries** (was a
        single aggregated `FnSignatureFacts` object).
    """
    asm_tells:              list[AsmTell]                       = field(default_factory=list)
    opt_remarks:            list[OptRemarkTell]                 = field(default_factory=list)
    fn_signature_patterns:  list[FnSignaturePattern]            = field(default_factory=list)
    source_patterns:        dict[str, list[SourcePattern]]      = field(default_factory=_empty_source_patterns)

    def to_dict(self) -> dict[str, Any]:
        return {
            "asm_tells":             [t.to_dict() for t in self.asm_tells],
            "opt_remarks":           [r.to_dict() for r in self.opt_remarks],
            "fn_signature_patterns": [p.to_dict() for p in self.fn_signature_patterns],
            "source_patterns":       {kind: [sp.to_dict() for sp in lst]
                                       for kind, lst in self.source_patterns.items()},
        }

    @classmethod
    def from_dict(cls, d: dict) -> Evidence:
        raw_sp = d.get("source_patterns") or {}
        sp: dict[str, list[SourcePattern]] = _empty_source_patterns()
        # Phase F supports BOTH new (dict) and legacy (flat list with `kind`
        # field) reads, so older JSONs continue to load.
        if isinstance(raw_sp, dict):
            for kind, items in raw_sp.items():
                if kind in sp:
                    sp[kind] = [SourcePattern.from_dict(x) for x in (items or [])]
                else:
                    # Unknown kind — keep it under its key so we don't lose data.
                    sp[kind] = [SourcePattern.from_dict(x) for x in (items or [])]
        elif isinstance(raw_sp, list):
            for x in raw_sp:
                pat = SourcePattern.from_dict(x)
                sp.setdefault(pat.kind, []).append(pat)
        return cls(
            asm_tells              = [AsmTell.from_dict(x)
                                       for x in (d.get("asm_tells") or [])],
            opt_remarks            = [OptRemarkTell.from_dict(x)
                                       for x in (d.get("opt_remarks") or [])],
            fn_signature_patterns  = [FnSignaturePattern.from_dict(x)
                                       for x in (d.get("fn_signature_patterns") or [])],
            source_patterns        = sp,
        )


# ---------------------------------------------------------------------------
# attempt_history (Phase F)
# ---------------------------------------------------------------------------

@dataclass
class AttemptHistoryEntry:
    """Phase F — one record of a rule application attempt. Populated by
    Stage B dispatch / orchestrator after each (rule, site) atom finishes.
    Profiling stage emits an empty array — it's filled live by the loop.

    `result`       — `"commit" | "rollback" | "skipped"`
    gate columns   — each `"pass" | "fail" | "not_run"`
    `reason`       — free-text on failure / skip
    `remaining_signals` — asm_tells / source_pattern kinds still present
                          after the attempt (used by next-round dispatch)"""
    rule:              str       = ""
    site:              str       = ""
    result:            str       = "skipped"   # commit | rollback | skipped
    compile:           str       = "not_run"   # pass | fail | not_run    # noqa: A003
    functional:        str       = "not_run"
    effect_check:      str       = "not_run"
    perf_gate:         str       = "not_run"
    reason:            str       = ""
    remaining_signals: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "rule":              self.rule,
            "site":              self.site,
            "result":            self.result,
            "compile":           self.compile,
            "functional":        self.functional,
            "effect_check":      self.effect_check,
            "perf_gate":         self.perf_gate,
            "reason":            self.reason,
            "remaining_signals": list(self.remaining_signals),
        }

    @classmethod
    def from_dict(cls, d: dict) -> AttemptHistoryEntry:
        return cls(
            rule              = d.get("rule", ""),
            site              = d.get("site", ""),
            result            = d.get("result", "skipped"),
            compile           = d.get("compile", "not_run"),
            functional        = d.get("functional", "not_run"),
            effect_check      = d.get("effect_check", "not_run"),
            perf_gate         = d.get("perf_gate", "not_run"),
            reason            = d.get("reason", ""),
            remaining_signals = list(d.get("remaining_signals") or []),
        )


# ---------------------------------------------------------------------------
# Top-level HotspotProfile
# ---------------------------------------------------------------------------

@dataclass
class HotspotProfile:
    """§2.5.7 — the per-hot-fn record. Phase F additions:
      * `schema_version` pinned to `"hotspot-profile-v1"`
      * `attempt_history` top-level array (empty at profiling time;
        Stage B dispatch fills it round by round)

    The 5 zones (anchor / routing / cpi_axis / ic_axis / evidence) are
    unchanged in role; only their inner shapes were tightened in Phase F."""
    schema_version:   str                        = SCHEMA_VERSION
    anchor:           Anchor                     = field(default_factory=Anchor)
    routing:          Routing                    = field(default_factory=Routing)
    cpi_axis:         CpiAxis                    = field(default_factory=CpiAxis)
    ic_axis:          IcAxis                     = field(default_factory=IcAxis)
    evidence:         Evidence                   = field(default_factory=Evidence)
    attempt_history:  list[AttemptHistoryEntry]  = field(default_factory=list)
    notes:            list[str]                  = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version":  self.schema_version,
            "anchor":          self.anchor.to_dict(),
            "routing":         self.routing.to_dict(),
            "cpi_axis":        self.cpi_axis.to_dict(),
            "ic_axis":         self.ic_axis.to_dict(),
            "evidence":        self.evidence.to_dict(),
            "attempt_history": [a.to_dict() for a in self.attempt_history],
            "notes":           list(self.notes),
        }

    @classmethod
    def from_dict(cls, d: dict) -> HotspotProfile:
        return cls(
            schema_version  = d.get("schema_version", SCHEMA_VERSION),
            anchor          = Anchor.from_dict(d.get("anchor") or {}),
            routing         = Routing.from_dict(d.get("routing") or {}),
            cpi_axis        = CpiAxis.from_dict(d.get("cpi_axis") or {}),
            ic_axis         = IcAxis.from_dict(d.get("ic_axis") or {}),
            evidence        = Evidence.from_dict(d.get("evidence") or {}),
            attempt_history = [AttemptHistoryEntry.from_dict(x)
                                for x in (d.get("attempt_history") or [])],
            notes           = list(d.get("notes") or []),
        )


# ---------------------------------------------------------------------------
# Sidecar types (produced by upstream extractors; pipeline converts these
# into per-hotspot SourcePattern entries — no longer top-level on the
# CharacterizationReport).
# ---------------------------------------------------------------------------

@dataclass
class LoopInvariantCandidate:
    """One `if`/`match` cond inside a loop body whose reads are not written
    in the loop body — a candidate for hoisting / pre-computation.

    `proposer.syn_walker.list_loop_invariant_conds` emit type. Used during
    characterization to derive per-hotspot
    `evidence.source_patterns[kind="loop_invariant_branch"]` (Step A
    populator). Not serialized on `CharacterizationReport` anymore — the
    relevant info lives inside the right hot fn's evidence."""
    file:          str       = ""
    line:          int       = 0
    loop_kind:     str       = ""        # "while" | "for" | "loop"
    loop_line:     int       = 0
    containing_fn: str       = ""
    cond_snippet:  str       = ""
    reads:         list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {"file": self.file, "line": self.line,
                "loop_kind": self.loop_kind, "loop_line": self.loop_line,
                "containing_fn": self.containing_fn,
                "cond_snippet": self.cond_snippet,
                "reads": list(self.reads)}

    @classmethod
    def from_dict(cls, d: dict) -> LoopInvariantCandidate:
        return cls(file=d.get("file", ""), line=int(d.get("line", 0)),
                   loop_kind=d.get("loop_kind", ""),
                   loop_line=int(d.get("loop_line", 0)),
                   containing_fn=d.get("containing_fn", ""),
                   cond_snippet=d.get("cond_snippet", ""),
                   reads=list(d.get("reads") or []))

    def to_source_pattern(self) -> SourcePattern:
        """Convert this LICM candidate into the schema-canonical
        `SourcePattern(kind="loop_invariant_branch")` form, which is how
        it lands inside the right hotspot's evidence pool."""
        return SourcePattern(
            kind = "loop_invariant_branch",
            loc  = f"{self.file}:{self.line}",
            details = {
                "loop_kind":    self.loop_kind,
                "loop_line":    self.loop_line,
                "loop_var":     "",   # syn_walker doesn't extract; left blank
                "condition":    self.cond_snippet,
                "cond_snippet": self.cond_snippet,
                "reads":        list(self.reads),
                "proof":        "condition does not depend on loop_var and is "
                                "not modified in loop",
            },
            related_rule = "C4 loop-unswitch",
            confidence   = "medium",
        )


# ---------------------------------------------------------------------------
# Top-level report
# ---------------------------------------------------------------------------

@dataclass
class CharacterizationReport:
    ""                                                                      
                                                                          

                                                                  
                                                                 

                                                                     
                                                                     
                                                      
                                                                  
    project_path:                 str
    workload_name:                str
    workload_args:                list[str]
    binary:                       str
    scanned_at:                   str                          # ISO-8601 UTC
    stability:                    WorkloadStability
    hotspots:                     list[HotspotProfile]          = field(default_factory=list)
    skipped:                      list[dict]                    = field(default_factory=list)
    notes:                        list[str]                     = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "project_path":  self.project_path,
            "workload_name": self.workload_name,
            "workload_args": list(self.workload_args),
            "binary":        self.binary,
            "scanned_at":    self.scanned_at,
            "stability":     asdict(self.stability),
            "hotspots":      [h.to_dict() for h in self.hotspots],
            "skipped":       list(self.skipped),
            "notes":         list(self.notes),
        }

    def write_json(self, path: Path) -> Path:
        """Serialize to JSON. Returns the resolved path written."""
        path = Path(path).resolve()
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(self.to_dict(), indent=2, ensure_ascii=False))
        return path

    @classmethod
    def read_json(cls, path: Path) -> CharacterizationReport:
        raw = json.loads(Path(path).read_text(encoding="utf-8"))
        return cls(
            project_path  = raw["project_path"],
            workload_name = raw["workload_name"],
            workload_args = list(raw.get("workload_args") or []),
            binary        = raw["binary"],
            scanned_at    = raw["scanned_at"],
            stability     = WorkloadStability(**raw["stability"]),
            hotspots      = [HotspotProfile.from_dict(h)
                              for h in (raw.get("hotspots") or [])],
            skipped       = list(raw.get("skipped") or []),
            notes         = list(raw.get("notes") or []),
        )
