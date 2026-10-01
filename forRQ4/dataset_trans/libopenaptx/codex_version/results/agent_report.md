# libopenaptx Performance Optimization Report

## Modified Files

- `code/src/openaptx.rs`
  - Hot-path helper functions were marked `#[inline(always)]` where they are tiny and called per sample/block.
  - Replaced fixed-size `while` loops in dither generation, QMF polyphase analysis/synthesis, 16-tap convolution, 4-subband encode/decode bookkeeping, and 2-channel pack/unpack paths with direct fixed-layout code.
  - Removed one `% order` from predictor history rotation and replaced repeated table lookups with a small helper.
  - Rewrote 24-bit PCM load/store in `aptx_encode` / `aptx_decode` to fixed block reads/writes, reducing per-sample pointer arithmetic and branch overhead.
- `work/bench/Cargo.toml`
  - Temporary benchmark crate used for local throughput checks only.
- `work/bench/src/main.rs`
  - Temporary benchmark driver used to measure encode/decode throughput and print output hashes for the optimized build.

## Expected Benefit Sources

- Lower loop overhead in the most frequently executed fixed-size kernels.
- Fewer repeated pointer recalculations and fewer array index/offset computations.
- Cheaper channel/subband dispatch by replacing runtime loops with fixed direct calls.
- More optimizer-friendly code shape for inlining and constant propagation.

## Local Build Check

- Verified successful release build with:
  - `cargo +nightly-2024-01-15 build --release` in `code/`

Note: plain stable `cargo build --release` failed in this environment because the translated crate still uses nightly-only feature gates already present in the project.

## Local Benchmark Observation

Benchmark harness: `work/bench`

Baseline before optimization:

- `mode=std encode_mib_s=26.36 decode_mib_s=40.69`
- `mode=hd encode_mib_s=28.35 decode_mib_s=42.02`

After optimization:

- `mode=std encode_mib_s=31.94 decode_mib_s=48.44 encoded_hash=0622f2413ca0361e decoded_hash=4f81edb995bf2311`
- `mode=hd encode_mib_s=31.24 decode_mib_s=48.72 encoded_hash=2b533259b52fe291 decoded_hash=3d3aec276270ad92`

Observed throughput change from this local run:

- Standard aptX encode: about `+21%`
- Standard aptX decode: about `+19%`
- aptX HD encode: about `+10%`
- aptX HD decode: about `+16%`

## Limitations

- I did not have a recoverable tracked `HEAD` snapshot for this sub-workspace, so I do not have pre-optimization hashes from the same benchmark harness.
- The local benchmark is synthetic and should only be treated as a directional signal.

## Final Note

Agent self-test data is only for reference. You should recompile and measure with the external post-hoc benchmarking script, and the external measurement result should be treated as the final performance conclusion.
