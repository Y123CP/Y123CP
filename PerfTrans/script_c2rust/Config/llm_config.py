"" 
             

                                                   
                            

     
                                                

                              
                                             
          
                                            
                              
                                                 
                               
       

            
                            
                              

     
                                     
                                    
                                                                
                               
   

import os
import logging
from typing import Dict, Optional

logger = logging.getLogger(__name__)


# ═══════════════════════════════════════════════════════
       
# ═══════════════════════════════════════════════════════
                               
                                                                         
#
# provider:
                                                                     
                                                          
#
                                          
                                                     

_MODEL_ROUTES = [
                                            
    (("gpt", "claude", "gemini"), "LLM_API_KEY", "LLM_BASE_URL", "openai"),
    # google → openrouter
    (("google",),               "LLM_OPENROUTER_API_KEY", "LLM_OPENROUTER_BASE_URL", "openai"),
]

                 
_DEFAULT_KEY_NAME = "LLM_API_KEY"
_DEFAULT_URL_NAME = "LLM_BASE_URL"
_DEFAULT_PROVIDER = "openai"


# ═══════════════════════════════════════════════════════
      
# ═══════════════════════════════════════════════════════

_cached_config: Optional[Dict[str, str]] = None


def _load_config() -> Dict[str, str]:
    ""                                      
    global _cached_config
    if _cached_config is not None:
        return _cached_config

    try:
        from Config.paths import load_config
        _cached_config = load_config()
    except Exception as e:
        logger.debug(f"[LLMConfig] Cannot load paths.conf: {e}")
        _cached_config = {}

    return _cached_config


def _get_value(key_name: str) -> str:
    ""                             
    config = _load_config()
    value = config.get(key_name, "")
    if not value:
        value = os.environ.get(key_name, "")
    return value


# ═══════════════════════════════════════════════════════
        
# ═══════════════════════════════════════════════════════

def get_llm_config(model_name: str) -> Dict[str, str]:
    "" 
                                     

         
                                                                          

            
             
         
                                      
                                        
                                        
                                                                  
         
       
    for prefixes, key_name, url_name, provider in _MODEL_ROUTES:
        if model_name.startswith(prefixes):
            api_key = _get_value(key_name)
            base_url = _get_value(url_name)

            if not api_key:
                logger.warning(f"[LLMConfig] API key '{key_name}' not found in paths.conf or env")
            if not base_url:
                logger.warning(f"[LLMConfig] Base URL '{url_name}' not found in paths.conf or env")

            return {
                "model_name": model_name,
                "api_key": api_key,
                "base_url": base_url,
                "provider": provider,
            }

                     
    logger.warning(f"[LLMConfig] Unknown model prefix for '{model_name}', using default route")
    return {
        "model_name": model_name,
        "api_key": _get_value(_DEFAULT_KEY_NAME),
        "base_url": _get_value(_DEFAULT_URL_NAME),
        "provider": _DEFAULT_PROVIDER,
    }


def reload_config():
    ""                        
    global _cached_config
    _cached_config = None
                       
    try:
        from Config.paths import load_config
        load_config(force_reload=True)
    except Exception:
        pass
