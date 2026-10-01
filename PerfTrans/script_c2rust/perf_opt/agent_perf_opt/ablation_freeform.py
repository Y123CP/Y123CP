"""Ablation: hot function → LLM → gates, with the rule layer removed.

Why this file exists
--------------------
In the full pipeline a rule does FOUR jobs, not one:

  ①  selects the target — a hot fn with no rule hit is skipped outright
      (`optimize_hot_fns`, the `no_fn_hits_entry` branch)
  ②  carves the regions — hit positions decide how a large fn is split
  ③  fills the prompt   — the Optimization_Card text
  ④  picks the mode     — `is_complex_fn(fired_rules, hits)` chooses
                          direct vs plan/execute; `_is_short_fn_no_leverage`
                          decides the pre-abstain

This module removes all four. Every hot function, whatever its size, is
handed to the LLM whole with no card and no region split; whatever comes
back goes through the SAME build/W1/W2 gates as the full pipeline.

That last point is the reason the gate helpers are imported from `agent`
rather than reimplemented: if the two arms were judged by two copies of
the gate logic, a difference between the arms could always be blamed on
the copies. They share one implementation, so it cannot.

What is deliberately NOT ablated
--------------------------------
The retry machinery stays. The full pipeline gets to see a compile error
or a W1 mismatch and try again; taking that away here would credit the
rules with a gap that is really "3 attempts vs 1". Only the rule-shaped
repairs are dropped (contract repair, subsumption repair) because there
is no `// Applied rules:` contract and no rule set to subsume.

The domain knowledge that had leaked into the system prompt IS ablated —
see `SYSTEM_FREEFORM`. So is the evidence block: profile percentages and
LLVM optimization remarks are removed from the user prompt, because a
remark that names why a loop did not vectorize carries much of what a card
would have said. What reaches the model is the function, the definitions
of the types it touches, and "make it faster" — see
`build_freeform_prompt`.
"""

from __future__ import annotations

import logging
from pathlib import Path
from typing import Optional

from perf_opt.hot_probe.symbol_source import build_fn_index
from perf_opt.hot_probe.types import EvidencePack, HotFunction
from perf_opt.verify.cargo import Verifier
from perf_opt.verify.workload import WorkloadAssets
from utils.llm_client import LLMClient

from perf_opt.agent_perf_opt.config import AgentConfig
from perf_opt.agent_perf_opt.gates import W2Session
from perf_opt.agent_perf_opt.rewrite_applier import (
    EditTarget,
    is_build_timeout,
    resolve_edit_target,
)
from perf_opt.agent_perf_opt.planners.llm_region import (
    quote_previous,
    w1_repair_note,
)
from perf_opt.agent_perf_opt.type_context import name_correction_note
from perf_opt.agent_perf_opt.state import AttemptRecord, RewriteAttempt, StateManager
from perf_opt.agent_perf_opt import reporting

# The gates, the accounting and the run scaffolding are SHARED with the full
# pipeline on purpose (see module docstring).
from perf_opt.agent_perf_opt.agent import (
    AgentResult,
    _all_perf_ops,
    _apply_and_gate,
    _finalize_w2,
    _init_llm,
    _load_evidence,
    _prepare_pre_bin,
    _run_pristine_w1_preflight,
    _sanity_check,
    _tokens_estimate,
    _truncate_stderr,
)
from perf_opt.agent_perf_opt.prompt_builder import (
    _byte_span_to_lines,
    _extract_current_signature,
    _read_target_source,
    _type_context_for,
)

logger = logging.getLogger(__name__)

_HARNESS_BIN_REL = Path("target/release/harness")

# The rule id every attempt in this arm is logged under, so `rewrites.log`
# stays the same schema and the two arms can be diffed with one reader.
FREEFORM_RULE_ID = "freeform"


