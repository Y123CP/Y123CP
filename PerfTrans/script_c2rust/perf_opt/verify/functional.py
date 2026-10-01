"""W1 functional verification over the harness_gen golden corpus.

`golden.jsonl` records the harness's stdout digest running against 0_raw (the
c2rust_raw reference) for each (op, corpus input). Functional verification of a
lift/opt target = build the harness against that target and assert every
(op, input) still reproduces the golden stdout sha256. Stage A (safety lift)
and Stage B (perf) both ask this same question, so it lives here, once.

The workload (harness src + corpus + golden.jsonl) is a READ-ONLY shared asset
(WorkloadAssets) — never copied. build_driver stands up a THIN build scaffold
(just the harness's build inputs: Cargo.toml/lock/toolchain/src) pointed at the
crate-under-test; corpus + golden are read IN PLACE from the assets.

  build_driver — thin harness build scaffold against a crate-under-test.
  golden_specs — W1Specs sampled per op (fast per-rewrite inner gate).
  replay_full  — full golden replay (definitive final gate).
"""

from __future__ import annotations

import json
import logging
import tempfile
from collections import defaultdict
from pathlib import Path

from .crate_ops import (_ensure_fair_build, _ensure_harness_compat,
                        _rewire_path_dep, _rsync_copy)
from .w1 import W1Spec
from .workload import WorkloadAssets

logger = logging.getLogger("verify.functional")

# The harness's runtime DATA + coverage junk — a build driver needs NONE of it
# (corpus/golden are read in place from the assets). Excluding it keeps the
# scaffold to a handful of build files instead of copying tens of thousands of
# corpus/coverage files (lz4's .hotcov alone is 6500+).
_WORKLOAD_DATA = ("corpus", "golden.jsonl", "corpus_stats.json", "seeds",
                  "perf_inputs", "covdata", "target-cov", "cov.profdata",
                  ".hotcov", ".perf_sweep")

# harness_gen Cargo.toml always declares `[[bin]] name = "harness"`.
HARNESS_BIN = "harness"


def build_driver(assets: WorkloadAssets, crate_under_test: Path,
                 build_dir: Path) -> Path:
    """Stand up a thin build scaffold in `build_dir` compiling the harness
    against `crate_under_test`, WITHOUT copying the workload's data.

    Copies only the harness's build inputs (Cargo.toml/lock/rust-toolchain/src
    + any build.rs — everything except `_WORKLOAD_DATA`), rewires the
    `<proj>_raw` path-dep onto the crate (package rename, e.g. lz4_raw →
    lz4_cleaned, handled by _rewire_path_dep), and injects harness-compat shims
    into the crate. Returns `build_dir`; its `target/release/harness` is the
    driver binary once the caller builds it via a Verifier bound to build_dir.
    Corpus + golden are NOT here — golden_specs / replay_full read them in place
    from the assets."""
    _rsync_copy(assets.harness_src, build_dir, extra_ignore=_WORKLOAD_DATA)
    _ensure_fair_build(build_dir)
    _rewire_path_dep(build_dir / "Cargo.toml", crate_under_test)
    _ensure_harness_compat(crate_under_test, build_dir)
    return build_dir


def load_golden(golden_path: Path) -> list[dict]:
    rows = []
    for ln in golden_path.read_text(encoding="utf-8").splitlines():
        ln = ln.strip()
        if ln:
            rows.append(json.loads(ln))
    return rows


def golden_specs(assets: WorkloadAssets, *,
                 sample_per_op: int = 8) -> list[W1Spec]:
    """W1Specs from the assets' golden.jsonl, sampled to at most `sample_per_op`
    per op. Corpus input paths resolve IN PLACE against `assets.harness_src`
    (the workload is not copied). Each spec runs `harness <op> <input>`; its
    stdout sha256 must equal golden.

    Sampling is EVENLY-SPREAD (strided) across each op's inputs, not first-N:
    a lift that breaks a contiguous run — or an input-order-correlated fraction
    — of an op's corpus is far more likely to be caught when the sample spans
    the whole corpus. (brotli 2026-07-16: a lift broke ~46% of one op's inputs
    yet 3 first-N samples all landed in the passing 54% and it shipped.)
    Deterministic. sample_per_op <= 0 → every entry.

    Spec names are `<op>:<input_sha8>` so filter_baseline_specs can drop/report
    by op (a stale/tainted op the baseline can't reproduce is dropped, not
    fatal)."""
    rows = load_golden(assets.golden_path)
    by_op: dict[str, list[dict]] = defaultdict(list)
    for r in rows:
        by_op[r["op"]].append(r)
    specs: list[W1Spec] = []
    for op in sorted(by_op):
        op_rows = by_op[op]
        if sample_per_op <= 0 or len(op_rows) <= sample_per_op:
            chosen = op_rows
        else:
            step = len(op_rows) / sample_per_op
            chosen = [op_rows[int(i * step)] for i in range(sample_per_op)]
        for r in chosen:
            specs.append(W1Spec(
                name=f"{op}:{r['input_sha256'][:8]}",
                binary=HARNESS_BIN,
                args=[r["op"], "$INPUT"],
                input_path=(assets.harness_src / r["input_path"]).resolve(),
                sha256=r["stdout_sha256"],
                exit_code=0,
                wrapper=None,
            ))
    return specs


def replay_full(assets: WorkloadAssets, binary: Path, *,
                ops: set[str] | None = None):
    """Full golden replay (every corpus input) via harness_gen.replay — the
    definitive functional gate. Corpus is read IN PLACE from assets.harness_src;
    `binary` is the driver built against the crate-under-test. Returns a
    ReplayReport (.ok / .total / .matched / .mismatches / .errors).

    `ops`: replay only these ops (those that survived the baseline filter), so a
    dropped stale/tainted op doesn't make the final gate spuriously FAIL — a
    FAIL then means a lift genuinely broke a validated path."""
    from harness_gen.replay import replay
    if ops is None:
        return replay(assets.harness_src, binary, assets.golden_path)
    # Filtered golden to a temp file — assets.harness_src is read-only.
    rows = [r for r in load_golden(assets.golden_path) if r["op"] in ops]
    with tempfile.NamedTemporaryFile("w", suffix=".jsonl", delete=False,
                                     encoding="utf-8") as f:
        f.write("".join(json.dumps(r) + "\n" for r in rows))
        tmp = Path(f.name)
    try:
        return replay(assets.harness_src, binary, tmp)
    finally:
        tmp.unlink()
