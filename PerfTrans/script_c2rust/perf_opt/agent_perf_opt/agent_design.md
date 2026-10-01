# Agent 设计文档

## 整体方法流程(Project-level pipeline, Phase 0-6)

> 本节:从"给一个 c2rust 出品的 Rust project" 到"输出优化后的 Rust
> project + 报告" 的端到端流程。Phase 4(rewrite)内部的详细决策见下方
> `关键设计决策(D1-D10)` + `增量讨论(D11-D16)` 两节。

### Phase 0 — Project 入口契约

**输入**:每个项目下的 `2_stage_a/`(Stage A 完成态的 crate)作为 agent
性能优化的**起点**;**不是** c2rust 直出的 `0_raw/`。


**目录布局约定**(`dataset_trans/<project>/` 现行标准,与 `main.py`
`--from perf_opt` 契约一致):

```
dataset_trans/<project>/
├── 2_stage_a/                          ← agent 只读输入
│   ├── crate/                          ← Stage A 完成态的 crate
│   └── harness/                        ← Stage A 的 boundary-adapted harness
│
├── workloads/                          ← pipeline 训练侧 workload(agent 用)
│   ├── pipeline.toml                   ← workload manifest
│   └── harness_gen/<proj>_harness/     ← harness_gen 产出
│       ├── src/  Cargo.toml
│       ├── corpus/                     ← 输入语料
│       └── golden.jsonl                ← W1 oracle
│
├── validation_workload/                ← ⚠️ held-out,agent 全流程绝不触碰
│                                          最终 Evaluation 章节用
│
└── 3_perf_opt/                         ← agent 输出(materialized 可变工作副本)
    ├── crate/                          ← 从 2_stage_a/crate 拷贝(git-init,fair-build)
    ├── harness/                        ← 从 2_stage_a/harness 拷贝 + path-dep rewire
    ├── baseline.json                   ← 初始 W2 baseline(pre-opt anchor)
    ├── hotspots.json                   ← hot fns + rule signals
    ├── class_I_hits.json               ← 每 fn 的 C1/C2/C3 命中
    ├── class_II_hits.json              ← 每 fn 的 II_vec/II_inl 命中
    ├── class_III_hits.json              ← 每 fn 的 III 命中
    └── evidence/<fn>.json              ← 每 hot fn 一份 EvidencePack
```

**关键**:
- **agent 只读** `2_stage_a/` 和 `workloads/harness_gen/`;**从不 in-place
  修改** 2_stage_a,所有 rewrite 都在 `3_perf_opt/crate/` 上,通过 git commit
  管理(D9)
- **CLI 入口**:`python -m main --project <NAME> --from perf_opt --run-agent`
  (via `perf_run.sh` 起频率锁)

### 横切模块 —— `perf_opt/verify/`(workload / W1 / W2 公共库)

Agent 不自己造 workload 抽象,统一用 `perf_opt/verify/`:

| 模块 | 提供什么 |
|---|---|
| `verify/workload.py` | `WorkloadAssets` dataclass + `.discover(project_dir)` → 扫 `workloads/harness_gen/*_harness/` |
| `verify/w1.py` | W1 correctness gate(golden.jsonl 比对)|
| `verify/w2.py` | W2 wall-clock gate(CV-aware tolerance)|
| `verify/functional.py` | `build_driver`、`golden_specs`、`replay_full` |
| `verify/measure.py` | `measure_harness`、`autotune_iters` |
| `verify/cargo.py` | `Verifier`(cargo build/run wrapper) |
| `verify/crate_ops.py` | `_ensure_fair_build`、`_git_init`、`_rewire_path_dep`、`_rsync_copy` |

Phase 4 里的 W1/W2 gate 都是 `verify/` 的薄 wrapper;`WorkloadAssets` 是所
有阶段共享的 workload 句柄。

### Phase 1 — Materialize + Baseline(driver.py Step 1-3)

1. **Materialize** — `_rsync_copy` 把 `2_stage_a/{crate,harness}` 拷贝到
   `3_perf_opt/{crate,harness}`;`_git_init` 初始化 git repo;
   `_ensure_fair_build` 强制 fair-build 参数(LLVM17 pin, no-LTO, debug=2);
   `_rewire_path_dep` 让 harness 指向新的 crate 副本。
2. **Build harness** — 一次 `cargo build --release` 产出可执行 harness。
3. **Initial W2 baseline** — 对每个 workload op 跑 3 次取 median +
   CV → 写入 `3_perf_opt/baseline.json`(pre-opt anchor;后续 W2 gate 的
   对照点)。

`baseline.json` **锁定** 后不再动;所有 rewrite 都跟这个比。W1 reference
= `WorkloadAssets.discover(project_dir).golden_path`(不需拷贝,原地读)。

### Phase 2 — Detection(driver.py Step 4-7 + hot_probe/)

流程:

1. **locate**(`hot_probe/locate.py::locate_hotspots`)—— perf record
   sampling → hot fn list + wrapper detection → `hotspots.json`
2. **一次 instrumented build**(`-Cremark=all --emit=llvm-ir`)—— 同一份
   产物给 Class I(IR scan)+ Class II(remark scan)用
3. **Class I/II/III scanner 各自扫**(scanner 内部 schema 不同,保留各自
   audit 桶给 characterize / SPEC 报表用):
   - `hot_probe/class_I/scan.py` → `class_I_hits.json`(C1/C2 counts + C3 set + audit)
   - `hot_probe/class_II/scan.py` → `class_II_hits.json`(II_vec/II_inl RemarkEvidence + residual_calls)
   - `hot_probe/class_III/scan.py` → `class_III_hits.json`(III①-④ CallSite/CursorSite lists + audit 桶)
4. **合并到统一 per-fn 视图**(`hot_probe/merged_hits.py`,agent 唯一入口)
   → `3_perf_opt/fn_hits.json`(见下方 §Phase 2.4 schema)
5. **characterize**(`hot_probe/characterize.py`)—— 给每个 hot fn 打包完整
   EvidencePack(profile + TMA + attribution_scope + tma_bottleneck + fn
   source + `fn_hits.json` 里对应 fn 的 hits)→ `3_perf_opt/evidence/<fn>.json`

**依赖关系**:Class I/II 需要 instrumented build 的产物;Class III **只需
tree-sitter parse 源码**,可以独立/先跑。

---

#### Phase 2.4 — 统一 hits schema(`fn_hits.json`)

**问题**:三个 scanner 的 ScanResult 内部 schema 差异极大(count / RemarkEvidence
/ Site 三种量纲混用,rule 命名和 audit 桶也各不相同),agent 主循环直接消费
要写 3 套 pivot 代码,还容易漏项。

**方案**:在 Phase 2 最后加一层 `merged_hits.py` 把三份 ScanResult 归并成
per-fn 视图。三个 scanner 内部**都已经有 site 级数据**(class_I 通过
`Site.dbg_id → MetaTables` 定位,class_II 通过 remark `line/message`,class_III
天然是 `CallSite`/`CursorSite`)—— 归并层只需暴露这些数据,不改 scanner 内部。

**Schema — 每 fn 一条,`hits` 是同构 site 列表**;**只收录 ≥1 命中的 fn**
(零命中 fn 不出现在 fn_hits.json 里,减少 clutter):

```json
{
  "csv_parse": {
    "fn":         "csv_parse",
    "file":       "src/parse.rs",
    "line_range": [100, 205],
    "hits": [
      {
        "rule":    "C1",
        "pattern": "bracket-indexing",
        "file":    "src/parse.rs",
        "line":    142,
        "col":     12,
        "snippet": "arr[i]",
        "extra":   {}
      },
      {
        "rule":    "C1",
        "pattern": "unwrap-expect",
        "file":    "src/parse.rs",
        "line":    155,
        "col":     8,
        "snippet": "cb1.expect(\"non-null function pointer\")",
        "extra":   {}
      },
      {
        "rule":    "C3",
        "pattern": "gvn-load-clobbered",
        "file":    "src/parse.rs",
        "line":    120,
        "col":     5,
        "snippet": "(*a).chars",
        "extra":   {"remark": "load of type i32 not eliminated"}
      },
      {
        "rule":    "II_vec",
        "pattern": "CantComputeNumberOfIterations",
        "file":    "src/parse.rs",
        "line":    175,
        "col":     5,
        "snippet": "loop { match current_block { ... } }",
        "extra":   {"remark_message": "loop trip count not computable"}
      },
      {
        "rule":    "II_inl",
        "pattern": "TooCostly",
        "file":    "src/parse.rs",
        "line":    148,
        "col":     16,
        "snippet": "accumulate(acc, e)",
        "extra":   {"callee": "accumulate", "cost": 380, "threshold": 250}
      },
      {
        "rule":    "III①",
        "pattern": "param-callback",
        "file":    "src/parse.rs",
        "line":    142,
        "col":     12,
        "snippet": "cb1.expect(\"...\")(field_ptr, field_len, data)",
        "extra":   {"callee_name": "cb1", "form": "B", "binding_source": "i"}
      }
    ]
  }
}
```

