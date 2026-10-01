# RQ3 Performance Issues

These nine entries reproduce the Definition, Observed Pattern, and
Optimization Direction from the paper's table, "Recurring performance issues
and optimization directions identified in the Study Dataset."
Only LaTeX formatting has been converted to Markdown.

| Primary manifestation | Performance issue |
|---|---|
| Residual runtime work | [Residual Bounds Checks](residual_bounds_checks.md) |
| Residual runtime work | [Redundant Conversion Saturation](redundant_conversion_saturation.md) |
| Missed compiler transformations | [Missed Loop Vectorization](missed_loop_vectorization.md) |
| Missed compiler transformations | [Missed Hot-Callee Inlining](missed_hot_callee_inlining.md) |
| Recurring source-level patterns | [Repeated Invariant Dispatch](repeated_invariant_dispatch.md) |
| Recurring source-level patterns | [Inefficient Buffer Management](inefficient_buffer_management.md) |
| Recurring source-level patterns | [Byte-Oriented Memory Operations](byte_oriented_memory_operations.md) |
| Recurring source-level patterns | [Scalar Element-Wise Processing](scalar_element_wise_processing.md) |
| Recurring source-level patterns | [Obscured Memory Access Properties](obscured_memory_access_properties.md) |

These are the RQ3 findings. In the approach, each pattern–direction pair is
encoded as an optimization rule and augmented with Rewrite Preconditions to
form an optimization card. Those implementation templates are provided in
[PerfTrans's Optimization_Card directory](../../PerfTrans/script_c2rust/perf_opt/agent_perf_opt/Optimization_Card/README.md).
They are not additional RQ3 issues.

The [analysis records](../analysis_records/) preserve working observations,
method corrections, and intermediate groupings. They are distinct from this
transcription of the final paper table.
