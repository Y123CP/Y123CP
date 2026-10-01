"""Explicit registry for typed ChangeSet operation handlers."""

from __future__ import annotations

from pathlib import Path
from typing import Protocol, TypeVar, cast

from .types import (
    ChangeOperation,
    OperationResolution,
    ValidationResult,
)


class OperationHandler(Protocol):
    def resolve(
        self, operation: ChangeOperation, crate: Path
    ) -> OperationResolution: ...

    def pre_validate(
        self,
        operation: ChangeOperation,
        resolution: OperationResolution,
        crate: Path,
    ) -> ValidationResult: ...

    def post_validate(
        self,
        operation: ChangeOperation,
        resolution: OperationResolution,
        crate: Path,
    ) -> ValidationResult: ...


OperationT = TypeVar("OperationT", bound=ChangeOperation)


class HandlerRegistry:
    def __init__(self) -> None:
        self._handlers: dict[type[ChangeOperation], object] = {}

    def register(
        self, operation_type: type[OperationT], handler: object
    ) -> None:
        if operation_type in self._handlers:
            raise ValueError(
                f"handler already registered for {operation_type.__name__}"
            )
        self._handlers[operation_type] = handler

    def handler_for(self, operation: ChangeOperation) -> OperationHandler:
        operation_type = type(operation)
        try:
            handler = self._handlers[operation_type]
        except KeyError as exc:
            raise KeyError(
                f"no handler registered for {operation_type.__name__}"
            ) from exc
        return cast(OperationHandler, handler)
