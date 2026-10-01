"""Stage 1 — top-level item de-duplication and path normalization.

Scope (what this stage actually does):

  1. Move type aliases / repr(C) structs / constants / opaque extern types /
     libc fn declarations + statics that c2rust emits in EVERY consuming TU
     into single shared modules under `src/`. After Stage 1, every such
     item is defined exactly once.

  2. Normalize `::core::ffi::c_X` paths inline to `c_X` (now that
     `use core::ffi::*;` is a stable header in every shared module).

That is all. Stage 1 is intentionally narrow:

  - It does NOT rewrite function bodies, signatures, or expressions.
  - It does NOT remove `as c_int` literal casts, `mut` keyword spam, or
    `unsafe extern "C" fn` on internal-only functions — those are deferred
    to Stage 2 (safety lift / D1.1.5 strip-extern / unsafe minimizer).
  - It does NOT lift `*mut T`/`*const T` to references — Stage 2 / Stage 3.
  - It does NOT touch idiomatic-Rust transformations — Stage 3.

The dedup passes use the ≥2-file rule (an item must appear in two or more
TUs before it gets migrated). The libc/POSIX-fn pass also relaxes this to
single-file when the signature only references types already in scope —
this catches per-binary libc decls (e.g. bzip2.rs's local `fn open(...);`
block) without breaking signatures that touch TU-local types.

Pass order (chained scope: each pass's migrated names feed the next):

  Milestone A+B (top-level item dedup):
    1.1a  type-alias dedup       → src/c_types.rs        (primitives ⇒ no deps)
    1.1b  repr(C) struct dedup   → src/c_structs.rs      (fields ref types/extern types)
    1.1c  constant dedup         → src/c_consts.rs       (refs types & structs)

  Milestone C (extern block integration):
    1.1d-1 opaque extern types   → src/c_extern_types.rs (UNLOCKS _IO_FILE → c_structs)
    1.1d-2 libc fns + statics    → src/ffi.rs            (≥2-file dedup +
                                                          single-file libc with
                                                          resolvable signatures)

  Final:
    1.1f  path normalization     `::core::ffi::c_X` → `c_X`

Pass 1.1d-1 runs FIRST so its migrated names are in scope when struct dedup
later checks `_IO_FILE { _markers: *mut _IO_marker, ... }`. Pass 1.1d-2 runs
LAST among the dedup passes so libc fn signatures benefit from all the type
aliases / structs already migrated to shared modules.
"""

from __future__ import annotations

import argparse
import logging
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from stages.cleanup.binary_cleanup import cleanup_binaries
from stages.cleanup.c2rust_compat import apply_all as apply_c2rust_compat_patches
from stages.cleanup.cargo_utils import read_pkg_name
from stages.cleanup.conflict_set import compute_conflict_set
from stages.cleanup.dedup import DedupConfig, dedup_pass
from stages.cleanup.extern_dedup import dedup_extern_fns, dedup_extern_types
from stages.cleanup.extern_prune import prune_unused_externs
from stages.cleanup.fn_dedup import dedup_top_level_fns
from stages.cleanup.path_norm import normalize_paths
from stages.stage0_c2rust import Stage0C2Rust
from stages.stage_base import StageBase

logger = logging.getLogger(__name__)


# Top-level item dedup passes (Milestones A+B). Each pass's migrated names
# enter `scope` so later passes can migrate items that reference them
# (e.g. `pub type FILE = _IO_FILE;` migrates after `_IO_FILE` is in scope).
# Type-aliases run TWICE: once before structs (basic primitives), once
# after structs to pick up alias chains like `FILE → _IO_FILE`.
_DEDUP_PASSES: tuple[DedupConfig, ...] = (
    DedupConfig(item_kinds=("type_item",),   dest_module="c_types",
                label="type aliases",        imports=("c_extern_types",)),
    DedupConfig(item_kinds=("struct_item",), dest_module="c_structs",
                label="repr(C) structs",     imports=("c_types", "c_extern_types")),
    # Milestone D: enum/union dedup. Runs after struct so any enum/union
    # variant that references a struct in c_structs.rs can resolve. Both
    # land in c_structs.rs because (a) they share the same `repr(C)`
    # layout role in the c2rust output, (b) the existing Pre-Stage-A
    # `unify_duplicate_pub_types` already treats enum / union / struct
    # uniformly — Stage 1 mirrors that.
    DedupConfig(item_kinds=("enum_item",),   dest_module="c_structs",
                label="enums",               imports=("c_types", "c_extern_types")),
    DedupConfig(item_kinds=("union_item",),  dest_module="c_structs",
                label="unions",              imports=("c_types", "c_extern_types")),
    DedupConfig(item_kinds=("type_item",),   dest_module="c_types",
                label="type aliases (r2)",   imports=("c_extern_types", "c_structs")),
    DedupConfig(item_kinds=("const_item",),  dest_module="c_consts",
                label="constants",           imports=("c_types", "c_structs", "c_extern_types")),
    # Milestone D: top-level immutable `static` dedup. `static mut` is
    # rejected inside _select_migrations (semantic-change risk: one cell
    # per TU vs one shared cell). Lands in c_consts.rs alongside const
    # since they share the "compile-time constant data" role and tmux's
    # 475 const-dup audit hit suggests they cluster in the same module.
    DedupConfig(item_kinds=("static_item",), dest_module="c_consts",
                label="immutable statics",   imports=("c_types", "c_structs", "c_extern_types")),
)


