""                                                                      

                                                      

                                                                  
                                                                         
                                                                 

           
                                                              
                                                               
                                                                        
                                                                             

                                                   
                                                     

                                         
                                                                 
                    

                                           
                                                                           
                                                                        

                                         
                                   

                                                           
                                                          

                                                                  
                                      
   

from perf_opt.hot_probe.class_I.attribute import (
    AggregatedHits, Attribution, STDLIB_ONLY, Site, UNRESOLVED, aggregate,
    attribute_site,
)
from perf_opt.hot_probe.class_I.build import BuildResult, build_and_emit_ir
from perf_opt.hot_probe.class_I.c3 import apply_c3_predicate, count_tbaa_per_fn
from perf_opt.hot_probe.class_I.ir_parse import (
    DIFile, DILocation, DISubprogram, MetaTables, is_stdlib_file,
    parse_ir_metadata,
)
from perf_opt.hot_probe.class_I.rules import scan_ir_for_c1_c2
from perf_opt.hot_probe.class_I.scan import ScanResult, scan

__all__ = [
    # top-level scan API
    "scan", "ScanResult",
    # build
    "build_and_emit_ir", "BuildResult",
    # rule scanners (composable primitives)
    "scan_ir_for_c1_c2",
    "count_tbaa_per_fn", "apply_c3_predicate",
    # attribution + IR parse (rule-agnostic infra)
    "parse_ir_metadata", "MetaTables",
    "DILocation", "DISubprogram", "DIFile",
    "is_stdlib_file",
    "aggregate", "attribute_site",
    "Site", "Attribution", "AggregatedHits",
    "UNRESOLVED", "STDLIB_ONLY",
]
