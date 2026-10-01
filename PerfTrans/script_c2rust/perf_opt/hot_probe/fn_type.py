""                            

                                                 
                                                                  
                                                                  
                                                          
                                                        
                                            
                             

                                                      
                                                            
   

from __future__ import annotations

import re
from pathlib import Path
from typing import Optional


_ORCH_KEYWORDS = (
    "pthread_", "calloc(", "malloc(", "free(",
    "exit(", "abort(", "perror(", "fprintf(stderr",
    "pthread_mutex_", "pthread_cond_",
)

                                                      
_PURE_ARITH_KEYWORDS = (
    ".wrapping_add(", ".wrapping_mul(", ".wrapping_sub(",
    ".wrapping_shl(", ".wrapping_shr(",
    ".rotate_left(", ".rotate_right(",
    " ^ ", " ^=", " >> ", " << ",
)

                                                     
_CODEC_KEYWORDS = (
    "libc::memcpy(", "libc::memmove(", "libc::memset(",
    "core::ptr::copy_nonoverlapping(", "core::ptr::copy(",
    "copy_from_slice(",
)


def classify_fn_type(fn_body: str, line_span: int) -> tuple[str, str]:
    ""                          

                                                
                        
       
    if line_span <= 0:
        line_span = fn_body.count("\n") + 1
    non_blank = [l.strip() for l in fn_body.splitlines() if l.strip()]
    n = max(1, len(non_blank))

                                                                    
    orch = sum(1 for l in non_blank
               if any(kw in l for kw in _ORCH_KEYWORDS))
    compute_loops = sum(1 for l in non_blank
                        if "while " in l or " for " in l or ".iter()" in l)
    if orch >= 5 and orch / n > 0.15 and compute_loops < 2:
        return "orchestration", (
            f"{orch}/{n} lines pthread/alloc/exit, {compute_loops} compute loops")

                                              
    if line_span < 30:
        arith = sum(1 for l in non_blank
                    if any(kw in l for kw in _PURE_ARITH_KEYWORDS))
                                                               
        mem_ops = sum(1 for l in non_blank
                      if "*const " in l or "*mut " in l or ".offset(" in l
                      or ".add(" in l or ".sub(" in l)
        if arith >= 2 and mem_ops <= 1:
            return "short_hash", (
                f"{line_span} lines, {arith} arith ops, {mem_ops} mem ops")

                                        
                                        
    arm_re = re.compile(r"=>\s*(?:\{|\|)")
    arms = len(arm_re.findall(fn_body))
    if arms >= 8:
        return "state_machine", (
            f"large match/switch with ~{arms} arms")

                                                           
    codec = sum(1 for l in non_blank
                if any(kw in l for kw in _CODEC_KEYWORDS))
    if codec >= 2 and compute_loops >= 1 and line_span >= 30:
        return "codec_hot", (
            f"{codec} memcpy/copy calls in {compute_loops} loop(s)")

                                                 
    if line_span >= 30 and compute_loops >= 1:
        return "algorithm_hot", (
            f"{line_span} lines, {compute_loops} compute loops")

    return "other", f"{line_span} lines, {compute_loops} loops"


def classify_fn_type_from_file(crate_root: Path, file_rel: str,
                                line_start: int, line_end: int,
                                ) -> Optional[tuple[str, str]]:
    ""                                                            
    try:
        src = (crate_root / file_rel).read_text(
            encoding="utf-8", errors="ignore")
    except OSError:
        return None
    lines = src.splitlines()
    if line_start <= 0 or line_end <= 0 or line_end > len(lines):
        return None
    body = "\n".join(lines[line_start - 1: line_end])
    return classify_fn_type(body, line_end - line_start + 1)
