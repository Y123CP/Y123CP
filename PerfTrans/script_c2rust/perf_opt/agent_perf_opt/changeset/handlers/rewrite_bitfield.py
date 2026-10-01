""                                                                   

                                                                           
                                              

                                           
                                                                   
                
                                           
                                                      
                                   

                                              
                                                      
   

from __future__ import annotations

from pathlib import Path

from ..resolver import ResolutionRejected, resolve_project_path, sha256_bytes
from ..types import (
    ChangeSetStatus, ConcreteEdit, OperationResolution, RewriteBitfieldStruct,
    ValidationResult,
)


class RewriteBitfieldStructHandler:
    def resolve(
        self, operation: RewriteBitfieldStruct, crate: Path
    ) -> OperationResolution:
        path = resolve_project_path(crate, operation.relative_path)
        data = path.read_bytes()
        if not (0 <= operation.start_byte < operation.end_byte <= len(data)):
            raise ResolutionRejected(
                ChangeSetStatus.REJECTED_STALE,
                "bitfield struct byte range out of file bounds",
            )
        span = data[operation.start_byte:operation.end_byte]
        if sha256_bytes(span) != operation.expected_span_hash:
            raise ResolutionRejected(
                ChangeSetStatus.REJECTED_STALE,
                "bitfield struct declaration changed since planning",
            )
        edit = ConcreteEdit(
            edit_id=f"edit-{operation.operation_id}",
            operation_id=operation.operation_id,
            relative_path=operation.relative_path,
            start_byte=operation.start_byte,
            end_byte=operation.end_byte,
            before_hash=sha256_bytes(data),
            replacement_text=operation.replacement_text,
        )
        return OperationResolution(operation.operation_id, (edit,), facts={
            "relative_path": operation.relative_path,
            "struct_name": operation.struct_name,
            "original_span": span.decode("utf-8", errors="replace"),
            "replacement_text": operation.replacement_text,
        })

    def pre_validate(
        self, operation: RewriteBitfieldStruct, resolution, crate: Path
    ) -> ValidationResult:
        path = resolve_project_path(crate, operation.relative_path)
        try:
            data = path.read_bytes()
        except OSError as exc:
            return ValidationResult(False, "bitfield_file_unreadable", str(exc))
        if not (0 <= operation.start_byte < operation.end_byte <= len(data)):
            return ValidationResult(False, "bitfield_span_out_of_bounds")
        span = data[operation.start_byte:operation.end_byte]
        if sha256_bytes(span) != operation.expected_span_hash:
            return ValidationResult(False, "bitfield_span_stale")
        return ValidationResult(True, "bitfield_span_fresh")

    def post_validate(
        self, operation: RewriteBitfieldStruct, resolution, crate: Path
    ) -> ValidationResult:
        path = resolve_project_path(crate, operation.relative_path)
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError as exc:
            return ValidationResult(False, "bitfield_file_unreadable", str(exc))
        if not text:
            return ValidationResult(False, "bitfield_file_emptied")
                                                    
                                                          
                                         
        marker = f"impl {operation.struct_name} {{"
        if marker not in text:
            return ValidationResult(False, "bitfield_impl_missing")
        return ValidationResult(True, "bitfield_struct_lowered")
