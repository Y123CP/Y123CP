from __future__ import annotations

import importlib
import json
import subprocess
import sys
import types
from pathlib import Path
from types import SimpleNamespace

import pytest

from perf_opt.agent_perf_opt.changeset.types import (
    AppliedChangeSet,
    ChangeSetStatus,
    RegionKind,
    RegionRef,
    ReplaceSourceRegion,
    SymbolKind,
    SymbolRef,
)
from perf_opt.agent_perf_opt.config import AgentConfig
from perf_opt.agent_perf_opt.regions.extractor import (
    RegionCandidate,
    RegionExtractionResult,
    RegionSkipReason,
    SkippedRegionHit,
)
from perf_opt.agent_perf_opt.regions.model import stable_hit_id
from perf_opt.agent_perf_opt.state import RewriteAttempt, StateManager


def _import_agent(monkeypatch: pytest.MonkeyPatch):
    llm_module = types.ModuleType("utils.llm_client")
    llm_module.LLMClient = object
    caller_module = types.ModuleType("perf_opt.agent_perf_opt.caller_lookup")
    caller_module.CallerLookup = object
    monkeypatch.setitem(sys.modules, "utils.llm_client", llm_module)
    monkeypatch.setitem(
        sys.modules, "perf_opt.agent_perf_opt.caller_lookup", caller_module
    )
    sys.modules.pop("perf_opt.agent_perf_opt.agent", None)
    return importlib.import_module("perf_opt.agent_perf_opt.agent")


def _hit(rule: str, line: int) -> dict:
    return {
        "file": "src/lib.rs",
        "function": "hot",
        "line": line,
        "col": 5,
        "rule": rule,
        "pattern": f"pattern-{line}",
        "snippet": f"snippet-{line}",
    }


def _candidate(hit: dict, *, start: int, digest: str) -> RegionCandidate:
    hit_id = stable_hit_id(hit, "hot")
    region = RegionRef(
        target_function=SymbolRef(
            qualified_name="crate::hot",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint="src/lib.rs",
            declaration_hash=digest * 64,
        ),
        relative_path="src/lib.rs",
        region_kind=RegionKind.STATEMENT,
        parent_kind="block",
        start_byte=start,
        end_byte=start + 8,
        expected_region_hash=digest * 64,
        anchor_hit_ids=(hit_id,),
        anchor_lines=(hit["line"],),
    )
    return RegionCandidate(region=region, rule_ids=(hit["rule"],))


class _FakeExtractor:
    def __init__(self, results: list[RegionExtractionResult]) -> None:
        self.results = list(results)
        self.calls: list[tuple[object, tuple[dict, ...]]] = []

    def extract(self, *, crate, edit_target, hits):
        self.calls.append((edit_target, tuple(hits)))
        return self.results.pop(0)


class _FakeLLM:
    def __init__(self, response: str = "region response") -> None:
        self.calls: list[dict] = []
        self.response = response

    def chat(self, system: str, user: str, meta: dict) -> str:
        self.calls.append(meta)
        return self.response


