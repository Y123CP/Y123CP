# Rule C8: CRC Table-Recurrence Slicing

## 1. What you're fixing

The hot function computes a **table-driven CRC** one byte at a time. The
canonical c2rust shape is a serial recurrence over a 256-entry table:

```rust
// crc carries across every byte; TABLE is a [u32; 256] static
while len != 0 {
    crc = crc >> 8 ^ TABLE[((crc ^ *p) & 0xff) as usize];
    p = p.offset(1);
    len -= 1;
}
```

(c2rust often unrolls this 4x, but each of the 4 statements is the same
`crc = crc >> 8 ^ TABLE[((crc ^ byte) & 0xff) as usize]` recurrence.)

Because byte `i+1`'s update reads the `crc` produced by byte `i`, the loop
carries a **serial dependency chain of one table lookup per byte**. LLVM
cannot vectorize it and the CPU stalls on the load-use latency of each
lookup; the faithfully translated C is equally serial. This is the CRC-32
kernel behind zlib/gzip/zip checksums, deflate integrity, and any
`crc32`-style digest.

**Direction**: apply **slicing-by-N** (Brumme / Intel). Derive `N` extra
tables once from the base table, then consume `N` bytes per iteration with
`N` **independent** table lookups XOR'd together. The independent lookups
expose data-level parallelism — the loads issue in parallel and the
critical path drops from `N` lookups to ~1 per `N` bytes. This is the same
principle as C5 (break a serial reduction into independent partial
computations), but the syntactic shape is a table recurrence, not a
dual-accumulator sum.

**Not this rule**: a CRC already using a hardware intrinsic
(`_mm_crc32_*` / `crc32` instruction); a single-table lookup that is not a
CRC recurrence (no `crc >> 8` self-shift, no `(crc ^ byte) & 0xff` index);
a checksum with a running-sum shape (that is C5, not this).

---

## 2. Rewrite template (slicing-by-8)

Keep the base 256-entry table exactly as-is. Build 8 tables **once**, using
an initialization strategy whose validity is established under §3, so repeated
calls pay the setup only once, then run the main loop 8 bytes at a time with an 8-wide tail-free
inner body, followed by the original byte-at-a-time loop for the `< 8`
remainder.

```rust
// BEFORE (byte-at-a-time, possibly 4x-unrolled):
//   while len != 0 {
//       crc = crc >> 8 ^ TABLE[((crc ^ *p) & 0xff) as usize];
//       p = p.offset(1); len -= 1;
//   }

// AFTER — slicing-by-8, conditional on the serialized-access proof in §3.
// S8[0] == the original TABLE; S8[s] is derived from it.
static mut S8: [[u32; 256]; 8] = [[0u32; 256]; 8];
static mut S8_INIT: bool = false;
if !S8_INIT {
    let mut i: usize = 0;
    while i < 256 { S8[0][i] = TABLE[i]; i += 1; }        // row 0 = base table
    let mut s: usize = 1;
    while s < 8 {
        let mut j: usize = 0;
        while j < 256 {
            S8[s][j] = S8[s - 1][j] >> 8
                ^ S8[0][(S8[s - 1][j] & 0xff) as usize];  // each row = one more byte
            j += 1;
        }
        s += 1;
    }
    S8_INIT = true;
}
// This unsynchronized example is applicable only if all calls are proven
// serialized and initialization is non-reentrant. Otherwise use immutable
// precomputed tables or safe one-time initialization, or abstain. Check
// functional behavior and measure performance after establishing validity.
while len >= 8 {
    // fold 8 input bytes + current crc into two 32-bit words (little-endian read)
    let one: u32 = (p as *const u32).read_unaligned() ^ crc;
    let two: u32 = (p.offset(4) as *const u32).read_unaligned();
    crc = S8[7][(one         & 0xff) as usize]
        ^ S8[6][(one  >> 8    & 0xff) as usize]
        ^ S8[5][(one  >> 16   & 0xff) as usize]
        ^ S8[4][(one  >> 24)         as usize]
        ^ S8[3][(two         & 0xff) as usize]
        ^ S8[2][(two  >> 8    & 0xff) as usize]
        ^ S8[1][(two  >> 16   & 0xff) as usize]
        ^ S8[0][(two  >> 24)         as usize];
    p = p.offset(8);
    len -= 8;
}
while len != 0 {                                          // unchanged tail
    crc = crc >> 8 ^ TABLE[((crc ^ *p) & 0xff) as usize];
    p = p.offset(1); len -= 1;
}
```

