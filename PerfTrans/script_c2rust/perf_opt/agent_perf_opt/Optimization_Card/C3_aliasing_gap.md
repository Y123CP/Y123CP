# Rule C3: Obscured Memory Access Properties

Implementation template: Redundant Memory Access (Aliasing Gap).

The three fields below reproduce the matched `zrsh` example in the paper
figure. For another region, instantiate the observed pattern from that
region and establish applicability using the implementation-specific
pattern and conditions below. The figure's ellipses denote omitted detail.

## Observed Pattern

Raw-pointer buffer traversal through `(*a).chars.offset(i)`.

## Optimization Direction

Recover valid &[T]/&mut [T] views to expose access properties to LLVM, ...

## Rewrite Preconditions

- [ ] The buffer extent can be established.
- [ ] All accesses remain within this buffer extent.
- [ ] No conflicting access occurs while the mutable slice is live.

...

---

## 1. Implementation-specific pattern

c2rust-emitted IR carries **no TBAA metadata** on `load` / `store` /
`getelementptr` through raw pointers (`*mut T` / `*const T`). Without
TBAA, LLVM must assume any store may alias any load — a conservative
default that blocks **GVN (load elimination)** and **LICM (loop-invariant
load hoisting)**. Inside a loop with `*mut T` parameters, the same
address-chain load is repeated every iteration because a store elsewhere
in the loop *might* invalidate it.

The compiler tells you via remarks like `gvn: load of type X not
eliminated` or `licm: failed to move load with loop-invariant address
because the loop may invalidate its value`.

**Template strategy**: recover LLVM's aliasing knowledge in one of two ways:

- **S1 — change the signature**: `*mut T` → `&mut T` / `&mut [T]`. The
  Rust reference carries `noalias` by default; LICM/GVN recover across
  every call site.
- **S2 — hoist the loop-invariant read into a local**: leave the raw
  pointer, but manually cache the invariant load in a local variable
  before the loop; write back after.

S1 is the higher-payoff choice; S2 is the fallback when the signature
must stay raw (extern "C" ABI, wide public API).

---

## 2. Rewrite templates

### 2.1 S1 — signature lift (`*mut T` → `&mut [T]`)

```rust
// BEFORE
unsafe fn f(a: *mut zahl, b: *mut zahl, out: *mut u64, len: usize) {
    let a_chars = (*a).chars;
    let b_chars = (*b).chars;
    for i in (0..len).rev() {
        // (*a).chars, (*b).chars re-loaded every iteration because
        // *out could alias — LLVM couldn't prove otherwise
        *out.offset(i as isize) =
            *a_chars.offset(i as isize) - *b_chars.offset(i as isize);
    }
}

// AFTER
fn f(a: &zahl, b: &zahl, out: &mut [u64]) {
    // SAFETY: caller guarantees `a.chars` / `b.chars` point to at least
    // out.len() u64's and none of the three regions overlap
    let a_chars: &[u64] = unsafe {
        core::slice::from_raw_parts(a.chars as *const u64, out.len())
    };
    let b_chars: &[u64] = unsafe {
        core::slice::from_raw_parts(b.chars as *const u64, out.len())
    };
    for i in (0..out.len()).rev() {
        out[i] = a_chars[i] - b_chars[i];   // LICM hoists a_chars/b_chars
    }
}
```

The resulting `out[i]` reintroduces a C1 bounds check. If the loop is
hot, stack C1 on top: `unsafe { *out.get_unchecked_mut(i) = ... }`.

### 2.2 S2 — local variable hoisting

```rust
// BEFORE
unsafe fn f(p: *mut Ctx) {
    for _ in 0..(*p).count {
        if (*p).threshold > 0 { ... }
        (*p).accumulator += 1;              // store — LLVM's alias fear
    }
}

// AFTER
unsafe fn f(p: *mut Ctx) {
    // SAFETY: only .accumulator is written in the loop; .threshold and
    // .count are not touched (grepped the loop body)
    let threshold = (*p).threshold;
    let count = (*p).count;
    let mut acc = (*p).accumulator;         // hoist accumulator too
    for _ in 0..count {
        if threshold > 0 { ... }
        acc += 1;
    }
    (*p).accumulator = acc;                 // single write-back
}
```

The signature stays `*mut Ctx`; the fix is scoped to the loop body.

### 2.2.1 The invariant is not always a field

An invariant wears two shapes. The one above is a field read through a raw
pointer. The other is a **call** whose result cannot change across iterations
— the accessor functions that a C header's macros expand into. A translator
emits them verbatim, once per iteration, everywhere the original C hid them
behind a macro:

```rust
// BEFORE — the accessor is re-called for every element
for i in 0..len {
    *out.add(i) = *(*__ctype_tolower_loc()).offset(*src.add(i) as isize) as c_char;
}

// AFTER — called once; the loop keeps only the table read
// SAFETY: the returned table pointer is fixed for this thread and locale,
// neither of which the loop changes.
let table = unsafe { *__ctype_tolower_loc() };
for i in 0..len {
    *out.add(i) = *table.offset(*src.add(i) as isize) as c_char;
}
```

The whole locale/thread accessor family has this shape (`__ctype_b_loc`,
`__ctype_tolower_loc`, `__ctype_toupper_loc`, `__errno_location`). Each call
is a TLS lookup, and the optimiser will not hoist it on its own: it cannot
prove the callee is side-effect free.

