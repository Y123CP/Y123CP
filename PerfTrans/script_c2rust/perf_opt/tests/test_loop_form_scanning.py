"""Byte-serial loops written as `loop { … }` must be found too.

c2rust lowers C's `do { … } while (cond)` to `loop { … if !cond { break } }`.
That is the shape C uses for exactly the code C4/C5/C6 target — LZ match
extension, bit-stream refills, unrolled scans — so a scanner keyed on
`while_expression` is blind to the whole class.

Measured on a vendored zlib: the loops in the two hottest optimisable
functions (48.6% and 29.6% self-time) are 100% `loop`, and C4 reported zero
hits on a crate whose hottest function is a textbook byte-serial match loop.
Hand-applying the rule to that loop measured -6.88% on the compression op with
the golden replay staying 2404/2404 green — the rule was right, the scanner
just never saw the code.

`for_expression` stays excluded on purpose: c2rust rarely emits it, and where
it does the cursor is an iterator rather than a raw pointer, so the shape these
rules exist to fix is absent.
"""

from __future__ import annotations

import pytest

from perf_opt.hot_probe.merged_hits import (
    _LOOP_SCAN_TYPES,
    _wa_is_byte_cursor_loop,
)


def _first_loop(src: str):
    import tree_sitter
    import tree_sitter_rust
    lang = tree_sitter.Language(tree_sitter_rust.language())
    data = src.encode()
    tree = tree_sitter.Parser(lang).parse(data)
    stack = [tree.root_node]
    while stack:
        node = stack.pop()
        if node.type in ("while_expression", "loop_expression"):
            return node, data
        stack.extend(node.children)
    raise AssertionError("no loop in fixture")


# ───────────────────────────────────────────── which node types are scanned

def test_loop_expression_is_scanned() -> None:
    assert "loop_expression" in _LOOP_SCAN_TYPES


def test_while_expression_is_still_scanned() -> None:
    assert "while_expression" in _LOOP_SCAN_TYPES


def test_for_expression_is_not_scanned() -> None:
    """Its cursor is already an iterator — nothing for these rules to fix."""
    assert "for_expression" not in _LOOP_SCAN_TYPES


# ───────────────────────────────────────────── the do-while shape is detected

UNROLLED_MATCH_LOOP = """
unsafe fn f(mut p: *mut u8, mut q: *mut u8, end: *mut u8) {
    loop {
        p = p.offset(1);
        q = q.offset(1);
        if !(*p as i32 == *q as i32
            && { p = p.offset(1); q = q.offset(1); *p as i32 == *q as i32 }
            && { p = p.offset(1); q = q.offset(1); *p as i32 == *q as i32 }
            && p < end)
        {
            break;
        }
    }
}
"""


def test_an_unrolled_do_while_compare_chain_is_a_byte_cursor_loop() -> None:
    """The exact shape C compilers use for LZ match extension."""
    node, data = _first_loop(UNROLLED_MATCH_LOOP)
    assert node.type == "loop_expression"
    assert _wa_is_byte_cursor_loop(node, data)


def test_a_plain_while_byte_loop_still_matches() -> None:
    src = """
unsafe fn f(mut p: *mut u8, mut q: *mut u8, end: *mut u8) {
    while p != end && *p as i32 == *q as i32 {
        p = p.offset(1);
        q = q.offset(1);
    }
}
"""
    node, data = _first_loop(src)
    assert _wa_is_byte_cursor_loop(node, data)


# ───────────────────────────── strictness is unchanged: no new false positives

def test_a_counter_loop_is_not_matched() -> None:
    """Widening it would be meaningless — nothing here reads bytes."""
    src = """
unsafe fn f(mut i: usize, n: usize, acc: *mut u64) {
    loop {
        i += 1;
        if !(i < n) { break; }
        *acc += 1;
    }
}
"""
    node, data = _first_loop(src)
    assert not _wa_is_byte_cursor_loop(node, data)


