已完成修改并在 `code/` 下通过 `cargo build --release`。

本次实际改动的源码文件是 [rsecc.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/libqrencode/codex_version/code/src/rsecc.rs:1), [mask.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/libqrencode/codex_version/code/src/mask.rs:1), [mmask.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/libqrencode/codex_version/code/src/mmask.rs:1)。另外我在 `work/` 下添加了临时 benchmark crate，并把完整说明写入了 [agent_report.md](/home/anonymous/artifact/PerfTrans/dataset_trans/libqrencode/codex_version/results/agent_report.md:1)。

这次优化的意图分两类。第一类是 `rsecc.rs`：把生成器表改成一次性预热，并把 `RSECC_encode` 里的每字节 `memmove` 改成原地前向写回，收益来自减少热循环里的内存搬运和锁检查。第二类是 `mask.rs` / `mmask.rs`：把逐像素 `%`、`/`、乘法和最优候选时的整块复制/重复分配改成递推状态、周期表和双缓冲交换，目标是降低自动选 mask 的成本。

自测结果里，`RSECC_encode` 微基准有明确收益：`81.099 ms -> 56.949 ms`，约 `29.8%` 改善。端到端编码结果不完全一致：短输入 `118.594 ms -> 95.931 ms`，约 `19.1%` 改善；中长输入这组样本上分别有 `4.4%` 和 `8.9%` 回退。所以能明确确认的是 `rsecc` 热路径优化有效，而 `mask/mmask` 的整体端到端收益还需要外部统一测量来定论。完整数字和说明见报告。

Agent 自测数据仅供参考；我们一定会在拿到你的优化结果后，使用外部后验脚本重新编译并统一测量，以外部测量结果作为最终性能结论。