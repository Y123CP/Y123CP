# Class II 规则凝练(缺口聚类,基于 6 项目采集层)

**输入**:`class_II/{bzip2,heman,libzahl,xxHash,binn,libcsv}.md`(6 个回归项目的 Class II 差分事实,采集方法见 `_method.md`)。
**层面界定**:Class II = 优化器**少做**的优化(M-b)——同一 LLVM 17.0.6 后端下,C 优化器成功、c2rust(Rust)输出失败/未做。与 Class I(前端**多生成**义务工作)正交,但常是**同一批 c2rust 缺陷在优化器层的下游**(见"与 Class I 的交叉")。

## 凝练原则(四条)

1. **同层面合并**:几种触发子机制,若造成**同一种物理性能缺口**(CPU 浪费同一类工作),凝练成**一条**规则——按缺口聚,不按 pass 名或失败原因拆。
2. **复现门槛 = 一条规则 ≥3 支持实例**;实例 = ≥3 种子机制(同层面)**或**同一子机制在 ≥3 个热函数/站点。跨项目广度不作准入(仅如实报告)。
3. **置信 ●/◐(Class II 专属)**:准入看 Rust 有没有 missed;C 侧同 remark 是否成功决定置信——**仅 Rust miss 而 C 成功 = ●(解释 C↔Rust gap)**;两侧共有 = ◐(不解释 gap,但改写仍可让 Rust 绝对变快)。
4. **计数用 unique `(file,line,col)` 站点,禁 raw 聚合数**(header-only/force-inline/宏展开会 raw 虚高;见 `_method.md`)。门槛与 ● 锚定**热函数内 unique 站点**。

按此,Class II 收敛为 **3 条规则**(D1/D2/D3),对应"优化器少做的优化"三大类:SIMD 并行 / 调用与跨函数优化 / 内存别名优化。**通道穷尽性**已由全 pass 枚举验证(loop-unroll/idiom/delete parity、unswitch/gvn-sink/machine-licm 零发射,见 `_method.md`)。其中 **D3(内存优化损失)根因是别名缺失、已回填 Class I C3,不重复计入门槛**,仅为完整呈现"优化器少做"全貌而单列。

---

## D1 — 向量化损失(SIMD 并行未实现)

- **缺口层面**:热循环 / 直线代码本可 SIMD,Rust 保持标量逐元素执行,浪费向量吞吐。
- **合并的三个子机制**(同缺口层面"向量化前提被 c2rust 代码形态破坏",故合一条):
  - **(a) bounds-check panic 破坏可数性**:c2rust 每次数组索引前的 `panic_bounds_check` 分支 = 循环**第二出口**,加 signed-`c_int` 归纳 + loop-carried 变量写回状态字段 → LLVM 算不出 trip-count / 值"用在循环外" → loop-vectorizer 放弃。**与 Class I C1 同根**(C1 的检查在 II 层显为向量化阻断)。
  - **(b) 内联缺失致 unroll 缺失 → SLP tree 退化**:热 kernel 保持 out-of-line call(见 D2),`call instruction cannot be vectorized` 卡住循环,且宽 SLP tree(28→6)无法形成。**是 D2 的下游**。
  - **(c) 裸指针别名阻断**:证不了 `*mut` 目标不别名 → strided-scatter 等散写循环不向量化。**与 Class I C3 同根**。LLVM17 下大多消解(libzahl 7 处 `cannot identify array bounds` 是 LLVM16 artifact),仅残个别。
- **检测签名**(Rust 侧 no-LTO LLVM17 remark,C 侧同源循环 `vectorized loop`):
  ```
  hot(f) ∧ C_remark(loop L) = "vectorized loop (width N)"
          ∧ Rust_remark(对应 L) ∈ { "could not determine number of loop iterations"     // (a)
                                    | "value ... used outside the loop"                   // (a)
                                    | "call instruction cannot be vectorized"             // (b)
                                    | "loop not vectorized"(别名类)}                     // (c)
  ```
- **改写 + 安全**(M-b;(b) 纯语义中性,(a)/(c) 借 C1/C3 改写):
  ```
  (a) 消 bounds-check + 控制流重构(迭代器 / .get_unchecked ⊢ i<len)+ 消状态字段写回 ⟹ 恢复 countability
  (b) 对 thin kernel 加 #[inline(always)] ⟹ 恢复 unroll ⟹ SLP tree 重现     (语义中性,无需证明)
  (c) *mut T ⟹ &mut [T](获 noalias)或标量外提  ⊢ writes ⊥ addr           (同 C3 不别名证明)
  ```
