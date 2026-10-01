"""Stage A Phase A — global static analyzer.

CST-based scan via tree-sitter-rust. Produces ONE `AnalysisReport` that
downstream phases (Transformer, Verifier) read; nobody else reaches into
the file system afterwards.

What it figures out:

  * `candidates`           — every `#[no_mangle] (pub) (unsafe) extern "C" fn`
                             (or `#[linkage = "external"]` equivalent) in the
                             project, recorded with precise CST byte ranges
                             for the attributes, the `extern "C"` modifier
                             slice, and the body span (line count fuels the
                             #[inline] decision).

  * `harness_imports`      — symbol → set of harness/* files that declare it
                             via `extern "C" { fn NAME(...); }`. These bind
                             the symbol to a real external linker contract.

  * `internal_extern_blocks` — every project-internal `extern "C" {}` block
                             plus its fn declarations. These are c2rust
                             same-crate re-export noise that the Transformer
                             will rewrite into `use crate::...::NAME;`.

  * `cross_file_call_sites` — symbol → set of files that reference the
                             symbol as an identifier (c2rust's central
                             ffi.rs re-export model means callers may not
                             have a per-file extern block; the Transformer
                             must still add a `use` import there).

  * `api_whitelist`        — symbols the user enumerated in
                             `workloads/<>.toml [api_surface]`.

  * `crate_has_c_consumers` — Cargo.toml [lib].crate-type contains
                             cdylib/staticlib AND no [[bin]] (a "true
                             librarian" project, not a c2rust translation
                             artifact noise).

Frozen / SafeToStrip computed in `classify_candidates()` from this report.
"""

from __future__ import annotations

import logging
import re as _re  # used only by `_crate_has_c_consumers` for `pub fn main` scan
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

from tree_sitter import Language, Node, Parser
import tree_sitter_rust

logger = logging.getLogger(__name__)

_LANG = Language(tree_sitter_rust.language())
_PARSER = Parser(_LANG)


# ─────────────────────────────────────────────────────────────────
# Data model
# ─────────────────────────────────────────────────────────────────

@dataclass
class Candidate:
    """A `#[no_mangle] pub (unsafe) extern "C" fn` definition we may strip."""
    name: str
    file: Path
    # Byte ranges (start, end) in the file's bytes.
    item_range: tuple[int, int]            # full function_item span
    body_line_count: int                   # for #[inline] decision
    # Attributes attached immediately above the fn (excluding doc comments)
    attr_no_mangle_range: tuple[int, int] | None
    attr_linkage_range: tuple[int, int] | None
    attr_inline_present: bool              # already has #[inline] (skip hint)
    attr_cold_present: bool                # has #[cold] (skip hint)
    # Range of just the `extern "C"` modifier inside function_modifiers,
    # including any trailing whitespace up to the next modifier or `fn`.
    extern_modifier_range: tuple[int, int]
    # Visibility info: c2rust translates internal helper fns as
    # `unsafe extern "C" fn foo(…)` (no `pub`), relying on extern "C"
    # + #[no_mangle] for cross-module linker resolution. Once we strip
    # extern "C", any `use lib::module::foo` from a binary or other
    # module requires `pub` visibility — so the transformer must promote
    # the fn to `pub` if it isn't already.
    has_pub: bool
    # The position where `pub ` should be inserted if has_pub is False
    # (1 byte before the first token of the fn item, after any leading
    # whitespace; transformer back-walks from this start anyway).
    fn_item_start: int


@dataclass
class ExternBlockDecl:
    """One `(pub) fn NAME(...);` declaration inside an `extern "C" {}` block."""
    name: str
    range: tuple[int, int]                 # full declaration span incl. ;


@dataclass
class ExternBlock:
    """One project-internal `extern "C" {...}` block (foreign_mod_item)."""
    file: Path
    block_range: tuple[int, int]
    inner_range: tuple[int, int]           # range between { and }
    decls: list[ExternBlockDecl]


