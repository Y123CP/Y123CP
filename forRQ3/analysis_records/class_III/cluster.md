# Class III rule synthesis (translation-output level, compared with idiomatic Rust)

**Input**: `class_III/_method.md` (method) + exhaustive recollection (6 candidates + open discovery; criterion: violation of perf-book guidance + recurrence in ≥3 hot functions from regressed workloads; **measured gains are not an admission criterion**).
**Scope**: Class III comprises source patterns in c2rust output that **violate idiomatic Rust best practices from The Rust Performance Book**. The reference is **idiomatic Rust**, rather than C; detection operates at the **source level** (`rust_raw/src`). It explains the distance between c2rust output and optimal Rust, which can **outperform** C after rewriting. Explaining why it is slower than C belongs to Class I/II.

## Underlying principle (synthesis axis)

One underlying principle connects all rules:

> **c2rust mechanically preserves C's low-level semantics (untyped bytes, manual memory management, runtime indirect dispatch) without adopting Rust's higher-level abstractions (typed containers, ownership/RAII, compile-time monomorphization). This both loses information useful to the optimizer and incurs runtime costs from the missing abstractions.**

The four rules are four aspects of this principle. **Synthesis is by principle, not syntax**: `libc::malloc`/`extern "C" fn` are only detection signatures; the substance is the Rust design principle being violated.

## Synthesis principles (three)

1. **Compare against idiomatic Rust, not C**: The criterion is whether c2rust violates a perf-book direction; perf-book provides the rationale for its performance relevance.
2. Establish a rule only if it **recurs in ≥3 hot functions from regressed workloads**. **Do not use measured gains as an admission criterion**; benefit validation belongs to Evaluation, and absence of measured gains does not invalidate a real violation.
3. **Merge by principle**: Combine source patterns that cause the same missing abstraction into one rule.

---

## III① Callbacks → generic monomorphization (perf-book Inlining explicitly mentions c2rust)

- **Rust principle violated**: **Zero-cost abstraction / compile-time monomorphization**. Idiomatic Rust uses generics so that the compiler specializes and inlines each concrete callee. c2rust preserves C's **runtime function pointers** (`extern "C" fn` / `Option<unsafe extern "C" fn>` dispatch tables), paying an indirect jump and `Option` null check on each call and **blocking inlining** because the callee is opaque to the optimizer.
- **Detection signature**: Hot-function signatures/fields contain `Option<unsafe extern "C" fn(...)>`, with indirect calls `f.expect("non-null function pointer")(...)` on the hot path.
- **Rewrite + safety**: Replace function-pointer dispatch with generic parameters / direct calls to concrete callees, removing indirect jumps and null checks while enabling inlining (semantics-preserving, safe rewrite category; checked by the equivalence gate).
- **≥3 supporting instances**: libcsv `csv_parse` (cb1/cb2/is_space/is_term, 22 indirect calls in parsing loops); xxHash `XXH3_hashLong_internal_loop` (f_acc/f_scramble, 3 calls in hot accumulate/scramble loops); bzip2 `BZ2_decompress` (bzalloc/bzfree allocator dispatch, 3 calls). Threshold met.
- **Empirical evidence**: Stage A callback monomorphization in libcsv measured **−32%**, the strongest direct evidence.

## III② Manual memory management → RAII containers (perf-book Heap)

- **Rust principle violated**: **Ownership + typed containers**. Idiomatic Rust uses `Vec`/`Box` to manage heap memory, carrying length/capacity, providing automatic RAII release, and exposing alias information to the optimizer. c2rust preserves **manual management** through C's `libc::malloc/free/calloc/realloc`: memory is treated as untyped bytes, FFI boundaries obstruct inlining, and lifetimes are hidden from the optimizer.
- **Detection signature**: Manual allocation/release through `libc::malloc/free/calloc/realloc` within hot-function bodies.
- **Rewrite + safety**: Replace temporary scratch buffers with `vec![0; n]`/`Vec`/`Box` (RAII). Custom memory-pool paths can be suggested, but their higher rewrite risk must be stated.
- **≥3 supporting instances**: heman `transform_to_distance` (scratch ff/dd/zz/ww, 8 calloc/free sites, straightforward `vec!` candidates); heman `heman_lighting_compute_occlusion` (startpts, 2 sites); libzahl `libzahl_realloc`/`zfree` (the size-bucket memory pool itself). Threshold met.

## III③ Manual memory operations → slices (perf-book Std-lib), the broadest coverage

- **Rust principle violated**: **Typed slice operations**. Idiomatic Rust uses `copy_from_slice`/`fill`/`to_be_bytes` and related operations, carrying length information that the compiler can fold into SIMD/intrinsics. c2rust preserves C's **handwritten byte-wise/unrolled operations** (`memcpy`/`memset`, handwritten byte-order swaps `copy_be*`, Duff's-device-unrolled `libzahl_mem*`), losing length information and optimization opportunities.
- **Detection signature**: `memcpy/memset/strlen`, handwritten `copy_be16/32/64`, and project-specific byte-wise unrolled `*_mem*` operations in hot-function bodies.
- **Rewrite + safety**: `copy_be*` → `to_be_bytes`/`from_be_bytes` + `copy_from_slice`; `*_mem*`/`memset` → `copy_from_slice`/`fill`; `strlen` → slice length/`CStr` (semantics-preserving, safe rewrite category).
- **≥3 supporting instances (8 hot functions / 3 projects, strongest coverage)**: binn `AddValue` (8 strlen/memcpy/copy_be* sites), `GetValue` (5 memset/copy_be* sites), `AdvanceDataPos` (2 copy_be32 sites); libzahl `libzahl_realloc`/`zadd_unsigned_assign`/`zrsh`/`zlsh` (unrolled libzahl_mem*); heman `compute_occlusion` (memset). Threshold met.

