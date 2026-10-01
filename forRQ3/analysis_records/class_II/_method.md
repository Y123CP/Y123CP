# Class II 采集方法(优化器少做的优化,M-b)

**层面界定**:Class II = 对回归热函数,**C 优化器成功、Rust(c2rust 输出)失败或未做**的优化。对应 RQ3 查法2("Rust 缺了 C 有的好东西")。与 Class I 正交:Class I 是 Rust 相对 C **多生成**的义务工作(义务符号存活);Class II 是 Rust 相对 C **少做**的优化(优化机会被放弃)。

## 采集通道:编译器 remark

同一 LLVM 17.0.6 后端(clang-17 == rustc nightly-2024-01-15),让两侧编译器**自述**每个优化点的成败:
- **Rust**:`RUSTFLAGS="-Cdebuginfo=1 -Ctarget-cpu=native -Ctarget-feature=-avx512f -Cllvm-args=-pass-remarks[-missed/-analysis]=<regex>"`(remark 带 `!dbg` 源码位置)。
- **C**:`clang-17 -O3 -march=native -mno-avx512f -gline-tables-only -c <lib>.c -o /dev/null -Rpass[-missed/-analysis]=<regex>`。

### 采集陷阱(四个必守点,任一不守即得假数据)

1. **同名 flag 不叠加,单合并正则**。`clang -Rpass=` 与 rustc `-pass-remarks=` 各取**单个正则,后者覆盖前者**。分开传 `=loop-vectorize` `=slp-vectorizer` `=inline` 会被最后一个静默覆盖(只剩 `inline`、向量化恒 0 → 假"全损失")。必须写成一个合并正则:`-Rpass='loop-vectorize|slp-vectorizer|inline'`。
2. **两侧都要 debug info**。clang-17 的**向量化** remark 无 `-g` 不发射(内联 remark 不需要)。C 加 `-gline-tables-only`、Rust 加 `-Cdebuginfo=1`,否则得假"C 也没向量化"。
3. **两侧 no-LTO 对称**。fat-LTO 把向量化/内联推迟到 **link 阶段 plugin**,per-CGU `-Cllvm-args`/`-Cremark` 常**采不到**(实测 heman/libzahl/binn 的库 crate fat-LTO 下向量化 passed=0=漏采;内联则因全程序合并致 cost 虚高膨胀,伪造损失如 binn `copy_value`、heman "cost 6325")。故两侧都用 no-LTO 镜像看优化器对**模块内代码形态**的固有决策:C = per-TU `-c` 去 `-flto`;Rust = c2rust 库 crate(`rust_raw`)单独 no-LTO release,不用 harness fat-LTO。(例外:bzip2 是 self-contained bin,其 CGU 布局使 fat-LTO 与 no-LTO 报**同一批**向量化 remark、未漏采;但为全语料统一口径,一律用 no-LTO。)此镜像反映 c2rust 代码形态的**固有**可优化性,与实测二进制绝对性能是两回事。
4. **rust_raw 自带 LLVM 16 pin,必须 override 到 LLVM 17**(最隐蔽)。所有 `rust_raw/rust-toolchain.toml` 都 pin `nightly-2023-04-15`=rustc 1.70=**LLVM 16**(c2rust 输出旧 pin)。在 rust_raw 目录 `cargo build` 不加 `+nightly-2024-01-15` 就用 LLVM 16 → 与 C 的 clang-17/LLVM17 **后端不对称**(向量化/内联能力不同,违反 fair build 核心)。**必须 `cargo +nightly-2024-01-15 build`**(=rustc 1.77=LLVM 17.0.6)。实测:LLVM16→17 会改变**裸指针向量化判定**(libzahl 7 处 `cannot identify array bounds` 在 16 是 artifact、17 消失),但**不改 inline cost model**(xxHash cost 395>375 两版一致)——故向量化损失须 LLVM17 对称验证,内联损失版本稳健。某些 toolchain rustc `-Cllvm-args=-pass-remarks` 不发,改用 rustc 原生 `-Cremark=all`(`note:` 行,passed token `(success)`)。

两个方向:
- **II① 向量化损失**:pass = `loop-vectorize`(循环向量化)+ `slp-vectorizer`(直线代码 SLP 打包)。
  - passed 措辞:`vectorized loop` / `SLP vectorized` / `Stores SLP vectorized` / `Vectorized horizontal reduction`(cost 为负=有利)。
  - missed 措辞:`loop not vectorized` / `Cannot SLP vectorize: impossible with available factors` / `not beneficial cost N>=N`。
- **II② 内联损失**:pass = `inline`。
  - missed 措辞:`'callee' not inlined into 'caller' because too costly to inline (cost=X, threshold=Y)` / `should never be inlined`。
  - 机制:c2rust 生成的函数体膨胀(义务检查 + 无 `#[inline]` 提示)使 cost 超 LLVM inline threshold,C 里被内联的小函数在 Rust 里保持 out-of-line call。