- **≥3 支持实例**(热锚定 unique 站点,3 子机制 / 3 项目):
  - (a) **bzip2 `BZ2_decompress` 5 个热解码循环**(decompress.c :311 w32 / :316 w16 / :409+:416 w4 / :447 w32,Rust 全 miss)——单项目单子机制即 ≥3;
  - (b) **xxHash `XXH3_hashLong` accumulate kernel**(SLP tree 28→6、scramble 完全丢);
  - (c) **heman `transform_to_distance` strided-scatter**(distance.c:90 w8 / distance.rs:173 miss,真 ●,量级微观)。✅
- **广度如实报告**:3 项目;主力是 (a) bzip2 + (b) xxHash;(c) 在 LLVM17 下仅 heman 残 1 站点(裸指针别名向量化损失基本被 LLVM17 消解)。

## D2 — 内联损失(调用开销 + 跨函数优化阻断)

- **缺口层面**:热路径小函数保留 out-of-line call,每次进出付 call/ret,并丢失内联本可暴露的常量传播 / 别名收窄 / 循环展开。
- **合并的两个子机制**(同缺口层面"热 callee 未内联",故合一条):
  - **(a) c2rust 丢弃 `force-inline` 属性**:C `XXH_FORCE_INLINE`=`__attribute__((always_inline))` 绕过 cost model 强制内联;c2rust 不保留该属性,LLVM 独立按 cost 评估并拒绝(cost 395 > threshold 375)。**根因型**(还引发 D1(b))。
  - **(b) c2rust body 膨胀抬高 inline-cost**:冗长 body(裸指针簿记 + 边界检查)使 callee cost 越阈值(`zinit_temp` 685 vs C 115、`zfree` 280 vs C 10),C 内联 Rust 不。
- **检测签名**:
  ```
  hot(caller) ∧ Rust_remark = "'callee' not inlined into 'caller' because too costly (cost=X, threshold=Y)"
              ∧ callee 是同-crate 小函数 ∧ C 侧同 callee inlined(always_inline 或 cost 低)
  ; 排除 cross-crate(LTO-resolvable)/ framework-noinline / MIR 已内联
  ```
- **改写 + 安全**(M-b,语义中性,无需正确性证明):
  ```
  (a) 对热 callee 加 #[inline(always)] ⟹ 恢复 C 的 always_inline 语义
  (b) 先按 C1–C3 缩小 body 使 cost 回落阈值内,或直接 #[inline]
  ```
- **≥3 支持实例**(热锚定 unique 站点,2 子机制 / 2 项目):
  - (a) **xxHash `XXH3_accumulate_512_scalar` 未内联进 4 个热 caller**(cost 395>375;`accumulate_scalar`/`hashLong`×2/`consumeStripes`/`digest_long`)——单项目单子机制即 ≥3;
  - (b) **libzahl `zmul_ll` 内 2 个 callee**(`zinit_temp` 685>325、`zfree` 280>250)。(`zsqr_ll` 同型但非 ≥5% 热函数,补充非热支撑,降权。)✅
- **广度如实报告**:2 项目;xxHash (a) 是根因型且引发 D1。

## D3 — 内存优化损失(冗余访存未消除;根因回填 Class I C3,不重复计门槛)

- **缺口层面**:优化器少做的**标量内存优化**——GVN 冗余 load 未消除、LICM 循环不变 load/gep 未外提,因 Rust 缺 `noalias`/`!tbaa`、优化器证不了不别名而保守保留跨迭代访存。
- **与 C3 的关系(承重)**:这是 **Class I C3(别名缺失)同一现象的优化器-remark 视角**——C3 记 **IR 访存计数**(裸指针冗余 load、`!tbaa`=0),D3 记 **优化器 remark**(GVN/LICM missed)。二者两口径同现象;**根因是前端不给别名(M-a),已计入 C3,D3 不重复计入 ≥3 门槛**,仅为"优化器少做"完整清单单列。
- **合并的两个子机制**(同缺口层面):GVN load-elimination missed(`load of type X not eliminated`)+ LICM hoist missed(`failed to hoist load with loop-invariant address`)。
- **检测签名**:
  ```
  hot(f) ∧ Rust_remark ∈ { "gvn: load ... not eliminated"
                          | "licm: failed to hoist load with loop-invariant address" }
          ∧ Rust missed ≫ C(措辞纯别名类)∧ !tbaa ∉ meta(访存)
  ```
