# Agent 实现计划(基于 `agent_design.md` D1-D15)

**目标**:把 15 条已确认决策落地成 module 骨架 + 端到端跑通 `main.py --from perf_opt --run-agent`。

**已有基础**:
- `hot_probe/` 5 规则检测跑通,产 `hotspots.json` + `evidence/<fn>.json`
- 6 张 `Optimization_Card/*.md`(C1 / C2 / C3 / II_vec / II_inl + TMA_hints)
- 现有基建:`Config/llm_config.py`(model 路由)/ `utils/llm_client.LLMClient`(retry + cache)/ `perf_opt/verify/w2.py`(CV-aware tolerance)
- `agent_perf_opt/driver.py` 已 orchestrate 到 characterize 完成

**本计划不重复讲 design 已定的决策**,只写:模块 / 数据结构 / prompt 骨架 / 阶段推进 / gotchas。

---

## 1. 总体架构与数据流

```
[已有 pipeline]
main.py --from perf_opt
    │
    ▼
driver.run_perf_opt()   ──►  hotspots.json + evidence/<fn>.json
    │
    │ (新增 --run-agent flag)
    ▼
[本计划新增]
agent.optimize_hot_fns(hot_fns, ep_dir, harness_dir, crate, cfg)
    │
    ├─ _sanity_check()                  —— ~10 行内嵌 verify(见 §4.7 Step 0)
    ├─ 主循环:for hf in hot_fns:
    │     for rule_id in ordered_rules(hf):    ← D1+D2 决定
    │        1. build_prompt (P3)
    │        2. LLMClient.chat            (utils.llm_client)
    │        3. apply_rewrite (P2)              → splice + cargo build
    │        4. w1_gate  (P1.3)                 → golden.jsonl
    │        5. w2_gate  (P1.3)                 → verify/w2.py
    │        6. state.commit / rollback (P1.2)  → git
    │
    ▼
AgentResult + rewrites.log + git log
```

---

## 2. 模块清单与职责

| 模块 | LOC | 单一职责 |
|---|---:|---|
| `agent.py` | ~200 | **顶层主循环** —— per-fn per-rule 编排 D1-D14 决策 + 顶层 sanity check |
| `config.py` | ~50 | **AgentConfig** dataclass —— retry / budget / threshold |
| `state.py` | ~150 | **git commit / rollback + audit log** |
| `gates.py` | ~200 | **W1 + W2 gate** —— golden.jsonl 匹配 + 保留 pre_bin 快照后交 verify/w2.py |
| `rewrite_applier.py` | ~250 | **LLM 响应 → 应用到磁盘 + cargo build 语法验证** |
| `prompt_builder.py` | ~150 | **组 prompt** —— system + card + evidence + instruction |
| `driver.py`(扩)| +30 | 加 `--run-agent` flag,把 agent 挂到 driver 后 |
| **合计** | **~1030** | + tests ~200 LOC |

### 2.1 `agent.py` — 顶层主循环
- **完成的功能**:遍历 hot fns × ordered rules,组 prompt → LLM → apply → gates → commit/rollback。
- **不做的事**:不写 tree-sitter、不 build cargo、不判 golden、不组 prompt 内文 —— 全 delegate 给下方模块。
- **对外只暴露 1 个函数**:`optimize_hot_fns(hot_fns, *, evidence_dir, harness_dir, crate, assets, cfg, opt_dir=None) -> AgentResult`。
    - v4 refactor 后**不再有 `ops` 和 `baseline` 参数**:W1 走 `golden_specs(assets)` 全量读 golden.jsonl,W2 走 `pre_bin/post_bin` 快照(agent 启动时冻结 harness bin 到 `<opt_dir>/pre_harness_bin`)。
- **输入过滤**:只吃 `hotspots.json.hot_functions[]`(**已 drop wrappers**);跳过 `hf.file is None`(unresolved)+ `hf.extern_wrapper == True`(双 check,hot_probe 已过滤);按 `hf.self_pct` **降序**(最热先跑,与 D10 早停语义配合)。

### 2.2 `config.py` — 配置集中
- **完成的功能**:`AgentConfig` dataclass 集中 retry / budget / threshold / rule order。**model 名不放这里**(→ `Config/paths.get_path("DEFAULT_LLM_MODEL")`,与 `stage_a/runner.py:875` 同惯例)。
- **不做的事**:不读 LLM API 配置。
- **接口**:`load_agent_config(overrides: dict = None) -> AgentConfig` —— 直接返回 defaults + apply overrides。

### 2.3 `state.py` — git + audit log
- **完成的功能**:每次 rewrite 前记录 sha,成功 `git commit`,失败 `git reset --hard`,写 `rewrites.log`。
- **不做的事**:不判 W1/W2,不解析 LLM;只管状态转移。
- **git 交互约束**:`git add <改动文件绝对路径>`,**不用 `git add .`**(防止误提交 `target/`)。commit message 格式 `agent: <fn>/<rule>/<attempt> — <status>`。

### 2.4 `gates.py` — W1 + W2 双 gate
- **完成的功能**:W1 = 对每个 op 跑 harness,stdout SHA-256 与 golden.jsonl 比对(第一个 mismatch 即 fail);W2 = 保留 pre_bin 快照 + rebuild 后 build post_bin,交 `verify/w2.py::w2_gate(pre_bin, post_bin, ...)` 判决(w2_gate 自带 CV-aware tolerance + repeats 聚合)。v1 只测 `hf.hottest_op`。
- **不做的事**:不重跑 characterize;不改 pre_bin;不做 measurement 编排(那是 `verify/measure.py` 的事)。
- **pre_bin 快照**:agent 启动时 build 一份 harness binary 存到 `<opt_dir>/pre_harness_bin`,**全程不刷新**(见 §10 M-2)。每次 rewrite 后 cargo build 产 post_bin,交 gate。

