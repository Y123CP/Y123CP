# Repeated Invariant Dispatch

Primary manifestation: Recurring source-level patterns.

## Definition

Loops dispatch on callbacks or mode values that remain invariant across iterations.

## Observed Pattern

A loop repeatedly invokes an invariant callback, e.g. `cb.expect(...)(args)`, or branches on an invariant mode.

## Optimization Direction

Resolve loop-invariant callback or mode dispatch once before the loop, then execute a specialized loop for the selected target or mode.
