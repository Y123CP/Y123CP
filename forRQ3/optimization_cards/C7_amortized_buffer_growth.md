# Rule C7: Amortized Buffer Growth

## 1. What you're fixing

The hot function grows a heap buffer by **reallocating to its exact new
length on every append**. The canonical c2rust shape is a `realloc`
whose size is the current length plus a small constant:

```rust
// append one char (grows by 1 element + NUL each call)
let new = realloc((*val).d as *mut c_void,
                  (*val).l.wrapping_add(2 as size_t)) as *mut c_char;
// ... write at (*val).l, then (*val).l += 1 ...
(*val).d = new;
```

Because the size is `len + k` (k a small literal), **each append
reallocates and copies the whole buffer**. Building an `n`-byte result
one piece at a time is therefore `Θ(n²)` in reallocation and copying.
The struct usually stores only a length + data pointer and **no
capacity**, so the exact-size `realloc` runs unconditionally.

This is the append engine behind string/list serialization
(`..._to_value`, `append_char`/`append_string`), token accumulation, and
any incrementally built output buffer.

**Not this rule** — a buffer that already grows *geometrically* under a
capacity guard is fine and must be left alone:

```rust
if list.c == list.cap {                 // capacity check
    cap = cap + cap / 2;                // geometric growth
    realloc(list.v, size_of::<T>() * cap);
}
```

Here the size is `elem * cap` (a factor of capacity), not `len + k`, so
it is already amortized `Θ(n)`. The detector only flags the `len + small
constant` shape.

**Direction**: give the buffer **amortized capacity** so `realloc`
runs `Θ(log n)` times instead of `Θ(n)`, turning construction from
`Θ(n²)` into amortized `Θ(n)`. Do not change *what* bytes are written —
only *when* the allocation grows. Output must stay byte-identical.

---

## 2. Rewrite templates

### 2.1 Buffer is a shared struct field (no ripple) — track capacity in place

When the buffer pointer is a struct field used across many functions,
converting it to a Rust container would ripple through every use and the
`free` path. Instead, recover amortized growth **without changing the
struct**, but only if the allocator contract permits using the queried
extent as writable capacity (§3). The usual `malloc_usable_size` contract
does not grant that permission; otherwise track explicitly requested capacity
within the allowed edit scope or abstain. This conditional edit
is **fully region-local**: declare `malloc_usable_size` in a
**function-local `extern` block** (Rust allows an `extern` block inside a
function body) so nothing outside the function changes — do NOT rely on a
file-level extern or a `libc` dependency, which the region edit cannot add:

```rust
pub unsafe extern "C" fn append_char(val: *mut Value, ch: c_char) -> c_int {
    extern "C" { fn malloc_usable_size(p: *mut c_void) -> size_t; }  // function-local

    // BEFORE: realloc every append
    //   let new = realloc((*val).d, (*val).l.wrapping_add(2 as size_t)) as *mut c_char;

    // AFTER: realloc only when the current block is too small; grow ~1.5x
    let need: size_t = (*val).l.wrapping_add(2 as size_t);
    let have: size_t = if (*val).d.is_null() { 0 }
        else { malloc_usable_size((*val).d as *mut c_void) };
    let new = if need <= have {
        (*val).d                                   // room already — NO realloc call
    } else {
        let mut cap = have.wrapping_add(have.wrapping_div(2));  // 1.5x
        if cap < need { cap = need; }
        if cap < 32 { cap = 32; }
        realloc((*val).d as *mut c_void, cap) as *mut c_char
    };
    // ... unchanged: write ch at (*val).l, NUL at (*val).l+1, (*val).l += 1 ...
}
```

The win comes from **skipping the `realloc` call entirely** when the block
already has room — that is what removes the `Θ(n²)` copying. Merely rounding
the `realloc` size up (e.g. `.next_power_of_two()`) does **not** help: it
still calls `realloc` on every append, and measured only ~-3%. You must
gate the call behind `need <= have`.

`free((*val).d)` is unchanged — `free` ignores the requested size. The
logical length field (`l`) and the bytes written must remain unchanged;
this depends on the allocation, bounds, and failure-handling conditions in §3.

Apply the same shape to sibling appenders (`append_string`, `append_val`):
compute `need = len + added + 1`, reuse the `malloc_usable_size` guard.

### 2.2 Buffer is function-local — use a Rust container

When the growable buffer is created and consumed within one function
(not stored in a long-lived struct), replace the raw pointer with `Vec`
/ `String`, which amortize natively:

```rust
// BEFORE: raw ptr + realloc-per-append
// AFTER:
let mut buf: Vec<u8> = Vec::new();
buf.reserve(hint);           // optional: pre-size if the total is known
buf.push(byte);              // amortized growth, no manual realloc/free
// pass buf.as_mut_ptr()/buf.len() to a C callee if needed; Vec drops = free
```

`Vec::with_capacity` / `Vec::reserve` set capacity; `push` / `extend_from_slice`
append with amortized `Θ(1)`. This also removes the manual `free`.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **Logical contents are unchanged.** Preserve written bytes, logical
  lengths, and terminator writes; spare capacity is never exposed as data.
- [ ] **Capacity is established from a valid allocation contract.** Track
  explicitly allocated capacity, or establish that the allocator/platform
  contract permits both the capacity query and writes to the reported
  extent. A usable-size query alone is not permission to write beyond the
  requested allocation; otherwise use explicit capacity or abstain.
- [ ] **Growth arithmetic and writes stay in bounds.** Account for element
  size, added length, terminators, and overflow; every successful allocation
  must cover the required extent.
- [ ] **Ownership and allocation-failure handling are preserved.** Match
  allocation and deallocation APIs, keep the old allocation live when a
  reallocation fails, and preserve the specified error/cleanup behavior.
  Check whether allocation timing or capacity is observable by the caller.
- [ ] **No stale pointer survives a possible reallocation.** Update all
  affected references and pointers; a local Rust container must not escape
  its lifetime or be freed by a C callee.

### Editing and performance constraints

- Preserve the function signature, ABI, and surrounding declarations within
  the driver's edit contract; do not add `#[inline]`. Removing an existing
  inline attribute is a separate measured choice only when the edit permits it.
- Use the pinned toolchain's supported local `extern "C" { ... }` syntax.
- Leave already-geometric, guarded growth unchanged. A shared struct field
  needs an ownership/capacity argument covering all users; a function-local
  buffer can use a Rust container when the conditions above hold.
- Functional and performance gates remain required; neither an allocation
  strategy nor an attribute change guarantees a speedup.

---

