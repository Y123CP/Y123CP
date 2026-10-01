from __future__ import annotations

from dataclasses import FrozenInstanceError
import hashlib
from importlib.metadata import version
from pathlib import Path

import pytest
from tree_sitter import Language, Parser
import tree_sitter_rust

from perf_opt.agent_perf_opt import regions
from perf_opt.agent_perf_opt.changeset import RegionKind
from perf_opt.agent_perf_opt.regions import extractor as extractor_module
from perf_opt.agent_perf_opt.regions import (
    RegionExtractionResult,
    RegionExtractor,
    RuleCapability,
    stable_hit_id,
)
from perf_opt.agent_perf_opt.rewrite_applier import EditTarget


_PARSER = Parser(Language(tree_sitter_rust.language()))


def test_rust_tree_sitter_runtime_is_declared_and_loadable() -> None:
    repository = Path(__file__).resolve().parents[3]
    pyproject = (repository / "pyproject.toml").read_text(encoding="utf-8")
    requirements = (repository / "requirements.txt").read_text(encoding="utf-8")

    for declaration in (
        "tree-sitter==0.25.2",
        "tree-sitter-rust==0.24.2",
    ):
        assert declaration in pyproject
        assert declaration in requirements
    assert version("tree-sitter") == "0.25.2"
    assert version("tree-sitter-rust") == "0.24.2"

    parser = Parser(Language(tree_sitter_rust.language()))
    tree = parser.parse(b"fn smoke() {}")
    assert not tree.root_node.has_error


def _walk(node):
    stack = [node]
    while stack:
        current = stack.pop()
        yield current
        stack.extend(reversed(current.children))


def _write_target(
    tmp_path: Path,
    source: str,
    *,
    relative_path: str = "src/lib.rs",
    fn_name: str = "target",
    include_attrs: bool = False,
) -> tuple[Path, Path, bytes, EditTarget]:
    crate = tmp_path / "crate"
    file_path = crate / relative_path
    file_path.parent.mkdir(parents=True, exist_ok=True)
    source_bytes = source.encode("utf-8")
    file_path.write_bytes(source_bytes)

    root = _PARSER.parse(source_bytes).root_node
    functions = [
        node
        for node in _walk(root)
        if node.type == "function_item"
        and source_bytes[
            node.child_by_field_name("name").start_byte : node.child_by_field_name(
                "name"
            ).end_byte
        ].decode("utf-8")
        == fn_name
    ]
    assert len(functions) == 1
    function = functions[0]
    start = 0 if include_attrs else function.start_byte
    target = EditTarget(
        file=file_path,
        fn_name=fn_name,
        span=(start, function.end_byte),
    )
    return crate, file_path, source_bytes, target


def _line_and_col(source: str, needle: str, *, occurrence: int = 1) -> tuple[int, int]:
    lines = source.splitlines()
    seen = 0
    for line_number, text in enumerate(lines, start=1):
        if needle in text:
            seen += 1
            if seen == occurrence:
                return line_number, len(text[: text.index(needle)].encode("utf-8")) + 1
    raise AssertionError(f"missing needle: {needle!r}")


def _hit(
    relative_path: str,
    line: int,
    col: int,
    *,
    rule: str = "C1",
    snippet: str = "",
    pattern: str = "",
) -> dict[str, object]:
    return {
        "file": relative_path,
        "line": line,
        "col": col,
        "rule": rule,
        "snippet": snippet,
        "pattern": pattern,
    }


def _extract(
    crate: Path,
    target: EditTarget,
    hits: list[dict[str, object]],
    **limits: int,
) -> RegionExtractionResult:
    return RegionExtractor(**limits).extract(
        crate=crate,
        edit_target=target,
        hits=hits,
    )


def _slice(source: bytes, result: RegionExtractionResult, index: int = 0) -> bytes:
    region = result.candidates[index].region
    return source[region.start_byte : region.end_byte]


def test_region_extractor_public_api_is_exported_and_immutable() -> None:
    assert hasattr(regions, "RegionExtractor")
    assert hasattr(regions, "RegionCandidate")
    assert hasattr(regions, "SkippedRegionHit")
    assert hasattr(regions, "RegionExtractionResult")
    assert hasattr(regions, "RegionSkipReason")

    skipped = regions.SkippedRegionHit(
        hit_id="0" * 64,
        rule_id="C1",
        reason=regions.RegionSkipReason.REGION_UNPROVEN,
    )
    with pytest.raises(FrozenInstanceError):
        skipped.reason = "changed"
    with pytest.raises(TypeError, match="reason must be a RegionSkipReason"):
        regions.SkippedRegionHit(
            hit_id="0" * 64,
            rule_id="C1",
            reason="region_unproven",
        )


