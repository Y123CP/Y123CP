"""agent_perf_opt/agent.py — top-level per-fn per-rule optimization loop.

Composes P0-P3 modules (config / state / gates / rewrite_applier /
prompt_builder) + external LLMClient + verify basecrafters into the
end-to-end agent per impl_plan §4.7 (Steps 0-4) + §4.7.α (Path C).

Not this module's job:
    * building infrastructure (rewrite_applier does splice + cargo)
    * prompt content (prompt_builder)
    * git ops (state)
    * gate details (gates + verify.*)
"""

from __future__ import annotations

import hashlib
import json
import logging
import os
import shutil
from copy import deepcopy
from dataclasses import asdict, dataclass, field, replace as dc_replace
from pathlib import Path
from typing import Any, Iterable, Optional

from Config.paths import get_path
from perf_opt.hot_probe.symbol_source import build_fn_index
from perf_opt.hot_probe.types import EvidencePack, HotFunction
from perf_opt.verify.cargo import Verifier
from perf_opt.verify.workload import WorkloadAssets
from utils.llm_client import LLMClient

from perf_opt.agent_perf_opt.caller_lookup import CallerLookup
from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.audit import AuditWriter
from perf_opt.agent_perf_opt.changeset.executor import ChangeSetExecutor
from perf_opt.agent_perf_opt.changeset.handlers.replace_function import (
    ReplaceFunctionHandler,
)
from perf_opt.agent_perf_opt.changeset.handlers.replace_region import (
    ReplaceSourceRegionHandler,
)
from perf_opt.agent_perf_opt.changeset.handlers.promote_static import (
    PromoteStaticHandler,
)
from perf_opt.agent_perf_opt.changeset.handlers.replace_call_site import (
    ReplaceCallSiteHandler,
)
from perf_opt.agent_perf_opt.changeset.handlers.rewrite_bitfield import (
    RewriteBitfieldStructHandler,
)
from perf_opt.agent_perf_opt.changeset.handlers.set_inline_attr import (
    SetFunctionInlineAttrHandler,
    locate_inline_site,
)
from perf_opt.agent_perf_opt.changeset.types import SetFunctionInlineAttr
from perf_opt.agent_perf_opt.planners.inline_attr import (
    choose_inline_attribute,
    ii_inl_callees,
    plan_inline_attr,
    rank_callees,
)
from perf_opt.agent_perf_opt.changeset.registry import HandlerRegistry
from perf_opt.agent_perf_opt.changeset.resolver import ResolutionRejected
from perf_opt.agent_perf_opt.changeset.types import (
    ChangeSetStatus,
    GateResultRecord,
    ImpactScope,
    PromoteStaticToConst,
    ReplaceCallSite,
    ReplaceFunctionBody,
    ReplaceSourceRegion,
    RewriteBitfieldStruct,
)
from perf_opt.agent_perf_opt.config import AgentConfig, AgentError
from perf_opt.agent_perf_opt.gates import W1Result, W2Session, w1_gate
from perf_opt.agent_perf_opt.plan_dispatch import build_effective_plan
from perf_opt.agent_perf_opt.prompt_builder import (
    build_execute_prompt, build_multi_card_prompt,
    build_region_prompt, build_plan_prompt, is_complex_fn, load_card,
)
from perf_opt.agent_perf_opt.rewrite_applier import (
    RewriteStatus, apply_rewrite, cargo_check, find_added_bounds_checks_in_crate,
    find_undeclared_executions, is_build_timeout,
    parse_applied_rules, parse_plan_json, parse_skipped_rules,
    resolve_edit_target,
    cargo_error_blocks, STDERR_EXCERPT_CHARS,
)
from perf_opt.agent_perf_opt.planners.llm_function import (
    plan_llm_function_change,
)
from perf_opt.agent_perf_opt.planners.bitfield import plan_bitfield_lowering
from perf_opt.agent_perf_opt.planners.goto_dispatch import (
    STRATEGIES,
    plan_goto_dispatch,
)
from perf_opt.agent_perf_opt.planners.ii_const import plan_ii_const
from perf_opt.agent_perf_opt.planners.llm_cross_fn import (
    build_cross_fn_prompt, plan_llm_cross_fn_change,
)
from perf_opt.agent_perf_opt.planners.llm_region import (
    build_repair_note,
    contract_repair_note,
    is_contract_repairable,
    is_post_validation_repairable,
    plan_region_with_contract_repair,
    post_validation_repair_note,
    quote_previous,
    w1_repair_note,
    w2_finding_repair_note,
    find_untested_subsumption,
    subsumption_repair_note,
)
from perf_opt.agent_perf_opt.type_context import name_correction_note
from perf_opt.agent_perf_opt.regions.extractor import (
    RegionExtractor,
    RegionSkipReason,
)
from perf_opt.agent_perf_opt.regions.model import (
    hit_capability,
    RuleCapability,
    rule_capability,
    stable_hit_id,
)
from perf_opt.agent_perf_opt import reporting
from perf_opt.agent_perf_opt.state import (
    AttemptRecord, RewriteAttempt, StateManager,
)

logger = logging.getLogger(__name__)

_HARNESS_BIN_REL = Path("target/release/harness")


_CHANGESET_ATTEMPT_STATUS = {
    ChangeSetStatus.DETECTED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.PLANNED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.RESOLVED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.PRE_VALIDATED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.APPLIED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.BUILD_PASSED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.POST_VALIDATED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.W1_PASSED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.W2_PASSED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.COMMIT_SUCCEEDED: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.COMMITTED: RewriteAttempt.APPLIED_COMMITTED,
    ChangeSetStatus.RECOVERED_AFTER_COMMIT: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.ABSTAINED_UNPROVEN: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.REJECTED_STALE: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.REJECTED_CONFLICT: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.REJECTED_BUILD: RewriteAttempt.SYNTAX_ERROR,
    ChangeSetStatus.REJECTED_POST_VALIDATION: RewriteAttempt.ABSTAINED,
    ChangeSetStatus.REJECTED_W1: RewriteAttempt.W1_FAIL,
    ChangeSetStatus.REJECTED_W2_NO_GAIN: RewriteAttempt.W2_REGRESS,
    ChangeSetStatus.REJECTED_W2_REGRESS: RewriteAttempt.W2_REGRESS,
    ChangeSetStatus.REJECTED_UNMEASURABLE: RewriteAttempt.W2_REGRESS,
    ChangeSetStatus.FAILED_INTERNAL: RewriteAttempt.SYNTAX_ERROR,
}

_CHANGESET_TERMINAL_VALUES = frozenset(
    status.value for status in ChangeSetStatus
)
_VALID_TERMINAL_VALUES = (
    _CHANGESET_TERMINAL_VALUES
    | frozenset(status.value for status in RewriteAttempt)
    | reporting.AGENT_ONLY_TERMINAL_STATUSES
    | reporting.REGION_TERMINAL_STATUSES
)


def _classify_changeset_terminal_status(
    terminal_status: ChangeSetStatus,
) -> tuple[RewriteAttempt, str]:
    """Return the legacy classification and exact executor terminal status."""
    if not isinstance(terminal_status, ChangeSetStatus):
        raise ValueError("terminal_status must be a ChangeSetStatus")
    try:
        attempt_status = _CHANGESET_ATTEMPT_STATUS[terminal_status]
    except KeyError as exc:
        raise ValueError(
            f"unmapped ChangeSetStatus: {terminal_status.value}"
        ) from exc
    return attempt_status, terminal_status.value


# ─── result aggregation (impl_plan §4.5) ──────────────────────────────

@dataclass
class AgentResult:
    total_attempts:           int = 0
    committed_attempts:       int = 0
    abstained_count:          int = 0
    form_rejected_count:      int = 0
    regressed_count:          int = 0
    syntax_failed_count:      int = 0
    w1_failed_count:          int = 0
    rejected_build_count:     int = 0
    w2_no_gain_count:         int = 0
    w2_regress_count:         int = 0
    unmeasurable_count:       int = 0
    per_rule_gain:            dict = field(default_factory=dict)
    per_fn_summary:           dict = field(default_factory=dict)
    marginal_results:         list[dict] = field(default_factory=list)
    final_total_gain_pct:      Optional[float] = None
    final_measurement:         Optional[dict] = None
    llm_tokens_used_estimated: int = 0
    early_stop_reason:        Optional[str] = None

    @property
    def total_wall_gain_pct(self) -> Optional[float]:
        """Compatibility name backed only by a direct pristine/final result."""
        return self.final_total_gain_pct

    def add(self, record: AttemptRecord) -> None:
        terminal_value = record.terminal_status
        if terminal_value is not None and (
            not isinstance(terminal_value, str)
            or terminal_value not in _VALID_TERMINAL_VALUES
        ):
            raise ValueError(f"unknown terminal_status: {terminal_value!r}")

        terminal_status = (
            ChangeSetStatus(terminal_value)
            if terminal_value in _CHANGESET_TERMINAL_VALUES
            else None
        )

        self.total_attempts += 1
        self.llm_tokens_used_estimated += record.tokens_in + record.tokens_out
        self.per_fn_summary.setdefault(record.fn_name, []).append(record)

        if terminal_value == reporting.REJECTED_FORM:
            # Counted apart from `abstained_count` on purpose: this is our
            # checker refusing the reply, not the model declining the rewrite.
            self.form_rejected_count += 1
            return
        if terminal_status is ChangeSetStatus.REJECTED_BUILD:
            self.syntax_failed_count += 1
            self.rejected_build_count += 1
            return
        if terminal_status is ChangeSetStatus.REJECTED_W1:
            self.w1_failed_count += 1
            return
        if terminal_status is ChangeSetStatus.REJECTED_W2_NO_GAIN:
            self.w2_no_gain_count += 1
            return
        if terminal_status is ChangeSetStatus.REJECTED_W2_REGRESS:
            self.regressed_count += 1
            self.w2_regress_count += 1
            return
        if terminal_status is ChangeSetStatus.REJECTED_UNMEASURABLE:
            self.unmeasurable_count += 1
            return
        if terminal_status is ChangeSetStatus.FAILED_INTERNAL:
            self.syntax_failed_count += 1
            return

        s = (
            _CHANGESET_ATTEMPT_STATUS[terminal_status]
            if terminal_status is not None
            else record.status
        )
        if s == RewriteAttempt.APPLIED_COMMITTED:
            self.committed_attempts += 1
            if record.w2_delta_pct is not None:
                self.marginal_results.append({
                    "fn_name": record.fn_name,
                    "rule_id": record.rule_id,
                    "delta_pct": record.w2_delta_pct,
                    "commit_sha": record.commit_sha,
                })
                self.per_rule_gain.setdefault(record.rule_id, []).append(
                    record.w2_delta_pct
                )
        elif s == RewriteAttempt.ABSTAINED:
            self.abstained_count += 1
        elif s == RewriteAttempt.W2_REGRESS:
            self.regressed_count += 1
        elif s == RewriteAttempt.SYNTAX_ERROR:
            self.syntax_failed_count += 1
        elif s == RewriteAttempt.W1_FAIL:
            self.w1_failed_count += 1


def _finalize_w2(
    result: AgentResult,
    w2_session: W2Session,
    final_ops: list[str],
) -> None:
    skipped = w2_session.maybe_skip_final(result.committed_attempts)
    if skipped is not None:
        result.final_measurement = skipped
        return
    if not final_ops:
        return
    final_report = w2_session.measure_final(w2_session.parent_bin, final_ops)
    result.final_measurement = asdict(final_report)
    result.final_total_gain_pct = final_report.aggregate_mean_pct


# ─── F1 2026-08-04: large-fn pre-abstain ──────────────────────────────
                                             
                                                                     
                                                
                                                             
                                                                          
                                                                           
                                              
                                                                    
_LARGE_FN_LINE_THRESHOLD: int = 250
_LARGE_FN_HITS_THRESHOLD: int = 25


_HIT_CLUSTER_RATIO_THRESHOLD: float = 0.30                                             
_HIT_CLUSTER_MIN_HITS: int = 3                                         


def _is_large_fn(hf: HotFunction, hits_count: int,
                  hits: Optional[list[dict]] = None) -> tuple[bool, str]:
    ""                                                        

                                                                 
                                                       

                               
       
    line_span = (hf.line_end - hf.line_start + 1) if hf.line_end else 0
    if line_span > _LARGE_FN_LINE_THRESHOLD:
                                                        
        if hits and len(hits) >= _HIT_CLUSTER_MIN_HITS and line_span > 0:
            hit_lines = [h.get("line", 0) for h in hits if h.get("line")]
            if hit_lines:
                hit_span = max(hit_lines) - min(hit_lines)
                ratio = hit_span / line_span if line_span else 1.0
                if ratio < _HIT_CLUSTER_RATIO_THRESHOLD:
                    return False, ""                 
        return True, (f"fn spans {line_span} lines > {_LARGE_FN_LINE_THRESHOLD} — "
                       f"LLM whole-fn EXECUTE precision drops on large fns")
    if hits_count > _LARGE_FN_HITS_THRESHOLD:
        return True, (f"fn has {hits_count} hits > {_LARGE_FN_HITS_THRESHOLD} — "
                       f"too many interacting rewrite sites for whole-fn EXECUTE")
    return False, ""


# ─── F5 2026-08-04: short-fn pre-abstain ──────────────────────────────
                                                  
                                                                     
                                                           
                                                               
                                                     
_SHORT_FN_LINE_THRESHOLD:  int = 30
_SHORT_FN_HITS_THRESHOLD:  int = 5
_SHORT_FN_RULES_THRESHOLD: int = 1


def _is_short_fn_no_leverage(hf: HotFunction, hits_count: int,
                              unique_rules: int,
                              fired_rules: Optional[list[str]] = None,
                              ) -> tuple[bool, str]:
    ""                                                                   

                                                                            
                                                                           
                                                                               
                                                                             
                                                      

                                              
    rules = set(fired_rules or [])
    if "C3" in rules:
        return False, ""
    line_span = (hf.line_end - hf.line_start + 1) if hf.line_end else 0
    if (0 < line_span < _SHORT_FN_LINE_THRESHOLD
            and hits_count <= _SHORT_FN_HITS_THRESHOLD
            and unique_rules <= _SHORT_FN_RULES_THRESHOLD):
        return True, (f"short fn ({line_span} lines, {hits_count} hits, "
                       f"{unique_rules} rule) — no lift leverage, "
                       f"LLM would silently drop or abstain")
    return False, ""


# ─── P2 2026-08-04: orchestration abstain ─────────────────────────────
                                                                    
                                                                
                                                                
                                                      
                                                     

_ORCH_PATTERNS: tuple[str, ...] = (
    "pthread_", "calloc(", "malloc(", "free(",
    "exit(", "abort(", "perror(",
    "fprintf(stderr", "pthread_mutex_",
)
_COMPUTE_LOOP_PATTERNS: tuple[str, ...] = (
                                                      
    "while ",                                                      
    " for ",                    
    ".iter()", ".iter_mut()",
    "match ",                 
)


def _is_orchestration_fn(hf: HotFunction, crate: Path) -> tuple[bool, str]:
    ""                                                       

                                                                
                                          

        
                                   
                                                            
       
    try:
        src = (crate / hf.file).read_text()
    except (OSError, TypeError):
        return False, ""
    body_lines = src.splitlines()[hf.line_start - 1: hf.line_end]
    non_blank = [l for l in body_lines if l.strip()]
    if len(non_blank) < 5:
        return False, ""
    orch = sum(1 for l in non_blank
               if any(kw in l for kw in _ORCH_PATTERNS))
    compute = sum(1 for l in non_blank
                  if any(kw in l for kw in _COMPUTE_LOOP_PATTERNS))
    if orch >= 5 and orch / len(non_blank) > 0.15 and compute < 2:
        return True, (f"orchestration fn: {orch}/{len(non_blank)} lines "
                       f"are pthread/alloc/exit; only {compute} compute loops")
    return False, ""


# ─── token estimate fallback (impl_plan §4.2) ─────────────────────────
                                                            
def _tokens_estimate(text: str) -> int:
    return len(text) // 4


# ─── §4.7 Step 0.a sanity check (replaces preflight module) ───────────

def _sanity_check(assets: WorkloadAssets, crate: Path, harness_dir: Path,
                   state: StateManager) -> None:
    ""                                                                    
                                                                          
       
    # tree-sitter available (rewrite_applier imports it lazily; check now to fail early)
    import tree_sitter_rust                       # noqa: F401
    from tree_sitter import Language, Parser      # noqa: F401

    if not assets.golden_path.exists():
        raise AgentError(f"golden.jsonl missing: {assets.golden_path}")

    if not state.working_tree_clean():
        raise AgentError(
            f"crate {crate} has uncommitted changes; commit or stash first "
            "(agent rollback = git reset --hard HEAD would wipe them)")

    if not harness_dir.is_dir():
        raise AgentError(f"harness_dir missing: {harness_dir}")

                                                 
    if os.environ.get("PERF_RUN_ENV") != "1":
        if os.environ.get("AGENT_ALLOW_NO_PERF_RUN") == "1":
            logger.warning(
                "[agent] AGENT_ALLOW_NO_PERF_RUN=1 —— bypassing perf_run.sh "
                "check. W2 numbers may be unreliable (CV inflated by "
                "governor/turbo/L3 contention).")
        else:
            raise AgentError(
                "agent 需要在 perf_run.sh 环境下跑(未检测到 PERF_RUN_ENV=1)。\n"
                "  正确起法:\n"
                "    bash /home/anonymous/artifact/PerfTrans/perf_run.sh \\\n"
                "         script_c2rust/.venv/bin/python -m main "
                "--project <NAME> --from perf_opt --run-agent\n"
                "  perf_run.sh 会锁频/关 turbo/RDT 隔离 + 测量核占用体检,\n"
                "  确保 W2 gate 拿到稳定 wall-clock;不加会让 rewrite 的\n"
                "  接受/拒绝判定漂移。\n"
                "  quick debug 可 bypass:AGENT_ALLOW_NO_PERF_RUN=1")


# ─── pre_bin snapshot ─────────────────────────────────────────────────

def _prepare_pre_bin(harness_dir: Path, opt_dir: Path) -> Path:
    ""                                                                  
                                                                         

                                                               
                                                                   
                                                           
                                                            
                                                          
                                             
       
    ok, stderr = cargo_check(harness_dir, timeout_s=600)
    if not ok:
        raise AgentError(
            f"clean release rebuild for pre_bin failed in {harness_dir}:\n"
            f"{(stderr or '')[-800:]}")
    src = harness_dir / _HARNESS_BIN_REL
    if not src.is_file():
        raise AgentError(
            f"pre-harness binary not found at {src}; driver should build "
            "the harness before agent starts")
    dst = opt_dir / "pre_harness_bin"
    shutil.copy2(src, dst)
    dst.chmod(0o755)
    logger.info("[agent] pre_bin snapshot (clean release rebuild) → %s", dst)
    return dst


