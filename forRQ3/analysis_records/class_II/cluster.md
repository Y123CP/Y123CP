# Class II rule synthesis (gap clustering based on evidence from 6 projects)

**Input**: `class_II/{bzip2,heman,libzahl,xxHash,binn,libcsv}.md` (Class II differential observations from 6 projects with regressions; collection method in `_method.md`).
**Scope**: Class II covers **missed optimizer transformations** (M-b): under the same LLVM 17.0.6 backend, an optimization succeeds in C but fails or is not performed on c2rust (Rust) output. It is orthogonal to Class I's **additional frontend-generated** obligation work, but often represents **downstream optimizer consequences of the same c2rust defects** (see the relationship to Class I).

## Synthesis principles (four)

1. **Merge by gap dimension**: Triggering mechanisms that cause **the same physical performance gap** (the same category of wasted CPU work) are synthesized into **one** rule. Cluster by gap, rather than splitting by pass name or failure reason.
2. **Recurrence threshold = ≥3 supporting instances per rule**. Instances may be ≥3 mechanisms in the same dimension **or** the same mechanism in ≥3 hot functions/sites. Cross-project breadth is reported but is not an admission criterion.
3. **Confidence ●/◐ (specific to Class II)**: Admission depends on a Rust miss; success of the corresponding C remark determines confidence. **Rust missed while C succeeded = ● (explains the C↔Rust gap)**; a shared miss = ◐ (does not explain the gap, but rewriting may still improve Rust's absolute performance).
4. **Count unique `(file,line,col)` sites, never raw aggregates**. Header-only code, force-inlining, and macro expansion inflate raw counts; see `_method.md`. Thresholds and ● judgments use **unique sites in hot functions**.

These principles yield **3 Class II rules** (D1/D2/D3), covering three categories of missed optimization: SIMD parallelism / calls and interprocedural optimization / memory-alias optimization. **Channel exhaustiveness** is verified by all-pass enumeration (loop-unroll/idiom/delete parity; no unswitch/gvn-sink/machine-licm remarks; see `_method.md`). **D3 (memory-optimization loss) is rooted in missing alias information and is already attributed to Class I C3; it is not counted again toward the threshold.** It is listed separately to present the complete set of missed optimizer transformations.

---

## D1 — Vectorization loss (unrealized SIMD parallelism)

- **Gap dimension**: Hot loops / straight-line code could use SIMD, but Rust retains scalar element-wise execution and loses vector throughput.
- **Three merged mechanisms** (one gap dimension: c2rust code structure breaks vectorization prerequisites):
  - **(a) Bounds-check panics break countability**: A c2rust `panic_bounds_check` branch before each array access adds a **second loop exit**. Together with signed-`c_int` induction and writes of loop-carried variables back to state fields, it prevents LLVM from determining the trip count or makes values used outside the loop, causing loop-vectorizer rejection. **This shares its root cause with Class I C1**: C1 checks obstruct vectorization at the Class II level.
  - **(b) Missing inlining prevents unrolling and degrades SLP trees**: A hot kernel retains an out-of-line call (see D2); `call instruction cannot be vectorized` blocks the loop, and a wide SLP tree cannot form (28→6). **This is downstream of D2.**
  - **(c) Raw-pointer aliasing blocks vectorization**: Non-aliasing of `*mut` targets cannot be proved, so strided-scatter and similar scattered-write loops are not vectorized. **This shares its root cause with Class I C3.** Most such issues disappear under LLVM17 (libzahl's 7 `cannot identify array bounds` sites were LLVM16 artifacts), leaving only isolated cases.
- **Detection signature** (Rust no-LTO LLVM17 remarks, with `vectorized loop` for the corresponding C loop):
  ```
  hot(f) ∧ C_remark(loop L) = "vectorized loop (width N)"
          ∧ Rust_remark(corresponding L) ∈ { "could not determine number of loop iterations" // (a)
                                    | "value ... used outside the loop"                   // (a)
                                    | "call instruction cannot be vectorized"             // (b)
                                    | "loop not vectorized" (alias-related)}              // (c)
  ```
- **Rewrite + safety** (M-b; (b) is semantics-preserving, while (a)/(c) use C1/C3 rewrites):
  ```
  (a) Remove bounds checks + restructure control flow (iterators / .get_unchecked ⊢ i<len) + remove state-field writebacks ⟹ restore countability
  (b) Add #[inline(always)] to thin kernels ⟹ restore unrolling ⟹ restore SLP trees (semantics-preserving; no proof needed)
  (c) *mut T ⟹ &mut [T] (obtain noalias) or scalar hoisting ⊢ writes ⊥ addr (same non-aliasing proof as C3)
  ```
- **≥3 supporting instances** (unique sites in hot functions; 3 mechanisms / 3 projects):
  - (a) **5 hot decoding loops in bzip2 `BZ2_decompress`** (decompress.c :311 w32 / :316 w16 / :409+:416 w4 / :447 w32; all Rust misses). One mechanism in one project already satisfies ≥3.
  - (b) **xxHash `XXH3_hashLong` accumulate kernel** (SLP tree 28→6; scramble SLP entirely lost).
  - (c) **heman `transform_to_distance` strided-scatter** (distance.c:90 w8 / distance.rs:173 missed; true ●, small scope). Threshold met.
- **Breadth limitation**: 3 projects; the main evidence is (a) bzip2 + (b) xxHash. Under LLVM17, (c) has only 1 remaining heman site; LLVM17 largely resolves raw-pointer aliasing-related vectorization losses.

## D2 — Inlining loss (call overhead + blocked interprocedural optimization)

- **Gap dimension**: Small hot-path functions retain out-of-line calls, paying call/ret overhead and losing constant propagation, alias refinement, and loop unrolling that inlining could expose.
- **Two merged mechanisms** (one gap dimension: hot callees remain uninlined):
  - **(a) c2rust discards `force-inline` attributes**: C `XXH_FORCE_INLINE`=`__attribute__((always_inline))` bypasses the cost model to force inlining. c2rust does not preserve this attribute, so LLVM evaluates cost independently and rejects the call (cost 395 > threshold 375). **A root cause**, also triggering D1(b).
  - **(b) c2rust body inflation raises inline cost**: Verbose bodies (raw-pointer bookkeeping + bounds checks) push callee cost above the threshold (`zinit_temp` 685 vs C 115, `zfree` 280 vs C 10); C inlines them while Rust does not.
- **Detection signature**:
  ```
  hot(caller) ∧ Rust_remark = "'callee' not inlined into 'caller' because too costly (cost=X, threshold=Y)"
              ∧ callee is a small same-crate function ∧ the corresponding C callee is inlined (always_inline or low cost)
  ; Exclude cross-crate (LTO-resolvable) / framework-noinline / already inlined by MIR
  ```
- **Rewrite + safety** (M-b, semantics-preserving; no correctness proof required):
  ```
  (a) Add #[inline(always)] to hot callees ⟹ restore C's always_inline behavior
  (b) First shrink the body using C1–C3 to bring cost below the threshold, or directly add #[inline]
  ```
- **≥3 supporting instances** (unique sites in hot functions; 2 mechanisms / 2 projects):
  - (a) **xxHash `XXH3_accumulate_512_scalar` is not inlined into 4 hot callers** (cost 395>375; `accumulate_scalar`/`hashLong`×2/`consumeStripes`/`digest_long`). One mechanism in one project already satisfies ≥3.
  - (b) **2 callees in libzahl `zmul_ll`** (`zinit_temp` 685>325, `zfree` 280>250). `zsqr_ll` has the same pattern but is not a ≥5% hot function, so it provides lower-weight supplementary non-hot evidence. Threshold met.
- **Breadth limitation**: 2 projects; xxHash mechanism (a) is a root cause that also triggers D1.

## D3 — Memory-optimization loss (redundant memory accesses remain; root cause attributed to Class I C3, without duplicate threshold counting)

- **Gap dimension**: Missed **scalar memory optimizations**: GVN does not eliminate redundant loads, and LICM does not hoist loop-invariant loads/GEPs. Rust lacks `noalias`/`!tbaa`, so the optimizer cannot prove non-aliasing and conservatively retains cross-iteration memory accesses.
- **Relationship to C3 (central)**: This is the **optimizer-remark view of the same phenomenon as Class I C3 (missing alias information)**. C3 records **IR memory-access counts** (redundant raw-pointer loads, `!tbaa`=0); D3 records **optimizer remarks** (GVN/LICM misses). These are two measurements of the same phenomenon. **The root cause is the frontend's lack of alias information (M-a), already counted under C3; D3 does not independently count toward the ≥3 threshold.** It is listed only to complete the missed-optimization inventory.
- **Two merged mechanisms** (same gap dimension): Missed GVN load elimination (`load of type X not eliminated`) + missed LICM hoisting (`failed to hoist load with loop-invariant address`).
- **Detection signature**:
  ```
  hot(f) ∧ Rust_remark ∈ { "gvn: load ... not eliminated"
                          | "licm: failed to hoist load with loop-invariant address" }
          ∧ Rust missed ≫ C (alias-related wording) ∧ !tbaa ∉ meta(memory accesses)
  ```
- **Rewrite + safety** (= C3 rewrite, requiring a non-aliasing proof):
  ```
  *mut T ⟹ &mut [T] (restore noalias) or scalar hoisting   ⊢ writes(L) ⊥ addr(p)
  ```
- **Supporting instances** (clearest in libzahl remarks; threshold evidence belongs to C3's cross-project IR memory accesses): libzahl `zrsh` gvn missed 46/16 (R/C), `zsub` gvn 43/27 + licm 16/4, `zlsh` gvn 54; all 6 projects have `!tbaa`=0 + redundant loads (C3 IR: bzip2 1010/755, libzahl zrsh 73/24).
- **Breadth**: The remark evidence is clear in libzahl (bignum raw-pointer cursors). In other projects, macro expansion/force-inlining **confounds per-instance gvn/licm counts**: higher C missed counts in bzip2/heman are expansion artifacts. Therefore, use C3 IR memory-access counts as the reference across projects.
- **Same cause as D1(c), different gap**: D1(c) blocks vectorization and D3 blocks scalar memory optimization. Both originate from missing alias information (C3), but concern SIMD and GVN/LICM respectively.

---

## Causal chain and relationship to Class I (central Class II insight)

- **D2 → D1 causality** (clear in xxHash): Inlining loss (D2(a), discarded force-inline) → out-of-line kernel → blocked unrolling → degraded SLP trees (D1(b)). **Inlining loss is the cause; vectorization loss is the consequence.**
- **Class II as downstream consequences of Class I defects**: The classes are not entirely orthogonal.
  - D1(a) is a **second-order consequence of C1 (redundant bounds checks)**: panic branches break loop countability.
  - D1(c) + the already-attributed licm/gvn misses are **optimizer consequences of C3 (missing alias information)**: inability to prove non-aliasing makes vectorization and memory optimization conservative.
  - For the same c2rust defects, Class I records additional obligation work, and Class II records optimizations consequently missed.
- **Multiple rewrite benefits**: Fixing C1 (removing bounds checks) also enables D1(a) vectorization; fixing C3 (&mut slices) also enables D1(c). D2 + D1(b) use **purely semantics-preserving `#[inline]` rewrites specific to Class II**, requiring no correctness proof and offering the most straightforward transformation.

## Exclusion record (rechecked against the same threshold)

- **licm/gvn clusters of `load not eliminated` / `can't hoist` misses**: See **D3 (memory-optimization loss)**. The root cause is attributed to C3 without duplicate ≥3 threshold counting; D3 is listed, not excluded.
- **loop-unroll / loop-idiom / loop-delete**: Parity across both sides in all projects; no gap, excluded.
- **loop-unswitch / gvn-sink / machine-licm**: No remarks emitted on either LLVM17 side; nothing to collect.
- **binn / libcsv**: No Class II gap (vectorization 0/0 on both sides; symmetric inlining, sometimes slightly favoring Rust). Degradation belongs entirely to Class I (C1+C3).
- **Pure ◐ clusters**: None in this corpus. libzahl arithmetic kernels have carry dependencies that prevent vectorization on both sides, rather than an available opportunity. **All Class II evidence in this corpus is ●** (C succeeds, Rust fails), overturning the earlier class_II conclusion of no ● and all ◐, which used outdated workloads and asymmetric backends.

## Summary

Class II contains exactly **3 rules**, covering the complete set of missed optimizer transformations:
- **D1 (vectorization loss)**: 3 mechanisms (broken countability / missing unrolling / aliasing barriers), bzip2 5 loops + xxHash kernel + heman 1 site; meets ≥3.
- **D2 (inlining loss)**: 2 mechanisms (discarded force-inline / body inflation), xxHash 4 callers + libzahl 2 callees; meets ≥3.
- **D3 (memory-optimization loss)**: GVN/LICM misses in multiple libzahl hot functions + missing tbaa across all projects. **The root cause is attributed to C3 without duplicate threshold counting**; listed for completeness.

Detection uses **no-LTO LLVM17 remarks from the target Rust project**, with success for the corresponding C loop/callee supporting ●. Rewrite categories: **D2 + D1(b) are purely semantics-preserving `#[inline]` changes**, the most straightforward Class II contribution, requiring no correctness proof. D1(a)/D1(c)/D3 use C1/C3 rewrites and share their safety proofs. **The central insight is that Class II captures optimizer consequences of Class I defects (C1→countability, C3→aliasing/memory optimization), and inlining loss (D2) causally triggers vectorization loss (D1). All Class II evidence in this corpus is ●: real, structural sources of C↔Rust gaps verified with symmetric backends. The remark channel has a coverage limit: machine-level scheduling/regalloc/peephole transformations do not emit remarks. CPI-related gaps are addressed by RQ2.**
