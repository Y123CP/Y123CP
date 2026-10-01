"""Admit functions whose rule payoff is per-call fixed cost, not self-time.

`hot(s)` — self-time ≥ tau — is the right filter for every rule whose win
scales with how long the function runs: hoist a load out of a hot loop and
you get back a share of that loop's time. It is the wrong filter for C12.

C12 is an oversized eager zero-init: c2rust cannot say "uninitialized", so a
`struct S s;` becomes a full zero literal and the binary runs a memset the C
original never ran. That memset costs the same whether the call processes
48 KB or 200 MB, so as a *share* of a large-input profile it rounds to
nothing — and the function holding it (`*_init`, `*_reset`, `*Clear*`) never
clears tau. The cost is real all the same, and on small inputs it dominates.

Measured on lz4 (validation workload, canonical codegen, vs the C build):

    compress_hc      / silesia-xml       raw +0.04%
    compress_fast    / silesia-dickens   raw +2.88%
    compress_fast    / xml-48k           raw +23.94%      <-- 48 KB input

The gap explodes as the input shrinks, which is the signature of a fixed
per-call cost, and lz4's C12 hits sit exactly where you would predict:
`LZ4_prepareTable` (16412 B x4) and `LZ4_compress_fast` (16416 B x8). None of
those functions is hot. Across the 12-project set, 296 project-side C12 sites
were detected and 9 reached the agent; the other 287 were filtered by a
predicate that cannot see this kind of cost.

The fix is NOT to loosen tau. Loosening it admits ~1992 genuinely cold
functions across the set and spends the LLM budget on them. Instead this
module applies a predicate matched to the rule's shape:

  * the site is a C12 hit in the crate under test (not stdlib, not the harness)
  * the memset is at least `MIN_BYTES` — bigger than the reporting floor the
    detector uses, because we are now proposing to spend a rewrite on it
  * the workload actually reaches the function — it appears in the profile's
    sub-tau table. A function the profile never saw is dead code under this
    workload and a rewrite there is unmeasurable by construction.
  * at most `MAX_ADMIT` per project, ranked by total zeroed bytes

Admitted functions carry `admit_reason="fixed_cost"` so nothing downstream,
or anyone reading hotspots.json, mistakes them for self-time hot functions.
Their `per_op` comes from the profile's sub-tau table, so W2 gates them on
the ops that really execute them rather than on a guessed op set. W1 and W2
remain the arbiters — this module only decides who gets asked.
"""

from __future__ import annotations

import logging
from pathlib import Path

from perf_opt.hot_probe.types import HotFunction

logger = logging.getLogger("hot_probe.fixed_cost_admit")

# Above the detector's 256 B reporting floor. 256 B is where the memset call
# becomes visible in IR; it is not where a rewrite is worth an LLM round and a
# pair of W2 measurements. 1 KiB is 16 cache lines.
MIN_BYTES = 1024

# A cap, not a target. The point is to stop the fixed-cost blind spot from
# hiding a 16 KB memset, not to hand the agent a second work queue.
MAX_ADMIT = 6

_RULE = "C12"


def admit_fixed_cost_fns(
    hotspots: list[HotFunction],
    *,
    class_i_result,
    sub_tau_profile: dict[str, dict[str, float]],
    fn_index,
    min_bytes: int = MIN_BYTES,
    max_admit: int = MAX_ADMIT,
) -> tuple[list[HotFunction], dict[str, dict[str, int]]]:
    """Return (extra `HotFunction`s, their rule hits). Never mutates inputs.

    The second element exists because `ScanResult.c1_c2_hits_by_fn` was pruned
    against the *old* hot set, so an admitted function would otherwise arrive
    at the agent with an empty `class_i_hits` — carrying the C12 evidence that
    justified admitting it is the caller's job, and this hands it over.

    Args:
      hotspots         — the self-time hot list; used only to avoid duplicates
      class_i_result   — a `class_I.ScanResult` carrying `.sites` and `.tables`
                         (the raw site list, which is NOT pruned by hot(s))
      sub_tau_profile  — {fn: {op: self%}} for fns the profile saw below tau,
                         from `hotspots.json["sub_tau_profile"]`
      fn_index         — `symbol_source.build_fn_index(crate)`, for locating
                         the function's source range
    """
    sites = getattr(class_i_result, "sites", None) or []
    tables = getattr(class_i_result, "tables", None)
    if not sites or tables is None:
        return [], {}

    from perf_opt.hot_probe.class_I.attribute import (
        STDLIB_ONLY, UNRESOLVED, attribute_site,
    )

    already = {hf.name for hf in hotspots}
    # fn -> [total bytes, n sites]
    weight: dict[str, list[int]] = {}
    for s in sites:
        if s.rule_id != _RULE:
            continue
        try:
            nbytes = int(s.detail)
        except (TypeError, ValueError):
            continue
        if nbytes < min_bytes:
            continue
        a = attribute_site(s, tables)
        fn = a.define_fn
        if fn in (STDLIB_ONLY, UNRESOLVED) or not fn or fn in already:
            continue
        w = weight.setdefault(fn, [0, 0])
        w[0] += nbytes
        w[1] += 1

    if not weight:
        return [], {}

    hits: dict[str, dict[str, int]] = {}
    admitted: list[HotFunction] = []
    skipped_unprofiled: list[str] = []
    skipped_unlocatable: list[str] = []
    for fn, (total, n) in sorted(weight.items(), key=lambda kv: -kv[1][0]):
        if len(admitted) >= max_admit:
            break
        per_op = sub_tau_profile.get(fn)
        if not per_op:
            # Never sampled under this workload. Its memset may well be
            # expensive somewhere, but not here, and W2 could not measure the
            # rewrite even if the LLM wrote a perfect one.
            skipped_unprofiled.append(fn)
            continue
        loc = fn_index.resolve(fn)
        if loc is None:
            # No source range → the evidence pack would be empty. Same rule as
            # `locate`'s unresolved-symbol drop.
            skipped_unlocatable.append(fn)
            continue
        hottest = max(per_op.items(), key=lambda kv: kv[1])
        admitted.append(HotFunction(
            name=fn,
            self_pct=round(hottest[1], 2),
            hottest_op=hottest[0],
            per_op=dict(per_op),
            file=loc[0], line_start=loc[1], line_end=loc[2],
            admit_reason="fixed_cost",
        ))
        hits[fn] = {_RULE: n}
        logger.info("[fixed_cost_admit] + %s — %s zeroed %d B over %d site(s), "
                    "self-time only %.2f%% (below tau)",
                    fn, _RULE, total, n, hottest[1])

    if skipped_unprofiled:
        logger.info("[fixed_cost_admit] skipped %d fn(s) with %s hits the "
                    "workload never reached: %s",
                    len(skipped_unprofiled), _RULE, skipped_unprofiled[:8])
    if skipped_unlocatable:
        logger.warning("[fixed_cost_admit] skipped %d fn(s) with %s hits and "
                       "no source location: %s",
                       len(skipped_unlocatable), _RULE, skipped_unlocatable[:8])
    logger.info("[fixed_cost_admit] admitted %d/%d candidate fn(s) (cap %d)",
                len(admitted), len(weight), max_admit)
    return admitted, hits