@dataclass
class AnalysisReport:
    project_dir: Path
    crate_name: str
    crate_has_c_consumers: bool
    bin_file_stems: set[str]
    candidates: list[Candidate]
    harness_imports: dict[str, set[Path]]
    internal_extern_blocks: list[ExternBlock]
    cross_file_call_sites: dict[str, set[Path]]
    api_whitelist: set[str]
    # Fns whose identifier appears on the LHS of a
    # `<name> as (unsafe) extern "C" fn(...)` cast anywhere in the project
    # or its harness. Such fns CANNOT have their extern "C" stripped — doing
    # so makes the cast E0605 (non-primitive cast from non-extern fn to
    # extern fn type). Detected mechanically via tree-sitter in `_walk_file`.
    address_taken_fns: set[str] = field(default_factory=set)
    # Computed by classify_candidates()
    safe_to_strip: list[Candidate] = field(default_factory=list)
    frozen: list[Candidate] = field(default_factory=list)
    frozen_reasons: dict[str, str] = field(default_factory=dict)


# ─────────────────────────────────────────────────────────────────
# Parser helpers
# ─────────────────────────────────────────────────────────────────

def _parse(path: Path) -> tuple[bytes, Node]:
    src = path.read_bytes()
    tree = _PARSER.parse(src)
    return src, tree.root_node


def _txt(src: bytes, n: Node) -> str:
    return src[n.start_byte:n.end_byte].decode("utf-8", errors="replace")


def _child(node: Node, kind: str) -> Node | None:
    for c in node.children:
        if c.type == kind:
            return c
    return None


def _all_children(node: Node, kind: str) -> list[Node]:
    return [c for c in node.children if c.type == kind]


def _line_count(src: bytes, range_: tuple[int, int]) -> int:
    return src[range_[0]:range_[1]].count(b"\n") + 1


# ─────────────────────────────────────────────────────────────────
# Visitor — walk the file tree once and emit everything
# ─────────────────────────────────────────────────────────────────

@dataclass
class _FileFacts:
    candidates: list[Candidate] = field(default_factory=list)
    extern_blocks: list[ExternBlock] = field(default_factory=list)
    identifier_calls: set[str] = field(default_factory=set)
    # Identifier names appearing on the LHS of a
    # `<name> as (unsafe) extern "C" fn(...)` cast in this file.
    address_taken: set[str] = field(default_factory=set)


def _attribute_name(src: bytes, attr_item: Node) -> str:
    """Extract attribute name. Handles `#[no_mangle]`, `#[linkage = "..."]`,
    `#[inline(always)]`, etc. Returns the bare path identifier
    (e.g. "no_mangle", "linkage", "inline", "cold")."""
    # attribute_item -> '#' '[' attribute ']'
    attr = _child(attr_item, "attribute")
    if attr is None:
        return ""
    # attribute -> identifier (= 'no_mangle')  OR  scoped_identifier
    #              optionally followed by '= literal' or '(...)'
    for c in attr.children:
        if c.type == "identifier":
            return _txt(src, c)
        if c.type == "scoped_identifier":
            # scoped_identifier > identifier (last)
            ids = _all_children(c, "identifier")
            if ids:
                return _txt(src, ids[-1])
    return ""


def _function_modifiers(node: Node) -> Node | None:
    return _child(node, "function_modifiers")


def _has_extern_c(src: bytes, fn_item: Node) -> tuple[bool, tuple[int, int] | None]:
    """Is this function_item `extern "C"`? Returns (yes, range_of_extern_C).
    range covers `extern "C"` token + trailing whitespace to next sibling."""
    mods = _function_modifiers(fn_item)
    if mods is None:
        return False, None
    em = _child(mods, "extern_modifier")
    if em is None:
        return False, None
    # extern_modifier -> 'extern' string_literal
    abi = _child(em, "string_literal")
    if abi is None:
        return False, None
    if _txt(src, abi).strip('"') != "C":
        return False, None
    # Range = extern_modifier start .. trailing whitespace before next sibling.
    start = em.start_byte
    end = em.end_byte
    # Eat trailing whitespace up to the next sibling under function_modifiers
    # (typically a space before `fn`).
    while end < len(src) and src[end:end+1] in (b" ", b"\t"):
        end += 1
    return True, (start, end)


