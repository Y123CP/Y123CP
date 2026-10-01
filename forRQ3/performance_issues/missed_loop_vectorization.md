# Missed Loop Vectorization

Primary manifestation: Missed compiler transformations.

## Definition

Loop vectorization achieved in C is lost after translation to Rust.

## Observed Pattern

LLVM vectorizes a C loop but leaves its Rust counterpart scalar; remarks identify control-flow, call, or memory-dependence barriers.

## Optimization Direction

Restructure blocking control flow or calls, or expose valid dependence information to enable loop vectorization.
