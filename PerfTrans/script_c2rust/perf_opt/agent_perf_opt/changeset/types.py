"""Immutable data exchanged by planners, handlers, and the executor."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from pathlib import PurePosixPath, PureWindowsPath
import re
from typing import Any


_SHA256_RE = re.compile(r"[0-9a-f]{64}")


class ImpactScope(str, Enum):
    LOCAL_FUNCTION = "local_function"
    CALLERS = "callers"
    CRATE_GLOBAL = "crate_global"


class SymbolKind(str, Enum):
    FUNCTION = "function"
    STATIC = "static"


class ChangeSetStatus(str, Enum):
    DETECTED = "detected"
    PLANNED = "planned"
    RESOLVED = "resolved"
    PRE_VALIDATED = "pre_validated"
    APPLIED = "applied"
    BUILD_PASSED = "build_passed"
    POST_VALIDATED = "post_validated"
    W1_PASSED = "w1_passed"
    W2_PASSED = "w2_passed"
    COMMIT_SUCCEEDED = "commit_succeeded"
    COMMITTED = "committed"
    RECOVERED_AFTER_COMMIT = "recovered_after_commit"

    ABSTAINED_UNPROVEN = "abstained_unproven"
    REJECTED_STALE = "rejected_stale"
    REJECTED_CONFLICT = "rejected_conflict"
    REJECTED_BUILD = "rejected_build"
    REJECTED_POST_VALIDATION = "rejected_post_validation"
    REJECTED_W1 = "rejected_w1"
    REJECTED_W2_NO_GAIN = "rejected_w2_no_gain"
    REJECTED_W2_REGRESS = "rejected_w2_regress"
    REJECTED_UNMEASURABLE = "rejected_unmeasurable"
    FAILED_INTERNAL = "failed_internal"


@dataclass(frozen=True)
class SymbolRef:
    qualified_name: str
    symbol_kind: SymbolKind
    file_hint: str | None
    declaration_hash: str


class RegionKind(str, Enum):
    EXPRESSION = "expression"
    STATEMENT = "statement"
    LOOP = "loop"
    MATCH_ARM = "match_arm"
    BLOCK = "block"
    NODE_SEQUENCE = "node_sequence"


@dataclass(frozen=True)
class RegionRef:
    target_function: SymbolRef
    relative_path: str
    region_kind: RegionKind
    parent_kind: str
    start_byte: int
    end_byte: int
    expected_region_hash: str
    anchor_hit_ids: tuple[str, ...]
    anchor_lines: tuple[int, ...]

    def __post_init__(self) -> None:
        if not isinstance(self.target_function, SymbolRef):
            raise TypeError("target_function must be a SymbolRef")
        if self.target_function.symbol_kind is not SymbolKind.FUNCTION:
            raise ValueError("target_function symbol kind must be FUNCTION")
        _validate_sha256(
            self.target_function.declaration_hash,
            "target_function declaration_hash",
        )

        _validate_relative_path(self.relative_path)
        if not isinstance(self.region_kind, RegionKind):
            raise TypeError("region_kind must be a RegionKind")
        if not isinstance(self.parent_kind, str):
            raise TypeError("parent_kind must be a string")
        if not self.parent_kind.strip():
            raise ValueError("parent_kind must be a non-empty string")

        if not _is_plain_int(self.start_byte) or not _is_plain_int(self.end_byte):
            raise TypeError("byte offsets must be integers")
        if self.start_byte < 0 or self.start_byte >= self.end_byte:
            raise ValueError("byte range must satisfy 0 <= start_byte < end_byte")
        _validate_sha256(self.expected_region_hash, "expected_region_hash")

        if not isinstance(self.anchor_hit_ids, tuple) or not isinstance(
            self.anchor_lines, tuple
        ):
            raise TypeError("anchor_hit_ids and anchor_lines must be tuples")
        if not self.anchor_hit_ids:
            raise ValueError("anchors must not be empty")
        if len(self.anchor_hit_ids) != len(self.anchor_lines):
            raise ValueError("anchor_hit_ids and anchor_lines must have equal length")
        for hit_id in self.anchor_hit_ids:
            _validate_sha256(hit_id, "anchor_hit_ids")
        if len(set(self.anchor_hit_ids)) != len(self.anchor_hit_ids):
            raise ValueError("anchor_hit_ids must not contain duplicates")
        if any(not _is_plain_int(line) for line in self.anchor_lines):
            raise TypeError("anchor_lines must contain integers")
        if any(line <= 0 for line in self.anchor_lines):
            raise ValueError("anchor_lines must contain positive integers")
        if tuple(sorted(self.anchor_lines)) != self.anchor_lines:
            raise ValueError("anchor_lines must be in stable ascending order")


@dataclass(frozen=True)
class ChangeOperation:
    operation_id: str
    rule_id: str
    evidence_hit_ids: tuple[str, ...]


@dataclass(frozen=True)
class PromoteStaticToConst(ChangeOperation):
    target: SymbolRef


@dataclass(frozen=True)
class ReplaceFunctionBody(ChangeOperation):
    target: SymbolRef
    replacement_function_source: str


@dataclass(frozen=True)
class ReplaceSourceRegion(ChangeOperation):
    region: RegionRef
    replacement_region_source: str

    def __post_init__(self) -> None:
        if not isinstance(self.region, RegionRef):
            raise TypeError("region must be a RegionRef")
        if not isinstance(self.replacement_region_source, str):
            raise TypeError("replacement_region_source must be a string")
        if self.evidence_hit_ids != self.region.anchor_hit_ids:
            raise ValueError(
                "evidence_hit_ids must exactly match region.anchor_hit_ids"
            )


@dataclass(frozen=True)
class ReplaceCallSite(ChangeOperation):
    ""                                         

                                                   
                                                        
                                                        
                                                             
                         
       
    relative_path: str
    start_byte: int
    end_byte: int
    expected_span_hash: str
    replacement_text: str

    def __post_init__(self) -> None:
        if not isinstance(self.replacement_text, str):
            raise TypeError("replacement_text must be a string")
        if not (0 <= self.start_byte < self.end_byte):
            raise ValueError("byte range must satisfy 0 <= start_byte < end_byte")


@dataclass(frozen=True)
class RewriteBitfieldStruct(ChangeOperation):
    ""                                                          

                                                                   
                                                      
                                                          
                                                    
                   
       

    relative_path: str
    start_byte: int
    end_byte: int
    expected_span_hash: str
    replacement_text: str
    struct_name: str

    def __post_init__(self) -> None:
        if not isinstance(self.replacement_text, str):
            raise TypeError("replacement_text must be a string")
        if not self.replacement_text:
            raise ValueError("replacement_text must not be empty")
        if not (0 <= self.start_byte < self.end_byte):
            raise ValueError("byte range must satisfy 0 <= start_byte < end_byte")


@dataclass(frozen=True)
class SetFunctionInlineAttr(ChangeOperation):
    """Set the `#[inline…]` attribute of one function definition (II_inl / II_iso).

    Resolved by NAME at resolve time (nearest `line_hint` when a name has
    several definitions), not by planned byte offsets: earlier commits in the
    same run shift offsets, and the attribute is valid wherever the item sits.
    An existing `#[inline…]` on the item is replaced; otherwise one is inserted
    directly above it.
    """

    ALLOWED = ("inline", "inline(always)", "inline(never)")

    relative_path: str
    fn_name: str
    line_hint: int
    attribute: str

    def __post_init__(self) -> None:
        _validate_relative_path(self.relative_path)
        if not isinstance(self.fn_name, str) or not self.fn_name.strip():
            raise ValueError("fn_name must be a non-empty string")
        if not _is_plain_int(self.line_hint) or self.line_hint < 0:
            raise ValueError("line_hint must be a non-negative integer")
        if self.attribute not in self.ALLOWED:
            raise ValueError(f"attribute must be one of {self.ALLOWED}")


@dataclass(frozen=True)
class TriggerContext:
    rule_id: str
    candidate_id: str
    hot_function: str
    hit_ids: tuple[str, ...]
    evidence_artifacts: tuple[str, ...]


@dataclass(frozen=True)
class ProposedChangeSet:
    changeset_id: str
    base_head: str
    trigger: TriggerContext
    operations: tuple[ChangeOperation, ...]
    impact_scope: ImpactScope

    def __post_init__(self) -> None:
        operation_ids = [operation.operation_id for operation in self.operations]
        if not operation_ids:
            raise ValueError("changeset requires at least one operation")
        if len(operation_ids) != len(set(operation_ids)):
            raise ValueError("duplicate operation_id")


@dataclass(frozen=True)
class ConcreteEdit:
    edit_id: str
    operation_id: str
    relative_path: str
    start_byte: int
    end_byte: int
    before_hash: str
    replacement_text: str


@dataclass(frozen=True)
class OperationResolution:
    operation_id: str
    edits: tuple[ConcreteEdit, ...]
    facts: dict[str, Any] = field(default_factory=dict)


@dataclass(frozen=True)
class ResolvedChangeSet:
    proposal: ProposedChangeSet
    resolved_head: str
    resolutions: tuple[OperationResolution, ...]

    @property
    def edits(self) -> tuple[ConcreteEdit, ...]:
        return tuple(
            edit
            for resolution in self.resolutions
            for edit in resolution.edits
        )


@dataclass(frozen=True)
class ValidationResult:
    ok: bool
    code: str
    detail: str = ""

    def as_reason(self) -> str:
        """The failure as a reason string that still carries its CODE.

        Callers used to write `result.detail or result.code`, which drops the
        code the moment a detail exists — and a detail almost always exists,
        because that is the human-readable half. What reaches the log is then
        free text that nothing downstream can classify, so a repairable
        envelope failure becomes indistinguishable from the model deciding
        the rewrite should not happen. Both end up filed as
        `planner_abstained`; only one of them is a judgment.

        Measured cost of the confusion: `replacement_function_count` IS listed
        as contract-repairable, but its detail masked the code, so the repair
        turn it was entitled to never fired and the rewrite was discarded
        unjudged.

        Format matches what `_reason_code` already splits on: ``code: detail``.
        """
        return f"{self.code}: {self.detail}" if self.detail else self.code


@dataclass(frozen=True)
class GateResultRecord:
    gate: str
    ok: bool
    detail: dict[str, Any] = field(default_factory=dict)


@dataclass(frozen=True)
class AppliedChangeSet:
    resolved: ResolvedChangeSet | None
    terminal_status: ChangeSetStatus
    validations: tuple[ValidationResult, ...] = ()
    gates: tuple[GateResultRecord, ...] = ()
    commit_sha: str | None = None
    rollback_verified: bool = False


def _validate_relative_path(relative_path: str) -> None:
    if not isinstance(relative_path, str):
        raise TypeError("relative_path must be a string")
    if not relative_path.strip() or "\x00" in relative_path or "\\" in relative_path:
        raise ValueError("relative_path must be a non-empty safe relative path")
    path = PurePosixPath(relative_path)
    windows_path = PureWindowsPath(relative_path)
    if (
        path.is_absolute()
        or bool(windows_path.drive)
        or ".." in path.parts
        or str(path) != relative_path
    ):
        raise ValueError("relative_path must be a normalized safe relative path")


def _validate_sha256(value: object, field_name: str) -> None:
    if not isinstance(value, str):
        raise TypeError(f"{field_name} must be a string")
    if _SHA256_RE.fullmatch(value) is None:
        raise ValueError(f"{field_name} must be lowercase SHA-256 hex")


def _is_plain_int(value: object) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)
