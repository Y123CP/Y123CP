已完成优化，核心源码改动在 [code/src/http_parser.rs](/home/anonymous/artifact/PerfTrans/dataset_trans/http-parser/codex_version/code/src/http_parser.rs:1419)，说明已写入 [results/agent_report.md](/home/anonymous/artifact/PerfTrans/dataset_trans/http-parser/codex_version/results/agent_report.md:1)。

本次实际修改的文件：
- `code/src/http_parser.rs`
  - 把 `tokens`、`unhex`、`normal_url_char` 这几张纯只读查表改成不可变静态，减少别名限制。
  - 为 header 热路径加了内联辅助函数：`token_char`、`token_char_i32`、`update_header_field_state`、`parse_header_value_start`。
  - 重写了 `http_parser_execute` 里最热的 header 名称解析分支和 header value 起始分支，用局部变量缓存 `index`、`header_state`、`uses_transfer_encoding`、`flags`，减少每字符一次的 bitfield getter/setter 开销。
  - 我认为收益主要来自：更少的 packed bitfield 读改写、更少的重复分支、以及更利于编译器优化的只读查表访问。
- `results/agent_report.md`
  - 记录了改动意图、构建验证和自测结果。
- `work/bench_orig/`、`work/bench_opt/`、`work/bench_common_main.rs`
  - 只是自测基准，不属于交付源码。
- `work/orig_code/`
  - 原始版本副本，用于同环境前后对比。

自测结果：
- 已执行 `cargo build --release`（`code/` 下），编译通过。
- 基准输入覆盖了 `Connection`、`Upgrade`、`Content-Length`、`Transfer-Encoding` 等本次优化的 header 路径。
- 第 1 轮：原版 `1168385893 ns`，优化版 `627846839 ns`，约 `1.86x`，耗时下降 `46.3%`。
- 第 2 轮：原版 `1185456120 ns`，优化版 `625553992 ns`，约 `1.90x`，耗时下降 `47.2%`。
- 两轮平均：约 `1.88x`，平均耗时下降 `46.8%`。

限制说明：
- 我额外尝试做原版/优化版双库逐字段对照 harness，但因为两个 path dependency 同名同版本，Cargo lockfile 发生 package collision，没有在时间盒内继续绕过。
- 目前我确认了：优化版 release 可编译；原版和优化版在 bench 输入上的 consumed bytes 与 callback checksum 一致。

Agent 自测数据仅供参考；我们一定会在拿到你的优化结果后，使用外部后验脚本重新编译并统一测量，以外部测量结果作为最终性能结论。