# ─── LLM client bootstrap (feedback_llm_config_unified) ───────────────

def _init_llm(crate: Path) -> LLMClient:
    model = get_path("DEFAULT_LLM_MODEL")
    if not model:
        raise AgentError("DEFAULT_LLM_MODEL not set in Config/paths.conf")
    llm_dir = crate / ".perf_opt" / "agent_llm"
    llm_dir.mkdir(parents=True, exist_ok=True)
    return LLMClient(model, transcript_path=llm_dir / "transcript.log",
                     cache_path=llm_dir / "cache.jsonl")


# ═════════════════════════════════════════════════════════════════════
                                                     
# ═════════════════════════════════════════════════════════════════════

def _load_fn_hits(opt_dir: Path) -> dict:
    ""                                          
    p = opt_dir / "fn_hits.json"
    if not p.is_file():
        logger.warning("[agent] fn_hits.json missing at %s; agent 只能走 Path C", p)
        return {}
    return json.loads(p.read_text())


def _w2_ops_for_fn(hf: HotFunction, assets: WorkloadAssets,
                    cfg: AgentConfig) -> list[str]:
    ""                                

                                                 
                                                                 
                                                  
       
    if cfg.w2_scope == "hottest_op":
        return [hf.hottest_op] if hf.hottest_op else []
                                                  
    from perf_opt.verify.measure import pick_input
    fn_ops = list((hf.per_op or {}).keys())
    if not fn_ops and hf.hottest_op:
        fn_ops = [hf.hottest_op]
    return [op for op in fn_ops
            if pick_input(assets.harness_src, op) is not None]


def _high_risk_ops_for_fn(hf: HotFunction, ops: list[str]) -> list[str]:
    """Prioritize the hottest mapped workload without changing W2 scope."""
    return [hf.hottest_op] if hf.hottest_op in ops else []


def _all_perf_ops(
    assets: WorkloadAssets, perf_ops: Optional[list[str]] = None
) -> list[str]:
    """Return the full, deterministic set of perf-measurable workload ops."""
    if perf_ops:
        return list(dict.fromkeys(perf_ops))

    from perf_opt.verify.measure import pick_input
    perf_inputs = assets.harness_src / "perf_inputs"
    candidates = (
        sorted(
            path.name.removesuffix(".perf.bin")
            for path in perf_inputs.glob("*.perf.bin")
        )
        if perf_inputs.is_dir() else []
    )
    return [op for op in candidates
            if pick_input(assets.harness_src, op) is not None]


def _run_pristine_w1_preflight(verifier: Verifier, assets: WorkloadAssets) -> None:
    """Reject an invalid starting point before generating any candidate."""
    result = w1_gate(verifier, assets, sample_per_op=0)
    if not result.passed:
        raise AgentError(f"pristine full W1 preflight failed: {result.reason}")


def _truncate_stderr(stderr: str, max_chars: int = STDERR_EXCERPT_CHARS) -> str:
    """cargo stderr, trimmed to what a retry can act on.

    Keeping the tail was the whole rule here, on the theory that the real
    error sits at the end. It does not. A failed build carries far more
    warning lines than error lines, and a warning's `-->` locations are
    indistinguishable from an error's in a line-wise filter — measured on one
    build, 17 location lines against 2 error headings, with the headings
    trimmed away. Select the `error` diagnostics first, THEN trim.

    The trim itself drops whole diagnostics from the END and says how many —
    see `cargo_error_blocks`. Two rules make that survive a second pass over
    text that has already been excerpted once (which is exactly what happens
    on the retry path, where cargo_check has already run the extractor):
    the omission notice is a block the extractor keeps, and dropping is by
    whole diagnostic so a re-run never meets a headless fragment. Both are
    load-bearing. The previous version cut a character window out of the tail
    and re-extracted it, and the leading `... [truncated] ...` had no
    diagnostic head — so the second pass discarded the notice along with the
    partial block behind it. The model was handed 5 of 22 errors formatted as
    if they were all of them.

    Falls back to plain tail-trimming for text that is not cargo output —
    a W1 replay reason comes through here too.
    """
    if not stderr:
        return ""
    focused = cargo_error_blocks(stderr, max_chars=max_chars)
    if focused:
        return focused
    if len(stderr) <= max_chars:
        return stderr
    return "... [head truncated] ...\n" + stderr[-max_chars:]


def _rule_base(rule: str) -> str:
    """`C3.S2` → `C3`. Rules split into sub-strategies; the plan fires the
    base name while execute reports the sub-name."""
    return rule.split(".", 1)[0]


def _short_fn_skips(fired, short_reason: str) -> list[tuple[str, str]]:
    """Every rule the short-fn filter declined, with the reason it declined.

    The filter runs before the model is asked anything, so nothing else will
    ever file these rules — this is their only chance to be accounted for.
    """
    skipped: list[tuple[str, str]] = []
    _account_abstain_reason(fired, [], skipped, f"short_no_leverage: {short_reason}")
    return skipped


def _account_abstain_reason(
    fired: list[str], applied: list[str],
    skipped: list[tuple[str, str]], reason: str | None,
) -> None:
    """Give every unaccounted fired rule the abstain's own reason, in place.

    An abstain DOES account for its rules — in prose, in `extra["reason"]`,
    one level up from where `_reconcile_fired_rules` looks. Without this the
    reason sits in the record while every rule it explains is filed as
    `silently_dropped`, i.e. as a contract violation by the model.

    Measured on lodepng `advanceBits`: `reason` read "fn is < 30 lines and
    has no loop; C3 hoist has no leverage" and C3 was reported in the same
    record as silently dropped. Corpus-wide the mislabel ran 8 of 10 flagged
    rules, all of them reasoned declines — which points the "the cards do not
    fit" conclusion at the cards instead of at this function.

    The PLAN/EXECUTE path already folds `abstained_rules` in the same way;
    only the direct/SIMPLE path lacked it.
    """
    if not reason:
        return
    accounted = {_rule_base(r.strip()) for r in (applied or []) if r.strip()}
    accounted |= {_rule_base(r.strip()) for r, _ in (skipped or []) if r.strip()}
    for rule in fired or []:
        rs = rule.strip()
        if rs and _rule_base(rs) not in accounted:
            skipped.append((rs, reason))
            accounted.add(_rule_base(rs))


def _reconcile_fired_rules(fired: list[str], applied: list[str],
                            skipped: list[tuple[str, str]]) -> list[list[str]]:
    ""                                                   
                                                   

                                                                       
       
    # A fired rule is "covered" if it — OR any of its sub-strategies — was
    # applied or skipped. Rules like C3 split into sub-strategies (C3.S1 sig
    # lift / C3.S2 local hoist); the plan fires the base name "C3" while
    # execute reports the sub-name "C3.S2" applied and "C3.S1" skipped. Match
    # on the base name (before the first ".") so this normal, correct handling
    # is NOT mis-flagged as a silently-dropped contract violation.
    _base = _rule_base

    applied_set = {r.strip() for r in (applied or [])}
    applied_bases = {_base(r) for r in applied_set}
    skipped_map: dict[str, str] = {}
    for r, reason in (skipped or []):
        skipped_map[r.strip()] = reason.strip() or "unspecified"
    skipped_bases = {_base(r) for r in skipped_map}
    for r in fired or []:
        rs = r.strip()
        base = _base(rs)
        covered = (
            rs in applied_set or rs in skipped_map
            or base in applied_bases or base in skipped_bases
        )
        if rs and not covered:
            skipped_map[rs] = "silently_dropped"               
    return [[r, reason] for r, reason in skipped_map.items()]


def _apply_and_gate_legacy(
    edit_target, response: str, applied_rules: list[str],
    hf: HotFunction, harness_dir: Path, post_bin: Path,
    verifier: Verifier, assets: WorkloadAssets, w2_session: W2Session,
    state: StateManager, cfg: AgentConfig,
) -> tuple[RewriteAttempt, dict]:
    ""                                          

                                                  
                                                                 

                                                                   
                                                                 
                            
       
    extra: dict = {}
    outcome = apply_rewrite(
        edit_target, response, harness_dir,
        cargo_timeout_s=cfg.cargo_build_timeout_s)

    if outcome.status == RewriteStatus.ABSTAINED:
        extra["reason"] = outcome.abstain_reason
        return RewriteAttempt.ABSTAINED, extra

    if outcome.status == RewriteStatus.SYNTAX_ERROR:
        extra["error"] = _truncate_stderr(outcome.cargo_stderr)
        extra["cargo_stderr_full"] = outcome.cargo_stderr               
        return RewriteAttempt.SYNTAX_ERROR, extra

                                                         
                                                                   
                                                                 
                             
                                                                    
                                      
                                                                 
                                            
    declared_applied = parse_applied_rules(response) if response else []
    undeclared_exec = find_undeclared_executions(declared_applied, response or "")
                                                                      
                                                                            
    executed_applied = [r for r in declared_applied if r not in undeclared_exec]
    if undeclared_exec:
        logger.warning("[agent] %s: LLM 假申报 rules %s (无 fingerprint);"
                        "F9 partial-strip → executed=%s, 继续 W1/W2 gate",
                        hf.name, undeclared_exec, executed_applied)
        extra["stripped_rules"] = undeclared_exec
    extra["executed_applied_rules"] = executed_applied

                                     
    w1r: W1Result = w1_gate(verifier, assets, sample_per_op=0)
    if not w1r.passed:
        state.rollback()
        extra["w1_result"] = w1r.reason
        extra["error"] = _truncate_stderr(w1r.reason)
        return RewriteAttempt.W1_FAIL, extra

                                    
    ops_to_test = _w2_ops_for_fn(hf, assets, cfg)
    w2v, trig_op = w2_session.gate_ops(
        post_bin, ops_to_test,
        op_weights=(hf.per_op or None),                              
        high_risk_ops=_high_risk_ops_for_fn(hf, ops_to_test),
    )
    if not w2v.ok:
        state.rollback()
        extra["w1_result"] = "pass"
        extra["w2_delta_pct"] = w2v.delta_pct
        extra["measured_cv"] = w2v.measured_cv                                      
        extra["error"] = (f"[{trig_op}] {w2v.reason}" if trig_op else w2v.reason)
        return RewriteAttempt.W2_REGRESS, extra
                                                   
    extra.pop("cargo_stderr_full", None)
    extra["measured_cv"] = w2v.measured_cv                             

    # W1 + W2 both pass → commit
    delta = w2v.delta_pct if w2v.delta_pct is not None else 0.0
    ops_tag = (f"{len(ops_to_test)}op" if cfg.w2_scope == "all_ops"
                else hf.hottest_op)
    msg = (f"agent: {hf.name}/[{','.join(applied_rules)}] — "
           f"APPLIED_COMMITTED (W2 {delta:+.2f}% worst@{trig_op or ops_tag})")
    sha = state.commit_success([edit_target.file], msg)
    extra["w1_result"] = "pass"
    extra["w2_delta_pct"] = w2v.delta_pct
    extra["commit_sha"] = sha
    return RewriteAttempt.APPLIED_COMMITTED, extra


def _execute_local_changeset_and_gate(
    *,
    proposal,
    registry: HandlerRegistry,
    hf: HotFunction,
    harness_dir: Path,
    post_bin: Path,
    verifier: Verifier,
    assets: WorkloadAssets,
    w2_session: W2Session,
    state: StateManager,
    cfg: AgentConfig,
) -> tuple[RewriteAttempt, dict]:
    """Run one LOCAL_FUNCTION proposal through build, full W1, and W2."""

    opt_dir = state.audit_log_path.parent
    applier = ChangeSetApplier(state.crate, opt_dir / ".changeset_txn")
    applier.recover_incomplete()
    gate_details: dict[str, Any] = {}

    def build_callback() -> GateResultRecord:
        ok, stderr = cargo_check(
            harness_dir, timeout_s=cfg.cargo_build_timeout_s
        )
        gate_details["build_stderr"] = stderr
        return GateResultRecord("build", ok, {"stderr": stderr})

    def w1_callback() -> GateResultRecord:
        result: W1Result = w1_gate(verifier, assets, sample_per_op=0)
        gate_details["w1_reason"] = result.reason
        return GateResultRecord(
            "w1", result.passed, {"reason": result.reason}
        )

    def w2_callback(_impact_scope: ImpactScope) -> GateResultRecord:
        ops = _w2_ops_for_fn(hf, assets, cfg)
        verdict, triggering_op = w2_session.gate_ops(
            post_bin,
            ops,
            op_weights=None,
            high_risk_ops=_high_risk_ops_for_fn(hf, ops),
        )
        gate_details.update(
            verdict=verdict, triggering_op=triggering_op, ops=ops
        )
        reason = verdict.reason or ""
        # `insufficient_net_gain` maps to no_gain, NOT regress. Nothing
        # regressed — both harm gates passed; the candidate simply did not
        # earn the relink it costs. The distinction is load-bearing: a
        # REGRESS verdict asks the agent to split the bundle and retry, on
        # the theory that one rule inside is being dragged down by another,
        # and that theory is tested with `w2_delta_pct < 0`. A verdict from
        # the TOTAL gate carries the crate's position against pristine —
        # -68.9% on http-parser — so it is negative whatever the candidate
        # did, and every such rejection would buy three more measurements
        # to re-learn the same answer.
        reason_code = (
            "accepted"
            if verdict.ok
            else "unmeasurable"
            if "unmeasurable" in reason
            else "no_gain"
            if ("no real gain" in reason or "no_gain" in reason
                or "insufficient_net_gain" in reason)
            else "per_op_regress"
        )
        return GateResultRecord(
            "w2",
            verdict.ok,
            {
                "reason": reason_code,
                "detail": reason,
                "delta_pct": verdict.delta_pct,
                "measured_cv": verdict.measured_cv,
                "triggering_op": triggering_op,
                "ops": ops,
                "measurements": _w2_measurements(verdict),
            },
        )

    applied = ChangeSetExecutor(
        crate=state.crate,
        registry=registry,
        applier=applier,
        state=state,
        audit=AuditWriter(opt_dir),
        candidate_binary=post_bin,
        build_gate=build_callback,
        w1_gate=w1_callback,
        w2_gate=w2_callback,
        parent_promoter=w2_session.promote_candidate,
    ).execute(proposal)
    attempt_status, terminal_status = _classify_changeset_terminal_status(
        applied.terminal_status
    )
    verdict = gate_details.get("verdict")
    extra: dict[str, Any] = {"terminal_status": terminal_status}

    if applied.terminal_status is ChangeSetStatus.COMMITTED:
        extra.update(
            w1_result="pass",
            w2_delta_pct=getattr(verdict, "delta_pct", None),
            measured_cv=getattr(verdict, "measured_cv", None),
            commit_sha=applied.commit_sha,
        )
    elif applied.terminal_status is ChangeSetStatus.REJECTED_BUILD:
        stderr = gate_details.get("build_stderr", "")
        extra.update(
            error=_truncate_stderr(stderr), cargo_stderr_full=stderr
        )
    elif applied.terminal_status is ChangeSetStatus.REJECTED_W1:
        reason = gate_details.get("w1_reason", "W1 rejected")
        extra.update(w1_result=reason, error=_truncate_stderr(reason))
    elif applied.terminal_status in {
        ChangeSetStatus.REJECTED_W2_NO_GAIN,
        ChangeSetStatus.REJECTED_W2_REGRESS,
        ChangeSetStatus.REJECTED_UNMEASURABLE,
    }:
        triggering_op = gate_details.get("triggering_op", "")
        reason = getattr(verdict, "reason", applied.terminal_status.value)
        extra.update(
            w1_result="pass",
            w2_delta_pct=getattr(verdict, "delta_pct", None),
            measured_cv=getattr(verdict, "measured_cv", None),
            error=_w2_rejection_line(verdict, triggering_op, reason),
        )
        # A W2 verdict is a stopwatch reading and normally earns no retry —
        # there is nothing to tell the model to fix. The exception is a
        # candidate that also carries a mechanical finding explaining a cost
        # it introduced. Then the rejection is a defect report, and one
        # corrective turn is worth its build. See `find_introduced_scans_in_crate`.
        note = next((v for v in (applied.validations or ())
                     if v.ok and v.code == "introduced_scan"), None)
        if note is not None:
            extra["w2_finding_code"] = note.code
            extra["w2_finding_detail"] = note.detail
    elif applied.terminal_status in {
        ChangeSetStatus.ABSTAINED_UNPROVEN,
        ChangeSetStatus.REJECTED_STALE,
        ChangeSetStatus.REJECTED_CONFLICT,
        ChangeSetStatus.REJECTED_POST_VALIDATION,
    }:
        extra["reason"] = applied.terminal_status.value
        # Carry the failing check out with it. A post-validation rejection is
        # a mechanical finding that names the offending expression — the same
        # kind of feedback a compiler error is, and the only thing that makes
        # another turn worth spending. Without it a retry is a blind reroll.
        failed = [v for v in (applied.validations or ()) if not v.ok]
        if failed:
            extra["validation_code"] = failed[0].code
            extra["validation_detail"] = failed[0].detail
    else:
        extra["error"] = applied.terminal_status.value
    return attempt_status, extra


def _w2_measurements(verdict) -> list:
    """The per-op rows a W2 verdict carries, or none if it carries none.

    Defensive by intent, for the same reason the recording inside the gate is:
    this runs while a gate result is being written, and an AttributeError here
    would turn a measured, passing candidate into a gate failure — the record
    of the measurement destroying its subject. A verdict without the field is
    a verdict from a path that did not measure per-op, which is information,
    not an error.
    """
    detail = getattr(verdict, "detail", None)
    if not isinstance(detail, dict):
        return []
    rows = detail.get("measurements")
    return rows if isinstance(rows, list) else []


def _w2_op_delta(verdict, op: str) -> float | None:
    """The op's measured delta in the gate that produced the verdict.

    The rejection line names the op that vetoed the candidate but not by how
    much, and those are different findings: an op that collapsed and an op
    that drifted a percent on code layout read identically. The magnitude is
    already measured and already written to `changesets/<id>/result.json` —
    it just never reached `rewrites.log`, which is the file anyone opens
    first. Reconstructing it afterwards costs a re-measurement, i.e. as much
    as the run.

    Reads the verdict-producing gate's block first; that is the gate that
    refused. Same defensive posture as `_w2_measurements` — this runs while a
    gate result is being recorded, and must never be the thing that fails.
    """
    if not op:
        return None
    for block in _w2_measurements(verdict):
        if not isinstance(block, dict):
            continue
        for row in block.get("ops") or ():
            if isinstance(row, dict) and row.get("op") == op:
                value = row.get("mean_delta_pct")
                return value if isinstance(value, (int, float)) else None
    return None