def _run(
    agent,
    monkeypatch: pytest.MonkeyPatch,
    tmp_path: Path,
    *,
    hits: list[dict],
    extraction_results: list[RegionExtractionResult],
    outcomes: list[tuple[RewriteAttempt, dict]] | None = None,
    cfg: AgentConfig | None = None,
    candidate_budget: int = 20,
    planner_abstain_reason: str | None = None,
    llm_response: str = "region response",
    apply_override=None,
    persisted_out: list | None = None,
):
    extractor = _FakeExtractor(extraction_results)
    monkeypatch.setattr(agent, "RegionExtractor", lambda: extractor)
    monkeypatch.setattr(
        agent,
        "build_region_prompt",
        lambda **kwargs: ("system", "user"),
    )
    planned_regions: list[RegionRef] = []

    def fake_plan(**kwargs):
        planned_regions.append(kwargs["region"])
        if planner_abstain_reason is not None:
            return SimpleNamespace(
                proposal=None, abstain_reason=planner_abstain_reason
            )
        operation = ReplaceSourceRegion(
            operation_id=f"op-{len(planned_regions)}",
            rule_id=",".join(kwargs["rule_ids"]),
            evidence_hit_ids=kwargs["region"].anchor_hit_ids,
            region=kwargs["region"],
            replacement_region_source="x += 1;",
        )
        return SimpleNamespace(
            proposal=SimpleNamespace(operations=(operation,)),
            abstain_reason=None,
        )

    # Patch where it is CALLED, not where it was once imported. The region
    # loop now goes through `plan_region_with_contract_repair`, which resolves
    # the planner in its own module — a stub on `agent` binds a name nothing
    # reads, and every response silently falls through to the real planner.
    from perf_opt.agent_perf_opt.planners import llm_region as _llm_region
    monkeypatch.setattr(_llm_region, "plan_llm_region_change", fake_plan)
    queued_outcomes = list(outcomes or [])

    def fake_apply(**kwargs):
        if queued_outcomes:
            return queued_outcomes.pop(0)
        return RewriteAttempt.ABSTAINED, {
            "terminal_status": ChangeSetStatus.REJECTED_W2_NO_GAIN.value,
            "error": "no gain",
        }

    monkeypatch.setattr(agent, "_apply_region_and_gate",
                        apply_override or fake_apply)
    llm = _FakeLLM(llm_response)
    # `log` is part of the contract now: the region loop persists each record
    # as it is produced rather than handing the batch back to the caller.
    # Capturing them here lets a test assert that, and a stub without `log`
    # would just raise.
    persisted: list = persisted_out if persisted_out is not None else []
    state = SimpleNamespace(
        crate=tmp_path,
        audit_log_path=tmp_path / "rewrites.log",
        head_sha=lambda **kwargs: "head-1",
        log=persisted.append,
        persisted=persisted,
    )
    hf = SimpleNamespace(name="hot", hottest_op="op", per_op={"op": 1.0})
    edit_target = SimpleNamespace(
        file=tmp_path / "src/lib.rs", fn_name="hot", span=(0, 100)
    )
    records, returned_llm, committed = agent._try_large_fn_regions(
        hf,
        SimpleNamespace(),
        edit_target,
        hits,
        llm=llm,
        state=state,
        w2_session=SimpleNamespace(),
        verifier=object(),
        assets=object(),
        harness_dir=tmp_path / "harness",
        crate=tmp_path,
        cfg=cfg or AgentConfig(),
        candidate_budget=candidate_budget,
    )
    return records, returned_llm, committed, extractor, planned_regions, llm


def _git(repo: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=repo,
        text=True,
        capture_output=True,
        check=True,
    )
    return result.stdout.strip()


