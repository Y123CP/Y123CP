""                                                                       

          
                       
                                                    
                                                 

                                                   
                                             
   

from __future__ import annotations

import logging
import json
import re
from collections.abc import Mapping, Sequence
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Optional

from perf_opt.agent_perf_opt.changeset.handlers.replace_region import (
    resolve_region_ref,
)
from perf_opt.agent_perf_opt.changeset.resolver import ResolutionRejected
from perf_opt.agent_perf_opt.changeset.types import ChangeSetStatus, RegionRef
from perf_opt.agent_perf_opt.config import AgentConfig
from perf_opt.agent_perf_opt.regions.cst import same_node
from perf_opt.agent_perf_opt.regions.model import (
    rules_region_local_for,
    RuleCapability,
    rule_capability,
    stable_hit_id,
)
from perf_opt.agent_perf_opt.rewrite_applier import EditTarget
from perf_opt.agent_perf_opt.type_context import build_type_context
from perf_opt.hot_probe.types import EvidencePack, HotFunction

logger = logging.getLogger(__name__)

_CARD_DIR = Path(__file__).parent / "Optimization_Card"


# ─── SYSTEM prompt (impl_plan §5.1, fixed) ────────────────────────────
SYSTEM_PROMPT = """\
You are a Rust performance optimization agent. Your task this turn is to
apply exactly ONE optimization rule to the target function.

Contract:
  * INPUT: target fn source, ONE rule card, evidence bundle
  * OUTPUT: rewritten fn wrapped in a single ```rust fenced block; include
    ALL original attributes (#[inline], #[cold], #[no_mangle], ...) unless
    the rule explicitly changes them
  * Every `unsafe { ... }` must carry an inline `// SAFETY: ...` comment
    stating the proven precondition
  * A view whose length must be SCANNED to exist (`CStr::from_ptr`,
    strlen-style search) pays that scan on EVERY call — never build one
    in a hot function, however clean the resulting code looks.
  * NEVER add a runtime check the original code did not have. c2rust emits
    `*p.offset(i)`, which is unchecked; writing `slice[i]` in a hot loop pays
    a bounds check EVERY iteration — measured at 8.35% on one crate's hottest
    loop. Use `get_unchecked` with a SAFETY proof, or leave the raw pointer.
  * If you cannot prove the safety condition, output exactly:
      abstain: <one-line reason>
    and STOP
  * DO NOT apply any rule other than the one in `## Task rule`
  * Any deviation from the output format is treated as W1 failure
"""


                                                                       

_card_cache: dict[str, str] = {}

                                                          
                        
_RULE_ID_NORMALIZE: dict[str, str] = {
    "III①": "III1",
    "III②": "III2",
    "III③": "III3",
    "III④": "III4",
}


def _normalize_rule_id(rule_id: str) -> str:
    ""                                                        
    base = rule_id.split(".", 1)[0]
    return _RULE_ID_NORMALIZE.get(base, base)


def load_card(rule_id: str) -> str:
    ""                                                          

                                                                  
                                                         

                                                  
                               
       
    base = _normalize_rule_id(rule_id)
    if base not in _card_cache:
        matches = sorted(_CARD_DIR.glob(f"{base}_*.md"))
        # fallback: a card filed as `<base>.md` with no underscore suffix
        if not matches:
            single = _CARD_DIR / f"{base}.md"
            if single.is_file():
                matches = [single]
        if not matches:
            raise FileNotFoundError(
                f"no Optimization_Card matches base={base!r} in {_CARD_DIR}")
        _card_cache[base] = matches[0].read_text(encoding="utf-8")
    return _card_cache[base]


def _read_target_source(edit_target: EditTarget) -> str:
    ""                                                           
    try:
        return edit_target.file.read_bytes()[
            edit_target.span[0]:edit_target.span[1]
        ].decode("utf-8", errors="replace")
    except OSError:
        return ""


def _extract_current_signature(edit_target: EditTarget) -> str:
    ""                                             
    src = _read_target_source(edit_target)
    idx = src.find("{")
    return (src[:idx] if idx >= 0 else src).strip()


def _byte_span_to_lines(edit_target: EditTarget) -> tuple[int, int]:
    """(line_start, line_end) 1-indexed, computed from byte span. Used for
    the human-friendly ``location: L..L`` field in the Target block (§5.2)."""
    try:
        b = edit_target.file.read_bytes()
    except OSError:
        return (0, 0)
    start, end = edit_target.span
    return (b[:start].count(b"\n") + 1, b[:end].count(b"\n") + 1)


                                                                  
                                                          
                                             
_LONG_FN_CHAR_THRESHOLD = 40_000     # ~ 10K tokens (4 chars/token)
_LONG_FN_HEAD_LINES = 30
_LONG_FN_TAIL_LINES = 30


def _maybe_truncate_source(src: str) -> str:
    ""                                                      
    if len(src) <= _LONG_FN_CHAR_THRESHOLD:
        return src
    lines = src.splitlines()
    if len(lines) <= _LONG_FN_HEAD_LINES + _LONG_FN_TAIL_LINES:
        return src                    
    head = lines[:_LONG_FN_HEAD_LINES]
    tail = lines[-_LONG_FN_TAIL_LINES:]
    omitted = len(lines) - _LONG_FN_HEAD_LINES - _LONG_FN_TAIL_LINES
    marker = f"    // ... [{omitted} lines omitted for brevity — showing head + tail only] ..."
    return "\n".join(head + [marker] + tail)


