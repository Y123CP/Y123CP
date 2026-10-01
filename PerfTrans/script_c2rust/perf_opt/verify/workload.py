"""Verification workload source for the dataset_trans working set.

Each project ships a harness_gen golden harness under
`<proj>/workloads/harness_gen/<proj>_harness/`:

    Cargo.toml            depends on <proj>_raw (0_raw) via a path-dep
    src/{lib,main}.rs     CLI: `harness <op> <input-file> [iters]`
                          → one deterministic digest line on stdout
    golden.jsonl          {op, input_sha256, input_path, stdout_sha256}
    corpus/<op>/*.bin     inputs
    perf_inputs/, seeds/  (Stage B / perf)

`golden.jsonl` IS the W1 oracle: it records the harness's stdout digest running
against 0_raw (the c2rust_raw reference) for each (op, corpus input).

`WorkloadAssets` is the read-only INTERFACE to this: verify.functional builds a
thin driver against whatever crate-under-test and reads corpus/golden from the
assets IN PLACE — the workload is never copied or mutated. This replaced the
retired TOML-manifest source (pipeline.toml / correctness*.toml /
[oracle].stdout_hash), which no dataset_trans project carries.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass
class WorkloadAssets:
    """Read-only verification assets for one project (its harness_gen golden
    harness). NEVER copied or mutated: verify.functional.build_driver stands up
    a thin build scaffold against the crate-under-test, and golden_specs /
    replay_full read corpus + golden from here in place.

      harness_src  <proj>/workloads/harness_gen/<>_harness/  (src + Cargo.toml
                   + corpus/ + golden.jsonl)
      golden_path  harness_src/golden.jsonl  (the oracle)
    """
    project: str
    harness_src: Path
    golden_path: Path

    @classmethod
    def discover(cls, project_dir: Path) -> "WorkloadAssets | None":
        """Locate a project's harness_gen golden harness. Returns None if the
        project has none. The crate dir isn't always `<proj>_harness`
        (libopenaptx → openaptx_harness), so we glob
        `workloads/harness_gen/*_harness/` requiring a sibling golden.jsonl."""
        project_dir = Path(project_dir)
        hg = project_dir / "workloads" / "harness_gen"
        if not hg.is_dir():
            return None
        cands = sorted(p for p in hg.glob("*_harness")
                       if p.is_dir() and (p / "golden.jsonl").is_file())
        if not cands:
            return None
        h = cands[0]
        return cls(project=project_dir.name, harness_src=h,
                   golden_path=h / "golden.jsonl")