def test_large_function_region_e2e_commits_and_logs_anchors(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    crate = tmp_path / "crate"
    source = crate / "src" / "lib.rs"
    source.parent.mkdir(parents=True)
    lines = ["pub fn hot(mut acc: u64) -> u64 {"]
    target_line = 0
    for index in range(1, 261):
        if index == 160:
            target_line = len(lines) + 1
            lines.append("    acc = acc.wrapping_add(1); // target-region")
        else:
            lines.append(
                f"    acc = acc.wrapping_add(0); // pad-{index:03d}"
            )
    lines.extend(["    acc", "}"])
    source.write_text("\n".join(lines) + "\n", encoding="utf-8")
    (crate / "Cargo.toml").write_text(
        '[package]\nname = "region_fixture"\nversion = "0.1.0"\n'
        'edition = "2021"\n',
        encoding="utf-8",
    )
    _git(crate, "init", "-q")
    _git(crate, "config", "user.email", "test@example.com")
    _git(crate, "config", "user.name", "Test User")
    _git(crate, "add", "Cargo.toml", "src/lib.rs")
    _git(crate, "commit", "-q", "-m", "pristine")
    pristine_head = _git(crate, "rev-parse", "HEAD")
    before = source.read_text(encoding="utf-8")

    harness_dir = tmp_path / "harness"
    candidate_binary = harness_dir / "target" / "release" / "harness"
    candidate_binary.parent.mkdir(parents=True)
    candidate_binary.write_bytes(b"fake candidate binary")
    opt_dir = tmp_path / "opt"
    state = StateManager(crate, opt_dir / "rewrites.log")
    hot_fn = SimpleNamespace(
        name="hot",
        file="src/lib.rs",
        line_start=1,
        line_end=len(lines),
        hottest_op="op",
        per_op={"op": 100.0},
    )
    evidence = SimpleNamespace(llvm_opt_remarks=[])
    edit_target = agent.resolve_edit_target(
        hot_fn,
        evidence,
        "C3",
        agent.build_fn_index(crate),
        crate,
    )
    assert edit_target is not None
    hit = {
        "file": "src/lib.rs",
        "function": "hot",
        "line": target_line,
        "col": 28,
        "rule": "C3",
        "pattern": "loop-invariant load",
        "snippet": "1",
    }
    is_large, large_reason = agent._is_large_fn(hot_fn, 1, [hit])
    assert is_large is True
    assert "250" in large_reason

    class FakeLLM:
        def __init__(self) -> None:
            self.calls: list[tuple[str, str, dict]] = []

        def chat(self, system: str, user: str, meta: dict) -> str:
            self.calls.append((system, user, meta))
            # The editable window is grown to a complete-statement shape, and
            # the region validator requires the replacement to have that same
            # shape — so this stands in for a whole statement, not the bare
            # expression an earlier (expression-sized region) revision used.
            return (
                "```rust\n"
                "// Applied rules: [C3]\n"
                "// Skipped rules: []\n"
                "acc = acc.wrapping_add(2);\n"
                "```"
            )

    gate_calls: list[str] = []
    promotions: list[tuple[Path, str]] = []

    def fake_cargo_check(*args, **kwargs):
        gate_calls.append("build")
        return True, ""

    def fake_w1(*args, **kwargs):
        gate_calls.append("w1")
        assert kwargs["sample_per_op"] == 0
        return SimpleNamespace(passed=True, reason="pass")

    class FakeW2:
        def gate_ops(self, binary, ops, **kwargs):
            gate_calls.append("w2")
            assert binary == candidate_binary
            assert ops == ["op"]
            return (
                SimpleNamespace(
                    ok=True,
                    reason="accepted",
                    delta_pct=-2.5,
                    measured_cv=0.1,
                ),
                "op",
            )

        def promote_candidate(self, binary: Path, commit_sha: str) -> None:
            promotions.append((binary, commit_sha))

    monkeypatch.setattr(agent, "cargo_check", fake_cargo_check)
    monkeypatch.setattr(agent, "w1_gate", fake_w1)
    llm = FakeLLM()
    records, returned_llm, committed = agent._try_large_fn_regions(
        hot_fn,
        evidence,
        edit_target,
        [hit],
        llm=llm,
        state=state,
        w2_session=FakeW2(),
        verifier=object(),
        assets=SimpleNamespace(),
        harness_dir=harness_dir,
        crate=crate,
        cfg=AgentConfig(w2_scope="hottest_op"),
        candidate_budget=1,
    )
    assert returned_llm is llm
    assert committed is True
    assert len(records) == 1
    record = records[0]
    # Nothing is logged here on purpose. The region loop persists each record
    # as it produces it, so the audit line below must already be on disk —
    # that is the property this asserts. Logging from the caller, as this test
    # used to, would pass whether or not the loop did its part.

    assert gate_calls == ["build", "w1", "w2"]
    assert record.status is RewriteAttempt.APPLIED_COMMITTED
    assert record.terminal_status == ChangeSetStatus.COMMITTED.value
    assert record.attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "region",
            "rewrite_status": "applied_committed",
            "terminal_status": "committed",
            "error": None,
        }
    ]
    assert len(llm.calls) == 1
    _, user_prompt, meta = llm.calls[0]
    assert meta["target"] == "region"
    # Whole-function context: distant same-function body lines are now visible
    # so the LLM can establish length sources / loop bounds the region hides.
    assert "pad-001" in user_prompt
    assert "pad-260" in user_prompt
    assert source.read_text(encoding="utf-8") == before.replace(
        "acc = acc.wrapping_add(1);",
        "acc = acc.wrapping_add(2);",
        1,
    )
    commit_sha = _git(crate, "rev-parse", "HEAD")
    assert commit_sha != pristine_head
    assert _git(crate, "rev-list", "--count", "HEAD") == "2"
    assert _git(crate, "show", "--name-only", "--format=", "HEAD") == "src/lib.rs"
    assert promotions == [(candidate_binary, record.commit_sha)]

    result_paths = list((opt_dir / "changesets").glob("*/result.json"))
    assert len(result_paths) == 1
    changeset_result = json.loads(result_paths[0].read_text(encoding="utf-8"))
    operation = changeset_result["resolved"]["proposal"]["operations"][0]
    region = operation["region"]
    assert operation["evidence_hit_ids"] == region["anchor_hit_ids"]
    assert region["anchor_lines"] == [target_line]

    rows = [
        json.loads(line)
        for line in state.audit_log_path.read_text(encoding="utf-8").splitlines()
    ]
    assert len(rows) == 1
    assert rows[0]["terminal_status"] == "committed"
    assert rows[0]["attempt_trace"] == record.attempt_trace
    assert rows[0]["anchor_hit_ids"] == operation["evidence_hit_ids"]


