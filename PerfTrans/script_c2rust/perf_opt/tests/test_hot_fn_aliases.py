"""Hot-function names must carry both the link name and the source name.

`hot_fn_names` is built from perf, so it holds LINK names. Scanners that
attribute through DWARF report SOURCE names. The two differ whenever
`#[export_name]` is in play, and c2rust emits it for every C identifier that
is a Rust keyword — `match` becomes `fn match_0` plus
`#[export_name = "match"]`.

Without the alias expansion the `hot_fns` prune in `class_I.scan` drops every
hit the DWARF side produced for that function: measured on fzy, all three C12
sites on `match_0` (26,632 bytes) were discarded against a hot set that knew
the same function only as `match`.
"""

from __future__ import annotations

import pytest

from perf_opt.hot_probe.symbol_source import build_fn_index

pytest.importorskip("tree_sitter_rust")


def _index(tmp_path, body: str):
    (tmp_path / "src").mkdir(parents=True, exist_ok=True)
    (tmp_path / "src" / "m.rs").write_text(body, encoding="utf-8")
    return build_fn_index(tmp_path)


KEYWORD_FN = '#[export_name = "match"]\nfn match_0() {}\nfn other() {}\n'


def test_the_link_name_resolves_to_the_source_name(tmp_path) -> None:
    assert _index(tmp_path, KEYWORD_FN).aliases_of("match") == {"match_0"}


def test_and_the_reverse(tmp_path) -> None:
    assert _index(tmp_path, KEYWORD_FN).aliases_of("match_0") == {"match"}


def test_a_name_is_never_its_own_alias(tmp_path) -> None:
    idx = _index(tmp_path, KEYWORD_FN)
    assert "match" not in idx.aliases_of("match")


def test_a_plain_fn_has_no_aliases(tmp_path) -> None:
    assert _index(tmp_path, KEYWORD_FN).aliases_of("other") == set()


def test_an_unknown_name_has_no_aliases(tmp_path) -> None:
    """A `#[bitfield]`-generated accessor has no definition and must stay empty."""
    assert _index(tmp_path, KEYWORD_FN).aliases_of("status_code") == set()


def test_two_unrelated_fns_do_not_alias_each_other(tmp_path) -> None:
    idx = _index(tmp_path, "fn a() {}\nfn b() {}\n")
    assert idx.aliases_of("a") == set()


def test_expansion_is_what_a_hot_set_would_do(tmp_path) -> None:
    """The exact shape `driver` uses when building `hot_fn_names`."""
    idx = _index(tmp_path, KEYWORD_FN)
    hot = {"match"}                       # what perf reported
    expanded = set(hot)
    for n in hot:
        expanded |= idx.aliases_of(n)
    assert expanded == {"match", "match_0"}


# ─────────────────── fn_hits keys must match hotspots keys

def test_merge_hits_files_dwarf_named_hits_under_the_hotspot_name(tmp_path) -> None:
    """The agent looks a function up in fn_hits.json by its HOTSPOT name.

    `locate` names it from perf (`match`); the IR scanners attribute through
    DWARF (`match_0`). Filed under the DWARF name, the lookup misses and the
    agent logs "no fn_hits entry, skip" — measured on fzy over 26,632 bytes
    of removable zeroing that every earlier stage had correctly found.
    """
    from perf_opt.hot_probe.merged_hits import Hit, merge_hits

    idx = _index(tmp_path, KEYWORD_FN)

    class _CI:
        c3_hits = None
    # stand in for iter_class_i's output: keyed by the DWARF/source name
    import perf_opt.hot_probe.merged_hits as mh
    real = mh.iter_class_i
    mh.iter_class_i = lambda sr, crate_root=None, fn_index=None: {
        "match_0": [Hit(rule="C12", pattern="oversized-zero-init-kb",
                        file="src/m.rs", line=1, col=1, snippet="",
                        extra={"zeroed_bytes": 8192})]}
    try:
        merged = merge_hits(_CI(), None, None, None, idx,
                            crate_root=tmp_path, hot_fn_names={"match"})
    finally:
        mh.iter_class_i = real

    assert "match" in merged, sorted(merged)
    assert "match_0" not in merged
    assert merged["match"].hits[0].rule == "C12"


def test_a_name_already_in_the_hot_set_is_left_alone(tmp_path) -> None:
    from perf_opt.hot_probe.merged_hits import Hit, merge_hits
    import perf_opt.hot_probe.merged_hits as mh

    idx = _index(tmp_path, KEYWORD_FN)

    class _CI:
        c3_hits = None
    real = mh.iter_class_i
    mh.iter_class_i = lambda sr, crate_root=None, fn_index=None: {
        "other": [Hit(rule="C1", pattern="p", file="src/m.rs", line=3,
                      col=1, snippet="", extra={})]}
    try:
        merged = merge_hits(_CI(), None, None, None, idx,
                            crate_root=tmp_path, hot_fn_names={"other"})
    finally:
        mh.iter_class_i = real
    assert "other" in merged


def test_a_widened_hot_set_does_not_defeat_the_key_fold(tmp_path) -> None:
    """The two sets have different jobs and must not be conflated.

    `hot_fn_names` is deliberately widened with aliases so each scanner's own
    `hot_fns` prune keeps sites filed under either name. `canonical_fn_names`
    is the un-widened hotspot set, and folding must test membership against
    THAT — against the widened set, `match_0` reads as "already hot" and the
    fold is skipped, leaving fn_hits.json keyed by a name the agent never
    looks up. Measured: both fixes in place, key still `match_0`, agent still
    logged "no fn_hits entry, skip".
    """
    from perf_opt.hot_probe.merged_hits import Hit, merge_hits
    import perf_opt.hot_probe.merged_hits as mh

    idx = _index(tmp_path, KEYWORD_FN)

    class _CI:
        c3_hits = None
    real = mh.iter_class_i
    mh.iter_class_i = lambda sr, crate_root=None, fn_index=None: {
        "match_0": [Hit(rule="C12", pattern="oversized-zero-init-kb",
                        file="src/m.rs", line=2, col=1, snippet="",
                        extra={"zeroed_bytes": 8192})]}
    try:
        merged = merge_hits(
            _CI(), None, None, None, idx, crate_root=tmp_path,
            hot_fn_names={"match", "match_0"},      # widened, as driver builds it
            canonical_fn_names={"match"},           # what hotspots.json says
        )
    finally:
        mh.iter_class_i = real

    assert "match" in merged and "match_0" not in merged, sorted(merged)


def test_without_the_canonical_set_it_falls_back(tmp_path) -> None:
    """Older callers pass only `hot_fn_names`; behaviour must not change."""
    from perf_opt.hot_probe.merged_hits import Hit, merge_hits
    import perf_opt.hot_probe.merged_hits as mh

    idx = _index(tmp_path, KEYWORD_FN)

    class _CI:
        c3_hits = None
    real = mh.iter_class_i
    mh.iter_class_i = lambda sr, crate_root=None, fn_index=None: {
        "match_0": [Hit(rule="C1", pattern="p", file="src/m.rs", line=2,
                        col=1, snippet="", extra={})]}
    try:
        merged = merge_hits(_CI(), None, None, None, idx,
                            crate_root=tmp_path, hot_fn_names={"match"})
    finally:
        mh.iter_class_i = real
    assert "match" in merged
