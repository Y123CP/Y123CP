"""A rewrite cannot name a type it was never shown.

c2rust puts struct definitions in one module and the functions that walk them
in another, and a function's own text names only the outermost one. A function
taking `*mut hashmap_t` and indexing `hm.cell[i].e[j].k` never spells the
element type of `e` — it is three definitions away. Asked to slice that walk,
the model invented `hashmap_entry_t` where the crate says `hashentry_t`, and
the compile error that came back said the name was unresolved without saying
which name was not. Four consecutive runs of the crate, three LLM turns and
three builds burned each time.

Two fixes, tested here: show the reachable definitions up front, and — for a
build that failed anyway — say what the crate actually defines.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from perf_opt.agent_perf_opt.type_context import (
    _index,
    build_type_context,
    name_correction_note,
    unresolved_names,
)


# A crate in the shape c2rust emits: aliases in one file, composites in
# another, attributes on the lines above each definition, and a field whose
# element type is only reachable through two other definitions.
_C_TYPES = """\
use core::ffi::*;

pub type size_t = usize;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type map_t = _map_t;
"""

_LIB = """\
use core::ffi::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _map_t {
    pub cell: [cell_t; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell_t {
    pub e: *mut entry_t,
    pub c: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry_t {
    pub k: *mut c_char,
    pub v: *mut c_void,
}

#[repr(C)]
pub union bits_t {
    pub w: u64,
    pub b: [u8; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum mode_t {
    Fast = 0,
    Small = 1,
}

pub struct Handle(*mut _map_t);
"""


@pytest.fixture()
def crate(tmp_path: Path) -> Path:
    src = tmp_path / "src"
    src.mkdir()
    (src / "c_types.rs").write_text(_C_TYPES, encoding="utf-8")
    (src / "lib.rs").write_text(_LIB, encoding="utf-8")
    return tmp_path


# ───────────────────────────────────────────────────────── the index

def test_every_definition_kind_is_indexed(crate: Path) -> None:
    index = _index(crate)
    assert index["map_t"].kind == "type"
    assert index["_map_t"].kind == "struct"
    assert index["bits_t"].kind == "union"
    assert index["mode_t"].kind == "enum"
    assert index["Handle"].kind == "struct"


def test_a_definition_carries_its_attributes(crate: Path) -> None:
    """`#[repr(C)]` is why the layout is what the walk assumes."""
    text = _index(crate)["cell_t"].text
    assert text.startswith("#[derive(Copy, Clone)]")
    assert "#[repr(C)]" in text
    assert text.rstrip().endswith("}")


def test_a_tuple_struct_ends_at_its_semicolon(crate: Path) -> None:
    text = _index(crate)["Handle"].text
    assert text.strip() == "pub struct Handle(*mut _map_t);"


def test_an_alias_ends_at_its_semicolon(crate: Path) -> None:
    assert _index(crate)["map_t"].text.strip() == "pub type map_t = _map_t;"


# ────────────────────────────────────────── the closure, which is the point

_WALK = """\
unsafe fn destroy(m: *mut map_t) {
    let v: &map_t = &*m;
    let mut i: size_t = 0;
    while i < 256 {
        free((*v.cell[i as usize].e.offset(0)).k as *mut c_void);
        i = i.wrapping_add(1);
    }
}
"""


def test_the_element_type_two_hops_away_arrives(crate: Path) -> None:
    """`entry_t` appears nowhere in the function. That is the whole bug."""
    assert "entry_t" not in _WALK
    ctx = build_type_context(crate, _WALK)
    assert "pub struct entry_t" in ctx
    assert "pub struct cell_t" in ctx
    assert "pub struct _map_t" in ctx
    assert "pub type map_t" in ctx


def test_definitions_come_nearest_first(crate: Path) -> None:
    """So that a truncating budget drops the far ones, not the near ones."""
    ctx = build_type_context(crate, _WALK)
    order = [ctx.index(n) for n in
             ("pub type map_t", "pub struct _map_t",
              "pub struct cell_t", "pub struct entry_t")]
    assert order == sorted(order)


def test_a_scalar_alias_is_dropped(crate: Path) -> None:
    """`size_t = usize` is noise; it is also most of a c2rust type table."""
    ctx = build_type_context(crate, "fn f(n: size_t, m: int64_t) {}")
    assert ctx == ""


def test_an_alias_chain_to_a_composite_is_kept(crate: Path) -> None:
    ctx = build_type_context(crate, "fn f(m: *mut map_t) {}")
    assert "pub type map_t = _map_t;" in ctx
    assert "pub struct _map_t" in ctx


def test_an_arithmetic_kernel_gets_no_block_at_all(crate: Path) -> None:
    """Most hot functions touch no composite. They must pay nothing."""
    src = "fn crc(mut x: u32) -> u32 { x ^= 0xffff_ffff; x >> 3 }"
    assert build_type_context(crate, src) == ""


def test_an_unreachable_type_stays_out(crate: Path) -> None:
    ctx = build_type_context(crate, _WALK)
    assert "bits_t" not in ctx
    assert "mode_t" not in ctx


def test_the_definition_count_is_capped(crate: Path) -> None:
    ctx = build_type_context(crate, _WALK, max_defs=2)
    assert "pub type map_t" in ctx
    assert "pub struct entry_t" not in ctx


def test_the_character_budget_is_respected(crate: Path) -> None:
    ctx = build_type_context(crate, _WALK, max_chars=60)
    assert len(ctx) < 400
    assert "pub type map_t = _map_t;" in ctx


def test_a_missing_crate_directory_is_not_an_error(tmp_path: Path) -> None:
    assert build_type_context(tmp_path / "nope", _WALK) == ""


def test_the_index_is_cached_per_crate(crate: Path) -> None:
    """Cached because no rule in the set rewrites a type definition — bodies,
    signatures and regions are what get replaced, and field retyping is
    deliberately out of scope because W1 cannot verify it. If a rule ever
    does edit one, this cache serves a stale definition and has to go."""
    first = _index(crate)
    assert _index(crate) is first


# ─────────────────────────────────────── after a build has already failed

def test_the_closest_real_name_is_offered(crate: Path) -> None:
    stderr = "error[E0412]: cannot find type `map_entry_t` in this scope\n"
    note = name_correction_note(crate, stderr)
    assert "`map_entry_t` does not exist" in note
    assert "`entry_t`" in note


@pytest.mark.parametrize("code", ["E0412", "E0422", "E0425", "E0433", "E0405"])
def test_every_unresolved_name_diagnostic_is_read(code: str) -> None:
    stderr = f"error[{code}]: cannot find value `wonky_name` in this scope"
    assert unresolved_names(stderr) == ["wonky_name"]


def test_a_name_is_reported_once_however_often_it_recurs() -> None:
    stderr = ("error[E0412]: cannot find type `map_entry_t` in this scope\n"
              "error[E0412]: cannot find type `map_entry_t` in this scope\n")
    assert unresolved_names(stderr) == ["map_entry_t"]


def test_an_unrelated_compile_error_adds_nothing(crate: Path) -> None:
    assert name_correction_note(
        crate, "error: unexpected closing delimiter: `}`") == ""


def test_a_name_with_no_near_neighbour_adds_nothing(crate: Path) -> None:
    """Offering a wild guess is worse than offering nothing."""
    stderr = "error[E0412]: cannot find type `zzqqwx_unrelated` in this scope"
    assert name_correction_note(crate, stderr) == ""


def test_an_empty_stderr_adds_nothing(crate: Path) -> None:
    assert name_correction_note(crate, "") == ""
    assert name_correction_note(crate, None) == ""


def test_the_note_is_short(crate: Path) -> None:
    """It rides on a retry prompt that already carries the whole function."""
    stderr = "\n".join(
        f"error[E0412]: cannot find type `map_entry_{i}` in this scope"
        for i in range(20))
    assert len(name_correction_note(crate, stderr)) < 800


# ─────────────────────────────── the prompts that write code carry the block

_FN_SOURCE = """\
unsafe fn destroy(m: *mut map_t) {
    let v: &map_t = &*m;
    free((*v.cell[0].e.offset(0)).k as *mut c_void);
}
"""


@pytest.fixture()
def target(crate: Path):
    """An EditTarget over a real file, so `_read_target_source` reads it."""
    from perf_opt.agent_perf_opt.rewrite_applier import EditTarget
    path = crate / "src" / "walk.rs"
    path.write_text(_FN_SOURCE, encoding="utf-8")
    return EditTarget(file=path, fn_name="destroy",
                      span=(0, len(_FN_SOURCE.encode())))


def _evidence():
    from perf_opt.hot_probe.types import EvidencePack, HotFunction
    return (HotFunction(name="destroy", self_pct=12.0, hottest_op="op"),
            EvidencePack(
                symbol="destroy", location="src/walk.rs:1-4", workload="op",
                hot_region=None, rust_source=_FN_SOURCE,
                self_time_ratio=0.12, retired_instructions=None, cpi=None))


def test_the_direct_prompt_carries_the_definitions(crate: Path, target) -> None:
    from perf_opt.agent_perf_opt.config import AgentConfig
    from perf_opt.agent_perf_opt.prompt_builder import build_multi_card_prompt
    hf, ep = _evidence()
    _, user = build_multi_card_prompt(
        hf, ep, target, ["III④"], [], AgentConfig(), crate_dir=crate)
    assert "pub struct entry_t" in user


def test_the_execute_prompt_carries_the_definitions(crate: Path, target) -> None:
    from perf_opt.agent_perf_opt.config import AgentConfig
    from perf_opt.agent_perf_opt.prompt_builder import build_execute_prompt
    hf, ep = _evidence()
    plan = {"applied_rules": [{"rule": "III④", "target_hits": [0]}]}
    _, user = build_execute_prompt(
        hf, ep, target, ["III④"], plan, [], AgentConfig(), crate_dir=crate)
    assert "pub struct entry_t" in user


def test_the_plan_prompt_does_not(crate: Path, target) -> None:
    """PLAN decides which rules to apply; it writes no code, so it needs no
    type names and should not pay for them."""
    from perf_opt.agent_perf_opt.config import AgentConfig
    from perf_opt.agent_perf_opt.prompt_builder import build_plan_prompt
    hf, ep = _evidence()
    _, user = build_plan_prompt(
        hf, ep, target, ["III④"], [], AgentConfig(), crate_dir=crate)
    assert "pub struct entry_t" not in user


def test_an_arithmetic_target_leaves_no_empty_section(crate: Path) -> None:
    """A blank block would show up as a stray blank run between sections."""
    from perf_opt.agent_perf_opt.config import AgentConfig
    from perf_opt.agent_perf_opt.prompt_builder import build_multi_card_prompt
    from perf_opt.agent_perf_opt.rewrite_applier import EditTarget
    src = "fn crc(mut x: u32) -> u32 { x ^ 0xffff_ffff }\n"
    path = crate / "src" / "crc.rs"
    path.write_text(src, encoding="utf-8")
    hf, ep = _evidence()
    _, user = build_multi_card_prompt(
        hf, ep,
        EditTarget(file=path, fn_name="crc", span=(0, len(src.encode()))),
        ["C10"], [], AgentConfig(), crate_dir=crate)
    assert "Type definitions reachable" not in user
    assert "\n\n\n" not in user


def test_the_region_prompt_reaches_types_named_only_in_its_tail() -> None:
    """The region window is a slice of one function; a struct named in the
    read-only AFTER block is still one the rewrite may have to spell."""
    import inspect
    from perf_opt.agent_perf_opt import prompt_builder as pb
    src = inspect.getsource(pb.build_region_prompt)
    assert "_type_context_for(" in src
    call = src[src.index("_type_context_for("):]
    for block in ("header", "before_text", "original_region", "after_text"):
        assert block in call[:200], block
