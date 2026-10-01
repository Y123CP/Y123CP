"""Python wrapper for the `syn_walker` Rust subprocess.

Public entry points:
    list_structs(project_path) -> StructWalkerOutput
    list_allocs(project_path)  -> AllocWalkerOutput

Both build the syn_walker binary on first use (cached under
`target/release/syn_walker`) then shell it out with the right
subcommand, parse its JSON stdout, and hand back dataclasses.

The binary lives in the same directory as this module:
    proposer/syn_walker/Cargo.toml
    proposer/syn_walker/target/release/syn_walker  (built on demand)
"""

from __future__ import annotations

import json
import logging
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

logger = logging.getLogger(__name__)

_CRATE_DIR = Path(__file__).resolve().parent
_BINARY    = _CRATE_DIR / "target" / "release" / "syn_walker"


# ---------------------------------------------------------------------------
# Output schemas (mirror Rust-side serde structs)
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class FieldEvidence:
    name: str
    ty:   str
    line: int


@dataclass(frozen=True)
class StructEvidence:
    name:           str
    file:           str
    line:           int
    fields:         list[FieldEvidence]
    repr_attribute: str | None
    ffi_boundary:   bool
    is_pub:         bool
    derives:        list[str]


@dataclass(frozen=True)
class StructWalkerOutput:
    project_root:       str
    files_scanned:      int
    files_parse_failed: list[str]
    items:              list[StructEvidence]


@dataclass(frozen=True)
class AllocSiteEvidence:
    file:                str
    line:                int
    kind:                str
    size_arg:            str
    containing_fn:       str
    alloc_freq_estimate: int


@dataclass(frozen=True)
class AllocWalkerOutput:
    project_root:       str
    files_scanned:      int
    files_parse_failed: list[str]
    items:              list[AllocSiteEvidence]


@dataclass(frozen=True)
class LoopInvariantCond:
    """One `if`/`match` condition inside a loop body whose reads are not
    written anywhere in the loop body (nor the for-loop's induction
    variable) — a candidate for hoisting / pre-computation.

    The detection is lightweight syntactic — conservative writes (any
    `recv.method(...)` taints `recv`), so it under-reports rather than
    falsely claim invariance. Used by future rule triggers; *not* part of
    the §2.5 HotspotProfile (which is perf+compiler-only)."""
    file:          str
    line:          int
    loop_kind:     str        # "while" | "for" | "loop"
    loop_line:     int
    containing_fn: str
    cond_snippet:  str
    reads:         list[str]


@dataclass(frozen=True)
class LoopInvariantOutput:
    project_root:       str
    files_scanned:      int
    files_parse_failed: list[str]
    items:              list[LoopInvariantCond]


# ---------------------------------------------------------------------------
# Binary build / run
# ---------------------------------------------------------------------------

def ensure_built(*, force: bool = False) -> Path:
    """Build syn_walker if its release binary is missing. Returns the path.

    Cargo decides whether to rebuild based on source mtime — passing
    `force=True` runs `cargo build --release` regardless.
    """
    if not force and _BINARY.is_file():
        return _BINARY
    cargo = shutil.which("cargo")
    if cargo is None:
        raise RuntimeError("cargo not found on PATH; install Rust toolchain to use syn_walker")
    logger.info(f"[syn_walker] building {_CRATE_DIR}/syn_walker (release) ...")
    proc = subprocess.run(
        [cargo, "build", "--release"],
        cwd=str(_CRATE_DIR),
        capture_output=True, text=True, timeout=600,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"cargo build --release failed for syn_walker:\n"
            f"stdout: {proc.stdout}\nstderr: {proc.stderr}"
        )
    if not _BINARY.is_file():
        raise RuntimeError(f"syn_walker binary missing after build: {_BINARY}")
    return _BINARY


def _run_subcommand(subcmd: str, project_path: Path) -> dict:
    project_path = Path(project_path).resolve()
    binary = ensure_built()
    proc = subprocess.run(
        [str(binary), subcmd, "--root", str(project_path)],
        capture_output=True, text=True, timeout=120,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"syn_walker {subcmd} failed (rc={proc.returncode}):\n"
            f"stdout: {proc.stdout[-2000:]}\nstderr: {proc.stderr[-2000:]}"
        )
    return json.loads(proc.stdout)


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------

def list_structs(project_path: Path) -> StructWalkerOutput:
    """Walk all `<project>/src/**.rs`, return one StructEvidence per
    `struct` definition.
    """
    raw = _run_subcommand("structs", project_path)
    items = [
        StructEvidence(
            name           = it["name"],
            file           = it["file"],
            line           = it["line"],
            fields         = [FieldEvidence(**f) for f in it["fields"]],
            repr_attribute = it.get("repr_attribute"),
            ffi_boundary   = bool(it.get("ffi_boundary", False)),
            is_pub         = bool(it.get("is_pub", False)),
            derives        = list(it.get("derives") or []),
        )
        for it in raw.get("items", [])
    ]
    out = StructWalkerOutput(
        project_root       = raw["project_root"],
        files_scanned      = raw["files_scanned"],
        files_parse_failed = list(raw.get("files_parse_failed") or []),
        items              = items,
    )
    logger.info(f"[syn_walker] structs: {len(items)} struct(s) in "
                f"{out.files_scanned} file(s)")
    return out


