# Rule III②: Manual Heap Management → RAII Container

## 1. What you're fixing

The function contains **manual C-style heap management** — direct calls
to `libc::malloc` / `calloc` / `realloc` / `free` (or aligned variants
`aligned_alloc` / `posix_memalign` / `memalign` / `valloc`). c2rust
preserved these instead of using Rust's owning containers, so:

- The compiler treats the allocation as an opaque FFI byte buffer,
  loses type / length / alias information.
- Manual `free` is easy to forget, double-free, or leak on early
  return / panic paths.
- The block's lifetime is invisible to the borrow checker, so
  downstream slice / iterator rewrites (III③, III④) cannot see it.

**Direction**: replace the manual pair with a Rust owning container
(`Vec<T>` for growable buffers, `Box<[T]>` for fixed-size,
`vec![0; n]` for zero-initialized scratch). RAII drops the allocation
automatically at scope end; length + type + noalias information all
become visible to LLVM.

---

## 2. Rewrite templates

### 2.1 Uninitialized scratch buffer

```rust
// BEFORE
let buf = libc::malloc(n * mem::size_of::<u32>()) as *mut u32;
if buf.is_null() { return -1; }
for i in 0..n {
    *buf.offset(i as isize) = compute(i);
}
consume(buf, n);
libc::free(buf as *mut c_void);

// AFTER
let mut buf: Vec<u32> = Vec::with_capacity(n);
for i in 0..n {
    buf.push(compute(i));                       // or: buf.spare_capacity_mut() + set_len
}
consume(buf.as_mut_ptr(), buf.len());           // if callee still wants raw ptr
// buf drops here — free is automatic
```

### 2.2 Zero-initialized buffer

```rust
// BEFORE
let buf = libc::calloc(n, mem::size_of::<u8>()) as *mut u8;
work(buf, n);
libc::free(buf as *mut c_void);

// AFTER
let mut buf = vec![0u8; n];                     // heap-allocated, zeroed, RAII
work(buf.as_mut_ptr(), buf.len());
```

For non-integer element types with a `Default` impl:
`vec![T::default(); n]`.

### 2.3 Growable buffer with `realloc`

```rust
// BEFORE
let mut buf = libc::malloc(cap) as *mut u8;
loop {
    if len + need > cap {
        cap *= 2;
        buf = libc::realloc(buf as *mut c_void, cap) as *mut u8;
    }
    *buf.offset(len as isize) = byte;
    len += 1;
}
libc::free(buf as *mut c_void);

// AFTER
let mut buf: Vec<u8> = Vec::with_capacity(cap);
loop {
    buf.push(byte);                             // reallocates automatically
}
```

### 2.4 Aligned allocation

```rust
// BEFORE
let mut ptr: *mut c_void = ptr::null_mut();
libc::posix_memalign(&mut ptr, 64, n * 4);
let buf = ptr as *mut f32;
work(buf, n);
libc::free(ptr);

// AFTER
use std::alloc::{alloc, dealloc, Layout};
let layout = Layout::from_size_align(n * 4, 64).unwrap();
// SAFETY: layout has non-zero size (n > 0 guaranteed by caller)
let buf = unsafe { alloc(layout) as *mut f32 };
if buf.is_null() { std::alloc::handle_alloc_error(layout); }
work(buf, n);
// SAFETY: buf came from alloc(layout) with the same layout
unsafe { dealloc(buf as *mut u8, layout); }
```

`Vec` / `Box` don't offer alignment > `align_of::<T>()`, so aligned
buffers stay with the `std::alloc` API — but the pairing is still
RAII-friendly via a small wrapper struct with a `Drop` impl if needed.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The allocation remains local with a known owner.** Establish all
  uses and releases. The allocation must not escape through a return,
  stored field, or retaining callee; a temporary FFI borrow may not retain
  or free the container's storage.
- [ ] **Allocation and deallocation contracts match.** Preserve alignment,
  element count, and allocator/layout requirements. Do not adopt a C
  allocation into a Rust owner without proving allocator compatibility.
- [ ] **Initialized elements are valid `T` values.** Capacity is not length;
  never expose uninitialized storage as `Vec<T>` elements or references.
  A byte buffer cast must also satisfy the target alignment and validity.
- [ ] **Borrowed pointers remain valid at every use.** A container must not
  drop or reallocate while a callee or local alias still uses its pointer;
  its lifetime alone does not guarantee pointer stability.
- [ ] **Cleanup and observable failure behavior are preserved.** Each owned
  allocation is released exactly once, including early exits; preserve
  element destruction, allocation-error handling, and relevant panic paths.

---

## 4. When to abstain

Skip the rewrite if any of these hold:

- The allocation **escapes the function** (returned, stored in a struct
  field for later use, or passed to `extern "C"` code that will free
  it later).
- Alloc and free are in **different functions** (ownership crosses fn
  boundary).
- The function uses `setjmp` / `longjmp` or custom stack unwinding —
  Rust drop won't run on non-local exit; RAII is unsound.
- The pointer is stored in a **union** or reinterpreted from raw bytes
  — the RAII wrapper can't track type identity through unions.
- The alloc is passed to a **custom memory pool** (e.g. libzahl's
  size-bucket pool) — the pool's own semantics carry information that
  `Vec` cannot express; leave the manual alloc, apply body-level
  optimizations only.
- The alignment requirement exceeds `align_of::<T>()` **and** the site
  is trivial — the `std::alloc` fallback is more code than the saved
  free; measure first.
- The allocation size is **zero at runtime** and `Vec::with_capacity(0)`
  would not call the allocator (behavioral difference — libc returns a
  distinct pointer, Vec returns dangling) — this matters only if the
  caller compares pointer values.

---

## 5. Post-rewrite self-check

- [ ] No manual `libc::malloc` / `calloc` / `realloc` / `free` remains
      for the rewritten pair.
- [ ] Every `unsafe { }` introduced (only for aligned alloc / FFI
      pointer handoff) carries a `// SAFETY:` comment.
- [ ] Early-return paths and `?` operators still leave the buffer to
      drop naturally (no leak).
- [ ] `cargo check` passes; the fn compiles with no new `extern "C"`
      references except those already present.
