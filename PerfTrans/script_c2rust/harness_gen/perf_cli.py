"""CLI: profile a generated harness to find per-op performance targets.

    python -m harness_gen.perf_cli --harness <harness_dir> \
        [--target-wall 1.5] [--pin]

Writes perf_workload.json next to the harness and prints a per-op table
(input, iters, wall, instructions, CPI, self_time_own, top hot fn).
"""

from __future__ import annotations

import argparse
import json
import logging
import sys
from pathlib import Path

_SCRIPT_ROOT = Path(__file__).resolve().parent.parent
if str(_SCRIPT_ROOT) not in sys.path:
    sys.path.insert(0, str(_SCRIPT_ROOT))

from harness_gen import perf_workload   # noqa: E402


def _load_spec(hdir: Path) -> dict:
    for cand in (hdir.parent / "logs" / "spec.json",
                 hdir / "logs" / "spec.json"):
        if cand.exists():
            return json.loads(cand.read_text())
    raise FileNotFoundError("spec.json not found next to the harness")


def main() -> int:
    ap = argparse.ArgumentParser(prog="harness_gen.perf_cli")
    ap.add_argument("--harness", required=True)
    ap.add_argument("--target-wall", type=float, default=1.5,
                    help="target steady-state wall time per op (s)")
    ap.add_argument("--pin", action="store_true",
                    help="pin CPU freq via perf_run.sh (for final wall-clock)")
    ap.add_argument("--ops", nargs="+", default=None,
                    help="restrict to these ops")
    ap.add_argument("-v", "--verbose", action="store_true")
    args = ap.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s", datefmt="%H:%M:%S")

    hdir = Path(args.harness)
    if not hdir.is_absolute() and not hdir.exists():
        try:
            from Config.paths import get_path
            root = get_path("PROJECT_ROOT")
            if root and (Path(root) / hdir).exists():
                hdir = Path(root) / hdir
        except Exception:
            pass
    hdir = hdir.resolve()

    spec = _load_spec(hdir)
    ops = args.ops or [op["name"] for op in spec["operations"]]

    results = perf_workload.profile_harness(
        hdir, ops, target_wall=args.target_wall, pin=args.pin)

    (hdir / "perf_workload.json").write_text(
        json.dumps([r.as_dict() for r in results], indent=2), encoding="utf-8")

    print(f"\n{'op':<24}{'src':<9}{'inB':>8}{'iters':>10}{'wall':>7}"
          f"{'self%':>7}{'crate%':>8}  top hot fn")
    print("-" * 98)
    clean = 0
    for r in results:
        if r.iters == 0:
            print(f"{r.op:<24}{'—':<9}{'':>8}{'':>10}{'':>7}{'':>7}{'':>8}  {r.note}")
            continue
        topfn = r.top[0][1] if r.top else ""
        flag = "" if r.ok else "  ⚠LOW"
        if r.harness_bound:
            flag += "  ⚠HARNESS-BOUND"
        clean += 1 if r.ok else 0
        print(f"{r.op:<24}{r.input_source[:8]:<9}{r.input_bytes:>8}"
              f"{r.iters:>10}{r.wall_s:>6.1f}s{100*r.self_time_own:>6.0f}%"
              f"{100*r.crate_share:>7.0f}%"
              f"  {topfn}{flag}")
    print("-" * 98)
    print(f"clean perf targets (self_time_own ≥ {perf_workload.SELF_TIME_GATE:.0%}): "
          f"{clean}/{sum(1 for r in results if r.iters)}")
    bound = [r.op for r in results if r.harness_bound]
    if bound:
        print(f"⚠️  HARNESS-BOUND (crate_share < "
              f"{perf_workload.CRATE_SHARE_GATE:.0%}): {', '.join(bound)}")
        print("   这些 op 量的是 harness 自己，不是库。"
              "把循环不变的 digest/校验重活移出计时循环。")
    print(f"→ {hdir/'perf_workload.json'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
