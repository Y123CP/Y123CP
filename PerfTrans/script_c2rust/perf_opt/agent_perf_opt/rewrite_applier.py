""                                                                           

                              
                                                                                
                                                                             
                                                                          
                                                                       

                        
                               
                                     
                                  
                                 
   

from __future__ import annotations

import logging
import re
import subprocess
import time
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Optional

from perf_opt.agent_perf_opt.regions.cst import get_parser

from perf_opt.hot_probe.symbol_source import (
    FnIndex, _exported_symbol, _iter_function_items,
)
from perf_opt.hot_probe.types import EvidencePack, HotFunction

logger = logging.getLogger(__name__)


# ─── data structures (impl_plan §4.3 / §4.6) ─────────────────────────────

@dataclass(frozen=True)
class EditTarget:
    ""                                      
                                                                       
                                                                      
                                                                   
       
    file:    Path                                
    fn_name: str                                
    span:    tuple[int, int]   # (start_byte, end_byte) inclusive of preceding attrs


class RewriteStatus(Enum):
    APPLIED      = "applied"                                 
    ABSTAINED    = "abstained"                                               
    SYNTAX_ERROR = "syntax_error"                            


# A build that ran out of clock, not one that failed to compile. It lands in
# the same `(ok, stderr)` channel as a compile error, so it needs a mark of
# its own: what goes in the retry prompt where the stderr belongs is
# `cargo build timeout after 120s`, and a model asked to "fix the compile
# error" in that has nothing to fix and edits working code instead.
BUILD_TIMEOUT_PREFIX = "cargo build timeout"
# How much of a failed build's diagnostics a retry prompt is allowed to carry.
# Sized to hold every `error` block of a typical failure rather than a fixed
# slice of one: the old 2000 left 5 of 22 diagnostics on a libxml2 retry, and
# the model cannot fix what it is not shown.
STDERR_EXCERPT_CHARS = 8000


def is_build_timeout(stderr: Optional[str]) -> bool:
    """True when a build reported the clock, not the compiler."""
    return bool(stderr) and stderr.lstrip().startswith(BUILD_TIMEOUT_PREFIX)


@dataclass
class RewriteOutcome:
    status:           RewriteStatus
    committed_source: Optional[str] = None                               
    cargo_stderr:     Optional[str] = None                                
    abstain_reason:   Optional[str] = None                         


                                                                        

                                       
_FENCE_RE = re.compile(r"```(?:rust)?\s*\n(.*?)\n```", re.DOTALL)

                             
_ABSTAIN_RE = re.compile(r"^\s*abstain\s*:\s*(.+)", re.IGNORECASE | re.MULTILINE)

# A model with nothing to list does not always write `[]`. Measured on
# lodepng `lodepng_compute_color_stats`: it wrote `// Skipped rules: none`,
# was refused on form, was told the line must read `// Skipped rules: [...]`,
# and dutifully sent `[none]` — which parses as a rule id nobody offered, so
# `declared - candidates` discarded a 159-line rewrite that had already been
# through one repair turn. `none` is not a card name and never will be, so
# the parser is where this gets reconciled rather than the model.
_EMPTY_LIST_SENTINELS = frozenset({
    "", "-", "--", "n/a", "na", "none", "nil", "null",
    "empty", "nothing", "no rules", "(none)",
})


def _is_empty_rule_list(inner: str) -> bool:
    """True when the bracket content means "nothing", however it is spelled."""
    return inner.strip().strip(".").casefold() in _EMPTY_LIST_SENTINELS


                                                         
#   `// Applied rules: [C1, III④]`  → ["C1", "III④"]
#
                                                
                                             
                                                      
                                             
                                    
                                        
                                    
_APPLIED_RULES_RE = re.compile(
    r"^\s*//\s*Applied\s+rules\s*:\s*\[(.*)\][ \t\r]*$",
    re.IGNORECASE | re.MULTILINE,
)

                                                            
#   `// Skipped rules: [II_inl: callee too large; C1: no hot loop]`
#     → [("II_inl", "callee too large"), ("C1", "no hot loop")]
_SKIPPED_RULES_RE = re.compile(
    r"^\s*//\s*Skipped\s+rules\s*:\s*\[(.*)\][ \t\r]*$",
    re.IGNORECASE | re.MULTILINE,
)

                                        
                                     
#   `III②: ... \`[*mut Node; 2]\` ...; III③: ...`
                                          
                                                            
                                
_BRACKET_PAIRS = {"[": "]", "(": ")", "{": "}"}


def _top_level_positions(text: str, sep: str):
    """Indices where `sep` appears outside brackets and outside backticks."""
    depth = 0
    in_code = False
    for index, ch in enumerate(text):
        if ch == "`":
            in_code = not in_code
            continue
        if in_code:
            continue
        if ch in _BRACKET_PAIRS:
            depth += 1
        elif ch in _BRACKET_PAIRS.values():
            depth = max(0, depth - 1)
        elif ch == sep and depth == 0:
            yield index


def _split_top_level(text: str, sep: str) -> list[str]:
    """Split on `sep`, ignoring separators nested in brackets or backticks."""
    parts: list[str] = []
    start = 0
    for index in _top_level_positions(text, sep):
        parts.append(text[start:index])
        start = index + 1
    parts.append(text[start:])
    return parts


def _split_rule_and_reason(part: str) -> tuple[str, str] | None:
    """`<rule>: <reason>` split on the first TOP-LEVEL colon, or None.

    Top-level matters because reasons quote Rust, and Rust paths carry `::`.
    A bare `":" in part` reads `` `ptr::copy` `` as a rule/reason boundary and
    turns half a sentence into a rule id — the same class of mistake as
    splitting on a semicolon that was only punctuation, one layer down.
    """
    for index in _top_level_positions(part, ":"):
        return part[:index], part[index + 1:]
    return None


def parse_llm_response(response: str) -> tuple[Optional[str], Optional[str]]:
    ""                                                                  

                                              
                                               
                                               
                                                                
       
    if not response or not response.strip():
        return None, "empty_response"

    # Check abstain marker first — anywhere in the response takes precedence
    m = _ABSTAIN_RE.search(response)
    if m:
        return None, m.group(1).strip()

    # Then look for fenced code
    m = _FENCE_RE.search(response)
    if m:
        code = m.group(1).rstrip()
        if not code:
            return None, "empty_fence"
        return code, None

    return None, "parse_fail"       # no fence, no abstain marker → abstain


def parse_applied_rules(code: str) -> list[str]:
    ""                                                             

                                               
                                        
                                                     

                                           
       
    if not code:
        return []
    m = _APPLIED_RULES_RE.search(code)
    if not m:
        return []
    inner = m.group(1)
    if _is_empty_rule_list(inner):
        return []
    return [r.strip() for r in inner.split(",") if r.strip()]


                               
                                                       
                                                                 
                                                          
                                   
                                           
