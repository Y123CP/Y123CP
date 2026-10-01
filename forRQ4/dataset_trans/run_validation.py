#!/usr/bin/env python3
"""Three-way validation runner: C vs c2rust_raw vs ours.

Drives a validation_workload/study.toml (Method B: one C driver, N link
targets). Protocol follows empirical_study/run_study.py: per-(op,input)
equivalence gate (sha256 of stdout vs C oracle), warmup, paired alternating
measurement, IQR outlier drop, median, CV gate.

Usage (on 37.2, from repo root, under perf_run.sh for freq pinning):
  bash perf_run.sh script_c2rust/.venv/bin/python \
      dataset_trans/run_validation.py dataset_trans/lz4/validation_workload/study.toml \
      [--runs N] [--ops op ...] [--inputs name ...] [--out results.json]

Builds whose binary is missing or whose status is "pending" are skipped with
a warning (so the same study.toml works before `ours` exists).
"""
import argparse
import hashlib
import json
import os
import statistics
import subprocess
import sys
import time
import tomllib
from pathlib import Path


def find_repo_root(start: Path) -> Path:
    p = start.resolve()
    while p != p.parent:
        if (p / "dataset_source").is_dir() and (p / "dataset_trans").is_dir():
            return p
        p = p.parent
    sys.exit("cannot locate repo root (dataset_source + dataset_trans)")


def loadavg() -> list[float]:
    return [round(x, 2) for x in os.getloadavg()]


def governor(cpu: int) -> str:
    try:
        return Path(f"/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_governor").read_text().strip()
    except OSError:
        return "?"


def run_once(cmd: list[str], capture: bool) -> tuple[float, int, bytes]:
    t0 = time.perf_counter()
    r = subprocess.run(cmd, stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL)
    dt = time.perf_counter() - t0
    return dt, r.returncode, (r.stdout or b"") if capture else b""


def perf_stat(cmd: list[str]) -> dict:
    ""                                                                

                                                     
                                                  
                                                   
                               
       
    try:
        r = subprocess.run(["perf", "stat", "-x", ",",
                            "-e", "instructions,cycles", "--"] + cmd,
                           stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                           timeout=600)
    except (OSError, subprocess.SubprocessError):
        return {}
    out = {}
    for line in r.stderr.decode(errors="replace").splitlines():
        f = line.split(",")
        if len(f) >= 3 and f[0].replace(".", "").isdigit():
            if f[2] == "instructions":
                out["instructions"] = int(float(f[0]))
            elif f[2] == "cycles":
                out["cycles"] = int(float(f[0]))
    if "instructions" in out and out.get("cycles"):
        out["ipc"] = round(out["instructions"] / out["cycles"], 3)
    return out


def iqr_filter(xs: list[float]) -> list[float]:
    if len(xs) < 4:
        return xs
    qs = statistics.quantiles(xs, n=4)
    q1, q3 = qs[0], qs[2]
    lo, hi = q1 - 1.5 * (q3 - q1), q3 + 1.5 * (q3 - q1)
    kept = [x for x in xs if lo <= x <= hi]
    return kept or xs


