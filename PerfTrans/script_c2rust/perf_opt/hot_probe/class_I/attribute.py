""                                                                      
                                               

                                                                         
                                                          

                                                           
                                                                        
                                                           
                                                            
                                                                        
                                                                     
                                                     

                                                                    
                                                                   
                                                                       
       

                                                                        
                                                                         
                                                               
                                             

                                                                
                                                                       
                                                                       
   

from __future__ import annotations

import logging
from dataclasses import dataclass, field

from perf_opt.hot_probe.class_I.ir_parse import (
    DIFile, DILocation, DISubprogram, MetaTables, is_stdlib_file,
)

logger = logging.getLogger("hot_probe.class_I.attribute")


UNRESOLVED = "__no_dbg__"           # sentinel bucket for sites without !dbg
STDLIB_ONLY = "__stdlib_only__"     # site's entire inlinedAt chain is stdlib


@dataclass(frozen=True)
class Site:
    """One matched IR call site awaiting attribution.

    Rule-agnostic — `rule_id` is any string the caller uses to key their
    per-rule aggregation (`"C1"`, `"C2"`, etc.); `attribute` treats it as
    opaque.
    """
    rule_id: str           # caller-defined bucket id (e.g. "C1" | "C2" | "C3")
    dbg_id: int            # `!<N>` DILocation id (0 if the instruction has no !dbg)
    callee: str            # mangled callee symbol at the site
    define_mangled: str    # enclosing `define`'s mangled name — RQ3
                                                               
    detail: str = ""       # optional per-rule payload the callee name cannot
                           # carry (C12 puts the constant byte count here).
                           # Defaulted, so existing producers are unchanged.


@dataclass(frozen=True)
class Attribution:
    """One site attributed to two frames on the inlinedAt chain."""
    rule_id: str
    define_fn: str          # innermost non-stdlib DISubprogram name (or sentinel)
    host_fn: str            # outermost frame's DISubprogram name (or sentinel)
    define_file: str | None
    host_file: str | None


def _walk_chain(dbg_id: int, tables: MetaTables) -> list[tuple[DILocation, DISubprogram | None, DIFile | None]]:
    """Follow DILocation.inlined_at recursively, yielding (loc, scope-fn, file)
    per frame. Stops on missing DILocation (broken chain) or missing scope
    subprogram (DILexicalBlock chain exhausted without landing on a subprogram)."""
    frames: list[tuple[DILocation, DISubprogram | None, DIFile | None]] = []
    cur = dbg_id
    guard = 0
    while cur:
        loc = tables.locations.get(cur)
        if loc is None:
            break
        sp = tables.scope_to_subprogram(loc.scope)
        fi = tables.files.get(sp.file_id) if (sp and sp.file_id) else None
        frames.append((loc, sp, fi))
        cur = loc.inlined_at or 0
        guard += 1
        if guard > 64:
            logger.warning("[attribute] inlinedAt chain > 64 frames at !%d",
                           dbg_id)
            break
    return frames


def _lookup_by_linkage(tables: MetaTables, mangled: str) -> DISubprogram | None:
    """Reverse-lookup DISubprogram by its `linkageName:` (the mangled symbol
    on a `define` line). O(N) — called only for no-dbg fallback so not on
    the hot path."""
    if not mangled:
        return None
    for sp in tables.subprograms.values():
        if sp.linkage_name == mangled:
            return sp
    return None


