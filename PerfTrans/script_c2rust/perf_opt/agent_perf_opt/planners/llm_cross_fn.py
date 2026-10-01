""                                              

                                       
                                                                                
                                                             

                                                
                                                
              

           

                           
                  
                       
                                               
                       
                    

                                        
   

from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING

from perf_opt.agent_perf_opt.changeset.handlers.replace_function import (
    validate_single_function_source,
)
from perf_opt.agent_perf_opt.changeset.handlers.replace_region import (
    validate_region_replacement_source,
)
from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ProposedChangeSet,
    RegionKind,
    ReplaceCallSite,
    ReplaceFunctionBody,
    SymbolKind,
    SymbolRef,
    TriggerContext,
)
from perf_opt.agent_perf_opt.rewrite_applier import EditTarget

if TYPE_CHECKING:                                          
    from perf_opt.agent_perf_opt.caller_lookup import CallerSite


@dataclass(frozen=True)
class LLMCrossFnPlan:
    proposal: ProposedChangeSet | None
    abstain_reason: str | None = None


def _abstain(reason: str) -> LLMCrossFnPlan:
    return LLMCrossFnPlan(None, reason)


_TARGET_MARK = "=== TARGET_FUNCTION ==="
_SITE_RE = re.compile(r"^===\s*CALL_SITE\s+(\d+)\s*===\s*$", re.MULTILINE)
_FENCE_RE = re.compile(r"^\s*```[a-zA-Z]*\s*$", re.MULTILINE)


def _strip_fences(text: str) -> str:
    ""                           
    return _FENCE_RE.sub("", text)


def _parse_cross_fn_response(
    response: str, n_sites: int
) -> tuple[str, dict[int, str]] | str:
    ""                                          

                          
       
    if response is None or not response.strip():
        return "empty_response"
    text = _strip_fences(response)

                                      
    head = text.split(_TARGET_MARK, 1)[0]
    m_ab = re.search(r"ABSTAIN:\s*(.+)", head)
    if m_ab and _TARGET_MARK not in text:
        return f"llm_abstain:{m_ab.group(1).strip()[:120]}"

    if _TARGET_MARK not in text:
        return "no_target_function_section"
    after_target = text.split(_TARGET_MARK, 1)[1]

                      
    site_marks = list(_SITE_RE.finditer(after_target))
    if not site_marks:
        return "no_call_site_sections"

    target_src = after_target[: site_marks[0].start()].strip()
    if not target_src:
        return "empty_target_function"

    sites: dict[int, str] = {}
    for idx, m in enumerate(site_marks):
        site_no = int(m.group(1))
        seg_start = m.end()
        seg_end = (
            site_marks[idx + 1].start()
            if idx + 1 < len(site_marks)
            else len(after_target)
        )
        call_src = after_target[seg_start:seg_end].strip()
        if not call_src:
            return f"empty_call_site:{site_no}"
        if site_no in sites:
            return f"duplicate_call_site:{site_no}"
        sites[site_no] = call_src

    if set(sites) != set(range(n_sites)):
        return (
            f"call_site_index_mismatch:got={sorted(sites)}"
            f"_expected=0..{n_sites - 1}"
        )
    return target_src, sites


