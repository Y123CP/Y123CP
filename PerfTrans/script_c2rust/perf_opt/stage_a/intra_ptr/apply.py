"""Stage A intra_ptr — apply chain (P2a).

The first milestone that actually REWRITES code. For each airtight lift unit
(P1 PLAN), deterministically rewrite the function body (rewrite.py), then
verify cargo check + cargo build + W1 sha256, keeping or rolling back the
unit atomically. Purpose: validate the rewrite→compile→W1→rollback chain
end-to-end before adding the LLM tier (P2b).

Self-bin projects only for now (bzip2-style). Harness projects (lib + driver
crate) are a TODO — the W1 build/run there needs the harness wiring that
main.py:_stage_a sets up.

Usage:
    python -m perf_opt.stage_a.intra_ptr.apply <cleaned_dir> <manifest.toml>
        [--work-root DIR] [--only fnA fnB ...] [--broad] [--keep]
"""

from __future__ import annotations

import argparse
import logging
import shutil
import tomllib
from dataclasses import dataclass
from pathlib import Path

from perf_opt.bench_pipeline import (
    _ensure_fair_build, _ensure_harness_compat, _git_init,
    _rewire_path_dep, _rsync_copy,
)
from perf_opt.verify.cargo import Verifier
from .plan import plan_crate
from .rewrite import rewrite_fn_in_file
from .collect import collect_crate

logger = logging.getLogger("intra_ptr.apply")


@dataclass
class UnitResult:
    fn_name: str
    anchors: list[str]
    status: str          # keep | reject_check | reject_build | reject_w1 | no_edits
    detail: str = ""


def _count_raw_derefs(crate: Path) -> int:
    """Cheap safety metric: total raw deref/offset use-sites across all
    pointer params (the surface the lift removes)."""
    return sum(
        len([u for u in f.use_sites
             if u.kind in ("offset_read", "offset_write",
                           "deref_read", "deref_write",
                           "field_read", "field_write")])
        for f in collect_crate(crate)
    )


