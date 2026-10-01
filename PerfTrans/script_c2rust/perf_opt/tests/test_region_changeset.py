from __future__ import annotations

from dataclasses import dataclass, replace
import hashlib
import json
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.changeset import (
    ChangeOperation,
    ChangeSetStatus,
    ConcreteEdit,
    GateResultRecord,
    HandlerRegistry,
    ImpactScope,
    OperationResolution,
    ProposedChangeSet,
    RegionKind,
    RegionRef,
    ReplaceSourceRegion,
    TriggerContext,
    ValidationResult,
)
from perf_opt.agent_perf_opt.changeset.applier import ChangeSetApplier
from perf_opt.agent_perf_opt.changeset.audit import AuditWriter
from perf_opt.agent_perf_opt.changeset.executor import ChangeSetExecutor
from perf_opt.agent_perf_opt.changeset.handlers import ReplaceSourceRegionHandler
from perf_opt.agent_perf_opt.changeset.resolver import ResolutionRejected
from perf_opt.agent_perf_opt.regions import RegionExtractor
from perf_opt.agent_perf_opt.rewrite_applier import (
    EditTarget,
    locate_fn_span_with_attrs,
)


def _line_col(source: str, needle: str, occurrence: int = 1) -> tuple[int, int]:
    seen = 0
    for line_number, line in enumerate(source.splitlines(), 1):
        if needle in line:
            seen += 1
            if seen == occurrence:
                return line_number, len(line[: line.index(needle)].encode()) + 1
    raise AssertionError(f"missing needle {needle!r}")


def _case(
    tmp_path: Path,
    source: str,
    needles: tuple[str, ...] = ("let value",),
    *,
    replacement: str = "let value = 2;",
    span_start: int | None = None,
    relative_path: str = "src/lib.rs",
    function_occurrence: int = 1,
) -> tuple[Path, Path, ReplaceSourceRegion]:
    crate = tmp_path / "crate"
    path = crate / relative_path
    path.parent.mkdir(parents=True, exist_ok=True)
    data = source.encode("utf-8")
    path.write_bytes(data)

    # Locate the function through the existing extractor contract.  Tests can
    # opt into an attached doc/attribute span by supplying span_start=0.
    from tree_sitter import Language, Parser
    import tree_sitter_rust

    root = Parser(Language(tree_sitter_rust.language())).parse(data).root_node
    stack = [root]
    functions = []
    while stack:
        node = stack.pop()
        if node.type == "function_item":
            name = node.child_by_field_name("name")
            if name is not None and data[name.start_byte : name.end_byte] == b"target":
                functions.append(node)
        stack.extend(reversed(node.children))
    functions.sort(key=lambda node: node.start_byte)
    assert len(functions) >= function_occurrence
    function = functions[function_occurrence - 1]
    target = EditTarget(
        file=path,
        fn_name="target",
        span=(function.start_byte if span_start is None else span_start, function.end_byte),
    )
    hits = []
    for index, needle in enumerate(needles):
        line, col = _line_col(source, needle, index + 1 if len(set(needles)) == 1 else 1)
        hits.append(
            {
                "file": relative_path,
                "function": "target",
                "line": line,
                "col": col,
                "rule": ("C1", "C2", "C3")[index % 3],
                "pattern": needle,
                "snippet": "",
            }
        )
    extracted = RegionExtractor().extract(crate=crate, edit_target=target, hits=hits)
    assert extracted.skipped == ()
    assert len(extracted.candidates) == 1
    region = extracted.candidates[0].region
    operation = ReplaceSourceRegion(
        operation_id="op-region",
        rule_id="C1",
        evidence_hit_ids=region.anchor_hit_ids,
        region=region,
        replacement_region_source=replacement,
    )
    return crate, path, operation


