"""A rewrite must not buy safety the original code never paid for.

c2rust emits `*p.offset(i)` — unchecked. Turning that into `slice[i]` inside a
hot loop adds a bounds check on every iteration: an expense the original never
paid, that nobody asked for, and that no rule card demands.

Measured on one crate's hottest inner loop (`GetBestLengths`), two runs
produced rewrites of the SAME lines under the SAME rules (`C1,C3`), differing
only in this one expression. Same starting commit, same path, same measurement
session, CV < 0.1%:

    baseline                                  29.961s
    rewrite using `get_unchecked`             27.433s   -8.44%
    rewrite using `sublen[k as usize]`        29.964s   +0.01%
    the slow one, with only that expression
      swapped back to `get_unchecked`         27.700s   -7.55%

**8.35 percentage points for one bounds check.** And an `assert!` outside the
loop, intended to let the compiler fold the check away, made it 2.18% SLOWER
still: LLVM did not propagate the bound, and the assert kept its own branch.

W2 cannot catch this. It correctly judges +0.01% to be "no regression" and
commits it — permanently, occupying the region so no better rewrite can be
attempted there. Replayed across 24 committed rewrites from five runs, this
predicate agreed with the measurement on all three that had one: it passed the
-8.44% rewrite and flagged both that measured +1.02% and +0.01%.
"""

from __future__ import annotations

import inspect
import pytest

from perf_opt.agent_perf_opt.planners import llm_region

from perf_opt.agent_perf_opt.rewrite_applier import (
    describe_added_bounds_checks,
    find_added_bounds_checks,
    find_added_bounds_checks_in_crate,
)


# ───────────────────────────────── the regression it exists for

def test_the_slow_rewrite_is_flagged() -> None:
    """The exact shape that cost 8.35 points."""
    code = """
unsafe fn f(sublen: [u16; 259], kend: usize) {
    let mut k = 3usize;
    while k <= kend {
        let v = sublen[k as usize] as u32;
        k += 1;
    }
}
"""
    assert find_added_bounds_checks(code)


def test_the_fast_rewrite_is_not_flagged() -> None:
    """`get_unchecked` matches the original's safety level exactly."""
    code = """
unsafe fn f(sublen: [u16; 259], kend: usize) {
    let mut k = 3usize;
    while k <= kend {
        let v = unsafe { *sublen.get_unchecked(k as usize) } as u32;
        k += 1;
    }
}
"""
    assert not find_added_bounds_checks(code)


def test_leaving_the_raw_pointer_alone_is_not_flagged() -> None:
    """Doing nothing is always allowed — the rule forbids ADDING a check."""
    code = "unsafe fn f(p: *const u8, n: usize) { let mut k=0usize; while k<n { let v=*p.offset(k as isize); k+=1; } }"
    assert not find_added_bounds_checks(code)


# ───────────────────────────────── scope: only inside loops

def test_indexing_outside_a_loop_is_allowed() -> None:
    """One check per call is not what this is about; per-iteration is."""
    code = "fn f(t: [u8; 4], n: usize) { let x = t[n as usize]; while n > 0 { g(); } }"
    assert not find_added_bounds_checks(code)


@pytest.mark.parametrize("head", ["while a < b", "loop", "for i in 0..n"])
def test_every_loop_form_counts(head) -> None:
    body = "{ let v = t[j as usize]; }"
    assert find_added_bounds_checks(f"fn f() {{ {head} {body} }}")


def test_a_nested_loop_is_still_a_loop() -> None:
    code = "fn f() { loop { while a < b { let v = t[j as usize]; } } }"
    assert find_added_bounds_checks(code)


# ───────────────────────────────── robustness

def test_empty_input_is_not_a_violation() -> None:
    assert find_added_bounds_checks("") == []
    assert find_added_bounds_checks(None or "") == []


def test_unparsable_text_does_not_raise() -> None:
    """The checker runs on raw model output, which may carry prose or fences."""
    assert isinstance(find_added_bounds_checks("```rust\nfn f( {\n"), list)


def test_the_finding_names_the_expression() -> None:
    """The log line has to say WHICH access, or the report is unactionable."""
    found = find_added_bounds_checks(
        "fn f() { while k <= n { let v = sublen[k as usize]; } }")
    assert any("sublen" in f for f in found)


# ───────────────────────────────── it is wired into the gate

def _agent_src() -> str:
    from pathlib import Path
    return (Path(__file__).resolve().parents[1] / "agent_perf_opt"
            / "agent.py").read_text(encoding="utf-8")


