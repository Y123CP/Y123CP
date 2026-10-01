from dataclasses import FrozenInstanceError
import importlib
import os
from pathlib import Path
import subprocess
import sys

import pytest


SHA_A = "a" * 64
SHA_B = "b" * 64


def _model():
    return importlib.import_module("perf_opt.agent_perf_opt.regions.model")


def _types():
    return importlib.import_module("perf_opt.agent_perf_opt.changeset.types")


def _target(*, kind=None, declaration_hash: str = SHA_A):
    types = _types()
    return types.SymbolRef(
        qualified_name="crate::codec::decode",
        symbol_kind=kind or types.SymbolKind.FUNCTION,
        file_hint="src/codec.rs",
        declaration_hash=declaration_hash,
    )


def _region(**overrides):
    model = _model()
    values = {
        "target_function": _target(),
        "relative_path": "src/codec.rs",
        "region_kind": model.RegionKind.LOOP,
        "parent_kind": "function_item",
        "start_byte": 10,
        "end_byte": 40,
        "expected_region_hash": SHA_B,
        "anchor_hit_ids": (SHA_A, SHA_B),
        "anchor_lines": (12, 18),
    }
    values.update(overrides)
    return model.RegionRef(**values)


def test_region_kind_values_are_stable() -> None:
    model = _model()
    assert {kind.name: kind.value for kind in model.RegionKind} == {
        "EXPRESSION": "expression",
        "STATEMENT": "statement",
        "LOOP": "loop",
        "MATCH_ARM": "match_arm",
        "BLOCK": "block",
        "NODE_SEQUENCE": "node_sequence",
    }


def test_region_ref_is_frozen() -> None:
    region = _region()
    with pytest.raises(FrozenInstanceError):
        region.start_byte = 11  # type: ignore[misc]


@pytest.mark.parametrize(
    "relative_path",
    [
        "",
        "/tmp/codec.rs",
        "C:/tmp/codec.rs",
        r"C:src\codec.rs",
        r"src\..\secret.rs",
        r"\\server\share\codec.rs",
        "src/../codec.rs",
    ],
)
def test_region_ref_requires_canonical_posix_crate_relative_paths(
    relative_path: str,
) -> None:
    with pytest.raises(ValueError, match="relative"):
        _region(relative_path=relative_path)


@pytest.mark.parametrize(
    ("start_byte", "end_byte"),
    [(-1, 20), (10, 10), (20, 10)],
)
def test_region_ref_rejects_invalid_byte_range_values(start_byte, end_byte) -> None:
    with pytest.raises(ValueError, match="byte"):
        _region(start_byte=start_byte, end_byte=end_byte)


@pytest.mark.parametrize(
    ("start_byte", "end_byte"),
    [(True, 20), (10, False), ("10", 20), (10, 20.0)],
)
def test_region_ref_rejects_invalid_byte_range_types(start_byte, end_byte) -> None:
    with pytest.raises(TypeError, match="byte"):
        _region(start_byte=start_byte, end_byte=end_byte)


@pytest.mark.parametrize("digest", ["", "a" * 63, "g" * 64, "A" * 64])
def test_region_ref_rejects_invalid_region_hash(digest: str) -> None:
    with pytest.raises(ValueError, match="expected_region_hash"):
        _region(expected_region_hash=digest)


@pytest.mark.parametrize(
    ("hit_ids", "lines"),
    [
        ((), ()),
        ((SHA_A,), (10, 20)),
        (("not-a-hash",), (10,)),
        ((SHA_A, SHA_A), (10, 20)),
        ((SHA_A, SHA_B), (20, 10)),
        ((SHA_A,), (0,)),
    ],
)
def test_region_ref_rejects_invalid_anchor_values(hit_ids, lines) -> None:
    with pytest.raises(ValueError, match="anchor"):
        _region(anchor_hit_ids=hit_ids, anchor_lines=lines)


@pytest.mark.parametrize(
    ("hit_ids", "lines"),
    [
        ([SHA_A], (10,)),
        ((SHA_A,), [10]),
        ((123,), (10,)),
        ((SHA_A,), (True,)),
    ],
)
def test_region_ref_rejects_invalid_anchor_types(hit_ids, lines) -> None:
    with pytest.raises(TypeError, match="anchor"):
        _region(anchor_hit_ids=hit_ids, anchor_lines=lines)


