from __future__ import annotations

from dataclasses import FrozenInstanceError, replace
import hashlib
from pathlib import Path
import re

import pytest
from tree_sitter import Language, Parser
import tree_sitter_rust

from perf_opt.agent_perf_opt.changeset import (
    ConcreteEdit,
    ImpactScope,
    RegionKind,
    RegionRef,
    ReplaceSourceRegion,
    SymbolKind,
    SymbolRef,
)
from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.handlers import (
    validate_region_replacement_source,
)
from perf_opt.agent_perf_opt.changeset.resolver import ResolutionRejected
from perf_opt.agent_perf_opt.planners import plan_llm_region_change
from perf_opt.agent_perf_opt.prompt_builder import build_region_prompt, load_card
from perf_opt.agent_perf_opt.regions import RegionExtractor
from perf_opt.agent_perf_opt.regions import RegionSkipReason
from perf_opt.agent_perf_opt.rewrite_applier import EditTarget


_PARSER = Parser(Language(tree_sitter_rust.language()))


def _walk(node):
    stack = [node]
    while stack:
        current = stack.pop()
        yield current
        stack.extend(reversed(current.children))


def _line_col(source: str, needle: str) -> tuple[int, int]:
    for line_number, line in enumerate(source.splitlines(), 1):
        if needle in line:
            return line_number, len(line[: line.index(needle)].encode("utf-8")) + 1
    raise AssertionError(f"missing source needle {needle!r}")


def _extracted_region(
    tmp_path: Path,
) -> tuple[Path, EditTarget, RegionRef, dict[str, object], dict[str, object], str]:
    before = ["    let remote_before_sentinel = 0;"] + [
        f"    let before_{index} = {index};" for index in range(1, 18)
    ]
    after = [f"    let after_{index} = {index};" for index in range(1, 18)] + [
        "    let remote_after_sentinel = 0;"
    ]
    source = "\n".join(
        [
            "// remote_header_comment_sentinel",
            "#[cold]",
            "/* between_attrs_comment_sentinel */",
            "#[inline(never)]",
            "pub(crate) fn target(input: i32) -> i32 {",
            *before,
            "    let hot_value = expensive(input);",
            *after,
            "    hot_value",
            "}",
            "",
        ]
    )
    crate = tmp_path / "crate"
    path = crate / "src" / "lib.rs"
    path.parent.mkdir(parents=True)
    data = source.encode("utf-8")
    path.write_bytes(data)
    functions = [node for node in _walk(_PARSER.parse(data).root_node) if node.type == "function_item"]
    assert len(functions) == 1
    function = functions[0]
    target = EditTarget(path, "target", (0, function.end_byte))
    line, col = _line_col(source, "expensive")
    anchor_hit = {
        "file": "src/lib.rs",
        "function": "target",
        "line": line,
        "col": col,
        "rule": "C1",
        "pattern": "expensive call",
        "snippet": "let hot_value = expensive(input);",
        "extra": {"note": "anchor-detail"},
    }
    unrelated_hit = {
        "file": "src/lib.rs",
        "function": "target",
        "line": 3,
        "col": 5,
        "rule": "C1",
        "pattern": "unrelated-pattern",
        "snippet": "remote_before_sentinel",
    }
    # window_pad_lines=0 keeps the editable region on the single anchor
    # statement (`let hot_value = expensive(input);`) so the surrounding body
    # stays as read-only whole-function context; with the default 30-line pad
    # the region would swallow the whole body (including the tail expression,
    # which is not a statement) and no longer resolve as a clean region.
    extracted = RegionExtractor(window_pad_lines=0).extract(
        crate=crate,
        edit_target=target,
        hits=[anchor_hit],
    )
    assert extracted.skipped == ()
    assert len(extracted.candidates) == 1
    return crate, target, extracted.candidates[0].region, anchor_hit, unrelated_hit, source


def _context_block(prompt: str, heading: str) -> list[str]:
    block = prompt.split(heading, 1)[1].split("```rust\n", 1)[1].split("\n```", 1)[0]
    return [] if not block else block.splitlines()


