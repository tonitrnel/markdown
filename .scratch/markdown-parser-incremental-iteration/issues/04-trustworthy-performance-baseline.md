# Trustworthy performance baseline

**Type:** task
**Status:** resolved
**Blocked by:** None - can start immediately.

## Question

What corrected benchmark lanes, semantic comparison, allocation accounting, and reference-machine record are required before v2A can decide whether a module refactor earned its place?

## Resolution criteria

- The allocation benchmark reads an existing fixture and reports per-parse alloc/realloc/dealloc counts, bytes, and a true median.
- CommonMark, shared GFM, and OFM product lanes make their comparison scope explicit.
- Parse-only, render-only, and parse-plus-render results consume their values and can be reproduced from the recorded command.
- A deterministic semantic digest catches structural, position, metadata, and HTML changes.

## Comments

- Its answer must provide the evidence used by [Choose v2A module gates and order](05-choose-v2a-module-gates-and-order.md).
- Claimed on 2026-07-26 to establish the measurement evidence for v2A decisions.

## Answer

P0 现在有一份可复现的基线。它只测量当前解析器；不选择 v2A module，也不声称与 `pulldown-cmark` 语义等价。

### 护栏

- `bench/benches/alloc_count.rs` 内嵌既有的 `bench/fixtures/curated/_data.md` fixture，预热一次后测量 500 次解析。输出按每次解析统计，包含 alloc/realloc/dealloc 次数、各自请求的字节数和偶数样本的真实中位数。`REALLOC_BYTES` 累加 `new_size`。
- `bench/benches/phase_bench.rs` 分别提供 `parse_ast_only`、`html_render_only` 和 `full_parse_and_html` 通道。每个被消费的结果都传入 `criterion::black_box`。
- `bench/compare/native/benches/parser_compare.rs` 明确区分 CommonMark、共享 GFM 与 OFM 产品通道。CommonMark 使用本地默认解析选项和空的 `pulldown-cmark` 选项；共享 GFM 仅限表格、删除线和任务列表。只有前两条通道将本地完整 AST 路径与 `pulldown-cmark` 事件消费或 HTML 渲染对照；OFM 只记录本地绝对性能。
- `tests/support/semantic.rs` 构建确定性的摘要，覆盖树前序和父子关系、位置、节点类型与 payload、BlockId、排序后的文档 tags 及最终 HTML。`tests/semantic_digest.rs` 证明它能捕获结构、payload、位置、元数据和 HTML 变化。

### 参考机器

| Field | Value |
| --- | --- |
| 日期 | 2026-07-26 |
| 提交 | `0ea9a8917b5def57d42d908f2d877a2e936062b0`（`dev/v2`，包含本 P0 工作树改动） |
| 操作系统 | macOS 26.5.2 (25F84) |
| CPU | Apple M1 |
| 内存 | 16 GiB |
| Rust | `rustc 1.96.0 (ac68faa20 2026-05-25)`、LLVM 22.1.2、`aarch64-apple-darwin` |
| Cargo | `cargo 1.96.0 (30a34c682 2026-05-25)` |
| Criterion 配置 | 默认：预热 3 s、测量目标 5 s、100 个样本 |

当前 crate 会为既有的 `correct_cjk_spacing` 发出一条无关的 `dead_code` 警告；它不影响以下命令的成功状态。

### 记录的命令

```bash
cargo fmt --check
cargo test --test semantic_digest
cargo test --all-features
cargo bench --bench benchmark --no-run
cargo bench --bench phase_bench --no-run
cargo bench -p parser-compare-bench --bench parser_compare --no-run
cargo bench --bench alloc_count
cargo bench --bench phase_bench
cargo bench -p parser-compare-bench --bench parser_compare
```

比较器也可按单个完整 Criterion 结果复现：

```bash
cargo bench -p parser-compare-bench --bench parser_compare -- '<phase>/<lane>/<implementation>/<dataset>'
```

其中 `phase` 为 `parse_only` 或 `parse_and_html`；`lane` 为 `commonmark`、`shared_gfm` 或 `ofm_product`；`implementation` 是该 phase 支持的 `markdown`、`pulldown_cmark_events` 或 `pulldown_cmark_html`；`dataset` 为 `curated` 或 `markdown_it_corpus`。

### 结果

测试均通过：`semantic_digest` 有 5 个测试，`cargo test --all-features` 在 25 个套件中通过 619 个测试。三个 benchmark target 全部编译成功。以下点估计为 Criterion 报告的置信区间中点，仅适用于这台机器。

#### 分配和阶段通道

```text
parses=500 allocs_per_parse=20571.00 reallocs_per_parse=1910.00
deallocs_per_parse=20571.00 alloc_bytes_per_parse=2885133.00
realloc_bytes_per_parse=3195447.00 dealloc_bytes_per_parse=4492367.00
median_us=2256.04
```

| 通道 | Curated fixture 估计值 |
| --- | ---: |
| parse-only | 2.3465 ms |
| render-only | 0.7901 ms |
| parse-plus-render | 3.0945 ms |

#### 比较通道

| Phase 与通道 | Curated 本地 | Curated pulldown | markdown-it 本地 | markdown-it pulldown |
| --- | ---: | ---: | ---: | ---: |
| parse-only CommonMark | 2.4181 ms | 0.8661 ms (events) | 7.8715 ms | 2.8101 ms (events) |
| parse-only shared GFM | 2.2783 ms | 0.8283 ms (events) | 7.3242 ms | 2.8048 ms (events) |
| parse-only OFM product | 2.4003 ms | N/A | 7.4146 ms | N/A |
| parse-plus-render CommonMark | 3.0539 ms | 0.9811 ms | 10.732 ms | 2.9750 ms |
| parse-plus-render shared GFM | 3.3256 ms | 0.9911 ms | 9.5848 ms | 2.9778 ms |
| parse-plus-render OFM product | 3.1055 ms | N/A | 9.7234 ms | N/A |

### v2A 决策输入

这些测量确定了回归通道和当前分配负担。它们**不能**独自证明某项架构选择：`pulldown-cmark` 消费事件而不构建本地的完整 AST，OFM 通道也刻意没有跨实现的对照对象。下一张票只能选择可逆的 module 切片，它必须在命名通道与语义摘要中证明改进，同时保持上述不回归通道。