**强制字段**(每 hit 必有,agent 消费):

- `rule`:统一大写(`C1` / `C2` / `C3` / `II_vec` / `II_inl` / `III①` /
  `III②` / `III③` / `III④`)
- `pattern`:sub-pattern 名(kebab-case;每规则的 pattern 空间预定义,见下)
- `file` / `line` / `col`:1-indexed 源码定位
- `snippet`:单行 ≤ 200 字符的源码片段,LLM prompt 直接引用

**可选字段**:

- `extra`:规则自己的额外元数据(remark 原文 / callee 名 / cost / binding
  source 等),丰富 LLM prompt 上下文,不影响主流程

**每条规则的 pattern 空间**(预定义,固定):

| Rule | Pattern 集 |
|---|---|
| **C1** | `bracket-indexing` / `unwrap-expect` / `unwrap-plain` / `slice-range` / `division-runtime` |
| **C2** | `float-to-int-scalar` / `float-to-int-vector` / `float-to-uint-scalar` / `float-to-uint-vector` |
| **C3** | `gvn-load-clobbered` / `licm-invariant-invalidated` |
| **II_vec** | 直接用 remark reason 名(`CantComputeNumberOfIterations` / `NonReductionValueUsedOutsideLoop` / `LoopContainsSwitch` / `NoCFGForSelect` / `CantVectorizeLibcall`)|
| **II_inl** | `TooCostly` / `NoDefinition` / `NeverInline` |
| **III①** | `param-callback` / `field-callback` / `static-callback` / `match-pattern-callback` |
| **III②** | `malloc-free` / `calloc-free` / `realloc-free` / `aligned-alloc` |
| **III③** | `libc-mem` / `libc-str` / `libc-libm` / `custom-mem-wrapper` |
| **III④** | `cursor-index-deref` / `post-increment` / `post-increment-disambiguated` |

固定 pattern 空间让 audit / 统计不会因 typo 出错;新增 pattern 须走 SPEC 更新。

**为什么这个 schema 好用**:

1. 打开 JSON 一眼看清:**哪个 fn 的哪一行有哪种 pattern 命中**
2. LLM prompt 直接引用 site:"line 142 是 `bracket-indexing` pattern,用卡
   片 §2.1 的 template 改"
3. 同 pattern 多 site(比如 fn 内 6 处 bracket-indexing)天然扁平,不用嵌套
4. rule combination 统计一行 `groupby(hits, key=lambda h: h.rule)`
5. audit 数据分层:scanner 各自的 audit 桶(unresolved / iii1_deep_receiver
   / ambiguous_callees)照旧留在各自 ScanResult JSON 里,fn_hits.json 只装
   "agent 可 rewrite 的确定命中"

**落地代码估算**:

- 新建 `perf_opt/hot_probe/merged_hits.py` ~150 行(`Hit` + `FnHits`
  dataclass + `merge_scanner_outputs` + `write_fn_hits`)
- 给三个 scanner 补 `iter_sites(scan_result) → Iterator[Hit]` 薄 API
  (~30 行 × 3):class_I 从 `Site.dbg_id + MetaTables` 反查 file/line;
  class_II 从 remark 原文抽 file/line/reason(放宽 samples 上限至全量);
  class_III 直接从 `CallSite`/`CursorSite` 转
- `driver.py` 加一步:三 scanner 跑完调 `merge_scanner_outputs` +
  `write_fn_hits` ~5 行
- **总 impact:~245 行新代码,零删除**,scanner 内部 ScanResult 不动


### Phase 3 — 基于性能卡片，对fn进行性能优化-主循环(agent_perf_opt/,D16 全卡打包)

**每个 fn 一个原子事务**(详细决策见下方 D1-D18);Phase 3 已移除,直接
迭代 `fn_hits.json`。**核心链路**:PLAN → cross-fn dispatch → EXECUTE →
gate → commit。

```python
for fn, entry in sorted(fn_hits.items()):
    fired_rule_ids = sorted({h["rule"] for h in entry["hits"]})
    cards = [load_card(rid) for rid in fired_rule_ids]

    # ── D17 分层 direct: 简单 fn 跳过 PLAN 直接 EXECUTE ──
    complex = len(fired_rule_ids) >= 3 or len(entry["hits"]) >= 15
    if complex:
        # 1. PLAN(LLM 先写方案:改哪些规则,顺序,rule 交互,cross-fn 影响)
        plan = llm.plan(fn, entry["hits"], evidence_pack[fn], cards)

        # 2. D18.v1 cross-fn dispatch —— PLAN 声明的 cross-fn 规则 abstain
        applied = []
        for r in plan["applied_rules"]:
            if not r.get("needs_cross_fn"):
                applied.append(r); continue
            callers = caller_lookup.callers_of(fn)   # lazy tree-sitter query
            if any(c.kind in ("extern_c_facing", "cross_crate") for c in callers):
                audit.log("abstain_cross_fn_boundary", r); continue
            if any(c.kind == "rust_same_crate" for c in callers):
                audit.log("abstain_v1_defer_v2", r); continue   # V2 才做
            applied.append(r)     # 无 caller(私有 fn)→ 允许改签名
        if not applied:
            continue              # 全 abstain,该 fn 整个跳过

        # 3. EXECUTE(把 plan + applied rules 塞回,LLM 输出 rewrite)
        response = llm.execute(fn, applied, evidence_pack[fn], cards, plan)
    else:
        # 简单 fn: 单 call direct rewrite(D16 全卡打包)
        response = llm.chat(build_multi_card_prompt(fn, entry, cards))

    # 4. Apply(D5:tree-sitter splice Step 1-4)
    apply_rewrite(fn, response)

    # 5. Syntax gate(D3:cargo build,retry 2x)
    if not cargo_build():
        retry_with_error(response.stderr, max=2)

    # 6. W1 gate(D6:golden.jsonl 比对)
    if not w1_gate(workloads):
        # 降级到单卡 fallback(D16 § "Option B 代价与如何弥补")
        for fallback_rule in priority_order(fired_rule_ids):
            if single_card_pass(fn, fallback_rule): break
        else:
            rollback + abstain; continue

    # 7. W2 性能 gate(D7:CV-aware wall-clock,复用 verify/w2.py)
    #    对照 baseline.json(锁定的 pre-opt 锚点,不跟上次 commit 比,防漂移)
    if w2_regress(workloads, baseline_json_path):
        rollback + record_regression; continue

    # 8. 通过 → git commit per fn(D9)
    git_commit(f"perf_opt: {fn} apply {applied_rules}")
    caller_lookup.invalidate()    # crate 变了,caller cache 失效
```

**budget 约束(D10)**:每 fn ≤ 3 次 LLM attempt / ≤ 150K tokens(PLAN
+ EXECUTE 分层版);整 project 连续 3 次 regression → abort。

### Phase 5 — Report

```
project/report/
├── rewrites.jsonl        — 每 fn 一行:{fn, applied_rules, w1, w2_delta, verdict}
├── coverage.json         — {total_fn, agent_seen, w1_passed, w2_kept, abstained}
├── rule_combination.json — {combo: freq} 统计 rule 组合频次(替代 per-rule wall-clock 归因)
└── final_wall.json       — baseline vs post-agent wall-clock,per workload + geomean
```

### Phase 6(可选)— Workload-only fallback(D11 Path C)

`--optimize-workload-only` 开启时:hot fn 无规则命中 → 用
`TMA_hints.md` 的 `tma_bottleneck` 分诊(memory_bound / frontend_bound /
bad_speculation / core_bound / retiring),confidence tier ◔,靠 W2 兜底。

### 端到端数据流

```
dataset_trans/<project>/2_stage_a/{crate,harness}   ← Stage A 完成态,只读
     +  workloads/harness_gen/<proj>_harness/         ← W1 oracle 来源
     │
     ▼  Phase 0 入口契约(直接信任 Stage A 已过 gate,不做 preflight)
     ▼  Phase 1 materialize:2_stage_a → 3_perf_opt/(git-init 可变副本)
[3_perf_opt/crate + harness + baseline.json(初始 W2 anchor)]
   注:validation_workload/ 是 held-out,agent 全流程绝不触碰
     │
     ▼  Phase 1 baseline(perf + wall-clock + fn index)
[baseline/perf_profile / baseline_wall / fn_index]
     │
     ▼  Phase 2 detection(3 scanner 并行 + merged_hits 归一)
[class_{i,ii,iii}_hits.json(scanner 原始 + audit)
 + fn_hits.json(统一 per-fn 视图,agent 唯一入口)
 + evidence/<fn>.json(完整 EvidencePack)]
     │
     │  (Phase 3 已移除 —— fn_hits.json 直接就是 agent 输入)
     ▼  Phase 4 rewrite(per-fn 原子事务)
     │   ├─ prompt = multi-card + evidence
     │   ├─ LLM call
     │   ├─ tree-sitter splice
     │   ├─ syntax → W1 → W2 gate
     │   └─ git commit per fn(or rollback)
     │
     ▼  Phase 5 report
[rewrites.jsonl + coverage + rule_combination + final_wall]
```

