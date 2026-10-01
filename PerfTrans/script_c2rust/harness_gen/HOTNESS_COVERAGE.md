# harness-gen：从代码覆盖到热度覆盖（候选发现设计）

> 状态：设计稿
> 定位：给定一个 c2rust 翻译的 Rust crate，harness-gen 生成 workload。本文档讨论
> **如何让 harness-gen 生成的 workload 能用来"找优化候选"**，以及为此要在现有实现上补什么。
> 只谈"找候选（改什么）"这一面；不谈下游 LLM 改写。

---

## 0. 背景：workload 的两个角色，要的"覆盖"不是一种

同一个 harness-gen workload 被压了两份活，它们要的"覆盖"根本不同：

| 角色 | 用途 | 该用的覆盖 |
|---|---|---|
| **功能 oracle** | 每次 LLM 改完，重跑 workload 查功能有没有漂（改前 Rust vs 改后 Rust 输出一致） | **代码覆盖**：行/分支覆盖 + 输出校验。摸到即可。 |
| **候选发现** | 从 Rust 项目里定位"要优化的函数集" | **热度覆盖**：函数被压到 self-time 够高。摸到 ≠ 够。 |

**为什么候选发现只能靠动态（跑 workload），不能靠静态：** 已系统验证——汇编级和 IR 级的
C-vs-Rust 静态比较都分不开 regressed/preserved（见 `../opt_find/`，memory
`opt-find-static-signal-invalid`）。pre-LTO IR 层 c2rust ≈ C（median 比≈1.0），gap 生于
IR 之后（跨模块特化丢失 / codegen 膨胀 / CPI），全是 per-function 静态的盲区。
所以候选只能来自**跑 Rust workload 看 self-time**，纯 Rust、不碰 C。

**self-time（自身时间）定义：** 函数**自己代码里**花的时间，**不含**它调用的子函数（callee）。
perf 采样得到——落在该函数自身指令上的采样占比。超过阈值 τ → hot → 候选。用 self-time 而非
总时间：`main()` 总时间≈100% 但 self-time≈0，self-time 把真正在算的那段核挑出来。

**阈值要高过测量噪声底**：函数 self-time=f，优化快 x% → 总时间只动 f·x，只有 f 够大才测得出来。
所以"self-time 够高"同时保证**值得改** + **改了验证得出来**。

---

## 1. 现在的 harness-gen：架构与问题

**流程**（`agent.py:run`）：
```
inventory(扫 pub extern "C" fn) → plan(LLM 出 2–5 个 operation)
→ codegen(LLM 写 lib.rs: ops()/gen_seeds/gen_perf)
→ 门循环: build → smoke → coverage，失败反馈给 LLM 修
```
三道确定性门（`gates.py`）：A build、B smoke（确定性/输入敏感/iters/error-as-data）、
**C coverage（代码覆盖：行≥60% 函数≥70%，没覆盖的 pub fn 喂回 LLM 加 operation）**。

`perf_workload.py` 是**另一条、事后**的分析（不在生成回路里）：每 op 挑大输入、autotune iters、
测 `self_time_own`（项目代码 vs libc 的采样占比），`< 0.70` 标"不是干净的 perf 目标"。

**做对的**（保留）：gate 驱动、LLM 只写代码的架构；确定性/输入敏感/error-as-data 对功能
oracle 完全正确；gen-perf + self_time + size sweep 已意识到"小输入→libc 主导"的坑
（fzy 52%→93%）。

**问题（针对候选发现角色，按重要性）：**

