# Class III detector — SPEC (v3, cluster.md 对齐版)

**Class III = 翻译产物层违背地道 Rust(perf-book)最佳实践的源码形态.**
四条规则,凝练自 `empirical_study/RQ3/class_III/cluster.md`(6 项目采集层):

- **III① 回调 → 泛型单态化**  —— 违背零成本抽象 + 编译期单态化
- **III② 手动堆 → RAII 容器**  —— 违背所有权 + 类型化容器
- **III③ 手动 mem-op → slice** —— 违背类型化 slice 操作
- **III④ 裸指针游标 → slice/iter** —— 违背 slice/迭代器抽象;
  **源码总病因,rewrite 归 C1/C3/D1,不产独立 rewrite direction**

## §0 观察通道模型 —— Class III 独立通道

Class I 的两通道(remark / opt-IR)对 Class III 无关。Class III 观察面**只有**
CST(源码语法树)—— 不看 `.ll`、不看 remark、不 build。

| 通道 | 来源 | 语义 | 粒度 |
|---|---|---|---|
| **③翻译产物 Rust CST** | c2rust 输出的 `.rs` **tree-sitter-rust CST** | 翻译形态 —— 语法结构 + 类型标注 | 节点级 |

**四条规则对通道的组合**:

| 规则 | ③CST | 组合语义 | 归属粒度 |
|---|---|---|---|
| **III①** | ✓ fn ptr 签名 + body 间接 call | 纯③ | 函数 f |
| **III②** | ✓ callee ∈ Alloc(canonical + 对齐扩展)| 纯③ | 函数 f |
| **III③** | ✓ callee ∈ (libc-extern \ Alloc) ∪ CustomMemOpFns | 纯③ + body-pattern | 函数 f |
| **III④** | ✓ CST P1/P2 pattern(裸指针游标 deref / 后增)| 纯③ | 函数 f + site 详情 |

**关键设计决策 D0**:全部**用 tree-sitter-rust CST 遍历**,**不用 regex**。
regex 只在最外层"取候选站点"时可以粗筛加速(且必须与 CST 结果做交集
验证);任何**判定**(命中/不命中)都走 CST 结构判断。

## §1 III① 回调 → 泛型单态化

**违背的 Rust 原理**:零成本抽象 + 编译期单态化 —— 地道 Rust 用泛型让编译器
为每个具体 callee 特化 + 内联;c2rust 保留 C 的**运行时函数指针**
(`extern "C" fn` / `Option<unsafe extern "C" fn>` 派发表),每次调用付间接
跳转 + `Option` null-check,并**挡住内联**。

**通道**:纯③(CST)。**归属粒度**:函数 f。

### §1.1 fn ptr 绑定源(4 类)

一个 `call_expression` 的 `function` 子表达式其运行时值来源于一个 fn ptr
类型的绑定,不是一个具名 fn 定义。绑定源分类:

| 编号 | 绑定源 | CST 判定方式 |
|---|---|---|
| (i)   | fn 参数类型是 fn ptr | 从 `function_item` 的 `parameters` 子树取每个 param 的 type,判 `is_fn_ptr` |
| (ii)  | 局部 `let` 绑定的类型标注或 rhs 是 fn ptr(match arm 的 pattern-bound identifier 也归此,见 §1.3) | 遍历 `let_declaration`,类型标注命中 fn ptr 或 rhs 是已知 fn ptr 表达式 |
| (iii) | struct field 类型是 fn ptr / Option<fn ptr> | 从 `struct_item` 全项目扫,存 `{struct → {field → type}}`,receiver 类型判定回查 |
| (iv)  | module-level `static [mut] NAME: T = ...;` 类型是 fn ptr / Option<fn ptr> | 全项目扫 `static_item`,存 `{name → type}`;`build_local_scope` 里作为 fn scope 的 fall-back(local 优先,static 兜底);local shadow 生效 |

