""                                                                        

                                                             
                         

                                                      
                                           

   
                  
                                                           
               
                                                     
                                               
                                            
       
     
   

                                                  
                                                 
   

from __future__ import annotations

import json
import logging
import re
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

logger = logging.getLogger("hot_probe.merged_hits")


# ── Schema ────────────────────────────────────────────────────────────────────

@dataclass
class Hit:
    """One rule match at one source site."""
    rule: str              # "C1" | "C2" | "C3" | "II_vec" | "II_inl" | "III①"…
    pattern: str           # kebab-case sub-pattern (see agent_design §Phase 2.4)
    file: str              # crate-relative path
    line: int              # 1-indexed (0 if unresolved)
    col: int               # 1-indexed (0 if unavailable)
    snippet: str           # single-line source ≤ 200 chars
    extra: dict[str, Any] = field(default_factory=dict)


@dataclass
class FnHits:
    """Per-fn aggregation. Only fns with ≥ 1 hit appear."""
    fn: str
    file: str
    line_range: tuple[int, int]     # (line_start, line_end); (0,0) if unknown
    hits: list[Hit] = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "fn": self.fn,
            "file": self.file,
            "line_range": list(self.line_range),
            "hits": [asdict(h) for h in self.hits],
        }


# ── Pattern classification ────────────────────────────────────────────────────

                                                                              
_C1_CALLEE_TO_PATTERN: dict[str, str] = {
    "panic_bounds_check":             "bracket-indexing",
    "slice_start_index_len_fail":     "slice-range",
    "slice_end_index_len_fail":       "slice-range",
    "slice_index_order_fail":         "slice-range",
    "slice_end_index_overflow_fail":  "slice-range",
    "slice_error_fail":               "slice-range",
    "expect_failed":                  "unwrap-expect",
    "unwrap_failed":                  "unwrap-plain",
    "panic_const_div_by_zero":        "division-runtime",
    "panic_const_rem_by_zero":        "division-runtime",
}


def _classify_c1_pattern(callee: str) -> str:
    for key, pat in _C1_CALLEE_TO_PATTERN.items():
        if key in callee:
            return pat
    return "bracket-indexing"           


def _classify_c2_pattern(callee: str) -> str:
    ""                                    

                                                            
                                                     
                                   
                                                   
                                                

                                                                        
       
    c = callee.lower()
    sign = "uint" if "fptoui" in c else "int"
    width = "vector" if ".v" in c else "scalar"
    return f"float-to-{sign}-{width}"


_C12_ZERO_ARRAY_LIT = re.compile(
    r"\[\s*(?:0|0\.|0\.0|0(?:u|i)(?:8|16|32|64|size)|0f(?:32|64))\s*;\s*(\d+)\s*\]")
_C12_MIN_LITERAL_ELEMS = 32


def _c12_has_declared_zero_array(body: str) -> bool:
    """Does this function DECLARE a zero-filled array, or merely zero one?

    Both reach the IR as `llvm.memset`, and the rule only applies to the
    first. LLVM's loop-idiom-recognize rewrites `for (i..n) a[i] = 0;` into a
    memset too, and that memset is the C program's own semantics — the C build
    runs it as well. Deleting it changes behaviour; there is nothing to
    recover. What C12 targets is the zeroing c2rust ADDED because it cannot
    spell "uninitialized", and that always appears in the source as a literal.

    Measured across the corpus: three sites in one compression crate attribute
    to functions whose only zeroing is `*p.offset(i) = 0` inside a loop, with
    no array literal anywhere in the body. Passing those to the model asks it
    to delete a loop the C original also runs.

    The element floor keeps an incidental small literal in a large function
    from vouching for a memset it has nothing to do with: a `[0; 4]` field
    cannot be the source of a 256-byte memset.
    """
    return any(int(m.group(1)) >= _C12_MIN_LITERAL_ELEMS
               for m in _C12_ZERO_ARRAY_LIT.finditer(body))


def _classify_c12_pattern(nbytes: int) -> str:
    """Bucket by size, because the size IS the argument for rewriting it.

    A buffer sized in kilobytes whose used extent is a runtime length is a
    different proposition from a 300-byte one: the first is worth a
    `MaybeUninit` prefix-init even if the prefix is usually most of it, the
    second only pays when the prefix is typically tiny.
    """
    if nbytes >= 4096:
        return "oversized-zero-init-kb"
    if nbytes >= 1024:
        return "oversized-zero-init-page"
    return "oversized-zero-init-small"


def _classify_iii_pattern(rule_num: int, callee_name: str,
                          pattern_kind: str | None = None,
                          form: str = "") -> str:
    ""                                                         
    if rule_num == 4:
        return pattern_kind or "cursor-index-deref"
    if rule_num == 1:
                                                 
                                                                  
                                                         
                                                         
                                       
        #   D       — match-arm pattern-bound
        if form in ("B_field", "C"):
            return "field-callback"
        if form == "D":
            return "match-pattern-callback"
        if form == "E":
                                                        
                                                    
                               
            return "higher-order-libc"
        if form in ("A", "B_param", "B", ""):
            return "param-callback"
        return "param-callback"        
    if rule_num == 2:
                  
        if callee_name in ("malloc",):
            return "malloc-free"
        if callee_name == "calloc":
            return "calloc-free"
        if callee_name == "realloc":
            return "realloc-free"
        if callee_name in ("aligned_alloc", "posix_memalign",
                           "memalign", "valloc"):
            return "aligned-alloc"
                                   
        return "malloc-free"
    if rule_num == 3:
        # mem / str / libm / custom wrapper
                                                                    
                                             
        if callee_name in ("abort", "exit", "_exit", "_Exit", "perror",
                            "strerror", "raise", "__assert_fail",
                            "__assert_perror_fail", "longjmp", "siglongjmp"):
            return "unknown"
        if callee_name.startswith("mem"):
            return "libc-mem"
        if callee_name.startswith("str") or callee_name in (
                "snprintf", "sprintf", "printf", "fprintf"):
            return "libc-str"
        if callee_name in ("sqrt", "cos", "sin", "exp", "log", "pow",
                           "tan", "atan", "atan2", "floor", "ceil"):
            return "libc-libm"
        return "custom-mem-wrapper"
    return "unknown"


# ── iter_sites per scanner ────────────────────────────────────────────────────

def _rel_to_crate(path: str, crate_root: Path | None) -> str:
    ""                                           
                                                                
       
    if not path or crate_root is None:
        return path
    try:
        rel = Path(path).resolve().relative_to(Path(crate_root).resolve())
        return str(rel)
    except (ValueError, OSError):
        return path


def iter_class_i(sr, crate_root: Path | None = None,
                 fn_index=None) -> dict[str, list[Hit]]:
    ""                                               

                                                                   
                                                           
                                               
       
    from perf_opt.hot_probe.class_I.attribute import (
        STDLIB_ONLY, UNRESOLVED, attribute_site,
    )

    _zero_array_cache: dict[str, bool] = {}

    def _c12_declares_zero_array(fn_name: str) -> bool:
        """`_c12_has_declared_zero_array` over the fn body, memoized.

        No source (no crate root, no index, unresolvable name) → keep the
        site: the check exists to remove a known false positive, not to gate
        the rule on the availability of source.
        """
        if fn_name in _zero_array_cache:
            return _zero_array_cache[fn_name]
        keep = True
        if crate_root is not None and fn_index is not None:
            loc = fn_index.resolve(fn_name)
            if loc is not None:
                fp = Path(crate_root) / loc[0]
                try:
                    body = "\n".join(fp.read_text(
                        encoding="utf-8", errors="replace"
                    ).splitlines()[loc[1] - 1:loc[2]])
                except OSError:
                    body = ""
                if body:
                    keep = _c12_has_declared_zero_array(body)
        _zero_array_cache[fn_name] = keep
        return keep

    out: dict[str, list[Hit]] = {}
    if not sr or not sr.sites or sr.tables is None:
        logger.debug("[merged_hits] class_I sites/tables 未填充 —— C1/C2 skipped")
    else:
        tables = sr.tables
                                                                    
                                                             
                                                      
                                                    
        dedup: dict[tuple, Hit] = {}
        for site in sr.sites:
            attr = attribute_site(site, tables)
            if attr.define_fn in (UNRESOLVED, STDLIB_ONLY):
                continue
            loc = tables.locations.get(site.dbg_id) if site.dbg_id else None
            line = loc.line if loc and loc.line else 0
            col = loc.col if loc and loc.col else 0
            file = _rel_to_crate(attr.define_file or "", crate_root)
            if site.rule_id == "C1":
                pattern = _classify_c1_pattern(site.callee)
            elif site.rule_id == "C2":
                pattern = _classify_c2_pattern(site.callee)
            elif site.rule_id == "C12":
                if not _c12_declares_zero_array(attr.define_fn):
                    continue
                pattern = _classify_c12_pattern(int(site.detail or 0))
            else:
                continue
            key = (attr.define_fn, site.rule_id, pattern, file, line, col,
                   site.callee, site.detail)
            if key in dedup:
                dedup[key].extra["hit_count"] = \
                    dedup[key].extra.get("hit_count", 1) + 1
            else:
                extra = {"callee": site.callee, "hit_count": 1}
                if site.rule_id == "C12" and site.detail:
                    # The byte count is the whole point of the hit — without
                    # it the model cannot tell a 10 KB buffer from a 300 B one.
                    extra["zeroed_bytes"] = int(site.detail)
                dedup[key] = Hit(
                    rule=site.rule_id, pattern=pattern,
                    file=file, line=line, col=col, snippet="",
                    extra=extra,
                )
        for (fn_name, *_), hit in dedup.items():
            out.setdefault(fn_name, []).append(hit)

    # C3 (gvn-load-clobbered) is NO LONGER emitted here as a per-fn boolean at
    # the fn-declaration line — that gave the LLM no load to hoist (col 0, no
    # snippet, dropped by the region extractor). It is now emitted per-site at
    # the remark's precise line/col by iter_class_ii, gated by sr.c3_hits.
    return out


