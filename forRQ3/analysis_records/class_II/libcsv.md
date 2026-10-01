# libcsv — Class II differential evidence (missed optimizer transformations; collection stage, no classification decisions)

**Collection (corrected method; see `_method.md`)**: One combined regex `'loop-vectorize|slp-vectorizer|inline'`; debug information on both sides; symmetric no-LTO builds. C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -gline-tables-only -c libcsv.c` (main.c harness excluded; LLVM 17.0.6). Rust = c2rust library crate `rust_raw`, **`cargo +nightly-2024-01-15`** (rustc 1.77 = LLVM 17, matching clang-17), `CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all` (rust_raw/Cargo.toml has no op_write feature, so omit --features). **Backend-symmetry correction**: Earlier verification mistakenly used rust_raw's bundled `nightly-2023-04-15` (LLVM 16). This round forces +nightly-2024-01-15 so that C and Rust both use LLVM 17. rustc 1.77 emits inlining/vectorization remarks normally; no objdump fallback is needed.

**Hot-function inventory (reused from Class I)**:
- parse(M1=1.267,gap +35.1%):`csv_parse` 83.9% / `cb_field` 5.2% (mostly inlined into csv_parse)
- write(M1=0.994,gap +8.3%):`csv_count::main` (including inlined `csv_write`) 97.1%

---

## II① Vectorization loss — None (symmetric)

| pass | C passed / missed | Rust passed / missed |
|---|---|---|
| slp-vectorizer | 0 / 31 | 0 / 46 |
| loop-vectorize | 0 / 7 | 0 / 17 |

Neither side vectorizes any hot loop. The key point is that the main byte loop in `csv_parse` is **not a vectorization candidate on either side** (no `loop not vectorized` inside it; only a no-op SLP remark at the header): it is a byte-at-a-time `switch` state machine. The `loop not vectorized` remarks refer to the realloc-retry loop in `csv_increase_buffer` and character-by-character escaping loops in `csv_write2`/`csv_fwrite2`. These are **inherently non-vectorizable on both sides** because of data-dependent control flow and unknown trip counts. All SLP misses are `not beneficial cost 0>=0` / `Cannot SLP (impossible)`, symmetrically.

## II② Inlining loss — None (symmetric LLVM 17, confirmed by remarks)

All 4 C inline-passed sites **also succeed in Rust (LLVM 17)**, with comparable costs/thresholds:

| callee → caller | C | Rust |
|---|---|---|
| `csv_increase_buffer → csv_parse` (site 1) | success cost=110/250 | success cost=165/250 |
| `csv_increase_buffer → csv_parse` (site 2) | success cost=−14890 | success cost=−14835 |
| `csv_write2 → csv_write` | success 115/250 | success 115/250 |
| `csv_fwrite2 → csv_fwrite` | success 220/250 | success 220/250 |

The only Rust inline-missed entries into hot functions are `__assert_fail`/`expect_failed` (cold panic paths, external symbols). C's `__assert_fail`/`fputc` are likewise external and cannot be inlined; this benign symmetry is excluded by the filtering rules. Every internal library callee inlined into a C hot function is also inlined in Rust. Under LLVM 17, the LLVM inliner explicitly emits success remarks; this is not early MIR folding as previously assumed. **There is no gap in whether these callees are inlined.**

## Relationship to Class I

libcsv has no Class II gap. Its +35.1%/+8.3% gaps are **driven by Class I**: C1 (`csv_parse` Option unwrapping 4) + C3 (`csv_parse` `!tbaa`=0 vs C 103; write main 0 vs 85). Class II **does not explain the libcsv gap**.

## Summary

libcsv has **no Class II degradation** (symmetric vectorization 0/0 and inlining remarks). The byte-at-a-time state-machine parser has no data-parallel hot loops; c2rust's vectorization/inlining behavior matches the C reference. Degradation belongs entirely to Class I (C1+C3). The conclusion is reproduced with **symmetric backends (LLVM17=LLVM17)** and agrees with the earlier LLVM16 conclusion.
