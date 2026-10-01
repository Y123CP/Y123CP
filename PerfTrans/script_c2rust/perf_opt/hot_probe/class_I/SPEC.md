# Class I detector — SPEC

**Class I = c2rust 前端多生成的性能缺口(M-a from RQ3).** 三条规则,
按 IR 层性能缺口向量聚类得到 —— 见 `empirical_study/result.md` §结果 2:

- **C1 冗余检查分支**  —— gap = 条件分支 + 冷 panic 块
- **C2 标量转换膨胀**  —— gap = 饱和 cast 的多指令 min/max/select
- **C3 冗余访存**      —— gap = 循环内同址 load 反复(TBAA 缺失致优化器保守)

本文档描述**部署期检测**(deploy-time),即 **Rust-only** 检测——只读目标
项目 Rust 侧的**优化后 LLVM IR** 和 **rustc `-C remark=all` 输出**,
不依赖 C 参照(C 对照仅在研究期用于归 ●/◐ 档,见 result.md line 340)。

## §0 两通道模型(设计一等公民)

**检测有两条独立信号通道**:

| 通道 | 来源 | 语义 | 粒度 |
|---|---|---|---|
| **①编译器优化信号** | rustc `-C remark=all` stderr | 优化 pass 自述:试了什么、成功或失败、失败原因 | file:line(粗)|
| **②优化后 IR** | rustc `--emit=llvm-ir` 出的 `.ll`(fat-LTO 全程序模块)| 优化 pass **全跑完后**代码的**最终形态** | 指令级(细)|

**5 条规则对两通道的组合**(本文档只讲 Class I 的 3 条):

| 规则 | ①编译器信号 | ②优化后 IR | 组合语义 | 归属粒度 |
|---|---|---|---|---|
| **C1** | — | ✓ callee 名匹配 | 纯②:C1 panic runtime 幸存 | 站点 s(经 inlinedAt 链归到源 fn)|
| **C2** | — | ✓ intrinsic 名匹配 | 纯②:饱和 cast intrinsic 幸存 | 站点 s |
| **C3** | ✓ gvn/licm miss remark | ✓ `!tbaa = 0` | **①∩②** | 函数 f |

**一次 build 服务多个规则**:RUSTFLAGS `-C remark=all -C debuginfo=1 --emit=llvm-ir`
一次 cargo build 同时产出 remark(通道①)和 `.ll`(通道②);C3 的两通道
AND 需要**同一 build 出的两份 artifact**,不是优化考虑,是**正确性要求**。

## §1 C1 冗余检查分支

**RQ3 formula**(result.md line 373-377):

```
P_C1(s) ≜ hot(s) ∧ optIR(s) ⊇ { call panic_bounds_check
                              | call core::option::expect_failed
                              | call core::panicking::panic }
```

**通道**:纯② (优化后 IR)。**归属粒度**:站点 s。

### 检测(通道②)

**Union 准入判据(原理制,非枚举制)**——一个 callee 属 C1 union 当且仅当同时满足:

- **(i) Rust 语义强制**:前端不能不生成(bounds check / unwrap / division-by-zero / borrow-state 等)
- **(ii) IR 形态**:优化后 IR 是"条件分支 + 冷 panic 块"(控制流层多做的活)
- **(iii) rewrite family**:属"证明前置条件 → `_unchecked` 变体"家族;安全档 = 值域/状态/并发证明

**"是否曾在 RQ3 6 项目采集层出现"是覆盖事实,不是准入门槛**。规则本身的 ≥3 独立热函数门槛由 `panic_bounds_check` + `expect_failed` 已过(cluster.md §C1 支持实例),同原理成员**继承**规则准入,不需要各自凑门槛。

**Callee union**(见 `rules.py::_C1_CALLEE_PATTERNS`,按语法源分组):

*索引 / slice-range checks(源:`arr[i]`、`slice[a..b]`)*
| Pattern | 语法机制 |
|---|---|
| `panic_bounds_check` | `arr[i]` 越界 |
| `slice_start_index_len_fail` | `slice[a..]` 且 `a > len` |
| `slice_end_index_len_fail` | `slice[..b]` 且 `b > len` |
| `slice_index_order_fail` | `slice[a..b]` 且 `a > b` |
| `slice_end_index_overflow_fail` | slice 长度算术溢出(slice-语义驱动,非通用 `with.overflow`)|
| `slice_error_fail` | str 切片错误 |