@pytest.mark.parametrize(
    ("needle", "expected_kind", "expected_source"),
    [
        # A hit anywhere inside a statement now grows to the LARGEST enclosing
        # statement that is a direct child of an in-body block (not the smallest
        # expression/loop/match-arm node). With no padding a single hit anchors
        # on exactly that one statement, so the region_kind is always STATEMENT.
        ("+", RegionKind.STATEMENT, b"let a = x + 1;"),
        ("consume(a);", RegionKind.STATEMENT, b"consume(a);"),
        ("loop", RegionKind.STATEMENT, b"loop { consume(a); }"),
        (
            "=>",
            RegionKind.STATEMENT,
            b"match a {\n        0 => a + 2,\n        _ => a,\n    }",
        ),
        ("{ // block", RegionKind.STATEMENT, b"{ // block\n        consume(a);\n    }"),
    ],
)
def test_hit_grows_to_largest_enclosing_statement(
    tmp_path: Path,
    needle: str,
    expected_kind: RegionKind,
    expected_source: bytes,
) -> None:
    source = """fn target(x: i32) -> i32 {
    let a = x + 1;
    consume(a);
    loop { consume(a); }
    match a {
        0 => a + 2,
        _ => a,
    }
    { // block
        consume(a);
    }
    a
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, needle)
    if needle == "consume(a);":
        col += len("consume(a)".encode("utf-8"))
    # window_pad_lines=0 isolates the enclosing-statement anchoring from the
    # sibling-padding feature (covered by the clustering tests below).
    result = _extract(
        crate, target, [_hit("src/lib.rs", line, col)], window_pad_lines=0
    )

    assert result.skipped == ()
    assert len(result.candidates) == 1
    assert result.candidates[0].region.region_kind is expected_kind
    assert _slice(source_bytes, result) == expected_source


def test_hit_inside_control_structure_grows_to_enclosing_loop_statement(
    tmp_path: Path,
) -> None:
    source = """fn target(x: i32) {
    while x < 10 {
        consume(x);
    }
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "<")

    result = _extract(crate, target, [_hit("src/lib.rs", line, col)])

    # A hit on the loop condition grows to the whole enclosing while statement
    # (a valid replaceable unit), not the bare `x < 10` sub-expression.
    assert result.candidates[0].region.region_kind is RegionKind.STATEMENT
    assert _slice(source_bytes, result) == b"while x < 10 {\n        consume(x);\n    }"


@pytest.mark.parametrize(
    ("source", "needle", "expected_source"),
    [
        ("fn target(x: i32) -> i32 { x }\n", "x }", b"x"),
        (
            "fn target() -> i32 { module::VALUE }\n",
            "module",
            b"module::VALUE",
        ),
        (
            "struct S { value: i32 }\n"
            "impl S { fn target(&self) -> i32 { self.value } }\n",
            "self.value",
            b"self.value",
        ),
    ],
)
def test_leaf_value_nodes_anchor_on_enclosing_tail_statement(
    tmp_path: Path,
    source: str,
    needle: str,
    expected_source: bytes,
) -> None:
    crate, _, source_bytes, target = _write_target(tmp_path, source)

    result = _extract(
        crate,
        target,
        [_hit("src/lib.rs", *_line_and_col(source, needle))],
    )

    # A block's trailing value (tail expression, no semicolon) is NOT a
    # statement — its true classify kind is EXPRESSION, and the resolver
    # validates a single-node region by that kind. A hit inside it (e.g.
    # `self` of `self.value`) anchors on the complete tail expression.
    assert result.skipped == ()
    assert result.candidates[0].region.region_kind is RegionKind.EXPRESSION
    assert _slice(source_bytes, result) == expected_source


def test_large_function_tail_identifier_anchors_on_its_own_statement(
    tmp_path: Path,
) -> None:
    statements = "".join(
        f"    let value_{index} = {index};\n" for index in range(85)
    )
    source = f"fn target(x: i32) -> i32 {{\n{statements}    x\n}}\n"
    crate, _, source_bytes, target = _write_target(tmp_path, source)

    # With no padding the tail identifier anchors on its own one-line statement
    # rather than swallowing the surrounding 85-statement function body.
    result = _extract(
        crate,
        target,
        [_hit("src/lib.rs", *_line_and_col(source, "    x"))],
        window_pad_lines=0,
    )

    assert result.skipped == ()
    assert _slice(source_bytes, result) == b"x"


def test_let_binding_identifier_is_not_misclassified_as_expression(
    tmp_path: Path,
) -> None:
    source = """fn target() {
    let binding_name = compute();
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)

    result = _extract(
        crate,
        target,
        [_hit("src/lib.rs", *_line_and_col(source, "binding_name"))],
    )

    assert result.skipped == ()
    assert result.candidates[0].region.region_kind is RegionKind.STATEMENT
    assert _slice(source_bytes, result) == b"let binding_name = compute();"


def test_shorthand_field_initializer_stays_atomic(tmp_path: Path) -> None:
    source = """struct S { x: i32 }
