"""LLVM IR text parser for the DI metadata we need to attribute call
sites to source functions.

We DO NOT parse LLVM IR into an AST — that would require a full LLVM
bindings dependency. Instead we do line-oriented regex extraction:

  * `!N = !DILocation(line: L, column: C, scope: !M, inlinedAt: !K)`
    → `di_location[N] = DILocation(line=L, col=C, scope=M, inlined_at=K)`
  * `!N = distinct !DISubprogram(name: "...", ..., file: !F, ...)`
    → `di_subprogram[N] = DISubprogram(name="...", file_id=F)`
  * `!N = !DIFile(filename: "...", directory: "...")`
    → `di_file[N] = DIFile(filename="...", directory="...")`

DILocation's `line`/`column` may be absent (unwind blocks etc.); we
default missing fields to None. `inlinedAt: !K` is optional (top-level
frame has no inlinedAt); we default to None.

Building the maps is one linear pass over the .ll file's tail section
(metadata is emitted after all functions, so we can grep from the last
`define` down, but a full-file grep is fast enough and simpler).
"""

from __future__ import annotations

import logging
import re
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger("hot_probe.class_I.ir_parse")


# ── metadata dataclasses ────────────────────────────────────────────────────

@dataclass(frozen=True)
class DILocation:
    line: int | None
    col: int | None
    scope: int
    inlined_at: int | None    # None = top-level frame

@dataclass(frozen=True)
class DISubprogram:
    name: str                 # unmangled name (`name:` attr)
    linkage_name: str | None  # mangled (`linkageName:` attr) — may be absent
    file_id: int | None       # DIFile id, or None if not resolvable

@dataclass(frozen=True)
class DIFile:
    filename: str
    directory: str

    @property
    def path(self) -> str:
        """`directory/filename`. If directory is absolute, that's the abs
        path; else it's relative to the crate root and is kept relative
        (crate-side attribution only needs the crate-relative path)."""
        if not self.directory:
            return self.filename
        if self.directory.endswith("/"):
            return self.directory + self.filename
        return self.directory + "/" + self.filename


# ── regex patterns ──────────────────────────────────────────────────────────

# `!42 = !DILocation(line: 501, column: 22, scope: !37, inlinedAt: !56)`
# `line:` / `column:` may be absent; `inlinedAt:` optional.
_RE_DILOC = re.compile(
    r"^!(?P<id>\d+)\s*=\s*!DILocation\("
    r"(?:line:\s*(?P<line>\d+)(?:,\s*)?)?"
    r"(?:column:\s*(?P<col>\d+)(?:,\s*)?)?"
    r"scope:\s*!(?P<scope>\d+)"
    r"(?:,\s*inlinedAt:\s*!(?P<inl>\d+))?"
    r"\)"
)

# `!43 = distinct !DISubprogram(name: "foo", linkageName: "_ZN..", scope: !x,
#                               file: !y, line: 12, ...)`
# name / linkageName / file are what we need; others are ignored.
_RE_SUBPROG = re.compile(
    r"^!(?P<id>\d+)\s*=\s*(?:distinct\s+)?!DISubprogram\((?P<body>.*)\)\s*$"
)
_RE_SP_NAME       = re.compile(r'\bname:\s*"([^"]*)"')
_RE_SP_LINKAGE    = re.compile(r'\blinkageName:\s*"([^"]*)"')
_RE_SP_FILE       = re.compile(r'\bfile:\s*!(\d+)')

# `!44 = !DIFile(filename: "src/foo.rs", directory: "/home/…/crate")`
_RE_DIFILE = re.compile(
    r"^!(?P<id>\d+)\s*=\s*!DIFile\("
    r'filename:\s*"(?P<fn>[^"]*)"'
    r'(?:,\s*directory:\s*"(?P<dir>[^"]*)")?'
)

# DILexicalBlock / DILexicalBlockFile — intermediate scope wrappers. Both
# just delegate to their `scope:` field, which itself may be another lexical
# block, or the DISubprogram we actually want. `scope_to_subprogram` walks
# through these transparently.
#   `!47 = distinct !DILexicalBlock(scope: !37, file: !1, line: 400, column: 5)`
#   `!47 = !DILexicalBlockFile(scope: !37, file: !1, discriminator: 0)`
_RE_DILEXBLK = re.compile(
    r"^!(?P<id>\d+)\s*=\s*(?:distinct\s+)?!DILexicalBlock"
    r"(?:File)?\("
    r".*?\bscope:\s*!(?P<scope>\d+)"
)


