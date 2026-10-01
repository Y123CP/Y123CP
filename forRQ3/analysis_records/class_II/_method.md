# Class II collection method (missed optimizer transformations, M-b)

**Scope**: Class II concerns optimizations that **succeed in C but fail or are not performed in Rust (c2rust output)** within hot functions of regressed workloads. It corresponds to RQ3 inspection method 2: optimizations present in C but missing from Rust. It is orthogonal to Class I: Class I captures obligation work **additionally generated** in Rust relative to C (surviving obligation symbols); Class II captures optimizations **performed less often** in Rust (abandoned opportunities).

## Collection channel: Compiler remarks

Use the same LLVM 17.0.6 backend (clang-17 == rustc nightly-2024-01-15) and have both compilers **report** success or failure at each optimization site:
- **Rust**: `RUSTFLAGS="-Cdebuginfo=1 -Ctarget-cpu=native -Ctarget-feature=-avx512f -Cllvm-args=-pass-remarks[-missed/-analysis]=<regex>"` (remarks include `!dbg` source locations).
- **C**: `clang-17 -O3 -march=native -mno-avx512f -gline-tables-only -c <lib>.c -o /dev/null -Rpass[-missed/-analysis]=<regex>`.

### Collection pitfalls (four requirements; violating any produces misleading data)

1. **Repeated flags do not accumulate; use one combined regex.** Both clang `-Rpass=` and rustc `-pass-remarks=` accept **one regex; later occurrences override earlier ones**. Passing `=loop-vectorize`, `=slp-vectorizer`, and `=inline` separately silently retains only the last (`inline`), making vectorization appear consistently 0 and falsely suggesting complete loss. Use a combined regex: `-Rpass='loop-vectorize|slp-vectorizer|inline'`.
2. **Enable debug information on both sides.** clang-17 does not emit **vectorization** remarks without `-g` (inlining remarks do not require it). Add `-gline-tables-only` for C and `-Cdebuginfo=1` for Rust; otherwise C also appears to lack vectorization.
3. **Use symmetric no-LTO builds.** fat-LTO postpones vectorization/inlining to the **link-stage plugin**, where per-CGU `-Cllvm-args`/`-Cremark` often **miss the remarks**. In observed fat-LTO builds of the heman/libzahl/binn library crates, vectorization passed=0 reflected missing collection; whole-program merging inflated inlining costs, producing apparent losses such as binn `copy_value` and heman "cost 6325". Therefore, use no-LTO counterparts on both sides to inspect intrinsic optimizer decisions on **module-local code structure**: C = per-TU `-c` without `-flto`; Rust = a standalone no-LTO release build of the c2rust library crate (`rust_raw`), without harness fat-LTO. Exception: bzip2 is a self-contained binary whose CGU layout yields **the same** vectorization remarks under fat-LTO and no-LTO, without missing collection. Nevertheless, use no-LTO uniformly across the corpus. This counterpart reflects the **intrinsic** optimizability of c2rust code structure, which is distinct from the absolute performance of the measured binary.
4. **Override the LLVM 16 pin bundled with rust_raw to LLVM 17** (the least obvious pitfall). All `rust_raw/rust-toolchain.toml` files pin `nightly-2023-04-15` = rustc 1.70 = **LLVM 16**, an old pin in c2rust output. Running `cargo build` in rust_raw without `+nightly-2024-01-15` therefore uses LLVM 16, creating **backend asymmetry** with C's clang-17/LLVM17 (different vectorization/inlining capabilities), contrary to the fair-build requirement. **Use `cargo +nightly-2024-01-15 build`** (= rustc 1.77 = LLVM 17.0.6). Observed LLVM16→17 changes affect **raw-pointer vectorization decisions**: the 7 libzahl `cannot identify array bounds` sites are LLVM 16 artifacts that disappear in 17. They **do not change the inline cost model** in the observed xxHash case (cost 395>375 in both versions). Vectorization loss therefore requires symmetric LLVM17 verification; the inlining loss is version-robust. Some toolchains emit nothing for rustc `-Cllvm-args=-pass-remarks`; use native rustc `-Cremark=all` instead (`note:` lines, with `(success)` as the passed token).

Two directions:
- **II① Vectorization loss**: pass = `loop-vectorize` (loop vectorization) + `slp-vectorizer` (packing straight-line code with SLP).
  - Passed wording: `vectorized loop` / `SLP vectorized` / `Stores SLP vectorized` / `Vectorized horizontal reduction` (negative cost = beneficial).
  - Missed wording: `loop not vectorized` / `Cannot SLP vectorize: impossible with available factors` / `not beneficial cost N>=N`.
