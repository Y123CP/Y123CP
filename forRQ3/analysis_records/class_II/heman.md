# heman — Class II differential evidence (missed optimizer transformations; collection stage, no classification decisions)

**Collection (corrected method; see `_method.md`)**: One combined regex; debug information on both sides; symmetric no-LTO builds. C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -gline-tables-only -c` (25 TUs: heman_pipeline_c.c/src/*.c/kazmath/*.c; libm math functions excluded). Rust = c2rust library crate `rust_raw` (`heman_raw`), **`cargo +nightly-2024-01-15`** (rustc 1.77 = LLVM 17; overrides rust_raw's LLVM16 pin to match clang-17), `CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all`. Counts are deduplicated by distinct `(file,line,col)`.

> **Two collection pitfalls that otherwise produce false conclusions**: (1) **fat-LTO misses remarks**: Running `-Cremark=all` directly on the fat-LTO harness yields loop-vec passed=0 and inline-missed=0 because vectorization/inlining occurs in the link-stage plugin, where `-Cremark` is not propagated. (2) **rust_raw pins LLVM16**: Force LLVM17 with `+nightly-2024-01-15`. An earlier version of this record reported "hot loop-vec C12/R0 / SLP C183/R28 / inline-missed 296/cost 6325" from fat-LTO. Those results combined both artifacts and **are invalidated**; the following are the corrected measurements from symmetric no-LTO + LLVM17 collection.

**Hot-function inventory (reused from Class I)**: pipeline(M1=1.078,gap +27.6%); generate(M1=1.226,gap +19.0%); lighting(M1=0.935,gap +29.5%).

---

## II① Vectorization loss — Largely absent (parity under symmetric LLVM17)

| pass | C | Rust (LLVM17) |
|---|---:|---:|
| loop-vectorize full-library passed / missed | 41 / 63 | **39** / 71 (actual `vectorized loop width 8` remarks with source lines) |
| slp-vectorizer full-library passed / missed | 140 / 430 | **136** / 686 |

Per-hot-function loop-vectorize passed/missed counts show **approximate parity**: `transform_to_distance` C2/R1; `create_df` C1/R1; `heightmap` C0/**R1** (Rust has more: generate.rs:271 vectorized w8, while both C loops miss); `occlusion` C2/R2; `edt` C0/R0; `island_noise` C0/R0; `apply_gradient` C0/R0; `main` C0/R0. Per-hot-function SLP counts are nearly all 0, except `edt` C2/R0 (below) and `occlusion` C3/R2.

**Only true residual loss** (C succeeds / Rust misses, unchanged from LLVM16, with no new loss in LLVM17): The strided-scatter writeback loop in `transform_to_distance`. C `distance.c:90` (`SDISTFIELD_TEXEL(x,y)=d[y]`) vectorizes w8; Rust `distance.rs:173` (`*(*sdf).data.offset(y*width).offset(x)=*d.offset(y)`, stride=image width) does not. The paired gather-load loop, Rust `distance.rs:167`, **vectorizes w8 on both sides**: Rust handles the reads but not the scattered writes. The root cause is that Rust cannot prove non-aliasing between `d` and `(*sdf).data`. **This is a true ●** (C `distance.c:90` w8 passed / Rust `distance.rs:173` missed due to aliasing), **but its scope is small: a single loop rather than a function-wide collapse. It does not change the overall judgment that heman has little Class II loss.** Separately, `edt`'s SLP 2-wide store (distance.c:19, cost −2) is rejected in Rust as cost-0 neutral, rather than illegal; the effect is negligible.

**Reconciliation**: LLVM16→17 changes loop-vec C41/R37→C41/**R39** (Rust improves, confirming LLVM17 Rust ≥ LLVM16) and SLP C140/R144→C140/R136 (cost-model variation within the parity band). **Parity is not an artifact of asymmetric backends; LLVM17 reproduces and slightly strengthens the conclusion.** This also overturns class_I inspection method 2's claim of "Rust vectors 581≥C, no loss": static objdump counts are misleading because they include scalar xmm, struct SLP, and std code. Remarks show approximate loop-vec parity (~40 each). The actual picture is comparable vectorization capability, rather than complete loss on either side.

## II② Inlining loss — None (same decisions)

The only same-TU internal callee relation, `edt → transform_to_distance`, is **rejected as too costly on both sides** (C cost=285>250, Rust cost=295>250; the same decision at nearly the same cost). All other Rust inline-missed entries resolve to libc/libm (calloc/free/atan, definition unavailable, as in C), cross-TU callees exposed by the single-crate layout (`open_simplex_noise`/`_2`, invisible to per-TU C and therefore not reported there; not a loss), or cold panic paths. **There is no true inlining loss.**

## Relationship to Class I

heman has no Class II gap apart from the small residual loss in 1 strided-scatter store loop. Degradation belongs to **Class I**: C2 (float-to-integer saturating casts throughout the hot paths) + C3 (`!tbaa`=0 in all hot functions). The lighting CPI disadvantage (slower despite M1<1, M2=1.39) is attributed to machine-level differences addressed by RQ2, **not** vectorization loss (occlusion loop-vec parity C2/R2).

## Summary

Class II **largely does not apply to heman** (vectorization C41/R39 ≈ parity, SLP parity, same inlining decisions). The only true small-scale effect is one `transform_to_distance` strided-scatter store loop that fails to vectorize because of raw-pointer aliasing. **Methodological lesson: missing fat-LTO remarks and rust_raw's LLVM16 pin can jointly create spurious vectorization loss (false R0) and inlining loss (false cost inflation); symmetric LLVM17 collection overturns them.** heman degradation is attributed to Class I (C2+C3).
