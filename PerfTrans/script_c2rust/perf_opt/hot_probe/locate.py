"""hot_probe locate phase — which crate functions did the workload push HOT.

**Pure workload sampling** — 5-rule signature detection lives in
`class_I.scan` + `class_II.scan` and is combined by the driver AFTER
locate. Locate's job: perf-record → deepest-crate-frame self% → hot fn list.

Per op: `perf record` (sampling, no call-graph → SELF-time, so a fn hot only
because it calls a hot callee is NOT counted) the harness on the op's perf
input, attribute samples to the crate-under-test's own functions (libc / the
harness's own code filtered out), aggregate each fn's MAX self-time% across
ops, keep `self% ≥ τ` (top-N capped), and resolve each to a source location.

Attribution uses `crate_selftime` (call-graph + inline, deepest crate frame),
which is robust to a crate kernel LTO-inlined into the harness wrapper and to a
crate function delegating to libc — both of which symbol self-time misses. The
build path (2_stage_a's adapted harness, built with debug info) is set up by the
caller (driver).

hotspots.json shape (post 5-rule migration):

  {
    "crate": ..., "tau": ..., "ops": [...],
    "hot_functions":  [ HotFunction.to_dict() ... ],
    "dropped_wrappers": [ HotFunction.to_dict() + "callers": [...] ... ],
    "unresolved_symbols": [ HotFunction.to_dict() ... ]
  }

`class_i_hits` / `class_ii_hits` fields on hot_functions are EMPTY at write
time; the driver rewrites hotspots.json AFTER running `class_I.scan` +
`class_II.scan` + `augment_hot_fns` to fill them.
"""

from __future__ import annotations

import json
import logging
import re
from pathlib import Path

from harness_gen.inventory import scan_crate
from perf_opt.verify.measure import autotune_iters, pick_input
from perf_opt.hot_probe.selftime import crate_selftime
from perf_opt.hot_probe.symbol_source import build_fn_index
from perf_opt.hot_probe.types import HotFunction

logger = logging.getLogger("hot_probe.locate")

# perf record wall per op — longer than a single measured run so enough samples
# accumulate for a stable self-time distribution (frequency-independent).
LOCATE_TARGET_WALL = 2.0


def _build_callers_index(crate: Path, wrapper_names: set[str],
                         index) -> dict[str, set[str]]:
    """{wrapper_name: {caller_fn, ...}} — for each dropped wrapper, list the
    crate functions whose bodies call it. Uses `index.all_sources()` and a
    plain word-boundary search so a wrapper `strcasechr` matches a caller's
    `strcasechr(c)` but not a substring. Empty set for a wrapper called from
    macros / cold init / FFI-only edges."""
    if not wrapper_names:
        return {}
    all_src = index.all_sources()
    patterns = {w: re.compile(rf"(?<![.\w]){re.escape(w)}\s*\(")
                for w in wrapper_names}
    result: dict[str, set[str]] = {w: set() for w in wrapper_names}
    for caller, src in all_src.items():
        if caller in wrapper_names:
            continue                     # wrappers don't count as callers
        for w, pat in patterns.items():
            if pat.search(src):
                result[w].add(caller)
    return result


def _package_name(crate: Path) -> str:
    """The crate's package name — the mangling prefix of its own internal
    symbols (`_ZN<pkg>..`). MUST be the package name, not a dependent's dep
    alias (fzy harness aliases `fzy_raw` but the crate is `fzy_cleaned`)."""
    txt = (crate / "Cargo.toml").read_text(encoding="utf-8", errors="replace")
    m = re.search(r'^\s*\[package\][^\[]*?^\s*name\s*=\s*"([^"]+)"',
                  txt, re.M | re.S)
    return m.group(1) if m else crate.name


