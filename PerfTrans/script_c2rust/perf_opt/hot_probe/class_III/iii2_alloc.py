""                                                           

                                                              
                                                
                                                        
                          

                                                             

                                                   
                                              
                                                            

                                                                           

           
                                                    

                                                
                                                                       
   

from __future__ import annotations

import logging
from dataclasses import dataclass, field

from tree_sitter import Node

from perf_opt.hot_probe.class_III.config import ALLOC
from perf_opt.hot_probe.class_III.cst_utils import (
    CalleePrefixKind,
    CallSite,
    FnCstEntry,
    callee_name_and_prefix,
    iter_call_expressions,
)

logger = logging.getLogger(__name__)


@dataclass
class RuleIIITwoHits:
    """Per-fn III② detection outcome."""
    hits: list[CallSite] = field(default_factory=list)
    ambiguous_callees: list[CallSite] = field(default_factory=list)


def detect(
    fn_entry: FnCstEntry,
    non_extern_base: set[str],
) -> RuleIIITwoHits:
    """Iterate `call_expression`s; classify each callee by §2.4's three tiers.

    NOTE: no ExternIndex needed — Alloc is a fixed constant set. The three-tier
    decision only uses ALLOC and non_extern_base.
    """
    result = RuleIIITwoHits()
    for call in iter_call_expressions(fn_entry.node):
        function_node = call.child_by_field_name("function")
        if function_node is None:
            continue
        site = _classify_and_record(
            call, function_node, fn_entry, non_extern_base,
        )
        if site is None:
            continue
        if site[1] == "ambiguous":
            result.ambiguous_callees.append(site[0])
        else:
            result.hits.append(site[0])
    return result


def _classify_and_record(
    call: Node,
    function_node: Node,
    fn_entry: FnCstEntry,
    non_extern_base: set[str],
) -> tuple[CallSite, str] | None:
    """§2.4 three-tier callee resolution over the ALLOC set."""
    base_name, kind = callee_name_and_prefix(function_node)
    if not base_name:
        return None

    if kind is CalleePrefixKind.INTERNAL:
        return None

    if base_name not in ALLOC:
        return None

    if kind is CalleePrefixKind.EXTERNAL:
        return _build_site(call, base_name, fn_entry), "hit"

    if base_name in non_extern_base:
        return _build_site(call, base_name, fn_entry), "ambiguous"
    return _build_site(call, base_name, fn_entry), "hit"


def _build_site(call: Node, callee_name: str, fn_entry: FnCstEntry) -> CallSite:
    row, col = call.start_point
    return CallSite(
        file=str(fn_entry.file),
        line=row + 1,
        col=col + 1,
        callee_name=callee_name,
        marker_seen=False,
    )
