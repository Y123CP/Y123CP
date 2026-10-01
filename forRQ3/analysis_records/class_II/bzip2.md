# bzip2 — Class II 差分数据(优化器少做的优化,采集层,不含归类判定)

**采集(修正方法,见 `_method.md`)**:单合并正则;两侧 debug info;两侧 no-LTO 对称——C = per-TU `clang-17 -O3 -march=native -mno-avx512f -gline-tables-only -DNDEBUG -D_FILE_OFFSET_BITS=64 -c`(7 个库 TU);Rust = c2rust crate `rust_raw` 强制 no-LTO(`CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1` + `-Cremark=all`,该 toolchain `-pass-remarks` 不发)。同一 LLVM 17.0.6(nightly-2024-01-15)。内联通道排除 cross-crate(LTO-resolvable)+ framework-noinline。

> **Rust passed 经双 build 交叉验证为真实(非漏采)**:no-LTO 新 build 与既有 fat-LTO `lto_remarks.log` 报**同一批** Rust 向量化成功(decompress.rs:2504/2553/2959 loop-vec + 3008 SLP)。故 bzip2 的 Rust 向量化 passed 是真值,早期"R0"是旧的错数字(与 heman 的 fat-LTO 漏采不同——bzip2 两种 build 一致)。

**热函数账本(复用 Class I,decompress workload:M1=1.348,gap +15.5%)**:`BZ2_decompress` 61.21%(c2rust 输出为 ~2880 行单体状态机,decompress.rs:191–3071)+ `BZ2_bzDecompress` 37.58%(控制包装,两侧零可向量化循环,非损失来源)。

> **计数口径**:下表为 raw remark 计数(C 侧库全量含宏展开 `GET_BITS` 的多份 load 站点,聚合数偏高、**不作跨项目对比**);● 判定与 ≥3 门槛一律锚定**热函数内 unique 站点**——即下方 5 个热解码循环,与去重口径无关。

---

## II① 向量化损失 —— ●(真实,残余 5 个热循环)

| pass | 侧 | 热函数 passed / missed | 库全量 passed / missed |
|---|---|---:|---:|
| **loop-vectorize** | C | **8** / 54 | 23 / 227 |
| | Rust | **3** / 58 | 18 / 135 |
| **slp-vectorizer** | C | **5** / 382 | 14 / 1295 |
| | Rust | **2** / 16 | 2 / 193 |

- Rust 热函数确实做成的:`decompress.rs:2504`(=C cftab 累加循环)、`:2553`(=C cftabCopy)、`:2959`(=C maxLen 扫描);SLP `mtfa[kk]=…` store(=C 同站点)。**Rust 并非全输**,与 C 有交集。
- **残余真损失(C 向量化 / Rust miss)**:

| C 站点(decompress.c) | 宽度 | 源码语义 | Rust |
|---|---|---|---|
| `:311` | 32 | `for(v;v<nGroups;v++) pos[v]=v`(selector 初始化) | missed |
| `:316` | 16 | `while(v>0){pos[v]=pos[v-1];v--;}`(MTF selector 左移) | missed |
| `:409/416` | 4/4 | `while(es>0){…}`(RLE run-expand) | missed |
| `:447` | 32 | `while(nn>0){…}`(MTF value copy) | missed |

**Rust 放弃原因**(`BZ2_decompress` 内直方图):**53× `could not determine number of loop iterations`** + **45× `value that could not be identified as reduction is used outside the loop`**(+1 switch,+1 unsafe-dep)。

**根因(承重,已确认)**:(i) c2rust 每次数组索引前的 `panic_bounds_check` 分支 = 循环**第二个出口** → LLVM 算不出 countable trip-count;(ii) signed `c_int` 归纳变量 + loop-carried 变量写回 `(*s).save_*` 状态字段 → 值"在循环外被使用",击溃 reduction/IV 识别。二者精确命中 C 向量化的 RLE `while(es>0)/(nn>0)` 与 MTF-selector `while(v>0)` 循环。

## II② 内联损失 —— 无

剔除 cross-crate(LTO-resolvable)与 framework-noinline 后,in-crate cost-based 内联损失两侧皆 0:Rust **成功内联**所有真实热 callee(`makeMaps_d`、`unRLE_obuf_to_output_FAST/SMALL`、`BZ2_indexIntoF`、`BZ2_bz__AssertH__fail`),与 C 一致。bzip2 的 gap 不在内联。

## 与 Class I 的交叉(重要)

bzip2 向量化损失根因 = **Class I C1(冗余边界检查)的二阶后果**:C1 记录的 `panic_bounds_check` 分支不仅本身多执行,还在每个热循环插入第二出口、破坏 countability → 向量器放弃。同一 c2rust 缺陷在 Class I 表现为"多做检查"、Class II 表现为"少做向量化"——加强 C1,但不改 C1 规则。(回填 class_I:那里旧查法2 用 objdump 静态计数误判"Rust 向量 ≥ C",被本 remark 通道证伪。)

## 小结

bzip2 = **Class II 向量化损失 ●(真实,残余 5 个热解码循环,C w4–32 / Rust miss),内联无损失**。根因是 c2rust 的边界检查 panic 分支(第二出口)+ signed-int 循环 + 状态字段写回,破坏 trip-count 可判定性。与 xxHash(内联→unroll→SLP)、libzahl(裸指针别名)同属"c2rust 代码形态破坏向量化前提"缺口层面,触发子机制不同(countability vs 别名 vs unroll)。
