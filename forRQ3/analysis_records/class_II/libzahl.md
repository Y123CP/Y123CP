# libzahl — Class II 差分数据(优化器少做的优化,采集层,不含归类判定)

**采集(修正方法,见 `_method.md`)**:单合并正则;两侧 debug info;两侧 no-LTO 对称——C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -gline-tables-only -c`(`libzahl_opsuite.c src/*.c`);Rust = c2rust 库 crate `rust_raw` **`cargo +nightly-2024-01-15`**(rustc 1.77 = LLVM 17,override rust_raw 自带 nightly-2023-04-15/LLVM16 pin,与 clang-17 对称)`CARGO_PROFILE_RELEASE_LTO=off` + `-Cremark=all`。
> **后端对称订正(重要)**:早期采集误用 rust_raw 自带 LLVM 16,得出"向量化小损失"。本轮 LLVM 17 对称重采**推翻**该结论(见下)。

**热函数账本(复用 Class I)**:mul_4096bit(M1=1.231,gap +37.3%):`zmul_ll`/`libzahl_realloc`/`zadd_unsigned_assign`/`zfree`/`zrsh`/`zlsh`;modpow_4096bit(M1=1.123,gap +10.2%):`libzahl_zsub_unsigned`/`realloc`/`zrsh`/`zadd`/`zfree`/`zmul_ll`。

---

## II① 向量化损失 —— 无(LLVM16 artifact,LLVM17 对称消失)

| pass | Rust(LLVM17) | C(LLVM17) |
|---|---:|---:|
| SLP passed / missed | 10 / 165 | 14 / 856 |
| loop-vectorize passed / missed(热文件) | 1 / 17 | 0 / 21 |

- **SLP:无 gap**——R10/C14 平衡,两侧热 bignum limb 算术都不 SLP(carry 链 loop-carried 依赖),主导 miss 皆 "impossible / not beneficial"。
- **loop-vectorize:对称**——热文件 R1/C0(≈都 0);主导失败原因两侧一致 "could not determine number of loop iterations"(R16/C17)+ switch + 非规约。carry-chain limb 循环两侧皆不可向量化。
- **7 处 `cannot identify array bounds` = LLVM16 artifact,已消失**:LLVM16 下 zsub.rs 有 7 处裸指针 bounds 失败(Rust-only);LLVM17 下**全消失**(只剩 1 处且在 stdlib `core/ptr/mut_ptr.rs`,非热函数),zsub.rs 循环现与 zsub.c **同因**失败(switch/trip-count)。LLVM17 改进了裸指针 bounds 分析——早期"小损失"是 16-vs-17 mismatch 假警报。

## II② 内联损失 —— ●(唯一真实、版本无关的 gap)

热 caller `zmul_ll`(两侧递归、都不内联进其 caller,对称);其 **callee** 分化(同 scope:C `zmul.c` statics / Rust `zmul` module):

| callee → zmul_ll | C(clang-17) | Rust(c2rust,LLVM17) |
|---|---|---|
| `zinit_temp` | **inlined** cost=115(thr 325) | **MISSED** cost=**685**(thr 325) |
| `zfree`/`zfree_temp` | **inlined** cost=10(thr 487) | **MISSED** cost=**280**(thr 250) |

`zsqr_ll` 同型(Rust miss `zfree` 280、`zadd_unsigned_assign` 440、`zlsh` 780)——但 `zsqr_ll` **不在 Class I ≥5% 热函数账本**(热 caller 仅 `zmul_ll`),作补充非热支撑,**凝练时降权**;libzahl 热锚定的内联损失站点是 `zmul_ll` 内 2 个(`zinit_temp`/`zfree`)。

- **版本无关**:asymmetry 在 LLVM 16→17 **持续甚至更重**(`zinit_temp` cost 385→685,C 恒 115 → 5.9×;`zfree` 恒 280,C 10)。是真 codegen gap:c2rust 冗长 body(裸指针簿记 + 边界检查)抬高 LLVM inline-cost 估值,热 caller 保留 out-of-line call,clang 则折叠。
- **跨 scope caveat(不夸大)**:`libzahl_realloc` 未内联进 63 个 Rust caller(cost 1135/1180 ≫ 250),但 per-TU C 里 realloc 跨 TU、内联器根本不评估(0 remark)→ 两侧终态都 out-of-line;且 realloc 够大,C-LTO 同 cost model 也会拒。**属 build-model 可见性差异,按对称处理,非 Rust 退化**。

## 与 Class I 的交叉

libzahl 无向量化 Class II 缺口(早期误判已订正)。内联损失(`zmul_ll`/`zsqr_ll` 的 thin temp wrapper 未内联)是**独立 Class II 信号**,不与 Class I(主线 C3 冗余访存 `zrsh` load 73/24)重叠。

## 小结

libzahl = **向量化无(LLVM16 artifact,LLVM17 对称消失)+ 内联损失 ●(唯一真 gap,版本无关)**。内联根因 = c2rust body 膨胀抬高 inline-cost(`zinit_temp` 685 vs 115、`zfree` 280 vs 10),使 `zmul_ll`/`zsqr_ll` 保留 out-of-line call。可行改写 = 收缩 c2rust body 或对 thin temp wrapper 加 `#[inline]`。**方法学教训:LLVM 16→17 会改变裸指针向量化判定(bounds 分析),但不改 inline cost model——向量化损失须 LLVM17 对称验证,内联损失版本稳健。**