def test_records_survive_an_abort_partway_through_the_region_loop(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """Each region's record must be on disk before the next region starts.

    The loop commits to git one region at a time. If the records only reach
    the audit file after the loop returns, killing the process mid-loop leaves
    commits with no record of what produced them. Measured on libxml2: fifteen
    agent commits in git, twelve lines in `rewrites.log`, and the three that
    vanished were one function's whole 65-minute region loop.
    """
    agent = _import_agent(monkeypatch)
    hits = [_hit("C1", 200 + i * 10) for i in range(3)]
    candidates = [
        _candidate(hit, start=800 + i * 400, digest=chr(ord("a") + i))
        for i, hit in enumerate(hits)
    ]

    class Boom(RuntimeError):
        pass

    seen = 0

    def exploding_apply(**kwargs):
        nonlocal seen
        seen += 1
        if seen == 3:
            raise Boom("killed mid-loop")
        return RewriteAttempt.ABSTAINED, {
            "terminal_status": ChangeSetStatus.REJECTED_W2_NO_GAIN.value,
            "error": f"no gain {seen}",
        }

    persisted: list = []
    with pytest.raises(Boom):
        _run(
            agent,
            monkeypatch,
            tmp_path,
            hits=hits,
            extraction_results=[
                RegionExtractionResult(tuple(candidates), ()),
                RegionExtractionResult(tuple(candidates[1:]), ()),
                RegionExtractionResult(tuple(candidates[2:]), ()),
            ],
            apply_override=exploding_apply,
            persisted_out=persisted,
        )

    assert len(persisted) == 2, (
        "the two regions that finished before the abort must be on disk"
    )
    assert [r.error for r in persisted] == ["no gain 1", "no gain 2"]


def test_rejected_region_does_not_block_later_independent_region(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    first_hit = _hit("C1", 280)
    second_hit = _hit("C2", 120)
    first = _candidate(first_hit, start=800, digest="a")
    second = _candidate(second_hit, start=300, digest="b")

    records, _, committed, extractor, planned, llm = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[first_hit, second_hit],
        extraction_results=[RegionExtractionResult((first, second), ())],
        outcomes=[
            (
                RewriteAttempt.W2_REGRESS,
                {
                    "terminal_status": ChangeSetStatus.REJECTED_W2_REGRESS.value,
                    "error": "regressed",
                },
            ),
            (
                RewriteAttempt.ABSTAINED,
                {
                    "terminal_status": ChangeSetStatus.REJECTED_W2_NO_GAIN.value,
                    "error": "no gain",
                },
            ),
        ],
    )

    assert [region.start_byte for region in planned] == [800, 300]
    assert len(extractor.calls) == 1
    assert len(llm.calls) == 2
    assert committed is False
    assert [record.terminal_status for record in records] == [
        "rejected_w2_regress",
        "rejected_w2_no_gain",
    ]
    assert all(record.fn_mode == "region" for record in records)
    assert all(record.attempt_trace[-1]["phase"] == "region" for record in records)


def test_committed_region_discards_pending_refs_and_reextracts_remaining_hits(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    accepted_hit = _hit("C1", 280)
    pending_hit = _hit("III③", 120)
    accepted = _candidate(accepted_hit, start=800, digest="a")
    stale_pending = _candidate(pending_hit, start=300, digest="b")
    refreshed_pending = _candidate(pending_hit, start=320, digest="c")
    refreshed_target = SimpleNamespace(
        file=tmp_path / "src/lib.rs", fn_name="hot", span=(0, 110)
    )
    monkeypatch.setattr(agent, "build_fn_index", lambda crate: "fresh-index")
    refresh_routes: list[str] = []

    def resolve_refreshed_target(hf, ep, rule_id, fn_index, crate):
        refresh_routes.append(rule_id)
        return refreshed_target

    monkeypatch.setattr(agent, "resolve_edit_target", resolve_refreshed_target)

    records, _, committed, extractor, planned, _ = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[accepted_hit, pending_hit],
        extraction_results=[
            RegionExtractionResult((accepted, stale_pending), ()),
            RegionExtractionResult((refreshed_pending,), ()),
        ],
        outcomes=[
            (
                RewriteAttempt.APPLIED_COMMITTED,
                {
                    "terminal_status": ChangeSetStatus.COMMITTED.value,
                    "commit_sha": "commit-1",
                },
            ),
            (
                RewriteAttempt.ABSTAINED,
                {
                    "terminal_status": ChangeSetStatus.REJECTED_W2_NO_GAIN.value,
                    "error": "no gain",
                },
            ),
        ],
    )

    assert committed is True
    assert [region.expected_region_hash for region in planned] == ["a" * 64, "c" * 64]
    assert [call[0] for call in extractor.calls] == [
        extractor.calls[0][0],
        refreshed_target,
    ]
    assert refresh_routes == ["C1"]
    assert [hit["rule"] for hit in extractor.calls[1][1]] == ["III③"]
    assert len(records) == 2


@pytest.mark.parametrize(
    ("hits", "extraction", "expected_reason"),
    [
        ([_hit("unknown", 10)], RegionExtractionResult((), ()), "no_region_local_rules"),
        (
            [_hit("C1", 10)],
            RegionExtractionResult(
                (),
                (
                    SkippedRegionHit(
                        "1" * 64, "C1", RegionSkipReason.REGION_UNPROVEN
                    ),
                ),
            ),
            "all_regions_unproven",
        ),
        (
            [_hit("C1", 10)],
            RegionExtractionResult(
                (),
                (
                    SkippedRegionHit(
                        "1" * 64,
                        "C1",
                        RegionSkipReason.REGION_OVER_LIMIT_UNPROVEN,
                    ),
                ),
            ),
            "all_regions_over_budget",
        ),
        ([_hit("III①", 10)], RegionExtractionResult((), ()), "only_cross_fn_rules"),
    ],
)
def test_no_region_attempt_has_precise_final_reason(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    hits: list[dict],
    extraction: RegionExtractionResult,
    expected_reason: str,
) -> None:
    agent = _import_agent(monkeypatch)

    records, _, _, extractor, planned, llm = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=hits,
        extraction_results=[extraction],
    )

    assert len(records) == 1
    assert records[0].anchor_hit_ids is None
    assert records[0].reason == expected_reason
    assert records[0].attempt_trace[-1]["phase"] == "region"
    assert "pre_abstain_large_fn" not in records[0].rule_id
    assert planned == []
    assert llm.calls == []
    if expected_reason == "only_cross_fn_rules":
        assert records[0].terminal_status == "cross_fn_requires_changeset_v2"
        assert extractor.calls == []


