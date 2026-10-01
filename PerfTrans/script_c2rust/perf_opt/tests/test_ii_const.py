from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.audit import AuditWriter
from perf_opt.agent_perf_opt.changeset.executor import ChangeSetExecutor
from perf_opt.agent_perf_opt.changeset.handlers.promote_static import (
    PromoteStaticHandler,
)
from perf_opt.agent_perf_opt.changeset.registry import HandlerRegistry
from perf_opt.agent_perf_opt.changeset.types import (
    ChangeSetStatus, GateResultRecord, PromoteStaticToConst,
)
from perf_opt.agent_perf_opt.planners.ii_const import plan_ii_const
from perf_opt.hot_probe.static_facts import scan_global_declarations


def _fixture(tmp_path: Path):
    crate = tmp_path / "crate"
    constants = crate / "src" / "constants.rs"
    hot = crate / "src" / "hot.rs"
    constants.parent.mkdir(parents=True)
    constants.write_text(
        "pub(crate) static mut PRIME32_1: u32 = 2654435761;\n"
    )
    hot.write_text(
        "pub fn round32(x: u32) -> u32 { "
        "x.wrapping_mul(crate::constants::PRIME32_1) }\n"
    )
    fact = scan_global_declarations(crate)[0]
    hit = {
        "id": "hit-1",
        "rule": "II_const",
        "extra": {
            "symbol_name": fact.name,
            "qualified_name": fact.qualified_name,
            "symbol_kind": "static",
            "decl_file": fact.relative_path,
            "declaration_hash": fact.declaration_hash,
        },
    }
    return crate, hot, hit


def test_ii_const_edits_declaration_not_hot_function(tmp_path: Path) -> None:
    crate, hot, hit = _fixture(tmp_path)
    hot_before = hot.read_bytes()
    constants = crate / "src/constants.rs"
    constants_before = constants.read_bytes()
    planned = plan_ii_const(
        hot_function="round32",
        hits=[hit],
        base_head="head-1",
        candidate_id="round32-II_const",
    )
    assert planned.proposal is not None
    operation = planned.proposal.operations[0]
    assert isinstance(operation, PromoteStaticToConst)
    handler = PromoteStaticHandler()
    resolution = handler.resolve(operation, crate)
    assert handler.pre_validate(operation, resolution, crate).ok
    assert len(resolution.edits) == 1
    edit = resolution.edits[0]
    assert constants_before[edit.start_byte:edit.end_byte] == b"static mut"
    assert edit.replacement_text == "const"
    applier = ChangeSetApplier(crate, tmp_path / "txn")

    applied = applier.apply(planned.proposal.changeset_id, resolution.edits)

    assert handler.post_validate(operation, resolution, crate).ok
    assert hot.read_bytes() == hot_before
    promoted = constants.read_text()
    assert "pub(crate) const PRIME32_1" in promoted
    assert "const mut" not in promoted
    applier.restore(applied)


def test_two_mutable_statics_in_one_file_are_promoted_together(
    tmp_path: Path,
) -> None:
    crate = tmp_path / "crate"
    source = crate / "src" / "xxhash.rs"
    source.parent.mkdir(parents=True)
    source.write_text(
        "static mut PRIME64_1: u64 = 11;\n"
        "static mut PRIME64_2: u64 = 13;\n"
        "fn round(x: u64) -> u64 { "
        "x.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_2) }\n"
    )
    facts = {fact.name: fact for fact in scan_global_declarations(crate)}
    hits = [
        {
            "id": f"hit-{name}",
            "rule": "II_const",
            "extra": {
                "symbol_name": name,
                "qualified_name": facts[name].qualified_name,
                "symbol_kind": "static",
                "decl_file": facts[name].relative_path,
                "declaration_hash": facts[name].declaration_hash,
            },
        }
        for name in ("PRIME64_1", "PRIME64_2")
    ]
    planned = plan_ii_const(
        hot_function="round",
        hits=hits,
        base_head="head-1",
        candidate_id="round-II_const",
    )
    assert planned.proposal is not None
    handler = PromoteStaticHandler()
    resolutions = [
        handler.resolve(operation, crate)
        for operation in planned.proposal.operations
    ]
    edits = [edit for resolution in resolutions for edit in resolution.edits]

    ChangeSetApplier(crate, tmp_path / "txn").apply(
        planned.proposal.changeset_id, edits
    )

    promoted = source.read_text()
    assert promoted.count("const PRIME64_") == 2
    assert "static mut PRIME64_" not in promoted