def list_allocs(project_path: Path) -> AllocWalkerOutput:
    """Walk all `<project>/src/**.rs`, return one AllocSiteEvidence per
    Vec/Box/String/vec! call site.
    """
    raw = _run_subcommand("allocs", project_path)
    items = [
        AllocSiteEvidence(
            file                = it["file"],
            line                = it["line"],
            kind                = it["kind"],
            size_arg            = it["size_arg"],
            containing_fn       = it.get("containing_fn", "<top-level>"),
            alloc_freq_estimate = int(it.get("alloc_freq_estimate", 0)),
        )
        for it in raw.get("items", [])
    ]
    out = AllocWalkerOutput(
        project_root       = raw["project_root"],
        files_scanned      = raw["files_scanned"],
        files_parse_failed = list(raw.get("files_parse_failed") or []),
        items              = items,
    )
    logger.info(f"[syn_walker] allocs: {len(items)} alloc-site(s) in "
                f"{out.files_scanned} file(s)")
    return out


def list_loop_invariant_conds(project_path: Path) -> LoopInvariantOutput:
    """Walk all `<project>/src/**.rs`, return one LoopInvariantCond per
    `if`/`match` whose condition reads only loop-invariant identifiers.

    Used by rule triggers that need source-level signals beyond what
    HotspotProfile (perf + compiler-only) can carry."""
    raw = _run_subcommand("loop-invariant-conds", project_path)
    items = [
        LoopInvariantCond(
            file          = it["file"],
            line          = it["line"],
            loop_kind     = it["loop_kind"],
            loop_line     = it["loop_line"],
            containing_fn = it.get("containing_fn", "<top-level>"),
            cond_snippet  = it.get("cond_snippet", ""),
            reads         = list(it.get("reads") or []),
        )
        for it in raw.get("items", [])
    ]
    out = LoopInvariantOutput(
        project_root       = raw["project_root"],
        files_scanned      = raw["files_scanned"],
        files_parse_failed = list(raw.get("files_parse_failed") or []),
        items              = items,
    )
    logger.info(f"[syn_walker] loop-invariant-conds: {len(items)} candidate(s) "
                f"in {out.files_scanned} file(s)")
    return out


def filter_loop_invariant_for_fn(items: list[LoopInvariantCond],
                                  fn_name: str) -> list[LoopInvariantCond]:
    """Filter LoopInvariantCond list to one containing fn (bare name).

    `fn_name` can be the perf symbol; we match its last `::` segment
    against `containing_fn`."""
    bare = fn_name.rsplit("::", 1)[-1] if "::" in fn_name else fn_name
    return [it for it in items if it.containing_fn == bare]


# ---------------------------------------------------------------------------
# CLI for ad-hoc testing
# ---------------------------------------------------------------------------

def _cli() -> None:
    import argparse, sys
    p = argparse.ArgumentParser(prog="syn_walker (python wrapper)")
    sub = p.add_subparsers(dest="cmd", required=True)
    for c in ("structs", "allocs", "loop-invariant-conds"):
        s = sub.add_parser(c)
        s.add_argument("--project", required=True)
        s.add_argument("--limit", type=int, default=20,
                       help="show first N items (0 = all)")
        if c == "loop-invariant-conds":
            s.add_argument("--fn", default=None,
                           help="filter to one containing fn (bare name match)")
    args = p.parse_args()
    logging.basicConfig(level=logging.INFO,
                        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
                        datefmt="%H:%M:%S")
    if args.cmd == "structs":
        out = list_structs(Path(args.project))
    elif args.cmd == "allocs":
        out = list_allocs(Path(args.project))
    else:
        out = list_loop_invariant_conds(Path(args.project))
        if getattr(args, "fn", None):
            from dataclasses import replace
            kept = filter_loop_invariant_for_fn(out.items, args.fn)
            out = replace(out, items=kept)
    items = out.items if args.limit == 0 else out.items[:args.limit]
    for it in items:
        print(json.dumps(it.__dict__, default=lambda o: o.__dict__, ensure_ascii=False))
    print(f"\n# total: {len(out.items)} item(s) in {out.files_scanned} file(s)")
    if out.files_parse_failed:
        print(f"# parse-failed: {len(out.files_parse_failed)} file(s)")
    sys.exit(0)


if __name__ == "__main__":
    _cli()