# ─── system prompt ────────────────────────────────────────────────────
#
# This is the full pipeline's SYSTEM_MULTI_CARD with the DOMAIN KNOWLEDGE
# stripped and only the FORMAT CONTRACT kept. Three things were removed,
# and naming them matters because leaving any one in would have quietly
# handed this arm the very thing the experiment is trying to withhold:
#
#   * the `CStr::from_ptr` / strlen-scan warning   — that is card C2's finding
#   * the "never add a bounds check the original    — that is the measured
#     did not have, measured at 8.35%" warning        finding behind III③/C3
#   * "read each card's `When to abstain` first"    — there are no cards
#
# What remains is only what the PARSER needs (`parse_llm_response` wants one
# fenced block; the applier wants the attributes kept) plus the abstain
# escape hatch, which is not advice about optimization — it is the protocol
# for saying "nothing to do here".
SYSTEM_FREEFORM = """\
You are a Rust performance optimization agent. You receive one function
that profiling has shown to be hot. Make it faster.

Contract:
  * OUTPUT: the rewritten function wrapped in a single ```rust fenced
    block, and nothing else. No prose before or after.
  * Keep the function's name and signature. Keep ALL original attributes
    (#[inline], #[cold], #[no_mangle], ...).
  * The rewrite MUST preserve observable behaviour exactly. It is checked
    against a byte-for-byte replay of the original program's output.
  * Every `unsafe { ... }` you write must carry an inline `// SAFETY: ...`
    comment stating the precondition you are relying on.
  * If you see no worthwhile optimization, output exactly:
      abstain: <one-line reason>
    and STOP.

You are free to use any technique you judge appropriate. No further
guidance is given.
"""


# ─── prompt assembly ──────────────────────────────────────────────────

def _format_target_full(edit_target: EditTarget,
                        crate_dir: Optional[Path]) -> tuple[str, int]:
    """The target function, WHOLE — no head/tail truncation.

    The full pipeline's `_format_target` routes through
    `_maybe_truncate_source`, which cuts anything over 40_000 chars down to
    head 30 + tail 30 lines. It can afford that because a large function
    there is handled by the region path, which never needs the whole body
    in one prompt.

    This arm has no region path, so truncating would test a strictly
    weaker thing than "hand the model the function": it would test
    "hand the model 60 lines of a 900-line function". The whole body goes
    in, and the caller records how big it was.
    """
    src = _read_target_source(edit_target)
    sig = _extract_current_signature(edit_target)
    ls, le = _byte_span_to_lines(edit_target)
    try:
        rel = edit_target.file.relative_to(crate_dir) if crate_dir else edit_target.file
    except ValueError:
        rel = edit_target.file
    block = (
        f"## Target function\n"
        f"file: {rel}\n"
        f"symbol: {edit_target.fn_name}\n"
        f"signature: {sig}\n"
        f"location: {ls}..{le}  (attrs included)\n\n"
        f"```rust\n{src}\n```"
    )
    return block, len(src)


def build_freeform_prompt(
    edit_target: EditTarget, *, crate_dir: Optional[Path] = None,
) -> tuple[str, str, int]:
    """(system, user, source_chars).

    THREE blocks and nothing else: the whole target function, the
    definitions of the composite types it reaches, and the instruction.

    No card, no hits, no region — and no evidence. The profile numbers and
    the LLVM remarks are gone on purpose: a remark like "loop not
    vectorized: value that could not be identified as reduction is used
    outside the loop" is the compiler naming the obstacle, which is a
    large part of what a card would otherwise have said. Leaving it in
    would have let this arm keep a diluted form of the guidance the
    experiment is trying to withhold.

    The type block stays. It is not a finding about how to make the code
    faster — it is the definition of a struct the function already names,
    which the model cannot see because c2rust puts it in another file. Its
    absence costs compile round-trips, not optimization ideas, and the
    full arm has it too.
    """
    target_block, src_chars = _format_target_full(edit_target, crate_dir)
    user = "\n\n".join([blk for blk in [
        target_block,
        _type_context_for(crate_dir, _read_target_source(edit_target)),
        "## Instruction\n"
        "Rewrite this function to run faster, preserving behaviour "
        "exactly. Output the full rewritten function in one ```rust "
        "block, or `abstain: <reason>`.",
    ] if blk])
    return SYSTEM_FREEFORM, user, src_chars


# ─── one function, one candidate ──────────────────────────────────────