def iter_class_ii(sr, opt_remarks: list[dict], fn_index,
                  crate_root: Path | None = None,
                  c3_pass_fns: set[str] | None = None) -> dict[str, list[Hit]]:
    ""                                        

                                                         
                                                                
                      

                                                           
                                                                
                                 
       
    from perf_opt.hot_probe.class_II.rules import classify

    out: dict[str, list[Hit]] = {}
    if not opt_remarks:
        return out

                                       
    range_index: dict[str, list[tuple[int, int, str]]] = {}
    if fn_index is not None:
        for fn_name in fn_index.names():
            loc = fn_index.resolve(fn_name)
            if loc:
                f, l0, l1 = loc
                range_index.setdefault(f, []).append((l0, l1, fn_name))

    def _fn_of(file: str, line: int) -> str | None:
        for l0, l1, fn in range_index.get(file, []):
            if l0 <= line <= l1:
                return fn
        return None

    n_unattributed = 0
    _c3_seen: set[tuple[str, int, int]] = set()
    for r in opt_remarks:
        c = classify(r)
        if c.rule_id is None or getattr(c, "excluded", False):
            continue
        raw_file = r.get("file") or ""
        file = _rel_to_crate(raw_file, crate_root)
        line = r.get("line") or 0
        col = r.get("col") or 0
        fn = _fn_of(file, line)
        if fn is None:
            n_unattributed += 1
            continue
        rid = c.rule_id
        if rid == "C3":
            # C3 (gvn:LoadClobbered / licm:Invalidated): emit at the remark's
            # PRECISE load site (line/col/message), gated by the full P_C3 pass
            # set (class_I's !tbaa gate). Previously C3 was emitted per-fn at
            # the fn-declaration line by iter_class_i — an unlocatable target
            # (col 0, no snippet) the region extractor dropped and the LLM
            # could not act on. De-dup (fn,line,col): pre-link + LTO emit the
            # same remark twice.
            if c3_pass_fns is None or fn not in c3_pass_fns:
                continue
            if (fn, line, col) in _c3_seen:
                continue
            _c3_seen.add((fn, line, col))
            pattern = "gvn-load-clobbered"
        elif rid in ("II_vec", "II_inl"):
            pattern = c.reason or "unknown"
        else:
            continue
        full_message = str(r.get("message", ""))
        message = full_message[:200]
        extra = {"remark_pass": r.get("pass"),
                 "remark_status": r.get("status")}
        if rid == "II_inl":
            # The callee and its cost decide the II_inl edit, and the 200-char
            # snippet routinely cuts both (a mangled caller name alone is ~70
            # characters). Parse them from the whole remark, here, once.
            from perf_opt.agent_perf_opt.planners.inline_attr import (
                inline_remark_facts,
            )
            extra.update({k: v for k, v in inline_remark_facts(full_message).items()
                          if v is not None})
        out.setdefault(fn, []).append(Hit(
            rule=rid, pattern=pattern,
            file=file, line=line, col=col,
            snippet=message,
            extra=extra,
        ))
    if n_unattributed:
        logger.debug("[merged_hits] class_II: %d remark(s) 无法归属 fn "
                     "(可能来自 stdlib/dep,已跳过)", n_unattributed)
    return out


_III_RULE_MAP = {1: "III①", 2: "III②", 3: "III③", 4: "III④"}


                                                                           
                                                                    
                                                              
def _normalize_fn_key(fn_key) -> str:
    ""                                                       
    s = str(fn_key)
    for prefix in ("crate::", "self::", "super::"):
        while s.startswith(prefix):
            s = s[len(prefix):]
                                                                     
    if "::" in s:
        s = s.rsplit("::", 1)[-1]
    return s


def iter_class_iii(sr) -> dict[str, list[Hit]]:
    ""                                                                  
    out: dict[str, list[Hit]] = {}
    if sr is None:
        return out

                         
    static_bound: list[str] = []
    for rule_num, hits_dict in [
        (1, getattr(sr, "iii1_hits", {})),
        (2, getattr(sr, "iii2_hits", {})),
        (3, getattr(sr, "iii3_hits", {})),
    ]:
        rule_id = _III_RULE_MAP[rule_num]
        for fn_key, sites in hits_dict.items():
            fn_name = _normalize_fn_key(fn_key)
            for site in sites:
                form = getattr(site, "form", "")
                pattern = _classify_iii_pattern(rule_num, site.callee_name,
                                                form=form)
                                                                      
                                                  
                if pattern == "unknown":
                    continue
                binding = getattr(site, "binding", "")
                if rule_num == 1 and binding == "static":
                    # The callee is a module-level `static [mut]` hook, which
                    # any translation unit may reassign at run time. III①
                    # replaces an indirect call with a monomorphic one, and
                    # that is only sound if the target cannot change — which
                    # here needs a whole-program argument the rule does not
                    # have. Measured across the dataset, every such site was
                    # declined by the model for exactly this reason, at one
                    # model call apiece; libxml2 alone spent 34 of them.
                    # The site stays in `class_III_hits.json`, so nothing is
                    # lost if the rule later grows a way to prove it.
                    static_bound.append(f"{fn_name}:{site.callee_name}")
                    continue
                extra = {"callee_name": site.callee_name,
                         "marker_seen": site.marker_seen,
                         "form": form}
                if binding:
                    extra["binding"] = binding
                if form:
                    extra["form"] = form
                # Form E's rewrite is judged against the handed-over
                # function's body, which lives in another item. Carry the
                # name through so the prompt can fetch it; without it the
                # model can only abstain on "comparator not visible here".
                callback_fn = getattr(site, "callback_fn", "")
                if callback_fn:
                    extra["callback_fn"] = callback_fn
                out.setdefault(fn_name, []).append(Hit(
                    rule=rule_id, pattern=pattern,
                    file=site.file, line=site.line, col=site.col,
                    snippet="",
                    extra=extra,
                ))

    if static_bound:
        unique = sorted(set(static_bound))
        logger.info(
            "[merged_hits] III① 跳过 %d 个 static-bound 间接调用(全局可替换 "
            "hook,单态化需要跨 TU 证明,不进 fn_hits): %s%s",
            len(unique), ", ".join(unique[:6]),
            f" (+{len(unique) - 6} more)" if len(unique) > 6 else "",
        )

                                                 
                                                              
                                                          
    for fn_key, sites in getattr(sr, "iii4_hits", {}).items():
        fn_name = _normalize_fn_key(fn_key)
        for site in sites:
            out.setdefault(fn_name, []).append(Hit(
                rule="III④", pattern=site.pattern_kind,
                file=site.file, line=site.line, col=site.col,
                snippet=site.snippet,
                extra={},
            ))
    return out


                                                                        
                                                                      
                                                                        
                                                         
                                                          
                                                                  
                                                 

