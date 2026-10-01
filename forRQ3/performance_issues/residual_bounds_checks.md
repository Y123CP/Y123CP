# Residual Bounds Checks

Primary manifestation: Residual runtime work.

## Definition

Bounds checks remain even when established index-range invariants prove them unnecessary.

## Observed Pattern

Loop invariants establish valid indices, but optimized Rust IR retains checks leading to `panic_bounds_check` that are absent from C IR.

## Optimization Direction

Express valid iteration bounds through slices or iterators to expose index–length relationships and enable elimination of redundant checks.
