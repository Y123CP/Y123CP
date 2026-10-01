"""Stage A de-unsafe pass — drop the `unsafe fn` marker from functions with a
SAFE contract.

Soundness: a function may be `fn` (not `unsafe fn`) iff calling it carries no
caller-provided-validity precondition. c2rust marks EVERYTHING `unsafe fn`
defensively; the only real signature-level unsafe contract is a RAW-POINTER
PARAM ("caller must pass a valid pointer"). So a function with NO raw-pointer
params has a safe contract — its body's unsafe ops (raw derefs of LOCALS,
calls to other unsafe fns) depend only on values it creates itself, which an
`unsafe { }` block encapsulates.

Two outcomes per candidate (cargo check decides):
  · body already fully safe (E2 lifted its params to refs) → just drop
    `unsafe` → PURE win (unsafe_fn −1, no unsafe block added).
  · body still has unsafe ops → drop `unsafe` + wrap body in `unsafe { }`
    (unsafe_fn −1, unsafe_blk +1; the fn becomes safe-to-call, unsafe localized).

`extern` (FFI ABI) functions are left alone — they're the C boundary.
Gated by the runner's cargo check + build + W1 (all workloads); per-fn rollback.
"""
from __future__ import annotations

import re
from pathlib import Path

from .intra_ptr.collect import _PARSER, _txt


def _has_body(fn) -> bool:
    return any(c.type == "block" for c in fn.children)


def _fn_name_node(fn):
    return next((c for c in fn.children if c.type == "identifier"), None)


def _n_raw_ptr_params(src: bytes, fn) -> int:
    params = next((c for c in fn.children if c.type == "parameters"), None)
    if params is None:
        return 0
    n = 0
    for pm in params.named_children:
        if pm.type != "parameter":
            continue
        ty = pm.child_by_field_name("type")
        if ty is not None and ty.type == "pointer_type":
            n += 1
    return n


def find_candidates(crate_dir: Path) -> list[tuple[str, str]]:
    """`(fn_name, rel_file)` for every `unsafe fn` that (a) has a body, (b) is
    NOT extern (FFI), (c) has no raw-pointer params → safe contract."""
    out: list[tuple[str, str]] = []
    for f in sorted(crate_dir.rglob("*.rs")):
        if "target" in f.parts or f.name == "build.rs":
            continue
        src = f.read_bytes()
        st = [_PARSER.parse(src).root_node]
        while st:
            n = st.pop()
            st.extend(n.children)
            if n.type != "function_item":
                continue
            mods = next((c for c in n.children
                         if c.type == "function_modifiers"), None)
            if mods is None or not any(c.type == "unsafe" for c in mods.children):
                continue
            if any(c.type == "extern" for c in mods.children):
                continue                       # FFI boundary — keep unsafe
            if not _has_body(n):
                continue                       # forward decl
            if _n_raw_ptr_params(src, n) > 0:
                continue                       # real unsafe contract
            nm = _fn_name_node(n)
            if nm is not None:
                out.append((_txt(src, nm), str(f.relative_to(crate_dir))))
    return out


def _find_fn(root, src: bytes, name: str):
    st = [root]
    while st:
        n = st.pop()
        st.extend(n.children)
        if n.type == "function_item":
            nm = _fn_name_node(n)
            if nm is not None and _txt(src, nm) == name:
                return n
    return None


def drop_unsafe_kw(file_path: Path, fn_name: str) -> bool:
    """Remove the `unsafe` keyword from `fn_name`'s modifiers (+ one trailing
    space). Returns True if applied."""
    src = file_path.read_bytes()
    fn = _find_fn(_PARSER.parse(src).root_node, src, fn_name)
    if fn is None:
        return False
    mods = next((c for c in fn.children
                 if c.type == "function_modifiers"), None)
    if mods is None:
        return False
    tok = next((c for c in mods.children if c.type == "unsafe"), None)
    if tok is None:
        return False
    s, e = tok.start_byte, tok.end_byte
    if e < len(src) and src[e:e + 1] == b" ":
        e += 1                                 # consume trailing space
    file_path.write_bytes(src[:s] + src[e:])
    return True


