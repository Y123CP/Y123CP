# heman — Class I 差分对比数据(采集层,不含归类判定)

**构建**:C = `clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -Wno-unknown-pragmas -g -Wl,--plugin-opt=save-temps`(`heman_pipeline_c.c src/*.c kazmath/*.c -lm`)→ precodegen → `llvm-dis-17`;Rust = 实测二进制的 fat-LTO 全程序模块(`rust_harness/target/release/deps/heman_pipeline-*.ll`,`cargo build --release` + `-Cdebuginfo=1 --emit=llvm-ir`,nightly-2024-01-15 = LLVM 17.0.6)。

**归属**:按热函数 `define` 块统计,义务/对照构造按 `DILocation` scope 分 自身 / 内联继承 / nodbg。**libm 数学函数(`__atan_fma`/`__ieee754_pow_fma`/`__memmove_*`)两侧共享,非翻译差分,不计入**;C 侧 `compute_occlusion`/`generate_island_noise` 经 LTO 内联,无独立定义。

## Workload: pipeline(distance field)  (M1=1.078;RQ1 gap = +27.6% → **回归**)

**① 账本**(seed_1024²):

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `heman_pipeline::main` | 32.3 | ✓ |
| `distance::edt` | 20.0 | ✓ |
| `heman_color_apply_gradient` | 16.3 | ✓ |
| `distance::transform_to_distance` | 12.2 | ✓ |
| `heman_distance_create_df` | 10.9 | ✓ |

## Workload: lighting  (M1=0.935;RQ1 gap = +29.5% → **回归**)

**① 账本**(seed_256² + 光照):`heman_lighting_compute_occlusion` 46.1%;其余 `__atan_fma` 31.7% / `__ieee754_pow_fma` 7.8% 为 **libm**(两侧共享,不计)。M1<1(指令反少),退化以 CPI 为主(RQ2 M2=1.39),Class I 弱。

## Workload: generate(terrain)  (M1=1.226;RQ1 gap = +19.0% → **回归**)

**① 账本**(512² 岛屿):

| 函数 | self% | ≥5% |
|---|---:|:-:|
| `heman_internal_generate_island_noise` | 67.6 | ✓ |
| `heman_generate_island_heightmap` | 8.7 | ✓ |
| `distance::edt` | 7.3 | ✓ |

**② 每个热函数的优化后 IR 构造计数**:

| 函数 | 义务/对照构造(Rust) | Rust load/store/gep/**tbaa** | C load/store/gep/**tbaa** |
|---|---|---|---|
| `heman_color_apply_gradient` | **饱和 cast 继承 heman_image_sample 2** | 10 / 7 / 19 / **0** | 20 / 17 / 39 / **37** |
| `heman_lighting_compute_occlusion` | **饱和 cast self 1** | 34 / 41 / 60 / **0** | 内联于 main |
| `heman_internal_generate_island_noise` | **饱和 cast 继承 fastFloor 7** | 115 / 9 / 92 / **0** | 内联于 main |
| `heman_generate_island_heightmap` | **饱和 cast 继承 heman_image_sample 7** + nodbg 1;memcpy/memset(对照) | 18 / 13 / 34 / **0** | 172 / 58 / 194 / **230** |
| `distance::transform_to_distance` | memcpy self 1 | 27 / 15 / 59 / **0** | 7 / 11 / 39 / **21** |
| `heman_distance_create_df` | 八类全零 | 17 / 16 / 23 / **0** | 22 / 22 / 38 / **44** |
| `distance::edt` | 八类全零 | (130 行) / **0** | 14 / 8 / 20 / **22** |
| `heman_pipeline::main` | unwind nodbg 21;**饱和 cast 继承 heman_export_u8 8**;Option 拆包 nodbg 6;overflow 1;memcpy/memset(对照) | 139 / 122 / 193 / **0** | 49 / 0 / 37 / **47** |

**③ 指令区域大小(Rust/C 行)与开放扫描**:

| 函数 | Rust 行 | C 行 | 开放扫描(Rust-only 符号) |
|---|---:|---:|---|
| `heman_color_apply_gradient` | 147 | 285 | — |
| `heman_distance_create_df` | 216 | 310 | — |
| `heman_lighting_compute_occlusion` | 658 | 内联于 main | — |
| `heman_internal_generate_island_noise` | 1983 | 内联于 main | `heman_raw::src...` ×28(库内部) |
| `heman_generate_island_heightmap` | 406 | 3461 | — |
| `distance::transform_to_distance` | 326 | 273 | — |
| `distance::edt` | 130 | 237 | — |
| `heman_pipeline::main` | 2236 | 504 | `llvm.assume` ×15、`llvm.experimental.noalias.scope.decl` ×13、`alloc::*`(harness 框架) |

注:计算内核多数 Rust < C(apply_gradient 147/285、edt 130/237、heightmap 406/3461)——静态代码量反少,C2 饱和 cast 的膨胀在动态执行。库热函数无 harness 框架符号(noise 的是库内部);框架符号集中于 main。

**④ 查法2:优化成功痕迹(C 有 / Rust 少)**:

| 函数 | bswap R/C | vector`<N>` R/C | call/invoke R/C |
|---|---|---|---|
| `heman_color_apply_gradient` | 0 / 0 | 18 / 22 | 2 / 2 |
| `heman_internal_generate_island_noise` | 0 / 内联 | 581 / 内联 | 6 / 内联 |
| `distance::edt` | 0 / 0 | 0 / 1 | 0 / 0 |

注:bswap 全 0;**Rust 向量丰富(noise 581、apply_gradient 18)≥ C**——浮点热路径 Rust 也自动向量化了,不构成"C 有 Rust 少"。heman 的 gap 在查法1 的 C2 饱和 cast(标量转换)而非向量化。

---

**采集层小结(不含归类,仅记差分事实)**:
- **C2 标量转换膨胀(主线)**:浮点→整数 `as` cast 在 Rust 是饱和语义(`llvm.fptosi.sat`),**遍布 heman 热路径**——`fastFloor` 继承 7、`heman_export_u8` 继承 8、`heman_image_sample` 继承 2/7、`compute_occlusion` 自身 1;**C 侧全部为 0**(C 的 `(int)x` 是一条 `cvttsd2si`)。这是 heman(语料唯一浮点密集项目)的招牌前端多生成。
- **C3 冗余访存 / 别名缺失**:全部热函数 Rust `!tbaa` 恒为 0,C 侧 22–230。
- **C1 冗余检查分支**:仅 harness main 有零星 Option 拆包/overflow;计算内核(edt/create_df)无。
- **lighting 的特殊性**:M1<1(Rust 指令反少),Class I 照不到,退化在 CPI(RQ2 M2=1.39,五维全正),非前端多生成。
