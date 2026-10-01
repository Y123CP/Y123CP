"""No decision in this pipeline may depend on which project is running.

The pipeline is a method, not a set of per-project scripts: a rule that fires
only for the crate it was written against proves nothing about the next crate.
That is a standing requirement, and it has been violated in practice —

    # harness_gen/perf_workload.py, before this test existed
    if hot and not hot.startswith(("libxml2_raw_harness", "0x")) and "::" in hot:

which silently classified every OTHER project's hottest symbol wrong, for
weeks, while the code kept "working".

The judgement here is not "the file must not contain a project name" — naming
the crate a finding came from is exactly how evidence should be recorded, and
a prompt may show a worked example from some crate. The line is narrower and
mechanical:

    a project name may appear in TEXT (comments, docstrings, messages, prompts)
    but never in a PREDICATE.

So this test walks the AST of production code and looks only at comparisons
and at the string arguments of matching calls (`startswith`, `==`, `in`, …).
"""

from __future__ import annotations

import ast
import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
PROD_DIRS = ("harness_gen", "perf_opt/agent_perf_opt", "perf_opt/hot_probe",
             "utils")

# Crates that have been run through the pipeline. A name entering this list
# means the project was worked on — which is exactly when the temptation to
# special-case it appears.
PROJECT_NAMES = re.compile(
    r"libxml2|brotli|optipng|zopfli|lz4|miniz|lodepng|libcsv|bzip2|zlib|"
    r"http.?parser|libqrencode|libopenaptx|xxhash|libzahl|fzy|libpng|"
    r"\blil\b|\bpng\b|\bgif\b",
    re.I)

MATCHERS = {"startswith", "endswith", "match", "fullmatch", "search",
            "count", "find", "index", "split", "removeprefix", "removesuffix"}


def _python_files():
    for d in PROD_DIRS:
        yield from sorted((ROOT / d).rglob("*.py"))


def _predicate_strings(tree: ast.AST):
    """(lineno, value) for every string literal that steers a decision."""
    for node in ast.walk(tree):
        if isinstance(node, ast.Compare):
            operands = [node.left, *node.comparators]
        elif (isinstance(node, ast.Call)
              and isinstance(node.func, ast.Attribute)
              and node.func.attr in MATCHERS):
            operands = list(node.args)
        else:
            continue
        for operand in operands:
            for lit in ast.walk(operand):
                if isinstance(lit, ast.Constant) and isinstance(lit.value, str):
                    yield lit.lineno, lit.value


def test_no_project_name_steers_a_decision() -> None:
    offenders = []
    for path in _python_files():
        try:
            tree = ast.parse(path.read_text(encoding="utf-8"))
        except SyntaxError:                      # not ours to police
            continue
        for lineno, value in _predicate_strings(tree):
            if PROJECT_NAMES.search(value):
                offenders.append(
                    f"{path.relative_to(ROOT)}:{lineno} — {value!r}")
    assert not offenders, (
        "a project name decides control flow here; express the rule as a "
        "property of the input instead (read the crate name from the "
        "manifest, match on syntax, ask the inventory):\n  "
        + "\n  ".join(offenders))


def test_the_detector_catches_the_regression_it_was_written_for() -> None:
    """Guard the guard: a matcher that silently matches nothing is worse than
    no test, because it reads as a passing check."""
    tree = ast.parse(
        'if hot and not hot.startswith(("libxml2_raw_harness", "0x")):\n'
        '    pass\n')
    found = [v for _l, v in _predicate_strings(tree) if PROJECT_NAMES.search(v)]
    assert found == ["libxml2_raw_harness"]


@pytest.mark.parametrize("snippet", [
    'log("measured on libxml2: 44 type errors")',          # evidence in a message
    'HELP = "crate dir, e.g. dataset_trans/lodepng/0_raw"',  # help text
    '"""Observed on brotli: the 57% function."""',          # docstring
])
def test_naming_a_project_in_text_is_allowed(snippet) -> None:
    """Recording which crate a finding came from is how evidence works; the
    rule is about predicates, not about the alphabet."""
    tree = ast.parse(snippet)
    assert not [v for _l, v in _predicate_strings(tree)
                if PROJECT_NAMES.search(v)]
