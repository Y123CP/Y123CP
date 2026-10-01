# Rule C5: Scalar Element-Wise Processing

Implementation template: Reduction Reassociation.

The three fields below reproduce the matched `zrsh` example in the paper
figure. For another region, instantiate the observed pattern from that
region and establish applicability using the implementation-specific
pattern and conditions below. The figure's ellipses denote omitted detail.

## Observed Pattern

A unit-stride loop advances i by one and accesses adjacent elements at offsets i and i−1.

## Optimization Direction

Process independent regions in parallel, ...

## Rewrite Preconditions

- [ ] The recurrence admits an equivalent block-wise formulation.
- [ ] Required inputs can be read before overlapping outputs are written.
- [ ] Boundary cases preserve the original semantics.

...

---

## 1. Implementation-specific pattern

The hot function contains a **serial reduction recurrence**: a loop with
two (or more) running accumulators where one accumulator is updated
**from another accumulator's running value**. The canonical c2rust shape
is a running-sum-of-a-running-sum:

```rust
while i != n {
    s1 = s1.wrapping_add(*data.offset(i));   // A += input[i]      (reads data)
    s2 = s2.wrapping_add(s1);                 // B += A             (reads A)
    i += 1;
}
```

Because `s2` depends on the value of `s1` produced **this** iteration,
the loop carries a dependency chain of latency ~1 add per element. LLVM
**cannot** vectorize it (there is no independent lane), and the
faithfully translated C is equally serial. This idiom is the core of
checksum/rolling-sum kernels: Adler-32, Fletcher, and prefix-sum passes.

**Template strategy**: reassociate the recurrence into **independent** partial
reductions that LLVM auto-vectorizes (or the CPU runs on parallel ALU
ports), then recombine arithmetically. For the running-sum-of-running-sum
over a block of `n` elements starting from `(s1, s2)`:

```
s1_new = s1 + Σ d[j]
s2_new = s2 + n*s1 + Σ (n-j)*d[j]  =  s2 + n*s1 + n*Σd[j] − Σ j*d[j]
```

The two sums `Σ d[j]` and `Σ j*d[j]` are **independent** reductions —
each vectorizes on its own — and the combine is a handful of scalar ops
once per block.

**Not in scope**: a single accumulator (`sum += x` alone already
auto-vectorizes — that is Vectorization Restoration, not this rule);
non-linear recurrences with no closed form; reductions whose combine
would overflow the accumulator type without widening.

---

## 2. Rewrite template (running-sum-of-running-sum)

```rust
// BEFORE
while i != n {
    s1 = s1.wrapping_add(*d.offset(i) as u32);
    s2 = s2.wrapping_add(s1);
    i += 1;
}

// AFTER — two independent reductions + arithmetic combine.
// Widen the block accumulators (u64) so n*Σd and Σ j*d cannot overflow;
// the caller's block size must keep the closed form exact (see §3).
let mut bsum: u64 = 0;      // Σ d[j]
let mut bwsum: u64 = 0;     // Σ j*d[j]
let mut j: u32 = 0;
while j != n {              // both accumulators are independent → vectorizes
    let v = *d.offset(j as isize) as u64;
    bsum = bsum.wrapping_add(v);
    bwsum = bwsum.wrapping_add((j as u64).wrapping_mul(v));
    j += 1;
}
let s2u = (s2 as u64)
    .wrapping_add((n as u64).wrapping_mul(s1 as u64))
    .wrapping_add((n as u64).wrapping_mul(bsum))
    .wrapping_sub(bwsum);          // = n*s1 + Σ(n-j)*d[j]
s1 = ((s1 as u64).wrapping_add(bsum) % MOD) as u32;   // MOD from the original
s2 = (s2u % MOD) as u32;
```

Keep the surrounding structure identical: same block/chunk size, same
modulus (or none), same final combine. The pointer `d` advances by `n`
after the block, exactly as the byte-serial form advanced it `n` times.

---

## 3. Implementation-specific preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **Block equivalence evidence.**
  Derive equivalence for this template's operation and state, rather than
  inferring independence from unit stride or adjacent element accesses.
  For independent element operations, establish that grouping their scalar
  steps produces the same result and final state.
- [ ] **Read-before-write evidence.**
  Account for dependencies within and across blocks. Preserve values needed
  by later iterations, or establish that the input is never overwritten.
- [ ] **Boundary semantics evidence.** Cover empty input,
  the first and last elements, complete blocks, partial tails, and the
  original stopping condition and final state.
- [ ] **The closed form is exact for the original recurrence.** For the
  running-sum-of-running-sum template, derive the weighted contribution
  `sum((n-j)*d[j]) = n*sum(d[j]) - sum(j*d[j])`, including the incoming
  accumulator state and each block's final state.
- [ ] **Every intermediate has the required arithmetic semantics.** Prove
  widened products, sums, and subtractions fit their types, and preserve
  the original modular/wrapping behavior when recombining. Do not apply
  integer identities to floating-point recurrences without a separate
  equivalence argument.
- [ ] **Reduction boundaries remain valid.** Preserve the original chunk
  boundaries and modulus/carry steps unless equivalence and bounds for a
  different schedule are established. Read only initialized input elements
  and handle the incomplete final block with the original recurrence.

---

## 4. When to abstain

- The accumulators feed control flow inside the loop (e.g. an early exit
  on `s2 > threshold`) — the closed form skips intermediate values.
- The recurrence is non-linear (e.g. `s2 = s2 * s1`) or has no closed
  form over a block.
- The per-element body does substantial other work, so the recurrence is
  not the bottleneck.
- Deriving the closed form or the overflow bound is not clearly correct —
  a wrong reassociation silently corrupts the checksum. Prefer to abstain
  over guessing.

---

## 5. Post-rewrite self-check

- [ ] The two block sums are independent (neither reads the other inside
      the block loop) so LLVM can vectorize them.
- [ ] The combine reproduces the exact original value — verify on a small
      block by hand against the serial form.
- [ ] Block accumulators are wide enough (`u64`) that no intermediate
      overflows; the final reduce matches the original width and modulus.
- [ ] Same chunk boundaries and modulus as the original.
- [ ] `cargo check` passes and the digest is unchanged on every input.
