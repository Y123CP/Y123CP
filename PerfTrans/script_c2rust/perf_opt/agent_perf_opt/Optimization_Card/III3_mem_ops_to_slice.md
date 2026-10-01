# Rule III③: Manual Memory Operations → Slice / String API

## 1. What you're fixing

The function contains **untyped, C-style memory operations** — either
direct calls to libc extern functions (`memcpy` / `memmove` / `memset` /
`memcmp` / `strlen` / `strcmp` / `strncmp` / `strchr` / `snprintf` /
libm math etc.) or project-defined byte-wise wrappers (`copy_be16` /
`copy_be32` / `libzahl_memcpy`, typically short bodies with
`*p.offset(i) = ...` unrolled).

The Rust equivalent uses **typed slice / string API** that carries
length + alignment + `Copy` traits, letting the compiler fold each call
into a single `mov` / vector store / intrinsic. c2rust left them as
extern calls, so LLVM sees them as opaque and cannot inline, unroll, or
vectorize the surrounding code.

The driver tells you the sites and callee names. Each template below is
conditional on §3: matching a callee name alone does not establish compatible
units, representation, overlap, or string semantics.

---

## 2. Rewrite templates

### 2.1 `memcpy` / `memmove` → `copy_from_slice` / `ptr::copy`

```rust
// BEFORE
libc::memcpy(dst as *mut c_void, src as *const c_void, n);

// AFTER — byte-count example: dst/src are `&mut [u8]` / `&[u8]`
dst[..n].copy_from_slice(&src[..n]);

// AFTER — when still raw pointers, non-overlapping
// SAFETY: caller guarantees src/dst valid for n T's and non-overlapping
unsafe { core::ptr::copy_nonoverlapping(src, dst, n); }

// AFTER — when still raw pointers, may overlap (memmove semantics)
// SAFETY: caller guarantees src/dst valid for n T's
unsafe { core::ptr::copy(src, dst, n); }
```

### 2.2 `memset` → `fill` / `write_bytes`

```rust
// BEFORE
libc::memset(dst as *mut c_void, 0, n);

// AFTER — byte-count example: dst is `&mut [u8]`
dst[..n].fill(0);

// AFTER — when still a raw pointer of bytes
// SAFETY: caller guarantees dst valid for n bytes
unsafe { core::ptr::write_bytes(dst, 0, n); }
```

### 2.3 `memcmp` → slice equality

```rust
// BEFORE
if libc::memcmp(a as *const c_void, b as *const c_void, n) == 0 { ... }

// AFTER — a/b are `&[u8]`; only equality is observed
if a[..n] == b[..n] { ... }

// AFTER — when still raw pointers
// SAFETY: caller guarantees a/b valid for n T's
let (a, b) = unsafe {
    (core::slice::from_raw_parts(a, n), core::slice::from_raw_parts(b, n))
};
if a == b { ... }
```

### 2.4 `strlen` → `.len()` / `CStr`

```rust
// BEFORE
let n = libc::strlen(s);

// AFTER — s contains exactly the bytes before the first NUL
let n = s.len();

// AFTER — when s is a raw `*const c_char` pointing at a NUL-terminated buffer
// SAFETY: caller guarantees s is a valid NUL-terminated C string
let n = unsafe { core::ffi::CStr::from_ptr(s).to_bytes().len() };
```

### 2.5 `strcmp` / `strncmp` → slice / `CStr` comparison

```rust
// BEFORE
if libc::strncmp(a, b, n) == 0 { ... }

// AFTER — both are `&[u8]`, have length >= n, and no NUL in the first n bytes
if a[..n] == b[..n] { ... }

// AFTER — unbounded strcmp equality only, not general strncmp equality
// SAFETY: both point to valid NUL-terminated C strings
let (a, b) = unsafe {
    (core::ffi::CStr::from_ptr(a), core::ffi::CStr::from_ptr(b))
};
if a == b { ... }
```

### 2.6 `strchr` → `.iter().position(|&b| b == c)`

```rust
// BEFORE
let p = libc::strchr(s, c as i32);

// AFTER — s includes the first NUL, with no bytes beyond it; adapt index
// or None back to the pointer or null result expected by each consumer
let p: Option<usize> = s.iter().position(|&b| b == c as u8);
```

### 2.7 Byte-order helpers (`copy_be16` / `copy_be32` / `copy_be64`)

```rust
// BEFORE — typical hand-written wrapper c2rust preserves verbatim
unsafe fn copy_be32(dst: *mut u8, val: u32) {
    *dst.offset(0) = (val >> 24) as u8;
    *dst.offset(1) = (val >> 16) as u8;
    *dst.offset(2) = (val >>  8) as u8;
    *dst.offset(3) = (val >>  0) as u8;
}
copy_be32(dst, val);

// AFTER — one line, LLVM folds to a single `bswap` + `mov`
// SAFETY: caller guarantees dst has 4 bytes available
unsafe { dst.cast::<[u8; 4]>().write(val.to_be_bytes()); }

// OR, if dst is already `&mut [u8]` with length ≥ 4:
dst[..4].copy_from_slice(&val.to_be_bytes());
```

