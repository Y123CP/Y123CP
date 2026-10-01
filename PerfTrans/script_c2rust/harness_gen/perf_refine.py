"""perf_refine — refine gen_perf from the fuzzing corpus (HOTNESS_COVERAGE.md §6 ②).

gen_perf is the LLM's BLIND big-input guess written at codegen; it covers the
main path but misses shapes only fuzzing discovers (lodepng: 18 chunk/text/icc
functions the corpus reaches but gen_perf doesn't; binn: 0 — ops already split
shapes). This step, run AFTER the harness is generated AND fuzzed:

  1. gap  = own fns the CORPUS reaches but gen_perf's perf inputs don't
            (== shapes fuzzing found that gen_perf misses)
  2. sample a few representative corpus inputs per op
  3. LLM rewrites gen_perf (keeping ops()/gen_seeds intact) to ALSO produce
     BIG inputs covering the gap shapes — format-aware scale-up
  4. verify: build + smoke + re-measure; accept iff the new gen_perf now
     reaches strictly more of the gap (else revert — gen_perf never regresses)

This is §6 step ② (initial gen_perf = scale-up of fuzzing shapes). It does NOT
touch correctness (seeds/smoke/golden); it only widens the perf workload so more
real kernels can be surfaced as candidates.

Run on 37.2:  script_c2rust/.venv/bin/python -m harness_gen.perf_refine \
                 --harness <harness_dir>
"""
from __future__ import annotations

import argparse
import json
import logging
import re
import shutil
import sys
from pathlib import Path

_ROOT = Path(__file__).resolve().parent.parent
if str(_ROOT) not in sys.path:
    sys.path.insert(0, str(_ROOT))

from harness_gen import hotness_probe as hp                       # noqa: E402
from harness_gen import perf_workload as pw                       # noqa: E402
from harness_gen import prompts                                   # noqa: E402
from harness_gen.gates import (gate_build, gate_smoke, harness_bin,  # noqa: E402
                               run_cmd, LLVM_PROFDATA, LLVM_COV, RUN_TIMEOUT)
from harness_gen.inventory import scan_crate, CrateInventory      # noqa: E402

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s",
                    datefmt="%H:%M:%S")
logger = logging.getLogger("perf_refine")

COV_RUSTFLAGS = ("-C instrument-coverage -C link-dead-code "
                 "-C debug-assertions=off -C overflow-checks=off")
SAMPLES_PER_OP = 4          # representative corpus inputs shown to the LLM
SAMPLE_HEX_BYTES = 192      # per-sample hex cap in the prompt
MAX_REPAIRS = 3


# ---- coverage of an input set (own crate functions reached) ----------------
def reached(hdir: Path, binp: Path, inputs_by_op: dict[str, list[Path]],
            crate_name: str, exported: set[str], tag: str) -> set[str]:
    cd = hdir / f".refine_{tag}"
    shutil.rmtree(cd, ignore_errors=True); cd.mkdir()
    n = 0
    for op, files in inputs_by_op.items():
        for i, f in enumerate(files):
            if not Path(f).exists():
                continue
            run_cmd([str(binp), op, str(f)], hdir, RUN_TIMEOUT,
                    {"LLVM_PROFILE_FILE": str(cd / f"{op}-{i}-%p.profraw")})
            n += 1
    prof = cd / "c.profdata"
    raws = [str(p) for p in sorted(cd.glob("*.profraw"))]
    if not raws:
        shutil.rmtree(cd, ignore_errors=True); return set()
    run_cmd([LLVM_PROFDATA, "merge", "-sparse", "-o", str(prof)] + raws, hdir, 300)
    rc, out, _ = run_cmd([LLVM_COV, "export", str(binp),
                          f"-instr-profile={prof}", "-skip-expansions"], hdir, 300)
    fns: set[str] = set()
    if rc == 0:
        for fn in json.loads(out)["data"][0].get("functions", []):
            if fn.get("count", 0) <= 0:
                continue
            name = fn.get("name", "")
            b = hp.own_base(name, crate_name, exported) \
                or (name.rsplit("::", 1)[-1] if "::" in name else None)
            if b:
                fns.add(b)
    shutil.rmtree(cd, ignore_errors=True)
    logger.info("[cov:%s] ran %d inputs → own fns reached: %d", tag, n, len(fns))
    return fns


def corpus_inputs(hdir: Path, ops: list[str]) -> dict[str, list[Path]]:
    return {op: sorted((hdir / "corpus" / op).glob("*.bin"))
            for op in ops if (hdir / "corpus" / op).is_dir()}


def genperf_inputs(hdir: Path, ops: list[str]) -> dict[str, list[Path]]:
    pd = hdir / "perf_inputs"
    return {op: sorted(pd.glob(f"{op}*.perf.bin")) for op in ops}


