#!/usr/bin/env python3
"""Rebuild the RQ1 table (runtime overhead of c2rust output over C) from raw data.

Input : ../empirical_study/<project>/rq2_perf.json   (one file per Study project)
Output: ../results/rq1_per_input.csv       one row per (operation, input)
        ../results/rq1_per_operation.csv   one row per operation (= Table RQ1)

Per-operation overhead = geometric mean over its inputs of (t_Rust / t_C) - 1,
where t_* is the median of 30 paired, outlier-filtered measurements.

Usage: python3 scripts/summarize_rq1.py      (run from forRQ1/)
"""
import csv
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
STUDY = HERE / "empirical_study"
OUT = HERE / "results"
PROJECTS = ["libcsv", "libogg", "bzip2", "xxHash", "libzahl", "binn", "json-c", "heman"]


def gmean(xs):
    return math.exp(sum(map(math.log, xs)) / len(xs))


def main():
    OUT.mkdir(exist_ok=True)
    per_input, per_op = [], []
    for p in PROJECTS:
        d = json.loads((STUDY / p / "rq2_perf.json").read_text())
        by_op = {}
        for r in d["results"]:
            if not (r.get("timed") and r.get("c_ms")):
                continue
            by_op.setdefault(r["op"], []).append(r)
            per_input.append({
                "project": p, "operation": r["op"], "input": r["input"],
                "c_median_ms": r["c_ms"], "rust_median_ms": r["c2rust_raw_ms"],
                "c_cv": r.get("c_cv"), "rust_cv": r.get("c2rust_raw_cv"),
                "overhead_pct": r["gap_pct"],
            })
        for op, rs in by_op.items():
            ov = (gmean([r["c2rust_raw_ms"] / r["c_ms"] for r in rs]) - 1) * 100
            per_op.append({"project": p, "operation": op, "n_inputs": len(rs),
                           "overhead_pct": round(ov, 2),
                           "regression_gt_3pct": ov > 3.0})

    for name, rows in [("rq1_per_input.csv", per_input), ("rq1_per_operation.csv", per_op)]:
        with open(OUT / name, "w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0]))
            w.writeheader()
            w.writerows(rows)
    print(f"{len(per_op)} operations, {sum(r['regression_gt_3pct'] for r in per_op)} regressions > 3%")
    for r in per_op:
        print(f"  {r['project']:8} {r['operation']:22} {r['overhead_pct']:+7.2f}%")


if __name__ == "__main__":
    main()
