"""Resolve and validate the minimal `static` -> `const` keyword edit."""

from __future__ import annotations

from pathlib import Path

from perf_opt.hot_probe.static_facts import (
    scan_global_declarations,
    validate_const_promotion_safety,
)

from ..resolver import ResolutionRejected, resolve_project_path, sha256_bytes
from ..types import (
    ChangeSetStatus, ConcreteEdit, OperationResolution, PromoteStaticToConst,
    ValidationResult,
)


class PromoteStaticHandler:
    def resolve(self, operation: PromoteStaticToConst, crate: Path) -> OperationResolution:
        if not operation.target.file_hint:
            raise ValueError("static target has no file hint")
        path = resolve_project_path(crate, operation.target.file_hint)
        matches = [
            fact for fact in scan_global_declarations(crate)
            if fact.relative_path == operation.target.file_hint
            and fact.qualified_name == operation.target.qualified_name
            and fact.kind == "static"
        ]
        exact = [fact for fact in matches
                 if fact.declaration_hash == operation.target.declaration_hash]
        if not exact:
            raise ResolutionRejected(
                ChangeSetStatus.REJECTED_STALE,
                "stale static declaration target",
            )
        if len(exact) != 1:
            raise ResolutionRejected(
                ChangeSetStatus.REJECTED_CONFLICT,
                "conflicting static declaration targets",
            )
        fact = exact[0]
        data = path.read_bytes()
        edit_end = fact.mutability_end or fact.keyword_end
        edit = ConcreteEdit(
            edit_id=f"edit-{operation.operation_id}",
            operation_id=operation.operation_id,
            relative_path=fact.relative_path,
            start_byte=fact.keyword_start,
            end_byte=edit_end,
            before_hash=sha256_bytes(data),
            replacement_text="const",
        )
        return OperationResolution(operation.operation_id, (edit,), facts={
            "name": fact.name, "type_text": fact.type_text,
            "rhs_text": fact.rhs_text, "visibility": fact.visibility,
            "attrs": list(fact.attrs), "relative_path": fact.relative_path,
            "edit_start": fact.keyword_start, "edit_end": edit_end,
        })

    def pre_validate(self, operation, resolution, crate: Path) -> ValidationResult:
        facts = [fact for fact in scan_global_declarations(crate)
                 if fact.declaration_hash == operation.target.declaration_hash]
        if len(facts) != 1:
            return ValidationResult(False, "static_target_stale")
        safety = validate_const_promotion_safety(crate, facts[0])
        return ValidationResult(safety.ok, safety.code, safety.detail)

    def post_validate(self, operation, resolution, crate: Path) -> ValidationResult:
        expected = resolution.facts
        matches = [fact for fact in scan_global_declarations(crate)
                   if fact.relative_path == expected["relative_path"]
                   and fact.qualified_name == operation.target.qualified_name
                   and fact.kind == "const"]
        if len(matches) != 1:
            return ValidationResult(False, "promoted_const_missing")
        fact = matches[0]
        actual = {
            "name": fact.name, "type_text": fact.type_text,
            "rhs_text": fact.rhs_text, "visibility": fact.visibility,
            "attrs": list(fact.attrs), "relative_path": fact.relative_path,
        }
        expected_semantics = {
            key: expected[key]
            for key in (
                "name", "type_text", "rhs_text", "visibility", "attrs",
                "relative_path",
            )
        }
        if actual != expected_semantics:
            return ValidationResult(False, "promoted_const_changed_semantics")
        edit = resolution.edits[0]
        if (edit.start_byte, edit.end_byte, edit.replacement_text) != (
            expected["edit_start"], expected["edit_end"], "const"
        ):
            return ValidationResult(False, "promoted_const_nonminimal_edit")
                                                                    
                                                           
                                                             
                                                            
                                                                     
                                                             
                                                                 
                                     
        return ValidationResult(True, "promoted_static_to_const")
