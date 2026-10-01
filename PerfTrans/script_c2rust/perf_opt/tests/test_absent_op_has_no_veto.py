"""An op missing from a non-empty `library_share` map must lose its veto.

`_op_limits` read `self.library_share.get(op, 1.0)`. The default said "assume
this op is 100% library", so an op the map does not mention kept the tightest
standing there is. That is exactly inverted: `locate` writes a `per_op` entry
for every hot function it charges, so an op appearing nowhere in
`hotspots.json` carried NO hot function at all — the strongest form of the
very condition the floor tests.

Measured on libxml2. `globals_threads_memory` carries no hot function, yet it
vetoed five separate rewrites that were 4.9-5.8% net FASTER, reporting
+8.2-8.7% every time. The five edits were `xmlURIUnescapeString` (twice),
`xmlFileOpenW`, `xmlRegStateAddTrans` and `xmlParserPrintFileContextInternal`
— URI unescaping, file opening, regex state transitions and error printing.
Not one of them is on that op's hot path, and five unrelated edits do not
produce the same +8.6% by coincidence; what moved was code layout.

The empty map keeps meaning "could not measure" — hotspots.json missing or
unreadable — and there every op keeps full standing, as
`_measured_library_share`'s docstring promises. That distinction is the whole
fix: absent-from-a-map is evidence, absent-map is ignorance.
"""

from __future__ import annotations

import math

from perf_opt.agent_perf_opt.config import AgentConfig
from perf_opt.agent_perf_opt.gates import W2Session, _LIBRARY_SHARE_FLOOR


class _Session:
    bystander_op_upper_limit_pct = AgentConfig.w2_bystander_op_upper_limit_pct
    primary_op_share_pct = AgentConfig.w2_primary_op_share_pct
    catastrophic_regress_pct = AgentConfig.w2_catastrophic_regress_pct
    library_share: dict = {}
    _op_limits = W2Session._op_limits


OPS = ["html_entity_encode", "globals_threads_memory"]
# The rewritten function lives in the first op, so only it would otherwise
# keep the tight ceiling; the weights below are not what is under test.
WEIGHTS = {"html_entity_encode": 40.0, "globals_threads_memory": 0.0}


def _session(share: dict) -> _Session:
    s = _Session()
    s.library_share = share
    return s


def test_op_absent_from_a_non_empty_map_is_disarmed() -> None:
    limits = _session({"html_entity_encode": 0.91})._op_limits(OPS, WEIGHTS)
    assert limits["globals_threads_memory"] == math.inf


def test_the_measured_op_in_the_same_map_keeps_its_standing() -> None:
    limits = _session({"html_entity_encode": 0.91})._op_limits(OPS, WEIGHTS)
    assert limits.get("html_entity_encode") != math.inf


def test_an_empty_map_still_leaves_every_op_armed() -> None:
    """Absent MAP is ignorance, not evidence — nothing may be disarmed."""
    limits = _session({})._op_limits(OPS, WEIGHTS) or {}
    assert all(v != math.inf for v in limits.values())


def test_a_measured_op_below_the_floor_is_still_disarmed() -> None:
    """The pre-existing behaviour this fix must not disturb."""
    share = {"html_entity_encode": 0.91,
             "globals_threads_memory": _LIBRARY_SHARE_FLOOR / 2}
    limits = _session(share)._op_limits(OPS, WEIGHTS)
    assert limits["globals_threads_memory"] == math.inf


def test_a_measured_op_above_the_floor_keeps_its_veto() -> None:
    """zopfli's cache_roundtrip at 52.4% must NOT be disarmed by this change."""
    share = {"html_entity_encode": 0.91, "globals_threads_memory": 0.524}
    limits = _session(share)._op_limits(OPS, WEIGHTS)
    assert limits.get("globals_threads_memory") != math.inf


def test_absence_is_read_per_op_not_per_map() -> None:
    """One measured op is enough to make the map evidence for all of them."""
    share = {"globals_threads_memory": 0.91}      # the OTHER op is the absent one
    limits = _session(share)._op_limits(OPS, WEIGHTS)
    assert limits["html_entity_encode"] == math.inf
    assert limits.get("globals_threads_memory") != math.inf
