"""Small shared helpers copied from the top-level profiling module so hot_probe
is self-contained (does not import the retiring `profiling/` package)."""

from __future__ import annotations

import secrets


def _resolve_rand(args: list[str]) -> list[str]:
    """Per-invocation `$RAND` → 12-hex substitution (fresh every subprocess
    call, for workloads that embed it in shared resource names, e.g. a tmux
    socket). No-op when no arg contains the placeholder — the harness CLI
    (`<op> <input> <iters>`) never does."""
    if not any("$RAND" in a for a in args):
        return list(args)
    rand = secrets.token_hex(6)
    return [a.replace("$RAND", rand) for a in args]