def test_region_ref_requires_a_hashed_function_symbol() -> None:
    with pytest.raises(ValueError, match="declaration_hash"):
        _region(target_function=_target(declaration_hash="stale"))


@pytest.mark.parametrize(
    "imports",
    [
        (
            "from perf_opt.agent_perf_opt.changeset import RegionRef; "
            "from perf_opt.agent_perf_opt.regions import RegionRef as Exported; "
            "assert RegionRef is Exported"
        ),
        (
            "from perf_opt.agent_perf_opt.regions import RegionRef; "
            "from perf_opt.agent_perf_opt.changeset import RegionRef as Exported; "
            "assert RegionRef is Exported"
        ),
    ],
)
def test_region_and_changeset_modules_support_both_cold_import_orders(
    imports: str,
) -> None:
    env = dict(os.environ)
    env["PYTHONPATH"] = str(Path(__file__).resolve().parents[2])
    completed = subprocess.run(
        [sys.executable, "-c", imports],
        check=False,
        capture_output=True,
        env=env,
        text=True,
        timeout=10,
    )
    assert completed.returncode == 0, completed.stderr


def test_changeset_owns_and_publicly_exports_region_types() -> None:
    changeset = importlib.import_module("perf_opt.agent_perf_opt.changeset")
    types = _types()
    model = _model()

    assert types.RegionKind is model.RegionKind is changeset.RegionKind
    assert types.RegionRef is model.RegionRef is changeset.RegionRef
    source = Path(types.__file__).read_text(encoding="utf-8")
    assert "regions" not in source


def test_replace_source_region_is_frozen_and_preserves_provenance() -> None:
    region = _region()
    operation_type = getattr(_types(), "ReplaceSourceRegion")
    operation = operation_type(
        operation_id="op-region-1",
        rule_id="C1",
        evidence_hit_ids=region.anchor_hit_ids,
        region=region,
        replacement_region_source="for item in slice { consume(item); }",
    )
    with pytest.raises(FrozenInstanceError):
        operation.rule_id = "C2"

    for evidence in ((SHA_A,), (SHA_B, SHA_A)):
        with pytest.raises(ValueError, match="evidence_hit_ids"):
            operation_type(
                operation_id="op-region-bad",
                rule_id="C1",
                evidence_hit_ids=evidence,
                region=region,
                replacement_region_source="consume(slice);",
            )


@pytest.mark.parametrize("replacement", ["", "   \n\t"])
def test_replace_source_region_allows_empty_or_whitespace_source(
    replacement: str,
) -> None:
    region = _region()
    operation_type = getattr(_types(), "ReplaceSourceRegion")
    operation = operation_type(
        operation_id="op-region-empty",
        rule_id="C1",
        evidence_hit_ids=region.anchor_hit_ids,
        region=region,
        replacement_region_source=replacement,
    )
    assert operation.replacement_region_source == replacement
    # Task G3's CST-aware handler alone may allow deletion of a complete
    # statement; it must reject empty replacement for every other region kind.


@pytest.mark.parametrize("replacement", [None, b"", 0])
def test_replace_source_region_rejects_non_string_source(replacement) -> None:
    region = _region()
    operation_type = getattr(_types(), "ReplaceSourceRegion")
    with pytest.raises(TypeError, match="replacement_region_source"):
        operation_type(
            operation_id="op-region-non-string",
            rule_id="C1",
            evidence_hit_ids=region.anchor_hit_ids,
            region=region,
            replacement_region_source=replacement,
        )


