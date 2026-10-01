"""Prompt builders + LLM-output parsers for the harness-gen agent.

All prompts state the same fixed harness contract; the deterministic
gates in gates.py verify exactly what the contract promises, so a gate
failure message can be fed back verbatim as a repair instruction.
"""

from __future__ import annotations

import json
import re
from typing import Any

from .inventory import CrateInventory

# ────────────────────────────────────────────────────────────────────
# The harness contract (single source of truth, embedded in every prompt)
# ────────────────────────────────────────────────────────────────────

CONTRACT = """\
HARNESS CONTRACT (the gates verify every point below mechanically):

You write ONLY `src/lib.rs`. A fixed, pre-generated `src/main.rs` CLI
shell (and separate fuzz targets) call into it, so lib.rs MUST export
exactly this interface:

  /// (op_name, runner) registry — one entry per operation.
  pub fn ops() -> Vec<(&'static str, fn(&[u8], u64) -> String)>

  /// Write, for EVERY operation with "needs_input": true, exactly two
  /// DISTINCT, VALID input files: <dir>/<op>.1.bin and <dir>/<op>.2.bin
  /// (deterministic bytes — same files on every run). You may call the
  /// library itself to synthesize them (e.g. use the encoder to produce
  /// inputs for a decode operation). Return one line per file:
  /// `seed <op> <path>`.
  pub fn gen_seeds(dir: &str) -> Vec<String>

  /// Write, for each operation whose work SCALES with input size, ONE input
  /// `<dir>/<op>.perf.bin` that is LARGE and REALISTIC enough that the
  /// operation's inner loop — not per-call setup/malloc/memset/libc —
  /// dominates a profiled run (the library's own code should account for the
  /// bulk of the time). This usually needs SEVERAL MEGABYTES, not a few KB.
  ///
  /// The data MUST be REALISTIC for the operation, NOT random/uniform bytes.
  /// Random or degenerate data makes many kernels take a trivial path and stay
  /// cold: compressors/matchers find no repeats, parsers/decoders hit an early
  /// error and bail, searches find nothing. Generate data with the STRUCTURE
  /// and REDUNDANCY that real inputs of this domain have (repeated substrings,
  /// valid records/tokens, realistic value distributions) so the hot loop runs
  /// to completion over the whole input.
  ///
  /// The input MUST be a VALID input the operation processes end-to-end — one
  /// that is accepted, not rejected at a format/length/validation gate (a
  /// rejected input measures error handling, not the kernel). For an operation
  /// whose input is an ENCODED/COMPRESSED/PACKED blob (decode/decompress/
  /// verify/inverse), PRODUCE that blob by first running the FORWARD direction
  /// (the library's own encoder/compressor) over a large realistic buffer,
  /// then feed the result — do NOT hand a decoder raw or random bytes.
  ///
  /// Rules of thumb by category: text/format parsers → a large valid document
  /// with many records; codecs/compressors → a big buffer of realistic,
  /// compressible content; hashers → a big realistic data block; decoders →
  /// the encoder's output over such content; matrix/DP/search kernels → large
  /// dimensions / a long realistic haystack. Deterministic bytes.
  /// SIZE: aim for HUNDREDS OF KILOBYTES TO A FEW MEGABYTES — large enough
  /// that the kernel, not per-call setup, dominates one call. Do NOT emit
  /// tens of megabytes: the driver auto-tunes the iteration count to a fixed
  /// wall-clock budget, so a huge input buys no extra work — it only starves
  /// the iteration count (leaving any one-off per-run setup unamortized) and
  /// pushes the working set out of cache, turning the measurement into memory
  /// bandwidth. Prefer MORE RECORDS OF REALISTIC SIZE over one giant blob.
  /// OMIT an operation (leave it functional-only, write it no perf input)
  /// when EITHER of these holds — both make it a coverage op, not a perf
  /// target, and a perf input for it is a wasted measurement slot:
  ///   (a) its work does not scale with the input at all (config / option /
  ///       getter / free ops); or
  ///   (b) each individual library call is TRIVIAL — a predicate, validator,
  ///       classifier or single table/range lookup returning a bool or one
  ///       scalar (`xmlIsBaseChar`, `is_*`, `*_valid`, char-class sweeps).
  ///       Total work still grows with input, so (a) does not catch these,
  ///       but a call of a handful of instructions costs no more than the
  ///       loop's own counter and fold; at opt-level 3 the callee is inlined
  ///       and FUSED with that bookkeeping, leaving no library time that can
  ///       be separated from harness time, and nothing anyone actually runs
  ///       that way to optimize. Measured: such an op profiled at 2% library
  ///       even with full debug info and inline-aware attribution.
  /// A good perf op is one where ONE library call does a lot of work — parse
  /// a document, encode a buffer, build a tree, hash a stream.
  /// Return one line per file written: `perf <op> <path>`.
  pub fn gen_perf(dir: &str) -> Vec<String>

Each runner fn takes (input_bytes, iters) and RETURNS the digest line(s)
as a String (no trailing newline needed; the shell prints it).

Hard rules:
  1. DETERMINISM: identical (input, iters) → identical returned String.
     Never include pointers/addresses, time, PIDs, paths, or HashMap
     iteration order. Floating-point values must be folded into an
     integer checksum (e.g. to_bits()), never formatted as decimals.
     CRITICAL — a raw pointer VALUE is an address even when masked:
     NEVER `fold(digest, some_ptr as u64)` or `as usize & 0xFF`. A
     function returning a `*mut`/`*const` (e.g. lodepng_chunk_next) gives
     an address — fold a CONTENT-derived fact instead: whether it is null
     (`p.is_null() as u64`), or its OFFSET from a known base
     (`(p as usize).wrapping_sub(base as usize) as u64`). An address low
     byte looks stable within one build but changes across builds/hosts,
     silently rotting the golden oracle.
  2. INPUT-SENSITIVITY: the digest MUST depend on the input bytes
     (<op>.1.bin and <op>.2.bin must yield different digests).
  3. ERRORS ARE DATA — NEVER EXIT OR PANIC: these runners are also used
     as fuzz targets fed with arbitrary malformed bytes. When the
     library reports an error, fold the error code into the digest
     (e.g. return "op=decode err=83 digest=0") and return normally.
     No std::process::exit, no panics/unwrap on input-dependent values,
     no slice indexing that can go out of bounds on short inputs.
  4. ITERS: when iters > 1, re-run the operation each iteration and fold
     each iteration's digest into a running wrapping checksum; return
     the digest once at the end. Iterations must not accumulate state
     that changes results (re-init library state each iteration).
  5. THE LIBRARY MUST DOMINATE THE TIMED LOOP. The harness feeds inputs
     and folds outputs; it must never out-cost the call it measures. The
     loop body in rule 4 is a MEASUREMENT loop — everything in it is
     timed, so put nothing there that does not have to be recomputed.
     Concretely, inside `for _ in 0..iters`:
       a. Anything determined SOLELY by the input bytes (not by library
          state) is loop-invariant: compute it ONCE before the loop, bind
          it to a variable, and fold that saved value each iteration.
          This covers a checksum over the whole input, `windows()` /
          substring scans, and any length or format probing of the input.
       b. Never `clone()` a whole struct just to read a few fields off
          the copy — read the live value instead; a clone observes
          exactly the same values, so the digest is unchanged.
       c. Fold facts that are already computed (counts, sizes, error
          codes, a handful of scalar fields), never freshly recomputed
          O(input) work.
     Rule 4 still requires a per-iteration fold — hoisting changes only
     WHERE the value is computed, never the digest.
     An op whose profile is dominated by the harness's own symbols
     measures the harness rather than the library; it is flagged
     downstream as `harness_bound` in `perf_workload.json` and is useless
     as an optimization target.
     NEVER RE-IMPLEMENT LIBRARY LOGIC IN THE HARNESS. This has one
     tempting trigger, so it is spelled out: when a function you want is
     not reachable — `error[E0603]: function ... is private`, or it is
     simply absent — the fix is NOT to write your own copy of it. A local
     re-implementation compiles, passes smoke, and yields a stable digest,
     which is exactly why it slips through; but from then on the timed
     loop measures YOUR code and the op is worthless as a perf target.
     The allowed fixes, in order: call a PUBLIC function that reaches the
     same code path (an exported wrapper almost always exists — that is
     what the library's own users call), or DROP the operation from the
     spec. Building test INPUTS (documents, buffers, encodings) in the
     harness is fine and expected — what is banned is re-implementing the
     transformation the op claims to measure.
  6. Free / clean up library-allocated buffers with the library's own
     free functions where provided (also on error paths — no leaks per
     iteration, fuzzing runs millions of executions).
  7. Runner fns registered in ops() must be safe `fn` items (wrap the
     unsafe FFI work inside the function body).
  8. NON-ASCII TEST TEXT (two ways Rust differs from C here, both seen):
       a. `\\xNN` in a `&str` literal is limited to `\\x00-\\x7f` — writing
          `"Caf\\xe9"` is `error: out of range hex escape`, not a byte. For a
          `&str`, write the character itself ("Café") or `\\u{e9}`; `\\xNN`
          above 0x7f is legal ONLY in a byte string (`b"..."`) or byte
          escape context. To emit a specific NON-UTF-8 byte, build a
          `Vec<u8>` / `b"..."` — a `&str` cannot hold one at all.
       b. Byte indices are not character indices: `&s[..n]` on a `String`
          PANICS when `n` splits a multi-byte character ("byte index N is
          not a char boundary"). To truncate, work on `as_bytes()` /
          `Vec<u8>`, or use `char_indices()` to find a real boundary.
     Both bite exactly where non-ASCII belongs — encoding, markup and text
     corpora — so this applies to seeds, gen_seeds and gen_perf alike.
  9. Cross-module pointers need an INFERRED cast: `callee(p as *mut _)` or
     `p as *const _`. Never a named alias (`p as png_structrp` picks one
     module's type and fails the same way), never a transmute of the pointee.
     The TYPE SCOPING section above says which names this applies to.
"""

