# libcsv — Class I 差分对比数据(采集层,不含归类判定)

**构建**:C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -g -Wl,--plugin-opt=save-temps`(`main.c libcsv.c`)→ precodegen → `llvm-dis-17`;Rust = 实测二进制的 fat-LTO 全程序模块(`rust_harness/target/release/deps/csv_count-*.ll`,`cargo build --release --features op_write` + `-Cdebuginfo=1 --emit=llvm-ir`,nightly-2024-01-15 = LLVM 17.0.6)。

**归属**:按热函数 `define` 块统计,义务/对照构造按 `DILocation` scope 分 自身 / 内联继承 / nodbg。

## Workload: parse  (M1=1.267;RQ1 gap = +35.1% → **回归**)

**① 账本**(synth.csv):

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `csv_parse` | 83.9 | ✓ |
| `csv_count::cb_field` | 5.2 | ✓(大部分内联进 csv_parse,IR 仅残 3 行) |

## Workload: write  (M1=0.994;RQ1 gap = +8.3% → **回归**)

**① 账本**(10k 字段 escaping,`csv_write` 内联进 harness main):

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `csv_count::main`(含内联的 `csv_write`) | 97.1 | ✓ |

**② 每个热函数的优化后 IR 构造计数**:

| 函数 | 义务/对照构造(Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `csv_parse` | Option 拆包 继承 2 + nodbg 2 | 63 / 25 / 33 / **0** | 72 / 31 / 38 / **103** |
| `csv_count::main`(write) | unwind nodbg 9;Option 拆包 nodbg 5;边界检查 继承 write_workload 1 + nodbg 1;memcpy/memset(对照,继承 csv_init/csv_fini 等) | 88 / 149 / 161 / **0** | 39 / 46 / 58 / **85** |

**③ 指令区域大小(Rust/C 行)与开放扫描**:

| 函数 | Rust 行 | C 行 | 开放扫描(Rust-only 符号) |
|---|---:|---:|---|
| `csv_parse` | 727 | 880 | — |
| `csv_count::cb_field` | 3 | 内联 | `FIELD` ×1 |
| `csv_count::main`(write) | 1420 | 786 | `llvm.assume` ×6、`llvm.experimental.noalias.scope.decl` ×9、`alloc::alloc`/`raw_vec`/`__rust_no_alloc_shim`(harness 框架) |

注:`csv_parse` Rust 727 < C 880(静态代码量反少);write 的 harness main 里的 Rust-only 符号均为 std/alloc 框架,非被翻译库差分。

**④ 查法2:优化成功痕迹(C 有 / Rust 少)**:

| 函数 | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `csv_parse` | 0 / 0 | 0 / 0 | 3 / 0 |
| `csv_count::main`(write) | 0 / 0 | 23 / 16 | 55 / 23 |

注:bswap 全 0;向量 Rust ≥ C(字符扫描无实质向量化空间);无"C 有 Rust 少"差分。libcsv 的 gap 在查法1 的 C1+C3。

---

**采集层小结(不含归类,仅记差分事实)**:
- **C1 冗余检查分支**:`csv_parse` Option 拆包 Rust 4(继承 2 + nodbg 2)vs C **0**;write 路径的 harness main 另有 Rust 边界检查/unwind,C 侧无。
- **C3 冗余访存 / 别名缺失**:`csv_parse` `!tbaa` Rust **0** vs C **103**;write main Rust 0 vs C 85。
- **C2 饱和 cast**:libcsv 无浮点,两侧 `fptosi.sat` 为 0(不适用)。
