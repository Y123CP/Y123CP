# xxHash — Class I Differential evidence (collection stage; no classification decisions)

**Build**: C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -DXXH_VECTOR=0 -DXXH_STATIC_LINKING_ONLY -g -Wl,--plugin-opt=save-temps` (`xxh_bench.c xxhash.c`) → precodegen → `llvm-dis-17`; Rust = the fat-LTO whole-program module of the measured binary (`rust_harness/target/release/deps/xxh_bench-*.ll`, `cargo build --release` + `-Cdebuginfo=1 --emit=llvm-ir`, nightly-2024-01-15 = LLVM 17.0.6). Both sides use `XXH_VECTOR=0` (scalar only).

**Attribution**: Counts are collected per hot-function `define` region. Obligation and control constructs are attributed by `DILocation` scope to self / inherited from inlined functions / nodbg. **On the C side, `XXH3_hashLong_internal_loop` is inlined into `main` by LTO and has no standalone definition**.

## Workload: XXH3  (M1=1.270;RQ1 gap = +33.1% → **regression**)

**① Hotspot inventory**(64 KiB × 50000):

| Function | self% | ≥5% |
|---|---:|:-:|
| `xxhash::XXH3_hashLong_internal_loop` | 95.9 | ✓ |

## Workload: XXH32  (M1=0.475;RQ1 gap = +62.2% → **regression**)

**① Hotspot inventory**: `xxh_bench::main` (with XXH32 logic inlined into it) 99.4%.

**② Construct counts in optimized IR for each hot function**:

| Function | Obligation/control constructs (Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `XXH3_hashLong_internal_loop` | panic nodbg 1 | 32 / 24 / 40 / **0** | Inlined into main |
| `xxh_bench::main`(XXH32) | unwind nodbg 18;Option unwrapping nodbg 5;overflow 1;memcpy(control) | 57 / 69 / 78 / **0** | 37 / 101 / 133 / **113** |

**③ Instruction-region sizes (Rust/C lines) and open-ended scan**:

| Function | Rust lines | C lines | Open-ended scan (Rust-only symbols) |
|---|---:|---:|---|
| `XXH3_hashLong_internal_loop` | 262 | Inlined into main | `xxhash_raw::...` ×2(internal to the library) |
| `xxh_bench::main`(XXH32) | 933 | 2108 | `llvm.assume` ×10, `std::io::stdio` ×6, `alloc::*`, `llvm.experimental.noalias.scope.decl` ×4(harness infrastructure) |

Note: main has Rust 933 < C 2108 (substantially smaller static code size). Rust-only symbols all belong to std/alloc infrastructure in harness main, rather than the translated library. This is consistent with XXH32 M1=0.475 (fewer Rust instructions): its degradation is in CPI (see the summary), not static code size.

**④ Inspection method 2: Evidence of successful optimization (present in C / less frequent in Rust)**:

| Function | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `XXH3_hashLong_internal_loop` | 0 / inlined | 0 / inlined | 3 / inlined |
| `xxh_bench::main`(XXH32/XXH3) | 0 / **5** | **20 / 213** | 49 / 63 |

**Key difference**: Although both sources use `XXH_VECTOR=0` (forcing scalar code), **the C compiler still auto-vectorizes the hash loops into 213 vector operations + 5 bswap sites, versus only 20 vectors and 0 bswap in Rust**. Auto-vectorization succeeds in C but not Rust. This is an important mechanism for xxHash degradation (C SIMD throughput versus scalar Rust), classified as **vectorization loss (II①)**, supplementing the C3/CPI perspective below.

---

**Collection-stage summary (differential observations only; no classification decisions)**:
- **C3 Redundant memory accesses / missing alias information**: Rust `!tbaa` is consistently 0 in both hot functions, whereas C main reaches 113.
- **C1 Redundant check branches**: Very few obligation symbols on the hot path (only 1 panic in XXH3_hashLong; XXH32 unwind/expect all occur at the harness main level). The hash kernels contain almost no additional frontend work.
- **C2 Saturating casts**: No floating-point operations; `fptosi.sat` is 0 (not applicable).
- **Key special case: XXH32 (explicit limitation)**: **M1=0.475: Rust has only half as many instructions as C**. This does not satisfy the additional-frontend-work premise of Class I and is structurally outside its scope. Its +62.2% degradation comes entirely from CPI (RQ2 M2=3.42, with ΔCPI dominated by the two bottlenecks Mem+0.50 / Core+0.48): an execution-efficiency loss characterized by the RQ2 CPI decomposition, rather than Class I. XXH3 (M1=1.27) combines work inflation + C3.
- **II① Vectorization loss (new finding from inspection method 2, a more specific mechanism than CPI alone)**: C main auto-vectorizes into **213** vector operations (Rust has only **20**). Much of the XXH32/XXH3 CPI disadvantage stems from **successful C auto-vectorization while Rust remains scalar**. This explains why XXH32 has fewer instructions (M1<1) yet runs slower: C uses a small number of wide vector instructions to outperform the scalar Rust instruction stream.