def _handler_sources() -> dict[str, str]:
    """The post_validate body of every handler that rewrites source."""
    import inspect
    from perf_opt.agent_perf_opt.changeset.handlers import (
        replace_function, replace_region,
    )
    out = {}
    for module in (replace_function, replace_region):
        src = inspect.getsource(module)
        i = src.index("    def post_validate(")
        j = src.find("\n    def ", i + 10)
        out[module.__name__.rsplit(".", 1)[-1]] = src[i:j if j > 0 else len(src)]
    return out


def test_the_check_runs_where_the_applied_source_exists() -> None:
    """Ordering, not location — the property the first version got wrong.

    The predicate reads the WORKING TREE's diff. In the executor the order is
    `pre_validate -> apply -> build -> post_validate -> W1 -> W2 -> commit`,
    so `post_validate` is the first hook at which the rewrite is on disk.
    The guard originally sat in `_apply_and_gate`, ahead of the executor
    entirely: it ran before `applier.apply`, inspected a tree this candidate
    had not touched, and returned nothing every single time. Right function,
    wrong moment — and no test noticed, because every test asserted where it
    lived rather than what it could see.
    """
    import inspect
    from perf_opt.agent_perf_opt.changeset import executor

    src = inspect.getsource(executor.ChangeSetExecutor.execute)
    assert src.index("self.applier.apply(") < src.index("handler.post_validate(")


def test_every_rewriting_handler_carries_the_check() -> None:
    """Both paths, not one. The whole-function path had the (dead) guard; the
    region path never had one at all — and the -8.44% / +0.01% pair that this
    check exists for was a REGION rewrite."""
    missing = [name for name, body in _handler_sources().items()
               if "find_added_bounds_checks_in_crate(" not in body]
    assert not missing, f"handlers with no bounds-check guard: {missing}"


def test_a_flagged_rewrite_is_rejected_not_merely_noted() -> None:
    for name, body in _handler_sources().items():
        block = body[body.index("find_added_bounds_checks_in_crate("):]
        assert "ValidationResult(" in block, name
        assert "added_bounds_check_in_loop" in block, name
        assert "False," in block, name


def test_the_executor_rolls_back_a_failed_post_validation() -> None:
    """Rejecting is only safe if the applied edit is undone — otherwise the
    next candidate builds on top of the rewrite that was just refused."""
    import inspect
    from perf_opt.agent_perf_opt.changeset import executor

    src = inspect.getsource(executor.ChangeSetExecutor.execute)
    block = src[src.index("handler.post_validate("):]
    assert "REJECTED_POST_VALIDATION" in block
    assert "self._reject(" in block


def test_the_agent_no_longer_runs_it_before_the_executor() -> None:
    """Two places judging the same thing, one of which can never see the
    evidence, is worse than one: the dead call reads as coverage."""
    src = _agent_src()
    start = src.index("def _apply_and_gate(")
    body = src[start:src.index("\ndef ", start + 10)]
    assert "find_added_bounds_checks_in_crate(" not in body


def test_both_live_call_sites_route_through_the_executor() -> None:
    src = _agent_src()
    assert src.count("status, extra = _apply_and_gate(") >= 2


def test_the_contract_tells_the_model_this_rule() -> None:
    """Detection is the backstop; the prompt is what prevents the wasted call."""
    from perf_opt.agent_perf_opt import prompt_builder as pb
    contracts = [t for k, t in vars(pb).items()
                 if k.startswith("SYSTEM") and isinstance(t, str)]
    rewriting = [t for t in contracts if "```rust" in t or "rewritten fn" in t]
    assert rewriting
    for text in rewriting:
        assert "NEVER add a runtime check" in text
        assert "get_unchecked" in text


# ───────────────────────────────── the fragment-based version is not enough

def test_a_region_fragment_cannot_answer_the_question_alone() -> None:
    """Why the gate reads the working tree instead of the model's response.

    A region rewrite returns only the statements inside the loop — the `while`
    itself is never in the fragment — so nothing in the response says whether
    a loop encloses the index. Run against the two real transcripts this check
    exists for, the fragment-based version found nothing in either and would
    have let the +0.01% rewrite through unchanged.
    """
    fragment = "let v = sublen[k as usize] as u32;"                   
    assert find_added_bounds_checks(fragment) == []


def test_the_gate_reads_the_tree_not_the_response() -> None:
    """The `_in_crate` variant, everywhere. The response-based one was tried
    first and, run against the two real transcripts this check exists for,
    found nothing in either — see the test above for why."""
    for name, body in _handler_sources().items():
        assert "find_added_bounds_checks_in_crate(" in body, name
        assert "find_added_bounds_checks(operation" not in body, name
        assert "find_added_bounds_checks(replacement" not in body, name


