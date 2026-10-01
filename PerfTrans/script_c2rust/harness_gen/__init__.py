"""harness_gen — LLM agent that generates a workload harness for a
c2rust-translated Rust crate (e.g. dataset_trans_process/lodepng/0_raw).

The agent follows the errfixer pattern used elsewhere in this repo
(stage_a intra_ptr / buffer_lift): the LLM only writes code; every
judgement is made by a deterministic gate:

  [1] API inventory        — regex scan of `pub extern "C" fn` surface (no LLM)
  [2] plan                 — LLM emits a JSON harness spec (operations + seeds)
  [3] codegen              — LLM emits src/main.rs; Cargo.toml etc. are scaffolded
  [4] gate A: cargo build  — stderr fed back on failure, bounded retries
      gate B: smoke        — gen-seeds, run twice (determinism), digest must
                             depend on the input bytes (anti-cheat), iters mode
      gate C: coverage     — -C instrument-coverage + llvm-cov-17; uncovered
                             pub fns are fed back so the LLM adds operations

Generated harness contract (single bin named `harness`):

    harness gen-seeds <dir>            # write <op>.1.bin / <op>.2.bin per op
    harness <op> <input-file> [iters]  # run op, print digest lines on stdout

Entry point:  python -m harness_gen --project dataset_trans_process/<p>/0_raw
LLM config comes exclusively from Config/paths.conf via utils.llm_client.
"""

__all__ = ["agent", "inventory", "gates", "prompts"]
