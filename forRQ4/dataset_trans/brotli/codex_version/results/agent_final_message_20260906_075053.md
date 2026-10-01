已完成一轮以 encoder 热路径为主的微优化，改动已写入 `code/`，自测记录写入 [agent_report.md](/home/anonymous/artifact/PerfTrans/dataset_trans/brotli/codex_version/results/agent_report.md)。

修改的文件与意图如下：
- `code/src/enc/compress_fragment.rs`
  - 将热路径里的 `memcpy` 式非对齐 `Read32/Read64/Write64` 改为 `core::ptr::read_unaligned` / `write_unaligned`。
  - 收益来源：减少 one-pass 压缩主循环中的 helper 开销，让 LLVM 更容易内联和消除临时对象。
- `code/src/enc/compress_fragment_two_pass.rs`
  - 同样替换非对齐读写 helper。
  - 收益来源：降低 two-pass 压缩热循环的内存访问辅助开销。
- `code/src/enc/backward_references_hq.rs`
  - 替换非对齐读取 helper。
  - 给 `StartPosQueueSize`、`StartPosQueuePush`、`StartPosQueueAt`、`ComputeMinimumCopyLength`、`ComputeDistanceShortcut`、`ComputeDistanceCache`、`EvaluateNode` 加 `#[inline(always)]`。
  - 收益来源：减少 HQ/Zopfli 匹配与最短路径搜索中的小函数调用成本，提升优化器跨 helper 折叠能力。
- `code/src/enc/backward_references.rs`
  - 替换非对齐 `Read32/Read64`，以及指针读取 helper 为 `read_unaligned`。
  - 收益来源：降低通用 backward reference 匹配过程中的基础 helper 成本。
- `code/src/enc/encode.rs`
  - 替换通用 encoder helper 中的非对齐 `Read32/Read64/Write64`。
  - 收益来源：这些 helper 被多个编码路径复用，优化后能给多条压缩路径带来累计收益。
- `code/src/enc/static_dict.rs`
  - 替换静态字典匹配中的非对齐读取 helper。
  - 收益来源：降低字典匹配过程的基础访问成本。
- `code/src/enc/compound_dictionary.rs`
  - 替换复合字典中的非对齐 64-bit 读取 helper。
  - 收益来源：减少字典哈希/匹配辅助开销。
- `code/src/enc/brotli_bit_stream.rs`
  - 替换 bitstream 输出里的非对齐 64-bit 写入 helper。
  - 收益来源：降低输出写位流时的基础写入成本。

验证与自测结果：
- 已通过 `code/` 下的 `cargo build --release --lib`。
- `cargo build --release` 的 bin 目标仍有仓库里原有的链接问题，不是这次改动引入的；库目标可成功编译。
- 我额外写了临时基准 [bench_compress.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/brotli/codex_version/work/bench_compress.rs)，并从 `HEAD` 导出 baseline 到 `work/baseline_root/` 做 A/B。
- 本地基准输入约 `8.9 MiB`，调用 `BrotliEncoderCompress`，`quality=11`，循环 `12` 次。
- baseline：`4.544 MiB/s`，`elapsed_secs=22.431150`，`output_checksum=13671721486523537702`
- 优化后：`4.836 MiB/s`，`elapsed_secs=21.075849`，`output_checksum=13671721486523537702`
- 观察到的本地提升约 `+6.4%`，且输出 checksum 一致，说明该基准输入下压缩结果字节级未变。

Agent 自测数据仅供参考；你们拿到这份优化结果后，仍应使用外部后验脚本重新编译并统一测量，并以外部测量结果作为最终性能结论。