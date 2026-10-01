"""hot_probe data types — the evidence-pack building blocks (5-rule taxonomy).

`HotFunction` is the locate-phase output: a crate-under-test function the
workload pushed hot (self-time ≥ τ in some op), with its identity + source
location. Augmented by the driver with per-fn C1/C2/C3/II_vec/II_inl hits
from `class_I.scan` and `class_II.scan` results, so downstream consumers
(agent_perf_opt) see rule signals directly on the hot fn record.

`EvidencePack` is the full characterize-phase output per hot fn: identity +
profile counters + TMA + rule hits + raw remarks.

Rule-hit schema (used by both HotFunction and EvidencePack):

  class_i_hits = {
    "C1":       int,   # C1 site count in this fn (union of 10 panic-runtime patterns)
    "C2":       int,   # C2 site count (fptosi.sat + fptoui.sat + vector variants)
    "C3":       bool,  # P_C3 predicate result (remark ∩ !tbaa=0)
  }

  class_ii_hits = {
    "II_vec":   {reason: count, ...},   # actionable loop-vectorize reasons
    "II_inl":   {reason: count, ...},   # actionable inline reasons (post residual gate)
  }

Zero counts / False predicates / empty reason dicts are omitted for brevity
in JSON output.
"""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass
class HotFunction:
    """One workload-hot crate-under-test function + its 5-rule signature hits.

      name        base fn name (perf/demangled), e.g. "BZ2_decompress"
      self_pct    MAX self-time% across ops (hot if some op heats it)
      hottest_op  the op where self_pct is highest (the W2 measurement op)
      per_op      {op: self%} for every op the fn appeared in
      file/line*  source location in the crate (None if unresolved)

    Wrapper filtering (rule-agnostic, applied before rule scans):
      extern_wrapper  True iff the fn body is essentially one libc/syscall
                      call — dropped from the hot list (no rewrite target)
      wrapper_target  the libc symbol it delegates to (for transparency)

    Rule signals (populated by driver from class_I.ScanResult + class_II.ScanResult):
      class_i_hits    {"C1": int, "C2": int, "C3": bool} — Class I hits
      class_ii_hits   {"II_vec": {r: n, ...}, "II_inl": {r: n, ...}} — Class II hits
    """
    name: str
    self_pct: float
    hottest_op: str
    per_op: dict[str, float] = field(default_factory=dict)
    file: str | None = None
    line_start: int | None = None
    line_end: int | None = None
    extern_wrapper: bool = False
    wrapper_target: str | None = None
    # 5-rule hits (empty until driver augments after class_I/class_II scan)
    class_i_hits: dict = field(default_factory=dict)
    class_ii_hits: dict = field(default_factory=dict)
    # F7 2026-08-04:fn-type tag(orchestration / short_hash / state_machine /
                                                                 
    fn_type: str | None = None
    fn_type_reason: str | None = None
    # Why this fn is in the list. None = it cleared tau on self-time, the
    # ordinary case. Anything else names the predicate that let it in despite
    # NOT clearing tau — today only "fixed_cost", for rules whose payoff is
    # per-call overhead rather than self-time (see hot_probe.fixed_cost_admit).
    # Kept explicit so a reader of hotspots.json never has to infer why a
    # 0.1%-self-time function is being rewritten.
    admit_reason: str | None = None

    def to_dict(self) -> dict:
        loc = (f"{self.file}:{self.line_start}-{self.line_end}"
               if self.file else None)
        return {
            "name": self.name,
            "admit_reason": self.admit_reason,
            "self_pct": self.self_pct,
            "hottest_op": self.hottest_op,
            "per_op": self.per_op,
            "location": loc,
            "file": self.file,
            "line_start": self.line_start,
            "line_end": self.line_end,
            "extern_wrapper": self.extern_wrapper,
            "wrapper_target": self.wrapper_target,
            "class_i_hits": self.class_i_hits,
            "class_ii_hits": self.class_ii_hits,
            "fn_type": self.fn_type,
            "fn_type_reason": self.fn_type_reason,
        }

    @classmethod
    def from_dict(cls, d: dict) -> "HotFunction":
        """Rebuild from `to_dict` output — the inverse, for reusing a run's
        `hotspots.json` instead of re-locating.

        `location` is dropped on the way back in: it is a rendering of
        file/line_start/line_end, and those three are stored separately.

        This exists so an ablation arm can be measured against the SAME hot
        function set as the arm it is compared with. Locating twice does not
        give the same set — profiled twice on one unchanged crate with one
        unchanged workload, libxml2 produced 33 shared functions plus 5 seen
        only in one run and 6 only in the other, and `--call-graph dwarf`
        inline attribution moved 17 points of self time between two of them
        (`UTF8ToHtml` 25.9%→8.9%, `htmlEncodeEntities` 26.6%→45.5%). A
        difference of that size between two arms would be read as the ablated
        layer's effect when it is sampling noise.
        """
        return cls(
            name=d["name"],
            self_pct=d["self_pct"],
            hottest_op=d["hottest_op"],
            per_op=dict(d.get("per_op") or {}),
            file=d.get("file"),
            line_start=d.get("line_start"),
            line_end=d.get("line_end"),
            extern_wrapper=bool(d.get("extern_wrapper", False)),
            wrapper_target=d.get("wrapper_target"),
            class_i_hits=dict(d.get("class_i_hits") or {}),
            class_ii_hits=dict(d.get("class_ii_hits") or {}),
            fn_type=d.get("fn_type"),
            fn_type_reason=d.get("fn_type_reason"),
            admit_reason=d.get("admit_reason"),
        )


