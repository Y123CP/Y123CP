""                                                                        
                                                           
                      
                    
                    

                                                      
   

from pathlib import Path

from perf_opt.agent_perf_opt.caller_lookup import CallerLookup
from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.audit import AuditWriter
from perf_opt.agent_perf_opt.changeset.executor import ChangeSetExecutor
from perf_opt.agent_perf_opt.changeset.registry import HandlerRegistry
from perf_opt.agent_perf_opt.changeset.handlers.replace_call_site import (
    ReplaceCallSiteHandler,
)
from perf_opt.agent_perf_opt.changeset.handlers.replace_function import (
    ReplaceFunctionHandler,
)
from perf_opt.agent_perf_opt.changeset.types import (
    ChangeSetStatus, GateResultRecord, ImpactScope, ReplaceCallSite,
    ReplaceFunctionBody,
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

_RESPONSE = (
    "=== TARGET_FUNCTION ===\n"
    "fn target(a: i32, b: i32) -> i32 { a.wrapping_add(b) }\n"
    "=== CALL_SITE 0 ===\n"
    "target(10, 20)\n"
    "=== CALL_SITE 1 ===\n"
    "target(30, 40)\n"
)


class _FakeState:
    def __init__(self, crate: Path) -> None:
        self.crate = crate
        self.audit_log_path = crate / "audit.log"
        self.commits: list = []

    def head_sha(self, *, full: bool = True) -> str:
        return "head-1"

    def commit_success(self, paths, message: str) -> str:
        self.commits.append((tuple(Path(p) for p in paths), message))
        return "commit-1"


def _ok_gate(name):
    def gate(*_args, **_kw):
        return GateResultRecord(name, True, {"reason": "accepted"})
    return gate


def test_mixed_op_changeset_applies_and_commits(tmp_path: Path) -> None:
    (tmp_path / "src").mkdir(parents=True)
    lib = tmp_path / "src" / "lib.rs"
    lib.write_text(_LIB, encoding="utf-8")
    src = lib.read_bytes()

    sites = sorted(
        CallerLookup(tmp_path).callers_of("target"),
        key=lambda s: (s.file, s.start_byte),
    )
    assert len(sites) == 2
    edit_target = EditTarget(
        file=lib, fn_name="target", span=(0, src.index(b"\n")),
    )

    plan = plan_llm_cross_fn_change(
        response=_RESPONSE, edit_target=edit_target, caller_sites=sites,
        rule_ids=("III①",), hit_ids=(), base_head="head-1",
        candidate_id="cand", crate=tmp_path,
    )
    assert plan.proposal is not None, plan.abstain_reason

    registry = HandlerRegistry()
    registry.register(ReplaceFunctionBody, ReplaceFunctionHandler(edit_target))
    registry.register(ReplaceCallSite, ReplaceCallSiteHandler())

    (tmp_path / "bin").write_bytes(b"\x7fELF")
    state = _FakeState(tmp_path)
    result = ChangeSetExecutor(
        crate=tmp_path,
        registry=registry,
        applier=ChangeSetApplier(tmp_path, tmp_path / "journal"),
        state=state,
        audit=AuditWriter(tmp_path / "audit"),
        candidate_binary=tmp_path / "bin",
        build_gate=_ok_gate("build"),
        w1_gate=_ok_gate("w1"),
        w2_gate=_ok_gate("w2"),
        parent_promoter=lambda *_a, **_k: None,
    ).execute(plan.proposal)

                     
    assert result.terminal_status is ChangeSetStatus.COMMITTED, result.terminal_status
    assert len(state.commits) == 1

                       
    out = lib.read_text()
    assert "a.wrapping_add(b)" in out                    
    assert "target(1, 2)" not in out and "target(3, 4)" not in out
    assert "target(10, 20)" in out and "target(30, 40)" in out
                       
    assert out.count("fn ") == 3