def _case_for_kind(
    tmp_path: Path,
    source: str,
    needle: str,
    kind: RegionKind,
    *,
    replacement: str,
    relative_path: str = "src/lib.rs",
) -> tuple[Path, Path, ReplaceSourceRegion]:
    """Build an operation whose region IS the requested `RegionKind`.

    `_case` derives the region through `RegionExtractor`, which grows a hit into
    the smallest COMPLETE-STATEMENT window around it. That is the right behavior
    for the extractor, but it means the extractor no longer hands back a bare
    `block` / `loop` / `match_arm` for these small fixtures — so a test that
    wants to exercise the kind-specific branches of
    `validate_region_replacement_source` cannot get there through the extractor.
    Here the region is pinned directly to the innermost node that both spans
    `needle` and classifies as `kind`, which is exactly what those branches are
    written against.
    """
    from perf_opt.agent_perf_opt.changeset.types import SymbolKind, SymbolRef
    from perf_opt.agent_perf_opt.regions.cst import (
        attached_function_spans,
        classify_node,
        get_parser,
        named_function_items,
        walk,
    )

    crate = tmp_path / "crate"
    path = crate / relative_path
    path.parent.mkdir(parents=True, exist_ok=True)
    data = source.encode("utf-8")
    path.write_bytes(data)

    root = get_parser().parse(data).root_node
    offset = data.index(needle.encode("utf-8"))
    span = (offset, offset + len(needle.encode("utf-8")))
    matches = [
        node
        for node in walk(root)
        if classify_node(node) is kind
        and node.start_byte <= span[0]
        and span[1] <= node.end_byte
    ]
    assert matches, f"no {kind.value} node spans {needle!r}"
    node = min(matches, key=lambda n: n.end_byte - n.start_byte)

    functions = named_function_items(root, data, "target")
    function = next(
        fn for fn in functions
        if fn.start_byte <= node.start_byte and node.end_byte <= fn.end_byte
    )
    declaration_span = attached_function_spans(function)[0]

    region = RegionRef(
        target_function=SymbolRef(
            qualified_name="target",
            symbol_kind=SymbolKind.FUNCTION,
            file_hint=relative_path,
            declaration_hash=hashlib.sha256(
                data[declaration_span[0]:declaration_span[1]]
            ).hexdigest(),
        ),
        relative_path=relative_path,
        region_kind=kind,
        parent_kind=node.parent.type,
        start_byte=node.start_byte,
        end_byte=node.end_byte,
        expected_region_hash=hashlib.sha256(
            data[node.start_byte:node.end_byte]
        ).hexdigest(),
        anchor_hit_ids=(hashlib.sha256(needle.encode()).hexdigest(),),
        anchor_lines=(data[:node.start_byte].count(b"\n") + 1,),
    )
    operation = ReplaceSourceRegion(
        operation_id="op-region",
        rule_id="C1",
        evidence_hit_ids=region.anchor_hit_ids,
        region=region,
        replacement_region_source=replacement,
    )
    return crate, path, operation


def _apply(path: Path, resolution: OperationResolution) -> None:
    edit = resolution.edits[0]
    data = path.read_bytes()
    replacement = edit.replacement_text.encode("utf-8")
    path.write_bytes(data[: edit.start_byte] + replacement + data[edit.end_byte :])


def test_registry_round_trips_stateless_region_handler(tmp_path: Path) -> None:
    _, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    registry = HandlerRegistry()
    handler = ReplaceSourceRegionHandler()

    registry.register(ReplaceSourceRegion, handler)

    assert registry.handler_for(operation) is handler
    assert not vars(handler)


def test_resolve_reparses_and_returns_one_exact_edit_with_serializable_facts(
    tmp_path: Path,
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )

    resolution = ReplaceSourceRegionHandler().resolve(operation, crate)

    assert resolution.operation_id == operation.operation_id
    assert len(resolution.edits) == 1
    edit = resolution.edits[0]
    data = path.read_bytes()
    assert edit.relative_path == "src/lib.rs"
    assert data[edit.start_byte : edit.end_byte] == b"let value = 1;"
    assert edit.before_hash == hashlib.sha256(data).hexdigest()
    assert edit.replacement_text == "let value = 2;"
    assert resolution.facts["function_span"] == [0, len(data) - 1]
    assert resolution.facts["region_kind"] == "statement"
    assert resolution.facts["parent_kind"] == "block"
    assert resolution.facts["original_region_node_types"] == ["let_declaration"]
    assert resolution.facts["original_file_length"] == len(data)
    assert resolution.facts["original_region_length"] == len(b"let value = 1;")
    assert resolution.facts["region_line_bounds"] == [2, 2]
    assert isinstance(resolution.facts["prefix_sha256"], str)
    assert isinstance(resolution.facts["suffix_sha256"], str)
    json.dumps(resolution.facts)