def _fn_name(src: bytes, fn_item: Node) -> str | None:
    for c in fn_item.children:
        if c.type == "identifier":
            return _txt(src, c)
    return None


def _fn_body_node(fn_item: Node) -> Node | None:
    return _child(fn_item, "block")


def _preceding_attributes(fn_item: Node) -> list[Node]:
    """Find contiguous `attribute_item` siblings immediately preceding
    `fn_item` (no statements / non-attribute items in between)."""
    parent = fn_item.parent
    if parent is None:
        return []
    out: list[Node] = []
    siblings = list(parent.children)
    try:
        idx = siblings.index(fn_item)
    except ValueError:
        return []
    j = idx - 1
    while j >= 0:
        sib = siblings[j]
        if sib.type in ("attribute_item", "inner_attribute_item"):
            out.append(sib)
            j -= 1
        elif sib.is_named:
            break
        else:
            j -= 1
    return list(reversed(out))


def _function_type_is_extern_c(src: bytes, ft_node: Node) -> bool:
    """True if a `function_type` node carries `extern "C"`. CST shape:
        function_type
          function_modifiers
            extern_modifier
              extern
              string_literal '"C"'
    """
    mods = _child(ft_node, "function_modifiers")
    if mods is None:
        return False
    em = _child(mods, "extern_modifier")
    if em is None:
        return False
    abi = _child(em, "string_literal")
    if abi is None:
        return False
    return _txt(src, abi).strip('"') == "C"


def _cast_lhs_identifier(src: bytes, cast_node: Node) -> str | None:
    """For a `type_cast_expression` whose RHS is an `extern "C" fn` type,
    return the LHS identifier name (the address-taken fn). Returns None for
    non-identifier LHS (e.g. parenthesized expr, field access) — those are
    unusual and out of scope for Plan A.

    CST shape:
        type_cast_expression
          identifier                ← we want this
          as
          function_type             ← must be extern "C" fn
    """
    # The CST stores children in source order; first non-trivia child is LHS.
    lhs = None
    rhs = None
    for c in cast_node.children:
        if c.type == "as":
            continue
        if lhs is None:
            lhs = c
        else:
            rhs = c
            break
    if lhs is None or rhs is None:
        return None
    if lhs.type != "identifier":
        return None
    if rhs.type != "function_type":
        return None
    if not _function_type_is_extern_c(src, rhs):
        return None
    return _txt(src, lhs)


def _decl_in_extern_block(src: bytes, mod_node: Node) -> list[ExternBlockDecl]:
    """Walk `foreign_mod_item` children of `mod_node` (a `declaration_list`)
    and pull out `function_signature_item` nodes."""
    out: list[ExternBlockDecl] = []
    for c in mod_node.children:
        if c.type != "function_signature_item":
            continue
        name = _fn_name(src, c)
        if name is None:
            continue
        # We'll consume the whole declaration including trailing semicolon
        # AND trailing newline (so deleting it doesn't leave a blank line).
        start = c.start_byte
        # Back-walk to start of line to also pull in any leading whitespace
        # (keeps indentation clean after deletion).
        while start > 0 and src[start-1:start] in (b" ", b"\t"):
            start -= 1
        end = c.end_byte
        if end < len(src) and src[end:end+1] == b"\n":
            end += 1
        out.append(ExternBlockDecl(name=name, range=(start, end)))
    return out


