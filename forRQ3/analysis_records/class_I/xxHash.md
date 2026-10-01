# xxHash — Class I 差分对比数据(采集层,不含归类判定)

**构建**:C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -DXXH_VECTOR=0 -DXXH_STATIC_LINKING_ONLY -g -Wl,--plugin-opt=save-temps`(`xxh_bench.c xxhash.c`)→ precodegen → `llvm-dis-17`;Rust = 实测二进制的 fat-LTO 全程序模块(`rust_harness/target/release/deps/xxh_bench-*.ll`,`cargo build --release` + `-Cdebuginfo=1 --emit=llvm-ir`,nightly-2024-01-15 = LLVM 17.0.6)。两侧均 `XXH_VECTOR=0`(纯标量)。

**归属**:按热函数 `define` 块统计,义务/对照构造按 `DILocation` scope 分 自身 / 内联继承 / nodbg。**C 侧 `XXH3_hashLong_internal_loop` 经 LTO 内联于 `main`,无独立定义**。

## Workload: XXH3  (M1=1.270;RQ1 gap = +33.1% → **回归**)

**① 账本**(64 KiB × 50000):

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `xxhash::XXH3_hashLong_internal_loop` | 95.9 | ✓ |

## Workload: XXH32  (M1=0.475;RQ1 gap = +62.2% → **回归**)

**① 账本**:`xxh_bench::main`(XXH32 逻辑内联其中)99.4%。

**② 每个热函数的优化后 IR 构造计数**:

| 函数 | 义务/对照构造(Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `XXH3_hashLong_internal_loop` | panic nodbg 1 | 32 / 24 / 40 / **0** | 内联于 main |
| `xxh_bench::main`(XXH32) | unwind nodbg 18;Option 拆包 nodbg 5;overflow 1;memcpy(对照) | 57 / 69 / 78 / **0** | 37 / 101 / 133 / **113** |

**③ 指令区域大小(Rust/C 行)与开放扫描**:

| 函数 | Rust 行 | C 行 | 开放扫描(Rust-only 符号) |
|---|---:|---:|---|
| `XXH3_hashLong_internal_loop` | 262 | 内联于 main | `xxhash_raw::...` ×2(库内部) |
| `xxh_bench::main`(XXH32) | 933 | 2108 | `llvm.assume` ×10、`std::io::stdio` ×6、`alloc::*`、`llvm.experimental.noalias.scope.decl` ×4(harness 框架) |

注:main Rust 933 < C 2108(静态显著少);Rust-only 符号均为 harness main 的 std/alloc 框架,非被翻译库差分。这与 XXH32 M1=0.475(Rust 指令反少)一致——其退化在 CPI(§ 小结),非静态代码量。

**④ 查法2:优化成功痕迹(C 有 / Rust 少)**:

| 函数 | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `XXH3_hashLong_internal_loop` | 0 / 内联 | 0 / 内联 | 3 / 内联 |
| `xxh_bench::main`(XXH32/XXH3) | 0 / **5** | **20 / 213** | 49 / 63 |

**关键差分**:尽管两侧源码 `XXH_VECTOR=0`(强制标量),**C 编译器仍对哈希循环 auto-vectorize 出 213 个向量 op + 5 处 bswap,Rust 仅 20 向量、0 bswap**——C 自动向量化成功、Rust 没有。这是 xxHash 退化的重要机制(C SIMD 吞吐 vs Rust 标量),属**向量化损失(II①)**,补充下方 C3/CPI 视角。

---

**采集层小结(不含归类,仅记差分事实)**:
- **C3 冗余访存 / 别名缺失**:两个热函数 Rust `!tbaa` 恒为 0,C 侧 main 达 113。
- **C1 冗余检查分支**:热路径义务符号极少(XXH3_hashLong 仅 1 处 panic;XXH32 的 unwind/expect 均在 harness main 层)——哈希内核几乎无前端多生成。
- **C2 饱和 cast**:无浮点,`fptosi.sat` 为 0(不适用)。
- **XXH32 的关键特殊性(诚实标注)**:**M1=0.475——Rust 指令数反而只有 C 的一半**,这不是"前端多生成"(Class I 的前提),Class I 结构性照不到。其 +62.2% 退化完全来自 CPI(RQ2 M2=3.42,ΔCPI 由 Mem+0.50 / Core+0.48 双瓶颈主导),属纯执行效率损失,不在 Class I 范围,由 RQ2 的 CPI 分解刻画。XXH3(M1=1.27)则是工作量膨胀 + C3。
- **II① 向量化损失(查法2 新发现,比"纯 CPI"更具体的机制)**:C 侧 main auto-vectorize 出 **213** 个向量 op(Rust 仅 **20**),XXH32/XXH3 的 CPI 劣势很大程度源于 **C 自动向量化成功、Rust 保持标量**。这解释了为何 XXH32 指令反少(M1<1)却慢——C 用少量宽向量指令跑赢 Rust 的标量流。
