"""Class III detector — configuration constants (SPEC §9 D2/D5/D10/D14/D15).

All tunables live here so implementation modules stay free of magic numbers.
"""

from __future__ import annotations

# ---- III③ alloc set ---------------------------------------------------------
# SPEC §3.1 — canonical from result.md line 552/563, extended by D5.

ALLOC_CANONICAL: frozenset[str] = frozenset({
    "malloc", "free", "realloc", "calloc",
})

ALLOC_EXTENDED: frozenset[str] = frozenset({
    "aligned_alloc", "posix_memalign", "memalign", "valloc",
})

ALLOC: frozenset[str] = ALLOC_CANONICAL | ALLOC_EXTENDED


                                                                         
                                          
                                  
                                  
                                         
                                
                                                                
                                                          
PROCESS_TERMINATION_EXCLUDE: frozenset[str] = frozenset({
    "abort", "exit", "_exit", "_Exit",
    "perror", "strerror",
    "raise",
    "__assert_fail", "__assert_perror_fail",
    "longjmp", "siglongjmp",
})


# ---- III② scoped_identifier prefix taxonomy (SPEC §2.4 D14) -----------------
# Prefix segments (leading path component) that unambiguously mark a callee's
# origin. The three-tier resolution in §2.4 keys off these.

EXTERNAL_CRATE_PREFIXES: frozenset[str] = frozenset({
    "libc",
})

INTERNAL_CRATE_PREFIXES: frozenset[str] = frozenset({
    "crate", "self", "super",
})


# ---- III① receiver depth cap (SPEC §1.4 D17) --------------------------------
# Max consecutive `field_expression` nesting when resolving receiver type.
# Deref (`*x`) and paren wrapping don't count but pass through.

RECEIVER_FIELD_DEPTH_CAP: int = 3


# ---- III① c2rust marker string (SPEC §1.5) ----------------------------------
# Observational only — NEVER used as a matching predicate. Recorded in
# CallSite.marker_seen for downstream audit.

CALLBACK_MARKER_STRING: str = "non-null function pointer"


# ---- III④ raw-pointer cursor pattern methods --------------------------------
                                                                 
#
                                                             
                 
                                                       
                                                        
                                                                 
                                      
#
                                                  
                                                             
                                     
#
                                  
#
                                                           
                                                      
                      
#
# **P2**(`<var> = <var>.<method>(<n>)` self-assign):
                                             
                                                     
                                                                      
                        
#
       
#
                                                   
                                                          
         

PTR_CURSOR_METHODS_STRICT: frozenset[str] = frozenset({
    "offset", "wrapping_offset",
    "add", "sub",
})

PTR_CURSOR_METHODS_AMBIGUOUS: frozenset[str] = frozenset({
    "wrapping_add", "wrapping_sub",
})

PTR_CURSOR_METHODS_UNION: frozenset[str] = (
    PTR_CURSOR_METHODS_STRICT | PTR_CURSOR_METHODS_AMBIGUOUS
)

# Back-compat alias for callers still using the pre-audit name (iii3_mem_ops
# body-pattern detector). Callers that need pointer-only should use
# `PTR_CURSOR_METHODS_STRICT` explicitly.
PTR_CURSOR_METHODS = PTR_CURSOR_METHODS_UNION


# ---- III③ custom mem-op body-pattern detection thresholds -------------------
# Body-pattern criteria for identifying project-defined mem-op wrappers
                                                                     
                                         
#
                                             
                                        
                                              
                                                           
CUSTOM_MEM_OP_MAX_STATEMENTS: int = 24                                         
CUSTOM_MEM_OP_MAX_BRANCHES: int = 1                                    
CUSTOM_MEM_OP_MIN_PTR_OPS: int = 2                           

                                                            
                                                        
                                              
        
#
                                          
#     tree_codegen               1.4439s → 1.0003s   -30.72%
#     lz77_blocksplit_analysis   1.3531s → 1.2513s   - 7.52%
                                                       
#
                                         
                                      
HIGHER_ORDER_LIBC_FNS: frozenset[str] = frozenset({
    "qsort", "qsort_r", "bsearch", "lfind", "lsearch", "twalk", "tsearch",
})
