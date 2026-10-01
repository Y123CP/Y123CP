""                                                                       

                                                           
                                                               
                                             

                                                      
                                    

         

                                                                          
                                                      
                                                  
                                                    
                       

          

                                                                                          
                                                                                              
                                                                                     
                                                                                    
                                                                                           
                                                                                    

                        

                                                                   
                                                                    
                                                             
                                                        
                                                    
                           
                                                             
                                     
                                                             
                                                      
                                 
   

from __future__ import annotations

import logging
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterator

from tree_sitter import Language, Node, Parser
import tree_sitter_rust

logger = logging.getLogger("agent_perf_opt.caller_lookup")


@dataclass(frozen=True)
class CallerSite:
    """One call site of the target fn."""
    file: str          # crate-relative path
    line: int          # 1-indexed
    col: int           # 1-indexed
    caller_fn: str                                 
    context: str                                
    kind: str          # "rust_same_crate" | "extern_c_facing" | "test_fn"
                                               
                                                   
    # [start_byte, end_byte)。
    start_byte: int = 0
    end_byte: int = 0


                                                                         

_PARSER: Parser | None = None


def _get_parser() -> Parser:
    global _PARSER
    if _PARSER is None:
        _PARSER = Parser(Language(tree_sitter_rust.language()))
    return _PARSER


# ── Helpers ────────────────────────────────────────────────────────────────

def _walk(node: Node) -> Iterator[Node]:
    ""               
    yield node
    for child in node.children:
        yield from _walk(child)


def _git_head_sha(crate_dir: Path) -> str:
    ""                                            
                                                      
    try:
        r = subprocess.run(
            ["git", "-C", str(crate_dir), "rev-parse", "HEAD"],
            capture_output=True, text=True, timeout=5,
        )
        return r.stdout.strip() if r.returncode == 0 else ""
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return ""


                                                             
_INTERNAL_PATH_PREFIXES = {"crate", "self", "super"}


def _extract_callee_name(function_node: Node) -> str | None:
    ""                                                 

                       
                                             
                                                   
                                                  
                                                   
                                                              
                                    
                                                              
                                                                 
                                                    
       
    if function_node.type == "identifier":
        return function_node.text.decode(errors="replace")
    if function_node.type == "scoped_identifier":
                                                
                                                                                                          
        path = function_node.child_by_field_name("path")
        name_node = function_node.child_by_field_name("name")
        if name_node is None:
            return None
                          
        root_seg = path
        while root_seg is not None and root_seg.type == "scoped_identifier":
            root_seg = root_seg.child_by_field_name("path")
        if root_seg is None:
            return None
        root_text = root_seg.text.decode(errors="replace")
        if root_seg.type in ("crate", "self", "super") or \
           root_text in _INTERNAL_PATH_PREFIXES:
            return name_node.text.decode(errors="replace")
                                                         
        return None
    return None


def _enclosing_fn(node: Node) -> Node | None:
    ""                            
    cur = node.parent
    while cur is not None:
        if cur.type == "function_item":
            return cur
        cur = cur.parent
    return None


def _fn_name(fn_item: Node) -> str:
    ""                         
    n = fn_item.child_by_field_name("name")
    return n.text.decode(errors="replace") if n is not None else "<unknown>"


def _classify_caller(fn_item: Node, file_rel: str) -> str:
    ""                                 

                
                                                            
                                                    
                             

                                                                        
                                                      
                                                         
       
    has_no_mangle = False
    has_test_marker = False    # #[test] or #[cfg(test)]
    parent = fn_item.parent
    if parent is not None:
                                              
                                                                
                                                          
        siblings = parent.children
        try:
            idx = next(i for i, c in enumerate(siblings) if c.id == fn_item.id)
        except StopIteration:
            idx = -1
                                                                                       
        j = idx - 1
        while j >= 0 and siblings[j].type in (
            "attribute_item", "inner_attribute_item",
            "line_comment", "block_comment",
        ):
            if siblings[j].type in ("attribute_item", "inner_attribute_item"):
                text = siblings[j].text.decode(errors="replace")
                if "no_mangle" in text:
                    has_no_mangle = True
                                             
                #   #[test]            — bare unit test attr
                #   #[cfg(test)]       — module-level test gate
                                                                 
                                                              
                            
                stripped = text.strip()
                if (stripped == "#[test]"
                    or "cfg(test)" in stripped and "not(test)" not in stripped
                    or "cfg (test)" in stripped
                    or "::test]" in stripped):                             
                    has_test_marker = True
            j -= 1

                                                                   
                          
    is_extern_c = False
    for child in fn_item.children:
        if child.type == "function_modifiers":
                                                  
            if b'extern' in child.text and b'"C"' in child.text:
                is_extern_c = True

    if has_no_mangle and is_extern_c:
        return "extern_c_facing"
    if has_test_marker:
        return "test_fn"
                                       
    if file_rel.startswith("tests/") or "/tests/" in file_rel:
        return "test_fn"

    return "rust_same_crate"