def strip_unnecessary_unsafe(check_dir: Path, max_iter: int = 8) -> int:
    """Remove `unsafe { … }` wrappers that rustc flags as `unused_unsafe`
    (e.g. `unsafe { &mut *s_view }` where s_view is a reference → the reborrow
    is safe; or any inner unsafe block inside an `unsafe fn`). Drops just the
    `unsafe` keyword, leaving the `{ … }` block — a NO-OP at runtime (unsafe is
    compile-time only), so behaviour cannot change. rustc guarantees the
    flagged blocks are unnecessary, so the result still compiles. Iterates to a
    fixpoint (stripping an outer block can expose an inner one). Returns the
    number of `unsafe` keywords removed.

    Runs `cargo check --message-format=json` in `check_dir` (self-bin crate or
    harness); diagnostic spans give byte offsets directly (no line/col math)."""
    import json
    import subprocess
    from collections import defaultdict

    total = 0
    for _ in range(max_iter):
        r = subprocess.run(
            ["cargo", "check", "--release", "--message-format=json"],
            cwd=check_dir, capture_output=True, text=True, timeout=600)
        spans: list[tuple[str, int, int]] = []
        for line in r.stdout.splitlines():
            try:
                msg = json.loads(line)
            except ValueError:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            m = msg.get("message", {})
            if (m.get("code") or {}).get("code") != "unused_unsafe":
                continue
            for sp in m.get("spans", []):
                if sp.get("is_primary"):
                    spans.append((sp["file_name"], sp["byte_start"],
                                  sp["byte_end"]))
        if not spans:
            break
        flagged: dict[str, set[int]] = defaultdict(set)
        for f, s, e in spans:
            flagged[f].add(s)                  # byte_start of the `unsafe` kw
        n = 0
        for f, starts in flagged.items():
            path = check_dir / f
            if not path.is_file():
                path = Path(f)
                if not path.is_file():
                    continue
            src = path.read_bytes()
            root = _PARSER.parse(src).root_node
            edits: list[tuple[int, int, bytes]] = []
            st = [root]
            while st:
                node = st.pop()
                st.extend(node.children)
                if node.type != "unsafe_block" or node.start_byte not in starts:
                    continue
                block = next((c for c in node.children if c.type == "block"),
                             None)
                inner = block.named_children if block is not None else []
                if (len(inner) == 1 and not inner[0].type.endswith("_statement")
                        and inner[0].type != "let_declaration"):
                    # `unsafe { EXPR }` → `EXPR` (single tail expression)
                    edits.append((node.start_byte, node.end_byte,
                                  src[inner[0].start_byte:inner[0].end_byte]))
                else:
                    # multi-stmt block → drop just the `unsafe` keyword
                    kw = node.children[0]
                    end = kw.end_byte
                    if end < len(src) and src[end:end + 1] == b" ":
                        end += 1
                    edits.append((kw.start_byte, end, b""))
                n += 1
            if edits:
                out = bytearray(src)
                for s, e, rep in sorted(edits, key=lambda x: x[0], reverse=True):
                    out[s:e] = rep
                path.write_bytes(bytes(out))
        total += n
        if n == 0:
            break
    return total


def _ref_var_names(src: bytes, fn) -> set[str]:
    """Names of params + annotated locals whose type is a REFERENCE (`&T` /
    `&mut T`) — and NOT a raw pointer. Only `(*x).field` on these auto-derefs;
    `(*p).field` on a `*mut T` p MUST keep the explicit deref."""
    out: set[str] = set()
    params = next((c for c in fn.children if c.type == "parameters"), None)
    if params is not None:
        for pm in params.named_children:
            if pm.type != "parameter":
                continue
            pat = pm.child_by_field_name("pattern")
            ty = pm.child_by_field_name("type")
            if pat is not None and ty is not None and ty.type == "reference_type":
                nm = _txt(src, pat)
                out.add(nm[4:].strip() if nm.startswith("mut ") else nm)
    for ld in (n for n in _walk_all(fn) if n.type == "let_declaration"):
        pat = ld.child_by_field_name("pattern")
        ty = ld.child_by_field_name("type")
        if pat is not None and ty is not None and ty.type == "reference_type":
            nm = _txt(src, pat)
            out.add(nm[4:].strip() if nm.startswith("mut ") else nm)
    return out


def _walk_all(n):
    st = [n]
    while st:
        x = st.pop()
        st.extend(x.children)
        yield x


