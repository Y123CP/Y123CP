"""C9 — deterministic lowering of `c2rust-bitfields` accessors to mask+shift.

c2rust translates every C bitfield struct into a `[u8; N]` storage field plus a
`#[derive(BitfieldStruct)]` that expands to one getter/setter pair per field.
Those generated accessors loop **once per bit** of the field
(`FieldType::{get,set}_field`), each iteration doing a bounds-checked byte
index, a shift and a mask — and they carry no `#[inline]`.  A C compiler lowers
the same access to one `and`/`or`/`shift` on the containing word, so a state
machine that reads/writes bitfields on every input byte executes several times
the instructions the C original does.

This module rewrites such a struct **deterministically** — no LLM in the loop —
because the transformation is fully determined by the declaration:

    #[bitfield(name = "state", ty = "c_uint", bits = "10..=16")]
    pub type_0_flags_state_..._headers: [u8; 4],

The storage field is a little-endian integer of `N` bytes (that is exactly the
layout `#[repr(C)]` already guarantees), so bits `lo..=hi` of accessor `name`
are `(raw >> lo) & mask` and a write is `(raw & !(mask << lo)) | (v << lo)`.
Emitted accessors keep the original names, signatures and visibility, so no
call site changes; only the struct declaration is replaced.

Deliberate conservatism — the whole struct is skipped (never partially
rewritten) when any of these hold, since equivalence would stop being provable:
  * a storage block wider than 8 bytes (no single integer covers it),
  * a `bool` field wider than one bit (the derive ORs every bit together),
  * a bit range that escapes its storage block,
  * a field type this module cannot classify as signed/unsigned.
"""

from __future__ import annotations

from dataclasses import dataclass
import re

from .regions.cst import get_parser

# `#[bitfield(name = "x", ty = "c_uint", bits = "0..=1")]` — the accessor form.
# `ty` may be written fully qualified (`::core::ffi::c_uint`) depending on how
# the crate was emitted, so the path is matched and normalized to its last
# segment before classification.
_BITFIELD_ATTR_RE = re.compile(
    r'name\s*=\s*"(?P<name>\w+)"\s*,\s*'
    r'ty\s*=\s*"(?P<ty>[\w:]+)"\s*,\s*'
    r'bits\s*=\s*"(?P<lo>\d+)\s*\.\.=\s*(?P<hi>\d+)"',
    re.DOTALL,
)
# `#[bitfield(padding)]` marks a storage field that carries no accessor at all.
# It is not an error — the field simply contributes nothing to lower.
_BITFIELD_PADDING_RE = re.compile(r"#\[\s*bitfield\s*\(\s*padding\s*\)\s*\]")
_STORAGE_TY_RE = re.compile(r"\[\s*u8\s*;\s*(?P<len>\d+)\s*\]")
_DERIVE_BITFIELD_RE = re.compile(r"\bBitfieldStruct\b")

# Rust integer types c2rust emits for bitfield accessors.  `c_char` is omitted
# on purpose: its signedness is platform-dependent, so we refuse rather than
# guess.
_UNSIGNED_TYPES = frozenset({
    "c_uint", "c_ushort", "c_uchar", "c_ulong", "c_ulonglong",
    "u8", "u16", "u32", "u64", "usize",
})
_SIGNED_TYPES = frozenset({
    "c_int", "c_short", "c_schar", "c_long", "c_longlong",
    "i8", "i16", "i32", "i64", "isize",
})

# Storage-block byte width → the unsigned integer wide enough to hold it. C
# bitfield storage is not always a power of two (a 3-byte block is common), so
# odd widths borrow the next larger carrier and are zero-padded on the way in /
# truncated on the way out. Bits above `8*byte_len` are unreachable (they are
# rejected by _validate), so the padding can never be observed.
_CARRIER = {
    1: ("u8", 8), 2: ("u16", 16), 3: ("u32", 32), 4: ("u32", 32),
    5: ("u64", 64), 6: ("u64", 64), 7: ("u64", 64), 8: ("u64", 64),
}
_SIGNED_CARRIER = {8: "i8", 16: "i16", 32: "i32", 64: "i64"}


@dataclass(frozen=True)
class BitfieldSpec:
    """One accessor pair declared by a `#[bitfield(...)]` attribute."""

    name: str
    ty: str          # exactly as written, possibly a path (`::core::ffi::c_int`)
    lo: int
    hi: int

    @property
    def width(self) -> int:
        return self.hi - self.lo + 1

    @property
    def ty_name(self) -> str:
        """Final path segment — what signedness classification keys off."""
        return self.ty.rsplit("::", 1)[-1]


