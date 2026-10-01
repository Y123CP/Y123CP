# libcsv — Class II 差分数据(优化器少做的优化,采集层,不含归类判定)

**采集(修正方法,见 `_method.md`)**:单合并正则 `'loop-vectorize|slp-vectorizer|inline'`;两侧 debug info;两侧 no-LTO 对称——C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -gline-tables-only -c libcsv.c`(排除 main.c harness,LLVM 17.0.6);Rust = c2rust 库 crate `rust_raw` **`cargo +nightly-2024-01-15`**(rustc 1.77 = LLVM 17,与 clang-17 对称)`CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all`(rust_raw/Cargo.toml 无 op_write feature,不加 --features)。⚠️ **后端对称订正**:早期复核误用 rust_raw 自带 `nightly-2023-04-15`(LLVM 16),本轮强制 +nightly-2024-01-15 使 C/Rust 同 LLVM 17;rustc 1.77 正常发 inline/向量化 remark,无需 objdump fallback。

**热函数账本(复用 Class I)**:
- parse(M1=1.267,gap +35.1%):`csv_parse` 83.9% / `cb_field` 5.2%(大部内联进 csv_parse)
- write(M1=0.994,gap +8.3%):`csv_count::main`(含内联的 `csv_write`)97.1%

---

## II① 向量化损失 —— 无(两侧对称)

| pass | C passed / missed | Rust passed / missed |
|---|---|---|
| slp-vectorizer | 0 / 31 | 0 / 46 |
| loop-vectorize | 0 / 7 | 0 / 17 |

两侧均不向量化任何热循环。关键:`csv_parse` 的主字节循环在**两侧都不是向量化候选**(循环内无 `loop not vectorized`,只有 SLP 头部 no-op)——它是 byte-at-a-time 的 `switch` 状态机。`loop not vectorized` remark 落在 `csv_increase_buffer` 的 realloc-retry 循环、`csv_write2`/`csv_fwrite2` 的逐字符转义循环——**两侧同样固有不可向量化**(数据依赖控制流、trip-count 未知)。SLP miss 全是 `not beneficial cost 0>=0` / `Cannot SLP (impossible)`,对称。

## II② 内联损失 —— 无(LLVM 17 对称,remark 确认)

C 4 处 inline-passed 在 Rust(LLVM 17)侧**全部也 success**,cost/threshold 量级相当:

| callee → caller | C | Rust |
|---|---|---|
| `csv_increase_buffer → csv_parse`(站点1) | success cost=110/250 | success cost=165/250 |
| `csv_increase_buffer → csv_parse`(站点2) | success cost=−14890 | success cost=−14835 |
| `csv_write2 → csv_write` | success 115/250 | success 115/250 |
| `csv_fwrite2 → csv_fwrite` | success 220/250 | success 220/250 |

Rust 侧唯一 inline-missed 进热函数的是 `__assert_fail`/`expect_failed`(panic 冷路径,外部符号,C 侧 `__assert_fail`/`fputc` 同样外部不可内联,对称良性,按过滤规则排除)。即 C 内联进热函数的每个库内部 callee,Rust 也全内联(LLVM 17 下由 LLVM 内联器显式内联,发 success remark;非早期误以为的 MIR 提前折叠)——**"是否内联"无 gap**。

## 与 Class I 的交叉

libcsv 无 Class II 缺口。其 +35.1%/+8.3% 的 gap 由 **Class I 驱动**:C1(`csv_parse` Option 拆包 4)+ C3(`csv_parse` `!tbaa`=0 vs C 103、write main 0 vs 85)。Class II 通道对 libcsv **不解释 gap**。

## 小结

libcsv = **Class II 无退化**(向量化 0/0 对称、内联 remark 对称)。byte-at-a-time 状态机 parser 无数据并行热循环,c2rust 输出的向量化/内联行为与 C 参考一致;退化纯属 Class I(C1+C3)。结论在**后端对称(LLVM17=LLVM17)**下复现,LLVM16 旧结论一致。
