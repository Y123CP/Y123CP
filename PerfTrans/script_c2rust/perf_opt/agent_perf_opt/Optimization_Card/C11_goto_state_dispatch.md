# Rule C11: Goto State-Machine Dispatch

## 1. What you're fixing

C has `goto`; Rust does not. The translator lowers a function's gotos into a
state machine — a local holding an opaque 64-bit label, plus `match` blocks
routing to whatever each goto targeted:

```rust
let mut current_block: u64;
current_block = 12147880666119273379;
loop {
    match current_block {
        12147880666119273379 => { /* the goto target */ }
        8038949400865391589  => { break; }
        _ => { }
    }
    // more `match current_block` blocks, in sequence
}
```

The C paid one `jmp` per goto. This pays a store, a re-entry, and a chain of
comparisons — and the labels are random 64-bit values, so each comparison needs
its own 10-byte `movabs` to materialise the immediate.

The variable name is the translator's, not the program's: `current_block`, or
`current_block_<n>` when one function needs several. One function may declare
more than one; they are independent state machines.

## 2. Two costs, two rewrites

Pick by which cost the function actually carries. They are not cumulative —
once either one takes the dispatch off the hot path, the other has nothing left
to remove.

### 2.1 Narrow the labels (`goto-dispatch-wide-constants`)

Renumber the labels of one variable to small dense integers and narrow the
carrier's type. A comparison then uses a short immediate instead of a
`movabs`.

```rust
let mut current_block: u32;          // was u64
current_block = 0;                   // was 12147880666119273379
match current_block {
    0 => { }                         // was 12147880666119273379
    1 => { break; }                  // was 8038949400865391589
    _ => { }
}
```

**No control flow moves.** The rewrite is a bijection applied to every read and
every write of the same variable, so it preserves values by construction.

Tooling applies this deterministic mapping. Its applicability still depends
on the label-use conditions in §3.1.

### 2.2 Split the merged loop (`goto-dispatch-loop-head`)

When a dispatch is the *first statement of a loop body*, the loop's own back
edge re-runs it every iteration. The translator merges what were separate C
loops into one `loop { match .. }`; give the hot arm its own loop again:

```rust
'outer: loop {
    if current_block == HOT {
        'fast: loop {
            /* the hot arm's body, verbatim */
            //   `current_block = HOT; continue;`  ->  `continue 'fast;`
            //   `current_block = ERR; break;`     ->  `current_block = ERR; break 'outer;`
            break 'fast;      // falling off the end leaves the loop, exactly
                              // as falling off the arm fell through before
        }
    } else {
        /* the other arm's body, verbatim */
    }
    /* everything after the dispatch, unchanged */
}
```

The trailing `break 'fast;` is what makes this safe without reasoning about
which states can be live at the exit: it reproduces the original "fall off the
end of the arm and continue past the dispatch" edge exactly.

## 3. Rewrite Preconditions

Establish every applicable item below from the source region, enclosing
function, and relevant type, global, function, and project-local call-site
context. A pattern match alone does not establish these conditions. Exclude
this card if a required condition is unmet or unresolved; skip the region if
no applicable card remains. Coordinate overlapping directions in one rewrite.
Build, functional, and performance checks follow this assessment and do not
replace it.

### 3.1 Label narrowing (§2.1)

- [ ] **The label mapping is a bijection over all reachable states.** Every
  read, write, comparison, and match of this variable uses the same mapping;
  the narrowed type represents every mapped value.
- [ ] **Numeric label values are not externally observable.** Rule out
  arithmetic on labels, escaping values, or aliases that bypass the mapping.
  Other state-machine variables retain their own independent mappings.

### 3.2 Loop splitting (§2.2)

- [ ] **The dispatch heads the loop and is reevaluated on each outer
  iteration.** Keep the dispatch at its original position; preserve the
  fallback arm and code following the dispatch.
- [ ] **Every relabeled jump targets the original state-machine loop.**
  Resolve nested loops and existing labels; do not redirect their jumps.
- [ ] **A fast-loop back edge returns to the same hot state.** Use
  `continue 'fast` only when every reaching path establishes that state;
  otherwise retain dispatch through `continue 'outer`.
- [ ] **Fallthrough, exits, and effects are preserved.** Preserve the arm's
  own back edges, leave nested dispatches in place, and reproduce the
  original fallthrough and break targets without moving observable work.

---

## 4. When to abstain

- **Fewer than two labels** for a variable: that is a flag, not a dispatch.
- **The labels already fit a 32-bit immediate**: no `movabs` exists to remove,
  so 2.1 changes bytes and buys nothing.
- **No dispatch heads a loop**: 2.2 has no target. A dispatch further down the
  body is paid once per pass through that point — which is what the C `goto`
  paid too, so there is nothing to recover.
- **The arm you would hoist contains a `break` or `continue` whose target you
  cannot establish**: abstain rather than guess. A mis-targeted branch is not
  something the correctness gate reliably catches.

## 5. What decides whether it pays

Nothing in the source does. The backend already threads the dispatch away in
some functions and not others, and the same rewrite that moves one function
measurably leaves another bit-identical in instruction count. The measurement
gate decides; propose the rewrite, do not predict its size.
