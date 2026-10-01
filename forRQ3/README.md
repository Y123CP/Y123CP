# RQ3: Performance Issue Patterns

The paper's RQ3 identifies nine recurring performance issues and derives an
Observed Pattern and Optimization Direction for each. This directory provides
analysis records and a reference copy of the implementation's optimization
card templates.

## Terminology and reading order

| Term | Role in the paper | Artifact location |
|---|---|---|
| Performance Issue | A recurring performance problem identified by RQ3 | The nine issue names below; supporting working records in `analysis_records/` |
| Observed Pattern | The recurring manifestation used to locate candidate occurrences | Analysis records and card templates |
| Optimization Direction | Guidance for addressing the underlying mechanism | Analysis records and card templates |
| Optimization Rule | The approach's pairing of an Observed Pattern with an Optimization Direction | Implemented by PerfTrans's analyzers and rule mapping |
| Optimization Card | A rule augmented with Rewrite Preconditions | `optimization_cards/` (18 implementation templates) |
| Matched Optimization Card | A card instantiated for a concrete source region | Constructed by the prompt builder using the source region and program context |

Read the analysis records as evidence-collection and grouping notes. They
retain intermediate classifications and corrections to earlier observations.
Their historical Class I/II/III labels do not define the final paper taxonomy;
the paper groups the nine issues as follows.

| Primary manifestation | Performance issue |
|---|---|
| Residual runtime work | Residual Bounds Checks |
| Residual runtime work | Redundant Conversion Saturation |
| Missed compiler transformations | Missed Loop Vectorization |
| Missed compiler transformations | Missed Hot-Callee Inlining |
| Recurring source-level patterns | Repeated Invariant Dispatch |
| Recurring source-level patterns | Inefficient Buffer Management |
| Recurring source-level patterns | Byte-Oriented Memory Operations |
| Recurring source-level patterns | Scalar Element-Wise Processing |
| Recurring source-level patterns | Obscured Memory Access Properties |

## Files

- [Class I records](analysis_records/class_I/): optimized-IR comparisons.
- [Class II method](analysis_records/class_II/_method.md) and project records:
  compiler-remark comparisons, including collection corrections.
- [Class III method](analysis_records/class_III/_method.md) and
  [grouping record](analysis_records/class_III/cluster.md): source-level inspection.
- [Optimization card templates](optimization_cards/README.md): the mapping from
  paper issues to implementation templates, their preconditions, and the
  relationship to the paper's matched `zrsh` example.
- `scripts/`: evidence collection and implementation-ID attribution utilities.

`optimization_cards/` retains its name because its files include rewrite
preconditions and serve as card templates in the approach. Several templates
cover different rewrite forms associated with the same paper issue; 18
Markdown templates do not mean 18 performance issues. The executable copy
is in
[`PerfTrans/.../Optimization_Card/`](../PerfTrans/script_c2rust/perf_opt/agent_perf_opt/Optimization_Card/).
The two copies are kept identical. Refer to the card-template guide for the
scope of the figure-aligned wording and its relationship to historical runs.
