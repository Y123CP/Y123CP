# Class II detector — SPEC

**Class II = 同一优化器对 c2rust 产物少做的优化(M-b from RQ3).** 两条规则,
按 IR 层性能缺口向量聚类得到 —— 见 `empirical_study/result.md` §结果 3:

- **II_vec 向量化损失** —— gap = SIMD 通道未利用,标量逐元素
- **II_inl 未内联**    —— gap = call/ret 开销 + 阻断跨函数优化

**Class II 的性质(必读)**:置信皆 **◐**,本语料 **无一处 "C 侧优化开火成功
而 Rust 放弃"**(● 档为空;反向 Rust 还多 23 处)—— result.md line 356 明写:

> 故 Class II **不解释 C↔Rust gap**,但改写让 Rust 做 C 也没做的优化、
> **绝对变快乃至超越 C**

**这与 Class I 完全不同**:
- Class I(C1/C2/C3)= **解释 gap** (● 档),rewrite **要**证明 safety
- Class II(II_vec/II_inl)= **不解释 gap**(◐ 档),rewrite **不需**证明
  (语义中性),但因"改了未必真变快"必过 **W1 功能等价 + 实测墙钟** 双 gate

## §0 两通道模型

**同 `class_I/SPEC.md §0`**——两通道模型是全局设计一等公民。此处只列 Class II
对两通道的组合:

| 规则 | ①编译器信号 | ②优化后 IR | 组合语义 | 归属粒度 |
|---|---|---|---|---|
| **II_vec** | ✓ `loop-vectorize:missed` reason ∈ actionable | — | 纯①:pass 说什么 | 函数 f |
| **II_inl** | ✓ `inline:missed` reason ∈ actionable | ✓ ∃ 残余 call | **①∩②** | 函数 f |

Class II 有一条 ①∩② 规则(II_inl 用 IR 二次验证残余 call);另一条纯 ①。

## §1 II_vec 向量化损失

**RQ3 formula**(result.md line 484-489):

```
P_II_vec(f) ≜ hot(f) ∧ loop-vectorize:missed ∈ remark(f)
                    ∧ reason(f) ∈ { CantComputeNumberOfIterations,
                                    NonReductionValueUsedOutsideLoop,
                                    LoopContainsSwitch,
                                    NoCFGForSelect,          ; 控制流失真型
                                    CantVectorizeLibcall }   ; libcall 阻断型
                    ∧ reason(f) ∉ { NotBeneficial }
```

**通道**:纯①。**归属粒度**:函数 f。

### 检测(通道①)

**Actionable reason union**(5 项,按 cluster.md §D1 三子机制分组;同 union 成员共享 D1 规则准入,分组仅用于 rewrite direction 优先级):

**(a) 控制流失真类** —— 借 C1 rewrite(消 bounds-check + 迭代器化)恢复 countability

| Name | LLVM stderr message 正则 | 触发形态 |
|---|---|---|
| `CantComputeNumberOfIterations` | `could not determine (?:the )?number of loop iterations` | c2rust 每次数组索引前的 `panic_bounds_check` 分支 = 循环第二出口 |
| `NonReductionValueUsedOutsideLoop` | `value that could not be identified as reduction is used outside the loop` | 循环外 `let mut x` + 循环内 `x = ...` 状态字段写回 |
| `LoopContainsSwitch` | `loop contains a switch statement` | current_block 状态机 dispatch |
| `NoCFGForSelect` | `control flow cannot be substituted for a select` | 循环体分支不能算术化 |

**(b) 内联缺失致 unroll 塌 → SLP tree 退化** —— 借 D2 rewrite(热 kernel `#[inline(always)]`)恢复 unroll

| Name | LLVM stderr message 正则 | 触发形态 |
|---|---|---|
| `CantVectorizeLibcall` | `(?:library )?call instruction cannot be vectorized` | 循环内 `memcpy` / 数学库 call,或未内联的热 kernel call |

**(c) 别名阻断** —— cluster.md 明说"LLVM17 下大多消解(libzahl 7 处 `cannot identify array bounds` 是 LLVM16 artifact),仅残个别 heman"