*Option/Result state checks(源:`.expect` / `.unwrap`)*
| Pattern | 语法机制 |
|---|---|
| `expect_failed` | `Option::expect(msg)` / `Result::expect` |
| `unwrap_failed` | `Option::unwrap` / `Result::unwrap` |

*算术语义 checks(源:`/`、`%`、`RefCell::borrow`)*
| Pattern | 语法机制 |
|---|---|
| `panic_const_div_by_zero` | `a / b` 且 const-eval 判 `b = 0` |
| `panic_already_borrowed` | `RefCell::borrow()` |

**Excluded patterns**(见 `rules.py::_C1_EXCLUDED_PATTERNS`)——**违反准入原理**故排除:

`panic_in_cleanup` / `panic_fmt` / `panic_cannot_unwind` / `panic_nounwind` /
`__rust_panic_cleanup`(泛化 panic runtime,违反 (iii):无 `_unchecked` rewrite)、
`assert_failed`(用户显式断言意图,违反 (i):不是前端强制)。cluster.md §排除记录
另有 `with.overflow` 通用整数溢出、generic `core::panicking::panic` runtime 一并
遵循同一逻辑(过粗家族匹配 → 用精确子符号代替)。

**Precedence**:排除集**优先**于包含集(defensive,防止 substring 撞击)。

### Rewrite + Safety schema

**Instance-specific rewrite direction + safety proof**(result.md line 379-383):

```
arr[i]                     ⟹ arr.get_unchecked(i)          provided ⊢ i < len(arr)
slice[a..b]                ⟹ slice.get_unchecked(a..b)     provided ⊢ a ≤ b ∧ b ≤ len(slice)
x.expect(m)()              ⟹ (x.unwrap_unchecked())()      provided ⊢ x ≠ None
x.unwrap()                 ⟹ x.unwrap_unchecked()          provided ⊢ x ≠ None(or Ok(_))
a / b                      ⟹ a / b(保持)                   provided ⊢ b ≠ 0
RefCell::borrow()          ⟹ RefCell::borrow_unchecked_hint provided ⊢ 无并发 borrow_mut
```

**Rewrite family**:precondition-proof + `_unchecked` 变体。
**Safety schema**:instance 级 —— 值域证明(idx-range / non-zero / non-null)、
状态证明(Option non-None / Result Ok)、并发证明(RefCell exclusive)。

### 归属

**站点级**(每处 `call` 指令一个 site):
- 用 `!dbg !N` 的 inlinedAt 链走到最内层非 stdlib 帧 → **define fn**(源函数)
- 用链的最外层帧 → **host fn**(perf self% 归属对象)
- std/core 泛型帧(`Option::expect` 等)穿透(result.md line 324)

**无 `!dbg` fallback**(RQ3 line 46 "计入自身"):
用当前 enclosing `define` 的 `linkageName` 反查 DISubprogram → 归到源 fn。
若 define 无 DISubprogram(如 miniz_oxide build-dep 无 debuginfo)→ 归 `STDLIB_ONLY`。

**代码**:`class_I/attribute.py`(rule-agnostic)。

## §2 C2 标量转换膨胀

**RQ3 formula**(result.md line 405-406):

```
P_C2(s) ≜ hot(s) ∧ optIR(s) ⊇ { llvm.fptosi.sat.* | llvm.fptoui.sat.* }
```

**通道**:纯②。**归属粒度**:站点 s(同 C1)。

### Union 准入判据(原理制,同 C1)

一个 callee 属 C2 union 当且仅当同时满足:

- **(i) Rust 语义强制**:浮点→整数 `as` 在 Rust 是**饱和**语义(NaN → 0、越界 → 钳到 `I::MIN`/`I::MAX`),前端必须展开成 min/max/select 多指令;C 的 `(int)x` 越界是 UB,一条 `cvttsd2si` 完事
- **(ii) IR 形态**:优化后 IR 里以 `llvm.fptosi.sat.*` / `llvm.fptoui.sat.*` intrinsic 存活(标量层多做的算术)
- **(iii) rewrite family**:`to_int_unchecked::<I>()`(证明有限+在界)或 `f.clamp(lo,hi) as I`(值域不可证时显式钳位)

### 检测(通道②)

**Callee 匹配**:
- `llvm.fptosi.sat.*` —— 有符号饱和 float→int
- `llvm.fptoui.sat.*` —— 无符号饱和 float→uint

**覆盖变体**:标量(`.i32.f64` etc.)+ 向量(`.v2i32.v2f64` / `.v8i8.v8f32` etc.)—— 
substring 匹配 `llvm.fptosi.sat` 或 `llvm.fptoui.sat` 一次搞定所有变体。

### Rewrite + Safety schema

```
f as I  ⟹ f.to_int_unchecked::<I>()   provided ⊢ f ∈ [I::MIN, I::MAX] ∧ ¬f.is_nan()
        ⟹ f.clamp(lo, hi) as I         (值域不可证时,显式钳位以助优化器融合)
```

**Rewrite family**:precondition-proof + `_unchecked` 变体(同 C1 的技术家族)。
**Safety schema**:值域 + NaN 证明。

### 一次限制

C2 只捕捉 **`f as i32/u32` 等浮点→整数** 一支;整数↔整数 `as` 已被 RQ3 结果 1
证明零成本(`mainGtU` 356 < 465),违反原理准入 (i)(无 Rust 语义强制的额外
工作),**不扫**。

### 广度说明(cluster.md §C2 语义)

C2 在 6 项目采集层**只在 heman 单项目触发**(语料唯一浮点密集项目)。这不是
empirical 短板 —— 规则通用性由 **Rust `as` 饱和语义的语言规范担保**,不需要
跨项目广度。cluster.md §C2 支持实例门槛靠"同一模式 5 热函数"(全 heman)
满足;任何有浮点热路径的 c2rust 项目都会命中,与 empirical 语料无关。

## §3 C3 冗余访存

**RQ3 formula**(result.md line 442-445,deploy 版):

```
P_C3(f) ≜ hot(f) ∧ remark(f) ⊇ { gvn:LoadClobbered,
                                 licm:LoadWithLoopInvariantAddressInvalidated }
                ∧ !tbaa = 0(Rust 侧)
```

**通道**:**①∩②**(两通道 AND)。**归属粒度**:函数 f。

### Union 准入判据(原理制,同 C1/C2)

一个 fn 属 C3 命中当且仅当:

- **(i) Rust 语义强制**:rustc 不发 `!tbaa` → `*mut/*const` 无 noalias 保证 → 优化器保守,GVN/LICM 无法证明"循环内的写不碰被缓存的地址",跨迭代 load 不能消、循环不变 gep 不能外提;C 侧带 `!tbaa` 的访存可被 GVN/LICM 清掉
- **(ii) IR 形态**:优化后 IR 中冗余 load 表现(remark 通道的 gvn:LoadClobbered / licm:Invalidated 是 LLVM 亲口报告的等价证据)
- **(iii) rewrite family**:恢复 noalias(改签名 `fn f(*mut T) → fn f(&mut [T])`)或标量外提(`let v = (*p).field; for L { … v … } (*p).field = v`);安全档 = 不别名证明

### 主筛选信号 = remark(重要澄清)

现有 `c3.py` 判据形式是 `remark ∩ !tbaa=0`,但由于 **c2rust 输出的 `!tbaa=0` 恒成立**(cluster.md 明说 6/6 项目 Rust !tbaa 全 0),这个 AND 事实上**只靠 remark 一条做 per-fn 筛选**。`!tbaa=0` 退化为"目标是不是 c2rust 项目"的背景 sanity check,不参与 per-fn 判别。

