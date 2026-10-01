# heman — Class II 差分数据(优化器少做的优化,采集层,不含归类判定)

**采集(修正方法,见 `_method.md`)**:单合并正则;两侧 debug info;两侧 no-LTO 对称——C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -gline-tables-only -c`(25 个 TU:heman_pipeline_c.c/src/*.c/kazmath/*.c,libm 数学函数不计);Rust = c2rust 库 crate `rust_raw`(`heman_raw`)**`cargo +nightly-2024-01-15`**(rustc 1.77 = LLVM 17,override rust_raw 自带 LLVM16 pin,与 clang-17 对称)`CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all`。计数按 `(file,line,col)` distinct 去重。

> ⚠️ **两个采集陷阱(否则得假结论)**:(1) **fat-LTO 是漏采**——直接对 fat-LTO harness 跑 `-Cremark=all` 得 loop-vec passed=0、inline-missed=0(向量化/内联在链接期 plugin,`-Cremark` 不 plumb);(2) **rust_raw pin LLVM16**——须 `+nightly-2024-01-15` 强制 LLVM17。本文件的更早版本(fat-LTO 得"loop-vec 热 C12/R0 / SLP C183/R28 / inline-missed 296/cost 6325")系两坑叠加的假象,**已作废**;以下为 no-LTO + LLVM17 对称重采真值。

**热函数账本(复用 Class I)**:pipeline(M1=1.078,gap +27.6%);generate(M1=1.226,gap +19.0%);lighting(M1=0.935,gap +29.5%)。

---

## II① 向量化损失 —— 基本无(LLVM17 对称 parity)

| pass | C | Rust(LLVM17) |
|---|---:|---:|
| loop-vectorize 库全量 passed / missed | 41 / 63 | **39** / 71(真实 `vectorized loop width 8`,带源码行) |
| slp-vectorizer 库全量 passed / missed | 140 / 430 | **136** / 686 |

逐热函数 loop-vectorize passed/missed **基本 parity**:`transform_to_distance` C2/R1、`create_df` C1/R1、`heightmap` C0/**R1**(Rust 反多,vectorize generate.rs:271 w8、C 两循环全 miss)、`occlusion` C2/R2、`edt` C0/R0、`island_noise` C0/R0、`apply_gradient` C0/R0、`main` C0/R0。SLP 逐热函数几乎全 0,唯 `edt` C2/R0(见下)、`occlusion` C3/R2。

**唯一真残余**(C 成功 / Rust miss,与 LLVM16 一致、LLVM17 未新增):`transform_to_distance` 的 strided-scatter 回写循环——C `distance.c:90`(`SDISTFIELD_TEXEL(x,y)=d[y]`)vectorized w8;Rust `distance.rs:173`(`*(*sdf).data.offset(y*width).offset(x)=*d.offset(y)`,stride=图宽)not vectorized。配对的 gather 载入循环 Rust `distance.rs:167` **两侧都 w8 向量化**——即 Rust 抓住了读、没抓住散写。根因:Rust 证不了 `d` 与 `(*sdf).data` 不别名。**这是一个真 ●**(C `distance.c:90` w8 passed / Rust `distance.rs:173` missed,别名原因),**但量级微观(单循环级,非函数级坍塌),不改 heman"Class II 基本无"的总判**。另 `edt` 的 SLP 2-wide store(distance.c:19,cost −2)Rust cost-0 中性拒绝(非合法性失败,negligible)。

**对账**:LLVM16→17 loop-vec C41/R37→C41/**R39**(Rust 反升,印证 LLVM17 Rust ≥ LLVM16)、SLP C140/R144→C140/R136(parity band 内 cost-model churn)。**parity 结论非后端不对称 artifact,LLVM17 复现并略强化**。也推翻 class_I 查法2 的"Rust 向量 581≥C 无损失"——静态 objdump 计数误导(含标量 xmm/结构体 SLP/std),remark 通道证明两侧 loop-vec 基本 parity(各 ~40),真正结构是各自向量化能力相当,非 Rust 全输也非 C 全输。

## II② 内联损失 —— 无(两侧同决策)

唯一同-TU 内部 callee `edt → transform_to_distance` **两侧都 too-costly 拒绝**(C cost=285>250、Rust cost=295>250,同决策近同 cost)。Rust 其余 inline-missed 全解析为:libc/libm(calloc/free/atan,def unavailable,C 同)、单-crate 暴露的跨-TU callee(`open_simplex_noise`/`_2`,C per-TU 看不到故不报,非损失)、panic 冷路径。**无真内联损失。**

## 与 Class I 的交叉

heman 无 Class II 缺口(唯 1 个 strided-scatter store 循环微观残余)。退化纯属 **Class I**:C2(浮点→整数饱和 cast 遍布热路径)+ C3(全热函数 `!tbaa`=0)。lighting 的 CPI 劣势(M1<1 却慢,M2=1.39)由 machine-level 差异(RQ2 承接),**非**向量化损失(occlusion 两侧 loop-vec C2/R2 parity)。

## 小结

heman = **Class II 基本不成立**(向量化 C41/R39 ≈ parity、SLP parity、内联同决策);唯一真实微观现象是 `transform_to_distance` 一个 strided-scatter store 循环因裸指针别名未向量化(单循环级)。**方法学教训:fat-LTO 漏采 + rust_raw LLVM16 两坑会同时伪造向量化损失(假 R0)与内联损失(假 cost 膨胀);LLVM17 对称重采推翻。** heman 退化归 Class I(C2+C3)。
