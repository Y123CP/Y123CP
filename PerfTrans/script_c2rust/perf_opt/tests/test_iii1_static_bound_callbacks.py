"""A global allocator hook is not a callback parameter, however it parses.

III① replaces an indirect call with a monomorphic one. That is sound when
the callee is bound by the function — a parameter, a local — and unsound
when it is a module-level `static [mut]` any translation unit can reassign
at run time; proving nobody does is a whole-program argument the rule does
not make.

The two cases were indistinguishable downstream. `build_local_scope` folds
module-level statics into the scope so that `xmlFree.expect("..")(args)`
resolves at all, and once folded, the site comes out labelled `B_param` —
the same label a genuine callback parameter gets. Measured across the
dataset: libxml2 produced 2059 static-bound sites against 117 bound by
parameter or local, and every static-bound one the run reached was declined
by the model, one call apiece, always for this reason. Its two
parameter-bound sites (`channel`) committed.

Nine of the eleven projects have no static-bound sites at all, so this
distinction costs them nothing.
"""

from __future__ import annotations

import pathlib

import pytest
from tree_sitter import Language, Parser
import tree_sitter_rust

from perf_opt.hot_probe.class_III import iii1_callback as m
from perf_opt.hot_probe.class_III.cst_utils import (
    FnCstEntry,
    build_local_scope,
    collect_fn_items,
    collect_static_items,
    collect_struct_fields,
    collect_type_aliases,
    parse_crate,
)

_SOURCE = '''
pub type FreeFunc = Option<unsafe extern "C" fn(*mut ()) -> ()>;

pub static mut xmlFree: FreeFunc = None;
pub static mut characters: FreeFunc = None;

pub struct Sax { pub characters: FreeFunc }

pub unsafe fn via_static(p: *mut ()) {
    xmlFree.expect("non-null function pointer")(p);
}

pub unsafe fn via_param(channel: FreeFunc, p: *mut ()) {
    channel.expect("non-null function pointer")(p);
}

pub unsafe fn via_local(p: *mut ()) {
    let cb: FreeFunc = xmlFree;
    cb.expect("non-null function pointer")(p);
}

pub unsafe fn via_field(sax: *mut Sax, p: *mut ()) {
    (*sax).characters.expect("non-null function pointer")(p);
}
'''


@pytest.fixture
def sites(tmp_path: pathlib.Path) -> dict[str, m.CallSite]:
    src = tmp_path / "src"
    src.mkdir()
    (src / "lib.rs").write_text(_SOURCE, encoding="utf-8")
    trees = parse_crate(tmp_path)
    structs = collect_struct_fields(trees)
    statics = collect_static_items(trees)
    aliases = collect_type_aliases(trees)
    out: dict[str, m.CallSite] = {}
    for key, entry in collect_fn_items(trees).items():
        name = str(key).split("::")[-1]
        for site in m.detect(entry, structs, aliases, statics).hits:
            out[name] = site
    return out


def test_a_static_hook_is_labelled_static(sites) -> None:
    site = sites["via_static"]
    assert site.form == "B_param", "the form alone cannot tell the two apart"
    assert site.binding == "static"


def test_a_parameter_callback_is_labelled_param(sites) -> None:
    site = sites["via_param"]
    assert site.form == "B_param"
    assert site.binding == "param"


def test_a_local_binding_is_labelled_let(sites) -> None:
    assert sites["via_local"].binding == "let"


def test_a_struct_field_is_not_looked_up_in_the_scope(sites) -> None:
    """`characters` is both a struct field here and a module-level static.

    Form B_field records the FIELD name, which is not a scope binding at all.
    Resolving it against the scope would label this site `static` on a name
    collision and strip a rule from a site the rule can actually rewrite.
    """
    site = sites["via_field"]
    assert site.form == "B_field"
    assert site.binding == ""


def test_scope_origins_are_reported_without_changing_the_scope(
    tmp_path: pathlib.Path,
) -> None:
    src = tmp_path / "src"
    src.mkdir()
    (src / "lib.rs").write_text(_SOURCE, encoding="utf-8")
    trees = parse_crate(tmp_path)
    statics = collect_static_items(trees)
    entry = next(
        e for k, e in collect_fn_items(trees).items()
        if str(k).endswith("via_param")
    )
    origins: dict[str, str] = {}
    with_origins = build_local_scope(entry, statics, origins=origins)
    without = build_local_scope(entry, statics)
    assert with_origins.keys() == without.keys()
    assert origins["channel"] == "param"
    assert origins["xmlFree"] == "static"


def test_static_bound_sites_do_not_reach_fn_hits() -> None:
    """The filter that spends the model call, or does not."""
    from perf_opt.hot_probe.merged_hits import iter_class_iii

    def site(callee: str, binding: str) -> m.CallSite:
        return m.CallSite(
            file="src/lib.rs", line=10, col=5, callee_name=callee,
            marker_seen=False, form="B_param", binding=binding,
        )

    class Scan:
        iii1_hits = {
            "crate::hot": [site("xmlFree", "static"), site("channel", "param")]
        }
        iii2_hits: dict = {}
        iii3_hits: dict = {}
        iii4_hits: dict = {}

    hits = iter_class_iii(Scan())["hot"]
    callees = [h.extra["callee_name"] for h in hits]
    assert "channel" in callees
    assert "xmlFree" not in callees
    assert hits[0].extra["binding"] == "param"
