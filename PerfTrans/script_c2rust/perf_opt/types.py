"""perf-opt data types — slimmed to SA-facts only after old-Stage-B cleanup.

Originally this module was the single source of truth for every datatype
used by the old Stage B (dispatch / orchestrator / rules / patcher / ripple /
verify / state / snippet). Those modules were removed in the 2026-06-01
cleanup; `perf_opt/sa_iface.py` is now the only remaining consumer, and the
only types it needs are the four that describe pointer-analysis facts.

If a new Stage B re-introduces orchestration types (Diff / AtomResult /
GuardsResult / etc.), prefer adding them to the new module that owns the
loop — keep this file focused on SA Protocol / facts only.

Naming convention preserved:
  - `Site = "<fn_id>"` or `"<fn_id>:<callsite_idx>"` /
    `"<fn_id>:loop:<idx>"` / `"<fn_id>:field:<name>"`
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Protocol


# ---------------------------------------------------------------------------
# Site — stable across commits as long as the containing fn isn't fully
# rewritten. String, not class, so it serializes transparently.
# ---------------------------------------------------------------------------

Site = str


def make_site(fn_id: str, kind: str = "", idx: int | str | None = None) -> Site:
    """Build a Site string. `kind` = "" | "loop" | "field" | "call"."""
    if not kind and idx is None:
        return fn_id
    if kind and idx is not None:
        return f"{fn_id}:{kind}:{idx}"
    if idx is not None:
        return f"{fn_id}:{idx}"
    return f"{fn_id}:{kind}"


def site_fn(site: Site) -> str:
    """Extract the fn_id prefix of a Site."""
    return site.split(":", 1)[0]


# ---------------------------------------------------------------------------
# SA-engine facts — Protocols implemented by sa_iface's Stub / Real classes
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class CallerInfo:
    """One caller of a fn — from sa_engine PA_func.callers()."""
    caller_fn: str
    callsite:  Site
    file:      Path
    line:      int
    snippet:   str           # ±3 lines around the callsite, for ripple/adapter context


@dataclass(frozen=True)
class FieldInfo:
    """One user of a struct field — from sa_engine PA_struct.field_users()."""
    struct_name: str
    field_name:  str
    user_fn:     str
    file:        Path
    line:        int


class PAFunc(Protocol):
    """Function-level pointer analysis facts (from sa_engine PA_func)."""

    def callers(self, fn_id: str) -> list[CallerInfo]: ...
    """Every caller of `fn_id` in the project."""

    def alias_set(self, ptr_id: str) -> list[str]:  ...
    """Other ptr identities that may alias `ptr_id`."""

    def has_cross_fn_rw(self, alias_set: list[str]) -> bool: ...
    """Is any ptr in the alias set written-then-read across fn boundaries?"""

    def callsites_in(self, fn_id: str) -> list[Site]: ...
    """All callsites inside `fn_id`'s body, in AST-traversal order."""

    def get_signature(self, fn_id: str) -> str: ...
    """Current source signature of `fn_id` (text, not parsed AST)."""

    def is_static_unique_target(self, callsite: Site) -> tuple[bool, str | None]: ...
    """For indirect callsites: can def-use prove a single static target?
    Returns (True, target_fn_id) or (False, None)."""


class PAStruct(Protocol):
    """Struct-level pointer analysis facts (from sa_engine PA_struct)."""

    def field_users(self, struct: str, field: str) -> list[FieldInfo]: ...
    """Every site that reads/writes <struct>.<field>."""


@dataclass
class SAEngineFacts:
    """Bundle of pointer-analysis facts for the current commit."""
    pa_func:   PAFunc
    pa_struct: PAStruct


__all__ = [
    "Site", "make_site", "site_fn",
    "CallerInfo", "FieldInfo",
    "PAFunc", "PAStruct", "SAEngineFacts",
]