def _w2_rejection_line(verdict, triggering_op: str, reason: str) -> str:
    """`[op +23.09%] reason` — the veto with its magnitude attached."""
    if not triggering_op:
        return reason
    delta = _w2_op_delta(verdict, triggering_op)
    if delta is None:
        return f"[{triggering_op}] {reason}"
    return f"[{triggering_op} {delta:+.2f}%] {reason}"


def _planner_abstained(fn_name: str, kind: str,
                       rule_ids: Iterable[str],
                       reason: str | None) -> dict[str, Any]:
    """Surface a planner abstain, and return the `extra` payload for it.

    An abstain here happens BEFORE the changeset ledger, so it writes no
    `changesets/<id>/` directory; and its reason is carried only in an
    `AttemptRecord`, which is never persisted. Without this log line the
    attempt leaves no trace whatsoever once the process exits — the run summary
    can say `abstain=N` and nothing more. That hurts most where it matters
    most: the hottest function in a project is the likeliest to be large and
    rule-dense, hence the likeliest to abstain, and the most expensive one to
    leave unexplained. Measured on brotli: the 57%-self-time function produced
    a rewrite that applied three rules, then vanished with nothing logged.
    """
    mechanical = reporting.is_mechanical_rejection(reason)
    logger.warning(
        "[agent] %s: %s %s (rules=%s): %s",
        fn_name, kind,
        "reply refused on form" if mechanical else "planner abstained",
        ",".join(rule_ids) or "-",
        reason or reporting.PLANNER_ABSTAINED,
    )
    return {"reason": reason,
            "terminal_status": (reporting.REJECTED_FORM if mechanical
                                else reporting.PLANNER_ABSTAINED)}



def _apply_and_gate(
    edit_target, response: str, applied_rules: list[str],
    hf: HotFunction, harness_dir: Path, post_bin: Path,
    verifier: Verifier, assets: WorkloadAssets, w2_session: W2Session,
    state: StateManager, cfg: AgentConfig,
) -> tuple[RewriteAttempt, dict]:
    """Plan a typed function operation, then run the common ChangeSet gates."""
    extra: dict = {}
    declared_applied = parse_applied_rules(response) if response else []
    undeclared_exec = find_undeclared_executions(declared_applied, response or "")
    executed_applied = [rule for rule in declared_applied
                        if rule not in undeclared_exec]
    if undeclared_exec:
        logger.warning(
            "[agent] %s: LLM declared rules %s without fingerprints; "
            "typed execution keeps %s",
            hf.name, undeclared_exec, executed_applied)
        extra["stripped_rules"] = undeclared_exec
    extra["executed_applied_rules"] = executed_applied

    # The added-bounds-check guard used to sit here. It reads the working
    # tree's diff, and nothing has been applied at this point — `applier.apply`
    # runs inside the executor below — so it inspected a tree this candidate
    # had not touched and found nothing, on every candidate. It now runs in
    # each handler's `post_validate`, the first hook after apply, which also
    # gives the region path the same protection it never had. See
    # `ReplaceFunctionHandler.post_validate`.

    candidate_digest = hashlib.sha256((response or "").encode()).hexdigest()[:12]
    planned = plan_llm_function_change(
        response=response,
        edit_target=edit_target,
        rule_ids=tuple(executed_applied or applied_rules),
        hit_ids=(),
        base_head=state.head_sha(full=True),
        candidate_id=f"{hf.name}-{candidate_digest}",
        crate=state.crate,
    )
    if planned.proposal is None:
        extra.update(_planner_abstained(
            hf.name, "whole-fn", executed_applied or applied_rules,
            planned.abstain_reason))
        return RewriteAttempt.ABSTAINED, extra

    operation = planned.proposal.operations[0]
    if not isinstance(operation, ReplaceFunctionBody):
        raise RuntimeError("legacy planner returned a non-function operation")

    registry = HandlerRegistry()
    registry.register(ReplaceFunctionBody, ReplaceFunctionHandler(edit_target))
    attempt_status, gate_extra = _execute_local_changeset_and_gate(
        proposal=planned.proposal,
        registry=registry,
        hf=hf,
        harness_dir=harness_dir,
        post_bin=post_bin,
        verifier=verifier,
        assets=assets,
        w2_session=w2_session,
        state=state,
        cfg=cfg,
    )
    extra.update(gate_extra)
    return attempt_status, extra


def _apply_region_and_gate(
    *,
    proposal,
    hf: HotFunction,
    harness_dir: Path,
    verifier: Verifier,
    assets: WorkloadAssets,
    w2_session: W2Session,
    state: StateManager,
    cfg: AgentConfig,
) -> tuple[RewriteAttempt, dict]:
    """Execute one proven Region ChangeSet through the standard full gates."""

    registry = HandlerRegistry()
    registry.register(ReplaceSourceRegion, ReplaceSourceRegionHandler())
    status, extra = _execute_local_changeset_and_gate(
        proposal=proposal,
        registry=registry,
        hf=hf,
        harness_dir=harness_dir,
        post_bin=harness_dir / _HARNESS_BIN_REL,
        verifier=verifier,
        assets=assets,
        w2_session=w2_session,
        state=state,
        cfg=cfg,
    )
    extra.pop("cargo_stderr_full", None)
    return status, extra


def _region_terminal_record(
    hf: HotFunction,
    *,
    attempt_no: int,
    rule_ids: list[str],
    reason: str,
    terminal_status: str,
    status: RewriteAttempt = RewriteAttempt.ABSTAINED,
    error: Optional[str] = None,
    skipped_rules: Optional[list[list[str]]] = None,
    tokens_in: int = 0,
    anchor_hit_ids: Optional[tuple[str, ...]] = None,
) -> AttemptRecord:
    trace = [reporting.make_trace_entry(
        1, "region", status, terminal_status, error
    )]
    return AttemptRecord(
        fn_name=hf.name,
        rule_id=",".join(rule_ids) if rule_ids else "region",
        round_no=1,
        attempt_no=attempt_no,
        status=status,
        reason=reason,
        error=error,
        tokens_in=tokens_in,
        fired_rules=rule_ids or None,
        skipped_rules=skipped_rules,
        fn_mode="region",
        anchor_hit_ids=(
            list(anchor_hit_ids) if anchor_hit_ids is not None else None
        ),
        **reporting.with_attempt_trace({}, trace),
    )


def _region_hit_id(hit: dict, region, function_name: str) -> str:
    """Reproduce extractor canonicalization for post-commit hit removal."""

    identity = dict(hit)
    identity["file"] = region.relative_path
    return stable_hit_id(identity, function_name)



def _decomposition_key(candidate, applied_rules) -> tuple:
    """Identifies a bundle for the purpose of "have we already split this".

    Keyed on the region's own span plus the rule set, so re-extracting the
    same window after an unrelated commit does not look like a new bundle.
    """
    region = candidate.region
    return (region.relative_path, region.start_byte, region.end_byte,
            tuple(sorted(applied_rules or ())))


def _should_decompose(status, extra: dict, applied_rules, cfg,
                      decomposed: set) -> bool:
    """Is this rejection worth splitting into single-rule retries?

    Only when the rewrite was, on the whole, an improvement that one operation
    voted down. That is the shape where a profitable rule is being outvoted by
    a costly one: measured, a bundle at aggregate -5.64% with four of five ops
    safe. A bundle whose aggregate never improved has no such rule to rescue,
    and each retry costs a full build, W1 and W2.
    """
    if status is RewriteAttempt.APPLIED_COMMITTED:
        return False
    if extra.get("terminal_status") != ChangeSetStatus.REJECTED_W2_REGRESS.value:
        return False
    if len(applied_rules or ()) <= 1:
        return False
    delta = extra.get("w2_delta_pct")
                                      
    return delta is not None and delta < 0


def _not_yet_decomposed(candidate, applied_rules, decomposed: set) -> bool:
    """Guards against splitting the same bundle twice.

    Anchors of a bundle awaiting decomposition are deliberately left in
    `remaining_hits`, so a later re-extraction can rebuild the same window
    with the same rules. Without this the pair would cycle: extract, reject,
    keep anchors, extract again.
    """
    return _decomposition_key(candidate, applied_rules) not in decomposed


def _try_large_fn_regions(
    hf: HotFunction,
    ep: EvidencePack,
    edit_target,
    hits: list[dict],
    *,
    llm: Optional[LLMClient],
    state: StateManager,
    w2_session: W2Session,
    verifier: Verifier,
    assets: WorkloadAssets,
    harness_dir: Path,
    crate: Path,
    cfg: AgentConfig,
    candidate_budget: int,
) -> tuple[list[AttemptRecord], Optional[LLMClient], bool]:
    """Optimize independent proven regions, refreshing all refs after commit."""

    records: list[AttemptRecord] = []

    def _emit(record: AttemptRecord) -> AttemptRecord:
        """Record the attempt AND persist it, before the next region starts.

        Collecting the whole function's records and logging them after the
        loop loses every one of them if the process does not reach the end of
        the loop — while the commits they describe are already in git, made
        one region at a time inside it. Measured on libxml2: a run killed
        during its fifteenth changeset left git holding fifteen agent commits
        and `rewrites.log` holding twelve. The three that vanished were the
        last three, all from one function's region loop, and nothing recorded
        which rules they applied or what they measured — that function had
        taken 65 minutes and thirteen model calls.
        """
        records.append(record)
        state.log(record)
        return record

    committed_any = False
    tokens_used = 0
    processed_hit_ids: set[str] = set()
    local_hits = [
        hit for hit in hits
        if hit_capability(hit) is RuleCapability.REGION_LOCAL
    ]
    cross_hits = [
        hit for hit in hits
        if hit_capability(hit) is RuleCapability.CROSS_FUNCTION
    ]
    unsupported_hits = [
        hit for hit in hits
        if hit_capability(hit) is RuleCapability.UNSUPPORTED
    ]

    if candidate_budget <= 0:
        return records, llm, committed_any

    cross_rules = sorted({hit["rule"] for hit in cross_hits})
    pending_cross_audit = [
        [rule, reporting.CROSS_FN_REQUIRES_CHANGESET_V2]
        for rule in cross_rules
    ]
    if not local_hits:
        if cross_hits and not unsupported_hits:
            _emit(_region_terminal_record(
                hf,
                attempt_no=1,
                rule_ids=cross_rules,
                reason=reporting.ONLY_CROSS_FN_RULES,
                terminal_status=reporting.CROSS_FN_REQUIRES_CHANGESET_V2,
                skipped_rules=pending_cross_audit,
            ))
        else:
            rules = sorted({hit.get("rule", "") for hit in hits if hit.get("rule")})
            _emit(_region_terminal_record(
                hf,
                attempt_no=1,
                rule_ids=rules,
                reason=reporting.NO_REGION_LOCAL_RULES,
                terminal_status=reporting.NO_REGION_LOCAL_RULES,
                skipped_rules=pending_cross_audit or None,
            ))
        return records, llm, committed_any

    if edit_target is None:
        _emit(_region_terminal_record(
            hf,
            attempt_no=1,
            rule_ids=sorted(
                {hit["rule"] for hit in local_hits} | set(cross_rules)
            ),
            reason=reporting.ALL_REGIONS_UNPROVEN,
            terminal_status=reporting.ALL_REGIONS_UNPROVEN,
            skipped_rules=pending_cross_audit or None,
        ))
        return records, llm, committed_any

    extractor = RegionExtractor()
    remaining_hits = list(local_hits)
    current_target = edit_target

    # A subset retry runs its own round, off a queue rather than off a fresh
    # extraction. Re-extracting would rebuild the SAME candidates the round
    # just rejected — including the bundle whose rejection queued the retry —
    # so the loop would re-reject it, queue again, and only stop when the
    # candidate budget ran out. Measured before this was fixed: one function
    # consumed 12 of a run's 29 attempts and about three hours of wall clock
    # re-measuring one identical bundle four times.
    #
    # `in_retry` also bounds the recursion: a retry round may not queue
    # another one. Each iteration therefore either returns, or commits
    # something (which shrinks `remaining_hits`), or runs the one retry round
    # and then returns.
    pending_retries: list | None = None
    in_retry = False
                                                 
    decomposed: set = set()

    while remaining_hits:
        if pending_retries is not None:
            candidates, pending_retries, in_retry = pending_retries, None, True
        else:
            in_retry = False
            extraction = extractor.extract(
                crate=crate, edit_target=current_target, hits=remaining_hits
            )
            candidates = list(extraction.candidates)
        if not candidates:
            semantic_skip_reasons = [
                item.reason
                for item in extraction.skipped
                if item.reason is not RegionSkipReason.DUPLICATE_HIT
            ]
            final_reason = (
                reporting.ALL_REGIONS_OVER_BUDGET
                if semantic_skip_reasons
                and all(
                    reason is RegionSkipReason.REGION_OVER_LIMIT_UNPROVEN
                    for reason in semantic_skip_reasons
                )
                else reporting.ALL_REGIONS_UNPROVEN
            )
            if len(records) < candidate_budget:
                _emit(_region_terminal_record(
                    hf,
                    attempt_no=len(records) + 1,
                    rule_ids=sorted(
                        {hit["rule"] for hit in remaining_hits} | set(cross_rules)
                    ),
                    reason=final_reason,
                    terminal_status=final_reason,
                    skipped_rules=pending_cross_audit or None,
                ))
            return records, llm, committed_any

        refresh_required = False
        subset_retries: list = []
        for index, candidate in enumerate(candidates):
            slots_left = candidate_budget - len(records)
            if slots_left <= 0:
                return records, llm, committed_any

            try:
                system_prompt, user_prompt = build_region_prompt(
                    crate=crate,
                    edit_target=current_target,
                    region=candidate.region,
                    rule_ids=candidate.rule_ids,
                    hits=remaining_hits,
                )
            except (ResolutionRejected, ValueError) as exc:
                # A single window could not be resolved to its declared CST
                # shape (stale hash / sequence not reconstructible), or the
                # prompt builder refused the candidate outright. Either is a
                # per-candidate defect, NOT a run-fatal error — record it as a
                # skip and move to the next candidate. Without this guard one
                # un-resolvable region crashes the whole per-fn pass (and, since
                # the exception is uncaught upstream, the entire project run).
                #
                # `ValueError` is here because that is exactly what happened:
                # the builder raised "candidate rules must exactly match the
                # region anchor hit rules" on a subset-retry candidate and took
                # down a run 11.5 hours in, four commits deep, with no final
                # measurement. The guard above it was written for the same
                # class of failure and named the same consequence; it simply
                # listed one exception type. Anything raised while BUILDING a
                # prompt is about that one candidate.
                logger.warning(
                    "[region] skip un-resolvable candidate fn=%s rules=%s: %s",
                    hf.name, ",".join(candidate.rule_ids), exc,
                )
                if len(records) < candidate_budget:
                    _emit(_region_terminal_record(
                        hf,
                        attempt_no=len(records) + 1,
                        rule_ids=list(candidate.rule_ids),
                        reason=reporting.REGION_RESOLVE_FAILED,
                        terminal_status=reporting.REGION_RESOLVE_FAILED,
                        error=str(exc),
                        anchor_hit_ids=candidate.region.anchor_hit_ids,
                    ))
                processed_hit_ids.update(candidate.region.anchor_hit_ids)
                continue
            tokens_in = (
                _tokens_estimate(system_prompt) + _tokens_estimate(user_prompt)
            )
            if tokens_used + tokens_in > cfg.max_llm_tokens_per_fn:
                error = (
                    f"per-fn region budget: used={tokens_used} + "
                    f"this={tokens_in} > {cfg.max_llm_tokens_per_fn}"
                )
                _emit(_region_terminal_record(
                    hf,
                    attempt_no=len(records) + 1,
                    rule_ids=list(candidate.rule_ids) + cross_rules,
                    reason=reporting.BUDGET_EXCEEDED,
                    terminal_status=reporting.BUDGET_EXCEEDED,
                    status=RewriteAttempt.BUDGET_EXCEEDED,
                    error=error,
                    skipped_rules=pending_cross_audit or None,
                    tokens_in=tokens_in,
                    anchor_hit_ids=candidate.region.anchor_hit_ids,
                ))
                return records, llm, committed_any

            if llm is None:
                llm = _init_llm(crate)
            turn = 0

            def _chat(system: str, user: str) -> str:
                nonlocal turn
                turn += 1
                return llm.chat(
                    system,
                    user,
                    meta={
                        "function": hf.name,
                        "rule_id": ",".join(candidate.rule_ids),
                        "file": candidate.region.relative_path,
                        "target": "region" if turn == 1 else "region-repair",
                    },
                )

            def _candidate_id(reply: str) -> str:
                digest = hashlib.sha256(
                    (candidate.region.expected_region_hash + reply).encode()
                ).hexdigest()[:16]
                return f"{hf.name}-region-{digest}"

            # A rejection the model can be told how to fix earns one more
            # turn — a guard finding, or a W1 stdout mismatch — exactly as
            # on the direct path. Retries were wired there and on the
            # COMPLEX path only; miniz's hot functions are all large enough
            # to route here, so six of its seven guard rejections died with
            # no retry at all and the crate landed at -1.251%.
            repair_note = ""
            # One entry per turn, not one per candidate. A repair turn
            # OVERWRITES `status`/`extra`, so a rejection that was then
            # repaired left no trace of itself: lodepng's `filter` was
            # refused by `added_bounds_check_in_loop`, repaired, and
            # committed, and `rewrites.log` showed only the commit. The
            # guard rejection — the thing worth counting — was invisible in
            # the file anyone opens first. The COMPLEX path already keeps
            # every turn; this one kept one and hard-coded its number.
            trace: list[dict] = []
            for repair_turn in range(2):
                attempt = plan_region_with_contract_repair(
                    chat=_chat,
                    system_prompt=system_prompt,
                    user_prompt=user_prompt + repair_note,
                    candidate_id_for=_candidate_id,
                    token_estimate=_tokens_estimate,
                    budget_remaining=cfg.max_llm_tokens_per_fn - tokens_used,
                    region=candidate.region,
                    rule_ids=candidate.rule_ids,
                    base_head=state.head_sha(full=True),
                    hits=remaining_hits,
                )
                response = attempt.response
                planned = attempt.plan
                tokens_out = attempt.tokens_out
                tokens_used += attempt.tokens_in + attempt.tokens_out
                if attempt.repaired_from is not None:
                    logger.info(
                        "[region] %s: contract repair after %s — %s",
                        hf.name, attempt.repaired_from,
                        "accepted" if planned.proposal is not None
                        else f"still rejected ({planned.abstain_reason})",
                    )

                if planned.proposal is None:
                    status = RewriteAttempt.ABSTAINED
                    extra: dict[str, Any] = _planner_abstained(
                        hf.name, "region", candidate.rule_ids,
                        planned.abstain_reason)
                    applied_rules: list[str] = []
                    skip_reason = (
                        planned.abstain_reason or reporting.PLANNER_ABSTAINED
                    )
                    skipped = [
                        [rule_id, skip_reason]
                        for rule_id in candidate.rule_ids
                    ]
                else:
                    operation = planned.proposal.operations[0]
                    if not isinstance(operation, ReplaceSourceRegion):
                        raise RuntimeError(
                            "region planner returned a non-region operation"
                        )
                    status, extra = _apply_region_and_gate(
                        proposal=planned.proposal,
                        hf=hf,
                        harness_dir=harness_dir,
                        verifier=verifier,
                        assets=assets,
                        w2_session=w2_session,
                        state=state,
                        cfg=cfg,
                    )
                    applied_rules = operation.rule_id.split(",")
                    skipped = _reconcile_fired_rules(
                        list(candidate.rule_ids),
                        applied_rules,
                        parse_skipped_rules(response),
                    )

                pv_code = (extra.get("validation_code")
                           if status == RewriteAttempt.ABSTAINED else None)
                w1_reason = (extra.get("w1_result")
                             if status == RewriteAttempt.W1_FAIL else None)
                w2_finding = (extra.get("w2_finding_code")
                              if status == RewriteAttempt.W2_REGRESS else None)
                subsumed = (find_untested_subsumption(
                                parse_skipped_rules(response))
                            if (status == RewriteAttempt.W2_REGRESS
                                and not w2_finding) else None)
                trace.append(reporting.make_trace_entry(
                    len(trace) + 1,
                    "region",
                    status,
                    extra["terminal_status"],
                    extra.get("error"),
                ))
                if repair_turn == 0 and tokens_used < cfg.max_llm_tokens_per_fn:
                    # A build failure is the one class the compiler names
                    # precisely, and the only one region mode used to end a
                    # candidate on. `_should_decompose` does not rescue it
                    # either — that path is for W2 rejections.
                    if status is RewriteAttempt.SYNTAX_ERROR:
                        build_stderr = (extra.get("cargo_stderr_full")
                                        or extra.get("error") or "")
                        repair_note = (
                            build_repair_note(
                                _truncate_stderr(build_stderr), response)
                            + name_correction_note(crate, build_stderr))
                        logger.info(
                            "[region] %s: build failed — one repair turn",
                            hf.name)
                        continue
                    if is_post_validation_repairable(pv_code):
                        repair_note = post_validation_repair_note(
                            pv_code, extra.get("validation_detail", ""),
                            response)
                        logger.info(
                            "[region] %s: post_validation %s — one repair turn",
                            hf.name, pv_code)
                        continue
                    if w1_reason:
                        repair_note = w1_repair_note(w1_reason, response)
                        logger.info(
                            "[region] %s: W1 failed — one repair turn",
                            hf.name)
                        continue
                    if w2_finding:
                        repair_note = w2_finding_repair_note(
                            w2_finding, extra.get("w2_finding_detail", ""),
                            response)
                        logger.info(
                            "[region] %s: W2 finding %s — one repair turn",
                            hf.name, w2_finding)
                        continue
                    if subsumed is not None:
                        repair_note = subsumption_repair_note(
                            subsumed[0], subsumed[1], response)
                        logger.info(
                            "[region] %s: untested subsumption of %s — one "
                            "repair turn", hf.name, subsumed[0])
                        continue
                break


            fired_for_record = list(candidate.rule_ids)
            if pending_cross_audit:
                fired_for_record.extend(cross_rules)
                skipped.extend(pending_cross_audit)
                pending_cross_audit = []
            record_fields = reporting.with_attempt_trace(extra, trace)
            _emit(AttemptRecord(
                fn_name=hf.name,
                rule_id=",".join(candidate.rule_ids),
                round_no=1,
                attempt_no=len(records) + 1,
                status=status,
                tokens_in=tokens_in,
                tokens_out=tokens_out,
                fired_rules=fired_for_record,
                applied_rules=applied_rules,
                skipped_rules=skipped or None,
                fn_mode="region",
                anchor_hit_ids=list(candidate.region.anchor_hit_ids),
                **record_fields,
            ))
            # Whether this candidate's anchors are spent depends on what
            # happens next. A bundle that is about to be decomposed is NOT
            # finished with them: its rules still have to be tried one at a
            # time, and if the anchors are retired here they vanish from
            # `remaining_hits` and no later extraction can bring them back.
            #
            # That is what happened. A bundle carrying the `qsort` rewrite was
            # rejected, its three rules were queued for individual retry — and
            # its anchors were retired on this line. Another candidate in the
            # same round then committed, which restarts extraction; the queue
            # was rebuilt from `remaining_hits`, the queued rules were no
            # longer in it, and the function finished. The log said
            # "re-offering each rule alone" and offered nothing.
            will_decompose = (
                _should_decompose(status, extra, applied_rules, cfg, decomposed)
                and _not_yet_decomposed(candidate, applied_rules, decomposed)
            )
            if not will_decompose:
                processed_hit_ids.update(candidate.region.anchor_hit_ids)

            if status is RewriteAttempt.APPLIED_COMMITTED:
                committed_any = True
                if len(records) >= candidate_budget:
                    return records, llm, committed_any
                remaining_hits = [
                    hit
                    for hit in remaining_hits
                    if _region_hit_id(hit, candidate.region, hf.name)
                    not in processed_hit_ids
                ]
                if not remaining_hits:
                    return records, llm, committed_any
                current_target = resolve_edit_target(
                    hf,
                    ep,
                    "C1",
                    build_fn_index(crate),
                    crate,
                )
                if current_target is None:
                    if len(records) < candidate_budget:
                        _emit(_region_terminal_record(
                            hf,
                            attempt_no=len(records) + 1,
                            rule_ids=sorted(
                                {hit["rule"] for hit in remaining_hits}
                            ),
                            reason=reporting.ALL_REGIONS_UNPROVEN,
                            terminal_status=reporting.ALL_REGIONS_UNPROVEN,
                        ))
                    return records, llm, committed_any
                refresh_required = True
                break

            # A bundle that measures as no-gain is not evidence that every
            # rule in it was worthless — only that their SUM was. W2 accepts
            # or rejects the whole changeset, so one rule that costs more
            # than it saves discards the ones that would have paid.
            #
            # Observed: a region offered {hoist-invariant-load, pointer→slice}.
            # One run proposed the hoist alone and committed it, measurably
            # faster; another proposed both, W2 measured no gain (the slice
            # form kept a bounds check per probe in a masked ring-buffer
            # scan), and the function shipped with NO rewrite at all. Same
            # function, same rules — the difference was purely which subset
            # the model happened to bundle.
            #
            # So on no-gain with a multi-rule bundle, re-offer the candidate
            # restricted to each rule alone, cheapest signal first. Bounded
            # by the same candidate budget as everything else.
            #
            # REGRESS counts too, and is in fact the stronger case: a flat sum
            # only says the members cancelled, while a regressed sum says one
            # of them is actively costing more than the others save. Measured
            # across two crates, keying this on no-gain alone made it nearly
            # dead code — 2 no-gain rejections against 23 regressions, and the
            # regressions were where the bundles were (`III④`+`C3` on the
            # hottest functions of both crates, which then shipped with no
            # rewrite at all).
            #
            # A catastrophic regression is excluded: past that point the
            # bundle is not "one bad member among good ones", and re-offering
            # each rule costs a full build + W1 + W2 per rule to confirm it.
            delta = extra.get("w2_delta_pct")
            if will_decompose and len(records) < candidate_budget:
                decomposed.add(_decomposition_key(candidate, applied_rules))
                logger.info(
                    "[agent] %s: bundle %s rejected (%s, %s) — re-offering "
                    "each rule alone (a bundle's verdict is its SUM; one "
                    "member can discard the ones that would have paid)",
                    hf.name, ",".join(applied_rules),
                    extra.get("terminal_status"),
                    "delta unknown" if delta is None else f"{delta:+.2f}%")
                # Queued, not spliced into `candidates` mid-iteration, and
                # only for bundles — a solo retry is itself single-rule, so
                # it can never queue another round.
                #
                # A candidate carries BOTH its rules and the anchor hits those
                # rules came from, and the prompt builder requires the two to
                # agree exactly. Narrowing only `rule_ids` leaves the region
                # anchored on every rule's hits, which the builder rejects —
                # it raised on the first subset retry that ever executed and
                # ended the run. So narrow the anchors alongside the rules,
                # and drop a rule that owns no anchor in this region.
                anchor_rule = {
                    _region_hit_id(hit, candidate.region, hf.name):
                        hit.get("rule")
                    for hit in remaining_hits
                }
                # `anchor_hit_ids` and `anchor_lines` are built from one
                # sequence of anchors and are positionally paired, so they must
                # be filtered by the SAME indices.
                region = candidate.region
                # Fewest anchors first. The rejection being rescued is a
                # single operation voting the bundle down, so the rule least
                # likely to have caused it is the one touching the least
                # code — and it is also the cheapest thing to prove. Measured
                # on the bundle that carried the `qsort` rewrite: III① held 1
                # anchor, III④ held 2, C3 held 3; under the previous order
                # (whatever sequence the model happened to write) the 1-anchor
                # rule was tried last and never reached.
                by_rule = {
                    solo: [
                        i for i, hit_id in enumerate(region.anchor_hit_ids)
                        if anchor_rule.get(hit_id) == solo
                    ]
                    for solo in applied_rules
                }
                orphans = [r for r, keep in by_rule.items() if not keep]
                if orphans:
                    logger.info(
                        "[agent] %s: %s own no anchor in this region — not "
                        "retried alone", hf.name, ",".join(orphans))
                for solo, keep in sorted(
                        ((r, k) for r, k in by_rule.items() if k),
                        key=lambda rk: (len(rk[1]), rk[0])):
                    subset_retries.append(dc_replace(
                        candidate,
                        region=dc_replace(
                            region,
                            anchor_hit_ids=tuple(
                                region.anchor_hit_ids[i] for i in keep),
                            anchor_lines=tuple(
                                region.anchor_lines[i] for i in keep),
                        ),
                        rule_ids=(solo,),
                    ))

            if len(records) >= candidate_budget:
                return records, llm, committed_any

        if refresh_required:
            continue
        if (subset_retries and not in_retry
                and len(records) < candidate_budget):
            pending_retries = subset_retries
            continue
        return records, llm, committed_any

    return records, llm, committed_any


