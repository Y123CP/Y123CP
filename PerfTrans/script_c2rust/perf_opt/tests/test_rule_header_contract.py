"""The declaration header is prose about Rust, and prose about Rust has `]`.

The contract asks the model to prefix its rewrite with two lines:

    // Applied rules: [III④, C3, III①]
    // Skipped rules: [III②: <reason>; III③: <reason>]

Both the gate that validates them and the parser that reads them matched the
bracket as `\\[([^\\]]*)\\]\\s*$` — everything up to the FIRST `]`. A skip
reason explains why a rule does not fit, which means quoting the code, which
means array types and index expressions:

    // Skipped rules: [III②: … `Node` / `[*mut Node; 2]` initialization
    //   requirements are not visible here …; III③: …]

The bracket closed at `; 2]`, `$` failed against the rest of the line, and the
header was judged absent. `metadata_headers_missing_or_not_first` — a response
that had applied three rules correctly, including the `qsort` →
`sort_unstable_by` rewrite hand-measured at -30.72% on one operation, was
discarded whole and the region never re-offered.

Widening the bracket alone is not enough. `parse_skipped_rules` then splits
entries on `;`, which is also Rust's statement terminator and the separator
inside `[T; N]`. The same reason splits into three, the middle fragment
becomes a rule id nobody declared, and `declared - candidates` rejects the
response one check later — a different error, same lost rewrite.

So both the bracket and the split have to understand that the reason is
quoting code: the list ends at the LAST `]` on the line, and `;` separates
entries only at bracket depth zero, outside backticks.
"""

from __future__ import annotations

import pytest

from perf_opt.agent_perf_opt.planners.llm_region import (
    _APPLIED_HEADER,
    _SKIPPED_HEADER,
    _parse_metadata,
)
from perf_opt.agent_perf_opt.rewrite_applier import (
    _split_top_level,
    parse_applied_rules,
    parse_skipped_rules,
)


# The exact two lines the run produced, verbatim.
REAL_APPLIED = "// Applied rules: [III④, C3, III①]"
REAL_SKIPPED = (
    "// Skipped rules: [III②: `leaves`, `nodes`, and `lists` are used as "
    "uninitialized scratch/raw FFI-style buffers, and `Node` / `[*mut Node; 2]` "
    "initialization requirements are not visible here for a sound RAII "
    "conversion; III③: this region has no suitable libc/manual byte-memory "
    "operation to replace with slice/string APIs]"
)
CANDIDATES = {"III④", "C3", "III②", "III③", "III①"}


# ─────────────────────────────────── the gate accepts the real header

def test_the_real_skipped_header_is_recognised() -> None:
    assert _SKIPPED_HEADER.fullmatch(REAL_SKIPPED)


def test_the_real_applied_header_is_recognised() -> None:
    assert _APPLIED_HEADER.fullmatch(REAL_APPLIED)


@pytest.mark.parametrize("quoted", [
    "`[*mut Node; 2]`",      # the array type that broke it
    "`[u8; 256]`",
    "`buf[i]`",              # an index expression
    "`v[..n]`",              # a slice range
    "`Option<[u8; 4]>`",
])
def test_a_reason_may_quote_any_bracketed_rust(quoted) -> None:
    line = f"// Skipped rules: [C3: cannot hoist past {quoted} here]"
    assert _SKIPPED_HEADER.fullmatch(line)
    assert parse_skipped_rules(line) == [
        ("C3", f"cannot hoist past {quoted} here")]


# ─────────────────────────── a semicolon inside a reason is punctuation

# The contract separates entries with `;`. English also joins two clauses
# with `;`, and the model does exactly that when a skip reason has two halves
# to it. Every fragment below is real prose that was cut out of a reason and
# then read as a rule id nobody had offered — `declared - candidates` marked
# it `unknown_declared_rule` and discarded the entire rewrite. 62 of these
# across the corpus, from four projects.
@pytest.mark.parametrize("reason", [
    "no alloc/free pair to replace; the function only releases existing ownership",
    "no alloc/free pair in this function; it only frees existing heap objects",
    "the loop's loads advance with the cursors; nothing useful to hoist",
    "one clause; two clause; three clause",
])
def test_a_semicolon_inside_a_reason_does_not_invent_a_rule(reason) -> None:
    line = f"// Skipped rules: [III②: {reason}]"
    assert parse_skipped_rules(line) == [("III②", reason)], (
        "the tail after a bare `;` is the rest of the reason, not an entry")


def test_real_entries_still_split_around_a_reason_that_has_a_semicolon() -> None:
    """The fix must not swallow the next entry: a fragment is folded back only
    when it carries no `<rule>:` prefix of its own."""
    line = ("// Skipped rules: [III②: no pair here; it only frees; "
            "C1: no hot loop]")
    assert parse_skipped_rules(line) == [
        ("III②", "no pair here; it only frees"),
        ("C1", "no hot loop"),
    ]


