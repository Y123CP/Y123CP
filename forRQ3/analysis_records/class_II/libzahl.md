# libzahl — Class II differential evidence (missed optimizer transformations; collection stage, no classification decisions)

**Collection (corrected method; see `_method.md`)**: One combined regex; debug information on both sides; symmetric no-LTO builds. C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -gline-tables-only -c` (`libzahl_opsuite.c src/*.c`). Rust = c2rust library crate `rust_raw`, **`cargo +nightly-2024-01-15`** (rustc 1.77 = LLVM 17; overrides rust_raw's nightly-2023-04-15/LLVM16 pin to match clang-17), `CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all`.
> **Backend-symmetry correction (important)**: Earlier collection mistakenly used rust_raw's bundled LLVM 16 and concluded that there was a small vectorization loss. Symmetric LLVM 17 recollection **overturns** that conclusion (below).

**Hot-function inventory (reused from Class I)**: mul_4096bit(M1=1.231,gap +37.3%):`zmul_ll`/`libzahl_realloc`/`zadd_unsigned_assign`/`zfree`/`zrsh`/`zlsh`; modpow_4096bit(M1=1.123,gap +10.2%):`libzahl_zsub_unsigned`/`realloc`/`zrsh`/`zadd`/`zfree`/`zmul_ll`.

---

## II① Vectorization loss — None (LLVM16 artifact, absent under symmetric LLVM17)

| pass | Rust (LLVM17) | C (LLVM17) |
|---|---:|---:|
| SLP passed / missed | 10 / 165 | 14 / 856 |
| loop-vectorize passed / missed (hot files) | 1 / 17 | 0 / 21 |

- **SLP: no gap**. R10/C14 is balanced. Neither side applies SLP to hot bignum limb arithmetic because of loop-carried carry-chain dependencies; the dominant misses are "impossible / not beneficial" on both sides.
- **loop-vectorize: symmetric**. Hot files have R1/C0 (approximately 0 on both sides). The dominant failure reasons match: "could not determine number of loop iterations" (R16/C17), switch, and non-reduction. Carry-chain limb loops are non-vectorizable on both sides.
- **The 7 `cannot identify array bounds` sites were LLVM16 artifacts and have disappeared**: Under LLVM16, zsub.rs had 7 raw-pointer bounds failures (Rust only). Under LLVM17, **all disappear**; only 1 remains in stdlib `core/ptr/mut_ptr.rs`, outside the hot functions. zsub.rs loops now fail for the **same reasons** as zsub.c (switch/trip count). LLVM17 improves raw-pointer bounds analysis; the earlier small loss was a false alarm from a 16-vs-17 mismatch.

## II② Inlining loss — ● (the only true, version-independent gap)

Hot caller `zmul_ll` is recursive on both sides and is not inlined into its callers on either side. Its **callees**, however, differ within the same scope (C `zmul.c` statics / Rust `zmul` module):

| callee → zmul_ll | C (clang-17) | Rust (c2rust, LLVM17) |
|---|---|---|
| `zinit_temp` | **inlined** cost=115(thr 325) | **MISSED** cost=**685**(thr 325) |
| `zfree`/`zfree_temp` | **inlined** cost=10(thr 487) | **MISSED** cost=**280**(thr 250) |

`zsqr_ll` has the same pattern (Rust misses `zfree` 280, `zadd_unsigned_assign` 440, `zlsh` 780), but **is not in the Class I ≥5% hot-function inventory**; only `zmul_ll` is a hot caller. It provides supplementary non-hot evidence and **receives lower weight during synthesis**. The hot inlining-loss evidence for libzahl consists of 2 sites in `zmul_ll` (`zinit_temp`/`zfree`).

- **Version-independent**: The asymmetry **persists or becomes stronger** from LLVM 16→17 (`zinit_temp` cost 385→685, while C remains 115 → 5.9×; `zfree` remains 280, C 10). This is a true code-generation gap: verbose c2rust bodies (raw-pointer bookkeeping + bounds checks) increase LLVM's inline-cost estimate, leaving out-of-line calls in hot callers where clang folds them.
- **Cross-scope caveat (avoid overstatement)**: `libzahl_realloc` is not inlined into 63 Rust callers (cost 1135/1180 ≫ 250), but realloc is cross-TU in per-TU C and never evaluated by the inliner (0 remarks). Both final states are out-of-line. Moreover, realloc is sufficiently large that the same cost model would reject it under C-LTO. **This is a build-model visibility difference, treated as symmetric rather than Rust degradation.**

## Relationship to Class I

libzahl has no vectorization-related Class II gap; the earlier misclassification has been corrected. Inlining loss (thin temporary wrappers not inlined into `zmul_ll`/`zsqr_ll`) is an **independent Class II signal**, distinct from Class I's main C3 redundant-memory-access finding (`zrsh` loads 73/24).

## Summary

libzahl has **no vectorization loss (LLVM16 artifact, absent under symmetric LLVM17) + inlining loss ● (the only true, version-independent gap)**. c2rust body inflation raises inline cost (`zinit_temp` 685 vs 115, `zfree` 280 vs 10), leaving out-of-line calls in `zmul_ll`/`zsqr_ll`. Feasible rewrites are to shrink the c2rust body or add `#[inline]` to thin temporary wrappers. **Methodological lesson: LLVM 16→17 changes raw-pointer vectorization decisions through bounds analysis but does not change the inline cost model; vectorization losses require symmetric LLVM17 verification, while the inlining loss is version-robust.**