def _try_fn_direct(
    hf: HotFunction, ep: EvidencePack, edit_target,
    fired_rules: list[str], hits: list[dict],
    *, llm: LLMClient, state: StateManager, w2_session: W2Session,
    verifier: Verifier, assets: WorkloadAssets, harness_dir: Path, crate: Path,
    cfg: AgentConfig,
) -> AttemptRecord:
    ""                                      
                                                            
                                   
       
    post_bin = harness_dir / _HARNESS_BIN_REL
    sys_p, user = build_multi_card_prompt(
        hf, ep, edit_target, fired_rules, hits, cfg, crate_dir=crate)

    total_tokens_in = 0
    total_tokens_out = 0
    response = ""
    prev_stderr = ""
    prev_output = ""
    prev_contract_reason: str | None = None
    prev_pv: tuple[str, str] | None = None
    prev_w1: str | None = None
    w1_repairs = 0
    prev_w2f: tuple[str, str] | None = None
    prev_subsumed: tuple[str, str] | None = None
    w2f_repairs = 0
    status = None
    extra: dict = {}
    attempt_trace: list[dict] = []

    for attempt in range(1, cfg.max_attempts_per_fn + 1):
        if attempt == 1:
            cur_sys, cur_user = sys_p, user
            target_tag = "direct"
        else:
            if prev_contract_reason:
                # The rewrite was never judged — only its envelope was wrong.
                # See `llm_region.contract_repair_note` for why this is worth
                # a turn: the discarded reply may be entirely correct, and the
                # loss is silent (the run records "the rule did not apply").
                retry_block = "\n\n" + contract_repair_note(
                    prev_contract_reason, tuple(fired_rules), prev_output)
                target_tag = f"direct_repair{attempt}"
            elif prev_pv:
                retry_block = post_validation_repair_note(
                    prev_pv[0], prev_pv[1], prev_output)
                target_tag = f"direct_pvrepair{attempt}"
            elif prev_w1:
                retry_block = w1_repair_note(prev_w1, prev_output)
                target_tag = f"direct_w1repair{attempt}"
            elif prev_w2f:
                retry_block = w2_finding_repair_note(
                    prev_w2f[0], prev_w2f[1], prev_output)
                target_tag = f"direct_w2repair{attempt}"
            elif prev_subsumed:
                retry_block = subsumption_repair_note(
                    prev_subsumed[0], prev_subsumed[1], prev_output)
                target_tag = f"direct_subsumed{attempt}"
            else:
                                               
                retry_block = (
                    "\n\n## Previous attempt failed (syntax_error / cargo build)\n"
                    f"cargo stderr:\n```\n{_truncate_stderr(prev_stderr)}\n```\n\n"
                    f"Your previous output was:\n{quote_previous(prev_output)}\n\n"
                    "Fix the compile error and output the corrected fn "
                    "(same format contract, keep the // Applied rules: header)."
                    # An unresolved name is the one compile error replaying the
                    # stderr cannot fix: it says the name is not there without
                    # saying what is, so the retry gets the same invented name
                    # back. Measured three times running on one function.
                    + name_correction_note(crate, prev_stderr)
                )
                target_tag = f"direct_retry{attempt}"
            cur_sys, cur_user = sys_p, user + retry_block

        tokens_in = _tokens_estimate(cur_sys) + _tokens_estimate(cur_user)
        response = llm.chat(cur_sys, cur_user, meta={
            "function": hf.name, "rule_id": ",".join(fired_rules),
            "file": str(edit_target.file), "target": target_tag,
        })
        tokens_out = _tokens_estimate(response)
        total_tokens_in += tokens_in
        total_tokens_out += tokens_out

        status, extra = _apply_and_gate(
            edit_target, response, fired_rules, hf, harness_dir, post_bin,
            verifier, assets, w2_session, state, cfg)
        attempt_trace.append(reporting.make_trace_entry(
            attempt,
            "direct",
            status,
            extra["terminal_status"],
            extra.get("error"),
        ))

        # Four kinds of failure earn another turn, and only four.
        #
        #  * `syntax_error` — the rewrite exists and does not compile (D3).
        #  * a CONTRACT abstain — the planner never looked at the rewrite,
        #    because the reply was not shaped the way the contract asks. The
        #    same loss cost two whole runs on the region path before it was
        #    given a repair turn; this path has had a retry loop all along and
        #    simply never let this class into it.
        #  * a repairable POST-VALIDATION rejection — a guard read the applied
        #    source and named the offending expression. That is a mechanical
        #    finding like a compile error, not a verdict on speed, and it was
        #    landing in the ABSTAINED bucket where nothing retries. Four runs
        #    of one crate rejected the same function every time and never once
        #    landed it.
        #
        # Everything else still stops here: W1/W2 verdicts are about the
        # rewrite, and a semantic abstain is the model's own judgment.
        contract_reason = (
            extra.get("reason")
            if status == RewriteAttempt.ABSTAINED else None
        )
        if not is_contract_repairable(contract_reason):
            contract_reason = None
        pv_code = (extra.get("validation_code")
                   if status == RewriteAttempt.ABSTAINED else None)
        if not is_post_validation_repairable(pv_code):
            pv_code = None
        #  * a W1 failure. See `w1_repair_note` — a byte-for-byte stdout
        #    mismatch names the op and costs under a second to reach, so it is
        #    the cheapest definite signal in the run. Capped at ONE turn:
        #    unlike a compile error, a second failure means the model does not
        #    understand the transform, not that it mistyped.
        w1_reason = (extra.get("w1_result")
                     if status == RewriteAttempt.W1_FAIL and w1_repairs < 1
                     else None)
        # A W2 rejection retries ONLY when it carries a mechanical finding
        # (see `w2_finding_repair_note`); a bare verdict still stops here.
        w2_finding = (extra.get("w2_finding_code")
                      if status == RewriteAttempt.W2_REGRESS and w2f_repairs < 1
                      else None)
        # The other thing a W2 rejection can carry: a rule the reply dropped
        # by asserting another rule covered it. The rewrite that was supposed
        # to cover it just lost, so the assertion is now evidence against
        # itself. Shares `w2f_repairs` — one W2-triggered turn per candidate.
        subsumed = (find_untested_subsumption(parse_skipped_rules(response))
                    if (status == RewriteAttempt.W2_REGRESS
                        and w2f_repairs < 1 and not w2_finding)
                    else None)
        # A build that ran out of clock is not a rewrite to repair. The
        # source is the same source that compiled before; `cargo_check`
        # already rebuilt it once. Retrying the MODEL here spends a turn
        # telling it to fix a compile error whose stderr reads
        # `cargo build timeout` — it has nothing to fix, so it edits working
        # code. Measured on lodepng `update_adler32`: +7.947%, the run's
        # worst regression, from exactly this turn.
        build_timed_out = (status == RewriteAttempt.SYNTAX_ERROR
                           and is_build_timeout(
                               extra.get("cargo_stderr_full")))
        if ((status != RewriteAttempt.SYNTAX_ERROR or build_timed_out)
                and contract_reason is None and pv_code is None
                and not w1_reason and not w2_finding and subsumed is None):
            break
        if attempt >= cfg.max_attempts_per_fn:
            break
                     
        prev_contract_reason = contract_reason
        prev_pv = ((pv_code, extra.get("validation_detail", ""))
                   if pv_code else None)
        prev_w1 = w1_reason
        if w1_reason:
            w1_repairs += 1
            logger.info("[agent] %s: W1 failed — one repair turn", hf.name)
        prev_w2f = ((w2_finding, extra.get("w2_finding_detail", ""))
                    if w2_finding else None)
        prev_subsumed = subsumed
        if w2_finding or subsumed:
            w2f_repairs += 1
            logger.info("[agent] %s: W2 rejected (%s) — one repair turn",
                        hf.name,
                        w2_finding or f"untested subsumption of {subsumed[0]}")
        prev_stderr = extra.get("cargo_stderr_full", extra.get("error", ""))
        prev_output = response
        logger.info("[agent] %s: %s attempt %d/%d, retrying",
                    hf.name,
                    "contract" if contract_reason else
                    pv_code if pv_code else
                    "w1" if w1_reason else "syntax_error",
                    attempt, cfg.max_attempts_per_fn)

    tokens_in = total_tokens_in
    tokens_out = total_tokens_out

                                                             
                                                                            
                                                                   
                                                 
                                         
    executed = extra.pop("executed_applied_rules", None) if extra else None
    applied = (executed if executed is not None
                else (parse_applied_rules(response) if response else []))
    stripped = extra.pop("stripped_rules", None) if extra else None
    skipped_raw = parse_skipped_rules(response) if response else []
    if stripped:
                                                                  
        for r in stripped:
            skipped_raw.append((r, "no_fingerprint(F9 stripped)"))
    if status == RewriteAttempt.ABSTAINED:
        _account_abstain_reason(
            fired_rules, applied, skipped_raw, (extra or {}).get("reason"))
    skipped_list = _reconcile_fired_rules(fired_rules, applied, skipped_raw)
    dropped = [r for r, why in skipped_list if why == "silently_dropped"]
    if dropped:
        logger.warning("[agent] %s: LLM silently dropped fired rules %s "
                        "(contract violation — no skip reason)",
                        hf.name, dropped)

                                                              
    extra.pop("cargo_stderr_full", None)
    record_fields = reporting.with_attempt_trace(extra, attempt_trace)

    return AttemptRecord(
        fn_name=hf.name,
        rule_id=",".join(fired_rules),
        round_no=1, attempt_no=attempt,
        status=status,
        tokens_in=tokens_in, tokens_out=tokens_out,
        fired_rules=fired_rules,
        applied_rules=applied,
        skipped_rules=skipped_list or None,
        fn_mode="simple",
        **record_fields,
    )