### 实现状态一览(2026-08-03 现状)

| Phase | 模块 | 状态 | 位置 |
|---|---|---|---|
| 0 | 入口契约(不 preflight)| ✅ 契约层,driver.py 断言 2_stage_a 存在 | `main.py::_perf_opt` |
| 1 | Materialize + build harness | ✅ 已建 | `agent_perf_opt/driver.py::_materialize` + `verify/crate_ops.py` |
| 1 | Initial W2 baseline | ✅ 已建 | `agent_perf_opt/driver.py::_initial_baseline` + `verify/measure.py` |
| 1 | WorkloadAssets 抽象 | ✅ 已建 | `verify/workload.py::WorkloadAssets.discover` |
| 2 | locate_hotspots | ✅ 已建 | `hot_probe/locate.py` |
| 2 | Class I scanner | ✅ 已建 | `hot_probe/class_I/scan.py` |
| 2 | Class II scanner | ✅ 已建 | `hot_probe/class_II/scan.py` |
| 2 | Class III scanner | ✅ 已建 | `hot_probe/class_III/scan.py` |
| 2.4 | **merged_hits 归一层(fn_hits.json)** | ❌ **未建**(见 Phase 2.4 schema)| `hot_probe/merged_hits.py`(新)|
| 2.4 | class_I/II/III `iter_sites` API | ❌ **未建**(scanner 内部有 site 数据,需暴露)| 各 `scan.py` 加薄 API |
| 2 | EvidencePack 打包 | ✅ 已建(需接入 fn_hits)| `hot_probe/characterize.py` + `profiling/evidence.py` |
| 3 | Prioritization | ❌ **移除**(2026-08-03)—— Phase 4 直接消费 fn_hits.json | — |
| 4 | agent 主循环 | ⚠️ **已建 per-rule 版**(D1B);D16/D17/D18 重构**未落地** | `agent_perf_opt/agent.py::optimize_hot_fns` |
| 4 | prompt_builder | ⚠️ 已建 per-rule 版;D16 多卡 + D17 PLAN/EXECUTE 未落地 | `agent_perf_opt/prompt_builder.py` |
| 4 | rewrite_applier(tree-sitter splice)| ✅ 已建 | `agent_perf_opt/rewrite_applier.py` |
| 4 | gates(W1 + W2)| ✅ 已建 | `agent_perf_opt/gates.py` + `verify/{w1,w2}.py` |
| 4 | state(git commit/rollback)| ✅ 已建 | `agent_perf_opt/state.py` |
| 4 | **`caller_lookup.py`(D18 V1 用)** | ❌ **未建** | `agent_perf_opt/caller_lookup.py`(新)|
| 4 | Class III → agent 接线 | ⚠️ merged_hits 落地后自动解 | `agent_perf_opt/agent.py::_fired_rules` |
| 5 | rewrites.jsonl audit 输出 | ⚠️ `state.py` 有 AttemptRecord,coverage 报告未汇总 | `agent_perf_opt/state.py` |
| 6 | Path C TMA fallback dispatch | ⚠️ `TMA_hints.md` 有,dispatch 未建 | — |

**核心待办(2026-08-03,按优先级)**:

1. **Phase 2.4 merged_hits 归一层** —— 新建 `merged_hits.py` + 三个
   scanner 补 `iter_sites` API,产出 `fn_hits.json`(agent 唯一 detection
   入口)。**必须先做,后续都依赖它**
2. **`caller_lookup.py` — D18 V1 用** —— 新建 lazy tree-sitter caller
   查询模块 + kind 分类 + git-head cache invalidation
3. **Phase 4 主循环重构** —— D16 全卡打包 + D17 PLAN→EXECUTE 分层
   direct + D18 V1 cross-fn dispatch(依赖 1 + 2)
4. **rewrites.jsonl + coverage 汇总** —— Phase 5 report 层
5. **Path C TMA fallback dispatch** —— workload-only fn 走 `TMA_hints.md`

**V2 未来**(不在 V1 范围):
- **multi_file_splice.py** —— D18 V2 multi-fn atomic rewrite 用
- **签名 lift 规则收益解锁**(libcsv III①.a −32% 那类)—— V2 才可拿

其余模块**已实现,不需重建**——D1-D16 的骨架大部分早在了。

### Open design decisions(待拍板)

1. **Phase 1 profile 依赖哪个 workload?** — 若 project 有多个 workload,
   `self_time_ratio` 排序按哪个来?**建议**:每 workload 各排一次 + 取
   并集(union of hot fn),Phase 4 里跑 W1/W2 时对每个 workload 都过一遍。
2. **Phase 4 fn 内多轮重试 budget?** — D10 说每 fn ≤ 3 attempt / 100K
   tokens,但 D16 全卡打包一次调用可能就 ~30K tokens,3 次逼近上限。
   **建议**:per-fn tokens 放宽到 150K。
3. **Phase 5 rule_combination 归因口径?** — 论文 evaluation 章节写法:
   "cards 组合贡献 -X%" vs "常见 rule combo 是 {III④, C1, C3}" 频次
   统计?——需要跟 paper narrative 对齐。

---

## Phase 4 内部设计:关键设计决策(D1-D10)

### D1 —— **agent 粒度**:per-fn 顺序 or per-rule round-robin or all-at-once?

| 模式 | 描述 | 优点 | 缺点 |
|---|---|---|---|
| **A** per-fn 顺序 | 抓一个 hot fn,该 fn 应用所有 fired rule,W2,下一 fn | attribution 清晰,失败易 rollback | 慢(每 fn 至少一次 W2) |
| **B** per-fn/per-rule 分离 | fn1 应用 C1,W2,fn1 应用 C3,W2... | **每条 rule 单独测**,能算 per-rule 贡献 | W2 次数最多,最慢 | [NOTE: 感觉第一个版本，可以先尝试这个。]
| **C** batch | 所有 fn 所有 rule 一次改,一次 W2 | 快 | 失败无法 attribute,rollback 变整个 crate |

**我推荐 B**——per-fn/per-rule 分离。理由:
- **empirical value**:知道每条 rule 贡献多少 wall clock,才能证 "C1 值 −X%,II_inl 值 −Y%"
- **rollback 容易**:某条 rule 失败只回退它,不影响其他成功的 rewrite
- **cost 可控**:热 fn 通常 5-15 个,每个 fn 平均 2-3 条 rule → ~30 次 W2 measurement,单 project ~30-60 min。可承受

**✅ 已确认(2026-07-23)**:v1 采用 **B(per-fn/per-rule 分离)**。攻击面小、attribution 清晰,后续如果 W2 次数太多再考虑收敛到 A。

> ⚠️ **Superseded by D16(2026-08-03)**:落地 Class III 后发现规则间存在**结构性重叠**(III④ ⊃ C1+C3+D1),B 会导致双改冲突。改为 **per-fn 一次给全部匹配卡**(即 A 的变体但保留 attribution audit)。详见 D16。

---

### D2 —— **rule 内 fn 应用顺序** ⚠️ **Superseded by D16**

原方案:driver 按 `C1 → C2 → C3 → II_inl → II_vec` 顺序在同 fn 内串行跑
各 rule,C3 引入新 C1 site 允许多轮。

**D16 之后**:LLM 一次看全部匹配卡,自选改法顺序,driver 不再排序。
优先级表随规则数指数爆炸的问题从根上解决。详见 D16。

---

### D3 —— **重试策略**:LLM 一次没写对怎么办?

失败类型:
- **syntax error**:编译不过(缺 `unsafe`、类型错、missing import)
- **W1 fail**:功能变了(golden output 不匹配)
- **W2 regress**:变慢

| 场景 | 建议策略 |
|---|---|
| syntax error | 重试 up to 2 次,把编译错误消息塞进 prompt(LLM 能自我纠正) |
| W1 fail | 重试 1 次(给 LLM diff 提示),仍失败 → **降级到单卡 fallback pass**(D16),再失败 roll back |
| W2 regress | **不重试**——直接 roll back;记为"empirical W2 rejection" |
| abstain | 尊重 LLM 自证的 abstain,不 retry |

**✅ 已确认(2026-07-23)+ D16 修订(2026-08-03)**:retry 计数从 per-rule 改为 per-fn;W1 失败增加"降级到单卡 fallback"一层(优先级 III④ > III① > III③ > C3 > C1 > D1 > D2 > II_vec)。

---

### D4 —— **LLM 选型 + 客户端抽象** [NOTE: 这里就使用默认的配置，在 script_c2rust/Config 文件当中，有配置对应的模型，使用其中的配置。]

**✅ 已确认(2026-07-23)**——**用现有基建**,不再新写抽象层。

