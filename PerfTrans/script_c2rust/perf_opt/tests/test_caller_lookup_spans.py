""                                                         

                                              
                                    
   

from pathlib import Path

from perf_opt.agent_perf_opt.caller_lookup import CallerLookup


def _make_crate(tmp_path: Path, lib_src: str) -> Path:
    (tmp_path / "src").mkdir(parents=True, exist_ok=True)
    (tmp_path / "src" / "lib.rs").write_text(lib_src, encoding="utf-8")
    return tmp_path


def test_span_slices_to_exact_call_expression(tmp_path: Path) -> None:
    lib = (
        "fn target(a: i32, b: i32) -> i32 { a + b }\n"
        "fn caller_one() -> i32 { target(1, 2) }\n"
        "fn caller_two() -> i32 {\n"
        "    let r = target(target(3, 4), 5);\n"
        "    r\n"
        "}\n"
    )
    crate = _make_crate(tmp_path, lib)
    source = (crate / "src" / "lib.rs").read_bytes()

    callers = CallerLookup(crate).callers_of("target")

                                            
    assert len(callers) == 3
    for c in callers:
        assert c.start_byte < c.end_byte
        sliced = source[c.start_byte:c.end_byte].decode()
                                        
        assert sliced.startswith("target("), sliced
        assert sliced.endswith(")"), sliced

                                
    texts = sorted(
        (source[c.start_byte:c.end_byte].decode() for c in callers),
        key=len,
    )
    assert "target(1, 2)" in texts
    assert "target(3, 4)" in texts
               
    assert any(t == "target(target(3, 4), 5)" for t in texts)


def test_span_is_call_expr_not_whole_line(tmp_path: Path) -> None:
    ""                                  
    lib = (
        "fn target(x: i32) -> i32 { x }\n"
        "fn caller() -> i32 { let y = target(42) + 100; y }\n"
    )
    crate = _make_crate(tmp_path, lib)
    source = (crate / "src" / "lib.rs").read_bytes()

    callers = CallerLookup(crate).callers_of("target")
    assert len(callers) == 1
    sliced = source[callers[0].start_byte:callers[0].end_byte].decode()
    assert sliced == "target(42)"                                 