def _try_fn_plan_execute(
    hf: HotFunction, ep: EvidencePack, edit_target,
    fired_rules: list[str], hits: list[dict],
    *, llm: LLMClient, state: StateManager, w2_session: W2Session,
    verifier: Verifier, assets: WorkloadAssets, harness_dir: Path, crate: Path,
    cfg: AgentConfig, caller_lookup: CallerLookup,
) -> AttemptRecord:
    ""                                                        
    post_bin = harness_dir / _HARNESS_BIN_REL

    # ─── 1. PLAN ─────────────────────────────────────────────────────
    plan_sys, plan_user = build_plan_prompt(
        hf, ep, edit_target, fired_rules, hits, cfg, crate_dir=crate)
    plan_tokens_in = _tokens_estimate(plan_sys) + _tokens_estimate(plan_user)
    plan_response = llm.chat(plan_sys, plan_user, meta={
        "function": hf.name, "rule_id": ",".join(fired_rules),
        "file": str(edit_target.file), "target": "plan",
    })
    plan_tokens_out = _tokens_estimate(plan_response)
    plan, err = parse_plan_json(plan_response)
    if plan is None:
        plan_trace = [reporting.make_trace_entry(
            1,
            "plan",
            RewriteAttempt.ABSTAINED,
            "plan_parse_fail",
            err,
        )]
        return AttemptRecord(
            fn_name=hf.name, rule_id=",".join(fired_rules),
            round_no=1, attempt_no=1,
            status=RewriteAttempt.ABSTAINED,
            reason=f"plan_parse_fail: {err}",
            tokens_in=plan_tokens_in, tokens_out=plan_tokens_out,
            fired_rules=fired_rules, fn_mode="complex",
            **reporting.with_attempt_trace({}, plan_trace),
        )
    original_plan = deepcopy(plan)

                                        
                                                                 
                                                                 
    hit_count = len(hits)
    moved_to_abstain: list[dict] = []
    kept_applied: list[dict] = []
    for r in plan.get("applied_rules", []):
        raw_targets = r.get("target_hits")
        valid_targets: list[int] = []
        if isinstance(raw_targets, list):
            for t in raw_targets:
                if isinstance(t, int) and 0 <= t < hit_count:
                    valid_targets.append(t)
        if not valid_targets:
            moved_to_abstain.append({
                "rule": r.get("rule"),
                "reason": f"F6: target_hits invalid or empty "
                          f"(got {raw_targets!r},"
                          f" fn has {hit_count} hits)",
            })
            continue
        r["target_hits"] = valid_targets        
        kept_applied.append(r)
    if moved_to_abstain:
        logger.warning("[agent] %s: F6 moved %d rule(s) to abstain "
                        "(bad target_hits): %s", hf.name,
                        len(moved_to_abstain),
                        [x["rule"] for x in moved_to_abstain])
        plan.setdefault("abstained_rules", []).extend(moved_to_abstain)
    plan["applied_rules"] = kept_applied

    # ─── 2. D18 V1 cross-fn dispatch ─────────────────────────────────
    applied: list[dict] = []
    abstained_cross_fn: list[dict] = []
    for r in plan.get("applied_rules", []):
        if not r.get("needs_cross_fn"):
            applied.append(r)
            continue
        callers = caller_lookup.callers_of(hf.name)
        kinds = {c.kind for c in callers}
        if kinds & {"extern_c_facing", "cross_crate"}:
            rejected = deepcopy(r)
            rejected.update(
                reason="cross_fn_boundary",
                caller_count=len(callers),
                caller_kinds=sorted(kinds),
            )
            abstained_cross_fn.append(rejected)
            continue
        if "rust_same_crate" in kinds:
            rejected = deepcopy(r)
            rejected.update(
                reason="cross_fn_v1_defer_v2",
                caller_count=len(callers),
                caller_kinds=sorted(kinds),
            )
            abstained_cross_fn.append(rejected)
            continue
                                            
        applied.append(r)

    effective_plan = build_effective_plan(
        plan,
        applied_rules=applied,
        dispatcher_abstained=abstained_cross_fn,
    )

    if not applied:
        plan_trace = [reporting.make_trace_entry(
            1,
            "plan",
            RewriteAttempt.ABSTAINED,
            "all_rules_abstained_by_dispatch",
        )]
        return AttemptRecord(
            fn_name=hf.name, rule_id=",".join(fired_rules),
            round_no=1, attempt_no=1,
            status=RewriteAttempt.ABSTAINED,
            reason="all_rules_abstained_by_dispatch",
            tokens_in=plan_tokens_in, tokens_out=plan_tokens_out,
            fired_rules=fired_rules, fn_mode="complex",
            plan_json=effective_plan,
            original_plan_json=original_plan,
            v1_abstained_cross_fn=abstained_cross_fn,
            **reporting.with_attempt_trace({}, plan_trace),
        )

    # ─── 3. EXECUTE (+ P1 2026-08-04:syntax retry loop) ─────────────
                                                             
    #  → syntax_error `use of undeclared crate or module 'libc'`。SIMPLE
                                                       
    applied_rule_ids = [
        r.get("rule")
        for r in effective_plan["applied_rules"]
        if r.get("rule")
    ]
    exec_sys, exec_user = build_execute_prompt(
        hf, ep, edit_target, applied_rule_ids, effective_plan, hits, cfg,
        crate_dir=crate)

    exec_tokens_in_total = 0
    exec_tokens_out_total = 0
    exec_response = ""
    prev_stderr = ""
    prev_output = ""
    prev_pv: tuple[str, str] | None = None
    prev_w1: str | None = None
    w1_repairs = 0
    prev_w2f: tuple[str, str] | None = None
    prev_subsumed: tuple[str, str] | None = None
    w2f_repairs = 0
    status = None
    extra = {}
    attempt_trace: list[dict] = []

    for exec_attempt in range(1, cfg.max_attempts_per_fn + 1):
        if exec_attempt == 1:
            cur_sys, cur_user = exec_sys, exec_user
            target_tag = "execute"
        elif prev_pv:
            retry_block = post_validation_repair_note(
                prev_pv[0], prev_pv[1], prev_output)
            cur_sys, cur_user = exec_sys, exec_user + retry_block
            target_tag = f"execute_pvrepair{exec_attempt}"
        elif prev_w1:
            cur_sys = exec_sys
            cur_user = exec_user + w1_repair_note(prev_w1, prev_output)
            target_tag = f"execute_w1repair{exec_attempt}"
        elif prev_w2f:
            cur_sys = exec_sys
            cur_user = exec_user + w2_finding_repair_note(
                prev_w2f[0], prev_w2f[1], prev_output)
            target_tag = f"execute_w2repair{exec_attempt}"
        elif prev_subsumed:
            cur_sys = exec_sys
            cur_user = exec_user + subsumption_repair_note(
                prev_subsumed[0], prev_subsumed[1], prev_output)
            target_tag = f"execute_subsumed{exec_attempt}"
        else:
            retry_block = (
                "\n\n## Previous attempt failed (syntax_error / cargo build)\n"
                f"cargo stderr:\n```\n{_truncate_stderr(prev_stderr)}\n```\n\n"
                f"Your previous output was:\n{quote_previous(prev_output)}\n\n"
                "Fix the compile error and output the corrected fn (same "
                "format contract, keep the // Applied rules: / // Skipped "
                "rules: headers). Common pitfall: `libc::` is often NOT a "
                "dependency in c2rust output — use `core::ffi::*` / already-"
                "imported extern shims instead."
                + name_correction_note(crate, prev_stderr)
            )
            cur_sys, cur_user = exec_sys, exec_user + retry_block
            target_tag = f"execute_retry{exec_attempt}"

        exec_tokens_in = _tokens_estimate(cur_sys) + _tokens_estimate(cur_user)
        exec_response = llm.chat(cur_sys, cur_user, meta={
            "function": hf.name, "rule_id": ",".join(applied_rule_ids),
            "file": str(edit_target.file), "target": target_tag,
        })
        exec_tokens_out = _tokens_estimate(exec_response)
        exec_tokens_in_total += exec_tokens_in
        exec_tokens_out_total += exec_tokens_out

        status, extra = _apply_and_gate(
            edit_target, exec_response, applied_rule_ids, hf, harness_dir,
            post_bin, verifier, assets, w2_session, state, cfg)
        attempt_trace.append(reporting.make_trace_entry(
            exec_attempt,
            "execute",
            status,
            extra["terminal_status"],
            extra.get("error"),
        ))

        # A guard that read the applied source and named the offending
        # expression is giving the same kind of feedback a compile error does.
        # It reached here as ABSTAINED, which ended the loop; see
        # `is_post_validation_repairable`.
        pv_code = (extra.get("validation_code")
                   if status == RewriteAttempt.ABSTAINED else None)
        if not is_post_validation_repairable(pv_code):
            pv_code = None
        # A W1 failure names the op and the mismatch; see `w1_repair_note`.
        # One turn only.
        w1_reason = (extra.get("w1_result")
                     if status == RewriteAttempt.W1_FAIL and w1_repairs < 1
                     else None)
        w2_finding = (extra.get("w2_finding_code")
                      if status == RewriteAttempt.W2_REGRESS and w2f_repairs < 1
                      else None)
        subsumed = (find_untested_subsumption(
                        parse_skipped_rules(exec_response))
                    if (status == RewriteAttempt.W2_REGRESS
                        and w2f_repairs < 1 and not w2_finding)
                    else None)
        # Same as the direct path: a timeout is the clock's verdict, not the
        # compiler's, and re-asking the model repairs nothing.
        build_timed_out = (status == RewriteAttempt.SYNTAX_ERROR
                           and is_build_timeout(
                               extra.get("cargo_stderr_full")))
        if ((status != RewriteAttempt.SYNTAX_ERROR or build_timed_out)
                and pv_code is None
                and not w1_reason and not w2_finding and subsumed is None):
            break
        if exec_attempt >= cfg.max_attempts_per_fn:
            break
        prev_pv = ((pv_code, extra.get("validation_detail", ""))
                   if pv_code else None)
        prev_w1 = w1_reason
        if w1_reason:
            w1_repairs += 1
            logger.info("[agent] %s: COMPLEX execute W1 failed — one repair "
                        "turn", hf.name)
        prev_w2f = ((w2_finding, extra.get("w2_finding_detail", ""))
                    if w2_finding else None)
        prev_subsumed = subsumed
        if w2_finding or subsumed:
            w2f_repairs += 1
            logger.info("[agent] %s: COMPLEX execute W2 rejected (%s) — one "
                        "repair turn", hf.name,
                        w2_finding or f"untested subsumption of {subsumed[0]}")
        prev_stderr = extra.get("cargo_stderr_full", extra.get("error", ""))
        prev_output = exec_response
        if prev_w1:
            pass
        elif prev_pv:
            logger.info("[agent] %s: COMPLEX execute post_validation "
                        "attempt %d/%d, retrying with %s finding in prompt",
                        hf.name, exec_attempt, cfg.max_attempts_per_fn,
                        prev_pv[0])
        else:
            logger.info("[agent] %s: COMPLEX execute syntax_error attempt "
                        "%d/%d, retrying with stderr in prompt",
                        hf.name, exec_attempt, cfg.max_attempts_per_fn)

    exec_tokens_in = exec_tokens_in_total
    exec_tokens_out = exec_tokens_out_total

                                                                
                                              
    executed = extra.pop("executed_applied_rules", None) if extra else None
    applied_from_output = (executed if executed is not None
        else (parse_applied_rules(exec_response) if exec_response else []))
    stripped = extra.pop("stripped_rules", None) if extra else None
    skipped_raw = (parse_skipped_rules(exec_response)
                    if exec_response else [])
    if stripped:
        for r in stripped:
            skipped_raw.append((r, "no_fingerprint(F9 stripped)"))
                                                          
    for r in effective_plan.get("abstained_rules", []):
        rid = r.get("rule")
        if rid:
            skipped_raw.append((rid, r.get("reason", "plan_abstain")))
    skipped_list = _reconcile_fired_rules(
        fired_rules, applied_from_output, skipped_raw)
    dropped = [r for r, why in skipped_list if why == "silently_dropped"]
    if dropped:
        logger.warning("[agent] %s: LLM silently dropped fired rules %s "
                        "(PLAN/EXEC contract violation)",
                        hf.name, dropped)

                                                              
                                                           
                                                           
    extra.pop("cargo_stderr_full", None)
    record_fields = reporting.with_attempt_trace(extra, attempt_trace)

    return AttemptRecord(
        fn_name=hf.name,
        rule_id=",".join(applied_rule_ids),
        round_no=1, attempt_no=1,
        status=status,
        tokens_in=plan_tokens_in + exec_tokens_in,
        tokens_out=plan_tokens_out + exec_tokens_out,
        fired_rules=fired_rules,
        applied_rules=applied_from_output,
        skipped_rules=skipped_list or None,
        plan_json=effective_plan,
        original_plan_json=original_plan,
        fn_mode="complex",
        v1_abstained_cross_fn=abstained_cross_fn if abstained_cross_fn else None,
        **record_fields,
    )


def _load_evidence(path: Path) -> Optional[EvidencePack]:
    if not path.exists():
        return None
    d = json.loads(path.read_text())["hot_function"]
    prof = d.get("profile", {})
    return EvidencePack(
        symbol=d["symbol"], location=d.get("location"),
        workload=d["workload"], hot_region=d.get("hot_region"),
        rust_source=d.get("rust_source", ""),
        self_time_ratio=prof.get("self_time_ratio", 0.0),
        retired_instructions=prof.get("retired_instructions"),
        cpi=prof.get("cpi"),
        tma=prof.get("tma", {}),
        branch_miss_rate=prof.get("branch_miss_rate"),
        class_i_hits=d.get("class_i_hits", {}),
        class_ii_hits=d.get("class_ii_hits", {}),
        llvm_opt_remarks=d.get("llvm_opt_remarks", []),
        signature=d.get("signature", ""),
        attribution_scope=d.get("attribution_scope", {}),
        tma_bottleneck=d.get("tma_bottleneck", ""),
    )


# ═════════════════════════════════════════════════════════════════════
                                                    
# ═════════════════════════════════════════════════════════════════════

                                                              
                                               
_TYPED_RULES: tuple[str, ...] = ("C9", "C11", "II_const", "II_inl", "II_iso")


def _partition_fn_hits(
    hits: list[dict],
) -> tuple[dict[str, list[dict]], list[dict]]:
    """Split a fn's hits into {typed rule → its hits} and the LLM-bound rest."""
    typed_hits: dict[str, list[dict]] = {}
    llm_hits: list[dict] = []
    for hit in hits:
        rule = hit.get("rule")
        if rule in _TYPED_RULES:
            typed_hits.setdefault(rule, []).append(hit)
        else:
            llm_hits.append(hit)
    return typed_hits, llm_hits


