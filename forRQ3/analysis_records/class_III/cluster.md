# Class III 规则凝练(翻译产物层,对照地道 Rust)

**输入**:`class_III/_method.md`(方法)+ 穷尽重采(6 候选 + 开放发现,判据=违背 perf-book + ≥3 回归热函数复现,**不看实测收益**)。
**层面界定**:Class III = c2rust 翻译产物**违背地道 Rust(The Rust Performance Book)最佳实践**的源码模式。对照系是**地道 Rust**(不是 C);检测在**源码层**(`rust_raw/src`);解释的是"c2rust 离最优 Rust 多远"(改后可**超越** C),不是"为什么比 C 慢"(那是 Class I/II)。

## 原理主线(凝练轴)

一条深层原理贯穿全部规则:

> **c2rust 机械照搬 C 的"低级语义"(无类型字节、手动内存管理、运行时间接分发),没有升级到 Rust 的"高级抽象"(类型化容器、所有权/RAII、编译期单态化)——既丢失了给优化器的信息,又付了抽象缺失的运行时代价。**

四条规则是这条原理的四个面。**凝练按原理,不按语法**:`libc::malloc`/`extern "C" fn` 只是检测签名,规则的本质是违背了 Rust 的哪条设计原理。

## 凝练原则(三条)

1. **对照地道 Rust,不对照 C**:判据是"c2rust 是否违背 perf-book 方向",perf-book 权威担保方向的性能意义;
2. **≥3 回归热函数复现**才立,**不看实测收益**(收益验证留 Evaluation,禁用"没性能收益"砍真违背);
3. **同原理合并**:造成同一"抽象缺失"的多种源码模式合成一条。

---

## III① 回调 → 泛型单态化(perf-book Inlining **点名 c2rust**)

- **违背的 Rust 原理**:**零成本抽象 / 编译期单态化**。地道 Rust 用泛型让编译器为每个具体 callee 特化 + 内联;c2rust 保留 C 的**运行时函数指针**(`extern "C" fn` / `Option<unsafe extern "C" fn>` 派发表),每次调用付间接跳转 + `Option` null-check,并**挡住内联**(callee 对优化器不透明)。
- **检测签名**:热函数签名/字段含 `Option<unsafe extern "C" fn(...)>`,热路径内 `f.expect("non-null function pointer")(...)` 间接调用。
- **改写 + 安全**:fn 指针派发 → 泛型参数 / 直接调用具体 callee,消间接跳转 + null-check + 解锁内联(语义等价,安全档;等价门兜底)。
- **≥3 支持实例**:libcsv `csv_parse`(cb1/cb2/is_space/is_term,解析循环内 22 处间接调用)、xxHash `XXH3_hashLong_internal_loop`(f_acc/f_scramble accumulate/scramble 热循环内 3 处)、bzip2 `BZ2_decompress`(bzalloc/bzfree 分配器派发 3 处)。✅
- **实证**:libcsv 回调单态化 Stage A 实测 **−32%**(最硬证据)。

## III② 手动内存管理 → RAII 容器(perf-book Heap)

- **违背的 Rust 原理**:**所有权 + 类型化容器**。地道 Rust 用 `Vec`/`Box` 管理堆内存(带长度/容量、RAII 自动释放、给优化器别名信息);c2rust 保留 C 的 `libc::malloc/free/calloc/realloc` **手动管理**(把内存当无类型字节 + FFI 边界挡内联 + 优化器看不见生命周期)。
- **检测签名**:热函数体内 `libc::malloc/free/calloc/realloc` 手动分配/释放。
- **改写 + 安全**:临时 scratch 缓冲 → `vec![0; n]`/`Vec`/`Box`(RAII);自定义内存池路径可提示但重写风险高,如实标注。
- **≥3 支持实例**:heman `transform_to_distance`(scratch ff/dd/zz/ww,8 处 calloc/free,干净可 `vec!`)、heman `heman_lighting_compute_occlusion`(startpts,2 处)、libzahl `libzahl_realloc`/`zfree`(size-bucket 内存池本体)。✅

## III③ 手动内存操作 → slice(perf-book Std-lib)—— 覆盖最广

