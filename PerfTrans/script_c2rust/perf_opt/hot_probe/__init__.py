"""hot_probe — produce the evidence pack from a 2_stage_a working copy.

Deterministic measurement (no LLM). Two phases:
  * locate      — which crate functions are hot (self-time ≥ τ) + identity/source.
  * characterize — enrich each into the full evidence pack (later).

The evidence pack is the contract consumed by `agent_perf_opt` (card selection
+ rewrite). Called by `agent_perf_opt.driver`.
"""

from .characterize import characterize
from .locate import locate_hotspots
from .types import EvidencePack, HotFunction

__all__ = ["locate_hotspots", "characterize", "HotFunction", "EvidencePack"]
