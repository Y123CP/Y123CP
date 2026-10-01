"""Conservative declaration facts for deterministic static-to-const edits."""

from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path


@dataclass(frozen=True)
class GlobalDeclFact:
    kind: str
    name: str
    qualified_name: str
    relative_path: str
    line: int
    declaration_start: int
    declaration_end: int
    keyword_start: int
    keyword_end: int
    mutability_start: int | None
    mutability_end: int | None
    visibility: str
    mutability: str
    type_text: str
    rhs_text: str
    attrs: tuple[str, ...]
    declaration_hash: str
    declaration_bytes: bytes


@dataclass(frozen=True)
class ConstPromotionSafety:
    ok: bool
    code: str
    detail: str = ""


_DECL_HEAD = re.compile(
    rb"(?m)^[ \t]*(?P<visibility>pub(?:\s*\([^\n)]*\))?[ \t]+)?"
    rb"(?P<kind>static|const)[ \t]+(?P<mut>mut[ \t]+)?"
    rb"(?P<name>[A-Za-z_][A-Za-z0-9_]*)[ \t]*:"
)
_BAD_ATTRS = ("no_mangle", "used", "link_section", "export_name")
_INTERIOR_MUTABLE = ("Atomic", "Cell", "UnsafeCell", "LazyLock", "OnceLock", "Mutex", "RwLock")


def _rust_char_literal_end(source: bytes, quote_index: int) -> int | None:
    """Return the closing quote for a Rust character literal.

    A bare apostrophe also starts lifetimes and labels.  Treat it as a char
    literal only when exactly one scalar/escape is followed by a closing
    apostrophe; otherwise leave it as code.
    """
    cursor = quote_index + 1
    if cursor >= len(source) or source[cursor] in {10, 13, 39}:
        return None
    if source[cursor] == 92:  # escaped scalar: '\\n', '\\x7f', '\\u{...}'
        cursor += 1
        if cursor >= len(source) or source[cursor] in {10, 13}:
            return None
        if source[cursor] == 120:  # x
            cursor += 3
        elif (
            source[cursor] == 117
            and cursor + 1 < len(source)
            and source[cursor + 1] == 123
        ):  # u{...}
            closing_brace = source.find(b"}", cursor + 2)
            if closing_brace < 0 or b"\n" in source[cursor:closing_brace]:
                return None
            cursor = closing_brace + 1
        else:
            cursor += 1
    else:
        first = source[cursor]
        if first < 0x80:
            cursor += 1
        elif first & 0xE0 == 0xC0:
            cursor += 2
        elif first & 0xF0 == 0xE0:
            cursor += 3
        elif first & 0xF8 == 0xF0:
            cursor += 4
        else:
            return None
    if cursor < len(source) and source[cursor] == 39:
        return cursor
    return None


def _rust_raw_string_end(source: bytes, start: int) -> int | None:
    """Return the inclusive end of a Rust raw string/byte string literal."""
    if start > 0 and (
        source[start - 1] == 95
        or 48 <= source[start - 1] <= 57
        or 65 <= source[start - 1] <= 90
        or 97 <= source[start - 1] <= 122
    ):
        return None
    if source.startswith((b"br", b"cr"), start):
        raw_marker = start + 1
    elif source.startswith(b"r", start):
        raw_marker = start
    else:
        return None
    cursor = raw_marker + 1
    while cursor < len(source) and source[cursor] == 35:
        cursor += 1
    if cursor >= len(source) or source[cursor] != 34:
        return None
    hashes = source[raw_marker + 1:cursor]
    terminator = b'"' + hashes
    closing = source.find(terminator, cursor + 1)
    if closing < 0:
        return None
    return closing + len(terminator) - 1


def _mask_non_code(source: bytes, *, mask_macros: bool = True) -> bytes:
    masked = bytearray(source)
    index = 0
    state = "code"
    block_depth = 0
    while index < len(source):
        char = source[index]
        nxt = source[index + 1] if index + 1 < len(source) else 0
        if state == "line":
            if char == 10:
                state = "code"
            else:
                masked[index] = 32
        elif state == "block":
            if char == 47 and nxt == 42:
                masked[index:index + 2] = b"  "
                block_depth += 1
                index += 1
            elif char == 42 and nxt == 47:
                masked[index:index + 2] = b"  "
                block_depth -= 1
                index += 1
                if block_depth == 0:
                    state = "code"
            elif char != 10:
                masked[index] = 32
        elif state in {"string", "char"}:
            quote = 34 if state == "string" else 39
            if char == 92:
                masked[index] = 32
                if index + 1 < len(source):
                    if source[index + 1] != 10:
                        masked[index + 1] = 32
                    index += 1
            elif char == quote:
                masked[index] = 32
                state = "code"
            elif char != 10:
                masked[index] = 32
        else:
            raw_end = _rust_raw_string_end(source, index)
            if raw_end is not None:
                for raw_index in range(index, raw_end + 1):
                    if source[raw_index] != 10:
                        masked[raw_index] = 32
                index = raw_end
            elif char == 47 and nxt == 47:
                masked[index:index + 2] = b"  "
                state = "line"
                index += 1
            elif char == 47 and nxt == 42:
                masked[index:index + 2] = b"  "
                state = "block"
                block_depth = 1
                index += 1
            elif char == 34:
                masked[index] = 32
                state = "string"
            elif char == 39 and _rust_char_literal_end(source, index) is not None:
                masked[index] = 32
                state = "char"
        index += 1
    if mask_macros:
        _mask_macro_rules(source, masked)
    return bytes(masked)