def test_region_prompt_is_bounded_auditable_and_does_not_leak_remote_body(
    tmp_path: Path,
) -> None:
    crate, target, region, anchor_hit, unrelated_hit, source = _extracted_region(tmp_path)
    original_region = target.file.read_bytes()[region.start_byte : region.end_byte].decode()

    system, user = build_region_prompt(
        crate=crate,
        edit_target=target,
        region=region,
        rule_ids=("C1",),
        hits=(anchor_hit, unrelated_hit),
    )

    assert "#[inline(never)]" in user
    assert "#[cold]" in user
    assert "pub(crate) fn target(input: i32) -> i32" in user
    assert original_region in user
    assert region.anchor_hit_ids[0] in user
    assert "anchor-detail" in user
    assert "unrelated-pattern" not in user
    assert load_card("C1").strip() in user
    assert "C1" in user
    assert "remote_header_comment_sentinel" not in user
    assert "between_attrs_comment_sentinel" not in user
    assert source.count("let hot_value = expensive(input);") == 1
    before = _context_block(user, "Read-only function body BEFORE the editable region")
    after = _context_block(user, "Read-only function body AFTER the editable region")
    # Whole-function context: the FULL body (all before_*/after_* lines and the
    # same-function distant sentinels) must now be visible so the LLM can find a
    # length source / loop bound the region alone hides. (>= 18 rather than an
    # exact count: leading/trailing whitespace-only body lines are noise.)
    assert len(before) >= 18   # remote_before_sentinel + before_1..before_17
    assert len(after) >= 18    # after_1..after_17 + remote_after_sentinel
    assert any("remote_before_sentinel" in line for line in before)
    assert any("remote_after_sentinel" in line for line in after)
    assert any("before_17" in line for line in before)
    assert any("after_1 = 1" in line for line in after)
    # The editable region line itself is shown once, in the region block only.
    assert all("hot_value = expensive" not in line for line in before + after)
    # Module-level / above-signature content must STILL never leak.
    assert "remote_header_comment_sentinel" not in user
    assert "between_attrs_comment_sentinel" not in user
    assert "read-only" in system.lower()
    assert "replacement region" in system.lower()
    assert "full function" in system.lower()
    assert "Applied rules" in system
    assert "Skipped rules" in system


def test_region_prompt_caps_context_and_rejects_non_region_rules(tmp_path: Path) -> None:
    crate, target, region, anchor_hit, _, _ = _extracted_region(tmp_path)

    _, no_context = build_region_prompt(
        crate=crate,
        edit_target=target,
        region=region,
        rule_ids=("C1",),
        hits=(anchor_hit,),
        context_lines=0,
    )
    assert _context_block(
        no_context, "Read-only function body BEFORE the editable region"
    ) == []
    assert _context_block(
        no_context, "Read-only function body AFTER the editable region"
    ) == []
    for bad in (-1, True, "12"):
        with pytest.raises(ValueError, match="context_lines"):
            build_region_prompt(
                crate=crate,
                edit_target=target,
                region=region,
                rule_ids=("C1",),
                hits=(anchor_hit,),
                context_lines=bad,
            )
    for rejected_rule in ("II_inl", "not-a-rule"):
        with pytest.raises(RuntimeError, match="region-local"):
            build_region_prompt(
                crate=crate,
                edit_target=target,
                region=region,
                rule_ids=(rejected_rule,),
                hits=(anchor_hit,),
            )
    with pytest.raises(ValueError, match="candidate rules.*anchor hit rules"):
        build_region_prompt(
            crate=crate,
            edit_target=target,
            region=region,
            rule_ids=("C2",),
            hits=(anchor_hit,),
        )
    forged_hit = dict(anchor_hit)
    forged_hit.update(
        anchor_hit_id=region.anchor_hit_ids[0],
        line=anchor_hit["line"] + 1,
        pattern="forged-detail",
    )
    with pytest.raises(ValueError, match="missing anchor hit details"):
        build_region_prompt(
            crate=crate,
            edit_target=target,
            region=region,
            rule_ids=("C1",),
            hits=(forged_hit,),
        )


@pytest.mark.parametrize(
    "mutate",
    [
        lambda region: replace(region, expected_region_hash="0" * 64),
        lambda region: replace(
            region,
            target_function=replace(
                region.target_function,
                declaration_hash="0" * 64,
            ),
        ),
        lambda region: replace(
            region,
            target_function=replace(
                region.target_function,
                file_hint="src/not-the-target.rs",
            ),
        ),
        lambda region: replace(region, parent_kind="source_file"),
        lambda region: replace(
            region,
            anchor_lines=tuple(line + 10_000 for line in region.anchor_lines),
        ),
    ],
    ids=[
        "region-hash",
        "declaration-hash",
        "file-hint",
        "parent-kind",
        "anchor-lines",
    ],
)
def test_region_prompt_rejects_unproven_or_stale_region_ref(
    tmp_path: Path, mutate
) -> None:
    crate, target, region, anchor_hit, _, _ = _extracted_region(tmp_path)

    with pytest.raises(ResolutionRejected):
        build_region_prompt(
            crate=crate,
            edit_target=target,
            region=mutate(region),
            rule_ids=("C1",),
            hits=(anchor_hit,),
        )


