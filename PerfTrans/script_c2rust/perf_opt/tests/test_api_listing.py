"""The API listing handed to the harness generator.

A model can only drive functions it has been shown. The listing used to take
functions in file order until a character budget ran out, which on a large
crate meant whole modules fell off the end — and a model that cannot see a
module cannot write an op for it, no matter how many coverage-extension rounds
it gets. Worse, nothing told it the list was partial, so it filled the gaps
from memory of the upstream C library and produced imports for names that are
private or absent here.

Measured on a 1603-function crate: 72% of the API cut (all of it the tail),
function coverage stalled at ~10%, and `E0603 private` / `E0432 unresolved
import` appeared in every single run, consuming the repair budget that the
coverage extension needed.

These tests pin the properties, not the numbers of any one project.
"""

from __future__ import annotations

from dataclasses import dataclass

import pytest

from harness_gen.prompts import _compress_signature, _fn_listing


@dataclass
class _Fn:
    name: str
    link_name: str
    signature: str
    module_path: str
    file: str = "src/x.rs"


class _Inv:
    def __init__(self, fns, crate_name="proj_raw", duplicated=None):
        self.fns = fns
        self.crate_name = crate_name
        self.structs: list[str] = []
        self.lib_rs = ""
        self.duplicated_types = duplicated or {}


def _crate(n_modules: int, per_module: int, sig_len: int = 60) -> _Inv:
    fns = []
    for m in range(n_modules):
        for i in range(per_module):
            pad = "x" * max(0, sig_len - 30)
            fns.append(_Fn(
                name=f"m{m}_f{i}",
                link_name=f"m{m}_f{i}",
                signature=f'pub unsafe extern "C" fn m{m}_f{i}(a: *const {pad})',
                module_path=f"src::mod{m}"))
    return _Inv(fns)


# ────────────────────────────────────────────────── compression is lossless

def test_compression_strips_uniform_c2rust_noise() -> None:
    sig = ('pub unsafe extern "C" fn htmlAutoCloseTag( mut doc: htmlDocPtr,'
           " mut name: *const xmlChar, ) -> ::core::ffi::c_int")
    out = _compress_signature(sig)
    assert out == ("fn htmlAutoCloseTag(doc: htmlDocPtr, name: *const xmlChar)"
                   " -> c_int")


def test_compression_keeps_the_name_and_every_type() -> None:
    """Shorter must not mean less informative — the caller needs the types."""
    sig = 'pub unsafe extern "C" fn f(a: *mut u8, b: usize) -> *const i32'
    out = _compress_signature(sig)
    for token in ("f", "*mut u8", "usize", "*const i32"):
        assert token in out


def test_compression_does_not_eat_lookalike_identifiers() -> None:
    sig = 'pub unsafe extern "C" fn f(x: mycore::ffi::T, mutable: u8)'
    out = _compress_signature(sig)
    assert "mycore::ffi::T" in out
    assert "mutable: u8" in out


# ─────────────────────────────────────── small crates: nothing may be dropped

@pytest.mark.parametrize("n_modules,per_module", [(1, 15), (1, 77), (5, 28)])
def test_a_crate_that_fits_is_listed_in_full(n_modules, per_module) -> None:
    inv = _crate(n_modules, per_module)
    listing = _fn_listing(inv)
    assert "omitted" not in listing
    for fn in inv.fns:
        assert fn.name in listing


def test_every_module_gets_its_import_path_header(monkeypatch) -> None:
    inv = _crate(3, 4)
    listing = _fn_listing(inv)
    for m in range(3):
        assert f"## use proj_raw::src::mod{m}::<fn>;" in listing


# ──────────────────────────────────── large crates: breadth over file order

def test_truncation_still_shows_every_module() -> None:
    """The regression: truncating in file order made late modules invisible."""
    inv = _crate(n_modules=40, per_module=40, sig_len=120)
    listing = _fn_listing(inv, limit_chars=20_000)
    assert "omitted" in listing            # the budget really did bind
    for m in range(40):
        assert f"src::mod{m}" in listing, f"module {m} is invisible"


