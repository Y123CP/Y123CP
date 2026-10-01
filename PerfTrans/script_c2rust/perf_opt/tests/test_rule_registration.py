"""Every rule that can fire must have somewhere to go.

A detector writes its hits into `fn_hits.json`; the region extractor then asks
`rule_capability(rule_id)` how to route them. An unregistered rule answers
UNSUPPORTED, and every one of its hits is dropped before it can become a
candidate — no warning, no changeset, nothing in the run summary except a
slightly lower attempt count.

That is what happened to III②. Its detector ran, its card existed, the
reporting layer listed it among applied rules — and the routing table had
never heard of it. Measured across the dataset at the time: 38 hits on one
compression crate (two of them in functions holding 40% and 7% of self time),
16 on another, 9 on a third. All discarded.

The failure is invisible by construction, so it needs a test that compares the
two lists mechanically rather than a reviewer noticing a missing line.
"""

from __future__ import annotations

import re
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.regions.model import (
    RuleCapability,
    _RULE_CAPABILITIES,
    rule_capability,
)

CARDS = (Path(__file__).resolve().parents[1] / "agent_perf_opt"
         / "Optimization_Card")

# Rules deliberately absent from the routing table because they own a
# dedicated planner and never reach the region extractor. Each entry names the
# planner, so "it has its own path" stays a checkable claim rather than an
# excuse for the next omission.
DEDICATED_PLANNERS = {
    "II_const": "perf_opt/agent_perf_opt/planners/ii_const.py",
    "C9": "perf_opt/agent_perf_opt/planners/bitfield.py",
    # Its rewrite is a bijection over one variable's state constants plus the
    # carrier type — no control flow moves, so there is nothing for the model
    # to decide and nothing for the region extractor to route.
    "C11": "perf_opt/agent_perf_opt/planners/goto_dispatch.py",
    # One inlining attribute on a named function: on the callee (II_inl) or on
    # the hot loop kernel itself (II_iso). No model reply to route or verify.
    "II_inl": "perf_opt/agent_perf_opt/planners/inline_attr.py",
    "II_iso": "perf_opt/agent_perf_opt/planners/inline_attr.py",
}


def _card_rule_ids() -> dict[str, str]:
    """{rule id: card filename} for every optimization card."""
    out: dict[str, str] = {}
    for path in sorted(CARDS.glob("*.md")):
        if path.name == "README.md" or path.name.startswith("TMA"):
            continue
        match = re.search(r"^# Rule ([^\s:]+)", path.read_text(encoding="utf-8"),
                          re.M)
        if match:
            out[match.group(1)] = path.name
    return out


def test_every_card_rule_is_routable() -> None:
    """A card without a route is a rule that can never fire."""
    unrouted = [
        f"{rid} ({card})"
        for rid, card in _card_rule_ids().items()
        if rid not in _RULE_CAPABILITIES and rid not in DEDICATED_PLANNERS
    ]
    assert not unrouted, (
        "these rules have a card and a detector but no entry in "
        "_RULE_CAPABILITIES, so the extractor silently drops every hit:\n  "
        + "\n  ".join(unrouted))


def test_the_dedicated_planner_exemptions_are_real() -> None:
    """Guard the exemption list: a planner that moved or was deleted would
    turn this allowance back into the very hole it documents."""
    root = Path(__file__).resolve().parents[2]
    for rule, planner in DEDICATED_PLANNERS.items():
        assert (root / planner).is_file(), f"{rule}: {planner} not found"
        assert rule_capability(rule) is RuleCapability.UNSUPPORTED, (
            f"{rule} now has a routing entry — drop it from DEDICATED_PLANNERS")


def test_every_routed_rule_has_a_card() -> None:
    """The other direction: routing a rule the model has no instructions for
    produces a prompt that asks for an unspecified rewrite."""
    cards = _card_rule_ids()
    assert not [r for r in _RULE_CAPABILITIES if r not in cards]


# ─────────────────────────────────────────── III② specifically

def test_iii2_is_region_local() -> None:
    """Its own card abstains on every shape that would need a caller change —
    "the allocation escapes the function", "alloc and free are in different
    functions" — so what is left is a body-local rewrite."""
    assert rule_capability("III②") is RuleCapability.REGION_LOCAL


def test_iii2_hits_are_no_longer_dropped_as_unsupported() -> None:
    from perf_opt.agent_perf_opt.regions.model import RuleCapability as RC
    assert rule_capability("III②") is not RC.UNSUPPORTED


@pytest.mark.parametrize("rule", ["C1", "C3", "C4", "III③", "III④", "II_vec"])
def test_known_region_local_rules_stay_region_local(rule) -> None:
    assert rule_capability(rule) is RuleCapability.REGION_LOCAL


@pytest.mark.parametrize("rule", ["III①"])
def test_cross_function_rules_stay_cross_function(rule) -> None:
    """These genuinely need a caller-side change; region-routing them would
    produce rewrites the extractor cannot apply."""
    assert rule_capability(rule) is RuleCapability.CROSS_FUNCTION