def _mask_macro_rules(source: bytes, masked: bytearray) -> None:
    for match in list(re.finditer(rb"\bmacro_rules\s*!", bytes(masked))):
        opening = bytes(masked).find(b"{", match.end())
        if opening < 0:
            continue
        depth = 0
        end = opening
        while end < len(masked):
            if masked[end] == 123:
                depth += 1
            elif masked[end] == 125:
                depth -= 1
                if depth == 0:
                    end += 1
                    break
            end += 1
        for index in range(match.start(), min(end, len(masked))):
            if source[index] != 10:
                masked[index] = 32


def _macro_invocation_mentions(masked: bytes, name: bytes) -> bool:
    """Fail-closed when a static is passed as an opaque macro argument."""
    openings = {40: 41, 91: 93, 123: 125}
    closings = set(openings.values())
    for bang in re.finditer(rb"!", masked):
        before = bang.start() - 1
        while before >= 0 and masked[before] in {9, 10, 13, 32}:
            before -= 1
        if before < 0 or not (
            masked[before] == 95
            or 48 <= masked[before] <= 57
            or 65 <= masked[before] <= 90
            or 97 <= masked[before] <= 122
            or masked[before] >= 128
        ):
            continue
        cursor = bang.end()
        while cursor < len(masked) and masked[cursor] in {9, 10, 13, 32}:
            cursor += 1
        if cursor >= len(masked) or masked[cursor] not in openings:
            continue
        stack = [openings[masked[cursor]]]
        end = cursor + 1
        while end < len(masked) and stack:
            char = masked[end]
            if char in openings:
                stack.append(openings[char])
            elif char == stack[-1]:
                stack.pop()
            elif char in closings:
                break
            end += 1
        if not stack and re.search(
            rb"\b" + re.escape(name) + rb"\b", masked[cursor + 1:end]
        ):
            return True
    return False


@lru_cache(maxsize=64)
def _parse_rust_source(source: bytes):
    from tree_sitter import Language, Parser
    import tree_sitter_rust

    return Parser(Language(tree_sitter_rust.language())).parse(source)


def _node_contains(container, node) -> bool:
    return (
        container is not None
        and container.start_byte <= node.start_byte
        and node.end_byte <= container.end_byte
    )


def _has_descendant_type(node, wanted: str) -> bool:
    stack = [node]
    while stack:
        current = stack.pop()
        if current.type == wanted:
            return True
        stack.extend(current.children)
    return False


def _is_binding_or_item_name(node) -> bool:
    current = node
    while current.parent is not None:
        parent = current.parent
        if parent.type in {
            "static_item", "const_item", "function_item", "mod_item",
            "type_item", "struct_item", "enum_item", "union_item",
        }:
            name_node = parent.child_by_field_name("name")
            return name_node is not None and name_node.id == node.id
        if parent.type in {
            "let_declaration", "let_condition", "parameter", "for_expression",
            "match_arm", "closure_expression",
        }:
            pattern = parent.child_by_field_name("pattern")
            if _node_contains(pattern, node):
                return True
        if parent.type.endswith("expression") or parent.type in {
            "arguments", "token_tree", "use_declaration",
        }:
            break
        current = parent
    return False