class Stage1Cleanup(StageBase):
    name = "stage1_cleanup"
    requires_oracle = False
    requires_microbench = False
    # 2026-05-29: switch to soft gate. Each cleanup pass is now wrapped
    # by `run_with_step_rollback`, which restores any pass that grows
    # the cargo-check error count. So the final build's residual errors
    # are inherent to c2rust's output (e.g. brotli's `usize → u64`
    # implicit casts), NOT introduced by our cleanup. Throwing away all
    # the genuine progress (m128i_u fix etc.) under strict gate destroys
    # signal that downstream stages need. Soft gate: preserve 1_cleaned
    # for inspection; return ok=False so callers know it didn't build.
    strict_cargo_gate = False

    def _apply(self, project_path: Path) -> None:
        # Read crate name once; needed for cross-crate `use ::<crate>::src::*;`
        # imports when binaries participate in dedup.
        crate = read_pkg_name(project_path / "Cargo.toml")

        def step_apply_compat(p: Path) -> None:
            """c2rust 0.22.1 known output bug patches (m128i_u, etc.).
            See stages/cleanup/c2rust_compat.py for the bug taxonomy."""
            apply_c2rust_compat_patches(p)

        def step_promote_main(p: Path) -> None:
            """Promote `pub fn main()` TUs into Cargo `[[bin]]` entries.
            Stage 0 normally does this, but cleanup may run on an older
            0_raw. Idempotent."""
            promoted = Stage0C2Rust._promote_main_tus(p, crate)
            if promoted:
                logger.info(f"[promote] added [[bin]] for: {promoted}")

        def step_dedup_block(p: Path) -> None:
            """Atomic block: conflict-set scan + extern-types + DEDUP_PASSES
            + extern-fns. Wrapped as one because they share `scope` /
            `created` state — partial rollback inside this block would
            leave the in-memory tracking inconsistent with disk."""
            scope: set[str] = set()
            created: set[str] = set()
            conflicts = compute_conflict_set(p / "src")
            if dedup_extern_types(p, scope, include_bins=True,
                                   crate_name=crate, conflicts=conflicts) > 0:
                created.add("c_extern_types")
            for cfg in _DEDUP_PASSES:
                live_imports = tuple(m for m in cfg.imports if m in created)
                cfg = DedupConfig(
                    item_kinds=cfg.item_kinds, dest_module=cfg.dest_module,
                    label=cfg.label, imports=live_imports,
                )
                if dedup_pass(p, cfg, scope=scope,
                              include_bins=True, crate_name=crate,
                              conflicts=conflicts) > 0:
                    created.add(cfg.dest_module)
            if dedup_extern_fns(p, scope, include_bins=True,
                                 crate_name=crate, conflicts=conflicts) > 0:
                created.add("ffi")
            # Milestone D — top-level fn dedup. Runs AFTER all type / const
            # / extern passes have populated `scope`, so a fn body can
            # reference (e.g.) `FILE`, `Int32`, `Bool`, `BZ_OK` etc. and
            # still pass the leaf check. c2rust per-TU inline helpers
            # (BrotliUnaligned*, myfeof, FastLog2, …) collapse here.
            if dedup_top_level_fns(p, scope, include_bins=True,
                                    crate_name=crate, conflicts=conflicts) > 0:
                created.add("c_inlined_fns")

        def step_prune(p: Path) -> None:
            """Drop extern decls that aren't referenced in this file."""
            prune_unused_externs(p)

        def step_cleanup_bins(p: Path) -> None:
            """Bring binary TUs in line with shared modules — safety net."""
            cleanup_binaries(p)

        def step_normalize_paths(p: Path) -> None:
            """Collapse `::core::ffi::c_X` to bare `c_X`."""
            n = normalize_paths(p)
            if n > 0:
                logger.info(f"[paths] normalized ::core::ffi paths in {n} file(s)")

        # Per-step rollback: each pass that grows the cargo-check error
        # count is reverted, the rest land. Without this, a downstream
        # pass that touches a corner case in one project wipes out the
        # genuine progress every earlier pass made (observed on brotli:
        # `apply_c2rust_compat_patches` correctly fixes 3× `__m128i_u`
        # but the final strict gate sees residual `usize → u64` errors
        # from c2rust's missing-cast output bug, rolls back ALL of
        # _apply → 1_cleaned loses the m128i fix, brotli looks worse
        # than it actually is).
        from stages.cleanup.step_rollback import run_with_step_rollback
        run_with_step_rollback(
            project_path,
            steps=[
                ("apply_c2rust_compat_patches", step_apply_compat),
                ("promote_main_tus", step_promote_main),
                ("dedup_block", step_dedup_block),
                ("prune_unused_externs", step_prune),
                ("cleanup_binaries", step_cleanup_bins),
                ("normalize_paths", step_normalize_paths),
            ],
            cargo_check_dir=project_path,
            log_prefix=self.name,
        )


def _cli():
    p = argparse.ArgumentParser(description="Stage 1: structural cleanup")
    p.add_argument("--input",  required=True)
    p.add_argument("--output", required=True)
    p.add_argument("--skip-oracle", action="store_true")
    args = p.parse_args()
    logging.basicConfig(level=logging.INFO,
                        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
                        datefmt="%H:%M:%S")
    result = Stage1Cleanup(args=args).run(Path(args.input), Path(args.output))
    print(result)
    sys.exit(0 if result.ok else 1)


if __name__ == "__main__":
    _cli()
