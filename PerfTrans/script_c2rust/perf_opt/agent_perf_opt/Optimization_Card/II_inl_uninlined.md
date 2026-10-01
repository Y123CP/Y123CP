# Rule II_inl: Uninlined Hot Callee

## 1. What you're fixing

LLVM's inliner declined to inline a callee that appears in this hot
function's body, and a `call` / `invoke` instruction survives in the
fn's optimized IR. Each uninlined call costs 2–5 cycles of call/ret
overhead per hit and blocks cross-function optimization — caller-side
value flow is lost at the call boundary, constant propagation stops,
redundant loads across the boundary cannot be merged.

The driver hands you the miss reasons via
`class_ii_hits.II_inl: { TooCostly: N, NoDefinition: M, NeverInline: K }`.
Different reasons need different rewrites — but the most common
c2rust case (`Option<extern "C" fn>` callback field, giving an indirect
call marked `NoDefinition`) is handled by **III①**
(`III1_callback_monomorph.md`), not here. This card covers the
remaining cases where the callee is a normal Rust function that just
did not inline.

**Direction**: pick one of

- Edit the **callee's** definition to hint or force inlining (§2.1)
- **Shrink** the callee by extracting its cold path (§2.2)
- **Convert recursion to iteration** so the fn becomes inlineable at
  its own callers (§2.3)

---

## 2. Rewrite templates

### 2.1 Add `#[inline]` to the callee

Edit the callee's definition (which may be in a different file from
the hot caller):

```rust
// BEFORE — accumulate is called in the hot loop of hot_fn but not inlined
fn accumulate(acc: u32, e: &Element) -> u32 {
    /* body */
}

// AFTER
#[inline]
fn accumulate(acc: u32, e: &Element) -> u32 {
    /* body */
}
```

Attribute choice:

| Attribute | When to use |
|---|---|
| `#[inline]` | Default — recommends inlining across crate boundaries. |
| `#[inline(always)]` | Force inline against the cost model. Use only when the body is `≤ 10` lines and the caller loop iterates very often. |
| `#[inline(never)]` | Do not use here — this is the opt-out. |

### 2.2 Extract the cold path

If `#[inline]` alone still gets rejected on cost, shrink the callee by
splitting off the cold branches:

```rust
// BEFORE
#[inline]
fn accumulate(acc: u32, e: &Element) -> u32 {
    if e.is_special_case() {
        cold_case_handling(acc, e)              // rare + complex
    } else {
        acc + e.value                           // hot + tiny
    }
}

// AFTER
#[inline]
fn accumulate(acc: u32, e: &Element) -> u32 {
    if e.is_special_case() {
        accumulate_cold(acc, e)
    } else {
        acc + e.value
    }
}

#[cold]
#[inline(never)]
fn accumulate_cold(acc: u32, e: &Element) -> u32 {
    cold_case_handling(acc, e)
}
```

`#[cold]` tells LLVM the branch is unlikely; `#[inline(never)]` keeps
the cold body out of the hot fn.

### 2.3 Convert recursion to iteration

If the remark reason is `NeverInline: recursive`, the callee cannot be
inlined at all. Convert to an explicit-stack loop:

```rust
// BEFORE (recursive)
fn descend(node: &Node, depth: u32) -> u32 {
    if node.is_leaf() { return node.value; }
    node.children.iter().map(|c| descend(c, depth + 1)).sum()
}

// AFTER (iterative)
fn descend(root: &Node) -> u32 {
    let mut stack: Vec<&Node> = vec![root];
    let mut sum = 0;
    while let Some(node) = stack.pop() {
        if node.is_leaf() {
            sum += node.value;
        } else {
            stack.extend(node.children.iter());
        }
    }
    sum
}
```

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **Attribute changes preserve the callable contract.** Keep the
  function body, signature, ABI, arguments, return values, and call order
  unchanged for attribute-only edits. Respect linkage and edit constraints.
- [ ] **Recursion-to-iteration preserves the recurrence.** For §2.3, map
  recursive state, pending work, and return values to loop state explicitly;
  preserve base cases, traversal order, and termination on the original
  supported inputs.
- [ ] **Observable behavior is preserved on every path.** Account for
  callee effects, mutation through aliases, error/panic paths, and cleanup
  timing when changing control flow. Golden tests are additional validation,
  not a substitute for this equivalence argument.

---

## 4. When to abstain

Skip the rewrite if any of these hold:

- **Callee body is large** (> 40 lines): forcing `#[inline]` on a big
  callee bloats `.text`, hurts icache locality, and often nets slower
  wall clock — LLVM's cost model bailed for a reason. Prefer to
  abstain unless callee is ≤ 20 lines.
- **Call site is outside a hot loop**: a single non-loop call costs
  ~2-5 cycles; inlining saves that but the tradeoff (icache pressure)
  usually swamps the gain outside hot loops.
- The uninlined callee lives in `std`, `libc`, or a third-party crate
  you cannot edit.
- The callee already carries `#[inline(never)]` — someone opted out
  intentionally, respect it.
- The callee is a public API where changing the compilation model
  (adding `#[inline]` makes the body cross crate boundaries) would
  affect downstream users you cannot audit.
- The recursion depth is bounded and small (< 10) — the iterative
  rewrite's overhead may exceed the recursive cost.
- The recursion is mutual (two fns call each other) or crosses fn-pointer
  indirection — the explicit-stack rewrite becomes intractable.
- The callee handles a `Option<extern "C" fn>` callback field — apply
  **III①** instead; monomorphization is the right fix, not `#[inline]`.

**When in doubt, abstain** — adding `#[inline]` to a callee LLVM
already decided against usually loses (that's why LLVM bailed).

---

## 5. Post-rewrite self-check

- [ ] Rebuilding the fn's IR: the `call` instructions to the target
      callee(s) should be replaced by inlined bodies OR direct calls.
- [ ] Function public signature unchanged (attribute changes only).
- [ ] Golden tests still pass; recursion-to-iteration case produces
      identical output.
- [ ] W2 gate is prepared for possible regression — inlining a large
      body can pessimize icache; the driver rolls back if wall clock
      regresses.
