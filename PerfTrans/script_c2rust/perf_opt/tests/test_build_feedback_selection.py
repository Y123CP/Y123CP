"""Build feedback must carry the errors, not whatever happened to be last.

`gate_build` used to return `err[-8000:]` on the premise that "rustc puts the
actionable errors last". That premise holds only while the output is small. The
crate under test compiles in the same invocation, and a c2rust translation of a
large C library emits thousands of warnings; on libxml2 (1603 fns) they filled
the entire window, so the repair prompt contained no `error:` line at all —
only warnings about the library's own source, which the harness may not edit.
Six repair rounds were spent on a build the model could not see.
"""

from __future__ import annotations

from harness_gen.gates import FEEDBACK_BUDGET, select_build_feedback

HARNESS_ERR = (
    "error[E0308]: mismatched types\n"
    "  --> src/lib.rs:412:31\n"
    "   |\n"
    "412 |     xmlSchemaNewFacet(ctxt)\n"
    "   |                       ^^^^ expected `*mut relaxng::_xmlSchemaFacet`\n"
)
SECOND_ERR = (
    "error[E0425]: cannot find value `xmlFree` in this scope\n"
    "  --> src/lib.rs:501:9\n"
)
# what a big c2rust crate floods the tail with
LIB_WARNINGS = "".join(
    f"warning: unused comparison that must be used\n"
    f"  --> /abs/libxml2/0_raw/src/relaxng.rs:{n}:21\n"
    f"   |\n"
    f"{n} |     ret == 0 as c_int;\n"
    f"   |     ^^^^^^^^^^^^^^^^^ the comparison produces a value\n"
    for n in range(1000, 1400)
)


def test_the_regression_error_survives_a_flood_of_library_warnings() -> None:
    out = select_build_feedback(HARNESS_ERR + LIB_WARNINGS)
    assert "E0308" in out
    assert len(LIB_WARNINGS) > FEEDBACK_BUDGET      # the flood really overflows
    assert out[-FEEDBACK_BUDGET:] != (HARNESS_ERR + LIB_WARNINGS)[-FEEDBACK_BUDGET:]


def test_tail_truncation_would_have_lost_it() -> None:
    """Pins the old behaviour as the thing being fixed."""
    stderr = HARNESS_ERR + LIB_WARNINGS
    assert "E0308" not in stderr[-FEEDBACK_BUDGET:]


def test_warnings_are_dropped_when_errors_exist() -> None:
    out = select_build_feedback(HARNESS_ERR + LIB_WARNINGS)
    assert "unused comparison" not in out


def test_all_errors_are_kept_in_source_order() -> None:
    out = select_build_feedback(HARNESS_ERR + LIB_WARNINGS + SECOND_ERR)
    assert out.index("E0308") < out.index("E0425")


def test_error_body_is_kept_not_just_the_headline() -> None:
    """The `-->` location and the caret line are what make it fixable."""
    out = select_build_feedback(HARNESS_ERR + LIB_WARNINGS)
    assert "src/lib.rs:412:31" in out
    assert "expected `*mut relaxng::_xmlSchemaFacet`" in out


def test_budget_is_respected_and_elision_is_announced() -> None:
    many = "".join(
        f"error[E0308]: mismatched types\n  --> src/lib.rs:{n}:1\n"
        f"   |\n   | {'x' * 200}\n"
        for n in range(400)
    )
    out = select_build_feedback(many)
    assert len(out) <= FEEDBACK_BUDGET
    assert "more error(s) elided" in out


def test_no_diagnostics_falls_back_to_the_tail() -> None:
    """Linker failures and OOM-killed rustc produce no `error:` blocks."""
    raw = "some linker noise\n" * 50 + "collect2: fatal error\n"
    out = select_build_feedback(raw)
    assert out.endswith("collect2: fatal error\n")


def test_warnings_only_output_falls_back_to_the_tail() -> None:
    out = select_build_feedback(LIB_WARNINGS)
    assert out == LIB_WARNINGS[-FEEDBACK_BUDGET:]


def test_empty_stderr_is_not_a_crash() -> None:
    assert select_build_feedback("") == ""