def test_mixed_cross_function_hits_are_reported_but_never_extracted(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    local_hit = _hit("C1", 200)
    cross_hit = _hit("III①", 210)
    candidate = _candidate(local_hit, start=500, digest="a")

    records, _, _, extractor, _, _ = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[local_hit, cross_hit],
        extraction_results=[RegionExtractionResult((candidate,), ())],
        candidate_budget=1,
    )

    assert [hit["rule"] for hit in extractor.calls[0][1]] == ["C1"]
    assert len(records) == 1
    assert records[0].terminal_status == "rejected_w2_no_gain"
    assert records[0].fired_rules == ["C1", "III①"]
    assert records[0].skipped_rules == [
        ["III①", "cross_fn_requires_changeset_v2"]
    ]
    assert records[0].fn_mode == "region"


def test_mixed_cross_audit_survives_token_budget_terminal(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    local_hit = _hit("C1", 200)
    cross_hit = _hit("III①", 210)
    candidate = _candidate(local_hit, start=500, digest="a")

    records, _, _, extractor, planned, llm = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[local_hit, cross_hit],
        extraction_results=[RegionExtractionResult((candidate,), ())],
        cfg=AgentConfig(max_llm_tokens_per_fn=0),
        candidate_budget=1,
    )

    assert [hit["rule"] for hit in extractor.calls[0][1]] == ["C1"]
    assert planned == []
    assert llm.calls == []
    assert len(records) == 1
    assert records[0].terminal_status == "budget_exceeded"
    assert records[0].fired_rules == ["C1", "III①"]
    assert records[0].skipped_rules == [
        ["III①", "cross_fn_requires_changeset_v2"]
    ]


def test_cross_plus_unsupported_without_local_keeps_cross_audit(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    cross_hit = _hit("III①", 20)
    unsupported_hit = _hit("unknown", 30)

    records, _, _, extractor, planned, llm = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[cross_hit, unsupported_hit],
        extraction_results=[RegionExtractionResult((), ())],
    )

    assert extractor.calls == []
    assert planned == []
    assert llm.calls == []
    assert len(records) == 1
    assert records[0].reason == "no_region_local_rules"
    assert records[0].terminal_status == "no_region_local_rules"
    assert records[0].skipped_rules == [
        ["III①", "cross_fn_requires_changeset_v2"]
    ]