项目里已有的模块:

- **`Config/llm_config.py`**:集中式 LLM 配置管理器,根据 `model_name` 前缀自动路由 api_key / base_url / provider,统一走 llm-proxy.xyz / openrouter 代理。用法:
  ```python
  from Config.llm_config import get_llm_config
  cfg = get_llm_config("claude-sonnet-4-20250514")
  # → {"model_name": ..., "api_key": ..., "base_url": ..., "provider": "openai"}
  ```
- **`utils/llm_client.py`**:`LLMClient` 类,OpenAI-compatible 接口,带 **retry + streaming + JSONL prompt-cache**。用法:
  ```python
  from utils.llm_client import LLMClient
  client = LLMClient(
      model_name="claude-sonnet-4-20250514",
      cache_path=opt_dir / "llm_cache.jsonl",   # per-project cache,resume 复用
  )
  response = client.chat(system="...", user="...")
  ```

**Agent 只需 import 上面两个模块**,不写自己的 client。

**好处**:
- 不重复造轮子
- **prompt cache 天然支持** —— resume 时同一次 prompt 直接命中,不烧 token
- 换 model 只改配置,不改 agent 代码

**~~原推荐~~**:~~Anthropic Claude 优先,抽象 client~~ — **已废弃**,改为直接用现有基建。

---

### D5 —— **重写范围**:LLM 输出整 fn 还是 diff?

- **整 fn**:让 LLM 输出**完整重写后的 fn 体**,agent 用 tree-sitter 找到原 fn 边界替换 [NOTE: 这里说具体一点，到底怎么做？可以使用一个例子来说明？]
  - ✓ 简单,LLM 不会漏 import
  - ✗ 长 fn 可能超 token 预算
- **diff**:让 LLM 输出 unified diff
  - ✓ 短
  - ✗ LLM 容易在 diff format 上出错(行号偏移、context 不匹配)
- **site-by-site edit**:LLM 每处 rule site 单独 rewrite,agent 组装
  - ✓ 局部化,精确
  - ✗ 编排复杂

**我推荐"整 fn"**——用 tree-sitter 找 fn 定义,原地替换。cargo 一 build 就知道对不对。

**✅ 已确认(2026-07-23) + 具体流程**——分 4 步:

#### Step 1:LLM 输出格式(prompt 里明确要求)

```
Please output ONLY the rewritten function in a single ```rust fenced block.
Start with the fn signature (including `unsafe extern "C"`/attrs if any),
end with the closing `}`. No prose outside the fence.
```

LLM 输出示例:

````
```rust
#[inline]
unsafe extern "C" fn encodeLZ77(
    out: *mut ucvector,
    hash: *mut Hash,
    inp: *const c_uchar,
    inpos: size_t,
    insize: size_t,
    /* ... */
) -> u32 {
    // ... rewritten body with unsafe get_unchecked ...
    // SAFETY: i < insize by loop bound; ...
    // ...
}
```
````

#### Step 2:tree-sitter 定位原 fn 边界

```python
import tree_sitter_rust
from tree_sitter import Language, Parser

parser = Parser(Language(tree_sitter_rust.language()))
src = Path(hf.file).read_text()
tree = parser.parse(src.encode())

# 遍历 AST 找 function_item,name = 目标 fn
target_name = hf.name  # "encodeLZ77"
for node in walk(tree.root_node):
    if node.type == "function_item":
        name_node = node.child_by_field_name("name")
        if name_node.text.decode() == target_name:
            fn_start = node.start_byte     # e.g. 12456
            fn_end   = node.end_byte       # e.g. 15678
            break
```

#### Step 3:splice + 写回

```python
# 从 LLM 响应里抽取 ```rust ... ``` 里的代码
new_fn = extract_fenced_code(response, lang="rust")

# splice:[file_head] + [new fn] + [file_tail]
new_src = src[:fn_start] + new_fn + src[fn_end:]
Path(hf.file).write_text(new_src)
```

#### Step 4:cargo build 语法验证

```python
proc = subprocess.run(["cargo", "build", "--release"],
                       cwd=harness_dir, capture_output=True, text=True)
if proc.returncode != 0:
    # syntax error → retry(把 proc.stderr 塞回下轮 prompt)
    return RewriteResult(status="syntax_error", cargo_stderr=proc.stderr)
# 编译过 → 进 W1 gate
```

**为什么选整 fn 不选 diff**:
- LLM 生成 diff 时**行号 + context 匹配**极容易出错(经验值 30%+ 出错)
- 整 fn 只需要 fn 名对齐,tree-sitter 精确定位,不依赖 LLM 的行号
- `hf.file` / `hf.line_start` / `hf.line_end` 已在 EvidencePack 里,tree-sitter 用它剪枝

---

### D6 —— **W1 gate 严格度**:功能等价怎么判?

选项:
- **bit-exact**:workload 输出的 sha256 匹配 c2rust_raw baseline —— 严,但快
- **semantic-equivalent**:允许输出等价但不同 bit(比如浮点微差)——需要 domain-specific comparator
- **golden.jsonl 匹配**:已有的 harness_gen golden —— 现成基建

**我推荐 golden.jsonl 匹配**(现有 harness_gen 已经产 golden.jsonl 每 workload)。

**✅ 已确认(2026-07-23)**:用现有 golden.jsonl 匹配。

---

### D7 —— **W2 gate 阈值**:多少算"regress"?

- **单次测量**: cv (变异系数)~0.5-1%,单次 W2 不可信
- **建议**:每次 W2 跑 3 次取 median,与 baseline 比:
  - **-1% ~ +1%**:视为 noise,accept
  - **> +1%**:regress,roll back
  - **< -1%**:improvement,commit

RQ3 已有 `perf_opt/verify/w2.py` 的 CV-aware tolerance 逻辑(公式:`effective_tol = min(max(3×cv, 0.3%), 1.0%)`)。**建议直接复用**。

**✅ 已确认(2026-07-23)**:复用 `verify/w2.py`。

---

### D8 —— **多 fn 交互**:fn A 的 rewrite 影响 fn B?

举例:C3 改 fn A 的签名 `*mut T → &mut [T]`,fn B 是 A 的 caller 也要改。

选项:
- **naive**:只改被选中的 fn,其他 build error 自动 abort → 视为 W1 失败
- **impact tracking**:每次 rewrite 后 grep 所有调用者,如需更新则更新
- **conservative**:C3 sig 改写只在 fn 无外部调用者时应用

**我推荐 conservative** —— C3 卡片里的 REJECT if 就明写 "extern C or wide callers → abstain",符合精神。避免多 fn cascade。[NOTE: 同意； 但是，我记得有个规则是内联的。 这个请问你怎么处理函数之间的优化问题？]

**✅ 已确认(2026-07-23) + II_inl 跨 fn 处理策略**:

你抓到了 D8 的漏洞——推 conservative 是为了避开 **C3 sig 改写**,但 **II_inl 本身就是函数间优化**。5 条规则**编辑对象不同**,agent 需要按规则决定编辑哪个 fn:

| 规则 | Hot fn(target) | 实际编辑对象 | 编辑范围 |
|---|---|---|---|
| **C1** | encodeLZ77 | encodeLZ77 自身(fn body 内 `arr[i]` → `get_unchecked`) | 单 fn |
| **C2** | fastFloor | fastFloor 自身(`x as i32` → `to_int_unchecked`) | 单 fn |
| **C3 S2**(标量外提)| BZ2_decompress | BZ2_decompress 自身(hoist read into local) | 单 fn |
| **C3 S1**(改签名)| zsub_unsigned | zsub_unsigned + **所有 caller** | **多 fn 级联** |
| **II_inl.a**(加 `#[inline]`)| encodeLZ77 里调 accumulate | **accumulate**(被调 fn),不是 encodeLZ77 | **另一 fn,单点** |
| **II_inl.b**(callback 单态化)| csv_parse | csv_parse + **所有 caller** | **多 fn 级联** |
| **II_inl.c**(递归转迭代)| descend | descend 自身 | 单 fn |
| **II_vec** | encodeLZ77 | encodeLZ77 自身 | 单 fn |

#### v1 允许的编辑模式(3 种)

1. **In-fn edit**(hot fn 自己):C1 / C2 / C3-S2 / II_inl.c / II_vec —— 最简单
2. **Callee-side single-point edit**(在 hot fn 调的 callee 上加 `#[inline]`):**II_inl.a**
   - Agent 从 `EvidencePack.llvm_opt_remarks` 抽出**被拒的 callee 名**
   - tree-sitter 找 callee 定义位置,加 `#[inline]` attribute
   - 编辑量:一行(加 attribute)
3. **Abstain in v1**:C3-S1 sig 改写 + II_inl.b callback 单态化(涉及所有 caller)
   - v1 直接 abstain,记录到"v2 待做"清单
   - conservative 策略仍然生效——REJECT if 里明写 "wide callers → abstain"