def test_the_handler_is_handed_the_crate_root_directly() -> None:
    """No path guessing any more. `post_validate(operation, resolution, crate)`
    receives the crate the executor is operating on, so the walk-up-to-`.git`
    helper the agent used is gone — one less way to point the diff at the
    wrong tree."""
    import inspect
    for name, body in _handler_sources().items():
        assert "find_added_bounds_checks_in_crate(crate)" in body, name


def test_a_clean_tree_reports_nothing(tmp_path) -> None:
    """No diff means no rewrite to judge — must not raise, must not flag."""
    import subprocess
    subprocess.run(["git", "init", "-q", str(tmp_path)], check=True)
    assert find_added_bounds_checks_in_crate(tmp_path) == []


def test_a_non_git_path_is_handled(tmp_path) -> None:
    assert find_added_bounds_checks_in_crate(tmp_path / "nope") == []


# ────────────── functional: a real dirty tree, not an assertion about source

def _crate_with_rewrite(tmp_path, before: str, after: str):
    """A git crate whose working tree holds `after` over a committed `before`.

    Structural tests say the guard is wired in. Only this says it can SEE
    anything — which is the exact question the previous version got wrong:
    it was wired into the right function, ran before the rewrite reached
    disk, and returned an empty list on every candidate for a whole run.
    """
    import subprocess
    src = tmp_path / "src"
    src.mkdir()
    f = src / "lib.rs"
    f.write_text(before, encoding="utf-8")
    env = {"GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t",
           "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@t",
           "PATH": __import__("os").environ.get("PATH", "")}
    run = lambda *a: subprocess.run(a, cwd=str(tmp_path), check=True,
                                    capture_output=True, env=env)
    run("git", "init", "-q", ".")
    run("git", "add", "-A")
    run("git", "commit", "-qm", "base")
    f.write_text(after, encoding="utf-8")
    return tmp_path


UNCHECKED = """pub unsafe fn f(p: *const u32, n: usize) -> u32 {
    let mut acc = 0u32;
    let mut i = 0usize;
    while i < n {
        acc = acc.wrapping_add(*p.offset(i as isize));
        i += 1;
    }
    acc
}
"""

CHECKED_IN_LOOP = """pub unsafe fn f(p: *const u32, n: usize) -> u32 {
    let s = ::core::slice::from_raw_parts(p, n);
    let mut acc = 0u32;
    let mut i = 0usize;
    while i < n {
        acc = acc.wrapping_add(s[i as usize]);
        i += 1;
    }
    acc
}
"""

UNCHECKED_SLICE = """pub unsafe fn f(p: *const u32, n: usize) -> u32 {
    let s = ::core::slice::from_raw_parts(p, n);
    let mut acc = 0u32;
    let mut i = 0usize;
    while i < n {
        acc = acc.wrapping_add(*s.get_unchecked(i as usize));
        i += 1;
    }
    acc
}
"""


def test_a_bounds_check_added_in_a_loop_is_found_in_a_real_tree(tmp_path) -> None:
    crate = _crate_with_rewrite(tmp_path, UNCHECKED, CHECKED_IN_LOOP)
    found = find_added_bounds_checks_in_crate(crate)
    assert found, "the guard saw nothing in a tree that plainly has one"
    assert "s[i as usize]" in " ".join(found)


def test_the_get_unchecked_rewrite_of_the_same_lines_is_allowed(tmp_path) -> None:
    """Same slice, same loop, same rules — the only difference is the one
    expression that measured -8.44% against +0.01%."""
    crate = _crate_with_rewrite(tmp_path, UNCHECKED, UNCHECKED_SLICE)
    assert find_added_bounds_checks_in_crate(crate) == []


def test_an_untouched_tree_is_not_flagged_by_pre_existing_indexing(tmp_path) -> None:
    """Only ADDED lines count. A crate that already indexed this way must not
    fail every candidate that happens to touch the same file."""
    crate = _crate_with_rewrite(tmp_path, CHECKED_IN_LOOP,
                                CHECKED_IN_LOOP + "\npub fn g() {}\n")
    assert find_added_bounds_checks_in_crate(crate) == []


# ───────────────── a constant subscript is not a bounds check anyone pays for

CONST_INDEX_IN_LOOP = """pub unsafe fn f(p: *const u32, n: usize) -> u32 {
    let hdr: [u32; 4] = [1, 2, 3, 4];
    let mut acc = 0u32;
    let mut i = 0usize;
    while i < n {
        acc = acc.wrapping_add(*p.offset(i as isize)).wrapping_add(hdr[0 as c_int as usize]);
        i += 1;
    }
    acc
}
"""


