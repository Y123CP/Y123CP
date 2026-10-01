""                                                                 
                                   

                                               

                                                                          
                                                                          
                                                                            
                                                                    
                                                                                  

           
                                                      
                                                        

                                                                      
                                                
   

from perf_opt.hot_probe.class_III.cst_utils import (
    CalleePrefixKind, CallSite, ExternIndex, FnCstEntry, FnKey, StructIndex,
)
from perf_opt.hot_probe.class_III.iii1_callback import (
    RuleIIIOneHits, detect as detect_iii1,
)
from perf_opt.hot_probe.class_III.iii2_alloc import (
    RuleIIITwoHits, detect as detect_iii2,
)
from perf_opt.hot_probe.class_III.iii3_mem_ops import (
    RuleIIIThreeHits, detect as detect_iii3, detect_custom_mem_op_fns,
)
from perf_opt.hot_probe.class_III.iii4_raw_ptr_cursor import (
    CursorSite, RuleIIIFourHits, detect as detect_iii4,
)
from perf_opt.hot_probe.class_III.scan import ScanResult, scan

__all__ = [
    # top-level scan API
    "scan", "ScanResult",
    # rule detectors (composable primitives)
    "detect_iii1", "RuleIIIOneHits",
    "detect_iii2", "RuleIIITwoHits",
    "detect_iii3", "RuleIIIThreeHits", "detect_custom_mem_op_fns",
    "detect_iii4", "RuleIIIFourHits",
    # shared dataclasses
    "CallSite", "CursorSite", "FnKey", "FnCstEntry",
    "StructIndex", "ExternIndex", "CalleePrefixKind",
]