def test_region_prompt_uses_first_duplicate_raw_hit_like_extractor(
    tmp_path: Path,
) -> None:
    crate, target, _, anchor_hit, _, _ = _extracted_region(tmp_path)
    duplicate = dict(anchor_hit)
    extracted = RegionExtractor(window_pad_lines=0).extract(
        crate=crate,
        edit_target=target,
        hits=(anchor_hit, duplicate),
    )
    assert len(extracted.candidates) == 1
    assert [skip.reason for skip in extracted.skipped] == [
        RegionSkipReason.DUPLICATE_HIT
    ]

    _, user = build_region_prompt(
        crate=crate,
        edit_target=target,
        region=extracted.candidates[0].region,
        rule_ids=extracted.candidates[0].rule_ids,
        hits=(anchor_hit, duplicate),
    )

    assert extracted.candidates[0].region.anchor_hit_ids[0] in user


def test_region_prompt_sanitizes_hit_detail_and_rejects_wrong_function(
    tmp_path: Path,
) -> None:
    crate, target, region, anchor_hit, _, _ = _extracted_region(tmp_path)
    display_hit = dict(anchor_hit)
    display_hit.update(
        file=str(target.file.resolve()),
        id="FORGED_ID_SENTINEL",
        anchor_hit_id="FORGED_ANCHOR_SENTINEL",
    )

    _, user = build_region_prompt(
        crate=crate,
        edit_target=target,
        region=region,
        rule_ids=("C1",),
        hits=(display_hit,),
    )

    assert str(target.file.resolve()) not in user
    assert "FORGED_ID_SENTINEL" not in user
    assert "FORGED_ANCHOR_SENTINEL" not in user
    assert "anchor-detail" in user

    wrong_function = dict(anchor_hit, function="not_target")
    with pytest.raises(ValueError, match="missing anchor hit details"):
        build_region_prompt(
            crate=crate,
            edit_target=target,
            region=region,
            rule_ids=("C1",),
            hits=(wrong_function,),
        )


@pytest.mark.parametrize("needle", ["first_work", "last_work"])
def test_region_prompt_context_never_crosses_target_function_body(
    tmp_path: Path, needle: str
) -> None:
    source = """fn remote_before() {
    let SECRET_BEFORE = 1;
}

#[inline]
fn target() {
    first_work();
    middle_work();
    last_work();
}

fn remote_after() {
    let SECRET_AFTER = 2;
}
"""
    crate = tmp_path / "crate"
    path = crate / "src" / "lib.rs"
    path.parent.mkdir(parents=True)
    data = source.encode("utf-8")
    path.write_bytes(data)
    functions = [
        node
        for node in _walk(_PARSER.parse(data).root_node)
        if node.type == "function_item"
        and data[
            node.child_by_field_name("name").start_byte : node.child_by_field_name(
                "name"
            ).end_byte
        ]
        == b"target"
    ]
    assert len(functions) == 1
    target = EditTarget(
        path,
        "target",
        (source.index("#[inline]"), functions[0].end_byte),
    )
    line, col = _line_col(source, needle)
    hit = {
        "file": "src/lib.rs",
        "function": "target",
        "line": line,
        "col": col,
        "rule": "C1",
        "pattern": needle,
        "snippet": f"{needle}();",
    }
    extracted = RegionExtractor().extract(
        crate=crate,
        edit_target=target,
        hits=(hit,),
    )
    assert len(extracted.candidates) == 1

    _, user = build_region_prompt(
        crate=crate,
        edit_target=target,
        region=extracted.candidates[0].region,
        rule_ids=("C1",),
        hits=(hit,),
    )

    assert "SECRET_BEFORE" not in user
    assert "SECRET_AFTER" not in user


def _region(kind: RegionKind = RegionKind.STATEMENT, *, identity: str = "a") -> RegionRef:
    digest = hashlib.sha256(identity.encode()).hexdigest()
    return RegionRef(
        target_function=SymbolRef(
            qualified_name="crate::target",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint="src/lib.rs",
            declaration_hash=hashlib.sha256(f"fn-{identity}".encode()).hexdigest(),
        ),
        relative_path="src/lib.rs",
        region_kind=kind,
        parent_kind="block",
        start_byte=10,
        end_byte=20,
        expected_region_hash=digest,
        anchor_hit_ids=(hashlib.sha256(f"hit-{identity}".encode()).hexdigest(),),
        anchor_lines=(2,),
    )