**这是有意为之,不是设计漂移**:
- LLVM 的 GVN/LICM pass **本身就做完了** alias analysis + 冗余 load 判定;remark 就是它亲口报告的判定结果 —— 权威、精度高
- 从 IR 侧从头做冗余 load 检测 = **重造 LLVM 的 alias analysis + LICM/GVN 决策逻辑**(LLVM 源码几千行 C++,Python 侧的简化实现要么覆盖率低、要么假阳率高)
- 理论上限:IR 侧检测得再好也**不可能强于 remark**,只可能等于或弱于
- remark 通道天然与 `class_II` 共享同一次 build(`-C remark=all` 一起出),不是额外耦合

**cluster.md §C3 检测签名里"∃ 循环不变 load 未外提 / 跨迭代冗余 load"这一条**,在 cluster 语境里是"IR 侧等价物"的表述,不是"必须用 IR 侧不能用 remark"。我们的部署实现**直接用 remark 作为等价信号**,理由如上;文档保留 `!tbaa=0` 作为项目级 sanity check,以便未来若遇到 !tbaa>0 的 Rust 输出(如非 c2rust 来源)能触发 mechanism warn。

### `⊇` 读法(重要设计决策 F5)

**采用 OR 读法(机制读法)**,即 "任一 remark 存在即 C3 命中",不是严格集合论
读法(两个都要)。理由:

- **同机制**:gvn:LoadClobbered("消不掉")和 licm:Invalidated("外提不了")是
  **同一别名机制**(`store σ` 可能 alias `load ℓ` → 优化器保守)在两个 pass
  阶段的表现。
- **empirical**:RQ3 Discovery set 里强 case(bzip2 BZ2_decompress)两种都有,
  但弱 case 可能只有一种。
- **fallback**:`!tbaa = 0` 是 c2rust 通用背景事实(不是 per-fn 判别),
  作为**机制假设的 defensive assertion**。若哪天遇到 !tbaa > 0 的 Rust 输出,
  机制假设失效,该 fn 放到 `c3_dropped_tbaa` 桶,logger warn。

将来若需收紧到 AND,改 `class_I/c3.py::apply_c3_predicate` 一行即可,已在
docstring 里 explicit 记录了 trade-off。

### 检测:通道①(编译器信号)

**Remark 分类**(在 `class_II/rules.py::classify` 里做——因 C3 和 II 共用
一条 remark stream,分类逻辑放在 class_II)。经 T1 (2026-07-23) 实证:

| Name | LLVM stderr message 正则 | Pass | Status |
|---|---|---|---|
| `LoadClobbered` | `load of type .+ not eliminated` | `gvn` | `missed` |
| `LoadWithLoopInvariantAddressInvalidated` | `may invalidate its value` | `licm` | `missed` |

**明确不收**:`licm:LoadWithLoopInvariantAddressCondExecuted`(`conditionally executed`)——
RQ3 formula 里没列出;它是**独立的**别名信号(条件执行的 load),不属 C3。

### 检测:通道②(优化后 IR)

**`!tbaa` 计数**(见 `class_I/c3.py::count_tbaa_per_fn`):

- 每个 `define` 内扫 `!tbaa !<N>` 出现次数(regex `_RE_TBAA`)
- **预期**:c2rust 输出**恒为 0**(RQ3 result.md line 430 实测)
- 若某 fn > 0:机制假设被破坏,归到 `c3_dropped_tbaa` 集合供 audit

### Rewrite + Safety schema

```
fn f(.., p: *mut T, ..)  ⟹  fn f(.., p: &mut [T], ..)          (改类型获 noalias)
或标量外提:let v = (*p).field; for _ in L { … 用 v … } (*p).field = v
provided ⊢ writes(L) ⊥ addr(p.field)                          (排除自别名 a==b / 区间重叠)
```

**Rewrite family**:安全 reborrow / slice 转换 / 局部变量缓存。
**Safety schema**:不别名证明(基于 fn 签名 + 调用现场分析)。

### 归属

**函数级**:remark 的 `file:line` → `FnIndex.resolve` 得包含该行的 crate 源函数。
基名 fallback:文件全路径查不到时按文件名匹配。

