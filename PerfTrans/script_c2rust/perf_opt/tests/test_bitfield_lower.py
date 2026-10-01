"""C9 — deterministic bitfield lowering: analysis, codegen and detection."""

from __future__ import annotations

from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.bitfield_lower import analyze_source
from perf_opt.agent_perf_opt.planners.bitfield import plan_bitfield_lowering
from perf_opt.agent_perf_opt.regions.model import stable_hit_id
from perf_opt.hot_probe.merged_hits import find_bitfield_accessor_hits
from perf_opt.hot_probe.symbol_source import build_fn_index

# A two-block struct in the exact shape c2rust emits.
TWO_BLOCK = """\
use core::ffi::*;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct parser {
    #[bitfield(name = "type_0", ty = "c_uint", bits = "0..=1")]
    #[bitfield(name = "state", ty = "c_uint", bits = "10..=16")]
    pub type_0_state: [u8; 4],
    pub nread: u32,
    #[bitfield(name = "status_code", ty = "c_uint", bits = "0..=15")]
    #[bitfield(name = "upgrade", ty = "c_uint", bits = "31..=31")]
    pub status_code_upgrade: [u8; 4],
}
"""


def _derive_get(field: bytes, lo: int, hi: int, signed: bool, total: int) -> int:
    """Reference semantics: the derive's per-bit `get_field` loop, verbatim."""
    val = 0
    for i, bit_index in enumerate(range(lo, hi + 1)):
        if field[bit_index // 8] & (1 << (bit_index % 8)):
            val |= 1 << i
    if signed:
        width = hi - lo + 1
        unused = total - width
        val = (val << unused) & ((1 << total) - 1)
        # arithmetic shift right on a `total`-bit two's-complement value
        sign = 1 << (total - 1)
        val = (val ^ sign) - sign
        val >>= unused
    return val


def _derive_set(field: bytearray, lo: int, hi: int, value: int) -> None:
    """Reference semantics: the derive's per-bit `set_field` loop, verbatim."""
    for i, bit_index in enumerate(range(lo, hi + 1)):
        mask = 1 << (bit_index % 8)
        if (value >> i) & 1:
            field[bit_index // 8] |= mask
        else:
            field[bit_index // 8] &= ~mask & 0xFF


def _lowered_get(field: bytes, lo: int, width: int, carrier_bits: int) -> int:
    raw = int.from_bytes(field, "little")
    mask = (1 << width) - 1
    return (raw >> lo) & mask


def _lowered_set(field: bytearray, lo: int, width: int, value: int,
                 carrier_bits: int) -> None:
    raw = int.from_bytes(field, "little")
    mask = (1 << width) - 1
    raw = (raw & ~(mask << lo)) | ((value & mask) << lo)
    field[:] = raw.to_bytes(len(field), "little")


@pytest.mark.parametrize("lo,hi", [(0, 1), (2, 9), (10, 16), (24, 28), (31, 31),
                                   (0, 15), (16, 23)])
def test_mask_shift_matches_derive_bit_loop(lo: int, hi: int) -> None:
    """The emitted formula must agree with the derive on every value it can hold.

    This is the correctness argument for the whole rule: `analyze_source` only
    ever emits this formula, so equivalence here is equivalence everywhere.
    """
    width = hi - lo + 1
    for seed in (0x00000000, 0xFFFFFFFF, 0xA5A5A5A5, 0x12345678):
        for value in range(0, min(1 << width, 64)):
            derive_field = bytearray(seed.to_bytes(4, "little"))
            lowered_field = bytearray(seed.to_bytes(4, "little"))
            _derive_set(derive_field, lo, hi, value)
            _lowered_set(lowered_field, lo, width, value, 32)
            assert derive_field == lowered_field, (lo, hi, seed, value)
            assert (
                _derive_get(bytes(derive_field), lo, hi, False, 32)
                == _lowered_get(bytes(lowered_field), lo, width, 32)
            )


@pytest.mark.parametrize("byte_len,lo,hi", [
    (1, 0, 3), (1, 4, 7),
    (2, 0, 11), (2, 12, 15),
    (3, 0, 7), (3, 8, 19), (3, 20, 23),      # odd width — zero-padded carrier
    (6, 0, 31), (6, 32, 47),
])
def test_mask_shift_matches_derive_on_non_power_of_two_blocks(
    byte_len: int, lo: int, hi: int
) -> None:
    """Odd-width storage blocks widen through a padded carrier on read and are
    truncated on write; neither may become observable."""
    width = hi - lo + 1
    for seed in (0, (1 << (8 * byte_len)) - 1, 0xA5A5A5A5A5A5 % (1 << (8 * byte_len))):
        for value in range(0, min(1 << width, 48)):
            derive_field = bytearray(seed.to_bytes(byte_len, "little"))
            lowered_field = bytearray(seed.to_bytes(byte_len, "little"))
            _derive_set(derive_field, lo, hi, value)
            _lowered_set(lowered_field, lo, width, value, 8 * byte_len)
            assert derive_field == lowered_field, (byte_len, lo, hi, seed, value)
            assert (
                _derive_get(bytes(derive_field), lo, hi, False, 8 * byte_len)
                == _lowered_get(bytes(lowered_field), lo, width, 8 * byte_len)
            )


def test_analyze_extracts_blocks_and_specs() -> None:
    rewrites, skipped = analyze_source(TWO_BLOCK.encode())
    assert skipped == []
    assert len(rewrites) == 1
    rewrite = rewrites[0]
    assert rewrite.struct_name == "parser"
    assert [block.field_name for block in rewrite.blocks] == [
        "type_0_state", "status_code_upgrade",
    ]
    assert all(block.byte_len == 4 for block in rewrite.blocks)
    assert rewrite.accessor_names == frozenset({
        "type_0", "set_type_0", "state", "set_state",
        "status_code", "set_status_code", "upgrade", "set_upgrade",
    })


def test_generated_code_drops_derive_and_keeps_layout() -> None:
    rewrite = analyze_source(TWO_BLOCK.encode())[0][0]
    text = rewrite.replacement_text
    assert "BitfieldStruct" not in text
    assert "#[bitfield(" not in text
    # storage fields, and therefore the #[repr(C)] layout, are untouched
    assert "pub type_0_state: [u8; 4]," in text
    assert "pub status_code_upgrade: [u8; 4]," in text
    assert "#[repr(C)]" in text
    # accessors keep their names/signatures so call sites need no change
    assert "pub fn state(&self) -> c_uint" in text
    assert "pub fn set_state(&mut self, int: c_uint)" in text
    assert text.count("#[inline(always)]") >= 8
    # the emitted bit positions match the declaration
    assert "_bf_get_4(&self.type_0_state, 10, 7)" in text
    assert "_bf_get_4(&self.status_code_upgrade, 0, 16)" in text


def test_signed_field_gets_sign_extension() -> None:
    src = TWO_BLOCK.replace('ty = "c_uint", bits = "10..=16"',
                            'ty = "c_int", bits = "10..=16"')
    rewrite = analyze_source(src.encode())[0][0]
    text = rewrite.replacement_text
    assert "_bf_sext_4" in text
    assert "pub fn state(&self) -> c_int" in text


def test_bool_bitfield_is_skipped_whole_struct() -> None:
    src = TWO_BLOCK.replace('ty = "c_uint", bits = "31..=31"',
                            'ty = "bool", bits = "31..=31"')
    rewrites, skipped = analyze_source(src.encode())
    assert rewrites == []
    assert any("bool" in reason for reason in skipped)


def test_oversized_block_is_skipped() -> None:
    src = TWO_BLOCK.replace("pub type_0_state: [u8; 4],",
                            "pub type_0_state: [u8; 16],")
    rewrites, skipped = analyze_source(src.encode())
    assert rewrites == []
    assert any("16 bytes" in reason for reason in skipped)


# The two shapes below both appear verbatim in c2rust output for other projects;
# either one used to reject the whole struct.
def test_fully_qualified_field_type_is_handled_and_reemitted() -> None:
    src = TWO_BLOCK.replace('ty = "c_uint"', 'ty = "::core::ffi::c_uint"')
    rewrites, skipped = analyze_source(src.encode())
    assert skipped == []
    text = rewrites[0].replacement_text
    # signedness classifies off the last segment, but the accessor must keep the
    # path as written — the file may lack the `use` that a short name needs
    assert "pub fn state(&self) -> ::core::ffi::c_uint" in text
    assert "_bf_sext" not in text


def test_padding_attribute_is_ignored_not_rejected() -> None:
    src = TWO_BLOCK.replace(
        "    pub nread: u32,",
        "    #[bitfield(padding)]\n    pub _pad: [u8; 2],\n    pub nread: u32,",
    )
    rewrites, skipped = analyze_source(src.encode())
    assert skipped == []
    rewrite = rewrites[0]
    # the padding field contributes no accessor and no block
    assert [block.field_name for block in rewrite.blocks] == [
        "type_0_state", "status_code_upgrade",
    ]
    assert "#[bitfield(padding)]" not in rewrite.replacement_text
    assert "pub _pad: [u8; 2]," in rewrite.replacement_text


def test_three_byte_block_widens_through_padded_carrier() -> None:
    src = """\
use core::ffi::*;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct state {
    #[bitfield(name = "a", ty = "c_uint", bits = "0..=7")]
    #[bitfield(name = "b", ty = "c_uint", bits = "8..=23")]
    pub a_b: [u8; 3],
}
"""
    rewrites, skipped = analyze_source(src.encode())
    assert skipped == []
    text = rewrites[0].replacement_text
    assert "fn _bf_get_3(field: &[u8; 3]" in text
    assert "let mut buf = [0u8; 4];" in text
    assert "buf[..3].copy_from_slice(field);" in text
    # the write must put back only the block's own three bytes
    assert "field.copy_from_slice(&out.to_le_bytes()[..3]);" in text
    assert "*field = out.to_le_bytes();" not in text


def test_bits_escaping_block_is_skipped() -> None:
    src = TWO_BLOCK.replace('bits = "10..=16"', 'bits = "10..=40"')
    rewrites, skipped = analyze_source(src.encode())
    assert rewrites == []
    assert any("escape" in reason for reason in skipped)


def test_struct_without_bitfield_derive_is_ignored() -> None:
    src = """\
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plain { pub a: [u8; 4], pub b: u32 }
"""
    rewrites, skipped = analyze_source(src.encode())
    assert rewrites == []
    assert skipped == []


def _crate(tmp_path: Path, lib_src: str, hot_src: str) -> Path:
    crate = tmp_path / "crate"
    (crate / "src").mkdir(parents=True)
    (crate / "src" / "types.rs").write_text(lib_src)
    (crate / "src" / "hot.rs").write_text(hot_src)
    return crate


def test_detector_hits_hot_fn_and_resolves_decl_file(tmp_path: Path) -> None:
    hot = """\
pub fn run(p: *mut crate::src::types::parser) -> u32 {
    unsafe {
        (*p).set_state(3);
        let s = (*p).state();
        (*p).set_status_code(200);
        s + (*p).status_code()
    }
}
"""
    crate = _crate(tmp_path, TWO_BLOCK, hot)
    hits = find_bitfield_accessor_hits(crate, {"run"}, build_fn_index(crate))
    assert set(hits) == {"run"}
    (hit,) = hits["run"]
    assert hit.rule == "C9"
    assert hit.extra["decl_file"] == "src/types.rs"
    assert hit.extra["struct_name"] == "parser"
    # one hit per (fn, struct) — not one per call site
    assert hit.extra["accessor_calls"] == 4


def test_detector_ignores_same_named_methods_of_other_types(
    tmp_path: Path,
) -> None:
    """A method named like an accessor, but on a type with no lowerable
    bitfield struct, must not fire — the rule keys off the declaration."""
    hot = """\
pub fn run(v: &Vec<u8>) -> usize {
    let s = v.len();
    s + v.capacity()
}
"""
    plain = """\
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plain { pub a: [u8; 4] }
"""
    crate = _crate(tmp_path, plain, hot)
    assert find_bitfield_accessor_hits(crate, {"run"}, build_fn_index(crate)) == {}


def test_planner_emits_one_operation_per_struct(tmp_path: Path) -> None:
    hot = """\
pub fn run(p: *mut crate::src::types::parser) -> u32 {
    unsafe { (*p).set_state(3); (*p).state() }
}
"""
    crate = _crate(tmp_path, TWO_BLOCK, hot)
    hits = find_bitfield_accessor_hits(crate, {"run"}, build_fn_index(crate))
    flat = []
    for fn, fn_hits in hits.items():
        for hit in fn_hits:
            record = {
                "rule": hit.rule, "pattern": hit.pattern, "file": hit.file,
                "line": hit.line, "col": hit.col, "snippet": hit.snippet,
                "extra": hit.extra, "function": fn,
            }
            record["id"] = stable_hit_id(record, fn)
            flat.append(record)
    plan = plan_bitfield_lowering(
        crate=crate, hot_function="run", hits=flat,
        base_head="0" * 40, candidate_id="test",
    )
    assert plan.abstain_reason is None
    assert plan.struct_names == ("parser",)
    proposal = plan.proposal
    assert proposal is not None
    (operation,) = proposal.operations
    assert operation.rule_id == "C9"
    assert operation.relative_path == "src/types.rs"
    assert operation.struct_name == "parser"
    assert proposal.impact_scope.value == "crate_global"
    # the planned span must reproduce the analyzed rewrite byte for byte
    source = (crate / "src" / "types.rs").read_bytes()
    patched = (
        source[:operation.start_byte]
        + operation.replacement_text.encode()
        + source[operation.end_byte:]
    )
    assert b"BitfieldStruct" not in patched
    assert b"pub type_0_state: [u8; 4]," in patched


def test_planner_abstains_when_struct_not_lowerable(tmp_path: Path) -> None:
    src = TWO_BLOCK.replace('ty = "c_uint", bits = "31..=31"',
                            'ty = "bool", bits = "31..=31"')
    crate = _crate(tmp_path, src, "pub fn run() {}\n")
    plan = plan_bitfield_lowering(
        crate=crate, hot_function="run",
        hits=[{"rule": "C9", "id": "h1", "extra": {"decl_file": "src/types.rs"}}],
        base_head="0" * 40, candidate_id="test",
    )
    assert plan.proposal is None
    assert "no_lowerable_bitfield_struct" in (plan.abstain_reason or "")