@pytest.mark.parametrize(
    ("mutation", "match"),
    [
        (lambda data: data.replace(b"let value = 1", b"let value = 9"), "declaration"),
        (lambda data: b"\xff" + data[1:], "UTF-8"),
    ],
)
def test_resolve_rejects_stale_or_unproven_current_file(
    tmp_path: Path, mutation, match: str
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    path.write_bytes(mutation(path.read_bytes()))

    with pytest.raises(ResolutionRejected, match=match) as rejected:
        ReplaceSourceRegionHandler().resolve(operation, crate)

    assert rejected.value.status in {
        ChangeSetStatus.REJECTED_STALE,
        ChangeSetStatus.ABSTAINED_UNPROVEN,
    }


def test_resolve_rejects_ambiguous_same_name_function_at_region(
    tmp_path: Path,
) -> None:
    source = """fn parent() {
    fn target() {
        let value = 1;
    }
}
"""
    crate, path, operation = _case(tmp_path, source)
    path.write_bytes(path.read_bytes().replace(b"fn parent", b"fn target"))

    with pytest.raises(ResolutionRejected, match="ambiguous") as rejected:
        ReplaceSourceRegionHandler().resolve(operation, crate)

    assert rejected.value.status is ChangeSetStatus.REJECTED_STALE


@pytest.mark.parametrize(
    "field_update",
    [
        {"start_byte": 0},
        {"end_byte": 10_000},
        {"expected_region_hash": "0" * 64},
        {"region_kind": RegionKind.EXPRESSION},
        {"parent_kind": "source_file"},
        {"anchor_lines": (1,)},
    ],
)
def test_resolve_rejects_offset_hash_anchor_kind_and_parent_staleness(
    tmp_path: Path, field_update: dict[str, object]
) -> None:
    crate, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    stale_region = replace(operation.region, **field_update)
    stale = replace(
        operation,
        region=stale_region,
        evidence_hit_ids=stale_region.anchor_hit_ids,
    )

    with pytest.raises(ResolutionRejected) as rejected:
        ReplaceSourceRegionHandler().resolve(stale, crate)

    assert rejected.value.status is ChangeSetStatus.REJECTED_STALE


def test_resolve_accepts_attached_doc_comment_block_comment_and_attribute_hash(
    tmp_path: Path,
) -> None:
    source = """/// docs
/* safety */
#[inline]
fn target() {
    let value = 1;
}
"""
    crate, path, operation = _case(tmp_path, source, span_start=0)

    resolution = ReplaceSourceRegionHandler().resolve(operation, crate)

    edit = resolution.edits[0]
    assert path.read_bytes()[edit.start_byte : edit.end_byte] == b"let value = 1;"


@pytest.mark.parametrize("comment", ["// generated", "//// not doc"])
def test_real_locator_line_comment_span_survives_extractor_and_handler(
    tmp_path: Path,
    comment: str,
) -> None:
    source = f"""const BEFORE: i32 = 0;

{comment}
#[inline]
fn target() {{
    let value = 1;
}}
"""
    crate = tmp_path / "crate"
    path = crate / "src" / "lib.rs"
    path.parent.mkdir(parents=True)
    source_bytes = source.encode("utf-8")
    path.write_bytes(source_bytes)
    function_line, _ = _line_col(source, "fn target")
    span = locate_fn_span_with_attrs(
        path,
        "target",
        hint_line_range=(function_line, function_line),
    )
    assert span is not None
    assert source_bytes[span[0] :].startswith(comment.encode("utf-8"))
    target = EditTarget(file=path, fn_name="target", span=span)
    hit_line, hit_col = _line_col(source, "let value")
    hit = {
        "file": "src/lib.rs",
        "function": "target",
        "line": hit_line,
        "col": hit_col,
        "rule": "C1",
        "pattern": "let value",
        "snippet": "",
    }

    extracted = RegionExtractor().extract(
        crate=crate,
        edit_target=target,
        hits=[hit],
    )

    assert extracted.skipped == ()
    assert len(extracted.candidates) == 1
    region = extracted.candidates[0].region
    assert region.target_function.declaration_hash == hashlib.sha256(
        source_bytes[span[0] : span[1]]
    ).hexdigest()
    operation = ReplaceSourceRegion(
        operation_id="op-attached-comment",
        rule_id="C1",
        evidence_hit_ids=region.anchor_hit_ids,
        region=region,
        replacement_region_source="let value = 2;",
    )

    resolution = ReplaceSourceRegionHandler().resolve(operation, crate)

    assert resolution.facts["declaration_span"] == list(span)
    assert len(resolution.edits) == 1


def test_resolve_rejects_file_hint_mismatch_and_path_escape(tmp_path: Path) -> None:
    crate, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    wrong_symbol = replace(operation.region.target_function, file_hint="src/other.rs")
    wrong_hint_region = replace(operation.region, target_function=wrong_symbol)
    wrong_hint = replace(
        operation,
        region=wrong_hint_region,
        evidence_hit_ids=wrong_hint_region.anchor_hit_ids,
    )
    with pytest.raises(ResolutionRejected) as rejected:
        ReplaceSourceRegionHandler().resolve(wrong_hint, crate)
    assert rejected.value.status is ChangeSetStatus.REJECTED_STALE

    # RegionRef prevents constructing an escaping path.  Corrupting the frozen
    # evidence models hostile/deserialized input reaching the handler anyway.
    object.__setattr__(operation.region, "relative_path", "../outside.rs")
    with pytest.raises(ResolutionRejected) as rejected:
        ReplaceSourceRegionHandler().resolve(operation, crate)
    assert rejected.value.status is ChangeSetStatus.REJECTED_STALE


def test_resolve_rejects_missing_nonfile_and_cst_error_inputs(tmp_path: Path) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    original = path.read_bytes()
    path.unlink()
    with pytest.raises(ResolutionRejected) as missing:
        ReplaceSourceRegionHandler().resolve(operation, crate)
    assert missing.value.status is ChangeSetStatus.REJECTED_STALE

    path.mkdir()
    with pytest.raises(ResolutionRejected) as nonfile:
        ReplaceSourceRegionHandler().resolve(operation, crate)
    assert nonfile.value.status is ChangeSetStatus.REJECTED_STALE

    path.rmdir()
    path.write_bytes(original.replace(b"= 1", b"=  "))
    with pytest.raises(ResolutionRejected) as syntax:
        ReplaceSourceRegionHandler().resolve(operation, crate)
    assert syntax.value.status is ChangeSetStatus.ABSTAINED_UNPROVEN

def test_node_sequence_requires_exact_consecutive_complete_statements(
    tmp_path: Path,
) -> None:
    source = """fn target() {
    let first = 1;
    let second = 2;
}
"""
    crate, _, operation = _case(
        tmp_path,
        source,
        ("let", "let"),
        replacement="let combined = 3;",
    )
    assert operation.region.region_kind is RegionKind.NODE_SEQUENCE
    handler = ReplaceSourceRegionHandler()
    assert handler.resolve(operation, crate).edits

    partial_region = replace(operation.region, end_byte=operation.region.end_byte - 1)
    partial_region = replace(
        partial_region,
        expected_region_hash=hashlib.sha256(
            (crate / partial_region.relative_path).read_bytes()[
                partial_region.start_byte : partial_region.end_byte
            ]
        ).hexdigest(),
    )
    partial = replace(
        operation,
        region=partial_region,
        evidence_hit_ids=partial_region.anchor_hit_ids,
    )
    with pytest.raises(ResolutionRejected) as rejected:
        handler.resolve(partial, crate)
    assert rejected.value.status is ChangeSetStatus.REJECTED_STALE


@pytest.mark.parametrize(
    ("kind", "source", "needle", "replacement", "expected_ok"),
    [
        (RegionKind.EXPRESSION, "fn target(x: i32) -> i32 { x + 1 }\n", "x +", "x * 2", True),
        (RegionKind.EXPRESSION, "fn target(x: i32) -> i32 { x + 1 }\n", "x +", "x * 2;", False),
        (RegionKind.BLOCK, "fn target() { { work(); } }\n", "{ work", "{ other(); }", True),
        (RegionKind.BLOCK, "fn target() { { work(); } }\n", "{ work", "other();", False),
        (
            RegionKind.LOOP,
            "fn target() { while ready() { work(); } }\n",
            "while",
            "loop { work(); }",
            True,
        ),
        (
            RegionKind.LOOP,
            "fn target() { while ready() { work(); } }\n",
            "while",
            "if ready() { work(); }",
            False,
        ),
        (
            RegionKind.MATCH_ARM,
            "fn target(x: i32) -> i32 { match x { 1 => 2, _ => 3 } }\n",
            "1 =>",
            "1 => 4,",
            True,
        ),
        (
            RegionKind.MATCH_ARM,
            "fn target(x: i32) -> i32 { match x { 1 => 2, _ => 3 } }\n",
            "1 =>",
            "4",
            False,
        ),
        (
            RegionKind.STATEMENT,
            "fn target() { let value = 1; }\n",
            "let value",
            "let a = 1; let b = 2;",
            True,
        ),
        (RegionKind.STATEMENT, "fn target() { let value = 1; }\n", "let value", "", True),
        (RegionKind.STATEMENT, "fn target() { let value = 1; }\n", "let value", "   \n", True),
    ],
)
def test_pre_validate_enforces_kind_specific_wrapper_shape(
    tmp_path: Path,
    kind: RegionKind,
    source: str,
    needle: str,
    replacement: str,
    expected_ok: bool,
) -> None:
    crate, _, operation = _case_for_kind(
        tmp_path, source, needle, kind, replacement=replacement,
    )
    assert operation.region.region_kind is kind
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)

    result = handler.pre_validate(operation, resolution, crate)

    assert result.ok is expected_ok
    assert result.code.startswith("replace_region_")


