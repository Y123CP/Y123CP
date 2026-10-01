"""CLI: python -m harness_gen --project dataset_trans_process/lodepng/0_raw

Run from the script_c2rust directory (so Config/ and utils/ import), on
the 37.2 compute host (cargo + llvm-cov-17 live there).
"""

from __future__ import annotations

import argparse
import logging
import sys
from pathlib import Path

# Make Config/ and utils/ importable regardless of CWD.
_SCRIPT_ROOT = Path(__file__).resolve().parent.parent
if str(_SCRIPT_ROOT) not in sys.path:
    sys.path.insert(0, str(_SCRIPT_ROOT))

from harness_gen.agent import HarnessGenAgent  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(
        prog="harness_gen",
        description="Generate a workload harness for a c2rust-translated crate")
    ap.add_argument("--project", required=True,
                    help="crate dir, e.g. dataset_trans_process/lodepng/0_raw "
                         "(relative paths resolve against PROJECT_ROOT, then CWD)")
    ap.add_argument("--out", default=None,
                    help="output root (default: <project>/../workloads/harness_gen)")
    ap.add_argument("--model", default=None,
                    help="override DEFAULT_LLM_MODEL from Config/paths.conf")
    ap.add_argument("--min-line-cov", type=float, default=60.0)
    ap.add_argument("--min-fn-cov", type=float, default=70.0)
    ap.add_argument("--max-repairs", type=int, default=6)
    ap.add_argument("--cov-rounds", type=int, default=2,
                    help="max LLM coverage-extension rounds")
    ap.add_argument("--skip-cov", action="store_true",
                    help="skip the coverage gate (build + smoke only)")
    ap.add_argument("-v", "--verbose", action="store_true")
    args = ap.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s", datefmt="%H:%M:%S")

    project = Path(args.project)
    if not project.is_absolute():
        try:
            from Config.paths import get_path
            root = get_path("PROJECT_ROOT")
            if root and (Path(root) / project).exists():
                project = Path(root) / project
        except Exception:
            pass
    project = project.resolve()
    if not (project / "Cargo.toml").exists():
        print(f"error: {project} is not a crate (no Cargo.toml)", file=sys.stderr)
        return 2

    agent = HarnessGenAgent(
        project_dir=project,
        out_root=Path(args.out) if args.out else None,
        model=args.model,
        min_line=args.min_line_cov, min_fn=args.min_fn_cov,
        max_repairs=args.max_repairs, cov_rounds=args.cov_rounds,
        skip_cov=args.skip_cov)
    report = agent.run()

    if report.ok:
        print(f"OK harness at {report.harness_dir}")
        print(f"   operations : {[op['name'] for op in report.spec['operations']]}")
        if not args.skip_cov:
            print(f"   coverage   : line {report.line_pct:.1f}%  "
                  f"fn {report.fn_pct:.1f}%  "
                  f"(uncovered API fns: {len(report.uncovered or [])})")
        print(f"   llm calls  : {report.llm_calls}")
        return 0
    print(f"FAILED: {report.failure}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
