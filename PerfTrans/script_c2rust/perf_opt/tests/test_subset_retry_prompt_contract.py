"""A subset retry must be a candidate the prompt builder will accept.

A candidate carries two things that have to agree: the rules it declares, and
the anchor hits those rules came from. `build_region_prompt` enforces the
agreement exactly --- `set(anchor rules) != set(candidate rules)` is a hard
`ValueError`.

The decomposition narrowed only the rules. Every subset retry it produced was
therefore malformed, and the first one that ever reached the builder raised.
The exception was not a per-candidate skip: it propagated out of the per-fn
pass, out of the driver, and killed a run **11.5 hours in, four commits deep,
with no final measurement written**.

It survived a full suite because every test of the decomposition asserted on
the TEXT of `agent.py` --- which guard, which status, which flag --- and none
of them built a prompt. So this file constructs a real two-rule region with a
real extractor, decomposes it the way the agent does, and calls the builder.
"""

from __future__ import annotations

from dataclasses import replace as dc_replace
from pathlib import Path

import pytest
from tree_sitter import Language, Parser
import tree_sitter_rust

from perf_opt.agent_perf_opt.prompt_builder import build_region_prompt
from perf_opt.agent_perf_opt.agent import _region_hit_id
from perf_opt.agent_perf_opt.regions import RegionExtractor
from perf_opt.agent_perf_opt.rewrite_applier import EditTarget

_PARSER = Parser(Language(tree_sitter_rust.language()))

SOURCE = """\
pub unsafe fn target(buf: *mut u8, n: usize) -> u32 {
    let mut total: u32 = 0;
    let mut i: usize = 0;
    let limit = n;
    while i < limit {
        total = total.wrapping_add(*buf.offset(i as isize) as u32);
        i = i.wrapping_add(1);
    }
    let scale = n as u32;
    total = total.wrapping_mul(scale);
    total
}
"""


def _walk(node):
    stack = [node]
    while stack:
        cur = stack.pop()
        yield cur
        stack.extend(reversed(cur.children))


def _line_col(needle: str) -> tuple[int, int]:
    for lineno, text in enumerate(SOURCE.splitlines(), start=1):
        if needle in text:
            return lineno, len(text[: text.index(needle)].encode()) + 1
    raise AssertionError(needle)


@pytest.fixture
def setup(tmp_path: Path):
    crate = tmp_path / "crate"
    path = crate / "src" / "lib.rs"
    path.parent.mkdir(parents=True)
    data = SOURCE.encode()
    path.write_bytes(data)

    root = _PARSER.parse(data).root_node
    fn = next(n for n in _walk(root) if n.type == "function_item")
    target = EditTarget(file=path, fn_name="target", span=(fn.start_byte, fn.end_byte))

    hits = []
    for rule, needle in (("III④", "*buf.offset(i as isize)"),
                         ("C3", "let scale = n as u32;")):
        line, col = _line_col(needle)
        hits.append({"rule": rule, "file": "src/lib.rs", "line": line,
                     "col": col, "snippet": needle, "pattern": rule})
    return crate, target, hits


def _decompose(candidate, hits, fn_name):
    """Exactly what the agent does when a bundle is rejected."""
    region = candidate.region
    rule_of = {_region_hit_id(h, region, fn_name): h["rule"] for h in hits}
    out = []
    for solo in candidate.rule_ids:
        keep = [i for i, hid in enumerate(region.anchor_hit_ids)
                if rule_of.get(hid) == solo]
        if not keep:
            continue
        out.append(dc_replace(
            candidate,
            region=dc_replace(
                region,
                anchor_hit_ids=tuple(region.anchor_hit_ids[i] for i in keep),
                anchor_lines=tuple(region.anchor_lines[i] for i in keep),
            ),
            rule_ids=(solo,),
        ))
    return out


def test_the_bundle_itself_builds(setup) -> None:
    """Baseline: the undecomposed candidate was always fine."""
    crate, target, hits = setup
    result = RegionExtractor().extract(crate=crate, edit_target=target, hits=hits)
    assert result.candidates
    for cand in result.candidates:
        build_region_prompt(crate=crate, edit_target=target,
                            region=cand.region, rule_ids=cand.rule_ids, hits=hits)


def test_every_subset_retry_builds(setup) -> None:
    """The regression. Before the fix this raised `ValueError: candidate rules
    must exactly match the region anchor hit rules` and ended the run."""
    crate, target, hits = setup
    result = RegionExtractor().extract(crate=crate, edit_target=target, hits=hits)
    retries = [r for cand in result.candidates
               for r in _decompose(cand, hits, "target")]
    assert retries, "no multi-rule candidate to decompose — fixture is wrong"
    for retry in retries:
        build_region_prompt(crate=crate, edit_target=target,
                            region=retry.region, rule_ids=retry.rule_ids,
                            hits=hits)


def test_a_retry_keeps_only_its_own_rules_anchors(setup) -> None:
    crate, target, hits = setup
    result = RegionExtractor().extract(crate=crate, edit_target=target, hits=hits)
    for cand in result.candidates:
        for retry in _decompose(cand, hits, "target"):
            assert len(retry.rule_ids) == 1
            assert len(retry.region.anchor_hit_ids) <= len(cand.region.anchor_hit_ids)
            assert retry.region.anchor_hit_ids, "a retry with no anchor is malformed"


def test_the_paired_anchor_fields_stay_the_same_length(setup) -> None:
    """`anchor_hit_ids` and `anchor_lines` come from one sequence of anchors
    and are positionally paired; filtering one without the other silently
    misaligns every line with the wrong hit."""
    crate, target, hits = setup
    result = RegionExtractor().extract(crate=crate, edit_target=target, hits=hits)
    for cand in result.candidates:
        for retry in _decompose(cand, hits, "target"):
            assert len(retry.region.anchor_hit_ids) == len(retry.region.anchor_lines)