### 其余「少做优化」通道已查证穷尽(全 pass 枚举)

用全 pass 正则(Rust `-Cremark=all` / C `-Rpass-missed='.*'`)在 bzip2/xxHash/libzahl/heman 热函数上枚举,除 vectorize/inline 外只有 licm/gvn/loop-unroll/loop-idiom/loop-delete 发 remark:
- **licm / gvn 的 `load not eliminated` / `can't hoist load with loop-invariant address` missed 簇 → 归 Class I C3**(别名缺失),不重复立 II 规则。措辞纯别名类(Rust 缺 `noalias`/`!tbaa` → 内存优化器保守),是 C3 冗余访存的优化器-remark 视角(libzahl `zrsh` gvn missed 46/16 = class_I 记录的 C3 load 73/24 同现象两口径);计数 per-instance 被宏展开/force-inline 混淆,以 C3 的 IR 访存计数为准。
- **loop-unroll / loop-idiom / loop-delete**:两侧 parity,无缺口。
- **loop-unswitch / gvn-sink / machine-licm**:LLVM17 两侧零发射,无可采。
- xxHash 的 licm/gvn C-passed≫Rust 是 II② 内联损失的**下游**(force-inline 融合体上运作),非独立通道。

⇒ **Class II 限定 II① 向量化 + II② 内联两方向,作为独立通道集合是穷尽的**(全 pass 枚举验证)。

## 过滤规则(去框架噪声)

- **向量化** remark 的位置是 callsite 源码 `文件:行`,按此过滤到**被翻译库源码 .rs**(排除 `/rustc/` std、harness `main.rs` 框架部分)。
- **内联** remark 位置是 callee 定义处,须按 **caller ∈ 热函数 且 callee ∈ 库内部函数** 过滤;并额外排除两类假损失:
  - **框架条目**:callee/caller 含 `core/alloc/std/demangle/gimli/miniz/backtrace/fmt/panic`(`panic=abort` 下 backtrace/格式化残留,不属被翻译库);`cost=never` 的 `noinline`(by-design)。
  - **cross-crate(LTO-resolvable)**:no-LTO 库 crate 采集里跨 crate 的 callee 报 not-inlined,但实测 fat-LTO 二进制在 link 阶段会内联它 → **不算真损失**(如 bzip2 `BZ2_hbCreateDecodeTables×20` 在 huffman crate)。真内联损失须是**同 crate/同 TU 内**因 c2rust 代码膨胀致 cost 越过阈值的条目。
  - **已内联(不同阶段都算已内联)**:callee 若被 rustc MIR inliner 在 LLVM 前折叠(LLVM 无 remark),或被 LLVM 内联器折叠(发 success remark),都是**已内联**而非损失。勿因"无 inline-pass remark"或"有 inline-missed 但另有 success"误判。(libcsv `csv_increase_buffer` 在 LLVM17 下经 **LLVM 内联器**显式 success、cost 110/250——非早期以为的 MIR 提前折叠;但两种终态都是"已内联"。)

## 计数口径(去重单位)

计数单位统一为 **unique `(file,line,col)` 站点**(或热 kernel 站点),**不用 raw remark 实例数**。同一源站点会被多次报告:(a) header-only / force-inline 的 callee 每份内联一条(xxHash C SLP raw 78 = **14 unique 站点**被内联进 ~40 caller;bzip2 C 侧库全量聚合 passed 同理 raw 虚高);(b) 流水线多轮跑同一 pass。**raw 聚合数跨侧/跨项目不可比,严禁进 ≥3 门槛**。门槛与 ● 判定一律锚定**热函数内 unique 站点**(bzip2 5 个热循环、xxHash accumulate 4 caller、libzahl zmul_ll 2 callee)。各 md 表中的库全量聚合数仅作背景,不作跨项目对比或门槛依据。

## 置信定义(●/◐)

- **●(强,解释 gap)**:同源逻辑下 **C passed 而 Rust missed**——纯优化器差异(同一 LLVM),c2rust 的 IR 形态让优化器放弃了 C 能做成的优化。
- **◐(中,不解释 gap 但改写有益)**:两侧都 missed,或 Rust passed ≥ C——gap 不由此优化差异产生,但主动改写(引导向量化/加内联提示)仍可让 Rust 绝对变快。

## 门槛(同 Class I)

一条规则 ≥3 支持实例;实例 = ≥3 种模式(同层面)**或**同一模式在 ≥3 个热函数/站点。跨项目广度不作准入,仅如实报告。

## 账本口径

复用 Class I 的热函数账本(perf self%,≥5% 门槛,workload 经 RQ1 判为回归);Class II 只在这些热函数(及其内联体)内统计 remark。
