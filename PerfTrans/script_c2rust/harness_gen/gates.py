"""Deterministic gates for the harness-gen agent.

  gate A  build      — cargo build (dev profile: fast loop; correctness
                       does not depend on opt level)
  gate B  smoke      — gen-seeds, determinism (run twice), input
                       sensitivity (seed.1 vs seed.2 digests differ),
                       iters mode
  gate C  coverage   — rebuild with -C instrument-coverage
                       -C link-dead-code (so un-called dep functions are
                       kept and counted), run all ops × seeds, merge with
                       llvm-profdata-17, evaluate with llvm-cov-17 export

Every gate returns (ok, feedback) where `feedback` is written to be fed
back to the LLM verbatim on failure.
"""

from __future__ import annotations

import json
import logging
import os
import re
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from .inventory import CrateInventory

logger = logging.getLogger(__name__)

BUILD_TIMEOUT = 1800
RUN_TIMEOUT = 60
COV_TOOL_TIMEOUT = 300

_FAILING_OP_RE = re.compile(r"^op `([A-Za-z_][A-Za-z0-9_]*)`")

# Spec key holding operations that a gate loop gave up on. The smoke gate
# self-heals the spec from the seeds the harness emits, so a caller that only
# deletes an operation from `spec["operations"]` sees it adopted straight back
# on the next gate run — an unbounded drop↔re-adopt loop, because dropping is
# deliberately free of repair budget. Recording the decision IN the spec makes
# it stick across every gate_smoke call the spec is passed to, including the
# ones in the perf loop and in perf_refine.
DROPPED_KEY = "dropped_operations"


def mark_dropped(spec: dict[str, Any], op_name: str) -> None:
    """Remove `op_name` from the spec and remember that it must stay out."""
    spec["operations"] = [o for o in spec["operations"] if o["name"] != op_name]
    dropped = spec.setdefault(DROPPED_KEY, [])
    if op_name not in dropped:
        dropped.append(op_name)


def failing_op(feedback: str) -> str | None:
    """The single operation a smoke/perf failure is about, if it is about one.

    Every per-op gate message opens with ``op `<name>` …``; build failures and
    infrastructure errors do not, and return None. Used to spend the repair
    budget on the harness rather than on one broken operation: a repair rewrites
    the WHOLE lib.rs, so a stubborn op is repaired at the cost of every other
    op's stability, and when the budget runs out the run reverts the entire
    coverage extension — losing the good ops to save none.
    """
    if not feedback:
        return None
    m = _FAILING_OP_RE.match(feedback.lstrip())
    return m.group(1) if m else None


FEEDBACK_BUDGET = 8000
_DIAG_START = re.compile(r"^(error|warning)(\[[^\]]+\])?:", re.M)


def select_build_feedback(stderr: str, budget: int = FEEDBACK_BUDGET) -> str:
    """The part of a cargo failure the repair LLM actually needs.

    Taking the tail assumes the errors come last. That holds for a small crate,
    and breaks completely for a large one: the crate under test is compiled in
    the same invocation, and a c2rust translation of a big C library emits
    thousands of warnings (unused_must_use, unused parens, dead code). On
    libxml2 (1603 fns) those warnings filled the whole window, so the repair
    prompt carried NOT ONE `error:` line — only diagnostics about the library's
    own source, which the harness must not touch. The model was asked to fix a
    build it could not see, and each repair round broke something new.

    So select by content: every `error` diagnostic first, in source order,
    then tail context if budget remains. Falls back to the tail when there are
    no error diagnostics at all (linker failures, OOM-killed rustc).
    """
    starts = [m.start() for m in _DIAG_START.finditer(stderr)]
    if not starts:
        return stderr[-budget:]
    blocks = [stderr[a:b] for a, b in zip(starts, starts[1:] + [len(stderr)])]
    errors = [b for b in blocks if b.startswith("error")]
    if not errors:
        return stderr[-budget:]

    kept: list[str] = []
    used = 0
    for block in errors:
        if used + len(block) > budget:
            kept.append(f"\n... {len(errors) - len(kept)} more error(s) elided "
                        f"— fix these first, then rebuild ...\n")
            break
        kept.append(block)
        used += len(block)
    return "".join(kept)[:budget]


