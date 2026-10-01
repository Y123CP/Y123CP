""                                                         

                                                                              
                                                      
                         

                                                                       
                                                                        
                                    

                                                                     
                                                        
   

from __future__ import annotations

import logging
import os
from dataclasses import dataclass, field
from typing import Optional

logger = logging.getLogger(__name__)


class AgentError(RuntimeError):
    ""                                                              


@dataclass
class AgentConfig:
    # ─── Rule ordering ──────────────────────────────────────────────────
                                                     
                                                               
                                                               
    rule_order: tuple[str, ...] = (
        "III④", "III①", "III③", "III②",                        
        "C9",                                                              
        "C11",                                                     
        "C3", "C1", "C2", "C4", "C5", "C6", "C7", "C8", "C10",                            
        "C12",                                                            
        "II_inl", "II_vec",                                     
    )
    max_rounds: int = 1
                                               
                                  

    # ─── Retry (D10 approved: single counter, not per-error-type) ──────
    max_attempts_per_fn: int = 3
    #  Includes the initial attempt. Internal dispatch: syntax_error
                                                                    
    #  W2_regress ends the attempt loop with no retry.

                                                                  
                                                       
                                                    
                                                        
                                                       
                                           
    complex_fn_rules_threshold: int = 2      # fired unique rules ≥ 2 → complex
    complex_fn_hits_threshold:  int = 15     # total hits ≥ 15 → complex

                                                                      
    #  LLMClient.chat() returns str only, tokens estimated via tiktoken.
                                               
    max_llm_tokens_per_fn:        int             = 150_000
    max_project_usd:              Optional[float] = None
    # ─── W1 gate (D6) ──────────────────────────────────────────────────
    w1_sample_per_op: int = 0
                                                       
                                                  
    #  env override: AGENT_W1_SAMPLE

    # ─── W2 gate (D7) ──────────────────────────────────────────────────
                                                                        
                                                               
                                                                     
                                                                         
                                                          
    w2_measurement_repeats: int = 30
    w2_scope:               str = "all_ops"      # "hottest_op" | "all_ops"
                                                                  
                                                            
                                             
    #  env override: AGENT_W2_SCOPE

    w2_pair_checkpoints: tuple[int, ...] = (10, 20, 40)
    w2_confidence: float = 0.95
    w2_per_op_upper_limit_pct: float = 1.0
    # The ceiling for ops the candidate is only a BYSTANDER in — ops the
    # rewritten function appears in but does not dominate. Committing any edit
    # relinks the binary and moves every op, including ones whose source was
    # never touched: measured across seven commits of a compression crate,
    # none of which touched `cache.rs`, the cache operation moved by up to
    # 4.12%, reproducible to 0.01s across independent runs. At the tight
    # ceiling those layout moves reject candidates and are indistinguishable
    # in the log from a real regression. The op the function actually lives in
    # keeps `w2_per_op_upper_limit_pct`; the aggregate gate and the final
    # holistic re-measurement still backstop accumulation.
    w2_bystander_op_upper_limit_pct: float = 3.0
    # Share of an op's self time below which the function is a bystander in it.
    w2_primary_op_share_pct: float = 10.0
                                                
                                                      
                                                  
    w2_regress_tolerance_pct: float = 0.3
    # Past this, a rejected bundle is not "one bad member among good
    # ones" — re-offering each rule alone would spend a build + W1 +
    # W2 per rule to confirm what the size of the regression already
    # says. Below it, the subset retry runs (see `_try_large_fn_regions`).
    w2_catastrophic_regress_pct: float = 5.0
                                              
                                                   
                                                             
                                             
                                             
                                                      
                
    w2_cumulative_op_share_pct: float = 50.0
    w2_cumulative_op_limit_pct: float = 2.0
                                    
    w2_cumulative_op_limit_strict_pct: float = 1.0
                                                     
                                         
                                             
                                                          
                                                             
                                                        
                                                          
                                                    
                                          
                                                      
                                                          
                                                     
                                                  
                                            
                       
                                                                      
                                                   
                                  
    w2_min_total_gain_pct: float = 0.1
    max_candidates: int = 500
    max_w2_pairs: int = 40

    # ─── ① fair per-function scheduling (P0-b) ─────────────────────────
    #  A single large function must not drain the global candidate budget
    #  before every hot function gets a first shot.  Each hot fn's first
    #  visit is capped at an even share of the global budget so all fns are
    #  explored (lz4: 2 big fns ate 92/100, 6 fns got 0 → this caps them).
    #    per_fn_candidate_cap = 0 → auto = max_candidates // num_hot_fns,
    #    floored at per_fn_candidate_floor.  A positive value pins the cap.
    #  Simple/complex fns make ~1 attempt each so the cap only bites the
    #  large-fn region path (the only multi-candidate consumer).
    per_fn_candidate_cap:   int = 0
    per_fn_candidate_floor: int = 6

    # ─── cargo build (impl_plan §8.4) ─────────────────────────────────
    cargo_build_timeout_s: int = 120


# ─── env-var → field mapping ────────────────────────────────────────
#   env var                  field                     parser
_ENV_OVERRIDES: tuple[tuple[str, str, type], ...] = (
    ("AGENT_W2_REPEATS",         "w2_measurement_repeats",  int),
    ("AGENT_W1_SAMPLE",          "w1_sample_per_op",        int),
    ("AGENT_W2_SCOPE",           "w2_scope",                str),
    ("AGENT_MAX_TOKENS",         "max_llm_tokens_per_fn",   int),
    ("AGENT_W2_PAIR_CHECKPOINTS", "w2_pair_checkpoints",
     lambda s: tuple(int(part.strip()) for part in s.split(",") if part.strip())),
)


def _apply_env_overrides(cfg: AgentConfig) -> None:
    """Read env vars per `_ENV_OVERRIDES` and mutate cfg in place.
    Invalid values are logged (not fatal) so a typo doesn't kill the run.
    Called AFTER CLI overrides so env vars win — env is the outer knob.
    """
    for env_name, field_name, parser in _ENV_OVERRIDES:
        raw = os.environ.get(env_name)
        if raw is None:
            continue
        try:
            value = parser(raw)
        except (TypeError, ValueError):
            logger.warning("[config] ignoring %s=%r (parser=%s failed)",
                            env_name, raw, parser.__name__)
            continue
        setattr(cfg, field_name, value)
        logger.info("[config] env override: %s = %r → cfg.%s",
                     env_name, value, field_name)


def load_agent_config(overrides: Optional[dict] = None) -> AgentConfig:
    """Return AgentConfig defaults, apply `overrides` dict + env-var overrides.

    Precedence (weakest → strongest): defaults → overrides → env vars.
    Unknown keys in overrides raise AgentError (typo protection).
    """
    cfg = AgentConfig()
    if overrides:
        known = {f.name for f in cfg.__dataclass_fields__.values()}
        unknown = set(overrides) - known
        if unknown:
            raise AgentError(
                f"unknown AgentConfig keys: {sorted(unknown)}; "
                f"known: {sorted(known)}"
            )
        for k, v in overrides.items():
            setattr(cfg, k, v)
    _apply_env_overrides(cfg)
    return cfg
