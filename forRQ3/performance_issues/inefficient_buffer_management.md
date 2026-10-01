# Inefficient Buffer Management

Primary manifestation: Recurring source-level patterns.

## Definition

Repeated allocation or fixed-increment growth incurs avoidable heap-allocation overhead.

## Observed Pattern

An internal buffer is repeatedly allocated and freed, or grows by fixed increments through `realloc`.

## Optimization Direction

Reuse buffers to reduce allocations; reserve capacity upfront or use geometric growth to reduce reallocations.
