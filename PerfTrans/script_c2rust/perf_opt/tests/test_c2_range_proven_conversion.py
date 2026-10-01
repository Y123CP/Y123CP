""                                                      

                                                           
                                                         
                                               
                                           
                                

           
                                                                    
                                                              
                                              
          

                            
   

from __future__ import annotations

import re
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.rewrite_applier import (
    _RULE_EXEC_FINGERPRINTS,
    find_undeclared_executions,
)
from perf_opt.hot_probe.class_I.rules import _matches_C2
from perf_opt.hot_probe.merged_hits import _classify_c2_pattern

CARD = (Path(__file__).resolve().parents[1]
        / "agent_perf_opt" / "Optimization_Card" / "C2_scalar_conversion.md")


                                                                           

@pytest.mark.parametrize("callee", [
    "llvm.fptosi.sat.i32.f64",       # float -> signed
    "llvm.fptosi.sat.i64.f32",
    "llvm.fptoui.sat.i32.f64",                                    
    "llvm.fptoui.sat.i64.f64",
    "llvm.fptosi.sat.v4i32.v4f32",         
    "llvm.fptoui.sat.v8i32.v8f32",
])
def test_detector_covers_signed_and_unsigned(callee):
    assert _matches_C2(callee)


@pytest.mark.parametrize("callee", [
    "llvm.fptosi.i32.f64",                              
    "llvm.sitofp.f64.i32",                
    "llvm.ctlz.i32",
    "llvm.bswap.i32",                                      
    "core::panicking::panic_bounds_check",
])
def test_detector_rejects_non_c2(callee):
    assert not _matches_C2(callee)


                                                                     

@pytest.mark.parametrize("callee,sign,width", [
    ("llvm.fptosi.sat.i32.f64",     "int",  "scalar"),
    ("llvm.fptoui.sat.i32.f64",     "uint", "scalar"),
    ("llvm.fptosi.sat.v4i32.v4f32", "int",  "vector"),
    ("llvm.fptoui.sat.v4i32.v4f32", "uint", "vector"),
])
def test_pattern_carries_both_dimensions(callee, sign, width):
    ""                                                        
                                        
    pat = _classify_c2_pattern(callee)
    assert pat == f"float-to-{sign}-{width}", pat


def test_pattern_distinguishes_signedness():
    ""                                                 
                           
    assert (_classify_c2_pattern("llvm.fptosi.sat.i32.f64")
            != _classify_c2_pattern("llvm.fptoui.sat.i32.f64"))


                                                                 

_PRIMARY = "let i: I = unsafe { f.to_int_unchecked::<i32>() };"
_TURBOFISH_FREE = "let i: i32 = unsafe { f.to_int_unchecked() };"
_FALLBACK = "let i: i32 = f.clamp(i32::MIN as f64, i32::MAX as f64) as i32;"


@pytest.mark.parametrize("code", [_PRIMARY, _TURBOFISH_FREE, _FALLBACK])
def test_fingerprint_accepts_card_taught_rewrites(code):
    ""                               
    assert find_undeclared_executions(["C2"], code) == []


def test_fingerprint_rejects_empty_declaration():
    ""                                
    code = "// Applied rules: [C2]\nlet i: i32 = f as i32;"
    assert find_undeclared_executions(["C2"], code) == ["C2"]


def test_fingerprint_rejects_stale_byteswap_form():
    ""                                                   
                            
    code = "let v = u32::from_be(raw.swap_bytes());"
    assert find_undeclared_executions(["C2"], code) == ["C2"]


                                                                

def test_card_teaches_every_fingerprinted_api():
    ""                                           
    card = CARD.read_text(encoding="utf-8")
    for fp in _RULE_EXEC_FINGERPRINTS["C2"]:
        api = fp.lstrip(".").rstrip("(:")
        assert api in card, f"fingerprint {fp!r} 未在卡片中教过"


def test_card_states_the_unsigned_obligation():
    ""                                           
    card = CARD.read_text(encoding="utf-8")
    assert re.search(r"[Uu]nsigned targets? carr(y|ies) an extra obligation", card)


def test_card_names_floor_as_an_anti_pattern():
    ""                                                
                               
    card = CARD.read_text(encoding="utf-8")
    anti = card.split("## 4. Anti-patterns")[1].split("## 5.")[0]
    for kw in ("floor", "ceil", "trunc"):
        assert kw in anti, f"{kw} 未列入反模式"


def test_card_keeps_integer_casts_out_of_scope():
    ""                                       
                               
    card = CARD.read_text(encoding="utf-8")
    assert "Not in scope" in card
    scope = card.split("Not in scope")[1][:400]
    assert "as isize" in scope or "integer-to-integer" in scope