### 2.5 `rewrite_applier.py` — LLM 响应落地
- **完成的功能**:parse ```rust fence → tree-sitter 定位 fn(含前置 attribute)→ splice → cargo build。
- **不做的事**:不 gate、不 commit;所有 apply 失败(parse / locate / syntax) → 磁盘回滚到原字节。
- **返回 `RewriteOutcome`**(3 值 status,见 §4.4)。

### 2.6 `prompt_builder.py` — prompt 组装
- **完成的功能**:按 D12/D15 组 `(system, user)` prompt 元组。
- **不做的事**:不调 LLM;不解析响应。
- **input**:`(hf, ep, edit_target, current_rule_id, cfg)` → `(system_prompt, user_prompt)`。**每次只带 ONE card**(D12)。

---

## 3. 模块依赖关系

### 3.1 依赖 DAG

```
                    ┌─────────────────┐
                    │ hot_probe/types │   (已有:HotFunction, EvidencePack)
                    └────────┬────────┘
                             │ import
             ┌───────────────┼──────────────┐
             ▼               ▼              ▼
       ┌──────────┐  ┌──────────┐   ┌──────────┐
       │ config   │  │ state    │   │ gates    │
       │ (P1.1)   │  │ (P1.2)   │   │ (P1.3)   │
       └────┬─────┘  └────┬─────┘   └────┬─────┘
            │             │              │
            └─────┬───────┴────┬─────────┘
                  │            │
                  ▼            ▼
          ┌─────────────────────────┐
          │  rewrite_applier (P2)   │
          │  prompt_builder  (P3)   │
          └────────────┬────────────┘
                       │
                       ▼
                ┌────────────┐        ┌────────────────┐
                │  agent.py  │◄──────►│ utils.llm_client│  (已有)
                │   (P4)     │        │  Config.paths    │
                └──────┬─────┘        │  Config.llm_config│ (已有)
                       │              └────────────────┘
                       ▼
                ┌────────────┐
                │  driver.py │  (P5:加 --run-agent)
                └────────────┘
```

### 3.2 依赖矩阵(谁 import 谁)

| 模块 → 依赖 | types | config | state | gates | applier | prompt | llm_client | paths |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| agent.py | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| config.py | | | | | | | | |
| state.py | | | | | | | | |
| gates.py | ✓ | ✓ | | | | | | |
| rewrite_applier.py | ✓ | | | | | | | |
| prompt_builder.py | ✓ | ✓ | | | ✓ | | | |

**关键**:
- 无环 —— DAG 保证
- `state.py` 零 agent 内部依赖 —— 可先独立完成(见 §6 P1.2)
- `rewrite_applier` 与 `prompt_builder` **互不 import** —— agent.py 编排它们协作,通过 `EditTarget` 对象传参
- **不再有独立 `preflight.py` 模块** —— `agent.py` 顶层内嵌 sanity check(~10 行,见 §4.7 Step 0)

---

## 4. 关键数据结构

### 4.1 EvidencePack 3 新字段(P0 加进 `hot_probe/types.py`)

对齐 D13:

```python
@dataclass
class EvidencePack:
    # ...(已有字段不列)...
    signature:         str  = ""     # D13-1: fn 到 body '{' 前
    attribution_scope: dict = field(default_factory=dict)   # D13-2
    tma_bottleneck:    str  = ""     # D13-3: top TMA 维度名
```

`attribution_scope` 硬编码常量(characterize.py 填):
```python
_ATTRIBUTION_SCOPE = {
    "self_time_ratio":       "function",
    "retired_instructions":  "function",
    "cpi":                   "function",
    "tma":                   "process",
    "branch_miss_rate":      "process",
    "call_count":            "unavailable",
    "instructions_per_call": "unavailable",
}
```

### 4.2 `AgentConfig`(config.py)

```python
@dataclass
class AgentConfig:
    # ─── LLM(model 名不放这里!!!!)─────────────────
    # model 由 agent.py 里 get_path("DEFAULT_LLM_MODEL") 拿
    # 详见 stage_a/runner.py:875 同惯例

    # ─── Rule ordering(D2 approved:2 轮)──────────
    rule_order:     tuple[str, ...] = ("C1", "C2", "C3", "II_inl", "II_vec")
    max_rounds:     int = 2                # D2:C3 之后再跑一轮 C1
    #                                      round-1 全走一遍;若 round-1 里 C3 有 commit,
    #                                      则 round-2 只重跑 C1 找新 site。

    # ─── Retry(D10 approved:单 counter)─────────
    max_attempts_per_rule: int = 3         # 首次 + 重试,合计 ≤3 次 LLM call/rule
    #                                      内部分派:syntax_err 重试消耗 1 次,W1_fail 消耗 1 次,
    #                                      W2_regress 直接结束不算 retry。

    # ─── Budget(D10)─────────────────────────────
    # NOTE: LLMClient.chat() 返回纯 str,没有 usage 对象 —— 精确 tokens/usd
    # 追踪不可得。tokens 用 tiktoken 估算(input len + output len);usd 追不了。
    max_llm_tokens_per_fn:        int         = 100_000    # tiktoken 估算 sum
    max_project_usd:              None        = None       # 追不了,固定 None
    consecutive_regressions_stop: int         = 3

    # ─── W2 gate(D7)─────────────────────────────
    w2_measurement_repeats: int = 3
    w2_scope:               str = "hottest_op"     # v1 = hottest_op 一个 op

    # ─── workload-only(D11)───────────────────────
    optimize_workload_only: bool = False           # opt-in flag

    # ─── TMA thresholds(from TMA_hints.md)──────
    tma_thresholds: dict = field(default_factory=lambda: {
        "memory_bound": 25.0, "frontend_bound": 20.0,
        "bad_speculation": 15.0, "core_bound": 25.0,
    })
    tma_retiring_saturation: float = 60.0
    tma_validity_min_self:   float = 0.20
