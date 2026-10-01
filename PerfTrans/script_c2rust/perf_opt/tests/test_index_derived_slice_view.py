"""A slice view whose length is derived from the very index that reads it.

    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, i.wrapping_add(1)) };
    out_slice[i] = gray;

The "length" is invented from the index, so it bounds nothing: it is not the
buffer's size, it only says "at least i+1 elements exist", which the subscript
already assumes. What it does add is a bounds check per access that the
optimiser cannot fold (the `wrapping_add` may wrap, so `i < i + 1` is not
provable). The original raw-pointer store paid none.

Measured on a per-pixel colour-conversion helper that is called once per pixel:
one such rewrite gave its function 31 bounds checks where it had 0, grew it
from 143 to 326 instructions, and made a whole conversion op execute 13.8% more
instructions — while the pipeline's own benchmark, which exercised other
branches of the same function, saw +0.8% and committed it.

The rule card already forbids a view that serves a single access; this check
makes that mechanical, because the model did not follow the card.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt import reporting
from perf_opt.agent_perf_opt.changeset.handlers.replace_function import (
    validate_single_function_source,
)
from perf_opt.agent_perf_opt.changeset.handlers.replace_region import (
    validate_region_replacement_source,
)
from perf_opt.agent_perf_opt.changeset.types import RegionKind
from perf_opt.agent_perf_opt.rewrite_applier import find_index_derived_slice_views


# ───────────────────────────────── the shapes it exists for

def test_single_element_view_is_flagged() -> None:
    code = """
unsafe fn f(out: *mut u8, i: usize, gray: u8) {
    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, i.wrapping_add(1)) };
    out_slice[i] = gray;
}
"""
    assert find_index_derived_slice_views(code)


def test_multi_element_view_at_a_derived_offset_is_flagged() -> None:
    code = """
unsafe fn f(out: *mut u8, i: usize, r: u8, g: u8, b: u8) {
    let base = i.wrapping_mul(3 as size_t);
    let out_slice =
        unsafe { core::slice::from_raw_parts_mut(out, base.wrapping_add(3 as size_t)) };
    out_slice[base] = r;
    out_slice[base.wrapping_add(1 as size_t)] = g;
    out_slice[base.wrapping_add(2 as size_t)] = b;
}
"""
    findings = find_index_derived_slice_views(code)
    assert findings and "out_slice" in findings[0]


def test_plus_spelling_is_flagged() -> None:
    code = """
unsafe fn f(p: *const u8, k: usize) -> u8 {
    let s = unsafe { core::slice::from_raw_parts(p, k + 1) };
    s[k]
}
"""
    assert find_index_derived_slice_views(code)


# ───────────────────────────────── what must NOT be flagged

def test_real_length_with_a_different_index_is_fine() -> None:
    """The zopfli shape: the length is a parameter-derived bound on the whole
    access range, and the reads go through a different index."""
    code = """
unsafe fn f(sublen: *const u16, length: usize) {
    let sublen_slice = unsafe { core::slice::from_raw_parts(sublen, length.wrapping_add(2 as size_t)) };
    let mut i = 3 as size_t;
    while i <= length {
        let curr = *unsafe { sublen_slice.get_unchecked(i) };
        i += 1;
    }
}
"""
    assert not find_index_derived_slice_views(code)


def test_buffer_length_walked_by_a_loop_index_is_fine() -> None:
    code = """
unsafe fn f(buf: *mut u8, len: usize) {
    let s = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    let mut i = 0;
    while i < len {
        s[i] = 0;
        i += 1;
    }
}
"""
    assert not find_index_derived_slice_views(code)


def test_constant_length_is_fine() -> None:
    code = """
unsafe fn f(p: *const u32) -> u32 {
    let t = unsafe { core::slice::from_raw_parts(p, 256) };
    t[3]
}
"""
    assert not find_index_derived_slice_views(code)


def test_a_view_with_other_uses_is_not_flagged() -> None:
    """Only a view that exists SOLELY to be subscripted at its own length's
    base is pointless. One that is also iterated serves a real purpose."""
    code = """
unsafe fn f(p: *mut u8, n: usize) {
    let s = unsafe { core::slice::from_raw_parts_mut(p, n.wrapping_add(1)) };
    s[n] = 0;
    for x in s.iter_mut() { *x += 1; }
}
"""
    assert not find_index_derived_slice_views(code)


def test_raw_pointer_original_is_fine() -> None:
    code = """
unsafe fn f(out: *mut u8, i: usize, gray: u8) {
    *out.offset(i as isize) = gray;
}
"""
    assert not find_index_derived_slice_views(code)


# ───────────────────────────────── wired into both reply checks, as FORM

_BAD_FN = """unsafe fn rgba8ToPixel(out: *mut u8, i: usize, gray: u8) -> u32 {
    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, i.wrapping_add(1)) };
    out_slice[i] = gray;
    return 0;
}
"""


def test_whole_function_reply_is_refused() -> None:
    result = validate_single_function_source(_BAD_FN, "rgba8ToPixel")
    assert not result.ok
    assert result.code == "index_derived_slice_view"


def test_clean_whole_function_reply_still_passes() -> None:
    ok = _BAD_FN.replace(
        "    let out_slice = unsafe { core::slice::from_raw_parts_mut(out, i.wrapping_add(1)) };\n"
        "    out_slice[i] = gray;\n",
        "    *out.offset(i as isize) = gray;\n")
    assert validate_single_function_source(ok, "rgba8ToPixel").ok


def test_region_reply_is_refused() -> None:
    fragment = (
        "let out_slice = unsafe { core::slice::from_raw_parts_mut(out, i.wrapping_add(1)) };\n"
        "out_slice[i] = gray;"
    )
    result = validate_region_replacement_source(RegionKind.NODE_SEQUENCE, fragment)
    assert not result.ok
    assert result.code == "index_derived_slice_view"


def test_the_refusal_is_filed_as_form() -> None:
    assert "index_derived_slice_view" in reporting.MECHANICAL_REJECTION_CODES
    assert reporting.is_mechanical_rejection("index_derived_slice_view: detail")
