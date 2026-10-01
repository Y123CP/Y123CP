"""Pre-Stage-A working-copy preparation — orchestrator.

Stage A's E1 strip of `pub unsafe extern "C"` on lib fns exposes any
nominal-type duplication that the c2rust per-TU translation left in
the working copy. The orchestrator runs a fixed pipeline of cleanup
transforms — each wrapped by `step_rollback` — to fold those
duplicates into shared canonicals BEFORE Stage A starts.

Two flavours of pipeline, picked at call-site by whether a harness
crate is supplied:

  · harness path (5 steps): canonicalize → unify_duplicate →
    unify_opaque → normalize_harness_extern → strip_staticlib.
    Gate is cargo-check on the harness (that's what we ultimately
    build).

  · self-bin path (4 steps): canonicalize → unify_duplicate →
    normalize_self_bin_extern → strip_staticlib. Gate is cargo-check
    on the crate itself.

Each step is snapshotted (lib crate + harness if present) and rolled
back if it grows the cargo-check error count. Survivors land; failures
don't poison downstream steps.
"""

from __future__ import annotations

import logging
from pathlib import Path

from .anon_types import canonicalize_c2rust_anon_types
from .pre_stage_a_extern import (
    normalize_harness_extern_blocks,
    normalize_self_bin_extern_blocks,
    strip_staticlib_from_crate_type,
)
from .pub_type_unify import unify_duplicate_pub_types, unify_opaque_foreign_types
from .step_rollback import run_with_step_rollback

logger = logging.getLogger(__name__)


def prep_for_stage_a(crate_dir: Path, harness_dir: Path | None = None) -> None:
    """Run the pre-Stage-A cleanup pipeline on `crate_dir`.

    If `harness_dir` is given, the crate is consumed via a sibling
    harness crate (`workload["harness_dir"]`) — the harness extern
    block pass runs and the cargo-check gate is the harness. If
    `harness_dir` is None, the lib hosts a self-contained `[[bin]]`
    (e.g. bzip2-1.0.8's bzip2.rs binary) — the self-bin variant runs
    and the gate is the crate itself.

    The harness extern step needs the harness path as a closure
    argument — wrap it in a lambda so `run_with_step_rollback`'s
    uniform `fn(crate_dir)` signature is preserved.

    Rollback context — each named step is reverted if its `cargo check`
    error count exceeds the prior baseline. See `step_rollback` module
    docstring for why this matters: tmux's `unify_duplicate_pub_types`
    introduces a `pub use crate::environ::environ` that clashes with
    a libc `extern { static mut environ }` (+2 errors); the rest of the
    chain is fine and survives the rollback.
    """
    if harness_dir is not None:
        run_with_step_rollback(
            crate_dir,
            steps=[
                ("canonicalize_c2rust_anon_types", canonicalize_c2rust_anon_types),
                ("unify_duplicate_pub_types",      unify_duplicate_pub_types),
                ("unify_opaque_foreign_types",     unify_opaque_foreign_types),
                ("normalize_harness_extern_blocks",
                    lambda p: normalize_harness_extern_blocks(p, harness_dir)),
                ("strip_staticlib_from_crate_type", strip_staticlib_from_crate_type),
            ],
            extra_snapshot_dirs=[harness_dir],
            cargo_check_dir=harness_dir,  # harness is what we actually build
            log_prefix="prep-working-copy(harness)",
        )
    else:
        run_with_step_rollback(
            crate_dir,
            steps=[
                ("canonicalize_c2rust_anon_types",   canonicalize_c2rust_anon_types),
                ("unify_duplicate_pub_types",        unify_duplicate_pub_types),
                ("normalize_self_bin_extern_blocks", normalize_self_bin_extern_blocks),
                ("strip_staticlib_from_crate_type",  strip_staticlib_from_crate_type),
            ],
            cargo_check_dir=crate_dir,
            log_prefix="prep-working-copy(self-bin)",
        )