def test_duplicate_skip_does_not_hide_canonical_all_over_budget_reason(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    hit = _hit("C1", 20)
    extraction = RegionExtractionResult(
        (),
        (
            SkippedRegionHit(
                "1" * 64, "C1", RegionSkipReason.DUPLICATE_HIT
            ),
            SkippedRegionHit(
                "2" * 64,
                "C1",
                RegionSkipReason.REGION_OVER_LIMIT_UNPROVEN,
            ),
        ),
    )

    records, *_ = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[hit],
        extraction_results=[extraction],
    )

    assert records[0].reason == "all_regions_over_budget"
    assert records[0].terminal_status == "all_regions_over_budget"


def test_region_candidate_budget_runs_highest_priority_and_token_budget_is_structured(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    first_hit = _hit("C1", 280)
    second_hit = _hit("C2", 120)
    first = _candidate(first_hit, start=800, digest="a")
    second = _candidate(second_hit, start=300, digest="b")

    records, _, _, _, planned, budget_llm = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[first_hit, second_hit],
        extraction_results=[RegionExtractionResult((first, second), ())],
        candidate_budget=1,
    )
    assert len(records) == 1
    assert [region.start_byte for region in planned] == [800]
    assert len(budget_llm.calls) == 1
    assert records[-1].terminal_status == "rejected_w2_no_gain"
    assert records[-1].status is RewriteAttempt.ABSTAINED

    token_records, _, _, _, _, token_llm = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[first_hit],
        extraction_results=[RegionExtractionResult((first,), ())],
        cfg=AgentConfig(max_llm_tokens_per_fn=0),
    )
    expected_anchor_hit_ids = list(first.region.anchor_hit_ids)
    assert token_records[-1].terminal_status == "budget_exceeded"
    assert token_records[-1].anchor_hit_ids == expected_anchor_hit_ids
    assert token_records[-1].attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "region",
            "rewrite_status": "budget_exceeded",
            "terminal_status": "budget_exceeded",
            "error": token_records[-1].error,
        }
    ]
    assert token_llm.calls == []
    budget_audit = tmp_path / "budget-rewrites.log"
    StateManager(tmp_path, budget_audit).log(token_records[-1])
    payload = json.loads(budget_audit.read_text(encoding="utf-8"))
    assert payload["anchor_hit_ids"] == expected_anchor_hit_ids