def _format_target(edit_target: EditTarget, crate_dir: Optional[Path]) -> str:
    src = _read_target_source(edit_target)
    src_shown = _maybe_truncate_source(src)          # §8.6
    sig = _extract_current_signature(edit_target)
    ls, le = _byte_span_to_lines(edit_target)
    try:
        rel = edit_target.file.relative_to(crate_dir) if crate_dir else edit_target.file
    except ValueError:
        rel = edit_target.file
    return (
        f"## Target function\n"
        f"file: {rel}\n"
        f"symbol: {edit_target.fn_name}\n"
        f"signature: {sig}\n"
        f"location: {ls}..{le}  (含前置 attrs)\n\n"
        f"```rust\n{src_shown}\n```"
    )


# Passes that describe the build rather than report an optimization the
# compiler could not perform. `asm-printer` names basic blocks, `size-info`
# reports code size, `prologepilog` and `stack-frame-layout` describe the
# frame. None of it tells the model anything it can act on.
_REMARK_NOISE_PASSES = frozenset({
    "asm-printer", "size-info", "annotation-remarks",
    "prologepilog", "stack-frame-layout", "TTI",
})

# Passes whose findings map onto a rule the model is being offered:
# a loop that did not vectorize, an invariant that did not hoist, a load
# that was not eliminated, a callee that was not inlined.
_REMARK_ACTIONABLE_PASSES = frozenset({
    "loop-vectorize", "slp-vectorizer", "licm", "gvn",
    "loop-unroll", "loop-idiom", "loop-delete", "memcpyopt", "inline",
})


def _remark_tier(remark: dict) -> Optional[int]:
    """Lower is more useful; None means drop it entirely."""
    pass_name = remark.get("pass")
    if pass_name in _REMARK_NOISE_PASSES:
        return None
    missed = remark.get("status") == "missed"
    actionable = pass_name in _REMARK_ACTIONABLE_PASSES
    if missed and actionable:
        return 0          # a named opportunity the compiler declined
    if missed:
        return 1          # some other pass declined; still a miss
    if actionable:
        return 2          # it succeeded — useful as "already done, skip it"
    return 3


def _filter_top_remarks(remarks: list, top_n: int = 10) -> list:
    """The most useful N remarks for this fn, spread across passes.

    This was `remarks[:top_n]` — the compiler's own emission order, which
    correlates with nothing. Measured across the corpus: of 34,760 remarks
    collected, 11.3% fit in the window, and a fifth of what did fit was
    `asm-printer` naming basic blocks, while 8,568 non-noise `missed`
    remarks were cut. The window was effectively a random sample.

    Two things decide the order, and the second matters more than it looks:

      1. Tier — a miss the model can act on outranks a miss it cannot,
         which outranks a success, which outranks everything else.
      2. Round-robin **within** a tier, one remark per pass per pass.
         Ranking by tier alone lets a single chatty pass fill the whole
         window: sorting by tier only, 139 functions saw exactly one kind
         of actionable signal and 51 saw three. Rotating raises three-kind
         coverage to 119 functions and drops single-kind to 85, with the
         same budget. The model is choosing between rules — one signal
         repeated ten times cannot inform that choice; three different
         signals can.

    Stable: ties keep the compiler's original relative order, so the same
    build produces the same window every run.
    """
    if not remarks:
        return []
    tiers: dict[int, dict[str, list]] = {}
    for remark in remarks:
        if not isinstance(remark, dict):
            continue
        tier = _remark_tier(remark)
        if tier is None:
            continue
        tiers.setdefault(tier, {}).setdefault(
            remark.get("pass") or "", []).append(remark)

    out: list = []
    for tier in sorted(tiers):
        buckets = tiers[tier]
        while len(out) < top_n and any(buckets.values()):
            for queue in buckets.values():
                if queue and len(out) < top_n:
                    out.append(queue.pop(0))
        if len(out) >= top_n:
            break
    return out


# ═════════════════════════════════════════════════════════════════════════
                                                              
                
                                                         
                                                           
                                                              
                                 
# ═════════════════════════════════════════════════════════════════════════

                                                                     

SYSTEM_MULTI_CARD = """\
You are a Rust performance optimization agent. This turn you receive a
target function plus one or more optimization cards. Each card describes
a distinct symptom that may share a root cause with others.

**BEFORE rewriting**:
  1. Read each card's "When to abstain" section FIRST.
  2. Check the target fn against each abstain criterion:
     - No-leverage hoist (target read only 1×; short fn no loop)?
     - C1 site outside hot loop + fn < 30 lines?
     - Inlining large callee (> 40 lines)?
  3. If ALL fired rules hit an abstain criterion for this fn, output
     exactly:  `abstain: <one-line reason>`  and STOP.
     Do NOT produce a "cosmetic" rewrite that just moves code around
     without gain — the W2 gate rejects rewrites with no measurable
     speedup (require_perf_gain=True).

Contract:
  * Read ALL cards; identify if any single rewrite (typically the most
    structural one, e.g. slice/iter lifting) subsumes multiple symptoms.
  * Prefer minimal rewrites that address maximum symptoms.
  * OUTPUT: rewritten fn wrapped in a single ```rust fenced block; keep
    ALL original attributes (#[inline], #[cold], #[no_mangle], ...) unless
    a rule explicitly changes them.
  * **EVERY fired rule MUST be accounted for.** Prepend TWO lines before
    the fn signature (both required, order matters):
        // Applied rules: [<comma-list you applied>]
        // Skipped rules: [<rule>: <one-line reason>; <rule>: <reason>]
    Skipped rules use `[]` if none. Reasons must be specific (e.g.
    "subsumed by III④", "callee too large > 40 lines", "no hot loop, fn
    < 30 lines"). Silently dropping a fired rule is a contract violation.
  * Every `unsafe { ... }` must carry an inline `// SAFETY: ...` comment
    stating the proven precondition.
  * A view whose length must be SCANNED to exist (`CStr::from_ptr`,
    strlen-style search) pays that scan on EVERY call — never build one
    in a hot function, however clean the resulting code looks.
  * NEVER add a runtime check the original code did not have. c2rust emits
    `*p.offset(i)`, which is unchecked; writing `slice[i]` in a hot loop pays
    a bounds check EVERY iteration — measured at 8.35% on one crate's hottest
    loop. Use `get_unchecked` with a SAFETY proof, or leave the raw pointer.
  * Per-site abstain is fine — leave that site unchanged.
  * If NO rewrite is safe or all rules are no-leverage on this fn,
    output exactly:  `abstain: <one-line reason>`  and STOP.
"""