@dataclass(frozen=True)
class BitfieldBlock:
    """A `[u8; N]` storage field and the accessors packed into it."""

    field_name: str
    byte_len: int
    specs: tuple[BitfieldSpec, ...]


@dataclass(frozen=True)
class BitfieldStructRewrite:
    """A single replaceable span covering `#[derive(...)] … struct X { … }`."""

    struct_name: str
    start_byte: int
    end_byte: int
    replacement_text: str
    blocks: tuple[BitfieldBlock, ...]

    @property
    def accessor_names(self) -> frozenset[str]:
        """Every method name the rewritten struct defines (getters + setters)."""
        names: set[str] = set()
        for block in self.blocks:
            for spec in block.specs:
                names.add(spec.name)
                names.add(f"set_{spec.name}")
        return frozenset(names)


class UnsupportedBitfield(Exception):
    """The struct cannot be lowered while provably preserving semantics."""


def _text(source: bytes, node) -> str:
    return source[node.start_byte:node.end_byte].decode("utf-8", errors="replace")


def _carrier_for(byte_len: int) -> tuple[str, int]:
    if byte_len not in _CARRIER:
        raise UnsupportedBitfield(f"storage block of {byte_len} bytes")
    return _CARRIER[byte_len]


def _emit_helpers(byte_len: int, need_signed: bool) -> list[str]:
    """Associated get/set helpers for one storage width.

    They live inside the struct's own `impl`, so several rewritten structs in a
    file never collide on a name.
    """
    carrier, bits = _carrier_for(byte_len)
    carrier_bytes = bits // 8
    exact = carrier_bytes == byte_len
    # For an exact-width block the bytes ARE the carrier; otherwise widen through
    # a zero-padded buffer and write back only the block's own bytes.
    load = (
        f"        let raw = {carrier}::from_le_bytes(*field);"
        if exact else
        f"        let mut buf = [0u8; {carrier_bytes}];\n"
        f"        buf[..{byte_len}].copy_from_slice(field);\n"
        f"        let raw = {carrier}::from_le_bytes(buf);"
    )
    store = (
        "        *field = out.to_le_bytes();"
        if exact else
        f"        field.copy_from_slice(&out.to_le_bytes()[..{byte_len}]);"
    )
    out = [
        f"    /// Read `width` bits starting at `lhs` out of a {byte_len}-byte",
        "    /// little-endian bitfield block (replaces the derive's per-bit loop).",
        "    #[inline(always)]",
        f"    fn _bf_get_{byte_len}(field: &[u8; {byte_len}], lhs: u32, "
        f"width: u32) -> {carrier} {{",
        load,
        f"        let mask = if width >= {bits} {{ {carrier}::MAX }} "
        f"else {{ ((1 as {carrier}) << width) - 1 }};",
        "        (raw >> lhs) & mask",
        "    }",
        "    /// Write the low `width` bits of `val` at `lhs`, leaving the rest",
        "    /// of the block untouched.",
        "    #[inline(always)]",
        f"    fn _bf_set_{byte_len}(field: &mut [u8; {byte_len}], lhs: u32, "
        f"width: u32, val: {carrier}) {{",
        f"        let mask = if width >= {bits} {{ {carrier}::MAX }} "
        f"else {{ ((1 as {carrier}) << width) - 1 }};",
        load,
        "        let out = (raw & !(mask << lhs)) | ((val & mask) << lhs);",
        store,
        "    }",
    ]
    if need_signed:
        signed = _SIGNED_CARRIER[bits]
        out += [
            "    /// Sign-extend a `width`-bit value read out of the block, so a",
            "    /// signed bitfield keeps the derive's sign-extension semantics.",
            "    #[inline(always)]",
            f"    fn _bf_sext_{byte_len}(val: {carrier}, width: u32) "
            f"-> {carrier} {{",
            f"        let shift = {bits} - width;",
            f"        (((val << shift) as {signed}) >> shift) as {carrier}",
            "    }",
        ]
    return out


