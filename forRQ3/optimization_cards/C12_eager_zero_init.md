# Rule C12: Oversized Eager Zero-Initialization

## 1. What you're fixing

A large fixed-size local that is **zeroed in full at declaration**, while the
code only ever reads a **prefix whose length is a runtime value**.

```rust
// c2rust output — the array is declared at its maximum capacity …
let mut buf: [f64; CAP] = [0.; CAP];      // … and CAP elements are zeroed
fill_prefix(&mut buf, n);                 // … but only n are written
use_prefix(&buf[..n]);                    // … and only n are read
```

The C original declared the same local **uninitialized** (`double buf[CAP];`)
and paid nothing. The translation cannot express "uninitialized", so it emits a
full zero literal, and that literal survives to the binary as a `memset` the C
program never runs.

**LLVM cannot remove it.** The overwritten extent is `n`, a runtime value, so
the optimiser cannot prove the whole buffer is written before it is read and
must keep the store. This is the reason the cost is still there at `-O3` with
fat LTO: it is not a missed optimisation, it is a semantic difference
introduced by the translation.

The hit carries `zeroed_bytes` — the constant size of the `memset` as it
appears in the optimized IR. The waste is that size **minus** the extent the
program actually touches, so a 10 KB buffer whose typical prefix is a few
hundred bytes is where the payoff lives.

### What this is not

- **Not a small struct.** A handful of fields lowers to a few stores, not a
  `memset` call, and there is nothing to remove.
- **Not a buffer that is read in full.** A lookup table, a histogram, a hash
  array — anything whose whole extent is live — must keep every zero. The
  size alone does not tell you which one you are looking at; §4 does.
- **Not a non-constant length.** `vec![x; n]` already allocates the extent the
  program needs. That is the shape this rule rewrites *toward*.

### Where it sits among the other rules

This rule fires on functions that other rules also want, and the order
matters:

- **III② (manual heap → RAII) wins.** If the buffer becomes a `Vec`, it
  allocates the extent the program needs and there is no eager zeroing left to
  shrink. Do III② and drop this rule for that buffer — do not do both.
- **III④ (raw-pointer cursor → slice) is a partner, not a rival.** §2.1 hands
  the rest of the function a `&mut [T]`, which is exactly the view III④
  builds. Do this rewrite FIRST and let III④ walk the slice it leaves behind;
  building two separate views over one buffer is how the aliasing gets wrong.
- **C1 / C3 are unaffected.** They act on the accesses, this one acts on the
  declaration. Nothing needs sequencing between them.

---

## 2. The two forms

### 2.0 Shrink the memset — do NOT replace it with a loop

The goal is a memset whose size is the RUNTIME extent instead of the constant
capacity. It is not to remove the memset: `memset` is one of the most heavily
optimized routines on the machine, and a hand-written per-element loop is not
in the same class.

**Measured, and this is the whole reason this section exists.** A rewrite that
replaced `[0.; 1024]` with a `for slot in &mut store[..n] { slot.write(0.0) }`
loop was **6.2% and 10.0% FASTER** on two operations whose runtime extent was a
few dozen elements — and **4.5% SLOWER** on a third whose extent was ~1000, i.e.
nearly the whole array. Same rewrite, same binary. On the third operation it had
turned one vectorized bulk store into a thousand scalar ones; the instruction
count actually FELL 4.3% while wall time ROSE 4.5%.

Use `write_bytes`, which lowers to `llvm.memset` with a runtime size:

```rust
use core::mem::MaybeUninit;

// ❌ CAP elements zeroed, n read — a memset of CONSTANT size
let mut buf: [f64; CAP] = [0.; CAP];

// ✅ n elements zeroed, n read — a memset of RUNTIME size
let mut store = [MaybeUninit::<f64>::uninit(); CAP];
let buf: &mut [f64] = unsafe {
    // SAFETY: `write_bytes` initializes every element in `..n` (all-zero bits
    // is a valid `f64`, and `0.0` is what the original literal produced), and
    // nothing below indexes past `n` — the same bound the original code used.
    let p = store.as_mut_ptr() as *mut f64;
    core::ptr::write_bytes(p, 0, n);
    core::slice::from_raw_parts_mut(p, n)
};
```

This keeps the bulk store and only shrinks its length, so the worst case
(`n` ≈ `CAP`) is a wash rather than a regression, and the best case (`n` far
below `CAP`) collects the whole difference.

`write_bytes(p, 0, n)` counts ELEMENTS, not bytes. It is valid only where the
all-zero bit pattern is a valid value of `T` — true for every integer and IEEE
float the translation puts in these buffers, false for `bool` arrays holding
anything but `false`, references, and `NonZero*`. If `T` is not such a scalar,
abstain.

### 2.1 `prefix` — the used extent is known at the declaration

Apply the §2.0 form when the length that bounds every later read is already in
scope at the declaration.