```

### 4.3 `RewriteOutcome` + `RewriteStatus`(rewrite_applier.py)

**apply 层** 只 3 值(不是 8 —— 边角状态收拢到 attempt 层处理):

```python
class RewriteStatus(Enum):
    APPLIED       = "applied"        # splice 成功 + cargo build 过
    ABSTAINED     = "abstained"      # LLM 明说 abstain,或 fence 缺失 / fn 找不到 —— 一律不 retry
    SYNTAX_ERROR  = "syntax_error"   # cargo build 失败(可 retry)

@dataclass
class RewriteOutcome:
    status:           RewriteStatus
    committed_source: str | None       # new fn source(logging 用)
    cargo_stderr:     str | None       # SYNTAX_ERROR 时截尾 2000 chars
    abstain_reason:   str | None       # ABSTAINED 时给理由(fence-miss / locate-fail / llm-abstain)
```

### 4.4 `AttemptRecord` + `RewriteAttempt`(state.py)

**attempt 层** 收拢业务视角状态,由 agent.py 从 apply/gate 结果合成:

```python
class RewriteAttempt(Enum):
    APPLIED_COMMITTED = "applied_committed"
    ABSTAINED         = "abstained"
    SYNTAX_ERROR      = "syntax_error"
    W1_FAIL           = "w1_fail"
    W2_REGRESS        = "w2_regress"
    BUDGET_EXCEEDED   = "budget_exceeded"

@dataclass
class AttemptRecord:
    fn_name:      str
    rule_id:      str
    round_no:     int                # 1 或 2(D2 多轮)
    attempt_no:   int                # 1..max_attempts_per_rule
    status:       RewriteAttempt
    w1_result:    str | None         # "pass" / "mismatch:<op>"
    w2_delta_pct: float | None       # 负 = 更快
    commit_sha:   str | None
    tokens_in:    int
    tokens_out:   int
    error:        str | None
    reason:       str | None         # ABSTAINED 时的 reason
```

### 4.5 `AgentResult`(agent.py)

```python
@dataclass
class AgentResult:
    total_attempts:      int
    committed_attempts:  int
    abstained_count:     int
    regressed_count:     int
    syntax_failed_count: int
    w1_failed_count:     int
    total_wall_gain_pct: float                  # sum of accepted W2 gains
    per_rule_gain:       dict[str, float]       # {C1: -3.2, II_inl: -5.1, ...}
    per_fn_summary:      dict[str, list[AttemptRecord]]
    llm_tokens_used_estimated: int      # tiktoken sum,estimate,不精确
    # 删除 usd_spent —— LLMClient 无 usage 返回,无法追踪
```

### 4.6 `EditTarget`(rewrite_applier.py 内部)

```python
@dataclass(frozen=True)
class EditTarget:
    file:       Path            # 目标文件
    fn_name:    str             # 目标 fn 名(= hf.name 或 II_inl.a 的 callee 名)
    span:       tuple[int, int] # byte range,含前置 attributes(见 §8.1)
```

D8 的 3 种 v1 编辑模式:
- **In-fn**(C1/C2/C3-S2/II_inl.c/II_vec):`edit_target = hf` 本人
- **Callee-side single-point**(II_inl.a):`edit_target` = 从 `ep.llvm_opt_remarks` 抽的被拒 callee
- **Abstain in v1**(C3-S1/II_inl.b):`resolve_edit_target` 返 `None` → agent 跳过

### 4.7 规则触发与分诊逻辑(agent.py 的核心行为,D14)

**输入**:`hf: HotFunction`(已带 `class_i_hits` / `class_ii_hits`,由 hot_probe driver 灌入)+ `ep: EvidencePack`(P0 已扩 3 字段)+ `cfg: AgentConfig`。

#### Step 0:sanity check + 候选过滤(替代原 preflight.py)

**Step 0.a:agent 顶层 sanity check**(~10 行,替代整个 preflight.py 模块):

```python
def _sanity_check(assets, opt_dir: Path) -> Path:
    """启动时的最小 verify;缺项直接 raise,不做降级路径。"""
    import tree_sitter_rust                              # 缺 → ImportError
    from utils.llm_client import LLMClient               # 缺 → ImportError
    golden = _resolve_golden_path(assets.harness_src)    # 约定路径
    if not golden.exists():
        raise AgentError(f"golden.jsonl missing: {golden}")
    return golden
```

4 项外部依赖真接口**已 grep 确认**(记录到 §8.5):
- `verify/w2.py::w2_gate(pre_bin, post_bin, *, op, input_path, iters, repeats=None, pin_cpu=None)` —— 支持 `repeats`
- `LLMClient.chat(system, user, meta=None) -> str` —— **无 usage 返回**,tokens 只能 tiktoken 估算,usd 追不了
- `LLMClient._cache_key(system, user)` —— 按内容 hash,**user prompt 天然带 fn 源码,cache 自然新鲜**(不需要 extra_key)
- golden.jsonl 路径模板 `<proj>/workloads/harness_gen/<crate>_harness/golden.jsonl` —— 已验证 5+ 项目遵循

**Step 0.b:候选过滤**(GAP 4)

```
hot_fns = [ hf for hf in hotspots.hot_functions
            if hf.file is not None                    # 跳 unresolved 兜底
            and not hf.extern_wrapper ]               # 双 check,hot_probe 已过滤
