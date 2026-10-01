# Rule II_vec: Vectorization Loss

## 1. What you're fixing

LLVM's loop vectorizer attempted to SIMD-parallelize a hot loop and
gave up because the loop's **shape** blocks vectorization. c2rust
translation frequently produces such shapes: state machines that
simulate C's `goto`, loop variables declared outside the loop,
`memcpy` / `memmove` / math libcalls inside the loop body.

The driver hands you the failure reasons from the vectorizer's remarks
(`CantComputeNumberOfIterations`, `NonReductionValueUsedOutsideLoop`,
`LoopContainsSwitch`, `NoCFGForSelect`, `CantVectorizeLibcall`, …).
The reasons cluster into two families:

- **Control-flow distortion** (first four reasons): the loop's control
  flow is not a clean induction-variable loop.
- **Libcall barrier** (`CantVectorizeLibcall`): the loop body calls an
  opaque function.

**Direction**: canonicalize the loop shape after establishing the semantic
preconditions in §3. The rewritten loop may still regress performance;
the W2 gate evaluates that separately from rewrite validity.

---

## 2. Rewrite templates

### 2.1 `current_block` state machine (c2rust `goto` emulation) → structured control flow

c2rust emits `let mut current_block: u64 = <hash>; loop { match current_block { ... } }`
to simulate C's `goto`. The match on a `u64` is a switch that the
vectorizer refuses (`LoopContainsSwitch` /
`CantComputeNumberOfIterations`).

```rust
// BEFORE
let mut current_block: u64 = 5;
loop {
    match current_block {
        5  => { /* work_A */ current_block = 12; }
        12 => { /* work_B */ break; }
        _  => { break; }
    }
}

// AFTER — linear chain with no back-edges
/* work_A */
/* work_B */
```

If the state graph has back-edges, use `'label: loop { ... break 'label; }`:

```rust
'outer: loop {
    /* state 5 work */
    if <exit condition> { break 'outer; }
    /* state 12 work — falls through in the loop body */
}
```

**Count the distinct `match` arms first**: `> 3 arms` or any back-edge
means substantial redesign — LLVM's own optimizer usually folds these. A
state machine with hundreds of source-level transitions routinely survives
into optimized IR as a handful, which is why rewriting the source form buys
so little: you are restructuring something the backend already collapsed.
Prefer to abstain unless the state graph is small and structural.

### 2.2 Loop variable declared outside → `for` / iterator

```rust
// BEFORE
let mut i = 0;                          // declared OUTSIDE the loop
let mut sum = 0;
loop {
    if i >= n { break; }
    sum += arr[i];
    i += 1;
}

// AFTER
for i in 0..n {                         // i is loop-scoped
    sum += arr[i];
}
```

For raw-pointer sentinel loops (`while p < end_p { ... p = p.add(1); }`),
convert to slice iteration:

```rust
// BEFORE
let mut p = start_ptr;
while p < end_ptr {
    process(unsafe { *p });
    p = unsafe { p.offset(1) };
}

// AFTER
// SAFETY: establish independently that both pointers delimit one allocation,
// end_ptr >= start_ptr, and start_ptr is non-null/aligned with len initialized
// T's that remain readable and unmodified for the slice's lifetime.
// A debug assertion checks ordering only; it does not establish API validity.
debug_assert!(end_ptr >= start_ptr);
let len = unsafe { end_ptr.offset_from(start_ptr) } as usize;
let slice = unsafe { core::slice::from_raw_parts(start_ptr, len) };
for x in slice {
    process(*x);
}
```

### 2.3 Unbalanced `if / else` → branchless arithmetic

```rust
// BEFORE
for i in 0..n {
    if arr[i] > threshold { result[i] = 1; }
    else                  { result[i] = 0; }
}

// AFTER
for i in 0..n {
    result[i] = (arr[i] > threshold) as u32;   // bool → 0/1, no branch
}
```

Balanced `select`-shaped branches often vectorize on their own;
side-effecting or unbalanced arms need explicit arithmetization.

### 2.4 Libcall in loop — replace with typed slice API or intrinsic

For `memcpy` / `memmove` / `memset` / `memcmp` / `strlen` inside a hot
loop, apply **III③** (`III3_mem_ops_to_slice.md`) first — the resulting
slice-API call lowers to LLVM intrinsics the vectorizer knows about.

For math libcalls, prefer the Rust method form:

```rust
// BEFORE
for i in 0..n { result[i] = unsafe { libc::sqrt(arr[i]) }; }

// AFTER
for i in 0..n { result[i] = arr[i].sqrt(); }   // @llvm.sqrt intrinsic
```

For a small internal helper that failed to inline, apply **II_inl** first
to inline it, then re-check whether II_vec still fires.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The rewritten control flow is equivalent.** Preserve iteration
  count, required order, exits, and induction variables used after the loop.
  Resolve every state-machine edge before restructuring it.
- [ ] **Dependencies permit the proposed formulation.** Establish that
  reordered operations are independent, or derive an equivalent recurrence.
  Unit stride and a vectorizer remark alone do not establish independence.
- [ ] **Side effects and failures are preserved.** Do not speculate calls,
  reads, writes, or panics from an untaken branch; preserve volatile/atomic
  behavior and alias-visible effects. Do not remove reachable bounds checks.
- [ ] **Arithmetic and replacement calls are equivalent.** Preserve integer
  overflow and floating-point rounding/NaN behavior. Replacing a libcall
  also requires preserving any observable error or environment effects.
- [ ] **Introduced Rust APIs satisfy their validity requirements.** For a
  slice rewrite establish extent, initialized values, non-null/aligned
  pointers, lifetime, and access compatibility as in III4. Pointer distance
  requires a justified common allocation and ordering; a loop comparison
  or debug assertion alone is not sufficient.

---

## 4. When to abstain

Skip the rewrite if any of these hold:

- The `current_block` state machine has > 3 arms or any back-edge —
  substantial redesign, prefer to abstain.
- The loop body has a true cross-iteration data dependency (e.g. a
  stateful hash accumulator, non-associative floating-point sum in
  strict order).
- The libcall is a **user-defined callback** through a fn pointer —
  see II_inl / III① instead.
- Unrolling the state machine would expand the fn body by more than
  ~3× (may push it past LLVM's inline threshold).
- The loop already vectorized successfully in the IR remarks (any
  `loop-vectorize:passed` for the same loop) — the missed one is
  probably minor.

---

## 5. Post-rewrite self-check

- [ ] Iteration count, side-effect order, and panic behavior all
      preserved.
- [ ] Every new `unsafe { }` block is minimal and carries a
      `// SAFETY:` comment.
- [ ] Golden test cases still produce identical output.
- [ ] `cargo check` passes; W2 gate is prepared for possible
      regression (LLVM's cost model may have been right — the driver
      rolls back if wall clock regresses).
