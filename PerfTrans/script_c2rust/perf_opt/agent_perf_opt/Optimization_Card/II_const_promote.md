# Rule II_const — static → const promotion

## 1. What you're fixing

c2rust translates C's `#define UPPER 12345`, `enum { UPPER = 12345 }`,
or `static const T UPPER = ...` uniformly into Rust `static [mut] UPPER: T = <literal>;`.
The `static` binding forces the compiler to keep a real memory address:
every read compiles to a `load` (or, under `static mut`, a `unsafe { load }`
plus a warning), instead of being inlined as an immediate operand. On a
per-iteration hot path — hash mixing, hash-chain multipliers, CRC
constants, bit-reversal tables — this **prevents constant-folding**,
adds one dependent load per use, and inflates instruction count as well
as memory traffic.

The fix is to change the declaration from `static` to `const`. `const`
has no address; every use is inlined by `rustc` at name-resolution time,
letting LLVM perform constant folding across the expression tree.
The value written in the source is bit-for-bit identical, so there is
no observable semantic change.

## 2. Rewrite template

Rewrite the declaration site only. All read sites are unchanged — the
name resolves the same way from the caller's perspective, only the
lookup mechanism changes.

**Before**
```rust
static     PRIME32_1: u32 = 2654435761u32;      // or:
static mut PRIME32_1: u32 = 2654435761 as u32;
```

**After**
```rust
pub const  PRIME32_1: u32 = 2654435761u32;
```

Notes:
- Drop `mut`. `const` cannot be mutable.
- Add `pub` at whatever visibility the original had (`pub static`,
  module-private `static`, etc. — preserve the same visibility).
- Preserve the original literal exactly (including type suffix / `as`
  cast). Rely on LLVM to fold `1234 as u32` at codegen.
- If the declaration was `static mut` with a `#[used]` / `#[link_section]`
  attribute, or if the address is taken (`&PRIME32_1`, `&raw const PRIME32_1`,
  `ptr::addr_of!(PRIME32_1)`), abstain — `const` has no address so those
  patterns break.

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The value is immutable throughout all uses.** Recheck all direct,
  aliased, and transitive writes, including external callers where relevant;
  a local search or detector result alone is not the proof.
- [ ] **The initializer is a valid constant expression with the same value.**
  Keep this template to literal scalar values and supported trivial casts;
  exclude runtime allocation, side effects, atomics, and interior mutability.
- [ ] **Storage identity is unobservable.** No address-taking, pointer
  comparison, FFI/exported symbol, or other use depends on the static's
  single stable storage location. Audit all readers before replacing it
  with an inlined constant.

---

## 4. When to abstain

- The static is `#[used]` / `#[link_section]` / `#[no_mangle]` — has
  ABI-visible linkage that `const` cannot express.
- The static's value depends on a target-dependent constant that
  requires `#[cfg(...)]` — safer to keep `static` for the toolchain
  agility.
- The RHS uses `Cell<T>` / `AtomicX::new(...)` / `LazyLock` —
  interior-mutable containers where the `static` binding is load-bearing.
- The identifier appears in a `macro_rules!` context that stringifies
  it or takes its address — safer to keep unchanged.
- Address is taken anywhere in the crate (see §3).

## 5. Post-rewrite self-check

- [ ] Only the declaration site is edited; all read sites unchanged.
- [ ] `static [mut]` replaced with `const` (never `static const`; that
      is C syntax and does not exist in Rust).
- [ ] Visibility (`pub` or module-private) preserved.
- [ ] The literal value is bit-for-bit identical (same type, same
      numeric value, same `as` cast, no arithmetic rewrite).
- [ ] `cargo build --release` compiles without new warnings; no
      `unused` warnings on the promoted identifier (const items unused
      warn just like static ones).
- [ ] For each rewritten identifier: `rg -F "PRIME32_1" src/` shows the
      same set of use sites; no address-taking pattern appears.
