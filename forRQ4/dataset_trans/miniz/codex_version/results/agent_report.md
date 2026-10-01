# miniz 性能优化报告

## 修改文件

### `code/src/miniz_tinfl.rs`

- 在解压的 LZ 回拷热点中引入 `tinfl_copy_match_fast()`。
- 意图：
  - 替换原先按 3 字节展开、再处理尾字节的循环复制。
  - 对 `dist == 1` 的常见 RLE 场景直接使用 `write_bytes`。
  - 对不重叠或 `dist >= len` 的场景使用一次 `copy_nonoverlapping`。
  - 对允许“边写边读”的重叠场景，先复制一个周期，再按已经写出的内容指数扩张复制，保持 deflate 回引用语义不变。
- 预期收益来源：
  - 减少热点循环中的分支和逐字节指令数。
  - 让长匹配回拷更多落到编译器/LLVM 能识别的批量内存复制路径上。
  - 特别改善重复字节或短距离重复模式的解压吞吐。

### `work/bench/Cargo.toml`

- 新增本地基准工程，用于调用库的压缩/解压入口。

### `work/bench/src/main.rs`

- 新增一个自包含基准：
  - 构造约 13.57 MiB 的混合输入（结构化文本、伪随机字节、长重复段）。
  - 调用 `tdefl_compress_mem_to_mem()` 和 `tinfl_decompress_mem_to_mem()`。
  - 先做一次 round-trip 正确性校验，再测吞吐。

### `work/bench/rust-toolchain.toml`

- 让基准工程与 `code/` 使用相同 nightly 工具链，避免路径依赖在 stable 下因 `#![feature(...)]` 失败。

## 自测结果

### 构建确认

- 执行：`cargo build --release`（目录：`code/`）
- 结果：通过。

### 基准输入

- 输入大小：`14,227,072` bytes
- 压缩后大小：`2,299,870` bytes
- round-trip 校验：通过

### 同口径 A/B（同一 `work/bench`，同机本地测量）

- 基线版本：
  - `compress_total_ms=3256.379`
  - `compress_mb_s=49.999`
  - `decompress_total_ms=1073.046`
  - `decompress_mb_s=758.663`
- 最终优化版本：
  - `compress_total_ms=3287.660`
  - `compress_mb_s=49.523`
  - `decompress_total_ms=770.473`
  - `decompress_mb_s=1056.597`

### 观察

- 解压吞吐从 `758.663 MB/s` 提升到 `1056.597 MB/s`，约 `+39.3%`。
- 压缩吞吐从 `49.999 MB/s` 变为 `49.523 MB/s`，约 `-1.0%`，基本可视为持平但略有回落。
- 我尝试过同时优化压缩端 `tdefl_find_match()` 的逐字节匹配，但在这组输入上会让压缩吞吐进一步下降，因此该改动已撤回，没有保留到最终结果中。

## 说明

- Agent 自测数据仅供参考；我们一定会在拿到你的优化结果后，使用外部后验脚本重新编译并统一测量，以外部测量结果作为最终性能结论。
