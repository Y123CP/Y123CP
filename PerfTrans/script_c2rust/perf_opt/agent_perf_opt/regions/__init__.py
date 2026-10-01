"""Region-scoped performance-optimization model."""

from ..changeset.types import RegionKind, RegionRef
from .extractor import (
    RegionCandidate,
    RegionExtractionResult,
    RegionExtractor,
    RegionSkipReason,
    SkippedRegionHit,
)
from .model import RuleCapability, rule_capability, stable_hit_id

__all__ = [
    "RegionKind",
    "RegionRef",
    "RegionCandidate",
    "RegionExtractionResult",
    "RegionExtractor",
    "RegionSkipReason",
    "RuleCapability",
    "SkippedRegionHit",
    "rule_capability",
    "stable_hit_id",
]