- **P1（核心）生成回路只追代码覆盖，不追热度。** 驱动 LLM 的唯一反馈是"这些 pub fn 没被执行→加 op 碰它"。**碰一次 ≠ 压热**。harness 被优化成"每函数至少执行一次"，恰是 coverage≠hotness 的洞。
- **P2 self_time_own 粒度不对。** 它是**每 op 一个标量**（项目 vs libc），不回答"自己的哪些函数热、热了几个"。候选发现要的是 **per-function self-time 分布 + 热度覆盖率**。`top` 有 top 符号但只用于打印、不聚合、不反馈。
- **P3 perf 与生成解耦，没回路。** self_time 是 harness 定稿**之后**才测；低了、漏了热函数，**没有东西回炉**。coverage 会 loop，perf 不 loop。
- **P4 gen-perf 输入大小是 LLM 一次性猜的**（"hundreds of KB"）。没有测量驱动的自动放大（长输入直到某核主导）。size sweep 只是在固定候选里按 self_time_own 挑，不是为压热某冷函数去长输入。
- **P5 没有热度的"分母"。** inventory 只扫 pub extern fn（代码覆盖分母）。热度分母是**能干活的自己函数（含内部非导出的 c2rust static，如 AddValue 的被调用者）**，很多真核是内部函数，没进目标。
- **P6 operation 按"每 op 多覆盖"规划，不是"每 op 压热一个核"。** plan 只有一句软提示。多 op = 多核各自热的套件不是设计目标。

---

## 2. 目标：候选发现角色要"热度覆盖"

- **功能 oracle 角色**：现状（代码覆盖）就对，别动。
- **候选发现角色**：目标改成 **最大化热度覆盖**——让尽量多的**自己函数（导出+内部）**被某个
  operation 压到 self-time ≥ τ，并**列出压不热的洞**。

**候选集 = 热度覆盖到的自己函数**（= workload 能找 + 能验证的那批）。
一个哪套 workload 都压不热的函数 → 本来也验证不了对它的优化 → 合理地在范围外，且**列得出名字**（不是遗漏，是能力边界）。

需要现在没有的四件东西：per-function self-time、对**正确分母**的热度覆盖率、**测量驱动的输入放大**、一个**回路**（朝没压热的函数重生成/重放大，loop-until-hotness-dry）。

---

## 3. 第一步落地：热度覆盖探针（纯测量、不改生成回路、零风险）

对一个**已生成好**的 harness 跑，回答"当前 workload 把多少自己函数压热了、漏了谁"。四步，全读操作：

**① 定分母（自己函数有哪些）**
```
nm --defined-only target/release/harness | grep ' [tT] '
```
按 mangled 前缀 `_ZN<len><crate>…` + inventory 的 `#[no_mangle]` C-ABI 名，筛出 crate 自己的函数。
（被内联掉的小函数不在符号表——它们本来也不能单独热，排除是对的。）

**② 每 op 测 per-function self-time**
复用 `perf_workload._perf_selftime` 已在跑的 `perf record`，把
`perf report --sort overhead,symbol` 每行 `12.3% [.] <符号>` 解析出来，只留 DSO=自己二进制的符号
（libc/libm/ld/kernel 用现成 `NON_OWN_DSO` 剔），demangle 到函数名 → 每 op 一张"函数→self-time%"。

**③ 跨 op 聚合**：一个函数只要**某个 op** 让它热就算热，取每函数在所有 op 上的 **max self-time%**。

**④ 出报告**
```
hotness_coverage = |self-time ≥ τ 的自己函数| / |分母|     (τ 比如 1%)
```
输出：热函数表（+ 是哪个 op 压热的）、覆盖率、**洞清单**。再和 gate C 的 uncovered 交叉，把洞分两类。

**产出示意（lodepng，数字示意）：**
```
自己函数分母: 148
热函数 (max self-time across ops):
  unfilter            41.2%   (op=decode)
  encodeLZ77          22.8%   (op=encode)
  filter               9.1%   (op=encode)
  ...
热度覆盖: 9/148 ≥1%   (= 候选集就是这 9 个)

洞 (存在但从不热):
  [执行了但冷]  HuffmanTree_makeFromLengths, ...   ← 待裁决 (见 §5)
  [从没执行]    lodepng_encode_file, ...            ← 代码覆盖也没到 (见 §4)
```

---

## 4. 洞之一：从没执行（代码覆盖也没到）

