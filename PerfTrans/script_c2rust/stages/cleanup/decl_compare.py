"""Cross-module declaration-equality normalization.

Shared between `anon_types` (body-hash partitioning of C2RustUnnamed
clones) and `pub_type_unify` (body-equality gate for cross-module
struct/union/enum dedup).
"""

from __future__ import annotations


def normalize_decl_for_compare(decl_bytes: bytes) -> bytes:
    """Normalize a type declaration's text for cross-module equality.

    Strips:
      · trailing whitespace per line
      · blank lines
      · `//` line comments
    Normalizes:
      · `*mut T` / `*const T` → `*ptr T` — c2rust translation often emits
        the SAME C struct with `*mut Bytef` in one TU and `*const Bytef`
        in another (zlib's `z_stream_s` next_in / msg fields, observed
        on optipng 2026-05-29). The C semantics are equivalent (raw
        pointer is informational only in Rust); per-field mut/const
        divergence is a c2rust artifact, not a real semantic
        difference. Collapsing both forms to `*ptr` lets such cases
        unify safely. Inner type T must still match.
    Preserves:
      · field names
      · inner type identifiers (so `*ptr u32` vs `*ptr i32` stays distinct)
      · attribute attributes (`#[derive(...)]`, `#[repr(C)]`)
      · visibility / non-pointer mut

    Two structs/unions/enums with identical field shape (modulo the
    normalizations above) unify safely; genuinely different ones
    (e.g. c2rust's synthetic `C2RustUnnamed` per-TU anonymous structs)
    stay distinct.
    """
    out: list[bytes] = []
    for raw_line in decl_bytes.splitlines():
        # Strip // line comments (block comments /* */ are rare in
        # c2rust output; left alone — they would be unusual but safe).
        idx = raw_line.find(b"//")
        if idx >= 0:
            raw_line = raw_line[:idx]
        s = raw_line.strip()
        if not s:
            continue
        # Collapse raw-pointer mutability tokens. Byte substring
        # substitution (not regex): the canonical c2rust output forms
        # are `*mut T` and `*const T` with single space — we replace
        # the prefix `*mut ` / `*const ` with `*ptr `. False positives
        # would need `*mut`/`*const` appearing OUTSIDE a pointer-type
        # context, which doesn't happen in c2rust struct field decls.
        s = s.replace(b"*mut ",   b"*ptr ")
        s = s.replace(b"*const ", b"*ptr ")
        out.append(s)
    return b"\n".join(out)