@pytest.mark.parametrize("quoted", [
    "`ptr::copy`", "`core::slice::from_raw_parts`", "`<[u8]>::copy_from_slice`",
])
def test_a_rust_path_in_a_reason_is_not_a_rule_boundary(quoted) -> None:
    """Rust paths carry `::`. Deciding the rule/reason split with a bare
    `":" in part` reads one of those as the boundary and turns half a
    sentence into a rule id — the same mistake as splitting on a semicolon
    that was only punctuation, one layer down."""
    line = (f"// Skipped rules: [III③: the hot site is a match-copy; "
            f"replacing it with {quoted} risks the semantics]")
    assert parse_skipped_rules(line) == [
        ("III③", f"the hot site is a match-copy; replacing it with {quoted} "
                 "risks the semantics")]


def test_a_leading_fragment_with_no_rule_prefix_is_still_an_error() -> None:
    """Folding needs something to fold into. A list that opens with prose is
    malformed, and must stay visible as such rather than being absorbed."""
    parsed = parse_skipped_rules("// Skipped rules: [just prose; C1: no loop]")
    assert parsed[0] == ("just prose", "")


def test_trailing_text_after_the_list_is_still_rejected() -> None:
    """Widening the bracket must not turn the header into "anything goes" —
    the list has to be the last thing on the line."""
    assert not _SKIPPED_HEADER.fullmatch(
        "// Skipped rules: [C3: no] and then some prose")


def test_a_header_without_brackets_is_still_rejected() -> None:
    assert not _APPLIED_HEADER.fullmatch("// Applied rules: C3")


def test_the_header_does_not_span_lines() -> None:
    """`\\s*$` would let a greedy match walk onto a later line and anchor
    there; the trailing class is restricted to horizontal space for that."""
    assert not _APPLIED_HEADER.fullmatch("// Applied rules: [C3\n, III④]")


# ─────────────────────────────────── the split understands the reason

def test_the_real_skipped_header_yields_exactly_its_two_rules() -> None:
    parsed = parse_skipped_rules(REAL_SKIPPED)
    assert [rule for rule, _ in parsed] == ["III②", "III③"]


def test_no_fragment_becomes_a_phantom_rule() -> None:
    """This is what `declared - candidates` sees. One stray fragment and the
    whole response is rejected as `unknown_declared_rule`."""
    declared = {rule for rule, _ in parse_skipped_rules(REAL_SKIPPED)}
    assert declared <= CANDIDATES


def test_the_first_reason_survives_intact() -> None:
    reason = dict(parse_skipped_rules(REAL_SKIPPED))["III②"]
    assert "[*mut Node; 2]" in reason
    assert reason.endswith("for a sound RAII conversion")


@pytest.mark.parametrize("text, expected", [
    ("a; b", ["a", " b"]),
    ("a `x; y` b; c", ["a `x; y` b", " c"]),          # backticked code
    ("a [T; N] b; c", ["a [T; N] b", " c"]),          # array type
    ("a (f(x); g) b; c", ["a (f(x); g) b", " c"]),    # parens
    ("a {v; w} b; c", ["a {v; w} b", " c"]),          # braces
    ("no separator", ["no separator"]),
    ("", [""]),
])
def test_top_level_split_shapes(text, expected) -> None:
    assert _split_top_level(text, ";") == expected


def test_an_unbalanced_close_does_not_wedge_the_depth() -> None:
    """A stray `]` in prose must not drive depth negative and swallow every
    later separator — the reason is free text, not guaranteed balanced."""
    assert _split_top_level("a ] b; c", ";") == ["a ] b", " c"]


def test_a_reason_with_a_rust_statement_still_splits(  ) -> None:
    line = ("// Skipped rules: [III④: the loop body ends with `i += 1;` so the "
            "cursor is not a plain stride; C3: single use]")
    assert [r for r, _ in parse_skipped_rules(line)] == ["III④", "C3"]


# ─────────────────────────────────── end to end through the planner

def test_the_rejected_response_now_parses(  ) -> None:
    code = f"{REAL_APPLIED}\n{REAL_SKIPPED}\nlet x = 1;"
    result = _parse_metadata(code)
    assert not isinstance(result, str), f"still rejected: {result}"
    applied, skipped, replacement = result
    assert applied == ["III④", "C3", "III①"]
    assert {r for r, _ in skipped} == {"III②", "III③"}
    assert replacement == "let x = 1;"


def test_every_candidate_rule_is_accounted_for() -> None:
    """`declared != candidates` is the check that fired next. It has to pass
    for the same response, or fixing the header only moves the failure."""
    code = f"{REAL_APPLIED}\n{REAL_SKIPPED}\nlet x = 1;"
    applied, skipped, _ = _parse_metadata(code)
    assert set(applied) | {r for r, _ in skipped} == CANDIDATES


def test_no_reason_comes_back_empty() -> None:
    """`skipped_rule_reason_empty` is another downstream reject; a mis-split
    reason can easily produce one."""
    _, skipped, _ = _parse_metadata(f"{REAL_APPLIED}\n{REAL_SKIPPED}\nlet x = 1;")
    assert all(reason.strip() for _, reason in skipped)


def test_applied_rules_parses_the_same_line_from_a_whole_response() -> None:
    """The whole-fn path reads the header out of a larger body with
    `search`, not `fullmatch` — the same widening has to hold there."""
    body = f"{REAL_APPLIED}\n{REAL_SKIPPED}\npub fn f() {{}}\n"
    assert parse_applied_rules(body) == ["III④", "C3", "III①"]
    assert [r for r, _ in parse_skipped_rules(body)] == ["III②", "III③"]