def find_const_promote_hits(crate_root: Path,
                            hot_fn_names: set[str],
                            fn_index) -> dict[str, list[Hit]]:
    ""                                                       

                                                                              
       
    import re
    from perf_opt.hot_probe.static_facts import (
        scan_global_declarations,
        validate_const_promotion_safety,
    )
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    by_name: dict[str, list] = {}
    for fact in scan_global_declarations(crate_root):
        if fact.kind == "static":
            by_name.setdefault(fact.name, []).append(fact)
    promotable = {
        name: facts[0]
        for name, facts in by_name.items()
        if len(facts) == 1
        and validate_const_promotion_safety(crate_root, facts[0]).ok
    }
    if not promotable:
        logger.info("[merged_hits] II_const: no promotable statics")
        return {}
    logger.info("[merged_hits] II_const: %d promotable static(s): %s",
                len(promotable), sorted(promotable)[:8])
    out: dict[str, list[Hit]] = {}
    idents_alt = "|".join(re.escape(k) for k in promotable)
    idents_re = re.compile(rf"\b({idents_alt})\b")
    for fn in hot_fn_names:
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            src = (crate_root / file_rel).read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        lines = src.splitlines()
        if l0 <= 0 or l1 <= 0 or l1 > len(lines):
            continue
        body = "\n".join(lines[l0 - 1: l1])
        seen: dict[str, tuple[int, str]] = {}   # ident → (first-line, snippet)
        for m in idents_re.finditer(body):
            ident = m.group(1)
            if ident in seen:
                continue
            body_prefix = body[: m.start()]
            line_off = body_prefix.count("\n")
            src_line = l0 + line_off
            snippet = lines[src_line - 1].strip() if 0 < src_line <= len(lines) else ""
            seen[ident] = (src_line, snippet[:200])
        if not seen:
            continue
        hits: list[Hit] = []
        for ident, (line, snip) in seen.items():
            fact = promotable[ident]
            hits.append(Hit(
                rule="II_const",
                pattern="static-to-const",
                file=file_rel, line=line, col=1,
                snippet=snip,
                extra={
                    "symbol_name": fact.name,
                    "qualified_name": fact.qualified_name,
                    "symbol_kind": "static",
                    "decl_file": fact.relative_path,
                    "decl_line": fact.line,
                    "declaration_hash": fact.declaration_hash,
                    "type_text": fact.type_text,
                    "rhs_text": fact.rhs_text,
                    "attrs": list(fact.attrs),
                    "visibility": fact.visibility,
                },
            ))
        out[fn] = hits
    logger.info("[merged_hits] II_const: %d hot fn(s) with promotable "
                "reads (total %d hits)",
                len(out), sum(len(v) for v in out.values()))
    return out


                                                                         
                              
#     while p != end && *p == *q { p = p.offset(1); q = q.offset(1); }
                                                                     
                                                        
                                                 
                                                             
import re as _wa_re

_WA_PTR_STEP_1_RE = _wa_re.compile(rb"\.(?:offset|add)\(\s*1\b")

# Which loop nodes the C4/C5/C6 scanners walk.
#
# `loop_expression` is not optional here: c2rust lowers C's
# `do { … } while (cond)` to `loop { … if !cond { break } }`, and do-while is
# the shape C uses for exactly the code these rules target — LZ match
# extension, bit-stream refills, unrolled scan loops. Scanning only
# `while_expression` is blind to all of it.
#
# Measured on optipng: the loops in `deflate_slow` (48.6% self-time) and
# `longest_match` (29.6%) are 100% `loop`, `slide_hash` 100%, `inflate_fast`
# 89% — so C4 reported zero hits on a crate whose hottest function is a
# textbook byte-serial match loop. Hand-applying C4 to that loop measured
# -6.88% on zlib_compress_uncompress with W1 2404/2404 green.
#
# `for_expression` stays out: c2rust rarely emits it, and where it does the
# cursor is already an iterator rather than a raw pointer — the shape these
# rules exist to fix is absent.
_LOOP_SCAN_TYPES = ("while_expression", "loop_expression")


def _wa_unwrap(node):
    ""                                           
    while node is not None and node.type in ("type_cast_expression",
                                             "parenthesized_expression"):
        inner = node.child_by_field_name("value")
        if inner is None:
            named = [c for c in node.children if c.is_named]
            inner = named[0] if named else None
        if inner is None:
            break
        node = inner
    return node


def _wa_is_bare_deref(node) -> bool:
    ""                                       
    return (node is not None and node.type == "unary_expression"
            and any(c.type == "*" for c in node.children))


def _wa_subtree_has_byte_compare(root, stop=None) -> bool:
    ""                                        

                                        
                                                         
                                       
       
    stack = [root]
    while stack:
        n = stack.pop()
        if n is stop:
            continue
        if n.type == "binary_expression" \
                and any(c.type in ("==", "!=") for c in n.children):
            for opd in (c for c in n.children if c.is_named):
                if _wa_is_bare_deref(_wa_unwrap(opd)):
                    return True
        stack.extend(c for c in n.children if c is not stop)
    return False


def _wa_condition_is_byte_compare(while_node, block) -> bool:
    ""                        
    return _wa_subtree_has_byte_compare(while_node, stop=block)


def _wa_subtree_has_break(root) -> bool:
    ""                                            
    stack = [root]
    while stack:
        n = stack.pop()
        if n.type == "break_expression":
            return True
        if n is not root and n.type in (
                "while_expression", "loop_expression", "for_expression"):
            continue
        stack.extend(n.children)
    return False


def _wa_body_byte_compare_break(body) -> bool:
    ""                                                              

                                                     
                                                         
                                                                        
                                                
                                            
       
    stack = [body]
    while stack:
        n = stack.pop()
        if n.type == "if_expression":
            cond = n.child_by_field_name("condition")
            conseq = n.child_by_field_name("consequence")
            if cond is not None and conseq is not None \
                    and _wa_subtree_has_byte_compare(cond) \
                    and _wa_subtree_has_break(conseq):
                return True
        if n is not body and n.type in (
                "while_expression", "loop_expression", "for_expression"):
            continue                                         
        stack.extend(n.children)
    return False


def _wa_is_byte_cursor_loop(while_node, data: bytes) -> bool:
    ""                
                                                         
                                                                          
                                                               
                                           
    body = None
    for c in while_node.children:
        if c.type == "block":
            body = c
    if body is None:
        return False
    body_txt = data[body.start_byte:body.end_byte]
    if b"read_unaligned" in body_txt or b"u64" in body_txt:
        return False                      
    cond_cmp = _wa_condition_is_byte_compare(while_node, body)
    body_cmp = False if cond_cmp else _wa_body_byte_compare_break(body)
    if not (cond_cmp or body_cmp):
        return False
    stmts = list(body.named_children)
    if not stmts:
        return False
    max_stmts = 4 if cond_cmp else 8
    if len(stmts) > max_stmts:
        return False
    steps = sum(1 for s in stmts
                if _WA_PTR_STEP_1_RE.search(data[s.start_byte:s.end_byte]))
    return steps >= 1


def find_word_at_a_time_hits(crate_root: Path, hot_fn_names: set[str],
                             fn_index) -> dict[str, list[Hit]]:
    ""                                   

                                                                         
       
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except (OSError, Exception):
            continue
        hits: list[Hit] = []
        seen: set[int] = set()
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            if n.type in _LOOP_SCAN_TYPES:
                wl = n.start_point[0] + 1
                if l0 <= wl <= l1 and wl not in seen \
                        and _wa_is_byte_cursor_loop(n, data):
                    snip = data[n.start_byte:n.end_byte].split(b"\n", 1)[0]
                    hits.append(Hit(
                        rule="C4", pattern="byte-cursor-loop",
                        file=file_rel, line=wl, col=n.start_point[1] + 1,
                        snippet=snip.decode("utf-8", "ignore").strip()[:200],
                        extra={},
                    ))
                    seen.add(wl)
            stack.extend(n.children)
        if hits:
            out[fn] = hits
    if out:
        logger.info("[merged_hits] C4 word-at-a-time: %d hot fn(s), %d loop(s)",
                    len(out), sum(len(v) for v in out.values()))
    return out


                                                                        
                                                        
#     while i!=n { s1 = s1.wrapping_add(*d); s2 = s2.wrapping_add(s1); }
                                             
                                           
                                                      
_C5_ADD_METHODS = (b"wrapping_add", b"add", b"checked_add", b"saturating_add",
                   b"overflowing_add")


def _c5_added_operand(right, lname: bytes, data: bytes):
    ""                                                             
    if right.type == "call_expression":
        fn = right.child_by_field_name("function")
        args = right.child_by_field_name("arguments")
        if fn is not None and fn.type == "field_expression" and args is not None:
            recv = fn.child_by_field_name("value")
            meth = fn.child_by_field_name("field")
            if recv is not None and meth is not None \
                    and data[recv.start_byte:recv.end_byte] == lname \
                    and data[meth.start_byte:meth.end_byte] in _C5_ADD_METHODS:
                a = list(args.named_children)
                if a:
                    return data[a[0].start_byte:a[0].end_byte]
    if right.type == "binary_expression" \
            and any(c.type == "+" for c in right.children):
        ops = [c for c in right.children if c.is_named]
        if len(ops) == 2:
            l, r = ops
            if data[l.start_byte:l.end_byte] == lname:
                return data[r.start_byte:r.end_byte]
            if data[r.start_byte:r.end_byte] == lname:
                return data[l.start_byte:l.end_byte]
    return None


def _c5_acc_updates(loop_body, data: bytes) -> dict:
    ""                                                              
    updates: dict = {}
    stack = [loop_body]
    while stack:
        n = stack.pop()
        if n is not loop_body and n.type in (
                "while_expression", "for_expression", "loop_expression"):
            continue                                        
        if n.type == "assignment_expression":
            left = n.child_by_field_name("left")
            right = n.child_by_field_name("right")
            if left is not None and left.type == "identifier" and right is not None:
                lname = data[left.start_byte:left.end_byte]
                added = _c5_added_operand(right, lname, data)
                if added is not None:
                    updates[lname] = added
        elif n.type == "compound_assignment_expr":
            left = n.child_by_field_name("left")
            right = n.child_by_field_name("right")
            if left is not None and left.type == "identifier" \
                    and right is not None \
                    and any(c.type == "+=" for c in n.children):
                updates[data[left.start_byte:left.end_byte]] = \
                    data[right.start_byte:right.end_byte]
        stack.extend(n.children)
    return updates