def _classify_proven_value_use(node, source: bytes) -> str:
    """Classify one static occurrence; only explicit value contexts pass."""
    ancestor = node.parent
    while ancestor is not None:
        if ancestor.type == "macro_invocation":
            return "macro_argument_unproven"
        ancestor = ancestor.parent

    current = node
    aggregate_wrappers = {
        "array_expression", "parenthesized_expression", "scoped_identifier",
        "struct_expression", "tuple_expression",
    }
    while current.parent is not None:
        parent = current.parent
        if parent.type in aggregate_wrappers:
            current = parent
            continue
        if parent.type == "reference_expression":
            return "address_taken"
        if parent.type == "field_expression":
            return "implicit_mutation_unproven"
        if parent.type in {"assignment_expression", "compound_assignment_expr"}:
            left = parent.child_by_field_name("left")
            return "written" if _node_contains(left, current) else "safe"
        if parent.type in {"let_declaration", "let_condition"}:
            pattern = parent.child_by_field_name("pattern")
            if pattern is not None and _has_descendant_type(pattern, "ref_pattern"):
                return "implicit_mutation_unproven"
            return "safe"
        if parent.type in {"match_expression", "for_expression"}:
            if _has_descendant_type(parent, "ref_pattern"):
                return "implicit_mutation_unproven"
            return "safe"
        if parent.type == "unary_expression":
            operator = source[parent.start_byte:current.start_byte].strip()
            return "safe" if operator in {b"-", b"!"} else "use_context_unproven"
        if parent.type in {
            "arguments", "binary_expression", "block", "break_expression",
            "else_clause", "expression_statement", "if_expression",
            "range_expression", "return_expression", "type_cast_expression",
            "while_expression",
        }:
            return "safe"
        if parent.type == "use_declaration":
            return "safe"
        return "use_context_unproven"
    return "use_context_unproven"


def _validate_proven_value_uses(
    source: bytes,
    masked: bytes,
    fact: GlobalDeclFact,
    *,
    declaration_file: bool,
) -> ConstPromotionSafety | None:
    """Require every lexical occurrence to map to a proven read-only CST use."""
    tree = _parse_rust_source(source)
    name = fact.name.encode()
    occurrences = list(re.finditer(rb"\b" + re.escape(name) + rb"\b", masked))
    identifiers = []
    stack = [tree.root_node]
    while stack:
        node = stack.pop()
        if node.type == "identifier" and source[node.start_byte:node.end_byte] == name:
            identifiers.append(node)
        stack.extend(node.children)

    for occurrence in occurrences:
        if declaration_file and (
            fact.declaration_start <= occurrence.start() < fact.declaration_end
        ):
            continue
        node = next(
            (
                candidate
                for candidate in identifiers
                if candidate.start_byte == occurrence.start()
                and candidate.end_byte == occurrence.end()
            ),
            None,
        )
        if node is None:
            return ConstPromotionSafety(False, "syntax_unproven")
        if _is_binding_or_item_name(node):
            continue
        code = _classify_proven_value_use(node, source)
        if code != "safe":
            return ConstPromotionSafety(False, code)
    return None


def _module_qualified(relative_path: str, name: str) -> str:
    path = Path(relative_path)
    parts = list(path.with_suffix("").parts)
    if parts and parts[0] == "src":
        parts.pop(0)
    if parts and parts[-1] in {"lib", "main", "mod"}:
        parts.pop()
    return "::".join(("crate", *parts, name))


def _attrs_before(source: bytes, item_start: int) -> tuple[int, tuple[str, ...]]:
    line_start = source.rfind(b"\n", 0, item_start) + 1
    masked = _mask_non_code(source, mask_macros=False)
    cursor = item_start
    declaration_start = line_start
    attrs: list[str] = []
    whitespace = {9, 10, 13, 32}
    while True:
        while cursor > 0 and masked[cursor - 1] in whitespace:
            cursor -= 1
        if cursor == 0 or masked[cursor - 1] != 93:
            break
        depth = 0
        opening = cursor - 1
        while opening >= 0:
            if masked[opening] == 93:
                depth += 1
            elif masked[opening] == 91:
                depth -= 1
                if depth == 0:
                    break
            opening -= 1
        hash_start = opening - 1
        if opening < 0 or hash_start < 0 or masked[hash_start] != 35:
            break
        if hash_start > 0 and masked[hash_start - 1] == 33:
            break
        attrs.insert(
            0, source[hash_start:cursor].decode("utf-8", errors="replace")
        )
        declaration_start = source.rfind(b"\n", 0, hash_start) + 1
        cursor = declaration_start
    return declaration_start, tuple(attrs)