# Compact skeleton distilled from the hand-written libcsv harness in this
# repo (dataset_trans_process/libcsv/workloads/harness/csv_count_raw),
# restructured to the lib.rs registry contract.
FEWSHOT = """\
EXAMPLE SHAPE (for a DIFFERENT crate, libcsv — shows the expected style;
note MaybeUninit::zeroed for C structs, safe runner wrapping unsafe FFI,
error-as-data, checksum fold):

```rust
use std::fs;
use std::mem::MaybeUninit;
use libcsv_raw::src::libcsv::{csv_parser, csv_init, csv_parse, csv_fini, csv_free};

fn fold(acc: u64, x: u64) -> u64 { acc.rotate_left(1) ^ x }

pub fn op_parse(data: &[u8], iters: u64) -> String {
    let mut digest = 0u64;
    let mut last_err = 0i32;
    for _ in 0..iters {
        unsafe {
            let mut st = MaybeUninit::<csv_parser>::zeroed();
            let p = st.as_mut_ptr();
            csv_init(p, 0);
            // … feed `data` through csv_parse with counting callbacks;
            //   on parse error, record it and keep going (error is data) …
            last_err = 0; // e.g. csv_error(p)
            csv_fini(p, None, None, std::ptr::null_mut());
            csv_free(p);
        }
        digest = fold(digest, /* rows */ 0).wrapping_add(/* fields */ 0);
    }
    format!("op=parse err={} digest={:016x}", last_err, digest)
}

pub fn ops() -> Vec<(&'static str, fn(&[u8], u64) -> String)> {
    vec![("parse", op_parse as fn(&[u8], u64) -> String)]
}

pub fn gen_seeds(dir: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let s1 = b"a,b,c\\n1,2,3\\n".to_vec();          // deterministic bytes
    let s2 = b"x;y\\n\\\"q,uo\\\",z\\n7,8,9\\n".to_vec();
    for (op, idx, bytes) in [("parse", 1, &s1), ("parse", 2, &s2)] {
        let path = format!("{}/{}.{}.bin", dir, op, idx);
        fs::write(&path, bytes).unwrap();
        lines.push(format!("seed {} {}", op, path));
    }
    lines
}

pub fn gen_perf(dir: &str) -> Vec<String> {
    // one LARGE realistic input per perf-relevant op (here: a big CSV so
    // csv_parse's scanning loop dominates, not per-call setup)
    let mut big = Vec::new();
    for i in 0..200_000u32 { big.extend_from_slice(format!("f{},{},{}\\n", i, i*7, i^0x55).as_bytes()); }
    let path = format!("{}/parse.perf.bin", dir);
    fs::write(&path, &big).unwrap();
    vec![format!("perf parse {}", path)]
}
```
"""

