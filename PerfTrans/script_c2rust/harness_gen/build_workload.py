""                                                        

                                                
                                                       
                                    
                                              

                              

                                                                    
                                                                   
                                                                  

                                                           
                                                         
                                  

   
                                                                                                         
   
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from datetime import datetime
from pathlib import Path

_SCRIPT_ROOT = Path(__file__).resolve().parent.parent   # script_c2rust

MANIFEST_NAME = "workload_manifest.json"


def _run_step(mod: str, args: list[str]) -> tuple[int, str]:
    ""                                                             
                                                            
    cmd = [sys.executable, "-m", f"harness_gen.{mod}", *args]
    print(f"\n=================== [{mod}] ===================", flush=True)
    print(datetime.now().strftime("%H:%M:%S") + "  $ " + " ".join(cmd), flush=True)
                              
    proc = subprocess.Popen(
        cmd, cwd=str(_SCRIPT_ROOT),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
    )
    captured: list[str] = []
    assert proc.stdout is not None
    for line in proc.stdout:
        sys.stdout.write(line)
        sys.stdout.flush()
        captured.append(line)
    rc = proc.wait()
    return rc, "".join(captured)


def _refine_verdict(text: str) -> str:
    ""                                                
    if "nothing to refine" in text or "GAP (corpus reaches, gen_perf misses): 0" in text:
        return "skip_gap0"
    if "ACCEPT" in text:
        return "accepted"
    if "revert" in text.lower() or "REVERT" in text:
        return "reverted"
    return "ran"


def build_workload(harness: str, *, seconds_per_op: int | None = None,
                   ops: list[str] | None = None) -> int:
    hdir = Path(harness)
    hdir_abs = hdir if hdir.is_absolute() else (_SCRIPT_ROOT / hdir)
    if not hdir_abs.is_dir():
        print(f"!!! harness dir 不存在: {hdir_abs}", file=sys.stderr)
        return 2

    corpus_args = ["--harness", harness]
    if seconds_per_op is not None:
        corpus_args += ["--seconds-per-op", str(seconds_per_op)]
    perf_args = ["--harness", harness]
    if ops:
        perf_args += ["--ops", *ops]

                            
    rc_corpus, _ = _run_step("corpus_cli", corpus_args)
    print(f"corpus_cli rc={rc_corpus}", flush=True)
    if rc_corpus != 0:
        print("!!! corpus_cli FAILED, 中止", file=sys.stderr)
        return 1

                                                    
    rc_refine, refine_out = _run_step("perf_refine", ["--harness", harness])
    verdict = _refine_verdict(refine_out)
    print(f"perf_refine rc={rc_refine} verdict={verdict} (非致命,继续)", flush=True)

                          
    rc_perf, _ = _run_step("perf_cli", perf_args)
    print(f"perf_cli rc={rc_perf}", flush=True)
    if rc_perf != 0:
        print("!!! perf_cli FAILED", file=sys.stderr)
        return 1

    harness_bound = _harness_bound_ops(hdir_abs)

                                                          
    manifest = {
        "built_at": time.time(),
        "built_at_iso": datetime.now().isoformat(timespec="seconds"),
        "harness": str(hdir_abs),
        "steps": {
            "corpus_cli": {"rc": rc_corpus},
            "perf_refine": {"rc": rc_refine, "verdict": verdict},
            "perf_cli": {"rc": rc_perf},
        },
        "refine_ran": True,                                    
                                                 
        "harness_bound_ops": [op for op, _ in harness_bound],
    }
    mpath = hdir_abs / MANIFEST_NAME
    mpath.write_text(json.dumps(manifest, indent=2), encoding="utf-8")

    print("\n=================== DONE ===================", flush=True)
    print(datetime.now().strftime("%H:%M:%S"), flush=True)
    print(f"manifest → {mpath}", flush=True)
    print(f"refine verdict = {verdict}", flush=True)
    for f in ("golden.jsonl", "perf_workload.json"):
        p = hdir_abs / f
        print(f"  {f}: {'OK' if p.exists() else 'MISSING'}", flush=True)
    pinputs = hdir_abs / "perf_inputs"
    n = len(list(pinputs.glob("*.perf.bin"))) if pinputs.is_dir() else 0
    print(f"  perf_inputs/*.perf.bin: {n}", flush=True)

    if harness_bound:
                                                 
                                            
        print("\n  ⚠️  库占比低的 op(仍会进 hot_probe,由热点函数裁决):",
              flush=True)
        for op, row in harness_bound:
            rows = row.get("top_functions") or []
            top = rows[0][1] if rows and len(rows[0]) > 1 else ""
            crate = row.get("crate_share", 0)
            harn = row.get("harness_share", 0)
            print(f"     {op}: crate_share={crate:.0%} harness={harn:.0%}  "
                  f"top={top}", flush=True)
            print(f"       → {_low_share_hint(row, top)}", flush=True)
    return 0


def _low_share_hint(row: dict, top: str) -> str:
    ""           

                                                   
                                              
                                  
       
    top = (top or "").strip()
    if row.get("attribution_reliable") is False:
        return ("归因不可靠——二进制缺被测 crate 的调试信息,内联进 harness 的"
                "库函数被算到了 harness 头上。重建时带 [profile.release] debug = 2。")
    if row.get("harness_share", 0) >= 0.20:
        return ("harness 自耗:把**循环不变**的 digest/校验重活移出计时循环"
                "(整块输入 checksum、子串搜索、每迭代 clone 结构体);"
                "改动须逐 iter digest 不变,拿全量 golden 复验。")
    if top and not top.startswith(("0x", "[")) and "::" not in top:
        return (f"时间在进程外({top} 属 libc/内核)——分配器或 syscall 密集的 op "
                "不是靠改 harness 变成性能目标的;这多半是被测库本身的形态。")
    if top and "::" in top and "_harness::" not in top:
        return (f"最热符号 {top} 就是**库代码**,该 op 其实在跑库;"
                "缺口是它周围的 libc/分配器开销,通常换更大更密的输入即可。")
    return "库计算占比低;交给 hot_probe 判定是否仍有可优化热点。"


def _harness_bound_ops(hdir_abs: Path) -> list[tuple[str, dict]]:
    ""                                                            
    try:
        rows = json.loads(
            (hdir_abs / "perf_workload.json").read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return []
    if not isinstance(rows, list):
        return []
    return [
        (row.get("op", "?"), row)
        for row in rows
        if isinstance(row, dict) and row.get("harness_bound")
    ]


def main() -> int:
    ap = argparse.ArgumentParser(
        prog="harness_gen.build_workload",
        description="串死 workload 三步(corpus_cli → perf_refine → perf_cli),"
                    "perf_refine 漏不掉。")
    ap.add_argument("--harness", required=True,
                    help="generated harness crate dir(绝对,或相对 script_c2rust)")
    ap.add_argument("--seconds-per-op", type=int, default=None,
                    help="透传给 corpus_cli 的 fuzz 时长")
    ap.add_argument("--ops", nargs="+", default=None,
                    help="透传给 perf_cli 的 op 白名单(默认全部注册 op)")
    args = ap.parse_args()
    return build_workload(args.harness, seconds_per_op=args.seconds_per_op,
                          ops=args.ops)


if __name__ == "__main__":
    sys.exit(main())