**post-batch audit 修正(v3.1)**:(iv) 是 c2rust 输出的**主流形态**,libxml2 里
`xmlFree/xmlMalloc/xmlMallocAtomic` 等 300+ 个 module-level `static mut ... :
Option<fn ptr>`;不做 (iv) 会 87% 假阴(实证 libxml2:334 hit → 2400 hit,+2059)。

### §1.2 检测四形态(A/B/C/D)

`call_expression` 节点为锚点。**A/B/C/D 是形态分类,与 §1.1 的绑定源
(i)(ii)(iii)(iv) 正交** —— 一个形态可能来自不同绑定源。

| 形态 | CST 结构(简写) | 例子 | 依赖绑定源 |
|---|---|---|---|
| A | `call(function: identifier, args)` | `f(x)` where f is fn param | (i) 或 (ii) |
| B | `call(function: call(function: method_call(receiver, expect\|unwrap\|unwrap_unchecked), args), args)` | `cb.expect("non-null function pointer")(x)` | (i) 或 (iii) 或 (iv) |
| C | `call(function: field_expression(receiver, field), args)` | `(*p).cb(x)` | (iii) —— field 类型 = fn ptr |
| D | `match_expression` arm body 里 `call(function: identifier, args)`,identifier 是 pattern-bound | `Some(f) => f(x)` | 需要 §1.3 pattern-scope 判定 |

### §1.3-§1.6 detail

参见 `iii1_callback.py` 源码 —— 判定链、alias 穿透、marker 观察、C1↔III①
划界等 detail 全在实现里 docstring 化。

### §1.7 Rewrite + Safety

```
fn f(.., cb: Option<extern "C" fn(A,B) -> R>, ..)
   ⟹ fn f<C: Fn(A,B) -> R>(.., cb: C, ..)      // 泛型化,单态化
gate:  ⊢ W1 功能等价  ∧  实测墙钟不退化
```

**Safety schema**:无正确性证明义务(rewrite 语义中性)。

## §2 III② 手动堆管理 → RAII 容器

**违背的 Rust 原理**:所有权 + 类型化容器 —— 地道 Rust 用 `Vec<T>` /
`Box<T>` / `String` 管理堆内存(带长度/容量、RAII 自动释放、给优化器别名
信息);c2rust 保留 C 的 `libc::malloc / free / calloc / realloc` **手动
管理**(把内存当无类型字节 + FFI 边界挡内联 + 优化器看不见生命周期)。

**RQ3 formula**:

```
P_III②(f) ≜ hot(f) ∧ ∃ call ∈ Alloc ∈ f
```

**通道**:纯③(CST)。**归属粒度**:函数 f。

### §2.1 Alloc union(canonical + 对齐扩展)

**Canonical(cluster.md 明列)**:

```
Alloc_canonical = { malloc, free, realloc, calloc }
```

**对齐扩展**(D5,默认收):

```
Alloc_extended = { aligned_alloc, posix_memalign, memalign, valloc }
```

**Alloc = Alloc_canonical ∪ Alloc_extended**。

**明确不收**(用户确认):
- `mmap / munmap / mremap` —— 文件映射语义,rewrite 方向不同(不换 Vec/Box)
- 项目自定义 alloc wrapper —— 与 III① 双计风险,不收

### §2.2 call site 匹配

同 SPEC §3.2 三档 callee resolution(EXTERNAL / INTERNAL / bare)。命中条件:
`cname ∈ Alloc ∧ cname ∉ NonExternBase(除非 EXTERNAL prefix)`。

### §2.3 检测层不做什么(SPEC §3.3 D7)

- **不判定** call 是否在循环里
- **不判定** alloc/free 是否配对
- **不判定** 是否可预分配

**理由**:rewrite direction 有多种(→ Vec/Box RAII / 移出循环 / 避免拷贝 /
预分配),各自需要不同上下文分析。**检测层只负责"这个 fn 有 alloc 出现"**
—— 交给 rule dispatcher + LLM。

### §2.4 Rewrite + Safety

