"""Shared verification module — functional (W1) and performance (W2).

Verification is a cross-cutting concern: Stage A (safety lift), Stage B
(perf), and any stage that rewrites a crate must answer two questions —
"did it stay correct?" (functional / W1) and "did it get faster?"
(performance / W2). Historically the W1 logic was duplicated across
main.py (`_w1_run`), stage_a/verifier.py (`Verifier.w1`), the three
standalone lift drivers (`_w1_specs` + baseline filter + `_w1_suite`),
and harness_gen/replay.py (golden replay). This package is the single
home for the shared pieces.

Layout:
  cargo.py     — GateResult + Verifier (cargo check / build + W1 run).
                 The verification primitive; moved here from stage_a so
                 verify/ is the independent foundation.
  crate_ops.py — crate/harness working-copy + fair-build + path-dep-rewire
                 + harness-compat primitives (moved from bench_pipeline).
  w1.py        — source-agnostic W1 primitives: W1Spec, run_w1_suite,
                 filter_baseline_specs. Consume a Verifier + specs; do
                 not know where the specs came from.
  workload.py  — the spec SOURCE for the dataset_trans working set:
                 discover the harness_gen golden harness and derive W1
                 specs from golden.jsonl. (golden wiring lands next.)
  measure.py   — shared W2 measurement primitives: Measurement, perf_stat
                 (counting, -r N low-cv wall), autotune_iters, pick_input.
                 Used by w2.py AND perf_opt.hot_probe (one scaffolding).
  w2.py        — the W2 performance gate: W2Verdict + w2_gate (cv-aware
                 tolerance + anti-drift + direct-card real-gain requirement).
                 Wall-clock is the sole verdict; counters are reporting-only.

verify/ depends only on harness_gen (replay + corpus/golden assets), the
cleanup/toolchain leaf utils, and cargo — never on stage_a / stage_b /
bench_pipeline (those depend DOWN on verify/). measure.py is self-contained
(stdlib + perf), NOT importing bench_pipeline, to keep that direction. Functional
verification uses harness_gen.replay for full golden replay; w1.py drives the
sampled inner gate over the same corpus.
"""

from .cargo import GateResult, Verifier
from .functional import build_driver, golden_specs, replay_full
from .measure import (Measurement, autotune_iters, measure_harness,
                      perf_stat, pick_input)
from .w1 import W1Spec, filter_baseline_specs, run_w1_suite
from .w2 import W2Verdict, w2_gate
from .workload import WorkloadAssets

__all__ = ["GateResult", "Verifier",
           "W1Spec", "run_w1_suite", "filter_baseline_specs",
           "WorkloadAssets", "build_driver", "golden_specs", "replay_full",
           "Measurement", "perf_stat", "measure_harness", "autotune_iters",
           "pick_input", "W2Verdict", "w2_gate"]
