# Obscured Memory Access Properties

Primary manifestation: Recurring source-level patterns.

## Definition

Insufficiently exposed access relationships and invariants inhibit load elimination or hoisting.

## Observed Pattern

A loop accesses buffers through raw pointers (e.g. `ptr.add(i)`), with separately tracked bounds and implicit aliasing relationships.

## Optimization Direction

Recover valid `&[T]`/`&mut [T]` views to expose access properties to LLVM, or hoist loop-invariant loads.