```
临时 scratch 缓冲                        ⟹ vec![0; n] / Vec::with_capacity  (首选,RAII)
单一 owner 长期缓冲                      ⟹ Box<[T]> / Vec<T>
loop { let p = malloc(n); ... free(p); } ⟹ let p = malloc(n); loop { ... }; free(p);   (次选,减 alloc)
两次 malloc + memcpy 拼接               ⟹ 一次 malloc 直接写入
未知大小 malloc                          ⟹ 按已知上界预分配
gate:  ⊢ W1 功能等价  ∧  实测墙钟不退化
```

**无界涟漪警告**:换全局分配器(mimalloc/jemalloc)属重型,**不作默认 direction**。

## §3 III③ 手动 mem-op → slice 操作

**违背的 Rust 原理**:类型化 slice 操作 —— 地道 Rust 用
`copy_from_slice / fill / to_be_bytes / iter().copied()`(带长度信息 +
编译器可折叠成 SIMD/intrinsic);c2rust 保留 C 的手写逐字节 / 展开操作
(`memcpy` / `memset` 、手写字节序 swap `copy_be*`、Duff's-device 展开的
`libzahl_mem*`),丢失长度信息、优化不足。

**RQ3 formula**:

```
P_III③(f) ≜ hot(f) ∧ ∃ call g ∈ f,
             g ∈ (libc-extern \ Alloc) ∪ CustomMemOpFns
```

**通道**:纯③(CST + body-pattern for CustomMemOpFns)。**归属粒度**:函数 f。

### §3.1 libc-extern 集的动态派生

**不 hardcode 名字白名单**(D4)。步骤:

1. tree-sitter 抓项目所有 `foreign_mod_item` 节点(`extern "C" { ... }` 块)
2. 每块内 declare-only 的 fn signature 构成 `Ext = { fn_name → sig }`
3. `Ext` 里去掉 Alloc 集(见 §2.1)—— 与 III② 互斥

### §3.2 CustomMemOpFns 识别(body-pattern,cluster.md §III③ 用户选 b)

**动机**:cluster.md 明列 c2rust 特有的手写 mem-op wrapper(`copy_be*` /
`libzahl_mem*` 家族),名字不在 libc extern 集,但**本质是手写 mem-op**
—— rewrite direction 同 memcpy(`copy_from_slice` / `to_be_bytes`)。

