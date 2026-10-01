from pathlib import Path
from types import SimpleNamespace

import pytest

from perf_opt.hot_probe.static_facts import (
    scan_global_declarations,
    validate_const_promotion_safety,
)
from perf_opt.hot_probe.merged_hits import find_const_promote_hits


def _write(crate: Path, relative: str, source: str) -> Path:
    path = crate / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source)
    return path


def test_scan_records_exact_declaration_and_disqualifies_address_use(
    tmp_path: Path,
) -> None:
    source = _write(
        tmp_path,
        "src/lib.rs",
        "pub static PRIME32_1: u32 = 2654435761;\n"
        "fn hot() { let _ = &PRIME32_1; }\n",
    )

    facts = scan_global_declarations(tmp_path)
    fact = next(item for item in facts if item.name == "PRIME32_1")

    assert fact.relative_path == "src/lib.rs"
    assert fact.qualified_name == "crate::PRIME32_1"
    assert fact.kind == "static"
    assert fact.keyword_start == source.read_bytes().index(b"static")
    assert fact.declaration_hash
    safety = validate_const_promotion_safety(tmp_path, fact)
    assert not safety.ok
    assert safety.code == "address_taken"


def test_literal_static_is_safe_and_module_name_is_stable(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/constants.rs",
        "pub(crate) static PRIME: u32 = 0x9E37_79B1u32;\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert fact.qualified_name == "crate::constants::PRIME"
    assert fact.visibility == "pub(crate)"
    assert fact.type_text == "u32"
    assert fact.rhs_text == "0x9E37_79B1u32"
    assert validate_const_promotion_safety(tmp_path, fact).ok


def test_private_read_only_mutable_literal_is_safe(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME32_1: u32 = 2654435761 as u32;\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert fact.mutability == "mut"
    assert validate_const_promotion_safety(tmp_path, fact).ok


def test_labels_and_lifetimes_do_not_hide_later_statics(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "fn scan<'a>(value: &'a u8) { '_c2rust_label: { return; } }\n"
        "static mut PRIME64_1: u64 = 11400714785074694791 as u64;\n",
    )

    facts = {fact.name: fact for fact in scan_global_declarations(tmp_path)}

    assert "PRIME64_1" in facts
    assert validate_const_promotion_safety(tmp_path, facts["PRIME64_1"]).ok


def test_lz4_like_labels_still_emit_const_hits(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/xxhash.rs",
        "fn scan() { '_c2rust_label: { return; } }\n"
        "static mut PRIME64_1: u64 = 11400714785074694791 as u64;\n"
        "static mut PRIME64_2: u64 = 14029467366897019727 as u64;\n"
        "fn XXH64_round(acc: u64, input: u64) -> u64 {\n"
        "    acc.wrapping_add(input.wrapping_mul(PRIME64_2))\n"
        "        .wrapping_mul(PRIME64_1)\n"
        "}\n",
    )
    index = SimpleNamespace(resolve=lambda name: ("src/xxhash.rs", 4, 7))

    hits = find_const_promote_hits(tmp_path, {"XXH64_round"}, index)[
        "XXH64_round"
    ]

    assert {hit.extra["qualified_name"] for hit in hits} == {
        "crate::xxhash::PRIME64_1",
        "crate::xxhash::PRIME64_2",
    }
    assert {hit.extra["decl_file"] for hit in hits} == {"src/xxhash.rs"}
    assert all(len(hit.extra["declaration_hash"]) == 64 for hit in hits)


def test_written_mutable_static_is_rejected(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "unsafe fn update() { PRIME = 2; }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "written"


def test_macro_body_write_disqualifies_mutable_static(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "macro_rules! update { () => { PRIME = 2; } }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "written"


def test_addr_of_mut_disqualifies_mutable_static(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "fn expose() { let _ = core::ptr::addr_of_mut!(PRIME); }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "address_taken"


@pytest.mark.parametrize(
    "reference",
    (
        "&mut PRIME",
        "&crate::constants::PRIME",
        "&raw mut crate::constants::PRIME",
        "&(crate::constants::PRIME)",
        "&mut ((crate::constants::PRIME))",
        "&crate::模块::PRIME",
    ),
)
def test_qualified_and_mutable_references_take_static_address(
    tmp_path: Path, reference: str
) -> None:
    _write(
        tmp_path,
        "src/constants.rs",
        "pub(crate) static mut PRIME: u32 = 1;\n",
    )
    _write(
        tmp_path,
        "src/lib.rs",
        f"fn expose() {{ let _ = {reference}; }}\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "address_taken"


def test_renamed_import_of_mutable_static_fails_closed(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/constants.rs",
        "pub(crate) static mut PRIME: u32 = 1;\n",
    )
    _write(
        tmp_path,
        "src/lib.rs",
        "use crate::constants::PRIME as P;\n"
        "unsafe fn update() { P = 2; }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "renamed_import_unproven"
    )


def test_qualified_write_of_mutable_static_is_rejected(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/constants.rs",
        "pub(crate) static mut PRIME: u32 = 1;\n",
    )
    _write(
        tmp_path,
        "src/lib.rs",
        "unsafe fn update() { crate::constants::PRIME = 2; }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "written"


def test_parenthesized_qualified_write_is_rejected(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/constants.rs",
        "pub(crate) static mut PRIME: u32 = 1;\n",
    )
    _write(
        tmp_path,
        "src/lib.rs",
        "unsafe fn update() { ((crate::constants::PRIME)) = 2; }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "written"


def test_method_receiver_on_mutable_static_fails_closed(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "unsafe fn update() { PRIME.clone_from(&2); }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "implicit_mutation_unproven"
    )


def test_ref_mut_pattern_from_mutable_static_fails_closed(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "unsafe fn update() { let ref mut slot = PRIME; *slot = 2; }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "implicit_mutation_unproven"
    )


def test_destructuring_assignment_to_mutable_static_is_rejected(
    tmp_path: Path,
) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "static mut OTHER: u32 = 3;\n"
        "unsafe fn update() { (PRIME, OTHER) = (2, 4); }\n",
    )

    fact = next(
        fact for fact in scan_global_declarations(tmp_path) if fact.name == "PRIME"
    )

    assert validate_const_promotion_safety(tmp_path, fact).code == "written"


def test_proven_value_reads_of_mutable_static_remain_safe(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "fn sink(_: u32) {}\n"
        "unsafe fn read(x: u32) -> u64 {\n"
        "    let copy = PRIME;\n"
        "    sink(PRIME);\n"
        "    (x + PRIME) as u64 + PRIME as u64 + copy as u64\n"
        "}\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).ok


def test_unicode_renamed_import_of_mutable_static_fails_closed(
    tmp_path: Path,
) -> None:
    _write(
        tmp_path,
        "src/constants.rs",
        "pub(crate) static mut PRIME: u32 = 1;\n",
    )
    _write(
        tmp_path,
        "src/lib.rs",
        "use crate::constants::PRIME as 素;\n"
        "unsafe fn update() { 素 = 2; }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "renamed_import_unproven"
    )


def test_macro_argument_use_of_mutable_static_fails_closed(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "macro_rules! overwrite { ($slot:ident) => { $slot = 2; } }\n"
        "unsafe fn update() { overwrite!(PRIME); }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "macro_argument_unproven"
    )


def test_unicode_macro_argument_use_fails_closed(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "fn inspect() { 检查!(PRIME); }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "macro_argument_unproven"
    )


def test_raw_string_does_not_hide_later_write(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "unsafe fn update() { let _s = r#\"\"\"#; PRIME = 2; }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "written"


def test_raw_string_macro_argument_does_not_hide_static(tmp_path: Path) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        "fn inspect() { pass!(r#\"\" )\"#, PRIME); }\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "macro_argument_unproven"
    )


@pytest.mark.parametrize(
    "literal",
    (
        'r"plain"',
        'r###"embedded " and ##"###',
        'br##"byte " and #"##',
        'cr#"c string"#',
    ),
)
def test_rust_raw_string_forms_do_not_hide_later_write(
    tmp_path: Path, literal: str
) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "static mut PRIME: u32 = 1;\n"
        f"unsafe fn update() {{ let _s = {literal}; PRIME = 2; }}\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == "written"


def test_public_mutable_static_is_rejected(tmp_path: Path) -> None:
    _write(tmp_path, "src/lib.rs", "pub static mut EXPORTED: u32 = 2;\n")

    fact = scan_global_declarations(tmp_path)[0]

    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "externally_visible_mutable_static"
    )


def test_scanner_fails_closed_for_attrs_and_interior_mutability(
    tmp_path: Path,
) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "#[no_mangle]\npub static EXPORTED: u32 = 2;\n"
        "static CELL: Cell<u32> = Cell::new(1);\n",
    )

    facts = {fact.name: fact for fact in scan_global_declarations(tmp_path)}

    assert validate_const_promotion_safety(tmp_path, facts["EXPORTED"]).code == (
        "alias_sensitive_attribute"
    )
    assert validate_const_promotion_safety(tmp_path, facts["CELL"]).code == (
        "interior_mutability"
    )


def test_multiline_attribute_across_blank_line_is_attached_to_static(
    tmp_path: Path,
) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "#[unsafe(\n    no_mangle\n)]\n\n"
        "static mut PRIME: u32 = 1;\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert any("no_mangle" in attr for attr in fact.attrs)
    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "alias_sensitive_attribute"
    )


def test_comment_between_attribute_and_static_preserves_attribute(
    tmp_path: Path,
) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "#[cfg_attr(any(), export_name = \"PRIME\")]\n"
        "// kept with the declaration\n"
        "static mut PRIME: u32 = 1;\n",
    )

    fact = scan_global_declarations(tmp_path)[0]

    assert any("export_name" in attr for attr in fact.attrs)
    assert validate_const_promotion_safety(tmp_path, fact).code == (
        "alias_sensitive_attribute"
    )


def test_comments_strings_and_macro_bodies_do_not_create_declarations(
    tmp_path: Path,
) -> None:
    _write(
        tmp_path,
        "src/lib.rs",
        "// static COMMENT: u32 = 1;\n"
        "const TEXT: &str = \"static STRING: u32 = 2;\";\n"
        "macro_rules! fake { () => { static MACRO: u32 = 3; } }\n"
        "static REAL: i64 = -4i64;\n",
    )

    facts = scan_global_declarations(tmp_path)

    assert [fact.name for fact in facts] == ["TEXT", "REAL"]


def test_const_hit_separates_read_evidence_from_declaration_target(
    tmp_path: Path,
) -> None:
    _write(tmp_path, "src/constants.rs", "pub static PRIME: u32 = 7;\n")
    _write(
        tmp_path,
        "src/hot.rs",
        "pub fn hot(x: u32) -> u32 { x.wrapping_mul(crate::constants::PRIME) }\n",
    )
    index = SimpleNamespace(resolve=lambda name: ("src/hot.rs", 1, 1))

    hits = find_const_promote_hits(tmp_path, {"hot"}, index)["hot"]

    assert len(hits) == 1
    hit = hits[0]
    assert hit.file == "src/hot.rs"
    assert hit.extra["decl_file"] == "src/constants.rs"
    assert hit.extra["qualified_name"] == "crate::constants::PRIME"
    assert hit.extra["symbol_kind"] == "static"
    assert len(hit.extra["declaration_hash"]) == 64