def _tool(name: str, suffixed: str) -> str:
    return suffixed if shutil.which(suffixed) else name


LLVM_PROFDATA = _tool("llvm-profdata", "llvm-profdata-17")
LLVM_COV = _tool("llvm-cov", "llvm-cov-17")


def run_cmd(cmd: list[str], cwd: Path, timeout: int,
            env_extra: dict[str, str] | None = None) -> tuple[int, str, str]:
    env = os.environ.copy()
    env["CARGO_TERM_COLOR"] = "never"
    if env_extra:
        env.update(env_extra)
    try:
        p = subprocess.run(cmd, cwd=str(cwd), env=env, timeout=timeout,
                           capture_output=True, text=True, errors="replace")
        return p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired:
        return 124, "", f"TIMEOUT after {timeout}s: {' '.join(cmd)}"


# ────────────────────────────────────────────────────────────────────
# gate A — build
# ────────────────────────────────────────────────────────────────────

def gate_build(harness_dir: Path, target_dir: str = "target",
               rustflags: str | None = None) -> tuple[bool, str]:
    env = {"RUSTFLAGS": rustflags} if rustflags else None
    rc, _out, err = run_cmd(["cargo", "build", "--target-dir", target_dir],
                            harness_dir, BUILD_TIMEOUT, env)
    if rc == 0:
        return True, ""
    return False, select_build_feedback(err)


def harness_bin(harness_dir: Path, target_dir: str = "target") -> Path:
    return harness_dir / target_dir / "debug" / "harness"


# ────────────────────────────────────────────────────────────────────
# gate B — smoke: seeds, determinism, input sensitivity, iters
# ────────────────────────────────────────────────────────────────────

def gate_smoke(harness_dir: Path, spec: dict[str, Any],
               target_dir: str = "target") -> tuple[bool, str]:
    bin_path = harness_bin(harness_dir, target_dir)
    seeds_dir = harness_dir / "seeds"
    if seeds_dir.exists():
        shutil.rmtree(seeds_dir)
    seeds_dir.mkdir(parents=True)

    rc, out, err = run_cmd([str(bin_path), "gen-seeds", str(seeds_dir)],
                           harness_dir, RUN_TIMEOUT * 3)
    if rc != 0:
        return False, f"`harness gen-seeds` exited {rc}.\nstderr:\n{err[-3000:]}"

    # Self-heal the spec: a coverage-extension round may add operations to
    # main.rs without re-emitting the JSON spec. Any `<op>.1.bin` seed for
    # an op the spec doesn't know about is adopted (needs_input=true), so
    # the smoke and coverage gates exercise it too.
    #
    # An op the caller has dropped stays dropped: its seed is still emitted (the
    # code is still in lib.rs) but the spec is the contract every later gate,
    # the corpus and the golden replay read, so re-adopting it would undo the
    # decision and loop forever.
    known = {op["name"] for op in spec["operations"]}
    dropped = set(spec.get(DROPPED_KEY) or ())
    for seed in sorted(seeds_dir.glob("*.1.bin")):
        op_name = seed.name[:-len(".1.bin")]
        if op_name and op_name not in known and op_name not in dropped:
            spec["operations"].append({
                "name": op_name, "needs_input": True,
                "summary": "auto-discovered from gen-seeds output",
                "api_calls": [], "input_format": "", "digest": "",
                # Provenance, not decoration: an op the PLAN designed is a
                # main path of the library and is worth repair budget; an op a
                # coverage round invented is a bonus, and spending the shared
                # budget on it risks the whole extension it came with.
                "adopted": True})
            known.add(op_name)
            logger.info("[smoke] adopted op `%s` discovered via gen-seeds", op_name)

    for op in spec["operations"]:
        name = op["name"]
        if not op.get("needs_input", True):
            continue
        s1, s2 = seeds_dir / f"{name}.1.bin", seeds_dir / f"{name}.2.bin"
        for s in (s1, s2):
            if not s.exists() or s.stat().st_size == 0:
                return False, (f"gen-seeds did not produce a non-empty {s.name} "
                               f"for operation `{name}` (contract: two distinct "
                               f"valid inputs per needs_input operation)")
        if s1.read_bytes() == s2.read_bytes():
            return False, (f"{name}.1.bin and {name}.2.bin are byte-identical; "
                           f"the two seeds must be distinct inputs")

    for op in spec["operations"]:
        name = op["name"]
        needs = op.get("needs_input", True)
        s1 = str(seeds_dir / f"{name}.1.bin") if needs \
            else str(seeds_dir / "_none")
        if not needs:
            Path(s1).write_bytes(b"")
        # determinism: run twice, byte-identical stdout
        rc1, out1, err1 = run_cmd([str(bin_path), name, s1], harness_dir, RUN_TIMEOUT)
        if rc1 != 0:
            return False, (f"op `{name}` on its seed exited {rc1}.\n"
                           f"stderr:\n{err1[-3000:]}")
        if not out1.strip():
            return False, f"op `{name}` printed nothing; the digest line is required"
        rc2, out2, _ = run_cmd([str(bin_path), name, s1], harness_dir, RUN_TIMEOUT)
        if rc2 != 0 or out1 != out2:
            return False, (f"op `{name}` is NON-DETERMINISTIC: two identical "
                           f"invocations printed different stdout.\n"
                           f"run1:\n{out1[:1500]}\nrun2:\n{out2[:1500]}\n"
                           f"Remove every address/time/order-dependent value "
                           f"from the digest.")
        # input sensitivity: seed.2 must change the digest
        if needs:
            s2 = str(seeds_dir / f"{name}.2.bin")
            rc3, out3, err3 = run_cmd([str(bin_path), name, s2], harness_dir, RUN_TIMEOUT)
            if rc3 != 0:
                return False, (f"op `{name}` failed on {name}.2.bin (exit {rc3}) — "
                               f"gen-seeds must emit two VALID inputs.\n"
                               f"stderr:\n{err3[-3000:]}")
            if out3 == out1:
                return False, (f"op `{name}`: seed.1 and seed.2 give an identical "
                               f"digest — the digest does not depend on the input "
                               f"bytes. Fold input-derived values (sizes, checksums "
                               f"of output buffers) into it.")
        # iters mode must at least run cleanly
        rc4, _out4, err4 = run_cmd([str(bin_path), name, s1, "3"], harness_dir,
                                   RUN_TIMEOUT * 3)
        if rc4 != 0:
            return False, (f"op `{name}` with iters=3 exited {rc4} — the "
                           f"in-process repetition mode is broken.\n"
                           f"stderr:\n{err4[-3000:]}")
    return True, ""