#### II_inl.a 的具体实现步骤

```
1. 从 EvidencePack.llvm_opt_remarks 里找 pass=inline, status=missed 的行
   → 从 message parse 出被拒的 callee fn 名
2. FnIndex.resolve(callee_name) → (file, line_start, line_end)
3. 判断:
   - callee 在 crate 里 → 可以加 #[inline](进 v1)
   - callee 在 stdlib / cargo dep → 无法编辑,abstain
4. tree-sitter 找 callee fn 定义,加 #[inline]
   (若已有 #[inline(never)] 或 #[cold] → 尊重用户意图,abstain)
5. cargo build → W1 gate → W2 gate(全项目重跑,天然测跨 fn 效果)
6. 好使 → git commit;不好使 → git reset
```

#### 关键理解

**W1 / W2 gate 天然测跨 fn 优化的效果**——不管是 hot fn 内改还是 callee 加 attribute,重编译 + 重跑 workload,wall-clock 差距就体现了。所以 agent **不需要"跨 fn attribution"**,只要知道**该编辑哪个 fn**就行。

#### v2 扩展(v1 不做)

Multi-fn atomic rewrite —— 改 hot fn 签名的同时 grep 所有 caller,组成一个 atomic rewrite 单元,W1/W2 gate 一次判定。适用于 II_inl.b 和 C3-S1。v1 abstain。

---

### D9 —— **状态管理 + rollback**:git commit per fn

每次 accepted rewrite 一个 git commit,`git reset --hard` 回滚。既是
rollback 机制,也是 audit trail。

**✅ 已确认(2026-07-23)+ D16 修订(2026-08-03)**:commit **粒度由 per-rule 改为 per-fn**——D16 后一次 LLM call 打包 fn 内全部匹配卡,提交时以 fn 为原子单位。commit message 携带 `apply <rule_ids>` 便于审计。

---

### D10 —— **cost / budget 控制**

- **每 fn 最多 LLM tokens**:硬上限,超了 abort
- **每 rule 最多 attempt**:2-3 次上限
- **总 budget per project**:防止意外爆炸(比如 $50 上限)
- **早停**:如果连续 N 次 rewrite 都 W2 regress,认为该项目 rule 已榨干,停止

**推荐配置(可 override):**
```
max_attempts_per_rule = 3
max_llm_tokens_per_fn = 100_000
max_project_budget_usd = 5.0
consecutive_regressions_before_stop = 3
```

**✅ 已确认(2026-07-23)**:按上述默认配置,可 override。

---

## 模块架构(D16 落地后)

```
agent_perf_opt/
├── driver.py                    — 已有,加 --run-agent flag
├── Optimization_Card/           — LLM 消费的卡片(纯 rewrite recipe,无 driver 元讨论)
├── agent_design.md              — 本文档
│
├── agent.py                     — 顶层 optimize_hot_fns(evidence, hot_fns, cfg)
│   └─ 主循环:per-fn 全卡打包(D16);LLM 一次 call 消化 fn 内全部匹配卡
│   └─ W1 失败降级到单卡 fallback(优先级 III④ > III① > III③ > C3 > C1 > D1 > D2 > II_vec)
│   └─ 直接用 utils.llm_client.LLMClient 调 LLM
│
├── prompt_builder.py            — build_multi_card_prompt / build_plan_prompt /
│                                   build_execute_prompt(D16 + D17 三种 prompt)
│
├── caller_lookup.py             — ⬅ 新:D18 用的 lazy tree-sitter caller 查询
│   ├─ CallerLookup.callers_of(fn) → list[CallerSite(file/line/col/kind)]
│   ├─ kind ∈ {rust_same_crate, extern_c_facing, test_fn, cross_crate}
│   └─ cache TTL:git-head SHA 变了自动失效
│
├── rewrite_applier.py           — 解析 LLM 响应 + 应用
│   ├─ 提取 ```rust ...``` fenced code(D5 Step 1)
│   ├─ 抓 fn 头行 `// Applied rules: [...]` 写 audit log
│   ├─ tree-sitter 定位 fn 边界(hot fn 或 callee for II_inl.a)+ splice(D5 Step 2-3)
│   └─ cargo build syntax check(D5 Step 4)
│
├── gates.py                     — W1 + W2 gate
│   ├─ W1: golden.jsonl 匹配(D6)
│   └─ W2: 复用 perf_opt/verify/w2.py CV-aware tolerance(D7)
│                                   ★ 对照 baseline.json 锁定锚点,不跟上次 commit 比
│
├── state.py                     — per-fn 状态机 + rollback
│   ├─ git commit per fn(D9)
│   └─ audit trail(rewrites.log,记 rule combination 频次 + PLAN abstain 原因)
│
└── config.py                    — retry/budget policy(model_name 从 Config/llm_config 读)
                                   ★ complex_fn_threshold: {rules: 3, hits: 15}(D17)