def locate_hotspots(*, crate: Path, harness_bin: Path, assets,
                    ops: list[str], tau: float = 3.0, top_n: int = 15,
                    opt_dir: Path,
                    target_wall: float = LOCATE_TARGET_WALL,
                    ) -> list[HotFunction]:
    """Locate the crate's hot functions via workload perf sampling.

    Writes `opt/hotspots.json` (initial version, with empty class_i_hits /
    class_ii_hits — driver rewrites after scan). Returns the ranked
    `HotFunction` list.

    Ops that error on their perf input are skipped (already flagged by the
    driver's baseline). Post-filter: extern-wrapper fns and unresolved
    macro-generated symbols are moved to separate sections and NOT returned
    as hot fns."""
    crate_name = _package_name(crate)
    inv = scan_crate(crate)
    exported = set(inv.link_names) | set(inv.fn_names)
    scratch = opt_dir / ".hot_probe"
    scratch.mkdir(parents=True, exist_ok=True)
    index = build_fn_index(crate)
    resolvable = index.names()   # roll macro-generated frames up to editable callers

    best: dict[str, tuple[float, str]] = {}     # fn -> (max self%, op)
    per_op: dict[str, dict[str, float]] = {}    # fn -> {op: self%}
    for op in ops:
        inp = pick_input(assets.harness_src, op)
        if inp is None:
            continue
        iters = autotune_iters(harness_bin, op, inp, target_wall=target_wall)
        if iters == 0:
            logger.warning("[locate] %s skip (errors on input)", op)
            continue
        fn_pct = crate_selftime(harness_bin, op, inp, iters, scratch,
                                crate_name, exported, resolvable=resolvable)
        own = round(sum(fn_pct.values()), 1)
        top = sorted(fn_pct.items(), key=lambda kv: -kv[1])[:3]
        logger.info("[locate] %-22s own=%.0f%% top=%s", op, own,
                    ", ".join(f"{f}:{p:.0f}" for f, p in top))
        for fn, pct in fn_pct.items():
            per_op.setdefault(fn, {})[op] = round(pct, 2)
            if fn not in best or pct > best[fn][0]:
                best[fn] = (pct, op)

    # Rank all fns ≥ tau, materialize records + source locations.
    ranked = sorted(((fn, p, o) for fn, (p, o) in best.items() if p >= tau),
                    key=lambda x: -x[1])
    all_records: list[HotFunction] = []
    for fn, pct, op in ranked:
        loc = index.resolve(fn)
        all_records.append(HotFunction(
            name=fn, self_pct=round(pct, 2), hottest_op=op,
            per_op=per_op.get(fn, {}),
            file=loc[0] if loc else None,
            line_start=loc[1] if loc else None,
            line_end=loc[2] if loc else None))

    # Flag libc/syscall wrappers by inspecting each fn's source body — a
    # 4-line `strpbrk` shim has no code the rewrite rules can act on, and its
    # sampled time is really spent inside libc. Filter these out BEFORE the
    # top_n cap so real editable functions get the slots. Dropped fns stay in
    # `dropped_wrappers` for transparency.
    from perf_opt.hot_probe.wrapper_detect import mark_wrappers
    flagged = mark_wrappers(all_records, index)
    dropped = [hf for hf in all_records if hf.extern_wrapper]

    # Drop hot fns with no source location — a macro-generated accessor (e.g.
    # `#[bitfield]`-generated `status_code`) that the depth-first roll-up
    # could not attribute to any editable caller in some samples. Its evidence
    # pack would be `location=null, rust_source=""`, unusable for any rule.
    # Kept in `unresolved_symbols` for transparency; NEVER shipped as a hot fn.
    non_wrapper = [hf for hf in all_records if not hf.extern_wrapper]
    unresolved = [hf for hf in non_wrapper if hf.file is None]
    editable = [hf for hf in non_wrapper if hf.file is not None]
    # top_n <= 0 → no cap: keep every fn that cleared the tau (self%) floor.
    result = editable[:top_n] if top_n > 0 else editable

    # Enrich `dropped_wrappers` with their crate-level callers so the agent
    # can consider caller-side rewrites (inline the wrapper, replace the
    # libc call inside the caller loop). Missing callers → wrapper is called
    # from cold code / macros / an FFI edge; harmless empty list.
    callers_index = _build_callers_index(crate, {hf.name for hf in dropped},
                                          index)
    dropped_out = []
    for hf in dropped:
        d = hf.to_dict()
        d["callers"] = sorted(callers_index.get(hf.name, set()))
        dropped_out.append(d)

    logger.info("[locate] %d hot fn(s) ≥ %.1f%% (of %d seen); "
                "%d extern_wrapper(s) dropped; %d unresolved dropped",
                len(result), tau, len(best), flagged, len(unresolved))
    if unresolved:
        logger.warning("[locate] dropped unresolved (no source location): %s",
                       [hf.name for hf in unresolved])
    if dropped:
        logger.info("[locate] dropped wrappers (delegate to libc/syscall): %s",
                    [f"{hf.name}({hf.self_pct}%→{hf.wrapper_target})"
                     for hf in dropped])

    # Everything the profile saw but tau turned away. Kept because `hot(s)`
    # is a self-time predicate and some rules are not paid in self-time:
    # a per-call fixed cost (an oversized zero-init in an `*_init` /
    # `*_reset`) is by construction spread thin, so it never clears tau even
    # when it dominates on small inputs. `fixed_cost_admit` reads this table
    # to give such a function the real per-op numbers it needs for W2 — the
    # alternative, inventing an op set, would be guessing.
    hot_names = {hf.name for hf in result}
    sub_tau = {fn: ops_pct for fn, ops_pct in per_op.items()
               if fn not in hot_names
               and any(v >= 0.01 for v in ops_pct.values())}

    (opt_dir / "hotspots.json").write_text(json.dumps({
        "crate": crate_name, "tau": tau, "ops": ops,
        "hot_functions": [hf.to_dict() for hf in result],
        "dropped_wrappers": dropped_out,
        "unresolved_symbols": [hf.to_dict() for hf in unresolved],
        "sub_tau_profile": sub_tau,
    }, indent=2))
    logger.info("[locate] sub_tau_profile: %d fn(s) seen below tau kept "
                "for fixed-cost admission", len(sub_tau))
    return result


def rewrite_hotspots_with_hits(opt_dir: Path,
                                hot_fns: list[HotFunction]) -> None:
    """Re-serialize hotspots.json after the driver augments each HotFunction
    with class_i_hits + class_ii_hits (via `types.augment_hot_fns`).

    hotspots.json shape unchanged — hot_functions[i] just gets non-empty
    `class_i_hits` and `class_ii_hits` fields. Other sections
    (dropped_wrappers / unresolved_symbols) preserved as-is."""
    p = opt_dir / "hotspots.json"
    if not p.is_file():
        logger.warning("[locate] hotspots.json missing, cannot augment")
        return
    d = json.loads(p.read_text())
    # Replace hot_functions with augmented dicts
    d["hot_functions"] = [hf.to_dict() for hf in hot_fns]
    p.write_text(json.dumps(d, indent=2))
    with_hits = sum(1 for hf in hot_fns
                    if hf.class_i_hits or hf.class_ii_hits)
    logger.info("[locate] hotspots.json augmented: %d/%d hot fn(s) with "
                "5-rule signals", with_hits, len(hot_fns))