@pytest.mark.parametrize("subscript", [
    "0 as c_int as usize",     # the miniz shape, verbatim
    "3usize",
    "(2 + 1) as usize",
    "-(1) as usize",
])
def test_a_literal_subscript_is_not_flagged(subscript) -> None:
    """LLVM folds the comparison at compile time; there is no per-iteration
    cost to reject. The line-regex version flagged six of miniz's rewrites on
    exactly this shape and the crate landed at -1.251% against -17.324%."""
    code = ("unsafe fn f(t: [u32; 8], n: usize) -> u32 { let mut a = 0u32; "
            "let mut i = 0usize; while i < n { a += t[%s]; i += 1; } a }"
            % subscript)
    assert find_added_bounds_checks(code) == []


def test_a_runtime_subscript_beside_a_constant_one_is_still_flagged() -> None:
    code = ("unsafe fn f(t: [u32; 8], s: &[u32], n: usize) -> u32 { "
            "let mut a = 0u32; let mut i = 0usize; "
            "while i < n { a += t[0 as usize] + s[i as usize]; i += 1; } a }")
    found = find_added_bounds_checks(code)
    assert len(found) == 1 and "s[i as usize]" in found[0]


def test_a_constant_index_added_in_a_real_tree_is_allowed(tmp_path) -> None:
    crate = _crate_with_rewrite(tmp_path, UNCHECKED, CONST_INDEX_IN_LOOP)
    assert find_added_bounds_checks_in_crate(crate) == []


# ─────────── a subscript the compiler can prove in range costs nothing

CRC_BYTEWISE = """static TBL: [u32; 256] = [0; 256];
pub unsafe fn crc(mut c: u32, p: *const u8, n: usize) -> u32 {
    let mut i = 0usize;
    while i < n {
        c = (c >> 8) ^ TBL[((c ^ *p.offset(i as isize) as u32) & 0xff) as usize];
        i += 1;
    }
    c
}
"""

# The shape miniz's C8 rewrite actually produced, checked against the crate
# that committed it: every table index is masked to the table's width.
CRC_SLICING = """static TBL8: [[u32; 256]; 8] = [[0; 256]; 8];
pub unsafe fn crc(mut c: u32, p: *const u8, n: usize) -> u32 {
    let mut i = 0usize;
    while i + 4 <= n {
        let w = (p.add(i) as *const u32).read_unaligned() ^ c;
        c = TBL8[3][(w & 0xff) as usize]
            ^ TBL8[2][((w >> 8) & 0xff) as usize]
            ^ TBL8[1][((w >> 16) & 0xff) as usize]
            ^ TBL8[0][((w >> 24) & 0xff) as usize];
        i += 4;
    }
    c
}
"""


def test_a_mask_clamped_table_index_is_not_flagged(tmp_path) -> None:
    """`& 0xff` bounds the index at compile time, so LLVM folds the check
    away and there is nothing to reject. This IS C8's slicing-by-N rewrite —
    flagging it cost miniz's `mz_crc32` -21.559%."""
    crate = _crate_with_rewrite(tmp_path, CRC_BYTEWISE, CRC_SLICING)
    assert find_added_bounds_checks_in_crate(crate) == []


@pytest.mark.parametrize("subscript", [
    "(x & 0xff) as usize",                       # the CRC shape
    "((x >> 24) & 0xff) as usize",               # slicing-by-N
    "(x & (SIZE - 1) as u32) as usize",          # a const-named mask
    "(x % 16) as usize",
    "LEN as usize",                              # a const item
])
def test_a_provably_bounded_subscript_is_not_flagged(subscript) -> None:
    code = ("unsafe fn f(t: [u32; 256], n: usize, mut x: u32) -> u32 { "
            "let mut a = 0u32; let mut i = 0usize; "
            "while i < n { a += t[%s]; i += 1; } a }" % subscript)
    assert find_added_bounds_checks(code) == [], subscript


@pytest.mark.parametrize("subscript", [
    "k as usize",                    # zopfli's `sublen[k]`, k <= runtime kend
    "(x & mask) as usize",           # masked by a RUNTIME value
    "(x >> 24) as usize",            # bounded only if the width is known
    "(x + 1) as usize",
])
def test_a_runtime_subscript_is_still_flagged(subscript) -> None:
    code = ("unsafe fn f(t: [u32; 256], n: usize, k: usize, mask: u32, x: u32)"
            " -> u32 { let mut a = 0u32; let mut i = 0usize; "
            "while i < n { a += t[%s]; i += 1; } a }" % subscript)
    assert find_added_bounds_checks(code), subscript


