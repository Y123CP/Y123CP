# Rule C4: Scalar Element-Wise Processing

Implementation template: Word-at-a-Time Traversal.

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

The hot function contains a **byte-serial pointer loop**: a loop that
advances one byte per iteration and continues until a **data-dependent
condition** — a mismatch, a sentinel/zero byte, or a buffer end. The
canonical c2rust shape is

```rust
while fore != end && *back == *fore {   // or: *p == 0, *p != sep, ...
    back = back.offset(1);
    fore = fore.offset(1);
}
```

It appears in two syntactic disguises, and **the second is the common one in
LZ codecs**: C's `do { … } while (cond)` lowers to `loop { … if !cond
{ break } }`, which has no loop condition at all and usually carries a
hand-unrolled compare chain inside the `if`. Same traversal, same fix —
see 2.2. Do not skip a loop because it is not spelled `while`.

Because the trip count depends on the data, LLVM **cannot** vectorize
this loop (it emits no "vectorized loop" remark and there is nothing to
"restore"), and the faithfully translated C is equally byte-serial. Each
byte costs a load (or two), a compare, a branch, and a pointer bump.

This idiom is the inner engine of match-extension in LZ-family
compressors (`deflate`/`lz4`/`zstd`), of `memcmp`/`memchr`/`strlen`-style
scans, and of run-length / zero-run detection.

**Template strategy**: process a full machine word (`u64`, 8 bytes) per step.
Read a word from each cursor with one operation, combine the words
(`XOR` for equality, a comparison for a scan), and locate the first byte
of interest with a **bit scan** (`trailing_zeros` on a little-endian
target). Advance by the number of bytes resolved and finish the final
`< 8`-byte remainder with the original byte loop. This replaces up to
eight byte iterations — and their per-byte branches — with one word
operation.

**Not in scope**: loops whose element step is not one byte (e.g.
`.offset(1)` on a `*u32`), loops already reading words
(`read_unaligned::<u64>`), and loops whose body does non-cursor work per
iteration (those are a different rule).

---

## 2. Rewrite templates

### 2.1 Equality / match-extension (compare two cursors)

```rust
// BEFORE
while fore != end && *back == *fore {
    back = back.offset(1);
    fore = fore.offset(1);
}

// AFTER — first-byte gate, word-wide compare, then scalar tail.
// The gate is part of the rewrite, not an option: see "Entry cost" below.
if fore != end && *back == *fore {
    // Preconditions: end is a proven readable bound, fore <= end, and both
    // cursors have valid 8-byte windows whenever the guard succeeds. Prove
    // back's bounds separately; back < fore alone does not establish them.
    // Reads are non-volatile and the scanned bytes remain unchanged.
    // Only under these conditions does address subtraction bound the load.
    while (end as usize).wrapping_sub(fore as usize) >= 8 {
        let bw = (back as *const u64).read_unaligned();
        let fw = (fore as *const u64).read_unaligned();
        let x = bw ^ fw;
        if x != 0 {
            let adv = (x.trailing_zeros() >> 3) as isize;  // first differing byte (LE)
            back = back.offset(adv);
            fore = fore.offset(adv);
            break;
        }
        back = back.offset(8);
        fore = fore.offset(8);
    }
}
while fore != end && *back == *fore {   // sub-word tail (< 8 bytes); also the
    back = back.offset(1);              // whole traversal when the gate fails
    fore = fore.offset(1);
}
```

**Entry cost — why the gate is mandatory.** The word loop is not free to
enter: bound test, two loads, XOR, bit-scan, advance, and a re-check in the
tail cost about a dozen instructions per entry, while a first-byte mismatch
costs the scalar loop one compare. When the loop sits inside a candidate
walk (hash chain, dictionary probe, lazy-match retry), it is entered once
per candidate, and in repetitive data most candidates mismatch on their very
first byte — an earlier skip step (e.g. over a run of equal bytes) often
leaves exactly the byte that differs. Without the gate those entries make
the rewrite a net loss on that input shape while it still wins on others,
so a single benchmark input can show a large gain for a rewrite that
regresses a different input. The gate keeps the first-byte mismatch at the
scalar cost and leaves the word loop only for runs that have already
started matching. The same applies to 2.2 and 2.3: test the first element
with the original scalar condition before the first word read.

### 2.2 The same loop, lowered from C's `do { … } while (cond)`

c2rust lowers a do-while to `loop { … if !cond { break } }`, and C compilers
conventionally hand-unroll this particular loop. The result carries **no loop
condition at all** — every compare lives in one `&&` chain inside the `if`,
and the bound test is the chain's last term. It is the same byte-serial
traversal as 2.1 and takes the same fix; only the syntax differs, so match on
the shape, not on the keyword.

```rust
// BEFORE — N-way unrolled compare chain; each `&&` arm steps both cursors.
loop {
    p = p.offset(1);
    q = q.offset(1);
    if !(*p == *q
        && { p = p.offset(1); q = q.offset(1); *p == *q }
        /* … N-1 arms in total … */
        && p < end)
    {
        break;
    }
}