def _apply_ii_const_and_gate(
    hf: HotFunction,
    hits: list[dict],
    harness_dir: Path,
    verifier: Verifier,
    assets: WorkloadAssets,
    w2_session: W2Session,
    state: StateManager,
    cfg: AgentConfig,
    perf_ops: Optional[list[str]] = None,
) -> tuple[RewriteAttempt, dict]:
    """Execute typed II_const operations without constructing an LLM prompt."""
    candidate_seed = json.dumps(hits, sort_keys=True, ensure_ascii=False)
    planned = plan_ii_const(
        hot_function=hf.name,
        hits=hits,
        base_head=state.head_sha(full=True),
        candidate_id=(
            f"{hf.name}-II_const-"
            f"{hashlib.sha256(candidate_seed.encode()).hexdigest()[:12]}"
        ),
    )
    if planned.proposal is None:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, "II_const", ("II_const",), planned.abstain_reason)

    registry = HandlerRegistry()
    registry.register(PromoteStaticToConst, PromoteStaticHandler())
    opt_dir = state.audit_log_path.parent
    applier = ChangeSetApplier(state.crate, opt_dir / ".changeset_txn")
    applier.recover_incomplete()
    post_bin = harness_dir / _HARNESS_BIN_REL
    gate_details: dict[str, Any] = {}

    def build_callback() -> GateResultRecord:
        ok, stderr = cargo_check(harness_dir, timeout_s=cfg.cargo_build_timeout_s)
        gate_details["build_stderr"] = stderr
        return GateResultRecord("build", ok, {"stderr": stderr})

    def w1_callback() -> GateResultRecord:
        result = w1_gate(verifier, assets, sample_per_op=0)
        gate_details["w1_reason"] = result.reason
        return GateResultRecord("w1", result.passed, {"reason": result.reason})

    def w2_callback(impact_scope: ImpactScope) -> GateResultRecord:
                                                                      
                                                       
                                                           
                                                      
                                                     
        #
                                                                      
                                         
                                                  
                                                  
                                                          
                                             
                                                    
        from perf_opt.verify.measure import pick_input
        if hf.hottest_op and pick_input(assets.harness_src, hf.hottest_op) is not None:
            ops = [hf.hottest_op]
        else:
            ops = _w2_ops_for_fn(hf, assets, cfg) or _all_perf_ops(assets, perf_ops)
        verdict, triggering_op = w2_session.gate_ops(
            post_bin,
            ops,
            op_weights=None,
            high_risk_ops=_high_risk_ops_for_fn(hf, ops),
        )
        gate_details.update(verdict=verdict, triggering_op=triggering_op)
        reason = verdict.reason or ""
        # `insufficient_net_gain` maps to no_gain, NOT regress. Nothing
        # regressed — both harm gates passed; the candidate simply did not
        # earn the relink it costs. The distinction is load-bearing: a
        # REGRESS verdict asks the agent to split the bundle and retry, on
        # the theory that one rule inside is being dragged down by another,
        # and that theory is tested with `w2_delta_pct < 0`. A verdict from
        # the TOTAL gate carries the crate's position against pristine —
        # -68.9% on http-parser — so it is negative whatever the candidate
        # did, and every such rejection would buy three more measurements
        # to re-learn the same answer.
        reason_code = (
            "accepted" if verdict.ok else
            "unmeasurable" if "unmeasurable" in reason else
            "no_gain" if ("no_gain" in reason or "no real gain" in reason
                          or "insufficient_net_gain" in reason) else
            "per_op_regress"
        )
        return GateResultRecord("w2", verdict.ok, {
            "reason": reason_code,
            "detail": reason,
            "delta_pct": verdict.delta_pct,
            "measured_cv": verdict.measured_cv,
            "triggering_op": triggering_op,
            "ops": ops,
            "measurements": _w2_measurements(verdict),
        })

    applied = ChangeSetExecutor(
        crate=state.crate,
        registry=registry,
        applier=applier,
        state=state,
        audit=AuditWriter(opt_dir),
        candidate_binary=post_bin,
        build_gate=build_callback,
        w1_gate=w1_callback,
        w2_gate=w2_callback,
        parent_promoter=w2_session.promote_candidate,
    ).execute(planned.proposal)
    verdict = gate_details.get("verdict")
    extra = {
        "commit_sha": applied.commit_sha,
        "w2_delta_pct": getattr(verdict, "delta_pct", None),
        "measured_cv": getattr(verdict, "measured_cv", None),
    }
    attempt_status, terminal_status = _classify_changeset_terminal_status(
        applied.terminal_status
    )
    extra["terminal_status"] = terminal_status
    if applied.terminal_status is ChangeSetStatus.COMMITTED:
        extra["w1_result"] = "pass"
        return attempt_status, extra
    if applied.terminal_status is ChangeSetStatus.REJECTED_W1:
        extra["error"] = gate_details.get("w1_reason", "W1 rejected")
        return attempt_status, extra
    if applied.terminal_status in {
        ChangeSetStatus.REJECTED_W2_NO_GAIN,
        ChangeSetStatus.REJECTED_W2_REGRESS,
        ChangeSetStatus.REJECTED_UNMEASURABLE,
    }:
        extra["error"] = getattr(verdict, "reason", applied.terminal_status.value)
        return attempt_status, extra
    if applied.terminal_status in {
        ChangeSetStatus.ABSTAINED_UNPROVEN,
        ChangeSetStatus.REJECTED_STALE,
        ChangeSetStatus.REJECTED_CONFLICT,
        ChangeSetStatus.REJECTED_POST_VALIDATION,
    }:
        extra["reason"] = applied.terminal_status.value
        return attempt_status, extra
    extra["error"] = gate_details.get("build_stderr", applied.terminal_status.value)
    return attempt_status, extra


def _run_goto_dispatch_strategy(
    strategy: str,
    hf: HotFunction,
    hits: list[dict],
    harness_dir: Path,
    verifier: Verifier,
    assets: WorkloadAssets,
    w2_session: W2Session,
    state: StateManager,
    cfg: AgentConfig,
    perf_ops: Optional[list[str]] = None,
) -> tuple[RewriteAttempt, dict, Optional[ChangeSetStatus]]:
    """One C11 strategy, applied and gated. No prompt is constructed.

    Returns the terminal status alongside the attempt so the caller can tell a
    measurement rejection (try the other strategy) from a build or W1 failure
    (do not).
    """
    candidate_seed = json.dumps(hits, sort_keys=True, ensure_ascii=False)
    planned = plan_goto_dispatch(
        crate=state.crate,
        hot_function=hf.name,
        hits=hits,
        base_head=state.head_sha(full=True),
        candidate_id=(
            f"{hf.name}-C11-"
            f"{hashlib.sha256(candidate_seed.encode()).hexdigest()[:12]}"
        ),
        fn_span=(hf.file, hf.line_start, hf.line_end),
        strategy=strategy,
    )
    if planned.proposal is None:
        return (RewriteAttempt.ABSTAINED,
                _planner_abstained(hf.name, "C11", ("C11",),
                                   planned.abstain_reason),
                None)

    registry = HandlerRegistry()
    registry.register(ReplaceFunctionBody,
                      ReplaceFunctionHandler(planned.edit_target))
    opt_dir = state.audit_log_path.parent
    applier = ChangeSetApplier(state.crate, opt_dir / ".changeset_txn")
    applier.recover_incomplete()
    post_bin = harness_dir / _HARNESS_BIN_REL
    gate_details: dict[str, Any] = {}

    def build_callback() -> GateResultRecord:
        ok, stderr = cargo_check(harness_dir, timeout_s=cfg.cargo_build_timeout_s)
        gate_details["build_stderr"] = stderr
        return GateResultRecord("build", ok, {"stderr": stderr})

    def w1_callback() -> GateResultRecord:
        result = w1_gate(verifier, assets, sample_per_op=0)
        gate_details["w1_reason"] = result.reason
        return GateResultRecord("w1", result.passed, {"reason": result.reason})

    def w2_callback(impact_scope: ImpactScope) -> GateResultRecord:
        # Unlike II_const this touches no crate-global symbol — the state
        # variable is a local — so the ordinary per-fn op set applies and the
        # standard per-op regression gate can stay in force.
        ops = _w2_ops_for_fn(hf, assets, cfg) or _all_perf_ops(assets, perf_ops)
        verdict, triggering_op = w2_session.gate_ops(
            post_bin,
            ops,
            op_weights=None,
            high_risk_ops=_high_risk_ops_for_fn(hf, ops),
        )
        gate_details.update(verdict=verdict, triggering_op=triggering_op)
        reason = verdict.reason or ""
        # `insufficient_net_gain` maps to no_gain, NOT regress. Nothing
        # regressed — both harm gates passed; the candidate simply did not
        # earn the relink it costs. The distinction is load-bearing: a
        # REGRESS verdict asks the agent to split the bundle and retry, on
        # the theory that one rule inside is being dragged down by another,
        # and that theory is tested with `w2_delta_pct < 0`. A verdict from
        # the TOTAL gate carries the crate's position against pristine —
        # -68.9% on http-parser — so it is negative whatever the candidate
        # did, and every such rejection would buy three more measurements
        # to re-learn the same answer.
        reason_code = (
            "accepted" if verdict.ok else
            "unmeasurable" if "unmeasurable" in reason else
            "no_gain" if ("no_gain" in reason or "no real gain" in reason
                          or "insufficient_net_gain" in reason) else
            "per_op_regress"
        )
        return GateResultRecord("w2", verdict.ok, {
            "reason": reason_code,
            "detail": reason,
            "delta_pct": verdict.delta_pct,
            "measured_cv": verdict.measured_cv,
            "triggering_op": triggering_op,
            "ops": ops,
            "measurements": _w2_measurements(verdict),
        })

    applied = ChangeSetExecutor(
        crate=state.crate,
        registry=registry,
        applier=applier,
        state=state,
        audit=AuditWriter(opt_dir),
        candidate_binary=post_bin,
        build_gate=build_callback,
        w1_gate=w1_callback,
        w2_gate=w2_callback,
        parent_promoter=w2_session.promote_candidate,
    ).execute(planned.proposal)
    verdict = gate_details.get("verdict")
    extra = {
        "commit_sha": applied.commit_sha,
        "w2_delta_pct": getattr(verdict, "delta_pct", None),
        "measured_cv": getattr(verdict, "measured_cv", None),
    }
    attempt_status, terminal_status = _classify_changeset_terminal_status(
        applied.terminal_status
    )
    extra["terminal_status"] = terminal_status
    if applied.terminal_status is ChangeSetStatus.COMMITTED:
        extra["w1_result"] = "pass"
        return attempt_status, extra, applied.terminal_status
    if applied.terminal_status is ChangeSetStatus.REJECTED_W1:
        extra["error"] = gate_details.get("w1_reason", "W1 rejected")
        return attempt_status, extra, applied.terminal_status
    if applied.terminal_status in {
        ChangeSetStatus.REJECTED_W2_NO_GAIN,
        ChangeSetStatus.REJECTED_W2_REGRESS,
        ChangeSetStatus.REJECTED_UNMEASURABLE,
    }:
        extra["error"] = getattr(verdict, "reason", applied.terminal_status.value)
        return attempt_status, extra, applied.terminal_status
    if applied.terminal_status in {
        ChangeSetStatus.ABSTAINED_UNPROVEN,
        ChangeSetStatus.REJECTED_STALE,
        ChangeSetStatus.REJECTED_CONFLICT,
        ChangeSetStatus.REJECTED_POST_VALIDATION,
    }:
        extra["reason"] = applied.terminal_status.value
        return attempt_status, extra, applied.terminal_status
    extra["error"] = gate_details.get("build_stderr", applied.terminal_status.value)
    return attempt_status, extra, applied.terminal_status


# Retryable at the measurement gate: the rewrite was correct, it just did not
# pay. That is the signal to try the other strategy, and only that — a build or
# W1 failure means this function is not one we can rewrite at all.
_C11_TRY_NEXT = {
    ChangeSetStatus.REJECTED_W2_NO_GAIN,
    ChangeSetStatus.REJECTED_W2_REGRESS,
    ChangeSetStatus.REJECTED_UNMEASURABLE,
}


def _apply_goto_dispatch_and_gate(
    hf: HotFunction,
    hits: list[dict],
    harness_dir: Path,
    verifier: Verifier,
    assets: WorkloadAssets,
    w2_session: W2Session,
    state: StateManager,
    cfg: AgentConfig,
    perf_ops: Optional[list[str]] = None,
) -> tuple[RewriteAttempt, dict]:
    """C11's two rewrites, tried in order until one is committed.

    They are alternatives, not steps: whichever one takes the dispatch off the
    hot path leaves the other nothing to remove. Narrowing goes first because it
    moves no control flow; the split is reached only when the measurement gate
    says the narrowing bought nothing, which is exactly the case where the
    backend had already threaded the comparison chain away.
    """
    status, extra = RewriteAttempt.ABSTAINED, {}
    for strategy in STRATEGIES:
        status, extra, terminal = _run_goto_dispatch_strategy(
            strategy, hf, hits, harness_dir, verifier, assets, w2_session,
            state, cfg, perf_ops=perf_ops,
        )
        extra["sub_rule_id"] = f"C11.{strategy}"
        if terminal is ChangeSetStatus.COMMITTED:
            return status, extra
        if terminal is not None and terminal not in _C11_TRY_NEXT:
            return status, extra
        logger.info("[c11] %s: %s did not stick (%s) — trying the next strategy",
                    hf.name, strategy, terminal.value if terminal else
                    extra.get("reason", "abstained"))
    return status, extra


def _apply_bitfield_and_gate(
    hf: HotFunction,
    hits: list[dict],
    harness_dir: Path,
    verifier: Verifier,
    assets: WorkloadAssets,
    w2_session: W2Session,
    state: StateManager,
    cfg: AgentConfig,
    perf_ops: Optional[list[str]] = None,
) -> tuple[RewriteAttempt, dict]:
    """Execute C9 bitfield lowering — deterministic, no LLM prompt."""
    candidate_seed = json.dumps(hits, sort_keys=True, ensure_ascii=False)
    planned = plan_bitfield_lowering(
        crate=state.crate,
        hot_function=hf.name,
        hits=hits,
        base_head=state.head_sha(full=True),
        candidate_id=(
            f"{hf.name}-C9-"
            f"{hashlib.sha256(candidate_seed.encode()).hexdigest()[:12]}"
        ),
    )
    if planned.skipped:
        logger.info("[agent] C9 skipped struct(s): %s",
                    "; ".join(planned.skipped))
    if planned.proposal is None:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, "C9", ("C9",), planned.abstain_reason)
    logger.info("[agent] %s: C9 lowering bitfield struct(s) %s",
                hf.name, ", ".join(planned.struct_names))

    registry = HandlerRegistry()
    registry.register(RewriteBitfieldStruct, RewriteBitfieldStructHandler())
    opt_dir = state.audit_log_path.parent
    applier = ChangeSetApplier(state.crate, opt_dir / ".changeset_txn")
    applier.recover_incomplete()
    post_bin = harness_dir / _HARNESS_BIN_REL
    gate_details: dict[str, Any] = {}

    def build_callback() -> GateResultRecord:
        ok, stderr = cargo_check(harness_dir, timeout_s=cfg.cargo_build_timeout_s)
        gate_details["build_stderr"] = stderr
        return GateResultRecord("build", ok, {"stderr": stderr})

    def w1_callback() -> GateResultRecord:
        result = w1_gate(verifier, assets, sample_per_op=0)
        gate_details["w1_reason"] = result.reason
        return GateResultRecord("w1", result.passed, {"reason": result.reason})

    def w2_callback(impact_scope: ImpactScope) -> GateResultRecord:
                                                        
                                                         
                                                  
                                                
                               
                                                        
                              
                                                   
                                                    
        from perf_opt.verify.measure import pick_input
        if hf.hottest_op and pick_input(assets.harness_src, hf.hottest_op) is not None:
            ops = [hf.hottest_op]
        else:
            ops = _w2_ops_for_fn(hf, assets, cfg) or _all_perf_ops(assets, perf_ops)
        verdict, triggering_op = w2_session.gate_ops(
            post_bin,
            ops,
            op_weights=None,
            high_risk_ops=_high_risk_ops_for_fn(hf, ops),
        )
        gate_details.update(verdict=verdict, triggering_op=triggering_op)
        reason = verdict.reason or ""
        # `insufficient_net_gain` maps to no_gain, NOT regress. Nothing
        # regressed — both harm gates passed; the candidate simply did not
        # earn the relink it costs. The distinction is load-bearing: a
        # REGRESS verdict asks the agent to split the bundle and retry, on
        # the theory that one rule inside is being dragged down by another,
        # and that theory is tested with `w2_delta_pct < 0`. A verdict from
        # the TOTAL gate carries the crate's position against pristine —
        # -68.9% on http-parser — so it is negative whatever the candidate
        # did, and every such rejection would buy three more measurements
        # to re-learn the same answer.
        reason_code = (
            "accepted" if verdict.ok else
            "unmeasurable" if "unmeasurable" in reason else
            "no_gain" if ("no_gain" in reason or "no real gain" in reason
                          or "insufficient_net_gain" in reason) else
            "per_op_regress"
        )
        return GateResultRecord("w2", verdict.ok, {
            "reason": reason_code,
            "detail": reason,
            "delta_pct": verdict.delta_pct,
            "measured_cv": verdict.measured_cv,
            "triggering_op": triggering_op,
            "ops": ops,
            "measurements": _w2_measurements(verdict),
        })

    applied = ChangeSetExecutor(
        crate=state.crate,
        registry=registry,
        applier=applier,
        state=state,
        audit=AuditWriter(opt_dir),
        candidate_binary=post_bin,
        build_gate=build_callback,
        w1_gate=w1_callback,
        w2_gate=w2_callback,
        parent_promoter=w2_session.promote_candidate,
    ).execute(planned.proposal)
    verdict = gate_details.get("verdict")
    extra = {
        "commit_sha": applied.commit_sha,
        "w2_delta_pct": getattr(verdict, "delta_pct", None),
        "measured_cv": getattr(verdict, "measured_cv", None),
    }
    attempt_status, terminal_status = _classify_changeset_terminal_status(
        applied.terminal_status
    )
    extra["terminal_status"] = terminal_status
    if applied.terminal_status is ChangeSetStatus.COMMITTED:
        extra["w1_result"] = "pass"
        return attempt_status, extra
    if applied.terminal_status is ChangeSetStatus.REJECTED_W1:
        extra["error"] = gate_details.get("w1_reason", "W1 rejected")
        return attempt_status, extra
    if applied.terminal_status in {
        ChangeSetStatus.REJECTED_W2_NO_GAIN,
        ChangeSetStatus.REJECTED_W2_REGRESS,
        ChangeSetStatus.REJECTED_UNMEASURABLE,
    }:
        extra["error"] = getattr(verdict, "reason", applied.terminal_status.value)
        return attempt_status, extra
    if applied.terminal_status in {
        ChangeSetStatus.ABSTAINED_UNPROVEN,
        ChangeSetStatus.REJECTED_STALE,
        ChangeSetStatus.REJECTED_CONFLICT,
        ChangeSetStatus.REJECTED_POST_VALIDATION,
    }:
        extra["reason"] = applied.terminal_status.value
        return attempt_status, extra
    extra["error"] = gate_details.get("build_stderr", applied.terminal_status.value)
    return attempt_status, extra


                                                           

# ═════════════════════════════════════════════════════════════════════
# II_inl / II_iso: one inlining attribute, no LLM prompt
# ═════════════════════════════════════════════════════════════════════

# (crate, fn) pairs already attempted in this run. A callee named by several
# hot functions is tried once: a second attempt repeats a measurement whose
# answer is already known — committed means the attribute is set, rejected
# means it did not pay.
_INLINE_ATTR_TRIED: set[tuple[str, str]] = set()
_II_INL_CALLEES_PER_FN = 2


def _ii_inl_groups(hf: HotFunction, hits: list[dict], crate: Path,
                   hot_self_pct: dict[str, float]) -> list[list[dict]]:
    """One group (one changeset) per callee worth an attempt, best first."""
    tried = {name for (root, name) in _INLINE_ATTR_TRIED if root == str(crate)}
    chosen = rank_callees(ii_inl_callees(hits), hot_self_pct,
                          exclude=tried | {hf.name.split("::")[-1]},
                          limit=_II_INL_CALLEES_PER_FN)
    return [[{"rule": "II_inl", "pattern": "TooCostly",
              "extra": {"callee": c.name, "cost": c.cost, "n_sites": c.n_sites}}]
            for c in chosen]