# ── parser ──────────────────────────────────────────────────────────────────

@dataclass
class MetaTables:
    """Four lookup tables consumed by `attribute.walk_frames`."""
    locations:      dict[int, DILocation]
    subprograms:    dict[int, DISubprogram]
    files:          dict[int, DIFile]
    lexical_scopes: dict[int, int]                # lex-block id → parent scope id

    def scope_to_subprogram(self, scope_id: int) -> DISubprogram | None:
        """A DILocation's `scope` is either a DISubprogram or a DILexicalBlock/
        DILexicalBlockFile (which chains to a DISubprogram via its own `scope:`).
        Walk lexical-scope pointers until we land on a subprogram or exhaust
        the chain. Bounded to 32 hops (LLVM never nests that deep)."""
        cur = scope_id
        for _ in range(32):
            sp = self.subprograms.get(cur)
            if sp is not None:
                return sp
            nxt = self.lexical_scopes.get(cur)
            if nxt is None or nxt == cur:
                return None
            cur = nxt
        return None


def parse_ir_metadata(ir_path: Path) -> MetaTables:
    """One linear scan over the .ll file, populating four DI tables."""
    ir_path = Path(ir_path)
    locations:      dict[int, DILocation]   = {}
    subprograms:    dict[int, DISubprogram] = {}
    files:          dict[int, DIFile]       = {}
    lexical_scopes: dict[int, int]          = {}

    with ir_path.open("r", encoding="utf-8", errors="replace") as fh:
        for line in fh:
            # Metadata lines start with `!<n> = `; a strict prefix check
            # cheaply skips the ~95% of the .ll file that is IR body.
            if not line.startswith("!") or " = " not in line[:16]:
                continue
            s = line.rstrip("\n")

            m = _RE_DILOC.match(s)
            if m is not None:
                nid = int(m.group("id"))
                locations[nid] = DILocation(
                    line=int(m.group("line")) if m.group("line") else None,
                    col=int(m.group("col")) if m.group("col") else None,
                    scope=int(m.group("scope")),
                    inlined_at=int(m.group("inl")) if m.group("inl") else None,
                )
                continue

            m = _RE_SUBPROG.match(s)
            if m is not None:
                nid = int(m.group("id"))
                body = m.group("body")
                nm = _RE_SP_NAME.search(body)
                lk = _RE_SP_LINKAGE.search(body)
                fl = _RE_SP_FILE.search(body)
                if nm is None:
                    continue
                subprograms[nid] = DISubprogram(
                    name=nm.group(1),
                    linkage_name=lk.group(1) if lk else None,
                    file_id=int(fl.group(1)) if fl else None,
                )
                continue

            m = _RE_DIFILE.match(s)
            if m is not None:
                nid = int(m.group("id"))
                files[nid] = DIFile(
                    filename=m.group("fn"),
                    directory=m.group("dir") or "",
                )
                continue

            m = _RE_DILEXBLK.match(s)
            if m is not None:
                nid = int(m.group("id"))
                lexical_scopes[nid] = int(m.group("scope"))

    logger.info("[ir_parse] %s: %d DILocations, %d DISubprograms, "
                "%d DIFiles, %d lex-scopes",
                ir_path.name, len(locations), len(subprograms), len(files),
                len(lexical_scopes))
    return MetaTables(locations=locations, subprograms=subprograms,
                       files=files, lexical_scopes=lexical_scopes)


                                                                            

# std/core generic frames (Option::expect, slice Index::index, …) are call-site
                                                                       
# frame. Detection: the DIFile path starts with one of these prefixes.
_STDLIB_PREFIXES: tuple[str, ...] = (
    "library/", "/rustc/", "/rust/deps/",
    "/cargo/", "/.cargo/",
)


def is_stdlib_file(fi: DIFile | None) -> bool:
    """True if this DIFile refers to stdlib / rustc-embedded / cargo dep
    source — attribution should skip through it (see RQ3 note in
    class_I/bzip2.md line 7)."""
    if fi is None:
        return False
    p = fi.path
    return any(seg in p for seg in _STDLIB_PREFIXES)
