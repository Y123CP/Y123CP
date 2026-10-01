""                         

                                                 
                                                                    
                                                              
                                              
                                                            
                              

                                         
                                      

        
                                    
                              
   

from __future__ import annotations

import importlib
import json
import os

import pytest

from tools.facet_coverage import read_project


                                                                     

def test_instrumented_build_timeout_is_configurable(monkeypatch):
    ""                                    
    monkeypatch.setenv("PERF_OPT_INSTRUMENTED_BUILD_TIMEOUT", "7200")
    import perf_opt.agent_perf_opt.driver as drv
    importlib.reload(drv)
    assert drv._INSTRUMENTED_BUILD_TIMEOUT_S == 7200


def test_instrumented_build_timeout_default_clears_the_known_failure():
    ""                                               
                    
    monkeypatch = pytest.MonkeyPatch()
    monkeypatch.delenv("PERF_OPT_INSTRUMENTED_BUILD_TIMEOUT", raising=False)
    import perf_opt.agent_perf_opt.driver as drv
    importlib.reload(drv)
    monkeypatch.undo()
    assert drv._INSTRUMENTED_BUILD_TIMEOUT_S >= 1800


                                                             

def _mk_run(tmp_path, *, with_status: bool):
    ""                                      
    d = tmp_path / "3_perf_opt"
    d.mkdir()
    (d / "rewrites.log").write_text(json.dumps({
        "fn_name": "f", "fired_rules": ["C3"], "applied_rules": ["C3"],
        "terminal_status": "committed",
    }) + "\n", encoding="utf-8")
    if with_status:
        (d / "rule_scan_status.json").write_text(json.dumps({
            "class_i": "skipped", "class_ii": "skipped", "class_iii": "ran",
            "reason": "cargo build timed out after 600s",
        }), encoding="utf-8")
    return d


def test_complete_run_is_not_flagged(tmp_path):
    ""                                 
    got = read_project(_mk_run(tmp_path, with_status=False))
    assert got is not None
    assert not got["scan_incomplete"]


def test_incomplete_run_is_flagged_with_which_scans(tmp_path):
    ""                                    
    got = read_project(_mk_run(tmp_path, with_status=True))
    assert got is not None
    assert set(got["scan_incomplete"]) == {"class_i", "class_ii"}


def test_class_iii_still_counts_when_i_and_ii_are_skipped(tmp_path):
    ""                                             
                                     
    got = read_project(_mk_run(tmp_path, with_status=True))
    assert "class_iii" not in got["scan_incomplete"]


def test_flagged_run_still_reports_its_landed_rules(tmp_path):
    ""                                     
                            
    got = read_project(_mk_run(tmp_path, with_status=True))
    assert got["landed"].get("C3") == 1
