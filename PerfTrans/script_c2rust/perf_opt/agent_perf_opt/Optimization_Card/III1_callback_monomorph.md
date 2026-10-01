# Rule III①: Callback → Generic Monomorphization

## 1. What you're fixing

The function contains a C-style function-pointer callback that c2rust
preserved as `Option<unsafe extern "C" fn(...)>`. Every call site becomes
an **indirect call** through a runtime-loaded pointer, which blocks LLVM
from inlining the callee, propagating constants across the boundary, or
vectorizing the loop that contains the call.

Three shapes (the driver tells you which):

- **Param shape** — the callback is a function parameter:
  `fn parse(input: *const u8, len: usize, cb: Option<unsafe extern "C" fn(...)>, data: *mut c_void)`

- **Field shape** — the callback lives in a struct field:
  `struct Allocator { alloc_fn: Option<unsafe extern "C" fn(size_t) -> *mut c_void>, ... }`

- **Static shape** — the callback lives in a module-level `static mut`:
  `pub static mut xmlFree: Option<unsafe extern "C" fn(*mut c_void)> = None;`

All three share the same call-site tell:

```rust
f.expect("non-null function pointer")(args)    // canonical c2rust form
// or
if let Some(cb) = f { cb(args) }                // guarded form
```

**Direction**: turn the runtime function pointer into a **generic type
parameter** with a `Fn` / `FnMut` / `FnOnce` bound, so the compiler
monomorphizes each call site into a direct, inlineable call. Struct and
static shapes have a fallback: drop `Option` and hard-code a default
function pointer.

---

## 2. Rewrite templates

### 2.1 Param shape → generic parameter

```rust
// BEFORE
unsafe extern "C" fn csv_parse(
    input: *const c_char, len: size_t,
    cb1: Option<unsafe extern "C" fn(*mut c_void, size_t, *mut c_void)>,
    cb2: Option<unsafe extern "C" fn(i32, *mut c_void)>,
    data: *mut c_void,
) -> size_t {
    for i in 0..len {
        if is_field_end(*input.offset(i as isize)) {
            if let Some(cb) = cb1 { cb(field_ptr, field_len, data); }
        }
    }
    len
}

// AFTER
fn csv_parse<D, CB1, CB2>(
    input: &[u8],
    mut cb1: Option<CB1>,
    mut cb2: Option<CB2>,
    data: &mut D,
) -> usize
where
    CB1: FnMut(&[u8], &mut D),
    CB2: FnMut(usize, &mut D),
{
    for (i, &c) in input.iter().enumerate() {
        if is_field_end(c) {
            if let Some(cb) = cb1.as_mut() { cb(field_slice, data); }
        }
    }
    input.len()
}
```

Three co-changes:

1. Each callback param `Option<unsafe extern "C" fn(...)>` becomes an
   `Option<CB>` with `CB: FnMut(...)` in the where-clause.
2. The `*mut c_void` "user data" param becomes a new generic `D` with
   `data: &mut D` — this preserves C's type erasure while restoring type
   information at each caller.
3. `*const c_char + len` inputs become `&[u8]` (optional but idiomatic).

### 2.2 Field shape — generic-parameterize the struct

```rust
// BEFORE
#[repr(C)]
pub struct Allocator {
    pub alloc_fn: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub free_fn:  Option<unsafe extern "C" fn(*mut c_void)>,
    pub user:     *mut c_void,
}

// AFTER — Option 1: fully generic (preferred when no FFI boundary)
pub struct Allocator<AF, FF, D>
where AF: Fn(usize) -> *mut u8, FF: Fn(*mut u8),
{
    pub alloc_fn: AF,
    pub free_fn:  FF,
    pub user:     D,
}
```

If the struct must remain `#[repr(C)]` (crosses an FFI boundary or is
transmuted), use **Option 2**: drop `Option`, hard-code default fn
pointers.

```rust
// AFTER — Option 2: keep #[repr(C)], drop Option
#[repr(C)]
pub struct Allocator {
    pub alloc_fn: unsafe extern "C" fn(size_t) -> *mut c_void,
    pub free_fn:  unsafe extern "C" fn(*mut c_void),
    pub user:     *mut c_void,
}
unsafe extern "C" fn default_alloc(sz: size_t) -> *mut c_void { libc::malloc(sz) }
unsafe extern "C" fn default_free(p: *mut c_void)             { libc::free(p) }

impl Allocator {
    pub fn new() -> Self {
        Self { alloc_fn: default_alloc, free_fn: default_free, user: ptr::null_mut() }
    }
}
```

Option 2 does not fully devirtualize (the pointer is still loaded from
the struct at runtime), but it drops the `Option` unwrap and lets LLVM
devirtualize whenever it can constant-propagate the struct instance.

### 2.3 Static shape — drop `Option`, initialize with a default fn

```rust
// BEFORE
pub static mut xmlFree: Option<unsafe extern "C" fn(*mut c_void)> = None;

pub unsafe extern "C" fn xmlXPathFreeObject(obj: *mut xmlXPathObject) {
    if !obj.is_null() {
        xmlFree.expect("non-null function pointer")(obj as *mut c_void);
    }
}

// AFTER
pub static mut xmlFree: unsafe extern "C" fn(*mut c_void) = default_xml_free;

unsafe extern "C" fn default_xml_free(p: *mut c_void) { libc::free(p) }

pub unsafe extern "C" fn xmlXPathFreeObject(obj: *mut xmlXPathObject) {
    if !obj.is_null() {
        xmlFree(obj as *mut c_void);           // direct call, no unwrap
    }
}
```

