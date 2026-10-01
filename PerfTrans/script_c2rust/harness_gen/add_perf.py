"""Retrofit `gen_perf` onto an already-generated harness.

Existing harnesses (built before the gen-perf contract) export `ops()` +
`gen_seeds` but not `gen_perf`. This adds `gen_perf` via one targeted LLM
call: the model sees the current lib.rs (so it knows each op's input
format) and appends a `pub fn gen_perf(dir) -> Vec<String>` that writes
ONE large realistic input per perf-relevant op. It also refreshes the
static main.rs shell (which now routes `gen-perf`).

Gates: build → run `harness gen-perf` produces ≥1 non-empty file. Bounded
LLM repair on build failure. Idempotent — skips if gen_perf already
present and building.
"""

from __future__ import annotations

import logging
import re
import sys
from pathlib import Path

_SCRIPT_ROOT = Path(__file__).resolve().parent.parent
if str(_SCRIPT_ROOT) not in sys.path:
    sys.path.insert(0, str(_SCRIPT_ROOT))

from harness_gen import gates, prompts          # noqa: E402
from harness_gen.agent import MAIN_RS_TEMPLATE  # noqa: E402

logger = logging.getLogger(__name__)

ADD_PERF_SYSTEM = (
    "You are an expert Rust systems engineer. You extend an existing "
    "workload-harness lib.rs by ADDING one function; you output the ONE "
    "complete updated src/lib.rs in a single ```rust block and nothing else."
)


def _user(lib_rs: str, harness_name: str) -> str:
    return f"""\
This is the current `src/lib.rs` of a workload harness for a
c2rust-translated crate. It already exports `ops()` and `gen_seeds`.

ADD a `pub fn gen_perf(dir: &str) -> Vec<String>` with this contract:

{prompts.CONTRACT}

Only the `gen_perf` contract paragraph above is new — keep `ops()`,
`gen_seeds`, every `op_*` runner and all imports EXACTLY as they are.
Follow the `gen_perf` contract above (LARGE + REALISTIC, typically several
MB, kernel-dominant, VALID inputs, structured/redundant not random; for a
decoder produce its input via the forward encoder). OMIT operations whose
cost does not scale with input (config / option / getter ops). Reuse the
same input FORMAT each op already parses (study its `op_*` runner to match
how it splits the bytes). Return one `perf <op> <path>` line per file written.

CURRENT src/lib.rs:
```rust
{lib_rs}
```

Output the complete updated src/lib.rs (with gen_perf appended) in one
```rust block."""


def retrofit(hdir: Path, model: str | None = None, max_repairs: int = 3) -> bool:
    lib = hdir / "src" / "lib.rs"
    lib_rs = lib.read_text(encoding="utf-8")

    # refresh the static shell so `gen-perf` is routed
    harness_name = re.sub(r"[^a-z0-9_]", "_", hdir.name.lower())
    # derive the real lib name from Cargo.toml [lib] name
    cargo = (hdir / "Cargo.toml").read_text(encoding="utf-8")
    m = re.search(r'\[lib\][^\[]*?name\s*=\s*"([^"]+)"', cargo, re.S)
    if m:
        harness_name = m.group(1)
    (hdir / "src" / "main.rs").write_text(
        MAIN_RS_TEMPLATE.format(harness_name=harness_name), encoding="utf-8")

    if re.search(r"pub\s+fn\s+gen_perf\s*\(", lib_rs):
        ok, err = gates.gate_build(hdir)
        if ok:
            logger.info("[add_perf] gen_perf already present and builds; skip")
            return True
        logger.info("[add_perf] gen_perf present but build broken — regenerating")

    from Config.paths import get_path
    from utils.llm_client import LLMClient
    model = model or get_path("DEFAULT_LLM_MODEL")
    logs = hdir.parent / "logs"
    llm = LLMClient(model, transcript_path=logs / "transcript.log",
                    cache_path=logs / "cache.jsonl", timeout=180)

    user = _user(lib_rs, harness_name)
    for attempt in range(max_repairs + 1):
        phase = "add-perf" if attempt == 0 else f"add-perf-repair-{attempt}"
        logger.info("[add_perf] LLM call (%s)", phase)
        out = llm.chat(ADD_PERF_SYSTEM, user, meta={"phase": phase})
        new_lib = prompts.extract_rust(out)
        if not new_lib or "gen_perf" not in new_lib:
            logger.warning("[add_perf] reply missing gen_perf/rust block")
            continue
        lib.write_text(new_lib, encoding="utf-8")

        ok, err = gates.gate_build(hdir)
        if not ok:
            user = prompts.repair_user("build", err, new_lib)
            continue

        # smoke: gen-perf must produce ≥1 non-empty file
        bin_path = gates.harness_bin(hdir)
        # gate_build produces debug; ensure bin exists
        pdir = hdir / "perf_inputs"
        rc, o, e = gates.run_cmd([str(bin_path), "gen-perf", str(pdir)],
                                 hdir, 300)
        produced = [ln for ln in o.splitlines() if ln.startswith("perf ")]
        nonempty = any((pdir / f"{ln.split()[1]}.perf.bin").exists()
                       and (pdir / f"{ln.split()[1]}.perf.bin").stat().st_size > 1000
                       for ln in produced)
        if rc == 0 and produced and nonempty:
            logger.info("[add_perf] OK — gen-perf produced %d input(s): %s",
                        len(produced), [ln.split()[1] for ln in produced])
            return True
        logger.warning("[add_perf] gen-perf ran but produced no large input "
                       "(rc=%d, lines=%d)", rc, len(produced))
        user = (f"The harness built, but `harness gen-perf <dir>` did not "
                f"write any large (>1KB) `<op>.perf.bin`. rc={rc}, stderr:\n"
                f"{e[-1500:]}\nFix gen_perf so it writes at least one large "
                f"realistic input. Output the complete src/lib.rs.")
    logger.error("[add_perf] failed after %d attempts", max_repairs)
    return False


def _cli() -> int:
    import argparse
    ap = argparse.ArgumentParser(prog="harness_gen.add_perf")
    ap.add_argument("--harness", required=True)
    ap.add_argument("--model", default=None)
    ap.add_argument("-v", "--verbose", action="store_true")
    args = ap.parse_args()
    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s", datefmt="%H:%M:%S")
    hdir = Path(args.harness).resolve()
    ok = retrofit(hdir, model=args.model)
    print("OK" if ok else "FAILED")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(_cli())