PLAN_SYSTEM = """\
You are an expert Rust systems engineer. You design a minimal workload
harness for a Rust crate that was mechanically translated from C by
c2rust (all-unsafe, C-ABI functions, raw pointers). You answer with ONE
JSON object and nothing else."""

CODEGEN_SYSTEM = """\
You are an expert Rust systems engineer writing unsafe FFI-style Rust.
You output ONE complete `src/lib.rs` file in a single ```rust code
block and nothing else. The file must compile as the lib target of a
crate that path-depends on the crate under test; a fixed CLI shell and
fuzz targets call it through the `ops()` registry it exports."""


# Uniform c2rust codegen noise: every exported fn carries the same visibility /
# ABI prefix, every parameter a `mut` binding, every primitive its full path.
# Stripping it is lossless for the reader — the import shape and the types are
# unchanged — and buys room for more functions inside the same budget.
_SIG_NOISE = (
    (re.compile(r'\bpub\s+unsafe\s+extern\s+"C"\s+fn\s+'), "fn "),
    (re.compile(r'\bpub\s+extern\s+"C"\s+fn\s+'), "fn "),
    (re.compile(r"(?<![A-Za-z0-9_])(?:::)?(?:core|std)::(?:ffi|os::raw)::"), ""),
    (re.compile(r"\bmut\s+(?=[A-Za-z_][A-Za-z0-9_]*\s*:)"), ""),
    (re.compile(r"\s*\n\s*"), " "),
    (re.compile(r"\(\s+"), "("),
    (re.compile(r"\s+\)"), ")"),
    (re.compile(r",\s*\)"), ")"),
    (re.compile(r"\s{2,}"), " "),
)


