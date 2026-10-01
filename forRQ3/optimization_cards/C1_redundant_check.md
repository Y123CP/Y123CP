# Rule C1: Redundant Check Branches

## 1. What you're fixing

Rust semantics oblige the compiler to emit runtime **bounds check**,
**unwrap check**, and **division-by-zero check** for indexing,
unwrapping, and division. When LLVM cannot prove the check statically
eliminable, it survives in the optimized IR as a **conditional branch
plus a cold panic block** (`call panic_bounds_check` / `expect_failed` /
`slice_*_fail` / `panic_const_div_by_zero` + `unreachable`). Every such
survivor is one extra branch per hit and blocks vectorization inside
the containing loop.

Four site shapes (the driver tells you the count via
`class_i_hits.C1: N`; classify each source line yourself):

- **Bracket indexing**: `arr[i] = ...` / `let v = arr[i];`
- **Option/Result unwrap**: `opt.unwrap()` / `opt.expect(msg)` /
  `opt.expect("non-null function pointer")(...)` (c2rust idiom for
  callback fields — often combined with III①)
- **Slice range**: `&slice[a..b]` / `&slice[a..]` / `&slice[..b]`
- **Division / modulo by runtime denominator**: `a / b` / `a % b`
  where `b` is not a compile-time nonzero constant

**Direction**: prove the precondition, then replace the checked
operation with its `_unchecked` variant; the branch and panic block
disappear from codegen.

---

## 2. Rewrite templates

### 2.1 Bracket indexing → `get_unchecked` / `get_unchecked_mut`

```rust
// BEFORE
let v = arr[i];
arr[i] = x;

// AFTER
// SAFETY: i < arr.len() by <loop bound / preceding check / caller contract>
let v = unsafe { *arr.get_unchecked(i) };
unsafe { *arr.get_unchecked_mut(i) = x; }
```

### 2.2 Option/Result unwrap → `unwrap_unchecked`

```rust
// BEFORE
let x = opt.expect("non-null function pointer");
let y = result.unwrap();

// AFTER
// SAFETY: opt is Some (see <init site / preceding check>)
let x = unsafe { opt.unwrap_unchecked() };
let y = unsafe { result.unwrap_unchecked() };
```

**Stronger rewrite for c2rust `Option<extern "C" fn>` fields**: change
the field type to `extern "C" fn` (drop the `Option`) and initialize
with a real default fn — this removes the check permanently and enables
direct call + inlining. See `III1_callback_monomorph.md` for the full
pattern; only viable when a valid non-null default exists and every
setter can be updated.

### 2.3 Slice range → `get_unchecked(a..b)`

```rust
// BEFORE
let sub = &slice[a..b];

// AFTER
// SAFETY: a <= b and b <= slice.len()
let sub = unsafe { slice.get_unchecked(a..b) };
// use get_unchecked_mut for &mut slice[a..b]
```

For `slice[a..]` prove `a <= slice.len()`; for `slice[..b]` prove
`b <= slice.len()`.

### 2.4 Division / modulo — prove nonzero, don't rewrite the operator

```rust
// BEFORE — LLVM couldn't infer b != 0
let q = a / b;

// AFTER — hint style (preferred): let LLVM eliminate the check
debug_assert!(b != 0);
let q = a / b;
```

Do **not** reach for `core::intrinsics::unchecked_div` — it is nightly-
only. Prefer restructuring so LLVM can prove `b != 0` (e.g. use
`NonZeroU32::get()` at the source, or add an early `if b == 0 { return; }`).

For signed division: also rule out `b == -1 && a == iN::MIN` (overflow).

---

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

- [ ] **The removed check always succeeds at its original site.** For an
  index, prove `i < len`; for a range, prove `a <= b <= len`. Use loop
  induction, a dominating guard, a compile-time bound, or an established
  caller contract that covers every reachable call site.
- [ ] **The invariant holds on every reaching path.** Account for changes
  to indices, lengths, and backing storage, including changes by callees.
- [ ] **Unchecked unwraps receive the required variant.** Every path to
  the site supplies `Some` or `Ok`, as appropriate.
- [ ] **Arithmetic checks are redundant.** A removed division check needs
  a nonzero divisor; signed division also excludes `MIN / -1`. Preserve
  the original arithmetic and overflow behavior.
- [ ] **Only unreachable failures are removed.** Preserve evaluation order,
  side effects, and all failures reachable under the original contract.
  Keep each unsafe block minimal and state the site-specific proof in a
  `// SAFETY:` comment.

---

## 4. When to abstain

Skip the site if any of these hold:

- **The index is already provable** — a constant, a mask against a constant,
  or an induction variable under a fixed length. The check is folded away
  already; removing it in source buys nothing.
  Not a reason to skip: the site being outside a loop. Removing a check also
  un-splits the basic block, and that is paid per execution.
- The value bounding the index / divisor comes from **external input**
  without validation (untrusted JSON, IO, syscall return).
- The slice / Vec may be **mutated between the check and the use**
  (`push` / `truncate` / `clear` on `Vec`; borrow-checker won't help
  through raw pointers).
- The `expect(msg)` uses a **custom message** (not c2rust's fixed
  `"non-null function pointer"`) — the intent is likely an intentional
  runtime assert, not a c2rust artifact.
- The `Option<T>` field is mutated across multiple modules / crates and
  the setter set cannot be fully audited.
- Removing the check would require expanding an `unsafe` block to
  cover unrelated operations — keep `unsafe` minimal.

Per-site abstain is fine; a partial rewrite is safer than a wrong one.
**When in doubt, propose it and let W2 decide** — a measurement can tell a
free check from a costly one, and this card cannot.

---

## 5. Post-rewrite self-check

- [ ] Every rewritten site carries a `// SAFETY:` comment naming the
      proven precondition and the evidence line.
- [ ] Each `unsafe { }` block wraps only the specific unchecked call.
- [ ] The function's public signature is unchanged.
- [ ] `cargo check` and `cargo build --release` pass.