def test_ii_const_rejects_stale_declaration_hash(tmp_path: Path) -> None:
    crate, _, hit = _fixture(tmp_path)
    planned = plan_ii_const(
        hot_function="round32",
        hits=[hit],
        base_head="head-1",
        candidate_id="candidate",
    )
    assert planned.proposal is not None
    (crate / "src/constants.rs").write_text("pub static PRIME32_1: u32 = 9;\n")

    with pytest.raises(ValueError, match="stale"):
        PromoteStaticHandler().resolve(planned.proposal.operations[0], crate)


def test_ii_const_abstains_when_target_evidence_is_incomplete() -> None:
    planned = plan_ii_const(
        hot_function="round32",
        hits=[{"rule": "II_const", "extra": {"symbol_name": "PRIME"}}],
        base_head="head-1",
        candidate_id="candidate",
    )

    assert planned.proposal is None
    assert planned.abstain_reason == "incomplete_target_evidence"


def test_repeated_hit_after_promotion_is_audited_as_stale(tmp_path: Path) -> None:
    crate, _, hit = _fixture(tmp_path)
    planned = plan_ii_const(
        hot_function="round32", hits=[hit], base_head="head-1",
        candidate_id="candidate",
    )
    assert planned.proposal is not None
    (crate / "src/constants.rs").write_text("pub const PRIME32_1: u32 = 2654435761;\n")
    registry = HandlerRegistry()
    registry.register(PromoteStaticToConst, PromoteStaticHandler())

    class State:
        def head_sha(self, *, full=True):
            return "head-1"

        def commit_success(self, paths, message):
            raise AssertionError("stale proposal must not commit")

    executor = ChangeSetExecutor(
        crate=crate,
        registry=registry,
        applier=ChangeSetApplier(crate, tmp_path / "txn"),
        state=State(),
        audit=AuditWriter(tmp_path / "opt"),
        candidate_binary=tmp_path / "candidate",
        build_gate=lambda: GateResultRecord("build", True),
        w1_gate=lambda: GateResultRecord("w1", True),
        w2_gate=lambda scope: GateResultRecord("w2", True),
        parent_promoter=lambda path, sha: None,
    )

    result = executor.execute(planned.proposal)

    assert result.terminal_status is ChangeSetStatus.REJECTED_STALE


def test_multi_declaration_prevalidation_is_atomic(tmp_path: Path) -> None:
    crate = tmp_path / "crate"
    source_a = crate / "src" / "a.rs"
    source_b = crate / "src" / "b.rs"
    source_a.parent.mkdir(parents=True)
    source_a.write_text("pub static A: u32 = 1;\n")
    source_b.write_text("pub static B: u32 = 2;\nfn address() { let _ = &B; }\n")
    facts = {fact.name: fact for fact in scan_global_declarations(crate)}
    hits = []
    for name in ("A", "B"):
        fact = facts[name]
        hits.append({
            "id": f"hit-{name}", "rule": "II_const",
            "extra": {
                "symbol_name": name,
                "qualified_name": fact.qualified_name,
                "symbol_kind": "static",
                "decl_file": fact.relative_path,
                "declaration_hash": fact.declaration_hash,
            },
        })
    planned = plan_ii_const(
        hot_function="hot", hits=hits, base_head="head-1",
        candidate_id="multi",
    )
    assert planned.proposal is not None
    registry = HandlerRegistry()
    registry.register(PromoteStaticToConst, PromoteStaticHandler())

    class State:
        def head_sha(self, *, full=True):
            return "head-1"

        def commit_success(self, paths, message):
            raise AssertionError("unsafe multi-operation proposal must not commit")

    executor = ChangeSetExecutor(
        crate=crate, registry=registry,
        applier=ChangeSetApplier(crate, tmp_path / "txn"),
        state=State(), audit=AuditWriter(tmp_path / "opt"),
        candidate_binary=tmp_path / "candidate",
        build_gate=lambda: GateResultRecord("build", True),
        w1_gate=lambda: GateResultRecord("w1", True),
        w2_gate=lambda scope: GateResultRecord("w2", True),
        parent_promoter=lambda path, sha: None,
    )

    result = executor.execute(planned.proposal)

    assert result.terminal_status is ChangeSetStatus.ABSTAINED_UNPROVEN
    assert source_a.read_text().startswith("pub static A")
    assert source_b.read_text().startswith("pub static B")