def run_floor(cleaned_dir: Path, manifest: Path | None, work_root: Path, *,
              only_fns: set[str] | None = None,
              airtight: bool = True, keep: bool = False,
              check_only: bool = False) -> list[UnitResult]:
    """`check_only`: gate on cargo check + build only, skip W1. Lets the
    rewriter be validated on harness/lib-only crates (and broadly across
    projects) without each project's W1 harness. Self-bin + real oracle →
    omit check_only for full behavioural (W1) validation."""
    cleaned_dir = cleaned_dir.resolve()
    _w1 = None
    if not check_only:
        if manifest is None:
            raise RuntimeError("W1 mode needs a manifest (or pass check_only)")
        manifest = manifest.resolve()
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        workload = data.get("workload", {})
        oracle = data.get("oracle", {})
        osh = str(oracle.get("stdout_hash", ""))
        if not osh or osh.startswith("PLACEHOLDER"):
            raise RuntimeError(f"manifest {manifest.name} has no real oracle.stdout_hash")

    # 1. work copy (crate; + harness for lib-only projects)
    if work_root.exists():
        shutil.rmtree(work_root)
    work_root.mkdir(parents=True)
    crate = work_root / "crate"
    _rsync_copy(cleaned_dir, crate)
    _ensure_fair_build(crate)

    binary_dir = crate
    if not check_only and workload.get("harness_dir"):
        # lib-only project: set up the driving harness crate, path-dep wired
        # to our crate copy, exactly like main.py:_stage_a.
        harness_src = manifest.parent / workload["harness_dir"]
        if not harness_src.is_dir():
            raise RuntimeError(f"harness dir not found: {harness_src}")
        harness = work_root / "harness"
        _rsync_copy(harness_src, harness)
        _ensure_fair_build(harness)
        _rewire_path_dep(harness / "Cargo.toml", crate)
        _ensure_harness_compat(crate, harness)
        binary_dir = harness

    verifier = Verifier(crate, binary_dir=binary_dir)

    if not check_only:
        input_root = manifest.parent
        raw_input = workload.get("input", "")
        input_path = (input_root / raw_input).resolve() if raw_input else None
        args_list = list(workload.get("args", []))
        binary = workload["binary"]
        exit_code = int(oracle.get("exit_code", 0))
        wrapper_rel = workload.get("wrapper", "")
        wrapper = (manifest.parent / wrapper_rel).resolve() if wrapper_rel else None

        def _w1():
            return verifier.w1(binary=binary, args=args_list, input_path=input_path,
                               expected_sha256=osh, expected_exit=exit_code,
                               wrapper=wrapper)

    # 2. baseline: pristine work copy must build (+ pass W1 unless check_only).
    logger.info(f"[apply] baseline build{'' if check_only else ' + W1'} …")
    gb = verifier.cargo_build()
    if not gb.ok:
        raise RuntimeError(f"baseline build FAILED — bad work copy: {gb.detail}")
    if not check_only:
        gw = _w1()
        if not gw.ok:
            raise RuntimeError(f"baseline W1 FAILED — oracle/setup mismatch: {gw.detail}")
    logger.info("[apply] baseline OK")

    derefs_before = _count_raw_derefs(crate)
    _git_init(crate, "intra_ptr baseline")

    # 3. plan + per-unit apply/verify/rollback
    cp = plan_crate(crate, only_fns=only_fns, airtight=airtight,
                    sa_lookup_dir=cleaned_dir)
    units = cp.units
    logger.info(f"[apply] {len(units)} airtight unit(s) to attempt")

    results: list[UnitResult] = []
    for u in units:
        fpath = crate / u.fn_file
        snap = fpath.read_bytes()
        anchors = [p.anchor for p in u.pointers]
        any_edit = False
        for p in u.pointers:
            if rewrite_fn_in_file(fpath, u.fn_name, p.anchor,
                                  p.target_view, p.length):
                any_edit = True
        if not any_edit:
            results.append(UnitResult(u.fn_name, anchors, "no_edits"))
            continue
        gc = verifier.cargo_check()
        gb = verifier.cargo_build() if gc.ok else gc
        gw = (_w1() if (gb.ok and not check_only) else gb)
        w1_ok = gw.ok or check_only
        if gc.ok and gb.ok and w1_ok:
            results.append(UnitResult(u.fn_name, anchors, "keep"))
            logger.info(f"[apply] KEEP {u.fn_name} ({anchors})")
        else:
            fpath.write_bytes(snap)            # roll back this unit only
            verifier.cargo_build()             # restore a good build state
            st = ("reject_check" if not gc.ok else
                  "reject_build" if not gb.ok else "reject_w1")
            detail = (gc.detail if not gc.ok else
                      gb.detail if not gb.ok else gw.detail)[:300]
            results.append(UnitResult(u.fn_name, anchors, st, detail))
            logger.info(f"[apply] {st.upper()} {u.fn_name} — {detail[:120]}")

    derefs_after = _count_raw_derefs(crate)
    kept = [r for r in results if r.status == "keep"]
    logger.info(
        f"[apply] DONE  kept={len(kept)}/{len(units)}  "
        f"raw_derefs {derefs_before}→{derefs_after} "
        f"(−{derefs_before - derefs_after})"
    )
    if not keep:
        logger.info(f"[apply] work copy left at {crate} (--keep to preserve on rerun)")
    return results


def main() -> int:
    ap = argparse.ArgumentParser(description="intra_ptr apply chain (P2a)")
    ap.add_argument("cleaned_dir", type=Path)
    ap.add_argument("manifest", type=Path, nargs="?", default=None)
    ap.add_argument("--work-root", type=Path, default=Path("/tmp/intra_ptr_work"))
    ap.add_argument("--only", nargs="+", default=None)
    ap.add_argument("--broad", action="store_true")
    ap.add_argument("--keep", action="store_true")
    ap.add_argument("--check-only", action="store_true",
                    help="gate on cargo check+build only (skip W1) — for "
                         "harness/lib crates and broad rewriter validation")
    args = ap.parse_args()
    logging.basicConfig(level=logging.INFO,
                        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
                        datefmt="%H:%M:%S")
    results = run_floor(args.cleaned_dir, args.manifest, args.work_root,
                        only_fns=set(args.only) if args.only else None,
                        airtight=not args.broad, keep=args.keep,
                        check_only=args.check_only)
    print("\n=== units ===")
    for r in results:
        print(f"  {r.status:14} {r.fn_name} {r.anchors} {r.detail[:80]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