def _compress_signature(sig: str) -> str:
    for pattern, repl in _SIG_NOISE:
        sig = pattern.sub(repl, sig)
    return sig.strip()


# The edition the generated Cargo.toml declares. Single source of truth: the
# manifest template formats this in, and the prompts state it. It was stated
# nowhere before, and a model that has to guess reaches for the newest syntax
# it knows — one run emitted `unsafe extern "C" { … }` (a 2024 form) into a
# 2021 crate, which fails at AST validation and burns a whole repair round on
# a fact we had all along.
HARNESS_EDITION = "2021"

_SIG_IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
_SCOPING_LIST_MAX = 24


def _type_scoping(inv: CrateInventory) -> str:
    """Which type names denote several incompatible types, and where each lives.

    This is a FACT the listing cannot carry. c2rust writes signatures
    unqualified, because within a module no qualification is needed:

        // pngset.rs
        pub type png_const_structrp = *const png_struct;
        pub unsafe extern "C" fn png_set_IHDR(png_ptr: png_const_structrp, …)

    23 other modules carry that same alias line VERBATIM, each resolving to
    their own `png_struct`. So the name in a signature is not enough to know
    the type, and nothing in the name hints that a choice even exists — which
    is why a caller reading only the listing writes E0308 after E0308.
    (Measured: 44 type errors in one generated harness of a vendored libpng;
    a crate with zero duplicated types needed zero repairs at all.)

    The saving grace is that the listing is already grouped by module, so one
    sentence — "a signature's types belong to that function's own module" —
    plus the list of ambiguous names is the whole fact. No per-type
    qualification needed, which would have cost far more characters than the
    entire section.
    """
    dupes = inv.duplicated_types
    if not dupes:
        return ""
    # Only names the model can actually run into: those in a signature it is
    # shown. A type duplicated 30 times that never appears in the API is not
    # this caller's problem, and listing it just dilutes the ones that are.
    in_sigs: set[str] = set()
    for fn in inv.fns:
        in_sigs.update(_SIG_IDENT.findall(fn.signature))
    # Sorted here as well as in the inventory: worst-first is a property of
    # this section, not something to inherit from whoever built the mapping.
    hot = sorted(((n, len(m)) for n, m in dupes.items() if n in in_sigs),
                 key=lambda nc: (-nc[1], nc[0]))
    if not hot:
        return ""

    shown = hot[:_SCOPING_LIST_MAX]
    names = ", ".join(f"{n} ({c})" for n, c in shown)
    more = (f", and {len(hot) - len(shown)} more" if len(hot) > len(shown)
            else "")
    return f"""
TYPE SCOPING (read before writing any call)
Every type named in a signature belongs to THAT function's own module — the
`## use …` path shown above its group. c2rust copies each C struct and typedef
into every module that mentions it, so the names below denote SEVERAL
INCOMPATIBLE types, one per module (copy count in parens):

  {names}{more}

Two consequences:
  * A value obtained from one module's function has THAT module's type. Handing
    it to another module's function is E0308 unless you cast it.
  * The aliases are duplicated too, so `p as png_structrp` does not bridge
    anything. Use the inferred form `p as *mut _` / `p as *const _`, which
    takes its target from the callee's signature.
"""


