"""Deterministic planners for the two inlining-attribute rules.

II_inl — a hot function's callee that LLVM refused to inline ("too costly").
  The card's whole rewrite is one attribute on the CALLEE, and every LLM path
  locks its reply to the hot function itself, so the rule had never landed a
  single commit in any project. Here it is emitted directly: `#[inline]` when
  the remark's cost is within LLVM's inline-hint threshold (that is all the
  hint raises the call site to), `#[inline(always)]` otherwise.

II_iso — a hot loop kernel that was fully inlined into a much larger function.
  Inlined there, its loop is scheduled and register-allocated together with the
  whole container; `#[inline(never)]` gives it a function of its own. Whether
  that pays is not predictable from the source — isolation can also cost the
  constant propagation inlining provided — so the detector only nominates and
  W2 decides.

Both emit one `SetFunctionInlineAttr` per changeset, so a callee that does not
pay is rejected on its own and never drags down one that does.
"""

from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass
from typing import Any, Iterable, Optional

from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ProposedChangeSet,
    SetFunctionInlineAttr,
    TriggerContext,
)

# LLVM's default `-inlinehint-threshold`: what `#[inline]` raises a call
# site's budget to. A callee costed above it will not inline on a hint.
INLINE_HINT_THRESHOLD = 325

_CALLEE_RE = re.compile(r"'([^']+)' not inlined into")
_COST_RE = re.compile(r"cost=(-?\d+)")
_THRESHOLD_RE = re.compile(r"threshold=(-?\d+)")


def _field(hit: Any, name: str, default=None):
    return hit.get(name, default) if isinstance(hit, dict) else getattr(hit, name, default)


def demangle_last(symbol: str) -> str:
    """Final path segment of a legacy Rust symbol (`_ZN..<n>name17h<hash>E`).

    Anything that is not legacy-mangled is returned unchanged, so a remark
    that already names the function plainly still resolves.
    """
    if not (symbol.startswith("_ZN") and symbol.endswith("E")):
        return symbol
    body, segments, i = symbol[3:-1], [], 0
    while i < len(body):
        j = i
        while j < len(body) and body[j].isdigit():
            j += 1
        if j == i:
            return symbol
        n = int(body[i:j])
        segments.append(body[j:j + n])
        i = j + n
    if segments and re.fullmatch(r"h[0-9a-f]{16}", segments[-1]):
        segments.pop()
    return segments[-1] if segments else symbol


def inline_remark_facts(message: str) -> dict[str, Any]:
    """{callee, callee_symbol, cost, threshold} from an inline-missed remark.

    Parsed from whatever text is available; a field the text no longer holds
    (the snippet stored with a hit is cut at 200 characters) is None.
    """
    m = _CALLEE_RE.search(message or "")
    if m is None:
        return {}
    cost = _COST_RE.search(message)
    threshold = _THRESHOLD_RE.search(message)
    return {
        "callee": demangle_last(m.group(1)),
        "callee_symbol": m.group(1),
        "cost": int(cost.group(1)) if cost else None,
        "threshold": int(threshold.group(1)) if threshold else None,
    }


@dataclass(frozen=True)
class InlineCallee:
    name: str
    cost: Optional[int]
    n_sites: int


def ii_inl_callees(hits: Iterable[Any]) -> list[InlineCallee]:
    """Distinct too-costly callees named by a hot function's II_inl hits.

    The cost kept per callee is the largest over its call sites: the attribute
    has to get the costliest site inlined for the callee to stop being a call.
    """
    by_name: dict[str, list[Optional[int]]] = {}
    for hit in hits:
        if _field(hit, "rule") != "II_inl" or _field(hit, "pattern") != "TooCostly":
            continue
        extra = _field(hit, "extra", {}) or {}
        facts = ({"callee": extra["callee"], "cost": extra.get("cost")}
                 if extra.get("callee") else inline_remark_facts(_field(hit, "snippet", "")))
        if not facts.get("callee"):
            continue
        by_name.setdefault(facts["callee"], []).append(facts.get("cost"))
    out = []
    for name, costs in by_name.items():
        known = [c for c in costs if c is not None]
        out.append(InlineCallee(name, max(known) if known else None, len(costs)))
    return out


def choose_inline_attribute(cost: Optional[int]) -> str:
    """`inline` when a hint is enough to get it inlined, else `inline(always)`."""
    if cost is not None and cost <= INLINE_HINT_THRESHOLD:
        return "inline"
    return "inline(always)"


def rank_callees(callees: Iterable[InlineCallee], hot_self_pct: dict[str, float],
                 *, exclude: Iterable[str] = (), limit: int = 2,
                 hot_only: bool = True) -> list[InlineCallee]:
    """Callees worth an attempt, best first.

    Only a callee that is itself a hot function is attempted (`hot_only`):
    inlining pays per EXECUTED call, and the callee owning self time in the
    profile is the direct evidence that its call sits on the hot path. The
    remarks name every too-costly call in a hot function, and most are cold —
    error reporters, one-shot setup, resizes — where forcing a 1000-cost body
    into the caller is pure risk (a dry run over twelve projects nominated 51
    callees; the cold ones included `*ErrMemory`, `png_error`, and a 3055-cost
    command registrar). Among hot callees the hotter goes first, then the
    cheaper, which grows its callers least.
    """
    skip = set(exclude)
    ranked = sorted(
        (c for c in callees if c.name not in skip
         and (not hot_only or hot_self_pct.get(c.name, 0.0) > 0.0)),
        key=lambda c: (-hot_self_pct.get(c.name, 0.0),
                       c.cost if c.cost is not None else 1 << 30, c.name),
    )
    return ranked[:limit]


@dataclass(frozen=True)
class InlineAttrPlan:
    proposal: ProposedChangeSet | None
    abstain_reason: str | None = None


def plan_inline_attr(
    *, rule_id: str, hot_function: str, target_fn: str, relative_path: str,
    line_hint: int, attribute: str, base_head: str, candidate_id: str,
    hit_ids: Iterable[str] = (),
) -> InlineAttrPlan:
    if attribute not in SetFunctionInlineAttr.ALLOWED:
        return InlineAttrPlan(None, f"unsupported_inline_attribute:{attribute}")
    if not target_fn or not relative_path:
        return InlineAttrPlan(None, "inline_target_unresolved")
    hit_ids = tuple(hit_ids)
    seed = f"{candidate_id}|{target_fn}|{attribute}"
    operation = SetFunctionInlineAttr(
        operation_id="inline-attr-" + hashlib.sha256(seed.encode()).hexdigest()[:16],
        rule_id=rule_id,
        evidence_hit_ids=hit_ids,
        relative_path=relative_path,
        fn_name=target_fn,
        line_hint=line_hint,
        attribute=attribute,
    )
    prefix = rule_id.lower().replace("_", "-")
    proposal = ProposedChangeSet(
        changeset_id=f"{prefix}-" + hashlib.sha256(
            (seed + "|" + operation.operation_id).encode()).hexdigest()[:16],
        base_head=base_head,
        trigger=TriggerContext(
            rule_id=rule_id, candidate_id=candidate_id,
            hot_function=hot_function, hit_ids=hit_ids,
            evidence_artifacts=(),
        ),
        operations=(operation,),
        impact_scope=ImpactScope.CRATE_GLOBAL,
    )
    return InlineAttrPlan(proposal)