## III④ Raw-pointer element-wise cursors → slices/iterators (perf-book Iterators/Bounds-Checks), a common source cause whose consequences are attributed to C1/C3/D1

- **Rust principle violated**: **Slice/iterator abstractions**. Idiomatic Rust uses `.iter()`/`&[T]` to expose length and non-aliasing information, enabling bounds-check elimination, redundant-memory-access elimination, and auto-vectorization. c2rust consistently emits `*p.offset(i)` / post-increment cursor idioms such as `let fresh=p; p=p.offset(1); *fresh=..`, **losing bounds and alias information**.
- **This is the most prevalent source-level cause in c2rust output** (20+ hot functions across all 6 projects).
- **Attribute consequences without counting twice**: III④'s performance consequences are **Class I C1 (bounds checks) + C3 (redundant memory accesses) + Class II D1 (failed vectorization)**. They describe the same defect: III④ captures the **source-level cause**, while C1/C3/D1 capture **IR/remark symptoms**. **Rewrite benefits are attributed to C1/C3/D1; III④ is not counted again toward the Table 7 threshold.** It is listed separately to identify the common source cause, analogous to attributing D3 to C3.
- **Detection signature**: `*ptr.offset(i)` cursors / post-increment pointer idioms in hot loops.
- **Rewrite + safety**: Replace raw-pointer cursors with slice indexing / iterators so that the optimizer can prove non-aliasing and boundedness. This jointly enables C1 check elimination + C3 redundancy elimination + D1 vectorization; safety requires the same non-aliasing proof as C3.
- **Supporting instances**: 20+ hot functions across all 6 projects (bzip2 `BZ2_decompress`/`unRLE_*` 107+, libcsv `csv_parse` 17, heman 7 hot functions, binn 3, libzahl 3, xxHash `XXH3` loop). Widespread support.

---

## Shared principle and relationship to Class I/II

- **Four rules are four aspects of one principle**: c2rust preserves C's low-level semantics without adopting Rust's higher-level abstractions. III① lacks **monomorphization**; III② lacks **RAII containers**; III③ lacks **typed slice operations**; III④ lacks **slice/iterator abstractions**.
- **III④ is the common source cause, while C1/C3/D1 are its IR consequences**: cause versus symptom, two levels of the same defect.
- **Most Class III rewrites are directions for speedup** relative to idiomatic Rust. Rewritten Rust can **outperform** C, consistent with the premise that C is not the ceiling. Individual direct evidence: III① libcsv −32%.

## Exclusion record (rechecked against the same threshold: objectively <3 or unsupported by perf-book)

- **E Reduce allocations**: Only 0–1 repeated allocations actually occur inside hot loops; scratch buffers are allocated outside loops, libzahl growth is one-off, and the xxHash buffer is already hoisted. There are **no qualifying instances**, so no rule is established; this is not exclusion for lack of measured gains.
- **F Reduce type sizes**: `as` casts are pervasive (csv_parse 182, bzip2 ~1656…), but **perf-book has no general direction to avoid `as` casts**. Type-sizes concerns reducing type sizes, not removing casts, so this lacks a basis. The supported union+`transmute` pattern occurs only at 1 binn `GetValue` site, <3; no rule.
- **G Buffered I/O**: Hot functions contain only 2 bzip2 `verbosity`-gated `fprintf` calls (cold paths); libcsv `fputc` is not hot in this workload. Fewer than 3 and all cold; no rule.
- **Secondary structural note (no rule)**: c2rust's `current_block: u64` goto-emulation state machine (bzip2 `BZ2_decompress`, 3 arms; binn `AddValue`) fragments hot loops and obstructs slice/iterator restructuring. **This is not a perf-book direction and occurs in only 2 hot functions**, so it remains a structural note.

## Summary

Class III contains **4 rules** describing source-level problems in c2rust output from preserving C's low-level semantics without adopting Rust's higher-level abstractions:
- **III① (callbacks→generics)**: 3 hot functions / 3 projects, explicit perf-book support + −32% empirical evidence.
- **III② (manual memory→RAII)**: 4 hot functions / 2 projects.
- **III③ (manual memory operations→slices)**: 8 hot functions / 3 projects, broadest coverage.
- **III④ (raw pointers→slices/iterators)**: 20+ hot functions / all 6 projects; the common source cause, with consequences attributed to C1/C3/D1 and no duplicate threshold counting.

Detection operates on **Rust source code**, using perf-book as the reference and requiring no C implementation. Rewrites are **semantics-preserving** (safe rewrite category, checked by the equivalence gate) and mostly identify speedup directions that may outperform C. E/F/G objectively fail the criteria (no instances / no perf-book basis / only cold paths) and are excluded explicitly.