SYSTEM_PLAN = """\
You are a Rust performance optimization agent. This turn you write a
PLAN (NOT code). The plan declares which rules you will apply, in what
order, how they interact, and whether any needs cross-function rewrite.

Contract:
  * Output ONLY a JSON object in a single ```json fenced block. No prose.
  * Schema:
      {
        "applied_rules": [
          {
            "rule": "III④",
            "strategy": "one-line description",
            "target_hits": [<REQUIRED non-empty list of hit indices you will address>],
            "needs_cross_fn": true|false,
            "rationale": "why this order / interaction with other rules"
          },
          ...
        ],
        "abstained_rules": [
          {"rule": "II_inl", "reason": "subsumed by III①"}
        ],
        "rewrite_order": ["III④", "C1", ...],
        "interactions_noted": ["applying III④ subsumes 27 C1 sites"],
        "risk_flags": []
      }
  * needs_cross_fn=true means the rewrite would change fn signature or
    otherwise affect callers (III①.a / C3.S1 / II_inl.b). Agent will
    decide V1 abstain vs proceed based on caller analysis.
  * Be thorough about interactions — one structural rewrite often makes
    other rules moot (III④ slice-ification subsumes C1 bounds checks).
  * **F6 rule:** `target_hits` MUST be a non-empty list of integer indices
    from the Hits table shown above (the `[i]` column). If a rule cannot
    address any concrete hit — put it in `abstained_rules`, not
    `applied_rules`. Vague "hits" or "all" strings are rejected. An empty
    list or absent field auto-moves the rule to `abstained_rules`.
"""

SYSTEM_EXECUTE = """\
You are a Rust performance optimization agent. You wrote a PLAN in the
previous turn; now execute it and output the rewritten function.

Contract:
  * Follow the plan exactly — apply only the rules listed in
    `applied_rules` (with their assigned strategies).
  * OUTPUT: rewritten fn wrapped in a single ```rust fenced block; keep
    ALL original attributes unless a rule changes them.
  * PREPEND TWO lines before the fn signature (both required):
      // Applied rules: [<comma-list>]           # matches plan.applied_rules
      // Skipped rules: [<rule>: <reason>; ...]  # matches plan.abstained_rules
    Every rule the CALLER fired must appear in one list or the other.
  * Every `unsafe { ... }` must carry an inline `// SAFETY: ...` comment.
  * A view whose length must be SCANNED to exist (`CStr::from_ptr`,
    strlen-style search) pays that scan on EVERY call — never build one
    in a hot function, however clean the resulting code looks.
  * NEVER add a runtime check the original code did not have. c2rust emits
    `*p.offset(i)`, which is unchecked; writing `slice[i]` in a hot loop pays
    a bounds check EVERY iteration — measured at 8.35% on one crate's hottest
    loop. Use `get_unchecked` with a SAFETY proof, or leave the raw pointer.
  * Per-site abstain is fine within a rule.
  * If circumstances make the plan unsafe to execute, output:
      abstain: <one-line reason>
    and STOP.
"""


# ─── helpers ─────────────────────────────────────────────────────────

def load_cards(rule_ids: list[str]) -> dict[str, str]:
    """Load a set of rule cards → {rule_id: card_text}."""
    return {rid: load_card(rid) for rid in rule_ids}


def _format_hits_summary(hits: list[dict]) -> str:
    ""                                                
    if not hits:
        return "  (none)"
    lines = []
    for i, h in enumerate(hits):
        rule = h["rule"]
        pattern = h["pattern"]
        line = h.get("line", 0)
        col = h.get("col", 0)
        snippet = (h.get("snippet") or "").replace("\n", " ")[:100]
        extra = h.get("extra", {})
        extra_bits = []
        if "hit_count" in extra and extra["hit_count"] > 1:
            extra_bits.append(f"×{extra['hit_count']}")
        if "callee_name" in extra:
            extra_bits.append(f"callee={extra['callee_name']}")
        if "form" in extra:
            extra_bits.append(f"form={extra['form']}")
        if "remark_message" in extra:
            extra_bits.append(f"remark={extra['remark_message'][:60]!r}")
        extras_str = " ".join(extra_bits)
        lines.append(
            f"  [{i:3d}] {rule:6s} {pattern:30s} line {line}:{col}"
            + (f"  {extras_str}" if extras_str else "")
            + (f"\n         snippet: {snippet}" if snippet else "")
        )
    return "\n".join(lines)


def _format_cards_block(cards: dict[str, str]) -> str:
    ""                         
    blocks = []
    for rid, text in cards.items():
        blocks.append(f"### Card: {rid}\n\n{text.strip()}")
    return "\n\n---\n\n".join(blocks)