def test_rule_capability_uses_an_exact_explicit_table() -> None:
    model = _model()
    expected = {
        "C1": model.RuleCapability.REGION_LOCAL,
        "C2": model.RuleCapability.REGION_LOCAL,
        "C3": model.RuleCapability.REGION_LOCAL,
        "II_vec": model.RuleCapability.REGION_LOCAL,
        "III③": model.RuleCapability.REGION_LOCAL,
        "III④": model.RuleCapability.REGION_LOCAL,
        "III①": model.RuleCapability.CROSS_FUNCTION,
        # II_inl runs as a dedicated planner (one attribute on the callee);
        # the extractor must not be offered it at all.
        "II_inl": model.RuleCapability.UNSUPPORTED,
    }
    assert {rule: model.rule_capability(rule) for rule in expected} == expected
    assert model.rule_capability("C1.extra") is model.RuleCapability.UNSUPPORTED
    assert model.rule_capability("prefix-C1") is model.RuleCapability.UNSUPPORTED
    assert model.rule_capability("unknown") is model.RuleCapability.UNSUPPORTED


def _hit(**overrides):
    hit = {
        "file": "src/编码.rs",
        "function": "crate::编码::decode",
        "line": 12,
        "col": 3,
        "rule": "III④",
        "pattern": "游标",
        "snippet": "指针.add(i)",
    }
    hit.update(overrides)
    return hit


def test_stable_hit_id_is_canonical_and_supports_unicode() -> None:
    model = _model()
    hit = _hit()
    reordered = dict(reversed(tuple(hit.items())))
    assert model.stable_hit_id(hit) == model.stable_hit_id(reordered)
    assert model.stable_hit_id(hit) == (
        "4cf729502cd0ffa038822c715114f90306e10861c4c920fefb3430a509688cf1"
    )


def test_stable_hit_id_defaults_optional_fields_and_prefers_explicit_function() -> None:
    model = _model()
    minimal = {"file": "src/lib.rs", "function": "wrong", "line": 7, "rule": "C1"}
    explicit = dict(minimal, function="crate::right", col=0, pattern="", snippet="")
    assert model.stable_hit_id(
        minimal, function_name="crate::right"
    ) == model.stable_hit_id(explicit)


def test_stable_hit_id_is_stable_for_unresolved_zero_line() -> None:
    model = _model()
    unresolved = {
        "file": "src/lib.rs",
        "function": "crate::decode",
        "line": 0,
        "rule": "C1",
    }
    assert model.stable_hit_id(unresolved) == model.stable_hit_id(dict(unresolved))
    assert model.stable_hit_id(unresolved) != model.stable_hit_id(
        dict(unresolved, line=1)
    )


@pytest.mark.parametrize(
    ("field", "changed"),
    [
        ("file", "src/other.rs"),
        ("function", "crate::other"),
        ("line", 13),
        ("col", 4),
        ("rule", "C1"),
        ("pattern", "other"),
        ("snippet", "other()"),
    ],
)
def test_stable_hit_id_changes_when_any_tuple_field_changes(field, changed) -> None:
    model = _model()
    assert model.stable_hit_id(_hit()) != model.stable_hit_id(
        _hit(**{field: changed})
    )


@pytest.mark.parametrize("missing", ["file", "function", "line", "rule"])
def test_stable_hit_id_rejects_missing_required_fields(missing: str) -> None:
    hit = {"file": "x.rs", "function": "f", "line": 1, "rule": "C1"}
    del hit[missing]
    with pytest.raises(KeyError):
        _model().stable_hit_id(hit)


@pytest.mark.parametrize(
    "hit",
    [
        {"file": 1, "function": "f", "line": 1, "rule": "C1"},
        {"file": "x.rs", "function": 1, "line": 1, "rule": "C1"},
        {"file": "x.rs", "function": "f", "line": True, "rule": "C1"},
        {"file": "x.rs", "function": "f", "line": 1, "col": False, "rule": "C1"},
        {"file": "x.rs", "function": "f", "line": 1, "rule": 1},
    ],
)
def test_stable_hit_id_rejects_invalid_field_types(hit) -> None:
    with pytest.raises(TypeError):
        _model().stable_hit_id(hit)


@pytest.mark.parametrize(
    "hit",
    [
        {"file": "x.rs", "function": "f", "line": -1, "rule": "C1"},
        {"file": "x.rs", "function": "f", "line": 1, "col": -1, "rule": "C1"},
    ],
)
def test_stable_hit_id_rejects_invalid_field_values(hit) -> None:
    with pytest.raises(ValueError):
        _model().stable_hit_id(hit)
