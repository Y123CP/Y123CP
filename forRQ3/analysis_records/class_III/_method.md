# Class III 采集方法(翻译产物层,对照地道 Rust)

**层面界定**:Class III = c2rust 翻译产物**违背地道 Rust(The Rust Performance Book)性能最佳实践**的地方。与 Class I/II 三点不同:

| | Class I/II | **Class III** |
|---|---|---|
| 对照系 | C | **地道 Rust(perf-book)** |
| 判据 | Rust vs C 差分 | **c2rust 违背 perf-book 原理** |
| 层面 | 优化后 IR / remark | **源码层(`rust_raw/src`)** |
| 解释什么 | 为什么比 C 慢 | c2rust 离最优 Rust 多远(改后可**超越** C) |

## 原理主线(凝练轴)

Class III 的深层原理是**一条**:

> **c2rust 机械照搬 C 的"低级语义"(无类型字节、手动内存管理、运行时间接分发),没有升级到 Rust 的"高级抽象"(类型化容器、所有权/RAII、编译期单态化)——既丢失了给优化器的信息,又付了抽象缺失的运行时代价。**

每条规则是这条原理的一个**面**:A 回调=违背零成本抽象/单态化;C/D libc=违背所有权+类型化容器(优化器看不见长度/别名 + FFI 边界挡内联);E 减分配=违背分配昂贵/复用局部性;F 减类型=违背类型大小影响 cache/带宽;G I/O=违背 syscall 昂贵/批量缓冲。

## 语法 vs 原理(关键)

检测签名(`libc::malloc` / `extern "C" fn`)只是**定位手段**;规则的**本质是违背了 Rust 的哪条设计原理**。凝练**按原理,不按语法计数**——同 C1 本质是"语言语义义务"而非"grep panic_bounds_check"、D1 本质是"代码形态破坏向量化前提"而非"grep loop not vectorized"。

## 采集流程

1. **对照系**:perf-book 全书抽象出的地道 Rust 方向;
2. **范围**:回归热函数(≥80% 运行时间,同 I/II)的 c2rust 源码 `rust_raw/src`;
3. **判据**:c2rust 违背某方向 + **≥3 回归热函数复现**。**不看实测 perf 收益**——perf-book 权威 + 违背客观即可,收益验证留 Evaluation;禁止用"没性能收益"砍掉真违背;
4. **每方向 → 规则**:原理 + 检测签名 + 改写方向 + 安全条件 + 证据。

## 候选方向(perf-book 全书抽象,不限旧 4;开放发现)

- **A 回调 → 泛型**(Inlining,perf-book **点名 c2rust**)
- **C 手动内存 → RAII 容器**(Heap:`malloc/free/realloc` → `Vec`/`Box`)
- **D 手动内存操作 → slice**(Std-lib:`memcpy/memset/strlen` → `copy_from_slice`/`fill`/`&str`)
- **E 减分配**(Heap:热循环内重复 alloc → 循环外复用)
- **F 减类型**(Type-sizes:union+`transmute`、`as` cast 泛滥、`c_int`/`usize` 冗余)
- **G 缓冲 I/O**(I/O:`libc::fread/fwrite/fputc` 无缓冲 → `BufReader`/`BufWriter`)
- (开放:采集可报其他 perf-book 违背方向)

## 排除(诚实界定 + 理由)

- **B 裸指针 → 迭代器/slice**:c2rust 最普遍反模式(每个循环 `*p.offset(i)`),但其后果——边界检查、冗余访存、向量化失败——**已从 IR/remark 视角回填 Class I C1/C3 + Class II D1**;源码根因与 IR 后果是同一缺陷两个视角,不重复计门槛;
- **H 快速 hash**(FxHashMap):c2rust 不生成 Rust `HashMap`(保留 C 数组/手写表)→ 无违背对象;
- **I SIMD intrinsics**(machine-code "用 `core::arch`"):需**手写 SIMD**,超"机械改写翻译产物"范畴,且与 D1 向量化损失交叉;
- **J 并行**(rayon):perf-book 自述"beyond scope",需算法重构,超范畴。

## 置信

Class III 对照**地道 Rust**(非 C),故多为**提速方向**(改后 Rust 可超越 C),呼应"C 不是天花板"。个别有硬实证(A 回调单态化 Stage A 实测 libcsv −32%)。
