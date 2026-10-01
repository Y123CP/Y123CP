"""What reaches the ablation model is the function, its types, and "go faster".

The ablation arm exists to answer "what does the rule layer contribute".
Anything in its prompt that names an obstacle, ranks a spot, or quotes a
measurement is a diluted form of the very thing being withheld, and it
biases the answer toward "the rules contribute nothing".

Two channels have to stay shut, and neither is obvious from reading
`build_freeform_prompt` alone:

  * the EVIDENCE block — profile percentages, CPI, and the top LLVM
    optimization remarks. A remark like "loop not vectorized: value that
    could not be identified as reduction is used outside the loop" is the
    compiler naming the obstacle, which is most of what a card would say.

  * the RULE HITS — `driver.run_perf_opt` runs the class_I/II/III scans on
    both arms (they cost little next to the W2 measurements, and the full
    arm's would-have-skipped set is a useful column), so
    `3_perf_opt_freeform/` contains `fn_hits.json` and `class_*_hits.json`,
    and the `hotspots.json` it copies from the full arm carries
    `class_i_hits` per function. Present on disk, and none of it may be
    read.

The type-definition block is deliberately NOT covered here: a struct
definition is not a finding about how to go faster, it is a name the model
cannot otherwise see, and its absence costs compile round-trips rather than
optimization ideas. The full arm has it too.
"""

from __future__ import annotations

import ast
from pathlib import Path

import pytest

from perf_opt.agent_perf_opt import ablation_freeform as m

_SRC_PATH = Path(m.__file__)
_SRC = _SRC_PATH.read_text(encoding="utf-8")


def _code_only(src: str) -> str:
    """Source with docstrings and comments stripped.

    The module explains at length which channels it closes, so a plain
    substring search over the raw file matches its own prose. Only code
    counts.
    """
    tree = ast.parse(src)
    for node in ast.walk(tree):
        if isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef,
                             ast.AsyncFunctionDef)):
            body = node.body
            if (body and isinstance(body[0], ast.Expr)
                    and isinstance(body[0].value, ast.Constant)
                    and isinstance(body[0].value.value, str)):
                body[0].value.value = ""
    return ast.unparse(tree)


CODE = _code_only(_SRC)


# ─── the evidence channel ─────────────────────────────────────────────

def test_the_evidence_formatter_is_gone() -> None:
    assert not hasattr(m, "_format_evidence_freeform")


def test_the_prompt_builder_cannot_even_receive_evidence() -> None:
    """Not "it ignores the pack" — it has no parameter to pass one in."""
    import inspect
    params = inspect.signature(m.build_freeform_prompt).parameters
    assert set(params) == {"edit_target", "crate_dir"}


@pytest.mark.parametrize("leaked", [
    "llvm_opt_remarks",     # the compiler's own findings
    "_filter_top_remarks",  # the helper that ranked them
    "branch_miss_rate",
    "retired_instructions",
    ".cpi",
])
def test_no_evidence_field_is_read_anywhere_in_the_arm(leaked: str) -> None:
    assert leaked not in CODE, f"{leaked} is back in {_SRC_PATH.name}"


def test_self_pct_survives_only_as_a_log_line() -> None:
    """`hf.self_pct` is allowed — but only to say which fn is being tried.

    It must not reach the prompt. The guard is that the prompt builder no
    longer takes a HotFunction at all, so any use is confined to logging.
    """
    assert "hf.self_pct" in CODE                       # still logged
    src = m.build_freeform_prompt.__code__.co_names
    assert "self_pct" not in src


# ─── the rule-hit channel ─────────────────────────────────────────────

@pytest.mark.parametrize("artifact", [
    "fn_hits",
    "class_i_hits",
    "class_ii_hits",
    "class_iii_hits",
    "class_I_hits.json",
    "class_II_hits.json",
    "class_III_hits.json",
])
def test_the_hit_files_on_disk_are_never_read(artifact: str) -> None:
    """They exist in `3_perf_opt_freeform/`; the arm must ignore them."""
    assert artifact not in CODE, (
        f"{artifact} is read by {_SRC_PATH.name} — the ablation arm would be "
        f"consuming the rule layer it is supposed to be without")


def test_fn_type_triage_is_not_consulted() -> None:
    """`fn_type` is the rule layer's mode picker (job ④), copied into the
    shared hotspots.json. Reading it would restore that job."""
    assert "fn_type" not in CODE
    assert "is_complex_fn" not in CODE


def test_only_four_hot_function_fields_are_touched() -> None:
    """A field-level whitelist.

    `hotspots.json` is copied verbatim from the full arm, so every rule
    column travels with it. Naming the permitted fields makes a new read
    fail here rather than silently contaminate a run.
    """
    tree = ast.parse(_SRC)
    used = {
        node.attr
        for node in ast.walk(tree)
        if isinstance(node, ast.Attribute)
        and isinstance(node.value, ast.Name) and node.value.id == "hf"
    }
    assert used == {"name", "self_pct", "file", "extern_wrapper"}, used


# ─── what the model actually sees ─────────────────────────────────────

def test_the_instruction_asks_for_speed_and_nothing_more() -> None:
    """No technique, no direction — the arm's whole premise."""
    sys_p = m.SYSTEM_FREEFORM
    for banned in ("CStr::from_ptr", "bounds check", "vectoriz", "SIMD",
                   "card", "slice::from_raw_parts"):
        assert banned.lower() not in sys_p.lower(), banned
    assert "No further" in sys_p and "guidance is given" in sys_p