Fn 名 → mangled 反查(TBAA 计数聚合到源 fn 时):用 Rust `<len><name>`
length-prefix 匹配(F4 fix),不用裸 substring —— 短名 fn(`add` 3 chars,
`filter` 6 chars)裸 substring 假阳性严重。

## §4 归属基础设施(rule-agnostic,C1/C2 共用)

**IR 元数据 parser**(`class_I/ir_parse.py`):
- 四张表:`DILocation` / `DISubprogram` / `DIFile` / `DILexicalBlock`
- **DILexicalBlock 穿透**(fix 2026-07-22):`scope_to_subprogram` 遇 lex-block 会
  链式解析到 DISubprogram —— 缺这一步 attribution 全部 fall through(旧 bug)
- 一次线扫,20+ MB IR 秒级完成

**Attribution 走链**(`class_I/attribute.py`):
- 输入:`Site(rule_id, dbg_id, callee, define_mangled)`
- 走 `inlinedAt` 链,每帧解析 scope → subprogram
- 输出:`Attribution(rule_id, define_fn, host_fn, ...)`
- 归属两个信号:**define fn**(innermost 非-stdlib,rewrite 落点)+
  **host fn**(outermost,perf self% 归属)
- Sentinel bucket:`UNRESOLVED`(无 !dbg 且无 define 上下文)/ `STDLIB_ONLY`
  (整条链都在 stdlib)

## §5 Build recipe

**同一次 build 服务 C1/C2/C3 全部检测**:

```
RUSTFLAGS="-C remark=all -C debuginfo=1 --emit=llvm-ir"
cargo build --release
```

输出:
- `target/release/deps/harness-<hash>.ll`(fat-LTO 全程序模块,C1/C2 IR 扫 + C3 TBAA 计数)
- stderr `note: <file>:<line>:<col> <pass> (<status>): <msg>`(C3 remark 通道①)

**必须用 fat-LTO harness `.ll`**(不是 `--lib` per-crate IR)—— RQ3 result.md
class_II/bzip2.md line 5 明说:`--lib` 版内联少,主体代码(如 bzip2
`BZ2_blockSort` 48.5% self)在 `--lib` 里还是独立函数,漏 remark。

**Wall clock**:~30-60s per project;缓存复用支持见 `class_I/build.py`。

## §6 Public API

```python
from perf_opt.hot_probe.class_I import scan, ScanResult

result: ScanResult = scan(
    crate=..., harness_dir=...,
    ir_path=None,               # Optional. If None, builds via build_and_emit_ir
    class_ii_result=None,       # Optional. If given, applies P_C3 predicate.
                                # Without it, only C1/C2 fire.
    hot_fns=None,               # Optional set[str] satisfying hot(·).
                                # deploy 时通常传 perf sampling 出的 hot fn 名
    cache_dir=None,             # Optional .ll cache dir
    out_dir=None,               # Optional; writes class_I_hits.json
)
```

**返回**(`ScanResult` in `class_I/scan.py`):

- `c1_c2_hits_by_fn: {fn: {C1: n, C2: n}}` —— 站点数汇总(self + inherited)
- `c1_c2_inheritance: {host_fn: {source_fn: {rule: n}}}` —— RQ3 结构化归属报告
- `c3_hits: set[str]` —— 通过 P_C3 的 fn(remark 是主筛选信号,`!tbaa=0` 是 c2rust 项目级 sanity check,详见 §3 主筛选信号说明)
- `c3_dropped_tbaa: set[str]` —— remark 命中但 !tbaa>0 的 fn(sanity check 失败,机制假设不成立,警报)
- `unresolved: {rule: count}` —— 无 !dbg 且无 define 上下文的 site
- `stdlib_only: {rule: count}` —— 整条链在 stdlib 的 site
- `tbaa_counts: {fn_mangled: count}` —— 非零 !tbaa 计数(应该恒为空)

## §7 Files in this package