def _format_evidence_multi(hf: HotFunction,
                             fired_rules: list[str],
                             hits: list[dict]) -> str:
    """What fired, and where. NOT how fast it currently is.

    Two blocks were removed from this bundle, and what they were matters:

      * the PROFILE numbers — self_time_ratio, retired_instructions, cpi,
        branch_miss_rate, tma, tma_bottleneck
      * the top-10 COMPILER REMARKS

    Neither told the model anything it could act on. A remark reading
    "loop not vectorized: value that could not be identified as reduction
    is used outside the loop" names an obstacle in the compiler's own
    vocabulary, next to a card that already names the pattern and the
    transform — two voices pointing at the same code, and the model has to
    guess which one it is being graded on. `cpi` and `tma_bottleneck` are
    worse: they are a verdict about the whole function, offered while the
    question on the table is whether ONE specific rewrite applies.

    The measured hint came from the ablation arm, where the same two blocks
    were the only guidance present: removing them moved that arm from
    -3.826% to -4.107% aggregate and 1 commit to 3. One project, one run,
    overlapping CIs — not proof on its own, which is why the full arm gets
    its own paired run rather than inheriting the conclusion.

    What stays is the rule layer: which rules fired and where they hit.
    That is the method, not a hint about it.
    """
    return "\n".join([
        "## Evidence bundle",
        f"workload: {hf.hottest_op}",
        f"fired_rules: {fired_rules}",
        "",
        "## Rule hits (from fn_hits.json)",
        _format_hits_summary(hits),
    ])


# ─── D16: build_multi_card_prompt(simple fn direct)──────────────────

REGION_SYSTEM_PROMPT = """\
You are a Rust performance optimization agent editing one proven source region.

You are shown the WHOLE enclosing function (signature + full body) as
read-only context, with the editable region marked. Use the full function
to establish facts you cannot see inside the region alone — a slice-length
source (an `len: usize` parameter, a `const`, a struct length field, or a
sentinel loop bound), the enclosing loop's iteration bounds, the cursor's
full advance idiom, and whether any other reference aliases the same memory.
An abstain is only justified after checking the WHOLE function, not just the
region — do not claim "no length source is locally provable" when the length
source is visible in the signature or an outer loop.

Contract:
  * Return only the replacement region, never a full function, function
    signature, module, impl, trait, extern block, or any other item.
  * All source outside the marked editable region is read-only. Do not
    reproduce or modify it — only the marked region may change.
  * Use only the candidate region-local optimization cards in this prompt.
    Typed, cross-function, and unsupported rules must never enter this prompt.
  * Return either `abstain: <one-line reason>` or one ```rust fence.
  * The first two lines inside the fence must be exactly:
      // Applied rules: [<comma-list>]
      // Skipped rules: [<rule>: <non-empty reason>; ...]
    Every candidate rule must occur exactly once across those two lines.
    An empty list is written `[]` — literally nothing between the brackets.
    Do NOT write `none`, `-`, or `N/A`: those parse as a rule id nobody
    offered and the whole reply is discarded unevaluated.
  * The remaining fenced source is the replacement region. It must have the
    same syntactic region shape described by `region_kind`.
"""


def _fn_source_by_text(
    crate_root: Path, name: str, *, max_lines: int = 140
) -> Optional[str]:
    """Locate a function definition by TEXT search and return its source.

    Deliberately does NOT use the tree-sitter fn index: c2rust idioms such as
    ``x as size_t <= y`` make tree-sitter mark the file ``has_error`` and drop
    whole spans of ``function_item`` nodes, so index lookups miss exactly the
    color/filter callees a delegated C6 loop delegates to. A brace-matched
    text scan is immune to that. Returns None if not found.
    """
    pat = re.compile(r"\bfn\s+" + re.escape(name) + r"\s*[(<]")
    files: list[Path] = []
    if (crate_root / "src").is_dir():
        files += sorted((crate_root / "src").rglob("*.rs"))
    if (crate_root / "lib.rs").is_file():
        files.append(crate_root / "lib.rs")
    for f in files:
        try:
            text = f.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        m = pat.search(text)
        if m is None:
            continue
        brace = text.find("{", m.end())
        if brace < 0:
            continue
        depth, j = 0, brace
        while j < len(text):
            ch = text[j]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        body_lines = text[m.start():j + 1].splitlines()
        if len(body_lines) > max_lines:
            body_lines = body_lines[:max_lines] + [
                f"// ... ({len(body_lines) - max_lines} more lines truncated)"
            ]
        return "\n".join(body_lines)
    return None


