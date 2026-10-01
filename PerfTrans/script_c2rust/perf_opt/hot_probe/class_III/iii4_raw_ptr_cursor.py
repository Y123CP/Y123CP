""                                                                

                                                          
                                                   
                                                                         
                     

                           
                                                      
                                                     
      

                                                                   
                                                        
                                                           
                                                                 
                        

                   

                                                 

                                                       
                                               
                                                                
                                                              
                                                                  
                                   

                                                                     
                                                
                                                               
                                                                         
                                                              

               
                                                            
                                                            
                                          

             

                                    

         

             
                     
                                 
                                                                 
                                                       
                                                     

                  
                                                                
                                                                          
                                                                            

                        

                                           
                                                            
                                                         
                                 
                                                                    

                                                    
                                           
   

from __future__ import annotations

import logging
from dataclasses import dataclass, field

from tree_sitter import Node

from perf_opt.hot_probe.class_III.config import (
    PTR_CURSOR_METHODS_AMBIGUOUS,
    PTR_CURSOR_METHODS_STRICT,
    PTR_CURSOR_METHODS_UNION,
)
from perf_opt.hot_probe.class_III.cst_utils import FnCstEntry

logger = logging.getLogger(__name__)


@dataclass
class CursorSite:
    """Per-site III④ hit record (SPEC §4 schema)."""
    file: str
    line: int
    col: int
    pattern_kind: str                       
    snippet: str                                          


@dataclass
class RuleIIIFourHits:
    ""                                                                     
    hits: list[CursorSite] = field(default_factory=list)


def detect(fn_entry: FnCstEntry) -> RuleIIIFourHits:
    ""                                                            
    result = RuleIIIFourHits()
    body = fn_entry.node.child_by_field_name("body")
    if body is None:
        return result

                                         
                                                                     
                                                          
    pointer_vars = _collect_pointer_vars(body)

    def walk(node: Node) -> None:
                                                                   
                                                                               
        if node.type == "unary_expression" and len(node.children) >= 2:
            op = node.children[0]
            arg = node.children[1]
            if (op.text == b"*"
                and _is_cursor_method_call(arg, PTR_CURSOR_METHODS_UNION)):
                result.hits.append(_build_site(node, fn_entry, "cursor-index-deref"))

        # (P2) post-increment:`<var> = <var>.<method>(<n>)` self-assign
        if node.type == "assignment_expression":
            hit_kind = _classify_post_increment(node, pointer_vars)
            if hit_kind is not None:
                result.hits.append(_build_site(node, fn_entry, hit_kind))

        for child in node.children:
            walk(child)

    walk(body)
    return result


                                                                    

def _collect_pointer_vars(body: Node) -> set[bytes]:
    ""                                  

             
                                             
                                                               

                                                 
       
    pointer_vars: set[bytes] = set()

    def scan(node: Node) -> None:
                                                                           
                                                                        
        if node.type == "unary_expression" and len(node.children) >= 2:
            op = node.children[0]
            arg = node.children[1]
            if op.text == b"*" and arg.type == "identifier":
                pointer_vars.add(arg.text)
                                               
        if node.type == "call_expression":
            func = node.child_by_field_name("function")
            if func is not None and func.type == "field_expression":
                field_name = func.child_by_field_name("field")
                receiver = func.child_by_field_name("value")
                if (field_name is not None and receiver is not None
                    and receiver.type == "identifier"
                    and field_name.text.decode("utf-8", "replace")
                        in PTR_CURSOR_METHODS_STRICT):
                    pointer_vars.add(receiver.text)
        for child in node.children:
            scan(child)

    scan(body)
    return pointer_vars


                                                                 

def _classify_post_increment(
    node: Node, pointer_vars: set[bytes]
) -> str | None:
    ""                                                             
    lhs = node.child_by_field_name("left")
    rhs = node.child_by_field_name("right")
    if lhs is None or rhs is None:
        return None
    if lhs.type != "identifier":
        return None
    if not _is_cursor_method_call(rhs, PTR_CURSOR_METHODS_UNION):
        return None

    func = rhs.child_by_field_name("function")
    if func is None:
        return None
    receiver = func.child_by_field_name("value")
    field_name = func.child_by_field_name("field")
    if receiver is None or field_name is None or receiver.type != "identifier":
        return None
                                            
    if receiver.text != lhs.text:
        return None

    method = field_name.text.decode("utf-8", "replace")
    if method in PTR_CURSOR_METHODS_STRICT:
        return "post-increment"
    if method in PTR_CURSOR_METHODS_AMBIGUOUS:
                                                    
        if lhs.text in pointer_vars:
            return "post-increment-disambiguated"
                           
        return None
    return None


                                                                   

def _is_cursor_method_call(node: Node, method_set: frozenset[str]) -> bool:
    ""                                                     
    if node.type != "call_expression":
        return False
    func = node.child_by_field_name("function")
    if func is None or func.type != "field_expression":
        return False
    field_name = func.child_by_field_name("field")
    if field_name is None:
        return False
    return field_name.text.decode("utf-8", "replace") in method_set


def _build_site(node: Node, fn_entry: FnCstEntry, pattern_kind: str) -> CursorSite:
    ""                                 
    row, col = node.start_point
    text = node.text.decode("utf-8", "replace").strip()
                                     
    text = " ".join(text.split())
    if len(text) > 200:
        text = text[:197] + "..."
    return CursorSite(
        file=str(fn_entry.file),
        line=row + 1,
        col=col + 1,
        pattern_kind=pattern_kind,
        snippet=text,
    )
