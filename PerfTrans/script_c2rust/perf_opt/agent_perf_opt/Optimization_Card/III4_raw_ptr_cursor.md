# Rule III④: Raw-pointer Cursor → Slice / Iterator

## 1. What you're fixing

The function contains **raw-pointer cursor** patterns. c2rust preserved
C's per-element pointer arithmetic instead of Rust's slice / iterator
abstraction, so the compiler has lost both **length** and **noalias**
information — it cannot fold bounds checks, hoist invariant loads, or
auto-vectorize the loop.

Two shapes (the driver tells you the exact sites and `pattern_kind`):

- **P1 — cursor-index-deref**: `*<p>.<method>(<idx>)`
  e.g. `*buf.offset(i as isize)`, `*ptr.add(pos)`

- **P2 — post-increment**: `<p> = <p>.<method>(<n>)` (self-assign),
  often as a three-line c2rust idiom:
  `let fresh = p; p = p.offset(1); *fresh = ...`

`<method>` ∈ `{ offset, wrapping_offset, add, sub, wrapping_add,
wrapping_sub }`.

**Direction**: rewrite the raw pointer + length (or sentinel) into
`&[T]` / `&mut [T]` with indexing or iteration.

---

## 2. Establish the length first (soundness gate)

Every slice rewrite must first prove an upper bound on `len`. Four
sources, in order of confidence:

| Tier | Source | Example |
|---|---|---|
| L1 | Explicit `len: usize` parameter | `fn f(p: *mut T, len: usize)` |
| L2 | Struct field (must not be written in the rewrite region) | `for i in 0..(*s).size { *s.buf.offset(i) = ... }` |
| L3 | Compile-time constant | `const N: usize = 256;` |
| L4 | Sentinel termination | `while *p != 0`, `while p < end_p` |

**If none of L1–L4 applies, abstain from this site.** Never guess a
length — that is instant UB.

---

## 3. Rewrite templates

### 3.1 P1 — cursor-index-deref → slice indexing

```rust
// BEFORE
unsafe fn f(buf: *mut T, len: usize) {
    let mut i: usize = 0;
    while i < len {
        *buf.offset(i as isize) = compute(i);
        i = i.wrapping_add(1);
    }
}

// AFTER
fn f(buf: &mut [T]) {
    for (i, dst) in buf.iter_mut().enumerate() {
        *dst = compute(i);
    }
}
```

If the signature is fixed (`extern "C"` / wide public ABI), reborrow
inside the body:

```rust
unsafe fn f(buf: *mut T, len: usize) {
    // SAFETY: caller guarantees `buf` points to `len` T's with no aliasing.
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    for (i, dst) in buf.iter_mut().enumerate() {
        *dst = compute(i);
    }
}
```

### 3.2 P2 — post-increment → iterator

Fixed-count loop:

```rust
// BEFORE
let mut p_src = src;
let mut p_dst = dst;
let mut n = count;
while n != 0 {
    *p_dst = *p_src;
    p_src = p_src.add(1);
    p_dst = p_dst.add(1);
    n -= 1;
}

// AFTER
// SAFETY: caller guarantees `src`/`dst` each hold `count` T's and the
// two regions do not overlap.
let src = unsafe { core::slice::from_raw_parts(src, count) };
let dst = unsafe { core::slice::from_raw_parts_mut(dst, count) };
dst.copy_from_slice(src);           // requires T: Copy and the §4 conditions
```

Sentinel termination (L4):

```rust
// BEFORE
let mut p = str_start;
while *p != 0 {
    process(*p);
    p = p.add(1);
}

// AFTER
// SAFETY: caller guarantees the buffer is NUL-terminated within a
// single allocation.
let cstr = unsafe { core::ffi::CStr::from_ptr(str_start as *const i8) };
for &byte in cstr.to_bytes() {
    process(byte);
}
```

### 3.3 Paired P1 + P2 (the canonical c2rust three-line idiom)

```rust
// BEFORE
let fresh42 = pDst;
pDst = pDst.offset(1);
*fresh42 = byte;

// AFTER — collapse all three lines into an iterator / indexed write;
// never rewrite only the middle line.
```

### 3.4 Prefer the iterator form — it removes the per-element bounds check

A *naive index* rewrite (`for i in 0..n { a[i] = f(b[i]); }`) bounds-checks
`a[i]` / `b[i]` on **every iteration** — in a hot loop that overhead can
erase the win. The **iterator form** checks the length **once** at the
slice boundary and emits **no per-element bounds check** (and often
auto-vectorizes). This is what makes a hot-loop III④ rewrite come out
**neutral or faster** instead of slower.

