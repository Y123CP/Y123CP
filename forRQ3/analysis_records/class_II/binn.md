# binn — Class II 差分数据(优化器少做的优化,采集层,不含归类判定)

**采集(修正方法,见 `_method.md`)**:单合并正则 `'loop-vectorize|slp-vectorizer|inline'`;两侧 debug info;两侧 no-LTO 对称——C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -DBINN_NO_COMPRESS -gline-tables-only -c binn.c`(`main` 取 binn_roundtrip.c);Rust = c2rust 库 crate `rust_raw` **`cargo +nightly-2024-01-15`**(rustc 1.77 = LLVM 17,override rust_raw 自带 LLVM16 pin,与 clang-17 对称)`CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all`(passed 由缺失对应 missed 推断)。**后端对称订正**:早期用 rust_raw 自带 LLVM16,本轮强制 LLVM17;结论(无退化)复现。

**热函数账本(复用 Class I)**:
- build_serialize_100k(M1=1.530,gap +40.9%):`AddValue` 86.3% / `main` 12.9%
- iterate_decode_100k(M1=1.450,gap +27.9%):`main` 55.4% / `GetValue` 29.5% / `AdvanceDataPos` 14.4%

---

## II① 向量化损失 —— 无(两侧对称)

| pass | Rust(热/库) | C(热/库) |
|---|---|---|
| loop-vectorize passed | 0 / 0 | 0 / 0 |
| slp-vectorizer passed | 0 / 1(L342 非热) | 0 / 5(binn.c:293 非热) |

两侧均不向量化任何热路径循环。热函数内 Rust 6 条 SLP remark 全是 `not beneficial cost 0>=0`(向量器**主动放弃** cost-中性的 2-store 相邻字节写,`AdvanceDataPos` L618/L638、`GetValue` L1723),与 C 行为同性质,非损失。binn = typed 序列化 + 标量 byte-swap 拷贝,无数据并行循环。**delta = 0。**

## II② 内联损失 —— 无(对称)

- **热函数内联对称**:两侧都吸收同一批小 helper(`copy_be16/32/64`、`compress_int`、`strlen2`、`CheckAllocation`、`binn_get_type_info`、`IsValidBinnHeader`),Rust 侧这些已被 MIR/LLVM 折叠进热体(无 un-inline remark)。
- **唯一 caller-side 非对称:反而对 Rust 有利**——`type_family → AddValue`(binn.c:959 / binn.rs:1079):LLVM17 下 **C 仍 miss(cost=450>250)、Rust 内联成功**(零 inline-missed 行)。非 Rust 退化。
- 热函数作为 callee 不内联进各自 wrapper 是**共享决策**(cost 两侧几乎相同:`AddValue` 1040/1170、`GetValue` 525/690、`AdvanceDataPos` 270/270,均 ≫ thr),非 Rust artifact。
- **纠正早期 fat-LTO 假象**:此前在 fat-LTO harness 上手采得 `copy_value` 未内联(cost 超阈值)= ●。公平 no-LTO 库 crate 下,`copy_value` 与 4 个热函数**无任何内联关系**(无 caller/callee remark 连接)——那是 fat-LTO harness 采集口径伪影,不是热路径内联损失。

## 与 Class I 的交叉

binn 无 Class II 缺口。其 +40.9%/+27.9% 的 gap 由 **Class I 驱动**:C1(序列化/解码循环里密集的 Option 拆包 `main` 22 + `AddValue` 2)+ C3(全热函数 `!tbaa`=0,`main` 达 230)。Class II 通道对 binn **不解释 gap**。

## 小结

binn = **Class II 无退化**(向量化 delta=0、内联对称)。修正方法(flag-fixed + no-LTO + debuginfo 对称)证明 c2rust 输出在 binn 热路径的向量化与内联行为**与 C 参考一致**;binn 的退化纯属 Class I(C1+C3)。这也是一个方法学教训:fat-LTO harness 采集会伪造内联损失(`copy_value`),必须用 no-LTO 库 crate 对称复核。