def _walk_file(file: Path) -> _FileFacts:
    src, root = _parse(file)
    facts = _FileFacts()

    # Identifier name set used elsewhere in the file (for call-site scan).
    # We only care about identifiers used in expression position, but a
    # cheap proxy is "all identifier nodes whose parent is not a fn def or
    # extern block declaration". We'll filter against candidate names in
    # the Analyzer.
    # For perf, we don't materialize ALL identifiers — we materialize the
    # subset that matches candidate names. The caller will pass us the
    # candidate set later; for now we cache *all* identifier strings.
    # NB: tree-sitter walk is fast enough on these files.
    stack = [root]
    while stack:
        n = stack.pop()
        if n.type == "function_item":
            name = _fn_name(src, n)
            has_c, em_range = _has_extern_c(src, n)
            if name and has_c:
                attrs = _preceding_attributes(n)
                attr_nm = None
                attr_lk = None
                has_inline = False
                has_cold = False
                for a in attrs:
                    nm = _attribute_name(src, a)
                    if nm == "no_mangle":
                        end = a.end_byte
                        # eat trailing newline
                        if end < len(src) and src[end:end+1] == b"\n":
                            end += 1
                        attr_nm = (a.start_byte, end)
                    elif nm == "linkage":
                        end = a.end_byte
                        if end < len(src) and src[end:end+1] == b"\n":
                            end += 1
                        attr_lk = (a.start_byte, end)
                    elif nm == "inline":
                        has_inline = True
                    elif nm == "cold":
                        has_cold = True

                body = _fn_body_node(n)
                body_lc = (
                    _line_count(src, (body.start_byte, body.end_byte))
                    if body else 0
                )
                # Check for `pub` (or `pub(...)`) visibility modifier.
                vis_node = _child(n, "visibility_modifier")
                facts.candidates.append(Candidate(
                    name=name,
                    file=file,
                    item_range=(n.start_byte, n.end_byte),
                    body_line_count=body_lc,
                    attr_no_mangle_range=attr_nm,
                    attr_linkage_range=attr_lk,
                    attr_inline_present=has_inline,
                    attr_cold_present=has_cold,
                    extern_modifier_range=em_range,
                    has_pub=(vis_node is not None),
                    fn_item_start=n.start_byte,
                ))
            # Still descend so nested items / call sites in the body get scanned.
            stack.extend(n.children)
            continue

        if n.type == "foreign_mod_item":
            # `extern "C" { ... }` — the inner `declaration_list` lists fn decls.
            abi = None
            decl_list = None
            for c in n.children:
                if c.type == "extern_modifier":
                    abi_lit = _child(c, "string_literal")
                    if abi_lit and _txt(src, abi_lit).strip('"') == "C":
                        abi = "C"
                elif c.type == "declaration_list":
                    decl_list = c
            if abi == "C" and decl_list is not None:
                decls = _decl_in_extern_block(src, decl_list)
                facts.extern_blocks.append(ExternBlock(
                    file=file,
                    block_range=(n.start_byte, n.end_byte),
                    inner_range=(decl_list.start_byte, decl_list.end_byte),
                    decls=decls,
                ))
            stack.extend(n.children)
            continue

        if n.type == "type_cast_expression":
            name = _cast_lhs_identifier(src, n)
            if name is not None:
                facts.address_taken.add(name)
            # still descend — nested casts inside the LHS are unlikely but
            # cheap to keep covered
            stack.extend(n.children)
            continue

        if n.type == "identifier":
            facts.identifier_calls.add(_txt(src, n))
        stack.extend(n.children)
    return facts


# ─────────────────────────────────────────────────────────────────
# Top-level analyze() — Phase A entry point
# ─────────────────────────────────────────────────────────────────

def _load_cargo(project_dir: Path) -> dict:
    return tomllib.loads((project_dir / "Cargo.toml").read_text())


def _crate_name(cargo: dict) -> str:
    return cargo.get("lib", {}).get("name") or cargo["package"]["name"]


def _bin_stems(cargo: dict) -> set[str]:
    out: set[str] = set()
    for entry in cargo.get("bin", []):
        if path := entry.get("path"):
            out.add(Path(path).stem)
    return out


