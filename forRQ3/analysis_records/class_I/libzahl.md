# libzahl — Class I 差分对比数据(采集层,不含归类判定)

**构建**:C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -g -Wl,--plugin-opt=save-temps`(LTO 后整程序,`libzahl_opsuite.c src/*.c`)→ `*.precodegen.bc` → `llvm-dis-17`;Rust = 实测二进制的 fat-LTO 全程序模块(`rust_harness/target/release/deps/libzahl_opsuite-*.ll`,复刻 `cargo build --release` + `-Cdebuginfo=1 --emit=llvm-ir`,rust-toolchain = nightly-2024-01-15 = LLVM 17.0.6)。两侧同优化器、同整程序形态 ⇒ IR 差异只能来自 c2rust 代码生成。

**归属**:同 bzip2——按热函数 `define` 块统计,义务/对照构造按 `DILocation` scope 分 自身 / 内联继承 / nodbg。libzahl 热点分散(大数运算 + 内存管理),故账本含多个 ≥5% 函数。

**账本**取 c2rust_raw 侧 self%,降序累计 ≥80% 且余下无 ≥5% 函数方停;证据站点须 ≥5% self ∧ 所在 workload 经 RQ1 判为回归。

## Workload: mul_4096bit  (M1=1.231;RQ1 gap = +37.3% → **回归**)

**① 账本**(perf record,钉核干净环境,pairs_4096_seedB.bin),累计 ≈80%:

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `zmul_ll` | 16.9 | ✓ |
| `libzahl_realloc` | 16.9 | ✓ |
| `zadd_unsigned_assign` | 16.6 | ✓ |
| `zfree` | 11.4 | ✓ |
| `zrsh` | 10.2 | ✓ |
| `zlsh` | 7.9 | ✓ |

## Workload: modpow_4096bit  (M1=1.123;RQ1 gap = +10.2% → **回归**)

**① 账本**,累计 ≈81%:

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `libzahl_zsub_unsigned` | 38.3 | ✓ |
| `libzahl_realloc` | 9.8 | ✓ |
| `zrsh` | 9.1 | ✓ |
| `zadd_unsigned_assign` | 8.6 | ✓ |
| `zfree` | 8.0 | ✓ |
| `zmul_ll` | 6.8 | ✓ |

**② 每个热函数的优化后 IR 构造计数**(两 workload 共享,数据取自全程序模块):

| 函数 | 义务/对照构造(Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `zmul_ll` | memset nodbg 4 | 23 / 13 / 16 / **0** | 64 / 62 / 56 / **126** |
| `libzahl_realloc` | bounds_check self 1 + 继承 zfree 1 + nodbg 2 | 43 / 35 / 53 / **0** | 34 / 31 / 50 / **65** |
| `zadd_unsigned_assign` | overflow_check 继承 overflowing_add 1 | 15 / 8 / 16 / **0** | 13 / 4 / 12 / **17** |
| `zfree` | bounds_check self 1 + nodbg 1 | 9 / 7 / 7 / **0** | (内联,无独立定义) |
| `zrsh` | 八类全零 | **73** / 56 / 103 / **0** | **24** / 16 / 50 / **40** |
| `zlsh` | memset 继承 libzahl_memset_precise 2 | 64 / 58 / 105 / **0** | 34 / 31 / 82 / **67** |
| `libzahl_zsub_unsigned` | 八类全零 | 51 / 16 / 38 / **0** | 32 / 10 / 26 / **42** |

**③ 指令区域大小(Rust/C 行)与开放扫描**:

| 函数 | Rust 行 | C 行 | 开放扫描(Rust-only 符号) |
|---|---:|---:|---|
| `zmul_ll` | 218 | 658 | 库内部引用 |
| `libzahl_realloc` | 327 | 294 | — |
| `zadd_unsigned_assign` | 156 | 189 | — |
| `zfree` | 70 | 内联 | — |
| `zrsh` | 547 | 396 | — |
| `zlsh` | 537 | 555 | — |
| `libzahl_zsub_unsigned` | 411 | 372 | `libzahl_raw::...` ×8(库内部) |

注:`zmul_ll` Rust 静态 218 << C 658(Rust 代码量反少 3×);多数函数 Rust ≈ C——**膨胀在动态执行,非静态代码量**。无 harness Rust-only 框架符号(libzahl 无 std::io/alloc 密集热点)。

**④ 查法2:优化成功痕迹(C 有 / Rust 少)**:

| 函数 | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `zmul_ll` | 0 / 0 | 0 / 0 | 28 / 40 |
| `zrsh` | 0 / 0 | 6 / 4 | 2 / 3 |
| `libzahl_zsub_unsigned` | 0 / 0 | 0 / 0 | 8 / 11 |

注:bswap 全 0;向量两侧都极少(大数运算逐 limb 标量,无向量化空间);call C 略多。查法2 无"C 有 Rust 少"差分——libzahl 的 gap 全在查法1 的 C3(冗余访存)。

---

**采集层小结(不含归类,仅记差分事实)**:
- **C3 冗余访存 / 别名缺失(主线)**:**全部 7 个热函数 Rust `!tbaa` 恒为 0,C 侧数十至上百**(zmul_ll 0 vs 126、zlsh 0 vs 67、libzahl_realloc 0 vs 65);裸指针游标致冗余 load 最明显的是 **`zrsh` Rust load 73 vs C 24(≈3×)**、`libzahl_zsub_unsigned` 51 vs 32。这与 libzahl 的大数运算逐 limb 裸指针访问一致。
- **C1 冗余检查分支(弱)**:仅内存管理函数零星命中——`libzahl_realloc` 4 处边界检查、`zfree` 2 处;`zadd_unsigned_assign` 的 overflow_check 两侧皆有(Rust 继承 overflowing_add、C 继承 zadd_impl),非差分。
- **C2 饱和 cast**:libzahl 无浮点,两侧 `fptosi.sat` 为 0(不适用)。
