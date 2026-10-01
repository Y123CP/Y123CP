# fzy 性能优化报告

## 改动文件

### `code/src/choices.rs`

- 在 [`choices_search`](../code/src/choices.rs) 中为空查询增加直通快路径，并为空输入增加早返回，避免原先仍然创建 worker、做无意义筛选和排序。
- 给 `search_job` 和 `result_list` 增加显式 `worker_count` / `capacity` 字段，避免搜索时按“全部候选数”给每个 worker 预分配结果缓冲。
- 新增 `result_list_reserve` / `result_list_push`，把 worker 结果收集改成按需扩容，而不是 `size * worker_count` 级别的超量分配。
- 将多 worker 结果合并 `merge2` 改为在第一份结果缓冲上原地后向合并，避免每轮 merge 再分配一个新数组并复制两边结果。
- 将 worker 数量约束为 `1..=choices.size`，避免候选数很少或为空时创建无效线程。

预期收益来源：

- 显著减少搜索阶段的堆分配量和内存写放大。
- 降低多线程 merge 的额外分配/拷贝成本。
- 空查询直接返回全部候选，避免整个匹配流水线。

### `code/src/match.rs`

- 用 `lower_char` 替换转译出来的 `toupper + strpbrk` 组合；对 ASCII 走常量时间快路径，非 ASCII 仍保留 `ctype` fallback。
- 将 `has_match` 改成直接顺序扫描，减少函数调用和临时 accept 数组构造。
- 将 `setup_match_struct` 从 “`strlen` + 两次逐字符 lower + 一次 bonus 预处理” 改为单次遍历填充，减少字符串重复扫描。
- 将 `match_positions` 的两块独立矩阵分配改为一块连续内存，减少一次 `malloc/free`。

预期收益来源：

- 降低每个候选项在 `has_match` 与 `match` 前处理中的逐字符开销。
- 减少 DP 定位路径上的堆分配次数和缓存失配。

## 自测

测试输入：

- 生成了 `work/bench_input.txt`，包含 200000 行形如 `src/module000123/example_file_000123_test_case.txt` 的路径字符串。
- 使用 `cargo build --release` 分别构建基线副本 `work/baseline/code` 与优化后版本 `code`。

行为一致性检查：

- `-e module123` 输出逐行 diff 一致。
- `-e ''` 输出逐行 diff 一致。

基准观察（`--benchmark=20`，单位为 wall clock `real`）：

- 默认 worker，查询 `file`
  - 基线：`0.46s`
  - 优化后：`0.41s`
  - 约提升：`10.9%`
- 单 worker，查询 `file`
  - 基线：`2.99s`
  - 优化后：`2.23s`
  - 约提升：`25.4%`
- 默认 worker，空查询 `''`
  - 基线：`0.35s`
  - 优化后：`0.04s`
  - 约提升：`88.6%`
- 默认 worker，低匹配率查询 `nomatchzzz`
  - 基线：`0.08s`
  - 优化后：`0.10s`
  - 该场景有轻微回退

## 结论

- 这次优化主要改善了空查询、候选结果很多、以及单线程或高匹配率搜索下的分配与合并成本。
- 对低匹配率查询存在小幅回退，原因大概率是 `has_match` 改为 Rust/C 指针循环后，在“很快失败”的场景下没有完全吃到 libc 搜索实现的优势。

## 说明

Agent 自测数据仅供参考；拿到本次优化结果后，仍应以外部后验脚本重新编译并统一测量的结果，作为最终性能结论。