**A call in the loop body is not evidence against S2.** §3.2's obligation
reads the other way round as well: a call that might *write* the hoisted
location blocks the hoist — but a call that merely *returns the same pointer
every time* is itself the hoist target. Concluding "no repeated invariant
load here" after looking only for field reads misses this shape completely.

### 2.3 In-body views are a third shape

Building a slice inside the body while the signature stays raw is **not S1**:
`noalias` never reaches the call sites, so the caller-side LICM/GVN that S1
buys does not happen. It is not S2 either — no hoist is performed.

It is a separate decision with its own cost, and it must be judged separately
from S2. S2 pays off when the loop reloads the same invariant; the view pays
off when it lets the optimiser see a length relation it could not see before.
Neither implies the other, and doing one is never a reason to skip the other.

**When a view pays.** A loop advancing one input and one output. The length
relation between the two is simple enough for the optimiser to drop the bounds
checks and fuse the walk, so the view hands it information it did not have.

**When it does not.** A loop advancing four or more arrays at once. The nested
tuple state of a long `zip` chain becomes the burden itself, and the optimiser
gives up on the very relation the view was meant to expose. The cost then
exceeds anything the hoist would have earned, and the same loop left on raw
pointers with S2 applied is the faster of the two.

**Never build a partial view.** Converting one side while the other stays a
raw pointer driven by a hand-written counter — or slicing and then indexing it
with `get_unchecked` — pays the view's cost and forfeits its benefit at the
same time: the optimiser still faces the raw pointer's aliasing uncertainty,
and now carries an extra induction variable as well. Both mixed forms measure
worse than either pure form. Convert everything the loop walks, or nothing.

**S2 is never "subsumed".** Whether the hoist earns anything is settled by one
observation — does the loop reload the same invariant — not by a prediction
about what some other rewrite's slice conversion will let the optimiser do.
Nor by the shape that invariant happens to wear: "no repeated invariant
field / address-chain load" is not a finding while the loop still re-calls an
accessor on every iteration (§2.2.1). Look for both shapes before recording a
skip, and say which one you looked for.

---

## 3. Implementation-specific preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

### 3.1 S1 and local memory-view recovery

- [ ] **Buffer extent evidence.** Identify the allocation and
  a justified element count from the enclosing function and dependency
  context; a length parameter or field alone is not evidence of capacity.
- [ ] **Access bounds evidence.** Establish bounds
  for every index, range, widened access, and boundary path, including any
  adjacent-element access such as `i - 1`.
- [ ] **Access compatibility evidence.**
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
- [ ] **All affected call sites satisfy the new contract.** Trace pointer
  origins and ownership at each call site; `as_mut_ptr`, `into_raw`, or
  `malloc` alone does not prove that a mutable borrow is valid. Preserve
  public/FFI contracts; use a local view if the signature cannot change.

### 3.2 S2: local hoisting

- [ ] **A hoisted value is invariant throughout its use.** No direct,
  aliased, or transitive callee write changes the location, table pointer,
  or relevant environment (for example, locale) during the region.
- [ ] **Moving the read preserves observable behavior.** Do not newly
  evaluate a read or call on an originally skipped path, or move volatile,
  atomic, fallible, or side-effecting operations without equivalence.
- [ ] **Deferred write-back preserves all observations.** Initialize from
  the original field, preserve updates and arithmetic, and write back on
  every required exit. No intervening access observes a stale value.

Keep introduced unsafe operations minimal and attach a `// SAFETY:` comment
that states the established invariants.

---

## 4. When to abstain

Skip the rewrite if any of these hold:

- **No leverage** (most common wasted attempt): the field/value you
  would hoist is read **only once** — LLVM's CSE already handles the
  single-read case; hoisting adds a local var without any speedup, just
  noise that the W2 gate may falsely regress on.
  **A hoist is worth doing only if the target is read ≥ 2 times**, or is
  read under intervening writes that break CSE.
  Not a reason to skip: the fn having no loop of its own. A short fn is
  inlined into its caller's loop, and the reload it carries is then paid
  per iteration there — count reads of the same field, not the loops
  around them.
- The fn is called from **real C code** (`#[no_mangle] pub extern "C"`
  and part of a published C ABI) — S1 breaks the ABI; fall back to S2.
- Any call site passes overlapping memory regions (aliasing IS the
  intended semantics — rare but possible in low-level bit twiddling).
- Any call site holds `&owner` while calling this fn.
- The loop body contains a fn call that could plausibly mutate the
  hoisted field — be pessimistic.
- The field is written by a signal handler or async interrupt (rare in
  Rust; more of an FFI concern).

Per-site abstain is fine; use S2 for the sites S1 rejects, or leave
them unchanged. **When in doubt, abstain** — the W2 gate will not
reward a "cosmetic" hoist that doesn't help LLVM.

---

## 5. Post-rewrite self-check

- [ ] Every new `unsafe { }` is minimal and carries a `// SAFETY:`
      comment naming the aliasing guarantee.
- [ ] For S1: the signature change compiles at every call site (no
      leftover `.as_mut_ptr()` where the new signature expects a slice).
- [ ] For S2: each hoisted write has exactly one write-back point.
- [ ] `cargo check` passes.
