# Class I 规则凝练(缺口聚类,基于 6 项目采集层)

**输入**:`class_I/{bzip2,libzahl,libcsv,binn,heman,xxHash}.md`(6 个回归项目的采集层差分事实)。
**层面界定**:Class I = c2rust **前端相对 C 多生成**的工作(义务符号在 Rust 优化后 IR 存活、C 侧结构性为 0);查法2 的"优化器少做"(向量化损失/未内联)归 Class II。

## 凝练原则(两条)

1. **同层面合并**:几种不同的采集模式,若揭露的是**同一缺口层面**(CPU 多做的同一种物理工作、改写同类、安全条件同类),凝练成**一条**规则——不是一模式一规则。
2. **复现门槛 = 一条规则 ≥3 个支持实例**;实例可以是 **≥3 种不同模式**(同层面),也可以是**同一模式在 ≥3 个热函数**出现。**跨项目广度不作准入**(仅如实报告为证据属性)。

按此,Class I 收敛为 **3 条规则**(C1/C2/C3),对应三个正交缺口层面:控制流检查 / 标量算术 / 访存别名。

---

## C1 — 冗余检查分支

- **缺口层面**:Rust 语义强制"检查条件 → 失败即 panic"的控制流(条件分支 + 冷 panic 块);C 无对应语义,结构性为 0。
- **合并的采集模式**(同层面,故合一条):**边界检查** `panic_bounds_check` + **Option 拆包** `expect_failed`(数组越界 / Option 空 两种语法机制,但缺口向量同一:分支+panic块)。
- **检测签名(Rust 优化后 IR,C 侧为 0)**:
  ```
  hot(f) ∧ optIR(f) ⊇ { call @...panic_bounds_check
                       | call @...expect_failed / "non-null function pointer" }
  ```
- **改写 + 安全条件**(M-a,删检查须证):
  ```
  arr[i]          ⟹ arr.get_unchecked(i)        provided ⊢ i < len(arr)
  x.expect(m)(..) ⟹ (x.unwrap_unchecked())(..)   provided ⊢ x ≠ None
  ```
- **≥3 支持实例**(6 热函数 / 3 项目):bzip2 `BZ2_decompress`(边界 74)、`BZ2_bzDecompress`(边界 10);binn `main`(拆包 22)、`AddValue`(拆包 2);libcsv `csv_parse`(拆包 4);bzip2 `malloc_fn` 拆包 6。✅

## C2 — 标量转换膨胀

- **缺口层面**:浮点→整数的 `as` 在 Rust 是**饱和**语义,展开成无分支的标量算术(min/max/select + NaN 判 + 截断);C 的 `(int)x` 是一条 `cvttsd2si`。层面区别于 C1:是**标量算术展开**而非控制流分支。
- **合并的采集模式**:**饱和 cast** `llvm.fptosi.sat` / `llvm.fptoui.sat`(同一模式)。
- **检测签名**:
  ```
  hot(f) ∧ optIR(f) ⊇ { @llvm.fptosi.sat.* | @llvm.fptoui.sat.* }
  ```
- **改写 + 安全条件**(M-a):
  ```
  f as I ⟹ f.to_int_unchecked::<I>()   provided ⊢ f 有限 ∧ f ∈ [I::MIN, I::MAX] ∧ ¬f.is_nan()
         ⟹ f.clamp(lo, hi) as I        (值域不可证时,显式钳位助优化器融合)
  ```
- **≥3 支持实例**(同一模式 5 热函数,均 heman):`fastFloor`(继承 7)、`heman_export_u8`(继承 8)、`heman_image_sample`(2/7)、`heman_lighting_compute_occlusion`(自身 1)、`heman_generate_island_heightmap`。✅
- **广度如实报告**:仅 heman 一个项目触发(语料唯一浮点密集项目);通用性由 Rust `as` 饱和语义的语言规范担保,非跨项目实证。

## C3 — 冗余访存 / 别名缺失

