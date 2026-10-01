# Rule C2: Range-Proven Conversion Simplification

## 1. What you're fixing

The function contains a **float-to-integer cast** — `let i: I = f as I`
where `f: f32`/`f64` and `I` is an integer type. Rust's `as` is a
**total** conversion: it is defined on every operand, including the ones
C leaves undefined (NaN → 0; above `I::MAX` → `I::MAX`; below `I::MIN` →
`I::MIN`, which for unsigned `I` means 0). LLVM realizes that definition
with `@llvm.fptosi.sat.*` / `@llvm.fptoui.sat.*`.

C's `(int)f` is a single truncation instruction, because C declares the
same operands undefined and therefore emits nothing for them.

**Two costs, and the second is usually the larger one:**

1. **Per site**: the saturating form expands to a clamp pair, a NaN test
   and a select around the truncation.
2. **Per loop**: inside a loop the clamp/NaN handling is per-element, so
   the conversion cannot stay in the vector unit. The surrounding
   arithmetic can remain packed while the conversion alone is extracted
   to scalar form — packed FP arithmetic feeding scalar `cvtt*2si` plus
   per-element compares and inserts is the signature. When the cast sits
   in a unit-stride loop, restoring the packed form is where the payoff
   is, not the few instructions saved at the site.

**Direction**: establish that the operand is finite and its value truncated
toward zero is representable in `I`, then use `f.to_int_unchecked::<I>()` (stable since 1.44), which
lowers to the plain truncation and re-admits the loop to vectorization.

**Not in scope**: integer-to-integer casts (`i32 as i64`, `u8 as usize`,
`i32 as isize` used as an index, …). Those cost nothing — a narrow
induction variable widened for indexing is removed outright once the
loop is analysed — and they never fire C2.

---

## 2. Where the range invariant comes from

The hard part of this rule is never recognizing the cast. It is proving
the bound. Look for the invariant in these three places, in this order;
if none of them establishes the applicable obligations of §3, **abstain**.

### 2.1 A guard in the same expression or the enclosing branch

```rust
// The `&&` chain establishes the bound before the cast is reached,
// and short-circuiting also rules out NaN (any comparison with NaN
// is false, so control never reaches the cast).
if f > I::MIN as f64 && f < I::MAX as f64 && <uses f as I> { … }

// Or an enclosing branch:
if f >= 0.0 && f <= 255.0 { let b = f as u8; … }
```
Strongest case. Cite the guard's line in the SAFETY comment.

### 2.2 A bound produced upstream in the same function

A preceding statement can narrow the value even when no comparison
appears:

```rust
let rest = total - whole * SCALE;   // rest is now in [0, SCALE)
let k = (rest / SUB) as u32;        // therefore k < SCALE/SUB
```
This is the common shape in cascaded decompositions: the **first** step
takes an unbounded input and is *not* provable, while each later step
operates on a remainder the previous subtraction already bounded. Do not
treat the steps alike — check each one on its own.

### 2.3 A defining property of the data structure

The bound may be a documented or structural invariant rather than a
statement:

- a per-element count is bounded by the total it belongs to, so
  `log2(total) - log2(element)` is non-negative;
- a total held in a 32-bit counter bounds its own logarithm by 32;
- a normalized coordinate, a table index, or an interpolation between
  in-range endpoints stays inside the endpoints' range.

State the property explicitly in the SAFETY comment. If you cannot name
it in one sentence, you have not proved it.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The operand is finite.** Establish that the value at every
  rewritten cast is neither NaN nor infinite, including values produced
  by intermediate arithmetic and callees.
- [ ] **The value truncated toward zero is representable in the target
  integer type.** Prove `I::MIN <= trunc(f) <= I::MAX` mathematically for
  the actual floating-point operand. Do not use a rounded floating-point
  representation of `I::MAX` as an inclusive upper bound: for example,
  `u64::MAX as f64` rounds to `2^64`, which is outside `u64`.
- [ ] **The range proof covers every reaching path.** Establish the bound
  from a dominating guard, upstream computation, or a data-structure and
  caller invariant. Neither typical inputs nor a clamp after the cast is
  sufficient. Retain saturating behavior wherever the proof fails.
- [ ] **The conversion result and surrounding behavior are preserved.**
  Keep the rounding mode (truncation toward zero), target width, evaluation
  order, and side effects. Keep the unsafe block limited to the conversion
  and document the finite-value and representability arguments.

For an unsigned target, a finite operand strictly between `-1` and `0`
truncates to zero and is representable. Proving non-negativity is a useful
sufficient condition, not an additional API requirement. Precision loss
near `2^53` is not by itself a reason to reject a conversion of an already
computed `f64`; establish representability of that actual operand.

---

## 4. Anti-patterns — these do NOT establish the range

- **`floor` / `ceil` / `trunc` / `round` upstream.** These guarantee the
  value is *integral*. They say nothing about finiteness, NaN, or the
  target's range: `floor` of a huge quotient is a huge integral value,
  and `floor(NaN)` is NaN. A cast written as `floor(x / K) as I` is
  provable only when something *else* bounds `x`. This is the single
  most tempting wrong inference for this rule — do not take it.
- **A clamp placed *after* the cast.** `let d = f as I; if d > LIMIT { d = LIMIT; }`
  bounds the result, not the operand. The saturation has already run.
  It is a hint that a bound exists — go find it upstream — but it is not
  itself the proof.
- **The cast appearing to be "on small values in practice".** If the
  operand can reach the site from unvalidated program input — a parsed
  number, a scripting-language value, a decoded field — the saturating
  semantics are reachable and may be load-bearing. Replacing them
  introduces UB on inputs the original handled. Abstain.
- **A sibling site in the same function already proved.** Each cast gets
  its own derivation.

---

## 5. Rewrite templates

### 5.1 Preferred — prove the range, use `to_int_unchecked`

```rust
// BEFORE
let i: I = f as I;

// AFTER
// SAFETY: <value> is finite and truncates to a representable <I>:
//  1. Origin:     <where the value comes from; which of §2.1/2.2/2.3 applies>
//  2. Finite:     <argument that it is neither NaN nor infinite>
//  3. Lower:      <proof that trunc(value) >= the exact integer I::MIN>
//  4. Upper:      <proof that trunc(value) <= the exact integer I::MAX>
let i: I = unsafe { f.to_int_unchecked::<I>() };
```

The lower and upper arguments refer to exact integer bounds and the value
truncated toward zero. Conservative floating-point bounds are acceptable
only when their rounding is accounted for. Document all four items; do not
replace a missing argument with a test result.

### 5.2 Fallback — range unprovable but the clamp is acceptable

```rust
// BEFORE
let i: I = f as I;

// AFTER
let i: I = f.clamp(I::MIN as f64, I::MAX as f64) as I;
```

Making the clamp explicit sometimes lets the backend fuse it with the
truncation. Try `to_int_unchecked` first; this is a fallback, not a way
to avoid the proof.

---

## 6. Post-rewrite self-check

- [ ] Every rewritten site has a `// SAFETY:` comment with all four
      lines (origin, finite, lower, upper).
- [ ] Bounds apply to the value truncated toward zero, including unsigned
      targets; rounded floating-point limits are not mistaken for exact bounds.
- [ ] No `unsafe { }` block extends beyond the single
      `.to_int_unchecked()` call.
- [ ] No site was rewritten on the strength of an upstream
      `floor`/`ceil`/`trunc` or a post-cast clamp alone (§4).
- [ ] Behaviour is unchanged on the entire proven-safe input domain.