`[MaybeUninit::<T>::uninit(); CAP]` requires `T: Copy`, which every scalar the
translation puts in such a buffer satisfies. It compiles to nothing.

**Every later use must go through `buf`, not `store`.** If any site still
indexes the backing array, the rewrite is not finished.

### 2.2 `deferred` — the buffer is filled before it is read

When the value is written by a call that runs before any read, the zeroing is
dead and no prefix loop is needed.

```rust
// ❌ zeroed, then immediately overwritten
let mut buf: [u8; CAP] = [0; CAP];
init_from(&mut buf, src);          // writes buf[..n]

// ✅ no zeroing at all
let mut store = [MaybeUninit::<u8>::uninit(); CAP];
let buf = init_prefix_from(&mut store, src);   // returns &mut [u8] of len n
```

This form requires changing the filler to write through `MaybeUninit` and
return the initialized slice — and the filler must keep writing in bulk, for
the reason in §2.0. **If you cannot change the filler in the same
edit, use §2.1 instead** — a partially-initialized buffer handed to a function
expecting `&mut [T]` is undefined behaviour, not an optimization.

### 2.3 An early return is part of the contract

Translated code often bails out before the buffer is filled:

```rust
if len > CAP || other_bad_condition { return SENTINEL; }
```

Confirm the caller cannot read the buffer on that path. It usually cannot —
the early return propagates — but if any path reaches a read with the prefix
unwritten, this rule does not apply. That is not a bound you may assume.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **Every observed element is initialized before its first read.**
  Trace all reads, including in callees, early exits, and error paths. A
  previously zeroed value that can be observed cannot be left unwritten.
- [ ] **The initialized prefix covers every exposed view.** Establish that
  the write bound equals or covers the read bound. Slice `MaybeUninit<T>`
  to the initialized range before constructing a view of `T`; never create
  a `T` reference to uninitialized elements, including `u8` or `f64`.
- [ ] **Each initialized value is valid for its type.** `assume_init` and
  pointer casts require a complete valid `T`, not merely allocated storage.
- [ ] **Capacity and boundary behavior remain unchanged.** Preserve declared
  capacity, logical length, terminators, and the original handling of empty
  input, full capacity, invalid sizes, and early returns.
- [ ] **Ownership and destruction remain correct.** Do not drop uninitialized
  elements or omit required drops when changing the initialization strategy.

---

## 4. When to abstain

Skip the site (leave it unchanged) if any of these hold:

- **You cannot name the prefix bound.** If no expression in the function
  delimits the live extent, there is no rewrite — only a guess.
- **The whole buffer is read.** Lookup tables, bitmaps indexed by arbitrary
  input, anything a later loop walks end-to-end.
- **A read can precede the write.** Including on an early-return or error path
  (§2.3).
- **The buffer escapes.** Passed to an `extern` function, stored in a struct
  that outlives the frame, or handed to anything opaque — its whole extent is
  then observable.
- **`T`'s all-zero bit pattern is not a valid value.** `write_bytes` is the
  only sound bulk form here (§2.0); if you cannot use it, there is no rewrite.
- **The typical prefix is most of the capacity AND you cannot use the bulk
  form.** With §2.0 a near-full prefix merely breaks even, which is acceptable.
  With a per-element loop it is a measured **+4.5% regression** — so if
  anything forces you toward a loop, abstain instead.
- **The runtime extent is not available at the declaration.** Computing it
  early enough may reorder side effects; do not move work above the
  declaration to manufacture a bound.
- **The zeroed size is a few hundred bytes.** Below a few cache lines the
  `memset` is cheap and the `MaybeUninit` plumbing is not worth it.

---

## 5. Post-rewrite self-check

- [ ] **The initialization is still ONE bulk store, not a loop.** A
      `for`/`iter_mut` that zeroes elements one at a time is the failure mode
      §2.0 documents — measured at +4.5% on a near-full prefix. `write_bytes`
      / `fill` are the accepted forms.
- [ ] **Every element handed out as `&[T]` / `&mut [T]` was written first.**
      Point at the line that writes it.
- [ ] **The prefix bound and the read bound are the same expression**, not two
      quantities that merely happen to agree.
- [ ] **No `assume_init` on the whole array.** Only the sliced prefix is cast.
- [ ] **The declared capacity is unchanged.** Only the initialization shrank.
- [ ] **Every read site goes through the new slice**, not the backing storage.
- [ ] **Early-return paths cannot reach a read**, and you have checked each one
      rather than assuming the pattern.
- [ ] **No zero-filled array literal of the original capacity remains.**
      `[0; CAP]` / `[0.; CAP]` at the declaration is precisely the construct
      that becomes the `memset`; if one is still there, the rewrite changed
      the spelling and not the cost. (The IR is checked by the pipeline, not
      by you — this line is the source-level proxy you CAN check.)
