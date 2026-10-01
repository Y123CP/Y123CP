# libqrencode 性能优化报告

## 修改文件与意图

1. `code/src/rsecc.rs`
   - 在 `RSECC_init` 中一次性预计算全部 `generator` 表，避免 `RSECC_encode` 每次调用都做一次按 `ecc_length` 的互斥检查和懒初始化。
   - 在 `RSECC_encode` 中去掉每轮数据字节处理后的 `memmove`，改成单次前向扫描时原地写回左移后的余项。
   - 预期收益来源：这是 Reed-Solomon 编码的核心热循环。移除 `memmove` 和重复初始化检查可以直接减少每个输入字节的内存搬运与同步开销。

2. `code/src/mask.rs`
   - 将 `Mask_mask2/3/4` 中每像素重复执行的 `% 3`、`/ 2`、`/ 3` 改成按行递推状态。
   - 将 `Mask_mask5/6/7` 改成使用 6x6 周期表查找，替代每像素的乘法、取模和布尔组合。
   - 将 `Mask_mask` 在挑选最优 mask 时从 `memcpy(bestMask, mask, ...)` 改成双缓冲指针交换，避免每个更优候选触发整块复制。
   - 预期收益来源：自动选 mask 时需要反复生成并评估候选掩码，这一段有大量逐像素运算。减少除法/取模/乘法和复制可以降低该阶段成本。

3. `code/src/mmask.rs`
   - 对 Micro QR 的掩码生成做与 `mask.rs` 相同的周期表/递推化简。
   - 将 `MMask_mask` 的最优候选更新改成双缓冲交换，去掉反复 `free + malloc`。
   - 预期收益来源：Micro QR 自动选 mask 的路径更短，但同样受益于去掉逐像素高成本算术和额外分配。

## 编译验证

- 已在 `code/` 下执行 `cargo build --release`。
- 编译通过；仅保留项目原有 warning，没有新增编译错误。

## 自测结果

测试环境说明：

- 额外创建了 `work/bench_encode` 作为临时 benchmark crate。
- 为了获得 before/after 对比，我复制了一份基线源码到 `work/baseline_code.L0n2Oq/`，并仅对本次三个修改文件反向应用 patch。
- benchmark 使用 `cargo +nightly-2024-01-15 run --release` 运行。

端到端编码 benchmark：

| case | baseline ms | optimized ms | 变化 |
|---|---:|---:|---:|
| short, 2000 iters, len=22 | 118.594 | 95.931 | -19.1% |
| medium, 1500 iters, len=81 | 203.540 | 212.554 | +4.4% |
| long, 800 iters, len=192 | 265.685 | 289.315 | +8.9% |

热路径微基准（仅 `RSECC_encode`）：

| case | baseline ms | optimized ms | 变化 |
|---|---:|---:|---:|
| rsecc, 20000 iters, data=128, ecc=30 | 81.099 | 56.949 | -29.8% |

观察：

- `RSECC_encode` 的微基准有明确收益，说明 `rsecc.rs` 的改动方向是有效的。
- 端到端编码在短输入上也有明显收益。
- 但在这组中长输入样本上，端到端结果出现回退，说明 `mask/mmask` 的改动虽然理论上减少了算术和复制，但在当前编译器与这组输入分布下，没有稳定转化成整体收益，甚至可能引入了额外开销。
- 因此，本次结果应谨慎解读：可以确认 `rsecc` 热循环优化有效；`mask/mmask` 的整体收益仍需外部统一测量验证。

## 备注

Agent 自测数据仅供参考；我们一定会在拿到优化结果后，使用外部后验脚本重新编译并统一测量，以外部测量结果作为最终性能结论。
