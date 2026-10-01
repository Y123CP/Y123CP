# Performance optimization Region flow

Rust functions classified as large by `_is_large_fn` are optimized through
proven, bounded syntax regions. The whole classified-large function is never
sent to the LLM: extraction resolves a concrete `RegionRef`, the prompt
contains that region plus at most 12 lines of read-only context on either side,
and the resulting `ReplaceSourceRegion` operation may edit only the referenced
byte range.

The current classifier selects Region mode when the function spans more than
250 lines or, for functions not caught by that line rule, has more than 25
hits. A pre-existing exception keeps a line-large function on the ordinary
path when at least three located hits occupy less than 30% of its source span;
the Region guarantees in this document apply when `_is_large_fn` returns true.

## Region V1 routing

| Detector rule | V1 route |
| --- | --- |
| `C1`, `C2`, `C3`, `II_vec`, `III③`, `III④` | Region-local prompt and `ReplaceSourceRegion` ChangeSet |
| `II_inl`, `III①` | Cross-function work deferred to ChangeSet V2; excluded from Region extraction and prompts |
| Any unlisted rule | Unsupported and fail-closed |

Each extracted region must fit both limits: at most 80 lines and at most 12
KiB. A hit that cannot be resolved to one bounded, syntactically proven region
is not widened to the function. Candidates are ordered by descending source
offset so edits do not shift pending lower regions. Each candidate that yields
a valid proposal runs the standard build, full W1, and W2 gates. A
planner-abstained candidate or one stopped by the token budget does not reach
those gates. Rejection of one gated candidate does not block a later
independent candidate.

A successful candidate is committed before processing continues. That commit
invalidates every remaining `RegionRef`: the function index and edit target are
rebuilt, and remaining hits are re-extracted from the committed source. Stale
byte offsets or hashes are never reused after a commit.

## Fail-closed outcomes

Region routing records exact machine-readable reasons:

- `only_cross_fn_rules` is the reason when all actionable hits require
  cross-function work; its terminal status is
  `cross_fn_requires_changeset_v2`.
- `no_region_local_rules` means no supported Region-local hit was available.
- `all_regions_unproven` means extraction could not prove a safe region.
- `all_regions_over_budget` means every otherwise relevant region exceeded an
  extraction boundary.
- `planner_abstained` means the response could not produce a valid, bounded
  Region operation.
- `budget_exceeded` means the per-function LLM token limit prevented another
  Region attempt.

Unsupported, cross-function, stale, conflicting, malformed, or out-of-region
changes are rejected rather than guessed from exception text or widened to a
larger edit. In particular, Region V1 does not implement edits spanning
multiple functions.

## Audit and verification

`<opt_dir>/rewrites.log` is the attempt JSONL stream. Region candidate rows
include `anchor_hit_ids`, `attempt_trace`, and `terminal_status`.
`<opt_dir>/changesets/<id>/events.jsonl` records state transitions, while
`<opt_dir>/changesets/<id>/result.json` stores the resolved operation,
`RegionRef` hashes and anchors, concrete edits, gates, and terminal result.
`<opt_dir>/changesets.jsonl` is the ChangeSet summary stream.

Run the focused Region suite from the repository root:

```bash
env PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=script_c2rust \
  script_c2rust/.venv/bin/python -m pytest \
  script_c2rust/perf_opt/tests/test_large_function_regions.py -q
```
