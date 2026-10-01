#!/usr/bin/env python3
""                                               

                                                   
                                                    

     
                        
                                                            
                                                     
                                                             
                                         
                                    
                                        

                                                                                       
   
from __future__ import annotations

import argparse
import json
import statistics
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

ARTIFACT = Path(__file__).resolve().parents[2]
ROOT = ARTIFACT / "forRQ4" / "dataset_trans"
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ARTIFACT / "PerfTrans" / "script_c2rust"))
from _op_groups import GROUPS                                     # noqa: E402
from tools.facet_coverage import RULE_OF                          # noqa: E402
from _commit_history import commits                              # noqa: E402


def card_base(c: str) -> str:
    return c.strip().split(".")[0]          # "C11.split" -> "C11"


def total_pct(result: dict) -> float | None:
    for g in result.get("gates", []):
        if g.get("gate") == "w2":
            ms = [m for m in g.get("detail", {}).get("measurements", [])
                  if m.get("gate") == "total"]
            if ms:
                return ms[-1]["aggregate_pct"]
    return None


def attribution(proj: str) -> list[dict]:
    run = ROOT / proj / "3_perf_opt"
    by_sha = {}
    for f in (run / "changesets").glob("*/result.json"):
        r = json.loads(f.read_text())
        if r.get("commit_sha"):
            by_sha[r["commit_sha"][:7]] = r
    rows, prev = [], 0.0
    for sha, subj in commits(run):
        cards = [card_base(c) for c in subj.split(":", 1)[1].split(",")]
        rules = sorted({RULE_OF.get(c, f"?{c}") for c in cards})
        r = by_sha.get(sha[:7])
        tot = total_pct(r) if r else None
        eff = None if tot is None else ((1 + tot / 100) / (1 + prev / 100) - 1) * 100
        rows.append({"project": proj, "sha": sha, "cards": cards, "rules": rules,
                     "solo": len(rules) == 1, "total_pct": tot, "effect_pct": eff})
        if tot is not None:
            prev = tot
    return rows


def attempts(proj: str, arm_dir: str) -> dict:
    f = ROOT / proj / arm_dir / "rewrites.log"
    recs = [json.loads(l) for l in f.read_text().splitlines() if l.strip()]
    per_fn = Counter(r.get("fn_name") for r in recs if r.get("fn_name"))
    return {"attempts": len(recs), "fns": len(per_fn),
            "max_per_fn": max(per_fn.values()) if per_fn else 0,
            "terminal": Counter(r.get("terminal_status") for r in recs),
            "tokens_in": sum(r.get("tokens_in") or 0 for r in recs),
            "tokens_out": sum(r.get("tokens_out") or 0 for r in recs)}


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json")
    args = ap.parse_args()

    allrows = [row for p in GROUPS for row in attribution(p)]
    meas = [r for r in allrows if r["effect_pct"] is not None]
    print(f"== 1. full 臂逐 commit 归因:commits {len(allrows)},有整 crate 测量 {len(meas)}")
    print(f"   solo {sum(r['solo'] for r in meas)} / joint {sum(not r['solo'] for r in meas)}")
    if len(meas) < len(allrows):
        for r in allrows:
            if r["effect_pct"] is None:
                print(f"   (无 total 测量) {r['project']} {r['sha']} {r['cards']}")
    per_rule = defaultdict(lambda: {"solo": [], "joint": 0, "commits": 0})
    for r in allrows:
        for rule in r["rules"]:
            per_rule[rule]["commits"] += 1
    for r in meas:
        if r["solo"]:
            per_rule[r["rules"][0]]["solo"].append(r)
        else:
            for rule in r["rules"]:
                per_rule[rule]["joint"] += 1
    print(f"   {'rule':<42}{'commits':>8}{'solo':>6}{'joint':>6}  best solo")
    for rule, v in sorted(per_rule.items(), key=lambda kv: -kv[1]["commits"]):
        best = min(v["solo"], key=lambda r: r["effect_pct"], default=None)
        bs = f"{best['effect_pct']:+.2f}% {best['project']} {best['cards']}" if best else "—"
        med = (f"  median {statistics.median(r['effect_pct'] for r in v['solo']):+.2f}%"
               if v["solo"] else "")
        print(f"   {rule:<42}{v['commits']:>8}{len(v['solo']):>6}{v['joint']:>6}  {bs}{med}")
    print("   最大的 8 个 commit(不分 solo/joint):")
    for r in sorted(meas, key=lambda r: r["effect_pct"])[:8]:
        print(f"     {r['effect_pct']:+7.2f}%  {r['project']:<14}{r['sha']}  "
              f"{'solo ' if r['solo'] else 'joint'} {r['cards']} -> {r['rules']}")

    print("\n== 2. 尝试次数与门")
    agg = {}
    for arm, d in (("full", "3_perf_opt"), ("freeform", "3_perf_opt_freeform")):
        tot = {"attempts": 0, "fns": 0, "tokens_in": 0, "tokens_out": 0,
               "terminal": Counter(), "max_per_fn": 0}
        for p in GROUPS:
            a = attempts(p, d)
            for k in ("attempts", "fns", "tokens_in", "tokens_out"):
                tot[k] += a[k]
            tot["terminal"] += a["terminal"]
            tot["max_per_fn"] = max(tot["max_per_fn"], a["max_per_fn"])
        agg[arm] = tot
        print(f"   {arm:<9} attempts={tot['attempts']} fn_slots={tot['fns']} "
              f"att/fn={tot['attempts'] / tot['fns']:.2f} max/fn={tot['max_per_fn']} "
              f"tok_in={tot['tokens_in']:,} tok_out={tot['tokens_out']:,}")
        print(f"   {'':<9} {dict(tot['terminal'].most_common())}")

    print("\n== 3. freeform 臂落地的 commit")
    ff = []
    for p in GROUPS:
        run = ROOT / p / "3_perf_opt_freeform"
        cs = commits(run)
        stat = subprocess.run(["git", "-C", str(run / "crate"), "log", "--reverse",
                               "--shortstat", "--format=@%h"], capture_output=True,
                              text=True).stdout
        print(f"   {p:<14} {len(cs)} commits")
        ff.append({"project": p, "commits": [c[0] for c in cs]})
    if args.json:
        Path(args.json).write_text(json.dumps(
            {"full_commits": allrows,
             "attempts": {a: {**v, "terminal": dict(v["terminal"])} for a, v in agg.items()},
             "freeform_commits": ff}, indent=1))
        print(f"\n[written] {args.json}")


if __name__ == "__main__":
    main()