def _try_fn_freeform(
    hf: HotFunction, ep: EvidencePack, edit_target: EditTarget,
    *, llm: LLMClient, state: StateManager, w2_session: W2Session,
    verifier: Verifier, assets: WorkloadAssets, harness_dir: Path,
    crate: Path, cfg: AgentConfig,
) -> AttemptRecord:
    """One hot function → LLM → build/W1/W2, with the retry budget the full
    pipeline gets.

    Retries kept (they are general agent competence, not rule knowledge):
      * a compile error — replay the stderr
      * a W1 mismatch  — replay the failing op, ONE turn (same cap as the
        full pipeline: a second failure means the model does not understand
        the transform, not that it mistyped)

    Retries dropped (rule-shaped, nothing here to repair):
      * contract repair — there is no `// Applied rules:` header
      * subsumption repair — there is no rule set to subsume
      * post-validation repair — the guards it feeds on are rule guards
    """
    post_bin = harness_dir / _HARNESS_BIN_REL
    # `ep` is loaded but deliberately not passed: this arm keeps the full
    # arm's "no evidence → skip" condition so both arms process exactly the
    # same function set, while the evidence itself stays out of the prompt.
    sys_p, base_user, src_chars = build_freeform_prompt(
        edit_target, crate_dir=crate)

    total_tokens_in = 0
    total_tokens_out = 0
    prev_stderr = ""
    prev_output = ""
    prev_w1: str | None = None
    w1_repairs = 0
    status = None
    extra: dict = {}
    attempt_trace: list[dict] = []
    response = ""

    for attempt in range(1, cfg.max_attempts_per_fn + 1):
        if attempt == 1:
            cur_user = base_user
            target_tag = "freeform"
        elif prev_w1:
            cur_user = base_user + "\n\n" + w1_repair_note(prev_w1, prev_output)
            target_tag = f"freeform_w1repair{attempt}"
        else:
            cur_user = base_user + (
                "\n\n## Previous attempt failed to compile\n"
                f"cargo stderr:\n```\n{_truncate_stderr(prev_stderr)}\n```\n\n"
                f"Your previous output was:\n{quote_previous(prev_output)}\n\n"
                "Fix the compile error and output the corrected function "
                "(same format contract)."
                + name_correction_note(crate, prev_stderr)
            )
            target_tag = f"freeform_retry{attempt}"

        total_tokens_in += _tokens_estimate(sys_p) + _tokens_estimate(cur_user)
        response = llm.chat(sys_p, cur_user, meta={
            "function": hf.name, "rule_id": FREEFORM_RULE_ID,
            "file": str(edit_target.file), "target": target_tag,
            "arm": "ablation_freeform",
        })
        total_tokens_out += _tokens_estimate(response)

        # THE SHARED JUDGE. Same call the full pipeline makes — planning,
        # ChangeSet execution, build, W1, W2, commit-or-rollback.
        status, extra = _apply_and_gate(
            edit_target, response, [FREEFORM_RULE_ID], hf, harness_dir,
            post_bin, verifier, assets, w2_session, state, cfg)
        attempt_trace.append(reporting.make_trace_entry(
            attempt, "freeform", status, extra["terminal_status"],
            extra.get("error")))

        build_timed_out = (status == RewriteAttempt.SYNTAX_ERROR
                           and is_build_timeout(extra.get("cargo_stderr_full")))
        w1_reason = (extra.get("w1_result")
                     if status == RewriteAttempt.W1_FAIL and w1_repairs < 1
                     else None)
        retryable = ((status == RewriteAttempt.SYNTAX_ERROR
                      and not build_timed_out) or bool(w1_reason))
        if not retryable or attempt >= cfg.max_attempts_per_fn:
            break

        prev_w1 = w1_reason
        if w1_reason:
            w1_repairs += 1
            logger.info("[freeform] %s: W1 failed — one repair turn", hf.name)
        else:
            prev_w1 = None
            prev_stderr = extra.get("build_stderr", "") or ""
        prev_output = response

    return AttemptRecord(
        fn_name=hf.name,
        rule_id=FREEFORM_RULE_ID,
        round_no=1,
        attempt_no=len(attempt_trace),
        status=status,
        w1_result=extra.get("w1_result"),
        w2_delta_pct=extra.get("w2_delta_pct"),
        commit_sha=extra.get("commit_sha"),
        tokens_in=total_tokens_in,
        tokens_out=total_tokens_out,
        error=extra.get("error"),
        reason=extra.get("reason"),
        # The ablation arm's own accounting: how big the function actually
        # was. A large-fn failure is the region design's motivation showing
        # up as data, and without this column it is invisible.
        fn_mode=f"freeform:{src_chars}chars",
        measured_cv=extra.get("measured_cv"),
        terminal_status=extra.get("terminal_status"),
        attempt_trace=attempt_trace,
        validation_code=extra.get("validation_code"),
        validation_detail=extra.get("validation_detail"),
        w2_finding_code=extra.get("w2_finding_code"),
        w2_finding_detail=extra.get("w2_finding_detail"),
    )


# ─── top-level entry ──────────────────────────────────────────────────