`u32::to_be_bytes` / `from_be_bytes` / `to_le_bytes` / `from_le_bytes`
cover 16 / 32 / 64 / 128-bit variants. Wrapper fns like `copy_be*` can
often be **deleted entirely** once every caller is rewritten.

### 2.8 Custom byte-wise wrappers (`libzahl_memcpy` etc.)

Inline the wrapper's body at the call site, then apply the templates
above.

```rust
// BEFORE — libzahl_memcpy is a Duff's-device style unrolled byte copy
libzahl_memcpy(dst, src, n);

// AFTER — inline and replace
unsafe { core::ptr::copy_nonoverlapping(src, dst, n); }
// or, if the pointers are already slices:
dst[..n].copy_from_slice(&src[..n]);
```

The wrapper function itself may then have no remaining callers and can
be removed.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The operation's units and extents match.** Distinguish byte counts
  from element counts, prove both buffer bounds, and rule out size overflow.
  Creating a typed view additionally requires initialized valid elements,
  alignment, non-null pointers, a single allocation, and a valid lifetime.
- [ ] **Overlap behavior is equivalent.** `copy_from_slice` and
  `copy_nonoverlapping` require non-overlap. Use `ptr::copy` or `copy_within`
  for overlapping regions only when the original has snapshot-copy
  semantics; preserve forward-propagating copies explicitly. Never create
  conflicting references merely because a raw-pointer API permits overlap.
- [ ] **Typed operations preserve byte-level meaning.** `T: Copy` is needed
  for `copy_from_slice`; `clone_from_slice` is not a default replacement
  for a raw copy. Establish that typed equality/fill matches the original
  byte operation, including representation, padding, and any side effects.
- [ ] **String termination and comparison semantics match.** Establish
  a valid first NUL within the allocation before using `CStr`. A slice's
  length equals `strlen` only when it excludes exactly that terminator and
  contains no earlier NUL. For bounded comparison, preserve the `n` limit
  and early-NUL behavior; full-string equality is not a general replacement
  for `strncmp`. Preserve whether the caller observes equality or ordering.
- [ ] **Search results retain their meaning.** For `strchr`, preserve the
  first-match pointer/null result and searching for the terminator itself;
  an index/`Option` requires a corresponding adaptation of all consumers.
- [ ] **Loads, stores, and byte order are valid.** Use appropriate alignment
  or unaligned APIs; preserve endianness, ownership, and destructor behavior.
  Every introduced unsafe operation needs a site-specific `// SAFETY:` proof.

---

## 4. When to abstain

Skip the rewrite if any of these hold:

- Length `n` is a runtime value with no provable upper bound.
- The pointers point into different allocations that may alias each
  other in ways not derivable from the source (e.g. two `*mut u8`
  parameters that could be the same address).
- The libc call has semantics **beyond** raw memory movement — e.g.
  `strncpy` fills the remainder with NUL, and `snprintf` has
  format-string semantics that need a full parse.
- The callee is a custom wrapper whose body you cannot see (external
  crate, C-linked object) — you cannot inline what you cannot read.
- Element type has a destructor and the caller intended a shallow copy
  (would leak or double-drop).
- **Hot decode / compress loop perf abstain:** the
  `libc::memcpy` / `libc::memmove` / `libc::memset` call is inside a
  compressor's or decoder's inner encode / decode / match-copy loop
  (>100 iterations per outer call). Renaming to
  `core::ptr::copy_nonoverlapping`, using `<T>::from_le_bytes` for
  small unaligned loads, or hoisting a boolean flag out of such a loop
  **is not a free rewrite** — it perturbs LLVM's inlining /
  register-allocation / branch-fusion plan for the tightly-tuned
  surrounding code, often producing a measurable wall-clock regression
  **without any semantic win**. The rewrite is semantically correct;
  only the codegen kernel around it gets disturbed. Prefer to leave
  `libc::` calls unchanged inside such kernels; ship III③ only in
  cold / boundary code (bit-reader init, table setup, one-shot header
  path).

  Typical patterns to recognize (leave alone):
    * `libc::memcpy(dst, src, N)` with `N` a small fixed constant
      (4, 8, 16) inside an inner while/for loop,
    * `libc::memset` clearing a small stack buffer at each iteration,
    * a `bool`-typed context field read every iteration whose value
      is loop-invariant (hoisting also disturbs the kernel).

---

## 5. Post-rewrite self-check

- [ ] No `libc::memcpy` / `memset` / `memcmp` / `strlen` / `strcmp` /
      `strncmp` / `strchr` / `strncpy` / `snprintf` remains for the
      rewritten site.
- [ ] Every new `unsafe { }` block is minimal and carries a
      `// SAFETY:` comment.
- [ ] Custom byte-wise wrapper fns that lost all callers are either
      removed or marked `#[deprecated]` for later cleanup.
- [ ] `cargo check` passes; `cargo build --release` compiles without
      new warnings.
