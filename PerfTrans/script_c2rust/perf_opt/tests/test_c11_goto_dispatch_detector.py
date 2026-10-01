"""C11 detects c2rust's goto lowering, and nothing that merely looks like it.

C has `goto`; Rust does not. c2rust lowers a function's gotos into
`let mut current_block: u64` plus `match current_block { <const> => .. }`
blocks. The straight-line path then pays what the C never did.

Two costs, two patterns. `goto-dispatch-wide-constants` is the 10-byte `movabs`
each random 64-bit label needs to be compared against.
`goto-dispatch-loop-head` is a dispatch on a loop's back edge, re-run every
iteration.

The tests that matter here are the negative ones. A rename that reached a
non-label integer broke 3 of 5 operations on http-parser, whose
`http_parser_execute` arms a second `match` on `(method << 16) | (index << 8)
| ch` with labels the same width as real ones.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from perf_opt.hot_probe.merged_hits import (
    _c11_arm_constants,
    _c11_heads_a_loop,
    _c11_int,
    find_goto_dispatch_hits,
)


class _Index:
    """Minimal fn_index: one function spanning the whole file."""

    def __init__(self, rel: str, lines: int) -> None:
        self._rel, self._lines = rel, lines

    def resolve(self, fn):
        return (self._rel, 1, self._lines) if fn == "f" else None


def _hits(tmp_path: Path, body: str):
    src = "unsafe fn f() {\n" + body + "\n}\n"
    (tmp_path / "m.rs").write_text(src)
    idx = _Index("m.rs", src.count("\n") + 1)
    return find_goto_dispatch_hits(tmp_path, {"f"}, idx).get("f", [])


def _patterns(hits):
    return {h.pattern.replace("goto-dispatch-", "") for h in hits}


_SHAPE = """
    let mut current_block: u64;
    current_block = 12147880666119273379;
    loop {
        match current_block {
            12147880666119273379 => { work(); }
            8038949400865391589 => { break; }
            _ => { }
        }
        tail();
    }
"""


# ────────────────────────────────── the shape itself

def test_the_c2rust_shape_is_detected(tmp_path) -> None:
    hits = _hits(tmp_path, _SHAPE)
    assert _patterns(hits) == {"wide-constants", "loop-head"}


def test_the_counts_are_reported(tmp_path) -> None:
    e = _hits(tmp_path, _SHAPE)[0].extra
    assert e["var"] == "current_block"
    assert e["states"] == 2
    assert e["wide_states"] == 2
    assert e["dispatch_sites"] == 1
    assert e["loop_head_dispatches"] == 1


def test_the_rule_id_is_c11(tmp_path) -> None:
    assert {h.rule for h in _hits(tmp_path, _SHAPE)} == {"C11"}


# ────────────────────────────────── negatives that protect a rename

def test_a_match_on_another_variable_is_not_a_dispatch(tmp_path) -> None:
    """`http_parser_execute` arms a `match` on a packed method/index/char.
    Its labels are ordinary program data; a rename reaching them broke 3 of 5
    operations. The state count must stay 2 — the foreign arms must not be
    counted, and must not reach the rename list."""
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    current_block = 2222222222222222222;
    match (method << 16) | (index << 8) | ch {
        196929 => { set_method(PATCH); }
        1311298 => { set_method(PURGE); }
        _ => { }
    }
    match current_block { 1111111111111111111 => { } _ => { } }
""")
    assert len(hits) == 1
    e = hits[0].extra
    assert e["states"] == 2, e
    assert e["dispatch_sites"] == 1, "only the match on current_block counts"


def test_integers_in_an_arm_body_are_not_labels(tmp_path) -> None:
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 111111111111111111;
    current_block = 222222222222222222;
    match current_block {
        111111111111111111 => { let x = 9223372036854775807; }
        _ => { }
    }
""")
    assert hits[0].extra["states"] == 2, "the i64::MAX in the body is not a label"


def test_a_single_state_is_a_flag_not_a_dispatch(tmp_path) -> None:
    assert _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 12147880666119273379;
    match current_block { 12147880666119273379 => { } _ => { } }
""") == []


def test_no_match_means_nothing_is_routed(tmp_path) -> None:
    assert _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    current_block = 2222222222222222222;
