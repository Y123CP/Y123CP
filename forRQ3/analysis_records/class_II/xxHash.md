# xxHash — Class II 差分数据(优化器少做的优化,采集层,不含归类判定)

**采集(修正方法,见 `_method.md`)**:单合并正则;两侧 debug info;两侧 no-LTO 对称——C = per-TU `clang-17 -O3 -march=native -mno-avx512f -DNDEBUG -DXXH_VECTOR=0 -DXXH_STATIC_LINKING_ONLY -gline-tables-only -c xxhash.c`;Rust = c2rust 库 crate `rust_raw` **`cargo +nightly-2024-01-15`**(rustc 1.77 = LLVM 17.0.6,override rust_raw 自带的 nightly-2023-04-15/LLVM16 pin,与 clang-17 对称)`CARGO_PROFILE_RELEASE_LTO=off`。`-Cllvm-args=-pass-remarks` 不发射,改用 rustc 原生 **`-Cremark=all`**(`note:` 行,passed token 为 `(success)`)。两侧均 `XXH_VECTOR=0`(纯标量,XXH3 逻辑 header-only 在 `xxhash.h`)。
> **后端对称确认**:LLVM 16→17 重采数字**逐项一致**(SLP C78/14、Rust10/8;内联 cost 395>375 byte-for-byte)——inline cost model 与 SLP 决策在 16/17 间未变,证明 xxHash 的损失是 c2rust 丢弃 `always_inline` 的**结构性**后果,非 toolchain artifact。

**热函数账本(复用 Class I)**:XXH3 workload(M1=1.270,gap +33.1%)→ `XXH3_hashLong_internal_loop` 95.9%(Class II 主战场);XXH32 workload(M1=0.475,gap +62.2%)→ main(XXH32 逻辑内联其中,Class II 信号弱,退化主 CPI,见小结)。

---

## II② 内联损失 —— ●(根因)

C 侧 `XXH_FORCE_INLINE = __attribute__((always_inline))` 把整条哈希链强制内联:`XXH3_scalarRound → XXH3_accumulate_512_scalar → XXH3_accumulate_scalar → XXH3_hashLong_internal_loop`(+`consumeStripes`/`digest_long`),热 kernel **12 inline-passed / 0 not-inlined**。

Rust(c2rust)**丢弃 force-inline 属性**:`XXH3_accumulate_512_scalar`(**cost=395 > threshold=375**,仅超 ~5%)**未内联**进 4 个热 caller:
- `XXH3_accumulate_scalar`(stripe 循环——最紧内循环,`xxhash.rs:1641`)
- `XXH3_hashLong_internal_loop` ×2(`xxhash.rs:1641`/`1754`)
- `XXH3_consumeStripes` / `XXH3_digest_long`(`xxhash.rs:2446`)

Rust full-lib inline 312 passed / 50 not-inlined(vs C 257 / 33)。

## II① 向量化损失 —— ●(由内联损失引发)

| SLP | C | Rust |
|---|---|---|
| full lib passed(raw / unique 站点) | **78 / 14** | 10 / 8 |
| full lib missed | 1653 | 71 |
| **热 kernel passed** | **7** | **1** |

> raw 计数 C 偏高因整实现 header-only 被 force-inline 进 ~40 caller(每份内联一条 remark);unique 站点(14 vs 8)与热 kernel(7 vs 1)是公平对照。

loop-vectorize **非机制**(热 kernel 两侧 0 passed);损失全在 SLP。关键站点:
- `xxhash.h:6022`(scalarRound 8-lane paired stores)→ C **"Stores SLP vectorized, cost -1, tree size 28"**;Rust `xxhash.rs:1613` 仅 **"cost -2, tree size 6"**——大 tree-28 SLP 没形成;
- `xxhash.h:6072`(scalarScrambleRound)→ C **"Stores SLP vectorized, cost -15, tree size 16"**;Rust `xxhash.rs:1660` → **"not beneficial cost 0>=0"**——scramble 步骤**完全没向量化**;
- Rust stripe 循环 `xxhash.rs:1639/1641` → **"loop not vectorized: call instruction cannot be vectorized"**(未内联的 `accumulate_512_scalar` call 卡在循环体里)。

## 因果链(教科书级 Class II)

**内联损失 *导致* 向量化损失**:`accumulate_512_scalar`(cost 395>375)在 per-stripe 循环里保持 out-of-line call → 阻止 8-lane body 的 loop unroll → 宽 SLP tree 无法形成(28→6)、scramble SLP 完全丢失。此外 c2rust 裸指针无 `noalias`/`!tbaa`(Class I C3)进一步妨碍 SLP 证明 load/store 可打包——**内联损失(II②)+ 别名缺失(C3)共同压垮向量化(II①)**。

## 小结

xxHash = **Class II 最强实例:内联损失 ●(根因,force-inline 被 c2rust 丢弃)→ 向量化损失 ●(SLP tree 退化)**,两者因果耦合,并与 Class I C3 叠加。这解释 XXH3(M1>1 工作量膨胀 + 标量链跑不快)。

**XXH32 的 213-vs-20 闭环**:XXH32(M1=0.475 指令反少却 +62.2%)主要是 CPI 退化(RQ2 M2=3.42),Class II 信号弱。Class I 查法2 曾以静态 objdump 计"C 213 向量 op vs Rust 20"——remark 通道裁决:C 的 213 主要是 `xxh_bench.c` 的 **cov_workload setup 循环**(width 32,非 XXH32 计时哈希路径)auto-vec 产物,XXH32 哈希热路径本身两侧向量化都少;该 213-vs-20 是**静态计数虚高**(同 bzip2/heman 已被 remark 通道证伪的模式),XXH32 退化归 CPI,由 RQ2 分解承接。
