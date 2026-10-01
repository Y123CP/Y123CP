"""Stage A Phase 2 FATR — call graph + bottom-up topo order.

Why this module: FATR lifts fn signatures bottom-up — every callee must be
lifted (and its callsites already wrapped) before any caller is touched, so
that a caller's lift only sees post-lift callsite shapes. This requires a
directed call graph plus Tarjan SCC condensation (recursion / mutual
recursion) plus reverse topo on the condensed DAG.

The call graph is reconstructed from **Rust source via tree-sitter**, not
from PA_func.json — the PA_func callsite records contain only LLVM IR text
+ source line/col, not the enclosing caller fn name. Scanning the Rust CST
gives us caller→callee edges directly and at the exact byte ranges we will
later edit, which is also why FATR uses tree-sitter for the apply phase.

External contract:
    fn_order = build_topo(project_dir, pa_func_fn_names)

`pa_func_fn_names` constrains the graph to fns the SA engine analyzed —
otherwise we'd include c2rust-only helpers, libc shims, harness fns, etc.,
which inflate the topo and waste time.
"""

from __future__ import annotations

import logging
from collections import defaultdict
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
class CallEdge:
    """A call_expression site found in the Rust source.

    `caller_fn`     : enclosing fn name (the fn whose body contains the call)
    `callee_fn`     : called fn name (identifier of the call target)
    `file`          : source file containing the call
    `call_range`    : (start_byte, end_byte) of the call_expression node
    """
    caller_fn: str
    callee_fn: str
    file: Path
    call_range: tuple[int, int]


@dataclass
class CallGraph:
    """Adjacency built by `build_call_graph`.

    `nodes`         : all fn names that appear as either caller or callee
                       (intersected with `restrict_to` if provided)
    `callees`       : caller_fn → set(callee_fn)
    `callers`       : callee_fn → set(caller_fn)
    `edges`         : flat list of CallEdge (for downstream callsite ripple)
    """
    nodes: set[str] = field(default_factory=set)
    callees: dict[str, set[str]] = field(default_factory=lambda: defaultdict(set))
    callers: dict[str, set[str]] = field(default_factory=lambda: defaultdict(set))
    edges: list[CallEdge] = field(default_factory=list)


# ─────────────────────────────────────────────────────────────────
# Tree-sitter helpers
# ─────────────────────────────────────────────────────────────────

def _txt(src: bytes, n: Node) -> str:
    return src[n.start_byte:n.end_byte].decode("utf-8", errors="replace")


def _fn_name(src: bytes, fn_node: Node) -> str | None:
    """Get the identifier text of a `function_item` node."""
    for c in fn_node.children:
        if c.type == "identifier":
            return _txt(src, c)
    return None


def _call_callee_name(src: bytes, call: Node) -> str | None:
    """Extract the callee identifier from a `call_expression`.

    Handles:
      foo(x)          → "foo"
      crate::m::foo(x)→ "foo"
      (*p)(x)         → None  (indirect call via fn ptr; not a static edge)
      x.method(y)     → None  (method call has node type field_expression
                                inside; we skip — irrelevant to FATR)
    """
    fn_node = call.child_by_field_name("function")
    if fn_node is None:
        return None
    if fn_node.type == "identifier":
        return _txt(src, fn_node)
    if fn_node.type == "scoped_identifier":
        ident = fn_node.child_by_field_name("name")
        if ident is not None and ident.type == "identifier":
            return _txt(src, ident)
    return None


def _walk_calls_in_fn(src: bytes, fn_node: Node, fn_name: str,
                      file_path: Path, out: list[CallEdge]) -> None:
    """DFS the fn body, emit one CallEdge per call_expression we resolve."""
    stack: list[Node] = list(fn_node.children)
    while stack:
        n = stack.pop()
        if n.type == "function_item":
            # Nested fn: skip — its call edges belong to its own owner.
            continue
        if n.type == "call_expression":
            callee = _call_callee_name(src, n)
            if callee is not None:
                out.append(CallEdge(
                    caller_fn=fn_name,
                    callee_fn=callee,
                    file=file_path,
                    call_range=(n.start_byte, n.end_byte),
                ))
        stack.extend(n.children)


# ─────────────────────────────────────────────────────────────────
# Public: build the call graph
# ─────────────────────────────────────────────────────────────────

def build_call_graph(crate_dir: Path,
                     restrict_to: set[str] | None = None) -> CallGraph:
    """Scan all .rs files under `crate_dir` and build the call graph.

    If `restrict_to` is given (e.g. fns in PA_func.json), the graph is
    filtered to edges where BOTH endpoints are in the set. This is the
    paper-grade default — FATR only lifts PA_func-analyzed fns, and
    edges to unanalyzed callees would never be traversed anyway.
    """
    cg = CallGraph()
    edges: list[CallEdge] = []

    for f in sorted(crate_dir.rglob("*.rs")):
        if "target" in f.parts:
            continue
        src = f.read_bytes()
        tree = _PARSER.parse(src)
        for top in tree.root_node.children:
            if top.type != "function_item":
                continue
            fname = _fn_name(src, top)
            if fname is None:
                continue
            _walk_calls_in_fn(src, top, fname, f, edges)

    for e in edges:
        if restrict_to is not None:
            if e.caller_fn not in restrict_to or e.callee_fn not in restrict_to:
                continue
        cg.nodes.add(e.caller_fn)
        cg.nodes.add(e.callee_fn)
        cg.callees[e.caller_fn].add(e.callee_fn)
        cg.callers[e.callee_fn].add(e.caller_fn)
        cg.edges.append(e)

    # Ensure isolated fns (no in/out edges but in restrict_to) still appear.
    if restrict_to is not None:
        for n in restrict_to:
            cg.nodes.add(n)

    logger.info(
        "[call_graph] %d fns, %d edges (restrict_to=%s)",
        len(cg.nodes), len(cg.edges),
        "yes" if restrict_to is not None else "no",
    )
    return cg


