"""Compatibility handler for existing whole-function LLM rewrites."""

from __future__ import annotations

import hashlib
from pathlib import Path

from perf_opt.agent_perf_opt.rewrite_applier import (
    EditTarget,
    find_index_derived_slice_views,
    locate_fn_span_with_attrs,
)

from ..resolver import ResolutionRejected, resolve_project_path, sha256_bytes
from ..types import (
    ConcreteEdit,
    ChangeSetStatus,
    OperationResolution,
    ReplaceFunctionBody,
    ValidationResult,
)


_RUST_PARSER = None


def _parse_rust(source: str):
    """Parse a Rust snippet. Lazily built, reused across calls.

    Constructed here rather than via `utils.tree_sitter_runtime` because that
    module also imports `tree_sitter_c`, which this environment does not
    always have — pulling in a C grammar to validate Rust would make the
    changeset path fail for an unrelated missing dependency.
    """
    global _RUST_PARSER
    if _RUST_PARSER is None:
        import tree_sitter
        import tree_sitter_rust
        parser = tree_sitter.Parser()
        parser.language = tree_sitter.Language(tree_sitter_rust.language())
        _RUST_PARSER = parser
    return _RUST_PARSER.parse(source.encode("utf-8"))


_TS_AMBIGUOUS_TOKENS = frozenset({"<", ">", "<=", ">="})


def _only_cast_comparison_ambiguity(root, source: str) -> bool:
    """True when every parse error is tree-sitter's cast-vs-generic ambiguity.

    `x as SomeName < y` is ambiguous to a parser that has not resolved names:
    `SomeName<...>` could open a generic argument list. tree-sitter guesses
    generic and reports the `<` as an error; rustc accepts the expression
    (verified: 0 errors on the exact line this was first seen on).

    Rejecting on it would be catastrophic here rather than merely unlucky,
    because c2rust names every C scalar with an alias — `c_int`, `c_double`,
    `c_uint` — so `as c_double <= m` is ordinary translated code, not a corner
    case. Two rewrites of a compression crate were discarded as "not
    syntactically valid Rust" while being perfectly valid.

    The check stays for real syntax errors: those produce ERROR nodes spanning
    actual text (a missing `;`, an unclosed brace), not a bare comparison
    operator. And the build gate remains the final authority — this only
    decides whether we spend a compile to find out.
    """
    data = source.encode("utf-8")
    findings: list[str] = []
    stack = [root]
    while stack:
        node = stack.pop()
        if node.type == "ERROR":
            findings.append(
                data[node.start_byte:node.end_byte].decode("utf-8", "replace").strip())
            continue
        if node.is_missing:
            # `as T < y` parses as an unclosed generic argument list, so the
            # parser invents the closing `>`. `as T <= y` instead yields a
            # stray `<` ERROR. Both are the same ambiguity wearing different
            # hats, and both must be tolerated.
            findings.append(node.type)
        stack.extend(node.children)
    if not findings:
        return False
    return all(f in _TS_AMBIGUOUS_TOKENS for f in findings)