def _c5_is_reduction_recurrence(while_node, data: bytes) -> bool:
    body = None
    for c in while_node.children:
        if c.type == "block":
            body = c
    if body is None:
        return False
    accs = _c5_acc_updates(body, data)
    if len(accs) < 2:
        return False
                                               
    data_accs = {name for name, added in accs.items()
                 if b"*" in added or b"[" in added}
    if not data_accs:
        return False
                                                                    
    for bname, badded in accs.items():
        core = badded.split(b" as ")[0].strip()
        if core in data_accs and core != bname:
            return True
    return False


def find_reduction_reassoc_hits(crate_root: Path, hot_fn_names: set[str],
                                fn_index) -> dict[str, list[Hit]]:
    ""                                         
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except Exception:
            continue
        hits: list[Hit] = []
        seen: set[int] = set()
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            if n.type in _LOOP_SCAN_TYPES:
                wl = n.start_point[0] + 1
                if l0 <= wl <= l1 and wl not in seen \
                        and _c5_is_reduction_recurrence(n, data):
                    snip = data[n.start_byte:n.end_byte].split(b"\n", 1)[0]
                    hits.append(Hit(
                        rule="C5", pattern="serial-reduction-recurrence",
                        file=file_rel, line=wl, col=n.start_point[1] + 1,
                        snippet=snip.decode("utf-8", "ignore").strip()[:200],
                        extra={},
                    ))
                    seen.add(wl)
            stack.extend(n.children)
        if hits:
            out[fn] = hits
    if out:
        logger.info("[merged_hits] C5 reduction reassoc: %d hot fn(s), %d loop(s)",
                    len(out), sum(len(v) for v in out.values()))
    return out


                                                                           
                                            
                                                        
                                                           
                                          
                                                            


def _c6_base_ident(node, data: bytes):
    ""                                                            
    n = node
    for _ in range(12):
        if n is None:
            return None
        if n.type == "identifier":
            return data[n.start_byte:n.end_byte]
        if n.type == "field_expression":
            n = n.child_by_field_name("value"); continue
        if n.type == "call_expression":
            n = n.child_by_field_name("function"); continue
        named = [c for c in n.children if c.is_named]
        n = named[0] if named else None
    return None


def _c6_mutated(loop_body, data: bytes) -> set:
    ""                            
    muts = set()
    stack = [loop_body]
    while stack:
        n = stack.pop()
        if n.type in ("assignment_expression", "compound_assignment_expr"):
            left = n.child_by_field_name("left")
            b = _c6_base_ident(left, data) if left is not None else None
            if b:
                muts.add(b)
        elif n.type == "let_declaration":
            pat = n.child_by_field_name("pattern")
            b = _c6_base_ident(pat, data) if pat is not None else None
            if b:
                muts.add(b)
        stack.extend(n.children)
    return muts


def _c6_cond_invariant_base(cond, muts: set, data: bytes):
    ""                                                                    
    stack = [cond]
    while stack:
        n = stack.pop()
        if n.type == "binary_expression" \
                and any(c.type in ("==", "!=") for c in n.children):
            for opd in (c for c in n.children if c.is_named):
                u = _wa_unwrap(opd)
                if u is not None and u.type in ("field_expression", "identifier"):
                    b = _c6_base_ident(u, data)
                    if b and b not in muts:
                        return b
        stack.extend(n.children)
    return None


def _c6_find_inline_dispatch(loop_body, muts: set, data: bytes):
    ""                                               
    stack = [loop_body]
    while stack:
        n = stack.pop()
        if n is not loop_body and n.type in (
                "while_expression", "for_expression", "loop_expression"):
            continue
        if n.type == "match_expression":
            scr = n.child_by_field_name("value")
            b = _c6_base_ident(scr, data) if scr is not None else None
            if b and b not in muts:
                body = n.child_by_field_name("body")
                arms = sum(1 for c in (body.named_children if body else [])
                           if c.type == "match_arm")
                if arms >= 2:
                    return ("inline-match", b)
        elif n.type == "if_expression":
            alt = n.child_by_field_name("alternative")
            cond = n.child_by_field_name("condition")
            has_elif = alt is not None and any(
                c.type == "if_expression" for c in alt.children)
            if has_elif and cond is not None:
                b = _c6_cond_invariant_base(cond, muts, data)
                if b:
                    return ("inline-if", b)
        stack.extend(n.children)
    return None


def _c6_callee_dispatches(callee: str, crate_root: Path, fn_index) -> bool:
    ""                                                        
    loc = fn_index.resolve(callee)
    if not loc:
        return False
    file_rel, l0, l1 = loc
    try:
        lines = (crate_root / file_rel).read_text(
            encoding="utf-8", errors="ignore").splitlines()
    except OSError:
        return False
    if l0 <= 0 or l1 <= 0 or l1 > len(lines):
        return False
    body = "\n".join(lines[l0 - 1:l1])
    return body.count("else if") >= 2 or (
        "match " in body and body.count("=>") >= 2)


def _c6_find_delegated_dispatch(loop_body, muts: set, data: bytes,
                                crate_root: Path, fn_index):
    ""                                                         
    stack = [loop_body]
    while stack:
        n = stack.pop()
        if n is not loop_body and n.type in (
                "while_expression", "for_expression", "loop_expression"):
            continue
        if n.type == "call_expression":
            fn = n.child_by_field_name("function")
            args = n.child_by_field_name("arguments")
            if fn is not None and fn.type == "identifier" and args is not None:
                callee = data[fn.start_byte:fn.end_byte].decode("utf-8", "ignore")
                inv = None
                for a in args.named_children:
                    if a.type in ("field_expression", "identifier"):
                        b = _c6_base_ident(a, data)
                        if b and b not in muts:
                            inv = b
                            break
                if inv and _c6_callee_dispatches(callee, crate_root, fn_index):
                    return ("delegated", callee)
        stack.extend(n.children)
    return None


def find_dispatch_hoist_hits(crate_root: Path, hot_fn_names: set[str],
                             fn_index) -> dict[str, list[Hit]]:
    ""                                                        
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except Exception:
            continue
        hits: list[Hit] = []
        seen: set[int] = set()
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            if n.type in _LOOP_SCAN_TYPES:
                wl = n.start_point[0] + 1
                if l0 <= wl <= l1 and wl not in seen:
                    body = None
                    for c in n.children:
                        if c.type == "block":
                            body = c
                    if body is not None:
                        muts = _c6_mutated(body, data)
                        found = (_c6_find_inline_dispatch(body, muts, data)
                                 or _c6_find_delegated_dispatch(
                                     body, muts, data, crate_root, fn_index))
                        if found:
                            kind, subject = found
                            if isinstance(subject, bytes):
                                subject = subject.decode("utf-8", "ignore")
                            snip = data[n.start_byte:n.end_byte].split(b"\n", 1)[0]
                            hits.append(Hit(
                                rule="C6", pattern=kind,
                                file=file_rel, line=wl, col=n.start_point[1] + 1,
                                snippet=snip.decode("utf-8", "ignore").strip()[:200],
                                extra={"dispatch_on": subject},
                            ))
                            seen.add(wl)
            stack.extend(n.children)
        if hits:
            out[fn] = hits
    if out:
        logger.info("[merged_hits] C6 dispatch hoist: %d hot fn(s), %d loop(s)",
                    len(out), sum(len(v) for v in out.values()))
    return out


                                                                        
                                               
                                                                  
                                                          
                                                                    
                                                            
                                                                  
                                                  
                                                               
                                             
_C7_GROW_SMALL_MAX = 64                                                   
_C7_ADD_METHODS = (b"wrapping_add", b"add", b"checked_add", b"saturating_add")


def _c7_callee_is_realloc(call_node, data: bytes) -> bool:
    fn = call_node.child_by_field_name("function")
    if fn is None:
        return False
    txt = data[fn.start_byte:fn.end_byte]
    return txt == b"realloc" or txt.endswith(b"::realloc")


def _c7_realloc_size_arg(call_node):
    args = call_node.child_by_field_name("arguments")
    if args is None:
        return None
    named = [c for c in args.children if c.is_named]
    return named[1] if len(named) >= 2 else None


def _c7_is_small_literal(node, data: bytes) -> bool:
    node = _wa_unwrap(node)
    if node is None or node.type != "integer_literal":
        return False
    digits = _wa_re.sub(rb"[^0-9]", b"", data[node.start_byte:node.end_byte])
    if not digits:
        return False
    try:
        v = int(digits)
    except ValueError:
        return False
    return 0 < v <= _C7_GROW_SMALL_MAX