def _emit_impl(struct_name: str, blocks: tuple[BitfieldBlock, ...]) -> str:
    """Render the replacement `impl` carrying every accessor of the struct."""
    widths_needed: dict[int, bool] = {}
    for block in blocks:
        signed_here = any(spec.ty_name in _SIGNED_TYPES for spec in block.specs)
        widths_needed[block.byte_len] = (
            widths_needed.get(block.byte_len, False) or signed_here
        )

    lines = [
        "// C9 — bitfield accessors lowered from the `c2rust-bitfields` derive.",
        "// The derive loops once per bit (bounds-checked, not inlined); these do",
        "// the same read/modify/write in one mask+shift on the little-endian",
        "// integer the `[u8; N]` block already is.  Layout, names, signatures and",
        "// visibility are unchanged, so call sites keep working verbatim.",
        "#[automatically_derived]",
        f"impl {struct_name} {{",
    ]
    for byte_len in sorted(widths_needed):
        lines += _emit_helpers(byte_len, widths_needed[byte_len])
    for block in blocks:
        carrier, _ = _carrier_for(block.byte_len)
        for spec in block.specs:
            signed = spec.ty_name in _SIGNED_TYPES
            read = (
                f"Self::_bf_get_{block.byte_len}"
                f"(&self.{block.field_name}, {spec.lo}, {spec.width})"
            )
            if signed:
                read = f"Self::_bf_sext_{block.byte_len}({read}, {spec.width})"
            lines += [
                "    /// This method allows you to read from a bitfield to a value",
                "    #[inline(always)]",
                f"    pub fn {spec.name}(&self) -> {spec.ty} {{ "
                f"{read} as {spec.ty} }}",
                "    /// This method allows you to write to a bitfield with a value",
                "    #[inline(always)]",
                f"    pub fn set_{spec.name}(&mut self, int: {spec.ty}) {{ "
                f"Self::_bf_set_{block.byte_len}"
                f"(&mut self.{block.field_name}, {spec.lo}, {spec.width}, "
                f"int as {carrier}) }}",
            ]
    lines.append("}")
    return "\n".join(lines)


def _parse_bitfield_attr(text: str) -> BitfieldSpec | None:
    """Parse one `#[bitfield(...)]` attribute.

    Returns None both for attributes that are not `#[bitfield(...)]` at all and
    for `#[bitfield(padding)]`, which declares no accessor.
    """
    if not text.lstrip().startswith("#[bitfield"):
        return None
    if _BITFIELD_PADDING_RE.search(text):
        return None
    match = _BITFIELD_ATTR_RE.search(text)
    if match is None:
        raise UnsupportedBitfield(f"unparsable bitfield attribute: {text[:60]}")
    lo, hi = int(match.group("lo")), int(match.group("hi"))
    if lo > hi:
        raise UnsupportedBitfield(f"inverted bit range in {text[:60]}")
    # `ty` is kept AS WRITTEN — it is re-emitted as the accessor's return type,
    # and the file may not have the `use` that would make a short name resolve.
    # Signedness is classified off `BitfieldSpec.ty_name` instead.
    return BitfieldSpec(match.group("name"), match.group("ty"), lo, hi)


def _validate(block_len: int, specs: tuple[BitfieldSpec, ...]) -> None:
    _carrier_for(block_len)
    for spec in specs:
        if spec.hi >= block_len * 8:
            raise UnsupportedBitfield(
                f"{spec.name} bits {spec.lo}..={spec.hi} escape "
                f"a {block_len}-byte block"
            )
        if spec.ty_name == "bool":
            # The derive's bool impl ORs every bit of the range together on read
            # and splats the value across all of them on write. A mask+shift only
            # reproduces that for a 1-bit field, and the accessor would need a
            # `!= 0` conversion rather than an `as` cast — not worth the extra
            # shape for a field c2rust emits only for C `_Bool` bitfields.
            raise UnsupportedBitfield(f"bool bitfield {spec.name} is not lowered")
        if (spec.ty_name not in _UNSIGNED_TYPES
                and spec.ty_name not in _SIGNED_TYPES):
            raise UnsupportedBitfield(f"unclassified bitfield type: {spec.ty}")


def _collect_blocks(source: bytes, field_list) -> tuple[BitfieldBlock, ...]:
    """Walk a `field_declaration_list`, pairing attribute runs with their field."""
    blocks: list[BitfieldBlock] = []
    pending: list[BitfieldSpec] = []
    for child in field_list.children:
        if child.type == "attribute_item":
            spec = _parse_bitfield_attr(_text(source, child))
            if spec is not None:
                pending.append(spec)
            continue
        if child.type != "field_declaration":
            continue
        if not pending:
            continue
        decl = _text(source, child)
        name_node = child.child_by_field_name("name")
        type_node = child.child_by_field_name("type")
        if name_node is None or type_node is None:
            raise UnsupportedBitfield(f"unreadable field declaration: {decl[:60]}")
        storage = _STORAGE_TY_RE.fullmatch(_text(source, type_node).strip())
        if storage is None:
            raise UnsupportedBitfield(
                f"bitfield storage is not [u8; N]: {_text(source, type_node)[:40]}"
            )
        byte_len = int(storage.group("len"))
        specs = tuple(pending)
        _validate(byte_len, specs)
        blocks.append(BitfieldBlock(_text(source, name_node), byte_len, specs))
        pending = []
    if pending:
        raise UnsupportedBitfield("trailing #[bitfield] attributes with no field")
    return tuple(blocks)