def validate_single_function_source(source: str, expected_name: str) -> ValidationResult:
    """Accept exactly one TOP-LEVEL function definition named `expected_name`.

    "One function" is a statement about the item tree, and only a parser can
    check it. Counting `fn` tokens in text answers a different question and
    gets this one wrong: a `fn` also appears in a nested helper, an `impl`
    method, a closure's `fn` bound, a function-pointer type `fn(u8) -> bool`,
    and inside an `extern "C" { … }` declaration block.

    Measured cost of getting it wrong: on optipng the model returned a correct
    rewrite of `opng_strparse_rangeset_to_bitset` — the crate's hottest
    function at 99.75% self-time — carrying one `#[inline(always)] fn
    is_space_byte(..)` helper INSIDE the body. Nesting a helper is ordinary,
    good Rust. The text counter saw two `fn`s, abstained, and the function was
    never attempted again: the single largest optimization target in the run,
    lost to a miscount.

    So parse, and ask the tree the three questions directly. This also retires
    the hand-rolled brace matcher that used to find the body's end, which had
    no handling for raw strings (`r#"…"#`) or lifetimes.
    """
    tree = _parse_rust(source)
    root = tree.root_node
    if root.has_error and not _only_cast_comparison_ambiguity(root, source):
        return ValidationResult(
            False,
            "replacement_parse_error",
            "replacement is not syntactically valid Rust",
        )

    functions = [c for c in root.named_children if c.type == "function_item"]
    if len(functions) != 1:
        return ValidationResult(
            False,
            "replacement_function_count",
            "replacement_must_contain_exactly_one_function",
        )

    # Slice the BYTES: tree-sitter offsets are byte offsets, and indexing the
    # str with them silently shifts as soon as the snippet holds a multi-byte
    # character. It reliably does — the rewrite header names the rules it
    # applied, and the rule ids themselves are `III④` / `III③`.
    name_node = functions[0].child_by_field_name("name")
    data = source.encode("utf-8")
    name = (data[name_node.start_byte:name_node.end_byte].decode("utf-8")
            if name_node else "")
    if name != expected_name:
        return ValidationResult(
            False,
            "replacement_function_name",
            f"expected function {expected_name}, got {name}",
        )

    # Nothing may follow the function. Attributes and comments are part of the
    # item or trivia; an `extern "C" { … }` block a rewrite legitimately adds
    # (rule C7) parses as its own top-level item, so allow it explicitly rather
    # than by blanking text.
    allowed = {"function_item", "attribute_item", "line_comment",
               "block_comment", "foreign_mod_item"}
    stray = [c.type for c in root.named_children if c.type not in allowed]
    if stray:
        return ValidationResult(
            False,
            "replacement_extra_source",
            f"replacement contains source outside the target function: {stray}",
        )

    # A slice view whose length is invented from the index that reads it
    # bounds nothing and adds a check per access (see
    # `find_index_derived_slice_views`). Refused on the reply, before a build;
    # this path serves every whole-function rewrite, freeform included.
    derived = find_index_derived_slice_views(source)
    if derived:
        return ValidationResult(
            False, "index_derived_slice_view", " | ".join(derived)[:2000])
    return ValidationResult(True, "replacement_function_source")


def _function_definition_count(path: Path, fn_name: str) -> int:
    """Count named Rust function items without relying on text matches."""
    from tree_sitter import Language, Parser
    import tree_sitter_rust

    source = path.read_bytes()
    parser = Parser(Language(tree_sitter_rust.language()))
    tree = parser.parse(source)
    count = 0
    stack = [tree.root_node]
    while stack:
        node = stack.pop()
        if node.type == "function_item":
            name_node = node.child_by_field_name("name")
            if (
                name_node is not None
                and name_node.text.decode("utf-8", errors="replace") == fn_name
            ):
                count += 1
        stack.extend(reversed(node.children))
    return count


