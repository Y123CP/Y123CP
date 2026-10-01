# libzahl — Class I Differential evidence (collection stage; no classification decisions)

**Build**: C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -g -Wl,--plugin-opt=save-temps` (post-LTO whole program, `libzahl_opsuite.c src/*.c`) → `*.precodegen.bc` → `llvm-dis-17`; Rust = the fat-LTO whole-program module of the measured binary (`rust_harness/target/release/deps/libzahl_opsuite-*.ll`, reproducing `cargo build --release` + `-Cdebuginfo=1 --emit=llvm-ir`, rust-toolchain = nightly-2024-01-15 = LLVM 17.0.6). Both sides use the same optimizer and whole-program form ⇒ IR differences can only originate from c2rust code generation.

**Attribution**: As in bzip2, counts are collected per hot-function `define` region, with obligation/control constructs attributed by `DILocation` scope to self / inherited from inlined functions / nodbg. libzahl hotspots are distributed across big-integer operations and memory management, so the inventory includes several functions with ≥5%.

The **hotspot inventory** uses c2rust_raw self%, accumulated in descending order until coverage is ≥80% and no remaining function has ≥5%. Evidence sites must have ≥5% self-time and belong to a workload classified as a regression in RQ1.

## Workload: mul_4096bit  (M1=1.231;RQ1 gap = +37.3% → **regression**)

**① Hotspot inventory** (perf record, pinned CPU in a clean environment, pairs_4096_seedB.bin), cumulative coverage ≈80%:

| Function | self% | ≥5% |
|---|---:|:-:|
| `zmul_ll` | 16.9 | ✓ |
| `libzahl_realloc` | 16.9 | ✓ |
| `zadd_unsigned_assign` | 16.6 | ✓ |
| `zfree` | 11.4 | ✓ |
| `zrsh` | 10.2 | ✓ |
| `zlsh` | 7.9 | ✓ |

## Workload: modpow_4096bit  (M1=1.123;RQ1 gap = +10.2% → **regression**)

**① Hotspot inventory**, cumulative coverage ≈81%:

| Function | self% | ≥5% |
|---|---:|:-:|
| `libzahl_zsub_unsigned` | 38.3 | ✓ |
| `libzahl_realloc` | 9.8 | ✓ |
| `zrsh` | 9.1 | ✓ |
| `zadd_unsigned_assign` | 8.6 | ✓ |
| `zfree` | 8.0 | ✓ |
| `zmul_ll` | 6.8 | ✓ |

**② Construct counts in optimized IR for each hot function** (shared by both workloads; data from the whole-program module):

| Function | Obligation/control constructs (Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `zmul_ll` | memset nodbg 4 | 23 / 13 / 16 / **0** | 64 / 62 / 56 / **126** |
| `libzahl_realloc` | bounds_check self 1 + inherited zfree 1 + nodbg 2 | 43 / 35 / 53 / **0** | 34 / 31 / 50 / **65** |
| `zadd_unsigned_assign` | overflow_check inherited overflowing_add 1 | 15 / 8 / 16 / **0** | 13 / 4 / 12 / **17** |
| `zfree` | bounds_check self 1 + nodbg 1 | 9 / 7 / 7 / **0** | (inlined; no standalone definition) |
| `zrsh` | Zero in all eight categories | **73** / 56 / 103 / **0** | **24** / 16 / 50 / **40** |
| `zlsh` | memset inherited libzahl_memset_precise 2 | 64 / 58 / 105 / **0** | 34 / 31 / 82 / **67** |
| `libzahl_zsub_unsigned` | Zero in all eight categories | 51 / 16 / 38 / **0** | 32 / 10 / 26 / **42** |

**③ Instruction-region sizes (Rust/C lines) and open-ended scan**:

| Function | Rust lines | C lines | Open-ended scan (Rust-only symbols) |
|---|---:|---:|---|
| `zmul_ll` | 218 | 658 | Internal library references |
| `libzahl_realloc` | 327 | 294 | — |
| `zadd_unsigned_assign` | 156 | 189 | — |
| `zfree` | 70 | Inlined | — |
| `zrsh` | 547 | 396 | — |
| `zlsh` | 537 | 555 | — |
| `libzahl_zsub_unsigned` | 411 | 372 | `libzahl_raw::...` ×8(internal to the library) |

Note: `zmul_ll` static code size is Rust 218 << C 658 (Rust is smaller by 3×); most functions have Rust ≈ C. **Inflation occurs during dynamic execution, not in static code size.** No Rust-only harness infrastructure symbols occur (libzahl has no std::io/alloc-intensive hotspots).

**④ Inspection method 2: Evidence of successful optimization (present in C / less frequent in Rust)**:

| Function | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `zmul_ll` | 0 / 0 | 0 / 0 | 28 / 40 |
| `zrsh` | 0 / 0 | 6 / 4 | 2 / 3 |
| `libzahl_zsub_unsigned` | 0 / 0 | 0 / 0 | 8 / 11 |

Note: bswap counts are all 0; vectors are rare on both sides (scalar big-integer operations per limb, with no vectorization opportunity); C has slightly more calls. Inspection method 2 shows no optimization present in C but less frequent in Rust. The libzahl gap falls entirely under C3 (redundant memory accesses) in inspection method 1.

---

**Collection-stage summary (differential observations only; no classification decisions)**:
- **C3 Redundant memory accesses / missing alias information (main finding)**: **Rust `!tbaa` is consistently 0 in all 7 hot functions, versus tens to hundreds in C** (zmul_ll 0 vs 126, zlsh 0 vs 67, libzahl_realloc 0 vs 65). Redundant loads due to raw-pointer cursors are most apparent in **`zrsh`: Rust loads 73 vs C 24 (≈3×)**, and `libzahl_zsub_unsigned`: 51 vs 32. This is consistent with per-limb raw-pointer access in libzahl big-integer operations.
- **C1 Redundant check branches (weak evidence)**: Only sporadic occurrences in memory-management functions: `libzahl_realloc` has 4 bounds checks and `zfree` has 2. The overflow_check in `zadd_unsigned_assign` occurs on both sides (Rust inherited from overflowing_add, C from zadd_impl), so it is not a difference.
- **C2 Saturating casts**: libzahl has no floating-point operations; `fptosi.sat` is 0 on both sides (not applicable).
