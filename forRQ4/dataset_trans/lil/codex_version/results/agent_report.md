# lil 性能优化报告

## 修改文件

- `code/src/lil.rs`
  - 为解释器内部命令表、变量表、哈希桶增加容量字段，避免每次插入都按 `+1` 触发 `realloc`。
  - 给哈希桶查找增加 `hm_find_entry`，命中后把条目前移到桶头，降低重复命令名/变量名查找成本。
  - 将 `lil_list_to_value` 从逐字符/逐片段反复 `realloc` 改为“两遍法”：先计算总长度，再一次性分配并填充。
  - 缓存 `needs_escape` 中的 `__ctype_b_loc()` 结果，减少分类表重复取址。
  - 新增 `alloc_value_buffer` 和 `join_argv_with_spaces`，把 `quote`、`write`、`eval`、`expr` 的参数拼接改成一次分配，降低短字符串热点路径中的分配次数。

- `code/src/main.rs`
  - 将 `system` 命令的命令行拼接改为预计算总长度后一次性 `malloc`，替代原来的多次 `realloc`。
  - 修复 `readline` 扩容条件反向的问题；原逻辑几乎每读一个字符就会 `realloc`，现在只在缓冲区将满时扩容。

- `work/bench_quote_expr.lil`
- `work/bench_list_to_value.lil`
- `work/bench_funcs.lil`
- `work/run_bench.sh`
  - 临时自测脚本，仅用于本次 Agent 本地粗测，不参与正式交付逻辑。

## 收益来源判断

- 解释器里大量路径都在构造临时 `lil_value_t` 字符串；把多次小 `realloc` 改成一次分配，理论上能减少分配器开销、拷贝次数和缓存失效。
- 变量/命令查找是解释器热路径；哈希桶命中前移和容器几何扩容，主要减少重复脚本执行时的查找与增长开销。
- `readline` 的修复属于明确的性能 bug 修复；长输入行场景下会显著减少无效重分配。

## 构建验证

- 已在 `code/` 下执行 `cargo build --release`，构建通过。

## Agent 自测

### 功能烟测

- `work/bench_quote_expr.lil`：修改前后输出均为 `done`
- `work/bench_list_to_value.lil`：修改前后输出均为 `159`
- `work/bench_funcs.lil`：修改前后输出均为 `done`

### 粗测方法

- 直接运行单次脚本时，耗时低于 `/usr/bin/time` 的 `0.01s` 分辨率，因此改为“同一脚本重复执行 200 次”测总耗时。
- 这组数据噪声较大，只能作为方向性参考。

### 粗测结果

- `bench_quote_expr.lil` x200
  - 修改前：`0.51s`
  - 修改后：`0.54s`

- `bench_funcs.lil` x200
  - 修改前：`0.55s`
  - 修改后：`0.53s`

- `bench_list_to_value.lil` x200
  - 修改前：`0.54s`
  - 修改后：`0.53s`

### 自测结论

- 本地粗测显示：`list -> value` 和函数/命令相关路径存在轻微改善；`quote/expr` 路径在这组轻量脚本上没有稳定拉开差距，仍处于噪声范围。
- 考虑到本项目整体执行时间较短、脚本级测量粒度有限，本次 Agent 自测数据只能说明“修改没有引入明显功能回归，且部分热点存在小幅正向信号”。

## 说明

Agent 自测数据仅供参考；拿到这份优化结果后，请务必使用外部后验脚本重新编译并统一测量，并以外部测量结果作为最终性能结论。