def _apply_inline_attr_and_gate(
    hf: HotFunction, *, rule_id: str, target_fn: str, attribute: str,
    hits: list[dict], harness_dir: Path, verifier: Verifier,
    assets: WorkloadAssets, w2_session: W2Session, state: StateManager,
    cfg: AgentConfig, perf_ops: Optional[list[str]], ops_mode: str,
) -> tuple[RewriteAttempt, dict]:
    """Set one `#[inline…]` attribute and run it through the common gates."""
    loc = build_fn_index(state.crate).resolve(target_fn)
    if loc is None:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, rule_id, (rule_id,), f"inline_target_unresolved:{target_fn}")
    relative_path, line_start, _ = loc
    site = locate_inline_site((state.crate / relative_path).read_bytes(),
                              target_fn, line_start)
    if site is None:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, rule_id, (rule_id,), f"inline_target_unresolved:{target_fn}")
    _INLINE_ATTR_TRIED.add((str(state.crate), target_fn))
    candidate_seed = json.dumps([hits, target_fn, attribute], sort_keys=True,
                                ensure_ascii=False, default=str)
    planned = plan_inline_attr(
        rule_id=rule_id, hot_function=hf.name, target_fn=target_fn,
        relative_path=relative_path, line_hint=line_start, attribute=attribute,
        base_head=state.head_sha(full=True),
        candidate_id=(f"{hf.name}-{rule_id}-"
                      f"{hashlib.sha256(candidate_seed.encode()).hexdigest()[:12]}"),
    )
    if planned.proposal is None:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, rule_id, (rule_id,), planned.abstain_reason)

    registry = HandlerRegistry()
    registry.register(SetFunctionInlineAttr, SetFunctionInlineAttrHandler())
    opt_dir = state.audit_log_path.parent
    applier = ChangeSetApplier(state.crate, opt_dir / ".changeset_txn")
    applier.recover_incomplete()
    post_bin = harness_dir / _HARNESS_BIN_REL
    gate_details: dict[str, Any] = {}

    def build_callback() -> GateResultRecord:
        ok, stderr = cargo_check(harness_dir, timeout_s=cfg.cargo_build_timeout_s)
        gate_details["build_stderr"] = stderr
        return GateResultRecord("build", ok, {"stderr": stderr})

    def w1_callback() -> GateResultRecord:
        result = w1_gate(verifier, assets, sample_per_op=0)
        gate_details["w1_reason"] = result.reason
        return GateResultRecord("w1", result.passed, {"reason": result.reason})

    def w2_callback(impact_scope: ImpactScope) -> GateResultRecord:
        # II_inl judges on the hot caller's dominant op, as II_const does: the
        # attribute relinks the crate, and unrelated ops' codegen drifts on a
        # relink. II_iso judges on every op the kernel is hot in, because
        # isolation can pay in one op and cost in another (measured on an
        # LZ4-HC driver: -3% on one strategy's ops, +4% on another's). The
        # total and net-contribution gates inside `gate_ops` cover the rest.
        from perf_opt.verify.measure import pick_input
        if ops_mode == "hottest" and hf.hottest_op and pick_input(
                assets.harness_src, hf.hottest_op) is not None:
            ops = [hf.hottest_op]
        else:
            ops = _w2_ops_for_fn(hf, assets, cfg) or _all_perf_ops(assets, perf_ops)
        verdict, triggering_op = w2_session.gate_ops(
            post_bin, ops, op_weights=None,
            high_risk_ops=_high_risk_ops_for_fn(hf, ops),
        )
        gate_details.update(verdict=verdict, triggering_op=triggering_op)
        reason = verdict.reason or ""
        reason_code = (
            "accepted" if verdict.ok else
            "unmeasurable" if "unmeasurable" in reason else
            "no_gain" if ("no_gain" in reason or "no real gain" in reason
                          or "insufficient_net_gain" in reason) else
            "per_op_regress"
        )
        return GateResultRecord("w2", verdict.ok, {
            "reason": reason_code, "detail": reason,
            "delta_pct": verdict.delta_pct, "measured_cv": verdict.measured_cv,
            "triggering_op": triggering_op, "ops": ops,
            "measurements": _w2_measurements(verdict),
            "target_fn": target_fn, "attribute": attribute,
        })

    applied = ChangeSetExecutor(
        crate=state.crate, registry=registry, applier=applier, state=state,
        audit=AuditWriter(opt_dir), candidate_binary=post_bin,
        build_gate=build_callback, w1_gate=w1_callback, w2_gate=w2_callback,
        parent_promoter=w2_session.promote_candidate,
    ).execute(planned.proposal)
    verdict = gate_details.get("verdict")
    extra = {
        "commit_sha": applied.commit_sha,
        "w2_delta_pct": getattr(verdict, "delta_pct", None),
        "measured_cv": getattr(verdict, "measured_cv", None),
    }
    attempt_status, terminal_status = _classify_changeset_terminal_status(
        applied.terminal_status)
    extra["terminal_status"] = terminal_status
    if applied.terminal_status is ChangeSetStatus.COMMITTED:
        extra["w1_result"] = "pass"
    elif applied.terminal_status is ChangeSetStatus.REJECTED_W1:
        extra["error"] = gate_details.get("w1_reason", "W1 rejected")
    elif applied.terminal_status in {
        ChangeSetStatus.REJECTED_W2_NO_GAIN,
        ChangeSetStatus.REJECTED_W2_REGRESS,
        ChangeSetStatus.REJECTED_UNMEASURABLE,
    }:
        extra["error"] = getattr(verdict, "reason", applied.terminal_status.value)
    elif applied.terminal_status in {
        ChangeSetStatus.ABSTAINED_UNPROVEN,
        ChangeSetStatus.REJECTED_STALE,
        ChangeSetStatus.REJECTED_CONFLICT,
        ChangeSetStatus.REJECTED_POST_VALIDATION,
    }:
        extra["reason"] = applied.terminal_status.value
    else:
        extra["error"] = gate_details.get("build_stderr", applied.terminal_status.value)
    logger.info("[agent] %s: %s %s on %s -> %s", hf.name, rule_id, attribute,
                target_fn, terminal_status)
    return attempt_status, extra


def _apply_ii_inl_and_gate(
    hf: HotFunction, hits: list[dict], harness_dir: Path, verifier: Verifier,
    assets: WorkloadAssets, w2_session: W2Session, state: StateManager,
    cfg: AgentConfig, perf_ops: Optional[list[str]] = None,
) -> tuple[RewriteAttempt, dict]:
    """`#[inline]` / `#[inline(always)]` on one too-costly callee (one group)."""
    extra = (hits[0].get("extra") or {}) if hits else {}
    callee = extra.get("callee")
    if not callee:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, "II_inl", ("II_inl",), "no_ii_inl_callee")
    return _apply_inline_attr_and_gate(
        hf, rule_id="II_inl", target_fn=callee,
        attribute=choose_inline_attribute(extra.get("cost")), hits=hits,
        harness_dir=harness_dir, verifier=verifier, assets=assets,
        w2_session=w2_session, state=state, cfg=cfg, perf_ops=perf_ops,
        ops_mode="hottest")


def _apply_ii_iso_and_gate(
    hf: HotFunction, hits: list[dict], harness_dir: Path, verifier: Verifier,
    assets: WorkloadAssets, w2_session: W2Session, state: StateManager,
    cfg: AgentConfig, perf_ops: Optional[list[str]] = None,
) -> tuple[RewriteAttempt, dict]:
    """`#[inline(never)]` on the hot loop kernel itself."""
    return _apply_inline_attr_and_gate(
        hf, rule_id="II_iso", target_fn=hf.name.split("::")[-1],
        attribute="inline(never)", hits=hits,
        harness_dir=harness_dir, verifier=verifier, assets=assets,
        w2_session=w2_session, state=state, cfg=cfg, perf_ops=perf_ops,
        ops_mode="fn_ops")


# One entry per `_TYPED_RULES` name. Kept as a table rather than an if/else
# chain so a rule added to the tuple without an applier fails loudly at the
# lookup instead of silently running another rule's handler.
_TYPED_APPLIERS = {
    "C9": _apply_bitfield_and_gate,
    "C11": _apply_goto_dispatch_and_gate,
    "II_const": _apply_ii_const_and_gate,
    "II_inl": _apply_ii_inl_and_gate,
    "II_iso": _apply_ii_iso_and_gate,
}
assert set(_TYPED_APPLIERS) == set(_TYPED_RULES)


def _cross_fn_eligible(
    hf: HotFunction, fn_entry: Optional[dict], caller_lookup: CallerLookup,
) -> tuple[bool, list]:
    ""                                                    

                                                             
                                                 
       
    if fn_entry is None:
        return False, []
    hits = fn_entry.get("hits") or []
    # Form E hits are NOT cross-function work. A/B/C/D monomorphise a callback
    # by changing this function's signature and every call site; form E only
    # replaces one `qsort(..., Some(cmp))` inside the body with a native sort.
    # Letting a form-E hit pull the function into the cross-fn path would
    # spend a full signature rewrite on a change that needs none — and the
    # region path, which is where a body-local edit belongs, skips `III①`
    # entirely as `cross_fn_requires_changeset_v2`. Either way the rewrite
    # worth -30.72% on one operation would never be attempted.
    if not any(h.get("rule") == "III①"
               and (h.get("extra") or {}).get("form") != "E"
               for h in hits):
        return False, []
    callers = caller_lookup.callers_of(hf.name)
    if not callers:
        return False, []
    kinds = {c.kind for c in callers}
    if kinds - {"rust_same_crate"}:
        return False, []                                    
    sites = sorted(callers, key=lambda c: (c.file, c.start_byte))
    return True, sites


def _filter_editable_call_sites(
    caller_sites: list, target_rel: str, ts: int, te: int,
) -> tuple[list, Optional[str]]:
    ""                                           

                                               
                                        
                                                                   
                                    
                                                                
                                                            
       
    kept = [
        s for s in caller_sites
        if not (s.file == target_rel and ts <= s.start_byte and s.end_byte <= te)
    ]
    if not kept:
        return [], "only_in_target_call_sites"
    ordered = sorted(kept, key=lambda s: (s.file, s.start_byte, s.end_byte))
    for prev, cur in zip(ordered, ordered[1:]):
        if cur.file == prev.file and cur.start_byte < prev.end_byte:
            return [], "overlapping_call_sites"
    return kept, None


def _apply_cross_fn_and_gate(
    hf: HotFunction,
    edit_target,
    hits: list[dict],
    caller_sites: list,
    *,
    harness_dir: Path,
    verifier: Verifier,
    assets: WorkloadAssets,
    w2_session: W2Session,
    state: StateManager,
    cfg: AgentConfig,
    llm: LLMClient,
    crate: Path,
    perf_ops: Optional[list[str]] = None,
) -> tuple[RewriteAttempt, dict]:
    ""                                                      

                                               
                                             
                                     
       
    xrule = "III①"                                                      
    try:
        card = load_card(xrule)
    except FileNotFoundError as exc:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, "cross-fn", (xrule,), f"cross_fn_card_missing:{exc}")
    tdata = Path(edit_target.file).read_bytes()
    ts, te = edit_target.span
    target_source = tdata[ts:te].decode("utf-8", errors="replace")

                                                     
    target_rel = str(
        Path(edit_target.file).resolve().relative_to(crate.resolve())
    )
    caller_sites, xf_abstain = _filter_editable_call_sites(
        list(caller_sites), target_rel, ts, te
    )
    if xf_abstain is not None:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, "cross-fn", (xrule,), xf_abstain)

    user_prompt = build_cross_fn_prompt(
        fn_name=hf.name, target_source=target_source,
        caller_sites=list(caller_sites), card_text=card,
    )
    sys_prompt = (
        "You are a Rust performance-optimization assistant. Apply the rule "
        "card to the target function AND update every call site so the crate "
        "still compiles. Output MUST follow the contract EXACTLY."
    )
    response = llm.chat(sys_prompt, user_prompt, meta={
        "function": hf.name, "rule_id": xrule,
        "file": str(edit_target.file), "target": "cross_fn",
    })

    hit_ids = tuple(h.get("id") for h in hits if h.get("id"))
    candidate_seed = json.dumps(
        [f"{c.file}:{c.start_byte}-{c.end_byte}" for c in caller_sites],
        sort_keys=True,
    )
    planned = plan_llm_cross_fn_change(
        response=response,
        edit_target=edit_target,
        caller_sites=list(caller_sites),
        rule_ids=(xrule,),
        hit_ids=hit_ids,
        base_head=state.head_sha(full=True),
        candidate_id=(
            f"{hf.name}-xfn-"
            f"{hashlib.sha256(candidate_seed.encode()).hexdigest()[:12]}"
        ),
        crate=crate,
    )
    if planned.proposal is None:
        return RewriteAttempt.ABSTAINED, _planner_abstained(
            hf.name, "cross-fn", ("III①",), planned.abstain_reason)

    registry = HandlerRegistry()
    registry.register(ReplaceFunctionBody, ReplaceFunctionHandler(edit_target))
    registry.register(ReplaceCallSite, ReplaceCallSiteHandler())
    opt_dir = state.audit_log_path.parent
    applier = ChangeSetApplier(state.crate, opt_dir / ".changeset_txn")
    applier.recover_incomplete()
    post_bin = harness_dir / _HARNESS_BIN_REL
    gate_details: dict[str, Any] = {}

    def build_callback() -> GateResultRecord:
        ok, stderr = cargo_check(harness_dir, timeout_s=cfg.cargo_build_timeout_s)
        gate_details["build_stderr"] = stderr
        return GateResultRecord("build", ok, {"stderr": stderr})

    def w1_callback() -> GateResultRecord:
        result = w1_gate(verifier, assets, sample_per_op=0)
        gate_details["w1_reason"] = result.reason
        return GateResultRecord("w1", result.passed, {"reason": result.reason})

    def w2_callback(impact_scope: ImpactScope) -> GateResultRecord:
        ops = _w2_ops_for_fn(hf, assets, cfg) or _all_perf_ops(assets, perf_ops)
        verdict, triggering_op = w2_session.gate_ops(
            post_bin, ops, op_weights=None,
            high_risk_ops=_high_risk_ops_for_fn(hf, ops),
        )
        gate_details.update(verdict=verdict, triggering_op=triggering_op)
        reason = verdict.reason or ""
        # `insufficient_net_gain` maps to no_gain, NOT regress. Nothing
        # regressed — both harm gates passed; the candidate simply did not
        # earn the relink it costs. The distinction is load-bearing: a
        # REGRESS verdict asks the agent to split the bundle and retry, on
        # the theory that one rule inside is being dragged down by another,
        # and that theory is tested with `w2_delta_pct < 0`. A verdict from
        # the TOTAL gate carries the crate's position against pristine —
        # -68.9% on http-parser — so it is negative whatever the candidate
        # did, and every such rejection would buy three more measurements
        # to re-learn the same answer.
        reason_code = (
            "accepted" if verdict.ok else
            "unmeasurable" if "unmeasurable" in reason else
            "no_gain" if ("no_gain" in reason or "no real gain" in reason
                          or "insufficient_net_gain" in reason) else
            "per_op_regress"
        )
        return GateResultRecord("w2", verdict.ok, {
            "reason": reason_code, "detail": reason,
            "delta_pct": verdict.delta_pct, "measured_cv": verdict.measured_cv,
            "triggering_op": triggering_op, "ops": ops,
            "measurements": _w2_measurements(verdict),
        })

    applied = ChangeSetExecutor(
        crate=state.crate,
        registry=registry,
        applier=applier,
        state=state,
        audit=AuditWriter(opt_dir),
        candidate_binary=post_bin,
        build_gate=build_callback,
        w1_gate=w1_callback,
        w2_gate=w2_callback,
        parent_promoter=w2_session.promote_candidate,
    ).execute(planned.proposal)
    verdict = gate_details.get("verdict")
    extra = {
        "commit_sha": applied.commit_sha,
        "w2_delta_pct": getattr(verdict, "delta_pct", None),
        "measured_cv": getattr(verdict, "measured_cv", None),
    }
    attempt_status, terminal_status = _classify_changeset_terminal_status(
        applied.terminal_status
    )
    extra["terminal_status"] = terminal_status
    if applied.terminal_status is ChangeSetStatus.COMMITTED:
        extra["w1_result"] = "pass"
        return attempt_status, extra
    if applied.terminal_status is ChangeSetStatus.REJECTED_W1:
        extra["error"] = gate_details.get("w1_reason", "W1 rejected")
        return attempt_status, extra
    if applied.terminal_status in {
        ChangeSetStatus.REJECTED_W2_NO_GAIN,
        ChangeSetStatus.REJECTED_W2_REGRESS,
        ChangeSetStatus.REJECTED_UNMEASURABLE,
    }:
        extra["error"] = getattr(verdict, "reason", applied.terminal_status.value)
        return attempt_status, extra
    if applied.terminal_status in {
        ChangeSetStatus.ABSTAINED_UNPROVEN,
        ChangeSetStatus.REJECTED_STALE,
        ChangeSetStatus.REJECTED_CONFLICT,
        ChangeSetStatus.REJECTED_POST_VALIDATION,
    }:
        extra["reason"] = applied.terminal_status.value
        return attempt_status, extra
    extra["error"] = gate_details.get("build_stderr", applied.terminal_status.value)
    return attempt_status, extra


def _make_cross_fn_attempt_record(
    hf: HotFunction, status: RewriteAttempt, extra: dict,
) -> AttemptRecord:
    ""                                                           
    trace = [reporting.make_trace_entry(
        1, "cross_fn", status, extra["terminal_status"], extra.get("error"),
    )]
    record_fields = reporting.with_attempt_trace(extra, trace)
    return AttemptRecord(
        fn_name=hf.name,
        rule_id="III①",
        round_no=1, attempt_no=1,
        status=status,
        fired_rules=["III①"],
        applied_rules=(
            ["III①"] if status is RewriteAttempt.APPLIED_COMMITTED else []
        ),
        fn_mode="cross_fn",
        **record_fields,
    )


def _make_typed_attempt_record(
    hf: HotFunction,
    status: RewriteAttempt,
    extra: dict,
    rule_id: str = "II_const",
) -> AttemptRecord:
    """Create the candidate record for one deterministic (typed) execution."""
    # A typed rule with sub-strategies reports which one ran, the same way the
    # LLM path reports `C3.S2`; `_rule_base` splits it back off. It travels in
    # `extra` because the caller's loop is shared by every typed rule — and it
    # must be popped, since `extra` is splatted into AttemptRecord and a key
    # that is not a field is a TypeError that kills the whole run.
    extra = dict(extra)
    rule_id = extra.pop("sub_rule_id", rule_id)
    trace = [reporting.make_trace_entry(
        1,
        "typed",
        status,
        extra["terminal_status"],
        extra.get("error"),
    )]
    record_fields = reporting.with_attempt_trace(extra, trace)
    return AttemptRecord(
        fn_name=hf.name,
        rule_id=rule_id,
        round_no=1,
        attempt_no=1,
        status=status,
        fired_rules=[rule_id],
        applied_rules=(
            [rule_id]
            if status is RewriteAttempt.APPLIED_COMMITTED else []
        ),
        fn_mode="typed",
        **record_fields,
    )