def _crate_has_c_consumers(cargo: dict, project_dir: Path | None = None) -> bool:
    lib = cargo.get("lib", {})
    types = lib.get("crate-type", [])
    has_c_type = any(t in ("cdylib", "staticlib") for t in types)
    has_bin = bool(cargo.get("bin"))
    if not has_c_type or has_bin:
        return has_c_type and not has_bin
    # c2rust often emits Cargo.toml with crate-type=["staticlib","rlib"]
    # but NO [[bin]] entry even when one of the source .rs files contains
    # `pub fn main()` (translated from the C program's main). In that case
    # the staticlib is unused — the project is actually a binary. Detect
    # this and don't treat it as a C-consumer crate.
    if project_dir is not None:
        src_dir = project_dir / "src"
        if src_dir.is_dir():
            main_re = _re.compile(r"^\s*pub fn main\s*\(", _re.MULTILINE)
            for rs in src_dir.rglob("*.rs"):
                try:
                    text = rs.read_text(encoding="utf-8", errors="replace")
                except OSError:
                    continue
                if main_re.search(text):
                    return False
    return has_c_type and not has_bin


def _load_api_whitelist(project_dir: Path) -> set[str]:
    wl_dir = project_dir.parent / "workloads"
    api: set[str] = set()
    if not wl_dir.is_dir():
        return api
    for tml in wl_dir.glob("*.toml"):
        try:
            data = tomllib.loads(tml.read_text())
        except (OSError, tomllib.TOMLDecodeError):
            continue
        surface = data.get("api_surface")
        if isinstance(surface, list):
            api.update(str(s) for s in surface)
        elif isinstance(surface, dict):
            syms = surface.get("symbols")
            if isinstance(syms, list):
                api.update(str(s) for s in syms)
    return api


def _project_rs_files(project_dir: Path,
                      exclude_bin_stems: set[str] | None = None) -> list[Path]:
    """Return all .rs files under `project_dir` excluding `target/` and
    optionally any [[bin]] entry-point sources. The latter are NOT lib
    modules — they typically contain test/example code that would
    confuse an extern "C" fn scan (e.g. libcsv 0_raw's `src/test_csv.rs`
    defines `pub unsafe extern "C" fn cb1`, an example callback)."""
    exclude_bin_stems = exclude_bin_stems or set()
    out: list[Path] = []
    for p in sorted(set(project_dir.rglob("*.rs"))):
        if "target" in p.parts:
            continue
        # Skip files whose stem is registered as a [[bin]] in Cargo.toml.
        if p.stem in exclude_bin_stems:
            continue
        out.append(p)
    return out


def _harness_dirs(project_dir: Path) -> list[Path]:
    """Real external consumers under <project>/workloads/harness/*."""
    harness_root = project_dir.parent / "workloads" / "harness"
    if not harness_root.is_dir():
        return []
    return [p for p in harness_root.iterdir() if p.is_dir()]


