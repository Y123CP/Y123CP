"""A view whose length must be scanned for is a cost — but not always a loss.

The rewrite contract has forbidden this in prose from the start: "a view whose
length must be SCANNED to exist (`CStr::from_ptr`, strlen-style search) pays
that scan on EVERY call — never build one in a hot function, however clean the
resulting code looks." Nothing enforced it.

Measured: one crate's `precompute_bonus` walks a NUL-terminated string with
`while *h.offset(i) != 0` — its own loop IS the scan. The rewrite prefixed it
with `CStr::from_ptr(h).to_bytes()`, traversing twice, and W2 read **+7.818%**.
It was rejected with no diagnosis and no retry.

But a hard guard would be wrong. Replayed over all 128 committed rewrites in
the 12-project dataset, four introduced a scan and every one of them had
measured a win — the view paid for itself in what the rewrite then did with
it (one replaced per-byte `isspace()` libc calls with an inline `matches!`).
Rejecting on the scan would have blocked all four.

So it is recorded as a PASSING validation and only used if W2 later rejects:
then the verdict is a defect report rather than a stopwatch reading, and one
corrective turn is worth its build. A candidate that passes W2 never reaches
that path, so no winner can be blocked.
"""

from __future__ import annotations

import subprocess

import pytest

from perf_opt.agent_perf_opt.rewrite_applier import (
    _SCANNED_VIEW_CTORS,
    find_introduced_scans_in_crate,
)
from perf_opt.agent_perf_opt.planners.llm_region import w2_finding_repair_note


# ───────────────────────────────────────────── the predicate

def _crate(tmp_path, before: str, after: str):
    src = tmp_path / "src"
    src.mkdir()
    f = src / "lib.rs"
    f.write_text(before, encoding="utf-8")
    import os
    env = {"GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t",
           "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@t",
           "PATH": os.environ.get("PATH", "")}
    run = lambda *a: subprocess.run(a, cwd=str(tmp_path), check=True,
                                    capture_output=True, env=env)
    run("git", "init", "-q", ".")
    run("git", "add", "-A")
    run("git", "commit", "-qm", "base")
    f.write_text(after, encoding="utf-8")
    return tmp_path


SENTINEL_WALK = """pub unsafe fn f(h: *const u8, out: *mut u32) {
    let mut i = 0usize;
    while *h.add(i) != 0 {
        *out.add(i) = *h.add(i) as u32;
        i += 1;
    }
}
"""

WITH_CSTR = """pub unsafe fn f(h: *const u8, out: *mut u32) {
    let bytes = ::core::ffi::CStr::from_ptr(h as *const i8).to_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        *out.add(i) = b as u32;
    }
}
"""

ALREADY_SCANNED = """pub unsafe fn f(h: *const u8, out: *mut u32) {
    let n = strlen(h as *const i8);
    let mut i = 0usize;
    while i < n {
        *out.add(i) = *h.add(i) as u32;
        i += 1;
    }
}
"""

SUBSTITUTED = """pub unsafe fn f(h: *const u8, out: *mut u32) {
    let bytes = ::core::ffi::CStr::from_ptr(h as *const i8).to_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        *out.add(i) = b as u32;
    }
}
"""


def test_a_scan_the_original_did_not_do_is_reported(tmp_path) -> None:
    """`precompute_bonus`'s shape: the original's own loop was the scan."""
    found = find_introduced_scans_in_crate(_crate(tmp_path, SENTINEL_WALK, WITH_CSTR))
    assert found and "CStr::from_ptr" in found[0], found


def test_swapping_strlen_for_cstr_introduces_nothing(tmp_path) -> None:
    """One spelling for another. The original already walked to the NUL.

    This is the case that made a presence test useless: it flagged 11 of the
    dataset's 128 committed rewrites, all substitutions, in three projects
    that had measured them a win.
    """
    assert find_introduced_scans_in_crate(
        _crate(tmp_path, ALREADY_SCANNED, SUBSTITUTED)) == []