**Body-pattern 判据**(实现见 `iii3_mem_ops.detect_custom_mem_op_fns`):
- 语句数 ≤ `CUSTOM_MEM_OP_MAX_STATEMENTS`(默认 24;覆盖 Duff's-device 4/8 字节展开)
- 分支数 ≤ `CUSTOM_MEM_OP_MAX_BRANCHES`(默认 1;允许 unroll head/tail if)
- `*p.<offset|add|sub|wrapping_*>(_)` 次数 ≥ `CUSTOM_MEM_OP_MIN_PTR_OPS`(默认 2)

三条同时满足 → fn 加入 CustomMemOpFns 集。所有 callers 里调它算 III③ hit。

### §3.3 call site 匹配

对 hot fn 内每个 `call_expression`,callee 三档判定(§2.4 通用逻辑):
- `cname ∈ (libc-extern \ Alloc)` —— libc 主判据
- **OR** `cname ∈ CustomMemOpFns` —— 自定义 wrapper 判据(INTERNAL prefix 也算命中,不 skip)

Ambiguous 处理(callee 名字既在项目 non-extern 集又在 libc extern 集,且
无 EXTERNAL prefix):call-site 级扣除,归 `iii3_ambiguous_callees` 桶。

### §3.4 Rewrite + Safety

```
memcpy(dst, src, n)      ⟹ dst[..n].copy_from_slice(&src[..n])
memset(dst, c, n)        ⟹ dst[..n].fill(c as u8)
strlen(p as *const c_char) ⟹ CStr::from_ptr(p).to_bytes().len()
strchr(p, c)             ⟹ slice.iter().position(|x| *x == c as u8)
snprintf(buf, n, ..)     ⟹ write!(&mut buf[..n], ..)
copy_be16/32/64(p, v)    ⟹ p[..N].copy_from_slice(&v.to_be_bytes())
libzahl_mem*(dst, src, n) ⟹ dst[..n].copy_from_slice(&src[..n])
libm sqrt/cos/sin/...    ⟹ f64::sqrt(..) / f64::cos(..)
gate:  ⊢ W1 功能等价  ∧  实测墙钟不退化
```

**Safety schema**:等价证明(slice bounds, encoding invariance)。

## §4 III④ 裸指针逐元素游标 → slice/迭代器

**违背的 Rust 原理**:slice/迭代器抽象 —— 地道 Rust 用
`.iter() / &[T] / slice[i]`(让编译器掌握长度 + 无别名 → 折叠边界检查 /
消冗余访存 / 自动向量化);c2rust 一律发 `*p.offset(i)` / `let fresh = p;
p = p.offset(1); *fresh = ..` 后增游标习语,**丢失边界与别名信息**。

**⚠️ 关键特殊性**(cluster.md §III④):

> III④ 的性能后果 = Class I C1(边界检查)+ C3(冗余访存)+ Class II D1
> (向量化失败)—— 同一缺陷,III④ 从**源码病因**看、C1/C3/D1 从 IR/remark
> 症状看。**改写收益归 C1/C3/D1,III④ 不产独立 rewrite direction**。

III④ hits 用作**源码病因归属信号**:让下游 LLM 拿到 C1/C3/D1 fn 命中时能
定位到具体源码 pattern site。类同 cluster.md D3 回填 C3 的处理。

**RQ3 formula**:

```
P_III④(f) ≜ hot(f) ∧ ∃ site ∈ f,  site 满足 P1 ∨ P2
```

**通道**:纯③(CST)。**归属粒度**:函数 f + site 详情(user 明确要求)。

### §4.1 原理准入(和 C1/D2 同款)—— 满足原理的两个语法族

**PTR_CURSOR_METHODS 两档**(2026-08-03 brotli audit + (b) 消歧扩展):

```
PTR_CURSOR_METHODS_STRICT     = { offset, wrapping_offset, add, sub }
PTR_CURSOR_METHODS_AMBIGUOUS  = { wrapping_add, wrapping_sub }
PTR_CURSOR_METHODS_UNION      = STRICT ∪ AMBIGUOUS
```

- **STRICT**:pointer-only 语义,usize/整数无这些方法,单看名字 100% 是 pointer
- **AMBIGUOUS**:pointer + usize/整数共用(c2rust 惯用 `i = i.wrapping_add(1)`
  做整数溢出安全自增,也用 `p = p.wrapping_add(1)` 做 pointer 后增)

**(P1) cursor-index-deref**:

```
*<expr>.<method>(<idx>)
   where <method> ∈ PTR_CURSOR_METHODS_UNION
```

**P1 无需消歧**:外层 unary `*` deref 已保证 receiver 是 pointer
(usize 不能 deref,编译报错),AMBIGUOUS method 在 P1 内 100% 精度。

**(P2) post-increment**(c2rust 后增游标习语的核心特征):

```
<var> = <var>.<method>(<n>)     — self-assign
```

- **method ∈ STRICT** → 直接命中(`pattern_kind = "post-increment"`)
- **method ∈ AMBIGUOUS** → **消歧启发式**:同一 fn body 里 `<var>` 需有
  `*<var>` bare-deref 或 `<var>.<STRICT_METHOD>(_)` 证据,才认为 pointer
  → 命中(`pattern_kind = "post-increment-disambiguated"`);否则视为
  整数循环变量,跳过

**消歧原理**:usize 不能 deref,STRICT method 也是 pointer-only,任一
证据出现就必是 pointer。实证 brotli:(a) 严格 58 fn → (b) 消歧 66 fn
(+13.8%),主要覆盖 c2rust `<name>_view` (`*mut c_int` 输出参数)模式。

**未收但同原理**(known limitation):
- `let fresh = <p>; <p> = <p>.<method>(<n>); *fresh = ...` 完整三行 pattern
  —— 现只抓中间那行(P2 已覆盖);第一行/第三行独立没抓
- `while p < end_p { ...; p = p.add(1); }` cursor-loop —— 中间那行 P2 覆盖
- `arr[i].method(...)` where `arr` 是 pointer 数组的 index —— 罕见

### §4.2 检测算法

对 hot fn 的 body 递归遍历 CST:

- 遇 `unary_expression(op="*", argument=X)` 且 X 是 `call(function=field(receiver, method))`
  且 method ∈ PTR_CURSOR_METHODS → 记 P1 CursorSite
- 遇 `assignment_expression(left=id, right=call(function=field(id, method), args))`
  且 left.text == right.function.value.text(self-assign)且 method ∈ PTR_CURSOR_METHODS
  → 记 P2 CursorSite

### §4.3 Rewrite direction —— 本卡自主(2026-08-03 决策 D30 推翻旧 SPEC)

**旧决策**(D24,2026-08-02):III④ 无独立 rewrite direction,rewrite 归
C1/C3/D1。**已推翻**。

**新决策**(D30):裸指针游标 → slice/iter 是**同一改写机械** 消 C1+C3+D1
三症状,LLM 拿到一张卡语义最自洽。因此 III④ 拥有自主 rewrite direction:

- **P1** `*p.<method>(idx)` → slice 索引(`slice[i]` 或 `slice.get_unchecked(i)`)
- **P2** `p = p.<method>(n)` 后增游标 → iterator(`for x in slice.iter()` /
  `slice.copy_from_slice(_)` / `CStr::from_ptr` 等)

两种策略(参见 III4 card §3):

| 策略 | 触发 | 收益 |
|---|---|---|
| **S1 签名 lift** | 游标是 fn 参数 + 签名可编辑 | 最高(noalias 永久恢复 + Rust idiomatic) |
| **S2 局部 reborrow** | 签名固定(extern "C" / wide pub ABI)或游标是局部变量 | 中(逐 site 恢复,签名 ABI 不动) |

**长度推断四档**(soundness 硬门槛):

- L1:同 fn 显式 `len: usize` 参数
- L2:结构体字段(需证 field 在区间内不被写)
- L3:编译期常量
- L4:sentinel 终止(`while *p != 0` → CStr;`while p < end` → iterator)

任一无匹配 = abstain 该 site(不瞎猜长度)。

**详细 recipe / safety obligation / rejection checklist**:见
`agent_perf_opt/Optimization_Card/III4_raw_ptr_cursor.md`。

III④ **独立进** W1/W2 gate rewrite 循环(与 C1/C3/D1 优先级见 §5.2)。

## §5 与 Class I/II 的正交去重(driver 层)

Class III 检测层**独立跑**,输出自己的命中集。**与 Class I/II 的去重放在
driver 层**,不放在 Class III 自身。

### §5.1 已知重叠

- **III① ∩ II_inl(未内联)**:同一 fn 里的 fn ptr 间接调用,II_inl 通道会看到
  `inline: TooCostly` remark(rewrite:`#[inline(always)]` hint 或缩小 body);
  III① 看到 fn ptr 类型 + 间接调用(rewrite:泛型化)。**互补**:III① 是根治,
  II_inl 是缓解。driver 层 `III① > II_inl` 优先。
- **III① ∩ C1**:`expect("non-null function pointer")` 是同一个 CST 站点的
  两面 —— 已在 iii1_callback.py §1.6 检测层划分(shallow use-def chain)。
- **III② / III③ ∩ 其他**:与 Class I/II 不直接重叠。
- **III④ ∩ C1/C3/D1(常见)**:III④ 是这三者的**共同源码病因**——一次
  slice/iter 化改写同时消三 IR 症状。**优先走 III④ 自主 rewrite**(D30);
  C1/C3/D1 仅在 fn 未命中 III④ 时兜底。

### §5.2 driver dedup 策略(2026-08-03 D16 后简化)

**旧方案**(per-rule 优先级 dispatch)已废弃。见 `agent_perf_opt/agent_design.md
§D16`:优先级表随规则数指数爆炸,且规则重叠时会出现"先 III④ slice 化 → 后
C1 叠 get_unchecked 破坏 slice"的双改冲突。

**新方案**:driver 汇总时,给每个 fn 打 `provenance ∈ {C1, C2, C3, II_vec,
II_inl, III①, III②, III③, III④}` set(可多值),然后 **打包全部匹配卡片**
交给 LLM,LLM 自决:

- 一次 prompt 携带该 fn 命中的所有卡片全文
- Prompt meta instruction 提示 LLM "these cards may describe distinct
  symptoms of the SAME root cause; prefer minimal rewrites that address
  maximum symptoms"
- LLM 输出的 fn 头行加 `// Applied rules: <list>` 声明本次实际改的规则
- W1/W2 gate 整段判定,失败降级到单卡重试(优先级 III④ > III① > III③
  > C3 > C1 > D1 > D2 > II_vec)

**每张卡片**只讲自己的 rewrite recipe,**不再** 出现 Cross-reference /
优先级 / 与其他卡片的关系讨论——那是 driver + prompt meta 的事,不是卡
片内容的事。

## §6 Build recipe

**Class III 只需 tree-sitter 解析源码,不需要 build**(便宜、快):

```python
from tree_sitter import Language, Parser
import tree_sitter_rust

parser = Parser(Language(tree_sitter_rust.language()))
tree = parser.parse(src_bytes)
```

**venv**:`script_c2rust/.venv/bin/python`(唯一环境,不要用 uv / root venv)。

**依赖包**:
- `tree-sitter` (Python binding, ≥0.23)
- `tree-sitter-rust` (Rust grammar)

**Wall clock**:全项目 100 files ~1s(纯 CST 解析,无 build)。

## §7 Public API

```python
from perf_opt.hot_probe.class_III import scan, ScanResult

result: ScanResult = scan(
    crate=...,                  # Path to c2rust output crate root
    hot_fns=None,               # Optional set[FnKey] satisfying hot(·)
    out_dir=None,               # Optional; writes class_III_hits.json
)
```

### §7.1 FnKey 定义

**统一 fn 标识 = crate qualified path** —— 项目内同名 fn(不同 mod / 不同
impl 块)不撞:

```
FnKey = str, 形如 "crate::mod::submod::foo"
     or fallback "crate::impl<T>::method"  (impl 块内 method)
     or fallback "<file_relpath>::<fn_name>"  (无法拼 qualified path 时)
```

### §7.2 返回值 schema

- `iii1_hits: {FnKey: [CallSite]}`         —— III① 命中 fn 与 call sites
- `iii2_hits: {FnKey: [CallSite]}`         —— III② 命中 fn 与 call sites
- `iii3_hits: {FnKey: [CallSite]}`         —— III③ 命中 fn 与 call sites
- `iii4_hits: {FnKey: [CursorSite]}`       —— III④ 命中 fn 与 cursor sites
- `libc_extern_set: set[str]`              —— 本项目派生的 libc extern 集
- `custom_mem_op_fns: set[str]`            —— body-pattern 识别的项目自定义 mem-op wrapper 集
- `iii1_deep_receiver: [CallSite]`         —— §1.4 receiver 深度超限桶
- `iii1_scrutinee_unresolved: [CallSite]`  —— §1.3 scrutinee 类型判不了桶
- `iii1_c1_boundary_unresolved: [CallSite]` —— §1.6 use-def chain 边界桶
- `iii2_ambiguous_callees: [CallSite]`     —— §2.4 同名歧义桶(alloc)
- `iii3_ambiguous_callees: [CallSite]`     —— §3.3 同名歧义桶(libc)

**dataclass**:

```python
CallSite   = (file, line, col, callee_name, marker_seen: bool)   # marker_seen: §1.5 观察证据
CursorSite = (file, line, col, pattern_kind, snippet: str)       # snippet: 命中源码单行片段,供 LLM prompt 引用
```

## §8 Files layout

```
class_III/
├── __init__.py               — public API re-export
├── SPEC.md                   — 本文档
├── config.py                 — 常量(alloc 集 / prefix 分档 / receiver depth /
│                                PTR_CURSOR_METHODS / body-pattern 阈值)
├── cst_utils.py              — tree-sitter-rust CST 遍历公共函数
├── iii1_callback.py          — III① 检测(4 形态 A/B/C/D + §1.3 mini scope resolver + §1.6 划界)
├── iii2_alloc.py             — III② 检测(callee ∈ Alloc + §2.4 ambiguous)
├── iii3_mem_ops.py           — III③ 检测(callee ∈ libc-extern\Alloc ∪ CustomMemOpFns
│                                + `detect_custom_mem_op_fns` body-pattern)
├── iii4_raw_ptr_cursor.py    — III④ 检测(P1 cursor-index-deref + P2 post-increment)
├── report.py                 — markdown 报告生成
├── __main__.py               — CLI 入口:python -m perf_opt.hot_probe.class_III
└── scan.py                   — 顶层 orchestrator
```

**依赖**:
- `class_III → tree_sitter, tree_sitter_rust`(强依赖,不做 regex fallback)
- **不依赖** `class_I` / `class_II` —— 独立跑,driver 层做 dedup

## §9 设计决策记录(cluster.md 对齐版)

| ID | 决策 | 选值 | Rationale |
|---|---|---|---|
| **D0** | 匹配技术 | **tree-sitter-rust,不用 regex** | regex 无法处理 pattern binding / scoped_identifier / callee 类型判定 / receiver 类型链 |
| **D1** | III② / III③ 同名歧义处理 | **call-site 级扣除 + audit 桶** | 不做单-crate scope resolution;call-site 级比 fn 级精细,漏比错好 |
| **D4** | III③ libc 白名单 hardcode? | **不**,动态派生自 `extern "C"` block | 支持 non-glibc(musl/uclibc)/ 项目特化 extern;避免 project-specific overfit |
| **D5** | III② 收扩展 alloc 变体? | **收**(`aligned_alloc`/`posix_memalign`/`memalign`/`valloc`)| 与 mem-op 互斥集对齐,不双计 |
| **D7** | 检测层是否做 loop-context / alloc-free 配对分析? | **不做** | 检测层只给候选 fn;上下文分析交给 rewrite 阶段的 LLM/dataflow |
| **D8** | III 与 I/II 去重放哪? | **driver 层**,不放在 Class III 自身 | 各类独立跑更简单;driver 用 provenance set + 优先级选主 direction |
| **D14** | III②/③ callee resolution 分档? | **scoped_identifier 前缀权威 → 3 分档判定** | 分档 1 (外部 prefix) / 分档 2 (项目内 prefix) / 分档 3 (裸 identifier 走 ambiguous 反查) |
| **D22** | III④ type-size 保留吗? | **不,cluster.md 排除 F 减类型无 perf-book 依据** | 旧 III④(hot data footprint / -Zprint-type-sizes)整套代码删除;编号 III④ 让位给"裸指针游标"(cluster.md 新总纲) |
| **D23** | III② ↔ III③ 编号如何? | **按 cluster.md 互换**:III② = 手动堆,III③ = mem-op | 旧代码里 iii2_libc = III②,iii3_alloc = III③,现互换:iii2_alloc, iii3_mem_ops |
| **D24** | III④ 是否独立 rewrite? | **不独立,附给 C1/C3/D1** | cluster.md §III④ 明说源码病因,rewrite 归 IR 症状规则,不重复计门槛 |
| **D25** | III③ CustomMemOpFns 识别方式? | **body-pattern**(短 body + 高 ptr-op 密度 + 少分支) | 用户明确选 body-pattern(准确)而非命名启发式;`copy_be*` / `libzahl_mem*` 家族典型形态 |
| **D26** | III④ 检测粒度? | **fn 主键 + site 详情**(file/line/col/pattern_kind/snippet)| 用户明确:主 key = fn(用于命中归属),value 附 site 详情(供下游 C1/C3/D1 rewrite prompt 用) |
| **D27** | III④ PTR_CURSOR_METHODS 是否收 `wrapping_add`/`wrapping_sub`? | **两档(D28 补充):STRICT + AMBIGUOUS** | 2026-08-03 brotli 系统性审查发现:usize/整数也用 `.wrapping_add(1)` 表达溢出安全自增,单看语法假阳率 90%+ |
| **D28** | 如何用消歧启发式收回 wrapping_add/sub 的 pointer 场景? | **P1 全收(*deref 保证 pointer);P2 STRICT 直接命中 / AMBIGUOUS 需 body 内 var 有 `*var` 或 `.STRICT(_)` 证据** | usize 不能 deref + STRICT 是 pointer-only,任一证据出现必是 pointer;实证 brotli 58→66(+13.8%),覆盖 c2rust `<name>_view` 输出参数模式 |
| **D30** | III④ 拿回自主 rewrite direction(推翻 D24)| **是**:III④ 自主 recipe(P1 → slice 索引 / P2 → iterator);driver 优先级 III④ > C1/C3/D1 | 裸指针游标 → slice/iter 是**同一改写机械**,一次改写消 C1+C3+D1 三症状;LLM 拿一张卡语义最自洽;C1/C3/D1 仅在 fn 未命中 III④ 时兜底(非裸游标形态);见 `Optimization_Card/III4_raw_ptr_cursor.md` |

## §10 已知限制

1. **III① 跨函数 fn ptr 传播不覆盖** —— 若 fn ptr 从 caller 传入未标注类型
   的泛型参数,CST 判定不到。
2. **III① 跨 block scope 的 use-def chain 不覆盖** —— 归 `iii1_c1_boundary_unresolved` 桶。
3. **III① receiver 深度 > 3 层不覆盖** —— 归 `iii1_deep_receiver` 桶。
4. **III③ CustomMemOpFns body-pattern 阈值** —— body 极长的 mem-op wrapper 可能漏检
   (超 MAX_STATEMENTS 会被过滤)。用户可调 config 常量放宽。
5. **III④ 未覆盖形态**:
   - `let fresh = p; p = p.offset(1); *fresh = ..` 完整三行 pattern —— 只抓中间 P2 那行
   - `while p < end_p { ... p = p.add(1); }` cursor-loop —— 只抓中间 P2 那行(loop-context 未做)
   - `arr[i].offset(j)` 指针数组索引后偏移 —— 罕见
6. **hot(f) 澄清** —— 与 Class I/II 一致,`hot(f)` 由外部候选池提供,不必
   来自 workload profiling;可以是任何进入候选池的 fn。

## §11 与 cluster.md 的一致性检验

| cluster.md 条目 | SPEC 落点 | 状态 |
|---|---|---|
| III① 回调 → 泛型 | §1 | ✓ |
| III② 手动堆 → RAII | §2 | ✓ |
| III③ 手动 mem-op → slice | §3 | ✓ |
| III③ CustomMemOpFns(copy_be* / libzahl_mem*) | §3.2 body-pattern | ✓ 用户选 (b) 落地 |
| III④ 裸指针游标(source-cause,回填 C1/C3/D1) | §4 + §5.1 | ✓ D24 明确 |
| 排除 F 减类型(旧 III④ hot data footprint) | D22 删除 | ✓ |
| 排除 E 减分配(热循环内 0-1 处) | 未立独立规则 | ✓ 归 III② §2.4 次选 direction |
| 排除 G 缓冲 I/O(热函数内全冷 fprintf) | 未立独立规则 | ✓ 不覆盖 |
| 收益非准入,gate 判 | §0 header | ✓ |
