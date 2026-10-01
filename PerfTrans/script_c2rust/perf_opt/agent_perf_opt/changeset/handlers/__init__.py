"""Built-in handlers for typed ChangeSet operations."""

from .promote_static import PromoteStaticHandler
from .replace_call_site import ReplaceCallSiteHandler
from .replace_function import ReplaceFunctionHandler
from .replace_region import (
    ResolvedRegionRef,
    ReplaceSourceRegionHandler,
    resolve_region_ref,
    validate_region_replacement_source,
)
from .rewrite_bitfield import RewriteBitfieldStructHandler

__all__ = [
    "PromoteStaticHandler",
    "ReplaceCallSiteHandler",
    "ReplaceFunctionHandler",
    "ResolvedRegionRef",
    "ReplaceSourceRegionHandler",
    "RewriteBitfieldStructHandler",
    "resolve_region_ref",
    "validate_region_replacement_source",
]