""") == []


def test_a_function_without_the_shape_is_untouched(tmp_path) -> None:
    assert _hits(tmp_path, "    let x = 1; loop { if x > 2 { break; } }") == []


# ────────────────────────────────── shapes the scan must still see

def test_or_patterns_count_both_labels(tmp_path) -> None:
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    match current_block {
        1111111111111111111 | 2222222222222222222 => { }
        _ => { }
    }
""")
    assert hits and hits[0].extra["states"] == 2


def test_several_state_variables_are_reported_separately(tmp_path) -> None:
    """`http_parser_execute` carries `current_block` and `current_block_938`.
    They are separate namespaces; merging them invents one bogus dispatch."""
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    let mut current_block_938: u64;
    current_block = 1111111111111111111;
    current_block = 2222222222222222222;
    current_block_938 = 3333333333333333333;
    current_block_938 = 4444444444444444444;
    match current_block { 1111111111111111111 => { } _ => { } }
    match current_block_938 { 3333333333333333333 => { } _ => { } }
""")
    assert {h.extra["var"] for h in hits} == {"current_block", "current_block_938"}
    assert all(h.extra["states"] == 2 for h in hits)


# ────────────────────────────────── loop-head is about the back edge

def test_a_dispatch_below_the_loop_head_is_not_on_the_back_edge(tmp_path) -> None:
    """Paid once per pass through that point — which is what the C `goto` paid
    too, so there is nothing to recover. `http_parser_execute`'s 95-arm
    dispatch sits here, after an inner labelled loop closes."""
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    loop {
        prologue();
        match current_block {
            1111111111111111111 => { }
            2222222222222222222 => { }
            _ => { }
        }
    }
""")
    assert _patterns(hits) == {"wide-constants"}
    assert hits[0].extra["loop_head_dispatches"] == 0


def test_a_dispatch_outside_any_loop_is_not_on_a_back_edge(tmp_path) -> None:
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    current_block = 2222222222222222222;
    match current_block { 1111111111111111111 => { } _ => { } }
""")
    assert _patterns(hits) == {"wide-constants"}


def test_nested_loops_each_count_their_own_head(tmp_path) -> None:
    """miniz `tinfl_decompress` has two, one per loop nest."""
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    loop {
        match current_block {
            1111111111111111111 => { }
            2222222222222222222 => { }
            _ => { }
        }
        loop {
            match current_block {
                1111111111111111111 => { }
                _ => { }
            }
        }
    }
""")
    assert hits[0].extra["loop_head_dispatches"] == 2


def test_a_while_head_counts_too(tmp_path) -> None:
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    while going() {
        match current_block {
            1111111111111111111 => { }
            2222222222222222222 => { }
            _ => { }
        }
    }
""")
    assert "loop-head" in _patterns(hits)


# ────────────────────────────────── narrow labels cost no movabs

def test_labels_that_fit_an_imm32_raise_no_wide_pattern(tmp_path) -> None:
    """A `cmp $0x3, %rax` needs no `movabs`. Renaming buys nothing there — the
    pattern must not fire, or the pipeline burns a W2 on a no-op."""
    hits = _hits(tmp_path, """
    let mut current_block: u32;
    current_block = 3;
    loop {
        match current_block { 3 => { } 4 => { } _ => { } }
    }
""")
    assert _patterns(hits) == {"loop-head"}
    assert hits[0].extra["wide_states"] == 0


def test_one_wide_label_is_not_a_comparison_chain(tmp_path) -> None:
    hits = _hits(tmp_path, """
    let mut current_block: u64;
    current_block = 1111111111111111111;
    current_block = 4;
    match current_block { 1111111111111111111 => { } 4 => { } _ => { } }
""")
    assert _patterns(hits) == set()


# ────────────────────────────────── literal parsing

@pytest.mark.parametrize("text,want", [
    (b"12147880666119273379", 12147880666119273379),
    (b"0xff", 255),
    (b"1_000", 1000),
    (b"0b1010", 10),
    (b"0o17", 15),
])
def test_integer_literals_parse(text, want, tmp_path) -> None:
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    tree = _parse_rust_source(b"fn f(){ let x = " + text + b"; }")
    stack = [tree.root_node]
    lit = None
    while stack:
        n = stack.pop()
        if n.type == "integer_literal":
            lit = n
            break
        stack.extend(n.children)
    assert lit is not None
    assert _c11_int(lit, b"fn f(){ let x = " + text + b"; }") == want
