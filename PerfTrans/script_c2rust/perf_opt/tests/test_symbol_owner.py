"""Who a profiled symbol belongs to, decided by ownership rather than shape.

A c2rust crate exports its functions under their original C linkage names, so
one profile of ONE crate contains both

    zopfli_raw::src::squeeze::LZ77OptimalRun     ← two `::`
    ZopfliFindLongestMatch                       ← none
    crc32_z                                      ← none

Any test keyed on punctuation calls the last two foreign. That is what the
previous implementation did (`"::" in hot`), and combined with a hardcoded
harness name it reported optipng's own vendored-zlib hotspot, `crc32_z`, as
libc/kernel time — telling the reader the op could not be optimised, about a
function sitting in the crate under test.
"""

from __future__ import annotations

import pytest

from harness_gen.perf_workload import symbol_owner

HARNESS = "optipng_raw_harness"


# ───────────────────────────────────────────────── library code

@pytest.mark.parametrize("symbol", [
    "crc32_z",                                   # C linkage, no `::`
    "ZopfliFindLongestMatch",                    # CamelCase C linkage
    "png_read_filter_row",
    "zopfli_raw::src::squeeze::LZ77OptimalRun",  # Rust path, same crate
    "optipng_0_7_7_raw::src::zlib::deflate::deflate_slow",
])
def test_crate_symbols_are_library(symbol) -> None:
    assert symbol_owner(symbol, HARNESS) == "library"


def test_a_c_linkage_symbol_is_not_downgraded_for_lacking_colons() -> None:
    """The exact regression: punctuation is not evidence of ownership."""
    assert symbol_owner("crc32_z", HARNESS) == "library"
    assert "::" not in "crc32_z"


# ───────────────────────────────────────────────── harness code

def test_harness_symbols_are_harness() -> None:
    assert symbol_owner(f"{HARNESS}::op_zlib_roundtrip", HARNESS) == "harness"


def test_trait_impl_symbols_are_harness_too() -> None:
    """They read `<pkg::Ty as Trait>::method`, so the prefix is not at 0."""
    sym = f"<{HARNESS}::Folder as core::hash::Hasher>::write"
    assert symbol_owner(sym, HARNESS) == "harness"


def test_a_different_harness_name_is_honoured() -> None:
    """The name comes from the manifest, so any project's harness works."""
    assert symbol_owner("zopfli_raw_harness::op_deflate",
                        "zopfli_raw_harness") == "harness"
    # ...and is NOT recognised under someone else's name
    assert symbol_owner("zopfli_raw_harness::op_deflate",
                        "optipng_raw_harness") == "library"


# ───────────────────────────────────────────────── outside the process

@pytest.mark.parametrize("symbol,dso", [
    ("__memset_avx2_erms", "libc-2.31.so"),
    ("__memcpy_avx_unaligned_erms", "libc.so.6"),
    ("memcpy", "libc-2.31.so"),
    ("__pthread_mutex_lock", "libpthread-2.31.so"),
    ("do_syscall_64", "[kernel.kallsyms]"),
])
def test_libc_and_kernel_are_external(symbol, dso) -> None:
    assert symbol_owner(symbol, HARNESS, dso) == "外部"


def test_the_same_bare_name_can_be_either_side_of_the_boundary() -> None:
    """Why the DSO is required and the name can never be enough: a crate that
    vendors zlib defines `crc32_z` itself, and glibc defines `memcpy`. Both are
    bare C symbols; only the object they live in tells them apart."""
    assert symbol_owner("memcpy", HARNESS, "libc-2.31.so") == "外部"
    assert symbol_owner("memcpy", HARNESS, "harness") == "library"


def test_without_a_dso_it_does_not_guess() -> None:
    """Reporting a name-based hunch as fact is how `crc32_z` got called libc.
    The per-DSO self-time share stays the authority on what ran outside."""
    assert symbol_owner("__memset_avx2_erms", HARNESS) == "library"


# ───────────────────────────────────────────────── unresolved

@pytest.mark.parametrize("symbol", ["", "0xffffffff8b9a207c", "0x7f2a1c"])
def test_unresolved_addresses_report_nothing(symbol) -> None:
    """An address is absence of evidence; claiming an owner would be a guess,
    and the caller prints a different remedy for it."""
    assert symbol_owner(symbol, HARNESS) == ""


def test_no_harness_name_still_classifies() -> None:
    """`harness_crate_name` returns '' when the manifest cannot be read; the
    profile must still separate library from libc rather than crash."""
    assert symbol_owner("crc32_z", "", "harness") == "library"
    assert symbol_owner("__memset_avx2_erms", "", "libc-2.31.so") == "外部"


# ───────────────────────────────────────────────── the diagnosis it drives

def _row(self_time=0.41, reliable=True):
    from harness_gen.perf_workload import OpPerf
    r = OpPerf(op="x")
    r.self_time_own = self_time
    r.attribution_reliable = reliable
    return r


def test_a_library_hotspot_is_never_called_libc() -> None:
    """optipng's `gzip_file_roundtrip`: 41% self-time, hottest symbol its own
    vendored `crc32_z`. The old order checked the threshold first and reported
    'most time is OUTSIDE this process', hiding that every in-process cycle
    was library code."""
    from harness_gen.perf_workload import _low_crate_share_cause
    msg = _low_crate_share_cause(_row(), "crc32_z", HARNESS, "harness")
    assert "LIBRARY code" in msg
    assert "59%" in msg, "it should quantify what IS outside the process"


def test_a_real_libc_hotspot_still_says_so() -> None:
    from harness_gen.perf_workload import _low_crate_share_cause
    msg = _low_crate_share_cause(_row(), "__memset_avx2_erms", HARNESS,
                                 "libc-2.31.so")
    assert "libc/kernel" in msg


def test_unreliable_attribution_outranks_every_other_diagnosis() -> None:
    """Without the crate's DWARF the shares are a lower bound, so any story
    told from them may be fiction."""
    from harness_gen.perf_workload import _low_crate_share_cause
    msg = _low_crate_share_cause(_row(reliable=False), "crc32_z", HARNESS)
    assert "UNRELIABLE" in msg


def test_an_unresolved_hotspot_asks_for_symbols_not_a_code_change() -> None:
    from harness_gen.perf_workload import _low_crate_share_cause
    msg = _low_crate_share_cause(_row(), "0xffffffff8b9a207c", HARNESS)
    assert "did not resolve" in msg
