# binn — Class II differential evidence (missed optimizer transformations; collection stage, no classification decisions)

**Collection (corrected method; see `_method.md`)**: One combined regex `'loop-vectorize|slp-vectorizer|inline'`; debug information on both sides; symmetric no-LTO builds. C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -DBINN_NO_COMPRESS -gline-tables-only -c binn.c` (`main` from binn_roundtrip.c). Rust = c2rust library crate `rust_raw`, **`cargo +nightly-2024-01-15`** (rustc 1.77 = LLVM 17; overrides rust_raw's LLVM16 pin to match clang-17), `CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all` (passed inferred from absence of corresponding missed remarks). **Backend-symmetry correction**: Earlier collection used rust_raw's bundled LLVM16; this round forced LLVM17 and reproduced the no-degradation conclusion.

**Hot-function inventory (reused from Class I)**:
- build_serialize_100k(M1=1.530,gap +40.9%):`AddValue` 86.3% / `main` 12.9%
- iterate_decode_100k(M1=1.450,gap +27.9%):`main` 55.4% / `GetValue` 29.5% / `AdvanceDataPos` 14.4%

---

## II① Vectorization loss — None (symmetric)

| pass | Rust (hot/library) | C (hot/library) |
|---|---|---|
| loop-vectorize passed | 0 / 0 | 0 / 0 |
| slp-vectorizer passed | 0 / 1 (L342, non-hot) | 0 / 5 (binn.c:293, non-hot) |

Neither side vectorizes any hot-path loop. All 6 Rust SLP remarks in hot functions are `not beneficial cost 0>=0`: the vectorizer **deliberately rejects** cost-neutral adjacent-byte writes with 2 stores (`AdvanceDataPos` L618/L638, `GetValue` L1723). This matches the nature of C's decisions and is not a loss. binn performs typed serialization and scalar byte-swap copies, without data-parallel loops. **delta = 0.**

## II② Inlining loss — None (symmetric)

- **Symmetric hot-function inlining**: Both sides absorb the same small helpers (`copy_be16/32/64`, `compress_int`, `strlen2`, `CheckAllocation`, `binn_get_type_info`, `IsValidBinnHeader`). MIR/LLVM has already folded these into the Rust hot bodies (no not-inlined remarks).
- **The only caller-side asymmetry favors Rust**: `type_family → AddValue` (binn.c:959 / binn.rs:1079). Under LLVM17, **C still misses (cost=450>250), while Rust inlines successfully** (zero inline-missed lines). This is not Rust degradation.
- Not inlining the hot functions as callees into their wrappers is a **shared decision**, with nearly identical costs (`AddValue` 1040/1170, `GetValue` 525/690, `AdvanceDataPos` 270/270, all ≫ thr), rather than a Rust artifact.
- **Correction of an earlier fat-LTO artifact**: Manual collection on the fat-LTO harness previously labeled `copy_value` not being inlined (cost above threshold) as ●. Under fair no-LTO library-crate collection, `copy_value` has **no inlining relationship** with the 4 hot functions (no connecting caller/callee remarks). The earlier result was a fat-LTO harness collection artifact, not a hot-path inlining loss.

## Relationship to Class I

binn has no Class II gap. Its +40.9%/+27.9% gaps are **driven by Class I**: C1 (frequent Option unwrapping in serialization/decoding loops: `main` 22 + `AddValue` 2) + C3 (`!tbaa`=0 in all Rust hot functions, versus 230 in C `main`). Class II **does not explain the binn gap**.

## Summary

binn has **no Class II degradation** (vectorization delta=0; symmetric inlining). The corrected method (fixed flags + no-LTO + symmetric debug information) shows that vectorization and inlining in c2rust's binn hot paths **match the C reference**; degradation is entirely Class I (C1+C3). The methodological lesson is that fat-LTO harness collection can produce spurious inlining loss (`copy_value`), requiring symmetric no-LTO library-crate verification.
