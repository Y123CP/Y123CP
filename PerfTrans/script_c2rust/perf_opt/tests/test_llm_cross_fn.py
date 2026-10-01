""                                                       

                                        
   

from pathlib import Path

from perf_opt.agent_perf_opt.caller_lookup import CallerLookup
from perf_opt.agent_perf_opt.changeset.types import (
    ReplaceCallSite, ReplaceFunctionBody,
)
from perf_opt.agent_perf_opt.planners.llm_cross_fn import (
    plan_llm_cross_fn_change,
)
from perf_opt.agent_perf_opt.rewrite_applier import EditTarget

_LIB = (
    "fn target(a: i32, b: i32) -> i32 { a + b }\n"
    "fn caller_one() -> i32 { target(1, 2) }\n"
    "fn caller_two() -> i32 { target(3, 4) }\n"
)


def _setup(tmp_path: Path):
    (tmp_path / "src").mkdir(parents=True, exist_ok=True)
    (tmp_path / "src" / "lib.rs").write_text(_LIB, encoding="utf-8")
    src = (tmp_path / "src" / "lib.rs").read_bytes()
    sites = sorted(
        CallerLookup(tmp_path).callers_of("target"),
        key=lambda s: (s.file, s.start_byte),
    )
    tstart = 0
    tend = src.index(b"\n")                                 
    edit_target = EditTarget(
        file=tmp_path / "src" / "lib.rs", fn_name="target", span=(tstart, tend),
    )
    return tmp_path, sites, edit_target


def _plan(tmp_path, sites, edit_target, response):
    return plan_llm_cross_fn_change(
        response=response,
        edit_target=edit_target,
        caller_sites=sites,
        rule_ids=("III①",),
        hit_ids=("h1",),
        base_head="base",
        candidate_id="cand",
        crate=tmp_path,
    )


_GOOD = (
    "=== TARGET_FUNCTION ===\n"
    "fn target(a: i32, b: i32) -> i32 { a + b + 0 }\n"
    "=== CALL_SITE 0 ===\n"
    "target(10, 20)\n"
    "=== CALL_SITE 1 ===\n"
    "target(30, 40)\n"
)


def test_wellformed_response_builds_atomic_changeset(tmp_path: Path) -> None:
    crate, sites, et = _setup(tmp_path)
    plan = _plan(crate, sites, et, _GOOD)

    assert plan.proposal is not None, plan.abstain_reason
    ops = plan.proposal.operations
                    
    assert len(ops) == 3
    assert isinstance(ops[0], ReplaceFunctionBody)
    assert ops[0].target.qualified_name == "crate::target"
    assert "a + b + 0" in ops[0].replacement_function_source

    call_ops = [o for o in ops if isinstance(o, ReplaceCallSite)]
    assert len(call_ops) == 2
                                       
    by_start = sorted(call_ops, key=lambda o: o.start_byte)
    assert by_start[0].replacement_text == "target(10, 20)"
    assert by_start[1].replacement_text == "target(30, 40)"
                            
    src = (crate / "src" / "lib.rs").read_bytes()
    for o in call_ops:
        assert src[o.start_byte:o.end_byte].decode().startswith("target(")


def test_abstain_response(tmp_path: Path) -> None:
    crate, sites, et = _setup(tmp_path)
    plan = _plan(crate, sites, et, "ABSTAIN: monomorphization not worth it here")
    assert plan.proposal is None
    assert "monomorphization" in (plan.abstain_reason or "")


def test_wrong_call_site_count_abstains(tmp_path: Path) -> None:
    crate, sites, et = _setup(tmp_path)
                                            
    bad = (
        "=== TARGET_FUNCTION ===\n"
        "fn target(a: i32, b: i32) -> i32 { a + b }\n"
        "=== CALL_SITE 0 ===\n"
        "target(1, 2)\n"
    )
    plan = _plan(crate, sites, et, bad)
    assert plan.proposal is None
    assert "call_site_index_mismatch" in (plan.abstain_reason or "")


def test_invalid_target_function_abstains(tmp_path: Path) -> None:
    crate, sites, et = _setup(tmp_path)
    bad = (
        "=== TARGET_FUNCTION ===\n"
        "this is not a function\n"
        "=== CALL_SITE 0 ===\n"
        "target(1, 2)\n"
        "=== CALL_SITE 1 ===\n"
        "target(3, 4)\n"
    )
    plan = _plan(crate, sites, et, bad)
    assert plan.proposal is None


def test_call_site_not_single_expression_abstains(tmp_path: Path) -> None:
    crate, sites, et = _setup(tmp_path)
                             
    bad = (
        "=== TARGET_FUNCTION ===\n"
        "fn target(a: i32, b: i32) -> i32 { a + b }\n"
        "=== CALL_SITE 0 ===\n"
        "let x = target(1, 2);\n"
        "=== CALL_SITE 1 ===\n"
        "target(3, 4)\n"
    )
    plan = _plan(crate, sites, et, bad)
    assert plan.proposal is None
    assert "call_site_0_shape" in (plan.abstain_reason or "")


def test_fences_are_tolerated(tmp_path: Path) -> None:
    crate, sites, et = _setup(tmp_path)
    fenced = "```rust\n" + _GOOD + "```\n"
    plan = _plan(crate, sites, et, fenced)
    assert plan.proposal is not None, plan.abstain_reason
    assert len(plan.proposal.operations) == 3
