""                                                                

                                                  
                                          
                                                                         
                                                                      
                                           

                                       
                                           
                                         
                                          
                                      
                                                                         
                                                                          
                          

                                                                    
                                                                   
                                                                
                                                                      
                                                                
                
   

from __future__ import annotations

import json
import logging
from dataclasses import dataclass
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class SAParam:
    fn_name: str
    name: str
    mutability: str
    ownership: str
    count: str
    nullable: bool
    noalias_all_sites: bool   # True iff every callsite same_object == False
    n_callsites: int


def _load_sa_facts(sa_json_path: Path) -> dict[str, list[SAParam]]:
    """Returns {fn_name: [SAParam, …]} for every fn the SA engine analyzed."""
    if not sa_json_path.is_file():
        logger.warning(f"[sa] SA report missing: {sa_json_path}")
        return {}
    raw = json.loads(sa_json_path.read_text())
    # The report is serialized as [["analysis_type","..."], ["function_analysis", {…}]]
    fn_block = None
    if isinstance(raw, list):
        for entry in raw:
            if isinstance(entry, list) and len(entry) == 2 and entry[0] == "function_analysis":
                fn_block = entry[1]
                break
    elif isinstance(raw, dict):
        fn_block = raw.get("function_analysis")
    if not isinstance(fn_block, dict):
        logger.warning(f"[sa] unexpected SA report shape at {sa_json_path}")
        return {}
    out: dict[str, list[SAParam]] = {}
    for fname, finfo in fn_block.items():
        params_out: list[SAParam] = []
        for p in finfo.get("parameters", []):
            callsites = p.get("callsites", []) or []
            noalias = (
                len(callsites) > 0
                and all(not cs.get("same_object", True) for cs in callsites)
            )
            params_out.append(SAParam(
                fn_name=fname,
                name=p.get("param_name", ""),
                mutability=p.get("mutability", ""),
                ownership=p.get("ownership", ""),
                count=p.get("count", ""),
                nullable=p.get("nullability", "Nullable") == "Nullable",
                noalias_all_sites=noalias,
                n_callsites=len(callsites),
            ))
        out[fname] = params_out
    logger.info(f"[sa] loaded SA facts for {len(out)} fn(s) from {sa_json_path.name}")
    return out


def find_sa_report(project_dir: Path) -> Path | None:
    """Locate `<source_dir>/svf_analysis_output/func_analysis_report.json`
    for the project whose Stage A crate copy lives at `project_dir`.

    Path resolution is project-agnostic — it derives a project-name hint
    from `project_dir` and looks under `Config/paths.conf`'s
    `SOURCE_PROJECT_BASE` (the canonical root of C sources, also the
    parent of `svf_analysis_output/`). NO absolute paths or per-project
    knowledge are hardcoded here, in keeping with the method-stays-general
    rule.

    Three layout patterns are recognized for the project hint:
      * `<dataset_trans_process>/<proj_name>/2_stage_a/crate` (main.py
        Stage A output as of 2026-05-30 — crate is nested one level
        deeper to allow harness/ as a sibling) —
        `project_dir.parent.parent.name` == proj_name.
      * `<dataset_trans_process>/<proj_name>/2_stage_a` (older layout
        without the crate/ wrapper) — `project_dir.parent.name` == proj_name.
      * `/tmp/perfopt_runs/<proj_name>/ours-…/crate` (disposable
        bench_pipeline workflow) — proj_name is the path segment right
        after `perfopt_runs`.
    """
    try:
        from Config.paths import get_path
    except ImportError:
        logger.warning("[sa] Config.paths not importable — SA report skipped")
        return None
    src_base_str = get_path("SOURCE_PROJECT_BASE")
    if not src_base_str:
        logger.warning(
            "[sa] SOURCE_PROJECT_BASE missing from Config/paths.conf — "
            "SA report skipped"
        )
        return None
    src_base = Path(src_base_str)
    if not src_base.is_dir():
        logger.warning(f"[sa] SOURCE_PROJECT_BASE not a directory: {src_base}")
        return None

    # Extract project-name hint from `project_dir`.
    proj_hints: list[str] = []
    parts = project_dir.parts
    if "perfopt_runs" in parts:
        i = parts.index("perfopt_runs")
        if i + 1 < len(parts):
            proj_hints.append(parts[i + 1])
    # Nested-crate layout (post-2026-05-30 main.py): <out>/<proj>/2_stage_a/crate
    # → parent.parent is the project name (parent is the stage dir).
    if project_dir.parent.parent.name:
        proj_hints.append(project_dir.parent.parent.name)
    # Older flat layout: <out>/<proj>/<stage_dir> — parent IS the project
    # name. Kept for back-compat with any caller that passes a non-nested
    # project_dir (e.g. older bench_pipeline disposable workflow stage
    # dirs that don't have crate/ wrapper). Cheaper than scanning
    # `src_base` children.
    if project_dir.parent.name:
        proj_hints.append(project_dir.parent.name)

    seen: set[str] = set()
    for hint in proj_hints:
        if hint in seen:
            continue
        seen.add(hint)
        p = src_base / hint / "svf_analysis_output" / "func_analysis_report.json"
        if p.is_file():
            return p

    # Last-resort fallback: scan `src_base` children for any subdir whose
    # name fuzzy-matches a hint (handles `bzip2_1_0_8_raw` vs `bzip2-1.0.8`
    # mismatch). Only triggers when the direct lookup above misses.
    for child in src_base.iterdir():
        if not child.is_dir():
            continue
        cname = child.name
        if not any(h and (h in cname or cname in h) for h in proj_hints):
            continue
        p = child / "svf_analysis_output" / "func_analysis_report.json"
        if p.is_file():
            return p
    return None
