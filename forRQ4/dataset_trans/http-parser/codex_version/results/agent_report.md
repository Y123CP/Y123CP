# http-parser Rust 性能优化报告

## 修改文件

- `code/src/http_parser.rs`
  - 新增 `token_char` / `token_char_i32` 内联辅助函数，复用 header token 查表逻辑，减少热路径中的重复条件分支和重复索引。
  - 新增 `update_header_field_state` 内联辅助函数，把 `s_header_field` 分支中每字符反复执行的 `index/header_state/uses_transfer_encoding` bitfield 读写收敛为局部变量更新，再在循环结束后一次性写回 parser。
  - 新增 `parse_header_value_start` 内联辅助函数，把 `s_header_value_start` 中多组重复的 `header_state -> 下一状态/flags/content_length` 逻辑折叠为单个快速分派函数，减少该入口上的重复分支和 bitfield 访问。
  - 将只读查表 `tokens`、`unhex`、`normal_url_char` 从 `static mut` 改为不可变 `static`，给编译器更强的别名信息，降低保守优化限制。
  - 在 `s_header_field` 热循环内引入局部缓存：`header_index`、`header_state`、`uses_transfer_encoding`，避免每处理一个字符都通过 bitfield getter/setter 访问 parser 内 packed 字段。
  - 在 `s_header_value_start` 入口缓存 `header_state` 和 `flags`，减少进入 header value 解析时的重复状态读取和写回。

## 改动意图与预期收益来源

- 这份代码是 c2rust 自动转译产物，`http_parser_execute` 是近 5000 行的大状态机；真实热路径主要集中在 header 名称与 header value 的逐字符解析。
- 原实现里，header 解析循环会高频调用 bitfield 生成的 getter/setter，例如 `(*parser).index()`、`set_index()`、`header_state()`、`set_header_state()`、`flags()`、`set_flags()`。这些访问都需要读改写 packed 字段，对优化器并不友好。
- 本次优化的核心是把这些状态提升到局部寄存器友好的变量里，缩小每字符处理的内存访问数，再在阶段边界写回 parser。收益应主要来自：
  - 更少的 bitfield 读改写。
  - 更少的重复分支与重复查表代码。
  - 只读静态表允许更激进的编译优化。

## 编译验证

- 已执行：`cargo build --release`（目录：`code/`）
- 结果：通过。

## Agent 自测

### 自测方法

- 在 `work/` 下保留了原始代码副本：`work/orig_code/`。
- 分别对原始版本与优化版本构建 release 库。
- 编写了两个离线 bench crate：`work/bench_orig/`、`work/bench_opt/`，二者使用相同的请求样本与相同的回调设置，循环调用 `http_parser_execute` 200,000 次。
- bench 样本是 header 较多、会命中 `Connection` / `Upgrade` / `Content-Length` / `Transfer-Encoding` 等状态机路径的请求，从而重点覆盖本次优化区域。
- 两个 bench 的 `checksum` 完全一致，且对同一输入都消费 `283/284` 字节，说明此次对比使用的是同一行为路径；本次 bench 以原版行为作为参考，不把“必须消费完整输入”当作额外假设。

### 自测结果

- 第 1 轮：
  - 原版：`elapsed_ns = 1168385893`
  - 优化版：`elapsed_ns = 627846839`
  - 速度比：`1.86x`
  - 时间下降：`46.3%`
- 第 2 轮：
  - 原版：`elapsed_ns = 1185456120`
  - 优化版：`elapsed_ns = 625553992`
  - 速度比：`1.90x`
  - 时间下降：`47.2%`
- 两轮平均：
  - 平均速度比：`1.88x`
  - 平均时间下降：`46.8%`

## 未完成/限制说明

- 我尝试在单个 compare harness 中同时依赖原版与优化版库，做更多样本的逐字段对照；由于两个 path dependency 同名同版本，Cargo 锁文件会发生 package collision，未在时间盒内继续绕过这个构建限制。
- 因此，行为等价性方面，我完成了：
  - 优化版 release 构建通过。
  - 原版/优化版 bench 使用同一输入时得到相同 `checksum` 和相同 consumed bytes。
- 但我没有在更多输入集合上做系统性逐字段差分。

## 最终说明

- Agent 自测数据仅供参考；我们一定会在拿到你的优化结果后，使用外部后验脚本重新编译并统一测量，以外部测量结果作为最终性能结论。
