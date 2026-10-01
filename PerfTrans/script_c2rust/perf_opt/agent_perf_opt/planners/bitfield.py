"""Deterministic planner for C9 — lowering `c2rust-bitfields` accessors.

The rewrite is fully determined by the `#[bitfield(...)]` declarations, so this
planner reads the current source, renders the replacement itself, and emits one
`RewriteBitfieldStruct` operation per lowerable struct.  No LLM is involved —
the same reason `II_const` has a deterministic planner: there is nothing to
decide, only something to compute.

Scope is `CRATE_GLOBAL`: the struct declaration is shared by the whole crate,
so the edit triggers a full rebuild even though every call site is untouched.
"""

from __future__ import annotations

import hashlib
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from perf_opt.agent_perf_opt.bitfield_lower import analyze_source
from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ProposedChangeSet,
    RewriteBitfieldStruct,
    TriggerContext,
)


@dataclass(frozen=True)
class BitfieldPlan:
    proposal: ProposedChangeSet | None
    abstain_reason: str | None = None
    struct_names: tuple[str, ...] = ()
    skipped: tuple[str, ...] = ()


def _field(hit: Any, name: str, default=None):
    return (
        hit.get(name, default) if isinstance(hit, dict)
        else getattr(hit, name, default)
    )


def plan_bitfield_lowering(
    *,
    crate: Path,
    hot_function: str,
    hits: list[Any],
    base_head: str,
    candidate_id: str,
) -> BitfieldPlan:
    """Plan the lowering of every bitfield struct the C9 hits point at.

    `hits` carry the *declaring* file in `extra["decl_file"]` (relative to the
    crate root) — the detector resolves it when it sees accessor calls, since
    the struct usually lives in a different file from the hot function.
    """
    decl_files: dict[str, list[str]] = {}
    for hit in hits:
        if _field(hit, "rule") != "C9":
            continue
        extra = _field(hit, "extra", {}) or {}
        decl_file = extra.get("decl_file")
        if not decl_file:
            return BitfieldPlan(None, "incomplete_target_evidence")
        hit_id = str(_field(hit, "id", _field(hit, "hit_id", "")))
        decl_files.setdefault(decl_file, [])
        if hit_id:
            decl_files[decl_file].append(hit_id)
    if not decl_files:
        return BitfieldPlan(None, "no_c9_targets")

    operations: list[RewriteBitfieldStruct] = []
    struct_names: list[str] = []
    all_skipped: list[str] = []
    hit_ids: list[str] = []

    for decl_file in sorted(decl_files):
        path = crate / decl_file
        try:
            source = path.read_bytes()
        except OSError as exc:
            return BitfieldPlan(None, f"decl_file_unreadable: {exc}")
        rewrites, skipped = analyze_source(source)
        all_skipped.extend(f"{decl_file}: {reason}" for reason in skipped)
        # Descending byte order so that, should a future change put two structs
        # in one operation batch, earlier edits never shift later offsets.
        for rewrite in sorted(rewrites, key=lambda r: r.start_byte, reverse=True):
            span = source[rewrite.start_byte:rewrite.end_byte]
            seed = f"{candidate_id}|{decl_file}|{rewrite.struct_name}"
            evidence = tuple(decl_files[decl_file])
            hit_ids.extend(evidence)
            struct_names.append(rewrite.struct_name)
            operations.append(RewriteBitfieldStruct(
                operation_id=(
                    "bitfield-" + hashlib.sha256(seed.encode()).hexdigest()[:16]
                ),
                rule_id="C9",
                evidence_hit_ids=evidence,
                relative_path=decl_file,
                start_byte=rewrite.start_byte,
                end_byte=rewrite.end_byte,
                expected_span_hash=hashlib.sha256(span).hexdigest(),
                replacement_text=rewrite.replacement_text,
                struct_name=rewrite.struct_name,
            ))

    if not operations:
        reason = "no_lowerable_bitfield_struct"
        if all_skipped:
            reason += " (" + "; ".join(all_skipped[:3]) + ")"
        return BitfieldPlan(None, reason, skipped=tuple(all_skipped))

    changeset_seed = candidate_id + "|" + "|".join(
        op.operation_id for op in operations
    )
    proposal = ProposedChangeSet(
        changeset_id=(
            "bitfield-"
            + hashlib.sha256(changeset_seed.encode()).hexdigest()[:16]
        ),
        base_head=base_head,
        trigger=TriggerContext(
            rule_id="C9",
            candidate_id=candidate_id,
            hot_function=hot_function,
            hit_ids=tuple(dict.fromkeys(hit_ids)),
            evidence_artifacts=(),
        ),
        operations=tuple(operations),
        impact_scope=ImpactScope.CRATE_GLOBAL,
    )
    return BitfieldPlan(
        proposal,
        struct_names=tuple(struct_names),
        skipped=tuple(all_skipped),
    )