- **未单独匹配**,如出现按 C3 rewrite(`*mut → &mut [T]` 或标量外提)兜底
- 若未来在其他项目发现漏检,再补 pattern

**Excluded reason set**(1 项,`II_VEC_EXCLUDE_RE`):
- `NotBeneficial` —— `not beneficial`:LLVM cost-model 判"不划算",改了大概率
  无用甚至变慢(RQ3 line 470 明说 SLP `NotBeneficial` "不成规则")

**明确不收**:
- **`CantVectorizeInstruction`**("instruction cannot be vectorized" 不带 "call"
  前缀)—— **不在 RQ3 actionable 集**,无清晰 rewrite direction(F2 audit,
  2026-07-23 移除)
- **`slp-vectorizer` pass 的一切 miss**(RQ3 line 371 排除记录:`NotBeneficial`
  20 / `NotPossible` 17 聚类是"大杂烩,无单一可执行反模式")

**Status 域**:actionable reason 出现在 `analysis` 行(paired with `missed` 头行)。
`classify` 允许 `s in ("missed", "analysis")` —— 因 message 内容承载 reason。
不强制 missed+analysis site-level 配对(F8 audit 决策:两者总是配对出现)。

### Rewrite + Safety

**Rewrite family**:语义中性重构。**Safety schema**:**无正确性证明义务**,
但过 **W1 + 实测墙钟** 双 gate。

```
current_block 状态机       ⟹ 原生控制流(labeled break / 提前 return)
循环外 let mut x; x = …    ⟹ 循环内 let x = …
循环内 memcpy(dst,src,n)   ⟹ dst[..n].copy_from_slice(&src[..n])
gate:  ⊢ W1 功能等价  ∧  实测墙钟不退化
```

## §2 II_inl 未内联

**RQ3 formula**(result.md line 513-517):

```
P_II_inl(f) ≜ hot(f) ∧ inline:missed ∈ remark(f)
                     ∧ reason(f) ∈ { TooCostly, NoDefinition, NeverInline }
                     ∧ ∃ 残余(间接)call ∈ optIR(f)
```

**通道**:**①∩②**(两通道 AND)。**归属粒度**:函数 f。

### 检测:通道①(remark reason)

**Actionable reason(1 项,cluster.md §D2 (a)+(b) 同措辞合并)** —— 见 `II_INL_ACTIONABLE_RE`:

| Name | LLVM stderr message 正则 | 覆盖子机制 |
|---|---|---|
| `TooCostly` | `too costly to inline` | (a) force-inline 属性丢弃 + (b) body 膨胀致 cost 越阈 —— 两子机制同 remark 措辞 |

**Excluded reason set**(3 项,`II_INL_EXCLUDE_RE`)—— 依 `_method.md §过滤规则` 明列的"内联失败但不是真损失":

| Name | LLVM stderr message 正则 | 排除理由 |
|---|---|---|
| `NoDefinition` | `definition is unavailable` | cross-crate FFI / dep 无 def;fat-LTO 二进制在 link 阶段会内联,**不算真损失**;caller 侧加 `#[inline]` 也对没定义在同 crate 的 callee 无效 |
| `NoInlineAttribute` | `noinline attribute\|has uninlinable\|uninlinable pattern` | 用户显式 `#[inline(never)]` 或 callee 结构性不能内联,**by-design**,无 rewrite direction |
| `NeverInline` | `never be inlined\|recursive` | LLVM 明说"永不内联"或递归无法内联,**by-design 或结构性不能** |

**Precedence**:排除集**优先**于包含集(defensive,和 C1 同处理)。

**旧命中数据(lodepng 280 templates)**:

| Name | templates 数 | 新分类 |
|---|---:|---|
| TooCostly | 1 | actionable(保留)|
| NoDefinition | **277** | **exclude**(cross-crate 假损失,占 98.9%) |
| NeverInline (含 noinline attr) | 2 | **exclude**(by-design) |

意味着旧 SPEC.md §8 表里 II_inl hits 数字被大量 cross-crate 假阳污染,新口径下命中数会显著下降(见 §8 附注)。

### 检测:通道②(IR 残余 call gate)