# ────────────────────────────────────────────────────────────────────
# gate D — gen_perf actually runs (perf inputs are usable, not just compiled)
# ────────────────────────────────────────────────────────────────────
def gate_perf(harness_dir: Path, spec: dict[str, Any],
              target_dir: str = "target") -> tuple[bool, str]:
    """Run `gen_perf` and verify every perf input it writes actually runs its
    op end-to-end without crashing.

    gen_perf is otherwise NEVER executed during generation (smoke uses only the
    tiny gen_seeds inputs), so a mis-sized buffer or a bad forward-encode step
    in gen_perf core-dumps only later, at profiling time — the perf inputs ship
    broken. This gate runs gen_perf during generation so such bugs surface as a
    repair prompt instead.
    """
    bin_path = harness_bin(harness_dir, target_dir)
    perf_dir = harness_dir / ".perf_gate"
    if perf_dir.exists():
        shutil.rmtree(perf_dir, ignore_errors=True)
    perf_dir.mkdir(parents=True)

    # 1. gen-perf itself must run to completion (big inputs are where an
    #    encoder/buffer bug crashes).
    rc, _out, err = run_cmd([str(bin_path), "gen-perf", str(perf_dir)],
                            harness_dir, RUN_TIMEOUT * 6)
    if rc != 0:
        shutil.rmtree(perf_dir, ignore_errors=True)
        return False, (
            f"`harness gen-perf` exited {rc} — gen_perf crashes/hangs while "
            f"building a perf input. This is usually a mis-sized output buffer "
            f"in the forward-encode step for a decoder op, or an over-large "
            f"allocation. Size every encoder output buffer from the library's "
            f"bound function (e.g. *_compressBound) and keep total input to a "
            f"few MB. Fix gen_perf so it runs to completion.\n"
            f"stderr:\n{err[-3000:]}")

    # 2. every produced `<op>.perf.bin` must run its op end-to-end (exit 0).
    produced = sorted(perf_dir.glob("*.perf.bin"))
    if not produced:
        shutil.rmtree(perf_dir, ignore_errors=True)
        return False, (
            "gen_perf produced no `<op>.perf.bin` files. Emit ONE large "
            "realistic input per perf-relevant op and return a `perf <op> "
            "<path>` line for each.")
    for pf in produced:
        op = pf.name[: -len(".perf.bin")]
        rc2, _o2, err2 = run_cmd([str(bin_path), op, str(pf), "1"],
                                 harness_dir, RUN_TIMEOUT * 6)
        if rc2 != 0:
            shutil.rmtree(perf_dir, ignore_errors=True)
            return False, (
                f"op `{op}` CRASHES on the perf input gen_perf wrote for it "
                f"(exit {rc2}). That input must be a VALID input the op "
                f"processes end-to-end — for a decoder it must be a real "
                f"encoder output over realistic data, not raw or oversized "
                f"bytes. Fix gen_perf's input for `{op}`.\n"
                f"stderr:\n{err2[-3000:]}")
    shutil.rmtree(perf_dir, ignore_errors=True)
    return True, ""