def test_the_founding_case_survives_the_file_already_indexing_that_base(tmp_path) -> None:
    """The exemption must not be "the pre-rewrite source already did this".

    zopfli's `sublen[k as usize]` with `k <= kend` is in the c2rust output
    verbatim, and swapping exactly that one expression for `get_unchecked`
    measured -7.55%. A rule keyed on what the file already indexed would have
    let it through — which is why the predicate asks whether the COMPILER can
    see the bound, not whether the code is new.
    """
    before = """pub unsafe fn f(sublen: &[u16], kend: usize, n: usize) -> u32 {
    let mut a = 0u32;
    let mut k = 3usize;
    while k <= kend {
        a += sublen[k as usize] as u32;
        k += 1;
    }
    a
}
"""
    after = before + """
pub unsafe fn g(sublen: &[u16], kend: usize) -> u32 {
    let mut a = 0u32;
    let mut k = 3usize;
    while k <= kend {
        a += sublen[k as usize] as u32;
        k += 1;
    }
    a
}
"""
    found = find_added_bounds_checks_in_crate(_crate_with_rewrite(tmp_path, before, after))
    assert found and "sublen[k as usize]" in " ".join(found), found


def test_a_runtime_index_beside_a_provable_one_is_still_flagged(tmp_path) -> None:
    """Not a back door: a masked table lookup in the same loop must not buy
    a free pass for a raw pointer turned into a checked slice."""
    after = CRC_BYTEWISE.replace(
        "c = (c >> 8) ^ TBL[((c ^ *p.offset(i as isize) as u32) & 0xff) as usize];",
        "let s = ::core::slice::from_raw_parts(p, n);\n"
        "        c = (c >> 8) ^ TBL[((c ^ s[i as usize] as u32) & 0xff) as usize];")
    found = find_added_bounds_checks_in_crate(_crate_with_rewrite(tmp_path, CRC_BYTEWISE, after))
    assert found and any("s[i as usize]" in f for f in found), found


# ────── a `while` against a literal proves the index is inside a fixed array

# lodepng's `filter`, verbatim from the c2rust output. Three subscripts, each
# bounded by its own `while` against an integer literal, each reading a
# `[c_uint; 256]` declared in the same function. LLVM's induction-variable
# analysis folds all three checks away.
#
# The guard reported them as
#   `count[x as usize], count[type_2 as usize], count[type_2 as usize]`
# and rejected a 94-line rewrite that had used `get_unchecked` for the one
# raw-pointer array it did touch. The repair turn spent on that rejection
# produced a replacement that measured +0.130%.
LODEPNG_HISTOGRAM = """pub unsafe fn filter(h: u32) -> u64 {
    let mut count: [u32; 256] = [0; 256];
    let mut sum: u64 = 0;
    let mut type_2: u32 = 0;
    while type_2 != 5 as u32 {
        let mut x: u32 = 0;
        count[type_2 as usize] = count[type_2 as usize].wrapping_add(1);
        x = 0 as u32;
        while x != 256 as u32 {
            sum = sum.wrapping_add(count[x as usize] as u64);
            x = x.wrapping_add(1);
        }
        type_2 = type_2.wrapping_add(1);
    }
    sum
}
"""


def test_a_literal_bounded_while_over_a_fixed_array_is_not_flagged() -> None:
    """The shape that cost lodepng's `filter` its rewrite."""
    assert find_added_bounds_checks(LODEPNG_HISTOGRAM) == []


def test_a_named_subscript_resolves_to_its_induction_variable() -> None:
    """`let slot = type_2 as usize` is the same proof one hop away. Rewrites
    write this to name a subscript they use more than once; c2rust does not."""
    code = """pub unsafe fn f() {
    let mut attempt: [*mut u8; 5] = [::core::ptr::null_mut(); 5];
    let mut type_2: u32 = 0;
    while type_2 != 5 as u32 {
        let slot = type_2 as usize;
        attempt[slot] = ::core::ptr::null_mut();
        type_2 = type_2.wrapping_add(1);
    }
}
"""
    assert find_added_bounds_checks(code) == []


def test_a_runtime_bound_proves_nothing_about_a_fixed_array() -> None:
    """`while x as size_t != linebytes` is the same array and the same loop
    shape with a runtime limit. Nothing is proven and the check stays."""
    code = LODEPNG_HISTOGRAM.replace("while x != 256 as u32",
                                     "while x as usize != h as usize")
    found = find_added_bounds_checks(code)
    assert found and any("count[x as usize]" in f for f in found), found


def test_a_loop_that_runs_past_the_array_is_still_flagged() -> None:
    code = LODEPNG_HISTOGRAM.replace("let mut count: [u32; 256] = [0; 256];",
                                     "let mut count: [u32; 4] = [0; 4];")
    assert find_added_bounds_checks(code)