**必要条件**:remark 说"没内联成"之后,**IR 里必须实际存在残余 call**。这是
过滤 "remark 说 miss 但 call 已被 DCE" 的假阳性 —— 实证 lodepng 上 gate 过滤
掉 **~50%** 假 II_inl(104 candidates → 49 hits)。

**"残余 call" 定义**(`residual_calls.py::count_residual_calls`):
- 每个 `define` 内扫 `call`/`invoke` 指令
- **算残余**:
  - 直接 call(`call @<name>`)—— 除非 callee 在 skip 集
  - 间接 call(`call %<reg>`)—— `Option<extern "C" fn>` 回调编译出的形态
- **skip 集**:
  - LLVM intrinsic(`@llvm.*`)
  - 全部 **C1 panic runtime**(11 个 pattern,由 `class_I.rules._C1_CALLEE_PATTERNS`
    集中导入 —— F1 audit fix,防止 C1/II_inl 双计)

**归属 fn 名 → mangled**:用 Rust `<len><name>` length-prefix 匹配(F4 audit
fix),不用裸 substring —— 短名 fn `filter`(6 chars)、`add`(3 chars)
裸 substring 假阳性严重(实测 lodepng `add` 29 mangleds → 12 假阳性)。
`ResidualCallCounts.has_residual_for_source_fn` 封装了此逻辑。

### Rewrite + Safety

**Rewrite family**:hint / attribute / 单态化 / 缩小 body。**Safety schema**:**无正确性证明义务**,过 W1 + 实测墙钟。

```
热被调 fn g(..) {..}          ⟹ #[inline] fn g(..) {..}
Option<extern "C" fn> 回调    ⟹ 泛型 fn f<C: Fn(..)>(.., cb: C)   ; 单态化,消间接调用 + 解锁内联
过大热被调                     ⟹ 先按 C1–C3 缩小函数体使 cost 回落阈值内
gate:  ⊢ W1 功能等价  ∧  实测墙钟不退化
```

**RQ3 §example**(result.md line 526):`libcsv csv_parse` 单态化 callback,
下游 pipeline 实测 **−32% wall**、反超 C —— II_inl 里最有实测背书的形态。

### 归属

**函数级**:remark `file:line` → FnIndex,基名 fallback。
Fn → mangled(残余 call gate 用)——length-prefix。

## §3 Build recipe(与 Class I 共享)

**同一次 build 服务 II_vec / II_inl 全部检测**,并同时供 Class I 使用:

```
RUSTFLAGS="-C remark=all -C debuginfo=1 --emit=llvm-ir"
cargo build --release
```

`class_II/build.py::build_and_collect(harness_dir, also_emit_ir=True)` 是标准
入口 —— stderr 解析成 remark 列表,`.ll` 路径供 Class I + II_inl residual gate。

**Fallback build 也接住 emit-ir**(F2 audit fix 2026-07-23):若 caller 不给
`opt_remarks` 也不给 `ir_path`,`class_II/scan.scan()` 会自动 build 并用
`class_I/build._pick_harness_ll()` 找到 emit 出来的 `.ll` —— II_inl residual
gate 不再静默跳过。

## §4 Public API

```python
from perf_opt.hot_probe.class_II import scan, ScanResult

result: ScanResult = scan(
    crate=..., harness_dir=...,
    opt_remarks=None,       # Optional. If None, builds via build_and_collect
    ir_path=None,           # Optional harness .ll for II_inl residual gate
                            # If None but opt_remarks also None → auto-pickup
                            # after fallback build (F2 fix)
    hot_fns=None,           # Optional set[str] satisfying hot(·)
    out_dir=None,           # Optional; writes class_II_hits.json
)
```

**返回**(`ScanResult` in `class_II/scan.py`):

- `c3_remark_hits: {fn: RemarkEvidence}` —— **给 Class I 消费**(C3 的 ① 通道
  证据;单独看不构成命中,需 Class I 结合 `!tbaa` 才行)
- `ii_vec_hits: {fn: RemarkEvidence}` —— 通过 P_II_vec
- `ii_inl_candidates: {fn: RemarkEvidence}` —— 过 remark 前置条件的 fn(未过 IR 残余 gate)
- `ii_inl_hits: {fn: RemarkEvidence}` —— 通过 P_II_inl 完整谓词(remark ∩ IR)
- `residual_calls: ResidualCallCounts | None` —— IR 计出的残余 call 分布
- `n_remarks`, `n_classified`, `build_ok`, `error`