_RULE_EXEC_FINGERPRINTS: dict[str, tuple[str, ...]] = {
                                              
    "III④": ("from_raw_parts", "from_raw_parts_mut",
              ".get_unchecked(", ".get_unchecked_mut(",
              ".iter()", ".iter_mut()",
                                                                          
                                                                              
                                                                       
                                                        
              ".split_at(", ".split_at_mut(",
              ".chunks(", ".chunks_mut(", ".chunks_exact(", ".chunks_exact_mut(",
              ".windows(",
                                                     
                                             
              "[..].", "[.."),
                                                    
    "III③": ("copy_from_slice", "copy_nonoverlapping",
              "read_unaligned", "write_unaligned",
              "from_le_bytes", "from_be_bytes",
                                                                         
              "core::ptr::copy", "core::ptr::read", "core::ptr::write"),
                                                               
    "III①": ("impl Fn(", "impl FnMut(", "impl FnOnce(",
              "F: Fn(", "F: FnMut(", "F: FnOnce(",
              "<F:", "<F,"),
                                  
    "III②": ("Box::new(", "Box::from_raw(", "Vec::with_capacity(",
              "vec![", ".into_boxed_slice("),
                                               
                                                       
                                              
    # `write_bytes` / `fill` are the BULK forms the card mandates (§2.0); a
    # per-element loop measured +4.5% on a near-full prefix, so the accepted
    # shapes are named here as well as in the card.
    "C12": ("MaybeUninit", "maybe_uninit", "write_bytes", ".fill("),
                                                                                      
    "C1": (".get_unchecked(", ".get_unchecked_mut(",
            "from_raw_parts", "from_raw_parts_mut"),
                                                                            
                                                                
                                    
    "C3": (),
                                                    
    "C4": ("read_unaligned", "trailing_zeros", "leading_zeros"),
                                                         
    "C5": ("wrapping_mul", "* ", ".mul("),
                                                               
    "C6": (),
                                                                    
    "C7": ("malloc_usable_size", "with_capacity", ".reserve(", "Vec::", "spare_capacity"),
                                                           
                                                   
    "C8": ("read_unaligned", ">> 24", ">> 16"),
                                                         
                                         
                                                    
    "C10": ("copy_from_slice", "to_le_bytes", "to_be_bytes", "from_le_bytes",
            "from_be_bytes", "copy_within", "wrapping_mul", "try_into"),
                                                        
                                                          
                                                    
                                                              
                                             
                                             
    "C2": (".to_int_unchecked", "to_int_unchecked::", ".clamp("),
                                                       
    "II_vec": (".iter(", ".fold(", ".map(", ".zip(", ".enumerate("),
    # II_inl is not here: it runs as a deterministic planner (one attribute on
    # the callee, planners/inline_attr.py), so there is no model reply to
    # fingerprint.
}



_LOOP_NODES = frozenset({"while_expression", "loop_expression", "for_expression"})
_INDEX_RE = re.compile(r"\w\s*\[[^\]]*\bas\s+usize\s*\]")


def _index_parts(node):
    """`a[i]` has no `index` field in tree-sitter-rust — its named children
    are [base, subscript], in that order."""
    named = [c for c in node.children if c.is_named]
    if len(named) < 2:
        return None, None
    return named[0], named[1]


def _is_constant_expr(node) -> bool:
    """True when the value is fixed at compile time.

    Literals, `const` items, and arithmetic over them. c2rust names constants
    the way C did — `TINFL_FAST_LOOKUP_SIZE`, `TDEFL_LZ_DICT_SIZE_MASK` — and
    gives locals the lowercase names their C originals had, so an all-caps
    identifier is a `const`. That is a shape rule, not a project one.
    """
    if node is None:
        return False
    while True:
        if node.type == "parenthesized_expression":
            inner = [c for c in node.children if c.is_named]
            if len(inner) != 1:
                return False
            node = inner[0]
            continue
        if node.type == "type_cast_expression":
            value = node.child_by_field_name("value")
            if value is None:
                return False
            node = value
            continue
        break
    if node.type == "integer_literal":
        return True
    if node.type == "identifier":
        return node.text.decode("utf-8", "replace").isupper()
    if node.type == "scoped_identifier":
        # `crate::c_consts::TDEFL_LZ_DICT_SIZE_MASK`
        return node.text.decode("utf-8", "replace").rsplit(":", 1)[-1].isupper()
    # A struct field is never a `const` item, whatever it is called.
    if node.type in ("binary_expression", "unary_expression"):
        operands = [c for c in node.children if c.is_named]
        return bool(operands) and all(_is_constant_expr(c) for c in operands)
    return False


def _binary_op(node) -> str:
    for child in node.children:
        if not child.is_named:
            return child.text.decode("utf-8", "replace")
    return ""


def _is_provably_in_range(node) -> bool:
    """True when the compiler can see this subscript is in bounds.

    That, and not "did the original index this way", is what separates a
    bounds check that costs something from one that costs nothing. Two shapes
    account for all of it in c2rust output:

      `tbl[0 as c_int as usize]`               — a constant index
      `tbl[((crc ^ b) & 0xff) as usize]`       — clamped by a constant mask

    LLVM folds the comparison away in both, so flagging them buys nothing and
    costs a candidate. Measured: six of miniz's seven guard rejections were
    one of these two shapes, and the crate landed at -1.251% where the
    previous rule set had reached -17.324%. The `& 0xff` case alone was
    `mz_crc32`, worth -21.559%.

    A runtime index stays flagged even when the pre-rewrite source already had
    it. That case is not hypothetical either: zopfli's `sublen[k as usize]`
    with `k <= kend` runtime-bounded is in the c2rust output verbatim, and
    replacing exactly that one expression with `get_unchecked` measured
    -7.55%. Exempting what the file already indexed would have let it through.
    """
    if node is None:
        return False
    while True:
        if node.type == "parenthesized_expression":
            inner = [c for c in node.children if c.is_named]
            if len(inner) != 1:
                break
            node = inner[0]
            continue
        if node.type == "type_cast_expression":
            value = node.child_by_field_name("value")
            if value is None:
                break
            node = value
            continue
        break
    if _is_constant_expr(node):
        return True
    if node.type == "binary_expression":
        op = _binary_op(node)
        operands = [c for c in node.children if c.is_named]
        if len(operands) == 2:
            if op == "&":
                return any(_is_constant_expr(c) for c in operands)
            if op == "%":
                return _is_constant_expr(operands[1])
    return False


_INT_LITERAL_RE = re.compile(r"^(0[xX][0-9a-fA-F]+|0[bB][01]+|0[oO][0-7]+|\d+)")
# Steps that can only move an integer up. `wrapping_add` is the one c2rust
# emits; the other two appear in rewrites that kept the overflow semantics
# explicit. A `+` on a literal is the same statement in Rust's own spelling.
_MONOTONE_ADD = frozenset({"wrapping_add", "checked_add", "saturating_add"})
# The mirror set, for the descending walk `(N - 1) - i` c2rust writes when the
# C loop counted down.
_MONOTONE_SUB = frozenset({"wrapping_sub", "checked_sub", "saturating_sub"})


def _strip_casts(node):
    """Peel `(...)` and `x as T` down to the value underneath."""
    while node is not None:
        if node.type == "parenthesized_expression":
            inner = [c for c in node.children if c.is_named]
            if len(inner) != 1:
                return node
            node = inner[0]
            continue
        if node.type == "type_cast_expression":
            value = node.child_by_field_name("value")
            if value is None:
                return node
            node = value
            continue
        return node
    return node


def _ident(node) -> str:
    """The bare identifier a node denotes, or "" when it is not one."""
    node = _strip_casts(node)
    if node is not None and node.type == "identifier":
        return node.text.decode("utf-8", "replace")
    return ""


def _literal_int(node) -> Optional[int]:
    """The value of an integer literal, suffix and separators tolerated."""
    node = _strip_casts(node)
    if node is None or node.type != "integer_literal":
        return None
    text = node.text.decode("utf-8", "replace").replace("_", "")
    m = _INT_LITERAL_RE.match(text)
    if m is None:
        return None
    try:
        return int(m.group(0), 0)
    except ValueError:
        return None


def _const_int_in_scope(node, name: str) -> Optional[int]:
    """A module-level `const N: <int> = <literal>` this file declares.

    c2rust hoists C's `#define`d sizes and `enum` bounds to module consts, so
    the bound a loop compares against is as often a name as a literal. Sampled
    across the corpus: miniz declares 14 integer consts, brotli 39. Reading
    only literals means the proof below never fires on any of them.

    An array-typed item is not ours — `_module_array_len` answers that
    question, and answering it here too would let a `[T; N]` be read as the
    scalar `N`. A value that is not a literal abandons the proof rather than
    guessing.
    """
    if not name:
        return None
    root = _tree_root(node)
    if root is None:
        return None
    best: Optional[int] = None
    stack = [root]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type not in ("const_item", "static_item"):
            continue
        ident = n.child_by_field_name("name")
        if ident is None:
            continue
        if ident.text.decode("utf-8", "replace").strip() != name:
            continue
        if _array_type_len(n.child_by_field_name("type")) is not None:
            return None
        value = _literal_int(n.child_by_field_name("value"))
        if value is None:
            return None
        best = value if best is None else min(best, value)
    return best


