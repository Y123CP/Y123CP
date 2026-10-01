# SVF -- 指针分析工具

因为环境问题，没有使用SVF的源码进行编译，而是使用SVF的docker镜像:

```bash
docker pull svftools/svf:latest
```

## 如何工作

给定一个要分析的C project, 然后将这个C 挂载到svf 的 docker 当中, 以quadtree_0_1_0为例

```bash
docker run -it  -v /home/usr/CodeTrans_c2Rust/dataset/original_projects/quadtree_0_1_0:/mnt/quadtree svftools/svf:latest /bin/bash
```

第1步：进入docker后，使用 `cd /mnt/quadtree` 进入项目目录
第2步：编译项目生成 LLVM IR。需要使用 clang 编译器将 C 文件编译成 LLVM IR：

```bash
# 创建一个目录存放生成的 LLVM IR 文件
mkdir -p llvm-ir

# 编译所有 C 文件为 LLVM IR
clang -c -emit-llvm -g src/bounds.c -o llvm-ir/bounds.bc
clang -c -emit-llvm -g src/node.c -o llvm-ir/node.bc
clang -c -emit-llvm -g src/point.c -o llvm-ir/point.bc
clang -c -emit-llvm -g src/quadtree.c -o llvm-ir/quadtree.bc
clang -c -emit-llvm -g test.c -o llvm-ir/test.bc
clang -c -emit-llvm -g benchmark.c -o llvm-ir/benchmark.bc
# 指定 include 文件夹
 clang -c -emit-llvm -g -I./include alignalloc.c -o llvm-ir/alignalloc.bc

# 将所有 bitcode 文件链接成一个文件
llvm-link llvm-ir/*.bc -o llvm-ir/quadtree_all.bc
```

2.1 查看 LLVM IR 内容

```bash
# 使用 llvm-dis 查看 LLVM IR 内容
llvm-dis /mnt/quadtree/llvm-ir/quadtree_all.bc -o /mnt/quadtree/llvm-ir/quadtree_all.ll
```

![源代码定位](image/源代码定位.png)

3. 使用 SVF 进行分析

```bash
cd ~/SVF/Release-build/bin/
```

3.1 获取 给定项目的 call graph

```bash
./wpa -ander -dump-callgraph /mnt/quadtree/llvm-ir/quadtree_all.bc # 默认在当前目录下生成 callgraph_final.dot 
# NOTE: 生成的call graph 只有函数之间的调用。函数和 struct|enum|union.. 之间的调用关系不会在call graph当中显示
```

3.2 获取 给定项目的 pointer assignment graph (PAG)

Program Assignment Graph (PAG) 是静态程序分析中的核心数据结构，尤其在指针分析中广泛使用。它通过 **节点（Node）和边（Edge）的形式，刻画程序中变量（Variables）和内存对象（Memory Objects）** 之间的赋值、引用及操作关系。简单来说，PAG 是程序操作和内存关系的图形化表示，帮助分析工具（如SVF）追踪指针的指向、数据流传递及潜在问题。

```bash
./wpa -ander -dump-pag /mnt/quadtree/llvm-ir/quadtree_all.bc # 默认在当前目录下生成 pag.dot
```

![PAG](image/PAG.png)

3.3 使用 svf 进行指针分析
是用来如下的指令 [有待进一步研究]

```bash
./wpa -ander -print-pts -print-field -show-ir-value /mnt/quadtree/llvm-ir/quadtree_all.bc > /mnt/quadtree/llvm-ir/with_details.txt
./wpa -ander -print-pts -print-type -print-aliases -show-ir-value /mnt/quadtree/llvm-ir/quadtree_all.bc > /mnt/quadtree/llvm-ir/more_details.txt
```
./wpa -ander -cxt -print-all-pts -alias-check -type-check -field-limit=100000 -print-aliases /home/usr/CodeTrans_c2Rust/dataset/quadtree_0_1_0_expanded_dealed/svf_analysis_output_test/quadtree_all.ll -dump-json -human-readable analysis_result.json
---

