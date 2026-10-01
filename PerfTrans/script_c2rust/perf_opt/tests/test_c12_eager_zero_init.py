"""C12 detects the zeroing c2rust adds that the C original never ran.

C leaves a large local uninitialized and writes only the prefix a runtime
length selects. c2rust cannot spell "uninitialized", emits a full zero
literal, and LLVM cannot remove it: the overwritten extent is a runtime value,
so it cannot prove the buffer is written before it is read. The zeroing
reaches the binary as a memset with a CONSTANT size.

Measured on a string-matching crate: one function carried three such memsets
totalling 26,632 bytes — 10,248 for a struct and 8,192 twice for two score
arrays — against zero memset bytes in the C build of the same function.
"""

from __future__ import annotations

from perf_opt.hot_probe.class_I.rules import (
    C12_MIN_BYTES, _matches_C12, scan_ir_for_c1_c2,
)

CALLEE = "llvm.memset.p0.i64"


def _line(fill: str, size: str, deref: int = 8192) -> str:
    return (f"  call void @{CALLEE}(ptr noundef nonnull align 8 "
            f"dereferenceable({deref}) %buf, {fill}, {size}, i1 false), !dbg !7")


# ───────────────────────────────────────────────── what must be caught

def test_a_large_constant_zero_memset_is_a_hit() -> None:
    assert _matches_C12(CALLEE, _line("i8 0", "i64 8192")) == 8192


def test_the_byte_count_is_reported_not_just_a_boolean() -> None:
    """The size IS the argument for rewriting; a bare flag cannot rank sites."""
    assert _matches_C12(CALLEE, _line("i8 0", "i64 10248")) == 10248


def test_exactly_at_the_floor_counts() -> None:
    assert _matches_C12(CALLEE, _line("i8 0", f"i64 {C12_MIN_BYTES}")) == C12_MIN_BYTES


# ───────────────────────────────────────────────── what must NOT be caught

def test_a_runtime_length_is_not_a_hit() -> None:
    """`i64 %n` IS the extent the program needs — the shape we rewrite toward."""
    assert _matches_C12(CALLEE, _line("i8 0", "i64 %n")) is None


def test_a_non_zero_fill_is_not_a_hit() -> None:
    """`vec![usize::MAX; n]` is a load-bearing sentinel fill, not slack."""
    assert _matches_C12(CALLEE, _line("i8 -1", "i64 8192")) is None


def test_below_the_floor_is_not_a_hit() -> None:
    """Small buffers lower to widened stores, with no memset call to remove."""
    assert _matches_C12(CALLEE, _line("i8 0", "i64 64")) is None


def test_a_non_memset_callee_is_not_a_hit() -> None:
    assert _matches_C12("llvm.memcpy.p0.p0.i64", _line("i8 0", "i64 8192")) is None


def test_the_dereferenceable_operand_is_not_mistaken_for_the_size() -> None:
    """`dereferenceable(N)` carries a number too; only the i64 operand counts."""
    assert _matches_C12(CALLEE, _line("i8 0", "i64 %n", deref=99999)) is None


# ───────────────────────────────────────────────── through the real scanner

def test_the_scanner_emits_c12_sites_with_the_size_in_detail(tmp_path) -> None:
    ir = tmp_path / "m.ll"
    ir.write_text(
        'define void @hot() {\n'
        + _line("i8 0", "i64 8192") + "\n"
        + _line("i8 0", "i64 128") + "\n"       # below floor
        + _line("i8 -1", "i64 8192") + "\n"     # sentinel fill
        + _line("i8 0", "i64 %n") + "\n"        # runtime extent
        + "}\n", encoding="utf-8")
    c12 = [s for s in scan_ir_for_c1_c2(ir) if s.rule_id == "C12"]
    assert len(c12) == 1
    assert c12[0].detail == "8192"
    assert c12[0].define_mangled == "hot"


def test_other_rules_still_carry_no_detail(tmp_path) -> None:
    """`Site.detail` is defaulted; C1/C2 producers must be unchanged."""
    ir = tmp_path / "m.ll"
    ir.write_text(
        'define void @hot() {\n'
        '  call void @_ZN4core9panicking18panic_bounds_check17habcE(), !dbg !7\n'
        '}\n', encoding="utf-8")
    c1 = [s for s in scan_ir_for_c1_c2(ir) if s.rule_id == "C1"]
    assert c1 and all(s.detail == "" for s in c1)


# ──────────── declared zeroing vs. the program's own zeroing loop

from perf_opt.hot_probe.merged_hits import _c12_has_declared_zero_array as _decl


def test_a_declared_zero_array_qualifies() -> None:
    assert _decl("let mut buf: [f64; 1024] = [0.; 1024];")


def test_a_zeroing_loop_does_not() -> None:
    """LLVM's loop-idiom-recognize turns this into a memset too, but that
    memset is the C program's own semantics — the C build runs it as well."""
    body = """
    let mut i: c_int = 0;
    while i < n {
        *symbols.offset(i as isize) = 0 as c_uint;
        i += 1;
    }
    """
    assert not _decl(body)


def test_an_iterator_clear_does_not_either() -> None:
    assert not _decl("for dst in bitlengths_view.iter_mut() { *dst = 0; }")


def test_a_small_incidental_literal_does_not_vouch_for_a_big_memset() -> None:
    """A `[0; 4]` field cannot be the source of a 256-byte memset."""
    assert not _decl("let flags: [u8; 4] = [0; 4];\nwhile i < n { *p = 0; }")


def test_the_element_floor_is_inclusive() -> None:
    assert _decl("let a: [u8; 32] = [0; 32];")
    assert not _decl("let a: [u8; 31] = [0; 31];")


def test_struct_literal_fields_qualify() -> None:
    """c2rust spells a large uninitialized struct as a literal of zero fields."""
    body = """
    let mut m: match_struct = match_struct {
        needle_len: 0,
        lower_needle: [0; 1024],
        match_bonus: [0.; 1024],
    };
    """
    assert _decl(body)


def test_typed_zero_literals_are_recognised() -> None:
    for lit in ("[0u8; 64]", "[0i32; 64]", "[0usize; 64]",
                "[0f64; 64]", "[0.0; 64]", "[ 0 ; 64 ]"):
        assert _decl(f"let a = {lit};"), lit


def test_a_non_zero_fill_literal_does_not_qualify() -> None:
    assert not _decl("let a: [u8; 1024] = [0xff; 1024];")
    assert not _decl("let a = [usize::MAX; 1024];")