The `crc` here is the working accumulator **after** any initial
`^ 0xffffffff` / `!` that the original applied — keep that pre/post
inversion (`crc = seed ^ 0xffffffff` before the loop, `return !crc` after)
exactly as the original had it. The 8 lookups on each iteration are
independent (different tables, no dependency between them), so the loads
parallelize; only the final XOR reduces them.

`read_unaligned` matters: the input pointer has no alignment guarantee. A
plain `*(p as *const u32)` is UB on unaligned addresses — always use
`(p as *const u32).read_unaligned()`.

If the target is little-endian (x86-64 here), the word read above is
correct as written. On a big-endian target the byte-extraction order
would flip; since the study runs x86-64, the LE form is exact.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The recurrence admits an equivalent block-wise formulation.**
  Derive equivalence for this template's operation and state, rather than
  inferring independence from unit stride or adjacent element accesses.
  For independent element operations, establish that grouping their scalar
  steps produces the same result and final state.
- [ ] **Required inputs can be read before overlapping outputs are written.**
  Account for dependencies within and across blocks. Preserve values needed
  by later iterations, or establish that the input is never overwritten.
- [ ] **Boundary cases preserve the original semantics.** Cover empty input,
  the first and last elements, complete blocks, partial tails, and the
  original stopping condition and final state.
- [ ] **The CRC recurrence and tables match exactly.** Derive all slicing
  tables from the fixed base table using the original polynomial, width,
  and reflected/non-reflected convention. The displayed formula applies
  only to the right-shift recurrence in this template.
- [ ] **Seed and finalization semantics are unchanged.** Preserve entry
  xor, exit complement, state carried between calls, and returned digest.
- [ ] **All word reads stay within initialized input.** Use unaligned loads
  where needed, preserve byte order, and retain the original byte-at-a-time
  recurrence for every remaining byte.
- [ ] **Table initialization is valid for every caller.** The unsynchronized
  mutable-static example requires proven serialized access and no reentrant
  initialization. c2rust translation or a single-threaded benchmark alone
  does not establish this. Otherwise use a supported immutable precomputed
  table or safe one-time initialization, or abstain; measure its cost later.
- [ ] **The edit preserves the function contract.** Keep the signature,
  ABI, seed inputs, base-table meaning, and permitted edit scope unchanged.

---

## 4. When to abstain

- The recurrence is not actually a CRC (no `crc >> 8` self-shift feeding a
  `(crc ^ byte) & 0xff` table index) — the closed form does not apply.
- The table is not a fixed CRC table (e.g. it is recomputed per call from
  varying data) — deriving the slice tables would be wrong.
- The CRC is a negligible fraction of the function's work, so the serial
  chain is not the bottleneck.
- You cannot convince yourself the derived tables reproduce the exact
  digest — a wrong slice table silently corrupts every checksum. Prefer to
  abstain over guessing.

---

## 5. Post-rewrite self-check

- [ ] `S8[0]` is a byte-for-byte copy of the original table; rows 1..8 are
      derived by the recurrence above, not transcribed.
- [ ] The 8 per-iteration lookups are independent (different table rows,
      no lookup reads another's result) so the loads parallelize.
- [ ] Word reads use `read_unaligned`; the byte-extraction order matches
      little-endian.
- [ ] The `< 8`-byte tail runs the original byte-at-a-time recurrence.
- [ ] Entry seed xor and exit complement are unchanged.
- [ ] Initialization satisfies the concurrency and reentrancy conditions in §3.
- [ ] `cargo check` and W1 pass, including tests for empty input and
      non-multiple-of-8 lengths; these checks supplement the equivalence proof.
