"""C11's loop split takes the dispatch off a loop's back edge.

    loop {                            'outer: loop {
        match v {                         if v == L {
            L => { BODY }        ->           'fast: loop { BODY break 'fast; }
            _ => { REST }                 } else { REST }
        }
        TAIL                              TAIL
    }                                 }

The trailing `break 'fast;` reproduces the original edge — fall off the end of
the arm, continue past the dispatch — so the rewrite needs no argument about
which states can be live at the exit.

The hazard is `break` and `continue`: inside the arm they targeted the merged
loop, and once the arm is a loop of its own they would target that instead.
Only the ones at the state machine's own nesting level may be relabelled. An arm
body can hold its own `while`, and a `break` in there belongs to the `while` —
relabelling it is a silent semantic change a replay corpus need not reach.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.goto_dispatch_lower import (
    UnsupportedGotoDispatch,
    split_loop_head,
)

L = 12147880666119273379
M = 8038949400865391589


def _fn(body: str) -> bytes:
    return ("unsafe fn f() {\n" + body + "\n}\n").encode()


def _split(body: str):
    src = _fn(body)
    return split_loop_head(src, 1, src.decode().count("\n") + 1)


_SHAPE = f"""
    let mut current_block: u64;
    current_block = {L};
    loop {{
        match current_block {{
            {L} => {{
                step();
                current_block = {L};
                continue;
            }}
            _ => {{
                other();
            }}
        }}
        tail();
    }}
"""


# ────────────────────────────────── the rewrite

def test_the_loop_and_arm_become_their_own_loops() -> None:
    r = _split(_SHAPE)
    assert "'outer: loop {" in r.text
    assert "'fast: loop {" in r.text
    assert f"if current_block == {L} {{" in r.text
    assert "} else {" in r.text


def test_the_arm_ends_by_leaving_its_loop() -> None:
    """Falling off the end must exit, not iterate — that is the original edge."""
    assert "break 'fast;" in _split(_SHAPE).text


def test_a_hot_back_edge_shortcuts_to_the_fast_loop() -> None:
    r = _split(_SHAPE)
    assert "continue 'fast;" in r.text
    assert r.relabelled_continues == 1


def test_braces_stay_balanced() -> None:
    r = _split(_SHAPE)
    assert r.text.count("{") == r.text.count("}")


def test_the_match_keyword_is_gone() -> None:
    assert "match current_block" not in _split(_SHAPE).text


def test_the_label_is_reported() -> None:
    assert _split(_SHAPE).label == L


# ────────────────────────────────── break / continue targeting

def test_a_bare_break_leaves_the_whole_machine() -> None:
    r = _split(f"""
    let mut current_block: u64;
    loop {{
        match current_block {{
            {L} => {{ if bad() {{ break; }} }}
            _ => {{ }}
        }}
    }}
""")
    assert "break 'outer;" in r.text
    assert r.relabelled_breaks == 1


def test_a_break_inside_an_inner_loop_is_left_alone() -> None:
    """It belongs to the `while`. Relabelling it sends control somewhere the
    original never went, and the correctness gate need not reach that path."""
    r = _split(f"""
    let mut current_block: u64;
    loop {{
        match current_block {{
            {L} => {{
                while more() {{
                    if done() {{ break; }}
                    step();
                }}
            }}
            _ => {{ }}
        }}
    }}
""")
    assert r.relabelled_breaks == 0
    assert "break 'outer" not in r.text
    assert "break;" in r.text


def test_a_continue_inside_an_inner_loop_is_left_alone() -> None:
    r = _split(f"""
    let mut current_block: u64;
    loop {{
        match current_block {{
            {L} => {{ while more() {{ continue; }} }}
            _ => {{ }}
        }}
    }}
""")
    assert r.relabelled_continues == 0
    assert "continue;" in r.text


def test_an_already_labelled_jump_is_left_alone() -> None:
    r = _split(f"""
    let mut current_block: u64;
    'up: loop {{
        loop {{
            match current_block {{
                {L} => {{ break 'up; }}
                _ => {{ }}
            }}
        }}
    }}
""")
    assert "break 'up;" in r.text
    assert r.relabelled_breaks == 0


def test_a_continue_after_another_state_goes_round_the_dispatch() -> None:
    """Only a `continue` that follows `v = <hot label>` may shortcut. After any
    other state the original re-entered the dispatch and could land in the other
    arm; `continue 'outer` is the only faithful target."""
    r = _split(f"""
    let mut current_block: u64;
    loop {{
        match current_block {{
            {L} => {{ current_block = {M}; continue; }}
            _ => {{ }}
        }}
    }}
""")
    assert "continue 'outer;" in r.text
    assert "continue 'fast" not in r.text


# ────────────────────────────────── abstentions

def test_a_labelled_loop_is_declined() -> None:
    """Rewriting it would need the existing label kept consistent; not worth
    guessing."""
    with pytest.raises(UnsupportedGotoDispatch):
        _split(f"""
    let mut current_block: u64;
    'already: loop {{
        match current_block {{ {L} => {{ }} _ => {{ }} }}
    }}
""")


def test_more_than_two_arms_is_declined() -> None:
    with pytest.raises(UnsupportedGotoDispatch):
        _split(f"""
    let mut current_block: u64;
    loop {{
        match current_block {{ {L} => {{ }} {M} => {{ }} _ => {{ }} }}
    }}
""")


def test_a_dispatch_below_the_loop_head_is_declined() -> None:
    with pytest.raises(UnsupportedGotoDispatch):
        _split(f"""
    let mut current_block: u64;
    loop {{
        prologue();
        match current_block {{ {L} => {{ }} _ => {{ }} }}
    }}
""")


def test_a_match_on_another_variable_is_declined() -> None:
    with pytest.raises(UnsupportedGotoDispatch):
        _split(f"""
    let mut current_block: u64;
    loop {{
        match other {{ {L} => {{ }} _ => {{ }} }}
    }}
""")


def test_a_function_without_a_loop_dispatch_is_declined() -> None:
    with pytest.raises(UnsupportedGotoDispatch):
        _split("    let x = 1; loop { if x > 2 { break; } }")


def test_an_or_pattern_arm_is_declined() -> None:
    """Two labels behind one arm means the `if` condition is not a single
    equality; expressing it is possible but is not this rewrite."""
    with pytest.raises(UnsupportedGotoDispatch):
        _split(f"""
    let mut current_block: u64;
    loop {{
        match current_block {{ {L} | {M} => {{ }} _ => {{ }} }}
    }}
""")


# ────────────────────────────────── output hygiene

def test_no_blank_line_is_left_where_the_match_closed() -> None:
    r = _split(_SHAPE)
    assert "\n\n\n" not in r.text


def test_line_count_shrinks_by_exactly_the_deleted_brace_line() -> None:
    src = _fn(_SHAPE)
    r = split_loop_head(src, 1, src.decode().count("\n") + 1)
    assert r.text.count("\n") == src.decode().count("\n")