def _const_fold(node) -> Optional[int]:
    """The compile-time value of a literal, a const name, or `+`/`-` of them.

    Deliberately this small. Everything it folds is a value rustc itself has
    before codegen, which is exactly the precondition the callers need; a
    runtime term anywhere makes the whole expression unknown and the proof is
    abandoned.
    """
    node = _strip_casts(node)
    if node is None:
        return None
    value = _literal_int(node)
    if value is not None:
        return value
    name = _ident(node)
    if name:
        return _const_int_in_scope(node, name)
    if node.type == "binary_expression":
        op = _binary_op(node)
        if op in ("+", "-"):
            operands = [c for c in node.children if c.is_named]
            if len(operands) == 2:
                left = _const_fold(operands[0])
                right = _const_fold(operands[1])
                if left is not None and right is not None:
                    return left + right if op == "+" else left - right
    return None


def _enclosing_function(node):
    while node is not None and node.type != "function_item":
        node = node.parent
    return node


def _tree_root(node):
    while node is not None and node.parent is not None:
        node = node.parent
    return node


def _array_type_len(ty) -> Optional[int]:
    """`[T; N]` — N, when N is an integer literal. Otherwise None."""
    if ty is None or ty.type != "array_type":
        return None
    return _literal_int(ty.child_by_field_name("length"))


def _unwrap_block(node):
    """Peel `unsafe { e }` / `{ e }` down to `e` when that is all it holds."""
    while node is not None:
        if node.type in ("unsafe_block", "block"):
            inner = [c for c in node.children if c.is_named]
            if len(inner) != 1:
                return node
            node = inner[0]
            continue
        return node
    return node


def _slice_view_len(value) -> Optional[int]:
    """`from_raw_parts(p, N)` — N, when N is compile-time known.

    This is the length source the whole III\u2463 rule produces: it exists to
    turn a raw-pointer walk into a slice, and the slice it writes is always
    built here. Reading only `[T; N]` declarations means the guard can never
    see the length of anything this rule creates, so every III\u2463 rewrite of a
    loop looks like it added an unprovable check — the rule and the guard end
    up structurally opposed. Measured: `aptx_qmf_polyphase_analysis`, whose
    slices are `from_raw_parts(_, NB_FILTERS)` walked by `while i < NB_FILTERS`,
    was rejected on all three of its subscripts.

    A runtime length still proves nothing and abandons the proof.
    """
    node = _strip_casts(_unwrap_block(value))
    if node is None or node.type != "call_expression":
        return None
    fn = node.child_by_field_name("function")
    if fn is None:
        return None
    tail = fn.text.decode("utf-8", "replace").rsplit("::", 1)[-1].strip()
    if tail not in ("from_raw_parts", "from_raw_parts_mut"):
        return None
    args = node.child_by_field_name("arguments")
    named = [c for c in args.children if c.is_named] if args else []
    if len(named) != 2:
        return None
    return _const_fold(named[1])


def _length_base(node) -> str:
    """`X`, `X + k`, `X.wrapping_add(k)` with `k` compile-time known — X.

    The identifier a slice length, or a subscript, is measured from, casts
    peeled. Anything else (a product, a field, a call) returns "".
    """
    node = _strip_casts(node)
    if node is None:
        return ""
    name = _ident(node)
    if name:
        return name
    if node.type == "binary_expression" and _binary_op(node) == "+":
        operands = [c for c in node.children if c.is_named]
        if len(operands) == 2:
            if _const_fold(operands[1]) is not None:
                return _length_base(operands[0])
            if _const_fold(operands[0]) is not None:
                return _length_base(operands[1])
        return ""
    if node.type == "call_expression":
        fn = node.child_by_field_name("function")
        args = node.child_by_field_name("arguments")
        if fn is not None and fn.type == "field_expression" and args is not None:
            field = fn.child_by_field_name("field")
            method = field.text.decode("utf-8", "replace") if field is not None else ""
            named = [c for c in args.children if c.is_named]
            if (method in _MONOTONE_ADD and len(named) == 1
                    and _const_fold(named[0]) is not None):
                return _length_base(fn.child_by_field_name("value"))
    return ""


def find_index_derived_slice_views(source: str) -> list[str]:
    """Slice views whose length is derived from the index that reads them.

        let s = unsafe { from_raw_parts_mut(out, i.wrapping_add(1)) };
        s[i] = v;

    The length is not the buffer's size; it is invented from the subscript, so
    it bounds nothing the subscript did not already assume. What it does add
    is a bounds check per access that cannot be folded (`wrapping_add` may
    wrap). The raw-pointer original paid none, and the III④ card already
    forbids a view that serves a single access.

    Measured: one such rewrite of a per-pixel helper gave it 31 bounds checks
    where it had 0 and made a conversion op execute 13.8% more instructions,
    while the benchmark that admitted it exercised other branches (+0.8%).

    Only a view that exists SOLELY to be subscripted at its own length's base
    is flagged: every later use of the binding in its block must be such a
    subscript. A view that is also iterated, sliced, or read through another
    index (`get_unchecked(i)` over a real length) serves a purpose and passes.
    """
    data = source.encode("utf-8")
    root = get_parser().parse(data).root_node
    lets = []
    stack = [root]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "let_declaration":
            lets.append(n)
    findings: list[str] = []
    for let in sorted(lets, key=lambda n: n.start_byte):
        pattern = let.child_by_field_name("pattern")
        value = let.child_by_field_name("value")
        if pattern is None or value is None:
            continue
        name = pattern.text.decode("utf-8", "replace").strip()
        name = name.removeprefix("mut ").strip()
        if not name.isidentifier():
            continue
        call = _strip_casts(_unwrap_block(value))
        if call is None or call.type != "call_expression":
            continue
        fn = call.child_by_field_name("function")
        tail = (fn.text.decode("utf-8", "replace").rsplit("::", 1)[-1].strip()
                if fn is not None else "")
        if tail not in ("from_raw_parts", "from_raw_parts_mut"):
            continue
        args = call.child_by_field_name("arguments")
        named = [c for c in args.children if c.is_named] if args else []
        if len(named) != 2 or _const_fold(named[1]) is not None:
            continue
        base = _length_base(named[1])
        if not base:
            continue
        # Uses after the `let`, in the block that owns it. Sibling branches
        # commonly reuse the binding name for their own view; they are other
        # bindings and must not be read as uses of this one.
        scope = let.parent if let.parent is not None else root
        uses = []
        walk = [scope]
        while walk:
            m = walk.pop()
            walk.extend(m.children)
            if (m.type == "identifier" and m.start_byte >= let.end_byte
                    and m.text.decode("utf-8", "replace") == name):
                uses.append(m)
        if not uses:
            continue
        derived = []
        for use in uses:
            parent = use.parent
            if parent is not None and parent.type == "index_expression":
                sub_base, subscript = _index_parts(parent)
                if (sub_base is not None and sub_base.start_byte == use.start_byte
                        and sub_base.end_byte == use.end_byte
                        and _length_base(subscript) == base):
                    derived.append(parent)
                    continue
            derived = []
            break
        if derived:
            length_text = named[1].text.decode("utf-8", "replace")
            reads = ", ".join(sorted({d.text.decode("utf-8", "replace") for d in derived}))
            findings.append(
                f"`{name}` = {tail}(_, {length_text}) is only read as {reads}: its "
                f"length is derived from the index `{base}` it guards, so it bounds "
                "nothing and adds a bounds check per access. Keep the raw-pointer "
                "access here, or build one view over the buffer's real length "
                "outside the per-element path.")
    return findings