class ReplaceFunctionHandler:
    def __init__(self, edit_target: EditTarget) -> None:
        self.edit_target = edit_target

    def resolve(
        self, operation: ReplaceFunctionBody, crate: Path
    ) -> OperationResolution:
        target_path = self.edit_target.file.resolve()
        relative_path = str(target_path.relative_to(crate.resolve()))
        path = resolve_project_path(crate, relative_path)
        data = path.read_bytes()
        start, end = self.edit_target.span
        if not (0 <= start < end <= len(data)):
            raise ValueError("function target span is out of bounds")
        target_hash = hashlib.sha256(data[start:end]).hexdigest()
        if target_hash != operation.target.declaration_hash:
            raise ResolutionRejected(
                ChangeSetStatus.REJECTED_STALE,
                f"stale function target: expected {operation.target.declaration_hash}, "
                f"got {target_hash}"
            )
        edit = ConcreteEdit(
            edit_id=f"edit-{operation.operation_id}",
            operation_id=operation.operation_id,
            relative_path=relative_path,
            start_byte=start,
            end_byte=end,
            before_hash=sha256_bytes(data),
            replacement_text=operation.replacement_function_source,
        )
        return OperationResolution(
            operation_id=operation.operation_id,
            edits=(edit,),
            facts={
                "fn_name": self.edit_target.fn_name,
                "target_span_hash": target_hash,
            },
        )

    def pre_validate(
        self,
        operation: ReplaceFunctionBody,
        resolution: OperationResolution,
        crate: Path,
    ) -> ValidationResult:
        if operation.target.qualified_name.rsplit("::", 1)[-1] != self.edit_target.fn_name:
            return ValidationResult(
                False, "target_function_name", "operation target does not match EditTarget"
            )
        if len(resolution.edits) != 1:
            return ValidationResult(False, "replace_function_edit_count", "expected one edit")
        return validate_single_function_source(
            operation.replacement_function_source, self.edit_target.fn_name
        )

    def post_validate(
        self,
        operation: ReplaceFunctionBody,
        resolution: OperationResolution,
        crate: Path,
    ) -> ValidationResult:
        validation = self.pre_validate(operation, resolution, crate)
        if not validation.ok:
            return validation
        edit = resolution.edits[0]
        path = resolve_project_path(crate, edit.relative_path)
        data = path.read_bytes()
        replacement = operation.replacement_function_source.encode("utf-8")
        definition_count = _function_definition_count(path, self.edit_target.fn_name)
        if definition_count != 1:
            return ValidationResult(
                False,
                "replacement_function_ambiguous",
                f"expected one {self.edit_target.fn_name} definition, "
                f"found {definition_count}",
            )
        hint_line = data[: edit.start_byte].count(b"\n") + 1
        relocated = locate_fn_span_with_attrs(
            path,
            self.edit_target.fn_name,
            hint_line_range=(hint_line, hint_line),
        )
        expected_span = (edit.start_byte, edit.start_byte + len(replacement))
        if relocated != expected_span:
            return ValidationResult(
                False,
                "replacement_function_span",
                f"expected relocated span {expected_span}, got {relocated}",
            )
        actual = data[edit.start_byte : edit.start_byte + len(replacement)]
        if actual != replacement:
            return ValidationResult(
                False,
                "replacement_post_image",
                "target bytes do not equal the planned replacement",
            )
        from perf_opt.agent_perf_opt.rewrite_applier import (
            describe_added_bounds_checks,
    find_added_bounds_checks_in_crate,
            find_introduced_scans_in_crate,
        )

        # A rewrite must not buy safety the original code never paid for.
        # Checked HERE, in post-validation, because this is the first moment
        # the applied source exists on disk: the predicate reads the working
        # tree's diff, and every earlier hook runs before `applier.apply`.
        #
        # Measured on one crate's hottest inner loop: two runs produced
        # rewrites of the SAME lines under the SAME rules, differing only in
        # this one expression. Same start commit, same session, CV < 0.1%:
        # `get_unchecked` gave -8.44%; `slice[i]` gave +0.01%; swapping just
        # that expression back recovered -7.55%.
        #
        # It cannot be left to the gates. W2 reads +0.01% as "no regression"
        # and commits it, and the committed region then blocks any better
        # rewrite of those lines for the rest of the run.
        added = find_added_bounds_checks_in_crate(crate)
        if added:
            return ValidationResult(
                False,
                "added_bounds_check_in_loop",
                describe_added_bounds_checks(added),
            )

        # A finding, not a verdict. An introduced length scan is a real cost —
        # measured +7.818% on one crate's `precompute_bonus`, where the
        # original's own loop already walked to the NUL — but it is not always
        # a net loss: replayed over 128 committed rewrites, four had one and
        # every one of them had measured a win, because what the rewrite did
        # with the resulting view paid for the extra traversal. Rejecting here
        # would have blocked those four. So record it and let W2 judge; if W2
        # rejects, `agent` has a diagnosis to hand back instead of a stopwatch
        # reading. Candidates that pass W2 never reach that path, so this
        # cannot cost a winner.
        scans = find_introduced_scans_in_crate(crate)
        if scans:
            return ValidationResult(
                True,
                "introduced_scan",
                "rewrite builds a view whose length must be scanned for, "
                "which the original did not do: " + ", ".join(scans[:3]),
            )
        return ValidationResult(True, "replace_function_post")
