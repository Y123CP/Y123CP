"""Hotness-coverage probe (HOTNESS_COVERAGE.md §3).

For an ALREADY-generated harness, answer: which of the crate-under-test's own
functions did the workload push HOT (self-time >= tau), and which own functions
exist but never got hot (the holes)?

Pure measurement, read-only, does NOT touch the generation loop.

  1. denominator = own functions in the release binary (nm text symbols that
     belong to the crate under test: exported C-ABI names + mangled _ZN<crate>..).
  2. per op: perf record on a large input (gen-perf output / largest corpus),
     `perf report --sort symbol` -> per-own-function self-time%.
  3. aggregate: each function's MAX self-time% across ops (hot if some op heats it).
  4. hotness_coverage = |own fns >= tau| / |denominator|; list hot fns + holes.

Run on 37.2:  script_c2rust/.venv/bin/python -m harness_gen.hotness_probe \
                 --harness dataset_trans_process/lodepng/workloads/harness_gen/lodepng_harness
"""
from __future__ import annotations

import argparse
import json
import logging
import re
import sys
from pathlib import Path

_ROOT = Path(__file__).resolve().parent.parent
if str(_ROOT) not in sys.path:
    sys.path.insert(0, str(_ROOT))

from harness_gen import perf_workload as pw       # noqa: E402
from harness_gen.gates import (run_cmd, gate_build, harness_bin,  # noqa: E402
                               LLVM_PROFDATA, LLVM_COV, RUN_TIMEOUT)
from harness_gen.inventory import scan_crate      # noqa: E402

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s",
                    datefmt="%H:%M:%S")
logger = logging.getLogger("hotness_probe")

TAU = 1.0            # self-time% threshold for "hot"
TARGET_WALL = 2.0    # per-op perf record wall time
# A/B size-scan classifier (HOTNESS_COVERAGE.md §6 ⑤)
AB_TARGET_WALL = 1.0   # cheaper per-size probe
AB_MIN_OWN = 40.0      # largest size must reach this own self-time% to trust the scan
AB_DEGEN_OWN = 10.0    # drop an individual size below this (input truncated → rejected)


# ---- demangle (length-prefixed Rust legacy mangling) -----------------------
def _split_len_prefixed(body: str) -> list[str]:
    segs = []; i = 0
    while i < len(body):
        j = i
        while j < len(body) and body[j].isdigit(): j += 1
        if j == i: break
        n = int(body[i:j]); name = body[j:j + n]
        if not name: break
        segs.append(name); i = j + n
    return segs

_MANGLED = re.compile(r"^_ZN(.+?)(17h[0-9a-f]{16})?E$")

def demangle(sym: str) -> tuple[str, list[str]]:
    """Return (base_name, path_segments). Plain C names pass through."""
    m = _MANGLED.match(sym)
    if not m:
        return sym, [sym]
    segs = _split_len_prefixed(m.group(1))
    return (segs[-1] if segs else sym), segs


# ---- denominator: own functions in the release binary ----------------------
def own_denominator(binary: Path, crate_name: str, exported: set[str]
                    ) -> dict[str, str]:
    """symbol -> display_name for every own (crate-under-test) text symbol."""
    rc, out, err = run_cmd(["nm", "--defined-only", str(binary)], binary.parent, 120)
    if rc != 0:
        rc, out, err = run_cmd(["nm", str(binary)], binary.parent, 120)
    own: dict[str, str] = {}
    for line in out.splitlines():
        parts = line.split()
        if len(parts) < 3:
            continue
        typ, sym = parts[1], parts[2]
        if typ not in ("t", "T"):
            continue
        base, segs = demangle(sym)
        is_exported = sym in exported            # #[no_mangle] C-ABI name
        is_internal = (crate_name in segs)       # _ZN<crate>..internal static
        if is_exported or is_internal:
            own[sym] = base
    return own