def _c7_size_is_grow_by_exact(size_node, data: bytes) -> bool:
    ""                                                               
    n = _wa_unwrap(size_node)
    if n is None:
        return False
    if n.type == "call_expression":
        fld = n.child_by_field_name("function")
        args = n.child_by_field_name("arguments")
        if (fld is not None and fld.type == "field_expression"
                and args is not None):
            meth = fld.child_by_field_name("field")
            if meth is not None \
                    and data[meth.start_byte:meth.end_byte] in _C7_ADD_METHODS:
                named = [c for c in args.children if c.is_named]
                if named and _c7_is_small_literal(named[-1], data):
                    return True
    if n.type == "binary_expression":
        kids = [c for c in n.children if c.is_named]
        if len(kids) == 2 and any(c.type == "+" for c in n.children):
            if _c7_is_small_literal(kids[1], data):
                return True
    return False


def find_amortized_growth_hits(crate_root: Path, hot_fn_names: set[str],
                               fn_index) -> dict[str, list[Hit]]:
    ""                                                   

                                                                              
       
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except (OSError, Exception):
            continue
        hits: list[Hit] = []
        seen: set[int] = set()
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            if n.type == "call_expression" and _c7_callee_is_realloc(n, data):
                wl = n.start_point[0] + 1
                size = _c7_realloc_size_arg(n)
                if (l0 <= wl <= l1 and wl not in seen and size is not None
                        and _c7_size_is_grow_by_exact(size, data)):
                    snip = data[n.start_byte:n.end_byte].split(b"\n", 1)[0]
                    hits.append(Hit(
                        rule="C7", pattern="realloc-grow-by-exact",
                        file=file_rel, line=wl, col=n.start_point[1] + 1,
                        snippet=snip.decode("utf-8", "ignore").strip()[:200],
                        extra={},
                    ))
                    seen.add(wl)
            stack.extend(n.children)
        if hits:
            out[fn] = hits
    if out:
        logger.info("[merged_hits] C7 amortized growth: %d hot fn(s), %d realloc(s)",
                    len(out), sum(len(v) for v in out.values()))
    return out


                                                                              
                                     
#     crc = crc >> 8 ^ table[((crc ^ *byte) & 0xff) as usize];
                                                     
                                                        
                                                            
                                                                     
                                          
                                                               
                                                       


def _c8_has_ff_mask(node, data: bytes) -> bool:
    ""                                            
    stack = [node]
    while stack:
        n = stack.pop()
        if n.type == "binary_expression" and any(c.type == "&" for c in n.children):
            for opd in (c for c in n.children if c.is_named):
                u = _wa_unwrap(opd)
                if u is not None and u.type == "integer_literal":
                    raw = data[u.start_byte:u.end_byte]
                    digits = _wa_re.sub(rb"[^0-9a-fA-FxX]", b"", raw)
                    try:
                        if digits and int(digits, 0) == 0xff:
                            return True
                    except ValueError:
                        pass
        stack.extend(n.children)
    return False


def _c8_subtree_xors_var(node, lname: bytes, data: bytes) -> bool:
    ""                                              
    stack = [node]
    while stack:
        n = stack.pop()
        if n.type == "binary_expression" and any(c.type == "^" for c in n.children):
            for opd in (c for c in n.children if c.is_named):
                if _c6_base_ident(opd, data) == lname:
                    return True
        stack.extend(n.children)
    return False


def _c8_is_crc_recurrence(assign, data: bytes) -> bool:
    ""                                                                  
    left = assign.child_by_field_name("left")
    if left is None or left.type != "identifier":
        return False
    lname = data[left.start_byte:left.end_byte]
    right = _wa_unwrap(assign.child_by_field_name("right"))
    if right is None or right.type != "binary_expression":
        return False
    if not any(c.type == "^" for c in right.children):
        return False
    ops = [c for c in right.children if c.is_named]
    if len(ops) != 2:
        return False
    shift = index = None
    for op in ops:
        u = _wa_unwrap(op)
        if u is None:
            continue
        if u.type == "binary_expression" and any(c.type == ">>" for c in u.children):
            sub = [c for c in u.children if c.is_named]
            if sub and _c6_base_ident(sub[0], data) == lname:
                shift = u
        elif u.type == "index_expression":
            index = u
    if shift is None or index is None:
        return False
    return _c8_has_ff_mask(index, data) and _c8_subtree_xors_var(index, lname, data)


def find_crc_slicing_hits(crate_root: Path, hot_fn_names: set[str],
                          fn_index) -> dict[str, list[Hit]]:
    ""                                                         
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except Exception:
            continue
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            if n.type == "assignment_expression":
                wl = n.start_point[0] + 1
                if l0 <= wl <= l1 and _c8_is_crc_recurrence(n, data):
                    snip = data[n.start_byte:n.end_byte].split(b"\n", 1)[0]
                    out[fn] = [Hit(
                        rule="C8", pattern="crc-table-recurrence",
                        file=file_rel, line=wl, col=n.start_point[1] + 1,
                        snippet=snip.decode("utf-8", "ignore").strip()[:200],
                        extra={},
                    )]
                    break
            stack.extend(n.children)
    if out:
        logger.info("[merged_hits] C8 CRC slicing: %d hot fn(s)", len(out))
    return out


def find_bitfield_accessor_hits(crate_root: Path, hot_fn_names: set[str],
                                fn_index) -> dict[str, list[Hit]]:
    ""                                                                   

                                                          
                                                                   
                                                                     
                                                          
                        

                                                         
                                                   
                                                       
        
       
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.agent_perf_opt.bitfield_lower import analyze_source

                                                                
                                            
    accessors: dict[str, tuple[str, str]] = {}
    for path in sorted(Path(crate_root).rglob("*.rs")):
        if "target" in path.parts:
            continue
        try:
            source = path.read_bytes()
        except OSError:
            continue
        if b"BitfieldStruct" not in source:               
            continue
        try:
            rewrites, _skipped = analyze_source(source)
        except Exception as e:                   # noqa: BLE001
            logger.warning("[merged_hits] C9 analyze failed on %s: %s", path, e)
            continue
        try:
            rel = str(path.relative_to(crate_root))
        except ValueError:
            continue
        for rewrite in rewrites:
            for name in rewrite.accessor_names:
                accessors[name] = (rel, rewrite.struct_name)
    if not accessors:
        return {}

    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except Exception:
            continue
                                                    
        seen: dict[str, list] = {}
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            stack.extend(n.children)
            if n.type != "field_expression":
                continue
            line = n.start_point[0] + 1
            if not (l0 <= line <= l1):
                continue
            field = n.child_by_field_name("field")
            if field is None:
                continue
            name = data[field.start_byte:field.end_byte].decode("utf-8", "ignore")
            target = accessors.get(name)
            if target is None:
                continue
            decl_file, struct_name = target
            entry = seen.get(struct_name)
            if entry is None:
                snippet = data[n.start_byte:n.end_byte].split(b"\n", 1)[0]
                seen[struct_name] = [
                    line, n.start_point[1] + 1,
                    snippet.decode("utf-8", "ignore").strip()[:200],
                    decl_file, 1,
                ]
            else:
                entry[4] += 1
                if line < entry[0]:                                   
                    entry[0], entry[1] = line, n.start_point[1] + 1
        for struct_name, (line, col, snippet, decl_file, count) in sorted(
            seen.items()
        ):
            out.setdefault(fn, []).append(Hit(
                rule="C9", pattern="bitfield-accessor-per-bit-loop",
                file=file_rel, line=line, col=col, snippet=snippet,
                extra={
                    "decl_file": decl_file,
                    "struct_name": struct_name,
                    "accessor_calls": count,
                },
            ))
    if out:
        total = sum(h.extra["accessor_calls"] for hs in out.values() for h in hs)
        logger.info("[merged_hits] C9 bitfield accessors: %d hot fn(s), "
                    "%d accessor call site(s)", len(out), total)
    return out


                                                                 
#
                                        
#   * its trip count is known at compile time and smaller than a machine word,
#   * iterations carry no state to one another beyond the induction variable,
#   * nothing exits early, and
#   * each iteration touches one sub-word unit (bit / byte / nibble).
# All the work the loop does fits in one register, yet it pays N iterations of
# loop overhead and N branches to do it.
#
# Boundary with C4: C4's loop runs until the DATA says stop (sentinel, mismatch,
# buffer end) — nobody knows the count in advance, and the rewrite must reason
# about reading past the last needed byte. Here the count is a literal. That
# difference drives everything: different detection, different safety
# obligations, different templates. They are not two spellings of one rule.
#
# Four forms share one skeleton — fold N sub-word steps into one word step:
#   split   one word taken apart into N sub-words written to adjacent slots
#   pack    N sub-words accumulated into one word
#   copy    N sub-words moved or table-translated one at a time
#   reduce  N sub-words folded into one scalar
_C10_MAX_TRIP = 16          # more than this cannot be a sub-word count in 64 bits
_C10_MAX_STMTS = 6          # an iteration doing more than this is not "one unit"
_C10_SUBWORD_TYPES = (b"u8", b"i8", b"c_uchar", b"c_char", b"uint8_t",
                      b"int8_t", b"mz_uint8", b"c_schar")


