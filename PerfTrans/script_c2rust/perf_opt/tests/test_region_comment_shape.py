"""A comment is part of a rewrite, not a defect in it.

The region shape gate asked whether every NAMED child of the replacement
classifies as a statement. A comment is a named node and classifies as
nothing, so a replacement that explained itself between two statements was
rejected as "not a complete node_sequence shape" — indistinguishable, in the
run log, from the model returning broken Rust.

The gate therefore punished the model for obeying its own instructions: the
optimization cards REQUIRE a `// SAFETY:` justification above each `unsafe`
block. Whether a rewrite survived came down to where the model happened to put
that line — inside a call's argument list (accepted, because the comment is
then nested in an expression) or at statement level (rejected).

Measured on a compression crate: the same rule, the same function, the same
hits, in two runs of the same pipeline. The run whose model wrote the comment
inside the argument list committed and measured -8.15% on the crate's main
operation in that single commit; the run whose model wrote it at statement
level lost the rewrite entirely and finished 2.8 points worse overall. Nothing
about the optimization differed.

A second, smaller instance of the same confusion: the validation wrapper was
`fn __region_wrapper(){` + fragment + `}`, so a replacement ending in a `//`
comment commented out the closing brace and failed to parse.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.changeset.handlers.replace_region import (
    validate_region_replacement_source as validate,
)
from perf_opt.agent_perf_opt.regions.model import RegionKind


# ───────────────────────────── the regression: comments among statements

def test_a_safety_comment_between_statements_is_accepted() -> None:
    """The exact shape that cost the run its largest commit."""
    src = ("let slot = costs.offset(jk as isize);\n"
           "// SAFETY: `k <= kend <= 258` and `sublen` has length 259.\n"
           "let value = unsafe { *sublen.get_unchecked(k as usize) };")
    assert validate(RegionKind.NODE_SEQUENCE, src).ok


def test_the_same_comment_inside_an_argument_list_was_never_affected() -> None:
    """Baseline: this is the variant that got through, by luck of placement.
    Both forms must be accepted, or the gate is deciding on style."""
    src = ("let value = cost_fn(\n"
           "    k as c_uint,\n"
           "    // SAFETY: `k <= 258 < 259`.\n"
           "    unsafe { *sublen.get_unchecked(k as usize) },\n"
           ");")
    assert validate(RegionKind.NODE_SEQUENCE, src).ok


@pytest.mark.parametrize("src", [
    "// SAFETY: bounded above\nlet a = 1;",              # leading
    "let a = 1;\n// SAFETY: bounded above\nlet b = 2;",  # between
    "let a = 1;\n// note",                               # trailing
    "let a = 1;\n/* block note */\nlet b = 2;",          # block form
    "/* lead */ let a = 1; /* tail */",                  # both edges, inline
])
def test_comments_are_accepted_at_every_position(src) -> None:
    assert validate(RegionKind.NODE_SEQUENCE, src).ok, src


def test_a_trailing_line_comment_does_not_eat_the_closing_brace() -> None:
    """The wrapper's suffix must start on a new line. Without it this failed
    as `replace_region_parse` — "wrapper has CST errors" — which reads as the
    model producing invalid Rust."""
    result = validate(RegionKind.NODE_SEQUENCE, "let a = 1; // trailing")
    assert result.ok, result.detail


# ───────────────────────────── single-node kinds have the same right

@pytest.mark.parametrize("kind,src", [
    (RegionKind.EXPRESSION, "// SAFETY: in range\nx + 1"),
    (RegionKind.EXPRESSION, "x + 1 // widened"),
    (RegionKind.MATCH_ARM, "// fast path\n1 => 2,"),
    (RegionKind.LOOP, "// SAFETY: `p` stays below `end`\nwhile a < b { a += 1; }"),
    (RegionKind.BLOCK, "// hoisted\n{ let a = 1; }"),
])
def test_edge_comments_do_not_break_exact_node_matching(kind, src) -> None:
    """These kinds demand a node whose bytes are EXACTLY the replacement; a
    comment at either edge breaks the equality without making the replacement
    any less well-formed."""
    assert validate(kind, src).ok, src


# ───────────────────────────── the gate keeps everything it was for

@pytest.mark.parametrize("kind,src", [
    (RegionKind.NODE_SEQUENCE, "let a = 1;\nfn helper() {}"),   # nested item
    (RegionKind.NODE_SEQUENCE, "let a = 1;\nimpl T { fn m(&self) {} }"),
    (RegionKind.NODE_SEQUENCE, "let a = 1;\na + 2"),            # tail expression
    (RegionKind.NODE_SEQUENCE, "let a = ;"),                    # parse error
    (RegionKind.EXPRESSION, "x + 1; let y = 2;"),               # not one expression
    (RegionKind.LOOP, "let x = 1;"),                            # not a loop
])
def test_genuinely_wrong_shapes_are_still_rejected(kind, src) -> None:
    assert not validate(kind, src).ok, src


def test_a_comment_only_replacement_is_not_a_statement_sequence() -> None:
    """Deleting code and leaving the explanation behind is a different edit
    from the one the region was planned for; empty replacements have their own
    accepted path."""
    assert not validate(RegionKind.STATEMENT, "// everything removed").ok


def test_comments_do_not_let_a_forbidden_item_slip_through() -> None:
    """The item scan runs over the whole replacement, comments or not."""
    src = "// SAFETY: none\nlet a = 1;\nmod sneaky { pub fn f() {} }"
    result = validate(RegionKind.NODE_SEQUENCE, src)
    assert not result.ok
    assert result.code == "replace_region_forbidden_item"


# ───────────────────────────── coverage is still exact

@pytest.mark.parametrize("kind,src", [
    (RegionKind.EXPRESSION, "x + 1; y"),
    (RegionKind.BLOCK, "{ let a = 1; } { let b = 2; }"),
])
def test_the_replacement_must_still_cover_the_whole_fragment(kind, src) -> None:
    """Excluding comments from the "is it a statement" question must not
    weaken the check that the parsed nodes span the ENTIRE replacement — text
    left uncovered would be silently spliced in unvalidated."""
    assert not validate(kind, src).ok, src
