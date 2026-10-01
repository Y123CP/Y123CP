# bzip2 — Class I 差分对比数据(采集层,不含归类判定)

**构建**:C = `clang-17` (LLVM 17.0.6, `-O3 -flto -march=native -mno-avx512f`,LTO 后整程序模块;加 `-g` 取 debug info)→ `--plugin-opt=save-temps` 得 `*.0.5.precodegen.bc` → `llvm-dis-17`;Rust = **实测二进制本身的 fat-LTO 全程序模块**(`empirical_study/bzip2/rust_raw/target/release/deps/bzip2-*.ll`,复刻 `study.toml` 的 `cargo +nightly-2024-01-15 build --release` 后加 `-Cdebuginfo=1 --emit=llvm-ir`,nightly-2024-01-15 = LLVM 17.0.6)。两侧同一优化器、都是 LTO 后整程序形态 ⇒ IR 差异只能来自 c2rust 的代码生成。

**归属**:以每个热函数的 `define` 块(= 该函数经 LTO 后的代码区域,含内联进来的被调)为单位统计。义务/对照构造再按其指令的 `DILocation` scope 归到**定义它的源函数**:**自身** = 定义者即本热函数;**内联继承** = 来自被内联进来的具名源函数;**nodbg** = 该构造行无 `!dbg`(多为冷 panic 块的 `call`,无法判定,计入本函数区域)。

**账本**取 c2rust_raw 侧 self%(解释对象是 Rust 的额外耗时),self-time 降序累计 ≥80% 且余下无 ≥5% 函数方停;证据站点须落在 ≥5% 单函数内、且其 workload 经 RQ1 判为回归。

## Workload: decompress  (M1(动态指令比)=1.348;RQ1 墙钟 gap = +15.5% → **回归**)

**① 对比哪些函数** — self-time 降序(perf record,钉核干净环境,silesia-mozilla 输入),账本覆盖 **98.8%**:

| 函数 (perf符号) | self% | ≥5%门槛 |
|---|---:|:-:|
| `BZ2_decompress` | 61.21 | ✓ |
| `BZ2_bzDecompress` | 37.58 | ✓ |

**② 每个函数的优化后 IR 构造计数**:

### `BZ2_decompress` (self 61.21%)

Rust IR:独立定义,7746 条指令区域;C IR:独立定义,9642 条指令区域。

| 构造 | Rust 自身 | Rust 内联继承 / nodbg | C 自身 | C 内联继承 |
|---|---:|---|---:|---|
| 边界检查 (panic_bounds_check) | 36 | makeMaps_d 1;nodbg 37 | 0 | 0 |
| Option 拆包 (expect_failed) | 0 | malloc_fn 3;nodbg 3 | 0 | 0 |
| memset (对照) | 4 | 0 | 3 | BZ2_hbCreateDecodeTables 3 |

**访存 / 别名维度**(代码区域内计数。Rust 侧 `!tbaa` 恒为 0——rustc 不发类型别名信息,优化器无法证明不别名 ⇒ 跨迭代 load 无法消除、循环不变 gep 无法外提;C 侧带 `!tbaa` 的访存可被 GVN/LICM 清掉):

| 指标 | Rust | C |
|---|---:|---:|
| load | 1010 | 755 |
| store | 620 | 636 |
| getelementptr | 906 | 926 |
| 带 `!tbaa` 的访存 | 0 | 1394 |

### `BZ2_bzDecompress` (self 37.58%)

Rust IR:独立定义,1620 条指令区域;C IR:**无独立定义**(经 LTO 内联于 `BZ2_bzRead`)。

| 构造 | Rust 自身 | Rust 内联继承 / nodbg | C |
|---|---:|---|---:|
| 边界检查 (panic_bounds_check) | 0 | unRLE_obuf_to_output_FAST 5, unRLE_obuf_to_output_SMALL 5;nodbg 10 | 0 |
| Option 拆包 (expect_failed) | 0 | malloc_fn 1 + 4;nodbg 5 | 0 |

**访存 / 别名维度**:

| 指标 | Rust | C(内联于 bzRead) |
|---|---:|---:|
| load | 187 | — |
| store | 117 | — |
| getelementptr | 115 | — |
| 带 `!tbaa` 的访存 | 0 | — |

**③ 开放扫描(Rust-only 符号)**:`BZ2_decompress` 无;`BZ2_bzDecompress` 仅 `bzip2_1_0_8_raw::...` ×3(库内部引用)。bzip2 是 self-contained bin(rust_raw 直接产二进制,无独立 harness crate),故无 std::io/alloc 框架符号污染。指令区域:`BZ2_decompress` Rust 7746 < C 9642、`BZ2_bzDecompress` Rust 1620(C 内联于 bzRead)——静态代码量 Rust ≤ C,膨胀在动态执行。

**④ 查法2:优化成功痕迹(C 有 / Rust 少)**:

| 函数 | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `BZ2_decompress` | 0 / 0 | 384 / 267 | 57 / 3 |
| `BZ2_bzDecompress` | 0 / 内联 | 0 / 内联 | 19 / 内联 |

注:bswap 两侧 0(字节序未折叠,非差分);**Rust 向量 384 > C 267**(Rust 向量化反而更多,不构成"C 有 Rust 少"的 gap 来源);call Rust 57 ≫ C 3(Rust 残余调用多、内联更少 → II② 候选,留 Class II remark 判)。

---

**采集层小结(不含归类,仅记差分事实)**:
- **C1 冗余检查分支**:`BZ2_decompress` 边界检查 Rust 74(self 36 + 继承 1 + nodbg 37)vs C **0**;Option 拆包(`malloc_fn`)Rust 6 vs C 0;`BZ2_bzDecompress` 边界检查 Rust 10(继承自 unRLE_obuf_to_output_*)vs C 0(C 侧内联进 bzRead,无边界检查)。
- **C3 冗余访存 / 别名缺失**:`BZ2_decompress` `!tbaa` Rust **0** vs C **1394**;load Rust 1010 > C 755。
- **C2 饱和 cast**:bzip2 无浮点热路径,两侧 `fptosi.sat` 计数为 0(不适用)。
