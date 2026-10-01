"""Stage A intra_ptr — P1 dump CLI.

Runs COLLECT (tree-sitter) over a crate and writes the per-pointer-param
facts to JSON for human review of accuracy BEFORE any rewrite is attempted
(design doc P1 checkpoint). Also prints a summary so anomalies surface at
scale.

Usage:
    python -m perf_opt.stage_a.intra_ptr.dump <crate_dir> [--out facts.json]
                                              [--only fnA fnB ...]
                                              [--with-uses]

This is read-only. It rewrites nothing, calls no LLM, needs no SVF.
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path

from .collect import collect_crate
from .plan import plan_crate


def _summarize(facts) -> dict:
    n_params = len(facts)
    n_fns = len({(f.fn_file, f.fn_name) for f in facts})
    count = Counter(f.count for f in facts)
    arr = [f for f in facts if f.count == "Array"]
    len_kind = Counter(f.length_source.kind for f in arr)
    mut = Counter("Mutable" if f.body_mutates else "Immutable" for f in facts)
    cursor = sum(1 for f in facts if f.cursor_reassigned)
    use_kinds = Counter(u.kind for f in facts for u in f.use_sites)
    # "liftable hint" = Array with a resolved length, or Scalar (no length
    # needed). NOT a final verdict (no SVF alias gate yet) — just a tree-
    # sitter-side readiness signal for review.
    array_with_len = sum(1 for f in arr if f.length_source.kind != "UNKNOWN")
    array_unknown = sum(1 for f in arr if f.length_source.kind == "UNKNOWN")
    return {
        "fns_with_ptr_params": n_fns,
        "ptr_params": n_params,
        "count": dict(count),
        "array_length_source": dict(len_kind),
        "array_with_resolved_len": array_with_len,
        "array_length_UNKNOWN": array_unknown,
        "body_mutates": dict(mut),
        "cursor_reassigned": cursor,
        "use_site_kinds": dict(use_kinds),
    }


def _skip_category(reason: str | None) -> str:
    if not reason:
        return "?"
    if reason.startswith("passed to call"):
        return "passed to call (cross-callee mutation)"
    return reason.split(" —")[0].strip()


def _plan_summary(cp) -> dict:
    from collections import Counter
    elig = [p for p in cp.plans if p.eligible]
    skips = Counter(_skip_category(p.skip_reason)
                    for p in cp.plans if not p.eligible)
    view = Counter(p.target_view.split("[")[0].rstrip() if "[" in (p.target_view or "")
                   else p.target_view for p in elig)
    return {
        "ptr_params_total": len(cp.plans),
        "floor_eligible": len(elig),
        "eligible_units (fns)": len(cp.units),
        "target_view_kind": {"&[T] (Array)": sum(1 for p in elig if p.length),
                             "&T (Scalar)": sum(1 for p in elig if not p.length)},
        "skip_reasons": dict(skips.most_common()),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description="Stage A intra_ptr dump (P1, read-only)")
    ap.add_argument("crate_dir", type=Path)
    ap.add_argument("--out", type=Path, default=None,
                    help="write full per-param facts/plan JSON here")
    ap.add_argument("--only", nargs="+", default=None,
                    help="restrict to these fn names")
    ap.add_argument("--with-uses", action="store_true",
                    help="include full use_sites in the JSON (verbose)")
    ap.add_argument("--plan", action="store_true",
                    help="run PLAN (safe-floor) and report eligibility")
    ap.add_argument("--broad", action="store_true",
                    help="[--plan] disable the airtight gate (show the wider, "
                         "SVF-needing floor instead of the airtight subset)")
    args = ap.parse_args()

    if args.plan:
        cp = plan_crate(args.crate_dir,
                        only_fns=set(args.only) if args.only else None,
                        airtight=not args.broad)
        print(json.dumps(_plan_summary(cp), indent=2, ensure_ascii=False))
        if args.out:
            args.out.write_text(json.dumps(
                {"summary": _plan_summary(cp),
                 "units": [{"fn": u.fn_name, "file": u.fn_file, "kind": u.kind,
                            "priority": u.priority,
                            "pointers": [p.to_json() for p in u.pointers]}
                           for u in cp.units]},
                indent=2, ensure_ascii=False), encoding="utf-8")
            print(f"\n→ wrote {len(cp.units)} eligible units to {args.out}")
        return 0

    facts = collect_crate(args.crate_dir,
                          only_fns=set(args.only) if args.only else None)

    summary = _summarize(facts)
    print(json.dumps(summary, indent=2, ensure_ascii=False))

    if args.out:
        recs = []
        for f in facts:
            d = f.to_json()
            if not args.with_uses:
                d["use_sites"] = f"{len(f.use_sites)} sites (use --with-uses)"
            recs.append(d)
        args.out.write_text(
            json.dumps({"summary": summary, "params": recs},
                       indent=2, ensure_ascii=False),
            encoding="utf-8",
        )
        print(f"\n→ wrote {len(recs)} param records to {args.out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