# ────────────────────────────────────────────────────────────────────
# gate C — coverage of the crate under test
# ────────────────────────────────────────────────────────────────────

@dataclass
class CovResult:
    ok: bool
    line_pct: float = 0.0
    fn_pct: float = 0.0
    uncovered: list[str] = field(default_factory=list)          # exported, count==0
    uncovered_internal: list[str] = field(default_factory=list) # internal own, count==0
    internal_total: int = 0
    error: str = ""


def gate_coverage(harness_dir: Path, inv: CrateInventory,
                  spec: dict[str, Any], min_line: float,
                  min_fn: float) -> CovResult:
    target_dir = "target-cov"
    ok, err = gate_build(harness_dir, target_dir=target_dir,
                         rustflags="-C instrument-coverage -C link-dead-code "
                                   "-C debug-assertions=off -C overflow-checks=off")
    if not ok:
        return CovResult(False, error=f"coverage build failed:\n{err}")

    bin_path = harness_bin(harness_dir, target_dir)
    covdata = harness_dir / "covdata"
    if covdata.exists():
        shutil.rmtree(covdata)
    covdata.mkdir()

    seeds_dir = harness_dir / "seeds"
    ran = 0
    for op in spec["operations"]:
        name = op["name"]
        seeds = [seeds_dir / f"{name}.{i}.bin" for i in (1, 2)] \
            if op.get("needs_input", True) else [seeds_dir / "_none"]
        for i, seed in enumerate(seeds):
            if not seed.exists():
                continue
            rc, _o, e = run_cmd(
                [str(bin_path), name, str(seed)], harness_dir, RUN_TIMEOUT,
                {"LLVM_PROFILE_FILE": str(covdata / f"{name}-{i}-%p.profraw")})
            if rc == 0:
                ran += 1
            else:
                logger.warning("[cov] op %s on %s exited %d: %s",
                               name, seed.name, rc, e[-200:])
    if ran == 0:
        return CovResult(False, error="no operation ran under coverage")

    profdata = harness_dir / "cov.profdata"
    raws = [str(p) for p in sorted(covdata.glob("*.profraw"))]
    rc, _o, err = run_cmd([LLVM_PROFDATA, "merge", "-sparse", "-o",
                           str(profdata)] + raws, harness_dir, COV_TOOL_TIMEOUT)
    if rc != 0:
        return CovResult(False, error=f"{LLVM_PROFDATA} merge failed:\n{err[-2000:]}")

    rc, out, err = run_cmd([LLVM_COV, "export", str(bin_path),
                            f"-instr-profile={profdata}", "-skip-expansions"],
                           harness_dir, COV_TOOL_TIMEOUT)
    if rc != 0:
        return CovResult(False, error=f"{LLVM_COV} export failed:\n{err[-2000:]}")

    return _evaluate_export(out, inv, min_line, min_fn)


_IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*\Z")


