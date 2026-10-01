"""Deterministic API inventory of a c2rust-translated crate.

No LLM here: we regex-scan the crate for its exported C-ABI surface
(`pub [unsafe] extern "C" fn …`) and record, per function, the full
signature text plus the Rust module path a harness must `use` to reach
it (c2rust 0_raw layout is `lib.rs: pub mod src { pub mod <file>; }`,
so src/lodepng.rs → `<crate>::src::lodepng::<fn>`).
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path

# Signature head; the body of the signature is captured by brace-walking
# from the match so that multi-line c2rust signatures survive intact.
_FN_HEAD = re.compile(
    r'pub\s+(?:unsafe\s+)?extern\s+"C"\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)'
)
_STRUCT = re.compile(r"pub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)")
# Type aliases matter as much as the structs they wrap. c2rust re-emits the C
# typedefs in every module too, so 23 files can carry the BYTE-IDENTICAL line
#     pub type png_const_structrp = *const png_struct;
# and still declare 23 incompatible types, because `png_struct` differs per
# module. Nothing in the name or in the definition text reveals that.
_TYPE_ALIAS = re.compile(
    r"pub\s+type\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([^;]+);")
_IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
_PKG_NAME = re.compile(r'^\s*name\s*=\s*"([^"]+)"', re.M)
_EXPORT_NAME = re.compile(r'#\[\s*export_name\s*=\s*"([^"]+)"\s*\]')
_NO_MANGLE = re.compile(r'#\[\s*no_mangle\s*\]')


@dataclass
class ApiFn:
    name: str               # Rust item name, e.g. `match_0`
    link_name: str          # exported symbol llvm-cov records it under, e.g.
                            # `match` (from #[export_name]) or == name (#[no_mangle])
    signature: str          # `pub unsafe extern "C" fn foo(…) -> …` (no body)
    module_path: str        # e.g. "src::lodepng"
    file: str               # path relative to the crate dir


@dataclass
class CrateInventory:
    crate_dir: Path
    crate_name: str         # [package].name — what the harness path-deps on
    lib_rs: str             # lib.rs content (module tree; small in 0_raw)
    fns: list[ApiFn] = field(default_factory=list)
    structs: list[str] = field(default_factory=list)
    # Type name → every module that declares it. c2rust copies each C struct
    # and typedef into EVERY module that references it, and the copies are
    # distinct Rust types the compiler will not unify. Which module a type
    # came from is therefore load-bearing information for anyone calling the
    # crate — and it appears NOWHERE in a signature, because c2rust writes
    # signatures unqualified (same-module reference). Keeping only the deduped
    # NAME, as this inventory used to, discards exactly the fact a caller needs.
    type_modules: dict[str, list[str]] = field(default_factory=dict)
    toolchain_file: Path | None = None   # rust-toolchain.toml to copy verbatim

    # Alias name → right-hand side, for deciding which aliases are infected.
    type_aliases: dict[str, str] = field(default_factory=dict)

    @property
    def duplicated_types(self) -> dict[str, list[str]]:
        """Types with more than one INCOMPATIBLE copy, worst offenders first.

        Appearing in many modules is not by itself a problem. c2rust re-emits
        `pub type size_t = c_ulong;` in 41 modules of optipng, but all 41 are
        the same type — they bottom out in a primitive, so passing one where
        another is expected compiles fine. Listing those would be noise, and
        noise is what makes a caller ignore the list.

        A copy is incompatible only if it bottoms out in a struct that is
        itself duplicated. So: start from the duplicated structs, then take the
        closure over aliases that mention an already-infected name.
        `png_const_structrp = *const png_struct` is infected through
        `png_struct`, and that is exactly the alias callers get wrong — its
        definition is byte-identical in all 23 modules.
        """
        infected = {n for n, m in self.type_modules.items()
                    if len(m) > 1 and n not in self.type_aliases}
        changed = True
        while changed:                      # closure: aliases of aliases
            changed = False
            for name, rhs in self.type_aliases.items():
                if name in infected or len(self.type_modules.get(name, ())) < 2:
                    continue
                if any(tok in infected for tok in _IDENT.findall(rhs)):
                    infected.add(name)
                    changed = True
        return dict(sorted(((n, self.type_modules[n]) for n in infected),
                           key=lambda kv: (-len(kv[1]), kv[0])))

    @property
    def fn_names(self) -> set[str]:
        return {f.name for f in self.fns}

    @property
    def link_names(self) -> set[str]:
        """Exported symbols — what llvm-cov names each function's record."""
        return {f.link_name for f in self.fns}

    @property
    def link_to_name(self) -> dict[str, str]:
        """Exported symbol → Rust item name (for human-readable feedback)."""
        return {f.link_name: f.name for f in self.fns}


def _package_name(cargo_toml: str) -> str:
    # Take the `name =` that follows [package] (a [workspace] or [lib]
    # section may precede/follow it).
    in_pkg = False
    for line in cargo_toml.splitlines():
        s = line.strip()
        if s.startswith("["):
            in_pkg = s == "[package]"
            continue
        if in_pkg:
            m = _PKG_NAME.match(line)
            if m:
                return m.group(1)
    raise ValueError("no [package].name in Cargo.toml")