@pytest.mark.parametrize(
    "kind",
    [
        RegionKind.EXPRESSION,
        RegionKind.BLOCK,
        RegionKind.LOOP,
        RegionKind.MATCH_ARM,
    ],
)
def test_pre_validate_rejects_empty_nonstatement_replacement(
    tmp_path: Path, kind: RegionKind
) -> None:
    cases = {
        RegionKind.EXPRESSION: ("fn target(x: i32) -> i32 { x + 1 }\n", "x +"),
        RegionKind.BLOCK: ("fn target() { { work(); } }\n", "{ work"),
        RegionKind.LOOP: ("fn target() { loop { work(); } }\n", "loop"),
        RegionKind.MATCH_ARM: (
            "fn target(x: i32) -> i32 { match x { 1 => 2, _ => 3 } }\n",
            "1 =>",
        ),
    }
    source, needle = cases[kind]
    crate, _, operation = _case_for_kind(
        tmp_path, source, needle, kind, replacement=" \n",
    )
    assert operation.region.region_kind is kind
    handler = ReplaceSourceRegionHandler()

    result = handler.pre_validate(operation, handler.resolve(operation, crate), crate)

    assert not result.ok
    assert result.code == "replace_region_empty_not_allowed"


@pytest.mark.parametrize(
    "replacement",
    [
        "fn injected() {}",
        "mod injected {}",
        "impl Thing { fn injected() {} }",
        "trait Injected {}",
        'extern "C" { fn injected(); }',
        "fn target() {}",
    ],
)
def test_pre_validate_rejects_function_module_impl_trait_and_extern_items(
    tmp_path: Path, replacement: str
) -> None:
    crate, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
        replacement=replacement,
    )
    handler = ReplaceSourceRegionHandler()

    result = handler.pre_validate(operation, handler.resolve(operation, crate), crate)

    assert not result.ok
    assert result.code == "replace_region_forbidden_item"