@dataclass
class EvidencePack:
    """Full per-fn evidence pack — the contract consumed by agent_perf_opt.

    Identity + profile counters + TMA + 5-rule hits + raw remark samples.
    Fields the schema marks unavailable are None (never faked):
      * call_count / instructions_per_call — sampling can't recover reliably
      * tma / branch_miss_rate — process-scoped, honestly labeled

    5-rule hits schema is IDENTICAL to `HotFunction`; EvidencePack additionally
    keeps `llvm_opt_remarks` (raw sample) for LLM prompt context.
    """
    # —— identity & context ——
    symbol: str                          # base fn name (crate-under-test)
    location: str | None                 # "file:start-end"
    workload: str                        # the op these numbers were measured under
    hot_region: str | None               # hot span within the fn (v1: whole fn)
    rust_source: str                     # the function's source text

    # —— profile ——
    self_time_ratio: float               # 0..1 (cycles, per-fn deepest-frame)
    retired_instructions: int | None     # per-fn share × process total
    cpi: float | None                    # per-fn cycles/insns
    tma: dict = field(default_factory=dict)      # {scope: "process", retiring, ...}
    branch_miss_rate: float | None = None        # 0..1, process-scope
    call_count: int | None = None                # unavailable from sampling
    instructions_per_call: float | None = None   # derived; None w/o call_count

    # —— 5-rule hits (populated from class_I / class_II scan results) ——
    class_i_hits:  dict = field(default_factory=dict)   # {C1, C2, C3}
    class_ii_hits: dict = field(default_factory=dict)   # {II_vec, II_inl}

    # —— raw compiler evidence (for LLM prompt) ——
    llvm_opt_remarks: list = field(default_factory=list)   # remarks filtered to this fn
    llvm_ir: str | None = None                              # optional (off by default)

    # —— agent_perf_opt inputs (P0, D13) ——
    signature: str = ""                          # fn to body '{' — for prompt Target block
    attribution_scope: dict = field(default_factory=dict)  # {metric_key: "function"|"process"|"unavailable"}
    tma_bottleneck: str = ""                     # top TMA dim name (routes Path C)

    def to_dict(self) -> dict:
        return {
            "hot_function": {
                "symbol": self.symbol,
                "location": self.location,
                "workload": self.workload,
                "hot_region": self.hot_region,
                "rust_source": self.rust_source,
                "signature": self.signature,
                "profile": {
                    "self_time_ratio": self.self_time_ratio,
                    "call_count": self.call_count,
                    "retired_instructions": self.retired_instructions,
                    "instructions_per_call": self.instructions_per_call,
                    "cpi": self.cpi,
                    "tma": self.tma,
                    "branch_miss_rate": self.branch_miss_rate,
                },
                "attribution_scope": self.attribution_scope,
                "tma_bottleneck": self.tma_bottleneck,
                "class_i_hits": self.class_i_hits,
                "class_ii_hits": self.class_ii_hits,
                "llvm_opt_remarks": self.llvm_opt_remarks,
                "llvm_ir": self.llvm_ir,
            }
        }


# ── shared helper: extract rule hits from scan results for one fn ────────────


def hits_from_scan_results(fn_name: str,
                            class_i_result,
                            class_ii_result) -> tuple[dict, dict]:
    """Given a source-fn name plus `class_I.ScanResult` + `class_II.ScanResult`,
    return (class_i_hits, class_ii_hits) shaped per the schema above.

    Zero counts / False predicates / empty reason dicts are omitted so the
    resulting dicts are compact — consumers can treat a missing key as 0/False/{}.
    """
    class_i: dict = {}
    class_ii: dict = {}

    # ── Class I ──
    if class_i_result is not None:
        c1c2 = getattr(class_i_result, "c1_c2_hits_by_fn", {}).get(fn_name, {})
        if c1c2.get("C1", 0) > 0:
            class_i["C1"] = c1c2["C1"]
        if c1c2.get("C2", 0) > 0:
            class_i["C2"] = c1c2["C2"]
        if fn_name in getattr(class_i_result, "c3_hits", set()):
            class_i["C3"] = True

    # ── Class II ──
    if class_ii_result is not None:
        vec = getattr(class_ii_result, "ii_vec_hits", {}).get(fn_name)
        if vec is not None and vec.counts.get("II_vec"):
            class_ii["II_vec"] = dict(vec.counts["II_vec"])
        inl = getattr(class_ii_result, "ii_inl_hits", {}).get(fn_name)
        if inl is not None and inl.counts.get("II_inl"):
            class_ii["II_inl"] = dict(inl.counts["II_inl"])

    return class_i, class_ii


def augment_hot_fns(hot_fns: list[HotFunction],
                     class_i_result, class_ii_result) -> None:
    """Fill in `class_i_hits` + `class_ii_hits` on each HotFunction in place.

    Idempotent — calling twice with the same scan results produces the same
    output (dict copy semantics)."""
    for hf in hot_fns:
        ci, cii = hits_from_scan_results(hf.name, class_i_result, class_ii_result)
        hf.class_i_hits = ci
        hf.class_ii_hits = cii
