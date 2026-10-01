修改完成，核心变更在 [code/src/lz4.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/lz4/codex_version/code/src/lz4.rs)、[code/src/lz4hc.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/lz4/codex_version/code/src/lz4hc.rs) 和 [code/src/xxhash.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/lz4/codex_version/code/src/xxhash.rs)，报告写在 [results/agent_report.md](/home/anonymous/artifact/PerfTrans/dataset_trans/lz4/codex_version/results/agent_report.md)。

这轮优化的意图主要有三点：一是把 `lz4`/`lz4hc`/`xxhash` 热路径里的端序判断改成编译期常量，减少重复探测；二是把大量固定宽度的小 `memcpy` 替换为直接的非对齐 load/store helper，降低高频外部调用开销；三是在 `LZ4_decompress_unsafe_generic()` 里把一段逐字节 match 复制改成复用 offset-copy 的实现，优化解压时的小 offset 扩展复制路径。预期收益主要来自解压热路径，其次是辅助函数更易被 LLVM 内联。

我已在 `code/` 下执行 `cargo build --release`，构建通过。还做了一次离线参考自测：输入 `9,043,968` bytes，压缩后 `143,580` bytes；`LZ4_compress_default` 连续 200 次总耗时 `206,768,205 ns`，`LZ4_decompress_safe` 连续 400 次总耗时 `312,607,705 ns`，并验证了压缩解压往返结果正确。由于当前工作区里的 `code/` 不是上层 git `HEAD` 可直接抽取的已跟踪基线，本地没能做出可靠的“改前 vs 改后”成对数据，这一点已经在报告中明确说明。

Agent 自测数据仅供参考；你们一定会在拿到本次优化结果后，使用外部后验脚本重新编译并统一测量，并以外部测量结果作为最终性能结论。