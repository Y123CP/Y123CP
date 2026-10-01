# Rule C10: Scalar Element-Wise Processing

Implementation template: Fixed-Length Sub-Word Loop.

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

A loop whose **trip count is a compile-time constant smaller than a machine
word**, whose iterations carry nothing to one another, and whose body touches
**one sub-word unit** — a bit, a nibble, a byte — per step.

All the work such a loop does fits in one register. It nevertheless pays the
trip count in loop overhead and, when the body branches, in branches:

```rust
while j < 8 as size_t {                    // 8 iterations …
    if data[i] as c_int & mask as c_int != 0 {   // … 8 branches …
        *p = 1 as c_uchar;
    } else {
        *p = 0 as c_uchar;
    }
    p = p.offset(1);                       // … 8 stores of one byte each
    mask = (mask as c_int >> 1 as c_int) as c_uchar;
    j = j.wrapping_add(1);
}
```

The fix is always the same shape: **fold the N sub-word steps into one
word-wide step.**

### Not to be confused with C4

C4's loop runs until the **data** says stop — a sentinel, a mismatch, a buffer
end — so nobody knows the count in advance and the rewrite has to reason about
reading past the last needed byte. Here the count is a literal in the source.
That single difference changes the detection, the safety obligations, and the
templates. If the loop has a `break`, it is not this rule.

### The c2rust disguise

The loop variable is rarely the only thing stepping. c2rust lowers quantities
that are really functions of the iteration number into **self-advancing
locals** — an output cursor `p = p.offset(1)`, a bit mask `mask = mask >> 1`.
A write through such a cursor (`*p = …`) is an indexed write. Do not read those
updates as loop-carried state; their values depend only on how many times the
loop has run, so the folded form can compute them directly.

Genuine state — a value the next iteration reads that came from what this one
**loaded** — disqualifies the loop. That is a recurrence, not this pattern.

---

## 2. The four forms

All four collapse N sub-word steps into one word step. Pick by what the body
does, not by how it is spelled.

### 2.0 Every template below starts from a slice

The templates index and `copy_from_slice`; c2rust hands you raw pointers. The
step between the two — establishing the views the template reads and writes
through — belongs to this rewrite and is not optional. The word-wide store
**is** a slice operation. There is no way to express it through a raw cursor,
so a fold that never builds a view has not folded anything.

Take each length from the same quantity the original loop used to bound
itself: the outer trip count for the source, that count times the fixed inner
count for the destination. Both are already in the function; the fixed count
is the literal in the inner loop's condition.

If III④ also matches this region, establish both cards' preconditions and
coordinate view recovery with the fold in one candidate rewrite. Reuse the
same valid views instead of constructing a second, potentially conflicting
set. A match for either card does not establish the other's conditions.

**Unrolling the loop into N lines of raw stores is not this rewrite.** It
keeps every store, every mask, and every branch the loop had, and deletes only
the counter. If the result still writes one sub-word at a time, the fold did
not happen — whatever the plan declared.

### 2.1 `split` — one word taken apart into adjacent sub-words

Bit expansion, byte extraction from a word, nibble unpacking.

```rust
// ❌ 8 iterations, 8 branches, 8 one-byte stores
while j < 8 { *p = if byte & mask != 0 { 1 } else { 0 };
              p = p.offset(1); mask >>= 1; j += 1; }

// ✅ one multiply broadcasts the byte to 8 lanes, one mask selects one bit
//    per lane, one add-shift-mask normalises each lane to 0/1, one 8-byte store
const LANES:  u64 = u64::from_le_bytes([0x01; 8]);
const HALF:   u64 = u64::from_le_bytes([0x7f; 8]);
// lane i keeps the bit that lands in output byte i — MSB first, matching the
// loop's mask walking down from 0x80
const SELECT: u64 = u64::from_le_bytes([0x80,0x40,0x20,0x10,0x08,0x04,0x02,0x01]);

let spread = (byte as u64).wrapping_mul(LANES);
let ones   = ((spread & SELECT).wrapping_add(HALF) >> 7) & LANES;
out_chunk.copy_from_slice(&ones.to_le_bytes());
```

**Write lane constants as `from_le_bytes([...])`, never as a hex literal.**
Sixteen hex digits carry no evidence of which end is which, and a reversed mask
compiles, looks right, and fails W1 — measured, on exactly this rewrite. The
array form puts the byte order on the page: element `i` is the mask for output
byte `i`, in the order `to_le_bytes` writes them, so the two can be read
against each other.

Why the `+ 0x7f` step is sound: it turns "lane is non-zero" into "lane's top
bit is set", and the largest lane value is `0x80 + 0x7f = 0xff`, so **no lane
carries into its neighbour**. Verify any such constant by enumerating all 256
inputs before shipping it.

A 256-entry `[[u8; 8]; 256]` lookup table plus one `copy_from_slice` is the
other correct answer, and the better one when the mapping is not expressible
in arithmetic.

### 2.2 `pack` — N sub-words accumulated into one word

```rust
// ❌ while i < 4 { acc |= (b[i] as u32) << (8 * i); i += 1; }
// ✅ let acc = u32::from_le_bytes(b[..4].try_into().unwrap());
```