def scan_global_declarations(crate_root: Path) -> list[GlobalDeclFact]:
    facts: list[GlobalDeclFact] = []
    for path in sorted(Path(crate_root).rglob("*.rs")):
        if any(part in {"target", ".git", ".perf_opt"} for part in path.parts):
            continue
        source = path.read_bytes()
        masked = _mask_non_code(source)
        for match in _DECL_HEAD.finditer(masked):
            semicolon = masked.find(b";", match.end())
            if semicolon < 0:
                continue
            declaration_end = semicolon + 1
            colon = masked.find(b":", match.start("name"), match.end())
            equals = masked.find(b"=", colon + 1, declaration_end)
            if equals < 0:
                continue
            keyword_start, keyword_end = match.span("kind")
            mutability_start = (
                match.start("mut") if match.group("mut") else None
            )
            mutability_end = (
                mutability_start + len("mut")
                if mutability_start is not None else None
            )
            declaration_start, attrs = _attrs_before(source, match.start())
            name = match.group("name").decode()
            relative = str(path.relative_to(crate_root))
            declaration = source[declaration_start:declaration_end]
            facts.append(GlobalDeclFact(
                kind=match.group("kind").decode(),
                name=name,
                qualified_name=_module_qualified(relative, name),
                relative_path=relative,
                line=source.count(b"\n", 0, keyword_start) + 1,
                declaration_start=declaration_start,
                declaration_end=declaration_end,
                keyword_start=keyword_start,
                keyword_end=keyword_end,
                mutability_start=mutability_start,
                mutability_end=mutability_end,
                visibility=(match.group("visibility") or b"").decode().strip(),
                mutability="mut" if match.group("mut") else "",
                type_text=source[colon + 1:equals].decode().strip(),
                rhs_text=source[equals + 1:semicolon].decode().strip(),
                attrs=attrs,
                declaration_hash=hashlib.sha256(declaration).hexdigest(),
                declaration_bytes=declaration,
            ))
    return facts


def _literal_rhs(rhs: str) -> bool:
    literal = r"(?:true|false|'(?:\\.|[^'])'|[-+]?(?:0[xX][0-9A-Fa-f_]+|0[bB][01_]+|0[oO][0-7_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][-+]?\d+)?)(?:[iu](?:8|16|32|64|128|size)|f(?:32|64))?)"
    cast = r"(?:\s+as\s+[A-Za-z_][A-Za-z0-9_:<>]*)*"
    return re.fullmatch(literal + cast, rhs.strip()) is not None


def validate_const_promotion_safety(
    crate_root: Path, fact: GlobalDeclFact
) -> ConstPromotionSafety:
    if fact.kind != "static":
        return ConstPromotionSafety(False, "not_static")
    if any(marker in attr for attr in fact.attrs for marker in _BAD_ATTRS):
        return ConstPromotionSafety(False, "alias_sensitive_attribute")
    if any(marker in fact.type_text for marker in _INTERIOR_MUTABLE):
        return ConstPromotionSafety(False, "interior_mutability")
    if not _literal_rhs(fact.rhs_text):
        return ConstPromotionSafety(False, "complex_rhs_unproven")
    if fact.mutability and fact.visibility == "pub":
        return ConstPromotionSafety(False, "externally_visible_mutable_static")

    name = re.escape(fact.name).encode()
    ident_start = rb"(?:[A-Za-z_]|[\x80-\xff])"
    ident_continue = rb"(?:[A-Za-z0-9_]|[\x80-\xff])*"
    path_segment = rb"(?:r#)?" + ident_start + ident_continue
    symbol = rb"(?:::)?(?:" + path_segment + rb"\s*::\s*)*" + name + rb"\b"
    place = rb"(?:\(\s*)*" + symbol + rb"(?:\s*\))*"
    address = re.compile(
        rb"(?:&\s*(?:(?:raw\s+(?:const|mut)|mut)\s+)?"
        rb"|addr_of(?:_mut)?!\s*\(\s*)" + place
    )
    renamed_import = re.compile(
        rb"\buse\b[^;]*\b" + name + rb"\b\s+as\b"
    )
    write = re.compile(
        place + rb"\s*(?:=(?!=)|\+=|-=|\*=|/=|%=|<<=|>>=|&=|\^=|\|=)"
    )
    for path in Path(crate_root).rglob("*.rs"):
        source = path.read_bytes()
        masked = _mask_non_code(source, mask_macros=False)
        if address.search(masked):
            return ConstPromotionSafety(False, "address_taken")
        if fact.mutability and renamed_import.search(masked):
            return ConstPromotionSafety(False, "renamed_import_unproven")
        if fact.mutability and _macro_invocation_mentions(
            masked, fact.name.encode()
        ):
            return ConstPromotionSafety(False, "macro_argument_unproven")
        for match in write.finditer(masked):
            if path.resolve() == (Path(crate_root) / fact.relative_path).resolve() and (
                fact.declaration_start <= match.start() < fact.declaration_end
            ):
                continue
            return ConstPromotionSafety(False, "written")
        cst_failure = _validate_proven_value_uses(
            source,
            masked,
            fact,
            declaration_file=(
                path.resolve()
                == (Path(crate_root) / fact.relative_path).resolve()
            ),
        )
        if cst_failure is not None:
            return cst_failure
    return ConstPromotionSafety(True, "safe_literal_static")
