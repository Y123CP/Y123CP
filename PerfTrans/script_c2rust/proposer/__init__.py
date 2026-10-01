"""proposer — Rust source-side analyses that feed the future rule layer.

Currently:
  - `proposer.syn_walker` — syn-based Rust AST walker exposing
    `list_structs`, `list_allocs`, `list_loop_invariant_conds`.

Used by `profiling.pipeline` (loop-invariant cond detection) and by the
future rule dispatcher; not part of the §2.5 HotspotProfile itself.
"""
