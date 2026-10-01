"""Succeeding at a cheap rewrite must not forfeit the expensive ones.

Two independent places dropped work that was still on the table, both by
ending a function's processing early.

**Cross-function.** A committed cross-function rewrite used to `continue`,
skipping every remaining match in that function on the grounds that the
signature had changed and the recorded spans were stale. Measured: the
hottest function of a compression crate carries 41 matches, of which 3 belong
to the cross-function rule. Committing those 3 for **+0.03%** discarded the
other 38 (C1x1, C3x10, III4x20, III3x1, II_inlx6) --- and a run in which the
cross-function attempt ABSTAINED went on to commit a region rewrite worth
**-8.15%** on that same function. The cheap success cost more than it earned.

Staleness is not unique to that path: a committed region rewrite shifts the
spans of every later hit in the same function, and that path already handles
it by re-resolving and re-extracting against the updated source. Falling
through reaches the same machinery.

**Whole-function bundles.** The decomposition of a rejected multi-rule
candidate existed only on the region path. On the whole-function path a
rejected bundle kept every rule in it. Measured on the same crate: three such
rejections were a loop-invariant hoist paired with a pointer-to-slice
rewrite, on functions holding 26.7%, 7.6% and 3.5% of their operation.
"""

from __future__ import annotations

from pathlib import Path

import pytest

SRC = Path(__file__).resolve().parents[1] / "agent_perf_opt" / "agent.py"


@pytest.fixture(scope="module")
def source() -> str:
    return SRC.read_text(encoding="utf-8")



def _agent_src() -> str:
    from pathlib import Path
    return (Path(__file__).resolve().parents[1] / "agent_perf_opt"
            / "agent.py").read_text(encoding="utf-8")


def _cross_fn_block(source: str) -> str:
    start = source.index("cross-fn attempt")
    return source[start:source.index("orchestration abstain", start)]


def _complex_block(source: str) -> str:
    start = source.index("rec = _try_fn_plan_execute(")
    return source[start:source.index("no fn_hits entry, skip", start)]


# ─────────────────────────────── cross-fn no longer ends the function

def test_a_committed_cross_fn_rewrite_does_not_end_the_function(source) -> None:
    """The regression: a bare `continue` right after the commit bookkeeping."""
    block = _cross_fn_block(source)
    commit_tail = block[block.index("APPLIED_COMMITTED"):]
    first_stmts = [ln.strip() for ln in commit_tail.split("\n")[1:8]]
    assert "continue" not in first_stmts, commit_tail[:400]


def test_the_intra_fn_path_still_runs_after_a_cross_fn_commit(source) -> None:
    block = _cross_fn_block(source)
    assert "Fall through to the intra-fn path" in block


def test_the_spent_cross_fn_matches_are_removed(source) -> None:
    """Leaving them in would offer the agent a rule it has already applied,
    against a signature that no longer exhibits the pattern."""
    block = _cross_fn_block(source)
    assert 'h.get("rule") != "III①"' in block


def test_the_function_index_is_rebuilt_before_falling_through(source) -> None:
    """The intra-fn path resolves its edit target from this index; a stale one
    would point at the pre-rewrite signature."""
    block = _cross_fn_block(source)
    commit_tail = block[block.index("APPLIED_COMMITTED"):]
    assert "build_fn_index(crate)" in commit_tail


# ─────────────────────────────── whole-function bundles decompose too

def test_the_whole_fn_path_uses_the_same_predicate_as_the_region_path() -> None:
    """Retiring anchors, queueing region retries and queueing whole-fn retries
    all answer one question: is this rejection worth splitting? Three copies of
    that question drifted apart once already — the whole-fn copy still split on
    `no_gain` after the region copy had stopped."""
    src = _agent_src()
    assert src.count("_should_decompose(") == 3                              


def test_an_improving_whole_fn_bundle_is_decomposed() -> None:
    from perf_opt.agent_perf_opt.agent import RewriteAttempt, _should_decompose
    from perf_opt.agent_perf_opt.changeset.types import ChangeSetStatus
    from perf_opt.agent_perf_opt.config import AgentConfig
    extra = {"terminal_status": ChangeSetStatus.REJECTED_W2_REGRESS.value,
             "w2_delta_pct": -5.64}
    assert _should_decompose(RewriteAttempt.W2_REGRESS, extra,
                             ["C3", "III④"], AgentConfig(), set()) is True