def analyze(project_dir: Path) -> AnalysisReport:
    project_dir = project_dir.resolve()
    cargo = _load_cargo(project_dir)
    crate = _crate_name(cargo)
    bins = _bin_stems(cargo)
    has_c_consumers = _crate_has_c_consumers(cargo, project_dir)
    whitelist = _load_api_whitelist(project_dir)

    # Phase A.1 — scan project files. We split into two roles:
    #   (a) For CANDIDATE collection (fns we may strip): exclude [[bin]]
    #       sources — those are entry-point glue, not library code, and
    #       often define helper fns (e.g. test_csv.rs's `cb1`/`cb2`)
    #       that look like extern "C" fn but aren't library APIs.
    #   (b) For EXTERN BLOCK collection (declarations to rewrite when a
    #       lib fn is lifted): scan ALL files including bin, because bin
    #       sources frequently `extern "C" { fn BZ2_… }` to call the lib
    #       via C ABI and must be rewired alongside lib changes.
    #   (c) For CALL-SITE map (where lifted fns are used): again, scan
    #       all files — bin sources may also reference lifted fns.
    all_candidates: list[Candidate] = []
    all_blocks: list[ExternBlock] = []
    identifiers_by_file: dict[Path, set[str]] = {}
    address_taken_fns: set[str] = set()

    # (a) candidate-only scan
    for f in _project_rs_files(project_dir, exclude_bin_stems=bins):
        ff = _walk_file(f)
        all_candidates.extend(ff.candidates)

    # (b) + (c) extern-block + identifier scan over ALL files (incl. bin)
    for f in _project_rs_files(project_dir):
        ff = _walk_file(f)
        all_blocks.extend(ff.extern_blocks)
        identifiers_by_file[f] = ff.identifier_calls
        # Collect address-taken-as-extern-C-fn across the WHOLE project,
        # not just lib files — bin entry points (e.g. lil's main.rs in
        # src/) frequently hold the callback registration tables.
        address_taken_fns.update(ff.address_taken)

    # Phase A.2 — scan harness/* siblings for real external imports
    harness_imports: dict[str, set[Path]] = {}
    for hd in _harness_dirs(project_dir):
        for hf in _project_rs_files(hd):
            try:
                ff = _walk_file(hf)
            except Exception as e:
                logger.warning(f"[analyzer] skip harness file {hf}: {e}")
                continue
            for blk in ff.extern_blocks:
                for d in blk.decls:
                    harness_imports.setdefault(d.name, set()).add(hf)
            # Harness code may also stash project fns as callbacks.
            address_taken_fns.update(ff.address_taken)

    # Phase A.3 — cross-file call-site map (callers of each candidate).
    cand_names = {c.name for c in all_candidates}
    name_to_def_file = {c.name: c.file for c in all_candidates}
    cross_file_calls: dict[str, set[Path]] = {n: set() for n in cand_names}
    for f, idents in identifiers_by_file.items():
        for name in idents & cand_names:
            if f.resolve() == name_to_def_file[name].resolve():
                continue
            cross_file_calls[name].add(f.resolve())

    report = AnalysisReport(
        project_dir=project_dir,
        crate_name=crate,
        crate_has_c_consumers=has_c_consumers,
        bin_file_stems=bins,
        candidates=all_candidates,
        harness_imports=harness_imports,
        internal_extern_blocks=all_blocks,
        cross_file_call_sites=cross_file_calls,
        api_whitelist=whitelist,
        address_taken_fns=address_taken_fns,
    )
    logger.info(
        f"[analyzer] {project_dir.name}: "
        f"{len(all_candidates)} extern \"C\" fn candidates, "
        f"{len(all_blocks)} internal extern blocks, "
        f"{sum(len(v) for v in harness_imports.values())} harness import declarations, "
        f"{len(address_taken_fns)} address-taken-as-extern-C fn name(s)"
    )
    return report


# ─────────────────────────────────────────────────────────────────
# Classification (SafeToStripList vs FrozenSet)
# ─────────────────────────────────────────────────────────────────
#
# Address-taken detection: see `_cast_lhs_identifier` + `_walk_file` above.
# Replaces the previous `_has_fnptr_cast` regex scan (O(N_files × N_cands))
# with one CST walk per file (O(N_files)) emitted into
# `AnalysisReport.address_taken_fns`. `classify()` reads that set
# unconditionally — Plan A 2026-05-29.