// AFTER — guarded word compare, with the original chain as the tail.
loop {
    // The chain is SHORT-CIRCUIT: on a mismatch in arm 1 it reads only p+1.
    // A word read takes p+1..=p+8 unconditionally, so it reads bytes the
    // original may never touch — the guard, not the unroll width, is what
    // makes that sound. Address arithmetic in `usize`: never form the
    // out-of-bounds pointer that `p.offset(8) <= end` would.
    if (end as usize).wrapping_sub(p as usize) < 8 {
        break;                       // fall through to the scalar tail
    }
    let pw = ::core::ptr::read_unaligned(p.offset(1) as *const u64);
    let qw = ::core::ptr::read_unaligned(q.offset(1) as *const u64);
    let x = pw ^ qw;
    if x != 0 {
        let adv = 1 + (x.trailing_zeros() >> 3) as isize;  // first differing byte (LE)
        p = p.offset(adv);
        q = q.offset(adv);
        break;
    }
    p = p.offset(8);
    q = q.offset(8);
    if p >= end { break; }
}
// scalar tail — the ORIGINAL loop verbatim, reached when < 8 bytes remain.
loop {
    p = p.offset(1);
    q = q.offset(1);
    if !(*p == *q /* … the original chain … */ && p < end) { break; }
}
```

Three things decide whether this is equivalent, and all three are easy to get
wrong:

1. **Keep the `+1`.** The first compare is at `p+1`, not `p` — the chain steps
   the cursors *before* dereferencing. The word must be read from `p+1`, and a
   mismatch at word-index `k` means the cursors stop at `p+1+k`.
2. **Stop ON the differing byte, not after it.** Downstream code usually
   derives the match length from the final cursor (`len = MAX - (end - p)`),
   so an off-by-one here silently changes output rather than crashing.
3. **Test the bound AFTER advancing.** The chain checks `p < end` only once
   all N compares succeeded, so the word version must advance by 8 first and
   test after — testing before it changes the trip count at the boundary.

**An N-way unroll does NOT license an N-byte read.** The chain is
short-circuit: a mismatch in the first arm reads one byte and stops. The word
read is unconditional. So "the original already reads 8 bytes" is false, and
the guard in the template is doing real work — do not drop it on the grounds
that the loop looks 8-way unrolled.

**`end` here is usually a soft cap.** In LZ codecs the bound is typically
`start + MAX_MATCH`, a limit on match LENGTH, not the end of the allocation.
The wide-read bounds condition in §3 still applies to it: guard against `end`, and let the scalar tail handle
the last `< 8` bytes. Some codecs do over-allocate their window and zero a
slack region precisely so the match loop can read past the data (look for a
`high_water` / `WIN_INIT`-style mechanism). Even then, prefer the guard: the
slack is sized for the SCALAR loop's reach, and it is a property of one
codec's allocator, not something visible in the region you are editing.

### 2.3 Scan-for-byte (find first byte matching a predicate)

```rust
// BEFORE
while p != end && *p == 0 {          // or *p != sep
    p = p.offset(1);
}

// AFTER — broadcast the target byte, XOR, then trailing_zeros.
while (end as usize).wrapping_sub(p as usize) >= 8 {
    let w = (p as *const u64).read_unaligned();
    let hit = w ^ 0x0000_0000_0000_0000u64;   // XOR with the broadcast target byte
    if hit != 0 {                              // a non-target byte appears in this word
        p = p.offset((hit.trailing_zeros() >> 3) as isize);
        break;
    }
    p = p.offset(8);
}
while p != end && *p == 0 {                    // sub-word tail
    p = p.offset(1);
}
```

For a scan, broadcast the target byte into all 8 lanes
(`(byte as u64) * 0x0101_0101_0101_0101`) before the `XOR`; the shown
`0` case broadcasts zero. `trailing_zeros() >> 3` is the byte offset of
the first lane whose XOR is non-zero.

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
- [ ] **The operation matches this byte-scan template.** The loop reads
  bytes without modifying the scanned data; the comparison or bit test
  reproduces the scalar predicate and first-hit position. This is not a
  template for arbitrary adjacent-limb updates.
- [ ] **Every wide read is valid independently.** Establish the allocation,
  readable extent, and ordered cursor bounds for each input. Read eight
  bytes only when eight initialized bytes remain. A trailing address or
  wrapping address subtraction alone does not prove a second buffer valid.
- [ ] **Load and bit-scan semantics match.** Use unaligned reads when
  needed, and match byte order to the original memory order. Wider reads
  must not introduce observable volatile/atomic accesses or races.
- [ ] **The scalar tail and cursor results are preserved.** Do not overread
  at the last word, miss an early hit, or change a returned cursor/length.

---

## 4. When to abstain

- The loop step is not one byte, or the pointee is wider than `u8`.
- `end` is a soft limit (e.g. a "nice length" cap) rather than the
  physical end of valid memory — a word read could pass real data.
- The loop body writes to memory it also reads (overlap becomes
  order-sensitive).
- The loop is already word-wide (`read_unaligned`, `u64`, `.offset(8)`
  present).
- The typical run is provably short (a few bytes) AND the loop cannot be
  gated as in 2.1. An ungated word loop does not "only add a branch": its
  entry costs about a dozen instructions even when it resolves nothing, so
  on a loop entered once per short probe it is a net loss. With the
  first-element gate the short case keeps its scalar cost, so a short
  typical run is a reason to gate, not by itself a reason to abstain.

---

## 5. Post-rewrite self-check

- [ ] The word loop is entered only after the first element passed the
      original scalar condition (the 2.1 gate), so a probe that fails on
      its first element costs what the scalar loop costs.
- [ ] The word loop's guard is the `usize` address difference `>= 8`,
      not `cur.offset(8) <= end`.
- [ ] A scalar tail loop handles the final `< 8` bytes and the
      first-mismatch position, so the computed length/position is
      identical to the original.
- [ ] `end` is the physical end of valid data; no read can pass it.
- [ ] The rewrite is behavior-preserving: same match length, same scan
      stop position, same digest on every input.
- [ ] `cargo check` passes.