def optimize_hot_fns(
    hot_fns: list[HotFunction], *,
    evidence_dir: Path,
    harness_dir: Path,
    crate: Path,
    assets: WorkloadAssets,
    cfg: AgentConfig,
    opt_dir: Optional[Path] = None,
    perf_ops: Optional[list[str]] = None,
) -> AgentResult:
    """Top-level entry — orchestrates §4.7 Steps 0-4.

    * opt_dir defaults to `crate.parent` (i.e. 3_perf_opt/).
    * Consumes evidence/*.json produced by hot_probe.characterize.
    * Emits rewrites.log + git commits per accepted rewrite.
    """
    opt_dir = opt_dir or crate.parent

    # State first — sanity check needs it to check working_tree_clean
    state = StateManager(crate=crate, audit_log_path=opt_dir / "rewrites.log")
    _sanity_check(assets, crate, harness_dir, state)

    pre_bin = _prepare_pre_bin(harness_dir, opt_dir)
    fn_index = build_fn_index(crate)
    verifier = Verifier(project_dir=crate, binary_dir=harness_dir)
    _run_pristine_w1_preflight(verifier, assets)
    w2_session = W2Session(
        pristine_bin=pre_bin,
        assets=assets,
        opt_dir=opt_dir,
        checkpoints=(
            tuple(cp for cp in cfg.w2_pair_checkpoints if cp <= cfg.max_w2_pairs)
            or (cfg.max_w2_pairs,)
        ),
        per_op_upper_limit_pct=cfg.w2_per_op_upper_limit_pct,
        bystander_op_upper_limit_pct=cfg.w2_bystander_op_upper_limit_pct,
        primary_op_share_pct=cfg.w2_primary_op_share_pct,
        regress_tolerance_pct=cfg.w2_regress_tolerance_pct,
        catastrophic_regress_pct=cfg.w2_catastrophic_regress_pct,
        cumulative_op_share_pct=cfg.w2_cumulative_op_share_pct,
        cumulative_op_limit_pct=cfg.w2_cumulative_op_limit_pct,
        cumulative_op_limit_strict_pct=cfg.w2_cumulative_op_limit_strict_pct,
        min_total_gain_pct=cfg.w2_min_total_gain_pct,
        # The total gate measures these, not the candidate's own ops — see
        # PerformanceSession.all_ops. Same set the final measurement uses, so
        # "every commit passed the total gate" and "the final number" are
        # answers to the same question.
        all_ops=_all_perf_ops(assets, perf_ops),
    )
    llm: Optional[LLMClient] = None
    caller_lookup = CallerLookup(crate)                          

                                                      
    fn_hits = _load_fn_hits(opt_dir)

    result = AgentResult()
    # Step 0.b: filter + sort by self_pct desc (§4.7)
    fns_ordered = sorted(
        [hf for hf in hot_fns
         if hf.file is not None and not hf.extern_wrapper],
        key=lambda h: h.self_pct, reverse=True,
    )

                                                          
    only_fn = os.environ.get("AGENT_ONLY_FN", "").strip()
    if only_fn:
        fns_ordered = [hf for hf in fns_ordered if hf.name == only_fn]
        logger.info("[agent] AGENT_ONLY_FN=%r → filtered to %d fn(s)",
                     only_fn, len(fns_ordered))
    logger.info("[agent] %d hot_fns after filter, hottest=%s @ %.1f%%",
                 len(fns_ordered),
                 fns_ordered[0].name if fns_ordered else "-",
                 fns_ordered[0].self_pct if fns_ordered else 0)

    # ① fair per-fn cap (P0-b): even share of the global budget so a single
    #    large fn cannot starve the rest before every fn is tried once.
    if cfg.per_fn_candidate_cap > 0:
        per_fn_cap = cfg.per_fn_candidate_cap
    else:
        even_share = cfg.max_candidates // max(1, len(fns_ordered))
        per_fn_cap = max(cfg.per_fn_candidate_floor, even_share)
    logger.info("[agent] per-fn candidate cap = %d (global=%d over %d fns)",
                 per_fn_cap, cfg.max_candidates, len(fns_ordered))

    for hf in fns_ordered:
        if result.total_attempts >= cfg.max_candidates:
            result.early_stop_reason = (
                f"candidate budget exhausted ({cfg.max_candidates})"
            )
            logger.info("[agent] STOP: %s", result.early_stop_reason)
            break
        ep = _load_evidence(evidence_dir / f"{hf.name}.json")
        if ep is None:
            logger.warning("[agent] no evidence for %s, skipping", hf.name)
            continue

        fn_entry = (fn_hits.get(hf.name)
                    or fn_hits.get(f"crate::{hf.name}")
                    or None)
        if fn_entry and fn_entry.get("hits"):
            typed_hits, llm_hits = _partition_fn_hits(fn_entry["hits"])
                                                         
                                                      
            for typed_rule in _TYPED_RULES:
                rule_hits = typed_hits.get(typed_rule)
                if not rule_hits:
                    continue
                apply_typed = _TYPED_APPLIERS[typed_rule]
                # II_inl names several callees; each is its own changeset so a
                # callee that does not pay is rejected alone.
                groups = (
                    _ii_inl_groups(hf, rule_hits, crate,
                                   {h.name.split("::")[-1]: h.self_pct
                                    for h in hot_fns})
                    if typed_rule == "II_inl" else [rule_hits]
                )
                for group in groups:
                    typed_status, typed_extra = apply_typed(
                        hf, group, harness_dir, verifier, assets,
                        w2_session, state, cfg, perf_ops=perf_ops,
                    )
                    typed_rec = _make_typed_attempt_record(
                        hf, typed_status, typed_extra, rule_id=typed_rule)
                    state.log(typed_rec)
                    result.add(typed_rec)
                    if typed_status is RewriteAttempt.APPLIED_COMMITTED:
                        caller_lookup.invalidate()
                        fn_index = build_fn_index(crate)
            if not llm_hits:
                continue
            fn_entry = {**fn_entry, "hits": llm_hits}

                                                           
                                                         
                                                        
        #
        #  A committed cross-fn rewrite used to end the function here, on the
        #  grounds that the signature had changed and the recorded hit spans
        #  were stale. It costs far more than it saves. Measured: the hottest
        #  function of a compression crate carries 41 matches, of which 3 are
        #  the cross-fn rule; committing those 3 for +0.03% discarded the
        #  other 38, and a run whose cross-fn attempt ABSTAINED instead went
        #  on to commit a region rewrite worth -8.15% on the same function.
        #  Succeeding at the cheap rewrite must not forfeit the expensive one.
        #
        #  Staleness is not specific to this path: a committed REGION rewrite
        #  shifts the spans of every later hit in the same function, and that
        #  path already handles it by re-resolving the target and re-extracting
        #  regions against the updated source. The intra-fn path below does the
        #  same work, so falling through reaches the same machinery. Anything
        #  that fails to relocate is skipped by the extractor, and anything
        #  that relocates wrongly is caught by the build and W1 gates.
        if os.environ.get("CROSS_FN_V2", "1") != "0" \
                and result.total_attempts < cfg.max_candidates:
            xf_ok, xf_sites = _cross_fn_eligible(hf, fn_entry, caller_lookup)
            if xf_ok:
                                                                   
                                                         
                xf_et = resolve_edit_target(hf, ep, "C1", fn_index, crate)
                if xf_et is not None:
                    xf_hits = [
                        h for h in (fn_entry.get("hits") or [])
                        if h.get("rule") == "III①"
                    ]
                                                                    
                                                                
                                            
                    if llm is None:
                        llm = _init_llm(crate)
                    logger.info(
                        "[agent] %s: cross-fn attempt — %d in-crate call site(s)",
                        hf.name, len(xf_sites),
                    )
                    xf_status, xf_extra = _apply_cross_fn_and_gate(
                        hf, xf_et, xf_hits, xf_sites,
                        harness_dir=harness_dir, verifier=verifier,
                        assets=assets, w2_session=w2_session, state=state,
                        cfg=cfg, llm=llm, crate=crate, perf_ops=perf_ops,
                    )
                    xf_rec = _make_cross_fn_attempt_record(hf, xf_status, xf_extra)
                    state.log(xf_rec)
                    result.add(xf_rec)
                    if xf_status is RewriteAttempt.APPLIED_COMMITTED:
                        caller_lookup.invalidate()
                        fn_index = build_fn_index(crate)
                        # Fall through to the intra-fn path: the remaining
                        # matches in this function are still worth attempting.
                        # The cross-fn matches themselves are now spent —
                        # leaving them in would offer the agent a rule it has
                        # already applied, against a signature that no longer
                        # shows the pattern.
                        fn_entry = {
                            **fn_entry,
                            "hits": [
                                h for h in (fn_entry.get("hits") or [])
                                if h.get("rule") != "III①"
                            ],
                        }
                        if not fn_entry["hits"]:
                            # Nothing intra-fn is left. Falling through with an
                            # empty hit list would fail the `fn_entry["hits"]`
                            # guard below and drop the function into the legacy
                            # D1B path, which exists for functions that had NO
                            # hits at all and has never been exercised against
                            # one whose signature was just rewritten. This
                            # function is done.
                            continue

        # ─── P2 2026-08-04: orchestration abstain ─────────────────────
        is_orch, orch_reason = _is_orchestration_fn(hf, crate)
        if is_orch:
            logger.info("[agent] %s: pre-LLM abstain — %s",
                         hf.name, orch_reason)
            rec = AttemptRecord(
                fn_name=hf.name, rule_id="pre_abstain",
                round_no=1, attempt_no=1,
                status=RewriteAttempt.ABSTAINED,
                reason=f"orchestration_fn: {orch_reason}")
            state.log(rec)
            result.add(rec)
            continue

                                                                 
                                                       
                                    
        if fn_entry and fn_entry.get("hits"):
            hits = fn_entry["hits"]
            fired_rules = sorted({h["rule"] for h in hits})
            # Large-mode detection does not require a resolved source target.
            # Region orchestration owns the structured unproven outcome when
            # resolution fails; non-large missing-target behavior stays skip.
            is_large, large_reason = _is_large_fn(hf, len(hits), hits)
            edit_target = resolve_edit_target(hf, ep, "C1", fn_index, crate)

            # Large functions never enter a whole-function prompt.  Their
            # local hits are split into independently gated CST Regions.
            if is_large:
                logger.info(
                    "[agent] %s: REGION mode — %s", hf.name, large_reason
                )
                region_records, llm, region_committed = _try_large_fn_regions(
                    hf,
                    ep,
                    edit_target,
                    hits,
                    llm=llm,
                    state=state,
                    w2_session=w2_session,
                    verifier=verifier,
                    assets=assets,
                    harness_dir=harness_dir,
                    crate=crate,
                    cfg=cfg,
                    candidate_budget=min(
                        per_fn_cap,
                        max(0, cfg.max_candidates - result.total_attempts),
                    ),
                )
                for rec in region_records:
                    # Already persisted by the region loop's `_emit`, one at a
                    # time — see the note there.
                    result.add(rec)
                if result.total_attempts >= cfg.max_candidates:
                    result.early_stop_reason = (
                        f"candidate budget exhausted ({cfg.max_candidates})"
                    )
                if region_committed:
                    caller_lookup.invalidate()
                    fn_index = build_fn_index(crate)
                continue

            if edit_target is None:
                logger.info("[agent] %s: no editable target (D16/D17 path), "
                             "skipping", hf.name)
                continue

                                                                         
            is_short_nl, short_reason = _is_short_fn_no_leverage(
                hf, len(hits), len(fired_rules), fired_rules)
            if is_short_nl:
                logger.info("[agent] %s: pre-LLM abstain — %s",
                             hf.name, short_reason)
                # Terminal like any other outcome. Left unset, these land
                # in `rewrites.log` with `terminal_status: null` and drop out
                # of every count keyed on it — the filter's own work becomes
                # invisible in the tally of what the run decided.
                rec = AttemptRecord(
                    fn_name=hf.name, rule_id="pre_abstain_short_fn",
                    round_no=1, attempt_no=1,
                    status=RewriteAttempt.ABSTAINED,
                    reason=f"short_no_leverage: {short_reason}",
                    fired_rules=fired_rules,
                    # Every fired rule needs a destination. Left out, the
                    # rules land in `fired` and in neither `applied` nor
                    # `skipped`, and every tally keyed on that pairing reads
                    # them as silently dropped — the one shape the accounting
                    # exists to make impossible.
                    skipped_rules=_short_fn_skips(fired_rules, short_reason),
                    **reporting.with_attempt_trace({}, [
                        reporting.make_trace_entry(
                            1, "direct", RewriteAttempt.ABSTAINED,
                            "pre_abstain", None)]))
                state.log(rec)
                result.add(rec)
                continue

            complex_fn = is_complex_fn(fired_rules, hits, cfg)
            logger.info("[agent] %s: %s mode, %d rules, %d hits",
                         hf.name, "COMPLEX" if complex_fn else "SIMPLE",
                         len(fired_rules), len(hits))

            if llm is None:
                llm = _init_llm(crate)

            if complex_fn:
                rec = _try_fn_plan_execute(
                    hf, ep, edit_target, fired_rules, hits,
                    llm=llm, state=state, w2_session=w2_session,
                    verifier=verifier, assets=assets, harness_dir=harness_dir,
                    crate=crate, cfg=cfg, caller_lookup=caller_lookup)
            else:
                rec = _try_fn_direct(
                    hf, ep, edit_target, fired_rules, hits,
                    llm=llm, state=state, w2_session=w2_session,
                    verifier=verifier, assets=assets, harness_dir=harness_dir,
                    crate=crate, cfg=cfg)

            state.log(rec)
            result.add(rec)

            # A whole-function candidate is a bundle too, and it is judged by
            # its SUM exactly as a region candidate is (see the long note in
            # `_try_large_fn_regions`). The decomposition lived only on the
            # region path, so a bundle rejected here kept every rule in it,
            # including the ones that were paying. Measured on one crate:
            # three of the rejections were `C3` paired with a pointer→slice
            # rewrite on functions holding 26.7%, 7.6% and 3.5% of their
            # operation, all in the band where the pair regresses but the
            # hoist alone had committed elsewhere.
            #
            # Each retry carries one rule, so it cannot decompose further and
            # the loop is one pass. It does NOT stop at the first commit:
            # a commit changes the source, and the candidates planned against
            # the old source are refused by the region hash before they cost
            # anything — resolution happens ahead of the build, W1 and W2, so
            # a stale retry ends in milliseconds. Stopping early to avoid that
            # cost instead threw away every rule after the first success,
            # which is how a rewrite worth -30.72% by hand was never tried.
            #
            # Same predicate as the region path: retiring anchors and queueing
            # retries have to answer the same question the same way.
            solo_delta = rec.w2_delta_pct
            if (_should_decompose(rec.status,
                                  {"terminal_status": rec.terminal_status,
                                   "w2_delta_pct": solo_delta},
                                  rec.applied_rules, cfg, set())
                    and result.total_attempts < cfg.max_candidates):
                logger.info(
                    "[agent] %s: bundle %s rejected (%s, %s) — re-offering "
                    "each rule alone (a bundle's verdict is its SUM; one "
                    "member can discard the ones that would have paid)",
                    hf.name, ",".join(rec.applied_rules),
                    rec.terminal_status,
                    "delta unknown" if solo_delta is None
                    else f"{solo_delta:+.2f}%")
                for solo in list(rec.applied_rules):
                    solo_hits = [h for h in hits if h.get("rule") == solo]
                    if not solo_hits:
                        continue
                    if result.total_attempts >= cfg.max_candidates:
                        break
                    solo_rec = _try_fn_direct(
                        hf, ep, edit_target, [solo], solo_hits,
                        llm=llm, state=state, w2_session=w2_session,
                        verifier=verifier, assets=assets,
                        harness_dir=harness_dir, crate=crate, cfg=cfg)
                    state.log(solo_rec)
                    result.add(solo_rec)
                    if solo_rec.status == RewriteAttempt.APPLIED_COMMITTED:
                        rec = solo_rec
                        # The commit just moved this function. `edit_target`
                        # holds a byte span resolved before the bundle ran, so
                        # every later rule in this loop would splice its
                        # rewrite over the old range — the note below said a
                        # stale candidate dies cheaply in resolution, but that
                        # is the region path, which has a region hash to check
                        # against. Here there is nothing to check: the span is
                        # applied as given, and a span that no longer bounds
                        # the function leaves part of the old body behind.
                        #
                        # Measured: a C7 grew this function from 25 lines to
                        # 48, and the III③ that followed spliced into the old
                        # 25-line range three times, failing the build with
                        # `unexpected closing delimiter` each time — the tail
                        # of the previous body. Three LLM turns and three
                        # builds spent, and the rule was worth several points.
                        caller_lookup.invalidate()
                        fn_index = build_fn_index(crate)
                        edit_target = resolve_edit_target(
                            hf, ep, "C1", fn_index, crate)
                        if edit_target is None:
                            logger.info(
                                "[agent] %s: target no longer resolvable after "
                                "a solo commit — dropping the remaining rules",
                                hf.name)
                            break

                                                
            if rec.status == RewriteAttempt.APPLIED_COMMITTED:
                caller_lookup.invalidate()
            continue                         

                                                   
        #
                                                       
                                                           
                                                              
                                         
        #
                                                  
                                                  
        logger.info("[agent] %s: no fn_hits entry, skip", hf.name)
        state.log(AttemptRecord(
            fn_name=hf.name, rule_id="pre_abstain",
            round_no=1, attempt_no=1,
            status=RewriteAttempt.ABSTAINED,
            reason="no_fn_hits_entry: no rule matched this function",
            **reporting.with_attempt_trace({}, [
                reporting.make_trace_entry(
                    1, "direct", RewriteAttempt.ABSTAINED,
                    "pre_abstain", None)])))

    final_ops = _all_perf_ops(assets, perf_ops)
    _finalize_w2(result, w2_session, final_ops)

    total_wall = (
        f"{result.total_wall_gain_pct:.2f}%"
        if result.total_wall_gain_pct is not None
        else "unmeasured"
    )
    logger.info(
        "[agent] finished: attempts=%d, commit=%d, abstain=%d, "
        "form_rejected=%d, "
        "build_rejected=%d, w1_rejected=%d, w2_no_gain=%d, "
        "w2_regress=%d, unmeasurable=%d, regress_compat=%d, "
        "syntax_or_internal=%d; "
        "direct total wall=%s; tokens_est=%d",
        result.total_attempts,
        result.committed_attempts,
        result.abstained_count,
        result.form_rejected_count,
        result.rejected_build_count,
        result.w1_failed_count,
        result.w2_no_gain_count,
        result.w2_regress_count,
        result.unmeasurable_count,
        result.regressed_count,
        result.syntax_failed_count,
        total_wall,
        result.llm_tokens_used_estimated,
    )
    return result