def test_truncation_is_roughly_even_across_modules() -> None:
    inv = _crate(n_modules=10, per_module=50, sig_len=120)
    listing = _fn_listing(inv, limit_chars=12_000)
    counts = []
    for m in range(10):
        counts.append(sum(1 for line in listing.splitlines()
                          if line.startswith(f"fn m{m}_")))
    assert min(counts) >= 1
    assert max(counts) - min(counts) <= 1


def test_budget_is_respected() -> None:
    inv = _crate(n_modules=20, per_module=50, sig_len=150)
    for budget in (5_000, 20_000, 60_000):
        assert len(_fn_listing(inv, limit_chars=budget)) <= budget * 1.15


def test_compression_buys_real_headroom() -> None:
    """More functions fit than before, at the same budget."""
    inv = _crate(n_modules=8, per_module=60)
    shown = sum(1 for l in _fn_listing(inv, limit_chars=9_000).splitlines()
                if l.startswith("fn "))
    raw_len = sum(len(f"[{f.module_path}] {f.signature}") + 1 for f in inv.fns)
    naive = 0
    used = 0
    for f in inv.fns:
        used += len(f"[{f.module_path}] {f.signature}") + 1
        if used > 9_000:
            break
        naive += 1
    assert shown > naive, f"compressed={shown} vs naive={naive} (raw {raw_len})"


# ─────────────────────────────── a partial list must announce that it is partial

def test_omission_notice_states_the_counts_and_forbids_guessing() -> None:
    inv = _crate(n_modules=12, per_module=40, sig_len=140)
    listing = _fn_listing(inv, limit_chars=10_000)
    assert "of 480 functions omitted" in listing
    assert "all 12 modules" in listing
    # the two failure modes it must prevent
    assert "do not call it" in listing            # omitted ≠ callable
    assert "import ONLY names listed above" in listing   # no recall from memory


def test_no_omission_notice_when_nothing_is_omitted() -> None:
    assert "omitted" not in _fn_listing(_crate(2, 5))


# ────────────────────────────────────────── the dependency set must be stated

@pytest.mark.parametrize("builder", ["plan_user", "codegen_user"])
def test_prompts_state_the_only_available_dependencies(builder) -> None:
    """`use libc::…` produced 8 build errors in one run; the harness manifest
    has exactly one dependency and never said so."""
    import harness_gen.prompts as prompts

    inv = _crate(1, 2)
    fn = getattr(prompts, builder)
    text = fn(inv) if builder == "plan_user" else fn(inv, {"operations": []})
    assert "NOTHING ELSE" in text
    assert "libc" in text


# ────────────────────── the import path binds as hard as the name

def test_import_path_instruction_is_unconditional() -> None:
    """It used to ride along with the truncation notice, so a crate whose
    listing fits whole carried no warning at all — and a model that recognises
    the vendored upstream library then imports from memory.

    Measured on a 359-function crate (a vendored libpng) that does NOT
    truncate: three imports in the first attempt named REAL functions under
    the WRONG module — `png_set_error_fn` from `png` when the listing says
    `pngerror`, `png_data_freer` from `pngset` when the listing says `png`.
    The names were right; only the paths were invented."""
    complete = _fn_listing(_crate(3, 5))
    assert "omitted" not in complete            # precondition: not truncated
    assert "Copy it verbatim" in complete
    assert "ONLY from the group they are shown under" in complete


def test_truncated_listing_keeps_the_path_instruction_too() -> None:
    partial = _fn_listing(_crate(n_modules=12, per_module=40, sig_len=140),
                          limit_chars=10_000)
    assert "omitted" in partial                 # precondition: truncated
    assert "Copy it verbatim" in partial


def test_instruction_names_the_upstream_recall_failure_mode() -> None:
    """The model needs to know WHY, or it treats the rule as boilerplate: this
    crate's module split is c2rust's, not the original header's."""
    text = _fn_listing(_crate(2, 4))
    assert "upstream" in text
    assert "c2rust" in text