**`RemarkEvidence`**:`{rule_id: {reason: count}}` + ≤5 raw sample。

## §5 Files in this package

```
class_II/
├── __init__.py         — public API re-export
├── build.py            — cargo build with -C remark=all
├── rules.py            — classify(remark) → Classified(rule_id, reason)
│                         + pattern tables:II_VEC_ACTIONABLE_RE (5),
│                         II_VEC_EXCLUDE_RE (1), II_INL_ACTIONABLE_RE (3),
│                         C3_REMARK_RE (2 —— for class_I C3 通道①)
├── residual_calls.py   — count_residual_calls(ir_path) → ResidualCallCounts
│                         with length-prefix mangling match (F4)
│                         + SKIP set imports C1 patterns from class_I (F1)
├── scan.py             — 顶层 orchestrator, dedup, attribution, II_inl gate
└── SPEC.md             — 本文档
```

**依赖**:
- `class_II → class_I.rules._C1_CALLEE_PATTERNS`(单向,constants only)——
  避免 C1 与 II_inl residual gate 之间的漂移
- `class_II → hot_probe.symbol_source.FnIndex`(remark → fn 归属)
- `class_II → hot_probe.profiling.opt_remarks.OptRemarksReport`(build 返回)

## §6 设计决策记录(audit-rationalized)

| ID | 决策 | 选值 | Rationale |
|---|---|---|---|
| **D7** | II_vec 收 `CantVectorizeInstruction`? | **不收** | RQ3 formula line 484-489 明列 5 项,不含;这项无清晰 rewrite direction(F2 audit 2026-07-23 移除)|
| **D8** | II_vec 收 slp-vectorizer 任何 miss? | **不收** | RQ3 §排除记录 line 371:`NotBeneficial`+`NotPossible` "无单一可执行反模式"|
| **D9** | II_vec 强制 missed + analysis site-level 配对? | **不强制** | LLVM 总把两者同 site 一起打;实证冗余(F8 audit) |
| **D10** | II_inl 残余 call 定义窄(仅间接)还是宽(直调+间接)? | **宽** | RQ3 §example line 526:csv_parse(间接)+ BZ2_decompress(直调)都算 |
| **D11** | 残余 call skip 集 = intrinsic + C1 union? | **是,集中一处** | F1 audit fix:从 `class_I.rules._C1_CALLEE_PATTERNS` 导入,避免手抄漂移 |
| **D12** | 短 fn 名 → mangled 匹配? | **length-prefix**(`<len><name>`)| F4 audit fix;裸 substring 假阳性 30-60% |
| **D13** | Fallback build 后接住 emit-ir? | **是** | F2 audit fix;不然 II_inl residual gate 静默跳过 |
| **D14** | Class II 结果性质? | **◐ 档,不解释 gap** | RQ3 §结果 3 line 356 结论;LLM prompt 里明写 "让 Rust 反超 C,不是追平 C" |
| **D15** | D2 是否收 `NoDefinition` / `NeverInline` / `NoInlineAttribute`? | **不收(exclude)** | `_method.md §过滤规则` 明列:cross-crate `definition is unavailable` = LTO link 阶段会内联,非真损失;`noinline`/`recursive` = by-design 结构性不能内联;lodepng 280 templates 中 277 (98.9%) 是 `NoDefinition` 假阳,不排除会严重污染 hits 集 |
| **D16** | D1 union 用**原理准入**还是**枚举准入**? | **原理准入,分子机制 (a)/(b)/(c)** | 同 C1;cluster.md §D1 三子机制(控制流失真 / 内联缺失致 unroll 塌 / 别名阻断);`LoopContainsSwitch` + `NoCFGForSelect` 在 cluster.md 未直接列出但同 (a) 缺口原理,原理准入保留 |

## §7 与 Class I 的分工

**共用 remark stream**:一次 `-C remark=all` build 的 stderr 里,gvn/licm 归 C3,
loop-vectorize 归 II_vec,inline 归 II_inl。`class_II/rules.py::classify` 统一
分类,不同 rule_id 的 remark 走不同 bucket。

