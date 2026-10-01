"""C12 cannot answer its own question from the caller alone.

The rule asks whether only a PREFIX of an over-zeroed buffer is ever read.
Once the buffer is passed to another function, that is a fact about the
callee, and the card's abstain list says to give up when the buffer "escapes
... to anything opaque". With only the edited function in the prompt every
callee is opaque, so the rule abstains on precisely the shape it exists for.

Measured on the first crate where C12 fired and reached the model: it skipped
with "buffers escape to opaque callees (`setup_match_struct` and
`match_row`)" — both ordinary same-crate functions, one writing `[0..n)` and
the other reading `[0..m)`, which together are exactly the prefix bound it
said it could not establish.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.prompt_builder import _zero_init_consumer_context


@pytest.fixture()
def crate(tmp_path):
    (tmp_path / "src").mkdir(parents=True)
    (tmp_path / "src" / "m.rs").write_text(
        "unsafe fn fills(p: *mut u8, n: usize) { let _ = (p, n); }\n"
        "unsafe fn reads(p: *const u8, n: usize) { let _ = (p, n); }\n"
        "fn unrelated(x: i32) -> i32 { x }\n",
        encoding="utf-8")
    return tmp_path


CALLER = """
pub unsafe fn caller() {
    let mut buf: [u8; 1024] = [0; 1024];
    fills(&raw mut buf as *mut u8, n);
    let k = unrelated(3);
    reads(&raw const buf as *const u8, n);
}
"""


def test_a_callee_that_receives_a_buffer_is_included(crate) -> None:
    ctx = _zero_init_consumer_context(crate, ["C12"], CALLER)
    assert "callee `fills`" in ctx


def test_a_callee_that_receives_nothing_is_left_out(crate) -> None:
    """Bounded by the question, not by the call count — `unrelated` takes an
    `i32` and cannot say anything about the buffer's live extent."""
    ctx = _zero_init_consumer_context(crate, ["C12"], CALLER)
    assert "unrelated" not in ctx


def test_nothing_is_added_when_c12_did_not_fire(crate) -> None:
    """Every other rule's prompt budget must be untouched."""
    assert _zero_init_consumer_context(crate, ["C1", "C3"], CALLER) == ""


def test_multi_line_calls_are_matched(crate) -> None:
    """c2rust splits calls across lines; a line-wise scan would miss them."""
    src = """
    pub unsafe fn caller() {
        let mut d: [f64; 1024] = [0.; 1024];
        fills(
            &raw mut d as *mut u8,
            n,
        );
    }
    """
    assert "callee `fills`" in _zero_init_consumer_context(crate, ["C12"], src)


def test_keywords_are_not_mistaken_for_calls(crate) -> None:
    src = """
    pub unsafe fn caller() {
        let mut b: [u8; 512] = [0; 512];
        if (&raw mut b) as usize != 0 { }
        while (&raw mut b) as usize != 0 { break; }
        fills(&raw mut b as *mut u8, n);
    }
    """
    ctx = _zero_init_consumer_context(crate, ["C12"], src)
    assert "callee `if`" not in ctx and "callee `while`" not in ctx
    assert "callee `fills`" in ctx


def test_the_callee_count_is_capped(crate) -> None:
    (crate / "src" / "many.rs").write_text(
        "".join(f"unsafe fn f{i}(p: *mut u8) {{ let _ = p; }}\n" for i in range(6)),
        encoding="utf-8")
    src = ("pub unsafe fn caller() { let mut b: [u8; 512] = [0; 512];\n"
           + "".join(f"    f{i}(&raw mut b as *mut u8);\n" for i in range(6))
           + "}\n")
    ctx = _zero_init_consumer_context(crate, ["C12"], src, max_callees=2)
    assert ctx.count("### callee ") == 2


def test_an_unresolvable_callee_is_skipped_not_fatal(crate) -> None:
    src = ("pub unsafe fn caller() { let mut b: [u8; 512] = [0; 512];\n"
           "    memset(&raw mut b as *mut u8, 0, 512);\n}\n")
    assert _zero_init_consumer_context(crate, ["C12"], src) == ""


def test_the_block_tells_the_model_what_to_look_for(crate) -> None:
    """Context without a question is just more tokens."""
    ctx = _zero_init_consumer_context(crate, ["C12"], CALLER)
    assert "which prefix" in ctx and "abstain" in ctx