def test_a_safety_comment_naming_the_call_is_not_a_scan(tmp_path) -> None:
    """A `// SAFETY:` note that mentions `CStr::from_ptr` executes nothing.
    The presence test flagged one of those too."""
    after = SENTINEL_WALK.replace(
        "    let mut i = 0usize;",
        "    // SAFETY: unlike CStr::from_ptr, this walks the sentinel itself\n"
        "    let mut i = 0usize;")
    assert find_introduced_scans_in_crate(_crate(tmp_path, SENTINEL_WALK, after)) == []


def test_an_untouched_tree_reports_nothing(tmp_path) -> None:
    assert find_introduced_scans_in_crate(_crate(tmp_path, WITH_CSTR, WITH_CSTR)) == []


def test_a_non_git_path_is_handled(tmp_path) -> None:
    assert find_introduced_scans_in_crate(tmp_path / "nope") == []


@pytest.mark.parametrize("ctor", _SCANNED_VIEW_CTORS)
def test_every_listed_constructor_is_detected(tmp_path, ctor) -> None:
    after = SENTINEL_WALK.replace(
        "    let mut i = 0usize;",
        f"    let _v = {ctor}h);\n    let mut i = 0usize;")
    assert find_introduced_scans_in_crate(_crate(tmp_path, SENTINEL_WALK, after))


# ─────────────────────────────── it is a finding, never a rejection

def _handler_sources() -> dict[str, str]:
    import inspect
    from perf_opt.agent_perf_opt.changeset.handlers import (
        replace_function, replace_region)
    return {"replace_function": inspect.getsource(replace_function),
            "replace_region": inspect.getsource(replace_region)}


@pytest.mark.parametrize("name", ["replace_function", "replace_region"])
def test_the_handler_records_it_as_passing(name) -> None:
    """`ValidationResult(True, ...)`. Returning False here would have blocked
    four committed, measured-win rewrites."""
    body = _handler_sources()[name]
    at = body.index("find_introduced_scans_in_crate(crate)")
    window = body[at:at + 500]
    assert "ValidationResult(\n                True," in window, window[:200]


@pytest.mark.parametrize("name", ["replace_function", "replace_region"])
def test_the_bounds_check_still_rejects(name) -> None:
    """The two must not be confused: an added bounds check IS a rejection."""
    body = _handler_sources()[name]
    at = body.index("find_added_bounds_checks_in_crate(crate)")
    assert "ValidationResult(\n                False," in body[at:at + 400]


# ─────────────────────────────── the note

def _note() -> str:
    return w2_finding_repair_note(
        "introduced_scan",
        "rewrite builds a view whose length must be scanned for, which the "
        "original did not do: let bytes = CStr::from_ptr(h).to_bytes();",
        "// Applied rules: [III④]\nfn f() {}")


def test_the_note_explains_the_double_traversal() -> None:
    low = _note().lower()
    assert "twice" in low or "two" in low
    assert "sentinel" in low


def test_the_note_tells_the_model_what_to_keep() -> None:
    """"Undo it" alone makes the model abstain; it needs the salvage path."""
    low = _note().lower()
    assert "keep the original's traversal" in low
    assert "hoisted invariant" in low or "do not need a length" in low


def test_the_note_quotes_the_finding_and_the_reply() -> None:
    n = _note()
    assert "CStr::from_ptr(h).to_bytes()" in n
    assert "// Applied rules: [III④]" in n


def test_the_note_keeps_the_output_contract() -> None:
    n = _note()
    assert "Applied rules:" in n and "Skipped rules:" in n


def test_the_note_is_bounded() -> None:
    """Bounded in its prose. The previous reply is quoted whole on purpose —
    see `test_a_huge_note_is_bounded_in_its_prose_not_in_the_reply`."""
    reply = "y" * 9000
    note = w2_finding_repair_note("introduced_scan", "x" * 5000, reply)
    assert reply in note
    assert len(note) - len(reply) < 4000