# ---- per-op per-function self-time -----------------------------------------
def op_selftime(harness_bin: Path, op: str, inp: Path, iters: int, hdir: Path,
                crate_name: str, exported: set[str]) -> dict[str, float]:
    """base_fn -> self-time% (own crate only) for one op run."""
    data = hdir / f".hot_{op}.data"
    rec = pw._wrap(["perf", "record", "-o", str(data), "--",
                    str(harness_bin), op, str(inp), str(iters)], False)
    rc, _o, _e = run_cmd(rec, hdir, 600)
    if rc != 0 or not data.exists():
        return {}
    rc, out, _e = run_cmd(["perf", "report", "-i", str(data), "--stdio",
                           "-g", "none", "--sort", "symbol",
                           "--percent-limit", "0.02"], hdir, 300)
    data.unlink(missing_ok=True)
    fn_pct: dict[str, float] = {}
    for line in out.splitlines():
        # perf already demangles to `crate::mod::fn`; trailing `-  -` columns
        m = re.match(r"\s*([0-9.]+)%\s+\[[.k]\]\s+(\S+)", line)
        if not m:
            continue
        base = own_base(m.group(2), crate_name, exported)
        if base is not None:
            fn_pct[base] = fn_pct.get(base, 0.0) + float(m.group(1))
    return fn_pct


def own_base(sym: str, crate_name: str, exported: set[str]) -> str | None:
    """Base fn name if `sym` belongs to the crate under test, else None.
    perf renders own symbols either as a demangled `crate::mod::fn` path
    (internal statics) or as the plain C-ABI name (#[no_mangle] exports)."""
    if sym.startswith(crate_name + "::"):
        return sym.rsplit("::", 1)[-1]
    if "::" not in sym and sym in exported:
        return sym
    base, segs = demangle(sym)                     # fallback: raw _ZN form
    if sym in exported or crate_name in segs:
        return base
    return None


def pick_input(hdir: Path, op: str) -> Path | None:
    p = hdir / "perf_inputs" / f"{op}.perf.bin"
    if p.exists() and p.stat().st_size > 0:
        return p
    return pw.largest_corpus_input(hdir, op)


def all_inputs(hdir: Path, op: str) -> list[Path]:
    """Every input the harness ever runs for this op: seeds + full corpus +
    perf inputs. Reachability ("executed") must be measured over ALL of these,
    not just the single large perf input — otherwise seed-only-reachable
    functions get misreported as never-executed."""
    ins: list[Path] = []
    sd = hdir / "seeds"
    if sd.is_dir():
        ins += sorted(sd.glob(f"{op}.*.bin"))
    cd = hdir / "corpus" / op
    if cd.is_dir():
        ins += sorted(cd.glob("*.bin"))
    pd = hdir / "perf_inputs"
    if pd.is_dir():
        ins += sorted(pd.glob(f"{op}.perf.bin")) + sorted(pd.glob(f"{op}*.perf.bin"))
    # de-dup by resolved path, keep order
    seen, out = set(), []
    for p in ins:
        rp = str(p.resolve())
        if rp not in seen and p.exists() and p.stat().st_size >= 0:
            seen.add(rp); out.append(p)
    return out