判据：llvm-cov `count == 0`。**做法**：引导 LLM 加 operation + 覆盖率引导 fuzzing（AFL 那套）
生成能走到它的 input。这是标准的"让函数被调用到"，相对容易。

---

## 5. 为什么热度是最弱的一环：一条断开的、盲写的线

先讲清病灶（决定 §6 怎么改）。热度和"功能/覆盖"待遇完全不同：

**gen_perf 盲写**：`ops()/gen_seeds/gen_perf` 是同一次 codegen 一起写的。但生成回路（build→smoke→coverage）**全程只用种子**——`gen_perf` 在整个生成阶段**只被编译、从不被运行**。它造的大输入好不好（真让核主导，还是太小/畸形），**生成期无人检验**。

**perf 是独立线、不反馈**：生成器产出"定稿 harness"就结束。热度是**另外的工具**（`perf_workload.py` / `hotness_probe.py`）**事后**在定稿 harness 上单独测，而且**没有箭头回到 codegen**。

| | 生成期有【运行 + 反馈】闭环吗 |
|---|---|
| 功能 / 覆盖（gen_seeds）| ✅ smoke 跑它、coverage 跑它、没覆盖喂回重生成 |
| 热度（gen_perf）| ❌ 只编译不运行、事后独立测、不反馈 |

**fuzzing 的输入形状对 perf 是反的**：coverage-fuzzing 要**小 + 多样**（每个命中新分支，还刻意 minimize 缩小），perf 要**大 + 让核主导**。小输入 → malloc/setup/libc 主导（fzy：16B→52% libc）。所以直接拿 corpus 跑 perf，测到的多是 libc、不是核。

**结论**：热度弱，是因为它①生成期没被验证（gen_perf 盲写）②没闭环（perf 断在外面）③能拿到的多样输入（corpus）形状还不对。§6 就是把这三条缝一次缝上。

---

## 6. 新设计（定稿）：一条线 —— fuzzing 发现形状 → 扩大成 gen_perf → A/B 驱动定向精修

核心：**gen_perf 不再是"盲写的独立第一步"，而是"扩大 fuzzing 发现的形状"的产物，被 A/B 一轮轮定向精修。** fuzzing 前面跑一次（发现多样合法形状、够到 gen_perf 猜不到的长尾），之后是"扩大 → 运行 → A/B → 精修"的循环。

### 主干（就这一条线）

```
稳定 harness
  → ① fuzzing（跑一次）：发现多样合法形状；把 gen_perf 猜不到的长尾内部函数
        从"从没执行"拉到"执行了但冷"
  → ② 基于 fuzzing 结果【扩大】：格式感知地把这些形状变大 → 这就是初始 gen_perf
  → ③ 运行（release under perf）→ per-function self-time
  → ④ A/B 分类"执行了但冷"的函数：扩了会热=A / 扩不动=B
  → ⑤ 反馈：只对 A，继续沿它的维度【精修 gen_perf】（= 再扩大）
  → 回 ③ 运行 → ④ 再判 → loop-until-dry（没有新的 A 能再压热）
```

**关键：② 的"扩大" 和 ⑤ 的"精修 gen_perf" 是同一个操作**（把输入变大）。所以 **gen_perf = "扩大 fuzzing 形状"的产物**，② 出初始版、⑤ 出被 A/B 瞄准的定向版——一个机制、A/B 驱动，不再是一开始盲写的独立东西。

### 三个源各司其职

| 源 | 干嘛 | 覆盖 |
|---|---|---|
| **fuzzing corpus** | **发现多样形状**；把长尾函数从"没执行"拉到"执行了但冷" | gen_perf 猜不到的特征门控核（addChunk / color_tree / 16-bit / 隔行）|
| **扩大（= gen_perf）** | 把发现的每个形状变大到够热；被 A/B 定向精修 | 让形状里的核真正 self-time 冒出来 |
| **A/B** | 判哪个冷函数值得继续扩（A）、哪个天生冷（B）| 决定 ⑤ 只给谁精修 |

