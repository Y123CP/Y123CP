已完成优化并通过 `code/` 下的 `cargo build --release`。

**改动**
- [code/src/choices.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/fzy/codex_version/code/src/choices.rs:400)
  - 给搜索结果缓冲加入 `capacity`，改成按需扩容，不再为每个 worker 按“全部候选数”预分配结果数组。
  - 多 worker 结果合并改为原地后向 merge，减少额外分配和拷贝。
  - 空查询 `-e ''` 增加直通快路径，直接返回全部候选，避免无意义打分与排序。
  - 将 worker 数限制到有效范围，避免小输入或空输入时创建无效线程。
  - 主要收益来自减少堆分配、降低 merge 成本、缩短空查询路径。

- [code/src/match.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/fzy/codex_version/code/src/match.rs:1)
  - 用 `lower_char` 的 ASCII 快路径替代转译残留的 `toupper/strpbrk` 组合。
  - `has_match` 改为直接顺序扫描，减少函数调用和临时对象。
  - `setup_match_struct` 改为单次遍历完成 lower 和 bonus 预处理，减少重复扫描。
  - `match_positions` 的两块矩阵堆分配合并为一块连续内存。
  - 主要收益来自匹配前处理更轻、DP 定位分配更少。

**自测**
- 报告已写入 [results/agent_report.md](/home/anonymous/artifact/PerfTrans/dataset_trans/fzy/codex_version/results/agent_report.md:1)。
- 使用 `work/bench_input.txt` 上 200000 行合成输入，对比基线副本 `work/baseline/code` 与优化版。
- 输出一致性：
  - `-e module123` diff 一致
  - `-e ''` diff 一致
- 计时观察：
  - 默认 worker，查询 `file`：`0.46s -> 0.41s`，约 `+10.9%`
  - 单 worker，查询 `file`：`2.99s -> 2.23s`，约 `+25.4%`
  - 默认 worker，空查询 `''`：`0.35s -> 0.04s`，约 `+88.6%`
  - 默认 worker，低匹配率 `nomatchzzz`：`0.08s -> 0.10s`，有小幅回退

Agent 自测数据仅供参考；你们在拿到优化结果后，仍应使用外部后验脚本重新编译并统一测量，并以外部测量结果作为最终性能结论。