def optimize_hot_fns_freeform(
    hot_fns: list[HotFunction], *,
    evidence_dir: Path,
    harness_dir: Path,
    crate: Path,
    assets: WorkloadAssets,
    cfg: AgentConfig,
    opt_dir: Optional[Path] = None,
    perf_ops: Optional[list[str]] = None,
) -> AgentResult:
    """Ablation entry — mirrors `optimize_hot_fns` minus the rule layer.

    Every hot function is attempted. Nothing is filtered by rule hits, no
    function is judged too large, no pre-abstain heuristic runs. The only
    functions skipped are those with no resolvable source target, which is
    a mechanical failure, not a decision.
    """
    import os

    opt_dir = opt_dir or crate.parent
    state = StateManager(crate=crate, audit_log_path=opt_dir / "rewrites.log")
    _sanity_check(assets, crate, harness_dir, state)

    pre_bin = _prepare_pre_bin(harness_dir, opt_dir)
    fn_index = build_fn_index(crate)
    verifier = Verifier(project_dir=crate, binary_dir=harness_dir)
    _run_pristine_w1_preflight(verifier, assets)

    # Identical W2 configuration to the full pipeline — same checkpoints,
    # same per-op zero-tolerance regression gate, same op set. A different
    # gate setting here would make the two arms' numbers incomparable.
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
        all_ops=_all_perf_ops(assets, perf_ops),
    )

    llm: Optional[LLMClient] = None
    result = AgentResult()

    fns_ordered = sorted(
        [hf for hf in hot_fns
         if hf.file is not None and not hf.extern_wrapper],
        key=lambda h: h.self_pct, reverse=True,
    )
    only_fn = os.environ.get("AGENT_ONLY_FN", "").strip()
    if only_fn:
        fns_ordered = [hf for hf in fns_ordered if hf.name == only_fn]
        logger.info("[freeform] AGENT_ONLY_FN=%r → %d fn(s)",
                    only_fn, len(fns_ordered))

    logger.info(
        "[freeform] ABLATION ARM — no cards, no regions, no rule gating. "
        "%d hot fn(s), hottest=%s @ %.1f%%",
        len(fns_ordered),
        fns_ordered[0].name if fns_ordered else "-",
        fns_ordered[0].self_pct if fns_ordered else 0.0,
    )

    for hf in fns_ordered:
        if result.total_attempts >= cfg.max_candidates:
            result.early_stop_reason = (
                f"candidate budget exhausted ({cfg.max_candidates})")
            logger.info("[freeform] STOP: %s", result.early_stop_reason)
            break

        ep = _load_evidence(evidence_dir / f"{hf.name}.json")
        if ep is None:
            logger.warning("[freeform] no evidence for %s, skipping", hf.name)
            continue

        edit_target = resolve_edit_target(hf, ep, "C1", fn_index, crate)
        if edit_target is None:
            # Logged, not counted — the same treatment the full pipeline
            # gives an unresolvable target. A skip is not an attempt and
            # must not consume the candidate budget.
            logger.info("[freeform] %s: no editable target, skip", hf.name)
            state.log(AttemptRecord(
                fn_name=hf.name, rule_id="pre_abstain",
                round_no=1, attempt_no=1,
                status=RewriteAttempt.ABSTAINED,
                reason="no_editable_target",
                **reporting.with_attempt_trace({}, [
                    reporting.make_trace_entry(
                        1, "freeform", RewriteAttempt.ABSTAINED,
                        "pre_abstain", None)])))
            continue

        if llm is None:
            llm = _init_llm(crate)

        logger.info("[freeform] %s: self=%.2f%% → whole-function prompt",
                    hf.name, hf.self_pct)
        rec = _try_fn_freeform(
            hf, ep, edit_target,
            llm=llm, state=state, w2_session=w2_session, verifier=verifier,
            assets=assets, harness_dir=harness_dir, crate=crate, cfg=cfg)
        state.log(rec)
        result.add(rec)

        if rec.status == RewriteAttempt.APPLIED_COMMITTED:
            fn_index = build_fn_index(crate)

    _finalize_w2(result, w2_session, _all_perf_ops(assets, perf_ops))

    total_wall = (f"{result.total_wall_gain_pct:.2f}%"
                  if result.total_wall_gain_pct is not None else "unmeasured")
    logger.info(
        "[freeform] finished: attempts=%d, commit=%d, abstain=%d, "
        "build_rejected=%d, w1_rejected=%d, w2_no_gain=%d, w2_regress=%d, "
        "syntax_or_internal=%d; total wall=%s; tokens_est=%d",
        result.total_attempts, result.committed_attempts,
        result.abstained_count, result.rejected_build_count,
        result.w1_failed_count, result.w2_no_gain_count,
        result.w2_regress_count, result.syntax_failed_count,
        total_wall, result.llm_tokens_used_estimated,
    )
    return result
