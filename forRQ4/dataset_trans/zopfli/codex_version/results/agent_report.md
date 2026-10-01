# zopfli 性能优化报告

## 修改文件

- `code/src/lz77.rs`
  - 为 `ZopfliStoreLitLenDist` 增加了通用扩容辅助函数 `GrowIfNeeded`。
  - 将原先一次追加 token 时对 `litlens`、`dists`、`pos`、`ll_symbol`、`d_symbol` 五个数组分别重复判断容量、反复改写 `store.size` 的实现，改为：
    - 预先判断一次是否需要扩容。
    - 如需扩容，则对五个数组各执行一次扩容。
    - 长度/距离符号各只计算一次。
    - 最后顺序写入五个数组并一次性更新 `store.size`。
  - 将 `ZopfliCopyLZ77Store` 中的逐元素复制替换为 `copy_nonoverlapping` 批量复制，减少 store 拷贝成本。

## 改动意图与潜在收益来源

- `ZopfliStoreLitLenDist` 是 LZ77 贪心路径和最优解析路径都会高频调用的热点函数。原始 c2rust 转译版本存在明显的机械式低效：
  - 同一次追加操作中，对相同的 `origsize` 重复做 5 组完全相同的容量检查。
  - 通过多次把 `store.size` 改成 `origsize` 再加回去来驱动写入，产生多余的读写和依赖链。
  - 对匹配项的长度符号和距离符号重复调用计算函数，随后又重复用于计数更新。
- 这些开销虽然单次不大，但会在整个压缩过程中按 token 数量累计，因此属于典型的“热点小开销放大”。
- `ZopfliCopyLZ77Store` 在迭代优化阶段也会多次触发，改成批量复制可以降低大 store 复制成本。

## 编译验证

- 已执行：`cargo build --release`（目录：`code/`）
- 结果：编译通过。

## 自测方法

- 额外编写了 `work/bench.c`，以 C FFI 方式直接调用 `ZopfliInitOptions` 与 `ZopfliCompress`。
- 为了做前后对比，将基线版本通过 `git archive HEAD` 导出到 `work/baseline_code_2/` 并单独编译。
- 使用同一份 `bench.c` 分别链接：
  - 优化版：`code/target/release/libzopfli_raw.a`
  - 基线版：`work/baseline_code_2/target/release/libzopfli_raw.a`
- 测试输入为基准程序内部生成的混合重复文本样本。
- 大输入版本曾在基线和优化版上都以 `139` 崩溃，因此未采用该组数据；最终保留了一组规模较小、可稳定完成的对比结果。

## 自测观察结果

- 优化版：`input=621780 rounds=1 avg_ms=4552.336 out=21652 hash=86b32f57658aabfa`
- 基线版：`input=621780 rounds=1 avg_ms=4728.165 out=21652 hash=86b32f57658aabfa`
- 观察：
  - 输出大小一致。
  - 输出哈希一致。
  - 该样本下优化版耗时约下降 `3.72%`。

## 说明

- 本次仅修改了 `code/` 下源码，未修改 `Cargo.toml` 的 release profile，也未修改 `.cargo/config.toml`。
- 对外 API 和导出函数签名未变。
- Agent 自测数据仅供参考；我们一定会在拿到你的优化结果后，使用外部后验脚本重新编译并统一测量，以外部测量结果作为最终性能结论。
