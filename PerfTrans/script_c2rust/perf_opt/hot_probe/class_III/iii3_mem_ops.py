""                                                                 

                                         
                                                                   
                                               
                                                          
                            

                                      
                                                            

                            
                                                        
                                                                    
                                                                       
                                                                   
                                                     
                                                                     

                                                            

           
                                                                                       
                                                                    
   

from __future__ import annotations

import logging
from dataclasses import dataclass, field

from tree_sitter import Node

from perf_opt.hot_probe.class_III.config import (
    ALLOC,
    CUSTOM_MEM_OP_MAX_BRANCHES,
    CUSTOM_MEM_OP_MAX_STATEMENTS,
    CUSTOM_MEM_OP_MIN_PTR_OPS,
    PROCESS_TERMINATION_EXCLUDE,
    PTR_CURSOR_METHODS,
)
from perf_opt.hot_probe.class_III.cst_utils import (
    CalleePrefixKind,
    CallSite,
    ExternIndex,
    FnCstEntry,
    callee_name_and_prefix,
    iter_call_expressions,
)

logger = logging.getLogger(__name__)


@dataclass
class RuleIIIThreeHits:
    """Per-fn III③ detection outcome."""
    hits: list[CallSite] = field(default_factory=list)
    ambiguous_callees: list[CallSite] = field(default_factory=list)


def detect(
    fn_entry: FnCstEntry,
    extern_index: ExternIndex,
    non_extern_base: set[str],
    custom_mem_op_fns: set[str] | None = None,
) -> RuleIIIThreeHits:
    """Iterate `call_expression`s in fn body; classify each callee by §3.2.

    Args:
      fn_entry:          candidate fn CST entry
      extern_index:      crate-wide extern "C" decl catalogue
      non_extern_base:   crate-wide project fn base-name set (for ambiguity gate)
      custom_mem_op_fns: body-pattern-detected project fns (see
                         detect_custom_mem_op_fns). Calls to these are also
                         III③ hits.
    """
    result = RuleIIIThreeHits()
                                                                   
    ext_minus_alloc = (set(extern_index.decls.keys())
                       - ALLOC - PROCESS_TERMINATION_EXCLUDE)
    custom = custom_mem_op_fns or set()
                                                               
    memop_union = ext_minus_alloc | custom

    for call in iter_call_expressions(fn_entry.node):
        function_node = call.child_by_field_name("function")
        if function_node is None:
            continue
        site = _classify_and_record(
            call, function_node, fn_entry, memop_union, non_extern_base, custom,
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
    memop_union: set[str],
    non_extern_base: set[str],
    custom_mem_op_fns: set[str],
) -> tuple[CallSite, str] | None:
    ""                                   

          
                                                           
                                                              
                                                       
                                         
       
    base_name, kind = callee_name_and_prefix(function_node)
    if not base_name:
        return None

    if base_name not in memop_union:
        return None

                                                                     
                                                                      
                                                   
    if kind is CalleePrefixKind.INTERNAL and base_name not in custom_mem_op_fns:
        return None

    if kind is CalleePrefixKind.EXTERNAL:
        return _build_site(call, base_name, fn_entry), "hit"

                                                                  
                                                                   
    if base_name in non_extern_base and base_name not in custom_mem_op_fns:
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


                                                                           

def detect_custom_mem_op_fns(fn_items: dict[str, FnCstEntry]) -> set[str]:
    ""                                                  

                                                       
                                 
                                     
                                              
                                                         

                                                   

                                                  
                                       
       
    custom: set[str] = set()
    for key, entry in fn_items.items():
        if _looks_like_mem_op_wrapper(entry.node):
            custom.add(entry.name)
    logger.info(f"[iii3_mem_ops] custom mem-op fns detected: {len(custom)} "
                f"(sample: {sorted(custom)[:5]})")
    return custom


def _looks_like_mem_op_wrapper(fn_node: Node) -> bool:
    ""                                                            
    body = fn_node.child_by_field_name("body")
    if body is None:
        return False                          

    stmt_count = 0
    branch_count = 0
    ptr_op_count = 0

    def walk(node: Node) -> None:
        nonlocal stmt_count, branch_count, ptr_op_count
        if node.type in ("if_expression", "match_expression"):
            branch_count += 1
        elif node.type in ("loop_expression", "while_expression", "for_expression"):
            branch_count += 1
        elif node.type in ("let_declaration", "expression_statement"):
            stmt_count += 1
                                                    
                                                                        
        if node.type == "unary_expression" and len(node.children) >= 2:
            op = node.children[0]
            arg = node.children[1]
            if op.text == b"*" and _is_ptr_cursor_method_call(arg):
                ptr_op_count += 1
        for child in node.children:
            walk(child)

    walk(body)

    return (
        stmt_count <= CUSTOM_MEM_OP_MAX_STATEMENTS
        and branch_count <= CUSTOM_MEM_OP_MAX_BRANCHES
        and ptr_op_count >= CUSTOM_MEM_OP_MIN_PTR_OPS
    )


def _is_ptr_cursor_method_call(node: Node) -> bool:
    ""                                                 
    if node.type != "call_expression":
        return False
    func = node.child_by_field_name("function")
    if func is None or func.type != "field_expression":
        return False
    field_name = func.child_by_field_name("field")
    if field_name is None:
        return False
    return field_name.text.decode("utf-8", "replace") in PTR_CURSOR_METHODS
