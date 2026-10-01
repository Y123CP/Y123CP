# Byte-Oriented Memory Operations

Primary manifestation: Recurring source-level patterns.

## Definition

Byte-oriented processing fails to exploit known type, width, or length information.

## Observed Pattern

Typed values use `memcpy`/`memset` or hand-written byte-order helpers; known string lengths are recomputed with `strlen`.

## Optimization Direction

Express copy and fill through slices, use fixed-width byte-order conversions, and reuse known string lengths.
