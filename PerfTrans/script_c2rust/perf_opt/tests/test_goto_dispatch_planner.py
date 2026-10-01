"""C11's planner turns detector hits into one spliceable operation.

The rewrite is deterministic, so there is no prompt and no model judgement in
this path — the planner's job is to hand the changeset layer a byte span, the
hash of what is there now, and the replacement text.

The staleness hash is the subtle part: it must be a hash of the bytes the
handler will overwrite, not of the replacement. Hashing the replacement makes
every resolve fail as "stale" the moment it is compared against the file.
"""

from __future__ import annotations

import hashlib
import re
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ReplaceFunctionBody,
    SymbolKind,
)
from perf_opt.agent_perf_opt.planners.goto_dispatch import (
    NARROW_PATTERN,
    SPLIT_PATTERN,
    plan_goto_dispatch,
)

BIG_A = 12147880666119273379
BIG_B = 8038949400865391589

SRC = f"""#[inline(always)]
unsafe fn hot() {{
    let mut current_block: u64;
    current_block = {BIG_A};
    loop {{
        match current_block {{
            {BIG_A} => {{ current_block = {BIG_B}; }}
            {BIG_B} => {{ break; }}
            _ => {{ }}
        }}
    }}
}}
"""


SPLIT_SRC = f"""#[inline(always)]
unsafe fn hot() {{
    let mut current_block: u64;
    current_block = {BIG_A};
    loop {{
        match current_block {{
            {BIG_A} => {{ step(); current_block = {BIG_A}; continue; }}
            _ => {{ other(); current_block = {BIG_B}; }}
        }}
        tail();
    }}
}}
"""


def _split_crate(tmp_path: Path) -> Path:
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "m.rs").write_text(SPLIT_SRC)
    return tmp_path


def _crate(tmp_path: Path) -> Path:
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "m.rs").write_text(SRC)
    return tmp_path


def _hit(pattern: str = NARROW_PATTERN) -> dict:
    return {"rule": "C11", "pattern": pattern, "id": "h1",
            "file": "src/m.rs", "line": 2,
            "extra": {"var": "current_block", "states": 2}}


def _plan(tmp_path, hits=None, span=("src/m.rs", 1, SRC.count("\n") + 1)):
    return plan_goto_dispatch(
        crate=_crate(tmp_path), hot_function="hot",
        hits=hits if hits is not None else [_hit()],
        base_head="0" * 40, candidate_id="cand-1", fn_span=span,
    )


def test_a_narrowing_hit_produces_one_operation(tmp_path) -> None:
    plan = _plan(tmp_path)
    assert plan.abstain_reason is None, plan.abstain_reason
    assert len(plan.proposal.operations) == 1
    assert isinstance(plan.proposal.operations[0], ReplaceFunctionBody)


def test_the_operation_carries_the_rewritten_source(tmp_path) -> None:
    op = _plan(tmp_path).proposal.operations[0]
    assert "current_block: u32" in op.replacement_function_source
    assert str(BIG_A) not in op.replacement_function_source


def test_the_replacement_keeps_the_preceding_attribute(tmp_path) -> None:
    """The span the handler splices starts at `#[inline(always)]`. A
    replacement that began at `unsafe fn` would delete the attribute — and on a
    force-inlined function that is a performance change all by itself."""
    op = _plan(tmp_path).proposal.operations[0]
    assert op.replacement_function_source.startswith("#[inline(always)]")


def test_the_hash_is_of_the_bytes_being_replaced(tmp_path) -> None:
    crate = _crate(tmp_path)
    plan = plan_goto_dispatch(
        crate=crate, hot_function="hot", hits=[_hit()],
        base_head="0" * 40, candidate_id="cand-1",
        fn_span=("src/m.rs", 1, SRC.count("\n") + 1))
    op = plan.proposal.operations[0]
    lo, hi = plan.edit_target.span
    data = (crate / "src" / "m.rs").read_bytes()
    assert op.target.declaration_hash == hashlib.sha256(data[lo:hi]).hexdigest()
    assert op.target.declaration_hash != hashlib.sha256(
        op.replacement_function_source.encode()).hexdigest()


def test_the_edit_target_span_matches_the_replacement_extent(tmp_path) -> None:
    crate = _crate(tmp_path)
    plan = plan_goto_dispatch(
        crate=crate, hot_function="hot", hits=[_hit()],
        base_head="0" * 40, candidate_id="cand-1",
        fn_span=("src/m.rs", 1, SRC.count("\n") + 1))
    lo, hi = plan.edit_target.span
    data = (crate / "src" / "m.rs").read_bytes()
    spliced = data[:lo] + plan.proposal.operations[0].replacement_function_source.encode() + data[hi:]
    assert spliced.decode().count("{") == spliced.decode().count("}")
    assert str(BIG_A).encode() not in spliced


def test_the_scope_is_local(tmp_path) -> None:
    """The state variable is a local; nothing it declares escapes."""
    assert _plan(tmp_path).proposal.impact_scope is ImpactScope.LOCAL_FUNCTION


