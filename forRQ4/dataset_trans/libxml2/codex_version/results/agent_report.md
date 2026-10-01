# libxml2 Rust 性能优化报告

## 本次修改

1. `code/src/xmlstring.rs`
   - `xmlStrdup` 改为直接调用 libc `strlen` 获取长度，再走既有 `xmlStrndup` 分配拷贝路径。
   - `xmlCharStrdup` 同样改为使用 `strlen`，去掉手写逐字节长度扫描。
   - `xmlStrcat` 改为用 `strlen` 计算追加串长度，再调用既有 `xmlStrncat`。
   - 意图：这些函数是解析器、树操作、XPath、命名空间处理等共享的基础字符串原语。转译后的手写扫描会在每次调用时重复执行字节循环；改为 `strlen` 后可直接受益于 libc 的高度优化实现。

2. `code/src/dict.rs`
   - `xmlDictFindEntry` 的 QName 命中判断不再调用 `xmlStrQEqual` 做两段逐字节扫描，而是复用查找阶段已经算出的 `plen`/`len`，直接：
     - 比较 prefix 前缀
     - 检查 `:`
     - 比较 local name
     - 检查末尾 `\\0`
   - `xmlDictLookupInternal` / 子字典查找路径同步传递 `len` 和 `plen`，避免比较阶段重新扫描字符串。
   - 意图：字典查找位于元素名、属性名、QName、命名空间前缀等高频 intern 热路径中。原实现即使命中 hash，也会再次把 prefix/name 从头扫描一遍；新实现把已有长度直接用于定长比较，减少重复工作。

3. `work/microbench.rs`
   - 新增最小微基准，仅用于本地观察“旧实现模式”和“新实现模式”的相对成本。

## 收益来源判断

- `xmlstring` 的收益主要来自：减少显式 Rust/C2Rust 逐字节循环，改走 libc `strlen` 的优化路径。
- `dict` 的收益主要来自：QName 查找命中时避免重复扫描 `prefix` 和 `name`，把哈希阶段已得的长度直接复用到定长比较。
- 这两类优化都属于“底层公用热函数提速”，理论上能覆盖 parser/tree/xpath 多条上层调用链，而不是只优化单个功能分支。

## 编译验证

- 已执行：`cargo build --release`（目录：`code/`）
- 结果：编译通过。
- 说明：项目本身存在大量既有 warning，但本次修改后 release 构建成功。

## 本地自测

### 微基准说明

- 基准文件：`work/microbench.rs`
- 方法：在同一台机器上，对“旧实现模式”和“新实现模式”分别执行大量迭代，测单次平均纳秒开销。
- 注意：这是针对底层字符串/字典比较原语的微基准，不等价于整体 XML 解析吞吐，只能作为方向性参考。

### 结果

两次运行结果如下：

1. 第 1 次
   - `scan_short_ns_per_iter old=5.645 new=5.504`
   - `scan_medium_ns_per_iter old=14.162 new=2.757`
   - `scan_long_ns_per_iter old=32.974 new=3.356`
   - `qname_eq_ns_per_iter old=10.748 new=6.855`

2. 第 2 次
   - `scan_short_ns_per_iter old=2.596 new=2.241`
   - `scan_medium_ns_per_iter old=13.221 new=2.778`
   - `scan_long_ns_per_iter old=32.708 new=3.352`
   - `qname_eq_ns_per_iter old=8.919 new=6.720`

### 观察

- 中长字符串长度扫描的收益最明显，`strlen` 路径明显快于手写逐字节循环。
- QName 比较路径也有稳定正收益，量级大约在 20% 到 35% 左右。
- 短字符串收益较小，但方向仍为正。

## 修改文件清单

- `code/src/xmlstring.rs`
- `code/src/dict.rs`
- `work/microbench.rs`
- `results/agent_report.md`

## 结论声明

Agent 自测数据仅供参考；我们一定会在拿到这份优化结果后，使用外部后验脚本重新编译并统一测量，并以外部测量结果作为最终性能结论。