```

**依赖**:
- 用现有 `utils/llm_client.py::LLMClient` + `Config/llm_config`(D4)
- OpenAI-compatible API,不再有 provider-specific client

## 决策状态一览

| # | 决策主题 | 状态 |
|---|---|---|
| D1  | 应用粒度 | ⚠️ superseded by D16(per-fn 全卡打包)|
| D2  | rule 优先级顺序 | ⚠️ superseded by D16(LLM 自决)|
| D3  | 重试策略 | ✅ 有效(D16 后 W1 fail 增降级 fallback)|
| D4  | LLM client | ✅ 有效 |
| D5  | 整 fn + tree-sitter splice | ✅ 有效 |
| D6  | W1 gate = golden.jsonl 匹配 | ✅ 有效 |
| D7  | W2 gate = CV-aware tolerance | ✅ 有效 |
| D8  | 多 fn cascade + II_inl.a callee-side | ⚠️ **superseded by D18**(cross-fn 政策改由 caller_lookup + PLAN 决定)|
| D9  | git commit rollback | ✅ 有效(D16 后 per-fn 粒度)|
| D10 | budget / retry / regression 上限 | ✅ 有效(D17 后 per-fn tokens 放宽到 150K)|
| D11 | workload-only fns(Path A / C)| ✅ 有效 |
| D12 | context bundle(单卡 prompt)| ⚠️ superseded by D16(多卡 prompt)|
| D13 | EvidencePack 补 3 字段 | ✅ 有效 |
| D14 | agent 分诊逻辑 | ✅ 有效(D16 后内循环改多卡,D18 后加 cross-fn dispatch)|
| D15 | prompt 示例 + token budget | ⚠️ 示例 superseded;结论(token 不爆炸)仍成立 |
| **D16** | **per-fn 全卡打包** | ✅ 当前主架构 |
| **D17** | **PLAN → EXECUTE 分层 direct**(复杂 fn 两步走)| ✅ **新增,V1 落地必做** |
| **D18** | **Cross-fn rewrite 政策**(V1 abstain / V2 multi-fn atomic)| ✅ **新增,V1 走 abstain 分支** |

---
# 增量讨论

## D11 —— workload-only fns 怎么处理?

**问题回顾**:80.5% 覆盖率 = **91/113 workload_hot fn 有规则命中**。剩下 **22 个 fn(20%)是"workload 热但零签名"**——这些多数是 RQ3 说的"忠实翻译"(paethPredictor、mz_adler32、hash_init 等纯算术小 fn)。

Agent 面对它们怎么办?

### Path C —— TMA 引导(受限探索)

- 用 `EvidencePack.tma` 抽出**主导 bottleneck**(比如 `memory_bound: 35%` 是 top)
- 给一张**TMA → micro-optimization 映射表**当 hint
- LLM 只在这个受限空间里尝试

**TMA hint 映射表(可作为"第 6 张卡片" = `Optimization_Card/tma_hints.md`):**

| 主导 TMA 维度 | 常见成因 | LLM 可尝试的方向 |
|---|---|---|
| **memory_bound** ≥ 30% | 数据不命中 cache | 局部性重构(loop tiling)、prefetch hint、struct-of-arrays |
| **frontend_bound** ≥ 20% | icache 压力 / 分支预测 | 缩小热 fn body、拆冷路径 |
| **bad_speculation** ≥ 15% | 分支预测失败 | 分支算术化、`likely!`/`unlikely!` hints |
| **core_bound** ≥ 25% | ALU 端口打满 | ILP 改造(手工展开)、SIMD 显式化 |
| **retiring** ≥ 60% | 已经很好 | **abstain**——已经 saturate 了 |

### 我的推荐

**v1 默认 Path A,提供 `--optimize-workload-only` opt-in 走 Path C。**

理由:
- Path A 是**诚实的默认**——与 RQ3 empirical 一致
- Path C 作为 **research knob** 可以 opt-in 试试有没有增益
- Path B 不做——不受限的自由发挥不是 agent,是随机数生成器

### 论文口径

Evaluation 章节里 clean 分:
- **主表**(rule-matched fns):5 规则的 wall clock 贡献 —— 主贡献
- **附表**(workload-only fns with TMA hints):可选 exploration 数据 —— 承认这是 open problem 的 partial exploration,不掩盖 20% 覆盖率上限

**✅ 已确认(2026-07-23) + TMA 卡片结构决策**:

同意 Path C。参照 `Optimization_Card` 已有格式,写 **单张 `TMA_hints.md`**(Option A)。

#### 为什么单张而非五张(每维一张)?

- **TMA hint 天然是"分诊 → rewrite"**:一张卡承载最自然,顶部一个 decision tree
- **5 维不像 5 规则那样机制正交**:memory_bound 和 core_bound 都可能收益于 SIMD 化,rewrite 家族多有重叠 —— 拆五张会重复
- **Agent 只 load 一张卡**,不用 lookup
- **retiring 高不做,只有 4 个 actionable instance** —— 每维单独一卡的话有一张纯 abstain 卡,浪费

#### TMA 卡与规则卡的关键差异(必须在卡片顶部明写)

| 维度 | 规则卡(C1-II_inl) | **TMA 卡(新)** |
|---|---|---|
| **触发信号** | 规则谓词命中(如 C1 site count > 0)| workload-only ∧ TMA 主导维度 |
| **信号可靠度** | ●(RQ3 empirical)/ ◐(Class II) | **◔ 探索档**(process-scoped 弱信号) |
| **Safety schema** | C1/C2/C3 需证明前置条件 | **无**(rewrite 全都语义中性) |
| **Rewrite 覆盖范围** | 缺口专用 | 通用 micro-opt |
| **W2 期待值** | 正收益(RQ3 已证) | **不确定**——W2 gate 硬 filter,失败率高 |
| **fn 编辑对象** | 可能是 callee(II_inl.a)| 只在 hot fn 内 |

**推论**:TMA 卡**不能骗 LLM 说"跟 C1 一样自信"**——metadata 明写 confidence tier **◔**,告诉 LLM 这是"试试看",不适用就 abstain。

#### TMA 卡片(已实现)

见 `Optimization_Card/TMA_hints.md` —— 单卡承载 4 个 actionable instance
(TMA.mem / TMA.fe / TMA.bs / TMA.core)+ decision tree + confidence
tier **◔ 探索档**。retiring 高不做。

---

## D12 —— context bundle 组装 ⚠️ **Superseded by D16**

原方案:单卡 prompt(每次 LLM call 只带 current pass 的一张卡 + 全景 fired_rules
背景),与 D1B per-rule 循环配套。

**D16 之后**:改为**多卡 prompt**——一 fn 一次 LLM call 打包所有匹配卡片;LLM
自选最小改法覆盖多症状。EvidencePack 3 个新字段(D13:`signature` /
`attribution_scope` / `tma_bottleneck`)保留。详见 D16。

---

## D13 —— EvidencePack 要加 3 个字段

按 D12 的 schema,`EvidencePack` 需要补 3 个字段:

### Field 1: `signature: str`

- **含义**:fn 的签名部分(fn 关键字到 body `{` 之前)
- **抽取**:tree-sitter 抽 `function_item` 节点的 signature 子树,或简单 substring 截断到第一个 `{`
- **好处**:LLM prompt 里可以先展示 signature 后展示 body;长 fn 缩略时保留签名信息不丢

### Field 2: `attribution_scope: dict`

- **含义**:每个 profile metric 是 fn-level(可信) 还是 process-level(honest 标注)还是 unavailable(sampling 拿不到)
- **值(硬编码常量,characterize.py 填入)**:
  ```python
  attribution_scope = {
      "self_time_ratio":       "function",       # per-fn(perf record deepest crate frame)
      "retired_instructions":  "function",       # per-fn (share × total)
      "cpi":                   "function",       # per-fn (derived)
      "tma":                   "process",        # process(toplev 只测进程级)
      "branch_miss_rate":      "process",        # process
      "call_count":            "unavailable",    # sampling can't recover
      "instructions_per_call": "unavailable",    # derived from call_count → unavail
  }
  ```
- **好处**:LLM 看到"tma 是 process 级"就知道不该拿这个说事,看到"self_time_ratio 是 function 级"就相信这是 per-fn 归属

### Field 3: `tma_bottleneck: str`

- **含义**:TMA 6 维里最大的那一维(为 workload-only Path C 用)
- **计算**:
  ```python
  tma_bottleneck = max(
      ["retiring", "frontend_bound", "bad_speculation",
       "backend_bound", "memory_bound", "core_bound"],
      key=lambda k: ep.tma.get(k, 0) or 0
  )
  # → e.g. "memory_bound" if that's the highest
  ```
- **用途**:
  - 有规则命中时:LLM 用规则改,`tma_bottleneck` 只作为背景信息
  - 没规则命中的 workload-only fn(Path C):agent 从 `tma_hints.md` 查这个 key 对应的 rewrite 家族

## D14 —— agent 分诊逻辑(D16 后修订)

```python
for hf in hotspots.hot_functions:
    fired_rule_ids = collect_all_matched_rules(hf)   # union I/II/III

    if fired_rule_ids:
        # ── 有规则命中 → per-fn 全卡打包(D16)──
        cards = [load_card(rid) for rid in fired_rule_ids]
        prompt = build_prompt_multi_card(hf, ep, fired_rule_ids, cards)
        response = llm.chat(prompt)
        apply(response) + w1 + w2 gate
        # W1 失败 → 降级到单卡 fallback pass(D3)

    elif config.optimize_workload_only:
        # ── 无规则命中 + opt-in Path C → TMA hint 驱动(D11)──
        tma_card = load_card("tma_hints", key=ep.tma_bottleneck)
        prompt = build_prompt_workload_only(hf, ep, tma_card)
        response = llm.chat(prompt)
        apply + w1 + w2 gate

    else:
        # ── 无规则命中 + Path A 默认 → skip ──
        log("workload_only, no rule fired, skipping")
```

## 需要更新的代码

1. **`hot_probe/types.py::EvidencePack`** —— 加 D13 3 个字段
2. **`hot_probe/characterize.py`** —— populate 这 3 个字段
3. **`Optimization_Card/TMA_hints.md`** —— 第 6 张卡(D11 Path C,已完成)
4. **`agent_perf_opt/agent.py`** —— 分诊 + 主循环(D14 + D16 全卡打包)
5. **`agent_perf_opt/prompt_builder.py`** —— D16 多卡 prompt 组装

---

## D15 —— context budget(D16 后仅保留结论)

原节含"C1 pass" 单卡 prompt 完整示例(已 superseded by D16 多卡格式,示例见 D16)。
以下**仅保留 token budget 分析** —— 结论在 D16 多卡场景下仍适用。

### D16 多卡 prompt token 分布

| 段落 | Token 数 | 说明 |
|---|---:|---|
| SYSTEM | ~400 | 固定,多卡版稍长 |
| Target fn source | 2500-15000 | 依赖 fn 长度 |
| Matched cards(N=1~5)| ~5K-25K | 变量 —— 每卡 ~5K,合并 <30K |
| Evidence bundle | ~600-1500 | 依赖 opt_remarks 条数 |
| Instruction | ~300 | 固定 |
| **合计 input** | **~15K-40K** | 平均 ~25K |
| Output(LLM 返回 fn + Applied rules 声明)| ~3K-8K | 依赖 fn 长度 |
| **总 turn** | **~20K-48K** | 占 Claude Sonnet 200K context <25% |

**极端情况**:BZ2_decompress 900 行(~15K) + 5 张卡合计 25K + evidence 1K
≈ **41K input + 8K output = 49K total**,仍占 200K <25%。

### 5 个防御网(v1 生效)

1. **`opt_remarks` 过滤到 fn 行范围 + top-10**
2. **`llvm_ir` 默认 off**,极端场景 opt-in
3. **`hot_span` v2 收紧** 到热段落(v1 = 整 fn)
4. **多卡打包 fired 那些,不带全库 8 张卡** —— D16 语义,LLM 只看命中卡
5. **长 fn fallback**:源码 > 10K tokens 时截取 hot_span ± 30 行,或整 fn abstain

### Sanity check(v1 跑前)

```python
for proj in DATASET_TRANS:
    for hf in load_hotspots(proj):
        prompt = build_prompt_multi_card(hf, ep, fired_rule_ids)
        if count_tokens(prompt) > 50_000:
            log_warning(f"{proj}/{hf.name}: {tokens} tokens (超预期)")
```

**结论**:多卡打包后 context 仍不爆炸(<25% Claude Sonnet 200K),v1 可
直接跑。

---

## D16 —— per-fn 一次给全部匹配卡(推翻 D1B / D2 / D12 单卡策略)

### 起因

2026-08-03 落地 Class III 时发现一个架构问题:一个 fn 常同时命中多条规则,
且**规则之间存在结构性重叠**(不是意外共现):

- III④(裸指针游标)⊃ C1(bounds check)+ C3(aliasing gap)+ D1(向量化)
- III①(callback dispatch)⊃ D2(inline missed)
- III③(memcpy → slice)⊃ D1(向量化)+ C3(aliasing gap)

**cluster.md 本身就是按源码病因分类的**——这些规则从不同角度描述**同一
个改法的必要性**。III④ 的 slice 化 rewrite 一次可以消 C1+C3+D1 三个
症状;若按 D1B(per-fn/per-rule 分离)串行做,会出现:

1. 先跑 III④ pass → slice 化 → C1/C3/D1 症状自然消失
2. Driver 下一轮看到 C1/C3/D1 仍在 fired_rules(命中信号是 pre-rewrite 采的)
3. 派 C1 卡 → LLM 在已 slice 化的代码上再叠 `get_unchecked`,反而破坏
   刚建好的 slice 抽象

**Cross-reference 补丁**(2026-08-03 我在 C1/C3/D1 卡片顶部加"若同时命
中 III④,让位于 III4 卡"文字块)只是**打补丁,不是根治**——它把 driver
调度决策塞进了 LLM 指令卡,层次混乱。

### 两种策略的本质对比(结构性 vs 意外共现)

| 维度 | Option A(D1B 原方案:per-fn/per-rule 分离) | **Option B**(本节新方案:per-fn 全卡打包)|
|---|---|---|
| Prompt 结构 | 单卡 | 多卡(N=命中规则数)|
| LLM 认知 | 每次只想一条规则 | 全景诊断,自选最小改动消最多症状 |
| **重叠规则处理** | ❌ 双改风险(III④ 后叠 C1 破坏 slice)| ✅ LLM 看到全景,一次改法消多症状 |
| 顺序敏感度 | ❌ 需要维护优先级表(且规则数增长时爆炸)| ✅ 无顺序,LLM 自决 |
| Attribution | 每 rule 单独 W2 → 可算"C1 值 -3%" | 只知"该 fn 改后 -X%",不知哪条 rule 贡献 |
| 失败 rollback 粒度 | per-rule commit,精细回滚 | 整 fn commit,粗回滚(rewrite 失败整段回) |
| Prompt tokens | ~15K/次 | ~25-35K/次(4-5 张卡合计) |
| W2 gate 次数 | 每 fn ~2-3 次(每 rule 一次)| 每 fn 1 次 |

### 决策:**采用 Option B**

**理由三条**:

1. **规则重叠是架构性的,不是意外**。Class III 是按病因组织的,与 I/II
   的症状分类正交但相交。既然设计上就承认这层关系,dispatch 也该顺着
   这层关系——让 LLM 同时看到多角度诊断,选一次最好的改法。

2. **Cross-reference / 优先级表是脆弱工程**。优先级表随规则数增长指数
   爆炸(N 条规则潜在 2^N 组合)。今天写 `III④ > {C1, C3, D1}`,明天
   加 III⑤ 又要重画。Option B 完全消除这层设计负担——LLM 是决策者。

3. **Prompt 长度不是问题**。D15 的 token budget 分析:单卡 pass ~15K;
   全卡打包 4-5 张 = ~30K,仍占 200K context 的 <15%。反倒是**多轮
   per-rule 每次重启 context** 更浪费——3 轮各 15K = 45K total token,
   比 1 轮 30K 还多。

### Option B 的代价与如何弥补

**代价 1:Attribution 粒度粗**

- 只知道"该 fn 改后 wall-clock -X%",不知道 III④ 贡献多少 vs C1 贡献
  多少
- **弥补**:LLM 输出 rewrite 时附一段 `## Applied rules:` 声明本次实际
  改了哪些卡的哪些 site;审计层可以做**统计聚合**("跨 12 项目,III④
  被 apply 次数 = X,C1 单独 apply 次数 = Y"),不做per-rule wall-clock
  归因。论文口径改为 "**cards 组合** 贡献",不是"**单条 rule** 贡献"。

**代价 2:失败 rollback 粒度粗**

- 一次 rewrite 若 W1/W2 失败,整段 fn 回滚,不能保留部分成功
- **弥补**:失败后 driver 触发**降级 fallback pass**——第二次调用只带
  一张卡(优先级 III④ > III① > III③ > C3 > C1 > D1 > D2 > II_vec),
  再试。若二级也 fail,该 fn abstain

**代价 3:LLM 需要在多张卡之间做优先级判断**

- 认知负担变大,可能出错
- **弥补**:prompt 里加**meta instruction**:
  > "These cards flag distinct symptoms of possibly related root causes.
  > Read all cards; identify if any single rewrite (typically the most
  > structural one, e.g. slice/iter lifting) subsumes multiple symptoms.
  > Prefer minimal rewrites that address maximum symptoms. Per-site
  > abstain is fine."

### 修正后的 agent 主循环

```python
for hf in hotspots.hot_functions:
    fired_rule_ids = collect_all_matched_rules(hf)   # union of I/II/III

    if not fired_rule_ids:
        # workload-only fn → D11 Path A 或 opt-in Path C
        continue

    # ── 一次性打包所有匹配卡片 ──
    cards = [load_card(rid) for rid in fired_rule_ids]

    prompt = build_prompt_multi_card(hf, ep, fired_rule_ids, cards)
    response = llm.chat(prompt)                       # ← 一次 call

    apply(response)                                   # tree-sitter splice
    if not w1_gate():                                 # 整段 W1
        rollback + fallback_single_card_pass(hf, ep) # 降级重试
        continue
    if w2_regress():                                  # 整段 W2
        rollback + record_regression
        continue

    git_commit(f"perf_opt: {hf.name} apply {fired_rule_ids}")
```

### 修正后的 prompt 结构

```
[SYSTEM]  (与 D15 类似,把 "ONE rule" 改为 "one or more rules")
  You are a Rust performance optimization agent. This turn you receive
  a target function plus one or more optimization cards. Each card
  describes a distinct symptom that may share a root cause with others.
  Read all cards, choose the minimal rewrite that addresses the maximum
  number of symptoms, and prove all safety conditions.

[USER]
  ## Target function
    <name, signature, hot_span, full source>

  ## Matched cards (in this fn)
    <Card 1: III4_raw_ptr_cursor.md — full text>
    ---
    <Card 2: C1_redundant_check.md — full text>
    ---
    <Card 3: C3_aliasing_gap.md — full text>
    ---
    <Card 4: II_vec_vectorization_loss.md — full text>

  ## Evidence bundle
    fired_rules: [III④, C1, C3, D1]
    profile / tma / attribution_scope / opt_remarks

  ## Instruction
    These cards may describe distinct symptoms of the SAME root cause.
    Read all cards first. If one structural rewrite (e.g. slice/iter
    lifting) subsumes multiple symptoms, prefer it over stacking
    multiple narrow rewrites. Output the rewritten fn in a single
    ```rust fenced block. Add // SAFETY: for each unsafe. In the
    output prepend a `// Applied rules: <list>` line naming which
    cards you actually applied (for attribution audit).
```

### 影响到的既有决策(需 mark superseded)

| 决策 | 原状态 | D16 之后 |
|---|---|---|
| **D1** | ✅ B: per-fn/per-rule 分离 | ⚠️ **superseded by D16** — 改为 per-fn 全卡打包 |
| **D2** | ✅ rule 优先级顺序(C1 → C2 → C3 → II_inl → II_vec)| ⚠️ **superseded** — LLM 自决顺序,不再需要 driver 层排序 |
| **D3** | ✅ 每 rule 独立 retry 计数 | 改为每 fn 独立 retry;syntax=2 / W1=1(带降级 fallback)/ W2=0 |
| **D9** | ✅ git commit per rewrite | 改为 git commit per fn(不是 per rule)|
| **D12** | ✅ 单 pass 单卡 prompt | ⚠️ **superseded by D16** — 多卡打包 prompt(结构见上)|

### 影响到的卡片文件

D16 之后,以下改动同步落地:

1. **删除 `C1_redundant_check.md` / `C3_aliasing_gap.md` /
   `II_vec_vectorization_loss.md` 顶部的 Cross-reference 段**——
   卡片不该讲 driver 层调度,LLM 拿全套卡时自决优先级
2. **删除 `class_III/SPEC.md §5.2` 的优先级表**——改为 "driver 打包
   全部匹配卡,LLM 自决"
3. **卡片内容不变**——只删元讨论,rewrite recipe / safety / abstain
   规则保持

### D16 待落地清单

- [ ] agent.py 主循环从 per-rule 改为 per-fn 全卡打包
- [ ] prompt_builder.py `build_prompt` 签名 `current_rule_id` → `fired_rule_ids`
- [ ] rewrite_applier.py 解析响应时提取 `// Applied rules:` 头行,写到 audit log
- [ ] 3 张卡片删 Cross-reference(本文档 D16 决策已含此项)
- [ ] class_III/SPEC.md §5.2 简化
- [ ] fallback pass 逻辑(W1 失败降级到单卡再试)

---

## D17 —— PLAN → EXECUTE 分层 direct

### 起因

D16 全卡打包后,一个 fn 命中 6 张卡(C1 + C3 + II_inl + III① + III④ + …)
时,合计 prompt ~25K tokens。LLM 要同时消化多张卡的 rewrite recipe + safety
obligation + abstain 条件——**注意力被稀释**,产出"每条都做一点但每条都没
做透" 的中庸 rewrite。且规则间可能存在**相互作用**(III①.a slice 化后,原
callback 里的 `expect("non-null...")` C1 site 自动消失,不需再叠 C1),
LLM 一步 rewrite 里不一定能想清楚。

### 方案对比

| 策略 | 描述 | Cost/fn | 复杂 fn 质量 |
|---|---|---|---|
| **A. Direct**(D16 现设计)| 一次 LLM call,给 N 张卡 → 直接输出 rewrite | 1 call | 中(简单 fn OK,复杂 fn 稀释)|
| **B. PLAN → EXECUTE** | 第 1 call 写 plan(JSON:改哪些规则/顺序/交互/cross-fn 影响),第 2 call 按 plan 执行 | 2 calls | 高(强制先想清楚)|
| **C. 分层 direct**(本决策)| 简单 fn 走 A;复杂 fn 走 B | 平均 ~1.3 calls | 按需分配 |

### 决策:**采用 C 分层 direct**

**触发阈值**(可 config):`len(fired_rules) >= 3 or len(hits) >= 15`
→ 走 PLAN → EXECUTE;否则 direct。

### PLAN 阶段 prompt 结构

```
[SYSTEM] You will optimize this fn. First WRITE A PLAN (do not modify code yet).

[USER]
  ## Target fn source
  ## Matched cards (N cards)
  ## Evidence pack

  ## Task
  Output a plan as JSON:
  {
    "applied_rules": [
      {
        "rule": "III①",
        "strategy": "cross_fn_signature_lift",
        "sites": [...],
        "needs_cross_fn": true,
        "rationale": "..."
      },
      {"rule": "C1", "strategy": "local", "sites": [...], "needs_cross_fn": false},
      ...
    ],
    "rewrite_order": ["III① first (subsumes 2 C1 sites)", "C3 on residual", "C1 leftovers"],
    "abstained_rules": [
      {"rule": "II_inl", "reason": "III① monomorphization subsumes — direct
                                   call recovers inlining automatically"}
    ],
    "interactions_noted": [...],
    "risk_flags": []
  }
```

### EXECUTE 阶段 prompt

```
[SYSTEM] Now apply the plan. Output the rewritten fn only.

[USER]
  ## Plan you wrote (verbatim)
  <PLAN JSON 原样贴回>

  ## Target fn source (再贴一次)
  ## Cards (只带 applied_rules 里剩下的)
  ## Instruction
  Follow the plan exactly. Output the rewritten fn in a single ```rust fence.
  Prepend `// Applied rules: [<list>]` header for audit.
```

### 与 D18 的耦合

PLAN 阶段是 D18 决策的**天然载体** —— LLM 在 PLAN 里为每条规则声明
`needs_cross_fn` 布尔,agent 收到 PLAN 后按 D18 dispatch 决定 V1 是否
abstain。**没有 PLAN 就没地方装 site-level 的 cross-fn 判断**。所以 D17 是
D18 的前置。

### Budget 调整

单 fn 最大 tokens 从 D10 的 100K 放宽到 **150K**(PLAN + EXECUTE 两 call
合计留余量)。整 project budget 不变。

### D17 待落地清单

- [ ] `prompt_builder.py` 新增 `build_plan_prompt` + `build_execute_prompt`
- [ ] `agent.py` 主循环加复杂度分层判断 + PLAN 结构 parse
- [ ] `state.py` audit log 里存 PLAN 输出(便于事后 debug LLM 思路)
- [ ] `config.py` 加 `complex_fn_threshold: {rules: 3, hits: 15}`
- [ ] D10 budget 从 100K → 150K per-fn tokens

---

## D18 —— Cross-fn rewrite 政策(推翻 D8 的 conservative)

### 起因

D8 说"C3-S1 sig 改写 + II_inl.b callback 单态化(涉及所有 caller)—— v1
直接 abstain"。这个决策**缺少 site-level 判断能力** —— 同一条 III① 规则,
命中的 fn 可能:

- **无 caller**(私有 fn / 死代码)→ 可自由改签名,不涉及 cross-fn
- **只有同 crate 的 Rust caller**(≥1)→ V1 做不到,V2 才能改
- **有 extern C caller**(C 代码调用)→ 永远不能改签名(V1/V2 都 abstain)
- **cross-crate caller** → 单 crate scope 不覆盖,abstain

D8 的"一刀切 abstain" 把第 1、2 情况都错杀,漏掉可拿的收益。

### 规则的 cross-fn 需求分档

| 档 | 规则 | 何时需要 cross-fn |
|---|---|---|
| **纯单 fn(never)** | C1, C2, C3.S2, III②, III③, III④.S2, II_inl.c, II_vec.a, II_vec.b.a | 从不 |
| **可选 cross-fn(两 template 并存)** | C3.S1 vs C3.S2;III④.S1 vs S2 | 卡片有 signature-lift + local-reborrow 两 template,LLM 按 caller 上下文选 |
| **inherent cross-fn(必须改 caller)** | III①.a (param callback), III①.b (struct field), II_inl.b (= III①.a) | 结构性必须 |
| **单点 cross-fn(改另一 fn,不改 caller)** | II_inl.a (给 callee 加 `#[inline]`) | 编辑 callee,单点,不级联 |

### V1 政策(本次落地)—— **PLAN 声明 + agent 按 caller 分类 dispatch**

Agent 侧新增 `caller_lookup.py`(lazy tree-sitter caller 查询,cache 用
git-head SHA invalidate);Phase 4 主循环:

```python
for rule in plan["applied_rules"]:
    if not rule["needs_cross_fn"]:
        applied.append(rule); continue

    callers = caller_lookup.callers_of(fn)   # lazy 查询

    if any(c.kind in ("extern_c_facing", "cross_crate") for c in callers):
        audit.log("abstain_cross_fn_boundary", rule)   # V1 + V2 都 abstain
    elif any(c.kind == "rust_same_crate" for c in callers):
        audit.log("abstain_v1_defer_v2", rule)         # V1 abstain, V2 才做
    else:
        applied.append(rule)                            # 无 caller,可自由改签名
```

### V2 未来扩展 —— multi-fn atomic rewrite

**触发条件**:PLAN 声明 cross-fn + caller kind 全是 `rust_same_crate`。

**机制**:
1. Agent 调 `caller_lookup.callers_of(fn)` 得完整 caller 列表
2. Prompt 里附上 fn 源码 + **所有 caller 源码上下文** ±10 行
3. LLM 输出 multi-file rewrite:
   ```
   === FILE: src/parse.rs ===
   ```rust <改后的 fn> ```
   === FILE: src/main.rs (splice line 145-152) ===
   ```rust <改后的 caller> ```
   ```
4. `multi_file_splice` 原子应用(snapshot + 全 splice + build)
5. W1 + W2 gate 用**整 crate 单位**(所有 workload op)
6. 失败 → git reset 全部涉及文件
7. 通过 → 一次 `git commit` 覆盖所有改动文件

**V2 新模块**(~500 行,估算 2-3 周):
- `caller_lookup.py` 扩展:context_snippet 提取 + kind 细分
- `multi_file_splice.py`(新)—— 原子多文件 splice + snapshot rollback
- `agent.py` 加 V2 分支:PLAN 声明 cross-fn + kind 允许 → 走 multi_file
- prompt schema 加 multi-file 段

### V1 → V2 收益上界

**V1 拿不到的收益**:所有 `signature-lift-only` 规则的贡献。libcsv `csv_parse`
III①.a −32% 就是这一块,V1 里 abstain 掉。

**V1 拿得到的**:C1 / C2 / C3.S2 / II_inl.a / II_inl.c / II_vec / III②
/ III③ / III④.S2 —— 全部 local rewrite 收益。多数项目主贡献仍在此。

### D18 待落地清单

- [ ] `caller_lookup.py` 新建 —— tree-sitter caller 枚举 + kind 分类
  + cache with git-head invalidation
- [ ] `agent.py` Phase 4 循环加 PLAN 解析 + cross-fn dispatch
- [ ] `state.py` audit log 分 cross_fn abstain 原因(v1_defer / boundary /
  cross_crate)
- [ ] 每张 signature-lift 卡片补 local-only fallback template(如果本卡默认
  是签名 lift):C3 已有 S2、III④ 已有 S2;III① 需要检查
- [ ] V2 部分列入 `agent_perf_opt/roadmap.md`(future,本次不做)
- [ ] audit 层新增 rule-combination 统计报表(替代 per-rule 归因)