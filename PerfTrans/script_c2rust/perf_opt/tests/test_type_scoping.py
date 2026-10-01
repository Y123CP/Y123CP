"""The caller must be told which type names are ambiguous.

c2rust copies every C struct and typedef into each module that mentions it,
and the copies are distinct Rust types the compiler will not unify. A function
signature cannot reveal this — c2rust writes signatures unqualified, because
inside a module no qualification is needed:

    // pngset.rs
    pub type png_const_structrp = *const png_struct;
    pub unsafe extern "C" fn png_set_IHDR(png_ptr: png_const_structrp, …)

23 other modules of the same crate carry that alias line BYTE-IDENTICALLY,
each resolving to their own `png_struct`. Nothing in the name, and nothing in
the definition text, hints that a choice exists.

Measured across the dataset, the number of such types predicts the run almost
perfectly — better than crate size does:

    libopenaptx     0 duplicated types   100% fn coverage    2 LLM calls
    miniz           3                     92%                6
    lz4            12                     42%                9
    optipng        77                     33%                9  (budget spent)
    libxml2       176                     27%               13  (budget spent)

Adding rules did not help: the model FOLLOWED the "cast across modules" rule
and still failed, writing `p as png_structrp` — casting through an alias that
is itself per-module. It was missing a fact, not a rule.
"""

from __future__ import annotations

from dataclasses import dataclass

import pytest

from harness_gen.prompts import HARNESS_EDITION, _type_scoping, codegen_user


@dataclass
class _Fn:
    name: str
    link_name: str
    signature: str
    module_path: str
    file: str = "src/x.rs"


class _Inv:
    def __init__(self, fns, duplicated, crate_name="proj_raw"):
        self.fns = fns
        self.crate_name = crate_name
        self.structs: list[str] = []
        self.lib_rs = ""
        self.duplicated_types = duplicated


def _inv(sig_types=("png_structp",), duplicated=None):
    args = ", ".join(f"a{i}: {t}" for i, t in enumerate(sig_types))
    return _Inv([_Fn("f", "f", f'pub unsafe extern "C" fn f({args})',
                     "src::pngset")],
                duplicated if duplicated is not None
                else {"png_structp": [f"src::m{i}" for i in range(23)]})


# ───────────────────────────────────── a clean crate must pay nothing

def test_a_crate_without_duplicates_gets_no_section() -> None:
    """Zero cost where there is zero risk — libopenaptx is this case, and it
    is also the crate that needed no repairs at all."""
    assert _type_scoping(_inv(duplicated={})) == ""


def test_a_type_that_never_appears_in_a_signature_is_not_listed() -> None:
    """It is duplicated 30 times and the caller can never touch it; listing it
    only dilutes the names that matter."""
    inv = _inv(sig_types=("png_structp",),
               duplicated={"png_structp": ["a", "b"],
                           "internal_only_t": ["a", "b", "c"]})
    out = _type_scoping(inv)
    assert "png_structp" in out
    assert "internal_only_t" not in out


# ─────────────────────────────────────────── the fact itself

def test_the_section_states_where_a_signature_type_lives() -> None:
    """The whole fix rests on this sentence: the listing is already grouped by
    module, so scoping needs no per-type qualification."""
    out = _type_scoping(_inv())
    assert "belongs to THAT function's own module" in out


def test_each_listed_name_carries_its_copy_count() -> None:
    out = _type_scoping(_inv())
    assert "png_structp (23)" in out


def test_names_are_ordered_worst_first(monkeypatch) -> None:
    inv = _inv(sig_types=("rare_t", "common_t"),
               duplicated={"rare_t": ["a", "b"],
                           "common_t": [f"m{i}" for i in range(9)]})
    out = _type_scoping(inv)
    assert out.index("common_t") < out.index("rare_t")


def test_alias_casting_is_called_out_as_useless() -> None:
    """The precise failure observed: the model obeyed the cast rule but cast
    through an alias, which is duplicated too."""
    out = _type_scoping(_inv())
    assert "as *mut _" in out
    assert "does not bridge" in out


def test_the_list_is_capped_and_says_so() -> None:
    """libxml2 has 176 of them; a wall of names is noise, not information."""
    many = {f"t{i}": [f"m{j}" for j in range(3)] for i in range(60)}
    inv = _inv(sig_types=tuple(many), duplicated=many)
    out = _type_scoping(inv)
    assert "more" in out
    assert len(out) < 2_000, "the section must stay small on the worst crate"


# ────────────────────────────────────── it reaches the model

def test_codegen_prompt_carries_the_section() -> None:
    text = codegen_user(_inv(), {"operations": [], "harness_name": "h"})
    assert "TYPE SCOPING" in text
    assert "png_structp (23)" in text


def test_codegen_prompt_states_the_edition() -> None:
    """A model that must guess reaches for the newest syntax it knows; one run
    emitted a 2024-only `unsafe extern "C" { … }` into a 2021 crate and spent
    a repair round on a fact the manifest had all along."""
    text = codegen_user(_inv(), {"operations": [], "harness_name": "h"})
    assert f"edition {HARNESS_EDITION}" in text


def test_the_manifest_and_the_prompt_cannot_disagree() -> None:
    """Single source of truth: the Cargo.toml template formats this same
    constant in, so the two can never drift apart."""
    from harness_gen.agent import CARGO_TOML_TEMPLATE
    assert 'edition = "{edition}"' in CARGO_TOML_TEMPLATE
    rendered = CARGO_TOML_TEMPLATE.format(
        crate_name="c", harness_name="h", edition=HARNESS_EDITION,
        crate_path="/tmp/c")
    assert f'edition = "{HARNESS_EDITION}"' in rendered