def test_the_target_is_a_function(tmp_path) -> None:
    op = _plan(tmp_path).proposal.operations[0]
    assert op.target.symbol_kind is SymbolKind.FUNCTION
    assert op.rule_id == "C11"


# ────────────────────────────────── abstentions

def test_a_strategy_only_plans_its_own_pattern(tmp_path) -> None:
    """The two rewrites are alternatives, not steps. A hit set carrying only
    the loop-head pattern has nothing for the narrowing strategy to do."""
    plan = _plan(tmp_path, hits=[_hit(SPLIT_PATTERN)])
    assert plan.proposal is None
    assert plan.abstain_reason == "no_c11_narrow_targets"


def test_the_split_strategy_plans_the_loop_head_pattern(tmp_path) -> None:
    crate = _split_crate(tmp_path)
    plan = plan_goto_dispatch(
        crate=crate, hot_function="hot", hits=[_hit(SPLIT_PATTERN)],
        base_head="0" * 40, candidate_id="c",
        fn_span=("src/m.rs", 1, SPLIT_SRC.count("\n") + 1), strategy="split")
    assert plan.abstain_reason is None, plan.abstain_reason
    src = plan.proposal.operations[0].replacement_function_source
    assert "'outer: loop {" in src and "'fast: loop {" in src
    assert "break 'fast;" in src
    assert plan.strategy == "split"


def test_the_two_strategies_produce_different_changesets(tmp_path) -> None:
    """Same function, same candidate — the ids must not collide, or the second
    attempt would look like a replay of the first."""
    crate = _split_crate(tmp_path)
    span = ("src/m.rs", 1, SPLIT_SRC.count("\n") + 1)
    a = plan_goto_dispatch(crate=crate, hot_function="hot", hits=[_hit()],
                           base_head="0" * 40, candidate_id="c",
                           fn_span=span, strategy="narrow")
    b = plan_goto_dispatch(crate=crate, hot_function="hot",
                           hits=[_hit(SPLIT_PATTERN)], base_head="0" * 40,
                           candidate_id="c", fn_span=span, strategy="split")
    assert a.proposal.changeset_id != b.proposal.changeset_id
    assert (a.proposal.operations[0].operation_id
            != b.proposal.operations[0].operation_id)


def test_an_unknown_strategy_abstains(tmp_path) -> None:
    bad = plan_goto_dispatch(
        crate=_crate(tmp_path), hot_function="hot", hits=[_hit()],
        base_head="0" * 40, candidate_id="c",
        fn_span=("src/m.rs", 1, SRC.count("\n") + 1), strategy="nope")
    assert bad.proposal is None
    assert bad.abstain_reason == "unknown_strategy:nope"


def test_hits_for_another_rule_are_ignored(tmp_path) -> None:
    plan = _plan(tmp_path, hits=[{"rule": "C4", "pattern": NARROW_PATTERN,
                                  "id": "h1", "extra": {}}])
    assert plan.proposal is None


def test_a_missing_span_abstains(tmp_path) -> None:
    plan = plan_goto_dispatch(
        crate=_crate(tmp_path), hot_function="hot", hits=[_hit()],
        base_head="0" * 40, candidate_id="c", fn_span=None)
    assert plan.proposal is None
    assert plan.abstain_reason == "incomplete_target_evidence"


def test_a_function_the_span_cannot_locate_abstains(tmp_path) -> None:
    plan = plan_goto_dispatch(
        crate=_crate(tmp_path), hot_function="not_here", hits=[_hit()],
        base_head="0" * 40, candidate_id="c",
        fn_span=("src/m.rs", 1, SRC.count("\n") + 1))
    assert plan.proposal is None
    assert plan.abstain_reason == "function_span_not_located"


# ────────────────────────────────── the typed dispatch table

def test_every_typed_rule_has_its_own_applier() -> None:
    """Adding a name to `_TYPED_RULES` without an applier used to fall through
    to whichever branch the if/else ended on — a rule silently running another
    rule's handler."""
    from perf_opt.agent_perf_opt.agent import _TYPED_APPLIERS, _TYPED_RULES
    assert set(_TYPED_APPLIERS) == set(_TYPED_RULES)
    assert len({id(f) for f in _TYPED_APPLIERS.values()}) == len(_TYPED_APPLIERS)


# ────────────────────────────────── the two-strategy fallback

def test_only_a_measurement_rejection_moves_to_the_next_strategy() -> None:
    """The two rewrites are alternatives, so a rewrite that was correct but did
    not pay is the one case worth retrying differently. A build or W1 failure
    says this function cannot be rewritten at all — trying the other strategy
    would just spend another measurement to learn the same thing."""
    from perf_opt.agent_perf_opt.agent import _C11_TRY_NEXT
    from perf_opt.agent_perf_opt.changeset.types import ChangeSetStatus
    assert _C11_TRY_NEXT == {
        ChangeSetStatus.REJECTED_W2_NO_GAIN,
        ChangeSetStatus.REJECTED_W2_REGRESS,
        ChangeSetStatus.REJECTED_UNMEASURABLE,
    }
    assert ChangeSetStatus.COMMITTED not in _C11_TRY_NEXT
    assert ChangeSetStatus.REJECTED_W1 not in _C11_TRY_NEXT