# ─────────────────────────────────────────────────────────────────
# Tarjan SCC condensation
# ─────────────────────────────────────────────────────────────────

@dataclass
class SCCResult:
    """Strongly connected components of the call graph.

    `sccs`          : list of SCCs (each is a list of fn names). Order =
                       Tarjan emit order = reverse topo of the condensed
                       DAG (leaves first, roots last) — i.e. bottom-up.
    `scc_of`        : fn_name → SCC index
    """
    sccs: list[list[str]]
    scc_of: dict[str, int]


def tarjan_scc(cg: CallGraph) -> SCCResult:
    """Iterative Tarjan SCC. O(V + E)."""
    index = 0
    indices: dict[str, int] = {}
    lowlink: dict[str, int] = {}
    on_stack: set[str] = set()
    stack: list[str] = []
    sccs: list[list[str]] = []

    # Iterative driver to avoid Python recursion limit on large crates.
    sys_stack: list[tuple[str, iter]] = []

    nodes_sorted = sorted(cg.nodes)  # deterministic order

    for start in nodes_sorted:
        if start in indices:
            continue
        sys_stack.append((start, iter(sorted(cg.callees.get(start, set())))))
        indices[start] = index
        lowlink[start] = index
        index += 1
        stack.append(start)
        on_stack.add(start)

        while sys_stack:
            v, it = sys_stack[-1]
            advanced = False
            for w in it:
                if w not in indices:
                    indices[w] = index
                    lowlink[w] = index
                    index += 1
                    stack.append(w)
                    on_stack.add(w)
                    sys_stack.append((w, iter(sorted(cg.callees.get(w, set())))))
                    advanced = True
                    break
                if w in on_stack:
                    lowlink[v] = min(lowlink[v], indices[w])
            if advanced:
                continue
            # Finished v — pop SCC if root.
            if lowlink[v] == indices[v]:
                comp: list[str] = []
                while True:
                    w = stack.pop()
                    on_stack.discard(w)
                    comp.append(w)
                    if w == v:
                        break
                sccs.append(sorted(comp))
            sys_stack.pop()
            if sys_stack:
                pv, _ = sys_stack[-1]
                lowlink[pv] = min(lowlink[pv], lowlink[v])

    scc_of = {fn: i for i, comp in enumerate(sccs) for fn in comp}
    logger.info(
        "[call_graph] tarjan: %d SCCs (max size = %d)",
        len(sccs),
        max((len(c) for c in sccs), default=0),
    )
    return SCCResult(sccs=sccs, scc_of=scc_of)


# ─────────────────────────────────────────────────────────────────
# Public: bottom-up topo order
# ─────────────────────────────────────────────────────────────────

def bottom_up_topo(cg: CallGraph, scc: SCCResult) -> list[str]:
    """Flatten SCCs in bottom-up order (leaves of the DAG first).

    Tarjan emits SCCs in reverse-topo of the condensed DAG already, so
    `scc.sccs` is the right order out of the box. We just flatten with
    a deterministic within-SCC order (alphabetical).

    SCCs of size > 1 (mutual recursion) emit ALL members consecutively;
    the FATR driver currently treats them as a frozen group — see
    §5.2 in stage_a_fatr_design.md.
    """
    out: list[str] = []
    for comp in scc.sccs:
        out.extend(sorted(comp))
    return out


def split_recursive_groups(scc: SCCResult) -> tuple[list[str], list[list[str]]]:
    """Convenience splitter for the driver:
        single_fns   = fns whose SCC has size 1 and no self-loop
        recursive    = SCCs with size > 1 OR size 1 with self-edge
    Returns (single_fns_in_bottom_up_order, recursive_groups).

    NOTE: this signature doesn't carry self-loop info — the driver should
    detect self-loops via cg.callees[fn] containing fn. Kept simple here.
    """
    singles: list[str] = []
    recursive: list[list[str]] = []
    for comp in scc.sccs:
        if len(comp) == 1:
            singles.append(comp[0])
        else:
            recursive.append(sorted(comp))
    return singles, recursive


# ─────────────────────────────────────────────────────────────────
# One-shot helper used by the driver
# ─────────────────────────────────────────────────────────────────

def build_topo(crate_dir: Path,
               restrict_to: set[str] | None = None
               ) -> tuple[CallGraph, SCCResult, list[str]]:
    """End-to-end: scan source → build cg → tarjan → bottom-up order."""
    cg = build_call_graph(crate_dir, restrict_to)
    scc = tarjan_scc(cg)
    order = bottom_up_topo(cg, scc)
    return cg, scc, order
