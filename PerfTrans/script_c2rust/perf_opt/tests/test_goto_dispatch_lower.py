"""C11's narrowing rewrite touches integer literals and nothing else.

It renumbers c2rust's goto labels to small dense integers so a comparison no
longer needs a 10-byte `movabs`. Control flow does not move; the rewrite is a
bijection applied to every read and every write of one variable, so it is
value-preserving by construction.

The tests that earn their keep are the ones about *which* literals get renamed.
Two hand-written attempts selected constants by width and both corrupted the
source: `http_parser_execute` arms a second `match` on `(method << 16) |
(index << 8) | ch` whose labels (`196929`, `1311298`, …) are as wide as real
ones, and holds an `i64::MAX` used in a comparison. Renaming by width broke 3 of
its 5 operations. A literal is a label only if it sits in an assignment to the
state variable or in an arm pattern of a `match` on that same variable.
"""

from __future__ import annotations

import re

import pytest

from perf_opt.agent_perf_opt.goto_dispatch_lower import (
    UnsupportedGotoDispatch,
    narrow,
)

BIG_A = 12147880666119273379
BIG_B = 8038949400865391589
BIG_C = 11995618668192240200


def _fn(body: str) -> bytes:
    return ("unsafe fn f() {\n" + body + "\n}\n").encode()


def _narrow(body: str):
    src = _fn(body)
    return src, narrow(src, 1, src.decode().count("\n") + 1)


_SHAPE = f"""
    let mut current_block: u64;
    current_block = {BIG_A};
    loop {{
        match current_block {{
            {BIG_A} => {{ current_block = {BIG_B}; }}
            {BIG_B} => {{ break; }}
            _ => {{ }}
        }}
    }}
"""


# ────────────────────────────────── the rewrite itself

def test_labels_become_dense_from_zero() -> None:
    _, r = _narrow(_SHAPE)
    assert r.var_states == {"current_block": 2}
    assert f"current_block = 0;" in r.text
    assert f"current_block = 1;" in r.text
    assert str(BIG_A) not in r.text and str(BIG_B) not in r.text


def test_the_carrier_is_narrowed() -> None:
    _, r = _narrow(_SHAPE)
    assert "let mut current_block: u32;" in r.text
    assert r.types_narrowed == 1


def test_every_site_of_a_label_moves_together() -> None:
    """A bijection or nothing: an assignment renamed while its arm is not would
    route the state machine somewhere else entirely."""
    _, r = _narrow(_SHAPE)
    assert r.text.count("current_block = 0;") == 1
    assert re.search(r"^\s*0 =>", r.text, re.M)
    assert re.search(r"^\s*1 =>", r.text, re.M)


def test_only_numbers_and_the_type_change() -> None:
    src, r = _narrow(_SHAPE)
    before = re.sub(r"\b\d+\b", "#", src.decode()).replace("u64", "uN")
    after = re.sub(r"\b\d+\b", "#", r.text).replace("u32", "uN")
    assert before == after


def test_line_count_is_preserved() -> None:
    """Nothing moves, so a downstream line-based anchor stays valid."""
    src, r = _narrow(_SHAPE)
    assert src.decode().count("\n") == r.text.count("\n")


# ────────────────────────────────── what must NOT be renamed

def test_a_match_on_another_variable_is_left_alone() -> None:
    """The http-parser hazard, exactly."""
    src, r = _narrow(f"""
    let mut current_block: u64;
    current_block = {BIG_A};
    current_block = {BIG_B};
    match (method << 16) | (index << 8) | ch {{
        196929 => {{ set_method(1); }}
        1311298 => {{ set_method(2); }}
        _ => {{ }}
    }}
    match current_block {{ {BIG_A} => {{ }} _ => {{ }} }}
""")
    assert "196929 => " in r.text and "1311298 => " in r.text


def test_an_integer_in_an_arm_body_is_program_data() -> None:
    _, r = _narrow(f"""
    let mut current_block: u64;
    current_block = {BIG_A};
    current_block = {BIG_B};
    match current_block {{
        {BIG_A} => {{ let lim = 9223372036854775807; }}
        _ => {{ }}
    }}
""")
    assert "9223372036854775807" in r.text


