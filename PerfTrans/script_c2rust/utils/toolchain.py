"""Toolchain version gates for the script_c2rust pipeline."""

from __future__ import annotations

import os
import re
import shutil
import subprocess
from pathlib import Path

from Config.paths import get_path


def required_c2rust_version() -> str:
    return get_path("REQUIRED_C2RUST_VERSION", "0.22.1") or "0.22.1"


def required_rustc_minor() -> str:
    return get_path("REQUIRED_RUSTC_MINOR", "1.77") or "1.77"


def required_rust_toolchain() -> str:
    return get_path("REQUIRED_RUST_TOOLCHAIN", "nightly-2024-01-15") or "nightly-2024-01-15"


def required_rustc_channel() -> str:
    return get_path("REQUIRED_RUSTC_CHANNEL", "nightly") or "nightly"


def required_c_compiler_version() -> str:
    return get_path("REQUIRED_C_COMPILER_VERSION", "17.0.6") or "17.0.6"


def configured_c_compiler() -> str:
    return get_path("C_COMPILER_BIN", "clang-17") or "clang-17"


def _resolve_command(config_value: str | None, default: str) -> Path:
    raw = config_value or default
    path = Path(raw)
    if path.is_absolute() or os.sep in raw:
        return path
    found = shutil.which(raw)
    return Path(found) if found else path


def resolve_c2rust_bin(config_value: str | None) -> Path:
    return _resolve_command(config_value, "c2rust")


def require_c2rust_version(c2rust_bin: Path) -> Path:
    """Return the resolved c2rust path, or raise with an actionable message."""
    binary = resolve_c2rust_bin(str(c2rust_bin))
    want = required_c2rust_version()
    if not binary.exists():
        raise RuntimeError(
            f"c2rust binary not found at {binary}. "
            f"Install c2rust {want} or set C2RUST_BIN in Config/paths.conf."
        )
    if not os.access(binary, os.X_OK):
        raise RuntimeError(f"c2rust binary at {binary} is not executable")
    try:
        proc = subprocess.run(
            [str(binary), "--version"],
            capture_output=True, text=True, timeout=30,
        )
    except OSError as e:
        raise RuntimeError(f"failed to launch c2rust at {binary}: {e}") from e
    if proc.returncode != 0:
        raise RuntimeError(
            f"failed to read c2rust version from {binary}: "
            f"{(proc.stderr or proc.stdout or '').strip()}"
        )
    text = (proc.stdout or proc.stderr or "").strip()
    m = re.search(r"(\d+\.\d+\.\d+)", text)
    got = m.group(1) if m else text
    if got != want:
        raise RuntimeError(
            f"c2rust version mismatch: got {got!r} from {binary}, "
            f"want {want!r}. Install with: cargo install c2rust --version {want}"
        )
    return binary


def require_c_compiler_version() -> Path:
    """Require the configured C baseline compiler at the pinned version."""
    raw = configured_c_compiler()
    binary = _resolve_command(raw, "clang-17")
    want = required_c_compiler_version()
    if not binary.exists():
        raise RuntimeError(
            f"C compiler not found at {binary}. "
            f"Install Clang {want} or set C_COMPILER_BIN in Config/paths.conf."
        )
    if not os.access(binary, os.X_OK):
        raise RuntimeError(f"C compiler at {binary} is not executable")
    try:
        proc = subprocess.run(
            [str(binary), "--version"],
            capture_output=True, text=True, timeout=30,
        )
    except OSError as e:
        raise RuntimeError(f"failed to launch C compiler at {binary}: {e}") from e
    if proc.returncode != 0:
        raise RuntimeError(
            f"failed to read C compiler version from {binary}: "
            f"{(proc.stderr or proc.stdout or '').strip()}"
        )
    text = (proc.stdout or proc.stderr or "").strip()
    if want not in text:
        raise RuntimeError(
            f"C compiler version mismatch: got {text.splitlines()[0]!r} "
            f"from {binary}, want Clang {want}. "
            f"Install Clang {want} and set C_COMPILER_BIN={raw!r}."
        )
    return binary


def write_rust_toolchain(crate_dir: Path) -> None:
    toolchain = required_rust_toolchain()
    text = (
        "[toolchain]\n"
        f'channel = "{toolchain}"\n'
        'components = ["rustfmt"]\n'
    )
    (crate_dir / "rust-toolchain.toml").write_text(text, encoding="utf-8")


def require_rustc_version(cwd: Path) -> None:
    """Reject cargo/rustc work when the crate resolves to the wrong rustc."""
    want_minor = required_rustc_minor()
    want_channel = required_rustc_channel()
    want_toolchain = required_rust_toolchain()
    try:
        proc = subprocess.run(
            ["rustc", "--version"],
            cwd=str(cwd), capture_output=True, text=True, timeout=30,
        )
    except OSError as e:
        raise RuntimeError(
            f"failed to launch rustc in {cwd}: {e}. "
            f"Install Rust {want_minor} with: rustup toolchain install {want_toolchain}"
        ) from e
    if proc.returncode != 0:
        raise RuntimeError(
            f"failed to read rustc version in {cwd}: "
            f"{(proc.stderr or proc.stdout or '').strip()}. "
            f"Install/switch with: rustup toolchain install {want_toolchain}; "
            f"rustup override set {want_toolchain}"
        )
    text = (proc.stdout or proc.stderr or "").strip()
    m = re.search(r"rustc\s+(\d+\.\d+\.\d+)", text)
    got = m.group(1) if m else text
    if not got.startswith(f"{want_minor}."):
        raise RuntimeError(
            f"rustc version mismatch in {cwd}: got {got!r}, "
            f"want rustc {want_minor}.x. "
            f"Switch/install with: rustup toolchain install {want_toolchain}; "
            f"rustup override set {want_toolchain}"
        )
    if want_channel and want_channel not in text:
        raise RuntimeError(
            f"rustc channel mismatch in {cwd}: got {text!r}, "
            f"want {want_channel!r} because c2rust 0.22.1 emits "
            f"#![feature(...)] gates. "
            f"Switch/install with: rustup toolchain install {want_toolchain}; "
            f"rustup override set {want_toolchain}"
        )