**为什么必须有 fuzzing**：gen_perf 是主路径盲猜，**发现不了**特征门控的长尾（lodepng 49 个 addChunk/color_tree 它根本不含）；**只有 fuzzer 变异才碰得到**，碰到后才谈得上扩大压热。没有 fuzzing = 只优化主路径、放弃整条长尾。

### 三个必须守的坑（否则"扩大"翻车）

1. **corpus 的"多样"含大量畸形**（fuzzer 专探错误路径）。扩一个畸形输入只让错误分支跑更多遍、核不热 → **先筛出合法、真走核的**再扩。
2. **必须格式感知扩，不能盲扩**（补零/拼接对结构化格式变不合法 → 库第一关就 reject → 测到错误处理）。靠 LLM（懂格式）或库自己的编码器扩。
3. **大 ≠ 越大越热**：有些核有尺寸上限（fzy `MATCH_MAX_LEN`），超上限再大 → libc 反而主导。有**甜点大小** → 保留 size sweep，不一味求最大。

### ④ A/B 怎么分

一个 `cov>0` 但 `self-time<τ` 的函数（例 `HuffmanTree_makeFromLengths`：建一次树、热循环在别处 → 执行到但 0.1%），判它"该继续压"还是"天生冷"：

| | 情况 A：workload-limited | 情况 B：intrinsically bounded |
|---|---|---|
| 含义 | 工作量本可随某维度涨，当前输入没顶上去 | 不随任何现实输入增长 |
| 例 | `unfilter`（随 w×h）| `HuffmanTree_makeFromLengths`（固定 288 符号）|
| 做法 | ⑤ 继续沿它的维度扩 → 变热 → **进候选** | 判 **范围外**，列名报告（非遗漏）|

**理想的 A/B = "扩大后 self-time 涨没涨"**（在循环里自然得到）：精修一轮，A 的涨、B 的不涨；几轮不涨 → 判 B。测量始终是裁判。

**当前无扩大能力（token 阻塞）时的临时代理**：拿现成 gen_perf 大输入**反向截小**做 size 扫描（占比随大小涨=A / 跌·平=B），一次测量预判 A/B —— 即已实现的 `hotness_probe.classify_ab`，是权宜；**有了扩大闭环就退役**（A/B 直接由"扩了涨没涨"给出）。size 扫描判平、疑似随结构维度 scale 的，交 LLM 读源码指出维度（调色板条目数 / 16-bit 路径 / chunk 类型 / 树深）。

### 落地优先级

1. **§3 探针**（per-function self-time + 内部分母 + 洞拆分）—— ✅ 已实现，测量基座。
2. **A/B size 扫描分类器**（截小代理）—— ✅ 已实现（`classify_ab`），token 阻塞下先用它预判 A/B。
3. **接 fuzzing + 扩大成 gen_perf**：先跑 fuzzer 攒 corpus；扩大器先用 MVP（最大 corpus 输入 + size sweep，`perf_workload` 已有），再进阶到 LLM/编码器的格式感知 scaler。
4. **闭环**：扩大 → 运行 → A/B（直接看"扩了涨没涨"）→ 只对 A 定向精修 gen_perf → loop-until-dry。gen_perf 从"盲写"变成"被 A/B 驱动的 scaler"。
5. 进阶：② 的扩大用**性能 fuzzing（PerfFuzz/SlowFuzz，变异目标=最大化执行次数）**替 LLM scaler，自动长出让核变热的大输入。

---

## 7. 与 opt_find 的衔接

- **热度覆盖探针的输出直接就是 opt_find 的候选集**（热的自己函数）。
- **"执行了但冷 + 判为 A"的函数**：造输入压热后并入候选。
- **"从没执行"的洞**：覆盖率 fuzz 补。
- **判为 B 的函数**：列名报"范围外"（不可验证、优化无意义），不是遗漏。
- 静态路线（C-vs-Rust IR/汇编比较）已证死，不再作为候选来源；见 `../opt_find/RESULTS.md` 与 memory `opt-find-static-signal-invalid`。
