# lz4 性能优化报告

## 本次修改

### `code/src/lz4.rs`

- 在热路径辅助函数上增加了更强的内联和编译期端序判定，避免运行时反复执行小函数/端序探测。
  - 位置：`code/src/lz4.rs:152-205`, `code/src/lz4.rs:380-390`, `code/src/lz4.rs:502-510`
- 将固定宽度的 8/16/32 字节复制从 `libc::memcpy` 调用改为直接的非对齐读写 helper，减少热循环中的外部调用开销。
  - 位置：`code/src/lz4.rs:198-379`
- 在 `LZ4_decompress_unsafe_generic()` 中，把一段逐字节 match 复制替换为复用现有 offset-copy 逻辑的 `LZ4_forward_copy_match()`，优化解压热路径中的重复小 offset 扩展复制。
  - 位置：`code/src/lz4.rs:359-379`, `code/src/lz4.rs:2315-2316`

### `code/src/lz4hc.rs`

- 同步把 HC 路径中的端序判断和基础读写 helper 改成编译期判定/更易内联的形式，减少高压缩级别路径中的辅助开销。
  - 位置：`code/src/lz4hc.rs:155-220`, `code/src/lz4hc.rs:396-417`, `code/src/lz4hc.rs:1326-1345`

### `code/src/xxhash.rs`

- 将 `XXH_read32/XXH_read64` 从 `memcpy` 改为 `read_unaligned()`，并把端序检测改为编译期判定，减少 frame/hash 热路径上的小函数与小拷贝开销。
  - 位置：`code/src/xxhash.rs:112-163`, `code/src/xxhash.rs:764-818`

## 收益来源判断

- 主收益预期来自 `lz4` 解压/复制热路径：固定宽度复制改为直接非对齐 load/store，减少超高频 `memcpy` 调用。
- 次收益预期来自编译器更容易内联 `LZ4`/`LZ4HC`/`XXH` 的小 helper，缩短压缩、解压和 frame 校验路径上的分支与调用链。
- `LZ4_forward_copy_match()` 预计对重复数据较多、match 扩展频繁的解压场景更有帮助。

## 自测结果

### 构建验证

- 已在 `code/` 下执行：`cargo build --release`
- 结果：通过

### 参考性能快照

- 使用离线单文件驱动链接当前 `release` 产物，对 `LZ4_compress_default` / `LZ4_decompress_safe` 做往返校验和计时。
- 输入规模：`9,043,968` bytes
- 压缩后大小：`143,580` bytes
- 压缩：`200` 次，总耗时 `206,768,205 ns`
- 解压：`400` 次，总耗时 `312,607,705 ns`
- 同时验证了压缩后再解压的数据与原输入一致。

## 未完成项与限制

- 没有拿到可靠的“改前 vs 改后”本地成对数据。
- 原因：当前工作区中的 `code/` 源码不在上层 git `HEAD` 中，无法从仓库历史直接抽取基线版本；而独立 Cargo benchmark crate 在当前禁网环境下无法解析依赖索引。
- 因此，上述自测数据只是当前优化版本的参考快照，不构成最终性能结论。

## 结论说明

Agent 自测数据仅供参考；你们一定会在拿到本次优化结果后，使用外部后验脚本重新编译并统一测量，并以外部测量结果作为最终性能结论。