def _last_len_prefixed(s: str) -> str | None:
    """Last `<len><identifier>` segment of a mangled name (works for both Rust
    legacy `_ZN..` and v0 `_R..` — llvm-cov emits v0 for internal fns, e.g.
    `_RNvNt..17lodepng_bench_raw3src7lodepng13addChunk_sBIT` → `addChunk_sBIT`)."""
    best = None
    i, n = 0, len(s)
    while i < n:
        if s[i].isdigit():
            j = i
            while j < n and s[j].isdigit():
                j += 1
            ln = int(s[i:j])
            if 0 < ln and j + ln <= n and _IDENT.match(s[j:j + ln]):
                best = s[j:j + ln]
                i = j + ln
                continue
            i = j
        else:
            i += 1
    return best


def _demangle_base_any(name: str) -> str:
    if "::" in name:                       # already-demangled path
        return name.rsplit("::", 1)[-1]
    if name.startswith(("_R", "_Z")):      # v0 / legacy mangled
        return _last_len_prefixed(name) or name
    return name


def _evaluate_export(export_json: str, inv: CrateInventory,
                     min_line: float, min_fn: float) -> CovResult:
    data = json.loads(export_json)["data"][0]
    crate_prefix = str(inv.crate_dir)

    lines_total = lines_cov = 0
    for f in data.get("files", []):
        if not f.get("filename", "").startswith(crate_prefix):
            continue
        s = f.get("summary", {}).get("lines", {})
        lines_total += s.get("count", 0)
        lines_cov += s.get("covered", 0)

    # Function coverage over the crate's exported API. llvm-cov names each
    # record by the EXPORTED SYMBOL, which is the Rust fn name for
    # #[no_mangle] fns but the C name for #[export_name] fns (c2rust renames
    # keyword-colliding fns like C `match` → Rust `match_0`, exported as
    # `match`). So we match on link_name (the symbol), not the Rust name,
    # and report uncovered by the human-readable Rust name.
    link_to_name = inv.link_to_name          # symbol → Rust name
    api = set(link_to_name)                  # exported symbols
    executed: set[str] = set()               # symbols with count > 0
    present: set[str] = set()
    # internal (originally-C-static) own functions: c2rust emits them as
    # non-pub `unsafe extern "C" fn`, so inventory (pub-only) never sees them,
    # yet they hold the real hot kernels (unfilter, encodeLZ77, addChunk_*).
    # llvm-cov demangles them to `<crate>::mod::fn`; -C link-dead-code keeps
    # the uncalled ones with count 0. Track them so the LLM can be pushed to
    # reach them too (soft signal — exported coverage stays the hard gate).
    crate_pfx = inv.crate_name + "::"
    crate_tok = f"{len(inv.crate_name)}{inv.crate_name}"   # len-prefixed (v0/legacy)
    internal_exec: dict[str, bool] = {}      # base name -> executed?
    for fn in data.get("functions", []):
        name = fn.get("name", "")
        cnt = fn.get("count", 0)
        # llvm-cov may report a demangled path; take the last path segment
        # and also try the raw name, matching either against the symbol set.
        cand = {name, name.rsplit("::", 1)[-1]}
        hit = next((c for c in cand if c in api), None)
        if hit:
            present.add(hit)
            if cnt > 0:
                executed.add(hit)
            continue
        # own internal function (non-pub, mangled v0 `_R..` in llvm-cov output)
        if name.startswith(crate_pfx) or crate_tok in name:
            base = _demangle_base_any(name)
            internal_exec[base] = internal_exec.get(base, False) or (cnt > 0)

    denom = len(api) if api else 1
    fn_pct = 100.0 * len(executed) / denom
    line_pct = 100.0 * lines_cov / lines_total if lines_total else 0.0
    uncovered = sorted(link_to_name[s] for s in (api - executed))
    uncovered_internal = sorted(b for b, ex in internal_exec.items() if not ex)
    ok = line_pct >= min_line and fn_pct >= min_fn      # exported = hard gate
    if not present:
        return CovResult(False, line_pct, 0.0, sorted(link_to_name.values()),
                         error="no exported function appears in coverage data "
                               "(unexpected — check link-dead-code)")
    return CovResult(ok, line_pct, fn_pct, uncovered,
                     uncovered_internal=uncovered_internal,
                     internal_total=len(internal_exec))