def _response(
    replacement: str,
    *,
    applied: str = "C1",
    skipped: str = "C2: no leverage",
) -> str:
    return (
        "```rust\n"
        f"// Applied rules: [{applied}]\n"
        f"// Skipped rules: [{skipped}]\n"
        f"{replacement}\n"
        "```"
    )


def _plan(
    response: object,
    *,
    region: object | None = None,
    rules: tuple[str, ...] = ("C1", "C2"),
    candidate_id: object = "candidate-a",
):
    return plan_llm_region_change(
        response=response,
        region=_region() if region is None else region,
        rule_ids=rules,
        base_head="head-a",
        candidate_id=candidate_id,
    )


@pytest.mark.parametrize(
    ("kind", "replacement"),
    [
        (RegionKind.STATEMENT, "let value = 2;"),
        (RegionKind.EXPRESSION, "input + 1"),
        (RegionKind.LOOP, "while ready() { work(); }"),
        (RegionKind.MATCH_ARM, "0 => input + 1,"),
        (RegionKind.BLOCK, "{ work(); }"),
        (RegionKind.NODE_SEQUENCE, "work();\nfinish();"),
    ],
)
def test_valid_region_response_produces_one_local_operation_without_metadata(
    kind: RegionKind, replacement: str
) -> None:
    region = _region(kind)
    planned = _plan(_response(replacement), region=region)

    assert planned.abstain_reason is None
    assert planned.proposal is not None
    proposal = planned.proposal
    assert proposal.impact_scope is ImpactScope.LOCAL_FUNCTION
    assert proposal.trigger.rule_id == "C1"
    assert proposal.trigger.hit_ids == region.anchor_hit_ids
    assert len(proposal.operations) == 1
    operation = proposal.operations[0]
    assert isinstance(operation, ReplaceSourceRegion)
    assert operation.region is region
    assert operation.rule_id == "C1"
    assert operation.evidence_hit_ids == region.anchor_hit_ids
    assert operation.replacement_region_source == replacement
    assert "Applied rules" not in operation.replacement_region_source
    assert "Skipped rules" not in operation.replacement_region_source


@pytest.mark.parametrize(
    ("response", "reason"),
    [
        ("abstain: unsafe proof missing", "unsafe proof missing"),
        ("", "empty_response"),
        ("plain source", "parse_fail"),
        ("```rust\nlet value = 2;\n```", "metadata"),
        (_response("let value = 2;", applied="C1, unknown"), "unknown"),
        (_response("let value = 2;", applied="C1, C1"), "duplicate"),
        (_response("let value = 2;", applied="C1", skipped="C1: duplicate; C2: no"), "both"),
        (_response("let value = 2;", applied="C1", skipped=""), "coverage"),
        (_response("let value = 2;", applied="C1", skipped="C2"), "reason"),
        (_response("let value = 2;", applied="", skipped="C1: no; C2: no"), "no_applied"),
    ],
)
def test_planner_abstains_for_parse_and_rule_metadata_contract_failures(
    response: str, reason: str
) -> None:
    planned = _plan(response)

    assert planned.proposal is None
    assert planned.abstain_reason is not None
    assert reason in planned.abstain_reason


@pytest.mark.parametrize(
    "forged_header",
    [
        "// Applied rules: [FORGED]",
        "// Skipped rules: [C2: forged]",
    ],
)
def test_planner_rejects_metadata_headers_inside_replacement(
    forged_header: str,
) -> None:
    replacement = f"if true {{\n    {forged_header}\n    work();\n}}"

    planned = _plan(_response(replacement))

    assert planned.proposal is None
    assert planned.abstain_reason == "replacement_metadata_header"


@pytest.mark.parametrize(
    "response",
    [
        "explanation before fence\n" + _response("let value = 2;"),
        _response("let value = 2;") + "\n" + _response("let value = 3;"),
    ],
)
def test_planner_requires_exactly_one_bare_rust_fence(response: str) -> None:
    planned = _plan(response)

    assert planned.proposal is None
    assert planned.abstain_reason == "response_fence_contract"


