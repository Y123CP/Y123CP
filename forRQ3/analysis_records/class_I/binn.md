# binn — Class I 差分对比数据(采集层,不含归类判定)

**构建**:C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -DBINN_NO_COMPRESS -g -Wl,--plugin-opt=save-temps`(`binn_roundtrip.c binn.c`)→ precodegen → `llvm-dis-17`;Rust = 实测二进制的 fat-LTO 全程序模块(`rust_harness/target/release/deps/binn_roundtrip-*.ll`,`cargo build --release` + `-Cdebuginfo=1 --emit=llvm-ir`,nightly-2024-01-15 = LLVM 17.0.6)。

**归属**:按热函数 `define` 块统计,义务/对照构造按 `DILocation` scope 分 自身 / 内联继承 / nodbg。**C 侧 `AddValue`/`GetValue`/`AdvanceDataPos` 经 LTO 内联于 `main`,无独立定义**,其对照在 main 层。

## Workload: build_serialize_100k  (M1=1.530;RQ1 gap = +40.9% → **回归**)

**① 账本**(100k int32):

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `binn::AddValue` | 86.3 | ✓ |
| `binn_roundtrip::main` | 12.9 | ✓ |

## Workload: iterate_decode_100k  (M1=1.450;RQ1 gap = +27.9% → **回归**)

**① 账本**:

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `binn_roundtrip::main` | 55.4 | ✓ |
| `binn::GetValue` | 29.5 | ✓ |
| `binn::AdvanceDataPos` | 14.4 | ✓ |

**② 每个热函数的优化后 IR 构造计数**:

| 函数 | 义务/对照构造(Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `binn::AddValue` | Option 拆包 继承 1 + nodbg 1;memcpy self 2 | 32 / 28 / 37 / **0** | 内联于 main |
| `binn::GetValue` | memset self 1 | 30 / 31 / 57 / **0** | 内联于 main |
| `binn::AdvanceDataPos` | 八类全零 | 9 / 0 / 15 / **0** | 内联于 main |
| `binn_roundtrip::main` | unwind nodbg 19;Option 拆包 继承 9 + nodbg 13;overflow 继承 2;memset/memcpy(对照,继承 binn_free 等) | 124 / 105 / 153 / **0** | 119 / 116 / 178 / **230** |

**③ 指令区域大小(Rust/C 行)与开放扫描**:

| 函数 | Rust 行 | C 行 | 开放扫描(Rust-only 符号) |
|---|---:|---:|---|
| `binn::AddValue` | 427 | 内联于 main | — |
| `binn::GetValue` | 307 | 内联于 main | — |
| `binn::AdvanceDataPos` | 131 | 内联于 main | — |
| `binn_roundtrip::main` | 1659 | 1894 | `std::io::stdio` ×20+6、`llvm.assume` ×10、`alloc::raw_vec`/`alloc::alloc`/`__rust_no_alloc_shim`(harness 框架) |

注:main Rust 1659 < C 1894(静态少);Rust-only 符号集中于 harness main 的 std::io/alloc(打印+分配框架),非被翻译库差分。

**④ 查法2:优化成功痕迹(C 有 / Rust 少)**:

| 函数 | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `binn::AddValue` | 0 / 内联 | 0 / 内联 | 2 / 内联 |
| `binn_roundtrip::main` | 0 / 0 | 0 / 3 | 73 / 23 |

注:bswap 全 0;向量两侧都极少(typed 序列化无向量化);**call Rust 73 ≫ C 23(Rust 未内联更多 → II② 候选)**。无向量/bswap 差分——binn 的 gap 在查法1 的 C1(Option 拆包密集)+ C3。

---

**采集层小结(不含归类,仅记差分事实)**:
- **C1 冗余检查分支**:`AddValue` Option 拆包(`malloc_fn`)Rust 2 vs C 0;`main` Option 拆包 Rust 22(继承 9 + nodbg 13)+ overflow 2 + unwind 19 vs C 0——序列化/解码循环密集的空指针拆包与溢出检查是 binn 的主要前端多生成。
- **C3 冗余访存 / 别名缺失**:全部热函数 Rust `!tbaa` 恒为 0;C 侧 `main` 达 **230**(库函数内联其中)。
- **C2 饱和 cast**:binn 无浮点,两侧 `fptosi.sat` 为 0(不适用)。
