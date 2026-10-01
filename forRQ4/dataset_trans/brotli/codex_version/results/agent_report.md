# Brotli Performance Optimization Report

## Summary

I optimized hot encoder-side helper paths in the c2rust-generated Rust code without changing the public API or output format.

## Files Modified

- `code/src/enc/compress_fragment.rs`
  - Replaced `memcpy`-based unaligned 32/64-bit loads and 64-bit stores with `core::ptr::read_unaligned` / `write_unaligned`.
  - Intent: remove libc-call shaped helper code from one-pass compression hot loops so LLVM can inline and simplify the load/store path.

- `code/src/enc/compress_fragment_two_pass.rs`
  - Same unaligned read/write helper replacement as above for the two-pass fragment compressor.
  - Intent: reduce overhead in another compression hot loop family.

- `code/src/enc/backward_references_hq.rs`
  - Replaced `memcpy`-based unaligned reads with `read_unaligned`.
  - Added `#[inline(always)]` to small queue / Zopfli helper functions: `StartPosQueueSize`, `StartPosQueuePush`, `StartPosQueueAt`, `ComputeMinimumCopyLength`, `ComputeDistanceShortcut`, `ComputeDistanceCache`, `EvaluateNode`.
  - Intent: reduce call overhead and improve optimizer visibility in the HQ match-finding / shortest-path path.

- `code/src/enc/backward_references.rs`
  - Replaced `memcpy`-based unaligned reads and pointer loads with `read_unaligned`.
  - Intent: reduce helper overhead in general backward reference matching.

- `code/src/enc/encode.rs`
  - Replaced `memcpy`-based unaligned 32/64-bit loads and 64-bit stores with `read_unaligned` / `write_unaligned`.
  - Intent: improve the common encoder helper path used by multiple hasher / match routines.

- `code/src/enc/static_dict.rs`
  - Replaced `memcpy`-based unaligned reads with `read_unaligned`.
  - Intent: trim helper overhead in static dictionary matching.

- `code/src/enc/compound_dictionary.rs`
  - Replaced `memcpy`-based unaligned 64-bit loads with `read_unaligned`.
  - Intent: improve compound dictionary hashing/matching helper cost.

- `code/src/enc/brotli_bit_stream.rs`
  - Replaced `memcpy`-based unaligned 64-bit stores with `write_unaligned`.
  - Intent: reduce overhead in bitstream output writes.

## Why These Changes Should Help

The original c2rust output expresses tiny unaligned loads/stores via `memcpy`, which is semantically valid but often less optimizer-friendly in hot loops than direct unaligned pointer operations. Replacing those helpers with `read_unaligned` / `write_unaligned` removes extra local temporaries and gives LLVM a more direct representation of the memory operation.

The HQ encoder path also had several very small helper functions called from tight inner loops. Marking them `#[inline(always)]` improves the chance that release LTO fully flattens those calls into the main search loop, reducing branch/call overhead and enabling constant propagation across helper boundaries.

## Build Verification

- Verified with: `cargo build --release --lib` in `code/`
- Note: plain `cargo build --release` still fails in this repository because the `brotli` bin target has pre-existing unresolved external references during linking. The library target used for optimization work compiles successfully.

## Self-Test / Benchmark Notes

I created a temporary benchmark in `work/bench_compress.rs` and built a clean baseline copy from `HEAD` in `work/baseline_root/` for A/B comparison.

Benchmark setup:

- API used: `BrotliEncoderCompress`
- Quality: `11`
- Window: `22`
- Input: generated repetitive/semi-structured ~8.9 MiB buffer
- Iterations: `12`

Baseline (`HEAD`) result:

- `input_bytes=8905678`
- `output_bytes=56439`
- `elapsed_secs=22.431150`
- `throughput_mib_s=4.544`
- `output_checksum=13671721486523537702`

Optimized result:

- `input_bytes=8905678`
- `output_bytes=56439`
- `elapsed_secs=21.075849`
- `throughput_mib_s=4.836`
- `output_checksum=13671721486523537702`

Observed local improvement:

- Throughput improved from `4.544 MiB/s` to `4.836 MiB/s`
- Relative gain: about `+6.4%`
- Output checksum matched exactly, so the compressed output for this benchmark input was unchanged byte-for-byte.

## Important Caveat

Agent self-test data is only a local reference. The final performance conclusion must come from the external post-hoc measurement pipeline, which will rebuild and measure all candidates under a uniform environment.