**C3 双通道 AND**:
- 通道① 由 `class_II.scan` 产出 `c3_remark_hits`
- 通道② 由 `class_I.c3.count_tbaa_per_fn` 产出 `!tbaa` 计数
- `class_I.scan(class_ii_result=..., ...)` 传入 Class II 结果 → 
  `class_I.c3.apply_c3_predicate` 合成最终 `c3_hits`

**typical driver 流程**:
```python
# 一次 build 服务两侧
from perf_opt.hot_probe.class_II import build_and_collect, scan as scan_ii
from perf_opt.hot_probe.class_I  import scan as scan_i, _pick_harness_ll

rr = build_and_collect(harness_dir, also_emit_ir=True)
ir = _pick_harness_ll(harness_dir / "target", "harness")

ii = scan_ii(crate=..., harness_dir=..., opt_remarks=rr.remarks, ir_path=ir,
             hot_fns=hot_fns)
ci = scan_i(crate=..., harness_dir=..., ir_path=ir, class_ii_result=ii,
            hot_fns=hot_fns)

# 5 规则命中全在 (ii.ii_vec_hits, ii.ii_inl_hits) + (ci.c1_c2_hits_by_fn, ci.c3_hits)
```

## §8 empirical 结果参考

**⚠️ 旧数据(2026-07-23 batch)含 cross-crate + by-design 假阳**:下表 II_inl 数字在 D15 生效前采集,`NoDefinition` / `NoInlineAttribute` / `NeverInline` 未被 exclude。**新口径下重跑 lodepng 2_stage_a 实测**(A/B 对照,同一份 raw remarks 分别用旧/新 rules 分类):

| 层面 | OLD(exclude 未生效)| NEW(exclude 生效)| ↓ 幅度 |
|---|---:|---:|---:|
| candidate fn 数(仅 remark 通道) | 92 | 60 | **-34.8%** |
| hit fn 数(post IR 残余 call gate) | 43 | 38 | -11.6% |
| 逐 remark 计数:TooCostly | 420 | 420 | — |
| 逐 remark 计数:NoDefinition | 309 → actionable | 309 → **excluded** | 全部剔除 |
| 逐 remark 计数:NeverInline | 27 → actionable | 27 → **excluded** | 全部剔除 |

**观察**:candidate 层剔 34.8%(下游节省 LLM 调用 + prompt 更聚焦 TooCostly);post-gate hits 层影响较小(11.6%),因为很多 hit fn 同时含 TooCostly + NoDefinition remark,IR 残余 gate 早已过滤了纯 cross-crate 无 def 的 fn。**下表旧 II_inl 数字整体高估约 10-30%**(具体项目视 cross-crate 比例而定),需按新口径重跑。旧 II_vec 数据不受 D15 影响。

10 个 dataset_trans 项目(2026-07-23 batch)覆盖率:

| 项目 | wkld_hot | II_vec | II_inl | ∩ wkld_hot |
|---|---:|---:|---:|---:|
| fzy | 1 | 0 | 16 | 1 |
| http-parser | 4 | 0 | 3 | 3 |
| libopenaptx | 12 | 0 | 4 | 7 |
| libqrencode | 15 | 0 | 27 | 12 |
| lil | 13 | 0 | 82 | 12 |
| lodepng | 15 | 0 | 44 | 12 |
| lz4 | 15 | 0 | 14 | 12 |
| miniz | 15 | 0 | 31 | 13 |
| optipng | 8 | 0 | 51 | 6 |
| zopfli | 15 | 0 | 24 | 13 |
| **合计** | **113** | 0 | **296** | **91 = 80.5%** |

**关键观察**:
- **II_vec 全部 0** —— Validation set 10 项目**没有触发任何 actionable vectorize
  loss**;说明现代 c2rust + rustc 1.77 nightly 上向量化损失稀有,与 RQ3 "II
  类不解释 gap" 结论一致
- **II_inl 是 Class II 主要贡献**,与 RQ3 §结果 3 line 526 "class_II 里最有实测
  背书的一条" 呼应
- 残余 call gate 过滤强度大(lodepng 104→49),F4 length-prefix fix 后过滤
  精度进一步提升(去除了大量短名假阳性)
