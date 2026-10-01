# bzip2 — Class I Differential evidence (collection stage; no classification decisions)

**Build**: C = `clang-17` (LLVM 17.0.6, `-O3 -flto -march=native -mno-avx512f`, post-LTO whole-program module; add `-g` for debug information) → `--plugin-opt=save-temps` produces `*.0.5.precodegen.bc` → `llvm-dis-17`; Rust = **the fat-LTO whole-program module of the measured binary itself** (`empirical_study/bzip2/rust_raw/target/release/deps/bzip2-*.ll`, reproducing the `study.toml` command `cargo +nightly-2024-01-15 build --release` with `-Cdebuginfo=1 --emit=llvm-ir` added, nightly-2024-01-15 = LLVM 17.0.6). Both sides use the same optimizer and post-LTO whole-program form ⇒ IR differences can only originate from c2rust code generation.

**Attribution**: Counts are collected per hot-function `define` block, i.e., its post-LTO code region including inlined callees. Obligation/control constructs are then attributed to **the source function that defines them** using the instruction's `DILocation` scope: **self** = defined in the hot function itself; **inherited from inlining** = from a named source function inlined into it; **nodbg** = no `!dbg` on the construct line (usually a `call` in a cold panic block; attribution is unknown, so it is counted in the current function region).

The **hotspot inventory** uses c2rust_raw self% because the target is Rust's additional execution time. Functions are accumulated in descending self-time order until coverage is ≥80% and no remaining function has ≥5%. An evidence site must be in a function with ≥5% self-time, and its workload must be classified as a regression in RQ1.

## Workload: decompress  (M1 (dynamic instruction ratio)=1.348; RQ1 wall-clock gap = +15.5% → **regression**)

**① Functions compared**: Descending self-time (perf record, pinned CPU in a clean environment, silesia-mozilla input); inventory coverage: **98.8%**.

| Function (perf symbol) | self% | ≥5% threshold |
|---|---:|:-:|
| `BZ2_decompress` | 61.21 | ✓ |
| `BZ2_bzDecompress` | 37.58 | ✓ |

**② Construct counts in optimized IR for each function**:

### `BZ2_decompress` (self 61.21%)

Rust IR: standalone definition, instruction region of 7746 lines; C IR: standalone definition, instruction region of 9642 lines.

| Construct | Rust self | Rust inherited / nodbg | C self | C inherited |
|---|---:|---|---:|---|
| Bounds checks (panic_bounds_check) | 36 | makeMaps_d 1;nodbg 37 | 0 | 0 |
| Option unwrapping (expect_failed) | 0 | malloc_fn 3;nodbg 3 | 0 | 0 |
| memset (control) | 4 | 0 | 3 | BZ2_hbCreateDecodeTables 3 |

**Memory-access / alias dimension** (counts within the code region): Rust `!tbaa` is consistently 0. rustc emits no type-based alias information, so the optimizer cannot prove non-aliasing ⇒ cross-iteration loads cannot be eliminated and loop-invariant GEPs cannot be hoisted; C memory accesses with `!tbaa` can be eliminated by GVN/LICM.

| Metric | Rust | C |
|---|---:|---:|
| load | 1010 | 755 |
| store | 620 | 636 |
| getelementptr | 906 | 926 |
| Memory accesses with `!tbaa` | 0 | 1394 |

### `BZ2_bzDecompress` (self 37.58%)

Rust IR: standalone definition, instruction region of 1620 lines; C IR: **no standalone definition** (inlined into `BZ2_bzRead` by LTO).

| Construct | Rust self | Rust inherited / nodbg | C |
|---|---:|---|---:|
| Bounds checks (panic_bounds_check) | 0 | unRLE_obuf_to_output_FAST 5, unRLE_obuf_to_output_SMALL 5;nodbg 10 | 0 |
| Option unwrapping (expect_failed) | 0 | malloc_fn 1 + 4;nodbg 5 | 0 |

**Memory-access / alias dimension**:

| Metric | Rust | C (inlined into bzRead) |
|---|---:|---:|
| load | 187 | — |
| store | 117 | — |
| getelementptr | 115 | — |
| Memory accesses with `!tbaa` | 0 | — |

**③ Open-ended scan (Rust-only symbols)**: None in `BZ2_decompress`; only `bzip2_1_0_8_raw::...` ×3 in `BZ2_bzDecompress` (internal library references). bzip2 is a self-contained binary (rust_raw directly produces it without a separate harness crate), so std::io/alloc infrastructure symbols do not contaminate the scan. Instruction regions: `BZ2_decompress` Rust 7746 < C 9642; `BZ2_bzDecompress` Rust 1620 (C inlined into bzRead). Static code size is Rust ≤ C; inflation occurs during dynamic execution.

**④ Inspection method 2: Evidence of successful optimization (present in C / less frequent in Rust)**:

| Function | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `BZ2_decompress` | 0 / 0 | 384 / 267 | 57 / 3 |
| `BZ2_bzDecompress` | 0 / inlined | 0 / inlined | 19 / inlined |

Note: bswap is 0 on both sides (byte-order operations are not folded; no difference). **Rust vectors 384 > C 267**: Rust has more vectorization, so this is not a gap where C has an optimization that Rust lacks. Calls: Rust 57 ≫ C 3 (more residual calls and less inlining in Rust → candidate for II②, to be assessed using Class II remarks).

---

**Collection-stage summary (differential observations only; no classification decisions)**:
- **C1 Redundant check branches**: `BZ2_decompress` bounds checks: Rust 74 (self 36 + inherited 1 + nodbg 37) vs C **0**; Option unwrapping (`malloc_fn`): Rust 6 vs C 0; `BZ2_bzDecompress` bounds checks: Rust 10 (inherited from unRLE_obuf_to_output_*) vs C 0 (C is inlined into bzRead and has no bounds checks).
- **C3 Redundant memory accesses / missing alias information**: `BZ2_decompress` `!tbaa`: Rust **0** vs C **1394**; loads: Rust 1010 > C 755.
- **C2 Saturating casts**: bzip2 has no floating-point hot path; `fptosi.sat` counts are 0 on both sides (not applicable).