```rust
// ❌ per-element bounds check (dst[i] / src[i] checked every iteration)
for i in 0..n { dst[i] = f(src[i]); }

// ✅ length checked once at `[..n]`, zero checks inside the loop
for (d, s) in dst[..n].iter_mut().zip(&src[..n]) { *d = f(*s); }

// single-array walk — use iter_mut(), not dst[i]
for d in dst[..n].iter_mut() { *d = g(); }
```

Reach for `iter()` / `iter_mut()` / `zip` / `enumerate` / `chunks` /
`windows` **before** falling back to `slice[i]`. Only when the access
pattern cannot be a forward iterator (see §5) does naive indexing remain.

This applies to **contiguous** walks. A *strided* one belongs in
`(a..b).map(|i| *p.get_unchecked(i * stride))`, not
`.iter().step_by(stride)` — the adapter chain re-derives the position each
step instead of adding a constant. The distinction is not cosmetic: on the
same function in the same run, the index form carries a strided walk while
the `step_by` chain leaves it where it started. Spelling a strided walk as an
iterator adapter looks like this rule and delivers none of it.

---

### 3.5 Two regions of one array — split it, don't subscript it twice

When the function reaches the same array at two positions that provably
never coincide — a base and a fixed offset from it, two halves, a head and
a tail — hand the compiler two slices instead of one:

```rust
// ✅ two regions it knows cannot overlap
let (lo, hi) = buf.split_at_mut(n);
lo[i] = hi[i];

// ❌ one slice, two subscripts: the checks are gone, the aliasing is not
*buf.get_unchecked_mut(i) = *buf.get_unchecked(n + i);
```

The two forms pay the same in checks. They differ in what the compiler is
told. `split_at_mut` yields regions that provably cannot alias, so a write
through one does not force a reload through the other. `get_unchecked`
deletes the check and says nothing about aliasing — the reload stays. Once
such a function is inlined into a caller's loop, that reload is the cost
that decides the rewrite, and it is the larger of the two by far.

This is the general shape of the rule: the win comes from restoring what
the compiler may assume, not from the checks removed. When a rewrite can
buy only one of the two, buy the aliasing.

## 4. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The buffer extent can be established.** Identify the allocation and
  a justified element count from the enclosing function and dependency
  context; a length parameter or field alone is not evidence of capacity.
- [ ] **All accesses remain within this buffer extent.** Establish bounds
  for every index, range, widened access, and boundary path, including any
  adjacent-element access such as `i - 1`.
- [ ] **No conflicting access occurs while the mutable slice is live.**
  Account for other views, raw pointers, callbacks, and callees; all accesses
  during the borrow must respect its exclusivity. Use one mutable view or
  proven-disjoint reborrows rather than overlapping mutable slices.
- [ ] **The introduced slice API is valid.** The pointer is non-null and
  aligned even for an empty slice; the entire view lies in one live
  allocation and contains initialized, valid `T` values. The byte extent
  is at most `isize::MAX`, address arithmetic does not wrap, and storage
  stays valid without reallocation throughout the borrow. Shared views
  also obey their API's mutation restrictions. Preserve original empty or
  null-pointer paths without inventing an early return.
- [ ] **The traversal and dependencies are preserved.** Keep the original
  iteration bounds, read/write order, cursor results, and stopping behavior.
  A sentinel-derived view requires a justified terminator within the valid
  allocation and unchanged data during the view's lifetime.
- [ ] **Calls and boundaries remain compatible.** Audit callees that may
  access the buffer, all callers affected by a signature change, and the
  original public/FFI contract. Keep each unsafe block minimal and document
  its established invariants in a `// SAFETY:` comment.

For the paper's `zrsh` example, establish bounds for both `i` and `i - 1`
from the enclosing function and the `zahl` definition. Recovering a valid
view alone does not justify parallelizing the adjacent-limb updates. A
combined block-wise rewrite must additionally establish the scalar-family
conditions: an equivalent block formulation, input reads before overlapping
output writes, and preservation of boundary semantics.

---

## 5. When to abstain

Skip the site (leave it unchanged) if any of these hold:

- No L1–L4 length source can be established.
- **The view would serve a single access.** One read does not repay
  `from_raw_parts` + a length computation.
  Not a reason to skip: the access being outside a loop. What the rewrite
  removes is the aliasing, and that is paid per execution, not per
  iteration — count accesses to the same memory, not the loops around them.
