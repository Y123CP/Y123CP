#!/usr/bin/env python3
"""Empirical-study measurement driver — reads an empirical_study/<proj>/study.toml
and produces a per-input C vs c2rust_raw wall-clock comparison.

RUN ON 37.2, freq-pinned, e.g.:
    bash perf_run.sh python3 empirical_study/run_study.py empirical_study/bzip2/study.toml --runs 30

Protocol (docs/perf_tree_design.md §2.8.4):
  · per (operation,input): warmup, then N paired reps ALTERNATING the two builds
    (cancels slow drift), each rep pinned to the isolated core via taskset.
  · functional-equivalence GATE: build stdout sha256 must == input.stdout_sha256.
  · metric: median after dropping >1.5*IQR outliers; report CV; flag if CV>required.
  · gap%  = (raw_median - c_median) / c_median * 100   (negative => c2rust faster).

Only measures builds already compiled (uses binary_path). Pass --build to (re)build.
"""
import argparse, hashlib, json, os, statistics, subprocess, sys, tempfile, time, tomllib
from pathlib import Path

TOPLEV = os.path.expanduser("~/pmu-tools/toplev.py")   # pmu-tools TMA driver (RQ2)

def sh(cmd, **kw):
    return subprocess.run(cmd, shell=isinstance(cmd, str), **kw)

def _argv(binary, args, inp, pin):
    return ["taskset", "-c", str(pin), binary] + [inp if a == "$INPUT" else a for a in args]