def sample_corpus(inputs_by_op: dict[str, list[Path]]) -> dict[str, list[bytes]]:
    """A few size-diverse representative inputs per op (smallest, largest, middles)."""
    out: dict[str, list[bytes]] = {}
    for op, files in inputs_by_op.items():
        if not files:
            continue
        fs = sorted(files, key=lambda p: p.stat().st_size)
        idx = sorted(set([0, len(fs) - 1] +
                         [round(len(fs) * k / (SAMPLES_PER_OP - 1))
                          for k in range(SAMPLES_PER_OP)]))
        picks = [fs[min(i, len(fs) - 1)] for i in idx][:SAMPLES_PER_OP]
        out[op] = [p.read_bytes() for p in picks]
    return out


# ---- LLM prompt ------------------------------------------------------------
REFINE_SYSTEM = """\
You are an expert Rust systems engineer improving ONE function — `gen_perf` — of
an existing c2rust workload harness. You output ONE complete `src/lib.rs` file in
a single ```rust block and nothing else."""


def _samples_block(samples: dict[str, list[bytes]], gap_reached_by_op: dict) -> str:
    lines = []
    for op, blobs in samples.items():
        tag = f" (its corpus reaches gap fns: {', '.join(gap_reached_by_op[op])})" \
            if gap_reached_by_op.get(op) else ""
        lines.append(f"\noperation `{op}`{tag} — representative fuzzer inputs:")
        for b in blobs:
            h = b[:SAMPLE_HEX_BYTES].hex()
            more = f" …(+{len(b) - SAMPLE_HEX_BYTES}B)" if len(b) > SAMPLE_HEX_BYTES else ""
            lines.append(f"  {len(b):>6}B: {h}{more}")
    return "\n".join(lines)


def refine_user(inv: CrateInventory, lib_rs: str, gap: list[str],
                samples: dict[str, list[bytes]], gap_reached_by_op: dict) -> str:
    return f"""\
The harness below compiles and is a valid oracle. Its `gen_perf` produces ONE
large input per op to make the library's hot loop dominate under perf. But a
fuzzer explored these ops and its corpus REACHES {len(gap)} library functions
that `gen_perf`'s large inputs never touch — i.e. `gen_perf` misses whole
input SHAPES the fuzzer found:

MISSED functions (gen_perf must learn to exercise these at scale):
{chr(10).join('  ' + g for g in gap)}

The fuzzer inputs are TINY (bytes). Your job is FORMAT-AWARE SCALE-UP: figure
out, from these samples + the library API, what feature each missed function
needs (e.g. multiple/typed chunks, text/exif/icc metadata, palette, a specific
sub-format), then make `gen_perf` emit a LARGE input carrying that feature so
the function runs many times, NOT once. Never just pad/concatenate bytes —
that produces malformed input that hits error paths. Use the library's own
encoders (as gen_seeds does) to build big VALID inputs.

Representative fuzzer inputs per op (hex):
{_samples_block(samples, gap_reached_by_op)}

CRATE API (for the module paths / encoders you may call):
{prompts._fn_listing(inv, limit_chars=24_000)}

CURRENT src/lib.rs:
```rust
{lib_rs}
```

{prompts.CONTRACT}

RULES FOR THIS EDIT:
- Change ONLY `gen_perf` (and add private helper fns if needed). Keep `ops()`
  and `gen_seeds` BYTE-FOR-BYTE identical — they define correctness.
- Each op that currently has a perf input should still get one, now shaped to
  also exercise the missed functions belonging to that op.
- Inputs must stay VALID and LARGE (the library algorithm dominates, not libc).

Output the single complete updated `src/lib.rs` in one ```rust block."""


# ---- driver ----------------------------------------------------------------
def _load(hdir: Path):
    spec = None
    for c in (hdir / "logs" / "spec.json", hdir.parent / "logs" / "spec.json"):
        if c.exists():
            spec = json.loads(c.read_text()); break
    ops = [o["name"] for o in spec["operations"]] if spec else []
    cargo = (hdir / "Cargo.toml").read_text()
    m = re.search(r'^\s*(\w+)\s*=\s*\{\s*path\s*=\s*"([^"]+)"', cargo, re.M)
    inv = scan_crate(Path(m.group(2)))
    return spec, ops, m.group(1), inv


def _make_llm(hdir: Path):
    from Config.paths import get_path
    from utils.llm_client import LLMClient
    model = get_path("DEFAULT_LLM_MODEL")
    logs = hdir.parent / "logs"
    logs.mkdir(parents=True, exist_ok=True)
    return LLMClient(model, transcript_path=logs / "refine_transcript.log",
                     cache_path=logs / "refine_cache.jsonl", timeout=180)