def test_pre_and_post_preserve_identical_original_macro_invocation(
    tmp_path: Path,
) -> None:
    source = "fn target() {\n    let value = vec![1, 2];\n}\n"
    crate, path, operation = _case(
        tmp_path,
        source,
        replacement="let renamed = vec![1, 2];",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)

    assert len(resolution.facts["original_region_macro_hashes"]) == 1
    assert handler.pre_validate(operation, resolution, crate).ok
    _apply(path, resolution)
    assert handler.post_validate(operation, resolution, crate).ok


@pytest.mark.parametrize(
    "replacement",
    [
        "let value = vec![1, 3];",
        "let value = 3;",
        "let value = vec![1, 2]; make_fn!(generated);",
    ],
)
def test_pre_validate_rejects_changed_removed_or_added_macro_invocation(
    tmp_path: Path,
    replacement: str,
) -> None:
    crate, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = vec![1, 2];\n}\n",
        replacement=replacement,
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)

    result = handler.pre_validate(operation, resolution, crate)

    assert not result.ok
    assert result.code == "replace_region_macro_changed"


def test_pre_validate_rejects_macro_definition_that_can_generate_function(
    tmp_path: Path,
) -> None:
    replacement = """macro_rules! make_fn {
    ($name:ident) => { fn $name() {} };
}
make_fn!(generated);
"""
    crate, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
        replacement=replacement,
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)

    result = handler.pre_validate(operation, resolution, crate)

    assert not result.ok
    assert result.code == "replace_region_forbidden_item"


def test_pre_validate_rejects_invoking_existing_function_generating_macro(
    tmp_path: Path,
) -> None:
    source = """macro_rules! make_fn {
    ($name:ident) => { fn $name() {} };
}
fn target() {
    let value = 1;
}
"""
    crate, _, operation = _case(
        tmp_path,
        source,
        replacement="make_fn!(injected); injected();",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)

    result = handler.pre_validate(operation, resolution, crate)

    assert not result.ok
    assert result.code == "replace_region_macro_changed"


def test_post_validate_rejects_macro_change_when_pre_is_bypassed(
    tmp_path: Path,
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = vec![1, 2];\n}\n",
        replacement="let value = vec![1, 3];",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    _apply(path, resolution)

    result = handler.post_validate(operation, resolution, crate)

    assert not result.ok
    assert result.code == "replace_region_post_macro_changed"


def test_pre_validate_returns_stable_failure_for_invalid_utf8_encoding(
    tmp_path: Path,
) -> None:
    crate, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    operation = replace(operation, replacement_region_source="\ud800")
    handler = ReplaceSourceRegionHandler()

    result = handler.pre_validate(operation, handler.resolve(operation, crate), crate)

    assert not result.ok
    assert result.code == "replace_region_utf8"