def test_ii_inl_is_not_routed_to_the_model() -> None:
    """II_inl's edit is one attribute on the CALLEE. Routed as a cross-function
    LLM rule it could never land — every reply is locked to the hot function —
    and it never did, in any project. It now runs as a dedicated planner, so
    the routing table must not offer it to the extractor at all."""
    assert "II_inl" in DEDICATED_PLANNERS
    assert rule_capability("II_inl") is RuleCapability.UNSUPPORTED


def test_an_unknown_rule_is_still_unsupported() -> None:
    """The default must stay conservative — a typo in a detector must not
    silently acquire a route."""
    assert rule_capability("III⑨") is RuleCapability.UNSUPPORTED
    assert rule_capability("") is RuleCapability.UNSUPPORTED


# ═══════════════════════════════════════════════════════════════════════
# The other registration points
#
# A rule only works if it is declared in EVERY layer that touches it. The
# layers are independent files, so the failure mode is not "the rule is
# broken" but "the rule is absent from one table", and absence is silent by
# construction. These tests compare the tables mechanically.
# ═══════════════════════════════════════════════════════════════════════

def _detector_rules() -> set[str]:
    """Rule ids the hot_probe detectors can emit into `fn_hits.json`."""
    root = Path(__file__).resolve().parents[1] / "hot_probe"
    found: set[str] = set()
    pattern = re.compile(r'rule\s*=\s*["\']([^"\']+)["\']|"rule"\s*:\s*"([^"]+)"')
    for path in root.rglob("*.py"):
        for a, b in pattern.findall(path.read_text(encoding="utf-8")):
            rule = a or b
            if rule and not rule.startswith(("{", "%")):
                found.add(rule)
    return found


def _routable(rule: str) -> bool:
    return rule in _RULE_CAPABILITIES or rule in DEDICATED_PLANNERS


def test_every_detectable_rule_can_be_routed() -> None:
    """The III② failure, generalised.

    A detector that emits hits for a rule the extractor cannot route produces
    the worst kind of dead code: the scan runs, the hits are written, the
    reporting layer lists the rule among those that fired, and every hit is
    dropped without a log line. Measured: 38 hits on one crate, in functions
    holding 40% and 7% of self time.
    """
    orphans = sorted(r for r in _detector_rules() if not _routable(r))
    assert not orphans, (
        "these rules have detectors but no route — their hits are discarded "
        f"silently: {orphans}")


def test_rule_order_only_lists_rules_that_exist() -> None:
    """`rule_order` drives the W1 fallback priority. A name here that no
    detector emits and no card documents is dead configuration."""
    from perf_opt.agent_perf_opt.config import AgentConfig
    cards = _card_rule_ids()
    unknown = [r for r in AgentConfig.rule_order
               if r not in cards and r not in DEDICATED_PLANNERS]
    assert not unknown, f"rule_order names rules with no card/planner: {unknown}"


def test_every_llm_rule_appears_in_rule_order() -> None:
    """A rule the LLM path can apply but that `rule_order` omits has no
    defined priority when the fallback has to pick one."""
    from perf_opt.agent_perf_opt.config import AgentConfig
    missing = [r for r in _RULE_CAPABILITIES if r not in AgentConfig.rule_order]
    assert not missing, f"routed rules missing from rule_order: {missing}"


def test_execution_fingerprints_only_cover_documented_rules() -> None:
    """The fingerprint table decides whether a rule counts as really applied.
    An entry for a rule with no card can only ever mis-attribute."""
    from perf_opt.agent_perf_opt.rewrite_applier import _RULE_EXEC_FINGERPRINTS
    cards = _card_rule_ids()
    unknown = [r for r in _RULE_EXEC_FINGERPRINTS if r not in cards]
    assert not unknown, f"fingerprints for undocumented rules: {unknown}"


def test_deterministic_planners_need_no_card_or_fingerprint() -> None:
    """They emit fixed code, so there is nothing to instruct a model with and
    nothing to verify a model against. Asserted so the exemption is a stated
    property rather than an accident of omission."""
    from perf_opt.agent_perf_opt.rewrite_applier import _RULE_EXEC_FINGERPRINTS
    for rule in DEDICATED_PLANNERS:
        assert rule not in _RULE_CAPABILITIES
        assert rule not in _RULE_EXEC_FINGERPRINTS


def test_the_layers_agree_on_the_llm_rule_set() -> None:
    """One assertion for the whole invariant: for rules that go through the
    LLM, card / rule_order / routing table must describe the same set."""
    from perf_opt.agent_perf_opt.config import AgentConfig
    cards = set(_card_rule_ids()) - set(DEDICATED_PLANNERS)
    order = set(AgentConfig.rule_order) - set(DEDICATED_PLANNERS)
    routed = set(_RULE_CAPABILITIES)
    assert cards == order == routed, (
        f"card-only={sorted(cards - routed)} "
        f"order-only={sorted(order - routed)} "
        f"routed-only={sorted(routed - cards)}")