@pytest.mark.parametrize("step", [
    "x = j;",                      # reassigned from elsewhere
    "g(&mut x);",                  # address handed out
    "x = x.wrapping_sub(1);",      # not monotone up
])
def test_an_index_the_loop_does_not_only_step_up_is_still_flagged(step) -> None:
    """The induction argument needs the variable to move one way, by a known
    amount, and to be written nowhere else. Anything looser gives up."""
    code = ("pub unsafe fn f(j: u32, g: &mut dyn FnMut(&mut u32)) -> u64 {\n"
            "    let mut a: [u32; 256] = [0; 256];\n"
            "    let mut s: u64 = 0;\n"
            "    let mut x: u32 = 0;\n"
            "    while x != 256 as u32 {\n"
            "        s = s.wrapping_add(a[x as usize] as u64);\n"
            f"        {step}\n"
            "    }\n"
            "    s\n}\n")
    assert find_added_bounds_checks(code)


def test_a_same_named_binding_that_is_not_a_fixed_array_abandons_the_proof() -> None:
    """The length has to come from a declaration in the same function. A `Vec`
    under the same name could be what the subscript actually reads."""
    code = """pub unsafe fn f(v: Vec<u32>) -> u64 {
    let mut a: Vec<u32> = v;
    let mut s: u64 = 0;
    let mut x: u32 = 0;
    while x != 256 as u32 {
        s = s.wrapping_add(a[x as usize] as u64);
        x = x.wrapping_add(1);
    }
    s
}
"""
    assert find_added_bounds_checks(code)


def test_the_zopfli_rewrite_this_guard_exists_for_still_loses(tmp_path) -> None:
    """The proof must not reach the case the whole guard was built on: a
    runtime-bounded `sublen[k as usize]` whose `get_unchecked` form measured
    -7.55%."""
    after = LODEPNG_HISTOGRAM + """
pub unsafe fn g(sublen: &[u16], kend: usize) -> u32 {
    let mut a = 0u32;
    let mut k = 3usize;
    while k <= kend {
        a += sublen[k as usize] as u32;
        k += 1;
    }
    a
}
"""
    found = find_added_bounds_checks_in_crate(
        _crate_with_rewrite(tmp_path, UNCHECKED, after))
    assert found and any("sublen[k as usize]" in f for f in found), found


# ───────── the array a hot loop walks is as often a `static` as a local

# c2rust hoists C's file-scope tables to module items. lodepng's
# `Adam7_deinterlace` walks four of them with one induction variable:
#
#     static mut ADAM7_DX: [c_uint; 7] = [ ... ];
#     while i != 7 { ... ADAM7_DX[i as usize] ... }
#
# Identical in provability to the local-array case above — LLVM folds both —
# but the proof looked only inside `function_item`, so the guard reported
# `ADAM7_DX[i as usize], ADAM7_DY[i as usize], ADAM7_IX[i as usize]` and
# rejected a 126-line rewrite. Verified against that changeset: with the four
# real `static` declarations in scope, all four reports disappear.
STATIC_TABLE_WALK = """static mut ADAM7_DX: [u32; 7] = [0; 7];
pub unsafe fn walk() -> u64 {
    let mut s: u64 = 0;
    let mut i: u32 = 0;
    while i != 7 as u32 {
        s = s.wrapping_add(ADAM7_DX[i as usize] as u64);
        i = i.wrapping_add(1);
    }
    s
}
"""


def test_a_static_fixed_array_is_proven_like_a_local_one() -> None:
    assert find_added_bounds_checks(STATIC_TABLE_WALK) == []


@pytest.mark.parametrize("decl", [
    "static mut TBL: [u32; 16] = [0; 16];",
    "static TBL: [u32; 16] = [0; 16];",
    "const TBL: [u32; 16] = [0; 16];",
])
def test_every_module_item_form_counts(decl) -> None:
    code = (decl + "\npub unsafe fn f() -> u64 {\n"
            "    let mut s: u64 = 0; let mut i: u32 = 0;\n"
            "    while i < 16 as u32 { s += TBL[i as usize] as u64; i += 1; }\n"
            "    s\n}\n")
    assert find_added_bounds_checks(code) == []


# ───────── a local of the same name shadows the module item, either way

def test_a_local_vec_shadowing_a_static_abandons_the_proof() -> None:
    """The subscript reads the LOCAL. Falling through to the `static` would
    prove a bound about an array the code is demonstrably not indexing."""
    code = ("static TBL: [u32; 16] = [0; 16];\n"
            "pub unsafe fn f(v: Vec<u32>) -> u64 {\n"
            "    let TBL: Vec<u32> = v;\n"
            "    let mut s: u64 = 0; let mut i: u32 = 0;\n"
            "    while i < 16 as u32 { s += TBL[i as usize] as u64; i += 1; }\n"
            "    s\n}\n")
    assert find_added_bounds_checks(code)