def attribute_site(site: Site, tables: MetaTables) -> Attribution:
    """Map a single `Site` → `Attribution`. Handles no-dbg / stdlib-only /
    normal cases uniformly."""
    if site.dbg_id == 0:
                                                                       
        if not site.define_mangled:
            # Site is outside any `define` block (e.g. inside a global
            # initializer / const eval trampoline). Nothing to attribute.
            return Attribution(rule_id=site.rule_id,
                               define_fn=UNRESOLVED, host_fn=UNRESOLVED,
                               define_file=None, host_file=None)
        sp = _lookup_by_linkage(tables, site.define_mangled)
        if sp is None:
            # `define` exists but has no DISubprogram in the IR — this is
            # what happens for cargo build-deps compiled WITHOUT debuginfo
            # (miniz_oxide, addr2line, gimli, hashbrown, …). Not project
            # code; bucket to stdlib_only.
            return Attribution(rule_id=site.rule_id,
                               define_fn=STDLIB_ONLY,
                               host_fn=site.define_mangled,
                               define_file=None,
                               host_file=None)
        fi = tables.files.get(sp.file_id) if sp.file_id else None
        if is_stdlib_file(fi):
            # define is itself stdlib source — bucket to stdlib_only.
            return Attribution(rule_id=site.rule_id,
                               define_fn=STDLIB_ONLY, host_fn=sp.name,
                               define_file=None,
                               host_file=fi.path if fi else None)
        # Project-side define: attribute self.
        return Attribution(rule_id=site.rule_id,
                           define_fn=sp.name, host_fn=sp.name,
                           define_file=fi.path if fi else None,
                           host_file=fi.path if fi else None)

    frames = _walk_chain(site.dbg_id, tables)
    if not frames:
        return Attribution(rule_id=site.rule_id,
                           define_fn=UNRESOLVED, host_fn=UNRESOLVED,
                           define_file=None, host_file=None)

    # Define fn = innermost non-stdlib scope. Start from index 0 (deepest),
    # walk up until a scope whose file is NOT stdlib.
    define_sp: DISubprogram | None = None
    define_fi: DIFile | None = None
    for _loc, sp, fi in frames:
        if sp is None:
            continue
        if not is_stdlib_file(fi):
            define_sp, define_fi = sp, fi
            break

    # Host fn = OUTERMOST frame's scope (whether stdlib or not — perf
    # sampling attributes cycles to it regardless).
    host_loc, host_sp, host_fi = frames[-1]

    if define_sp is None:
        # Every frame in the chain is stdlib — this site is inside a
        # library-only inlining path.
        return Attribution(rule_id=site.rule_id,
                           define_fn=STDLIB_ONLY,
                           host_fn=host_sp.name if host_sp else UNRESOLVED,
                           define_file=None,
                           host_file=host_fi.path if host_fi else None)

    return Attribution(
        rule_id=site.rule_id,
        define_fn=define_sp.name,
        host_fn=host_sp.name if host_sp else define_sp.name,
        define_file=define_fi.path if define_fi else None,
        host_file=host_fi.path if host_fi else None,
    )


@dataclass
class AggregatedHits:
    """Aggregated attribution output for a whole IR file.

    Both dicts key on **define_fn** name (the source fn where the construct
    is written) — hits_by_fn is the flat total per rule (self + inherited),
    inheritance is the RQ3-style structured breakdown per host fn.
    """
    # Flat: {source_fn: {rule_id: total}} — self + inherited combined.
    hits_by_fn: dict[str, dict[str, int]] = field(default_factory=dict)
    # RQ3-style: host fn → source fn → rule → count.
    inheritance: dict[str, dict[str, dict[str, int]]] = field(default_factory=dict)
    unresolved: dict[str, int] = field(default_factory=dict)    # rule → count
    stdlib_only: dict[str, int] = field(default_factory=dict)

    def add(self, attr: Attribution) -> None:
        rid = attr.rule_id
        if attr.define_fn == UNRESOLVED:
            self.unresolved[rid] = self.unresolved.get(rid, 0) + 1
            return
        if attr.define_fn == STDLIB_ONLY:
            self.stdlib_only[rid] = self.stdlib_only.get(rid, 0) + 1
            return
        # flat
        self.hits_by_fn.setdefault(attr.define_fn, {})[rid] = \
            self.hits_by_fn.get(attr.define_fn, {}).get(rid, 0) + 1
        # structured
        host = attr.host_fn
        self.inheritance.setdefault(host, {}).setdefault(attr.define_fn, {})[rid] = \
            self.inheritance.get(host, {}).get(attr.define_fn, {}).get(rid, 0) + 1


def aggregate(sites: list[Site], tables: MetaTables) -> AggregatedHits:
    out = AggregatedHits()
    for s in sites:
        out.add(attribute_site(s, tables))
    logger.info("[attribute] aggregated %d site(s): %d fn(s) hit, "
                "unresolved=%s, stdlib_only=%s",
                len(sites), len(out.hits_by_fn),
                out.unresolved, out.stdlib_only)
    return out