- **II② Inlining loss**: pass = `inline`.
  - Missed wording: `'callee' not inlined into 'caller' because too costly to inline (cost=X, threshold=Y)` / `should never be inlined`.
  - Mechanism: c2rust-generated function-body inflation (obligation checks + missing `#[inline]` hints) pushes cost above LLVM's inline threshold. Small functions inlined in C remain out-of-line calls in Rust.

### Exhaustive check of other missed-optimization channels (all-pass enumeration)

Enumeration with an all-pass regex (Rust `-Cremark=all` / C `-Rpass-missed='.*'`) in bzip2/xxHash/libzahl/heman hot functions finds only licm/gvn/loop-unroll/loop-idiom/loop-delete remarks beyond vectorize/inline:
- **licm/gvn clusters of `load not eliminated` / `can't hoist load with loop-invariant address` misses belong to Class I C3** (missing alias information), rather than duplicate Class II rules. Their wording concerns aliasing (Rust lacks `noalias`/`!tbaa`, so memory optimizations are conservative). This is the optimizer-remark view of C3 redundant memory accesses: libzahl `zrsh` gvn missed 46/16 and the class_I C3 load counts 73/24 describe the same phenomenon through different measures. Macro expansion/force-inlining confounds per-instance counts; use C3 IR memory-access counts as the reference.
- **loop-unroll / loop-idiom / loop-delete**: Parity on both sides; no gap.
- **loop-unswitch / gvn-sink / machine-licm**: Neither LLVM17 side emits remarks; nothing can be collected.
- xxHash licm/gvn C-passed≫Rust is **downstream of** II② inlining loss (these passes operate on the force-inlined combined body), rather than an independent channel.

⇒ **Class II is limited to II① vectorization and II② inlining, an exhaustive set of independent channels** under the all-pass enumeration.

## Filtering rules (remove infrastructure noise)

- **Vectorization** remark locations are call-site source `file:line` locations. Filter these to **translated library .rs source files**, excluding `/rustc/` std and infrastructure portions of harness `main.rs`.
- **Inlining** remark locations refer to callee definitions. Filter by **caller ∈ hot functions and callee ∈ internal library functions**; also exclude the following spurious losses:
  - **Infrastructure entries**: callee/caller contains `core/alloc/std/demangle/gimli/miniz/backtrace/fmt/panic` (backtrace/formatting remnants under `panic=abort`, outside the translated library); `cost=never` for intentional `noinline`.
  - **Cross-crate (LTO-resolvable)**: A cross-crate callee reported as not inlined in a no-LTO library-crate build may be inlined at link time in the measured fat-LTO binary. **This is not a true loss**; an example is bzip2 `BZ2_hbCreateDecodeTables×20` in the huffman crate. True inlining losses must occur **within the same crate/TU** because c2rust body inflation pushes cost above the threshold.
  - **Already inlined (at any stage)**: A callee folded by rustc's MIR inliner before LLVM (no LLVM remark), or by LLVM's inliner (success remark), is **already inlined**, not lost. Neither absence of an inline-pass remark nor an inline-missed remark accompanied by a success establishes a loss. In LLVM17, libcsv `csv_increase_buffer` explicitly succeeds through the **LLVM inliner**, cost 110/250, contrary to the earlier interpretation of early MIR folding; both final states nevertheless mean already inlined.

## Counting convention (deduplication unit)

Use **unique `(file,line,col)` sites** (or hot-kernel sites), **not raw remark instances**. A source site can be reported repeatedly: (a) a header-only / force-inlined callee produces a remark per inlined copy (xxHash C SLP raw 78 = **14 unique sites** inlined into ~40 callers; full-library C passed totals in bzip2 are similarly inflated); (b) the pipeline runs a pass repeatedly. **Raw aggregates are not comparable across sides/projects and must not enter the ≥3 threshold.** Thresholds and ● judgments always use **unique sites within hot functions** (bzip2 5 hot loops; xxHash accumulate 4 callers; libzahl zmul_ll 2 callees). Full-library aggregate counts in the project tables provide background only, not cross-project comparisons or admission evidence.

## Confidence definitions (●/◐)

- **● (strong; explains the gap)**: For corresponding source logic, **C passed while Rust missed**. This is an optimizer difference under the same LLVM: c2rust's IR structure causes the optimizer to abandon an optimization that succeeds in C.
- **◐ (moderate; does not explain the gap, but rewriting can help)**: Both sides missed, or Rust passed ≥ C. The gap does not arise from this optimization difference, but an explicit rewrite (guiding vectorization/adding inline hints) may still improve Rust's absolute performance.

## Threshold (same as Class I)

A rule requires ≥3 supporting instances: ≥3 patterns in the same dimension **or** the same pattern in ≥3 hot functions/sites. Cross-project breadth is reported but is not an admission criterion.

## Hotspot inventory convention

Reuse the Class I hot-function inventory (perf self%, ≥5% threshold, workloads classified as regressions in RQ1). Class II counts remarks only within these hot functions and their inlined bodies.
