#!/usr/bin/env python3
"""Rebuild the evaluation table (RQ4 + RQ5) from the raw measurement files.

Input : ../dataset_trans/<project>/validation_workload/purebin/results/<batch>.json
        (<batch> per project is fixed in ../dataset_trans/_op_groups.py)
Output: ../results/eval_per_input.csv       one row per (operation, input) pair
        ../results/eval_per_operation.csv   the 41 table rows

Variants (columns):
  raw       c2rust output (0_raw)                         -> "Raw"
  codex     Codex CLI optimizing the raw project          -> "Codex"
  stage_a   Translation Refinement output (2_stage_a)     -> "Refined"      (RQ5)
  freeform  pipeline without rule guidance                -> "w/o rules"    (RQ5)
  full      PerfTrans (3_perf_opt)                        -> "PerfTrans"

Each cell = geometric mean over the row's (operation, input) pairs of
t_variant / t_C - 1. A variant whose output differs from C on any pair of the
row is not timed there and is reported as n/e (not equivalent).

Usage: python3 scripts/summarize_rq4_rq5.py      (run from forRQ4/)
"""
import csv
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
DT = HERE / "dataset_trans"
sys.path.insert(0, str(DT))
from _op_groups import GROUPS, result_name  # noqa: E402

ARMS = ["raw", "codex", "stage_a", "freeform", "full"]
LABEL = {"raw": "Raw", "codex": "Codex", "stage_a": "Refined",
         "freeform": "PerfTrans_w/o_rules", "full": "PerfTrans"}


def gmean(xs):
    return math.exp(sum(map(math.log, xs)) / len(xs))


def main():
    (HERE / "results").mkdir(exist_ok=True)
    pairs, rows = [], []
    for proj, groups in GROUPS.items():
        f = DT / proj / "validation_workload" / "purebin" / "results" / result_name(proj)
        d = json.loads(f.read_text())
        by_op = {}
        for r in d["results"]:
            b = r.get("builds", {})
            if not b.get("c", {}).get("median_ms"):
                continue
            by_op.setdefault(r["op"], []).append(r)
            row = {"project": proj, "operation": r["op"], "input": r.get("input"),
                   "c_median_ms": b["c"]["median_ms"]}
            for a in ARMS:
                m = b.get(a, {}).get("median_ms")
                row[f"{LABEL[a]}_overhead_pct"] = (round((m / b["c"]["median_ms"] - 1) * 100, 2)
                                                   if m else "n/e")
            pairs.append(row)
        for label, ops in groups:
            prs = [x for o in ops for x in by_op.get(o, [])]
            out = {"project": proj, "table_row": label, "n_pairs": len(prs)}
            for a in ARMS:
                ok = [x for x in prs if x["builds"].get(a, {}).get("median_ms")]
                out[LABEL[a]] = (round((gmean([x["builds"][a]["median_ms"] / x["builds"]["c"]["median_ms"]
                                               for x in ok]) - 1) * 100, 1)
                                 if ok and len(ok) == len(prs) else "n/e")
            rows.append(out)

    for name, data in [("eval_per_input.csv", pairs), ("eval_per_operation.csv", rows)]:
        with open(HERE / "results" / name, "w", newline="") as fh:
            w = csv.DictWriter(fh, fieldnames=list(data[0]))
            w.writeheader()
            w.writerows(data)

    num = lambda v: isinstance(v, float)
    full_beats_raw = sum(1 for r in rows if num(r["PerfTrans"]) and num(r["Raw"]) and r["PerfTrans"] < r["Raw"])
    codex_ne = sum(1 for r in rows if not num(r["Codex"]))
    both = [r for r in rows if num(r["PerfTrans"]) and num(r["Codex"])]
    win = sum(r["PerfTrans"] < r["Codex"] for r in both)
    lose = sum(r["PerfTrans"] > r["Codex"] for r in both)
    abl = sum(1 for r in rows if num(r["PerfTrans"]) and num(r["PerfTrans_w/o_rules"])
              and r["PerfTrans"] < r["PerfTrans_w/o_rules"])
    print(f"{len(rows)} table rows from {len(GROUPS)} projects")
    print(f"RQ4  PerfTrans faster than Raw on {full_beats_raw}/{len(rows)} rows")
    print(f"RQ4  PerfTrans faster than C on {sum(1 for r in rows if num(r['PerfTrans']) and r['PerfTrans'] < 0)} rows")
    print(f"RQ4  Codex n/e on {codex_ne} rows; vs Codex on {len(both)} comparable rows: "
          f"win {win} / lose {lose} / tie {len(both) - win - lose}")
    print(f"RQ5  PerfTrans faster than w/o-rules on {abl}/{len(rows)} rows")


if __name__ == "__main__":
    main()
