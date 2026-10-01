"""CLI: build a validated corpus + golden oracle for a generated harness,
then self-check it by replaying the baseline against its own golden.

    python -m harness_gen.corpus_cli --harness <harness_dir> \
        [--seconds-per-op 600] [--backend auto|libfuzzer|pymut] \
        [--per-op-cap 300] [--import <dir>]

<harness_dir> is the crate dir produced by harness_gen (contains
Cargo.toml with the `harness` bin + the harness lib). The baseline binary
is that crate's release build of `harness`.
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

from harness_gen import corpus, fuzz, replay          # noqa: E402
from harness_gen.gates import run_cmd, BUILD_TIMEOUT   # noqa: E402

logger = logging.getLogger(__name__)


def _load_spec(harness_dir: Path) -> dict:
    # logs/spec.json lives at the harness_gen out_root (parent of the crate)
    for cand in (harness_dir.parent / "logs" / "spec.json",
                 harness_dir / "logs" / "spec.json"):
        if cand.exists():
            return json.loads(cand.read_text(encoding="utf-8"))
    raise FileNotFoundError("spec.json not found next to the harness")


def _build_release(harness_dir: Path) -> Path:
    rc, _o, err = run_cmd(["cargo", "build", "--release", "--bin", "harness"],
                          harness_dir, BUILD_TIMEOUT)
    if rc != 0:
        raise RuntimeError(f"baseline release build failed:\n{err[-3000:]}")
    return harness_dir / "target" / "release" / "harness"


def _import_dir(root: Path, ops: list[str]) -> dict[str, list[Path]]:
    """--import <dir>: adopt external inputs. Layout <dir>/<op>/* is
    honoured; a flat dir applies every file to every op."""
    out: dict[str, list[Path]] = {op: [] for op in ops}
    per_op = [d for d in root.iterdir() if d.is_dir() and d.name in ops]
    if per_op:
        for d in per_op:
            out[d.name] = [p for p in d.rglob("*") if p.is_file()]
    else:
        flat = [p for p in root.rglob("*") if p.is_file()]
        for op in ops:
            out[op] = list(flat)
    return out


def main() -> int:
    ap = argparse.ArgumentParser(prog="harness_gen.corpus_cli")
    ap.add_argument("--harness", required=True, help="generated harness crate dir")
    ap.add_argument("--seconds-per-op", type=int, default=600)
    ap.add_argument("--backend", default="auto",
                    choices=["auto", "libfuzzer", "pymut"])
    ap.add_argument("--per-op-cap", type=int, default=300)
    ap.add_argument("--import", dest="import_dir", default=None,
                    help="also fold in external inputs from this dir")
    ap.add_argument("-v", "--verbose", action="store_true")
    args = ap.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s", datefmt="%H:%M:%S")

    harness_dir = Path(args.harness)
    if not harness_dir.is_absolute() and not harness_dir.exists():
        try:
            from Config.paths import get_path
            root = get_path("PROJECT_ROOT")
            if root and (Path(root) / harness_dir).exists():
                harness_dir = Path(root) / harness_dir
        except Exception:
            pass
    harness_dir = harness_dir.resolve()
    spec = _load_spec(harness_dir)
    harness_name = spec["harness_name"]
    ops = [op["name"] for op in spec["operations"]]

    logger.info("[corpus] building baseline release binary")
    baseline = _build_release(harness_dir)

    # ensure seeds exist (gen-seeds is idempotent + deterministic)
    seeds_dir = harness_dir / "seeds"
    run_cmd([str(baseline), "gen-seeds", str(seeds_dir)], harness_dir, 300)

    cfg = fuzz.FuzzConfig(seconds_per_op=args.seconds_per_op,
                          backend=args.backend)
    logger.info("[corpus] producing candidates (backend=%s, %ds/op)",
                args.backend, args.seconds_per_op)
    candidates, backend = fuzz.produce_candidates(
        harness_dir, harness_name, spec, baseline, cfg)
    logger.info("[corpus] backend used: %s", backend)

    # always fold in the deterministic seeds
    for op, paths in corpus.seed_candidates(harness_dir, ops).items():
        candidates.setdefault(op, [])
        candidates[op].extend(paths)
    if args.import_dir:
        for op, paths in _import_dir(Path(args.import_dir).resolve(), ops).items():
            candidates.setdefault(op, [])
            candidates[op].extend(paths)

    logger.info("[corpus] curating against baseline")
    golden, stats = corpus.curate(harness_dir, baseline, candidates,
                                  per_op_cap=args.per_op_cap)

    # self-check: baseline MUST match its own golden exactly
    logger.info("[corpus] self-check: replay baseline vs golden")
    rep = replay.replay(harness_dir, baseline, harness_dir / "golden.jsonl")

    print(f"backend           : {backend}")
    print(f"candidates seen   : {stats.total_candidates}")
    print(f"corpus kept       : {stats.as_dict()['kept_total']}  "
          f"{stats.per_op_kept}")
    print(f"rejected crash/nd : {stats.rejected_crash}  dup: {stats.rejected_dup}")
    print(f"golden rows       : {len(golden)}")
    print(f"self-check        : {'PASS' if rep.ok else 'FAIL'} "
          f"({rep.matched}/{rep.total} matched)")
    if not rep.ok:
        print(f"  mismatches={len(rep.mismatches)} errors={len(rep.errors)}",
              file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