- P2's step `<n>` is a runtime value with no provable upper bound
  (e.g. `p.offset(rand() as isize)`).
- The loop body calls a function that may mutate the field carrying
  the length.
- The length comes from L2 (a struct field) but that field is written
  inside the rewrite region.
- The function is `pub extern "C"`, changing the signature would break
  the ABI, and no local length source is available for body reborrow.
- **Hot inner loop — prefer the iterator form, let W2 judge (do NOT
  pre-abstain wholesale):**
  For a raw-pointer cursor inside a **tight, high-iteration inner loop**,
  a *naive index* rewrite (`slice[i]`) can add a per-iteration bounds
  check that comes out net slower — **but the iterator form (§3.4)
  carries no per-element check.** So rewrite it with an iterator
  (`iter_mut`/`zip`/…) and let **W2 be the judge**: W2 accepts a de-unsafe
  rewrite that does **not** regress and rejects one that does, so you do
  not need to predict the outcome. Do **not** blanket-abstain on hot loops
  just because they are hot.

  Abstain on the in-loop cursor **only** when the access pattern **cannot
  be a forward iterator**, so only naive `slice[i]` (with its per-element
  check) would remain:
    * **data-dependent / backward / overlapping reads** — LZ77-family
      match-copy (LZ4 / deflate / zstd / snappy) reads from `out - offset`,
      a runtime, possibly-overlapping earlier position, while writing
      `out`. This is not a forward `zip`; it must stay a forward
      per-element pointer loop (see §5.1). Slice-ifying only adds checks
      — abstain.
    * **fixed-size-array-field hash-chain walks** whose index is a
      data-dependent `u16`/`u32` chain (not a monotone `0..n`) — no clean
      iterator, so naive indexing adds a check per hop. Abstain (or leave
      raw and let W2 judge if genuinely unsure).
    * **mask-produced indices** (`pos & ring_mask`, often backwards, reading
      `buf[p]` / `buf[p + 1]`) — the wrap makes the walk non-monotone, so the
      optimiser cannot prove `p + 1 < len` and every probe keeps both checks.
      Keep the raw reads; if a hoist is also available, do it ALONE.

Abstaining per-site is fine — a partial rewrite is safer than a wrong
rewrite.

---

## 5.1 `copy_within` / `ptr::copy` warning

**Do NOT** use `slice::copy_within(src..end, dst)` or `ptr::copy` (both
call `memmove`) as a drop-in replacement for a **forward bytewise**
copy loop of the form:

```c
u = 0; while (u < ml) { dst[u] = src[u]; u += 1; }
```

when `src` and `dst` may **overlap forward** with offset `< ml`.

**Why it breaks:** the original forward loop with e.g. `ml = 4,
src = dst - 1` propagates the single byte at `src` into `(X, X, X, X)`
(each iteration reads the byte the previous iteration just wrote).
`memmove` semantically first snapshots `src..src+ml` then writes,
producing `(X, garbage, garbage, garbage)`. Any LZ77-family match
copy — LZ4, deflate, zstd, snappy — **relies on** the propagate-
forward semantics for short-offset RLE. Silent W1-pass on non-RLE
inputs hides the latent correctness bug — do not ship this rewrite.

**Safe alternatives:**
- Keep the explicit forward loop (`ptr.add(i)` cursor). LLVM already
  vectorizes it when safe.
- Only use `ptr::copy_nonoverlapping` after **proving** `dst..dst+ml`
  and `src..src+ml` do not overlap, in addition to proving both extents valid.
  A one-sided wrapping address difference is insufficient when either
  address ordering is possible.

---

## 6. Post-rewrite self-check

- [ ] Every new `unsafe { }` block is minimal and carries a
      `// SAFETY:` comment addressing length, non-null, and
      non-aliasing.
- [ ] Remove `unsafe` from a signature only if no caller-side safety
      obligation remains and all affected callers and ABI constraints allow
      it; wrapping an operation in `unsafe { ... }` does not remove its contract.
- [ ] `cargo check` passes.
- [ ] **No `slice[i]` inside a hot loop** — convert to an iterator (§3.4),
      split the array (§3.5), or revert that site to the raw pointer. A
      subscript into a `split_at_mut` region is not this violation. Do not
      ship a bare `slice[i]` and let W2 judge.
- [ ] **Every slice view serves more than one access** (§5).