def _c10_const_trip(loop_node, data: bytes):
    """The loop's compile-time trip count, or None.

    Only `<induction> < <literal>` counts. A compound condition
    (`i < 8 && p < end`) is deliberately rejected: the second clause hands the
    count back to the data, which makes it C4's kind of loop, not this one.
    Returns (induction_var_bytes, count).
    """
    if loop_node.type != "while_expression":
        return None
    cond = loop_node.child_by_field_name("condition")
    cond = _wa_unwrap(cond)
    if cond is None or cond.type != "binary_expression":
        return None
    if not any(c.type == "<" for c in cond.children):
        return None
    named = [c for c in cond.children if c.is_named]
    if len(named) != 2:
        return None
    lhs, rhs = _wa_unwrap(named[0]), _wa_unwrap(named[1])
    if lhs is None or rhs is None or lhs.type != "identifier":
        return None
    if rhs.type != "integer_literal":
        return None
    digits = _wa_re.sub(rb"[^0-9]", b"", data[rhs.start_byte:rhs.end_byte])
    if not digits:
        return None
    try:
        n = int(digits)
    except ValueError:
        return None
    if not (2 <= n <= _C10_MAX_TRIP):
        return None
    return data[lhs.start_byte:lhs.end_byte], n


def _c10_has_early_exit(block) -> bool:
    stack = [block]
    while stack:
        n = stack.pop()
        if n.type in ("break_expression", "return_expression",
                      "continue_expression"):
            return True
        stack.extend(n.children)
    return False


def _c10_has_inner_loop(block) -> bool:
    stack = list(block.children)
    while stack:
        n = stack.pop()
        if n.type in ("while_expression", "loop_expression", "for_expression"):
            return True
        stack.extend(n.children)
    return False


def _c10_indexed_by(node, var: bytes) -> bool:
    """This expression addresses memory through the induction variable.

    Covers `a[i]`, `a[i as usize]`, `*p.offset(i as isize)`, `p.add(i)`, and a
    bare `*p` where `p` is itself an induction variable — the question asked is
    "does the address move with the iteration", not "is it spelled as an index".
    The bare-deref case is the one c2rust emits most: an output cursor stepped
    by `p = p.offset(1)` and written through as `*p = …`.
    """
    stack = [node]
    while stack:
        n = stack.pop()
        if n.type in ("index_expression", "call_expression"):
            if var in n.text:
                return True
        if n.type == "unary_expression" and any(c.type == "*" for c in n.children):
            if var in n.text:
                return True
        stack.extend(n.children)
    return False


def _c10_reads_data(node) -> bool:
    """This expression loads something from memory.

    A method call's receiver does not: `p.offset(1)` and `i.wrapping_add(1)`
    parse as a `field_expression` under a `call_expression`, and reading that
    as a memory load makes every c2rust cursor step look like it carries data.
    A bare `(*d).field` is a load and still counts.
    """
    stack = [(node, None)]
    while stack:
        n, parent = stack.pop()
        if n.type == "index_expression":
            return True
        if n.type == "unary_expression" and any(c.type == "*" for c in n.children):
            return True
        if n.type == "field_expression":
            is_callee = (parent is not None
                         and parent.type == "call_expression"
                         and parent.child_by_field_name("function") is not None
                         and parent.child_by_field_name("function").id == n.id)
            if not is_callee:
                return True
        stack.extend((c, n) for c in n.children)
    return False


def _c10_addresses_depend_on(node, name: bytes) -> bool:
    """Some load in here computes its address from `name`."""
    stack = [node]
    while stack:
        n = stack.pop()
        if n.type == "index_expression":
            # tree-sitter-rust gives `a[i]` no `index` field — its named
            # children are [base, subscript], so take the second one. Asking
            # for a field that does not exist returns None, which silently
            # answers "no address depends on it" for every indexed load.
            named = [c for c in n.children if c.is_named]
            if len(named) >= 2 and name in named[1].text:
                return True
        elif n.type == "unary_expression" and any(c.type == "*" for c in n.children):
            operand = next((c for c in n.children if c.is_named), None)
            if operand is not None and name in operand.text:
                return True
        stack.extend(n.children)
    return False


def _c10_is_foldable_accumulation(rhs, lhs_txt: bytes) -> bool:
    """`v = f(v, <load>)` where no load's ADDRESS depends on `v`.

    The distinction that matters is not "does the right-hand side read memory"
    — it is "does the previous iteration's value decide WHERE this one reads".

        v = (v << 1) | *p            the old v is only shifted along; `p`
                                     alone picks the address → foldable
        acc = tbl[(acc ^ x) as usize] the old acc computes the index → a true
                                     recurrence, and folding changes the result

    Reading the first as state is what left `pack` with zero hits across the
    whole dataset while a bit-packing loop sat in a hot function of one of its
    projects.
    """
    if rhs is None or lhs_txt not in rhs.text:
        return False
    return not _c10_addresses_depend_on(rhs, lhs_txt)


def _c10_is_secondary_induction(rhs, lhs_txt: bytes) -> bool:
    """`x = x <op> <const>` — a second counter, not a carried value.

    c2rust lowers quantities that are really functions of the iteration number
    into self-updating locals: the output cursor `p = p.offset(1)`, the bit
    mask `mask = mask >> 1`. Their value depends only on how many times the
    loop has run, never on what it read, so folding the loop can compute them
    directly. Treating them as state is what makes a textbook bit-expansion
    look like a recurrence.

    The test is exactly that independence: the right-hand side mentions the
    same variable and constants, and loads nothing.
    """
    if rhs is None or lhs_txt not in rhs.text:
        return False
    return not _c10_reads_data(rhs)


def _c10_induction_set(block, var: bytes) -> set:
    """Every variable whose value is a function of the iteration number alone.

    Not just the loop variable. c2rust routinely lowers such quantities into
    self-advancing locals — an output cursor `p = p.offset(1)`, a bit mask
    `mask = mask >> 1` — and a write through one of those (`*p = …`) IS an
    indexed write, however little it looks like `out[i] = …`. Counting only the
    declared loop variable classifies a textbook bit-expansion as a recurrence.
    """
    # A variable that is ALSO accumulated somewhere in the body is not a
    # counter, whatever its other assignments look like. `v = v << 1` on its
    # own reads like a cursor step; paired with `v = v | *p` it is the
    # accumulator of a bit-packing loop. Admitting it here would make that
    # second assignment look like an induction step, and the body would end up
    # classified as nothing at all.
    accumulated = set()
    stack = [block]
    while stack:
        n = stack.pop()
        if n.type in ("assignment_expression", "compound_assignment_expr"):
            lhs = n.child_by_field_name("left")
            rhs = n.child_by_field_name("right")
            if lhs is not None and lhs.type == "identifier":
                if n.type == "compound_assignment_expr" or (
                        rhs is not None and _c10_reads_data(rhs)
                        and lhs.text in rhs.text):
                    accumulated.add(lhs.text)
        stack.extend(n.children)

    ind = {var}
    for _ in range(4):                       # cursors can be defined in terms
        grew = False                         # of each other; a few passes settle
        stack = [block]
        while stack:
            n = stack.pop()
            if n.type == "assignment_expression":
                lhs = n.child_by_field_name("left")
                rhs = n.child_by_field_name("right")
                if (lhs is not None and lhs.type == "identifier"
                        and lhs.text not in ind
                        and lhs.text not in accumulated
                        and _c10_is_secondary_induction(rhs, lhs.text)):
                    ind.add(lhs.text)
                    grew = True
            stack.extend(n.children)
        if not grew:
            break
    return ind


def _c10_classify(block, var: bytes, data: bytes):
    """Which of the four forms this body is, or None if it carries state.

    The disqualifier is an assignment whose new value depends on what THIS
    iteration read — the next iteration would see it, so folding the loop would
    change what the program computes. A self-update that reads nothing is a
    secondary induction variable, not state (see `_c10_induction_set`), and a
    compound assignment that does read is a recognisable reduction.
    """
    ind = _c10_induction_set(block, var)
    writes_indexed = accum_scalar = shifts = False
    stack = [block]
    while stack:
        n = stack.pop()
        if n.type == "binary_expression" and any(
                c.type in ("<<", ">>") for c in n.children):
            shifts = True
        if n.type in ("assignment_expression", "compound_assignment_expr"):
            lhs = n.child_by_field_name("left")
            rhs = n.child_by_field_name("right")
            if lhs is not None:
                compound = n.type == "compound_assignment_expr"
                if lhs.type == "identifier" and lhs.text in ind:
                    pass                     # the induction variable's own step
                elif any(_c10_indexed_by(lhs, v) for v in ind):
                    writes_indexed = True
                elif compound:
                    accum_scalar = True
                elif _c10_is_secondary_induction(rhs, lhs.text):
                    pass                     # a second counter, not carried data
                elif _c10_is_foldable_accumulation(rhs, lhs.text):
                    accum_scalar = True      # shift-accumulate: order matters,
                                             # but the addresses do not depend
                                             # on what was accumulated
                else:
                    return None              # real recurrence — not this rule
        stack.extend(n.children)
    if writes_indexed and shifts:
        return "split"
    if accum_scalar and shifts:
        return "pack"
    if writes_indexed:
        return "copy"
    if accum_scalar:
        return "reduce"
    return None


