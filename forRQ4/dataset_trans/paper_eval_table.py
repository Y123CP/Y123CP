#!/usr/bin/env python3
""                                           

                                
                                               
                                                 
                                                    
                                                                                               
                                                                        

                     
                                                             
                                                 
                                             

                                                 
                                    

   
                                              
                                               

                                                          
                                                       
                                             
   
   
from __future__ import annotations
import json, math, sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _op_groups import GROUPS, result_name                        # noqa: E402

ROOT = Path(__file__).resolve().parent
PAPER = ["raw", "codex", "stage_a", "freeform", "full"]            
EXTRA = []                                                                 
DISPLAY = {"optipng-0.7.7": "optipng"}


def geomean(xs): return math.exp(sum(math.log(x) for x in xs) / len(xs))


def load(proj):
    f = ROOT / proj / "validation_workload" / "purebin" / "results" / result_name(proj)
    return json.loads(f.read_text()) if f.is_file() else None


def collect(proj):
    """-> (rows, warnings);row = (label, n_pairs, c_ms, {arm: pct|None})"""
    d = load(proj)
    if d is None:
        return None, [f"缺 {result_name(proj)}"]
    warn, by_op = [], {}
    for r in d["results"]:
        if r.get("equivalence") == "FAIL":
            warn.append(f"整对作废(C 不等价或可测列 <2): {r['op']}/{r.get('input')}")
            continue
        if r.get("measurement") == "ABORTED":
            warn.append(f"测量中崩溃作废: {r['op']}/{r.get('input')} build={r.get('aborted_build')}")
            continue
        for ex in r.get("excluded_builds", []):
            warn.append(f"等价失败被排除: build={ex} @ {r['op']}/{r.get('input')}")
        if r.get("cv_gate") != "ok":
            warn.append(f"CV 未收敛: {r['op']}/{r.get('input')}")
        by_op.setdefault(r["op"], []).append(r)

    rows = []
    for label, ops in GROUPS[proj]:
        pairs = [p for o in ops for p in by_op.get(o, [])]
        if not pairs:
            warn.append(f"行 '{label}' 没有任何可用 pair")
            continue
        c_ms = sum(p["builds"]["c"]["median_ms"] for p in pairs)
        vals, sets = {}, {}
        for v in PAPER + EXTRA:
            ok = [p for p in pairs
                  if v in p["builds"] and p["builds"][v].get("median_ms")]
            sets[v] = {(p["op"], p["input"]) for p in ok}
            vals[v] = ((geomean([p["builds"][v]["median_ms"] / p["builds"]["c"]["median_ms"]
                                 for p in ok]) - 1) * 100) if ok else None
        full = {(p["op"], p["input"]) for p in pairs}
        for v in PAPER:
            if sets[v] != full:
                warn.append(f"⚠ 行 '{label}' 的 {v} 列少了 {sorted(full - sets[v])}"
                            f" —— 该格与同行其他列**不同分母**,不可直接横向比")
        rows.append((label, len(pairs), c_ms, vals))
    return rows, warn


def fmt(v): return "—" if v is None else f"{v:+.1f}\\%"


def main():
    check = "--check" in sys.argv
    all_rows, all_warn = [], []
    for proj in GROUPS:
        rows, warn = collect(proj)
        name = DISPLAY.get(proj, proj)
        all_warn += [f"{name}: {w}" for w in dict.fromkeys(warn)]
        if rows:
            all_rows.append((name, rows))

    if check:
        print(f"体检:{len(all_rows)}/12 项目有数据,{sum(len(r) for _,r in all_rows)}/41 行")
        print("\n".join(all_warn) if all_warn else "无告警 ✓")
        return

    cols = PAPER + EXTRA
    print("| Project | Operation | pairs | C (ms) | " + " | ".join(cols) + " |")
    print("|---|---|--:|--:|" + "--:|" * len(cols))
    for name, rows in all_rows:
        for i, (lbl, n, c_ms, o) in enumerate(rows):
            p = f"**{name}**" if i == 0 else ""
            print(f"| {p} | {lbl} | {n} | {c_ms:.0f} | "
                  + " | ".join(fmt(o[v]).replace('\\%', '%') for v in cols) + " |")
    print()
    for v in cols:                                          
        rs = []
        for proj in GROUPS:
            d = load(proj)
            if not d:
                continue
            for r in d["results"]:
                b = r.get("builds", {})
                if r.get("equivalence") == "FAIL" or r.get("measurement") == "ABORTED":
                    continue
                if v in b and b[v].get("median_ms") and b.get("c", {}).get("median_ms"):
                    rs.append(b[v]["median_ms"] / b["c"]["median_ms"])
        if rs:
            print(f"- **{v}** 全局几何平均({len(rs)} pairs): {(geomean(rs)-1)*100:+.1f}%")
    if all_warn:
        print("\n### 告警")
        for w in all_warn:
            print(f"- {w}")


if __name__ == "__main__":
    main()
