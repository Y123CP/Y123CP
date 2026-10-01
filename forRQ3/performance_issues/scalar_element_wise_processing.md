# Scalar Element-Wise Processing

Primary manifestation: Recurring source-level patterns.

## Definition

Element-wise processing leaves parallel or block-wise computation opportunities unused.

## Observed Pattern

A loop processes adjacent elements individually (e.g. `out[i] = f(in[i])`), or individually updates or compares elements.

## Optimization Direction

Process independent elements in groups, or replace per-element updates and comparisons with equivalent block-wise operations.
