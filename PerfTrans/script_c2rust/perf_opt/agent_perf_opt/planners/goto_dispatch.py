"""Deterministic planner for C11 — narrowing c2rust's goto-lowering labels.

The rewrite is a bijection over one variable's state constants plus its carrier
type; no control flow moves, so there is nothing for a model to decide. This
planner turns the detector's hits into a `ReplaceFunctionBody` operation
carrying the already-rewritten source.

Both patterns the detector emits are planned here, selected by `strategy`:
`narrow` renumbers the labels, `split` gives the hot arm of a loop-head
dispatch its own loop. They are alternatives, not steps — whichever one takes
the dispatch off the hot path leaves the other nothing to remove.
"""

from __future__ import annotations

import hashlib
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ProposedChangeSet,
    ReplaceFunctionBody,
    SymbolKind,
    SymbolRef,
    TriggerContext,
)
from perf_opt.agent_perf_opt.goto_dispatch_lower import (
    UnsupportedGotoDispatch,
    narrow,
    split_loop_head,
)
from perf_opt.agent_perf_opt.rewrite_applier import (
    EditTarget,
    locate_fn_span_with_attrs,
)

NARROW_PATTERN = "goto-dispatch-wide-constants"
SPLIT_PATTERN = "goto-dispatch-loop-head"

# Ordered: the narrowing is a pure symbol substitution and moves no control
# flow, so it is tried first. They do not stack — whichever one takes the
# dispatch off the hot path leaves the other nothing to remove — so the caller
# falls through to the split only when the measurement gate rejects the narrow.
STRATEGIES = ("narrow", "split")
_PATTERN_FOR = {"narrow": NARROW_PATTERN, "split": SPLIT_PATTERN}


@dataclass(frozen=True)
class GotoDispatchPlan:
    proposal: ProposedChangeSet | None
    abstain_reason: str | None = None
    var_states: dict[str, int] | None = None
    literals_rewritten: int = 0
    edit_target: EditTarget | None = None
    strategy: str = "narrow"


def _field(hit: Any, name: str, default=None):
    return (
        hit.get(name, default) if isinstance(hit, dict)
        else getattr(hit, name, default)
    )


def plan_goto_dispatch(
    *,
    crate: Path,
    hot_function: str,
    hits: list[Any],
    base_head: str,
    candidate_id: str,
    fn_span: tuple[str, int, int] | None = None,
    strategy: str = "narrow",
) -> GotoDispatchPlan:
    """Plan the label narrowing for one hot function.

    `fn_span` is (relative_path, first_line, last_line). When absent it is
    taken from the hits, which carry the declaration line the detector anchored
    on — but not the end, so a caller that has the real span should pass it.
    """
    if strategy not in _PATTERN_FOR:
        return GotoDispatchPlan(None, f"unknown_strategy:{strategy}")
    wanted = _PATTERN_FOR[strategy]
    relevant = [h for h in hits
                if _field(h, "rule") == "C11"
                and _field(h, "pattern") == wanted]
    if not relevant:
        return GotoDispatchPlan(None, f"no_c11_{strategy}_targets")

    if fn_span is None:
        return GotoDispatchPlan(None, "incomplete_target_evidence")
    rel, first, last = fn_span

    try:
        source = (crate / rel).read_bytes()
    except OSError as exc:
        return GotoDispatchPlan(None, f"unreadable_source:{exc.errno}")

    # The handler splices a byte span that includes the function's preceding
    # attributes, and checks it against a hash of the bytes as they are NOW.
    span = locate_fn_span_with_attrs(
        crate / rel, hot_function, hint_line_range=(first, last))
    if span is None:
        return GotoDispatchPlan(None, "function_span_not_located")

    # The caller's line numbers were recorded when the run started; by now
    # earlier commits have moved the file under them. Measured on a compression
    # crate: six commits shifted one hot function by 56 lines, and scanning the
    # stale window narrowed the declaration while leaving two assignments
    # beyond it untouched. `span` is the authority — derive the window from it.
    first = source[:span[0]].count(b"\n") + 1
    last = source[:span[1]].count(b"\n") + 1
    transform = narrow if strategy == "narrow" else split_loop_head
    try:
        result = transform(source, first, last, span=span)
    except UnsupportedGotoDispatch as exc:
        return GotoDispatchPlan(None, str(exc))

    hit_ids = tuple(
        str(_field(h, "id", _field(h, "hit_id", "")))
        for h in relevant
        if _field(h, "id", _field(h, "hit_id", ""))
    )
    seed = f"{candidate_id}|{hot_function}|{rel}|{strategy}"
    operation_id = strategy + "-" + hashlib.sha256(seed.encode()).hexdigest()[:16]
    operation = ReplaceFunctionBody(
        operation_id=operation_id,
        rule_id="C11",
        evidence_hit_ids=hit_ids,
        target=SymbolRef(
            qualified_name=hot_function,
            symbol_kind=SymbolKind.FUNCTION,
            file_hint=rel,
            # Hash of the CURRENT bytes — this is the staleness check, not a
            # fingerprint of the replacement.
            declaration_hash=hashlib.sha256(
                source[span[0]:span[1]]).hexdigest(),
        ),
        replacement_function_source=result.function_source,
    )
    var_states = dict(getattr(result, "var_states", {}) or {})
    proposal = ProposedChangeSet(
        changeset_id="c11-" + strategy + "-" + hashlib.sha256(
            (candidate_id + "|" + operation_id).encode()).hexdigest()[:16],
        base_head=base_head,
        trigger=TriggerContext(
            rule_id="C11", candidate_id=candidate_id,
            hot_function=hot_function, hit_ids=hit_ids,
            evidence_artifacts=(),
        ),
        operations=(operation,),
        # Only this function's body changes; nothing it declares is visible
        # elsewhere, since the state variable is a local.
        impact_scope=ImpactScope.LOCAL_FUNCTION,
    )
    return GotoDispatchPlan(
        proposal, None, var_states,
        getattr(result, "literals_rewritten", 0),
        EditTarget(file=crate / rel, fn_name=hot_function, span=span),
        strategy)