def stats(xs: list[float]) -> dict:
    kept = iqr_filter(xs)
    med = statistics.median(kept)
    mean = statistics.fmean(kept)
    cv = (statistics.stdev(kept) / mean) if len(kept) > 1 and mean > 0 else 0.0
    return {"median_ms": med * 1000, "mean_ms": mean * 1000,
            "min_ms": min(kept) * 1000, "cv": cv,
            "n_raw": len(xs), "n_kept": len(kept)}


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("study")
    ap.add_argument("--runs", type=int, default=None)
    ap.add_argument("--ops", nargs="*", default=None)
    ap.add_argument("--inputs", nargs="*", default=None)
    ap.add_argument("--builds", nargs="*", default=None,
                    help="只测这些 build(须含 'c' 作 oracle);默认测 study.compare 全部")
    ap.add_argument("--out", default=None)
    ap.add_argument("--max-load", type=float, default=None,
                    help="测前 load 门(1min);超过则 warn,或 --wait-quiet 时阻塞等安静")
    ap.add_argument("--cv-retries", type=int, default=None,
                    help="某版本 CV 超 required_cv 时,自动重测该 op 的最大次数")
    ap.add_argument("--wait-quiet", action="store_true",
                    help="load 超门时阻塞等待安静,而不是照跑出脏数")
    ap.add_argument("--warmup", type=int, default=None, help="覆盖 warmup 轮数(debug 调小)")
    ap.add_argument("--equiv-only", action="store_true",
                    help="只跑逐 build 等价门,不计时。开跑前的预检:先把 "
                         "(build,op,input) 的失败矩阵摊开,再决定要不要花几小时测")
    ap.add_argument("--perf-stat", action="store_true",
                    help="每个 (build,op,input) 额外跑一次 perf stat 采 insns/cycles/IPC,"
                         "供 §11.5 的 wall↔insns 交叉验证。开销 ≈ 每对每 build 多一轮")
    args = ap.parse_args()

    study_path = Path(args.study)
    cfg = tomllib.loads(study_path.read_text())
    root = find_repo_root(study_path.parent)

    measure = cfg["measure"]
    warmup = args.warmup if args.warmup is not None else int(measure.get("warmup_runs", 5))
    runs = args.runs or int(measure.get("measure_runs", 30))
    required_cv = float(measure.get("required_cv", 0.03))
    pin = int(measure.get("pin_cpu", 16))
    max_load = args.max_load if args.max_load is not None else float(measure.get("max_load", 1.0))
    cv_retries = args.cv_retries if args.cv_retries is not None else int(measure.get("cv_retries", 2))
    order = measure.get("order") or cfg["study"]["compare"]
    if args.builds:
        sel = set(args.builds) | {"c"}                      
        order = [n for n in order if n in sel]

    builds = {}
    for name in order:
        b = cfg.get("builds", {}).get(name)
        if b is None:
            continue
        binp = root / b["binary_path"]
        if b.get("status") == "pending" or not binp.is_file():
            print(f"[skip] build '{name}': pending or binary missing ({binp})")
            continue
        builds[name] = binp
    if "c" not in builds or len(builds) < 2:
        sys.exit("need the 'c' reference plus at least one other build")
    names = [n for n in order if n in builds]
    print(f"[info] builds: {', '.join(names)}   runs={runs} warmup={warmup} "
          f"pin_cpu={pin} (governor={governor(pin)})   load={loadavg()}")

    results = []
    load_pre = loadavg()
    for op in cfg.get("operations", []):
        if args.ops and op["name"] not in args.ops:
            continue
        if op.get("status") == "coverage_only":   # exercised for coverage, not timed
            print(f"[skip] op '{op['name']}': coverage_only")
            continue
        for inp in op.get("inputs", []):
            if args.inputs and inp["name"] not in args.inputs:
                continue
            ipath = root / inp["path"]
            if not ipath.is_file():
                print(f"[FAIL] {op['name']}/{inp['name']}: input missing {ipath}")
                continue
            argv = [a.replace("$INPUT", str(ipath)).replace("$ITERS", str(inp.get("iters", 1)))
                    for a in op["args"]]
            cmds = {n: ["taskset", "-c", str(pin), str(builds[n])] + argv for n in names}
            tag = f"{op['name']}/{inp['name']}"

            # -- equivalence gate, PER BUILD (2026-09-06) --
                                                             
                                                    
                                             
                                                           
                                                       
            oracle = inp.get("stdout_sha256", "")
            eq = {}
            for n in names:
                _, rc, out = run_once(cmds[n], capture=True)
                got = hashlib.sha256(out).hexdigest()
                bad = rc != 0 or (oracle and got != oracle)
                eq[n] = {"ok": not bad, "rc": rc, "sha256": got}
                if bad:
                    print(f"[EQUIVALENCE FAIL] {tag} build={n} rc={rc} sha={got[:16]}…")
            if not eq["c"]["ok"]:
                print(f"[skip] {tag}: C 参照自身不等价,无 oracle,整个 pair 作废")
                results.append({"op": op["name"], "input": inp["name"],
                                "equivalence": "FAIL", "equivalence_per_build": eq})
                continue
            timed = [n for n in names if eq[n]["ok"]]
            excluded = [n for n in names if not eq[n]["ok"]]
            if excluded:
                print(f"[gate] {tag}: 排除不等价的 build {excluded},其余 {timed} 照常计时")
            if len(timed) < 2 and not args.equiv_only:
                results.append({"op": op["name"], "input": inp["name"],
                                "equivalence": "FAIL", "equivalence_per_build": eq})
                continue

            if args.equiv_only:
                results.append({"op": op["name"], "input": inp["name"],
                                "equivalence": "ok" if not excluded else "PARTIAL",
                                "equivalence_per_build": eq,
                                "excluded_builds": excluded})
                mark = "  ".join(f"{n}={'ok' if eq[n]['ok'] else 'FAIL'}" for n in names)
                print(f"[equiv] {tag}: {mark}", flush=True)
                continue

                                                                                      
            attempt = 0
            while True:
                attempt += 1
                lo = os.getloadavg()[0]
                if lo > max_load:
                    if args.wait_quiet:
                        print(f"[gate] {tag}: load {lo} > {max_load},等待安静…", flush=True)
                        while os.getloadavg()[0] > max_load:
                            time.sleep(10)
                    else:
                        print(f"[gate ⚠] {tag}: load {lo} > {max_load} — 数字可能不可信(--wait-quiet 可阻塞等待)")
                for _ in range(warmup):
                    for n in timed:
                        run_once(cmds[n], capture=False)
                samples = {n: [] for n in timed}
                aborted = None
                for _ in range(runs):
                    for n in timed:
                        dt, rc, _ = run_once(cmds[n], capture=False)
                        if rc != 0:
                                                                  
                                                    
                            print(f"[FATAL-pair] {tag} build={n} rc={rc} 测量中崩溃,作废该 pair")
                            aborted = n
                            break
                        samples[n].append(dt)
                    if aborted:
                        break
                if aborted:
                    break
                worst_cv = max((stats(samples[n])["cv"] for n in timed), default=0.0)
                if worst_cv <= required_cv or attempt > cv_retries:
                    if worst_cv > required_cv:
                        print(f"[gate ⚠] {tag}: CV {worst_cv*100:.1f}% > {required_cv*100:.0f}% 重测 {attempt} 次仍超标,标记 unstable")
                    break
                print(f"[gate] {tag}: CV {worst_cv*100:.1f}% > {required_cv*100:.0f}%,重测({attempt}/{cv_retries})", flush=True)

            if aborted:
                results.append({"op": op["name"], "input": inp["name"],
                                "equivalence": "ok", "measurement": "ABORTED",
                                "aborted_build": aborted,
                                "equivalence_per_build": eq})
                continue

            row = {"op": op["name"], "input": inp["name"], "equivalence": "ok",
                   "iters": inp.get("iters"),
                   "cv_gate": "ok" if worst_cv <= required_cv else "unstable",
                   "attempts": attempt, "builds": {},
                   "equivalence_per_build": eq,
                   "excluded_builds": excluded}
            c_med = None
            for n in timed:
                st = stats(samples[n])
                st["cv_warn"] = st["cv"] > required_cv
                row["builds"][n] = st
                if n == "c":
                    c_med = st["median_ms"]
            for n in excluded:                                    
                row["builds"][n] = {"equivalence": "FAIL", "median_ms": None}
            for n in timed:
                if n != "c" and c_med:
                    row["builds"][n]["gap_vs_c_pct"] = round(
                        (row["builds"][n]["median_ms"] - c_med) / c_med * 100, 2)
                                                
            for prev, cur in zip(timed, timed[1:]):
                pm = row["builds"][prev]["median_ms"]
                if pm:
                    row["builds"][cur]["delta_vs_prev_pct"] = round(
                        (row["builds"][cur]["median_ms"] - pm) / pm * 100, 2)
                                                             
                                                           
            if args.perf_stat:
                for n in timed:
                    ps = perf_stat(cmds[n])
                    if ps:
                        row["builds"][n].update(ps)
                c_ins = row["builds"].get("c", {}).get("instructions")
                for n in timed:
                    ins = row["builds"][n].get("instructions")
                    if n != "c" and c_ins and ins:
                        row["builds"][n]["insns_vs_c_pct"] = round(
                            (ins - c_ins) / c_ins * 100, 2)

                                          
            if "c2rust_raw" in row["builds"] and "ours" in row["builds"]:
                raw_med = row["builds"]["c2rust_raw"]["median_ms"]
                row["builds"]["ours"]["gain_vs_raw_pct"] = round(
                    (row["builds"]["ours"]["median_ms"] - raw_med) / raw_med * 100, 2)
            results.append(row)

            cells = []
            for n in timed:
                st = row["builds"][n]
                warn = " ⚠cv" if st["cv_warn"] else ""
                gap = f" ({st['gap_vs_c_pct']:+.2f}%)" if "gap_vs_c_pct" in st else ""
                cells.append(f"{n}={st['median_ms']:.1f}ms cv={st['cv']*100:.1f}%{warn}{gap}")
            print(f"[done] {tag}: " + "  ".join(cells), flush=True)

    payload = {
        "study": cfg["study"]["project"], "runs": runs, "warmup": warmup,
        "pin_cpu": pin, "governor": governor(pin),
        "loadavg_pre": load_pre, "loadavg_post": loadavg(),
        "builds": {n: str(builds[n]) for n in names},
        "results": results,
    }
    out = Path(args.out) if args.out else study_path.parent / "results" / "validation_run.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(payload, indent=2))
    print(f"[info] wrote {out}   load_post={payload['loadavg_post']}")


if __name__ == "__main__":
    main()
