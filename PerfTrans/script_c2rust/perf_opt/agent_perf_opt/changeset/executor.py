"""State-machine execution for typed, recoverable ChangeSets."""

from __future__ import annotations

import hashlib
import logging
from pathlib import Path
from typing import Any, Callable, Protocol

from .applier import AppliedFiles, ChangeSetApplier
from .audit import AuditWriter
from .registry import HandlerRegistry
from .resolver import ResolutionRejected, validate_edit_ranges
from .types import (
    AppliedChangeSet,
    ChangeSetStatus,
    GateResultRecord,
    OperationResolution,
    ProposedChangeSet,
    ResolvedChangeSet,
    ValidationResult,
)
from .validator import (
    validate_operation_edit_bijection,
    validate_region_file_isolation,
)


logger = logging.getLogger("perf_opt.changeset")

# A rejection is only actionable if the ledger says WHY. Compiler output is far
# too large to keep whole, and the informative part is at the front, so keep a
# bounded head excerpt.
_REJECT_EXCERPT_LINES = 12
_REJECT_EXCERPT_CHARS = 2000


def _failure_excerpt(gates: list[GateResultRecord]) -> dict[str, Any]:
    """Evidence for why a changeset was rejected, trimmed to ledger size.

    Without this the audit trail records the verdict and nothing else — a build
    rejection reads as `{"status": "rejected_build", "detail": {}}`, which is
    indistinguishable from any other build rejection and cannot be diagnosed
    without re-running the whole attempt. That is at its worst exactly where it
    costs most: the hottest function in a project is the likeliest to be large
    and rule-dense, so it is the likeliest to fail here and the most expensive
    failure to leave unexplained.
    """
    failed = next((g for g in reversed(gates) if not g.ok), None)
    if failed is None:
        return {}
    text = ""
    for key in ("stderr", "reason", "detail", "message"):
        value = failed.detail.get(key)
        if isinstance(value, str) and value.strip():
            text = value
            break
    if not text:
        return {"gate": failed.gate}
    lines = [ln for ln in text.splitlines() if ln.strip()]
    excerpt = "\n".join(lines[:_REJECT_EXCERPT_LINES])[:_REJECT_EXCERPT_CHARS]
    payload: dict[str, Any] = {"gate": failed.gate, "excerpt": excerpt}
    if len(lines) > _REJECT_EXCERPT_LINES:
        payload["truncated_lines"] = len(lines) - _REJECT_EXCERPT_LINES
    return payload


class StateBackend(Protocol):
    def head_sha(self, *, full: bool = True) -> str: ...
    def commit_success(self, paths, message: str) -> str: ...


class ChangeSetInfrastructureError(RuntimeError):
    def __init__(self, message: str, *, commit_sha: str | None = None) -> None:
        super().__init__(message)
        self.commit_sha = commit_sha


class ChangeSetRejected(RuntimeError):
    def __init__(self, status: ChangeSetStatus, detail: str) -> None:
        super().__init__(detail)
        self.status = status


GateCallback = Callable[..., GateResultRecord]
ParentPromoter = Callable[[Path, str], None]


