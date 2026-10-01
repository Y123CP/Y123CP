"""Self-contained copies of the profiling collectors hot_probe reuses, so
hot_probe does not depend on the top-level `profiling/` package (which may be
cleaned up). Copied verbatim except for one import rewire (`_resolve_rand` →
`._util`). These are PROCESS-level collectors — per-function attribution uses
hot_probe's own deepest-crate-frame sampler (`selftime`), NOT these.

  tma.py        — run_tma: process TMA L1 buckets (toplev / perf topdown).
  perf_stat.py  — collect_cpi_counters + derive_cpi_metrics: process
                  cycles/instructions/branch counters → CPI / branch-miss rate.
  opt_remarks.py — collect_opt_remarks: build with LLVM remark flags, parse
                  which loops vectorized / calls inlined (missed vs passed).
"""

from .opt_remarks import OptRemarksReport, collect_opt_remarks, parse_opt_remarks
from .perf_stat import collect_cpi_counters, derive_cpi_metrics
from .tma import TmaResult, run_tma

__all__ = ["run_tma", "TmaResult",
           "collect_cpi_counters", "derive_cpi_metrics",
           "collect_opt_remarks", "OptRemarksReport", "parse_opt_remarks"]
