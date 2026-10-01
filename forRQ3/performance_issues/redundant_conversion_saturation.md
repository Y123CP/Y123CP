# Redundant Conversion Saturation

Primary manifestation: Residual runtime work.

## Definition

Saturating float-to-integer conversions remain even when range invariants prove them unnecessary.

## Observed Pattern

Optimized Rust IR uses `llvm.fptosi.sat` or `llvm.fptoui.sat` for range-bounded operands, whereas C IR uses `fptosi` or `fptoui`.

## Optimization Direction

Avoid unnecessary saturation with `to_int_unchecked` for finite operands whose values, truncated toward zero, fit the target integer type.
