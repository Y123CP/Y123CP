# Missed Hot-Callee Inlining

Primary manifestation: Missed compiler transformations.

## Definition

Hot calls inlined in C remain uninlined in translated Rust.

## Observed Pattern

A hot call is inlined in C but retained in Rust; IR comparison reveals missing inlining attributes, or remarks report excessive cost.

## Optimization Direction

Restore missing inlining annotations or simplify the callee to reduce its estimated inlining cost.
