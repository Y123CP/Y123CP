# PerfTrans — Reviewer Artifact

This artifact accompanies the paper on the runtime performance of mechanical
C-to-Rust translation (c2rust) and **PerfTrans**, a rule-guided approach that
recovers translation-induced performance loss.

It contains (1) the full implementation of PerfTrans and (2) the data, inputs,
build scripts and raw measurements behind every research question.

| Folder | Paper section | What it answers |
|---|---|---|
| [`forRQ1/`](#forrq1--performance-divergence) | §Study, RQ1 | How much slower/faster is c2rust output than C? |
| [`forRQ2/`](#forrq2--cost-decomposition) | §Study, RQ2 | Is the gap extra instructions (R_inst) or higher CPI (R_CPI)? |
| [`forRQ3/`](#forrq3--performance-issue-patterns) | §Study, RQ3 | Nine recurring performance issues, their optimization directions, and supporting analysis |
| [`forRQ4/`](#forrq4--optimization-effectiveness) | §Evaluation, RQ4 | PerfTrans vs. Raw vs. Codex on 12 projects / 41 operations |
| [`forRQ5/`](#forrq5--ablation-study) | §Evaluation, RQ5 | Refined and rule-free ablation; accepted-rewrite analysis |
| [`PerfTrans/`](#perftrans--implementation) | §Approach | Complete source code of PerfTrans |

Every table in the paper can be regenerated from the raw JSON files here with
one script per RQ (no special hardware needed). Re-*measuring* the numbers needs the
toolchain and machine setup described in [Environment](#environment).

---

## Restore nested Git histories after downloading

The repository contains the artifact files directly. Nested Git metadata is
stored separately in `artifact_support/git_histories/` so GitHub clones and
ZIP downloads include the complete anonymized experiment histories rather
than unresolved nested-repository links.

Run once from the artifact root with Python >= 3.8 and Git:

```bash
python3 artifact_support/restore_git_histories.py
```

This verifies archive checksums and restores 66 nested `.git` directories.
It preserves the published working files, including their release edits;
it does not check out old source versions or rerun experiments. Existing
histories are verified and never overwritten. RQ5 scripts that inspect
per-commit rewrites require this step. To verify the restored histories later,
run the same command with `--check`.

---

## Quick check: regenerate every table in about one minute

Only Python ≥ 3.9 is needed (standard library only).

```bash
cd forRQ1 && python3 scripts/summarize_rq1.py && cd ..      # Table: RQ1 overhead
cd forRQ2 && python3 scripts/summarize_rq2.py && cd ..      # Table: RQ2 cost decomposition + TMA
cd forRQ4 && python3 scripts/summarize_rq4_rq5.py && cd ..  # Table: evaluation (RQ4 + RQ5)
```

Expected console output (matches the paper):

```
forRQ1: 19 operations, 12 regressions > 3%
forRQ2: 12 regressions: 5 Work / 3 CPI / 4 Both
forRQ4: 41 table rows from 12 projects
        RQ4  PerfTrans faster than Raw on 40/41 rows
        RQ4  PerfTrans faster than C on 18 rows
        RQ4  Codex n/e on 4 rows; vs Codex on 37 comparable rows: win 31 / lose 5 / tie 1
        RQ5  PerfTrans faster than w/o-rules on 34/41 rows
```

The CSV files are written to each folder's `results/`. Pre-generated copies are
already there.

---

## Benchmarks

20 C projects: an 8-project **Study Dataset** (RQ1–RQ3, rule derivation) and a
disjoint 12-project **Evaluation Dataset** (RQ4–RQ5).

| Dataset | Project | KLoC | Operations |
|---|---|---:|---|
| Study | libcsv | 1.0 | Parsing; Field escaping |
| Study | libogg | 2.9 | Stream encoding; Decoding |
| Study | xxHash | 3.9 | XXH64; XXH3; XXH32 |
| Study | libzahl | 5.7 | Multiplication; Long division; Modular exponentiation |
| Study | bzip2 | 5.9 | Compression; Decompression |
| Study | binn | 6.4 | Serialization; Deserialization |
| Study | json-c | 6.9 | Parsing; Serialization |
| Study | heman | 13.8 | Distance field; Lighting; Terrain generation |
| Eval | libopenaptx | 1.3 | aptX encoding; aptX decoding; Stream recovery |
| Eval | http-parser | 2.5 | Request / response / incremental / URL parsing |
| Eval | fzy | 2.6 | Fuzzy scoring; Choice-list search; Candidate-list parsing |
| Eval | zopfli | 2.9 | DEFLATE compression |
| Eval | lil | 3.6 | Script execution; Embedding API |
| Eval | miniz | 4.2 | Compression; Decompression; Streaming; Checksums; ZIP processing |
| Eval | lz4 | 5.9 | Compression; Decompression; Streaming; Dictionary modes |
| Eval | lodepng | 5.9 | PNG decoding / encoding; Color conversion; zlib; Chunk processing |
| Eval | libqrencode | 6.7 | QR encoding; Micro-QR encoding; Structured append |
| Eval | optipng | 9.2 | PNG optimization; Format conversion |
| Eval | brotli | 16.0 | Compression; Decompression; Streaming; Dictionary modes |
| Eval | libxml2 | 180.0 | Parsing; Validation; XPath; Serialization; Dict/URI/string utilities |

Each operation's exact command line, inputs and expected output digest
(`stdout_sha256`) are listed in the project's `study.toml` (see below).

---

## forRQ1 — Performance Divergence

> *How does mechanical C-to-Rust translation affect runtime performance?*

```
forRQ1/
├── empirical_study/
│   ├── run_study.py              # measurement driver (paired C/Rust runs, CV gate, IQR filter)
│   └── <project>/                # binn bzip2 heman json-c libcsv libogg libzahl xxHash
│       ├── study.toml            # operations, inputs, oracle digests, build commands, protocol
│       ├── c_ref/                # C sources + build.sh (clang-17, -O3 -flto)
│       ├── rust_raw/             # unmodified c2rust 0.22.1 output (Cargo crate)
│       ├── rust_harness/         # Rust driver, line-by-line equivalent of the C driver
│       ├── inputs/               # workload inputs (Silesia, Native JSON Benchmark, generated)
│       ├── bin/                  # prebuilt C reference binary
│       └── rq2_perf.json         # RAW RESULT: wall-clock medians, CVs, perf counters, TMA
├── perf_run.sh                   # environment guard: performance governor, turbo off, core pin, RDT
├── dataset_trans/rdt_isolate.sh  # Intel RDT (L3 / memory-bandwidth) isolation used by perf_run.sh
├── scripts/summarize_rq1.py      # rebuilds the RQ1 table
└── results/
    ├── rq1_per_operation.csv     # = paper Table (RQ1), 19 operations
    └── rq1_per_input.csv         # every (operation, input) pair
```

**Metric.** `overhead = (t_Rust − t_C) / t_C`, `t` = median of 30 paired
measurements after 5 warm-ups and Tukey 1.5×IQR outlier removal; a batch
with CV > 3% is re-measured (≤ 2 retries). Per-operation overhead = geometric
mean over the operation's inputs.

**Re-measure one project** (Linux, root-less once the sudoers rule in
`perf_run.sh` is set; run from `forRQ1/`):

```bash
# 1. build both sides: run the two `build_cmd` entries of study.toml ([builds.c], [builds.c2rust_raw])
bash empirical_study/<project>/c_ref/build.sh
# 2. measure (wall clock + perf counters + TMA for RQ2)
bash perf_run.sh python3 empirical_study/run_study.py empirical_study/<project>/study.toml --perf --out new_rq2_perf.json
```

Every run first checks that the C and Rust outputs have the recorded
`stdout_sha256`, so only functionally equivalent runs are timed.

---

## forRQ2 — Cost Decomposition

> *What execution costs underlie the observed performance differences?*

```
forRQ2/
├── data/<project>/rq2_perf.json  # same co-measured session as RQ1
├── scripts/
│   ├── summarize_rq2.py          # rebuilds the RQ2 table and TMA breakdown
│   ├── perf_stat.py              # instruction / cycle collection (perf stat, 3 runs, median)
│   └── tma.py                    # top-down microarchitecture analysis (pmu-tools toplev, level 1)
└── results/
    ├── rq2_cost_decomposition.csv  # = paper Table (RQ2), the 12 regressions
    └── rq2_tma.csv                 # TMA shares and per-dimension ΔCPI contributions
```

Fields in `rq2_perf.json`: `inst_ratio` = R_inst = I_Rust/I_C, `cpi_ratio` =
R_CPI = CPI_Rust/CPI_C, `inst_x_cpi` = R_inst·R_CPI (self-check against the
wall-clock ratio), and for operations with R_CPI ≥ 1.05: `tma_c`, `tma_raw`,
`dcpi` (ΔCPI contribution of Retiring / Frontend / Bad Speculation / Memory /
Core). Example: xxHash XXH32 → `dcpi.Mem = 0.50`, `dcpi.Core = 0.48`.

Cost label: *Work* = R_inst ≥ 1.05; *CPI* = R_CPI ≥ 1.05; *Both* = both.

---

## forRQ3 — Performance Issue Patterns

RQ3 identifies nine recurring performance issues and their corresponding
optimization directions. Start with the [RQ3 reading guide](forRQ3/README.md).

```
forRQ3/
├── README.md                     # paper terminology and reading guide
├── analysis_records/
│   ├── class_I/<project>.md      # optimized-IR comparison (C vs Rust) of hot functions
│   ├── class_II/<project>.md     # LLVM optimization-remark comparison
│   ├── class_III/               # source-level inspection
│   └── */cluster.md             # working records of evidence grouping
├── performance_issues/          # nine findings from the final paper table
│   └── README.md                # issue catalog; each entry has a pattern and direction
└── scripts/                     # evidence collection and attribution utilities
    ├── opt_remarks_collector.py  # source-mapped LLVM remarks (C and Rust)
    ├── perf_annotate.py          # hot-function ranking (≥ 80% of sampled cycles)
    ├── source_patterns.py       # Rust source-pattern scanner
    └── facet_coverage.py        # implementation ID → optimization-direction mapping
```

The analysis records are working notes, provided in English. They preserve
observations, collection-method corrections, and intermediate groupings.
Their historical Class I/II/III labels and rule counts should not be read as
the final paper taxonomy. The paper presents two residual-runtime issues,
two missed-compiler-transformation issues, and five source-level issues.

In the approach, an **optimization rule** pairs an **Observed Pattern** with
an **Optimization Direction**. Adding **Rewrite Preconditions** produces an
**optimization card**. The `performance_issues/` directory presents the nine
RQ3 findings. The 18 implementation card templates, including their rewrite
preconditions, are kept in PerfTrans's `Optimization_Card/` directory.

**Paper performance issues and implementation IDs**
(`scripts/facet_coverage.py`, `RULE_OF`):

| Group | Performance issue in the paper | Implementation IDs |
|---|---|---|
| Residual runtime work | Residual Bounds Checks | C1 |
| | Redundant Conversion Saturation | C2 |
| Missed compiler transformations | Missed Loop Vectorization | II_vec |
| | Missed Hot-Callee Inlining | II_inl (+ II_iso) |
| Recurring source-level patterns | Repeated Invariant Dispatch | III1, C6, C11 |
| | Inefficient Buffer Management | C7, III2 |
| | Byte-Oriented Memory Operations | III3, C9, C12 |
| | Scalar Element-Wise Processing | C4, C5, C8, C10 |
| | Obscured Memory Access Properties | C3, III4, II_const |

The mapping groups implementation IDs for attribution; it does not establish
an additional paper issue for each template or historical variant. `C9` and
`II_iso` have deterministic implementation paths without Markdown cards.
See the [card-template guide](PerfTrans/script_c2rust/perf_opt/agent_perf_opt/Optimization_Card/README.md) for details.

---

## forRQ4 — Optimization Effectiveness

> *How effectively does PerfTrans reduce the performance gap between translated
> Rust and C, and how does it compare with a general-purpose coding agent?*

`forRQ4/` is laid out as a self-contained workspace: all relative paths in the
`study.toml` files and Cargo path-dependencies resolve inside it.

```
forRQ4/
├── dataset_source/<project>/            # original C sources (C reference build)
├── dataset_trans/
│   ├── run_validation.py                # multi-variant measurement driver
│   ├── _op_groups.py                    # operation → table-row grouping; which batch file per project
│   ├── paper_eval_table.py              # LaTeX table generator (used for the paper)
│   └── <project>/
│       ├── 0_raw/                       # Raw:       c2rust 0.22.1 output
│       ├── 1_cleaned/                   #            definition consolidation (Translation Refinement, step 1)
│       ├── 2_stage_a/crate/             # Refined:   boundary refinement output (RQ5 baseline)
│       ├── 3_perf_opt/                  # PerfTrans: final crate + full run log (see forRQ5)
│       ├── 3_perf_opt_freeform/         # PerfTrans w/o rules (RQ5 ablation)
│       ├── codex_version/               # Codex CLI baseline: code/, prompt/, logs/, results/
│       ├── workloads/harness_gen/       # Workload Synthesis output: harness, seeds, corpus,
│       │                                #   golden.jsonl (functional), perf_inputs/ (performance)
│       └── validation_workload/
│           ├── driver/                  # C driver (+ its Rust twin in purebin/driver.rs)
│           ├── inputs/                  # measured inputs
│           └── purebin/
│               ├── study.toml           # 6 variants, operations, inputs, oracle digests
│               ├── c/build.sh           # C: clang-17 -O3 -flto, 64-byte function alignment
│               ├── {raw,codex,stage_a,freeform,full}/   # one bin crate per Rust variant
│               ├── bin/                 # prebuilt binaries of all 6 variants
│               └── results/
│                   ├── paper_arms_*.json   # RAW RESULT: per-pair medians of all variants
│                   └── op_coverage.json    # per-operation function coverage
├── perf_run.sh, dataset_trans/rdt_isolate.sh
├── scripts/summarize_rq4_rq5.py         # rebuilds the evaluation table
└── results/
    ├── eval_per_operation.csv           # = paper Table (evaluation), 41 rows × 5 Rust variants
    └── eval_per_input.csv               # every (operation, input) pair
```

**Variant names.** In the measurement files the variants are keyed
`c`, `raw`, `codex`, `stage_a` (= *Refined*), `freeform` (= *w/o rules*), `full`
(= *PerfTrans*). A variant whose output differs from C on a pair is not timed there
(`n/e` in the table). The batch used for each project is fixed in
`_op_groups.py` (`RESULT` / `RESULT_OVERRIDE`).
The summary scripts read these recorded batches without searching or selecting
among alternative runs.

**Re-measure one project** (run from `forRQ4/`):

```bash
P=lz4
bash dataset_trans/$P/validation_workload/purebin/c/build.sh
for v in raw codex stage_a freeform full; do
  (cd dataset_trans/$P/validation_workload/purebin/$v && cargo build --release)   # toolchain pinned by rust-toolchain.toml
done   # copy target/release/<bin> into purebin/bin/ (names in study.toml)
bash perf_run.sh python3 dataset_trans/run_validation.py \
     dataset_trans/$P/validation_workload/purebin/study.toml --wait-quiet
```

**Codex baseline.** `codex_version/prompt/` holds the exact instruction,
`codex_version/logs/` and `results/` the agent transcript and final message,
`codex_version/code/` the resulting crate (Codex CLI v0.153.4, GPT-5.4,
reasoning effort `high`, 30-minute budget).

---

## forRQ5 — Ablation Study

> *What contributes to the performance improvement of PerfTrans, and what is
> the contribution of rule guidance?*

The runtime columns *Refined* and *w/o rules* are in the same table as RQ4
(`forRQ4/results/eval_per_operation.csv`). This folder holds the optimization
runs themselves.

```
forRQ5/
├── commit_id_map.json        # original experiment IDs ↔ anonymized Git commit IDs
├── runs/<project>/
│   ├── PerfTrans/            # from 3_perf_opt/            (rule-guided)
│   └── PerfTrans_NoRule/     # from 3_perf_opt_freeform/   (rule layer removed)
│       ├── commit_history.txt       # accepted rewrites = git commits after "snapshot from 2_stage_a"
│       ├── rewrites.log             # every attempt: function, cards, gate verdicts, reason
│       ├── changesets.jsonl         # the structured edit of every attempt
│       ├── hotspots.json            # Hotspot Localization output (shared by both arms)
│       ├── class_{I,II,III}_hits.json, fn_hits.json   # Optimization Pattern Detection output
│       ├── baseline.json            # pre-optimization measurement
│       └── final_measurement.json   # post-optimization measurement
├── results/
│   ├── _rule_attribution_20260917.json   # per-commit rule attribution (62 PerfTrans commits)
│   └── _rule_triggering_20260917.json    # which rules fired / landed per project
└── scripts/
    ├── _rule_attribution.py, _rule_triggering.py, _commit_history.py, _op_groups.py
    └── ablation_freeform.py     # how the rule-free arm is built (same gates, no cards)
```

**Accepted rewrites.** Counting commits after the snapshot commit in
`commit_history.txt` gives **62** for PerfTrans and **28** for the rule-free
arm:

```bash
cd forRQ5/runs
for a in PerfTrans PerfTrans_NoRule; do
  t=0; for p in *; do t=$((t + $(wc -l < $p/$a/commit_history.txt) - 1)); done; echo "$a: $t"
done
```

The complete per-commit source diffs are in the crates' git histories:
`git -C forRQ4/dataset_trans/<project>/3_perf_opt/crate log -p`
(and `3_perf_opt_freeform/crate` for the ablation).

The shipped RQ5 result JSON files are copies of the original recorded results.
Anonymizing Git metadata changed commit hashes; `forRQ5/commit_id_map.json`
maps each recorded ID to its corresponding artifact commit, with the project,
arm, and commit subject. The scripts use this mapping to read the original
measurements. No optimization or performance measurement is rerun.

To inspect the recorded attribution, run from the artifact root:

```bash
python3.11 -B forRQ5/scripts/_rule_attribution.py
```

To check the recorded rule-triggering summary without overwriting it:

```bash
python3.11 -B forRQ5/scripts/_rule_triggering.py /tmp/rq5_rule_triggering_check.json
```

---

## PerfTrans — Implementation

```
PerfTrans/
├── script_c2rust/
│   ├── main.py                       # pipeline entry point
│   ├── Config/paths.conf             # paths + LLM endpoint (fill in your own API key)
│   ├── stages/                       # c2rust transpilation (stage0), definition consolidation (stage1)
│   ├── harness_gen/                  # §Workload Synthesis
│   │   └── build_workload.py         #   one entry: functional corpus → perf refinement → perf inputs
│   ├── perf_opt/
│   │   ├── stage_a/                  # §Translation Refinement: ABI recovery, pointer-view
│   │   │                             #   recovery (intra_ptr, buffer_lift), unsafe narrowing (deunsafe)
│   │   ├── hot_probe/                # §Hotspot Localization + §Optimization Pattern Detection
│   │   │                             #   (class_I = optimized IR, class_II = remarks, class_III = source)
│   │   └── agent_perf_opt/           # §LLM-Based Optimization + §Validation
│   │       ├── Optimization_Card/    #   optimization card templates and rewrite preconditions
│   │       ├── gates.py, measurement.py   # functional gate, incremental + cumulative performance gates
│   │       └── ablation_freeform.py  #   RQ5 rule-free arm
│   ├── profiling/                    # perf record / perf stat / TMA / optimization remarks
│   ├── sa_engine/                    # SVF-based pointer analysis used by Translation Refinement
│   └── utils/, tools/, proposer/
├── dependencyLib/                    # SVF 2.9 (source zip), tree-sitter C/Rust grammars
├── dataset_trans/rdt_isolate.sh
├── perf_run.sh
├── pyproject.toml, requirements.txt
```

**Running the pipeline on a project.** `paths.conf` expects C sources under
`$PROJECT_ROOT/dataset_source/<name>/` and writes to
`$PROJECT_ROOT/dataset_trans/<name>/` (`$PROJECT_ROOT` = `PerfTrans/`); copy a
project from `forRQ4/dataset_source/` there first.

```bash
cd PerfTrans/script_c2rust
python -m main --project <name>                          # c2rust → 0_raw → 1_cleaned → 2_stage_a (Refined)
python -m harness_gen.build_workload --harness <harness_dir>   # Workload Synthesis
python -m main --project <name> --from perf_opt --run-agent    # → 3_perf_opt   (PerfTrans)
python -m main --project <name> --from perf_opt --run-agent --ablation-freeform   # → 3_perf_opt_freeform
```

Main settings used in the paper: LLM = GPT-5.4 (`DEFAULT_LLM_MODEL` in
`paths.conf`); workload target 70% public-function / 60% line coverage, ≤ 3
refinement rounds, ≤ 5 harness repairs; hotspot threshold 3% self time;
performance gates with 95% CIs and adaptive 10/20/40 pairs; per-operation /
aggregate regression tolerance 1.0% / 0.3%; minimum net gain 0.1%.

---

## Environment

| Item | Version / setting |
|---|---|
| Translator | c2rust 0.22.1 |
| C compiler | clang-17 (17.0.6), `-O3 -flto -march=native -DNDEBUG`, no AVX-512 instructions emitted (checked with `objdump`) |
| Rust compiler | rustc 1.77.0-nightly (`nightly-2024-01-15`, LLVM 17.0.6), `opt-level=3`, fat LTO, `codegen-units=1`, `panic=abort`, `target-cpu=native`, AVX-512 off |
| RQ4/RQ5 builds | additionally 64-byte function alignment for all variants, including C |
| CPU | Intel Core i9-10980XE (Cascade Lake), 18 cores; isolated core, `performance` governor, turbo off, Intel RDT |
| Profiling | Linux `perf`, pmu-tools `toplev` (TMA) |
| Python | 3.11 (`requirements.txt`) |
| Pointer analysis | SVF 2.9 on LLVM 14 (Translation Refinement only) |

The build flags are pinned in each crate's `Cargo.toml` `[profile.release]` and
`.cargo/config.toml`, and in each `build.sh`, not in environment variables.
`perf_run.sh` assumes isolated cores 16/34; edit `CORES`/`PIN_CORE` for another
machine. Absolute runtimes depend on the machine; the Rust/C ratios are what
the paper reports.

## Not included

Build outputs (`target/`), coverage profiles, Python caches, the SVF build tree
(rebuild from `dependencyLib/SVF-SVF-2.9.zip`), and per-stage copies of the
workload harness (the canonical copy is `workloads/harness_gen/`).