def _fn_listing(inv: CrateInventory, limit_chars: int = 60_000) -> str:
    """The API the harness may call, as much of it as the budget allows.

    Two properties matter more than compactness, and the naive "take functions
    in file order until full" had neither:

    * BREADTH. Truncating in order drops whole modules off the end of the
      list, and a module the model cannot see is a module it cannot drive —
      which caps coverage no matter how many extension rounds it is given.
      Measured on a 1603-function crate: 72% of the API was cut, all of it the
      tail, and coverage stalled at 10% of functions.
    * HONESTY. A model that is shown a partial list with no warning fills the
      gaps from memory of the upstream C library, and those guesses become
      `E0603 private` / `E0432 unresolved import` on names that were never in
      the crate. That failure appeared in every run of that crate and consumed
      the repair budget the coverage extension needed.

    So: compress first (often the whole API then fits), and if it still does
    not, round-robin across modules so every module is represented.
    """
    # Grouping by module also drops the per-line `[module]` prefix, which for a
    # large crate is a sixth of the budget spent restating the same path.
    by_module: dict[str, list[str]] = {}
    for fn in inv.fns:
        by_module.setdefault(fn.module_path, []).append(
            _compress_signature(fn.signature))

    def render(selected: dict[str, list[str]]) -> str:
        out = []
        for module, sigs in selected.items():
            if not sigs:
                continue
            out.append(f"## use {inv.crate_name}::{module}::<fn>;")
            out.extend(sigs)
        return "\n".join(out)

    # Stated on EVERY listing, not only a truncated one. A complete listing
    # used to carry no warning at all, and a model that recognises the upstream
    # C library then imports from memory: measured on a 359-function crate
    # (a vendored libpng) whose listing fits whole, three imports in the first
    # attempt named REAL functions under the WRONG module — `png_set_error_fn`
    # taken from `png` when the listing says `pngerror`, `png_data_freer` from
    # `pngset` when the listing says `png`. The name existing is not the
    # problem; the path is. So bind both.
    header = (
        "The `## use …` line above each group is the EXACT import path for"
        " every function in that group. Copy it verbatim.\n"
        "Do NOT place a function under a module you remember it from in the"
        " upstream C library — this crate's module split is c2rust's, not"
        " upstream's, and sibling modules here often hold functions the"
        " original header grouped together. Import ONLY names shown below,"
        " and ONLY from the group they are shown under.\n"
    )

    full = render(by_module)
    if len(full) <= limit_chars:
        return header + full

    # Round-robin so the budget is spread over modules instead of being spent
    # on whichever ones happen to come first.
    queues = {m: list(sigs) for m, sigs in by_module.items()}
    kept: dict[str, list[str]] = {m: [] for m in by_module}
    used = sum(len(f"## use {inv.crate_name}::{m}::<fn>;") + 1 for m in by_module)
    total = sum(len(sigs) for sigs in by_module.values())
    while any(queues.values()):
        progressed = False
        for module, queue in queues.items():
            if not queue:
                continue
            sig = queue[0]
            if used + len(sig) + 1 > limit_chars:
                queues = {m: [] for m in queues}
                break
            queue.pop(0)
            kept[module].append(sig)
            used += len(sig) + 1
            progressed = True
        if not progressed:
            break

    shown = sum(len(v) for v in kept.values())
    omitted = total - shown
    body = render(kept)
    if omitted:
        body += (
            f"\n\n… {omitted} of {total} functions omitted to fit this prompt,"
            f" taken evenly from all {len(by_module)} modules."
            " AN OMITTED FUNCTION STILL EXISTS in the crate — you simply have"
            " not been shown its signature, so do not call it. Equally, a name"
            " you remember from the upstream C library may not exist here, or"
            " may not be public: import ONLY names listed above.")
    return header + body


