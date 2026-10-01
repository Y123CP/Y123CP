# Rule C6: Loop-Invariant Dispatch Hoisting

## 1. What you're fixing

A hot loop **re-selects its per-element behavior every iteration** on
state that is **invariant across the whole loop** — a format tag, a color
type, a bit depth, a mode flag. The selection takes two forms:

**Inline dispatch** — the branch is in the loop body:

```rust
while i != n {
    match mode.colortype {            // colortype is fixed for the whole loop
        LCT_RGBA => { /* ... */ }
        LCT_RGB  => { /* ... */ }
        _        => { /* ... */ }
    }
    i += 1;
}
```

**Delegated dispatch** — the loop calls a callee that dispatches internally:

```rust
while i != n {
    get_pixel(&mut r, &mut g, &mut b, &mut a, in_ptr, i, mode);  // mode invariant
    i += 1;
}
// get_pixel(...): if (*mode).colortype == LCT_RGBA { ... } else if ... { ... }
```

Either way, a multi-way branch (or an indirect call whose target is fixed)
runs **once per element** even though its outcome is constant. This
blocks the compiler from specializing and vectorizing the loop body. The
faithfully translated C dispatches per element too.

**Direction**: perform the dispatch **once before the loop** and run a
specialized loop body for the selected case. The specialized body is
typically branch-free — a straight load/store, a copy, or a shuffle —
which lets LLVM further optimize or vectorize it. Keep a general
fallback path for cases without a specialization.

**Not in scope**: a branch whose condition genuinely varies per element
(depends on the element value or the induction variable); a dispatch that
is already hoisted.

---

## 2. Rewrite templates

### 2.1 Inline dispatch — unswitch the loop

```rust
// BEFORE
while i != n {
    match cfg.kind {  A => body_a(i),  B => body_b(i),  _ => body_g(i) }
    i += 1;
}

// AFTER — decide once, run a specialized loop per case.
match cfg.kind {
    A => { let mut i = 0; while i != n { body_a(i); i += 1; } }
    B => { let mut i = 0; while i != n { body_b(i); i += 1; } }
    _ => { let mut i = 0; while i != n { body_g(i); i += 1; } }
}
```

### 2.2 Delegated dispatch — hoist the callee's dispatch into the caller

```rust
// BEFORE
while i != n {
    get_pixel(&mut r, &mut g, &mut b, &mut a, in_ptr, i, mode);
    put(out, i, r, g, b, a);
    i += 1;
}

// AFTER — for the common configuration, inline the callee's selected
// branch so the per-iteration dispatch disappears. Fall back to the
// original call for other configurations.
if (*mode).colortype == LCT_RGBA && (*mode).bitdepth == 8 {
    while i != n {                                   // tight, branch-free body
        let base = i.wrapping_mul(4) as isize;
        put(out, i, *in_ptr.offset(base), *in_ptr.offset(base + 1),
                    *in_ptr.offset(base + 2), *in_ptr.offset(base + 3));
        i += 1;
    }
} else {
    while i != n {                                   // unchanged general path
        get_pixel(&mut r, &mut g, &mut b, &mut a, in_ptr, i, mode);
        put(out, i, r, g, b, a);
        i += 1;
    }
}
```

Read the callee's body for the selected configuration and reproduce
**exactly** the reads/writes it performs for that case (here: four
consecutive bytes for RGBA8). The specialized path must be
bit-identical to calling the callee.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The dispatched value is invariant for the whole loop.** Account
  for direct assignments, aliases, callbacks, and transitive callee writes;
  absence of a local assignment alone is insufficient.
- [ ] **Hoisting the dispatch preserves its evaluation behavior.** Moving
  the predicate/callee selection must not introduce observable effects,
  errors, or invalid reads on an originally empty or skipped loop.
- [ ] **Every original case remains covered.** Keep the original general
  path for configurations without a proven specialization.
- [ ] **Each specialization is equivalent to the selected callee path.**
  Preserve reads, writes, bounds, return values, error handling, and effects
  such as color-key or alpha processing, in their original required order.

---

## 4. When to abstain

- The branch condition depends on the element value or the loop index —
  it is not loop-invariant, so it cannot be hoisted.
- The callee's per-case behavior is not clear enough to reproduce
  faithfully (complex state, error paths) — specialize only the cases you
  can prove, or abstain.
- The loop body is dominated by other work, so the dispatch is not the
  bottleneck.
- There are many configurations and no dominant one — duplicating the
  loop per case bloats code without a clear win; specialize only the
  hot configuration(s).

---

## 5. Post-rewrite self-check

- [ ] The dispatch now runs once before the loop, not per iteration.
- [ ] Every original case is still handled — specialized branches plus an
      unchanged general fallback.
- [ ] Each specialized body reproduces the original per-element
      operations exactly (same reads/writes/order) for that case.
- [ ] The scrutinee is provably invariant across the loop.
- [ ] `cargo check` passes and the output digest is unchanged on every
      input.
