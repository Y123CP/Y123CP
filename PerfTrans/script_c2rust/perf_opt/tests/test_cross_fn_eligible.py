""                                                                        

from types import SimpleNamespace

from perf_opt.agent_perf_opt import reporting
from perf_opt.agent_perf_opt.agent import (
    _cross_fn_eligible, _filter_editable_call_sites,
    _make_cross_fn_attempt_record,
)
from perf_opt.agent_perf_opt.state import RewriteAttempt


def test_cross_fn_record_abstained_does_not_crash():
    ""                                                   
    extra = {
        "reason": "only_in_target_call_sites",
        "terminal_status": reporting.PLANNER_ABSTAINED,
    }
    rec = _make_cross_fn_attempt_record(
        SimpleNamespace(name="LZ4F_makeBlock"), RewriteAttempt.ABSTAINED, extra,
    )
    assert rec.fn_name == "LZ4F_makeBlock"
    assert rec.rule_id == "III①"
    assert rec.fn_mode == "cross_fn"
    assert rec.applied_rules == []


def test_cross_fn_record_committed():
    extra = {
        "commit_sha": "abc123", "w2_delta_pct": -4.0, "measured_cv": 0.3,
        "terminal_status": "committed", "w1_result": "pass",
    }
    rec = _make_cross_fn_attempt_record(
        SimpleNamespace(name="f"), RewriteAttempt.APPLIED_COMMITTED, extra,
    )
    assert rec.applied_rules == ["III①"]
    assert rec.commit_sha == "abc123"
    assert rec.fn_mode == "cross_fn"


def _cs(file, start, end):
    return SimpleNamespace(file=file, start_byte=start, end_byte=end)


def test_filter_excludes_recursive_in_target_span():
                                                                
    sites = [
        _cs("src/lib.rs", 150, 170),                           
        _cs("src/other.rs", 10, 25),                 
    ]
    kept, reason = _filter_editable_call_sites(sites, "src/lib.rs", 100, 300)
    assert reason is None
    assert [(s.file, s.start_byte) for s in kept] == [("src/other.rs", 10)]


def test_filter_all_recursive_abstains():
    sites = [_cs("src/lib.rs", 150, 170), _cs("src/lib.rs", 200, 220)]
    kept, reason = _filter_editable_call_sites(sites, "src/lib.rs", 100, 300)
    assert kept == [] and reason == "only_in_target_call_sites"


def test_filter_nested_overlap_abstains():
                                                           
    sites = [_cs("src/lib.rs", 10, 40), _cs("src/lib.rs", 17, 27)]
    kept, reason = _filter_editable_call_sites(sites, "src/lib.rs", 500, 600)
    assert kept == [] and reason == "overlapping_call_sites"


def test_filter_disjoint_sites_all_kept():
    sites = [_cs("src/a.rs", 10, 20), _cs("src/a.rs", 30, 40), _cs("src/b.rs", 5, 15)]
    kept, reason = _filter_editable_call_sites(sites, "src/lib.rs", 0, 5)
    assert reason is None and len(kept) == 3


def _hf(name="target"):
    return SimpleNamespace(name=name)


def _lookup(callers):
    return SimpleNamespace(callers_of=lambda fn: callers)


def _site(kind, file="src/lib.rs", start=10):
    return SimpleNamespace(kind=kind, file=file, start_byte=start)


def test_eligible_when_iii1_and_all_in_crate():
    fn_entry = {"hits": [{"rule": "III①"}, {"rule": "C3"}]}
    callers = [_site("rust_same_crate", start=30), _site("rust_same_crate", start=10)]
    ok, sites = _cross_fn_eligible(_hf(), fn_entry, _lookup(callers))
    assert ok
                               
    assert [s.start_byte for s in sites] == [10, 30]


def test_not_eligible_without_iii1_hit():
    fn_entry = {"hits": [{"rule": "C3"}, {"rule": "III④"}]}
    callers = [_site("rust_same_crate")]
    ok, sites = _cross_fn_eligible(_hf(), fn_entry, _lookup(callers))
    assert not ok and sites == []


def test_not_eligible_when_any_caller_crosses_boundary():
    fn_entry = {"hits": [{"rule": "III①"}]}
    callers = [_site("rust_same_crate"), _site("extern_c_facing")]
    ok, _ = _cross_fn_eligible(_hf(), fn_entry, _lookup(callers))
    assert not ok


def test_not_eligible_with_no_callers():
    fn_entry = {"hits": [{"rule": "III①"}]}
    ok, _ = _cross_fn_eligible(_hf(), fn_entry, _lookup([]))
    assert not ok


def test_not_eligible_when_fn_entry_none():
    ok, sites = _cross_fn_eligible(_hf(), None, _lookup([_site("rust_same_crate")]))
    assert not ok and sites == []