- **改写 + 安全**(= C3 改写,需证不别名):
  ```
  *mut T ⟹ &mut [T](恢复 noalias)或标量外提   ⊢ writes(L) ⊥ addr(p)
  ```
- **支持实例**(remark 视角以 libzahl 最显;门槛计入 C3 的全项目 IR 访存):libzahl `zrsh` gvn missed 46/16(R/C)、`zsub` gvn 43/27 + licm 16/4、`zlsh` gvn 54;全 6 项目 `!tbaa`=0 + 冗余 load(C3 IR:bzip2 1010/755、libzahl zrsh 73/24)。
- **广度**:remark 视角 libzahl 明显(bignum 裸指针游标);其他项目 gvn/licm 被宏展开/force-inline **per-instance 计数混淆**(bzip2/heman C missed 反多是宏/内联展开 artifact),故以 C3 的 IR 访存计数为准(全项目)。
- **与 D1(c) 同根不同缺口**:D1(c)(裸指针阻断向量化)与 D3(阻断标量内存优化)都源于别名缺失(C3),但缺口一为 SIMD、一为 GVN/LICM。

---

## 因果链与 Class I 交叉(Class II 的核心洞察)

- **D2 → D1 因果**(xxHash 教科书级):内联损失(D2(a),force-inline 丢弃)→ kernel out-of-line → 阻 unroll → SLP tree 退化(D1(b))。**内联损失是根,向量化损失是果**。
- **Class II ⊂ Class I 缺陷的下游**:并非完全正交——
  - D1(a) = **C1(冗余边界检查)的二阶后果**(panic 分支破坏循环可数性);
  - D1(c) + 已回填的 licm/gvn missed = **C3(别名缺失)的优化器后果**(证不了不别名 → 向量化/内存优化保守);
  - 即同一批 c2rust 缺陷,Class I 记"多做的义务工作",Class II 记"因此少做的优化"。
- **改写多重收益**:改 C1(消 bounds-check)顺带解锁 D1(a) 向量化;改 C3(&mut slice)顺带解锁 D1(c);而 D2 + D1(b) 是 **Class II 独有的纯 `#[inline]` 语义中性改写**(无需任何正确性证明,最干净)。

## 排除记录(用同一门槛复查)

- **licm / gvn 的 `load not eliminated` / `can't hoist` missed 簇** → 见 **D3(内存优化损失)**,根因回填 C3、不重复计入 ≥3 门槛(已单列 D3,非排除)。
- **loop-unroll / loop-idiom / loop-delete**:全项目两侧 parity,无缺口 → 排除。
- **loop-unswitch / gvn-sink / machine-licm**:LLVM17 两侧零发射 → 无可采。
- **binn / libcsv**:无 Class II(向量化两侧 0/0,内联对称甚至 Rust 略优)——退化纯 Class I(C1+C3)。
- **纯 ◐ 成簇**:本语料无(libzahl 算术核 carry 依赖两侧皆不可向量化,是"两侧都做不了"非"机会")。**本语料 Class II 全为 ●**(C 成功 Rust 失败),推翻旧 class_II"● 空/全 ◐"结论(旧错在过时 workload + 未后端对称)。

## 小结

Class II 恰好 **3 条**,完整呈现"优化器少做的优化"三大类:
- **D1(向量化损失)** — 3 子机制(countability 破坏 / unroll 缺失 / 别名阻断),bzip2 5 循环 + xxHash kernel + heman 1 站点;过 ≥3 门槛;
- **D2(内联损失)** — 2 子机制(force-inline 丢弃 / body 膨胀),xxHash 4 caller + libzahl 2 callee;过 ≥3 门槛;
- **D3(内存优化损失)** — GVN/LICM missed,libzahl 多热函数 + 全项目 tbaa 缺失;**根因回填 C3,不重复计门槛**(为完整性单列)。

检测统一在**目标项目 Rust 侧 no-LTO LLVM17 remark**(C 侧同源循环/callee 成功作 ● 佐证);改写档:**D2 + D1(b) 是纯 `#[inline]` 语义中性**(Class II 最干净的贡献,无需正确性证明),D1(a)/D1(c)/D3 借 C1/C3 改写并共享其安全证明。**核心洞察:Class II 是 Class I 缺陷的优化器下游(C1→countability、C3→别名/内存优化),且内联损失(D2)因果引发向量化损失(D1);本语料 Class II 全为 ●,是真实、结构性、后端对称验证的 C↔Rust gap 来源。remark 通道有天花板(machine-level 的调度/regalloc/peephole 不发 remark),CPI 型缺口由 RQ2 承接。**