def test_an_unrelated_wide_constant_survives() -> None:
    _, r = _narrow(f"""
    let mut current_block: u64;
    current_block = {BIG_A};
    current_block = {BIG_B};
    let mask = 18446744073709551615;
    match current_block {{ {BIG_A} => {{ }} _ => {{ }} }}
""")
    assert "18446744073709551615" in r.text


# ────────────────────────────────── several state variables

def test_each_variable_gets_its_own_namespace() -> None:
    """`http_parser_execute` carries `current_block` and `current_block_938`.
    Numbering them from a shared counter would be correct but unreadable;
    numbering them from the same base would collide only if a future change
    ever compared one against the other."""
    _, r = _narrow(f"""
    let mut current_block: u64;
    let mut current_block_938: u64;
    current_block = {BIG_A};
    current_block = {BIG_B};
    current_block_938 = {BIG_C};
    current_block_938 = 17999999999999999999;
    match current_block {{ {BIG_A} => {{ }} _ => {{ }} }}
    match current_block_938 {{ {BIG_C} => {{ }} _ => {{ }} }}
""")
    assert r.var_states == {"current_block": 2, "current_block_938": 2}
    assert "current_block = 0;" in r.text
    assert "current_block_938 = 1000;" in r.text
    assert r.types_narrowed == 2


# ────────────────────────────────── abstentions

def test_a_single_label_is_a_flag_not_a_dispatch() -> None:
    with pytest.raises(UnsupportedGotoDispatch):
        _narrow(f"""
    let mut current_block: u64;
    current_block = {BIG_A};
    match current_block {{ {BIG_A} => {{ }} _ => {{ }} }}
""")


def test_labels_that_already_fit_an_imm32_are_left_alone() -> None:
    """`cmp $0x3, %rax` needs no `movabs`. Rewriting here changes bytes and
    buys nothing, which costs a W2 measurement for a guaranteed no-op."""
    with pytest.raises(UnsupportedGotoDispatch):
        _narrow("""
    let mut current_block: u32;
    current_block = 3;
    current_block = 4;
    match current_block { 3 => { } 4 => { } _ => { } }
""")


def test_one_wide_label_is_not_a_comparison_chain() -> None:
    with pytest.raises(UnsupportedGotoDispatch):
        _narrow(f"""
    let mut current_block: u64;
    current_block = {BIG_A};
    current_block = 4;
    match current_block {{ {BIG_A} => {{ }} 4 => {{ }} _ => {{ }} }}
""")


def test_a_function_without_the_shape_abstains() -> None:
    with pytest.raises(UnsupportedGotoDispatch):
        _narrow("    let x = 1; loop { if x > 2 { break; } }")


# ────────────────────────────────── determinism

def test_the_rewrite_is_deterministic() -> None:
    """Same input, byte-identical output — the numbering follows source order,
    not the CST walk's LIFO stack order."""
    src = _fn(_SHAPE)
    a = narrow(src, 1, src.decode().count("\n") + 1).text
    b = narrow(src, 1, src.decode().count("\n") + 1).text
    assert a == b


def test_numbering_follows_source_order() -> None:
    """Label 0 is the first one the reader meets, so a diff can be eyeballed."""
    _, r = _narrow(f"""
    let mut current_block: u64;
    current_block = {BIG_C};
    current_block = {BIG_A};
    current_block = {BIG_B};
    match current_block {{ {BIG_C} => {{ }} {BIG_A} => {{ }} _ => {{ }} }}
""")
    lines = [l.strip() for l in r.text.split("\n") if "current_block = " in l]
    assert lines[:3] == ["current_block = 0;", "current_block = 1;",
                         "current_block = 2;"]


# ────────────────────────────────── or-patterns

def test_or_patterns_are_renamed_on_both_sides() -> None:
    _, r = _narrow(f"""
    let mut current_block: u64;
    current_block = {BIG_A};
    current_block = {BIG_B};
    match current_block {{ {BIG_A} | {BIG_B} => {{ }} _ => {{ }} }}
""")
    assert str(BIG_A) not in r.text and str(BIG_B) not in r.text
    assert re.search(r"0 \| 1 =>", r.text)