def _c10_touches_subword(block, data: bytes) -> bool:
    """Each step handles one sub-word unit.

    Required of `split` / `pack` only, and for a reason specific to them: their
    payoff is N bit-operations collapsing into one word-wide operation, which
    needs the units to be sub-word in the first place. `copy` / `reduce` earn
    their keep differently — N element loads and stores become one `memcpy` or
    one unrolled block — and six `u32`s fold just as well as six bytes. Making
    them pass this test rejected a textbook fixed-length array copy.
    """
    txt = data[block.start_byte:block.end_byte]
    if any(t in txt for t in _C10_SUBWORD_TYPES):
        return True
    stack = [block]
    while stack:
        n = stack.pop()
        if n.type == "binary_expression" and any(
                c.type in ("<<", ">>", "&", "|", "^") for c in n.children):
            return True
        stack.extend(n.children)
    return False


def find_fixed_subword_loop_hits(crate_root: Path, hot_fn_names: set[str],
                                 fn_index) -> dict[str, list[Hit]]:
    ""                                                
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except (OSError, Exception):
            continue
        hits: list[Hit] = []
        seen: set[int] = set()
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            stack.extend(n.children)
            if n.type != "while_expression":
                continue
            wl = n.start_point[0] + 1
            if not (l0 <= wl <= l1) or wl in seen:
                continue
            trip = _c10_const_trip(n, data)
            if trip is None:
                continue
            var, count = trip
            body = n.child_by_field_name("body")
            if body is None:
                continue
            if _c10_has_early_exit(body) or _c10_has_inner_loop(body):
                continue
            if len([c for c in body.named_children]) > _C10_MAX_STMTS:
                continue
            form = _c10_classify(body, var, data)
            if form is None:
                continue
            if form in ("split", "pack") and not _c10_touches_subword(body, data):
                continue
            snip = data[n.start_byte:n.end_byte].split(b"\n", 1)[0]
            hits.append(Hit(
                rule="C10", pattern=f"fixed-subword-loop-{form}",
                file=file_rel, line=wl, col=n.start_point[1] + 1,
                snippet=snip.decode("utf-8", "ignore").strip()[:200],
                extra={"trip_count": count, "form": form},
            ))
            seen.add(wl)
        if hits:
            out[fn] = hits
    if out:
        logger.info("[merged_hits] C10 fixed sub-word loops: %d hot fn(s), "
                    "%d loop(s)", len(out), sum(len(v) for v in out.values()))
    return out



                                                                               
#
# C has `goto`; Rust does not. c2rust lowers a function's gotos into a state
# machine: a `let mut current_block: u64` holding an opaque 64-bit label, plus
# `match current_block { <const> => .. }` blocks routing to whatever the goto
# targeted. A straight-line path then pays what the C never did — store the
# label, re-enter, compare the constants one at a time.
#
# `current_block` / `current_block_<n>` is c2rust's own generated name, never
# the project's, so matching on it carries no project knowledge. Surveyed
# 2026-08-29: present under exactly these two spellings in 10 of 10 translated
# projects, absent from none that use goto.
#
# Two independent costs, hence two patterns; a function may raise either or both.
#
#   goto-dispatch-wide-constants
#       Labels are random 64-bit values, so each comparison needs its own
#       10-byte `movabs` to materialise the immediate. Renumbering them dense
#       and narrow is a pure symbol substitution — no control flow moves.
#
#   goto-dispatch-loop-head
#       A dispatch sits at the head of a loop, so the loop's back edge pays it
#       every iteration. c2rust merges what were separate C loops into one
#       `loop { match .. }`; splitting the hot arm back into its own loop takes
#       the dispatch off that edge.
#
# The detector deliberately does NOT predict which one pays. LLVM threads the
# dispatch away in some functions and not others and nothing in the source says
# which: measured 2026-08-29, renumbering moved miniz `tinfl_decompress` by
# −14.57pp while leaving lz4 `LZ4_decompress_generic` and http-parser
# `http_parser_execute` bit-identical in instruction count. The counts go into
# `extra`; the W2 gate decides.
_C11_VAR_RE = _wa_re.compile(r"^current_block(?:_\d+)?$")
_C11_WIDE = 1 << 32          # above this a compare needs `movabs`, not an imm32


def _c11_int(node, data: bytes):
    """An `integer_literal`'s value, or None. Handles `1_000` and `0u32`."""
    txt = data[node.start_byte:node.end_byte].decode("ascii", "ignore")
    txt = txt.replace("_", "")
    m = _wa_re.match(r"^(0[xXoObB])?([0-9a-fA-F]+)", txt)
    if not m:
        return None
    base = {None: 10, "0x": 16, "0X": 16, "0o": 8, "0O": 8, "0b": 2, "0B": 2}
    try:
        return int(m.group(2), base.get(m.group(1), 10))
    except ValueError:
        return None


def _c11_arm_constants(match_node, data: bytes) -> list[int]:
    """Integer labels this `match` arms on, `A | B => ..` included.

    Arm *patterns* only. An integer in an arm's body is ordinary program data,
    not a label — counting it would inflate the state count and, worse, would
    put a non-label constant on the rename list. Measured on http-parser:
    `http_parser_execute` arms a second `match` on `(method << 16) | (index <<
    8) | ch`, whose labels (`196929`, `1311298`, …) are the same width as real
    ones; a rename that reached them broke 3 of 5 operations.
    """
    body = match_node.child_by_field_name("body")
    if body is None:
        return []
    out: list[int] = []
    for arm in body.children:
        if arm.type != "match_arm":
            continue
        pat = arm.child_by_field_name("pattern")
        if pat is None:
            continue
        stack = [pat]
        while stack:
            n = stack.pop()
            if n.type == "integer_literal":
                v = _c11_int(n, data)
                if v is not None:
                    out.append(v)
                continue
            stack.extend(n.children)
    return out


def _c11_heads_a_loop(match_node) -> bool:
    """This dispatch is the first statement of an enclosing loop body.

    That is the shape whose cost lands on the back edge: every iteration
    re-enters the head and re-runs the comparison chain. A dispatch elsewhere in
    the body is paid once per pass through that point — which is what the C
    `goto` paid too, so there is nothing to recover there.
    """
    stmt = match_node
    while stmt.parent is not None and stmt.parent.type == "expression_statement":
        stmt = stmt.parent
    block = stmt.parent
    if block is None or block.type != "block":
        return False
    loop = block.parent
    if loop is None or loop.type not in ("loop_expression", "while_expression",
                                         "for_expression"):
        return False
    named = [c for c in block.children if c.is_named]
    # Compare by source span, not identity: the tree-sitter binding hands back
    # a fresh wrapper object on every access, so `named[0] is stmt` is False
    # even when they are the same node — it silently reported "no dispatch
    # heads a loop" for every function surveyed.
    return bool(named) and named[0].start_byte == stmt.start_byte


def find_goto_dispatch_hits(crate_root: Path, hot_fn_names: set[str],
                            fn_index) -> dict[str, list[Hit]]:
    ""                                                
    if crate_root is None or fn_index is None or not hot_fn_names:
        return {}
    from perf_opt.hot_probe.static_facts import _parse_rust_source
    out: dict[str, list[Hit]] = {}
    for fn in sorted(hot_fn_names):
        loc = fn_index.resolve(fn)
        if not loc:
            continue
        file_rel, l0, l1 = loc
        try:
            data = (crate_root / file_rel).read_bytes()
            tree = _parse_rust_source(data)
        except Exception:
            continue

        def _in_fn(n) -> bool:
            return l0 <= n.start_point[0] + 1 <= l1

        # One function can declare several state variables — measured on
        # http-parser `http_parser_execute`: `current_block` for the outer
        # gotos, `current_block_938` for a nested one. They are separate
        # namespaces; merging them would report one bogus dispatch.
        declared: list[bytes] = []
        assigns: dict[bytes, int] = {}
        labels: dict[bytes, set] = {}
        dispatches: dict[bytes, list] = {}
        stack = [tree.root_node]
        while stack:
            n = stack.pop()
            stack.extend(n.children)
            if not _in_fn(n):
                continue
            if n.type == "let_declaration":
                pat = n.child_by_field_name("pattern")
                if pat is not None and pat.type == "identifier":
                    nm = data[pat.start_byte:pat.end_byte]
                    if (_C11_VAR_RE.match(nm.decode("ascii", "ignore"))
                            and nm not in declared):
                        declared.append(nm)
                continue
            if n.type == "assignment_expression":
                lhs = n.child_by_field_name("left")
                rhs = n.child_by_field_name("right")
                if (lhs is not None and rhs is not None
                        and lhs.type == "identifier"
                        and rhs.type == "integer_literal"):
                    nm = data[lhs.start_byte:lhs.end_byte]
                    if _C11_VAR_RE.match(nm.decode("ascii", "ignore")):
                        v = _c11_int(rhs, data)
                        if v is not None:
                            labels.setdefault(nm, set()).add(v)
                            assigns[nm] = assigns.get(nm, 0) + 1
                continue
            if n.type == "match_expression":
                val = n.child_by_field_name("value")
                if val is not None and val.type == "identifier":
                    nm = data[val.start_byte:val.end_byte]
                    if _C11_VAR_RE.match(nm.decode("ascii", "ignore")):
                        dispatches.setdefault(nm, []).append(n)

        hits: list[Hit] = []
        for var in declared:
            lab = set(labels.get(var, ()))
            disp = dispatches.get(var, [])
            for m in disp:
                lab.update(_c11_arm_constants(m, data))
            # One label is a flag, not a dispatch; and no `match` means the
            # variable never routes anything.
            if len(lab) < 2 or not disp:
                continue
            wide = sum(1 for v in lab if v >= _C11_WIDE)
            heads = [m for m in disp if _c11_heads_a_loop(m)]
            extra = {"var": var.decode(), "states": len(lab),
                     "wide_states": wide, "assignments": assigns.get(var, 0),
                     "dispatch_sites": len(disp),
                     "loop_head_dispatches": len(heads)}
            snippet = (f"let mut {var.decode()}: … — {len(lab)} states, "
                       f"{len(disp)} dispatch site(s)")
            if wide >= 2:
                hits.append(Hit(rule="C11",
                                pattern="goto-dispatch-wide-constants",
                                file=file_rel, line=l0, col=1,
                                snippet=snippet, extra=dict(extra)))
            if heads:
                hits.append(Hit(rule="C11",
                                pattern="goto-dispatch-loop-head",
                                file=file_rel,
                                line=heads[0].start_point[0] + 1, col=1,
                                snippet=snippet, extra=dict(extra)))
        if hits:
            out[fn] = hits
    if out:
        logger.info("[merged_hits] C11 goto dispatch: %d hot fn(s), %d hit(s)",
                    len(out), sum(len(v) for v in out.values()))
    return out