def test_narrowing_is_attempted_before_the_split() -> None:
    """It moves no control flow, so it is the cheaper thing to be wrong about."""
    from perf_opt.agent_perf_opt.planners.goto_dispatch import STRATEGIES
    assert STRATEGIES == ("narrow", "split")


# ────────────────────────────────── stale line hints

STALE_SRC = f"""// padding line
// padding line
// padding line
unsafe fn earlier() {{
    let mut current_block: u64;
    current_block = 1111111111111111111;
    current_block = 2222222222222222222;
    match current_block {{ 1111111111111111111 => {{ }} _ => {{ }} }}
}}

#[inline(always)]
unsafe fn hot() {{
    let mut current_block: u64;
    current_block = {BIG_A};
    loop {{
        match current_block {{
            {BIG_A} => {{ step(); }}
            _ => {{ current_block = {BIG_B}; }}
        }}
    }}
}}
"""


def test_a_stale_line_hint_does_not_split_the_rewrite(tmp_path) -> None:
    """`hf.line_start/line_end` are recorded when the run starts. By the time a
    rule executes, earlier commits have moved the file: measured on a
    compression crate, six commits shifted one hot function by 56 lines. The
    stale window then covered the tail of the previous function and missed the
    end of this one — narrowing the declaration while leaving two assignments
    beyond it at full width, which the compiler caught only because the carrier
    had been narrowed too. Had it not been, the state machine would have
    compiled and routed wrongly.

    The located span is the authority; the hint may only disambiguate names.
    """
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "m.rs").write_text(STALE_SRC)
    plan = plan_goto_dispatch(
        crate=tmp_path, hot_function="hot", hits=[_hit()],
        base_head="0" * 40, candidate_id="c",
        # Deliberately wrong: points at `earlier`, five lines short of `hot`.
        fn_span=("src/m.rs", 1, 9),
    )
    assert plan.abstain_reason is None, plan.abstain_reason
    replacement = plan.proposal.operations[0].replacement_function_source
    assert replacement.lstrip().startswith("#[inline(always)]")
    # Every label of the rewritten function moved, none left behind.
    assert str(BIG_A) not in replacement
    assert str(BIG_B) not in replacement
    assert "current_block: u32" in replacement


def test_the_neighbour_function_is_untouched_by_a_stale_hint(tmp_path) -> None:
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "m.rs").write_text(STALE_SRC)
    plan = plan_goto_dispatch(
        crate=tmp_path, hot_function="hot", hits=[_hit()],
        base_head="0" * 40, candidate_id="c", fn_span=("src/m.rs", 1, 9))
    lo, hi = plan.edit_target.span
    data = (tmp_path / "src" / "m.rs").read_bytes()
    spliced = (data[:lo]
               + plan.proposal.operations[0].replacement_function_source.encode()
               + data[hi:]).decode()
    assert "1111111111111111111" in spliced, "earlier() must keep its own labels"
    assert spliced.count("{") == spliced.count("}")


# ────────────────────────────────── the extra-key trap

def test_every_typed_extra_key_is_an_attempt_record_field() -> None:
    """`extra` is splatted into AttemptRecord, so a key that is not a field is a
    TypeError — and it fires after the rewrite has already been applied and
    gated, taking the whole run down with it. `sub_rule_id` is the one key that
    is allowed through, and only because the record builder pops it."""
    import dataclasses
    import inspect

    from perf_opt.agent_perf_opt import agent as agent_mod
    from perf_opt.agent_perf_opt.state import AttemptRecord

    fields = {f.name for f in dataclasses.fields(AttemptRecord)}
    src = inspect.getsource(agent_mod._run_goto_dispatch_strategy)
    keys = set(re.findall(r'^\s+"([a-z_]+)":', src, re.M))
    stray = keys - fields - {"reason", "detail", "delta_pct", "measured_cv",
                             "triggering_op", "ops", "measurements", "stderr"}
    assert not stray, f"extra keys that AttemptRecord cannot take: {stray}"


def test_the_record_builder_pops_the_sub_rule_id() -> None:
    from perf_opt.agent_perf_opt.agent import _make_typed_attempt_record
    from perf_opt.agent_perf_opt.state import RewriteAttempt
    rec = _make_typed_attempt_record(
        _FakeHF(), RewriteAttempt.APPLIED_COMMITTED,
        {"terminal_status": "committed", "sub_rule_id": "C11.split"},
        rule_id="C11")
    assert rec.rule_id == "C11.split"


class _FakeHF:
    name = "hot"
