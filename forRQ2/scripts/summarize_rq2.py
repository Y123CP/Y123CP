#!/usr/bin/env python3
"""Rebuild the RQ2 cost-decomposition table and TMA breakdown from raw data.

Input : ../data/<project>/rq2_perf.json
Output: ../results/rq2_cost_decomposition.csv   the 12 regressions (> 3% overhead)
        ../results/rq2_tma.csv                  TMA level-1 breakdown where collected

  R_inst = I_Rust / I_C       (retired instructions, median of 3 perf-stat runs)
  R_CPI  = CPI_Rust / CPI_C
  Cost label: Work = R_inst >= 1.05 only; CPI = R_CPI >= 1.05 only; Both = both.
Per-operation ratios are geometric means over the operation's inputs.

Usage: python3 scripts/summarize_rq2.py      (run from forRQ2/)
"""
import csv
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
DATA = HERE / "data"
OUT = HERE / "results"
THRESH = 1.05


def gmean(xs):
    return math.exp(sum(map(math.log, xs)) / len(xs))


def main():
    OUT.mkdir(exist_ok=True)
    rows, tma = [], []
    for f in sorted(DATA.glob("*/rq2_perf.json")):
        p = f.parent.name
        by_op = {}
        for r in json.loads(f.read_text())["results"]:
            if r.get("timed") and r.get("c_ms"):
                by_op.setdefault(r["op"], []).append(r)
            if r.get("tma_c"):
                for dim in r["tma_c"]:
                    tma.append({"project": p, "operation": r["op"], "input": r["input"],
                                "tma_dim": dim, "c_share": r["tma_c"][dim],
                                "rust_share": r["tma_raw"][dim],
                                "delta_cpi_contribution": r.get("dcpi", {}).get(dim)})
        for op, rs in by_op.items():
            ov = (gmean([r["c2rust_raw_ms"] / r["c_ms"] for r in rs]) - 1) * 100
            if ov <= 3.0:
                continue
            ri = gmean([r["inst_ratio"] for r in rs])
            rc = gmean([r["cpi_ratio"] for r in rs])
            work, cpi = ri >= THRESH, rc >= THRESH
            label = "Both" if work and cpi else "Work" if work else "CPI" if cpi else "-"
            rows.append({"project": p, "operation": op, "overhead_pct": round(ov, 2),
                         "R_inst": round(ri, 3), "R_CPI": round(rc, 3), "cost": label})

    for name, data in [("rq2_cost_decomposition.csv", rows), ("rq2_tma.csv", tma)]:
        with open(OUT / name, "w", newline="") as fh:
            w = csv.DictWriter(fh, fieldnames=list(data[0]))
            w.writeheader()
            w.writerows(data)
    from collections import Counter
    c = Counter(r["cost"] for r in rows)
    print(f"{len(rows)} regressions: {c['Work']} Work / {c['CPI']} CPI / {c['Both']} Both")
    for r in sorted(rows, key=lambda r: r["cost"]):
        print(f"  {r['cost']:5} {r['project']:8} {r['operation']:22} "
              f"R_inst={r['R_inst']:.3f} R_CPI={r['R_CPI']:.3f}")


if __name__ == "__main__":
    main()