def test_a_shorter_local_shadowing_a_longer_static_is_flagged() -> None:
    code = ("static TBL: [u32; 256] = [0; 256];\n"
            "pub unsafe fn f() -> u64 {\n"
            "    let TBL: [u32; 4] = [0; 4];\n"
            "    let mut s: u64 = 0; let mut i: u32 = 0;\n"
            "    while i < 16 as u32 { s += TBL[i as usize] as u64; i += 1; }\n"
            "    s\n}\n")
    assert find_added_bounds_checks(code)


def test_a_longer_local_shadowing_a_shorter_static_is_allowed() -> None:
    """Shadowing settles it in both directions — the local's length is the
    one that applies."""
    code = ("static TBL: [u32; 4] = [0; 4];\n"
            "pub unsafe fn f() -> u64 {\n"
            "    let TBL: [u32; 256] = [0; 256];\n"
            "    let mut s: u64 = 0; let mut i: u32 = 0;\n"
            "    while i < 16 as u32 { s += TBL[i as usize] as u64; i += 1; }\n"
            "    s\n}\n")
    assert find_added_bounds_checks(code) == []


# ───────── the same conservatism the local lookup already had

@pytest.mark.parametrize("code,why", [
    ("static TBL: [u32; 4] = [0; 4];\npub unsafe fn f() -> u64 {\n"
     "  let mut s: u64 = 0; let mut i: u32 = 0;\n"
     "  while i != 256 as u32 { s += TBL[i as usize] as u64; i += 1; }\n  s\n}\n",
     "loop runs past the table"),
    ("const N: usize = 7;\nstatic TBL: [u32; N] = [0; N];\n"
     "pub unsafe fn f() -> u64 {\n  let mut s: u64 = 0; let mut i: u32 = 0;\n"
     "  while i != 7 as u32 { s += TBL[i as usize] as u64; i += 1; }\n  s\n}\n",
     "length is not a literal"),
    ("static TBL: &[u32] = &[0; 16];\npub unsafe fn f() -> u64 {\n"
     "  let mut s: u64 = 0; let mut i: u32 = 0;\n"
     "  while i < 16 as u32 { s += TBL[i as usize] as u64; i += 1; }\n  s\n}\n",
     "a slice reference is not a fixed array"),
    ("static TBL: [u32; 16] = [0; 16];\nmod inner { pub static TBL: [u32; 4] = [0; 4]; }\n"
     "pub unsafe fn f() -> u64 {\n  let mut s: u64 = 0; let mut i: u32 = 0;\n"
     "  while i < 16 as u32 { s += TBL[i as usize] as u64; i += 1; }\n  s\n}\n",
     "the smallest same-named item wins"),
    ("static TBL8: [[u32; 256]; 8] = [[0; 256]; 8];\npub unsafe fn f(k: usize) -> u64 {\n"
     "  let mut s: u64 = 0; let mut i: u32 = 0;\n"
     "  while i < 8 as u32 { s += TBL8[i as usize][k] as u64; i += 1; }\n  s\n}\n",
     "the inner subscript of a nested table is unproven"),
    ("static TBL: [u32; 16] = [0; 16];\npub unsafe fn f(n: u32) -> u64 {\n"
     "  let mut s: u64 = 0; let mut i: u32 = 0;\n"
     "  while i != n { s += TBL[i as usize] as u64; i += 1; }\n  s\n}\n",
     "a runtime loop bound proves nothing"),
])
def test_a_static_does_not_widen_the_proof(code, why) -> None:
    assert find_added_bounds_checks(code), why


# ───────── c2rust's own spelling of a provable bound
#
# The guard's three proof paths each required a shape c2rust does not emit,
# so on real translated output none of them ever fired:
#
#   loop bound must be a literal      c2rust writes `while i < NB_FILTERS`
#   length must be `[T; N]`           III④'s whole product is a slice view
#   subscript must be the variable    a C down-counter arrives as `(N-1) - i`
#
# Measured, on `aptx_qmf_polyphase_analysis`: all three of its subscripts were
# reported, the III④ rewrite was rejected on post-validation, and the same
# rewrite had been committed by an earlier run whose guard did not yet exist.
# The rule and the guard were structurally opposed — every III④ rewrite of a
# loop produces exactly the shape the guard could not read.

