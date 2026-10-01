"""Fair-build config audit — see docs/perf_tree_design.md §2.8.

Verifies that every crate involved in a perf_opt run (lib + harness)
carries the canonical build config:

  1. Cargo.toml has [profile.release] with:
        opt-level     = 3
        lto           = "fat"
        codegen-units = 1
        panic         = "abort"   # binary / harness only
        debug         = false

  2. .cargo/config.toml has:
        [build]
        rustflags = ["-Ctarget-cpu=native"]

  3. pipeline.toml [c_reference].build_cmd carries
        -O3 -flto -march=native
     so the C baseline matches Rust's lto=fat + opt-level=3 + native CPU.

     Watch out for the Makefile-CFLAGS-override trap: a build_cmd like
        CFLAGS='-O3 -flto -march=native' make bzip2
     looks fair, but if the Makefile assigns CFLAGS with `=` (not `?=`)
     the environment value is silently ignored and the link runs at the
     Makefile default (typically `-O2 -g`). For Makefile-driven projects
     use either single-shot compile (`clang -O3 -flto ... *.c`) OR the
     `make CFLAGS='...'` *override* form (CFLAGS as a make argument, not
     an exported env var). Audit warns when it sees the env-export form
     applied to a Makefile project. (2026-05-24, bzip2 incident.)

If anything drifts, the run aborts with a precise diff. Rationale:
silent build-flag drift has produced bogus three-way comparison numbers
in the past (e.g. ours 808ms vs C 560ms turned out to be input mismatch
+ missing native-cpu). Future projects inherit this audit automatically.
"""

from __future__ import annotations

import logging
import re
import tomllib
from pathlib import Path

logger = logging.getLogger(__name__)


_PROFILE_EXPECT_LIB = {
    "opt-level":     3,
    "lto":           "fat",
    "codegen-units": 1,
    "debug":         False,
}
# Binary / harness adds panic=abort. Library crate can NOT carry panic=abort
# (cargo emits "profiles for the non root package will be ignored" but the
# value itself is fine for staticlib; we accept either presence or absence).
_PROFILE_EXPECT_BIN = {**_PROFILE_EXPECT_LIB, "panic": "abort"}

_RUSTFLAGS_EXPECT = "-Ctarget-cpu=native"
# The crate cargo is invoked in (the harness) additionally needs the 256-bit
# vector pin, because that file alone decides the codegen of the whole binary
# and the evaluation builds with both. See `_audit_cargo_config(timed=True)`.
_RUSTFLAGS_EXPECT_TIMED = ("-Ctarget-cpu=native", "-Ctarget-feature=-avx512f",
                           "-Cllvm-args=-align-all-functions=6")


def _has_bin(cargo_toml: dict) -> bool:
    """True if the Cargo.toml declares a `[[bin]]` target or a `main.rs`-style
    default binary (i.e. it's a driving crate that the profile applies to)."""
    bins = cargo_toml.get("bin")
    if bins:
        return True
    # `autobins=false` + no [[bin]] table → library only.
    if cargo_toml.get("package", {}).get("autobins") is False:
        return False
    return False  # we default to lib-only when [[bin]] missing


def _audit_cargo_profile(cargo_toml_path: Path) -> list[str]:
    """Return list of human-readable error strings; empty list = OK."""
    errors: list[str] = []
    try:
        data = tomllib.loads(cargo_toml_path.read_text())
    except (OSError, tomllib.TOMLDecodeError) as e:
        return [f"{cargo_toml_path}: cannot parse: {e}"]

    profile = data.get("profile", {}).get("release")
    if not profile:
        return [f"{cargo_toml_path}: missing [profile.release]"]

    expect = _PROFILE_EXPECT_BIN if _has_bin(data) else _PROFILE_EXPECT_LIB
    for k, want in expect.items():
        got = profile.get(k)
        if got != want:
            errors.append(
                f"{cargo_toml_path}: profile.release.{k} = {got!r}, "
                f"want {want!r}"
            )
    return errors


def _audit_cargo_config(crate_dir: Path, *, timed: bool = False) -> list[str]:
    """Verify `crate_dir/.cargo/config.toml` carries the expected rustflags.

    `timed=True` for the crate cargo is INVOKED in (the harness). That one file
    is the only `.cargo/config.toml` cargo reads for the build — it never
    follows a path dependency — so it alone decides the codegen of the whole
    binary, library included. It must therefore match what the evaluation
    builds with, which is `-Ctarget-cpu=native` PLUS the 256-bit vector pin.

    Missing the vector pin is not cosmetic: on fzy the same rewrite measured
    -22.3% under the x86-64 baseline and +0.3% under `target-cpu=native`, so a
    harness without these flags lets the gate accept changes that are worth
    nothing in the configuration the paper reports (2026-09-07).
    """
    expect = list(_RUSTFLAGS_EXPECT_TIMED) if timed else [_RUSTFLAGS_EXPECT]
    cfg_path = crate_dir / ".cargo" / "config.toml"
    if not cfg_path.is_file():
        want = ", ".join(f'"{f}"' for f in expect)
        return [f"{cfg_path}: missing (need [build] rustflags=[{want}])"]
    try:
        data = tomllib.loads(cfg_path.read_text())
    except (OSError, tomllib.TOMLDecodeError) as e:
        return [f"{cfg_path}: cannot parse: {e}"]
    rustflags = data.get("build", {}).get("rustflags", [])
    if isinstance(rustflags, str):
        rustflags = re.split(r"\s+", rustflags.strip())
    missing = [f for f in expect if f not in rustflags]
    if missing:
        return [f"{cfg_path}: rustflags missing {missing!r}; got {rustflags!r}"]
    return []