fn target(x: i32) -> S { S { x } }
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "x }")
    result = _extract(crate, target, [_hit("src/lib.rs", line, col)])

    # The shorthand struct expression is the block's tail expression (no
    # semicolon), so it is a single EXPRESSION region kept as one unit.
    assert result.skipped == ()
    region = result.candidates[0].region
    assert region.region_kind is RegionKind.EXPRESSION
    assert region.parent_kind == "block"
    assert _slice(source_bytes, result) == b"S { x }"


def test_explicit_field_initializer_value_grows_to_enclosing_statement(
    tmp_path: Path,
) -> None:
    source = """struct S { x: i32 }
fn target(input: i32) -> S { S { x: input } }
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "input }")
    result = _extract(crate, target, [_hit("src/lib.rs", line, col)])

    # A hit on the field value anchors on the block's tail expression
    # `S { x: input }` (no semicolon) — a single EXPRESSION region.
    assert result.skipped == ()
    region = result.candidates[0].region
    assert region.region_kind is RegionKind.EXPRESSION
    assert region.parent_kind == "block"
    assert _slice(source_bytes, result) == b"S { x: input }"


def test_literal_in_match_pattern_grows_to_enclosing_match_statement(
    tmp_path: Path,
) -> None:
    source = """fn target(value: i32) -> i32 {
    match value {
        7 => value,
        _ => 0,
    }
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)

    result = _extract(
        crate,
        target,
        [_hit("src/lib.rs", *_line_and_col(source, "7 =>"))],
    )

    # The pattern literal is not misread as a bare value expression: the hit
    # grows to the whole enclosing match statement, a valid replaceable unit.
    assert result.skipped == ()
    assert result.candidates[0].region.region_kind is RegionKind.STATEMENT
    assert _slice(source_bytes, result) == (
        b"match value {\n        7 => value,\n        _ => 0,\n    }"
    )


@pytest.mark.parametrize(
    ("needle", "expected_source"),
    [
        ("unsafe", b"unsafe { consume(); }"),
        ("const", b"const LOCAL: i32 = 1;"),
    ],
)
def test_other_complete_nodes_each_anchor_on_their_own_statement(
    tmp_path: Path,
    needle: str,
    expected_source: bytes,
) -> None:
    source = """fn target() {
    const LOCAL: i32 = 1;
    unsafe { consume(); }
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)

    # window_pad_lines=0 keeps each hit on its own statement instead of
    # clustering the two siblings into one padded node sequence.
    result = _extract(
        crate,
        target,
        [_hit("src/lib.rs", *_line_and_col(source, needle))],
        window_pad_lines=0,
    )

    assert result.skipped == ()
    assert result.candidates[0].region.region_kind is RegionKind.STATEMENT
    assert _slice(source_bytes, result) == expected_source


def test_same_node_hits_merge_with_stable_anchors_and_rule_ids(tmp_path: Path) -> None:
    source = """fn target() {
    let value = compute();
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    line, let_col = _line_and_col(source, "let")
    _, semicolon_col = _line_and_col(source, ";")
    first = _hit("src/lib.rs", line, semicolon_col, rule="C2")
    second = _hit("src/lib.rs", line, let_col, rule="C1")

    result = _extract(crate, target, [first, second])

    assert len(result.candidates) == 1
    candidate = result.candidates[0]
    expected = sorted(
        [
            (stable_hit_id(first, "target"), "C2"),
            (stable_hit_id(second, "target"), "C1"),
        ]
    )
    assert candidate.region.anchor_hit_ids == tuple(hit_id for hit_id, _ in expected)
    assert candidate.region.anchor_lines == (line, line)
    assert candidate.rule_ids == tuple(dict.fromkeys(rule for _, rule in expected))


def test_equivalent_target_paths_share_canonical_stable_hit_id(tmp_path: Path) -> None:
    source = """fn target() {
    let value = compute();
}
"""
    crate, file_path, _, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "let")
    hits = [
        _hit("src/lib.rs", line, col),
        _hit("src/./lib.rs", line, col),
        _hit(str(file_path.resolve()), line, col),
    ]
    canonical_hit = dict(hits[0])
    canonical_hit["file"] = "src/lib.rs"
    expected_hit_id = stable_hit_id(canonical_hit, "target")

    result = _extract(crate, target, hits)

    assert len(result.candidates) == 1
    assert result.candidates[0].region.anchor_hit_ids == (expected_hit_id,)
    assert [(skip.hit_id, skip.reason) for skip in result.skipped] == [
        (expected_hit_id, regions.RegionSkipReason.DUPLICATE_HIT),
        (expected_hit_id, regions.RegionSkipReason.DUPLICATE_HIT),
    ]


