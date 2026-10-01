# heman — Class I Differential evidence (collection stage; no classification decisions)

**Build**:C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -Wno-unknown-pragmas -g -Wl,--plugin-opt=save-temps`(`heman_pipeline_c.c src/*.c kazmath/*.c -lm`)→ precodegen → `llvm-dis-17`;Rust = the fat-LTO whole-program module of the measured binary(`rust_harness/target/release/deps/heman_pipeline-*.ll`,`cargo build --release` + `-Cdebuginfo=1 --emit=llvm-ir`,nightly-2024-01-15 = LLVM 17.0.6).

**Attribution**: Counts are collected per hot-function `define` region. Obligation and control constructs are attributed by `DILocation` scope to self / inherited from inlined functions / nodbg. **libm math functions (`__atan_fma`/`__ieee754_pow_fma`/`__memmove_*`) are shared by both sides and excluded because they are not translation differences**. On the C side, `compute_occlusion`/`generate_island_noise` are inlined by LTO and have no standalone definitions.

## Workload: pipeline(distance field)  (M1=1.078;RQ1 gap = +27.6% → **regression**)

**① Hotspot inventory**(seed_1024²):

| Function | self% | ≥5% |
|---|---:|:-:|
| `heman_pipeline::main` | 32.3 | ✓ |
| `distance::edt` | 20.0 | ✓ |
| `heman_color_apply_gradient` | 16.3 | ✓ |
| `distance::transform_to_distance` | 12.2 | ✓ |
| `heman_distance_create_df` | 10.9 | ✓ |

## Workload: lighting  (M1=0.935;RQ1 gap = +29.5% → **regression**)

**① Hotspot inventory** (seed_256² + lighting): `heman_lighting_compute_occlusion` 46.1%; the remaining `__atan_fma` 31.7% / `__ieee754_pow_fma` 7.8% belong to **libm** (shared by both sides; excluded). M1<1 (fewer instructions); degradation is mainly due to CPI (RQ2 M2=1.39), with weak Class I evidence.

## Workload: generate(terrain)  (M1=1.226;RQ1 gap = +19.0% → **regression**)

**① Hotspot inventory** (512² island):

| Function | self% | ≥5% |
|---|---:|:-:|
| `heman_internal_generate_island_noise` | 67.6 | ✓ |
| `heman_generate_island_heightmap` | 8.7 | ✓ |
| `distance::edt` | 7.3 | ✓ |

**② Construct counts in optimized IR for each hot function**:

| Function | Obligation/control constructs (Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `heman_color_apply_gradient` | **Saturating cast inherited heman_image_sample 2** | 10 / 7 / 19 / **0** | 20 / 17 / 39 / **37** |
| `heman_lighting_compute_occlusion` | **Saturating cast self 1** | 34 / 41 / 60 / **0** | Inlined into main |
| `heman_internal_generate_island_noise` | **Saturating cast inherited fastFloor 7** | 115 / 9 / 92 / **0** | Inlined into main |
| `heman_generate_island_heightmap` | **Saturating cast inherited heman_image_sample 7** + nodbg 1;memcpy/memset(control) | 18 / 13 / 34 / **0** | 172 / 58 / 194 / **230** |
| `distance::transform_to_distance` | memcpy self 1 | 27 / 15 / 59 / **0** | 7 / 11 / 39 / **21** |
| `heman_distance_create_df` | Zero in all eight categories | 17 / 16 / 23 / **0** | 22 / 22 / 38 / **44** |
| `distance::edt` | Zero in all eight categories | (130 lines) / **0** | 14 / 8 / 20 / **22** |
| `heman_pipeline::main` | unwind nodbg 21;**Saturating cast inherited heman_export_u8 8**;Option unwrapping nodbg 6;overflow 1;memcpy/memset(control) | 139 / 122 / 193 / **0** | 49 / 0 / 37 / **47** |

**③ Instruction-region sizes (Rust/C lines) and open-ended scan**:

| Function | Rust lines | C lines | Open-ended scan (Rust-only symbols) |
|---|---:|---:|---|
| `heman_color_apply_gradient` | 147 | 285 | — |
| `heman_distance_create_df` | 216 | 310 | — |
| `heman_lighting_compute_occlusion` | 658 | Inlined into main | — |
| `heman_internal_generate_island_noise` | 1983 | Inlined into main | `heman_raw::src...` ×28(internal to the library) |
| `heman_generate_island_heightmap` | 406 | 3461 | — |
| `distance::transform_to_distance` | 326 | 273 | — |
| `distance::edt` | 130 | 237 | — |
| `heman_pipeline::main` | 2236 | 504 | `llvm.assume` ×15, `llvm.experimental.noalias.scope.decl` ×13, `alloc::*`(harness infrastructure) |

Note: Most computation kernels have Rust < C static code size (apply_gradient 147/285, edt 130/237, heightmap 406/3461); C2 saturating-cast inflation occurs during dynamic execution. Library hot functions have no harness infrastructure symbols (those in noise are internal to the library); infrastructure symbols are concentrated in main.

**④ Inspection method 2: Evidence of successful optimization (present in C / less frequent in Rust)**:

| Function | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `heman_color_apply_gradient` | 0 / 0 | 18 / 22 | 2 / 2 |
| `heman_internal_generate_island_noise` | 0 / inlined | 581 / inlined | 6 / inlined |
| `distance::edt` | 0 / 0 | 0 / 1 | 0 / 0 |

Note: bswap counts are all 0; **Rust has abundant vectors (noise 581, apply_gradient 18) ≥ C**. Rust also auto-vectorizes the floating-point hot paths, so this is not a case where C has vectorization that Rust lacks. The heman gap falls under C2 saturating casts (scalar conversion) in inspection method 1 rather than vectorization.

---

**Collection-stage summary (differential observations only; no classification decisions)**:
- **C2 Scalar conversion inflation (main finding)**: Float-to-integer `as` casts have saturating semantics in Rust (`llvm.fptosi.sat`) and are **widespread in heman hot paths**: `fastFloor` inherited 7, `heman_export_u8` inherited 8, `heman_image_sample` inherited 2/7, `compute_occlusion` self 1; **all C counts are 0** (C `(int)x` is a single `cvttsd2si`). This is the characteristic additional frontend work in heman, the only floating-point-intensive project in the corpus.
- **C3 Redundant memory accesses / missing alias information**: Rust `!tbaa` is consistently 0 in all hot functions, versus 22–230 in C.
- **C1 Redundant check branches**: Only sporadic Option unwrapping/overflow in harness main; none in the computation kernels (edt/create_df).
- **Special case: lighting**: M1<1 (fewer Rust instructions), outside the explanatory scope of Class I. Degradation is in CPI (RQ2 M2=1.39, all five dimensions positive), rather than additional frontend work.
