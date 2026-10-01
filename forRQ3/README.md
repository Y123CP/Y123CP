# RQ3: Performance Issue Patterns

RQ3 identifies nine recurring performance issues, each with an Observed
Pattern and an Optimization Direction. Start with the
[performance-issue catalog](performance_issues/README.md), which reproduces
the final paper table in English.

## Contents

- [Performance issues](performance_issues/README.md): nine paper-level findings,
  grouped into two residual-runtime issues, two missed-compiler-transformation
  issues, and five source-level issues.
- [Class I analysis records](analysis_records/class_I/): optimized-IR comparisons.
- [Class II method and records](analysis_records/class_II/_method.md): compiler
  optimization remarks, including collection-method corrections.
- [Class III method](analysis_records/class_III/_method.md) and
  [grouping record](analysis_records/class_III/cluster.md): source-level inspection.
- `scripts/`: evidence collection and implementation-ID attribution utilities.

The analysis records are working notes. They retain intermediate classifications
and corrections to earlier observations. Their historical Class I/II/III labels
and rule counts are not the final paper taxonomy.

## Relationship to the approach

An **optimization rule** pairs an **Observed Pattern** with an
**Optimization Direction**. The approach augments a rule with
**Rewrite Preconditions** to form an **optimization card**, and instantiates
it for a source region to obtain a **matched optimization card**.

The 18 implementation card templates are kept in
[PerfTrans's Optimization_Card directory](../PerfTrans/script_c2rust/perf_opt/agent_perf_opt/Optimization_Card/README.md).
Several templates implement different rewrite forms for one paper issue.
The templates and their preconditions belong to the approach; they are not
18 separate RQ3 findings. The directory name `performance_issues/` here follows
the terminology of RQ3, while `Optimization_Card/` in PerfTrans follows the
terminology of rewrite generation.