def plan_user(inv: CrateInventory) -> str:
    return f"""\
CRATE UNDER TEST
  package name : {inv.crate_name}
  import shape : use {inv.crate_name}::<module_path>::<fn>;
  harness crate: edition {HARNESS_EDITION} — write only syntax valid in it
                 (e.g. `extern "C" {{ … }}`, never `unsafe extern "C" {{ … }}`)
  dependencies : `{inv.crate_name}` and the Rust std library — NOTHING ELSE.
                 The harness manifest has exactly one dependency, so `libc`,
                 `once_cell`, `rand` and friends are NOT available; use
                 `std::os::raw` / `core::ffi` for C types and `std` for the
                 rest. Adding a `use` for any other crate is a build error.
                 (the functions below are grouped under a `## use …` header
                  giving the exact import path for that group)

lib.rs (module tree):
```rust
{inv.lib_rs.strip()}
```

Exported C-ABI functions ({len(inv.fns)} in the crate; grouped by
import path — the ONLY names you may call):
{_fn_listing(inv)}

Public structs: {", ".join(inv.structs[:80])}
{_type_scoping(inv)}
{CONTRACT}

TASK: design the harness as 2–5 OPERATIONS. Each operation is one
end-to-end scenario driving a coherent slice of the API (e.g. decode,
encode, round-trip, state/config coverage). Together they should reach
as many of the exported functions as practical. Prefer one operation
whose inner loop is the library's computational hot path.

Answer with ONE JSON object, schema:
{{
  "harness_name": "<snake_case, e.g. lodepng_harness>",
  "operations": [
    {{
      "name": "<snake_case op name, not 'gen-seeds'>",
      "needs_input": true,
      "summary": "<1-2 sentences: what it drives end-to-end>",
      "api_calls": ["<fn names used>"],
      "input_format": "<what bytes <op>.N.bin contains>",
      "digest": "<what the printed digest folds over>"
    }}
  ],
  "seed_strategy": "<how gen-seeds deterministically synthesizes two
                    distinct valid inputs per operation>"
}}"""


def codegen_user(inv: CrateInventory, spec: dict[str, Any]) -> str:
    return f"""\
Write the complete `src/lib.rs` for the harness specified below.

CRATE UNDER TEST
  package name : {inv.crate_name}
  import shape : use {inv.crate_name}::<module_path>::<fn>;
  harness crate: edition {HARNESS_EDITION} — write only syntax valid in it
                 (e.g. `extern "C" {{ … }}`, never `unsafe extern "C" {{ … }}`)
  dependencies : `{inv.crate_name}` and the Rust std library — NOTHING ELSE.
                 The harness manifest has exactly one dependency, so `libc`,
                 `once_cell`, `rand` and friends are NOT available; use
                 `std::os::raw` / `core::ffi` for C types and `std` for the
                 rest. Adding a `use` for any other crate is a build error.

lib.rs (module tree):
```rust
{inv.lib_rs.strip()}
```

Exported C-ABI functions (grouped by import path — the ONLY names
you may call):
{_fn_listing(inv)}
{_type_scoping(inv)}
HARNESS SPEC (already agreed):
```json
{json.dumps(spec, indent=2, ensure_ascii=False)}
```

{CONTRACT}

{FEWSHOT}

Notes for THIS crate:
- Every library call is `unsafe extern "C"`: use raw pointers,
  MaybeUninit::<T>::zeroed() for C structs, and the library's own
  init/cleanup functions.
- The dependency is already declared in Cargo.toml; just `use` it.
- Do NOT add external crates (std only).

Output the single complete file in one ```rust block."""


def repair_user(stage: str, feedback: str, current_lib_rs: str) -> str:
    return f"""\
The harness you wrote failed the {stage} gate.

GATE FEEDBACK:
```
{feedback.strip()[:8000]}
```

CURRENT src/lib.rs:
```rust
{current_lib_rs}
```

{CONTRACT}

Fix the problem. Keep the CLI and all operations intact unless the
feedback requires otherwise. Output the complete corrected file in one
```rust block."""


_SNIP_CACHE: dict[str, dict[str, str]] = {}


def _internal_snippets(inv: CrateInventory, names: list[str],
                       limit: int = 20) -> str:
    """One signature line per uncovered internal fn, grepped from crate source,
    so the LLM knows what input/state triggers it (e.g. `addChunk_bKGD` →
    encode with a background color set) rather than guessing from the name."""
    if not names:
        return ""
    key = str(inv.crate_dir)
    idx = _SNIP_CACHE.get(key)
    if idx is None:
        idx = {}
        for p in inv.crate_dir.rglob("*.rs"):
            if "/target/" in str(p):
                continue
            try:
                t = p.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            for m in re.finditer(r'\bfn\s+([A-Za-z_]\w*)\s*(?:<[^>]*>)?\s*\(', t):
                nm = m.group(1)
                if nm in idx:
                    continue
                s = t.rfind("\n", 0, m.start()) + 1
                e = t.find("{", m.start())
                if e < 0:
                    e = m.start() + 200
                idx[nm] = f"{p.name}: " + re.sub(r"\s+", " ", t[s:e]).strip()[:180]
        _SNIP_CACHE[key] = idx
    out = []
    for nm in names[:limit]:
        sig = idx.get(nm)
        out.append(f"  {nm}" + (f"    // {sig}" if sig else ""))
    if len(names) > limit:
        out.append(f"  … ({len(names) - limit} more)")
    return "\n".join(out)


