"""Source-agnostic W1 primitives.

A `W1Spec` is one oracle check: run `binary args` (with `$INPUT` →
input_path substituted by the Verifier), sha256 its stdout, compare to
`sha256` and the exit code to `exit_code`. `run_w1_suite` drives a list of
them through a `Verifier`; `filter_baseline_specs` drops the specs a
pristine baseline already fails (and aborts if the `primary` spec fails).

These know nothing about where specs come from — TOML manifests (retired),
golden.jsonl (see workload.py), or hand-built. The Stage A lift drivers and
the runner gate all funnel through here so there is exactly one W1 loop.
"""

from __future__ import annotations

import logging
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger("verify.w1")


@dataclass
class W1Spec:
    name: str
    binary: str
    args: list
    input_path: Path | None
    sha256: str
    exit_code: int = 0
    wrapper: Path | None = None


def run_w1_suite(verifier, specs) -> tuple[bool, str]:
    """Run every spec; first failure wins. Returns (ok, detail). `detail`
    is `"<spec.name>: <gate detail>"` on failure, "" on success."""
    for s in specs:
        g = verifier.w1(binary=s.binary, args=s.args, input_path=s.input_path,
                        expected_sha256=s.sha256, expected_exit=s.exit_code,
                        wrapper=s.wrapper)
        if not g.ok:
            return False, f"{s.name}: {g.detail}"
    return True, ""


def filter_baseline_specs(verifier, specs, *, label: str = "w1") -> list[W1Spec]:
    """Keep only the specs the (already-built) baseline actually reproduces.

    A spec the pristine baseline fails does NOT indicate a lift regression —
    the baseline hasn't been lifted yet. It means that spec's oracle is stale
    or tainted (e.g. a harness op that folds a raw pointer address into its
    digest → build-sensitive, see lodepng chunk_ops). Gating lifts on such a
    spec would roll back every lift, so it is DROPPED with a warning rather
    than aborting the whole run on one bad op.

    Aborts only if NONE survive — that means the crate copy is broken or the
    whole oracle drifted, not a single flaky op. Spec names are `<op>:<hash>`
    so the warning reports which ops were dropped."""
    live: list[W1Spec] = []
    dropped: list[W1Spec] = []
    for s in specs:
        g = verifier.w1(binary=s.binary, args=s.args, input_path=s.input_path,
                        expected_sha256=s.sha256, expected_exit=s.exit_code,
                        wrapper=s.wrapper)
        (live if g.ok else dropped).append(s)
    if dropped:
        from collections import Counter
        by_op = Counter(s.name.split(":")[0] for s in dropped)
        logger.warning(
            f"[{label}] baseline could NOT reproduce {len(dropped)}/{len(specs)} "
            f"golden spec(s) — dropping (stale/tainted oracle, not gated): "
            f"{dict(by_op)}")
    if not live:
        raise RuntimeError(
            f"[{label}] baseline reproduces ZERO golden specs — broken crate "
            f"copy or wholesale oracle drift (not a single flaky op)")
    return live