def _delegated_callee_context(
    crate_root: Path,
    candidates: Sequence[str],
    anchor_hits: Mapping[str, Mapping[str, Any]],
    anchor_ids: Sequence[str],
    *,
    max_callees: int = 2,
    max_lines: int = 140,
) -> str:
    """Read-only source of the function(s) a C6 *delegated* dispatch loop calls.

    A delegated-dispatch hit stores the callee name in ``extra['dispatch_on']``.
    To apply C6 (§2.2) the LLM must inline the callee's common-configuration
    branch into a specialized copy of the loop — which it cannot do without
    seeing the callee's body. Region editing stays in the CALLER; only the
    prompt context is widened with the callee source. Returns "" when the
    region has no delegated C6 hit (so the fn_index is only built when needed).
    """
    if "C6" not in set(candidates):
        return ""
    callees: list[str] = []
    seen: set[str] = set()
    for hit_id in anchor_ids:
        hit = anchor_hits.get(hit_id)
        if not isinstance(hit, Mapping) or hit.get("rule") != "C6":
            continue
        extra = hit.get("extra")
        if not isinstance(extra, Mapping):
            continue
        callee = extra.get("dispatch_on")
        if isinstance(callee, str) and callee and callee not in seen:
            seen.add(callee)
            callees.append(callee)
    if not callees:
        return ""
    blocks: list[str] = []
    for callee in callees[:max_callees]:
        body = _fn_source_by_text(crate_root, callee, max_lines=max_lines)
        if body is None:
            continue
        blocks.append(
            f"### callee `{callee}` (read-only)\n```rust\n" + body + "\n```"
        )
    if not blocks:
        return ""
    return (
        "## Delegated-dispatch callee source (read-only context for C6)\n"
        "The editable region's hot loop delegates per-iteration work to the "
        "function(s) below. To apply C6 (Delegated dispatch, card §2.2): pick the "
        "common runtime configuration, inline that callee branch into a "
        "specialized copy of the loop, and keep the original call as the fallback "
        "for other configurations. Do NOT edit the callee — only the editable "
        "region above.\n\n" + "\n\n".join(blocks)
    )


_C12_CALL_RE = re.compile(r"\b([A-Za-z_]\w*)\s*\(")
# Passing a local buffer out: c2rust spells it `&raw mut x`, safe Rust `&mut x`,
# and a slice hands over `.as_mut_ptr()`.
_C12_HANDOVER_RE = re.compile(r"&\s*raw\s+mut\b|&\s*mut\b|\.as_mut_ptr\(\)")
_C12_NOT_CALLS = frozenset({
    "if", "while", "for", "match", "return", "let", "fn", "unsafe", "loop",
    "as", "in", "else", "impl", "mut", "move", "ref", "size_of", "align_of",
})


def _call_arg_text(source: str, open_paren: int) -> str:
    """Text between `(` at `open_paren` and its matching `)` (spans newlines)."""
    depth, j = 0, open_paren
    while j < len(source):
        ch = source[j]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return source[open_paren + 1:j]
        j += 1
    return ""


def _zero_init_consumer_context(
    crate_root: Optional[Path],
    fired_rules: Sequence[str],
    target_source: str,
    *,
    max_callees: int = 3,
    max_lines: int = 120,
) -> str:
    """Source of the functions a C12 candidate hands its zeroed buffers to.

    C12 asks whether only a prefix of an over-zeroed buffer is ever read. When
    the buffer is passed to another function, that question cannot be answered
    from the edited function alone — and the card's own abstain list says to
    give up when "the buffer escapes ... to anything opaque". With only the
    caller in the prompt, EVERY callee is opaque, so the rule abstains on
    exactly the shape it was written for.

    Measured on the first crate where C12 fired: the model skipped it with
    "buffers escape to opaque callees (`setup_match_struct` and `match_row`)",
    while both are ordinary same-crate functions — one writes `[0..n)`, the
    other reads `[0..m)`, which together prove the prefix bound it needed.

    Only callees that actually RECEIVE a buffer are included (`&raw mut` /
    `&mut` / `.as_mut_ptr()` in the call's arguments), so the widening is
    bounded by what the question requires rather than by the call count.
    """
    if crate_root is None or "C12" not in set(fired_rules) or not target_source:
        return ""
    names: list[str] = []
    seen: set[str] = set()
    for m in _C12_CALL_RE.finditer(target_source):
        name = m.group(1)
        if name in _C12_NOT_CALLS or name in seen:
            continue
        args = _call_arg_text(target_source, m.end() - 1)
        if not args or not _C12_HANDOVER_RE.search(args):
            continue
        seen.add(name)
        names.append(name)
    blocks: list[str] = []
    for name in names[:max_callees]:
        body = _fn_source_by_text(crate_root, name, max_lines=max_lines)
        if body is None:
            continue
        blocks.append(
            f"### callee `{name}` (read-only)\n```rust\n" + body + "\n```")
    if not blocks:
        return ""
    return (
        "## Buffer-consumer source (read-only context for C12)\n"
        "The function you are editing hands one or more of its zero-initialized "
        "buffers to the function(s) below. Read them to settle C12's central "
        "question: **which prefix of each buffer is actually written, and which "
        "is actually read.** Look for the length each callee bounds its loops "
        "by — that expression is the prefix bound the rewrite needs. If a "
        "callee reads beyond what any callee writes, the zeroing is "
        "load-bearing and you must abstain. Do NOT edit these callees; they "
        "are context only.\n\n" + "\n\n".join(blocks)
    )