class ChangeSetExecutor:
    def __init__(
        self,
        *,
        crate: Path,
        registry: HandlerRegistry,
        applier: ChangeSetApplier,
        state: StateBackend,
        audit: AuditWriter,
        candidate_binary: Path,
        build_gate: GateCallback,
        w1_gate: GateCallback,
        w2_gate: GateCallback,
        parent_promoter: ParentPromoter,
    ) -> None:
        self.crate = crate.resolve()
        self.registry = registry
        self.applier = applier
        self.state = state
        self.audit = audit
        self.candidate_binary = candidate_binary
        self.build_gate = build_gate
        self.w1_gate = w1_gate
        self.w2_gate = w2_gate
        self.parent_promoter = parent_promoter

    def recover_after_commit(self, changeset_id: str, commit_sha: str) -> None:
        """Finish parent promotion after a source commit already succeeded."""
        if not self.candidate_binary.is_file():
            raise ChangeSetInfrastructureError(
                f"recovery candidate binary is missing: {self.candidate_binary}",
                commit_sha=commit_sha,
            )
        candidate_hash = hashlib.sha256(self.candidate_binary.read_bytes()).hexdigest()
        try:
            self.audit.validate_recovery(
                changeset_id, commit_sha, candidate_hash
            )
        except ValueError as exc:
            raise ChangeSetInfrastructureError(
                str(exc), commit_sha=commit_sha
            ) from exc
        self.parent_promoter(self.candidate_binary, commit_sha)
        self.audit.mark_recovered_after_commit(changeset_id, commit_sha)

    def execute(self, proposal: ProposedChangeSet) -> AppliedChangeSet:
        resolved: ResolvedChangeSet | None = None
        applied: AppliedFiles | None = None
        validations: list[ValidationResult] = []
        gates: list[GateResultRecord] = []
        commit_sha: str | None = None
        self.audit.write_event(proposal.changeset_id, ChangeSetStatus.PLANNED)

        try:
            current_head = self.state.head_sha(full=True)
            if current_head != proposal.base_head:
                raise ChangeSetRejected(
                    ChangeSetStatus.REJECTED_STALE,
                    f"base HEAD {proposal.base_head} != current HEAD {current_head}",
                )

            resolutions: list[OperationResolution] = []
            for operation in proposal.operations:
                handler = self.registry.handler_for(operation)
                try:
                    resolutions.append(handler.resolve(operation, self.crate))
                except ResolutionRejected as rejected:
                    raise ChangeSetRejected(rejected.status, str(rejected)) from rejected
            resolved = ResolvedChangeSet(proposal, current_head, tuple(resolutions))
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.RESOLVED)

            bijection = validate_operation_edit_bijection(
                tuple(operation.operation_id for operation in proposal.operations),
                resolved.edits,
            )
            validations.append(bijection)
            if not bijection.ok:
                raise ChangeSetRejected(ChangeSetStatus.REJECTED_CONFLICT, bijection.detail)

            region_isolation = validate_region_file_isolation(self.crate, resolved)
            validations.append(region_isolation)
            if not region_isolation.ok:
                raise ChangeSetRejected(
                    ChangeSetStatus.REJECTED_CONFLICT,
                    region_isolation.as_reason(),
                )

            ranges = validate_edit_ranges(self.crate, resolved.edits)
            validations.append(ranges)
            if not ranges.ok:
                status = (
                    ChangeSetStatus.REJECTED_STALE
                    if ranges.code == "stale_before_hash"
                    else ChangeSetStatus.REJECTED_CONFLICT
                )
                raise ChangeSetRejected(status, ranges.detail)

            for operation, resolution in zip(proposal.operations, resolved.resolutions):
                handler = self.registry.handler_for(operation)
                validation = handler.pre_validate(operation, resolution, self.crate)
                validations.append(validation)
                if not validation.ok:
                    raise ChangeSetRejected(
                        ChangeSetStatus.ABSTAINED_UNPROVEN, validation.as_reason()
                    )
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.PRE_VALIDATED)

            applied = self.applier.apply(proposal.changeset_id, resolved.edits)
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.APPLIED)

            build = self.build_gate()
            gates.append(build)
            if not build.ok:
                return self._reject(
                    resolved,
                    applied,
                    ChangeSetStatus.REJECTED_BUILD,
                    validations,
                    gates,
                )
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.BUILD_PASSED)

            for operation, resolution in zip(proposal.operations, resolved.resolutions):
                handler = self.registry.handler_for(operation)
                validation = handler.post_validate(operation, resolution, self.crate)
                validations.append(validation)
                if not validation.ok:
                    return self._reject(
                        resolved,
                        applied,
                        ChangeSetStatus.REJECTED_POST_VALIDATION,
                        validations,
                        gates,
                    )
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.POST_VALIDATED)

            w1 = self.w1_gate()
            gates.append(w1)
            if not w1.ok:
                return self._reject(
                    resolved,
                    applied,
                    ChangeSetStatus.REJECTED_W1,
                    validations,
                    gates,
                )
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.W1_PASSED)

            w2 = self.w2_gate(proposal.impact_scope)
            gates.append(w2)
            if not w2.ok:
                # Already a reason CODE, not the gate's own string: the w2
                # callback in agent.py collapses the verdict to one of four
                # values before it reaches here, so that is where a new verdict
                # reason has to be classified. `insufficient_net_gain` is
                # mapped to `no_gain` there; unmapped, it would land in the
                # regress bucket by default.
                reason = str(w2.detail.get("reason", ""))
                if reason == "no_gain":
                    status = ChangeSetStatus.REJECTED_W2_NO_GAIN
                elif reason == "unmeasurable":
                    status = ChangeSetStatus.REJECTED_UNMEASURABLE
                else:
                    status = ChangeSetStatus.REJECTED_W2_REGRESS
                return self._reject(
                    resolved, applied, status, validations, gates
                )
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.W2_PASSED)

            touched = [self.crate / relative for relative in applied.touched_paths]
            rules = ",".join(operation.rule_id for operation in proposal.operations)
            commit_sha = self.state.commit_success(
                touched,
                f"agent changeset {proposal.changeset_id}: {rules}",
            )

            # The Git commit is the source-of-truth boundary. Once it succeeds,
            # backups must not restore the now-committed source.
            self.applier.finalize(applied)
            candidate_hash = hashlib.sha256(
                self.candidate_binary.read_bytes()
            ).hexdigest()
            self.audit.write_event(
                proposal.changeset_id,
                ChangeSetStatus.COMMIT_SUCCEEDED,
                {
                    "commit_sha": commit_sha,
                    "candidate_binary_sha256": candidate_hash,
                },
            )
            try:
                self.parent_promoter(self.candidate_binary, commit_sha)
            except Exception as exc:
                failed = AppliedChangeSet(
                    resolved=resolved,
                    terminal_status=ChangeSetStatus.FAILED_INTERNAL,
                    validations=tuple(validations),
                    gates=tuple(gates),
                    commit_sha=commit_sha,
                    rollback_verified=False,
                )
                self.audit.write_result(failed)
                raise ChangeSetInfrastructureError(
                    f"parent promotion failed after commit {commit_sha}: {exc}",
                    commit_sha=commit_sha,
                ) from exc

            result = AppliedChangeSet(
                resolved=resolved,
                terminal_status=ChangeSetStatus.COMMITTED,
                validations=tuple(validations),
                gates=tuple(gates),
                commit_sha=commit_sha,
                rollback_verified=False,
            )
            self.audit.write_event(proposal.changeset_id, ChangeSetStatus.COMMITTED)
            self.audit.write_result(result)
            return result

        except ChangeSetRejected as rejected:
            rollback_verified = False
            if applied is not None:
                self.applier.restore(applied)
                rollback_verified = True
            if resolved is None:
                # Preserve proposal identity in audit even when resolution did
                # not complete; there are deliberately no concrete edits.
                resolved = ResolvedChangeSet(proposal, self.state.head_sha(full=True), ())
            result = AppliedChangeSet(
                resolved=resolved,
                terminal_status=rejected.status,
                validations=tuple(validations),
                gates=tuple(gates),
                rollback_verified=rollback_verified,
            )
            self.audit.write_event(
                proposal.changeset_id, rejected.status, {"reason": str(rejected)}
            )
            self.audit.write_result(result)
            return result
        except ChangeSetInfrastructureError:
            raise
        except Exception as exc:
            rollback_verified = False
            if commit_sha is None and applied is not None:
                self.applier.restore(applied)
                rollback_verified = True
            if resolved is None:
                resolved = ResolvedChangeSet(proposal, self.state.head_sha(full=True), ())
            result = AppliedChangeSet(
                resolved=resolved,
                terminal_status=ChangeSetStatus.FAILED_INTERNAL,
                validations=tuple(validations),
                gates=tuple(gates),
                commit_sha=commit_sha,
                rollback_verified=rollback_verified,
            )
            try:
                self.audit.write_event(
                    proposal.changeset_id,
                    ChangeSetStatus.FAILED_INTERNAL,
                    {"reason": str(exc)},
                )
                self.audit.write_result(result)
            except Exception:
                # Preserve the original infrastructure failure.  Audit itself
                # may be the failed subsystem, so a second write is best-effort.
                pass
            if commit_sha is not None:
                raise ChangeSetInfrastructureError(
                    f"infrastructure failure after commit {commit_sha}: {exc}",
                    commit_sha=commit_sha,
                ) from exc
            return result

    def _reject(
        self,
        resolved: ResolvedChangeSet,
        applied: AppliedFiles,
        status: ChangeSetStatus,
        validations: list[ValidationResult],
        gates: list[GateResultRecord],
    ) -> AppliedChangeSet:
        self.applier.restore(applied)
        result = AppliedChangeSet(
            resolved=resolved,
            terminal_status=status,
            validations=tuple(validations),
            gates=tuple(gates),
            rollback_verified=True,
        )
        detail = _failure_excerpt(gates)
        changeset_id = resolved.proposal.changeset_id
        self.audit.write_event(changeset_id, status, detail or None)
        if detail.get("excerpt"):
            logger.warning(
                "[changeset] %s %s at gate=%s: %s",
                changeset_id, status.value, detail.get("gate"),
                detail["excerpt"].splitlines()[0][:200],
            )
        self.audit.write_result(result)
        return result