# ─────────────────────────────────────────────────────────────────
# C reference build_cmd audit
# ─────────────────────────────────────────────────────────────────

# Each flag pattern is a regex; the build_cmd matches when at least one
# of its alternation arms is present. We allow both clang-style and
# gcc-style invocations.
_C_FLAG_PATTERNS: list[tuple[str, re.Pattern]] = [
    ("-O3",            re.compile(r"(?<!\w)-O3(?!\w)")),
    ("-flto",          re.compile(r"(?<!\w)-flto(?:=\w+)?(?!\w)")),
    ("-march=native",  re.compile(r"-march=native(?!\w)")),
]

# Heuristic for the "Makefile + env-export CFLAGS" trap. Pattern:
# `CFLAGS=...` (no leading whitespace = treated as shell env export) followed
# eventually by `make ...` *without* CFLAGS= as a make argument. The fix is
# `make CFLAGS='...'` (override form) or a single-shot compiler invocation.
_MAKEFILE_TRAP_RE = re.compile(
    r"""(?x)
    (?:^|[;&|\s])           # start of statement
    CFLAGS\s*=              # env-style assignment …
    [^;|]*?                 # … (flag list)
    \bmake\b                # … followed somewhere by `make`
    (?![^;|]*CFLAGS\s*=)    # but no make-argument CFLAGS later in this stmt
    """,
)


def _audit_c_reference(pipeline_toml: Path) -> list[str]:
    """Inspect `[c_reference].build_cmd` of `pipeline_toml`. Returns a list of
    error / warning strings; empty list = pass. Absence of `[c_reference]`
    is not an error — Stage A still runs without a C reference; we only
    enforce flag parity when the user did opt into a C baseline."""
    if not pipeline_toml.is_file():
        return []
    try:
        data = tomllib.loads(pipeline_toml.read_text())
    except (OSError, tomllib.TOMLDecodeError) as e:
        return [f"{pipeline_toml}: cannot parse: {e}"]
    cref = data.get("c_reference")
    if not isinstance(cref, dict):
        return []
    cmd = cref.get("build_cmd")
    if not isinstance(cmd, str) or not cmd.strip():
        return [f"{pipeline_toml}: [c_reference] present but build_cmd is empty"]
    errors: list[str] = []
    for flag, pat in _C_FLAG_PATTERNS:
        if not pat.search(cmd):
            errors.append(
                f"{pipeline_toml}: [c_reference].build_cmd missing {flag!r} — "
                f"required for parity with Rust profile.release "
                f"(lto=fat + opt-level=3 + -Ctarget-cpu=native); see §2.8"
            )
    if _MAKEFILE_TRAP_RE.search(cmd):
        errors.append(
            f"{pipeline_toml}: [c_reference].build_cmd uses the "
            f"`CFLAGS='...' make` env-export form on a Makefile project — "
            f"Makefile assignments with `CFLAGS=...` silently override "
            f"environment CFLAGS, so the build runs at the Makefile default "
            f"(often -O2 -g). Rewrite as `make CFLAGS='...' <target>` "
            f"(override form) OR a single-shot compile (`clang -O3 -flto … *.c`). "
            f"See §2.8 + bzip2 incident 2026-05-24."
        )
    return errors


def audit(project_dir: Path, harness_dir: Path | None = None,
          *, strict: bool = True,
          pipeline_toml: Path | None = None) -> list[str]:
    """Audit every crate touched by perf_opt for §2.8 fair-build config.

    `project_dir`   the 2_safe/ (or equivalent) lib/binary crate.
    `harness_dir`   optional driving crate (for library benchmarks). Cargo
                    resolves `.cargo/config.toml` from the cwd of `cargo`
                    walking up; the driving crate's config is what actually
                    applies to the build, so audit it specifically.
    `pipeline_toml` optional path to `workloads/pipeline.toml`. When
                    supplied AND it has a `[c_reference]` section, we also
                    audit the `build_cmd` for -O3/-flto/-march=native parity
                    + the Makefile CFLAGS env-export trap.

    Returns a list of errors; empty list = pass.
    When `strict=True` and the list is non-empty, the caller should abort.
    """
    errors: list[str] = []
    errors.extend(_audit_cargo_profile(project_dir / "Cargo.toml"))
    errors.extend(_audit_cargo_config(project_dir))
    if harness_dir is not None:
        errors.extend(_audit_cargo_profile(harness_dir / "Cargo.toml"))
        # timed=True: the harness is where cargo runs, so its config is the one
        # that decides codegen for the whole binary — library included.
        errors.extend(_audit_cargo_config(harness_dir, timed=True))
    if pipeline_toml is not None:
        errors.extend(_audit_c_reference(pipeline_toml))

    if errors:
        for e in errors:
            logger.error(f"[fair-build] {e}")
        if strict:
            logger.error(
                "[fair-build] §2.8 fair-build config check FAILED. Fix the "
                "above before running perf_opt — silent drift here has "
                "produced bogus three-way comparison numbers in the past."
            )
    else:
        logger.info("[fair-build] §2.8 config OK on project + harness")
    return errors