@pytest.mark.parametrize(
    ("kind", "replacement"),
    [
        (RegionKind.EXPRESSION, ""),
        (RegionKind.EXPRESSION, "fn target() {}"),
        (RegionKind.STATEMENT, "fn replacement() {}"),
        (RegionKind.STATEMENT, "fn replacement()"),
        (RegionKind.STATEMENT, "impl Thing { fn extra() {} }"),
        (RegionKind.STATEMENT, "mod extra {}"),
        (RegionKind.EXPRESSION, "let value = 2;"),
        (RegionKind.MATCH_ARM, "let value = 2;"),
    ],
)
def test_planner_rejects_empty_forbidden_item_and_wrong_wrapper_shapes(
    kind: RegionKind, replacement: str
) -> None:
    planned = _plan(_response(replacement, applied="C1", skipped="C2: no"), region=_region(kind))

    assert planned.proposal is None
    assert planned.abstain_reason is not None


@pytest.mark.parametrize("kind", [RegionKind.STATEMENT, RegionKind.NODE_SEQUENCE])
def test_planner_accepts_empty_statement_or_sequence_replacement(kind: RegionKind) -> None:
    planned = _plan(_response("", applied="C1", skipped="C2: no"), region=_region(kind))

    assert planned.proposal is not None
    operation = planned.proposal.operations[0]
    assert operation.replacement_region_source == ""


def test_planner_ids_are_stable_and_change_with_replacement_or_region_identity() -> None:
    response = _response("let value = 2;")
    first = _plan(response)
    repeated = _plan(response)
    changed_source = _plan(_response("let value = 3;"))
    changed_region = _plan(response, region=_region(identity="b"))
    assert first.proposal is not None
    assert repeated.proposal is not None
    assert changed_source.proposal is not None
    assert changed_region.proposal is not None

    def ids(planned):
        return planned.proposal.operations[0].operation_id, planned.proposal.changeset_id

    assert ids(first) == ids(repeated)
    assert ids(first)[0] != ids(changed_source)[0]
    assert ids(first)[1] != ids(changed_source)[1]
    assert ids(first)[0] != ids(changed_region)[0]
    assert ids(first)[1] != ids(changed_region)[1]


def test_applied_rules_are_canonicalized_to_candidate_order_for_ids() -> None:
    canonical = _plan(_response("let value = 2;", applied="C1, C2", skipped=""))
    reversed_rules = _plan(
        _response("let value = 2;", applied="C2, C1", skipped="")
    )
    assert canonical.proposal is not None
    assert reversed_rules.proposal is not None

    canonical_operation = canonical.proposal.operations[0]
    reversed_operation = reversed_rules.proposal.operations[0]
    assert reversed_operation.rule_id == "C1,C2"
    assert reversed_operation.operation_id == canonical_operation.operation_id
    assert reversed_rules.proposal.changeset_id == canonical.proposal.changeset_id


def test_region_plan_ids_are_short_sha256_prefixes_and_safe_for_applier(
    tmp_path: Path,
) -> None:
    planned = _plan(_response("let value = 2;"))
    assert planned.proposal is not None
    proposal = planned.proposal
    operation = proposal.operations[0]
    assert re.fullmatch(r"replace-region-[0-9a-f]{16}", operation.operation_id)
    assert re.fullmatch(r"region-[0-9a-f]{16}", proposal.changeset_id)

    crate = tmp_path / "crate"
    crate.mkdir()
    relative_path = "x" * 210
    source = crate / relative_path
    source.write_bytes(b"old")
    edit = ConcreteEdit(
        edit_id=f"edit-{operation.operation_id}",
        operation_id=operation.operation_id,
        relative_path=relative_path,
        start_byte=0,
        end_byte=3,
        before_hash=hashlib.sha256(b"old").hexdigest(),
        replacement_text="new",
    )
    applier = ChangeSetApplier(crate, tmp_path / "journal")

    applied = applier.apply(proposal.changeset_id, (edit,))

    assert source.read_bytes() == b"new"
    applier.restore(applied)
    assert source.read_bytes() == b"old"
    assert not applied.journal_dir.exists()


@pytest.mark.parametrize(
    "kwargs",
    [
        {"response": object()},
        {"response": _response("let value = 2;"), "region": object()},
        {"response": _response("let value = 2;"), "candidate_id": object()},
    ],
)
def test_planner_fail_closes_unexpected_input_errors(kwargs: dict[str, object]) -> None:
    planned = _plan(**kwargs)

    assert planned.proposal is None
    assert planned.abstain_reason is not None
    assert planned.abstain_reason.startswith("invalid_")


def test_region_plan_result_is_frozen() -> None:
    planned = _plan("abstain: no proof")
    with pytest.raises(FrozenInstanceError):
        planned.abstain_reason = "changed"


def test_region_replacement_validator_rejects_non_region_kind() -> None:
    result = validate_region_replacement_source("statement", "let value = 1;")

    assert not result.ok
    assert result.code == "replace_region_invalid_kind"