hot_fns.sort(key=lambda h: h.self_pct, reverse=True)  # 最热先跑(GAP 2)
```

**只吃 `hot_functions[]`**,`dropped_wrappers` / `unresolved_symbols` 一律不进 agent。

#### Step 1:判断 hf 有没有规则命中(fired vs empty)

```
fired = { r for r, v in hf.class_i_hits.items()  if v } ∪
        { r for r, v in hf.class_ii_hits.items() if v }
```

`class_i_hits` 语义 `{"C1": int, "C2": int, "C3": bool}`,`class_ii_hits` 语义 `{"II_vec": {reason: count}, "II_inl": {reason: count}}` —— **truthy 过滤**(0 count / False / empty dict 都不算 fired)。

#### Step 2:三路分诊(D14 approved)

```
                   fired 非空 ?
                   /          \
                  yes          no
                   │            │
                   │      cfg.optimize_workload_only ?
                   │            /            \
                   │           yes            no
                   │            │              │
             【规则路径】   【Path C TMA】   【Path A skip】
                   │            │              │
                   ▼            ▼              ▼
       for r in ordered(     compute_tma_    log skip,
       fired, cfg):          diagnosis(ep)   下一 fn
         run_pass(hf,ep,r)   → route +
                             validity_ok
                                │
                               若 abstain → log,下一 fn
                               否则 → run_pass(hf, ep,
                                       "TMA_hints",
                                       tma_route=diag.route)
```

- **Path A default**:workload-hot 但零签名 → skip(诚实,与 RQ3 20% 覆盖率上限对齐)
- **Path C opt-in**(`--optimize-workload-only`):**方案 α agent 预分诊**(见下 §4.7.α)
- **规则路径**:按 `cfg.rule_order` 顺序遍历 fired 里的规则

#### Step 3:`ordered_rules(fired, cfg)` —— 规则内顺序 + D2 两轮

```
Round 1:                                                            ── 由 cfg.rule_order 定序 ──
  for r in cfg.rule_order:                                          C1 → C2 → C3 → II_inl → II_vec
    if r in fired:
      status = run_pass(hf, ep, r)   ← 每个 pass 独立 W1/W2 gate
      记 status 到 result

Round 2(仅当 Round 1 里 "C3" 有 commit):
  for r in ("C1",):                                                 只重刷 C1(D2:C3 后可能引新 C1 site)
    status = run_pass(hf, ep, r)

其他 workload_only fn 或 fired 空的 fn → 按 Step 2 分诊
```

**Path C 只走 1 轮**(TMA rewrite 不像 C3 会引新 site,无 round-2 语义)。

`run_pass` 内部:build_prompt → LLM → apply → **W1 → W2 → commit/rollback**。**Path C 与规则路径同样过 W1+W2 双 gate**(TMA 卡是 ◔ 探索档,失败率高,W2 兜底尤其重要)。

#### Step 4:全局提前止损(D10,GAP 3 精化)

主循环维护 `consecutive_regress`,**每次 pass 结束按下表转移**:

| Attempt 结果 | `consecutive_regress` 变化 |
|---|---|
| `W2_REGRESS` | **+1** —— 项目 rule 榨干信号 |
| `APPLIED_COMMITTED` | **重置 0** —— 有进展 |
| `ABSTAINED` / `SYNTAX_ERROR` / `W1_FAIL` | **不变** —— 模型问题,非项目信号 |

- `consecutive_regress >= cfg.consecutive_regressions_stop`(默认 3)→ **整个 agent 停**,认为项目 rule 已榨干
- 计数**跨 fn 跨规则累计**,不 per-fn 重置(D10 原意是项目级 stop)
- **注意**:abstain 是"空信号"(既不进展也不宣告项目死),**不能重置** —— 否则 abstain 频发会掩盖 W2 regress 的累积

#### 4.7.α Path C 里的 α 方案细节

**GAP 1 已选 α:agent 预分诊 → 单 instance 送 prompt**(见推理 §4.7 前的对比表)。

##### `compute_tma_diagnosis(ep, cfg) -> TMADiagnosis`

```python
@dataclass(frozen=True)
class TMADiagnosis:
    route:        str | None       # "TMA.mem" / "TMA.fe" / "TMA.bs" / "TMA.core" / None
    reason:       str              # 一行说明,写入日志用
    validity_ok:  bool             # ep.self_time_ratio >= cfg.tma_validity_min_self ?
    abstain:      str | None       # None = 可 rewrite;非 None = agent 直接 abstain,不调 LLM
```

**判决顺序**(照 `TMA_hints.md` §4 阈值):

1. **validity gate**:`ep.self_time_ratio < cfg.tma_validity_min_self`(默认 0.20)→ `abstain="tma_validity_low"`
2. **retiring 饱和**:`ep.tma["retiring"] >= cfg.tma_retiring_saturation`(默认 60)→ `abstain="retiring_saturated"`
3. **bottleneck 阈值判**(按 `ep.tma_bottleneck` 走):
   - `memory_bound >= cfg.tma_thresholds["memory_bound"]`(默认 25)→ `route="TMA.mem"`
   - `frontend_bound >= 20` → `route="TMA.fe"`
   - `bad_speculation >= 15` → `route="TMA.bs"`
   - `core_bound >= 25` → `route="TMA.core"`
   - 都不过阈 → `abstain="no_dominant_bottleneck"`

**agent 拿到 diag 后**:
- `diag.abstain != None` → 记 log,**不调 LLM**,进入下一 fn
- 否则 → `run_pass(hf, ep, "TMA_hints", tma_route=diag.route)`

##### `TMA_hints.md` 卡片的角色调整

- **§3 decision tree** 标注 "**由 agent 预分诊,LLM 不再走此段**"(卡片本体保留,人可读参考;prompt 不塞)
- **§4-§7 四个 Instance 段**(TMA.mem / .fe / .bs / .core)保留不变
- prompt_builder 拿到 `tma_route = "TMA.mem"` → 只抽 §4 那段塞进 `## Task rule` —— 见 §5.2 Path C 变体

