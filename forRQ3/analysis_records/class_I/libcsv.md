# libcsv — Class I Differential evidence (collection stage; no classification decisions)

**Build**:C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -g -Wl,--plugin-opt=save-temps`(`main.c libcsv.c`)→ precodegen → `llvm-dis-17`;Rust = the fat-LTO whole-program module of the measured binary(`rust_harness/target/release/deps/csv_count-*.ll`,`cargo build --release --features op_write` + `-Cdebuginfo=1 --emit=llvm-ir`,nightly-2024-01-15 = LLVM 17.0.6).

**Attribution**: Counts are collected per hot-function `define` region. Obligation and control constructs are attributed by `DILocation` scope to self / inherited from inlined functions / nodbg.

## Workload: parse  (M1=1.267;RQ1 gap = +35.1% → **regression**)

**① Hotspot inventory**(synth.csv):

| Function | self% | ≥5% |
|---|---:|:-:|
| `csv_parse` | 83.9 | ✓ |
| `csv_count::cb_field` | 5.2 | ✓ (mostly inlined into csv_parse; only 3 IR lines remain) |

## Workload: write  (M1=0.994;RQ1 gap = +8.3% → **regression**)

**① Hotspot inventory** (escaping 10k fields; `csv_write` inlined into harness main):

| Function | self% | ≥5% |
|---|---:|:-:|
| `csv_count::main` (including inlined `csv_write`) | 97.1 | ✓ |

**② Construct counts in optimized IR for each hot function**:

| Function | Obligation/control constructs (Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `csv_parse` | Option unwrapping inherited 2 + nodbg 2 | 63 / 25 / 33 / **0** | 72 / 31 / 38 / **103** |
| `csv_count::main`(write) | unwind nodbg 9;Option unwrapping nodbg 5;Bounds checks inherited write_workload 1 + nodbg 1;memcpy/memset(control, inherited csv_init/csv_fini and others) | 88 / 149 / 161 / **0** | 39 / 46 / 58 / **85** |

**③ Instruction-region sizes (Rust/C lines) and open-ended scan**:

| Function | Rust lines | C lines | Open-ended scan (Rust-only symbols) |
|---|---:|---:|---|
| `csv_parse` | 727 | 880 | — |
| `csv_count::cb_field` | 3 | Inlined | `FIELD` ×1 |
| `csv_count::main`(write) | 1420 | 786 | `llvm.assume` ×6, `llvm.experimental.noalias.scope.decl` ×9, `alloc::alloc`/`raw_vec`/`__rust_no_alloc_shim`(harness infrastructure) |

Note: `csv_parse` Rust 727 < C 880 (smaller static code size). Rust-only symbols in the write harness main all belong to std/alloc infrastructure, rather than the translated library.

**④ Inspection method 2: Evidence of successful optimization (present in C / less frequent in Rust)**:

| Function | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `csv_parse` | 0 / 0 | 0 / 0 | 3 / 0 |
| `csv_count::main`(write) | 0 / 0 | 23 / 16 | 55 / 23 |

Note: bswap counts are all 0; vectors: Rust ≥ C (little substantive scope for vectorizing character scans). There is no optimization present in C but less frequent in Rust. The libcsv gap falls under C1+C3 in inspection method 1.

---

**Collection-stage summary (differential observations only; no classification decisions)**:
- **C1 Redundant check branches**: `csv_parse` Option unwrapping: Rust 4 (inherited 2 + nodbg 2) vs C **0**; harness main on the write path also contains Rust bounds checks/unwind absent from C.
- **C3 Redundant memory accesses / missing alias information**: `csv_parse` `!tbaa`: Rust **0** vs C **103**; write main: Rust 0 vs C 85.
- **C2 Saturating casts**: libcsv has no floating-point operations; `fptosi.sat` is 0 on both sides (not applicable).