def coverage_counts(hdir: Path, ops: list[str], crate_name: str,
                    exported: set[str]) -> set[str] | None:
    """base_fn set that llvm instrument-coverage saw EXECUTED (count>0) over the
    UNION of all harness inputs (seeds+corpus+perf). Distinguishes truly
    never-executed from executed-but-cold. None if the build/export failed."""
    import shutil
    ok, err = gate_build(hdir, target_dir="target-cov",
                         rustflags="-C instrument-coverage -C link-dead-code")
    if not ok:
        logger.warning("coverage build failed: %s", err[-300:])
        return None
    binp = harness_bin(hdir, "target-cov")
    covdir = hdir / ".hotcov"
    if covdir.exists():
        shutil.rmtree(covdir)
    covdir.mkdir()
    ran = 0
    for op in ops:
        for i, inp in enumerate(all_inputs(hdir, op)):
            rc, _o, _e = run_cmd(
                [str(binp), op, str(inp)], hdir, RUN_TIMEOUT,
                {"LLVM_PROFILE_FILE": str(covdir / f"{op}-{i}-%p.profraw")})
            if rc == 0:
                ran += 1
    logger.info("coverage: ran %d input executions over %d ops", ran, len(ops))
    if ran == 0:
        return None
    profdata = covdir / "cov.profdata"
    raws = [str(p) for p in sorted(covdir.glob("*.profraw"))]
    rc, _o, err = run_cmd([LLVM_PROFDATA, "merge", "-sparse", "-o",
                           str(profdata)] + raws, hdir, 300)
    if rc != 0:
        logger.warning("profdata merge failed: %s", err[-300:]); return None
    rc, out, err = run_cmd([LLVM_COV, "export", str(binp),
                            f"-instr-profile={profdata}", "-skip-expansions"],
                           hdir, 300)
    if rc != 0:
        logger.warning("llvm-cov export failed: %s", err[-300:]); return None
    executed: set[str] = set()
    data = json.loads(out)["data"][0]
    for fn in data.get("functions", []):
        name = fn.get("name", "")
        if fn.get("count", 0) <= 0:
            continue
        base = own_base(name, crate_name, exported) \
            or (name.rsplit("::", 1)[-1] if "::" in name else None)
        if base is not None:
            executed.add(base)
    return executed


def _size_grid(inp: Path, hdir: Path, op: str) -> list[tuple[int, Path]]:
    """Ascending [(size_bytes, path)] = head-truncated variants + the full input.
    Head truncation is a generic (format-blind) size knob; a truncation that
    corrupts a structured input profiles poorly and is filtered out downstream
    by the own-time guard, so the trend stays directionally valid."""
    grid = [(inp.stat().st_size, inp)]
    for v in pw._size_variants(inp, hdir / ".ab_sweep", op):
        try:
            grid.append((v.stat().st_size, v))
        except OSError:
            pass
    seen, out = set(), []
    for sz, p in sorted(grid, key=lambda t: t[0]):
        if sz > 0 and sz not in seen:
            seen.add(sz); out.append((sz, p))
    return out


