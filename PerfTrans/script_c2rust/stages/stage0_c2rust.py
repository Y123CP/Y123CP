"""Stage 0 — invoke c2rust transpile on the source C project.

Input  : path to a C project (compile_commands.json optional — auto-generated)
Output : <stage_dir>/0_raw  — fresh c2rust output (a Cargo project)

Stage 0 does NOT inherit from StageBase: StageBase assumes the input is
already a Cargo project, but Stage 0's job is to *create* it. It still
returns a StageResult so main.py can chain stages uniformly.
"""

from __future__ import annotations

import argparse
import json
import logging
import os
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

# allow `python stages/stage0_c2rust.py` AND `python -m stages.stage0_c2rust`
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from Config.paths import get_path
from stages.stage_base import StageBase, StageResult

logger = logging.getLogger(__name__)


@dataclass
class C2RustOpts:
    overwrite_existing: bool = True
    preserve_unused_functions: bool = False
    emit_build_files: bool = True       # need Cargo.toml in output
    binary: Optional[str] = None        # TU whose main() to promote
    extra_clang_args: Optional[list[str]] = None


class Stage0C2Rust:
    name = "stage0_c2rust"

    def __init__(self, args, opts: Optional[C2RustOpts] = None):
        self.args = args
        self.opts = opts or C2RustOpts()
        self.c2rust_bin = Path(get_path("C2RUST_BIN") or "")

    # ------------------------------------------------------------------
    # Public entry
    # ------------------------------------------------------------------

    def run(self, c_project: Path, output: Path) -> StageResult:
        c_project, output = Path(c_project).resolve(), Path(output).resolve()
        logger.info(f"[{self.name}] {c_project} → {output}")

        if err := self._preflight(c_project):
            return self._fail(output, err)

        compile_db = self._ensure_compile_commands(c_project)
        if compile_db is None:
            return self._fail(output, f"could not locate or generate compile_commands.json under {c_project}")

        if err := self._transpile(compile_db, output):
            return self._fail(output, err)

        if err := self._normalize_output(output):
            return self._fail(output, err)

        if err := self._smoke_build(output):
            return self._fail(output, err)

        return StageResult(self.name, ok=True, output_path=output)

    # ------------------------------------------------------------------
    # Preflight checks
    # ------------------------------------------------------------------

    def _preflight(self, c_project: Path) -> Optional[str]:
        if not self.c2rust_bin.exists():
            return (f"c2rust binary not found at {self.c2rust_bin}. "
                    f"Set C2RUST_BIN in Config/paths.conf or install c2rust.")
        if not os.access(self.c2rust_bin, os.X_OK):
            return f"c2rust binary at {self.c2rust_bin} is not executable"
        if not c_project.is_dir():
            return f"source C project does not exist: {c_project}"
        return None

    # ------------------------------------------------------------------
    # compile_commands.json — exists or generate
    # ------------------------------------------------------------------

    def _ensure_compile_commands(self, c_project: Path) -> Optional[Path]:
        cdb = c_project / "compile_commands.json"

        # If the top-level file is missing, fall back to common build
        # subdirectories (CMake writes to build/, some projects keep one
        # at src/, _build/, out/). When we find one, copy it to the
        # project root so c2rust's `transpile` command — which only
        # accepts a single path argument — picks it up.
        if not cdb.exists():
            for sub in ("build", "src", "out", "_build", "Release-build", "Debug-build"):
                candidate = c_project / sub / "compile_commands.json"
                if candidate.is_file():
                    shutil.copy(candidate, cdb)
                    logger.info(
                        f"[{self.name}] using "
                        f"{candidate.relative_to(c_project)} → compile_commands.json"
                    )
                    break

        if cdb.exists():
            validated = self._validate_or_patch_cdb(c_project, cdb)
            if validated:
                return validated
            logger.info(f"[{self.name}] existing compile_commands.json unusable — regenerating")
        else:
            logger.info(f"[{self.name}] no compile_commands.json — auto-generating")

        for strategy in (self._gen_flat_c, self._gen_bear, self._gen_cmake):
            label = strategy.__name__.replace("_gen_", "")
            try:
                if strategy(c_project) and cdb.exists():
                    logger.info(f"[{self.name}] generated via [{label}]")
                    return cdb
            except Exception as e:
                logger.warning(f"[{self.name}] [{label}] failed: {e}")
        return None

    def _validate_or_patch_cdb(self, c_project: Path, cdb: Path) -> Optional[Path]:
        """Verify entries point to real files; rewrite stale `directory`
        fields to `c_project` if needed (common when projects are moved
        across machines / users). Returns cdb on success, None if the
        file is malformed or paths can't be reconciled.
        """
        try:
            entries = json.loads(cdb.read_text())
        except Exception as e:
            logger.warning(f"[{self.name}] {cdb.name} is not valid JSON: {e}")
            return None
        if not entries:
            return None

        def resolve(e: dict) -> Path:
            f = Path(e["file"])
            return f if f.is_absolute() else Path(e["directory"]) / f

        if all(resolve(e).exists() for e in entries):
            return cdb

        logger.info(f"[{self.name}] compile_commands.json has stale paths "
                    f"— rewriting `directory` → {c_project}")
        patched = [{**e, "directory": str(c_project)} for e in entries]
        missing = [e["file"] for e in patched if not resolve(e).exists()]
        if missing:
            logger.warning(f"[{self.name}] still missing after patch: {missing[:3]}...")
            return None
        cdb.write_text(json.dumps(patched, indent=2))
        return cdb

    @staticmethod
    def _gen_flat_c(c_project: Path) -> bool:
        """Single-source-tree project, no build system: enumerate .c files
        under top-level + `src/` and emit a flat compile_commands.json.

        Skips if a Makefile / makefile / CMakeLists.txt is present (those
        are handled by `_gen_bear` / `_gen_cmake`). Recursive enumeration
        is necessary because projects like binn keep their .c sources in
        `src/`; top-level-only glob misses them and Stage 0 falls through
        to the (also-skipping) bear / cmake strategies → soft-fail.
        """
        if (c_project / "CMakeLists.txt").exists():
            return False
        # Linux is case-sensitive; both `Makefile` (GNU convention) and
        # `makefile` (older / non-standard) are valid GNU make inputs.
        if any((c_project / n).exists() for n in ("Makefile", "makefile", "GNUmakefile")):
            return False
        # Recurse into src/ subtrees (binn / lil / libcsv layouts); skip
        # build/cache dirs and any test fixtures that look like sources.
        c_files: list[Path] = []
        for root in (c_project, c_project / "src"):
            if root.is_dir():
                c_files.extend(sorted(root.rglob("*.c")))
        if not c_files:
            return False
        entries = [
            {"directory": str(c_project),
             "file": str(c.relative_to(c_project)),
             "arguments": ["clang", "-c", "-O2", str(c.relative_to(c_project))]}
            for c in c_files
        ]
        (c_project / "compile_commands.json").write_text(json.dumps(entries, indent=2))
        return True

    @staticmethod
    def _gen_bear(c_project: Path) -> bool:
        """Makefile-based: run `bear -- make` if `bear` is installed.

        Accepts both `Makefile` (GNU convention) and lowercase `makefile`
        / `GNUmakefile` — Linux is case-sensitive, and projects like binn
        ship a `makefile` whose target is `make`. The check just probes
        for any name `make` will accept; the actual build is delegated to
        `make`, which finds its own Makefile via its built-in search.
        """
        if not any((c_project / n).exists()
                   for n in ("Makefile", "makefile", "GNUmakefile")):
            return False
        if shutil.which("bear") is None:
            logger.info("  [bear] not installed — skip; install via `apt install bear`")
            return False
        res = subprocess.run(["bear", "--", "make"], cwd=str(c_project),
                             capture_output=True, text=True, timeout=600)
        return res.returncode == 0

    @staticmethod
    def _gen_cmake(c_project: Path) -> bool:
        """CMake-based: configure with CMAKE_EXPORT_COMPILE_COMMANDS, then symlink."""
        if not (c_project / "CMakeLists.txt").exists():
            return False
        if shutil.which("cmake") is None:
            logger.info("  [cmake] not installed — skip")
            return False
        build = c_project / "build"
        build.mkdir(exist_ok=True)
        res = subprocess.run(
            ["cmake", "-S", str(c_project), "-B", str(build),
             "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON"],
            capture_output=True, text=True, timeout=600,
        )
        if res.returncode != 0:
            return False
        produced = build / "compile_commands.json"
        if produced.exists():
            shutil.copy(produced, c_project / "compile_commands.json")
            return True
        return False

    # ------------------------------------------------------------------
    # Transpile + normalize
    # ------------------------------------------------------------------

    def _transpile(self, compile_db: Path, output: Path) -> Optional[str]:
        if output.exists():
            shutil.rmtree(output)
        output.mkdir(parents=True, exist_ok=True)

        cmd = [str(self.c2rust_bin), "transpile", str(compile_db), "-o", str(output)]
        if self.opts.emit_build_files:          cmd.append("--emit-build-files")
        if self.opts.overwrite_existing:        cmd.append("--overwrite-existing")
        if self.opts.preserve_unused_functions: cmd.append("--preserve-unused-functions")
        if self.opts.binary:                    cmd += ["--binary", self.opts.binary]
        if self.opts.extra_clang_args:          cmd += ["--", *self.opts.extra_clang_args]

        logger.info(f"[{self.name}] $ {' '.join(cmd)}")
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=1800)
        if res.returncode != 0:
            tail = "\n".join((res.stderr or res.stdout or "").splitlines()[-50:])
            return f"c2rust transpile failed:\n{tail}"
        return None

    def _normalize_output(self, output: Path) -> Optional[str]:
        # c2rust occasionally nests the cargo project one level deep — flatten.
        if not (output / "Cargo.toml").exists():
            inner = next((p for p in output.iterdir()
                          if p.is_dir() and (p / "Cargo.toml").exists()), None)
            if inner:
                logger.info(f"[{self.name}] flattening {inner.name}/ → {output}")
                for entry in inner.iterdir():
                    shutil.move(str(entry), str(output / entry.name))
                inner.rmdir()
        if not (output / "Cargo.toml").exists():
            return "c2rust ran but no Cargo.toml in output"

        # Re-namespace package: <project>_<stage_suffix>, e.g. `sortbench_raw`.
        pkg_name = StageBase.derive_pkg_name(output)
        StageBase._patch_cargo_pkg_name(output / "Cargo.toml", pkg_name)
        logger.info(f"[{self.name}] Cargo package name → '{pkg_name}'")

        # Promote every TU containing `pub fn main()` into its own [[bin]],
        # otherwise multi-main projects (e.g. bzip2 + bzip2recover) collide
        # on top-level statics like `progName`.
        promoted = self._promote_main_tus(output, pkg_name)
        if promoted:
            logger.info(f"[{self.name}] promoted to bin: {promoted}")
        return None

    @staticmethod
    def _promote_main_tus(output: Path, pkg_name: str) -> list[str]:
        """For each .rs file containing `pub fn main()`, register it as a
        cargo `[[bin]]`. Two layouts are handled:

        - **Top-level** `src/<TU>.rs`: promote in-place. The TU file itself
          becomes the bin crate root; we prepend lib.rs's inner attrs and
          `use ::<pkg_name>;`, drop its `pub mod <TU>;` from lib.rs, and
          add a `[[bin]] path = "src/<TU>.rs"` entry. (This is the typical
          c2rust output for flat C projects: `bzip2.rs`, `bzip2recover.rs`,
          `test_csv.rs`, `main.rs`.)

        - **Nested** `src/<sub>/.../<TU>.rs`: leave the file untouched
          (its sibling-module references would break if we extracted it
          from the lib's mod tree). Instead, auto-generate a top-level
          wrapper at `src/bin/<TU>.rs` that delegates to the lib's nested
          main, and register the wrapper as `[[bin]]`. The wrapper uses
          `use ::<pkg> as project_lib;` so the bin keeps linking when
          later stages rename the package — Stage 1's
          `_patch_cargo_pkg_name` rewrites the `use ::<pkg>` line, and
          the body's `project_lib::...` expression is unaffected.
          (Triggered by multi-component C projects like `optipng`, whose
          `int main()` lives at `src/optipng/optipng.c`.)

        Returns the list of promoted bin names (target/release/<name> each).
        """
        import re
        src_dir, lib_rs, cargo = output / "src", output / "lib.rs", output / "Cargo.toml"
        if not src_dir.is_dir() or not lib_rs.exists():
            return []

        # Mine lib.rs's inner attrs — promoted bins (top-level OR wrapper)
        # need the same gates.
        lib_text = lib_rs.read_text()
        inner_attrs = []
        for line in lib_text.splitlines():
            s = line.lstrip()
            if s.startswith("#!["):
                inner_attrs.append(line)
            elif s and not s.startswith("//"):
                break
        attrs_block = ("\n".join(inner_attrs) + "\n") if inner_attrs else ""

        # Find all .rs files containing `pub fn main()`. Split into
        # top-level (in-place promote) and nested (wrapper generate).
        main_re = re.compile(r'(?m)^\s*pub\s+fn\s+main\s*\(\s*\)')
        top_mains: list[Path] = []
        nested_mains: list[Path] = []
        for rs in sorted(src_dir.rglob("*.rs")):
            if not main_re.search(rs.read_text()):
                continue
            if rs.parent == src_dir:
                top_mains.append(rs)
            else:
                nested_mains.append(rs)

        if not top_mains and not nested_mains:
            return []

        # ---------- Top-level mains: in-place promotion ----------
        prelude = attrs_block + f"#[allow(unused_imports)]\nuse ::{pkg_name};\n"
        top_bin_names: list[str] = []
        for rs in top_mains:
            top_bin_names.append(rs.stem)
            text = rs.read_text()
            if f"use ::{pkg_name}" not in text:
                rs.write_text(prelude + text)

        # Drop `pub mod <bin>;` from lib.rs for each top-level promotion.
        for bin_name in top_bin_names:
            lib_text = re.sub(rf'(?m)^\s*pub\s+mod\s+{re.escape(bin_name)}\s*;\s*\n?',
                              '', lib_text)
        lib_rs.write_text(lib_text)

        # ---------- Nested mains: generate top-level wrappers ----------
        nested_bin_names: list[str] = []
        if nested_mains:
            wrapper_dir = src_dir / "bin"
            wrapper_dir.mkdir(exist_ok=True)
        seen_bin_names: set[str] = set(top_bin_names)
        for nested_rs in nested_mains:
            # File path → lib mod path. e.g. "src/optipng/optipng.rs" maps
            # to lib mod path `src::optipng::optipng` (lib.rs's c2rust-
            # generated mod tree mirrors the file structure).
            rel_parts = list(nested_rs.relative_to(output).with_suffix("").parts)
            mod_path = "::".join(rel_parts)

            # Pick a wrapper basename. Default: nested file's basename
            # (e.g. `optipng.rs` → bin name `optipng`). Disambiguate on
            # collision by joining the parent directory name.
            base = nested_rs.stem
            bin_name = base
            if bin_name in seen_bin_names:
                bin_name = f"{nested_rs.parent.name}_{base}"
            seen_bin_names.add(bin_name)
            nested_bin_names.append(bin_name)

            wrapper_file = wrapper_dir / f"{bin_name}.rs"
            wrapper_content = (
                attrs_block
                + f"// Auto-generated by Stage 0 (_promote_main_tus): nested-main wrapper.\n"
                + f"// Real main: {nested_rs.relative_to(output)} (a lib module).\n"
                + "// Wrapper sits at top-level so the bin crate doesn't have to be\n"
                + "// extracted from the lib mod tree (which would break the real main's\n"
                + "// sibling-module calls). The `as project_lib` alias keeps the call\n"
                + "// site stable across stage pkg-name renames.\n"
                + f"use ::{pkg_name} as project_lib;\n"
                + "\n"
                + "fn main() {\n"
                + f"    project_lib::{mod_path}::main();\n"
                + "}\n"
            )
            wrapper_file.write_text(wrapper_content)

        # ---------- Append [[bin]] entries to Cargo.toml ----------
        cargo_text = cargo.read_text()
        new_entries: list[str] = []
        for b in top_bin_names:
            if f'name = "{b}"' not in cargo_text:
                new_entries.append(f'\n[[bin]]\npath = "src/{b}.rs"\nname = "{b}"\n')
        for b in nested_bin_names:
            if f'name = "{b}"' not in cargo_text:
                new_entries.append(f'\n[[bin]]\npath = "src/bin/{b}.rs"\nname = "{b}"\n')
        if new_entries:
            cargo.write_text(cargo_text.rstrip() + "\n" + "".join(new_entries))

        return top_bin_names + nested_bin_names

    @staticmethod
    def _smoke_build(output: Path) -> Optional[str]:
        """Build the raw c2rust output once to catch hard transpile breakage
        early. NEVER hard-fail Stage 0 — c2rust 0.22.1 is the latest published
        release and ships known output bugs (`__m128i_u`, missing
        `as size_t` casts, multi-byte string-literal transmute wrapping)
        whose patches live in Stage 1's `c2rust_compat.apply_all`. If we
        hard-gated here, those projects would never reach Stage 1 and
        their auto-fixes would never run. Reduced to an INFO log; Stage 1
        is the proper place for cargo-error gating (its step-rollback
        loop already counts errors per pass and reverts losers).
        """
        res = subprocess.run(["cargo", "build", "--release"], cwd=str(output),
                             capture_output=True, text=True, timeout=900)
        if res.returncode != 0:
            import re as _re
            n_errs = len(_re.findall(r"\berror\[E\d+\]|^error:",
                                     res.stderr or "", _re.MULTILINE))
            logger.info(
                f"[{Stage0C2Rust.name}] raw output has {n_errs} cargo "
                f"error(s) — Stage 1 will attempt to patch (c2rust 0.22.1 "
                f"output bugs covered by stages.cleanup.c2rust_compat)"
            )
        return None

    def _fail(self, output: Path, err: str) -> StageResult:
        return StageResult(self.name, ok=False, output_path=output, error=err)


# ---------------------------------------------------------------------------
# Standalone debug entry
# ---------------------------------------------------------------------------

def _cli():
    p = argparse.ArgumentParser(description="Stage 0: c2rust transpile")
    p.add_argument("--c-project", required=True, help="C project root")
    p.add_argument("--output",    required=True, help="output 0_raw/ path")
    p.add_argument("--binary", default=None, help="TU main() to promote")
    p.add_argument("--preserve-unused-functions", action="store_true")
    args = p.parse_args()

    logging.basicConfig(level=logging.INFO,
                        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
                        datefmt="%H:%M:%S")

    opts = C2RustOpts(binary=args.binary,
                      preserve_unused_functions=args.preserve_unused_functions)
    stage = Stage0C2Rust(args=argparse.Namespace(), opts=opts)
    result = stage.run(Path(args.c_project), Path(args.output))
    print(result)
    sys.exit(0 if result.ok else 1)


if __name__ == "__main__":
    _cli()