# ── Merge + write ──────────────────────────────────────────────────────────────

def merge_hits(
    ci_sr,
    cii_sr,
    ciii_sr,
    opt_remarks: list[dict] | None,
    fn_index,
    crate_root: Path | None = None,
    hot_fn_names: set[str] | None = None,
    canonical_fn_names: set[str] | None = None,
) -> dict[str, FnHits]:
    ""                                                  

                                                                        
                                                                             
                                                                        
                                     
       
    ci = iter_class_i(ci_sr, crate_root=crate_root, fn_index=fn_index) \
        if ci_sr is not None else {}
    cii = iter_class_ii(cii_sr, opt_remarks or [], fn_index,
                        crate_root=crate_root,
                        c3_pass_fns=getattr(ci_sr, "c3_hits", None)) \
        if opt_remarks else {}
    # Invariant: class_I judged these functions to carry C3, and the only way
    # C3 reaches the agent is through here. If none of them comes out the other
    # side, the detection result was thrown away — it is not an absence of
    # opportunity. That happened silently for three lz4 runs: the analysis
    # build started remapping source paths, `_rel_to_crate` could no longer
    # relativize the remark's file, and every C3 remark was dropped as
    # unattributable while II_vec, on a channel that prints relative paths,
    # kept arriving. Say it where the loss happens.
    # Every remark-borne rule, not just C3: II_inl rode the same channel and
    # went from 28 functions to 0 in the same three runs. The class_II scan
    # attributes by basename and so still "sees" them; only this stage, which
    # needs the crate-relative path, loses them — compare the two.
    _expected = {
        "C3": set(getattr(ci_sr, "c3_hits", None) or ()),
        "II_inl": set(getattr(cii_sr, "ii_inl_hits", None) or ())
                  | set(getattr(cii_sr, "ii_inl_candidates", None) or ()),
        "II_vec": set(getattr(cii_sr, "ii_vec_hits", None) or ()),
    }
    if opt_remarks:
        for _rule, _want in _expected.items():
            if not _want:
                continue
            _got = {fn for fn, hs in cii.items()
                    if any(h.rule == _rule for h in hs)}
            if not _got:
                logger.warning(
                    "[merged_hits] ✗ 检测层判定 %d 个函数有 %s,但 0 个 %s 进入 "
                    "fn_hits —— remark 归属不到函数(多为路径不匹配),%s 整体失效。"
                    "样例: %s", len(_want), _rule, _rule, _rule, sorted(_want)[:5])
    ciii = iter_class_iii(ciii_sr) if ciii_sr is not None else {}

                                                                    
    civ_const: dict[str, list[Hit]] = {}
    if crate_root is not None and fn_index is not None and hot_fn_names:
        try:
            civ_const = find_const_promote_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] II_const static→const skipped: %s", e)

                                              
    civ_c4: dict[str, list[Hit]] = {}
    civ_c5: dict[str, list[Hit]] = {}
    civ_c6: dict[str, list[Hit]] = {}
    civ_c7: dict[str, list[Hit]] = {}
    civ_c8: dict[str, list[Hit]] = {}
    civ_c9: dict[str, list[Hit]] = {}
    civ_c10: dict[str, list[Hit]] = {}
    civ_c11: dict[str, list[Hit]] = {}
    if crate_root is not None and fn_index is not None and hot_fn_names:
        try:
            civ_c4 = find_word_at_a_time_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C4 word-at-a-time skipped: %s", e)
        try:
            civ_c5 = find_reduction_reassoc_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C5 reduction reassoc skipped: %s", e)
        try:
            civ_c6 = find_dispatch_hoist_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C6 dispatch hoist skipped: %s", e)
        try:
            civ_c7 = find_amortized_growth_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C7 amortized growth skipped: %s", e)
        try:
            civ_c8 = find_crc_slicing_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C8 CRC slicing skipped: %s", e)
        try:
            civ_c9 = find_bitfield_accessor_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C9 bitfield accessors skipped: %s", e)
        try:
            civ_c10 = find_fixed_subword_loop_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C10 fixed sub-word loops skipped: %s", e)
        try:
            civ_c11 = find_goto_dispatch_hits(crate_root, hot_fn_names, fn_index)
        except Exception as e:
            logger.warning("[merged_hits] C11 goto dispatch skipped: %s", e)

    # The scanners do not agree on what a function is called. IR/DWARF-based
    # attribution reports the SOURCE name; `hot_fn_names` comes from perf and
    # holds the LINK name; `#[export_name]` makes the two differ, and c2rust
    # emits it for every C identifier that is a Rust keyword. The agent looks
    # a function up in `fn_hits.json` by its HOTSPOT name and skips on a miss,
    # so folding the keys here is what keeps the two files talking about the
    # same functions. Measured on fzy: `match` was located hot, its C12 hits
    # were filed under `match_0`, and the agent logged
    # "no fn_hits entry, skip" over 26,632 bytes of removable zeroing.
    # Fold onto the names `hotspots.json` uses — NOT onto `hot_fn_names`.
    # The two differ on purpose: `hot_fn_names` is deliberately widened with
    # aliases so the scanners' own `hot_fns` prune keeps sites filed under
    # either name, and testing membership against the widened set would say
    # `match_0` "is already hot" and skip the fold. That silently undoes this
    # normalization — measured: with both in place the key stayed `match_0`
    # and the agent still logged "no fn_hits entry, skip".
    canon_set = (canonical_fn_names if canonical_fn_names is not None
                 else hot_fn_names)

    def _canon(name: str) -> str:
        if not canon_set or fn_index is None or name in canon_set:
            return name
        for alias in fn_index.aliases_of(name):
            if alias in canon_set:
                return alias
        return name

    by_fn: dict[str, list[Hit]] = {}
    for scanner in (ci, cii, ciii, civ_const, civ_c4, civ_c5, civ_c6,
                    civ_c7, civ_c8, civ_c9, civ_c10, civ_c11):
        for raw_name, scanner_hits in scanner.items():
            if scanner_hits:
                by_fn.setdefault(_canon(raw_name), []).extend(scanner_hits)

    out: dict[str, FnHits] = {}
    for fn in sorted(by_fn):
        hits = by_fn[fn]
        if not hits:
            continue

                                                        
        loc = fn_index.resolve(fn) if fn_index is not None else None
        if loc:
            file, l0, l1 = loc
        else:
            file = next((h.file for h in hits if h.file), "")
            lines = [h.line for h in hits if h.line]
            l0 = min(lines) if lines else 0
            l1 = max(lines) if lines else 0

        out[fn] = FnHits(fn=fn, file=file, line_range=(l0, l1), hits=hits)

    logger.info("[merged_hits] merged: %d fn(s) with hits "
                "(class_I=%d, class_II=%d, class_III=%d fns)",
                len(out), len(ci), len(cii), len(ciii))
    return out


def write_fn_hits(merged: dict[str, FnHits], out_path: Path) -> None:
    """Write fn_hits.json with stable key order + pretty indent."""
    out_path = Path(out_path)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    data = {fn: fh.to_dict() for fn, fh in merged.items()}
    out_path.write_text(
        json.dumps(data, indent=2, ensure_ascii=False, sort_keys=True),
        encoding="utf-8",
    )
    logger.info("[merged_hits] wrote %s (%d fn(s))", out_path, len(data))
