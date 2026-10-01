# xxHash — Class II differential evidence (missed optimizer transformations; collection stage, no classification decisions)

**Collection (corrected method; see `_method.md`)**: One combined regex; debug information on both sides; symmetric no-LTO builds. C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -DXXH_VECTOR=0 -DXXH_STATIC_LINKING_ONLY -gline-tables-only -c xxhash.c`. Rust = c2rust library crate `rust_raw`, **`cargo +nightly-2024-01-15`** (rustc 1.77 = LLVM 17.0.6; overrides rust_raw's nightly-2023-04-15/LLVM16 pin to match clang-17), `CARGO_PROFILE_RELEASE_LTO=off`. Since `-Cllvm-args=-pass-remarks` emits nothing, use native rustc **`-Cremark=all`** (`note:` lines; passed token `(success)`). Both use `XXH_VECTOR=0` (scalar only; XXH3 logic is header-only in `xxhash.h`).
> **Backend-symmetry confirmation**: Recollection from LLVM 16→17 yields **identical results item by item** (SLP C78/14, Rust10/8; inline cost 395>375 byte-for-byte). The inline cost model and SLP decisions are unchanged between 16/17, establishing that xxHash's loss is a **structural** consequence of c2rust discarding `always_inline`, rather than a toolchain artifact.

**Hot-function inventory (reused from Class I)**: XXH3 workload(M1=1.270,gap +33.1%) → `XXH3_hashLong_internal_loop` 95.9% (the main Class II evidence); XXH32 workload(M1=0.475,gap +62.2%) → main (XXH32 logic inlined into it; weak Class II signal, degradation mainly in CPI; see summary).

---

## II② Inlining loss — ● (root cause)

On the C side, `XXH_FORCE_INLINE = __attribute__((always_inline))` forces inlining of the entire hash chain: `XXH3_scalarRound → XXH3_accumulate_512_scalar → XXH3_accumulate_scalar → XXH3_hashLong_internal_loop` (+`consumeStripes`/`digest_long`). The hot kernel has **12 inline-passed / 0 not-inlined**.

Rust (c2rust) **discards the force-inline attribute**. `XXH3_accumulate_512_scalar` (**cost=395 > threshold=375**, only ~5% above) is **not inlined** into 4 hot callers:
- `XXH3_accumulate_scalar` (stripe loop, the innermost loop, `xxhash.rs:1641`)
- `XXH3_hashLong_internal_loop` ×2 (`xxhash.rs:1641`/`1754`)
- `XXH3_consumeStripes` / `XXH3_digest_long` (`xxhash.rs:2446`)

Rust full-lib inline: 312 passed / 50 not-inlined (vs C 257 / 33).

## II① Vectorization loss — ● (caused by inlining loss)

| SLP | C | Rust |
|---|---|---|
| full lib passed (raw / unique sites) | **78 / 14** | 10 / 8 |
| full lib missed | 1653 | 71 |
| **hot kernel passed** | **7** | **1** |

> Raw C counts are inflated because the header-only implementation is force-inlined into ~40 callers, producing one remark per copy. Unique sites (14 vs 8) and hot-kernel counts (7 vs 1) provide fair comparisons.

loop-vectorize is **not the mechanism** (0 passed in the hot kernel on both sides); all loss is in SLP. Key sites:
- `xxhash.h:6022` (scalarRound 8-lane paired stores) → C **"Stores SLP vectorized, cost -1, tree size 28"**; Rust `xxhash.rs:1613` has only **"cost -2, tree size 6"**. The large tree-28 SLP structure does not form.
- `xxhash.h:6072` (scalarScrambleRound) → C **"Stores SLP vectorized, cost -15, tree size 16"**; Rust `xxhash.rs:1660` → **"not beneficial cost 0>=0"**. The scramble step **does not vectorize at all**.
- Rust stripe loop `xxhash.rs:1639/1641` → **"loop not vectorized: call instruction cannot be vectorized"**. The uninlined `accumulate_512_scalar` call remains in the loop body.

## Causal chain (a clear Class II case)

**Inlining loss causes vectorization loss**: `accumulate_512_scalar` (cost 395>375) remains an out-of-line call in the per-stripe loop, preventing unrolling of the 8-lane body. A wide SLP tree cannot form (28→6), and scramble SLP is entirely lost. In addition, c2rust raw pointers lack `noalias`/`!tbaa` (Class I C3), further obstructing SLP's proof that loads/stores can be packed. **Inlining loss (II②) and missing alias information (C3) jointly defeat vectorization (II①).**

## Summary

xxHash is **the strongest Class II instance: inlining loss ● (root cause: c2rust discards force-inline) → vectorization loss ● (SLP tree degradation)**. The two are causally coupled and compounded by Class I C3. This explains XXH3 (M1>1 work inflation plus poor scalar-chain execution efficiency).

**Resolution of XXH32's 213-vs-20 discrepancy**: XXH32 (M1=0.475, fewer instructions yet +62.2%) is mainly a CPI regression (RQ2 M2=3.42), with a weak Class II signal. Class I inspection method 2 previously counted "C 213 vector operations vs Rust 20" using static objdump output. Remarks show that C's 213 mainly come from auto-vectorization of the **cov_workload setup loop** in `xxh_bench.c` (width 32, outside the timed XXH32 hash path). The XXH32 hash hot path itself has little vectorization on either side. Thus, 213-vs-20 is an **inflated static count**, the same pattern refuted by remarks in bzip2/heman. XXH32 degradation is attributed to CPI and addressed by the RQ2 decomposition.