```
class_I/
├── __init__.py     — public API re-export(20 符号)
├── build.py        — cargo build with --emit=llvm-ir + 缓存
├── ir_parse.py     — DILocation / DISubprogram / DIFile / DILexicalBlock 四表 parser
├── attribute.py    — Site + inlinedAt 走链 + std/core 穿透(rule-agnostic)
├── rules.py        — C1/C2 IR 签名扫描器 + `_C1_CALLEE_PATTERNS` 集中定义
├── c3.py           — C3 联合谓词(remark ∩ !tbaa=0)+ `⊇` OR 读法 rationale
├── scan.py         — 顶层 orchestrator
└── SPEC.md         — 本文档
```

**规则依赖 `class_II`**(单向):
- C3 通道① remark 分类逻辑在 `class_II/rules.py::classify`——因两侧共用一条
  remark stream。`class_I.scan(class_ii_result=...)` 从 `class_II.ScanResult`
  取 `c3_remark_hits`。

## §8 设计决策记录(audit-rationalized)

| ID | 决策 | 选值 | Rationale |
|---|---|---|---|
| **D1** | C1 收 generic `core::panicking::panic`? | **不收** | 通用 panic runtime 抓 cleanup/fmt/unwind/assert 等多种无 rewrite 情况;用**精确子符号**(`panic_bounds_check` / `panic_const_div_by_zero` etc.)代替 |
| **D2** | C1 union 用**原理准入**还是**枚举准入**? | **原理准入(i+ii+iii)** | cluster.md §凝练原则 1 明说"同层面合并 = 同缺口的多种模式合成一条规则";任一 callee 满足(i)Rust 语义强制 +(ii)条件分支+冷 panic 块 IR 形态 +(iii)`_unchecked` rewrite family 即入 union;是否曾在 6 项目热函数出现是覆盖事实、不是准入门槛(≥3 独立热函数已由 `panic_bounds_check`+`expect_failed` 满足,同原理成员继承规则准入)|
| **D3** | C3 `⊇` 读法? | **OR (机制读法)** | 见 §3;文档 + `apply_c3_predicate` docstring 双记录 |
| **D4** | 短 fn 名 → mangled 匹配? | **length-prefix**(`<len><name>`)| 裸 substring 假阳性率 30–60%(实测 lodepng `filter`, optipng `add`);length-prefix 精确到 Rust mangling 规范 |
| **D5** | Attribution 用 IR fat-LTO 还是 `--lib`? | **fat-LTO harness `.ll`** | `--lib` 少内联,漏主体代码 remark(RQ3 line 5 明写)|
| **D6** | 一次 build 共享 remark + IR? | **是** | C3 双通道 AND 需要**同一 build** 出的 remark 和 IR —— 不是优化,是正确性 |
| **D7** | 是否实现 IR 侧冗余 load 检测器(cluster.md §C3 期望的等价物)? | **不实现,复用 remark** | LLVM 的 GVN/LICM 已跑完 alias analysis,remark 就是它亲口的判定结果(权威+精度高);IR 侧从头做 = 重造 LLVM 几千行 C++,理论上限只可能等于或弱于 remark;`class_II` 已在跑 `-C remark=all`,通道天然共享无额外成本;详见 §3 主筛选信号说明 |
| **D8** | C2 单项目实证是否算 empirical 短板? | **不算** | Rust `as` 饱和是**语言规范**担保的通用性,不需要跨项目广度实证;cluster.md §C2 明说"通用性由 Rust `as` 饱和语义的语言规范担保,非跨项目实证"|

## §9 已知限制

- **workload 覆盖度**:`hot_fns` 由外部 perf sampling 供;harness 覆盖不到的
  fn 不进 `hot_fns`,即使 IR 里 signature 命中也不算 hit。这是**empirical 上限**,
  见 result.md §"回答与优化阶段的部署" line 577(i)。
- **无 `!dbg` 站点**:~30% site 无 !dbg(优化时 debug info 丢失),走 define
  fallback 归属;实测 c2rust 项目 no-dbg 站点大多是 miniz_oxide / addr2line 等
  build-dep 中的 panic runtime,不是 crate 代码。
- **DILexicalBlock 链最深 32 层**:超过 log warn 并停;未观察到超限。
- **`is_stdlib_file` 用 substring**:5 个 prefix (`library/`, `/rustc/`, ...)
  任一出现即判 stdlib;用户 crate 若命名撞车会误判(极罕见)。