- **违背的 Rust 原理**:**类型化 slice 操作**。地道 Rust 用 `copy_from_slice`/`fill`/`to_be_bytes` 等(带长度信息、编译器可折叠成 SIMD/intrinsic);c2rust 保留 C 的**手写逐字节/展开操作**(`memcpy`/`memset`、手写字节序 swap `copy_be*`、Duff's-device 展开的 `libzahl_mem*`),丢失长度信息、优化不足。
- **检测签名**:热函数体内 `memcpy/memset/strlen`、手写 `copy_be16/32/64`、项目自定义 `*_mem*` 逐字节展开。
- **改写 + 安全**:`copy_be*` → `to_be_bytes`/`from_be_bytes` + `copy_from_slice`;`*_mem*`/`memset` → `copy_from_slice`/`fill`;`strlen` → slice 长度/`CStr`(语义等价,安全档)。
- **≥3 支持实例(8 热函数 / 3 项目,最强)**:binn `AddValue`(strlen/memcpy/copy_be* 8 处)、`GetValue`(memset/copy_be* 5)、`AdvanceDataPos`(copy_be32 2);libzahl `libzahl_realloc`/`zadd_unsigned_assign`/`zrsh`/`zlsh`(libzahl_mem* 展开);heman `compute_occlusion`(memset)。✅

## III④ 裸指针逐元素游标 → slice/迭代器(perf-book Iterators/Bounds-Checks)—— 源码总病因,后果回填 C1/C3/D1

- **违背的 Rust 原理**:**slice/迭代器抽象**。地道 Rust 用 `.iter()`/`&[T]` 让编译器掌握长度+无别名 → 折叠边界检查、消冗余访存、自动向量化;c2rust 一律发 `*p.offset(i)` / `let fresh=p; p=p.offset(1); *fresh=..` 后增游标,**丢失边界与别名信息**。
- **这是 c2rust 最普遍的源码病因**(20+ 热函数、全 6 项目)。
- **⚠️ 后果回填,不重复计门槛**:III④ 的性能后果 = **Class I C1(边界检查)+ C3(冗余访存)+ Class II D1(向量化失败)**——同一缺陷,III④ 从**源码病因**看、C1/C3/D1 从**IR/remark 症状**看。**改写收益归 C1/C3/D1,III④ 不重复计入 Table 7 门槛**(单列仅为点出源码总病因,同 D3 回填 C3 的处理)。
- **检测签名**:热循环内 `*ptr.offset(i)` 游标 / 后增指针习语。
- **改写 + 安全**:裸指针游标 → slice 索引 / 迭代器,使优化器可证无别名、有界(= 同时解锁 C1 折叠 + C3 消冗余 + D1 向量化;安全条件同 C3 的不别名证明)。
- **支持实例**:全 6 项目 20+ 热函数(bzip2 `BZ2_decompress`/`unRLE_*` 107+、libcsv `csv_parse` 17、heman 7 热函数、binn 3、libzahl 3、xxHash `XXH3` loop)。✅✅

---

## 原理主线串联 + 与 Class I/II 交叉

- **四条 = 一条原理的四个面**:c2rust 照搬 C 低级语义、未升级 Rust 高级抽象——III① 缺**单态化**、III② 缺**RAII 容器**、III③ 缺**类型化 slice 操作**、III④ 缺**slice/迭代器抽象**;
- **III④ 是源码总病因,C1/C3/D1 是其 IR 后果**(病因 vs 症状,同一缺陷两层视角);
- **Class III 改写多为"提速方向"**(对照地道 Rust,改后 Rust 可**超越** C,呼应"C 不是天花板");individual 硬实证 = III① libcsv −32%。

## 排除记录(用同一门槛复查,客观 <3 或无 perf-book 依据)

- **E 减分配**:热循环内重复 alloc 实际 0–1 处(scratch 全在循环外、libzahl 皆一次性增长、xxHash buffer 已外提)→ **客观无实例**,不立(非"无收益")。
- **F 减类型**:`as` cast 泛滥(csv_parse 182、bzip2 ~1656…)但 **perf-book 无"避免 as cast"方向**(Type-sizes 讲缩小类型尺寸,非删 cast),无依据;有依据的 union+`transmute` 仅 binn `GetValue` 1 处 <3 → 不立。
- **G 缓冲 I/O**:热函数内仅 bzip2 2 个 `verbosity`-gated `fprintf`(冷路径)、libcsv `fputc` 非该 workload 热点 → <3 且全冷,不立。
- **次要结构注记(不立)**:c2rust `current_block: u64` goto-仿真状态机(bzip2 `BZ2_decompress` 3 臂、binn `AddValue`)碎片化热循环、阻断 slice/iterator 重构——**非 perf-book 方向且仅 2 热函数**,仅作结构注记。

## 小结

Class III = **4 条**,揭示 c2rust "照搬 C 低级语义、未升级 Rust 高级抽象"的源码病灶:
- **III①(回调→泛型)** — 3 热函数/3 项目,perf-book 点名 + −32% 实证;
- **III②(手动内存→RAII)** — 4 热函数/2 项目;
- **III③(手动 mem-op→slice)** — 8 热函数/3 项目,覆盖最广;
- **III④(裸指针→slice/迭代器)** — 20+ 热函数/全 6 项目,源码总病因,后果回填 C1/C3/D1、不重复计门槛。

检测统一在 **Rust 侧源码**(对照 perf-book,不需 C);改写皆**语义中性**(安全档,等价门兜底),多为提速方向(可超越 C)。E/F/G 客观不达标(无实例 / 无 perf-book 依据 / 全冷),诚实排除。