def _strip_bitfield_attrs(source: bytes, struct_item) -> str:
    """Struct declaration text with every `#[bitfield(...)]` attribute removed."""
    field_list = struct_item.child_by_field_name("body")
    if field_list is None:
        raise UnsupportedBitfield("struct has no body")
    cuts: list[tuple[int, int]] = []
    for child in field_list.children:
        if child.type != "attribute_item":
            continue
        # EVERY `#[bitfield(...)]` has to go, including `#[bitfield(padding)]`,
        # which declares no accessor: once the derive is gone nothing consumes
        # the attribute and leaving one behind fails to compile.
        if not _text(source, child).lstrip().startswith("#[bitfield"):
            continue
        end = child.end_byte
        # Swallow the newline + indentation that followed the attribute so the
        # remaining declaration keeps c2rust's formatting.
        while end < struct_item.end_byte and source[end:end + 1] in (b" ", b"\t"):
            end += 1
        if source[end:end + 1] == b"\n":
            end += 1
        start = child.start_byte
        while start > struct_item.start_byte and source[start - 1:start] in (
            b" ", b"\t"
        ):
            start -= 1
        cuts.append((start, end))
    body = bytearray(source[struct_item.start_byte:struct_item.end_byte])
    base = struct_item.start_byte
    for start, end in sorted(cuts, reverse=True):
        del body[start - base:end - base]
    return bytes(body).decode("utf-8", errors="replace")


def _preceding_attributes(root, index: int) -> list:
    """The unbroken run of `attribute_item` siblings just before `index`."""
    attrs = []
    cursor = index - 1
    while cursor >= 0 and root.children[cursor].type == "attribute_item":
        attrs.append(root.children[cursor])
        cursor -= 1
    attrs.reverse()
    return attrs


def analyze_source(source: bytes) -> tuple[list[BitfieldStructRewrite], list[str]]:
    """Find every `BitfieldStruct` in `source` and render its replacement.

    Returns `(rewrites, skipped)` — `skipped` carries one human-readable reason
    per struct that was found but deliberately left alone.
    """
    tree = get_parser().parse(source)
    root = tree.root_node
    rewrites: list[BitfieldStructRewrite] = []
    skipped: list[str] = []

    for index, node in enumerate(root.children):
        if node.type != "struct_item":
            continue
        attrs = _preceding_attributes(root, index)
        derive = next(
            (
                attr for attr in attrs
                if "derive" in _text(source, attr)
                and _DERIVE_BITFIELD_RE.search(_text(source, attr))
            ),
            None,
        )
        if derive is None:
            continue
        name_node = node.child_by_field_name("name")
        struct_name = _text(source, name_node) if name_node else "<anonymous>"
        field_list = node.child_by_field_name("body")
        if field_list is None:
            skipped.append(f"{struct_name}: struct has no body")
            continue
        try:
            blocks = _collect_blocks(source, field_list)
            if not blocks:
                skipped.append(f"{struct_name}: derive present but no #[bitfield]")
                continue
            new_derive = _DERIVE_BITFIELD_RE.sub("", _text(source, derive))
            # Tidy the comma the removed item left behind, in either position.
            new_derive = re.sub(r",\s*,", ",", new_derive)
            new_derive = re.sub(r",\s*\)", ")", new_derive)
            new_derive = re.sub(r"\(\s*,", "(", new_derive)
            between = source[derive.end_byte:node.start_byte].decode(
                "utf-8", errors="replace"
            )
            replacement = (
                new_derive
                + between
                + _strip_bitfield_attrs(source, node)
                + "\n"
                + _emit_impl(struct_name, blocks)
            )
            rewrites.append(BitfieldStructRewrite(
                struct_name=struct_name,
                start_byte=derive.start_byte,
                end_byte=node.end_byte,
                replacement_text=replacement,
                blocks=blocks,
            ))
        except UnsupportedBitfield as exc:
            skipped.append(f"{struct_name}: {exc}")
    return rewrites, skipped