def test_file_mismatch_keeps_raw_stable_identity(tmp_path: Path) -> None:
    source = "fn target() {}\n"
    crate, _, _, target = _write_target(tmp_path, source)
    hit = _hit("src/other.rs", 1, 1)

    result = _extract(crate, target, [hit])

    assert result.candidates == ()
    assert [(skip.hit_id, skip.reason) for skip in result.skipped] == [
        (stable_hit_id(hit, "target"), regions.RegionSkipReason.FILE_MISMATCH)
    ]


def test_extraction_builds_parse_line_and_cst_indexes_once(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    assert hasattr(extractor_module, "_build_line_starts")
    assert hasattr(extractor_module, "_build_cst_index")
    source = """fn target() {
    let first = 1;
    let second = 2;
    let third = 3;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    hits = [
        _hit("src/lib.rs", *_line_and_col(source, name), pattern=name)
        for name in ("first", "second", "third")
    ]
    counts = {"parse": 0, "lines": 0, "cst": 0}
    real_parser = extractor_module._get_parser()
    real_line_builder = extractor_module._build_line_starts
    real_cst_builder = extractor_module._build_cst_index

    class CountingParser:
        def parse(self, source_bytes: bytes):
            counts["parse"] += 1
            return real_parser.parse(source_bytes)

    def count_lines(*args, **kwargs):
        counts["lines"] += 1
        return real_line_builder(*args, **kwargs)

    def count_cst(*args, **kwargs):
        counts["cst"] += 1
        return real_cst_builder(*args, **kwargs)

    def reject_sibling_rescan(*args, **kwargs):
        raise AssertionError("sibling indexes must come from the context index")

    monkeypatch.setattr(extractor_module, "_get_parser", lambda: CountingParser())
    monkeypatch.setattr(extractor_module, "_build_line_starts", count_lines)
    monkeypatch.setattr(extractor_module, "_build_cst_index", count_cst)
    monkeypatch.setattr(
        extractor_module,
        "_named_child_index",
        reject_sibling_rescan,
        raising=False,
    )

    result = _extract(crate, target, hits)

    assert result.candidates
    assert counts == {"parse": 1, "lines": 1, "cst": 1}


def test_deeply_nested_valid_function_does_not_use_python_recursion(
    tmp_path: Path,
) -> None:
    depth = 500
    source = (
        "fn target(x: i32) -> i32 {\n"
        + "{\n" * depth
        + "x\n"
        + "}\n" * depth
        + "}\n"
    )
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "x", occurrence=2)

    result = _extract(crate, target, [_hit("src/lib.rs", line, col)])

    # The 500-deep walk completes iteratively (no Python recursion / stack
    # overflow). Under the new semantics the hit grows to the LARGEST enclosing
    # block statement whose span stays within the window budget, so instead of
    # the bare `x` it yields a genuine, bounded enclosing block: still one
    # candidate, still STATEMENT-shaped, capped under window_target_lines, and
    # snapped to real `{ ... }` boundaries that contain the hit.
    assert result.skipped == ()
    assert len(result.candidates) == 1
    region = result.candidates[0].region
    assert region.region_kind is RegionKind.STATEMENT
    region_source = _slice(source_bytes, result)
    assert region_source.startswith(b"{")
    assert region_source.endswith(b"}")
    assert b"\nx\n" in region_source
    # Growth is bounded by the default window (window_target_lines == 250).
    assert region_source.count(b"\n") + 1 <= 250


def test_adjacent_window_pads_never_overlap(tmp_path: Path) -> None:
    # Two hits at opposite ends of one block are split into two windows whose
    # +/- pad ranges would collide in the middle. The de-overlap pass must
    # split the collision at the gap midpoint so the emitted candidate regions
    # NEVER share bytes: overlapping regions poison each other during the
    # sequential apply/rollback loop (the later one's ref goes stale and its
    # whole candidate is dropped). Regression guard for that fix.
    source = (
        "fn target() {\n"
        + "".join(f"    let a{index} = {index};\n" for index in range(8))
        + "}\n"
    )
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    hits = [
        _hit("src/lib.rs", *_line_and_col(source, "a0 =")),
        _hit("src/lib.rs", *_line_and_col(source, "a7 =")),
    ]

    result = _extract(
        crate, target, hits, window_target_lines=10, window_pad_lines=4
    )

    assert result.skipped == ()
    assert len(result.candidates) == 2
    ranges = sorted(
        (c.region.start_byte, c.region.end_byte) for c in result.candidates
    )
    for (start_a, end_a), (start_b, end_b) in zip(ranges, ranges[1:]):
        assert end_a <= start_b, (
            f"candidate regions overlap: [{start_a},{end_a}] & [{start_b},{end_b}]"
        )


@pytest.mark.parametrize(
    ("source", "needle"),
    [
        # tail expression (no semicolon) — used to be mis-labeled STATEMENT and
        # rejected by the resolver, which validates a single node by classify.
        ("fn target(x: i32) -> i32 {\n    let a = x + 1;\n    a + 2\n}\n", "a + 2"),
        ("fn target(x: i32) -> i32 { x }\n", "x }"),
        ("fn target() -> i32 { module::VALUE }\n", "module"),
        # statement sequence and a lone statement
        ("fn target() {\n    let a = 1;\n    let b = 2;\n}\n", "let a"),
        ("fn target(x: i32) {\n    let a = x + 1;\n    consume(a);\n}\n", "x + 1"),
        # a block-expression statement (while) is a STATEMENT and must resolve
        ("fn target(x: i32) -> i32 {\n    while x < 10 { consume(x); }\n    x\n}\n", "<"),
    ],
)
def test_extracted_regions_are_always_resolvable(
    tmp_path: Path, source: str, needle: str
) -> None:
    # Contract: every region the extractor emits MUST resolve — extractor and
    # resolver share one notion of a region-safe node (classify_node). A region
    # the resolver rejects becomes a silent skip in the agent loop, so this
    # guards against the extractor ever emitting one (the tail-expression bug).
    from perf_opt.agent_perf_opt.changeset.handlers.replace_region import (
        resolve_region_ref,
    )

    crate, _, _, target = _write_target(tmp_path, source)
    result = _extract(
        crate, target, [_hit("src/lib.rs", *_line_and_col(source, needle))],
        window_pad_lines=0,
    )
    assert result.skipped == ()
    for candidate in result.candidates:
        resolve_region_ref(crate, candidate.region)  # raises if unresolvable


def test_adjacent_sibling_statements_merge_into_node_sequence(tmp_path: Path) -> None:
    source = """fn target() {
    let a = 1;
    let b = 2;
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    hits = [
        _hit("src/lib.rs", *_line_and_col(source, "let", occurrence=1)),
        _hit("src/lib.rs", *_line_and_col(source, "let", occurrence=2), rule="C2"),
    ]

    result = _extract(crate, target, hits)

    assert len(result.candidates) == 1
    assert result.candidates[0].region.region_kind is RegionKind.NODE_SEQUENCE
    assert _slice(source_bytes, result) == b"let a = 1;\n    let b = 2;"
    assert result.candidates[0].region.parent_kind == "block"


def test_blank_line_separated_siblings_still_cluster_because_blanks_are_free(
    tmp_path: Path,
) -> None:
    source = """fn target() {
    let a = 1;



    let b = 2;
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    hits = [
        _hit("src/lib.rs", *_line_and_col(source, "let", occurrence=1)),
        _hit("src/lib.rs", *_line_and_col(source, "let", occurrence=2), rule="C2"),
    ]

    result = _extract(crate, target, hits)

    # Blank lines cost nothing against the window budget, so two hits in the
    # same block cluster into ONE node sequence regardless of how many empty
    # lines separate them (the old max_sibling_gap no longer splits them).
    assert len(result.candidates) == 1
    assert result.candidates[0].region.region_kind is RegionKind.NODE_SEQUENCE
    assert _slice(source_bytes, result) == b"let a = 1;\n\n\n\n    let b = 2;"


def test_sequence_is_greedily_split_at_complete_sibling_boundary(tmp_path: Path) -> None:
    source = """fn target() {
    let a = 1;
    let b = 2;
    let c = 3;
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    hits = [
        _hit("src/lib.rs", *_line_and_col(source, "let", occurrence=index), rule=rule)
        for index, rule in [(1, "C1"), (2, "C2"), (3, "C3")]
    ]

    # A window's core span (first->last hit) may hold at most `core_budget`
    # non-blank lines, where core_budget = window_target_lines - 2*pad. With
    # pad=0 and window_target_lines=2 the budget is two non-blank lines, so the
    # three-hit run splits at the complete-sibling boundary: {a, b} cluster and
    # {c} spills into its own window.
    result = _extract(
        crate, target, hits, window_pad_lines=0, window_target_lines=2
    )

    assert len(result.candidates) == 2
    # Candidates are ordered by likely hotness (loop depth, then anchor count,
    # then byte offset descending). Nothing here sits in a loop, so the
    # two-anchor cluster {a, b} outranks the single-anchor {c}.
    assert [candidate.region.region_kind for candidate in result.candidates] == [
        RegionKind.NODE_SEQUENCE,
        RegionKind.STATEMENT,
    ]
    assert [_slice(source_bytes, result, index) for index in range(2)] == [
        b"let a = 1;\n    let b = 2;",
        b"let c = 3;",
    ]


def test_single_minimal_node_over_limit_is_structurally_skipped(tmp_path: Path) -> None:
    source = """fn target() {
    let payload = "abcdefghijklmnopqrstuvwxyz";
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    hit = _hit("src/lib.rs", *_line_and_col(source, "let"))

    result = _extract(crate, target, [hit], max_bytes=12)

    assert result.candidates == ()
    assert [(skip.hit_id, skip.reason) for skip in result.skipped] == [
        (
            stable_hit_id(hit, "target"),
            regions.RegionSkipReason.REGION_OVER_LIMIT_UNPROVEN,
        )
    ]


@pytest.mark.parametrize(("line_count", "accepted"), [(400, True), (401, False)])
def test_default_line_limit_is_inclusive_and_rejects_next_line(
    tmp_path: Path,
    line_count: int,
    accepted: bool,
) -> None:
    inner_line_count = line_count - 2
    block_source = "{\n" + "        work();\n" * inner_line_count + "    }"
    assert block_source.encode("utf-8").count(b"\n") + 1 == line_count
    source = f"fn target() {{\n    {block_source}\n}}\n"
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    hit = _hit("src/lib.rs", *_line_and_col(source, "    {"))

    result = _extract(crate, target, [hit])

    if accepted:
        assert result.skipped == ()
        assert _slice(source_bytes, result) == block_source.encode("utf-8")
    else:
        assert result.candidates == ()
        assert [skip.reason for skip in result.skipped] == [
            regions.RegionSkipReason.REGION_OVER_LIMIT_UNPROVEN
        ]


@pytest.mark.parametrize(
    ("region_size", "accepted"),
    [(48 * 1024, True), (48 * 1024 + 1, False)],
)
def test_default_byte_limit_is_inclusive_and_rejects_next_byte(
    tmp_path: Path,
    region_size: int,
    accepted: bool,
) -> None:
    prefix = 'let payload = "'
    suffix = '";'
    payload_size = region_size - len((prefix + suffix).encode("utf-8"))
    statement = prefix + "x" * payload_size + suffix
    assert len(statement.encode("utf-8")) == region_size
    source = f"fn target() {{\n    {statement}\n}}\n"
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    hit = _hit("src/lib.rs", *_line_and_col(source, "let"))

    result = _extract(crate, target, [hit])

    if accepted:
        assert result.skipped == ()
        assert _slice(source_bytes, result) == statement.encode("utf-8")
    else:
        assert result.candidates == ()
        assert [skip.reason for skip in result.skipped] == [
            regions.RegionSkipReason.REGION_OVER_LIMIT_UNPROVEN
        ]


def test_preprocessing_skips_unresolved_mismatch_nonlocal_and_duplicate(
    tmp_path: Path,
) -> None:
    source = """fn target() {
    let a = 1;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "let")
    valid = _hit("src/lib.rs", line, col)
    line_zero = _hit("src/lib.rs", 0, 0, rule="C2")
    mismatch = _hit("src/other.rs", line, col, rule="C3")
    nonlocal_hit = _hit("src/lib.rs", line, col, rule="III①")

    result = _extract(
        crate,
        target,
        [valid, valid.copy(), line_zero, mismatch, nonlocal_hit],
    )

    assert len(result.candidates) == 1
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.DUPLICATE_HIT,
        regions.RegionSkipReason.UNRESOLVED_LINE,
        regions.RegionSkipReason.FILE_MISMATCH,
        regions.RegionSkipReason.CROSS_FN_REQUIRES_CHANGESET_V2,
    ]


@pytest.mark.parametrize(
    ("rule", "reason"),
    [
        (
            "III①",
            regions.RegionSkipReason.CROSS_FN_REQUIRES_CHANGESET_V2,
        ),
        ("unknown-rule", regions.RegionSkipReason.UNSUPPORTED_RULE),
    ],
)
def test_nonlocal_capability_has_precise_skip_reason(
    tmp_path: Path,
    rule: str,
    reason: str,
) -> None:
    source = "fn target() {}\n"
    crate, _, _, target = _write_target(tmp_path, source)
    hit = _hit("src/lib.rs", 1, 1, rule=rule)

    result = _extract(crate, target, [hit])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [reason]


def test_invalid_hit_identity_fails_the_call_instead_of_creating_candidate(
    tmp_path: Path,
) -> None:
    source = "fn target() {}\n"
    crate, _, _, target = _write_target(tmp_path, source)

    with pytest.raises(KeyError, match="missing required hit field: rule"):
        _extract(crate, target, [{"file": "src/lib.rs", "line": 1, "col": 1}])


@pytest.mark.parametrize("invalid_name", [None, ""])
def test_invalid_target_function_name_fails_closed_for_identifiable_hits(
    tmp_path: Path,
    invalid_name: object,
) -> None:
    source = """fn target() {
    let value = 1;
}
"""
    crate, file_path, _, target = _write_target(tmp_path, source)
    invalid_target = EditTarget(
        file=file_path,
        fn_name=invalid_name,
        span=target.span,
    )
    hit = _hit("src/lib.rs", *_line_and_col(source, "let"))
    hit["function"] = "target"

    result = _extract(crate, invalid_target, [hit])

    assert result.candidates == ()
    assert [(skip.hit_id, skip.reason) for skip in result.skipped] == [
        (stable_hit_id(hit), regions.RegionSkipReason.REGION_UNPROVEN)
    ]


def test_parser_runtime_uncertainty_fails_closed(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    source = """fn target() {
    let value = 1;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    hit = _hit("src/lib.rs", *_line_and_col(source, "let"))

    def fail_parser():
        raise RuntimeError("parser unavailable")

    monkeypatch.setattr(extractor_module, "_get_parser", fail_parser)

    result = _extract(crate, target, [hit])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.REGION_UNPROVEN
    ]


class _ParserAbort(BaseException):
    pass


@pytest.mark.parametrize(
    "error_type",
    [KeyboardInterrupt, SystemExit, MemoryError, _ParserAbort],
)
def test_process_level_parser_failures_propagate(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    error_type: type[BaseException],
) -> None:
    source = """fn target() {
    let value = 1;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    hit = _hit("src/lib.rs", *_line_and_col(source, "let"))

    def abort_parser():
        raise error_type("abort")

    monkeypatch.setattr(extractor_module, "_get_parser", abort_parser)

    with pytest.raises(error_type):
        _extract(crate, target, [hit])


def test_parse_call_runtime_uncertainty_fails_closed(
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    source = """fn target() {
    let value = 1;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    hit = _hit("src/lib.rs", *_line_and_col(source, "let"))

    class BrokenParser:
        def parse(self, source_bytes: bytes):
            raise RuntimeError("parse failed")

    monkeypatch.setattr(extractor_module, "_get_parser", BrokenParser)

    result = _extract(crate, target, [hit])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.REGION_UNPROVEN
    ]


def test_comment_only_and_outside_function_hits_are_unproven(tmp_path: Path) -> None:
    source = """fn target() {
    // comment only
    let a = 1;
}

fn other() {
    let outside = 2;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    comment = _hit("src/lib.rs", *_line_and_col(source, "//"))
    outside = _hit("src/lib.rs", *_line_and_col(source, "outside"), rule="C2")

    result = _extract(crate, target, [comment, outside])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.REGION_UNPROVEN,
        regions.RegionSkipReason.REGION_UNPROVEN,
    ]


def test_empty_snippet_and_whitespace_position_still_use_cst(tmp_path: Path) -> None:
    source = """fn target() {
    let value = 1;
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, _ = _line_and_col(source, "let")

    result = _extract(crate, target, [_hit("src/lib.rs", line, 1, snippet="")])

    assert result.skipped == ()
    assert _slice(source_bytes, result) == b"let value = 1;"


def test_valid_line_with_unknown_column_uses_first_syntax_byte(tmp_path: Path) -> None:
    source = """fn target() {
    let value = 1;
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, _ = _line_and_col(source, "let")

    result = _extract(crate, target, [_hit("src/lib.rs", line, 0)])

    assert result.skipped == ()
    assert _slice(source_bytes, result) == b"let value = 1;"


def test_extraction_never_mutates_target_source(tmp_path: Path) -> None:
    source = """fn target() {
    let value = compute();
}
"""
    crate, file_path, _, target = _write_target(tmp_path, source)
    before = file_path.read_bytes()

    result = _extract(
        crate,
        target,
        [_hit("src/lib.rs", *_line_and_col(source, "compute"))],
    )

    assert result.candidates
    assert file_path.read_bytes() == before


def test_nonzero_column_beyond_line_end_is_unproven(tmp_path: Path) -> None:
    source = """fn target() {
    let value = 1;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    line, _ = _line_and_col(source, "let")

    result = _extract(crate, target, [_hit("src/lib.rs", line, 10_000)])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.REGION_UNPROVEN
    ]


def test_explicit_comment_position_does_not_drift_to_same_line_syntax(
    tmp_path: Path,
) -> None:
    source = """fn target() {
    let value = 1; // detector comment
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "detector")

    result = _extract(crate, target, [_hit("src/lib.rs", line, col)])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.REGION_UNPROVEN
    ]


def test_unknown_column_on_comment_first_line_is_unproven(tmp_path: Path) -> None:
    source = """fn target() {
    /* detector */ let value = 1;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    line, _ = _line_and_col(source, "detector")

    result = _extract(crate, target, [_hit("src/lib.rs", line, 0)])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.REGION_UNPROVEN
    ]


def test_unicode_column_is_a_one_based_byte_column(tmp_path: Path) -> None:
    source = """fn target() {
    let café = α + 1;
}
"""
    crate, _, source_bytes, target = _write_target(tmp_path, source)
    line, col = _line_and_col(source, "+")

    result = _extract(crate, target, [_hit("src/lib.rs", line, col)])

    # The `+` column is resolved as a 1-based BYTE offset (multibyte `café`/`α`
    # earlier on the line shift it), landing correctly inside the statement,
    # which then grows to the whole enclosing `let` statement.
    assert result.skipped == ()
    assert _slice(source_bytes, result) == "let café = α + 1;".encode("utf-8")


def test_unicode_column_inside_multibyte_codepoint_is_unproven(tmp_path: Path) -> None:
    source = """fn target() {
    let café = α + 1;
}
"""
    crate, _, _, target = _write_target(tmp_path, source)
    line, alpha_col = _line_and_col(source, "α")

    result = _extract(
        crate,
        target,
        [_hit("src/lib.rs", line, alpha_col + 1)],
    )

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.REGION_UNPROVEN
    ]


def test_attributes_canonical_path_hashes_and_descending_order(tmp_path: Path) -> None:
    source = """#[inline]
fn target() {
    let first = 1;



    let second = 2;
}
"""
    crate, file_path, source_bytes, target = _write_target(
        tmp_path,
        source,
        include_attrs=True,
    )
    first = _hit("src/./lib.rs", *_line_and_col(source, "first"))
    second = _hit(str(file_path.resolve()), *_line_and_col(source, "second"), rule="C2")

    # Force the two hits into separate candidates (core_budget of one non-blank
    # line) so the descending-start ordering across candidates is exercised;
    # blanks are free, so a wider budget would cluster them into one.
    result = _extract(
        crate, target, [first, second], window_pad_lines=0, window_target_lines=1
    )

    assert len(result.candidates) == 2
    starts = [candidate.region.start_byte for candidate in result.candidates]
    assert starts == sorted(starts, reverse=True)
    assert starts[0] > starts[1]
    for candidate in result.candidates:
        region = candidate.region
        assert region.relative_path == "src/lib.rs"
        assert region.target_function.qualified_name == "target"
        assert region.target_function.file_hint == "src/lib.rs"
        assert region.target_function.declaration_hash == hashlib.sha256(
            source_bytes[target.span[0] : target.span[1]]
        ).hexdigest()
        assert region.expected_region_hash == hashlib.sha256(
            source_bytes[region.start_byte : region.end_byte]
        ).hexdigest()


def test_edit_target_span_may_include_doc_comment_and_attributes(tmp_path: Path) -> None:
    source = """/// hot path
#[inline]
fn target() {
    let value = 1;
}
"""
    crate, _, source_bytes, target = _write_target(
        tmp_path,
        source,
        include_attrs=True,
    )
    hit = _hit("src/lib.rs", *_line_and_col(source, "let"))

    result = _extract(crate, target, [hit])

    assert result.skipped == ()
    assert _slice(source_bytes, result) == b"let value = 1;"


@pytest.mark.parametrize("failure", ["parse", "span", "path"])
def test_uncertain_parse_span_or_path_fails_closed(
    tmp_path: Path,
    failure: str,
) -> None:
    if failure == "parse":
        source = "fn target() {\n    let broken = ;\n}\n"
        crate, file_path, _, target = _write_target(tmp_path, source)
        line, col = _line_and_col(source, "broken")
    else:
        source = "fn target() {\n    let value = 1;\n}\n"
        crate, file_path, _, target = _write_target(tmp_path, source)
        line, col = _line_and_col(source, "value")
    if failure == "span":
        target = EditTarget(file=file_path, fn_name="target", span=(0, 5))
    if failure == "path":
        outside = tmp_path / "outside.rs"
        outside.write_text(source, encoding="utf-8")
        target = EditTarget(file=outside, fn_name="target", span=target.span)
        hit_file = str(outside.resolve())
    else:
        hit_file = "src/lib.rs"

    hit = _hit(hit_file, line, col)
    result = _extract(crate, target, [hit])

    assert result.candidates == ()
    assert [(skip.hit_id, skip.reason) for skip in result.skipped] == [
        (stable_hit_id(hit, "target"), regions.RegionSkipReason.REGION_UNPROVEN)
    ]


def test_file_mismatch_is_decided_before_rust_parse_uncertainty(tmp_path: Path) -> None:
    source = "fn target() {\n    let broken = ;\n}\n"
    crate, _, _, target = _write_target(tmp_path, source)
    hit = _hit("src/not-the-target.rs", *_line_and_col(source, "broken"))

    result = _extract(crate, target, [hit])

    assert result.candidates == ()
    assert [skip.reason for skip in result.skipped] == [
        regions.RegionSkipReason.FILE_MISMATCH
    ]