def plan_llm_cross_fn_change(
    *,
    response: str,
    edit_target: EditTarget,
    caller_sites: list[CallerSite],
    rule_ids: tuple[str, ...],
    hit_ids: tuple[str, ...],
    base_head: str,
    candidate_id: str,
    crate: Path,
) -> LLMCrossFnPlan:
    ""                                    
    if not caller_sites:
        return _abstain("no_caller_sites")

    parsed = _parse_cross_fn_response(response, len(caller_sites))
    if isinstance(parsed, str):
        return _abstain(parsed)
    target_src, new_calls = parsed

                                                              
    v = validate_single_function_source(target_src, edit_target.fn_name)
    if not v.ok:
        return _abstain(v.as_reason())

                                                       
    for i, call in new_calls.items():
        vc = validate_region_replacement_source(RegionKind.EXPRESSION, call)
        if not vc.ok:
            return _abstain(f"call_site_{i}_shape:{vc.as_reason()}")

    rule_id = ",".join(rule_ids) if rule_ids else "cross_fn"

                                                            
    target_path = edit_target.file.resolve()
    crate_root = crate.resolve()
    target_rel = str(target_path.relative_to(crate_root))
    tdata = target_path.read_bytes()
    ts, te = edit_target.span
    if not (0 <= ts < te <= len(tdata)):
        return _abstain("target_span_out_of_bounds")
    target_span_hash = hashlib.sha256(tdata[ts:te]).hexdigest()

    op_seed = f"{candidate_id}|{target_rel}|{target_span_hash}|{rule_id}"
    target_op_id = "xfn-tgt-" + hashlib.sha256(op_seed.encode()).hexdigest()[:16]
    operations: list = [
        ReplaceFunctionBody(
            operation_id=target_op_id,
            rule_id=rule_id,
            evidence_hit_ids=hit_ids,
            target=SymbolRef(
                qualified_name=f"crate::{edit_target.fn_name}",
                symbol_kind=SymbolKind.FUNCTION,
                file_hint=target_rel,
                declaration_hash=target_span_hash,
            ),
            replacement_function_source=target_src,
        )
    ]

                                                
                           
    file_bytes: dict[str, bytes] = {}
    for i, site in enumerate(caller_sites):
        rel = site.file
        if rel not in file_bytes:
            file_bytes[rel] = (crate_root / rel).read_bytes()
        data = file_bytes[rel]
        if not (0 <= site.start_byte < site.end_byte <= len(data)):
            return _abstain(f"caller_{i}_span_out_of_bounds")
        span_hash = hashlib.sha256(
            data[site.start_byte:site.end_byte]
        ).hexdigest()
        call_op_seed = f"{candidate_id}|{rel}|{site.start_byte}|{site.end_byte}"
        operations.append(
            ReplaceCallSite(
                operation_id="xfn-call-"
                + hashlib.sha256(call_op_seed.encode()).hexdigest()[:16],
                rule_id=rule_id,
                evidence_hit_ids=hit_ids,
                relative_path=rel,
                start_byte=site.start_byte,
                end_byte=site.end_byte,
                expected_span_hash=span_hash,
                replacement_text=new_calls[i],
            )
        )

    cs_seed = f"{candidate_id}|{target_op_id}|{len(operations)}"
    changeset_id = "xfn-" + hashlib.sha256(cs_seed.encode()).hexdigest()[:16]
    trigger = TriggerContext(
        rule_id=rule_id,
        candidate_id=candidate_id,
        hot_function=edit_target.fn_name,
        hit_ids=hit_ids,
        evidence_artifacts=(),
    )
    return LLMCrossFnPlan(
        ProposedChangeSet(
            changeset_id=changeset_id,
            base_head=base_head,
            trigger=trigger,
            operations=tuple(operations),
            impact_scope=ImpactScope.CRATE_GLOBAL,
        )
    )


def build_cross_fn_prompt(
    *,
    fn_name: str,
    target_source: str,
    caller_sites: list[CallerSite],
    card_text: str,
) -> str:
    ""                                       

                                           
       
    lines: list[str] = []
    lines.append(card_text.strip())
    lines.append("")
    lines.append(
        "You are performing a CROSS-FUNCTION rewrite: you will change the "
        f"target function `{fn_name}` (e.g. its signature) AND update every "
        "call site so the crate still compiles. All call sites are in-crate "
        "Rust callers."
    )
    lines.append("")
    lines.append(f"=== TARGET FUNCTION `{fn_name}` (current) ===")
    lines.append(target_source.strip())
    lines.append("")
    lines.append(f"=== CALL SITES ({len(caller_sites)}) ===")
    for i, site in enumerate(caller_sites):
        lines.append(
            f"[CALL_SITE {i}] in fn `{site.caller_fn}` "
            f"({site.file}:{site.line}):"
        )
        lines.append(f"    {site.context}")
    lines.append("")
    lines.append(
        "Emit EXACTLY this format (no prose, no markdown fences):\n"
        "=== TARGET_FUNCTION ===\n"
        "<full rewritten function source, including signature>\n"
        "=== CALL_SITE 0 ===\n"
        "<new call expression for site 0 only, e.g. `foo(a, b, cb)`>\n"
        "=== CALL_SITE 1 ===\n"
        "<new call expression for site 1>\n"
        "... one CALL_SITE section per call site, indices 0.."
        f"{len(caller_sites) - 1}.\n"
        "Each CALL_SITE body must be a SINGLE expression that replaces the "
        "call `<callee>(...)` — not the whole statement.\n"
        "If the rewrite is not worthwhile or not soundly doable, emit only:\n"
        "ABSTAIN: <one-line reason>"
    )
    return "\n".join(lines)
