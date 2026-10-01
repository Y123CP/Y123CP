""                                                              

                                                     
                                                     
                                                                                  
                                        

                                                                    
                                                                      
                                                                      
                                                                      
                                                                    
                                                         

                                                                         
                                                                      
                                               

                                                                        
                                                                   
                
   

from __future__ import annotations

import logging
import re
from pathlib import Path

logger = logging.getLogger("hot_probe.class_I.c3")


_RE_DEFINE = re.compile(r"^define\s+[^@]*@(?P<mangled>[\w.]+)\s*\(")
_RE_TBAA = re.compile(r"!tbaa\s+!\d+")


def count_tbaa_per_fn(ir_path: Path) -> dict[str, int]:
    """{fn_mangled: tbaa_attachments_count} — expected to be all-zero for
    c2rust Rust output (module docstring). Nonzero on any project fn is
    a surprise and should be logged.
    """
    ir_path = Path(ir_path)
    counts: dict[str, int] = {}
    current: str = ""

    with ir_path.open("r", encoding="utf-8", errors="replace") as fh:
        for line in fh:
            if line.startswith("define"):
                m = _RE_DEFINE.match(line)
                if m is not None:
                    current = m.group("mangled")
                    counts.setdefault(current, 0)
                continue
            if line.startswith("}"):
                current = ""
                continue
            if not current:
                continue
            if "!tbaa" in line:
                n = len(_RE_TBAA.findall(line))
                counts[current] = counts.get(current, 0) + n

    nonzero = {k: v for k, v in counts.items() if v > 0}
    logger.info("[class_I.c3] %s: %d fn(s) w/ !tbaa attachments (expected 0 "
                "for pure c2rust output); %d fn(s) tallied total",
                ir_path.name, len(nonzero), len(counts))
    if nonzero:
        logger.warning("[class_I.c3] !tbaa > 0 fn(s) — mechanism assumption "
                       "may not hold: %s",
                       list(nonzero.items())[:10])
    return counts


def _tbaa_for_source_fn(fn_name: str, tbaa_counts: dict[str, int]) -> int:
    """Sum !tbaa across all mangled linkage names encoding `fn_name` at
    its Rust `<len><name>` position — length-prefix substring, not bare
    substring, to avoid short-name false positives (see analogous
    `ResidualCallCounts.has_residual_for_source_fn`).

    Note: c2rust-emitted Rust IR has `!tbaa = 0` universally, so this
    almost always sums to 0. The length-prefix precision matters for
    (a) audit robustness — if a project's mangling ever included !tbaa
    on a nearby-name fn, bare substring would leak; (b) the C3 dropped-
    by-tbaa audit bucket, which needs precise fn-level attribution."""
    needle = f"{len(fn_name)}{fn_name}"
    return sum(v for m, v in tbaa_counts.items() if needle in m)


def apply_c3_predicate(c3_remark_hits: dict[str, object],
                       tbaa_counts: dict[str, int]) -> tuple[set[str], set[str]]:
    """Apply the deploy-time P_C3 predicate on the union of C3 channel data.

    ⚠️ `⊇` reading in RQ3's formula:
       P_C3(f) ≜ hot(f) ∧ remark(f) ⊇ {gvn:LoadClobbered,
                                        licm:LoadWithLoopInvariantAddressInvalidated}
                        ∧ !tbaa = 0

    Two possible readings of `⊇ {a, b}`:
       (i)  strict set-theoretic — remark(f) contains BOTH a AND b
       (ii) mechanism reading   — gvn:LoadClobbered and licm:Invalidated are
                                  two symptoms of the SAME aliasing gap,
                                  either present is sufficient evidence

    **This implementation uses (ii) — OR**. Rationale:
       * The two remarks are the same-mechanism (LICM/GVN blocked by
         `store σ may alias load ℓ` → aliasing gap) reported by two passes.
       * Empirically in RQ3 Discovery set the strong cases have both, but
         weaker instances may fire only one.
       * Requiring both (strict) would drop legitimate C3 candidates whose
         gvn or licm pass didn't emit the paired symptom.
       * The safety net is `!tbaa = 0` — universal for c2rust output but
         a defensive assertion that mechanism holds.

    Rewriting this predicate to AND would be a formal-precision tightening
    at the cost of coverage; consider only if false-positive rate becomes
    a concern in downstream W2 outcomes.


    Args:
      c3_remark_hits  — from `class_II.ScanResult.c3_remark_hits`;
                        {fn_name: RemarkEvidence-shaped}
      tbaa_counts     — {mangled: count} from `count_tbaa_per_fn`

    Returns:
      (fns_hit, fns_dropped_by_tbaa_gate)
        fns_hit    — pass full P_C3
        dropped    — had remark hit but !tbaa > 0 → C3 mechanism doesn't
                     hold, log for audit
    """
    hit: set[str] = set()
    dropped: set[str] = set()
    for fn in c3_remark_hits:
        t = _tbaa_for_source_fn(fn, tbaa_counts)
        if t == 0:
            hit.add(fn)
        else:
            dropped.add(fn)
    logger.info("[class_I.c3] P_C3 predicate: %d hit / %d dropped (had !tbaa>0)",
                len(hit), len(dropped))
    return hit, dropped