def _handed_over_callback_context(
    crate_root: Path,
    candidates: Sequence[str],
    anchor_hits: Mapping[str, Mapping[str, Any]],
    anchor_ids: Sequence[str],
    *,
    max_callbacks: int = 2,
    max_lines: int = 140,
) -> str:
    """Read-only source of the function handed to a higher-order libc call.

    Same mechanism as the C6 widening above, for the same reason. A Form E
    III① hit is `qsort(base, n, size, Some(Cmp as ...))`: the site to edit is
    the call, but the rewrite is only *sound* if the comparator's ordering key
    and direction survive, and card §2.E makes reading it step 1. That body is
    a separate item, so a region prompt — which shows this function and
    nothing else — cannot contain it.

    The model behaves correctly without it, which is what makes the gap
    expensive rather than merely wrong: it abstains, citing "comparator body
    is not visible here, so the ordering cannot be proven equivalent", and the
    site is scored as "rule did not apply" rather than "prompt was missing its
    premise". Observed on a compression crate whose `qsort` call carried a
    hand-measured -30.72% on one operation.

    Returns "" when no Form E anchor names a callback (`callback_fn` empty —
    see `CallSite.callback_fn`), so the file scan only runs when it can pay.
    """
    if "III①" not in set(candidates):
        return ""
    names: list[str] = []
    seen: set[str] = set()
    for hit_id in anchor_ids:
        hit = anchor_hits.get(hit_id)
        if not isinstance(hit, Mapping) or hit.get("rule") != "III①":
            continue
        extra = hit.get("extra")
        if not isinstance(extra, Mapping) or extra.get("form") != "E":
            continue
        name = extra.get("callback_fn")
        if isinstance(name, str) and name and name not in seen:
            seen.add(name)
            names.append(name)
    if not names:
        return ""
    blocks: list[str] = []
    for name in names[:max_callbacks]:
        body = _fn_source_by_text(crate_root, name, max_lines=max_lines)
        if body is None:
            continue
        blocks.append(
            f"### callback `{name}` (read-only)\n```rust\n" + body + "\n```"
        )
    if not blocks:
        return ""
    return (
        "## Handed-over callback source (read-only context for III① form E)\n"
        "The editable region hands the function pointer(s) below to a "
        "higher-order libc routine, which calls back through the C ABI once "
        "per element. Card §2.E requires reading this body to establish the "
        "ordering key and direction before rewriting — it is reproduced here "
        "for exactly that purpose, so \"the comparator is not visible\" is not "
        "a reason to abstain. Do NOT edit the callback — only the editable "
        "region above; leaving the now-unused callback definition in place is "
        "correct.\n\n" + "\n\n".join(blocks)
    )


def _type_context_for(
    crate_dir: "Optional[Path]", *sources: str, max_defs: int = 10
) -> str:
    """Definitions of the composite types the shown code reaches.

    A c2rust struct definition lives nowhere near the function that walks it,
    and a function's own text names only the outermost one — so the element
    type of a field it indexes two levels down appears nowhere the model can
    see. Asked to slice such a walk it invents a name that reads right and
    does not exist, and the compile error that follows says the name is
    unresolved without saying which name is not. Measured: four consecutive
    runs of one crate, three LLM turns and three builds each.

    Empty for a function that touches no composite type, which is most of
    the arithmetic kernels — the block only appears where it is needed.
    """
    if crate_dir is None:
        return ""
    try:
        return build_type_context(
            Path(crate_dir), "\n".join(s for s in sources if s),
            max_defs=max_defs)
    except OSError:
        return ""


