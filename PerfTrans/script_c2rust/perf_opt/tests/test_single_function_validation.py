""""Exactly one function" is a question about the item tree, not about text.

Counting `fn` tokens answers a different question, and the difference is not
academic. On optipng the model returned a correct rewrite of
`opng_strparse_rangeset_to_bitset` — the crate's hottest function, 99.75%
self-time in its op — with one helper nested inside the body:

    #[no_mangle]
    pub unsafe extern "C" fn opng_strparse_rangeset_to_bitset(...) -> c_int {
        #[inline(always)]
        fn is_space_byte(b: u8) -> bool { matches!(b, b' ' | b'\\t' | ...) }
        ...
    }

Nesting a helper is ordinary Rust. The token counter saw two `fn`s, abstained,
and the planner never returned to that function: the single largest target of
the run, lost to a miscount. A `fn` token also appears in a function-pointer
type, an `impl` method, and an `extern "C" { … }` declaration — the last of
which had already been special-cased once, in text, after an earlier incident.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.changeset.handlers.replace_function import (
    validate_single_function_source as validate,
)


# ─────────────────────────────────────────── accepted: one top-level function

def test_a_nested_helper_is_not_a_second_function() -> None:
    """The regression, reduced. This is the whole reason this file exists."""
    src = """
#[no_mangle]
pub unsafe extern "C" fn parse(out: *mut u32) -> i32 {
    #[inline(always)]
    fn is_space_byte(b: u8) -> bool { matches!(b, b' ' | b'\\t') }
    if is_space_byte(b' ') { *out = 1; }
    0
}
"""
    assert validate(src, "parse").ok


def test_a_function_pointer_type_is_not_a_function() -> None:
    assert validate("pub fn f(cb: fn(u8) -> bool) { let _ = cb; }", "f").ok


def test_a_closure_with_a_fn_bound_is_not_a_function() -> None:
    src = "pub fn f() { let g: &dyn Fn(u8) -> bool = &|b| b == 0; let _ = g; }"
    assert validate(src, "f").ok


def test_an_extern_declaration_block_is_allowed_alongside() -> None:
    """Rule C7 legitimately adds one; it declares, it does not define."""
    src = ('extern "C" { fn malloc_usable_size(p: *mut u8) -> usize; }\n'
           "pub unsafe extern \"C\" fn f() { let _ = 1; }")
    assert validate(src, "f").ok


def test_attributes_and_comments_may_precede_the_function() -> None:
    src = "// note\n/* block */\n#[no_mangle]\npub fn f() {}"
    assert validate(src, "f").ok


# ─────────────────────────────────────────── byte offsets vs character offsets

def test_multibyte_characters_do_not_shift_the_name() -> None:
    """tree-sitter reports BYTE offsets; indexing the str with them shifts on
    the first multi-byte character. The rewrite header always carries some —
    the rule ids are literally `III④` / `III③` — so this is the common path,
    not an edge case. It produced `parse_rangeset_to_bitset(\\n    mu` as the
    "function name" and turned the fix into a different rejection."""
    src = "// Applied rules: [III④, III③]\npub fn f() { let s = \"→\"; let _ = s; }"
    assert validate(src, "f").ok


def test_multibyte_source_still_detects_a_wrong_name() -> None:
    src = "// rules: [III④]\npub fn g() {}"
    result = validate(src, "f")
    assert not result.ok
    assert result.code == "replacement_function_name"


# ─────────────────────────────────────────── rejected: what the gate is for

def test_two_top_level_functions_are_rejected() -> None:
    result = validate("pub fn f() {}\npub fn g() {}", "f")
    assert not result.ok
    assert result.code == "replacement_function_count"


def test_zero_functions_are_rejected() -> None:
    assert not validate("static X: u8 = 1;", "f").ok


def test_a_wrong_name_is_rejected() -> None:
    result = validate("pub fn g() {}", "f")
    assert not result.ok
    assert result.code == "replacement_function_name"


def test_trailing_source_after_the_function_is_rejected() -> None:
    """The replacement is spliced into a fn span; anything after it would be
    duplicated into the file."""
    result = validate("pub fn f() {}\nstatic X: u8 = 1;", "f")
    assert not result.ok
    assert result.code == "replacement_extra_source"


def test_an_impl_block_alongside_is_rejected() -> None:
    result = validate("pub fn f() {}\nimpl T { fn m(&self) {} }", "f")
    assert not result.ok
    assert result.code == "replacement_extra_source"


@pytest.mark.parametrize("src", [
    "pub fn f() { let x = ; }",          # missing expression
    "pub fn f() { ",                     # unclosed body
    "pub fn f( { }",                     # broken parameter list
])
def test_unparsable_replacements_are_rejected(src) -> None:
    """Previously a hand-rolled brace matcher decided this, with no handling
    for raw strings or lifetimes; the parser answers it directly."""
    result = validate(src, "f")
    assert not result.ok
    assert result.code == "replacement_parse_error"


def test_a_raw_string_containing_braces_does_not_confuse_the_end_of_body() -> None:
    """The retired brace matcher tracked normal strings but not `r#"…"#`, so a
    brace inside one would have ended the body early."""
    src = 'pub fn f() { let s = r#"} not the end {"#; let _ = s; }'
    assert validate(src, "f").ok


# ──────────────────── tree-sitter's cast-vs-generic ambiguity

def test_a_c2rust_style_cast_comparison_is_accepted() -> None:
    """`x as SomeName < y` is ambiguous to a parser that has not resolved
    names — `SomeName<...>` could open a generic argument list — so
    tree-sitter reports the `<` as an ERROR. rustc accepts it (verified: 0
    errors on the exact line this was first seen on).

    Rejecting on it is catastrophic rather than merely unlucky here, because
    c2rust names every C scalar with an alias (`c_int`, `c_double`, `c_uint`).
    `as c_double <= m` is ordinary translated code. Two rewrites of a
    compression crate were thrown away as "not syntactically valid Rust"
    while being perfectly valid.
    """
    src = ("unsafe fn f(x: *const c_double, m: c_double) -> bool "
           "{ !(*x as c_double <= m) }")
    assert validate(src, "f").ok


@pytest.mark.parametrize("op", ["<", "<=", ">", ">="])
def test_every_comparison_operator_after_a_cast_is_accepted(op) -> None:
    src = f"unsafe fn f(x: *const c_int, m: c_int) -> bool {{ *x as c_int {op} m }}"
    assert validate(src, "f").ok


def test_a_builtin_type_cast_was_never_affected() -> None:
    """Baseline: `f64` cannot take generic arguments, so tree-sitter never
    guessed wrong here. Kept so a future parser change is visible."""
    src = "unsafe fn f(x: *const f64, m: f64) -> bool { *x as f64 <= m }"
    assert validate(src, "f").ok


def test_real_syntax_errors_are_still_rejected() -> None:
    """The relaxation is scoped to bare comparison operators; anything that
    spans real text still fails, so the gate keeps its purpose."""
    for src in ("fn f() { let x = ; }", "fn f() { ", "fn f( { }"):
        result = validate(src, "f")
        assert not result.ok
        assert result.code == "replacement_parse_error"


def test_a_cast_ambiguity_does_not_mask_a_second_real_error() -> None:
    """All ERROR nodes must be the benign kind — one genuine error alongside
    the ambiguity still rejects."""
    src = ("unsafe fn f(x: *const c_int, m: c_int) -> bool "
           "{ let y = ; *x as c_int <= m }")
    assert not validate(src, "f").ok