- **缺口层面**:rustc 不发类型别名信息(`!tbaa`),`*mut/*const` 无 `noalias` → 优化器无法证明"循环内的写不碰被缓存的地址",保守地保留跨迭代 load、无法外提循环不变 gep;C 侧有 `!tbaa`,GVN/LICM 能消除/外提。
- **合并的采集模式**(同层面:别名缺失致冗余访存):**`!tbaa` 缺失**(根因,Rust 全模块=0)+ **load/gep 过剩**(表现)。
- **检测签名**(函数级,纯 Rust 侧):
  ```
  hot(f) ∧ !tbaa ∉ meta(所有访存)  ∧  ∃ 循环不变 load 未外提 / 跨迭代冗余 load
  ; (研究期辅以 gvn:LoadClobbered / licm:LoadWithLoopInvariantAddressInvalidated remark,且 Rust ≫ C)
  ```
- **改写 + 安全条件**(根因 M-a、表现 M-b):
  ```
  fn f(.., p: *mut T, ..) ⟹ fn f(.., p: &mut [T], ..)        (改类型获 noalias)
  或标量外提:let v = (*p).field; for _ in L { … 用 v … } (*p).field = v
  provided ⊢ writes(L) ⊥ addr(p.field)                       (排除自别名 / 区间重叠)
  ```
- **≥3 支持实例**(远超 3,6/6 项目全部热函数):**全部 6 项目 Rust `!tbaa`=0 vs C 有**(bzip2 `BZ2_decompress` 0/1394、binn `main` 0/230、xxHash `main` 0/113、libcsv `csv_parse` 0/103、libzahl 各函数、heman 各函数);冗余 load 最显 libzahl `zrsh` 73/24、bzip2 `BZ2_decompress` 1010/755。✅✅ 本语料最普遍的规则。

---

## 排除记录(用同一 ≥3 门槛复查,确认"不多")

- **溢出检查** `with.overflow`:差分实例分散在各 harness `main`(binn 2、heman 1、xxHash 1,均继承自源码显式 `overflowing_add`,属 harness 而非被翻译库热函数);libzahl `zadd` 则两侧都有(Rust `overflowing_add` / C `zadd_impl`,非差分)→ 集中度 <3 且非库,排除。
- **除零 / panic** `core::panicking::panic`:仅 xxHash `XXH3_hashLong_internal_loop` 1 处(nodbg)→ 单处 <3,不成规则,排除。
- **bswap** `@llvm.bswap`:6 项目两侧几乎全 0(唯 xxHash C main 5),字节序未折叠、非差分 → 排除。
- **unwind**(`landingpad`/`invoke`):仅出现于 harness/std 预编译代码(nodbg,`panic=abort` 下的框架残留),c2rust 翻译的库热函数为零 → 来源不属翻译器,排除。
- **memcpy/memset**:对照项,两侧皆有 → 非差分。
- **向量化损失**(C 有 Rust 少):仅 xxHash `main` 1 处(C auto-vec 213 vs Rust 20);bzip2/heman 反而 Rust 向量 ≥ C。单处 → 不属 Class I,归 **Class II(II① 向量化损失)**,其 ≥3 门槛在 Class II 阶段判。

## 小结

Class I 恰好 **3 条**,各过 ≥3 支持实例门槛且揭露正交缺口层面:
- **C3(冗余访存)** — 6/6 项目,本语料最普遍;
- **C1(冗余检查)** — 3 项目 6 热函数;
- **C2(标量转换)** — heman 5 热函数(单项目,语言规范担保通用)。

检测统一在**目标项目 Rust 侧优化后 IR**(C1/C2=义务符号存活;C3=`!tbaa`=0 + 冗余访存),部署不需 C 对照;源码仅作改写落点(`!dbg` 映射 IR 站点回源码行)。改写的安全档:C1/C2 是**值域/非空证明**(删检查/换转换),C3 是**不别名证明**(恢复别名保证)。