@pytest.mark.parametrize("code,why", [
    ("const N: usize = 4;\npub unsafe fn f() -> u64 {\n"
     "  let a: [u32; 4] = [0; 4]; let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += a[i] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "a const loop bound is as compile-time as a literal"),
    ("const N: usize = 7;\nstatic DX: [u32; 7] = [0; 7];\n"
     "pub unsafe fn f() -> u64 {\n  let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += DX[i] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "const bound against a module table"),
    ("const N: usize = 4;\npub unsafe fn f(p: *const u32) -> u64 {\n"
     "  let s2 = unsafe { core::slice::from_raw_parts(p, N) };\n"
     "  let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += s2[i] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "the slice view III④ creates carries its own length"),
    ("const N: usize = 4;\npub unsafe fn f(p: *const u32) -> u64 {\n"
     "  let s2 = unsafe { core::slice::from_raw_parts(p, N) };\n"
     "  let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += s2[(N - 1).wrapping_sub(i)] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "a descending walk stays inside the same array"),
])
def test_c2rust_shapes_are_provable(code, why) -> None:
    assert not find_added_bounds_checks(code), why


# ───────── and none of it may excuse a check the compiler really emits

@pytest.mark.parametrize("code,why", [
    ("pub unsafe fn f(p: *mut u32, kend: u32, n: usize) {\n"
     "  let sublen = unsafe { core::slice::from_raw_parts_mut(p, n) };\n"
     "  let mut k: u32 = 0;\n"
     "  while k <= kend { sublen[k as usize] = 0; k = k.wrapping_add(1); }\n}\n",
     "the -7.55% case: a runtime bound over a runtime-length view"),
    ("const N: usize = 4;\npub unsafe fn f(p: *const u32, n: usize) -> u64 {\n"
     "  let s2 = unsafe { core::slice::from_raw_parts(p, n) };\n"
     "  let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += s2[i] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "a runtime slice length proves nothing"),
    ("const N: usize = 8;\npub unsafe fn f() -> u64 {\n"
     "  let a: [u32; 4] = [0; 4]; let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += a[i] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "a const bound past the end is still past the end"),
    ("pub unsafe fn f(p: *const u32) -> u64 {\n"
     "  let s2 = unsafe { core::slice::from_raw_parts(p, 4) };\n"
     "  let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < 8 { s += s2[(3 as usize).wrapping_sub(i)] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "a descending walk that underflows wraps to a huge index"),
    ("const N: usize = 4;\npub unsafe fn f(d: usize) -> u64 {\n"
     "  let a: [u32; 4] = [0; 4]; let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += a[i] as u64; i = i.wrapping_add(d); }\n  s\n}\n",
     "a runtime step is not an induction variable"),
    ("const N: usize = 4;\npub unsafe fn f(a: [u32; 4]) -> u64 {\n"
     "  let mut s: u64 = 0; let mut i: usize = 0;\n"
     "  while i < N { s += a[i] as u64; i = i.wrapping_add(1); }\n  s\n}\n",
     "an array parameter is not a let-bound array; stay conservative"),
])
def test_const_folding_does_not_widen_the_proof(code, why) -> None:
    assert find_added_bounds_checks(code), why


# ───────── the finding drives the repair turn, so it must be complete
#
# `", ".join(added[:3])` named three sites however many were found. The model
# corrects what it is shown, so a rewrite with fourteen offending indexes came
# back with eleven, was rejected again, and the second turn — the last one —
# went the same way. Measured on lodepng: `Adam7_deinterlace` (14 found) and
# `filter` (4 found), all rejected twice.

def test_the_finding_names_every_site() -> None:
    added = [f"a{i}[i as usize]" for i in range(7)]
    text = describe_added_bounds_checks(added)
    for site in added:
        assert site in text, text
    assert "7 site(s)" in text


def test_the_finding_deduplicates() -> None:
    text = describe_added_bounds_checks(["a[i]", "a[i]", "b[i]"])
    assert "2 site(s)" in text
    assert text.count("a[i]") == 1


def test_the_finding_caps_a_pathological_diff() -> None:
    added = [f"a{i}[i as usize]" for i in range(60)]
    text = describe_added_bounds_checks(added)
    assert "60 site(s)" in text
    assert "and 40 more" in text
    assert len(text) < 2000, "must still fit the repair note's budget"


def test_the_repair_note_can_carry_the_whole_finding() -> None:
    """A finding wider than the note's slice would be silently trimmed back to
    the same partial list the cap exists to prevent."""
    note_src = inspect.getsource(llm_region.post_validation_repair_note)
    assert "detail[:600]" not in note_src
    added = [f"some_array_{i}[index_{i} as usize]" for i in range(20)]
    detail = describe_added_bounds_checks(added)
    note = llm_region.post_validation_repair_note(
        "added_bounds_check_in_loop", detail, "prev")
    for site in added:
        assert site in note, f"{site} was trimmed out of the repair note"
