# Class III collection method (translation-output level, compared with idiomatic Rust)

**Scope**: Class III identifies where c2rust output **violates idiomatic Rust performance practices from The Rust Performance Book**. It differs from Class I/II in the following respects:

| | Class I/II | **Class III** |
|---|---|---|
| Reference | C | **Idiomatic Rust (perf-book)** |
| Criterion | Rust vs C difference | **c2rust violates a perf-book principle** |
| Level | Optimized IR / remarks | **Source code (`rust_raw/src`)** |
| Explanation | Why it is slower than C | How far c2rust is from optimal Rust (rewrites can **outperform** C) |

## Underlying principle (synthesis axis)

Class III has **one** underlying principle:

> **c2rust mechanically preserves C's low-level semantics (untyped bytes, manual memory management, runtime indirect dispatch) without adopting Rust's higher-level abstractions (typed containers, ownership/RAII, compile-time monomorphization). This both loses information useful to the optimizer and incurs runtime costs from the missing abstractions.**

Each rule is one **aspect** of this principle: A callbacks violate zero-cost abstraction/monomorphization; C/D libc usage violates ownership and typed-container principles (length/alias information is hidden from the optimizer, and FFI boundaries obstruct inlining); E allocation reduction concerns allocation cost and reuse locality; F type-size reduction concerns cache/bandwidth effects; G I/O concerns syscall cost and buffered batching.

## Syntax versus principle (key distinction)

Detection signatures (`libc::malloc` / `extern "C" fn`) only **locate sites**; the **substance of a rule is the Rust design principle being violated**. Synthesis is **by principle, not syntax counts**. Similarly, C1 concerns language-semantic obligations rather than grepping for panic_bounds_check, and D1 concerns code structures that break vectorization prerequisites rather than grepping for loop not vectorized.

## Collection procedure

1. **Reference**: Idiomatic Rust directions abstracted from the full perf-book.
2. **Scope**: c2rust source code in `rust_raw/src` for hot functions in regressed workloads (≥80% runtime coverage, as in I/II).
3. **Criterion**: c2rust violates a direction and the violation **recurs in ≥3 hot functions from regressed workloads**. **Measured performance gains are not an admission criterion**: an authoritative perf-book principle and an objective violation suffice; benefit validation belongs to Evaluation. Do not reject a real violation merely because no measured gain has been observed.
4. **Each direction → rule**: Principle + detection signature + rewrite direction + safety conditions + evidence.

## Candidate directions (abstracted from the full perf-book, beyond the previous 4; open to discovery)

- **A Callbacks → generics** (Inlining; perf-book **explicitly mentions c2rust**)
- **C Manual memory → RAII containers** (Heap: `malloc/free/realloc` → `Vec`/`Box`)
- **D Manual memory operations → slices** (Std-lib: `memcpy/memset/strlen` → `copy_from_slice`/`fill`/`&str`)
- **E Reduce allocations** (Heap: repeated allocation in hot loops → reuse outside the loop)
- **F Reduce type sizes** (Type-sizes: union+`transmute`, pervasive `as` casts, redundant `c_int`/`usize`)
- **G Buffered I/O** (I/O: unbuffered `libc::fread/fwrite/fputc` → `BufReader`/`BufWriter`)
- Open discovery: collection may report other directions that violate perf-book guidance.

## Exclusions (scope and reasons)

- **B Raw pointers → iterators/slices**: The most prevalent c2rust antipattern (`*p.offset(i)` in every loop), but its consequences—bounds checks, redundant memory accesses, and failed vectorization—**are already attributed to Class I C1/C3 + Class II D1 through IR/remarks**. The source cause and IR consequence are two views of the same defect; do not count them twice toward the threshold.
- **H Fast hashing** (FxHashMap): c2rust does not generate Rust `HashMap`; it preserves C arrays/handwritten tables. There is no applicable violation.
- **I SIMD intrinsics** (machine-code guidance to use `core::arch`): Requires **handwritten SIMD**, beyond mechanical rewriting of translation output, and overlaps with D1 vectorization loss.
- **J Parallelism** (rayon): perf-book describes this as beyond its scope; it requires algorithm restructuring and is outside the scope here.

## Confidence

Class III compares against **idiomatic Rust**, not C. Most rules therefore identify **directions for speedup** that can make rewritten Rust outperform C, consistent with the premise that C is not the performance ceiling. Some have direct empirical support: A callback monomorphization, Stage A, measured libcsv −32%.