def _snippet(source_bytes: bytes, node: Node, max_len: int = 200) -> str:
    ""                     
                                          
    row_start = node.start_point[0]
                                 
    lines = source_bytes.split(b"\n")
    if row_start < len(lines):
        text = lines[row_start].decode("utf-8", errors="replace").strip()
        if len(text) > max_len:
            text = text[:max_len - 3] + "..."
        return text
    return ""


# ── CallerLookup ───────────────────────────────────────────────────────────

class CallerLookup:
    ""                                           

                                                      
                                                        

         
                                               
                                              
       

    def __init__(self, crate_dir: Path | str, scan_tests: bool = True):
        self._crate = Path(crate_dir).resolve()
        self._scan_tests = scan_tests
        self._cache: dict[str, list[CallerSite]] = {}
        self._sha = _git_head_sha(self._crate)
        self._files_cache: list[tuple[Path, str, bytes, Node]] | None = None

    def callers_of(self, fn_name: str) -> list[CallerSite]:
        ""                                   

                                                                          
                                                                     
           
                                                     
        current = _git_head_sha(self._crate)
        if current != self._sha:
            logger.debug("[caller_lookup] git HEAD 变 (%s → %s),cache 清",
                         self._sha[:8], current[:8])
            self._cache.clear()
            self._files_cache = None
            self._sha = current

        if fn_name in self._cache:
            return self._cache[fn_name]

        result = self._scan(fn_name)
        self._cache[fn_name] = result
        return result

    def invalidate(self) -> None:
        ""                                                
        self._cache.clear()
        self._files_cache = None
        self._sha = _git_head_sha(self._crate)

    def _load_files(self) -> list[tuple[Path, str, bytes, Node]]:
        ""                                                                     
        if self._files_cache is not None:
            return self._files_cache

        parser = _get_parser()
        roots = [self._crate / "src"]
        if self._scan_tests and (self._crate / "tests").is_dir():
            roots.append(self._crate / "tests")

        out: list[tuple[Path, str, bytes, Node]] = []
        for root in roots:
            if not root.is_dir():
                continue
            for rs_file in root.rglob("*.rs"):
                try:
                    source = rs_file.read_bytes()
                except OSError:
                    continue
                try:
                    tree = parser.parse(source)
                except Exception:
                    continue
                rel = str(rs_file.relative_to(self._crate))
                out.append((rs_file, rel, source, tree.root_node))
        self._files_cache = out
        return out

    def _scan(self, fn_name: str) -> list[CallerSite]:
        ""                                         
        fn_name_bytes = fn_name.encode()
        results: list[CallerSite] = []

        for _path, rel, source, root in self._load_files():
            for node in _walk(root):
                if node.type != "call_expression":
                    continue
                func = node.child_by_field_name("function")
                if func is None:
                    continue
                callee = _extract_callee_name(func)
                if callee != fn_name:
                    continue
                                           
                enclosing = _enclosing_fn(node)
                if enclosing is None:
                                                         
                    continue
                caller_fn = _fn_name(enclosing)
                kind = _classify_caller(enclosing, rel)
                row, col = node.start_point
                results.append(CallerSite(
                    file=rel,
                    line=row + 1,
                    col=col + 1,
                    caller_fn=caller_fn,
                    context=_snippet(source, node),
                    kind=kind,
                    start_byte=node.start_byte,
                    end_byte=node.end_byte,
                ))
        logger.debug("[caller_lookup] scanned crate for %s: %d caller(s)",
                     fn_name, len(results))
        return results