##### Path C 的 attempt 归类

- `AttemptRecord.rule_id = f"TMA_hints.{diag.route}"`(如 `"TMA_hints.mem"`)—— 便于 `per_rule_gain` 里分 instance 统计
- `RewriteAttempt` 枚举复用规则路径,不新增

#### 关键不变量

- **每个 pass 只 apply 一条 rule**(D1 = per-fn/per-rule,attribution 干净;Path C 视 TMA_hints.instance 为一条)
- **每个 pass 独立 W2 measure**(可算 per-rule / per-instance 贡献,论文 evaluation 用)
- **fired 空 + Path A** 是**多数情况**,与 RQ3 20% workload_only 一致 —— 不是 bug 是设计
- **Round 2 只重跑 C1,不重跑 II_inl,也不适用于 Path C**(D2 只谈 C3 → C1 一个 known interaction)
- **hot_fns 迭代顺序:`self_pct` 降序**(热的先跑,与 D10 早停配合)

---

## 5. Prompt 结构

### 5.1 SYSTEM 段(固定 ~350 tokens)

```
You are a Rust performance optimization agent. Your task this turn is to
apply exactly ONE optimization rule to the target function.

Contract:
  * INPUT: target fn source, ONE rule card, evidence bundle
  * OUTPUT: rewritten fn wrapped in a single ```rust fenced block; include
    ALL original attributes (#[inline], #[cold], #[no_mangle], ...) unless
    the rule explicitly changes them
  * Every `unsafe { ... }` must carry an inline `// SAFETY: ...` comment
    stating the proven precondition
  * If you cannot prove the safety condition, output exactly:
      abstain: <one-line reason>
    and STOP
  * DO NOT apply any rule other than the one in `## Task rule`
  * Any deviation from the output format is treated as W1 failure
```

### 5.2 USER 段(4 部分)

```
## Target function
  file:       <edit_target.file 相对 crate>
  symbol:     <edit_target.fn_name>
  signature:  <_extract_current_signature(edit_target),从磁盘现读>
  location:   <line_start>..<line_end>(含前置 attrs)
  ```rust
  <整 fn 源码,从磁盘现读 —— rewrite 后仍然新鲜>
  ```

## Task rule (single, this pass)
  <load_card(current_rule_id) 全文 —— ONE 张卡片>

## Evidence bundle
  workload:           <hf.hottest_op>
  fired_rules:
    current_pass:     <current_rule_id>              ← 本次任务
    all_class_i:      <hf.class_i_hits>              ← 全景背景
    all_class_ii:     <hf.class_ii_hits>
  profile:
    self_time_ratio:  <hf.self_pct>
    cpi:              <ep.cpi>
    branch_miss_rate: <ep.branch_miss_rate>
  tma:                <ep.tma>
  attribution_scope:  <ep.attribution_scope>         ← 告诉 LLM 哪些是 fn 级
  tma_bottleneck:     <ep.tma_bottleneck>            ← Path C 用,规则 pass 只作背景
  compiler_signals:
    opt_remarks:      <top-10 filtered to fn>
    llvm_ir:          null(默认 off)

## Instruction
Apply ONLY the "Task rule" (<current_rule_id>). Output in a single
```rust fenced block. If you cannot prove the safety condition,
output `abstain: <reason>` and STOP.

<若 current_rule_id == "C3":加一段>
  v1 constraint (STRICT): Apply Strategy S2 (local hoisting) only.
  Do NOT change the fn signature (Strategy S1 deferred to v2).
```

### 5.2.α Path C 变体(rule_id = `TMA_hints`)

与 §5.2 结构一致,只有 **Task rule** 段和 **Instruction** 段不同:

```
## Task rule (single, this pass)
  Rule: TMA_hints (◔ 探索档 —— confidence tier)
  Route: <diag.route,如 "TMA.mem">  ← agent 预分诊结果,LLM 不再判决
  Reason: <diag.reason,如 "memory_bound=35.2 > 25 threshold">

  <TMA_hints.md 里对应 Instance 段(仅 §4 TMA.mem 或 §5 TMA.fe 或 …)>
  —— 3-4 个 rewrite family(loop tiling / prefetch / SoA / …)

## Instruction
Apply ONLY the Route <diag.route> rewrite family above. Do NOT try other
TMA dimensions or the 5 signature rules — that's for other passes.

Confidence tier is ◔ (exploratory). If none of the rewrite family fits,
output `abstain: <reason>` and STOP — do not force-fit.

Output in a single ```rust fenced block, same format as rule passes.
```

**Evidence 段与规则路径完全一致**(fired_rules / profile / tma / opt_remarks / …)—— **保留完整 `ep.tma` 6 维数字**,LLM 兜底判断能力仍在(即使觉得 route 错也能 abstain)。

### 5.3 Retry prompt 变体

失败时(syntax_error / W1_fail)追加:

```
## Previous attempt failed
Error: <cargo_stderr 尾 2000 chars 或 W1 mismatch 描述>

Previous output:
<上次 LLM 输出>

Please fix and retry.
```

### 5.4 Token 预算

| 段 | Tokens | 备注 |
|---|---:|---|
| SYSTEM | ~350 | 固定 |
| Target fn source | 2500-15000 | 变量,依 fn 长度 |
| Task rule card | 4000-8000 | C1 最大,C2 最小 |
| Evidence bundle | 600-1500 | 依 opt_remarks 条数 |
| Instruction | ~200 | 固定 |
| **合计 input** | **~12K-25K** | 平均 ~15K |
| Output | ~3K-8K | |
| **总 turn** | **~15K-33K** | << 200K Sonnet ctx |

**5 个防爆炸策略**(见 §8.6):remarks 过滤 top-10、llvm_ir 默认 off、hot_span v2 收紧、single card、长 fn hot region 截取。

---

## 6. 阶段推进

**每阶段独立可测**(mock 依赖),失败不阻塞下一阶段。

### P0 EvidencePack 扩展(~20 min,~50 LOC)
- `hot_probe/types.py` EvidencePack 加 3 字段
- `hot_probe/characterize.py` populate 三字段(`_extract_signature` + `_pick_tma_bottleneck` + 常量 `_ATTRIBUTION_SCOPE`)
- 测试:重跑 fzy `main.py --from perf_opt`,`evidence/choices_search.json` 有值

### P1 基础设施(~430 LOC,~2h)
- **P1.1 config.py**(~50 LOC):`AgentConfig` dataclass + `load_agent_config`
- **P1.2 state.py**(~150 LOC):`StateManager` git wrapper + `AttemptRecord` + `RewriteAttempt`
- **P1.3 gates.py**(~200 LOC):`w1_gate` + `w2_gate` 薄 wrapper(交 `verify/w2.py::w2_gate(pre_bin, post_bin, ...)`);顶层需在 agent 启动时 build 并保存 `pre_bin` 快照
- 三个子模块**互不依赖,可并行写**

### P2 Rewrite pipeline(~250 LOC,~3h)
- `rewrite_applier.py`:`parse_llm_response` → `resolve_edit_target` → `locate_fn_span_with_attrs` → `splice_fn` → `cargo_check`
- 返回 3 值 `RewriteStatus`
- 测试:mock LLM response + 真实 lodepng src 上 splice 一个 fn 能过 cargo build

### P3 Prompt 组装(~150 LOC,~1.5h)
- `prompt_builder.py`:`load_card` + `build_prompt` + `build_prompt_retry` + `compute_tma_diagnosis`(P7 用)
- 测试:mock hf/ep,验证生成的 prompt < 25K tokens

### P4 Agent 主循环(~200 LOC,~3h)
- `agent.py`:`optimize_hot_fns` 顶层 + `_try_rule` 单次尝试 + `_pick_ordered_rules`(D2 两轮语义)
- LLMClient 初始化:`model = get_path("DEFAULT_LLM_MODEL")`;transcript/cache 目录 `<crate>/.perf_opt/agent_llm/`(与 `stage_a/intra_ptr_llm/` 同惯例)
- 测试:mock LLM,fzy 端到端跑完不 crash

### P5 Driver 集成(~30 LOC,~15 min)
- `driver.py` 加 `run_agent=False` 参数 + 挂 `optimize_hot_fns`
- `main.py` 加 `--run-agent` 和 `--optimize-workload-only` flag

### P6 Testing & smoke(~200 LOC 测试,~3-5h)
- 单元测试 per module(mock 依赖)
- Mock LLM smoke:fzy 全流程,覆盖 abstain / syntax retry / good rewrite / W2 regress 4 条路径
- 真 LLM smoke:fzy 至少 1 个 committed rewrite

### P7 TMA Path C(可选,~100 LOC,~1-2h)
- `prompt_builder.compute_tma_diagnosis` 实装(TMA_hints.md §3 逻辑)
- `build_prompt` 支持 `rule_id = "TMA_hints"` 分支
- `agent.py` 主循环 `if not fired and cfg.optimize_workload_only:` 走 TMA 分支

---

## 7. 里程碑 & 时间估计

| Milestone | 内容 | LOC | 时间 |
|---|---|---:|---:|
| M1 | P0 EvidencePack 3 字段 | ~50 | 20 min |
| M2 | P1 基础设施 3 module | ~430 | 2h |
| M3 | P2 rewrite pipeline | ~250 | 3h |
| M4 | P3 prompt builder | ~150 | 1.5h |
| M5 | P4 agent 主循环 | ~200 | 3h |
| M6 | P5 driver 集成 | ~30 | 15 min |
| M7 | P6 测试 + smoke | ~200 | 3-5h |
| M8 | (可选)P7 TMA Path C | ~100 | 1-2h |
| **合计** | ~1410 LOC + tests | **14-17.5h** |

单人 2-3 天工作量,按 milestone 分次交付。

---

## 8. 实现 NOTE(gotchas 集中)

### 8.1 tree-sitter fn 定位 + 前置 attribute 反向扫

**问题**:tree-sitter 的 `function_item` 节点 **不含** `#[inline]` / `#[cold]` / doc-comments。若只按 `function_item` 边界 splice,LLM 输出的新 attributes 会与原有 attributes 冲突。

**做法**:找到 `function_item` 后,向前扫 `prev_sibling`,收集 `attribute_item` / `line_comment` / `block_comment`,把 span 头往前推。

**同名歧义**(c2rust 的 `main` + `main_0`):tree-sitter walk 找所有 name 匹配的 `function_item`,用 `hf.line_start ~ hf.line_end` 剪枝到最接近的那一个。

### 8.2 LLM ```rust fence 解析

**约定**:LLM 输出必须是 **单个** ```rust ... ``` 块。regex `r"```(?:rust)?\n(.+?)\n```"` 取第一个匹配。

**abstain 分支**:若输出以 `abstain:` 打头,不走 fence 逻辑,直接返回 `ABSTAINED`。

**fence 缺失**:视为 `ABSTAINED`(reason="parse_fail"),不 retry —— design 说 abstain 尊重(D3),format 违反算 W1 fail(SYSTEM prompt 已明说);v1 简化为 abstain 不 retry。

### 8.3 git 交互约束

- **只 add 改动文件**:`git add <edit_target.file 绝对路径>`,禁用 `git add .`(会误提交 `target/`)
- **commit message 格式**:`agent: <fn>/<rule>/att<n> — <status> (W2 <delta:+.2f>%)`
- **rollback**:`git reset --hard HEAD`(把工作树恢复到上一个成功 commit)

### 8.4 cargo build stderr 截取

- `cargo build --release` stderr 可能几十 KB,LLM prompt 塞不下
- 保留**尾 2000 字符**(错误通常在末尾),优先保留 `error[E...]` 行
- 头部加 truncation marker `... [earlier output omitted] ...`
- timeout=120s(v1 硬编码);超时算 `SYNTAX_ERROR` 走 retry

### 8.5 LLM cache key —— 天然安全,不需要 extra_key(2026-07-23 grep 后确认)

**先前担心**:同一 (fn, rule, evidence) 输入,但文件已被前一 rule commit 改过 → cache 会返回旧答案。

**grep `utils/llm_client.py::_cache_key` 后确认**:cache key = `sha256(model_name + system + user)`。**user prompt 里已经带 fn 源码全文**(从磁盘现读,见 §5.2 `_extract_current_source`),文件改动 → user prompt 变 → key 天然不同 → cache 天然新鲜。**不需要** `cache_key_extra`,原担心多余。

**次生 gotcha**:`LLMClient.chat` 返回**纯 str**,无 usage 对象 —— tokens 只能用 tiktoken 估算(input/output len);usd 追不了(见 §4.2 config 注释)。

### 8.6 长 fn > 10K tokens fallback

**问题**:BZ2_decompress ~900 行 ≈ 15K tokens,加上 C1 卡 8K,可能超 output 预算。

**做法**:v1 简单 fallback —— fn source > 10K tokens 时,截 `hot_region ± 30 行`,配合注释 `// [<N> lines of outer fn omitted for brevity]`。**若 hot_region 也未定** → abstain 该 fn(标 v1 limitation)。

### 8.7 II_inl.a 找 callee(D8 v1 模式 2)

**流程**:
1. 从 `ep.llvm_opt_remarks` 里 filter `pass=inline, status=missed` 的行
2. 从 message parse 出被拒的 callee fn 名(regex `r"'(\w+)' not inlined into"`)
3. FnIndex 查 callee 位置:crate 内 → 可编辑;stdlib/dep → abstain
4. 尊重 `#[inline(never)]` / `#[cold]` —— 已有则 abstain
5. 加 `#[inline]` attribute(一行)

### 8.8 C3 只做 S2 —— 靠 prompt 约束,不做 post-hoc gate

design D8 说 C3 有 S1(改签名)+ S2(标量外提),v1 只做 S2。

**做法**:prompt 里明写 "Apply Strategy S2 only; do NOT change signature"(见 §5.2)。**不做** post-hoc sig diff check —— 若 LLM 硬改签名,W1 会因 caller broken 而 fail,自然 rollback。**简单可靠**,不引入 SIG_CHANGED 中间态。

### 8.9 W2 只测 `hf.hottest_op`(v1 scope)

**理由**:一个 hot fn 通常在 1-2 个 op 上是热点,取 hottest 一个即可代表;测所有 op × 3 次 median 太慢。

**limitation**:其他 op 若被此次 rewrite 悄悄改慢,v1 不检出。见 §10 M-4。

### 8.10 D2 round-2 revisit(C3 之后再跑一轮 C1)

design D2 approved 2 轮:round-1 走 5 规则一遍,若其中 C3 有 commit,round-2 只重跑 C1(因为改 `*mut T` → `&mut [T]` 后 slice 索引可能给出新 C1 sites)。

**做法**(agent 里):
- round-1 结束后,如果 `any commit's rule_id == "C3"`
- 再刷一次 EvidencePack 里的 C1 opt_remarks(重跑 characterize 的 class_I.scan **仅在该 fn**)
- 若有新 C1 sites → round-2 只跑 C1
- round-2 结束不再有第 3 轮

**v1 简化**(可选):不重跑 scan,直接 round-2 无脑再问一次 LLM "still any C1 sites?",让 LLM 自判 —— 只需 W1/W2 gate 兜底。这个是 v1 vs v1.1 的选择,写代码时定。

---

## 9. 风险 & 缓解

| 风险 | 概率 | 影响 | 缓解 |
|---|---|---|---|
| LLM 输出格式漂移(不 wrap 在 fence)| 中 | 高 | SYSTEM prompt 严格;abstain 不 retry;计入 `abstained_count` |
| tree-sitter 找不到 fn(同名歧义)| 中 | 中 | 按 `hf.line_start~line_end` 剪枝,找不到直接 abstain |
| cargo build 挂(大项目)| 高 | 中 | timeout=120s;超时算 syntax_error 走 retry |
| W2 CV 特别大(共享机器 noise)| 中 | 高 | 3 次 median + CV-aware tolerance;CV > 3% 标 `w2_unreliable` 不 pass 不 rollback,人工 review |
| LLM 频繁 abstain | 中 | 低 | 预期行为,单元测试覆盖 |
| prompt 超 200K(极端长 fn)| 低 | 高 | §8.6 长 fn fallback;超限直接 abstain |
| git rollback 冲突(手动 edit)| 低 | 高 | 每 rewrite 前记 sha;冲突 halt + user notify |
| budget 爆炸(死循环)| 低 | 中 | `max_attempts_per_rule=3` + tokens + usd 三层 hard limit |

---

## 10. v1 已知 limitations

以下是 v1 明确不做的取舍,**每一项都留给 v2**,现在不实现、不 hack。

- **M-2 evidence 冻结**:一次 characterize → `evidence/*.json` 固定;C1 commit 后 fn 已改,但下一 rule 用的还是老 opt_remarks / hot_span。tree-sitter 从磁盘现读能定位到,但 evidence 语义 stale。v1 accept。
- **M-4 W2 scope**:v1 只测 `hf.hottest_op`(见 §8.9);其他 op 潜在退化不报。
- **M-5 baseline 全程不刷**:agent 跑 10 个 fn,后跑的 fn W2 delta 都基于同一个 pre-agent baseline。分量 gain 之和 ≠ 总 wall-clock gain(有二阶交互)。
- **M-6 cross-fn conflict**:fn A 的 C1 rewrite 可能影响 fn B 依赖的 inlined callee;v1 串行 rewrite,不做冲突检测。
- **M-7 无 crash resume**:agent 跑到 fn 5/10 crash → git 已 commit 4 个,rewrites.log 中断。v1 明确不做断点续跑;手动 rerun 会从头,已 commit 的靠 evidence stale 自然跳过。
- **C3 S1 sig 改写 abstain**:design D8 v1 只做 S2,S1 涉及所有 caller 级联,v2 才做。
- **II_inl.b callback 单态化 abstain**:同上,多 fn atomic rewrite v1 不做。
- **W1 只测 stdout**:不 hash stderr/timing;假设 harness 输出对结果敏感。若某 workload stdout 稳定但 stderr 有语义(极少见),v1 漏检。

---

## 11. 开工顺序建议

推荐 **A(按 milestone M0→M8)**:每步产可复用代码,不返工。

- 优点:每步都 testable,失败早发现
- 缺点:M5 之前看不到端到端

**替代 B(mock 全链路后回填)**:先 P0 + P1.2 骨架 + P4 状态机 mock 版 → 端到端 mock 跑通 → 再补 P2/P3/P1.1/P1.3。

**替代 C(fzy 单项目 vertical slice)**:只针对 fzy `choices_search` + II_inl 走通,快速看到 real rewrite —— 代码质量差,不推荐。

**默认 A**。

---

## 修订记录

### 2026-07-23 v4 —— grep 掉 preflight,砍工程冗余
- **删 preflight.py 独立模块**:grep 4 项依赖真实接口后,全部编译时/启动时可 assert,不需要独立 preflight 模块
  - `verify/w2.py::w2_gate(pre_bin, post_bin, ...)` —— 支持 `repeats`,但接口是 pre/post 两 binary(不是我原来假设的 baseline dict)
  - `LLMClient.chat(sys, user, meta) -> str` —— **无 usage 返回**,`usd_spent` 追不了,tokens 只能 tiktoken 估算
  - `LLMClient._cache_key(sys, user)` —— **cache_key 天然带 fn 源码**(user prompt 里就有),`cache_key_extra` 不需要
  - `golden.jsonl` 路径 —— 5+ 项目实测确认约定路径
- **§2 模块 7 → 6**;§3 依赖矩阵删 preflight 行/列;`agent.py` 顶层加 ~10 行 `_sanity_check`
- **§2.4 gates.py 描述改**:agent 启动时 build 并保存 `pre_bin` 快照;每次 rewrite 后 build post_bin,交 verify/w2.py
- **§4.2 `max_project_usd = None`**(硬定,LLMClient 没成本 API)
- **§4.5 `usd_spent` 删除,`llm_tokens_used` 改 `llm_tokens_used_estimated`**
- **§8.5 gotcha 反转**:cache_key_extra 不需要,原担心多余
- **§6 P-1 phase 删,§7 M0 删**;新增 §4.7 Step 0.a 内嵌 sanity check(替代)

### 2026-07-23 v3 —— 与 design.md 对齐重写
- **DIV-1 恢复 D2 两轮 revisit**(见 §8.10)
- **DIV-2 恢复 D10 `max_attempts_per_rule = 3` 单 counter**(见 §4.2)
- **抽象层级下沉到"实现计划"应有的深度**:模块清单 + 数据结构 + prompt 骨架 + 实现 NOTE;删除函数体伪代码 / helper stubs / formatter stubs / 多 dataclass 详列
- 从 1247 行 → ~600 行
- `RewriteStatus` 从 8 值降为 3 值(边角状态在 attempt 层处理)
- 保留 §10 v1 limitations,砍到 8 条(去掉工程细节 m-1..m-6)

### 2026-07-23 v2 —— audit driven fixes(部分回滚)
- C-1..C-5 / S-1..S-4 / M-1 / M-4 → v3 里已重构,不再单独条目
- v2 的 `EditTarget` / attr 前置扫思路保留(见 §4.6 / §8.1)

### 2026-07-23 v1 —— 初稿
- P0-P7 阶段划分 + 里程碑 —— v3 继承