def classify(
    report: AnalysisReport,
    *,
    mode: str = "conservative",
    hot_allowlist: set[str] | None = None,
) -> AnalysisReport:
    """Decide frozen vs safe-to-strip. Mutates and returns `report`.

    `mode`:
      * "conservative" (default) — strip only candidates that explicitly
        export a stable C symbol (`#[no_mangle]` or `#[linkage]`).
        Leaves internal `extern "C" fn` alone.
      * "aggressive" — also strip internal `extern "C" fn` provided NO
        `<fn_name> as extern "C" fn(...)` cast appears anywhere in the
        project. The 3-gate verifier (cargo check / build / W1)
        catches false negatives downstream.

    Default flipped from "aggressive" → "conservative" on 2026-05-27 after
    empirical validation on bzip2 (`dataset_trans_process/bzip2-1.0.8/`)
    showed aggressive mode is a net loss in practice:
      * The design goal — also lift hot internal fns like
        `BZ2_blockSort` / `BZ2_compressBlock` — fails because c2rust's
        per-file translation creates nominally-distinct copies of
        `EState` / `DState` across modules; stripping `extern "C"` on
        these fns triggers E0308 type mismatches at every cross-module
        call, and they freeze at the cargo-check gate anyway.
      * Meanwhile aggressive lifts ~30 extra cold internal helpers
        (compress.c-static fns, blocksort.c-static fns, etc.). These
        clear the gate but their stripped form contributes nothing to
        the hot path while taking measurable W2 hit, pushing the whole
        Stage A pass past Gate 4's regression threshold → full rollback.
      * Comparison on bzip2 2_stage_a / 1_cleaned baseline:
          aggressive: 62 safe-to-strip → 8 frozen-by-check + 54 lifted
                      → Gate 4 W2 +1.87%  → full rollback (E1=0)
          conservative: 33 safe-to-strip → 4 frozen + 29 lifted
                        → final W2 −1.12% vs baseline ✓ (E1=29, E2=2)
      * Historical -3.88% lift number in `project_bzip2_stage_a_2026_05_22`
        was measured BEFORE commit 61255f1 changed the default to
        aggressive — i.e. that experiment was conservative-equivalent.
    Aggressive remains available via `--classify-mode aggressive` for
    project-specific experimentation.
    """
    report.safe_to_strip = []
    report.frozen = []
    report.frozen_reasons = {}

    for cand in report.candidates:
        reason = None
        # Plan A (2026-05-29): address-taken-as-extern-C fns are frozen
        # in ALL modes, including conservative + hot_allowlist override.
        # Reason: any `<name> as (unsafe) extern "C" fn(...)` cast in the
        # project becomes E0605 ("non-primitive cast") the moment we strip
        # `extern "C"` from the fn's definition (cast of non-extern fn to
        # extern fn type is rejected by rustc). c2rust commonly emits this
        # pattern at callback-registration sites
        # (e.g. `lil_register(lil, name, Some(fnc_set as unsafe extern "C" fn(...)))`).
        # Without this gate, lil bled 61 such fns to cargo-check rejects.
        if cand.name in report.address_taken_fns:
            reason = (f"`{cand.name} as extern \"C\" fn(...)` cast present "
                      f"— fn used as C-ABI fn pointer; stripping extern "
                      f"would invalidate the cast (E0605)")
        elif cand.name in report.api_whitelist:
            reason = "in workloads [api_surface] whitelist"
        elif cand.name in report.harness_imports:
            reason = (f"imported by harness "
                      f"{[p.name for p in report.harness_imports[cand.name]]}")
        elif report.crate_has_c_consumers:
            reason = "crate-type cdylib/staticlib without [[bin]]"
        elif mode == "conservative" and not (
            cand.attr_no_mangle_range or cand.attr_linkage_range
        ):
            # hot_allowlist override: even without #[no_mangle], strip
            # this fn if it's on the W2 hot-path. Profile-guided escape
            # hatch for projects where c2rust emits all-internal extern
            # "C" fns and the real perf hot fns aren't `#[no_mangle]`.
            # (Example: lodepng inflateHuffmanBlock, lil next_word.)
            if hot_allowlist and cand.name in hot_allowlist:
                pass  # fall through to safe-to-strip
            else:
                reason = ("non-exported extern \"C\" fn (no #[no_mangle]/"
                          "#[linkage]); conservative mode skip")

        if reason:
            report.frozen.append(cand)
            report.frozen_reasons[cand.name] = reason
        else:
            report.safe_to_strip.append(cand)

    hot_overrides = (
        sum(
            1 for c in report.safe_to_strip
            if hot_allowlist and c.name in hot_allowlist
            and not (c.attr_no_mangle_range or c.attr_linkage_range)
        ) if hot_allowlist else 0
    )
    extra = f", {hot_overrides} hot-overrides" if hot_overrides else ""
    logger.info(
        f"[analyzer.classify mode={mode}] {len(report.candidates)} candidates → "
        f"{len(report.safe_to_strip)} safe-to-strip / "
        f"{len(report.frozen)} frozen{extra}"
    )
    return report
