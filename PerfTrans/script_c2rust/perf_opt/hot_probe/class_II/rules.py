""                                                                   
                                                                       
                                

                                                                          
                                                       
                                                        
                                                                
                                                                       

                                                        

                     
                                       
                                                                         
                                   

                                                
                                                         
                                                           
                                                      
                                               
                                                                
                                              
                                                   

                             
                                           
                                                                                    
                                                                                     

                                                                   
                                                            
                                                               

                                                                  
                                                                            
                                                               
                                          
                                                                       
                                                            
                                                                  
                                              

                                                                             
                                                                           
                                                                          
                                                                        
   

from __future__ import annotations

import re
from dataclasses import dataclass


                                                                            
#
                                                             
                                                         
# Order: most-frequent first (short-circuits earlier for perf).
II_VEC_ACTIONABLE_RE: dict[str, re.Pattern] = {
                                                    
    "CantComputeNumberOfIterations":
        re.compile(r"could not determine (?:the )?number of loop iterations",
                   re.IGNORECASE),
    "NonReductionValueUsedOutsideLoop":
        re.compile(r"value that could not be identified as reduction is used outside the loop",
                   re.IGNORECASE),
    "LoopContainsSwitch":                                             
        re.compile(r"loop contains a switch statement", re.IGNORECASE),
    "NoCFGForSelect":                                 
        re.compile(r"control flow cannot be substituted for a select",
                   re.IGNORECASE),
                                                                  
    "CantVectorizeLibcall":
        re.compile(r"(?:library )?call instruction cannot be vectorized",
                   re.IGNORECASE),
                                                                             
                                                            
                                                                     
                                                         
}

                                                                
# but LLVM may emit this on other projects; ready to filter.
II_VEC_EXCLUDE_RE: dict[str, re.Pattern] = {
    "NotBeneficial":
        re.compile(r"not beneficial", re.IGNORECASE),
}


                                                                         
#
                                                                
                                               
II_INL_ACTIONABLE_RE: dict[str, re.Pattern] = {
    "TooCostly":
        re.compile(r"too costly to inline", re.IGNORECASE),
}

                                       
#
                                  
                                                                 
                                                               
                                                                       
                                                                  
                                                           
                                 
II_INL_EXCLUDE_RE: dict[str, re.Pattern] = {
    "NoDefinition":
        re.compile(r"definition is unavailable", re.IGNORECASE),
    "NoInlineAttribute":
        re.compile(r"noinline attribute|has uninlinable|uninlinable pattern",
                   re.IGNORECASE),
    "NeverInline":
        re.compile(r"never be inlined|recursive", re.IGNORECASE),
}


# ── C3 gvn / licm reason patterns (empirically verified on lodepng T1) ──────
# C3 detection uses these plus IR-side `!tbaa = 0`. See class_I/c3.py for the
# combined predicate. Kept here so Class II's build + parse pipeline can bucket
# gvn / licm remarks in one pass (they're on the same stderr stream).

C3_REMARK_RE: dict[str, re.Pattern] = {
    "LoadClobbered":
        re.compile(r"load of type .+ not eliminated", re.IGNORECASE),
    "LoadWithLoopInvariantAddressInvalidated":
        re.compile(r"may invalidate its value", re.IGNORECASE),
}
# licm CondExecuted (LoadWithLoopInvariantAddressCondExecuted) is a DIFFERENT
# reason and NOT in C3 per RQ3 formula. We keep the pattern here for
# transparency but do not include it in the fire set.
_C3_LICM_CONDEXEC_RE = re.compile(r"conditionally executed", re.IGNORECASE)


# ── classification API ─────────────────────────────────────────────────────


@dataclass(frozen=True)
class Classified:
    """One remark classified against the 5-rule taxonomy.
      rule_id:   "II_vec" | "II_inl" | "C3" | None
      reason:    normalized Name (e.g. "TooCostly") | None
      excluded:  True if it matched an exclusion pattern (e.g. NotBeneficial)
    """
    rule_id: str | None
    reason: str | None
    excluded: bool = False


def classify(r: dict) -> Classified:
    """Map one parsed remark dict to (rule_id, reason).

    `r` shape (from `profiling.opt_remarks.parse_opt_remarks`):
      {"file", "line", "col", "pass", "status", "message"}

    Rules:
      * status must be one of the actionable statuses (missed for gvn/licm/
        inline; missed OR analysis for loop-vectorize since actionable
        reasons live in analysis rows).
      * NotBeneficial → excluded=True (caller decides how to handle).
      * gvn:LoadClobbered / licm:Invalidated → rule_id="C3" (Class I gap,
        detected via Class II's remark channel — see result.md line 447).
      * inline:{TooCostly|NoDefinition|NeverInline} → rule_id="II_inl".
      * loop-vectorize:{6 actionable} → rule_id="II_vec".
      * anything else → rule_id=None (drop).
    """
    p = r.get("pass") or ""
    s = r.get("status") or ""
    msg = r.get("message") or ""

    if p == "gvn" and s == "missed":
        for name, pat in C3_REMARK_RE.items():
            if pat.search(msg) and name == "LoadClobbered":
                return Classified(rule_id="C3", reason=name)
    elif p == "licm" and s == "missed":
        for name, pat in C3_REMARK_RE.items():
            if pat.search(msg) and name == "LoadWithLoopInvariantAddressInvalidated":
                return Classified(rule_id="C3", reason=name)
    elif p == "inline" and s == "missed":
                                                      
                                                         
                                                   
        for name, pat in II_INL_EXCLUDE_RE.items():
            if pat.search(msg):
                return Classified(rule_id=None, reason=name, excluded=True)
        for name, pat in II_INL_ACTIONABLE_RE.items():
            if pat.search(msg):
                return Classified(rule_id="II_inl", reason=name)
    elif p == "loop-vectorize" and s in ("missed", "analysis"):
        # NotBeneficial exclusion checked first
        for name, pat in II_VEC_EXCLUDE_RE.items():
            if pat.search(msg):
                return Classified(rule_id=None, reason=name, excluded=True)
        for name, pat in II_VEC_ACTIONABLE_RE.items():
            if pat.search(msg):
                return Classified(rule_id="II_vec", reason=name)
        # loop-vectorize:missed bare row ("loop not vectorized" with no
        # reason) — carries no actionable info, drop.
    return Classified(rule_id=None, reason=None)
