"""CLI entry — `python -m perf_opt.hot_probe.class_III <crate> [options]`

Answer to: "given a rust project, which fns are Class III optimization
candidates and where?"

Input:  path to c2rust output crate root (contains src/ + Cargo.toml)
Output: markdown report on stdout (or --out file)
        + optional raw class_III_hits.json (via --json DIR)

The scan runs with hot_fns=None by default — the whole project is the
candidate pool. Pass --hot-fns-file only if you want profiler-driven
pre-pruning.
"""
from __future__ import annotations

import argparse
import logging
import sys
from pathlib import Path

from perf_opt.hot_probe.class_III.report import format_report
from perf_opt.hot_probe.class_III.scan import scan


def _infer_project_name(crate: Path, override: str | None) -> str:
    """Infer a human-readable project name from the crate.

    Precedence:
      1. `--project-name` CLI override (explicit user intent wins)
      2. `Cargo.toml [package] name` (the crate's own identity — the
         canonical, tool-agnostic source of truth for any rust crate)
      3. `crate.name` (last-resort fallback when Cargo.toml is missing or
         has no [package] section)

    Note (why not walk up the path): earlier drafts hardcoded a
    directory-name convention (`<proj>/<stage>/crate`) which was a
    project-layout overfit. Cargo.toml is the correct signal.
    """
    if override:
        return override
    cargo = crate / "Cargo.toml"
    if cargo.is_file():
        try:
            in_package = False
            for raw in cargo.read_text().splitlines():
                line = raw.strip()
                if line.startswith("["):
                    in_package = line == "[package]"
                    continue
                if not in_package:
                    continue
                if line.startswith("name") and "=" in line:
                    val = line.split("=", 1)[1].strip()
                    # Strip surrounding quotes and trailing comments
                    if "#" in val:
                        val = val.split("#", 1)[0].strip()
                    val = val.strip('"\'')
                    if val:
                        return val
        except OSError:
            pass
    return crate.name


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="python -m perf_opt.hot_probe.class_III",
        description=(
            "Class III optimization candidate scanner. "
            "Given a c2rust rust project, prints a markdown report of "
            "candidate fns per rule (III①/②/③/④). "
            "No profiler / no workload needed — pure source scan."
        ),
    )
    parser.add_argument(
        "crate",
        type=Path,
        help="Path to c2rust output crate root (contains src/ + Cargo.toml)",
    )
    parser.add_argument(
        "--out", type=Path, default=None,
        help="Write markdown report to file instead of stdout",
    )
    parser.add_argument(
        "--json-dir", type=Path, default=None,
        help="Also write raw class_III_hits.json to this dir (for driver "
             "consumption)",
    )
    parser.add_argument(
        "--top-n", type=int, default=15,
        help="Top-N candidates per rule to display (default: 15)",
    )
    parser.add_argument(
        "--hot-fns-file", type=Path, default=None,
        help="Optional: file with one FnKey or base name per line; "
             "restricts candidate pool to those fns (SPEC §10). "
             "Default: no pre-filter (whole project is the pool).",
    )
    parser.add_argument(
        "--project-name", type=str, default=None,
        help="Override the project name shown in the report header. "
             "Default: inferred from `crate` path — for c2rust layout "
             "`<proj>/<stage>/crate`, uses `<proj>`; else uses `crate.name`.",
    )
    parser.add_argument(
        "-v", "--verbose", action="store_true",
        help="Log scan progress to stderr",
    )
    args = parser.parse_args()

    logging.basicConfig(
        level=logging.INFO if args.verbose else logging.WARNING,
        format="%(asctime)s %(levelname)s %(message)s",
        datefmt="%H:%M:%S",
        stream=sys.stderr,
    )

    if not args.crate.is_dir():
        sys.stderr.write(f"Error: crate dir does not exist: {args.crate}\n")
        sys.exit(2)

    hot_fns: set[str] | None = None
    if args.hot_fns_file is not None:
        if not args.hot_fns_file.is_file():
            sys.stderr.write(
                f"Error: --hot-fns-file does not exist: {args.hot_fns_file}\n"
            )
            sys.exit(2)
        hot_fns = {
            line.strip()
            for line in args.hot_fns_file.read_text().splitlines()
            if line.strip() and not line.strip().startswith("#")
        }
        sys.stderr.write(
            f"[hot_fns] restricted candidate pool to {len(hot_fns)} entries "
            f"from {args.hot_fns_file}\n"
        )

    result = scan(
        crate=args.crate,
        hot_fns=hot_fns,
        out_dir=args.json_dir,
    )

    project_name = _infer_project_name(args.crate, args.project_name)
    md = format_report(result, project_name=project_name, top_n=args.top_n)

    if args.out is not None:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(md)
        sys.stderr.write(f"[write] {args.out}\n")
    else:
        sys.stdout.write(md)


if __name__ == "__main__":
    main()