def build_region_prompt(
    *,
    crate: Path,
    edit_target: EditTarget,
    region: RegionRef,
    rule_ids: Sequence[str],
    hits: Sequence[Mapping[str, Any]],
    context_lines: Optional[int] = None,
) -> tuple[str, str]:
    """Build a prompt for exactly one already-extracted region.

    ``context_lines`` controls how much of the enclosing function body is
    shown as read-only context around the editable region:
      * ``None`` (default) — the WHOLE function body (every line before and
        after the region, still scoped to this function; never other items).
        This lets the LLM see the signature-level length source, outer loop
        bounds and full cursor idiom that a bounded window hides — the root
        cause of the "no length source locally provable" false abstains.
      * ``int >= 0`` — legacy bounded window: the last / first N body lines.
    """

    if context_lines is not None and (
        not isinstance(context_lines, int)
        or isinstance(context_lines, bool)
        or context_lines < 0
    ):
        raise ValueError("context_lines must be None or a non-negative integer")
    if not isinstance(region, RegionRef):
        raise TypeError("region must be a RegionRef")
    candidates = tuple(rule_ids)
    if (
        not candidates
        or any(not isinstance(rule_id, str) or not rule_id for rule_id in candidates)
        or len(set(candidates)) != len(candidates)
    ):
        raise ValueError("rule_ids must be unique non-empty strings")
    non_local = rules_region_local_for(candidates, hits)
    if non_local:
        raise RuntimeError(
            f"only region-local rules may enter a region prompt: {non_local}"
        )

    crate_root = Path(crate).resolve()
    resolved = resolve_region_ref(crate_root, region)
    target_path = Path(edit_target.file)
    if not target_path.is_absolute():
        target_path = crate_root / target_path
    target_path = target_path.resolve()
    if not (
        target_path == resolved.path
        and edit_target.fn_name == resolved.target_name
        and edit_target.span == resolved.declaration_span
    ):
        raise ResolutionRejected(
            ChangeSetStatus.REJECTED_STALE,
            "EditTarget does not match the proven region declaration",
        )
    relative_path = region.relative_path
    data = resolved.source
    function = resolved.function
    body = resolved.body
    attributes = []
    if function.parent is not None:
        siblings = function.parent.named_children
        function_index = next(
            (
                index
                for index, sibling in enumerate(siblings)
                if same_node(sibling, function)
            ),
            None,
        )
        if function_index is not None:
            index = function_index - 1
            while index >= 0 and siblings[index].type in {
                "attribute_item",
                "line_comment",
                "block_comment",
            }:
                attachment = siblings[index]
                if attachment.start_byte < resolved.declaration_span[0]:
                    break
                if attachment.type == "attribute_item":
                    attributes.append(
                        data[attachment.start_byte : attachment.end_byte].decode(
                            "utf-8"
                        )
                    )
                index -= 1
            attributes.reverse()
    signature = data[function.start_byte : body.start_byte].decode("utf-8").rstrip()
    header = "\n".join([*attributes, signature])
    original_region = data[region.start_byte : region.end_byte].decode("utf-8")

    before_body = data[body.start_byte + 1 : region.start_byte].decode("utf-8")
    after_body = data[region.end_byte : body.end_byte - 1].decode("utf-8")
    if context_lines is None:
        # Whole-function context: every body line before/after the region.
        before = before_body.splitlines()
        after = after_body.splitlines()
    else:
        before = before_body.splitlines()[-context_lines:] if context_lines else []
        after = after_body.splitlines()[:context_lines] if context_lines else []

    anchor_hits: dict[str, Mapping[str, Any]] = {}
    anchor_set = set(region.anchor_hit_ids)
    for hit in hits:
        if not isinstance(hit, Mapping):
            raise TypeError("hits must contain mappings")
        raw_function = hit.get("function")
        if raw_function is not None and raw_function != resolved.target_name:
            continue
        identity = dict(hit)
        raw_file = identity.get("file")
        try:
            raw_path = (
                Path(raw_file) if isinstance(raw_file, (str, Path)) else None
            )
            matches_target = raw_path is not None and (
                raw_path.resolve()
                if raw_path.is_absolute()
                else (crate_root / raw_path).resolve()
            ) == target_path
            if matches_target:
                identity["file"] = relative_path
            hit_id = stable_hit_id(identity, edit_target.fn_name)
        except (KeyError, TypeError, ValueError, OSError):
            continue
        if hit_id in anchor_set:
            anchor_hits.setdefault(hit_id, hit)
    missing = [hit_id for hit_id in region.anchor_hit_ids if hit_id not in anchor_hits]
    if missing:
        raise ValueError(f"missing anchor hit details: {missing}")
    anchor_rule_values = [
        anchor_hits[hit_id].get("rule") for hit_id in region.anchor_hit_ids
    ]
    if (
        any(
            not isinstance(rule_id, str) or not rule_id
            for rule_id in anchor_rule_values
        )
        or set(anchor_rule_values) != set(candidates)
    ):
        raise ValueError(
            "candidate rules must exactly match the region anchor hit rules"
        )

    hit_blocks = []
    for hit_id in region.anchor_hit_ids:
        hit = anchor_hits[hit_id]
        detail_data = {
            key: hit[key]
            for key in ("rule", "pattern", "line", "col", "snippet")
            if key in hit
        }
        extra = hit.get("extra")
        if isinstance(extra, Mapping):
            safe_extra = {
                key: extra[key]
                for key in (
                    "hit_count",
                    "callee_name",
                    "callback_fn",
                    "form",
                    "remark_message",
                    "note",
                )
                if key in extra
            }
            if safe_extra:
                detail_data["extra"] = safe_extra
        detail = json.dumps(
            detail_data,
            ensure_ascii=False,
            sort_keys=True,
            default=str,
        )
        hit_blocks.append(f"- anchor_hit_id: {hit_id}\n  detail: {detail}")
    cards = load_cards(list(candidates))
    callee_ctx = "\n\n".join(
        block for block in (
            _delegated_callee_context(
                crate_root, candidates, anchor_hits, region.anchor_hit_ids
            ),
            _handed_over_callback_context(
                crate_root, candidates, anchor_hits, region.anchor_hit_ids
            ),
        ) if block
    )
    before_text = "\n".join(before)
    after_text = "\n".join(after)
    context_note = (
        " (whole function body)" if context_lines is None else ""
    )
    sections = [
            "## Target function attributes and signature\n"
            f"```rust\n{header}\n```",
            "## Region identity\n"
            f"file: {relative_path}\n"
            f"function: {edit_target.fn_name}\n"
            f"region_kind: {region.region_kind.value}\n"
            f"parent_kind: {region.parent_kind}\n"
            f"candidate_rule_ids: {list(candidates)!r}",
            f"## Read-only function body BEFORE the editable region{context_note}\n"
            "The signature above plus this block, the editable region, and the "
            "AFTER block together form the complete function. Read it to find the "
            "slice-length source, loop bounds and aliasing — but do not modify it.\n"
            f"```rust\n{before_text}\n```",
            "## EDITABLE region (replace this complete source; the only part you may change)\n"
            f"```rust\n{original_region}\n```",
            f"## Read-only function body AFTER the editable region{context_note}\n"
            f"```rust\n{after_text}\n```",
            "## Anchor hit details (only hits anchored to this region)\n"
            + "\n".join(hit_blocks),
            "## Candidate optimization cards\n\n" + _format_cards_block(cards),
            "## Instruction\nReturn only the replacement region under the exact "
            "Applied rules / Skipped rules fence contract, or abstain.",
    ]
    if callee_ctx:
        sections.insert(-1, callee_ctx)   # after the cards, before the instruction
    # The region window is a slice of one function, so its reachable types are
    # the same ones the whole function reaches — feed it all four blocks, not
    # just the editable part, or a struct named only in the read-only tail
    # goes missing exactly when the rewrite has to name it.
    type_ctx = _type_context_for(
        crate_root, header, before_text, original_region, after_text)
    if type_ctx:
        sections.insert(-1, type_ctx)
    user = "\n\n".join(sections)
    return REGION_SYSTEM_PROMPT, user