def perf_counts(binary, args, inp, pin):
    """`perf stat -e instructions,cycles` (per-process) → (instructions, cycles)."""
    argv = ["perf", "stat", "-x,", "-e", "instructions,cycles", "--"] + _argv(binary, args, inp, pin)
    p = subprocess.run(argv, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
    inst = cyc = None
    for line in p.stderr.splitlines():
        f = line.split(",")
        if len(f) < 3:
            continue
        try:
            v = float(f[0])
        except ValueError:
            continue
        if "instructions" in f[2]:
            inst = v
        elif "cycles" in f[2]:
            cyc = v
    return inst, cyc

def perf_median(binary, args, inp, pin, reps=3):
    ins, cys = [], []
    for _ in range(reps):
        i, c = perf_counts(binary, args, inp, pin)
        if i and c:
            ins.append(i); cys.append(c)
    if not ins:
        return None
    return statistics.median(ins), statistics.median(cys)

# toplev top-level node name -> our TMA symbol
_TMA = {"Frontend_Bound": "FE", "Bad_Speculation": "BS", "Retiring": "Ret",
        "Backend_Bound.Memory_Bound": "Mem", "Backend_Bound.Core_Bound": "Core"}

def toplev_tma(binary, args, inp, pin):
    """`toplev -l2 -v --no-multiplex` → {Ret,FE,BS,Mem,Core} as fractions (sum≈1)."""
    if not os.path.isfile(TOPLEV):
        return None
    fd, csv = tempfile.mkstemp(suffix=".csv"); os.close(fd)
    argv = ["python3", TOPLEV, "-l2", "-v", "--no-multiplex", "--single-thread",
            "-x", ",", "-o", csv, "--"] + _argv(binary, args, inp, pin)
    subprocess.run(argv, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    tma = {}
    try:
        for line in Path(csv).read_text().splitlines():
            f = line.split(",")
            if len(f) >= 2 and f[0] in _TMA:
                try:
                    tma[_TMA[f[0]]] = float(f[1]) / 100.0
                except ValueError:
                    pass
    finally:
        os.unlink(csv)
    return tma or None

def iqr_trim(xs):
    if len(xs) < 4: return xs
    s = sorted(xs); n = len(s)
    q1 = s[n//4]; q3 = s[(3*n)//4]; iqr = q3 - q1
    lo, hi = q1 - 1.5*iqr, q3 + 1.5*iqr
    return [x for x in xs if lo <= x <= hi] or xs

def time_run(binary, args, inp, pin, capture):
    """Run `taskset -c <pin> binary args(with $INPUT->inp)`; return (secs, sha256|None)."""
    argv = ["taskset", "-c", str(pin), binary] + [inp if a == "$INPUT" else a for a in args]
    t0 = time.perf_counter()
    p = subprocess.run(argv, stdout=(subprocess.PIPE if capture else subprocess.DEVNULL),
                       stderr=subprocess.DEVNULL)
    dt = time.perf_counter() - t0
    digest = hashlib.sha256(p.stdout).hexdigest() if capture else None
    if p.returncode != 0:
        raise RuntimeError(f"exit {p.returncode}: {' '.join(argv[:6])}...")
    return dt, digest

def measure(root, study, runs, ops_filter, inputs_filter, do_perf=False, perf_reps=3,
            max_load=1.0, cv_retries=2, wait_quiet=False):
    m = study["measure"]; pin = m["pin_cpu"]; warm = m["warmup_runs"]
    req_cv = m["required_cv"]
    min_ms_floor = m.get("min_runtime_ms")   # validity floor (previously never read = dead config)
    builds = {k: (root / study["builds"][k]["binary_path"])
              for k in study["study"]["compare"]}
    for k, b in builds.items():
        if not b.is_file(): sys.exit(f"[FATAL] build '{k}' binary missing: {b} (build it first)")
    order = study["study"]["compare"]           # e.g. ["c","c2rust_raw"]
    results = []
    for op in study["operations"]:
        if ops_filter and op["name"] not in ops_filter: continue
        if op.get("status") == "inputs_pending_generation":
            # inputs may now exist; proceed if files are present
            pass
        for inp in op["inputs"]:
            if inputs_filter and inp["name"] not in inputs_filter: continue
            ipath = root / inp["path"]
            if not ipath.is_file():
                print(f"  SKIP {op['name']}/{inp['name']}: input missing {ipath}"); continue
            exp = inp["stdout_sha256"]
            # gate: each build must reproduce the oracle hash
            gate = {}
            for k in order:
                _, dg = time_run(str(builds[k]), op["args"], str(ipath), pin, capture=True)
                gate[k] = (dg == exp)
            if not all(gate.values()):
                print(f"  !! {op['name']}/{inp['name']} EQUIVALENCE FAIL: {gate} "
                      f"(exp {exp[:12]}..) — NOT timed");
                results.append({"op":op["name"],"input":inp["name"],"equiv":gate,"timed":False})
                continue
            # -- warmup + paired measurement, with load gate + CV auto-retry (gate B,
            #    mirrors dataset_trans/run_validation.py) --
            tag = f"{op['name']}/{inp['name']}"
            attempt = 0
            while True:
                attempt += 1
                lo = os.getloadavg()[0]
                if lo > max_load:
                    if wait_quiet:
                        print(f"  [gate] {tag}: load {lo} > {max_load}, 等待安静…", flush=True)
                        while os.getloadavg()[0] > max_load:
                            time.sleep(10)
                    else:
                        print(f"  [gate ⚠] {tag}: load {lo} > {max_load} — 数字可能不可信"
                              f"(--wait-quiet 可阻塞等待)")
                # warmup
                for _ in range(warm):
                    for k in order: time_run(str(builds[k]), op["args"], str(ipath), pin, capture=False)
                # paired alternating measurement
                times = {k: [] for k in order}
                for _ in range(runs):
                    for k in order:
                        dt, _ = time_run(str(builds[k]), op["args"], str(ipath), pin, capture=False)
                        times[k].append(dt)
                stat = {}
                for k in order:
                    tr = iqr_trim(times[k])
                    med = statistics.median(tr); mean = statistics.mean(tr)
                    cv = (statistics.stdev(tr)/mean) if len(tr) > 1 else 0.0
                    stat[k] = {"median_ms": med*1000, "min_ms": min(tr)*1000, "cv": cv, "n": len(tr)}
                worst_cv = max(stat[k]["cv"] for k in order)
                if worst_cv <= req_cv or attempt > cv_retries:
                    if worst_cv > req_cv:
                        print(f"  [gate ⚠] {tag}: CV {worst_cv*100:.1f}% > {req_cv*100:.0f}% "
                              f"重测 {attempt} 次仍超标,标记 unstable")
                    break
                print(f"  [gate] {tag}: CV {worst_cv*100:.1f}% > {req_cv*100:.0f}%,"
                      f"重测({attempt}/{cv_retries})", flush=True)
            cm = stat[order[0]]["median_ms"]; rm = stat[order[1]]["median_ms"]
            gap = (rm - cm)/cm*100
            cvflag = worst_cv > req_cv
            below_floor = bool(min_ms_floor and min(stat[k]["median_ms"] for k in order) < min_ms_floor)
            row = {"op":op["name"],"input":inp["name"],"size":inp.get("size",""),
                   **{f"{k}_ms":round(stat[k]["median_ms"],2) for k in order},
                   **{f"{k}_min_ms":round(stat[k]["min_ms"],2) for k in order},
                   **{f"{k}_cv":round(stat[k]["cv"],4) for k in order},
                   "gap_pct":round(gap,2),"cv_warn":cvflag,
                   "cv_gate":"ok" if not cvflag else "unstable","attempts":attempt,
                   "below_min_runtime":below_floor,"timed":True}
            warn = ("  ⚠cv" if cvflag else "") + ("  ⚠<floor" if below_floor else "")
            perfmsg = ""
            if do_perf:
                # RQ2: dynamic work (Inst ratio) × execution efficiency (CPI ratio),
                # measured in the SAME session right after the wall-clock pass.
                pc = {}
                for k in order:
                    r = perf_median(str(builds[k]), op["args"], str(ipath), pin, reps=perf_reps)
                    if r: pc[k] = {"inst": r[0], "cyc": r[1], "cpi": r[1]/r[0]}
                if len(pc) == 2:
                    c0, r1 = order[0], order[1]
                    inst_ratio = pc[r1]["inst"]/pc[c0]["inst"]
                    cpi_ratio  = pc[r1]["cpi"]/pc[c0]["cpi"]
                    row["inst_ratio"] = round(inst_ratio, 4)   # Raw/C Inst  (M1)
                    row["cpi_ratio"]  = round(cpi_ratio, 4)    # Raw/C CPI   (M2)
                    row["inst_x_cpi"] = round(inst_ratio*cpi_ratio, 4)  # M3: ≈ 1+gap
                    perfmsg = f"  |  Inst={inst_ratio:.3f} CPI={cpi_ratio:.3f} InstxCPI={inst_ratio*cpi_ratio:.3f}"
                    # M4: ΔCPI-TMA breakdown — only when CPI actually moved (trigger rule)
                    if abs(cpi_ratio - 1.0) >= 0.05:
                        tma = {k: toplev_tma(str(builds[k]), op["args"], str(ipath), pin) for k in order}
                        if all(tma.values()):
                            dcpi = {x: round(pc[r1]["cpi"]*tma[r1].get(x,0) - pc[c0]["cpi"]*tma[c0].get(x,0), 4)
                                    for x in ("Ret","FE","BS","Mem","Core")}
                            row["tma_c"] = {x: round(tma[c0].get(x,0),4) for x in ("Ret","FE","BS","Mem","Core")}
                            row["tma_raw"] = {x: round(tma[r1].get(x,0),4) for x in ("Ret","FE","BS","Mem","Core")}
                            row["dcpi"] = dcpi
                            dom = max(dcpi, key=lambda x: dcpi[x])
                            perfmsg += f"  ΔCPI[{dom}]={dcpi[dom]:+.3f}"
            results.append(row)
            print(f"  {op['name']:10s} {inp['name']:16s} {inp.get('size',''):>6s}  "
                  f"C={row.get(order[0]+'_ms')}ms  raw={row.get(order[1]+'_ms')}ms  "
                  f"gap={gap:+.2f}%{warn}{perfmsg}")
    return results

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("study_toml")
    ap.add_argument("--runs", type=int, default=None, help="override measure_runs")
    ap.add_argument("--ops", nargs="+", default=None, help="only these operations")
    ap.add_argument("--inputs", nargs="+", default=None, help="only these input names")
    ap.add_argument("--perf", action="store_true",
                    help="RQ2: after the wall-clock pass, measure perf counters "
                         "(Inst ratio, CPI ratio, ΔCPI-TMA) in the same session")
    ap.add_argument("--perf-reps", type=int, default=3,
                    help="perf-counter repetitions (median); bump for CPI-sensitive workloads")
    ap.add_argument("--max-load", type=float, default=None,
                    help="gate B: pre-measure 1-min load gate; over it -> warn (or block with "
                         "--wait-quiet). Default from [measure].max_load or 1.0")
    ap.add_argument("--cv-retries", type=int, default=None,
                    help="gate B: auto re-measure an op up to N times while any build's CV > "
                         "required_cv. Default from [measure].cv_retries or 2")
    ap.add_argument("--wait-quiet", action="store_true",
                    help="gate B: block until load <= max_load instead of measuring dirty")
    ap.add_argument("--out", default=None, help="write JSON results here")
    a = ap.parse_args()
    root = Path(__file__).resolve().parent.parent      # project root
    study = tomllib.loads((root / a.study_toml).read_text())
    runs = a.runs or study["measure"]["measure_runs"]
    m = study["measure"]
    max_load = a.max_load if a.max_load is not None else float(m.get("max_load", 1.0))
    cv_retries = a.cv_retries if a.cv_retries is not None else int(m.get("cv_retries", 2))
    print(f"== {study['study']['project']}  |  compare {study['study']['compare']}  "
          f"|  runs={runs}  pin=cpu{study['measure']['pin_cpu']}  "
          f"|  gateB max_load={max_load} cv_retries={cv_retries} load={os.getloadavg()[0]} ==")
    res = measure(root, study, runs, a.ops, a.inputs, do_perf=a.perf, perf_reps=a.perf_reps,
                  max_load=max_load, cv_retries=cv_retries, wait_quiet=a.wait_quiet)
                                                                          
                                                                
                                              
    out = Path(a.out) if a.out else (root / a.study_toml).parent / "scratch_run.json"
    out.write_text(json.dumps({"project":study["study"]["project"],"runs":runs,"results":res}, indent=2))
    print(f"== wrote {out} ==")

if __name__ == "__main__":
    main()