def _local_array_len(fn_node, name: str) -> tuple[bool, Optional[int]]:
    """`let mut a: [T; N]` in this function — (name is bound here, N).

    Returning "is it bound here" separately from "how long" is what lets a
    local shadow a module item correctly: a same-named local that is NOT a
    fixed-size array must ABANDON the proof, not fall through to a `static`
    the code demonstrably is not reading.

    The smallest N wins when the name is bound more than once — a bound has to
    hold for every declaration the subscript could be reading.
    """
    if fn_node is None or not name:
        return False, None
    found = False
    best: Optional[int] = None
    stack = [fn_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type != "let_declaration":
            continue
        pattern = n.child_by_field_name("pattern")
        if pattern is None:
            continue
        text = pattern.text.decode("utf-8", "replace").strip()
        if text.removeprefix("mut ").strip() != name:
            continue
        found = True
        length = _array_type_len(n.child_by_field_name("type"))
        if length is None:
            length = _slice_view_len(n.child_by_field_name("value"))
        if length is None:
            return True, None
        best = length if best is None else min(best, length)
    return found, best


def _module_array_len(root, name: str) -> Optional[int]:
    """`static A: [T; N]` / `const A: [T; N]` anywhere in the file — N.

    c2rust hoists C's file-scope tables to module items, so the array a hot
    loop walks is as often a `static` as a local. Measured on lodepng
    `Adam7_deinterlace`: `static mut ADAM7_DX: [c_uint; 7]` indexed by a
    `while i != 7` induction variable was reported as an added bounds check,
    which is the same shape as the local-array case and just as foldable —
    the proof was skipped only because nothing looked outside the function.

    Same conservatism as the local lookup: a same-named item that is not a
    fixed-size array abandons the proof, and the smallest N wins when the
    name appears more than once (different modules, same file).
    """
    if root is None or not name:
        return None
    best: Optional[int] = None
    stack = [root]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type not in ("static_item", "const_item"):
            continue
        ident = n.child_by_field_name("name")
        if ident is None:
            continue
        if ident.text.decode("utf-8", "replace").strip() != name:
            continue
        length = _array_type_len(n.child_by_field_name("type"))
        if length is None:
            return None
        best = length if best is None else min(best, length)
    return best


def _array_len_in_scope(base, name: str) -> Optional[int]:
    """The fixed length of the array `name` denotes at this subscript.

    Locals shadow module items, so the function is consulted first and its
    answer is final either way.
    """
    if not name:
        return None
    found, local = _local_array_len(_enclosing_function(base), name)
    if found:
        return local
    return _module_array_len(_tree_root(base), name)


def _loop_upper_bound(loop_node, var: str) -> Optional[int]:
    """`while v != N` / `v < N` / `v <= N` — the first value `v` cannot hold.

    N may be a literal or a module const; see `_const_fold`.
    """
    if loop_node is None or loop_node.type != "while_expression":
        return None
    cond = _strip_casts(loop_node.child_by_field_name("condition"))
    if cond is None or cond.type != "binary_expression":
        return None
    operands = [c for c in cond.children if c.is_named]
    if len(operands) != 2 or _ident(operands[0]) != var:
        return None
    bound = _const_fold(operands[1])
    if bound is None:
        return None
    op = _binary_op(cond)
    if op in ("!=", "<"):
        return bound
    if op == "<=":
        return bound + 1
    return None


def _increments_monotonically(loop_node, var: str) -> bool:
    """True when the loop's only writes to `var` step it up by a literal.

    A second assignment from anywhere else, or the variable's address handed
    out by `&mut`, and the induction argument is off — so both give up rather
    than guess.
    """
    body = loop_node.child_by_field_name("body")
    if body is None:
        return False
    stack = [body]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "reference_expression":
            text = n.text.decode("utf-8", "replace")
            if "mut" in text and _ident(n.child_by_field_name("value")) == var:
                return False
            continue
        if n.type == "compound_assignment_expr":
            if _ident(n.child_by_field_name("left")) != var:
                continue
            if _binary_op(n) != "+=":
                return False
            step = _literal_int(n.child_by_field_name("right"))
            if step is None or step <= 0:
                return False
            continue
        if n.type != "assignment_expression":
            continue
        if _ident(n.child_by_field_name("left")) != var:
            continue
        if not _is_step_up(n.child_by_field_name("right"), var):
            return False
    return True


def _is_step_up(node, var: str) -> bool:
    """`v.wrapping_add(1)` / `v + 1` — a positive literal step on `v` itself."""
    node = _strip_casts(node)
    if node is None:
        return False
    if node.type == "call_expression":
        fn = node.child_by_field_name("function")
        if fn is None or fn.type != "field_expression":
            return False
        field = fn.child_by_field_name("field")
        if field is None:
            return False
        if field.text.decode("utf-8", "replace") not in _MONOTONE_ADD:
            return False
        if _ident(fn.child_by_field_name("value")) != var:
            return False
        args = node.child_by_field_name("arguments")
        named = [c for c in args.children if c.is_named] if args else []
        if len(named) != 1:
            return False
        step = _literal_int(named[0])
        return step is not None and step > 0
    if node.type == "binary_expression" and _binary_op(node) == "+":
        operands = [c for c in node.children if c.is_named]
        if len(operands) == 2 and _ident(operands[0]) == var:
            step = _literal_int(operands[1])
            return step is not None and step > 0
    return False


def _alias_source(loop_node, var: str) -> str:
    """`let slot = type_2 as usize;` — the name `slot` is standing in for.

    One hop, and only when the loop binds the alias exactly once and never
    assigns to it again. c2rust does not write this shape; rewrites do, to
    give a name to a subscript they use more than once.
    """
    body = loop_node.child_by_field_name("body")
    if body is None:
        return ""
    source = ""
    bindings = 0
    stack = [body]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type in ("assignment_expression", "compound_assignment_expr"):
            if _ident(n.child_by_field_name("left")) == var:
                return ""       # reassigned — not a stable alias
            continue
        if n.type != "let_declaration":
            continue
        pattern = n.child_by_field_name("pattern")
        if pattern is None:
            continue
        text = pattern.text.decode("utf-8", "replace").strip()
        if text.removeprefix("mut ").strip() != var:
            continue
        bindings += 1
        source = _ident(n.child_by_field_name("value"))
    return source if bindings == 1 else ""


def _descending_index(subscript) -> tuple[Optional[int], str]:
    """`C - i` / `C.wrapping_sub(i)` — (C, "i"), when C is compile-time known.

    A C loop that counted down reaches c2rust as an ascending induction
    variable subtracted from a constant, so the subscript is not the variable
    and the plain-name lookup gives up before any bound is considered. The
    caller supplies the range check; this only reports the shape.
    """
    node = _strip_casts(subscript)
    if node is None:
        return None, ""
    if node.type == "call_expression":
        fn = node.child_by_field_name("function")
        if fn is None or fn.type != "field_expression":
            return None, ""
        field = fn.child_by_field_name("field")
        if field is None:
            return None, ""
        if field.text.decode("utf-8", "replace") not in _MONOTONE_SUB:
            return None, ""
        origin = _const_fold(fn.child_by_field_name("value"))
        if origin is None:
            return None, ""
        args = node.child_by_field_name("arguments")
        named = [c for c in args.children if c.is_named] if args else []
        if len(named) != 1:
            return None, ""
        var = _ident(named[0])
        return (origin, var) if var else (None, "")
    if node.type == "binary_expression" and _binary_op(node) == "-":
        operands = [c for c in node.children if c.is_named]
        if len(operands) == 2:
            origin = _const_fold(operands[0])
            var = _ident(operands[1])
            if origin is not None and var:
                return origin, var
    return None, ""


def _is_bounded_by_enclosing_loop(base, subscript, loop_node) -> bool:
    """True when a `while` holds this subscript below a fixed array's length.

    The shape `_is_provably_in_range` cannot see, because it is handed the
    subscript alone and this proof needs the array and the loop too:

        let mut count: [c_uint; 256] = [0; 256];
        while x != 256 as c_uint {
            sum = sum.wrapping_add(count[x as usize] as c_ulong);
            x = x.wrapping_add(1);
        }

    LLVM's induction-variable analysis folds that check away, so flagging it
    buys nothing and costs a candidate — the same trade this module already
    makes for constant and masked subscripts, and for the same measured
    reason.

    Measured, on lodepng's `filter`: the guard reported
    `count[x as usize], count[type_2 as usize], count[type_2 as usize]` and
    rejected a 94-line rewrite that had used `get_unchecked` for the one
    raw-pointer array it actually touched. All three subscripts are in the
    c2rust output verbatim, each bounded by its own `while` against a literal,
    each reading a `[c_uint; 256]` declared in the same function. The repair
    turn spent on that rejection produced a replacement that measured +0.130%.

    Deliberately narrow, because the cost of excusing a check LLVM really does
    emit is the one `find_added_bounds_checks` exists to prevent: the loop must
    be a `while` comparing the index variable itself to an integer literal, the
    array's length must be a literal in the same function, and the body's only
    writes to that variable must step it up by a positive literal. A runtime
    bound (`while x as size_t != linebytes`) proves nothing here and stays
    flagged.
    """
    var = _ident(subscript)
    origin: Optional[int] = None
    if not var:
        origin, var = _descending_index(subscript)
        if not var:
            return False
    induction = var
    bound = _loop_upper_bound(loop_node, var)
    if bound is None:
        # The subscript may be a name the loop body gave the induction
        # variable: `let slot = type_2 as usize;`. Resolve one hop and ask
        # the same question of what it stands for.
        induction = _alias_source(loop_node, var)
        if not induction:
            return False
        bound = _loop_upper_bound(loop_node, induction)
        if bound is None:
            return False
    length = _array_len_in_scope(base, _ident(base))
    if length is None:
        return False
    if origin is None:
        if bound > length:
            return False
    else:
        # `origin - v`, with v in [0, bound). The largest subscript is `origin`
        # and the smallest is `origin - bound + 1`, so the walk stays inside
        # the array when `origin < length`, and stays non-negative — no
        # `wrapping_sub` underflow into a huge index — when `bound <= origin+1`.
        if origin >= length or bound > origin + 1:
            return False
    return _increments_monotonically(loop_node, induction)


def _loop_enclosed_indexes(source: bytes):
    """Every `x[i]` a loop re-executes whose bound the compiler cannot
    see, as (line, text)."""
    try:
        root = get_parser().parse(source).root_node
    except Exception:
        return []
    out = []
    stack = [(root, None)]
    while stack:
        node, in_loop = stack.pop()
        if in_loop is not None and node.type == "index_expression":
            base, subscript = _index_parts(node)
            if (not _is_provably_in_range(subscript)
                    and not _is_bounded_by_enclosing_loop(
                        base, subscript, in_loop)):
                out.append((
                    node.start_point[0] + 1,
                    node.text.decode("utf-8", "replace").strip(),
                ))
        if node.type == "for_expression":
            # `for pat in ITER { body }` — ITER is evaluated once, before the
            # loop starts, so an index inside it is not a per-iteration cost.
            # Measured: counting the `for` header as loop-inner flags the
            # idiomatic "slice once, then iterate"
            #   for (d, c) in out[..n].iter_mut().zip(inp[..m].chunks(8))
            # whose two slicings are hoisted by construction. It rejected
            # libqrencode's `BitStream_toByte` rewrite that the previous
            # run had committed, and the crate landed at -4.969% instead
            # of -5.836%. `while`'s condition IS re-evaluated per
            # iteration, so it keeps its full span.
            #
            # Compare spans, not identity: `children` and `child_by_field_name`
            # hand back distinct Python wrappers for the same node, so `is`
            # never matches and the body would lose its loop flag.
            body = node.child_by_field_name("body")
            span = (body.start_byte, body.end_byte) if body is not None else None
            stack.extend(
                (child,
                 node if (span is not None
                          and (child.start_byte, child.end_byte) == span)
                 else in_loop)
                for child in node.children)
        elif node.type in _LOOP_NODES:
            stack.extend((child, node) for child in node.children)
        else:
            stack.extend((child, in_loop) for child in node.children)
    return out


def find_added_bounds_checks(code: str) -> list[str]:
    """Indexing expressions the rewrite put inside a hot loop.

    c2rust emits `*p.offset(i)` — unchecked. Rewriting that as `slice[i]`
    inside a loop adds a bounds check on EVERY iteration, an expense the
    original code never paid and the user never asked for.

    Measured, on one crate's hottest inner loop: two rewrites of the same
    lines under the same rules, differing only here. `get_unchecked` gave
    -8.44%; the plain indexed form gave +0.01%. A third build, identical to the
    slow one except for this single expression, recovered -7.55%. The bounds
    check alone was worth 8.35 percentage points.

    Nor is a loop-invariant `assert!` a way out: adding one outside the loop
    so the compiler could fold the check made it 2.18% SLOWER still — LLVM did
    not propagate the bound, and the assert's own branch stayed.

    Replayed over 24 committed rewrites from five runs, this predicate agreed
    with the measurement on all three that had one: it passed the -8.44%
    rewrite and flagged both that measured +1.02% and +0.01%.
    """
    if not code:
        return []
    return [
        text[:60]
        for _, text in _loop_enclosed_indexes(code.encode("utf-8"))
        if "get_unchecked" not in text
    ]


# How many sites one finding may name. High enough that a real rewrite is
# described in full; capped so a pathological diff cannot push the prompt out
# of budget.
_ADDED_CHECK_REPORT_CAP = 20


def describe_added_bounds_checks(added: list[str]) -> str:
    """The finding text: every site, not just the first few.

    The repair turn is driven by this string — the model corrects what it is
    shown. Naming 3 of 14 means the corrected reply still carries the other
    11, is rejected again, and the second turn (the last one) is spent the
    same way. Measured on lodepng's `filter`: two turns, both rejected, on a
    finding that listed three sites out of four; and on `Adam7_deinterlace`,
    where the guard had found fourteen.
    """
    seen: list[str] = []
    for text in added:
        if text not in seen:
            seen.append(text)
    head = seen[:_ADDED_CHECK_REPORT_CAP]
    body = ", ".join(head)
    if len(seen) > len(head):
        body += f", ... and {len(seen) - len(head)} more"
    return ("rewrite adds a bounds check inside a loop the original "
            f"indexed unchecked ({len(seen)} site(s)): {body}")


def find_added_bounds_checks_in_crate(crate: Path) -> list[str]:
    """Same question as `find_added_bounds_checks`, asked of the working tree.

    It has to be asked here, not of the model's response. A region rewrite
    returns only the statements inside the loop — the `while` itself is not in
    the fragment — so parsing the response alone can never tell whether an
    index sits in a loop. Measured: run against the two real transcripts this
    check exists for, the fragment-based version found nothing in either, and
    would have let the +0.01% rewrite through exactly as before.

    So: read the uncommitted diff (the rewrite that was just applied), take
    the added lines that index, and locate each one in the resulting file to
    decide whether a loop encloses it.

    Only indexes whose bound the compiler cannot see are reported; see
    `_is_provably_in_range`.
    """
    try:
        diff = subprocess.run(
            ["git", "-C", str(crate), "diff", "--unified=0"],
            capture_output=True, text=True, timeout=60).stdout
    except (OSError, subprocess.SubprocessError):
        return []
    if not diff:
        return []
    added: set[str] = set()
    files: list[str] = []
    for line in diff.split("\n"):
        if line.startswith("+++ b/"):
            files.append(line[6:])
        elif line.startswith("+") and not line.startswith("+++"):
            body = line[1:]
            if _INDEX_RE.search(body) and "get_unchecked" not in body:
                added.add(body.strip())
    if not added:
        return []
    found: list[str] = []
    for rel in files:
        path = crate / rel
        try:
            source = path.read_bytes()
        except OSError:
            continue
        lines = source.decode("utf-8", "replace").split("\n")
        for lineno, text in _loop_enclosed_indexes(source):
            if "get_unchecked" in text:
                continue
            if lineno - 1 < len(lines) and lines[lineno - 1].strip() in added:
                found.append(text[:60])
    return found


# A view whose length is not known but SCANNED for. `CStr::from_ptr` walks to
# the NUL before it can hand back a pointer+len; `to_bytes` / `to_str` on the
# result do the same if the scan was deferred. `strlen` is the C spelling.
_SCANNED_VIEW_CTORS = (
    "CStr::from_ptr",
    "from_bytes_until_nul",
    "from_bytes_with_nul",
    "strlen(",
)


def find_introduced_scans_in_crate(crate: Path) -> list[str]:
    """Length scans the rewrite introduced that the original did not perform.

    The rewrite contract already forbids this in prose — "a view whose length
    must be SCANNED to exist pays that scan on EVERY call — never build one in
    a hot function, however clean the resulting code looks" — and nothing
    enforced it. Measured: one crate's `precompute_bonus` walks a NUL
    terminated string with `while *h.offset(i) != 0`, so the loop IS the scan.
    The rewrite prefixed it with `CStr::from_ptr(h).to_bytes()`, traversing the
    string twice, and measured **+7.818%**. It was rejected by W2 with no
    diagnosis and no retry.

    "Introduced" is the operative word, and it is why this reads the diff
    rather than the file: a function whose original already called `strlen`
    pays nothing new for a view built on that length. Only a scan the rewrite
    added is a cost the rewrite is responsible for.
    """
    try:
        diff = subprocess.run(
            ["git", "-C", str(crate), "diff", "--unified=0"],
            capture_output=True, text=True, timeout=60).stdout
    except (OSError, subprocess.SubprocessError):
        return []
    if not diff:
        return []
    added: list[str] = []
    removed: list[str] = []
    for line in diff.split("\n"):
        if line.startswith("+++") or line.startswith("---"):
            continue
        body = line[1:]
        if body.lstrip().startswith("//"):     # a SAFETY note is not a scan
            continue
        if line.startswith("+"):
            added.append(body)
        elif line.startswith("-"):
            removed.append(body)

    # Count, do not merely look. Replacing `strlen(p)` with
    # `CStr::from_ptr(p)` introduces nothing — the original already walked to
    # the NUL, and swapping one spelling for another is the rewrite doing its
    # job. Checked against every committed rewrite in the dataset: the
    # presence test flagged 11 of 128, all of them substitutions of exactly
    # this kind, in three projects that had measured them a net win.
    def _scans(lines: list[str]) -> int:
        return sum(line.count(ctor) for line in lines
                   for ctor in _SCANNED_VIEW_CTORS)

    if _scans(added) <= _scans(removed):
        return []
    return [line.strip()[:70] for line in added
            if any(ctor in line for ctor in _SCANNED_VIEW_CTORS)]


def find_undeclared_executions(applied: list[str], code: str) -> list[str]:
    ""                                                           

                                                
       
    if not code or not applied:
        return []
    undeclared: list[str] = []
    for rule in applied:
        rule = rule.strip()
                                                       
        main = rule.split(".", 1)[0]
        fps = _RULE_EXEC_FINGERPRINTS.get(main)
        if not fps:                                  
            continue
        if not any(fp in code for fp in fps):
            undeclared.append(rule)
    return undeclared


def parse_skipped_rules(code: str) -> list[tuple[str, str]]:
    ""                                                                

                                                    
                                                              
                                              

                                                          
       
    if not code:
        return []
    m = _SKIPPED_RULES_RE.search(code)
    if not m:
        return []
    inner = m.group(1).strip()
    if _is_empty_rule_list(inner):
        return []
    out: list[tuple[str, str]] = []
    for part in _split_top_level(inner, ";"):
        part = part.strip()
        if not part:
            continue
        split = _split_rule_and_reason(part)
        if split is not None:
            rule, reason = split
            out.append((rule.strip(), reason.strip()))
        elif out:
            # A fragment with no `<rule>:` prefix is not an entry — it is the
            # tail of the previous reason, cut off at a semicolon the model
            # used the way English uses one, to join two clauses:
            #
            #   [III②: no alloc/free pair to replace; the function only
            #    releases existing ownership]
            #
            # Taking the tail as an entry invented a rule id nobody offered,
            # and `declared - candidates` then discarded the whole rewrite as
            # `unknown_declared_rule`. Measured across the corpus: 62 such
            # fragments, every one of them ordinary prose. The contract picked
            # `;` as its separator and the model is not wrong to also use it
            # as punctuation, so the parser is where this gets reconciled.
            #
            # `_split_top_level` already exempts semicolons inside brackets
            # and backticks; this covers the bare ones it cannot see.
            rule, reason = out[-1]
            out[-1] = (rule, f"{reason}; {part}" if reason else part)
        else:
            out.append((part, ""))
    return out


def parse_plan_json(response: str) -> tuple[Optional[dict], Optional[str]]:
    ""                                                 

                                                          
       
    import json as _json
    if not response or not response.strip():
        return None, "empty_response"

                                      
    m = _ABSTAIN_RE.search(response)
    if m:
        return None, f"abstain: {m.group(1).strip()}"

                                                       
    json_fence = re.compile(r"```(?:json)?\s*\n(.*?)\n```", re.DOTALL)
    m = json_fence.search(response)
    if not m:
        return None, "no_json_fence"
    raw = m.group(1).strip()
    try:
        plan = _json.loads(raw)
    except _json.JSONDecodeError as e:
        return None, f"json_parse_error: {e}"
    if not isinstance(plan, dict):
        return None, "plan_not_object"
    if "applied_rules" not in plan:
        return None, "plan_missing_applied_rules"
    return plan, None


                                                                         

# LLVM inline remark message patterns — LLVM emits multiple phrasings:
#   * TooCostly:      "'paeth' not inlined into 'filter' because too costly ..."
#   * NoDefinition:   "calloc will not be inlined into ... because its definition is unavailable"
#   * NeverInline:    "foo will not be inlined into ... because it should never be inlined"
# Note: NoDefinition/NeverInline callees are usually libc/stdlib (definition
# not visible or #[inline(never)]/#[cold]) — after regex extract, fn_index
# lookup will filter them out. But NOT catching them here means we can't
# even LOG that we tried.
_INLINE_MSG_RES = (
    re.compile(r"'([\w:]+)'\s+not inlined into"),           # TooCostly
    re.compile(r"(\w+)\s+will not be inlined into"),         # NoDefinition / NeverInline
)


def _demangle_rust_legacy(mangled: str) -> Optional[str]:
    """Demangle a legacy Rust `_ZN<n><name>...<17h<hash>>E` symbol to its final
    fn base name (the segment right before the hash). Returns None if input
    isn't recognized as legacy Rust mangling.

    Example:
        _ZN11fzy_cleaned3src7choices20choices_reset_search17h66e53447a1302647E
        → 'choices_reset_search'
    """
    if not (mangled.startswith("_ZN") and mangled.endswith("E")):
        return None
    body = mangled[3:-1]
    segs: list[str] = []
    i = 0
    while i < len(body):
        j = i
        while j < len(body) and body[j].isdigit():
            j += 1
        if j == i:
            return None
        n = int(body[i:j])
        if j + n > len(body):
            return None
        segs.append(body[j:j + n])
        i = j + n
    # Trailing "h<hexhash>" (17 chars, starts with 'h', rest hex) is the
    # per-symbol hash — drop.
    if segs and len(segs[-1]) == 17 and segs[-1][0] == "h" \
            and all(c in "0123456789abcdef" for c in segs[-1][1:]):
        segs.pop()
    return segs[-1] if segs else None


def _extract_inline_callee_names(remarks: list[dict]) -> list[str]:
    """Callee names from ep.llvm_opt_remarks (pass=inline, status=missed).
    Handles quoted 'foo' not inlined + unquoted foo will not be inlined
    phrasings. **Demangles legacy Rust `_ZN...E` to base name** so it can
    resolve via fn_index (which is keyed by base names, per hot_probe
    convention). Ordered by appearance; caller can uniq."""
    out: list[str] = []
    for r in remarks:
        if r.get("pass") != "inline" or r.get("status") != "missed":
            continue
        msg = r.get("message") or ""
        for pat in _INLINE_MSG_RES:
            m = pat.search(msg)
            if m:
                raw = m.group(1)
                demangled = _demangle_rust_legacy(raw)
                out.append(demangled if demangled else raw)
                break
    return out


                                                                       

                              
_IN_FN_RULES = {"C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C10", "II_vec"}            
_CALLEE_SIDE_RULES = {"II_inl"}                                         

                                                     
_DO_NOT_INLINE_MARKERS = ("#[inline(never)]", "#[cold]")


def _rule_base(rule_id: str) -> str:
    ""                                                        
                                                                     
                                       
    return rule_id.split(".", 1)[0]


def _has_do_not_inline_attr(file_bytes: bytes, span: tuple[int, int]) -> bool:
    """True iff the span's leading attribute region contains #[inline(never)]
    or #[cold]. Simple substring check — c2rust attrs are always single-item
    `#[...]`, no multi-attr macros to parse."""
    text = file_bytes[span[0]:span[1]].decode("utf-8", errors="replace")
    return any(m in text for m in _DO_NOT_INLINE_MARKERS)


def resolve_edit_target(hf: HotFunction, ep: EvidencePack, rule_id: str,
                         fn_index: FnIndex,
                         crate: Path) -> Optional[EditTarget]:
    ""           
                                                       
                                                                                   
                                                 
                                                                    

                                                               
                                                         
       
    base = _rule_base(rule_id)

                       
    if base in _IN_FN_RULES:
        if hf.file is None:
            return None
        found = locate_fn_span_and_name(
            crate / hf.file, hf.name,
            hint_line_range=(hf.line_start, hf.line_end),
        )
        if found is None:
            return None
        start_byte, end_byte, source_name = found
        # Name it as the SOURCE does, not as perf does. `hf.name` is the LINK
        # name, and everything downstream of this target talks about source:
        # the prompt asks the model to reproduce this function, and the FORM
        # check compares what it wrote against this field. With the link name
        # the model is asked for `fn match` where the file says `fn match_0`,
        # writes the correct source name, and is rejected for it —
        # "expected function match, got match_0".
        return EditTarget(file=crate / hf.file, fn_name=source_name,
                          span=(start_byte, end_byte))

    # Callee-side: II_inl.a — find inlining-rejected callee.
    # v1 picks the FIRST editable callee (has definition in crate; lacks
    # #[inline(never)] / #[cold]). Not the highest-rejection-count one —
    # accepted v1 limitation, see impl_plan §8.7.
    if base in _CALLEE_SIDE_RULES:
        callees = _extract_inline_callee_names(ep.llvm_opt_remarks)
        if not callees:
            logger.info("[applier] %s/II_inl: 0 inline-missed callee names "
                         "extractable from ep.llvm_opt_remarks (%d remark(s))",
                         hf.name, len(ep.llvm_opt_remarks))
            return None
        libc_skip = 0
        attr_skip = 0
        span_skip = 0
        for callee_name in callees:
            loc = fn_index.resolve(callee_name)
            if loc is None:
                libc_skip += 1
                continue          # stdlib / external — cannot edit, try next
            file_rel, ls, le = loc
            callee_file = crate / file_rel
            span = locate_fn_span_with_attrs(
                callee_file, callee_name, hint_line_range=(ls, le),
            )
            if span is None:
                span_skip += 1
                continue
                                                         
            if _has_do_not_inline_attr(callee_file.read_bytes(), span):
                attr_skip += 1
                logger.info(
                    "[applier] callee %r has #[inline(never)]/#[cold], skipping",
                    callee_name)
                continue
            return EditTarget(file=callee_file, fn_name=callee_name, span=span)
        logger.info("[applier] %s/II_inl: %d callee(s) considered, all rejected "
                     "(libc/stdlib=%d, locate_fail=%d, do-not-inline=%d)",
                     hf.name, len(callees), libc_skip, span_skip, attr_skip)
        return None      # no editable callee in this fn's inline-missed set

    logger.warning("[applier] unknown rule_id %r; abstaining", rule_id)
    return None


                                                                  

def locate_fn_span_and_name(file_path: Path, fn_name: str,
                            hint_line_range: tuple[int, int]
                            ) -> Optional[tuple[int, int, str]]:
    """`locate_fn_span_with_attrs` plus the name the SOURCE gives the fn.

    `fn_name` may be the LINK name — perf reports that, and it is what
    `HotFunction.name` carries. Everything downstream of the span, however,
    talks about the source: the prompt tells the model which function to
    reproduce, and the FORM check compares the name it wrote against the one
    it was asked for. Handing those the link name asks the model to emit
    `fn match` for a function the file spells `fn match_0` — the model
    correctly writes the source name and the check rejects it:
    "expected function match, got match_0".
    """
    found = _locate_fn_node(file_path, fn_name, hint_line_range)
    if found is None:
        return None
    start_byte, end_byte, name = found
    return (start_byte, end_byte, name)


def locate_fn_span_with_attrs(file_path: Path, fn_name: str,
                                hint_line_range: tuple[int, int]
                                ) -> Optional[tuple[int, int]]:
    """Return (start_byte, end_byte) of the function_item AND its preceding
    attribute_item / line_comment / block_comment siblings, so splice replaces
    the whole attribute-decorated fn.

    On name collision (c2rust `main` + `main_0`), pick the definition whose
    start line is closest to hint_line_range[0] (impl_plan §8.1 heuristic).
    """
    found = _locate_fn_node(file_path, fn_name, hint_line_range)
    return None if found is None else (found[0], found[1])


def _locate_fn_node(file_path: Path, fn_name: str,
                    hint_line_range: tuple[int, int]
                    ) -> Optional[tuple[int, int, str]]:
    """Shared implementation: (start_byte, end_byte, source_fn_name)."""
    try:
        src_bytes = file_path.read_bytes()
    except OSError as e:
        logger.warning("[applier] cannot read %s: %s", file_path, e)
        return None

    from tree_sitter import Language, Parser
    import tree_sitter_rust
    parser = Parser(Language(tree_sitter_rust.language()))
    tree = parser.parse(src_bytes)

    hint_line = hint_line_range[0]
    candidates: list[tuple[int, "object"]] = []
    for node in _iter_function_items(tree.root_node):
        name_node = node.child_by_field_name("name")
        if name_node is None:
            continue
        if (name_node.text.decode("utf-8", errors="replace") != fn_name
                and _exported_symbol(node, src_bytes) != fn_name):
            # `fn_name` comes from perf, which reports the LINK name. c2rust
            # renames any C identifier that is a Rust keyword and exports it
            # back: `match` is emitted as `fn match_0` plus
            # `#[export_name = "match"]`. Matching on the source name alone
            # left `resolve_edit_target` with nothing and the agent logged
            # "no editable target (D16/D17 path), skipping" for a function
            # that is 15.07% of fzy's `choices_fread_search`.
            #
            # The span returned below already swallows preceding attributes,
            # so the `#[export_name]` line is part of what the model is shown
            # and replaces — it has to come back in the rewrite or the symbol
            # disappears, and the build gate catches that if it does not.
            continue
        # tree-sitter start_point is 0-indexed line; hint_line is 1-indexed
        fn_line = node.start_point[0] + 1
        candidates.append((abs(fn_line - hint_line), node))

    if not candidates:
        logger.warning("[applier] fn %r not found in %s", fn_name, file_path)
        return None
    if len(candidates) > 1:
        logger.info("[applier] fn %r has %d defs in %s; picking hint-closest",
                     fn_name, len(candidates), file_path)
    candidates.sort(key=lambda t: t[0])
    node = candidates[0][1]

    # Reverse-scan siblings to include preceding #[attr] / /// doc / /* … */.
    start_byte = node.start_byte
    prev = node.prev_sibling
    while prev is not None and prev.type in (
        "attribute_item", "line_comment", "block_comment",
    ):
        start_byte = prev.start_byte
        prev = prev.prev_sibling

    nn = node.child_by_field_name("name")
    source_name = (src_bytes[nn.start_byte:nn.end_byte]
                   .decode("utf-8", errors="replace") if nn else fn_name)
    return (start_byte, node.end_byte, source_name)


# ─── splice + cargo check ─────────────────────────────────────────────

def _splice(file_path: Path, span: tuple[int, int], new_source: str) -> bytes:
    """Replace file_path[span] with new_source; return original bytes (for rollback)."""
    original = file_path.read_bytes()
    start, end = span
    new_bytes = original[:start] + new_source.encode("utf-8") + original[end:]
    file_path.write_bytes(new_bytes)
    return original


# A cargo diagnostic starts at column 0 with `error:` / `error[E0599]:` /
# `warning:`; everything indented under it — the `-->` locations, the source
# excerpt, the `= note:` lines — belongs to that diagnostic.
_DIAG_HEAD = re.compile(r"^(error|warning)(\[[A-Za-z]+\d+\])?:")

# Stands in for the diagnostics an excerpt had no room for. It is recognised
# on the way back in (`_OMITTED_RE`) so that running the extractor over its own
# output keeps the notice instead of silently eating it — the model must be
# told that what it is reading is a excerpt, or it will fix the five errors it
# can see and resubmit with the other seventeen untouched.
_OMITTED_MARK = "... [{n} more error diagnostic(s) omitted from this excerpt] ..."
_OMITTED_RE = re.compile(r"^\.\.\. \[(\d+) more error diagnostic")
# The last block is cargo's own tally ("could not compile ... due to N previous
# errors"). It is what tells the reader how much is missing, so it is kept even
# when everything before it has to go.
_TALLY_PREFIXES = ("error: could not compile", "error: aborting")
# Marks a single diagnostic that had to be cut mid-way. Only reachable when one
# diagnostic alone exceeds the whole budget — a macro expansion or a type whose
# printed form runs for pages. Keeping a cut version beats keeping none: the
# heading and the first `-->` are what the model needs, and the alternative is
# returning a block longer than the caller asked for.
_CUT_MARK = "... [this diagnostic truncated] ..."


def cargo_error_blocks(stderr: str, *, max_chars: int | None = None) -> str:
    """The `error` diagnostics in cargo's stderr, warnings dropped.

    Filtering line by line does not work here. A warning contributes far more
    lines than its heading — measured on one failed build, 15 `-->` lines
    against 5 `error` lines — so a tail window keeps the warnings' locations
    and drops the errors that explain the failure. The retry prompt is built
    from this text, so what survives the trim decides what the model is told.

    Returns "" when there is no `error` diagnostic to keep, which lets the
    caller fall back to its own trimming for stderr that is not from cargo.

    `max_chars` trims by whole diagnostics from the END, keeping the earliest
    ones plus cargo's closing tally. Trimming from the end is the point: a
    failed build's first error is usually the cause and the rest are its
    cascade, so the tail is the disposable half. Measured on one libxml2
    retry — 22 errors reported, a 2000-char tail window left 5 of them, all on
    one line, with no notice that anything had been dropped; the model fixed
    that line and the next build failed with the 17 it had never seen.
    """
    if not stderr:
        return ""
    blocks: list[list[str]] = []
    keeping = False
    for line in stderr.splitlines():
        if _OMITTED_RE.match(line):
            blocks.append([line])       # a prior excerpt's notice — keep it
            keeping = False
            continue
        head = _DIAG_HEAD.match(line)
        if head:
            keeping = head.group(1) == "error"
            if keeping:
                blocks.append([line])
            continue
        if keeping:
            blocks[-1].append(line)
    if not blocks:
        return ""
    texts = ["\n".join(b).rstrip() for b in blocks]

    # A notice carried in from an earlier excerpt is a running total, not a
    # diagnostic. Pull it out before anything competes for the budget: left in
    # place it sorts last among the droppable blocks, so a second trim would
    # discard it and write its own count — turning "30 omitted" into
    # "1 omitted" and understating the loss by a factor of thirty.
    prior_omitted = 0
    diagnostics: list[str] = []
    for text in texts:
        carried = _OMITTED_RE.match(text)
        if carried:
            prior_omitted += int(carried.group(1))
            continue
        diagnostics.append(text)

    def _render(kept: list[str], dropped: int, tally: str | None) -> str:
        total = prior_omitted + dropped
        if total:
            kept = kept + [_OMITTED_MARK.format(n=total)]
        if tally:
            kept = kept + [tally]
        return "\n".join(kept)

    tally = (diagnostics[-1]
             if diagnostics and diagnostics[-1].startswith(_TALLY_PREFIXES)
             else None)
    body = diagnostics[:-1] if tally else diagnostics
    if max_chars is None or len(_render(body, 0, tally)) <= max_chars:
        return _render(body, 0, tally)

    budget = max_chars - (len(tally) + 1 if tally else 0)
    kept: list[str] = []
    used = 0
    for text in body:
        if kept and used + len(text) + 1 > budget:
            break
        kept.append(text)
        used += len(text) + 1
    if len(kept) == 1 and used > budget:
        kept = [_cut_one_diagnostic(kept[0], budget)]
    return _render(kept, len(body) - len(kept), tally)


def _cut_one_diagnostic(text: str, budget: int) -> str:
    """Trim a single over-long diagnostic to `budget`, by whole lines.

    Cutting by line rather than by character keeps the heading intact, so a
    second pass over this text still recognises it as one `error` block and
    the result stays stable. Without this, one diagnostic longer than the
    entire budget was returned whole: the loop keeps the first block
    unconditionally so that a caller always gets something to act on, and a
    2000-line `-->` list rode straight through an 8000-char limit.
    """
    lines = text.splitlines()
    kept: list[str] = []
    used = 0
    for line in lines:
        if kept and used + len(line) + 1 > budget - len(_CUT_MARK) - 1:
            break
        kept.append(line)
        used += len(line) + 1
    if len(kept) < len(lines):
        kept.append(_CUT_MARK)
    return "\n".join(kept)


def cargo_check(harness_dir: Path, *, timeout_s: int = 120) -> tuple[bool, str]:
    ""                                                                       

                                                                   
                                                     
                                                              
                                          
                
       
    for build_attempt in (1, 2):
        started = time.monotonic()
        try:
            proc = subprocess.run(
                ["cargo", "build", "--release"],
                cwd=str(harness_dir), capture_output=True, text=True,
                timeout=timeout_s,
            )
            break
        except subprocess.TimeoutExpired:
            # A timeout says nothing about the rewrite. The clock ran out —
            # on a cold dependency graph, on a machine busy with the previous
            # candidate's measurement — and the source that caused it is the
            # same source that compiled a minute ago. Build it again before
            # calling it a failure.
            #
            # Measured, lodepng `update_adler32`: the first build timed out at
            # 120s and the second, on identical source, finished. The timeout
            # had already been reported to the model as a compile error by
            # then, with `cargo build timeout after 120s` where the stderr
            # goes, and the turn it bought measured +7.947% — the run's worst
            # regression.
            if build_attempt == 1:
                logger.warning(
                    "[build] timeout after %ds — rebuilding once "
                    "(a timeout is not a compile error)", timeout_s)
                continue
            logger.info("[build] timeout after %ds (twice)", timeout_s)
            return False, f"{BUILD_TIMEOUT_PREFIX} after {timeout_s}s, twice"
    elapsed = time.monotonic() - started
    if proc.returncode == 0:
        logger.info("[build] ok in %.1fs", elapsed)
        return True, ""
    logger.info("[build] FAILED in %.1fs", elapsed)
    stderr = proc.stderr or ""
    summary = cargo_error_blocks(stderr, max_chars=STDERR_EXCERPT_CHARS)
    if not summary:
        summary = "\n".join(stderr.splitlines()[-40:])
    return False, summary


# ─── apply_rewrite orchestrator ───────────────────────────────────────

def apply_rewrite(edit_target: EditTarget, response: str,
                   harness_dir: Path, *, cargo_timeout_s: int = 120,
                   ) -> RewriteOutcome:
    """Compatibility helper for tests and out-of-tree callers.

    The production agent routes rewrites through ``ChangeSetExecutor``.  This
    legacy helper remains temporarily available while downstream callers
    migrate.  LLM response → maybe splice → maybe cargo check →
    RewriteOutcome.

    Restores file to original bytes on any failure (parse_fail / cargo err)
    so caller sees clean working tree modulo the failed attempt.
    """
    code, abstain_reason = parse_llm_response(response)
    if abstain_reason is not None:
        return RewriteOutcome(status=RewriteStatus.ABSTAINED,
                               abstain_reason=abstain_reason)

    # Splice; keep original for rollback
    original_bytes = _splice(edit_target.file, edit_target.span, code)

    ok, stderr = cargo_check(harness_dir, timeout_s=cargo_timeout_s)
    if not ok:
        edit_target.file.write_bytes(original_bytes)      # rollback disk
        return RewriteOutcome(status=RewriteStatus.SYNTAX_ERROR,
                               cargo_stderr=stderr)

    return RewriteOutcome(status=RewriteStatus.APPLIED,
                           committed_source=code)