def test_region_planner_abstain_is_an_exact_region_trace(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    hit = _hit("C1", 280)
    candidate = _candidate(hit, start=800, digest="a")
    records, *_ = _run(
        agent,
        monkeypatch,
        tmp_path,
        hits=[hit],
        extraction_results=[RegionExtractionResult((candidate,), ())],
        planner_abstain_reason="unsafe aliasing",
        llm_response="abstain: unsafe aliasing",
    )

    assert records[0].status is RewriteAttempt.ABSTAINED
    assert records[0].terminal_status == "planner_abstained"
    assert records[0].reason == "unsafe aliasing"
    assert records[0].skipped_rules == [["C1", "unsafe aliasing"]]
    assert records[0].attempt_trace == [
        {
            "attempt_no": 1,
            "phase": "region",
            "rewrite_status": "abstained",
            "terminal_status": "planner_abstained",
            "error": None,
        }
    ]


def test_each_region_executor_runs_build_full_w1_and_w2(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    calls = {"build": 0, "w1": 0, "w2": 0}

    class FakeApplier:
        def recover_incomplete(self) -> None:
            pass

    class FakeExecutor:
        def __init__(self, **kwargs) -> None:
            self.kwargs = kwargs

        def execute(self, proposal):
            assert self.kwargs["build_gate"]().ok
            assert self.kwargs["w1_gate"]().ok
            assert self.kwargs["w2_gate"](None).ok
            return AppliedChangeSet(
                resolved=None,
                terminal_status=ChangeSetStatus.COMMITTED,
                commit_sha="commit-sha",
            )

    monkeypatch.setattr(agent, "ChangeSetApplier", lambda *args: FakeApplier())
    monkeypatch.setattr(agent, "ChangeSetExecutor", FakeExecutor)
    monkeypatch.setattr(agent, "AuditWriter", lambda *args: object())

    def fake_cargo(*args, **kwargs):
        calls["build"] += 1
        return True, ""

    def fake_w1(*args, **kwargs):
        calls["w1"] += 1
        return SimpleNamespace(passed=True, reason="pass")

    class W2:
        promote_candidate = object()

        def gate_ops(self, *args, **kwargs):
            calls["w2"] += 1
            return (
                SimpleNamespace(
                    ok=True,
                    reason="accepted",
                    delta_pct=-1.0,
                    measured_cv=0.1,
                ),
                "op",
            )

    monkeypatch.setattr(agent, "cargo_check", fake_cargo)
    monkeypatch.setattr(agent, "w1_gate", fake_w1)
    monkeypatch.setattr(agent, "_w2_ops_for_fn", lambda *args: ["op"])
    state = SimpleNamespace(
        crate=tmp_path,
        audit_log_path=tmp_path / "rewrites.log",
    )
    hf = SimpleNamespace(name="hot", hottest_op="op", per_op={"op": 1.0})

    for _ in range(2):
        status, extra = agent._apply_region_and_gate(
            proposal=object(),
            hf=hf,
            harness_dir=tmp_path / "harness",
            verifier=object(),
            assets=object(),
            w2_session=W2(),
            state=state,
            cfg=AgentConfig(),
        )
        assert status is RewriteAttempt.APPLIED_COMMITTED
        assert extra["terminal_status"] == "committed"

    assert calls == {"build": 2, "w1": 2, "w2": 2}


def _patch_optimize_shell(
    agent,
    monkeypatch: pytest.MonkeyPatch,
    tmp_path: Path,
    hits: list[dict],
):
    logged: list = []
    state = SimpleNamespace(log=logged.append)
    monkeypatch.setattr(agent, "StateManager", lambda **kwargs: state)
    monkeypatch.setattr(agent, "_sanity_check", lambda *args: None)
    monkeypatch.setattr(
        agent, "_prepare_pre_bin", lambda *args: tmp_path / "pre-harness"
    )
    monkeypatch.setattr(agent, "build_fn_index", lambda crate: {})
    monkeypatch.setattr(agent, "Verifier", lambda **kwargs: object())
    monkeypatch.setattr(agent, "_run_pristine_w1_preflight", lambda *args: None)
    monkeypatch.setattr(agent, "W2Session", lambda **kwargs: object())
    monkeypatch.setattr(
        agent,
        "CallerLookup",
        lambda crate: SimpleNamespace(invalidate=lambda: None),
    )
    monkeypatch.setattr(agent, "_load_fn_hits", lambda opt_dir: {"hot": {"hits": hits}})
    monkeypatch.setattr(agent, "_load_evidence", lambda path: object())
    monkeypatch.setattr(agent, "_is_orchestration_fn", lambda *args: (False, ""))
    monkeypatch.setattr(
        agent,
        "resolve_edit_target",
        lambda *args: SimpleNamespace(
            file=tmp_path / "crate/src/lib.rs", fn_name="hot", span=(0, 100)
        ),
    )
    monkeypatch.setattr(agent, "_finalize_w2", lambda *args: None)
    monkeypatch.setattr(agent, "_all_perf_ops", lambda *args: [])
    cfg = AgentConfig(max_candidates=10)
    hot_fn = SimpleNamespace(
        name="hot",
        file=Path("src/lib.rs"),
        extern_wrapper=False,
        self_pct=50.0,
        line_start=1,
        line_end=400,
        hottest_op="op",
        per_op={"op": 1.0},
    )
    return logged, cfg, hot_fn


def _abstain_record(agent, *, mode: str, rule: str = "C1"):
    return agent.AttemptRecord(
        fn_name="hot",
        rule_id=rule,
        round_no=1,
        attempt_no=1,
        status=RewriteAttempt.ABSTAINED,
        reason="test",
        fired_rules=[rule],
        fn_mode=mode,
        **agent.reporting.with_attempt_trace(
            {},
            [
                agent.reporting.make_trace_entry(
                    1, mode if mode != "simple" else "direct",
                    RewriteAttempt.ABSTAINED, "planner_abstained"
                )
            ],
        ),
    )


def test_optimize_large_function_calls_region_orchestration_without_old_abstain(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    hits = [_hit("C1", 200)]
    logged, cfg, hot_fn = _patch_optimize_shell(
        agent, monkeypatch, tmp_path, hits
    )
    cfg.max_candidates = 1
    region_record = _abstain_record(agent, mode="region")
    region_calls: list[list[dict]] = []
    monkeypatch.setattr(agent, "_is_large_fn", lambda *args: (True, "large"))
    def _fake_regions(*args, **kwargs):
        region_calls.append(list(args[3]))
        # The real loop persists each record itself; the caller no longer
        # does. A stand-in that skips the log would make this test assert
        # that records reach the audit file when nothing puts them there.
        kwargs["state"].log(region_record)
        return [region_record], kwargs["llm"], False

    monkeypatch.setattr(
        agent, "_try_large_fn_regions", _fake_regions, raising=False,
    )
    monkeypatch.setattr(
        agent,
        "_try_fn_direct",
        lambda *args, **kwargs: (_ for _ in ()).throw(
            AssertionError("large function entered the direct whole-function path")
        ),
    )

    result = agent.optimize_hot_fns(
        [hot_fn],
        evidence_dir=tmp_path / "evidence",
        harness_dir=tmp_path / "harness",
        crate=tmp_path / "crate",
        assets=object(),
        cfg=cfg,
        opt_dir=tmp_path / "opt",
    )

    assert region_calls == [hits]
    assert logged == [region_record]
    assert result.per_fn_summary["hot"] == [region_record]
    assert result.total_attempts == 1
    assert result.early_stop_reason == "candidate budget exhausted (1)"
    assert all(record.rule_id != "pre_abstain_large_fn" for record in logged)


def test_optimize_large_function_with_unresolved_target_records_region_unproven(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    hits = [_hit("C1", 200)]
    logged, cfg, hot_fn = _patch_optimize_shell(
        agent, monkeypatch, tmp_path, hits
    )
    monkeypatch.setattr(agent, "_is_large_fn", lambda *args: (True, "large"))
    monkeypatch.setattr(agent, "resolve_edit_target", lambda *args: None)
    monkeypatch.setattr(
        agent,
        "_init_llm",
        lambda crate: (_ for _ in ()).throw(
            AssertionError("unproven large target must not initialize the LLM")
        ),
    )
    result = agent.optimize_hot_fns(
        [hot_fn],
        evidence_dir=tmp_path / "evidence",
        harness_dir=tmp_path / "harness",
        crate=tmp_path / "crate",
        assets=object(),
        cfg=cfg,
        opt_dir=tmp_path / "opt",
    )

    assert len(logged) == 1
    record = logged[0]
    assert record.fn_mode == "region"
    assert record.reason == "all_regions_unproven"
    assert record.terminal_status == "all_regions_unproven"
    assert record.attempt_trace[-1]["phase"] == "region"
    assert result.per_fn_summary == {"hot": [record]}


def test_optimize_non_large_function_keeps_existing_direct_path(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    hits = [_hit("C1", 20)]
    logged, cfg, hot_fn = _patch_optimize_shell(
        agent, monkeypatch, tmp_path, hits
    )
    direct_record = _abstain_record(agent, mode="simple")
    monkeypatch.setattr(agent, "_is_large_fn", lambda *args: (False, ""))
    monkeypatch.setattr(agent, "_is_short_fn_no_leverage", lambda *args: (False, ""))
    monkeypatch.setattr(agent, "is_complex_fn", lambda *args: False)
    monkeypatch.setattr(agent, "_init_llm", lambda crate: object())
    monkeypatch.setattr(
        agent, "_try_fn_direct", lambda *args, **kwargs: direct_record
    )
    monkeypatch.setattr(
        agent,
        "_try_large_fn_regions",
        lambda *args, **kwargs: (_ for _ in ()).throw(
            AssertionError("non-large function entered region orchestration")
        ),
        raising=False,
    )

    agent.optimize_hot_fns(
        [hot_fn],
        evidence_dir=tmp_path / "evidence",
        harness_dir=tmp_path / "harness",
        crate=tmp_path / "crate",
        assets=object(),
        cfg=cfg,
        opt_dir=tmp_path / "opt",
    )

    assert logged == [direct_record]


def test_optimize_non_large_missing_target_keeps_existing_silent_skip(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    agent = _import_agent(monkeypatch)
    hits = [_hit("C1", 20)]
    logged, cfg, hot_fn = _patch_optimize_shell(
        agent, monkeypatch, tmp_path, hits
    )
    monkeypatch.setattr(agent, "_is_large_fn", lambda *args: (False, ""))
    monkeypatch.setattr(agent, "resolve_edit_target", lambda *args: None)
    monkeypatch.setattr(
        agent,
        "_try_large_fn_regions",
        lambda *args, **kwargs: (_ for _ in ()).throw(
            AssertionError("non-large missing target entered Region mode")
        ),
    )
    monkeypatch.setattr(
        agent,
        "_init_llm",
        lambda crate: (_ for _ in ()).throw(
            AssertionError("non-large missing target initialized LLM")
        ),
    )

    result = agent.optimize_hot_fns(
        [hot_fn],
        evidence_dir=tmp_path / "evidence",
        harness_dir=tmp_path / "harness",
        crate=tmp_path / "crate",
        assets=object(),
        cfg=cfg,
        opt_dir=tmp_path / "opt",
    )

    assert logged == []
    assert result.total_attempts == 0