def test_a_field_compare_loop_is_not_matched() -> None:
    """`(*s).bits >= 8` is a bit-buffer refill, not a byte cursor — the
    predicate demands a BARE deref so this stays out."""
    src = """
unsafe fn f(s: *mut S, mut p: *mut u8) {
    loop {
        p = p.offset(1);
        if !((*s).bits >= 8) { break; }
    }
}
"""
    node, data = _first_loop(src)
    assert not _wa_is_byte_cursor_loop(node, data)


def test_an_already_widened_loop_is_skipped() -> None:
    """Idempotence: re-applying the rule to its own output must be a no-op."""
    src = """
unsafe fn f(mut p: *mut u8, mut q: *mut u8, end: *mut u8) {
    loop {
        let a = ::core::ptr::read_unaligned(p as *const u64);
        if a != 0 { break; }
        p = p.offset(8);
        q = q.offset(8);
        if p >= end { break; }
    }
}
"""
    node, data = _first_loop(src)
    assert not _wa_is_byte_cursor_loop(node, data)


def test_a_loop_without_pointer_stepping_is_not_matched() -> None:
    src = """
unsafe fn f(p: *mut u8, q: *mut u8) {
    loop {
        if !(*p as i32 == *q as i32) { break; }
    }
}
"""
    node, data = _first_loop(src)
    assert not _wa_is_byte_cursor_loop(node, data)


# ───────────────────────────────────────────── the card documents the shape

def test_the_card_documents_the_do_while_shape() -> None:
    """The scanner finding the loop is useless if the model cannot recognise
    it: the card's only template used to be the `while` form."""
    from pathlib import Path
    card = (Path(__file__).resolve().parents[1] / "agent_perf_opt"
            / "Optimization_Card" / "C4_word_at_a_time.md").read_text()
    assert "do { … } while (cond)" in card
    assert "loop {" in card
    # the three equivalence pitfalls a rewrite must respect
    assert "Keep the `+1`" in card
    assert "Stop ON the differing byte" in card
    assert "Test the bound AFTER advancing" in card


def test_the_card_does_not_license_an_unguarded_word_read() -> None:
    """The template must carry the bounds guard, and must say why.

    The unrolled chain is SHORT-CIRCUIT — a mismatch in its first arm reads
    one byte — so "the original already reads 8 bytes" is false and cannot
    justify an unguarded `u64`. An unguarded rewrite of a vendored zlib passed
    the golden replay only because that window is over-allocated by ~32KB at
    the default `w_size`; at the smallest legal window the slack is negative.
    Passing the oracle is not the same as being in bounds.
    """
    from pathlib import Path
    card = (Path(__file__).resolve().parents[1] / "agent_perf_opt"
            / "Optimization_Card" / "C4_word_at_a_time.md").read_text()
    assert "does NOT license" in card
    assert "wrapping_sub(p as usize) < 8" in card        # the guard itself
    assert "short-circuit" in card.lower()
    assert "soft cap" in card                            # what `end` usually is


def test_the_card_gates_the_word_loop_on_the_first_element() -> None:
    """The template must enter the word loop only after the first element
    matched, and must say why.

    An ungated word loop costs about a dozen instructions per entry even when
    the first byte already differs. Inside a candidate walk that is entered
    once per probe, and on repetitive input nearly every probe fails on its
    first byte right after a run-skip step: an ungated rewrite measured +14%
    instructions on such an input while winning −26% wall on another, and a
    single benchmark input only ever showed the win.
    """
    from pathlib import Path
    card = (Path(__file__).resolve().parents[1] / "agent_perf_opt"
            / "Optimization_Card" / "C4_word_at_a_time.md").read_text()
    template = card.split("### 2.1", 1)[1].split("### 2.2", 1)[0]
    assert "if fore != end && *back == *fore {" in template   # the gate
    assert template.index("if fore != end && *back == *fore {") \
        < template.index("read_unaligned")                    # before the word read
    assert "Entry cost" in card and "mandatory" in card       # and the reason
    assert "only adds a branch" not in card                   # the old, false claim