def test_a_single_rule_whole_fn_bundle_is_not_decomposed() -> None:
    from perf_opt.agent_perf_opt.agent import RewriteAttempt, _should_decompose
    from perf_opt.agent_perf_opt.changeset.types import ChangeSetStatus
    from perf_opt.agent_perf_opt.config import AgentConfig
    extra = {"terminal_status": ChangeSetStatus.REJECTED_W2_REGRESS.value,
             "w2_delta_pct": -5.64}
    assert _should_decompose(RewriteAttempt.W2_REGRESS, extra,
                             ["C3"], AgentConfig(), set()) is False


def test_the_retry_loop_no_longer_stops_at_the_first_commit() -> None:
    """A commit invalidates the candidates planned against the old source, but
    the region hash refuses those in milliseconds — before the build, W1 and
    W2. Stopping early to dodge that cost discarded every rule after the first
    success, which is how a rewrite worth -30.72% by hand was never tried."""
    src = _agent_src()
    block = _complex_block(src)
    tail = block[block.index("for solo in list(rec.applied_rules)"):]
    assert "rec = solo_rec" in tail
                                       
                                                     
                
    after_commit = tail[tail.index("rec = solo_rec"):]
                              
    stmts = [ln.split("#")[0].strip() for ln in after_commit.split("\n")]
    for i, stmt in enumerate(stmts):
        if stmt != "break":
            continue
        assert "if edit_target is None:" in stmts[:i], (
            "commit 之后的 break 必须由 `edit_target is None` 守着;"
            "无条件 break 会丢掉第一个成功之后的每一条规则")
                        
    assert "result.total_attempts >= cfg.max_candidates" in tail


def test_decomposition_respects_the_candidate_budget(source) -> None:
    block = _complex_block(source)
    assert block.count("result.total_attempts < cfg.max_candidates") >= 1
    assert "result.total_attempts >= cfg.max_candidates" in block


def test_a_rule_with_no_hits_of_its_own_is_skipped(source) -> None:
    """`applied_rules` is what the agent declared; a rule with no hits in this
    function would produce a prompt with an empty evidence section."""
    block = _complex_block(source)
    assert "if not solo_hits:" in block


def test_a_function_left_with_no_matches_ends_cleanly(source: str) -> None:
    """Removing the spent cross-fn matches can empty the hit list entirely —
    a function whose only matches were the cross-fn rule.

    An empty list fails the `fn_entry["hits"]` guard that gates the intra-fn
    path, and the function falls through to the legacy D1B path, which exists
    for functions that had NO hits to begin with and has never run against one
    whose signature was just rewritten. The function is finished; say so.
    """
    block = _cross_fn_block(source)
    assert 'if not fn_entry["hits"]:' in block
    tail = block[block.index('if not fn_entry["hits"]:'):]
    assert "continue" in tail.split("\n\n")[0]


def test_the_intra_fn_path_is_guarded_against_an_unresolved_target(source) -> None:
    """The other way this fall-through can go wrong: a rewritten signature may
    no longer resolve to an edit target. Both downstream branches must handle
    `None` rather than pass it along."""
    # non-large branch
    assert "if edit_target is None:" in source
    # large branch: the region entry point returns a terminal record instead
    region_fn = source[source.index("def _try_large_fn_regions("):
                       source.index("def _try_fn_direct(")]
    assert "if edit_target is None:" in region_fn
    assert "ALL_REGIONS_UNPROVEN" in region_fn


def test_a_solo_commit_reresolves_the_edit_target() -> None:
    """A commit moves the function; the next rule must not splice into the
    span the bundle was planned against.

    `edit_target` carries a byte range resolved once, before the bundle ran.
    The region path re-resolves after every commit and additionally has a
    region hash that refuses a stale candidate before it costs anything. The
    direct path had neither: the span was applied as given, and a span that no
    longer bounds the function leaves part of the old body in place.

    Measured: a C7 grew one function from 25 lines to 48; the III③ queued
    behind it spliced into the old 25-line range three times, each time
    failing the build with `unexpected closing delimiter` — the tail of the
    body it had half-overwritten. Three LLM turns, three builds, and the rule
    was worth several points on that crate.
    """
    src = _agent_src()
    block = _complex_block(src)
    tail = block[block.index("for solo in list(rec.applied_rules)"):]
    after_commit = tail[tail.index("rec = solo_rec"):]
    stmts = "\n".join(ln.split("#")[0] for ln in after_commit.split("\n"))
    assert "resolve_edit_target(" in stmts, (
        "solo 循环在 commit 之后必须重新解析 edit_target")
    assert "build_fn_index(crate)" in stmts, (
        "重新解析要用刷新过的 fn_index,否则拿回来的还是旧 span")
    assert stmts.index("resolve_edit_target(") < (
        stmts.index("break") if "break" in stmts else len(stmts)), (
        "先重新解析,再决定要不要停")