def refine(hdir: Path) -> int:
    spec, ops, crate_name, inv = _load(hdir)
    exported = set(inv.link_names) | set(inv.fn_names)

    rel = pw.build_release(hdir)
    run_cmd([str(rel), "gen-perf", str(hdir / "perf_inputs")], hdir, 300)
    corp_in = corpus_inputs(hdir, ops)
    if not corp_in:
        logger.error("no corpus — run fuzzing first"); return 2

    # instrumented binary for coverage
    ok, err = gate_build(hdir, target_dir="target-cov", rustflags=COV_RUSTFLAGS)
    if not ok:
        logger.error("cov build failed: %s", err[-300:]); return 1
    binp = harness_bin(hdir, "target-cov")

    corp_reached = reached(hdir, binp, corp_in, crate_name, exported, "corpus")
    gp_reached = reached(hdir, binp, genperf_inputs(hdir, ops), crate_name, exported, "gp0")
    gap = sorted(corp_reached - gp_reached)
    logger.info("GAP (corpus reaches, gen_perf misses): %d %s", len(gap), gap)
    if not gap:
        logger.info("gen_perf already covers everything the corpus reaches — nothing to refine")
        return 0

    # attribute gap fns to ops (cheap: sample each op's corpus)
    gap_by_op: dict[str, list[str]] = {}
    for op, files in corp_in.items():
        sub = {op: files[:30]}
        r = reached(hdir, binp, sub, crate_name, exported, f"attr_{op}")
        hit = sorted(set(gap) & r)
        if hit:
            gap_by_op[op] = hit
    samples = sample_corpus(corp_in)

    # LLM rewrite (bounded repair on build failure)
    llm = _make_llm(hdir)
    lib_path = hdir / "src" / "lib.rs"
    orig = lib_path.read_text()
    user = refine_user(inv, orig, gap, samples, gap_by_op)
    system = REFINE_SYSTEM
    new_lib = None
    for attempt in range(1 + MAX_REPAIRS):
        raw = llm.chat(system, user, meta={"phase": f"refine-{attempt}",
                                           "project": hdir.name})
        cand = prompts.extract_rust(raw)
        if not cand:
            logger.warning("attempt %d: no rust block", attempt); continue
        lib_path.write_text(cand, encoding="utf-8")
        bok, berr = gate_build(hdir,
                               rustflags="-C debug-assertions=off -C overflow-checks=off")
        if not bok:
            logger.info("attempt %d: build failed → repair", attempt)
            user = (f"Your rewrite failed to build:\n```\n{berr[-3000:]}\n```\n"
                    f"Fix it. Output the complete corrected src/lib.rs in one ```rust block.")
            continue
        sok, serr = gate_smoke(hdir, spec)
        if not sok:
            logger.info("attempt %d: smoke failed → repair", attempt)
            user = (f"Your rewrite broke the smoke gate (you must NOT change ops()/"
                    f"gen_seeds behaviour):\n```\n{serr[-3000:]}\n```\n"
                    f"Fix it. Output the complete corrected src/lib.rs in one ```rust block.")
            continue
        new_lib = cand
        break

    if new_lib is None:
        lib_path.write_text(orig, encoding="utf-8")
        logger.error("refine failed (no building+smoke-passing rewrite); reverted")
        return 1

    # re-measure: did the new gen_perf reach more of the gap?
    rel2 = pw.build_release(hdir)
    run_cmd([str(rel2), "gen-perf", str(hdir / "perf_inputs")], hdir, 300)
    gate_build(hdir, target_dir="target-cov", rustflags=COV_RUSTFLAGS)
    binp2 = harness_bin(hdir, "target-cov")
    gp2 = reached(hdir, binp2, genperf_inputs(hdir, ops), crate_name, exported, "gp1")
    now = sorted(set(gap) & gp2)
    gained = sorted(set(now) - set(set(gap) & gp_reached))

    print("\n" + "=" * 66)
    print(f"PERF_REFINE — {hdir.name}")
    print("=" * 66)
    print(f"gap (corpus reached, gen_perf missed): {len(gap)}")
    print(f"new gen_perf now reaches of the gap  : {len(now)}  {now}")
    print(f"newly gained vs old gen_perf         : {len(gained)}  {gained}")

    if gained:
        logger.info("ACCEPT: gen_perf now covers %d more gap fns", len(gained))
        return 0
    lib_path.write_text(orig, encoding="utf-8")
    logger.warning("no gap fn gained → reverted gen_perf (never regress)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--harness", required=True)
    args = ap.parse_args()
    hdir = Path(args.harness)
    if not hdir.is_absolute():
        hdir = (_ROOT.parent / hdir)
    return refine(hdir.resolve())


if __name__ == "__main__":
    sys.exit(main())