def idiomatize_auto_deref(crate_dir: Path) -> int:
    """Rewrite `(*x).field` / `(*x).method(..)` → `x.field` / `x.method(..)`
    where `x` is a REFERENCE — the explicit-auto-deref idiom the ref-lift
    exposes (E2/intra_ptr turn `*mut T` params into `&mut T`, but the rewriters
    keep the `(*x)` form). Deterministic replacement for `cargo clippy --fix`
    (which is fragile: breaks libcsv's transmute, structurally skips heman's
    overlapping derefs, can leave partial broken edits). ALWAYS value-exact
    (dereffing a reference is a no-op the compiler inserts anyway), and never
    touches a raw-pointer deref. Returns the number of sites rewritten; caller
    cargo-check + W1 gates (sanity, not soundness — the rewrite is identity)."""
    total = 0
    for f in sorted(crate_dir.rglob("*.rs")):
        if "target" in f.parts or f.name == "build.rs":
            continue
        src = f.read_bytes()
        root = _PARSER.parse(src).root_node
        edits: list[tuple[int, int, bytes]] = []
        for fn in (n for n in _walk_all(root) if n.type == "function_item"):
            refs = _ref_var_names(src, fn)
            if not refs:
                continue
            for fe in (n for n in _walk_all(fn) if n.type == "field_expression"):
                val = fe.child_by_field_name("value")
                if val is None:
                    continue
                m = re.fullmatch(r"\(\s*\*\s*([A-Za-z_]\w*)\s*\)",
                                 _txt(src, val).strip())
                if m and m.group(1) in refs:
                    edits.append((val.start_byte, val.end_byte,
                                  m.group(1).encode()))
        if edits:
            out = bytearray(src)
            for s, e, rep in sorted(edits, key=lambda x: x[0], reverse=True):
                out[s:e] = rep
            f.write_bytes(bytes(out))
            total += len(edits)
    return total


def simplify_raw_roundtrip(crate_dir: Path) -> int:
    """Rewrite `unsafe { &mut *&raw mut X }` → `&mut X` (and the `&` / `&raw
    const` variants) where X is a plain local — a SAFE borrow.

    E2's callsite wrapper, lifting `f(*mut T)` → `f(&mut T)`, wraps a callsite
    arg that was `&mut x as *mut T` / `&raw mut x` into `unsafe { &mut *&raw mut
    x }`. Taking the address of a local and immediately reborrowing it is a
    pointless round-trip through a raw pointer — `&mut *&raw mut x` ≡ `&mut x`,
    which needs no `unsafe`. Removing it drops a whole unsafe block per site
    (≈38 on bzip2) and is behaviour-identical (W1 trivially holds). Restricted
    to a bare identifier operand so the result has no residual raw deref (e.g.
    `&raw mut (*p).f` is left alone — `&mut (*p).f` still derefs `p`).

    Returns the number of sites simplified. Caller W1-gates."""
    pat = re.compile(
        r"unsafe\s*\{\s*&\s*mut\s*\*&raw\s+mut\s+([A-Za-z_]\w*)\s*\}")
    pat_const = re.compile(
        r"unsafe\s*\{\s*&\s*\*&raw\s+const\s+([A-Za-z_]\w*)\s*\}")
    total = 0
    for f in sorted(crate_dir.rglob("*.rs")):
        if "target" in f.parts or f.name == "build.rs":
            continue
        text = f.read_text(encoding="utf-8", errors="replace")
        new, n1 = pat.subn(r"&mut \1", text)
        new, n2 = pat_const.subn(r"&\1", new)
        if n1 + n2:
            f.write_text(new, encoding="utf-8")
            total += n1 + n2
    return total


def wrap_body_unsafe(file_path: Path, fn_name: str) -> bool:
    """Wrap `fn_name`'s body block in `unsafe { … }` (call AFTER drop_unsafe_kw
    for fns whose body still has unsafe ops)."""
    src = file_path.read_bytes()
    fn = _find_fn(_PARSER.parse(src).root_node, src, fn_name)
    if fn is None:
        return False
    body = next((c for c in fn.children if c.type == "block"), None)
    if body is None:
        return False
    bs, be = body.start_byte, body.end_byte    # `{` at bs, `}` at be-1
    inner = src[bs + 1:be - 1]
    file_path.write_bytes(src[:bs] + b"{ unsafe {" + inner + b"} }" + src[be:])
    return True