def _bin_paths(cargo_toml: str) -> set[str]:
    """Paths of [[bin]] targets — excluded from the library API scan."""
    paths: set[str] = set()
    for m in re.finditer(r'\[\[bin\]\](.*?)(?=\n\[|\Z)', cargo_toml, re.S):
        pm = re.search(r'path\s*=\s*"([^"]+)"', m.group(1))
        if pm:
            paths.add(pm.group(1))
    return paths


def _signature_at(text: str, start: int) -> str:
    """From the `pub … fn name` match, walk to the opening `{` of the body
    (or a terminating `;` for a decl) at paren depth 0."""
    depth = 0
    for i in range(start, min(len(text), start + 4000)):
        c = text[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        elif depth == 0 and c in "{;":
            sig = text[start:i]
            return re.sub(r"\s+", " ", sig).strip()
    return re.sub(r"\s+", " ", text[start:start + 400]).strip()


def _link_name_before(text: str, fn_start: int, rust_name: str) -> str:
    """The symbol the fn is exported under, from the attribute block that
    immediately precedes it. c2rust marks every `pub extern "C" fn` with
    either `#[no_mangle]` (symbol == Rust name) or, when the C name is a
    Rust keyword, `#[export_name = "<c name>"]` (symbol != Rust name, e.g.
    `match_0` exported as `match`). llvm-cov records functions by this
    symbol, so coverage matching must use it — not the Rust item name.

    Walk backward over the contiguous block of attribute/comment/blank
    lines directly above the fn; stop at the first real code line.
    """
    head = text.rfind("\n", 0, fn_start)
    lines = text[:head if head >= 0 else fn_start].splitlines()
    for line in reversed(lines):
        s = line.strip()
        if not s or s.startswith("//"):
            continue
        if s.startswith("#["):
            m = _EXPORT_NAME.search(s)
            if m:
                return m.group(1)
            # keep scanning the attr block (no_mangle / other attrs)
            continue
        break   # hit a non-attribute line — attribute block ended
    return rust_name


def _module_path(rel: Path) -> str:
    """src/lodepng.rs → 'src::lodepng'; src/foo/mod.rs → 'src::foo'."""
    parts = list(rel.parts[:-1])
    stem = rel.stem
    if stem != "mod":
        parts.append(stem)
    return "::".join(parts)


def scan_crate(crate_dir: Path) -> CrateInventory:
    crate_dir = crate_dir.resolve()
    cargo_toml_path = crate_dir / "Cargo.toml"
    if not cargo_toml_path.exists():
        raise FileNotFoundError(f"not a crate (no Cargo.toml): {crate_dir}")
    cargo_toml = cargo_toml_path.read_text(encoding="utf-8", errors="replace")

    crate_name = _package_name(cargo_toml)
    skip = {str(crate_dir / p) for p in _bin_paths(cargo_toml)}

    lib_rs = ""
    for cand in ("lib.rs", "src/lib.rs"):
        p = crate_dir / cand
        if p.exists():
            lib_rs = p.read_text(encoding="utf-8", errors="replace")
            skip.add(str(p))
            break

    inv = CrateInventory(crate_dir=crate_dir, crate_name=crate_name,
                         lib_rs=lib_rs[:4000])

    tc = crate_dir / "rust-toolchain.toml"
    if tc.exists():
        inv.toolchain_file = tc

    seen_structs: set[str] = set()
    for rs in sorted(crate_dir.rglob("*.rs")):
        sp = str(rs)
        if "/target/" in sp or sp in skip:
            continue
        text = rs.read_text(encoding="utf-8", errors="replace")
        rel = rs.relative_to(crate_dir)
        mod = _module_path(rel)
        for m in _FN_HEAD.finditer(text):
            rust_name = m.group(1)
            inv.fns.append(ApiFn(
                name=rust_name,
                link_name=_link_name_before(text, m.start(), rust_name),
                signature=_signature_at(text, m.start()),
                module_path=mod,
                file=str(rel),
            ))
        for m in _STRUCT.finditer(text):
            if m.group(1) not in seen_structs:
                seen_structs.add(m.group(1))
                inv.structs.append(m.group(1))
        # Record WHERE each type lives, structs and aliases alike. The dedupe
        # above is kept for `structs` (a plain name list, used as a menu), but
        # the module list must not be deduped — its whole value is the count.
        for m in _STRUCT.finditer(text):
            mods = inv.type_modules.setdefault(m.group(1), [])
            if mod not in mods:
                mods.append(mod)
        for m in _TYPE_ALIAS.finditer(text):
            mods = inv.type_modules.setdefault(m.group(1), [])
            if mod not in mods:
                mods.append(mod)
            inv.type_aliases.setdefault(m.group(1), m.group(2).strip())
    return inv