Every setter (`xmlFree = Some(f)`) must be updated to plain assignment
(`xmlFree = f`).

---

### 2.E — Form E: the callback is handed to a higher-order libc function

The other templates rewrite an indirect call made *inside* the function.
Here the function makes none: it hands a comparator to `qsort` (or
`bsearch`, `lfind`, `tsearch`, …) and libc performs one C ABI indirect call
**per element comparison**, which no inliner can reach across.

```rust
// BEFORE — c2rust's literal rendering of C's qsort
qsort(
    base as *mut c_void,
    n as size_t,
    ::core::mem::size_of::<Elem>() as size_t,
    Some(Cmp as unsafe extern "C" fn(*const c_void, *const c_void) -> c_int),
);

unsafe extern "C" fn Cmp(a: *const c_void, b: *const c_void) -> c_int {
    (*(a as *const Elem)).key.wrapping_sub((*(b as *const Elem)).key) as c_int
}

// AFTER — native sort, comparator inlines
{
    // SAFETY: `base` points to `n` initialised `Elem`s; the slice is used
    // only for this sort and does not outlive the call.
    let items = unsafe { ::core::slice::from_raw_parts_mut(base, n as usize) };
    items.sort_unstable_by(|a, b| a.key.cmp(&b.key));
}
```

**Proving the order is unchanged** — do this before rewriting, and state it
in the SAFETY comment:

1. Read the comparator body and identify the ordering key and direction.
   `a.key - b.key` is ascending by `key`; `b.key - a.key` is descending.
2. A subtraction-based comparator is only a valid total order when the
   subtraction cannot overflow. If the key can span the full integer range,
   establish a bound from the actual program context before replacing it
   with direct key comparison (`a.key.cmp(&b.key)`). Do not assume that the
   original subtraction had the intended ordering when it can overflow.
3. Multi-key comparators map to `.then_with(|| …)`, in the same order the
   C code tests them.
4. Both sorts are unstable, but this does not guarantee the same permutation
   of equal-key elements. Prove that ties are indistinguishable to consumers
   or that the comparator gives distinct observable elements a unique order.
   Also establish that comparator call count/order has no observable effects.
5. `bsearch` maps to `binary_search_by` with the *same* comparator
   direction; note that it returns `Result<usize, usize>`, not a pointer.

**When it pays.** The gain tracks one quantity: the share of the running
operation's self time held by the C sort routine together with the comparator
it reaches through the function pointer. Where that share dominates, this is
among the largest single-site wins available anywhere in the rule set; where
the same call site sits inside an operation that barely sorts, the identical
rewrite moves almost nothing. One call site is routinely both, depending on
which operation is running — so read the share off the profile for the
operation you are being judged on, not off the function's name.

The ordering key must come out unchanged, and the golden replay must stay
exact: a comparator that is not a total order will still sort, just into a
different permutation, and that is a silent W1 failure waiting for an input
with ties.

**Abstain** if the element type is not `Sized` or the length argument is not
provably the element count (`size` argument must equal
`size_of::<Elem>()`), or if the comparator reads state outside its two
arguments — a captured-context comparator is `qsort_r`, and its context
must be moved into the closure explicitly.

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The callback and environment retain their meaning.** Establish
  the selected target, arguments, return value, captured/user-data state,
  mutation, and call order for every affected call site.
- [ ] **All declarations, callers, constructors, setters, and readers are
  compatible with the new shape.** Audit parameter types, generic bounds,
  field uses, and global setters together. A callback that can change must
  not silently become a fixed target.
- [ ] **Removing `Option` does not remove reachable behavior.** Every use
  supplies `Some`; no initialization protocol, sentinel check, or reachable
  failure depends on `None` or `expect`.
- [ ] **Borrowing and lifetimes of captured data are valid.** Establish
  exclusivity for mutable captures and valid context/pointer lifetimes;
  closure wrapping alone does not justify creating a reference.
- [ ] **ABI, layout, and ownership contracts are preserved.** Do not change
  a public/FFI representation or calling convention without covering every
  consumer. Preserve allocator/free pairing and cleanup behavior.

---

## 4. When to abstain

Skip the rewrite if any of these hold:

- The function is called from **real C code** (`#[no_mangle] pub extern "C"`
  and part of a published C ABI) — C cannot call a generic Rust fn.
- The struct is passed to real C code with `#[repr(C)]` layout required
  — Option 1 breaks; Option 2 also breaks if C-side depends on the
  `Option<fn>` layout exactly.
- The caller list is unavailable from the EvidencePack — never guess
  which callers exist.
- Distinct instantiation count exceeds ~20 — monomorphization bloats
  `.text` past icache, often net-negative.
- The callback is looked up at runtime from a `HashMap<K, fn>` or
  similar plugin-style registry — a `HashMap` can only hold one
  concrete `F` type; abstain (or switch the whole design to an enum
  dispatch, which is a bigger refactor).
- The callee is defined in `std` or a third-party dependency you
  cannot edit — abstain.

---

## 5. Post-rewrite self-check

- [ ] Every touched caller / setter / field-read compiles.
- [ ] No `.expect("non-null function pointer")` remains in the fn body.
- [ ] No new `unsafe { }` block was introduced by the rewrite (the
      generic form is safe Rust).
- [ ] `cargo check` and `cargo build --release` pass.
- [ ] `cargo bloat --release -n 20` shows `.text` growth within ±10 %
      (log the delta if larger — bloat can slow things down).
