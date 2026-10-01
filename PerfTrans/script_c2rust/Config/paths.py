#!/usr/bin/env python3
"" 
          
                  

     
                                                  
    
                                           
                                   
    
              
                          
                                 
   

import os
from typing import Dict, Optional

      
_config_cache: Optional[Dict[str, str]] = None

def get_config_file_path() -> str:
    ""            
    current_dir = os.path.dirname(os.path.abspath(__file__))
    return os.path.join(current_dir, 'paths.conf')

def load_config(force_reload: bool = False) -> Dict[str, str]:
    "" 
                 
       
    global _config_cache
    
    if _config_cache is not None and not force_reload:
        return _config_cache
    
    config_file = get_config_file_path()
    if not os.path.exists(config_file):
        raise FileNotFoundError(f"配置文件不存在: {config_file}")
    
                
    raw_config = {}
    try:
        with open(config_file, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                
                         
                if not line or line.startswith('#'):
                    continue
                
                       
                if '=' not in line:
                    print(f"警告: 第{line_num}行格式不正确，跳过: {line}")
                    continue
                
                key, value = line.split('=', 1)
                key = key.strip()
                value = value.strip()
                
                        
                if value.startswith('"') and value.endswith('"'):
                    value = value[1:-1]
                elif value.startswith("'") and value.endswith("'"):
                    value = value[1:-1]
                
                                                                  
                if key == 'PROJECT_ROOT' and ('$(cd' in value or '${BASH_SOURCE' in value):
                                        
                    current_dir = os.path.dirname(os.path.abspath(__file__))
                    value = os.path.abspath(os.path.join(current_dir, '..', '..'))
                
                raw_config[key] = value
                
    except Exception as e:
        raise RuntimeError(f"加载配置文件失败: {e}")
    
                
    config = {}
    max_iterations = 10          
    
    for iteration in range(max_iterations):
        all_resolved = True
        
        for key, value in raw_config.items():
                       
            expanded_value = os.path.expandvars(value)
            
                                                       
            import re
            def _replace_var(match):
                                                     
                nonlocal all_resolved
                var_name = match.group(1) or match.group(2)
                if var_name in config:
                    return config[var_name]
                elif var_name in raw_config:
                    all_resolved = False
                    return match.group(0)             
                else:
                    print(f"警告: 未找到变量 {var_name}")
                    return match.group(0)
            expanded_value = re.sub(
                r'\$\{([A-Z_][A-Z0-9_]*)\}|\$([A-Z_][A-Z0-9_]*)',
                _replace_var, expanded_value,
            )
            config[key] = expanded_value
        
        if all_resolved:
            break
    else:
        print("警告: 可能存在循环引用，变量展开未完全完成")
    
    _config_cache = config
    return config

def get_path(key: str, default: Optional[str] = None) -> Optional[str]:
    config = load_config()
    return config.get(key, default)

def path_exists(key: str) -> bool:
    ""                 
    path = get_path(key)
    return path is not None and os.path.exists(path)