def build_multi_card_prompt(
    hf: HotFunction, ep: EvidencePack, edit_target: EditTarget,
    fired_rules: list[str], hits: list[dict], cfg: AgentConfig,
    *, crate_dir: Optional[Path] = None,
) -> tuple[str, str]:
    ""                                             

         
                                              
                                                 
       
    cards = load_cards(fired_rules)
    user = "\n\n".join([blk for blk in [
        _format_target(edit_target, crate_dir),
        _type_context_for(crate_dir, _read_target_source(edit_target)),
        _zero_init_consumer_context(
            crate_dir, fired_rules, _read_target_source(edit_target)),
        "## Matched cards (one or more)\n\n" + _format_cards_block(cards),
        _format_evidence_multi(hf, fired_rules, hits),
        "## Instruction\n"
        "**FIRST: check each card's `When to abstain` section against this\n"
        "fn.** If the fn is short + no hot loop, or if the hoist/inline\n"
        "target has no leverage (only used once, callee too large, ...),\n"
        "output `abstain: <reason>` and STOP — a cosmetic rewrite with no\n"
        "gain will fail the W2 gate anyway.\n\n"
        "Otherwise, apply the listed rules. Prefer one structural rewrite\n"
        "that subsumes multiple symptoms over stacking narrow rewrites.\n"
        "Per-site abstain is fine.",
    ] if blk])
    return SYSTEM_MULTI_CARD, user


# ─── D17: build_plan_prompt(complex fn PLAN)──────────────────────────

def build_plan_prompt(
    hf: HotFunction, ep: EvidencePack, edit_target: EditTarget,
    fired_rules: list[str], hits: list[dict], cfg: AgentConfig,
    *, crate_dir: Optional[Path] = None,
) -> tuple[str, str]:
    ""                                          
    cards = load_cards(fired_rules)
    user = "\n\n".join([blk for blk in [
        _format_target(edit_target, crate_dir),
        # PLAN decides which rules are worth attempting, so the evidence C12
        # needs has to be here too — otherwise the rule is dropped at planning
        # time for want of the very context that answers its question.
        _zero_init_consumer_context(
            crate_dir, fired_rules, _read_target_source(edit_target)),
        "## Matched cards (one or more)\n\n" + _format_cards_block(cards),
        _format_evidence_multi(hf, fired_rules, hits),
        "## Instruction\n"
        "Write a PLAN as JSON per SYSTEM contract. Do NOT modify code.\n"
        "For each rule you'll apply, declare `needs_cross_fn` — set true\n"
        "iff the rewrite requires changing fn signature or otherwise\n"
        "affects callers. Agent will decide V1 abstain vs proceed based\n"
        "on caller analysis.",
    ] if blk])
    return SYSTEM_PLAN, user


# ─── D17: build_execute_prompt(complex fn EXECUTE)─────────────────────

def _validate_execute_plan_consistency(
    applied_rule_ids: list[str],
    effective_plan: dict,
) -> None:
    """Fail closed when EXECUTE cards and the authorized PLAN diverge."""
    if not isinstance(applied_rule_ids, list) or not all(
        isinstance(rule_id, str) and rule_id
        for rule_id in applied_rule_ids
    ):
        raise ValueError(
            "invalid effective PLAN contract: applied_rule_ids must be a "
            "list of non-empty strings"
        )
    if not isinstance(effective_plan, dict):
        raise ValueError("invalid effective PLAN: expected an object")
    plan_entries = effective_plan.get("applied_rules")
    if not isinstance(plan_entries, list):
        raise ValueError(
            "invalid effective PLAN: applied_rules must be a list"
        )

    plan_rule_ids: list[str] = []
    for index, item in enumerate(plan_entries):
        if not isinstance(item, dict):
            raise ValueError(
                "invalid effective PLAN: applied_rules"
                f"[{index}] must be an object"
            )
        rule_id = item.get("rule")
        if not isinstance(rule_id, str) or not rule_id:
            raise ValueError(
                "invalid effective PLAN: applied_rules"
                f"[{index}].rule must be a non-empty string"
            )
        plan_rule_ids.append(rule_id)

    if plan_rule_ids != applied_rule_ids:
        raise ValueError(
            "EXECUTE rule cards do not match effective PLAN applied_rules: "
            f"cards={applied_rule_ids!r}, plan={plan_rule_ids!r}"
        )


def build_execute_prompt(
    hf: HotFunction, ep: EvidencePack, edit_target: EditTarget,
    applied_rules: list[str], plan_json: dict,
    hits: list[dict], cfg: AgentConfig,
    *, crate_dir: Optional[Path] = None,
) -> tuple[str, str]:
    ""                                    

         
                                                           
                                           
                                                        
       
    import json as _json
    _validate_execute_plan_consistency(applied_rules, plan_json)
    cards = load_cards(applied_rules)
    plan_str = _json.dumps(plan_json, indent=2, ensure_ascii=False)
    user = "\n\n".join([blk for blk in [
        _format_target(edit_target, crate_dir),
        _type_context_for(crate_dir, _read_target_source(edit_target)),
        _zero_init_consumer_context(
            crate_dir, applied_rules, _read_target_source(edit_target)),
        "## Effective plan authorized by the dispatcher\n"
        "```json\n" + plan_str + "\n```",
        "## Cards for rules you will apply\n\n" + _format_cards_block(cards),
        _format_evidence_multi(hf, applied_rules, hits),
        "## Instruction\n"
        "Execute the plan. Output the rewritten fn per SYSTEM contract.\n"
        "The `// Applied rules: [...]` header must match applied_rules\n"
        "in the plan (rules removed by agent V1 cross-fn dispatch are\n"
        "already excluded — do NOT try to apply them).",
    ] if blk])
    return SYSTEM_EXECUTE, user


                                                              

def is_complex_fn(fired_rules: list[str], hits: list[dict],
                  cfg: AgentConfig) -> bool:
    ""                                                     
    return (len(set(fired_rules)) >= cfg.complex_fn_rules_threshold
            or len(hits) >= cfg.complex_fn_hits_threshold)