Use `from_le_bytes` / `from_be_bytes` and let the endianness be explicit. If
the source bytes are not contiguous, keep the loop — assembling them costs more
than the loop saves.

**When each source byte contributes one BIT, not a whole byte**, the byte
conversions do not apply — this is §2.1 run backwards, and it folds the same
way, with one multiply:

```rust
// ❌ 8 iterations, each shifting the accumulator and or-ing one 0/1 byte
while j < 8 { v = (v << 1) | *p; p = p.offset(1); j += 1; }

// ✅ one 8-byte read, one multiply gathers the eight low bits into the top byte
const GATHER: u64 = u64::from_le_bytes([0x80,0x40,0x20,0x10,0x08,0x04,0x02,0x01]);
let word = u64::from_be_bytes(chunk.try_into().unwrap());
*dst = (word.wrapping_mul(GATHER) >> 56) as u8;
```

`from_be_bytes` because the first byte of the chunk becomes the high bit, which
is what `(v << 1) | bit` does. `GATHER` is `SELECT` from §2.1 — the same
byte-to-lane assignment, read the other way. Verified over all 256 outputs.

**`fold` is not this rewrite.** `chunk.iter().fold(0, |acc, &b| acc << 1 | b)`
is still eight dependent steps wearing an iterator; it belongs to III④. If the
loop is still there after your rewrite — spelled `for`, `fold`, or anything
else — C10 did not apply.

### 2.3 `copy` — N sub-words moved or translated one at a time

```rust
// ❌ while i < 6 { dst[i] = src[i]; i += 1; }
// ✅ dst[..6].copy_from_slice(&src[..6]);

// ❌ while i < 8 { dst[i] = TBL[src[i] as usize]; i += 1; }
// ✅ for (d, &s) in dst[..8].iter_mut().zip(&src[..8]) { *d = TBL[s as usize]; }
```

This form does **not** require sub-word elements. Its payoff is N element
loads and stores becoming one `memcpy`-shaped move, and six `u32`s fold as
well as six bytes. `copy_from_slice` on a fixed-length range lowers to a single
inlined move; a `[T; N]` assignment does too.

### 2.4 `reduce` — N sub-words folded into one scalar

```rust
// ❌ while i < 4 { sum += a[i] as u32; i += 1; }
// ✅ let sum: u32 = a[..4].iter().map(|&x| x as u32).sum();
```

The gain here is the smallest of the four — LLVM already unrolls many of these.
Apply it only when the loop is in the hot path and the body is otherwise
trivial; otherwise abstain.

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
- [ ] **The trip count is established independently.** The original loop
  executes the stated fixed number of iterations on every reaching path;
  a detector match or literal slice length alone does not prove this.
- [ ] **Source and destination extents cover the full operation.** Equal
  lengths are established before `copy_from_slice`; fixed-size conversion
  receives exactly the required bytes. Do not introduce reachable panics.
- [ ] **Aliasing preserves the original read/write dependencies.** Use
  non-overlapping views for `copy_from_slice`. Use `copy_within` only when
  snapshot-copy behavior is equivalent to the original loop; a forward
  propagating copy is not generally equivalent to `memmove`.
- [ ] **Packing and unpacking preserve the byte order.** Select `to_le_bytes`
  or `to_be_bytes` to match the original, with native order only when the
  original operation itself uses native order. Preserve final loop state
  and any effects that were interleaved with the element operations.

---

## 4. When to abstain

Skip the site (leave it unchanged) if any of these hold:

- **The loop can exit early.** A `break` / `return` means the count is not
  really fixed — that is C4's pattern, not this one.
- **An iteration reads what the previous one wrote.** A recurrence cannot be
  folded without changing the result. Self-advancing cursors and masks are not
  this (see §1).
- **The trip count is not a literal.** `while i < n` with a runtime `n` gives
  the optimiser nothing extra; leave it.
- **The body is more than a few statements**, or contains a nested loop or a
  call. Then the loop overhead is not what dominates, and folding it buys
  little while risking a lot.
- **A `reduce` whose body is already trivial arithmetic.** LLVM unrolls those;
  the rewrite is churn. Prefer abstaining over shipping a neutral change.

---

## 5. Post-rewrite self-check

- [ ] **The constant is verified, not guessed.** For any SWAR mask or magic
      multiplier, enumerate the whole input domain (256 values for a byte) and
      compare against the original loop before shipping.
- [ ] **Lane constants are `from_le_bytes([...])`, not hex literals.** A
      reversed 64-bit mask is invisible in hex and fails W1.
- [ ] **No lane carries into its neighbour.** State the maximum per-lane value
      and check it stays inside the lane width.
- [ ] **Every length is a literal**, and both sides of a `copy_from_slice`
      derive theirs from the same constant.
- [ ] **The loop is gone**, not merely shortened — if a loop remains, this rule
      did not apply and the rewrite should be reverted.
- [ ] **The store is word-wide.** N adjacent one-element stores, unrolled or
      not, means the fold did not happen; the rewrite has to go through a
      slice for the whole word to move at once.
- [ ] Endianness is explicit wherever bytes and words are converted.
