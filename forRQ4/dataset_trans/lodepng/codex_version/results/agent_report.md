# lodepng 性能优化报告

## 修改文件

- `code/src/lodepng.rs`

## 改动说明

1. 将项目内部手写的 `lodepng_memcpy` / `lodepng_memset` 替换为 `core::ptr::copy_nonoverlapping` 和 `core::ptr::write_bytes`。
   - 位置：`code/src/lodepng.rs:381-404`
   - 意图：原始 c2rust 代码把最基础的内存操作展开成逐字节循环，影响解码、编码、颜色转换、filter/unfilter、Huffman/块处理等多条热点路径。改为标准原语后，LLVM 更容易生成高效内建/平台优化代码。

2. 为 `uivector` 增加专门的 `uivector_reserve`，并将 `uivector_push_back` 改成“按需 reserve + 直接写入 + 单次更新 size”。
   - 位置：`code/src/lodepng.rs:478-511`
   - 意图：LZ77 编码阶段会频繁 push 单个符号。原实现每次 `push_back` 都走 `uivector_resize`，包含额外的尺寸计算和状态写回。拆开后可降低热路径上的固定开销。

3. 将 `addLengthDistance` 从“先 resize 再写 4 个元素”改成“先 reserve 再批量写入，再一次性更新 size”。
   - 位置：`code/src/lodepng.rs:2406-2439`
   - 意图：match token 是 deflate 编码核心热点之一；这个改动减少了每次 length-distance 发射时的冗余向量管理开销。

4. 将 `hash_init` 中对 `head`、`val`、`headz` 的逐元素 `-1` 初始化改成整块 `memset(0xFF)`。
   - 位置：`code/src/lodepng.rs:2448-2514`
   - 意图：压缩每次初始化哈希链表时都要清空较大数组，这里改成块初始化更高效。

5. 在 `filterScanline` / `unfilterScanline` 以及 `filter` 的部分分支中，用整段 `memcpy` 取代逐字节拷贝。
   - 位置：`code/src/lodepng.rs:9333-9382`, `12799-12847`, `13063-13278`
   - 意图：PNG filter 0、部分首行/无 prevline 场景、本行最佳 filter 回写都属于非常常见路径；用批量拷贝可以减少循环控制和单字节访存开销。

6. 将 `lodepng_decode_memory` / `lodepng_encode_memory` 里的超大 `LodePNGState` 字面量初始化改为 `mem::zeroed()` 后调用现有 `lodepng_state_init`。
   - 位置：`code/src/lodepng.rs:11718-11727`, `14587-14597`
   - 意图：保持现有初始化逻辑不变，但避免每次调用都生成并写入巨大的临时结构体字面量，降低单次 API 调用固定成本。

## 自测结果

自测方式：

- 在 `work/baseline_code/` 保留修改前副本。
- 新建 `work/bench_current/` 和 `work/bench_baseline/`，对同一合成 RGBA 图像分别执行 `lodepng_encode32` 与 `lodepng_decode32`。
- 运行命令：
  - `cd work/bench_current && cargo +nightly-2024-01-15 run --release`
  - `cd work/bench_baseline && cargo +nightly-2024-01-15 run --release`

观察到的数据：

- 第一轮
  - baseline: `512x512 decode 5.795 ms, encode 57.444 ms`
  - current: `512x512 decode 5.175 ms, encode 57.710 ms`
  - baseline: `2048x2048 decode 78.103 ms, encode 866.474 ms`
  - current: `2048x2048 decode 71.854 ms, encode 882.020 ms`

- 第二轮
  - baseline: `512x512 decode 5.888 ms, encode 58.352 ms`
  - current: `512x512 decode 4.782 ms, encode 58.788 ms`
  - baseline: `2048x2048 decode 78.291 ms, encode 886.485 ms`
  - current: `2048x2048 decode 72.034 ms, encode 901.709 ms`

- 最终复测（在撤回一部分不稳的 LZ77 预留策略后）
  - current: `512x512 decode 5.328 ms, encode 58.755 ms`
  - current: `2048x2048 decode 71.457 ms, encode 864.904 ms`

结论：

- 解码侧收益较稳定。
  - 2048x2048 workload 上，相对 baseline 约提升 `8% ~ 9%`。
  - 512x512 workload 上，相对 baseline 约提升 `8% ~ 19%`，但小图波动更大。
- 编码侧结果不如解码稳定。
  - 大图最终复测为 `864.904 ms`，略好于一轮 baseline `866.474 ms`，但不如另一轮 baseline `886.485 ms` 的对比那样有说服力。
  - 小图 encode 基本持平，存在约 `1%` 左右噪声范围内波动。

## 构建验证

- 已执行：`cd code && cargo build --release`
- 结果：编译通过。

## 说明

Agent 自测数据仅供参考；我们一定会在拿到这份优化结果后，使用外部后验脚本重新编译并统一测量，以外部测量结果作为最终性能结论。
