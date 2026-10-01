"""Which optimization rules fired, which were applied, which actually landed.

Reads `<project>/3_perf_opt/rewrites.log` — one JSON object per attempt — and
git history of the crate. Read-only.

Four counts per rule, and they mean different things. Reporting the wrong one
is the usual way this gets misread:

  fired      The dispatcher offered the rule for this function. A hit from
             hot_probe matched the card's pattern. Says nothing about whether
             the rule was any good here.

  applied    The model declared it applied, AND the F9 fingerprint check found
             the corresponding syntax in the output. Still NOT landed: the
             rewrite may have failed W1, regressed on W2, or been rolled back.

  committed  The attempt passed every gate and reached git. This is the only
             count that describes code in the final crate.

  skipped    The model declined with a reason. A reasoned decline is evidence
             about rule fit; `silently_dropped` (no reason given) is a
             contract violation and is counted separately.

The gap between `applied` and `committed` is the interesting one: it is how
often a rule produced a real rewrite that then failed to pay for itself.

**Bundles.** One attempt can apply several rules together, and W2 judges the
BUNDLE, not the rules in it. A rule's committed count therefore does not mean
that rule earned anything on its own — a drag rule can sink an earning one,
and vice versa. `solo` counts the attempts where a rule was the only one
applied; only those attribute a delta to a single rule. Everything else is
joint credit and is labelled as such.

Sub-strategies (`C3.S2`) are folded into their base (`C3`) for counting, and
listed under `variants`.

Usage:
    python -m tools.rule_usage lodepng
    python -m tools.rule_usage --all
    python -m tools.rule_usage lodepng --by-function
    python -m tools.rule_usage lodepng --dir 3_perf_opt.pre_fixes_20260827
    python -m tools.rule_usage --all --dir '3_perf_opt*'      # every rerun
    python -m tools.rule_usage --all --json
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable, Optional

REPO_ROOT = Path(__file__).resolve().parents[2]
DATASET = REPO_ROOT / "dataset_trans"

COMMITTED = "committed"


# The canonical card ids. Models annotate them in the header — a call site
# (`III③@snprintf`), the hits covered (`III④@21,22`), a partial application
# (`C3(partial:[4,5,6,9,10])`), a target (`C1_on_tree`). Every one of those is
# still the same card, and giving each its own row scatters a card's evidence
# across lines nobody reads. Anything that does not match keeps its own name
# rather than being forced into a bucket, so a genuinely new rule id shows up
# as itself instead of silently joining C1.
_RULE_ID_RE = re.compile(r"^(C\d+|III[①②③④]|II_[A-Za-z_]+)")


def _rule_base(rule: str) -> str:
    """`C3.S2` / `III③@snprintf` / `C1_on_tree` -> the card id."""
    rule = rule.strip()
    m = _RULE_ID_RE.match(rule)
    if m:
        return m.group(1)
    return rule.split(".", 1)[0].strip()


# A rule id is short and has no spaces: `C3`, `III④`, `II_const`, `C3.S2`.
# Older runs carry entries that are neither, because `parse_skipped_rules`
# used to split reasons on every `;` — including the ones the model wrote as
# English punctuation — and filed the trailing clause as a rule nobody
# offered. That parser has since been taught to only split at top level, but
# the logs it already wrote are still on disk, so a reader of those logs has
# to recognise the debris rather than tabulate it as rules.
_MAX_RULE_ID_LEN = 24


def _is_rule_id(rule: str) -> bool:
    rule = rule.strip()
    return bool(rule) and len(rule) <= _MAX_RULE_ID_LEN and not any(
        ch.isspace() for ch in rule)


def _load_attempts(log: Path) -> list[dict]:
    out: list[dict] = []
    for line in log.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            out.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    return out


def _committed_rules_from_git(crate: Path) -> list[str]:
    """Rule ids named in agent commit subjects — an independent cross-check.

    `rewrites.log` is written by the agent; git is written by the gate that
    accepted the commit. When the two disagree, the run ended between the
    commit and the log flush, and git is the one that reflects the crate.
    """
    if not (crate / ".git").exists():
        return []
    try:
        out = subprocess.run(
            ["git", "-C", str(crate), "log", "--format=%s"],
            capture_output=True, text=True, timeout=30).stdout
    except (OSError, subprocess.SubprocessError):
        return []
    rules: list[str] = []
    for subject in out.splitlines():
        if "agent changeset" not in subject or ":" not in subject:
            continue
        rules.extend(r.strip() for r in subject.rsplit(":", 1)[1].split(",")
                     if r.strip())
    return rules


class RuleStats:
    __slots__ = ("fired", "applied", "committed", "solo", "skipped",
                 "dropped", "variants", "fns", "solo_deltas", "bundle_deltas")

    def __init__(self) -> None:
        self.fired = 0
        self.applied = 0
        self.committed = 0
        self.solo = 0
        self.skipped = 0
        self.dropped = 0
        self.variants: set[str] = set()
        self.fns: dict[str, int] = defaultdict(int)
        self.solo_deltas: list[float] = []
        self.bundle_deltas: list[float] = []


def _plan_abstentions(rec: dict) -> list[tuple[str, str]]:
    """Rules the PLAN/EXECUTE path declined, as (rule, reason).

    That path files its reasons in `plan_json.abstained_rules` and leaves
    `skipped_rules` null; every other path uses `skipped_rules`. Reading only
    the latter drops these attempts out of the tally entirely — they raise
    `fired` and then land in neither `applied` nor `skipped`, so a rule that
    was declined with a reason reads as a rule nobody decided anything about.

    Measured, zopfli 2026-08-27: `BoundaryPM` (37.68% self) declined C3 and
    III④ with a paragraph of reasoning each, and neither reached this tally.
    """
    plan = rec.get("plan_json")
    if not isinstance(plan, dict):
        return []
    out: list[tuple[str, str]] = []
    for entry in (plan.get("abstained_rules") or []):
        if not isinstance(entry, dict):
            continue
        rule = str(entry.get("rule") or "").strip()
        if not rule or not _is_rule_id(rule):
            continue
        out.append((rule, str(entry.get("reason") or "").strip()))
    return out


def collect(attempts: Iterable[dict]) -> tuple[dict[str, RuleStats], int]:
    """Per-rule counts, plus the number of entries that were not rule ids."""
    stats: dict[str, RuleStats] = defaultdict(RuleStats)
    malformed = 0
    for rec in attempts:
        fn = rec.get("fn_name") or "?"
        terminal = rec.get("terminal_status")
        delta = rec.get("w2_delta_pct")
        applied = [r for r in (rec.get("applied_rules") or []) if r]
        applied_bases = {_rule_base(r) for r in applied}

        for rule in (rec.get("fired_rules") or []):
            if _is_rule_id(str(rule)):
                stats[_rule_base(rule)].fired += 1
            elif rule:
                malformed += 1

        for rule in applied:
            if not _is_rule_id(str(rule)):
                malformed += 1
                continue
            st = stats[_rule_base(rule)]
            st.applied += 1
            st.variants.add(rule)
            if terminal == COMMITTED:
                st.committed += 1
                st.fns[fn] += 1
                if len(applied_bases) == 1:
                    st.solo += 1
                    if isinstance(delta, (int, float)):
                        st.solo_deltas.append(float(delta))
                elif isinstance(delta, (int, float)):
                    st.bundle_deltas.append(float(delta))

        # Only what `skipped_rules` already counted. NOT `applied_bases`:
        # a rule can be applied at some hits and declined at others in one
        # attempt — measured, 74 of 741 records across 8 projects, in forms
        # like `C3(partial:[4,5,6])` and `III④@21,22`. Seeding this with the
        # applied rules would make the plan path silently drop a decline that
        # the `skipped_rules` path counts.
        accounted: set[str] = set()
        for pair in (rec.get("skipped_rules") or []):
            if not isinstance(pair, (list, tuple)) or len(pair) != 2:
                continue
            rule, reason = pair
            if not _is_rule_id(str(rule)):
                if rule:
                    malformed += 1
                continue
            base = _rule_base(str(rule))
            accounted.add(base)
            st = stats[base]
            if reason == "silently_dropped":
                st.dropped += 1
            else:
                st.skipped += 1

        for rule, _reason in _plan_abstentions(rec):
            base = _rule_base(rule)
            if base in accounted:
                continue
            accounted.add(base)
            stats[base].skipped += 1
    return stats, malformed


def _project_report(name: str, opt_dir: Path) -> Optional[dict[str, Any]]:
    log = opt_dir / "rewrites.log"
    if not log.exists():
        return None
    attempts = _load_attempts(log)
    stats, malformed = collect(attempts)
    git_rules = _committed_rules_from_git(opt_dir / "crate")
    git_committed: dict[str, int] = defaultdict(int)
    for r in git_rules:
        git_committed[_rule_base(r)] += 1

    final = opt_dir / "final_measurement.json"
    aggregate = None
    if final.exists():
        try:
            aggregate = json.loads(final.read_text()).get("aggregate_mean_pct")
        except (OSError, json.JSONDecodeError):
            aggregate = None

    return {
        "project": name,
        "opt_dir": str(opt_dir),
        "attempts": len(attempts),
        "aggregate_mean_pct": aggregate,
        "malformed_rule_entries": malformed,
        "rules": {
            rule: {
                "fired": s.fired,
                "applied": s.applied,
                "committed": s.committed,
                "committed_git": git_committed.get(rule, 0),
                "solo_committed": s.solo,
                "skipped_with_reason": s.skipped,
                "silently_dropped": s.dropped,
                "variants": sorted(s.variants),
                "functions": dict(sorted(s.fns.items(),
                                         key=lambda kv: -kv[1])),
                "solo_deltas_pct": s.solo_deltas,
                "bundle_deltas_pct": s.bundle_deltas,
            }
            for rule, s in sorted(stats.items())
        },
    }


def _fmt_deltas(values: list[float]) -> str:
    if not values:
        return "-"
    return "/".join(f"{v:+.2f}" for v in sorted(values)[:3]) + (
        " …" if len(values) > 3 else "")


def _print_report(rep: dict[str, Any], by_function: bool) -> None:
    agg = rep["aggregate_mean_pct"]
    agg_s = f"{agg:+.3f}%" if isinstance(agg, (int, float)) else "unmeasured"
    print(f"\n{'=' * 78}")
    debris = rep.get("malformed_rule_entries") or 0
    debris_s = f"   ⚠{debris} non-rule entries (pre-fix log)" if debris else ""
    run = Path(rep["opt_dir"]).name
    label = rep["project"] if run == "3_perf_opt" else f"{rep['project']}  [{run}]"
    print(f"{label}   attempts={rep['attempts']}   "
          f"aggregate={agg_s}{debris_s}")
    print(f"{'=' * 78}")
    if not rep["rules"]:
        print("  (no rules recorded)")
        return
    print(f"{'rule':<10} {'fired':>6} {'applied':>8} {'commit':>7} "
          f"{'solo':>5} {'skip':>5} {'drop':>5}  {'solo Δ% (committed)':<24}")
    print("-" * 78)
    for rule, s in rep["rules"].items():
        mismatch = ""
        if s["committed_git"] and s["committed_git"] != s["committed"]:
            mismatch = f"  ⚠git={s['committed_git']}"
        print(f"{rule:<10} {s['fired']:>6} {s['applied']:>8} "
              f"{s['committed']:>7} {s['solo_committed']:>5} "
              f"{s['skipped_with_reason']:>5} {s['silently_dropped']:>5}  "
              f"{_fmt_deltas(s['solo_deltas_pct']):<24}{mismatch}")
        if by_function and s["functions"]:
            for fn, n in s["functions"].items():
                print(f"           └─ {fn} ×{n}")


def _iter_projects(names: list[str], subdir: str):
    """(project, run_dir) pairs. `subdir` may be a glob to span reruns.

    Most projects keep only their latest `3_perf_opt`; earlier rounds are
    renamed (`3_perf_opt.pre_0825`, `.run2_before_w2fix`) because the driver
    deletes the directory it is about to rebuild. `--dir '3_perf_opt*'`
    therefore reads the whole history rather than whatever survived last.
    """
    roots = ([DATASET / n for n in names] if names
             else sorted(p for p in DATASET.iterdir() if p.is_dir()))
    for root in roots:
        if not root.is_dir():
            continue
        for run in sorted(root.glob(subdir)):
            if (run / "rewrites.log").exists():
                yield root.name, run


def main(argv: Optional[list[str]] = None) -> int:
    ap = argparse.ArgumentParser(
        description="Rule fire/apply/commit counts per project.")
    ap.add_argument("projects", nargs="*", help="project names (default: --all)")
    ap.add_argument("--all", action="store_true", help="every project with a run")
    ap.add_argument("--dir", default="3_perf_opt",
                    help="run directory under the project (default: 3_perf_opt)")
    ap.add_argument("--by-function", action="store_true",
                    help="list the functions each rule landed in")
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    args = ap.parse_args(argv)

    if not args.projects and not args.all:
        ap.error("give project names or --all")

    reports = []
    for name, opt_dir in _iter_projects(args.projects, args.dir):
        rep = _project_report(name, opt_dir)
        if rep is None:
            print(f"[skip] {name}: no {args.dir}/rewrites.log", file=sys.stderr)
            continue
        reports.append(rep)

    if args.json:
        json.dump(reports, sys.stdout, indent=2, ensure_ascii=False)
        print()
        return 0

    for rep in reports:
        _print_report(rep, args.by_function)

    if len(reports) > 1:
        totals: dict[str, RuleStats] = defaultdict(RuleStats)
        for rep in reports:
            for rule, s in rep["rules"].items():
                t = totals[rule]
                t.fired += s["fired"]
                t.applied += s["applied"]
                t.committed += s["committed"]
                t.solo += s["solo_committed"]
                t.skipped += s["skipped_with_reason"]
                t.dropped += s["silently_dropped"]
        print(f"\n{'=' * 78}")
        print(f"ALL {len(reports)} PROJECTS")
        print(f"{'=' * 78}")
        print(f"{'rule':<10} {'fired':>6} {'applied':>8} {'commit':>7} "
              f"{'solo':>5} {'skip':>5} {'drop':>5}   {'projects':>8}")
        print("-" * 78)
        for rule, t in sorted(totals.items(),
                              key=lambda kv: -kv[1].committed):
            n_proj = sum(1 for r in reports
                         if r["rules"].get(rule, {}).get("committed"))
            print(f"{rule:<10} {t.fired:>6} {t.applied:>8} {t.committed:>7} "
                  f"{t.solo:>5} {t.skipped:>5} {t.dropped:>5}   {n_proj:>8}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
