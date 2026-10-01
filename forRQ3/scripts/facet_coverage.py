"""Which of the nine RQ3 rules actually landed, per Evaluation project.

RQ3 states nine rules; the pipeline implements eighteen cards. That is not a
mismatch but two levels of one hierarchy — rule (abstract) -> facet (concrete
source shape) -> instance. A card is a facet. Several cards can serve one rule
because the SHAPES differ while the obscured property and the optimization
direction do not; `RULE_OF` below is that mapping, and every card belongs to
exactly one rule.

Read-only. Reads `<project>/3_perf_opt/rewrites.log` (the run evaluation.tex measured), one JSON object per
attempt, and reports three states per (rule, project):

    n   the rule landed n times — passed W1, W2 and reached git
    o   a card for the rule fired, but no attempt carrying it was committed
    .   no card for the rule fired on this project at all

The gap between `o` and `n` is the pipeline's own gates rejecting rewrites; the
gap between `.` and `o` is detector coverage. Reporting only "the rule is
implemented" would hide both.

Usage:
    python -m tools.facet_coverage                 # console table
    python -m tools.facet_coverage --tex out.tex   # LaTeX for the paper
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from datetime import date, datetime
from pathlib import Path
from typing import Optional

# Pipeline card -> RQ3 rule.
#
# Settled 2026-08-17 for the first sixteen cards; C11 was placed 2026-09-01 and
# C9/C10/C12 are proposed, not yet confirmed by the author:
#
#   C11  Invariant Dispatch Specialization. The translator lowers C's gotos into
#        `loop { match current_block { .. } }`; a hot arm ends by re-selecting
#        its own label, so its dispatch outcome is FIXED for as long as the arm
#        runs, yet the loop's back edge re-evaluates it every iteration. Same
#        mechanism as III1 (call target fixed, still dispatched indirectly) and
#        C6 (mode fixed, still branched on) — a dispatch settled before the hot
#        region enters it, executed inside the region anyway.
#   C9   Typed Memory Operations (weaker). c2rust-bitfields emits per-bit helper
#        loops for field access where C uses direct bit arithmetic; that matches
#        "helper loops that repeatedly perform assignments". It could also be
#        argued as residual runtime work.
#   C10  Data-Level Parallelism Exposure. A fixed 8-step subword loop becomes one
#        SWAR word operation — "replace scalar updates with block-wise ones".
#   C12  Typed Memory Operations. The Observed Pattern names `memset(dst, v, n)`
#        outright, and C12 targets exactly its extent mismatch: CAP elements
#        zeroed where n are read.
RULE_OF: dict[str, str] = {
    "C1": "Residual Bounds-Check Elimination",
    "C2": "Range-Proven Conversion Simplification",
    "II_vec": "Vectorization Restoration",
    "II_inl": "Hot-Callee Inlining Restoration",
    "II_iso": "Hot-Callee Inlining Restoration",
    "III①": "Invariant Dispatch Specialization",
    "C6": "Invariant Dispatch Specialization",
    "C11": "Invariant Dispatch Specialization",
    "C7": "Owned Buffer Management",
    "III②": "Owned Buffer Management",
    "III③": "Typed Memory Operations",
    "C9": "Typed Memory Operations",
    "C12": "Typed Memory Operations",
    "C4": "Data-Level Parallelism Exposure",
    "C5": "Data-Level Parallelism Exposure",
    "C8": "Data-Level Parallelism Exposure",
    "C10": "Data-Level Parallelism Exposure",
    "C3": "Memory Property Recovery",
    "III④": "Memory Property Recovery",
    "II_const": "Memory Property Recovery",
}

                                        
                                                   
#
                                                
                                          
                                           
                                                         
                                                                     
                                                                  
                                                                    
CARD_AVAILABLE_FROM: dict[str, datetime] = {
    "II_vec": datetime(2026, 9, 12),
    "II_iso": datetime(2026, 9, 15, 4, 7),
}

GROUPS: list[tuple[str, list[str]]] = [
    ("Residual runtime work", [
        "Residual Bounds-Check Elimination",
        "Range-Proven Conversion Simplification"]),
    ("Missed compiler transformations", [
        "Vectorization Restoration",
        "Hot-Callee Inlining Restoration"]),
    ("Recurring source-level patterns", [
        "Invariant Dispatch Specialization",
        "Owned Buffer Management",
        "Typed Memory Operations",
        "Data-Level Parallelism Exposure",
        "Memory Property Recovery"]),
]

SHORT = {"optipng-0.7.7": "optipng", "libqrencode": "libqrencode",
         "libopenaptx": "libopenaptx", "http-parser": "http-parser"}


def _base(rule: str) -> str:
    """Fold a sub-strategy (`C3.S2`) into its base card (`C3`)."""
    return re.split(r"[.]", str(rule))[0]


def _git_landed(crate: Path) -> Optional[list[set[str]]]:
    ""                                                        
    if not (crate / ".git").exists():
        return None
    out = subprocess.run(["git", "-C", str(crate), "log", "--format=%s"],
                         capture_output=True, text=True)
    if out.returncode != 0:
        return None
    return [{_base(c.strip()) for c in subj.split(":", 1)[1].split(",") if c.strip()}
            for subj in out.stdout.splitlines()
            if subj.startswith("agent changeset ") and ":" in subj]


def read_project(opt_dir: Path) -> Optional[dict]:
    log = opt_dir / "rewrites.log"
    if not log.is_file():
        return None
    fired: set[str] = set()
    landed: dict[str, int] = {}
                                                       
                                                  
                                                                  
    landed_commits: list[set[str]] = []
    for line in log.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        rec = json.loads(line)
        for r in rec.get("fired_rules") or []:
            fired.add(_base(r))
        if rec.get("terminal_status") == "committed":
            cards = {_base(r) for r in rec.get("applied_rules") or []}
            landed_commits.append(cards)
            for b in cards:
                landed[b] = landed.get(b, 0) + 1
                                                      
                                                            
                                                               
                                                          
    git = _git_landed(opt_dir / "crate")
    if git is not None:
        landed_commits = git
        landed = {}
        for cards in git:
            for b in cards:
                landed[b] = landed.get(b, 0) + 1

                                                              
                                                               
                                                        
                                           
    status = opt_dir / "rule_scan_status.json"
    scan_incomplete = None
    if status.is_file():
        try:
            st = json.loads(status.read_text(encoding="utf-8"))
            scan_incomplete = [k for k, v in st.items()
                               if k.startswith("class_") and v == "skipped"]
        except (json.JSONDecodeError, OSError):
            scan_incomplete = ["unknown"]

    return {"fired": fired, "landed": landed, "landed_commits": landed_commits,
            "scan_incomplete": scan_incomplete,
            "date": _run_started(opt_dir)}


def _run_started(opt_dir: Path) -> datetime:
    ""                                                   
    base = opt_dir / "baseline.json"
    src = base if base.is_file() else opt_dir
    return datetime.fromtimestamp(src.stat().st_mtime)


def collect(dataset: Path, subdir: str) -> dict[str, dict]:
    out = {}
    for proj in sorted(p.name for p in dataset.iterdir()
                       if p.is_dir() and not p.name.startswith(("_", "."))):
        got = read_project(dataset / proj / subdir)
        if got:
            out[proj] = got
    return out


def first_seen(projects: dict[str, dict]) -> dict[str, datetime]:
    """Earliest run in which each card fired. Printed for reference only.

    It must NOT decide availability: a card can exist for many runs without
    firing. See CARD_AVAILABLE_FROM.
    """
    seen: dict[str, datetime] = {}
    for v in projects.values():
        for card in v["fired"]:
            d = v["date"]
            if card not in seen or d < seen[card]:
                seen[card] = d
    return seen


def cell(v: dict, cards: list[str], seen=None) -> tuple[str, bool]:
    """(symbol, at_least_one_card_for_this_rule_could_fire_in_this_run).

    Availability comes from CARD_AVAILABLE_FROM, never from firing history.
    Distinguishing "could not fire" from "did not fire" keeps a rule that was
    never offered out of the denominator of one that did not apply.
    """
    n = sum(1 for got in v["landed_commits"] if got & set(cards))
    if n:
        return str(n), True
    if any(c in v["fired"] for c in cards):
        return "o", True
    available = any(c not in CARD_AVAILABLE_FROM or v["date"] >= CARD_AVAILABLE_FROM[c]
                    for c in cards)
    return ".", available


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--dataset", default="dataset_trans")
    ap.add_argument("--dir", default="3_perf_opt")
    ap.add_argument("--tex", help="write a LaTeX table to this path")
    args = ap.parse_args(argv)

    root = Path(args.dataset).resolve()
    projects = collect(root, args.dir)
    if not projects:
        print(f"no runs under {root}/*/{args.dir}")
        return 1
    seen = first_seen(projects)
    order = sorted(projects, key=lambda p: (projects[p]["date"], p))

    unmapped = {c for v in projects.values()
                for c in v["fired"] | set(v["landed"]) if c not in RULE_OF}
    w = 38
    print(" " * w + "".join(f"{SHORT.get(p, p)[:9]:>11}" for p in order) + f"{'landed':>9}")
    print("-" * (w + 11 * len(order) + 9))
    for group, rules in GROUPS:
        print(f"{group}")
        for rule in rules:
            cards = [c for c, r in RULE_OF.items() if r == rule]
            row, n, gap = f"  {rule:<{w-2}}", 0, 0
            for p in order:
                sym, avail = cell(projects[p], cards, seen)
                if not avail:
                    sym, gap = "-", gap + 1
                elif sym not in ("o", "."):
                    n += 1
                row += f"{sym:>11}"
            print(row + f"{n:>6}/{len(order)}" + (f"   (rule unavailable in {gap} projects at the time)" if gap else ""))
    print("-" * (w + 11 * len(order) + 9))
    print(f"unmapped cards: {sorted(unmapped) or 'none — all cards belong to a rule'}")

                                                 
    _bad = {p: v["scan_incomplete"] for p, v in projects.items()
            if v.get("scan_incomplete")}
    if _bad:
        print("\nWarning: detector coverage is unavailable because rule scans did not finish "
              "for the following projects. In their columns, `.` means 'not scanned', not 'no hits':")
        for proj, which in sorted(_bad.items()):
            print(f"      {proj}: {', '.join(which)} skipped "
                  f"(see {proj}/{args.dir}/rule_scan_status.json)")
    print("\ncard first fired (reference only):", {k: str(v) for k, v in sorted(seen.items())})

    if args.tex:
        Path(args.tex).write_text(render_tex(projects, order, seen), encoding="utf-8")
        print(f"\nLaTeX -> {args.tex}")
    return 0


def render_tex(projects, order, seen) -> str:
    L = []
    A = L.append
    A("% Generated by tools/facet_coverage.py — do not edit by hand.")
    A("% Regenerate:  python scripts/facet_coverage.py --tex tables/rule_commit.tex")
    A("")
    A(r"\begin{table*}[t]")
    A(r"  \centering")
    A(r"  \caption{Where each rule landed across the Evaluation Dataset. A number")
    A(r"  gives how many rewrites carrying that rule passed every gate and reached")
    A(r"  the final crate; $\circ$ marks a rule whose detector fired but whose")
    A(r"  rewrites were never accepted; $\cdot$ marks a rule no detector fired for.")
    A(r"  A dash marks a project optimized before the detector for that rule could")
    A(r"  fire, so it never had the opportunity; those projects are excluded from")
    A(r"  \emph{Landed}, whose denominator is therefore given per row.}")
    A(r"  \label{tab:rule-commit}")
    A("")
    A(r"  \scriptsize")
    A(r"  \setlength{\tabcolsep}{4pt}")
    A(r"  \renewcommand{\arraystretch}{1.05}")
    A("")
    A(r"  \begin{tabular}{@{}l" + "c" * len(order) + r"r@{}}")
    A(r"  \toprule")
    hdr = " & ".join(r"\rotatebox{90}{\textsc{" + SHORT.get(p, p).replace("_", r"\_") + "}}"
                     for p in order)
    A(r"  \textbf{Rule} & " + hdr + r" & \textbf{Landed} \\")
    A(r"  \midrule")
    for gi, (group, rules) in enumerate(GROUPS):
        if gi:
            A(r"  \midrule")
        A(r"  \multicolumn{" + str(len(order) + 2) + r"}{@{}l}{\textit{" + group + r"}} \\")
        for rule in rules:
            cards = [c for c, r in RULE_OF.items() if r == rule]
            cells, n = [], 0
            for p in order:
                sym, avail = cell(projects[p], cards, seen)
                if not avail:
                    cells.append(r"--")
                elif sym == "o":
                    cells.append(r"$\circ$")
                elif sym == ".":
                    cells.append(r"$\cdot$")
                else:
                    cells.append(sym)
                    n += 1
            denom = len(order) - cells.count(r"--")
            A(f"  {rule} & " + " & ".join(cells) + f" & {n}/{denom}" + r" \\")
    A(r"  \bottomrule")
    A(r"  \end{tabular}")
    A(r"\end{table*}")
    A("")
    return "\n".join(L)


if __name__ == "__main__":
    raise SystemExit(main())