def test_post_validate_accepts_real_applied_edit_and_rechecks_full_file(
    tmp_path: Path,
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    assert handler.pre_validate(operation, resolution, crate).ok
    _apply(path, resolution)

    result = handler.post_validate(operation, resolution, crate)

    assert result.ok
    assert result.code == "replace_region_post"


def test_post_validate_relocates_one_of_multiple_same_name_impl_methods(
    tmp_path: Path,
) -> None:
    source = """struct First;
struct Second;
impl First {
    // attached
    #[inline]
    fn target() {
        let value = 1;
    }
}
impl Second {
    fn target() {
        let other = 2;
    }
}
"""
    crate, path, operation = _case(
        tmp_path,
        source,
        replacement="let value = compute_more();",
        span_start=source.encode("utf-8").index(b"// attached"),
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    old_function_span = resolution.facts["function_span"]
    old_declaration_span = resolution.facts["declaration_span"]
    _apply(path, resolution)

    result = handler.post_validate(operation, resolution, crate)

    assert result.ok
    delta = len(operation.replacement_region_source.encode()) - len(b"let value = 1;")
    assert old_function_span[0] == old_declaration_span[0] + len(
        b"// attached\n    #[inline]\n    "
    )
    assert delta > 0


def test_post_validate_rejects_wrong_same_name_structure_or_function_boundary(
    tmp_path: Path,
) -> None:
    source = """struct First;
struct Second;
impl First {
    fn target() {
        let value = 1;
    }
}
impl Second {
    fn target() {
        let other = 2;
    }
}
"""
    crate, path, operation = _case(
        tmp_path,
        source,
        replacement="let value = compute_more();",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    _apply(path, resolution)

    wrong_structure = dict(resolution.facts)
    fingerprint = [
        dict(entry) for entry in wrong_structure["function_parent_fingerprint"]
    ]
    impl_entry = next(entry for entry in fingerprint if entry["type"] == "impl_item")
    impl_entry["index"] += 1
    wrong_structure["function_parent_fingerprint"] = fingerprint

    wrong_boundary = dict(resolution.facts)
    function_start, function_end = wrong_boundary["function_span"]
    declaration_start, declaration_end = wrong_boundary["declaration_span"]
    wrong_boundary["function_span"] = [function_start, function_end - 1]
    wrong_boundary["declaration_span"] = [declaration_start, declaration_end - 1]

    for facts in (wrong_structure, wrong_boundary):
        result = handler.post_validate(
            operation,
            replace(resolution, facts=facts),
            crate,
        )
        assert not result.ok
        assert result.code == "replace_region_post_function"


@pytest.mark.parametrize(
    ("fact_name", "fact_value"),
    [
        ("function_span", [True, 10]),
        ("function_span", [0, "10"]),
        ("declaration_span", [0]),
        ("original_region_length", True),
        ("original_region_length", "14"),
        ("original_file_length", True),
        ("function_parent_fingerprint", [{"type": "source_file"}]),
    ],
)
def test_post_validate_rejects_malformed_function_identity_facts(
    tmp_path: Path,
    fact_name: str,
    fact_value: object,
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    _apply(path, resolution)
    facts = dict(resolution.facts)
    facts[fact_name] = fact_value

    result = handler.post_validate(operation, replace(resolution, facts=facts), crate)

    assert not result.ok
    assert result.code == "replace_region_post_facts"


@pytest.mark.parametrize(
    ("tamper", "expected_code"),
    [
        ("prefix", "replace_region_post_prefix"),
        ("suffix", "replace_region_post_suffix"),
        ("replacement", "replace_region_post_image"),
    ],
)
def test_post_validate_rejects_external_tamper_or_wrong_post_image(
    tmp_path: Path, tamper: str, expected_code: str
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    _apply(path, resolution)
    edit = resolution.edits[0]
    data = path.read_bytes()
    replacement_length = len(edit.replacement_text.encode())
    if tamper == "prefix":
        data = data.replace(b"target", b"targat", 1)
    elif tamper == "suffix":
        data += b"// tamper\n"
    else:
        data = (
            data[: edit.start_byte]
            + b"let value = 7;"
            + data[edit.start_byte + replacement_length :]
        )
    path.write_bytes(data)

    result = handler.post_validate(operation, resolution, crate)

    assert not result.ok
    assert result.code == expected_code


def test_pre_validate_rejects_resolution_protocol_mismatch(tmp_path: Path) -> None:
    crate, _, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    handler = ReplaceSourceRegionHandler()
    resolved = handler.resolve(operation, crate)
    edit = resolved.edits[0]
    mismatches = [
        replace(resolved, operation_id="other"),
        replace(resolved, edits=()),
        replace(resolved, edits=(replace(edit, operation_id="other"),)),
        replace(resolved, edits=(replace(edit, relative_path="src/other.rs"),)),
        replace(resolved, edits=(replace(edit, start_byte=edit.start_byte + 1),)),
    ]

    for mismatch in mismatches:
        result = handler.pre_validate(operation, mismatch, crate)
        assert not result.ok
        assert result.code == "replace_region_resolution_mismatch"


def test_node_sequence_allows_multiple_statements_or_empty_deletion(
    tmp_path: Path,
) -> None:
    source = """fn target() {
    let first = 1;
    let second = 2;
}
"""
    crate, _, operation = _case(
        tmp_path,
        source,
        ("let", "let"),
        replacement="let a = 3; let b = 4;",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    assert handler.pre_validate(operation, resolution, crate).ok

    empty = replace(operation, replacement_region_source=" \n\t")
    empty_resolution = replace(
        resolution,
        edits=(replace(resolution.edits[0], replacement_text=" \n\t"),),
    )
    assert handler.pre_validate(empty, empty_resolution, crate).ok


def test_post_validate_rejects_forbidden_item_even_if_pre_is_bypassed(
    tmp_path: Path,
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
        replacement="fn injected() {}",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    assert not handler.pre_validate(operation, resolution, crate).ok
    _apply(path, resolution)

    result = handler.post_validate(operation, resolution, crate)

    assert not result.ok
    assert result.code == "replace_region_post_forbidden_item"


def test_post_validate_rejects_new_cst_error_if_pre_is_bypassed(
    tmp_path: Path,
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
        replacement="let value = ;",
    )
    handler = ReplaceSourceRegionHandler()
    resolution = handler.resolve(operation, crate)
    assert not handler.pre_validate(operation, resolution, crate).ok
    _apply(path, resolution)

    result = handler.post_validate(operation, resolution, crate)

    assert not result.ok
    assert result.code == "replace_region_post_parse"


class _ExecutorState:
    def __init__(self) -> None:
        self.commits: list[tuple[tuple[Path, ...], str]] = []

    def head_sha(self, *, full: bool = True) -> str:
        return "head-region"

    def commit_success(self, paths, message: str) -> str:
        self.commits.append((tuple(Path(path) for path in paths), message))
        return "commit-region"


def _proposal_many(
    operations: tuple[ChangeOperation, ...],
    *,
    changeset_id: str = "cs-region",
) -> ProposedChangeSet:
    first = operations[0]
    return ProposedChangeSet(
        changeset_id=changeset_id,
        base_head="head-region",
        trigger=TriggerContext(
            rule_id=first.rule_id,
            candidate_id="candidate-region",
            hot_function="target",
            hit_ids=tuple(
                hit_id
                for operation in operations
                for hit_id in operation.evidence_hit_ids
            ),
            evidence_artifacts=(),
        ),
        operations=operations,
        impact_scope=ImpactScope.LOCAL_FUNCTION,
    )


def _proposal(operation: ReplaceSourceRegion) -> ProposedChangeSet:
    return _proposal_many((operation,))


def _executor(
    tmp_path: Path,
    crate: Path,
    operation: ReplaceSourceRegion,
    *,
    build_gate,
    registry: HandlerRegistry | None = None,
) -> tuple[ChangeSetExecutor, _ExecutorState, list[tuple[Path, str]]]:
    if registry is None:
        registry = HandlerRegistry()
        registry.register(ReplaceSourceRegion, ReplaceSourceRegionHandler())
    state = _ExecutorState()
    candidate = tmp_path / "candidate-bin"
    candidate.write_bytes(b"candidate")
    promotions: list[tuple[Path, str]] = []
    executor = ChangeSetExecutor(
        crate=crate,
        registry=registry,
        applier=ChangeSetApplier(crate, tmp_path / "journal"),
        state=state,
        audit=AuditWriter(tmp_path / "audit"),
        candidate_binary=candidate,
        build_gate=build_gate,
        w1_gate=lambda: GateResultRecord("w1", True, {}),
        w2_gate=lambda scope: GateResultRecord("w2", True, {"reason": "accepted"}),
        parent_promoter=lambda path, sha: promotions.append((path, sha)),
    )
    assert registry.handler_for(operation).__class__ is ReplaceSourceRegionHandler
    return executor, state, promotions


def _two_region_operations_same_file(
    tmp_path: Path,
    *,
    replacements: tuple[str, str],
) -> tuple[Path, Path, tuple[ReplaceSourceRegion, ReplaceSourceRegion]]:
    source = """fn target() {
    let first = 1;


    let second = 2;
}
"""
    crate, path, first = _case(
        tmp_path,
        source,
        ("let first",),
        replacement=replacements[0],
    )
    _, _, second = _case(
        tmp_path,
        source,
        ("let second",),
        replacement=replacements[1],
    )
    return crate, path, (
        replace(first, operation_id="op-first"),
        replace(second, operation_id="op-second"),
    )


@dataclass(frozen=True)
class _OtherOperation(ChangeOperation):
    relative_path: str


class _OtherOperationHandler:
    def resolve(
        self, operation: _OtherOperation, crate: Path
    ) -> OperationResolution:
        path = crate / operation.relative_path
        data = path.read_bytes()
        start = data.index(b"const MARKER: i32 = 1;")
        end = start + len(b"const MARKER: i32 = 1;")
        return OperationResolution(
            operation_id=operation.operation_id,
            edits=(
                ConcreteEdit(
                    edit_id=f"edit-{operation.operation_id}",
                    operation_id=operation.operation_id,
                    relative_path=operation.relative_path,
                    start_byte=start,
                    end_byte=end,
                    before_hash=hashlib.sha256(data).hexdigest(),
                    replacement_text="const MARKER: i32 = 2;",
                ),
            ),
        )

    def pre_validate(self, operation, resolution, crate) -> ValidationResult:
        return ValidationResult(True, "other_pre")

    def post_validate(self, operation, resolution, crate) -> ValidationResult:
        return ValidationResult(True, "other_post")


def test_executor_commits_real_region_handler_and_applier_edit(tmp_path: Path) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    executor, state, promotions = _executor(
        tmp_path,
        crate,
        operation,
        build_gate=lambda: GateResultRecord("build", True, {}),
    )

    result = executor.execute(_proposal(operation))

    assert result.terminal_status is ChangeSetStatus.COMMITTED
    assert b"let value = 2;" in path.read_bytes()
    assert result.commit_sha == "commit-region"
    assert len(state.commits) == 1
    assert promotions == [(tmp_path / "candidate-bin", "commit-region")]


def test_executor_rolls_back_when_real_post_detects_external_byte_tamper(
    tmp_path: Path,
) -> None:
    crate, path, operation = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
    )
    original = path.read_bytes()

    def tampering_build_gate() -> GateResultRecord:
        path.write_bytes(path.read_bytes() + b"// external tamper\n")
        return GateResultRecord("build", True, {})

    executor, state, promotions = _executor(
        tmp_path, crate, operation, build_gate=tampering_build_gate
    )

    result = executor.execute(_proposal(operation))

    assert result.terminal_status is ChangeSetStatus.REJECTED_POST_VALIDATION
    assert result.rollback_verified
    assert path.read_bytes() == original
    assert state.commits == []
    assert promotions == []


@pytest.mark.parametrize(
    "replacements",
    [
        ("let first = 3;", "let second = 4;"),
        ("let first_with_a_longer_name = 3;", "let second = 4;"),
    ],
)
def test_executor_rejects_two_regions_in_the_same_file_before_apply(
    tmp_path: Path,
    replacements: tuple[str, str],
) -> None:
    crate, path, operations = _two_region_operations_same_file(
        tmp_path,
        replacements=replacements,
    )
    original = path.read_bytes()
    build_calls: list[None] = []
    executor, state, promotions = _executor(
        tmp_path,
        crate,
        operations[0],
        build_gate=lambda: build_calls.append(None)
        or GateResultRecord("build", True, {}),
    )

    result = executor.execute(_proposal_many(operations))

    assert result.terminal_status is ChangeSetStatus.REJECTED_CONFLICT
    assert [validation.code for validation in result.validations] == [
        "operation_edit_bijection",
        "region_file_not_isolated",
    ]
    assert not result.rollback_verified
    assert path.read_bytes() == original
    assert build_calls == []
    assert state.commits == []
    assert promotions == []


def test_executor_rejects_region_and_other_operation_on_the_same_file(
    tmp_path: Path,
) -> None:
    source = """const MARKER: i32 = 1;
fn target() {
    let value = 1;
}
"""
    crate, path, region = _case(tmp_path, source)
    other = _OtherOperation(
        operation_id="op-other",
        rule_id="C2",
        evidence_hit_ids=(),
        relative_path="src/lib.rs",
    )
    original = path.read_bytes()
    registry = HandlerRegistry()
    registry.register(ReplaceSourceRegion, ReplaceSourceRegionHandler())
    registry.register(_OtherOperation, _OtherOperationHandler())
    executor, state, _ = _executor(
        tmp_path,
        crate,
        region,
        build_gate=lambda: GateResultRecord("build", True, {}),
        registry=registry,
    )

    result = executor.execute(_proposal_many((region, other)))

    assert result.terminal_status is ChangeSetStatus.REJECTED_CONFLICT
    assert result.validations[-1].code == "region_file_not_isolated"
    assert not result.rollback_verified
    assert path.read_bytes() == original
    assert state.commits == []


@pytest.mark.parametrize("alias_kind", ["dot_segment", "symlink"])
def test_executor_rejects_region_file_path_alias_before_apply(
    tmp_path: Path,
    alias_kind: str,
) -> None:
    source = """const MARKER: i32 = 1;
fn target() {
    let value = 1;
}
"""
    crate, path, region = _case(tmp_path, source)
    if alias_kind == "dot_segment":
        alias = "src/./lib.rs"
    else:
        alias = "src/lib-alias.rs"
        (crate / alias).symlink_to("lib.rs")
    other = _OtherOperation(
        operation_id="op-alias",
        rule_id="C2",
        evidence_hit_ids=(),
        relative_path=alias,
    )
    original = path.read_bytes()
    build_calls: list[None] = []
    registry = HandlerRegistry()
    registry.register(ReplaceSourceRegion, ReplaceSourceRegionHandler())
    registry.register(_OtherOperation, _OtherOperationHandler())
    executor, state, _ = _executor(
        tmp_path,
        crate,
        region,
        build_gate=lambda: build_calls.append(None)
        or GateResultRecord("build", True, {}),
        registry=registry,
    )

    result = executor.execute(_proposal_many((region, other)))

    assert result.terminal_status is ChangeSetStatus.REJECTED_CONFLICT
    assert result.validations[-1].code == "region_file_not_isolated"
    assert not result.rollback_verified
    assert path.read_bytes() == original
    assert build_calls == []
    assert state.commits == []


def test_executor_allows_one_region_edit_per_distinct_file(tmp_path: Path) -> None:
    crate, first_path, first = _case(
        tmp_path,
        "fn target() {\n    let value = 1;\n}\n",
        replacement="let value = 2;",
        relative_path="src/first.rs",
    )
    _, second_path, second = _case(
        tmp_path,
        "fn target() {\n    let value = 10;\n}\n",
        replacement="let value = 20;",
        relative_path="src/second.rs",
    )
    first = replace(first, operation_id="op-first-file")
    second = replace(second, operation_id="op-second-file")
    executor, state, promotions = _executor(
        tmp_path,
        crate,
        first,
        build_gate=lambda: GateResultRecord("build", True, {}),
    )

    result = executor.execute(_proposal_many((first, second)))

    assert result.terminal_status is ChangeSetStatus.COMMITTED
    assert result.validations[1].code == "region_file_isolated"
    assert b"let value = 2;" in first_path.read_bytes()
    assert b"let value = 20;" in second_path.read_bytes()
    assert len(state.commits) == 1
    assert promotions == [(tmp_path / "candidate-bin", "commit-region")]