# SVF 工具
```bin$ ls
ae  cfl  dvf  llvm2svf  mta  saber  svf-ex  wpa
```
说明
wpa	--- Whole Program Pointer Analysis	--- 整个程序的指针分析工具（如果要进行指针分析，用的就是它）
mta	--- Multi-threaded Analysis	--- 多线程程序分析
saber --- Static Analyzer for Buffer Overflow	--- 缓冲区溢出静态分析器
dvf ---	Data-flow Vulnerability Finder --- 数据流漏洞检测工具
cfl --- Control-flow Logic Analysis --- 控制流逻辑分析
ae --- Abstract Interpretation Engine --- 抽象解释引擎
llvm2svf --- LLVM IR to SVF IR Converter --- LLVM IR转换工具
svf-ex --- SVF Examples	--- 示例程序

# WPA (Whole Program Analysis) 工具详解

WPA 是 SVF 框架中用于全程序指针分析的核心工具。让我为您详细解析其功能和关键参数：

## 基本功能

WPA 执行全程序静态分析，主要为了确定程序中指针的可能指向关系、别名关系以及构建各种程序表示图（如指针赋值图、值流图等）。

## 关键参数分类

### 1. 指针分析算法选择

最核心的选项决定了使用哪种算法：

```bash
./wpa -ander        # 使用改进的波动传播包含型分析（最常用，平衡精度和效率）
./wpa -nander       # 标准的包含型分析
./wpa -sander       # 带选择性循环检测的包含型分析 
./wpa -sfrander     # 带字段表示的包含型分析
./wpa -steens       # Steensgaard的指针分析（最快但精度较低）
./wpa -fspta        # 流敏感指针分析（更精确但耗时）
./wpa -vfspta       # 带版本控制的流敏感指针分析（最精确但也最耗时）
./wpa -type         # 基于类型的快速分析（用于构建调用图等）
```

### 2. 结果输出控制

控制分析结果如何显示：

```bash
./wpa -print-pts             # 打印顶层指针的指向集
./wpa -print-all-pts         # 打印所有指针（包括地址获取变量）的指向集
./wpa -print-aliases         # 打印所有指针对的别名关系
./wpa -print-type            # 打印类型信息
./wpa -show-ir-value         # 在输出中显示LLVM IR值
./wpa -print-field           # 打印带基对象ID前缀的字段对象
```

### 3. 图形生成

生成各种程序分析图：

```bash
./wpa -dump-pag              # 生成指针赋值图（PAG）
./wpa -dump-vfg              # 生成值流图（VFG）
./wpa -dump-callgraph        # 生成调用图
./wpa -dump-constraint-graph # 生成约束图
```

### 4. 分析精度控制

调整分析的精度与性能平衡：

```bash
./wpa -field-limit           # 字段敏感分析的最大字段数
./wpa -model-arrays          # 为数组访问建模
./wpa -model-consts          # 为常量对象建模
./wpa -cxt-limit             # 上下文敏感分析的上下文限制
```

### 5. 特殊分析

针对特定问题的分析：

```bash
./wpa -alias-check           # 启用别名检查
./wpa -leak                  # 内存泄漏检测
./wpa -dfree                 # 双重释放检测
./wpa -overflow              # 缓冲区溢出检测
```

## 常用命令组合

### 基本指针分析

```bash
./wpa -ander -print-pts /path/to/your.bc
```

### 详细指针分析（包含类型信息）

```bash
./wpa -ander -print-pts -print-type -show-ir-value /path/to/your.bc
```

### 别名分析

```bash
./wpa -ander -print-aliases /path/to/your.bc
```

### 生成分析图

```bash
./wpa -ander -dump-pag -dump-callgraph /path/to/your.bc
```

### 流敏感分析（更精确但更慢）

```bash
./wpa -fspta -print-pts /path/to/your.bc
```

## 参数解释技巧

查看帮助文档中的参数时：

* 带有 `-dump-*`的通常生成图文件
* 带有 `-print-*`的通常输出文本信息
* 不带等号的通常是布尔选项（添加即启用）
* 带等号的需要指定值（如 `-pt-type=sbv`）

## 何时使用不同的分析算法

* **-ander** : 适合大多数情况，良好的精度/性能平衡点
* **-steens** : 当速度是最重要因素时
* **-fspta** : 当精度是最重要因素且程序规模允许
* **-type** : 当只需要快速建立调用图时