def classify_ab(hdir: Path, harness_bin: Path, cold: list[str],
                best: dict[str, tuple[float, str]], sel: dict,
                crate_name: str, exported: set[str]) -> dict[str, tuple[str, str]]:
    """Size-scan each executed-but-cold fn on its hottest op; classify
    A (self-time share RISES with input → workload-limited, could become a
    candidate if the input is grown) vs B (flat/falls → intrinsically bounded,
    out of scope). Grouped by op so each op is swept once. Returns
    {fn: (label, detail)} with label in {'A','B','unclassified'}."""
    import shutil
    result: dict[str, tuple[str, str]] = {}
    by_op: dict[str, list[str]] = {}
    for f in cold:
        if f in best:                       # appeared in perf → has a hottest op
            by_op.setdefault(best[f][1], []).append(f)
        else:                               # never sampled even at full size
            result[f] = ("unclassified", "not sampled at full size (too cold)")

    for op, fns in by_op.items():
        r = sel.get(op)
        inp = Path(r.input_path) if (r and r.input_path) else pick_input(hdir, op)
        if inp is None or not inp.exists():
            for f in fns:
                result[f] = ("unclassified", "no input")
            continue
        grid = _size_grid(inp, hdir, op)
        if len(grid) < 2:
            for f in fns:
                result[f] = ("unclassified", f"input not scalable (1 size {grid[0][0]}B)")
            continue
        # measure per-fn self% at each size; drop degenerate (rejected) sizes
        series: list[tuple[int, dict[str, float]]] = []
        for sz, p in grid:
            iters, _ = pw._autotune_iters(harness_bin, op, p, hdir, AB_TARGET_WALL, False)
            if iters == 0:
                continue
            m = op_selftime(harness_bin, op, p, iters, hdir, crate_name, exported)
            if sum(m.values()) < AB_DEGEN_OWN:          # input truncated → rejected
                continue
            series.append((sz, m))
        if len(series) < 2 or sum(series[-1][1].values()) < AB_MIN_OWN:
            for f in fns:
                result[f] = ("unclassified",
                             f"op={op}: too few valid sizes / low own-time")
            continue
        sizes = [s for s, _ in series]
        for f in fns:
            pts = [m.get(f, 0.0) for _, m in series]
            lo, hi = pts[0], pts[-1]
            detail = (f"op={op} self%@{[s // 1024 for s in sizes]}KB="
                      f"{[round(x, 2) for x in pts]}")
            if hi >= TAU or (hi >= lo * 1.3 and hi - lo >= 0.3):
                result[f] = ("A", detail + " ↑")
            else:
                result[f] = ("B", detail)
    shutil.rmtree(hdir / ".ab_sweep", ignore_errors=True)
    return result


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--harness", required=True, help="generated harness dir")
    ap.add_argument("--tau", type=float, default=TAU)
    ap.add_argument("--no-ab", action="store_true",
                    help="skip the A/B size-scan classification of cold holes")
    args = ap.parse_args()

    hdir = Path(args.harness)
    if not hdir.is_absolute():
        hdir = (_ROOT.parent / hdir)
    hdir = hdir.resolve()

    # crate under test (from harness Cargo.toml dep) + its exported symbols
    cargo = (hdir / "Cargo.toml").read_text()
    dep = re.search(r'^\s*(\w+)\s*=\s*\{\s*path\s*=\s*"([^"]+)"', cargo, re.M)
    crate_name, crate_path = dep.group(1), Path(dep.group(2))
    inv = scan_crate(crate_path)
    exported = set(inv.link_names) | set(inv.fn_names)
    logger.info("crate under test: %s  exported C-ABI fns: %d",
                crate_name, len(inv.fns))

    # ops from spec.json (logs/ may sit beside the harness dir, not inside it)
    spec = None
    for cand in (hdir / "logs" / "spec.json", hdir.parent / "logs" / "spec.json"):
        if cand.exists():
            spec = json.loads(cand.read_text()); break
    ops = [o["name"] for o in spec["operations"]] if spec else []
    if not ops:                                    # fallback: corpus subdir names
        cdir = hdir / "corpus"
        ops = sorted(p.name for p in cdir.iterdir() if p.is_dir()) if cdir.is_dir() else []
    logger.info("operations: %d %s", len(ops), ops)

    harness_bin = pw.build_release(hdir)
    denom = own_denominator(harness_bin, crate_name, exported)
    logger.info("denominator (own fns in binary): %d", len(set(denom.values())))

    # input selection: reuse perf_workload's picker (gen-perf + size sweep +
    # top-k corpus, best by self_time_own) — a crude single-file pick badly
    # under-reports hotness on size-capped algorithms (e.g. fzy: 2% vs 93%).
    logger.info("selecting best input per op via perf_workload ...")
    sel = {r.op: r for r in pw.profile_harness(hdir, ops, target_wall=TARGET_WALL)}

    # measure each op
    best: dict[str, tuple[float, str]] = {}    # fn -> (max self%, op)
    per_op_own: dict[str, float] = {}          # op -> own self-time sum
    for op in ops:
        r = sel.get(op)
        inp = Path(r.input_path) if (r and r.input_path) else pick_input(hdir, op)
        if inp is None or not inp.exists():
            logger.warning("[%s] no input, skip", op); continue
        iters, _pi = pw._autotune_iters(harness_bin, op, inp, hdir, TARGET_WALL, False)
        if iters == 0:
            logger.warning("[%s] errored on input, skip", op); continue
        fn_pct = op_selftime(harness_bin, op, inp, iters, hdir, crate_name, exported)
        per_op_own[op] = round(sum(fn_pct.values()), 1)
        for fn, pct in fn_pct.items():
            if fn not in best or pct > best[fn][0]:
                best[fn] = (pct, op)
        top = sorted(fn_pct.items(), key=lambda kv: -kv[1])[:3]
        logger.info("[%s] iters=%d own=%.0f%% top=%s", op, iters, per_op_own[op],
                    ", ".join(f"{f}:{p:.0f}" for f, p in top))

    # cross with llvm-cov to split holes: never-executed vs executed-but-cold
    executed = coverage_counts(hdir, ops, crate_name, exported)

    # report
    all_fns = set(denom.values())
    hot = {f: best[f] for f in best if best[f][0] >= args.tau}
    cov = 100.0 * len(hot) / max(len(all_fns), 1)

    print("\n" + "=" * 72)
    print(f"HOTNESS COVERAGE — {hdir.name}")
    print("=" * 72)
    print(f"own functions (denominator): {len(all_fns)}")
    print(f"hot functions (>= {args.tau}% self-time in some op): {len(hot)}")
    print(f"hotness coverage: {cov:.1f}%   (= the candidate set)\n")

    print(f"{'HOT function':44s}{'self%':>7}  hottest-op")
    print("-" * 72)
    for fn, (pct, op) in sorted(hot.items(), key=lambda kv: -kv[1][0]):
        print(f"{fn[:44]:44s}{pct:>7.1f}  {op}")

    holes = all_fns - set(hot)
    print(f"\nHOLES (own fns never hot): {len(holes)}")
    if executed is None:
        # fallback: perf-only split (executed status unknown)
        seen_cold = sorted(f for f in best if best[f][0] < args.tau)
        never = sorted(all_fns - set(best))
        print("  (llvm-cov unavailable — perf-only split, executed status unknown)")
        print(f"  appeared-in-perf-but-cold: {len(seen_cold)}")
        for f in seen_cold[:30]:
            print(f"     {f}  ({best[f][0]:.2f}% in {best[f][1]})")
        print(f"  not-in-perf: {len(never)}")
        for f in never[:30]:
            print(f"     {f}")
    else:
        cold = sorted(f for f in holes if f in executed)     # cov>0, perf<tau
        never = sorted(f for f in holes if f not in executed)  # cov==0
        print(f"  [executed-but-cold]  (cov>0, self<{args.tau}%): {len(cold)}")
        if cold and not args.no_ab:
            logger.info("A/B size-scan on %d executed-but-cold fns ...", len(cold))
            ab = classify_ab(hdir, harness_bin, cold, best, sel, crate_name, exported)
            groups = [
                ("A", "A workload-limited → grow input to make it a candidate"),
                ("B", "B intrinsically-bounded → out of scope"),
                ("unclassified", "unclassified (too cold to size-scan)"),
            ]
            for label, title in groups:
                grp = sorted(f for f in cold if ab.get(f, ("unclassified",))[0] == label)
                print(f"    · {title}: {len(grp)}")
                for f in grp:
                    print(f"        {f}  {ab.get(f, ('', ''))[1]}")
            print("    (A/B via head-truncation size-scan — a B here may still "
                  "scale on a structural dim; that's the LLM-dimension step, §6)")
        else:
            for f in cold:
                pv = f"{best[f][0]:.2f}% in {best[f][1]}" if f in best else "not sampled"
                print(f"     {f}  ({pv})")
        print(f"  [never-executed]     (cov==0): {len(never)}"
              f"   ← needs coverage fuzzing (§4)")
        for f in never[:40]:
            print(f"     {f}")

    print(f"\nper-op own self-time%: " +
          ", ".join(f"{o}={p}" for o, p in per_op_own.items()))
    return 0


if __name__ == "__main__":
    sys.exit(main())
