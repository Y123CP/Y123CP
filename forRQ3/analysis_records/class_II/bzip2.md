# bzip2 — Class II differential evidence (missed optimizer transformations; collection stage, no classification decisions)

**Collection (corrected method; see `_method.md`)**: One combined regex; debug information on both sides; symmetric no-LTO builds. C = per-TU `clang-17 -O3 -march=native -mno-avx512f -gline-tables-only -DNDEBUG -D_FILE_OFFSET_BITS=64 -c` (7 library TUs). Rust = c2rust crate `rust_raw`, forced no-LTO (`CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1` + `-Cremark=all`; this toolchain emits nothing for `-pass-remarks`). Both use LLVM 17.0.6 (nightly-2024-01-15). The inlining channel excludes cross-crate (LTO-resolvable) and framework-noinline entries.

> **Rust passed counts were cross-checked between two builds and are real, rather than missed collection**: The new no-LTO build and existing fat-LTO `lto_remarks.log` report **the same** successful Rust vectorizations (decompress.rs:2504/2553/2959 loop-vec + 3008 SLP). Thus, the bzip2 Rust vectorization passed counts are valid; the earlier "R0" was incorrect. Unlike heman's missing fat-LTO remarks, bzip2 is consistent across both build forms.

**Hot-function inventory (reused from Class I; decompress workload: M1=1.348, gap +15.5%)**: `BZ2_decompress` 61.21% (c2rust output is a monolithic ~2880-line state machine, decompress.rs:191–3071) + `BZ2_bzDecompress` 37.58% (control wrapper; neither side has vectorizable loops, so this is not a loss source).

> **Counting convention**: The table below contains raw remark counts. Full-library C totals include multiple load sites from `GET_BITS` macro expansion, inflating aggregates; **do not use them for cross-project comparisons**. ● judgments and the ≥3 threshold always use **unique sites within hot functions**, specifically the 5 hot decoding loops below, unaffected by aggregate deduplication conventions.

---

## II① Vectorization loss — ● (real; 5 remaining hot loops)

| pass | Side | Hot-function passed / missed | Full-library passed / missed |
|---|---|---:|---:|
| **loop-vectorize** | C | **8** / 54 | 23 / 227 |
| | Rust | **3** / 58 | 18 / 135 |
| **slp-vectorizer** | C | **5** / 382 | 14 / 1295 |
| | Rust | **2** / 16 | 2 / 193 |

- Successful Rust hot-function sites: `decompress.rs:2504` (= C cftab accumulation loop), `:2553` (= C cftabCopy), `:2959` (= C maxLen scan); SLP `mtfa[kk]=…` store (= the corresponding C site). **Rust does not lose every optimization**; some successes overlap with C.
- **Remaining true losses (C vectorized / Rust missed)**:

| C site (decompress.c) | Width | Source semantics | Rust |
|---|---|---|---|
| `:311` | 32 | `for(v;v<nGroups;v++) pos[v]=v` (selector initialization) | missed |
| `:316` | 16 | `while(v>0){pos[v]=pos[v-1];v--;}` (MTF selector left shift) | missed |
| `:409/416` | 4/4 | `while(es>0){…}` (RLE run-expand) | missed |
| `:447` | 32 | `while(nn>0){…}` (MTF value copy) | missed |

**Reasons for Rust rejection** (histogram within `BZ2_decompress`): **53× `could not determine number of loop iterations`** + **45× `value that could not be identified as reduction is used outside the loop`** (+1 switch, +1 unsafe-dep).

**Root causes (central and confirmed)**: (i) c2rust's `panic_bounds_check` branch before each array access adds a **second loop exit**, preventing LLVM from determining a countable trip count; (ii) signed `c_int` induction variables and writes of loop-carried variables back to `(*s).save_*` state fields make values "used outside the loop," defeating reduction/IV recognition. These mechanisms occur precisely in the RLE `while(es>0)/(nn>0)` and MTF-selector `while(v>0)` loops vectorized in C.

## II② Inlining loss — None

After excluding cross-crate (LTO-resolvable) and framework-noinline entries, both sides have 0 in-crate cost-based inlining losses. Rust **successfully inlines** all actual hot callees (`makeMaps_d`, `unRLE_obuf_to_output_FAST/SMALL`, `BZ2_indexIntoF`, `BZ2_bz__AssertH__fail`), matching C. Inlining does not account for the bzip2 gap.

## Relationship to Class I (important)

The root cause of bzip2 vectorization loss is a **second-order consequence of Class I C1 (redundant bounds checks)**. The `panic_bounds_check` branches recorded under C1 both execute additional work themselves and add a second exit to each hot loop, breaking countability and causing the vectorizer to reject the loop. The same c2rust defect manifests as additional checks in Class I and missed vectorization in Class II. This reinforces C1 without changing the C1 rule. Correction to class_I: its earlier inspection method 2 used static objdump counts to infer "Rust vectors ≥ C"; the remark channel refutes that interpretation.

## Summary

bzip2 has **Class II vectorization loss ● (real; 5 remaining hot decoding loops, C w4–32 / Rust missed), with no inlining loss**. c2rust's bounds-check panic branches (second exits), signed-int loops, and state-field writebacks break trip-count analyzability. Alongside xxHash (inlining→unrolling→SLP) and libzahl (raw-pointer aliasing), this belongs to the gap dimension where c2rust code structure breaks vectorization prerequisites, with different triggering mechanisms (countability vs aliasing vs unrolling).