def coverage_user(inv: CrateInventory, spec: dict[str, Any],
                  line_pct: float, fn_pct: float,
                  min_line: float, min_fn: float,
                  uncovered: list[str], uncovered_internal: list[str],
                  current_lib_rs: str) -> str:
    internal_block = ""
    if uncovered_internal:
        internal_block = f"""\

Internal (non-pub) library functions NEVER reached by any operation
({len(uncovered_internal)} of {len(uncovered_internal)} shown below). These are
the library's own helpers (chunk encoders/decoders, palette/interlace/16-bit
paths, etc.) — reaching them usually means exercising a FEATURE the current
inputs skip: set the relevant encoder option/state before an encode op, or
feed a decode op an input that CONTAINS that feature (synthesize it with the
encoder in gen_seeds). Signatures for orientation:
{_internal_snippets(inv, uncovered_internal)}
"""
    return f"""\
The harness compiles, is deterministic and input-sensitive — but its
coverage of the crate under test is too low:
  line coverage     {line_pct:.1f}%  (required ≥ {min_line:.0f}%)
  function coverage {fn_pct:.1f}%  (required ≥ {min_fn:.0f}%)  [exported API]

Exported functions NOT yet executed by any operation ({len(uncovered)} shown):
{chr(10).join("  " + n for n in uncovered[:60]) or "  (all exported functions are covered)"}
{internal_block}
CURRENT src/lib.rs:
```rust
{current_lib_rs}
```

CRATE API REMINDER:
{_fn_listing(inv, limit_chars=30_000)}

{CONTRACT}

Extend the harness to raise coverage: widen existing operations (extra
API calls on the same input/state, richer seed content that reaches more
branches, encoder options that trigger the unreached internal paths above)
and/or ADD new operations. If you add or rename operations you MUST also
output the updated spec.

Output format (BOTH blocks are mandatory):
1. ONE ```json block with the FULL updated spec (same schema as before,
   listing EVERY operation the file implements);
2. then ONE ```rust block with the complete updated src/lib.rs."""


# ────────────────────────────────────────────────────────────────────
# Output parsers
# ────────────────────────────────────────────────────────────────────

def extract_json(text: str) -> dict[str, Any] | None:
    m = re.search(r"```json\s*(.*?)```", text, re.S)
    raw = m.group(1) if m else None
    if raw is None:
        # fall back to the outermost {...}
        i, j = text.find("{"), text.rfind("}")
        raw = text[i:j + 1] if 0 <= i < j else None
    if raw is None:
        return None
    try:
        obj = json.loads(raw)
        return obj if isinstance(obj, dict) else None
    except json.JSONDecodeError:
        return None


def extract_rust(text: str) -> str | None:
    m = re.search(r"```rust\s*\n(.*?)```", text, re.S)
    if m:
        return m.group(1)
    m = re.search(r"```\s*\n(fn |use |#\!\[)(.*?)```", text, re.S)
    if m:
        return m.group(1) + m.group(2)
    return None


def validate_spec(spec: dict[str, Any]) -> str | None:
    """Return an error message, or None if the spec is usable."""
    if not isinstance(spec.get("harness_name"), str) or not spec["harness_name"]:
        return "missing harness_name"
    ops = spec.get("operations")
    if not isinstance(ops, list) or not ops:
        return "operations must be a non-empty list"
    seen = set()
    for op in ops:
        name = op.get("name", "")
        if not re.fullmatch(r"[a-z][a-z0-9_]*", name or ""):
            return f"bad operation name: {name!r}"
        if name in seen or name == "gen-seeds":
            return f"duplicate/reserved operation name: {name!r}"
        seen.add(name)
        if not isinstance(op.get("needs_input"), bool):
            return f"operation {name}: needs_input must be a bool"
    return None
