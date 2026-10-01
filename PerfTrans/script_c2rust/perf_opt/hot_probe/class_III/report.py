"""Format Class III scan results as a human-readable markdown report.

For end users: 'given a rust project, which fns are optimization candidates
and where'. Complements the machine-readable `class_III_hits.json` (audit
artifact).

Public API:
  format_report(scan_result, project_name="...", top_n=15) → str (markdown)
"""
from __future__ import annotations

from perf_opt.hot_probe.class_III.scan import ScanResult


def _top_fns(hits: dict, top_n: int) -> list[tuple[str, list]]:
    return sorted(hits.items(), key=lambda kv: len(kv[1]), reverse=True)[:top_n]


def _sample_files(sites: list, k: int = 2) -> str:
    """Return a short `file:line` sample of the first k sites."""
    xs = [f"`{s.file}:{s.line}`" for s in sites[:k]]
    return " · ".join(xs)


def format_report(
    result: ScanResult,
    project_name: str = "<project>",
    top_n: int = 15,
) -> str:
    """Build a markdown candidate report from a `ScanResult`.

    Args:
        result:       ScanResult from `scan(crate, ...)`
        project_name: short label for the header (e.g. "miniz")
        top_n:        how many candidate fns to list per rule

    Returns:
        Markdown string (ends with newline). Written to stdout or file.
    """
    L: list[str] = []
    add = L.append

    add(f"# Class III Optimization Candidates — {project_name}\n")
    add("**Basis** — pure source-scan; no profiler / no workload data used.")
    add("Class III observes the *translated-product layer* (Rust CST); every")
    add("candidate below is a rewrite site derivable from the c2rust output")
    add("source alone. Realized speedup per candidate is decided downstream")
    add("by W1/W2 gates, not by the detector.\n")

    add("## Scan basis\n")
    add("| | |")
    add("|---|---:|")
    add(f"| `fns_seen` (candidate pool size) | {result.n_fns_seen} |")
    add(f"| `fns_analyzed` (actually scanned) | {result.n_fns_analyzed} |")
    add(f"| `libc_extern_set` (project's libc surface) | {len(result.libc_extern_set)} decls |")
    add(f"| `custom_mem_op_fns` (III③ body-pattern detected) | {len(result.custom_mem_op_fns)} fns |")
    add("")

    # ── Per-rule summary ───────────────────────────────────────────────────
    n1 = len(result.iii1_hits)
    s1 = sum(len(v) for v in result.iii1_hits.values())
    n2 = len(result.iii2_hits)
    s2 = sum(len(v) for v in result.iii2_hits.values())
    n3 = len(result.iii3_hits)
    s3 = sum(len(v) for v in result.iii3_hits.values())
    n4 = len(result.iii4_hits)
    s4 = sum(len(v) for v in result.iii4_hits.values())

    add("## Candidate summary — how many fns each rule flagged\n")
    add("| Rule | Card | Candidate fns | Total sites |")
    add("|---|---|---:|---:|")
    add(f"| **III①** callback → generic monomorphization | [III1_callback_monomorph.md](../../agent_perf_opt/Optimization_Card/III1_callback_monomorph.md) | {n1} | {s1} |")
    add(f"| **III②** manual heap → RAII container | [III2_manual_heap_to_raii.md](../../agent_perf_opt/Optimization_Card/III2_manual_heap_to_raii.md) | {n2} | {s2} |")
    add(f"| **III③** manual mem-op → slice ops | [III3_mem_ops_to_slice.md](../../agent_perf_opt/Optimization_Card/III3_mem_ops_to_slice.md) | {n3} | {s3} |")
    add(f"| **III④** raw-ptr cursor (source-cause; rewrite → C1/C3/D1) | [III4_raw_ptr_cursor.md](../../agent_perf_opt/Optimization_Card/III4_raw_ptr_cursor.md) | {n4} | {s4} |")
    add("")

    # ── III① details ───────────────────────────────────────────────────────
    if result.iii1_hits:
        add(f"## III① Callback Monomorphization — top {top_n} candidates\n")
        add("Indirect calls through `Option<extern \"C\" fn>` fn ptr — each site")
        add("compiles to an indirect call that blocks inlining + cross-fn opt.")
        add("Rewrite direction: generic `Fn`/`FnMut` monomorphization.\n")
        add("| # | Candidate fn | Sites | Marker rate | Sample |")
        add("|---:|---|---:|---:|---|")
        for i, (fn, sites) in enumerate(_top_fns(result.iii1_hits, top_n), 1):
            markers = sum(1 for s in sites if getattr(s, "marker_seen", False))
            rate = f"{markers}/{len(sites)}"
            add(f"| {i} | `{fn}` | {len(sites)} | {rate} | {_sample_files(sites)} |")
        add("")
        add("Marker rate = how many sites carry the c2rust literal")
        add("`expect(\"non-null function pointer\")`; 100% means all hits are")
        add("standard c2rust callback dispatch (no false positives from")
        add("look-alike code shapes).\n")

    # ── III② details ───────────────────────────────────────────────────────
    if result.iii2_hits:
        add(f"## III② Manual Heap → RAII Container — top {top_n} candidates\n")
        add("`malloc`/`free`/`realloc`/`calloc` (+ aligned variants) call sites.")
        add("Manual C-style heap management preserved by c2rust — violates")
        add("Rust ownership + typed-container principle. Rewrite direction:")
        add("`Vec<T>` / `Box<T>` (RAII); scratch buffers → `vec![0; n]`.\n")
        add("| # | Candidate fn | Sites | Callees | Sample |")
        add("|---:|---|---:|---|---|")
        for i, (fn, sites) in enumerate(_top_fns(result.iii2_hits, top_n), 1):
            callees = sorted({s.callee_name for s in sites})
            add(f"| {i} | `{fn}` | {len(sites)} | `{', '.join(callees)}` | {_sample_files(sites)} |")
        add("")

    # ── III③ details ───────────────────────────────────────────────────────
    if result.iii3_hits:
        add(f"## III③ Manual mem-op → slice ops — top {top_n} candidates\n")
        add("`memcpy`/`memset`/`strlen`/... libc mem-str-libm calls + project-")
        add("defined byte-copy/mem wrapper fns (`copy_be*`, `libzahl_mem*` 类)")
        add("detected by body-pattern (short body + repeated `*p.offset(i)`)")
        add("Rewrite direction: `copy_from_slice` / `fill` / `to_be_bytes`.\n")
        add("| # | Candidate fn | Sites | Callee kinds |")
        add("|---:|---|---:|---|")
        for i, (fn, sites) in enumerate(_top_fns(result.iii3_hits, top_n), 1):
            callees = sorted({s.callee_name for s in sites})[:6]
            more = "" if len(callees) < 6 else "…"
            add(f"| {i} | `{fn}` | {len(sites)} | `{', '.join(callees)}`{more} |")
        add("")
        if result.custom_mem_op_fns:
            add(f"### Custom mem-op fns detected by body-pattern (top {top_n})\n")
            add("These project-defined fns are treated as mem-op sites when")
            add("called from any hot fn (III③ union member).\n")
            add("| # | Fn name |")
            add("|---:|---|")
            for i, name in enumerate(sorted(result.custom_mem_op_fns)[:top_n], 1):
                add(f"| {i} | `{name}` |")
            add("")

    # ── III④ details ───────────────────────────────────────────────────────
    if result.iii4_hits:
        add(f"## III④ Raw-pointer Cursor — top {top_n} candidates\n")
        add("`*p.offset(i)` / `*p.add(i)` / post-increment cursor patterns —")
        add("**source-cause of C1(bounds check) + C3(alias gap) + D1(vec loss)**.")
        add("III④ has **no rewrite direction of its own**; the fix is applied")
        add("through C1/C3/D1 cards. This report exists to attribute the source")
        add("病因 to specific fns for LLM prompt context.\n")
        add("| # | Candidate fn | Sites | Pattern kinds | Sample |")
        add("|---:|---|---:|---|---|")
        for i, (fn, sites) in enumerate(_top_fns(result.iii4_hits, top_n), 1):
            kinds = sorted({s.pattern_kind for s in sites})[:4]
            more = "" if len(kinds) < 4 else "…"
            add(f"| {i} | `{fn}` | {len(sites)} | `{', '.join(kinds)}`{more} | {_sample_files(sites)} |")
        add("")

    # ── Audit buckets ──────────────────────────────────────────────────────
    add("## Audit buckets (detection quality signals)\n")
    add("Small = detection is confident. Large may indicate SPEC gaps.\n")
    add("| Bucket | Count | Meaning |")
    add("|---|---:|---|")
    add(f"| iii1_deep_receiver | {len(result.iii1_deep_receiver)} | receiver chain >3 field-access → conservatively skipped |")
    add(f"| iii1_scrutinee_unresolved | {len(result.iii1_scrutinee_unresolved)} | `match` scrutinee type not derivable (SPEC §1.3) |")
    add(f"| iii1_c1_boundary_unresolved | {len(result.iii1_c1_boundary_unresolved)} | III① vs C1 boundary case beyond same-block scope |")
    add(f"| iii2_ambiguous_callees | {len(result.iii2_ambiguous_callees)} | callee name shared by alloc extern + project fn (SPEC §2.4) |")
    add(f"| iii3_ambiguous_callees | {len(result.iii3_ambiguous_callees)} | callee name shared by libc extern + project fn (SPEC §3.2) |")
    add("")

    # ── Next-step guidance ─────────────────────────────────────────────────
    add("## What to do with this report\n")
    add("1. Each candidate fn ↔ its rule's Optimization_Card. LLM prompt =")
    add("   `EvidencePack(fn, rule, sites) + card` → rewritten Rust source.")
    add("2. Driver pipeline: `(fn, rule) → EvidencePack → card → LLM → apply`")
    add("   → **W1** (correctness) + **W2** (wall-clock) double gate. Only")
    add("   commits that pass BOTH stick.")
    add("3. **III④ 特殊**:no rewrite of its own — its hits attribute the source")
    add("   病因 for C1/C3/D1 rewrite (cluster.md §III④ 回填不重复计门槛)。")
    add("4. No candidate is pre-filtered by profiler here — that is the")
    add("   **methodology contract of Class III**. If you want profiler-driven")
    add("   pruning, pass `hot_fns=<set>` to `scan()` at call site.")
    return "\n".join(L) + "\n"
