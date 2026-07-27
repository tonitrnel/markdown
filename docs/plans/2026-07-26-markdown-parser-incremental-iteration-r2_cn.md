# Markdown 解析器渐进式迭代计划（r2 修订）

**状态：** Wayfinder 决策地图期间的执行假设，取代 [初版渐进式迭代计划](2026-07-26-markdown-parser-incremental-iteration_cn.md) 作为当前工作假设；同时作为 map ticket [05 Choose v2A module gates and order](../../.scratch/markdown-parser-incremental-iteration/issues/05-choose-v2a-module-gates-and-order.md) 的证据与候选答案输入。权威仍是 [Markdown Parser Incremental Iteration Map](../../.scratch/markdown-parser-incremental-iteration/map.md)；ticket 05 收敛后，本文按票面结论收敛为 spec 和 blocker-ordered tracer tickets。初版计划的问题证据、设计来源与已定决策（1–5 条）全部继续有效，本文只记录差异与新证据，不重复初版正文。

## r2 与初版的差异摘要

1. **新增 B1（正确性修复，优先于全部性能阶段）**：实测确认脚注编号跟随 pending inline HashMap 遍历顺序而非文档顺序的 bug（详见下文与 [bug ticket](../../.scratch/footnote-numbering-order/issues/01-footnote-numbering-follows-hashmap-order.md)）。初版把文档顺序完全交给 P4，会让一个用户可见的正确性问题排在三个性能阶段之后。
2. **新增 P0.5（测量通道补齐）**：初版 P1–P4 的验收门槛引用了"plain-text hotspot""delimiter-dense hotspot""link-dense""many-short-paragraphs""reference-heavy"等 lane，但 `bench/benches/hotspots.rs` 当前只有 3 个用例（`plain_ascii_4k`、`many_flushes_dense_inline`、`multiline_blockquote_dense`），且 `alloc_count` 只测 `_data.md` 单一 fixture、`bench/results/` 目录不存在。**门槛引用的 lane 必须先存在，各阶段才能"妥善运行"。**
3. **显式纳入 map ticket 06/07/08 的决策工作（D1）**：初版执行顺序从 P0 直接跳到 F1，遗漏了它自己的权威（map）要求的两个接口原型与 05 号收敛票。F/P 各阶段的实施必须发生在 05 收敛之后。
4. **P1 证据加强并细化改写点**：新采样确认内存子系统约占 parse-only 的 1/3，P1 是最大杠杆；同时列出全部四类假定 owned `String` 的文本改写位点，作为实现清单。
5. **P2/P3 期望校准**：语料级 delimiter 链自身成本仅约 1.3%，两阶段的主要收益是分配次数与后续提交模型的前置条件，门槛保持热点级不变。
6. **P5 扩展**：除 frees 集合外，同时压缩 `TreeNode` 的 4 个 `Option<usize>` 链接字段（每槽 −32 B，约 −21%）。
7. **P6/P7 证据更新**：Block 侧占比按新采样约 22%（不含行提取）～27%（含行提取，线程口径），行提取与位置预计算不属于 matcher 分派可省部分，P6 触发判断需独立测量；P7 增补位置模型（~10%）、渲染临时分配与 WASM 导出深拷贝三项记录要求。

## 新证据（2026-07-26，参考机器实测）

参考机器与工具链同 [issue 04](../../.scratch/markdown-parser-incremental-iteration/issues/04-trustworthy-performance-baseline.md)（Apple M1、macOS 26.5.2、rustc 1.96.0）。

### 基线复现

`cargo bench --bench alloc_count`（`dev/v2-temporary` @ `d55e180`，工作树干净）：

```text
parses=500 allocs_per_parse=20571.00 reallocs_per_parse=1910.00 deallocs_per_parse=20571.00
alloc_bytes_per_parse=2885133.00 realloc_bytes_per_parse=3195447.00 dealloc_bytes_per_parse=4492367.00
median_us=2142.77
```

分配计数与 issue 04 完全一致；中位耗时 2142.77 µs 对记录值 2256.04 µs，跨次波动约 5%，属正常范围。

### parse-only 自顶栈归因（新采样）

方法：`CARGO_PROFILE_BENCH_DEBUG=line-tables-only CARGO_PROFILE_BENCH_STRIP=none cargo bench --bench phase_bench --no-run`，随后 `/usr/bin/sample <pid> 12` 采样 `parse_ast_only`（OFM curated）稳态运行。线程共 9,578 个样本，其中 `Parser::parse` 子树 8,112（84.7%）、`Document` 析构子树约 828（8.6%）、其余为 criterion 与杂项。自顶栈（self-time）分类：

| 类别 | 样本 | 占线程 | 主要符号 |
| --- | ---: | ---: | --- |
| 内存子系统 | ≈3,085 | ≈32% | malloc/free/realloc 族 ≈2,100、`memmove` 612、`memset`/`bzero` ≈250、`RawVec` 增长胶水 ≈69 |
| Inline 处理 | ≈2,242 | ≈23% | `inlines::process` 981、`accumulate_run` 629、`flush_text_acc` 200、link/entity/escape 族 ≈300 |
| Block 扫描（不含行提取） | ≈2,090 | ≈22% | `continue_parse_checked` 自身 1,260、`skip_to_eol` 371、`blocks::after` 171、`table::parse_columns` 106 |
| 行提取与位置 | ≈973 | ≈10% | `Span::extract` 504、`MergedSpan::start_location` 216、`location_at_byte` 130 |
| Tree 操作与析构 | ≈739 | ≈8% | `append_child` 196、`drop TreeNode` 189、`append` 103、`remove`/`set_parent`/`unlink` ≈216 |
| HashMap | ≈185 | ≈2% | insert/rehash |

其中 delimiter 链自身（`process_final`、`remove_delimiter`、标点判定）合计约 1.3%。若按 `parse()` 内部口径（扣除析构与 harness），Block 侧含行提取约 32%、不含约 26%。

三个直接结论：

- **分配即时间**：约 1/3 的 parse-only 时间在内存子系统里；`Document` 析构又占 8.6% 且与分配数成正比。分配次数/字节的下降会直接兑换为时间，P1 的优先级由此成立。
- **delimiter 链是分配问题不是时间问题（语料级）**：P2/P3 的语料级时间期望应保守，热点级门槛照旧。
- **Block 侧占比处于 P6 触发线附近，但构成复杂**：`Span::extract` 与 `skip_to_eol`（合计 ≈9%）是行提取，matcher 预筛（LineHead）帮不了它们；P6 的 25% 触发线必须以"分派可省部分"的独立测量为准。

### 实测正确性 bug：脚注编号乱序

20 个段落各引用一个脚注（`[^n1]`…`[^n20]`，定义在文末），OFM 解析后前三个段落渲染为 `[14]`、`[7]`、`[1]`，脚注列表同样乱序。原因链：`parse_inlines` 按 FxHashMap 遍历顺序 drain pending Block（`src/parser.rs:444-458`）→ 脚注引用编号在 inline 处理时按 `footnote_refs.len()+1` 赋值（`src/inlines/footnote.rs:82-83`）→ `parse_footnote_list` 只按已赋 index 排序（`src/parser.rs:544`）。单 Block 内引用不受影响，故现有 fixture 未捕获。详见 [bug ticket](../../.scratch/footnote-numbering-order/issues/01-footnote-numbering-follows-hashmap-order.md)。

这同时说明：selective 设计中"事件遍历和选中 Inline 处理都使用文档前序"的要求（2026-07-19 spec）在当前完整解析路径上并不成立，F3 与 P4 共享同一个待修前提。

### 结构事实（代码证据，供各阶段引用）

- `TreeNode<Node>` ≈ 152 B/槽：`Option<Node>`（≈80 B，其中 `Location`×2 = 32 B）+ `parent: usize`（8 B）+ 4×`Option<usize>` 链接（64 B，无 niche）；槽位只增不回收（`src/tree.rs:38-48,189-214`）。curated 语料 ≈12,942 槽、11.2% 失效。
- pending inline 主存储：`FxHashMap<usize, Vec<Span>>`（`src/parser.rs:216`），每个可接收 Inline 的 Block 一个 `Vec<Span>`；`MergedSpan` 以移动方式接管该 Vec，无二次分配（`src/inlines/mod.rs:38-56`）。
- 每个新文本 run 一个 `String`（`content.to_owned()`，`src/parser.rs:880-883`）；`TextAccumulator` 已有零分配 `Slice` 快路径（`src/inlines/mod.rs:312-327`）。
- 每个 emphasis 类 run：一个 `Rc<RefCell<Delimiter>>` + 一个 marker `String` Text 节点（`src/inlines/delimiter.rs:28-29,385-420`）；每个 `[`/`![`：一个 `Rc<RefCell<Bracket>>` + `"["` String（`src/inlines/bracket.rs:27-28,80-101`）。
- 假定 owned `String` 的文本改写位点（P1 实现清单）：
  1. 相邻 Text 合并 `push_str`/兄弟移除（`src/inlines/text.rs:7-66`）；
  2. delimiter 收尾 `truncate` 与引号改写（`src/inlines/delimiter.rs:519-544,611-703`）；
  3. smart punctuation / CJK 的 `Cow::Owned` 整体替换（`src/parser.rs:824-905`、`src/inlines/text.rs:87-107`）;
  4. `append_text_to*` 新建路径（`src/parser.rs:798-981`）。
- 渲染为"单一输出缓冲 + 每节点小额 `format!` 临时分配"（`src/render/html.rs:33-178,617,825-863`）；转义按需 `Cow`。
- WASM AST 导出对每个节点深拷贝 `body`（含全部 Text String）再序列化（`wasm-binding/src/lib.rs:78-108`）。

## 共同护栏

沿用初版全部护栏：每阶段独立提交、结束时解析器可工作、至少一项可观察结果；全阶段运行

```bash
cargo fmt --all -- --check
cargo test --workspace --all-features
cargo test --doc
```

影响公共类型、Serde 或 binding 时加运行 `cargo check -p markdown-binding --target wasm32-unknown-unknown`。性能阶段固定机器/toolchain/fixture 并记录、以 Criterion 中位数与 allocation count 判断、完整 `_data.md` parse-only 回归 ≤2%、门槛不过当场回退、结果追加至 `bench/results/incremental-iteration.md`。

r2 增补两条：

- 每个性能阶段前后各跑一次 `cargo bench --bench alloc_count`（P0.5 后为多 fixture 输出）并把两份输出一起记录。
- 每阶段结束 `cargo test --test semantic_digest` 必须通过；语义摘要相对上一阶段不变，唯 B1 属预期语义修正（脚注编号），须在结果记录中注明并以修复后输出为新基线。

`5 ms`、CommonMark/共享 GFM 相对 `pulldown-cmark` `2x`、分配降低 50% 仍为**挑战目标**，不是任何阶段的进入条件；即使 P1–P5 全部达标也不自动承诺达成，由 P7 依证据裁决。

## 阶段

### P0: 测量与语义护栏 — 已完成，仅余补录

Issue 04 已解决：alloc_count 修复（内嵌 fixture、预热、按次均值、真实中位数、`REALLOC_BYTES` 累加 `new_size`）、phase_bench 三通道、parser_compare 三 lane、semantic digest 及其 5 个测试均已落地并有完整基线记录。

**剩余动作：**

- 创建 `bench/results/incremental-iteration.md`，收录 issue 04 的机器记录/命令/结果表、上文的复现输出与自顶栈归因表（注明采样方法）。
- 处理 `src/utils/cjk.rs:49` 的 `correct_cjk_spacing` dead_code 警告：确认无调用者后删除，或保留并加 `#[allow(dead_code)]` 与保留理由注释。

**验收：** `cargo bench --bench alloc_count` 输出与记录一致；`cargo test --workspace --all-features` 无该警告；results 文件存在。

### P0.5: 补齐门槛所需的 lane 与 per-fixture 分配统计

**交付：** 后续每个阶段门槛所引用的 benchmark lane 都真实存在、可单独运行。

**范围：**

- `bench/benches/hotspots.rs` 增加合成用例（风格沿用现有生成器，均 parse-only、`Throughput::Bytes`）：`link_dense_flat`（`"[f](g) "` 重复）、`nested_brackets`（`"[![a](b)](c) "` 重复）、`many_short_paragraphs`（`"para text\n\n"` 重复）、`reference_heavy`（正文引用 `[a][rN]` + 文末定义块）、`cjk_dense`（CJK 长段落重复）。保留现有 3 个用例与命名。
- `bench/benches/alloc_count.rs` 参数化为多 fixture：`_data.md` 输出行保持现有格式不变（兼容既有记录），其后每个合成 fixture 追加一行 `fixture=<name> ...` 同字段输出。
- `bench/compare/polyglot/run.sh` 与 `README.md` 的陈旧路径（`bench/third_party/polyglot`、`bench/fixures/...`）：修复为实际路径，或在 README 顶部标注 deprecated 并移出常规流程，二选一。
- `bench/compare/native/Cargo.toml` 中未被 bench 使用的 `comrak`/`markdown_rs`/`rushdown` 依赖：移除或注释说明"v2B 比较时按独立约束重新引入"，避免误读为已有对照。

**明确不做：** 不改 `src/`；不改现有 lane 命名与 criterion 配置。

**验收：**

```bash
cargo bench --bench hotspots --no-run
cargo bench --bench alloc_count
cargo bench --bench hotspots -- --test
```

全部成功；alloc_count 对每个 fixture 输出一行；新 lane 首次结果记入 results 文件作为各自基线。

### B1: 脚注编号文档顺序修复（正确性，优先于性能阶段）

**交付：** 跨多 Block 的脚注引用编号与脚注列表恢复文档顺序；pending inline 的消费收敛到单一有序入口。

**实现范围：** 按 [bug ticket](../../.scratch/footnote-numbering-order/issues/01-footnote-numbering-follows-hashmap-order.md)：drain 后按 node id（扫描期单调分配，等价文档顺序）排序再处理，收敛为私有辅助（如 `drain_pending_in_document_order`）；F3 与 P4 之后都必须复用该入口，P4 以有序存储移除排序。新增跨 ≥3 Block 的脚注顺序回归测试。

**验收：** 新回归测试通过；`cargo test --workspace --all-features` 通过（semantic digest 差异属预期修正，更新记录）；`cargo bench --bench alloc_count` 与 phase_bench parse-only 回归 ≤2%。

### D1: 决策收敛（map tickets 06/07/05，与 B1 可并行）

**交付：** 三张票按 map 流程解决，F/P 实施获得权威依据。

- [07 Source-backed Text interface](../../.scratch/markdown-parser-incremental-iteration/issues/07-source-backed-text-interface.md)（原型）：给出 borrowed/owned 构造签名、`Document<'source>` 生命周期行为、text view 读取缝、owned-source 无自引用的移交方式（ADR 0001 契约）。
- [06 Selective parsing session interface](../../.scratch/markdown-parser-incremental-iteration/issues/06-selective-parsing-session-interface.md)（原型）：最小阶段接口。**必须与 07 的生命周期形态联合评审**：F 系列公共类型（`SelectiveParseOutput` 等）一旦按无生命周期 `Document` 定型，P1 引入 `Document<'source>` 时会二次破坏公共 API；两票结论必须相互兼容。
- [05 Choose v2A module gates and order](../../.scratch/markdown-parser-incremental-iteration/issues/05-choose-v2a-module-gates-and-order.md)（grilling）：以 issue 04 基线 + 本文新证据 + 两个原型为输入收敛 module 列表、顺序与门槛。**本文其余阶段即候选答案**；如票面结论不同，以票为准并修订本文。

**验收：** 三票 `Status: resolved` 且 map `Decisions so far` 更新；随后按 `/to-spec` → `/to-tickets` 把本计划收敛为 spec 与 tracer tickets。

### F1 → F2 → F3: 功能迭代

语义、范围、"明确不做"与验收全部沿用初版与 [selective spec](../specs/2026-07-19-selective-inline-events-design.md)（顶层 Block 终态停止 → 语义准备与目标遍历 → 选择性 Inline 物化），此处只记 r2 增补：

- **F1**：事件在稳定行边界派发（游标扫描 `Document` 直接子节点中新近 finalized 者；观察者以 `Option<&mut dyn FnMut>` 参数只存在于扫描函数内，不改 `blocks::*` 签名）。实施中确认：`finalize` 深层调用点不适合作为派发 seam（table 会把段落 finalize 后吞并、观察者存储有生命周期问题）；行边界派发的视图弱化已记入 2026-07-19 spec 的 F1 实施注。事件只读、不承诺回调后可保存的 NodeId（map 决策 02）。性能护栏复用 `phase_bench parse_ast_only`：不注册 visitor 回归 ≤2%。
- **F2**：语义准备 owner 内先做整个已接受前缀的 reference definition，再 BlockId 提取（共享扫描器）、再全部 Heading Inline 物化；目标 ID 仅 owner 生命周期内有效（map 决策 02）。
- **F3**：pending 消费**必须**走 B1 的 `drain_pending_in_document_order` 入口（文档顺序由该入口保证，选择性与完整解析共享）；结果用 `SelectiveParseOutput` 包装（map 决策 02/04）。验收含"完整选择与现有 `parse()` 语义摘要和 HTML 逐字节一致"。

### P1: 当前 Tree 中的 Source-Backed Text（最大杠杆）

**依赖：** P0.5、B1、D1（07 号票的接口结论）。可在 F1–F3 之后执行；也可依 05 号票结论提前——r2 依据（费用/收益）：约 1/3 的 parse-only 时间在内存子系统、`Document` 析构再占 8.6%，而保留分配 8,817 次的大头是 Text `String`；文本累计/落盘（`accumulate_run`+`flush_text_acc`）另占 ≈8.7%。

**范围（沿用初版 + r2 细化）：**

- `Document<'source>` 借用为默认、owned-source 显式路径（ADR 0001，签名以 07 号票为准）；Text payload 改为 source range 或 owned 转换文本，读取走 document-bound text view；`Tree`、Block 算法与非 Text payload 不动。
- 第一切片只覆盖普通连续文本；entity/escape/smart punctuation/CJK/code/跨段文本先保持 owned，逐类由 profile 决定扩展。
- **delimiter/bracket 的 marker 文本是连续源码切片，随本切片一并转为 source range**（消除 `delimiter.rs:385-397`、`bracket.rs:80-86` 的 per-marker `String`）——这是 P1 排在 P2/P3 之前的原因之一。
- 上文"结构事实"列出的四类 owned-String 改写位点逐一处理：合并改写需 copy-on-write 或延后合并；`truncate`/引号改写在 view/表示层等价实现；每处附对应语法 fixture 验证。
- owned-source 路径在解析结束、scanner 与 pending 状态释放后移交 `String`，禁止自引用；WASM 走 owned-source，输出不变。

**验收（lane 已由 P0.5 落实）：** borrowed 输入不复制完整源码（指针相等测试）；owned 输入不需外部源码存活；`plain_ascii_4k` 分配字节 ≥−50%；`_data.md` 总分配字节 ≥−20% 或 parse-only ≥+10%；HTML/Serde/WASM/位置/全语法 fixture 与 semantic digest 不变；Rust breaking change 附迁移说明。

```bash
cargo bench --bench alloc_count
cargo bench --bench hotspots -- plain_ascii_4k
cargo bench --bench phase_bench
cargo check -p markdown-binding --target wasm32-unknown-unknown
```

### P2: Delimiter workspace / P3: Bracket workspace

范围与验收沿用初版（`Vec<Delimiter>`/`Vec<Bracket>` + 索引链替换 `Rc<RefCell<_>>`，只换临时状态、继续直写当前 Tree，不引入 resolved IR）。r2 校准：

- **期望**：语料级 delimiter 链自身仅 ≈1.3%，两阶段的语料收益主要是分配次数（每 run/每括号一个 `Rc`；marker `String` 已由 P1 消除）与消除 `Rc` 克隆/动态借用检查；同时是未来"标记不进正式 AST"提交模型的前置。门槛保持热点级：P2 用 `many_flushes_dense_inline`，P3 用 `link_dense_flat`/`nested_brackets`，≥5% 时间或 ≥10% 分配次数；`_data.md` 回归 ≤2%。
- P2 结束时记录 delimiter/bracket 计数与 marker 节点数（一次性统计脚本或 debug 计数即可），作为 P7 评估"受限 Inline commit module"的证据。

### P4: 有序 Pending Inline module

范围沿用初版（私有 `PendingInlines`：文档顺序条目 + node-to-entry 稠密索引 + 单/双段 SmallVec 内联；reference/table/BlockId 调整走同一 interface；`Span` 逻辑前缀/缩进/换行/位置语义不丢）。r2 增补：

- **吸收 B1**：有序存储原生保证文档顺序后移除 B1 的临时排序；B1 的脚注顺序回归测试与 F3 的文档顺序测试继续通过。
- 门槛：`many_short_paragraphs` ≥5% 时间或 ≥10% 分配；`reference_heavy`、容器与完整语料回归 ≤2%。

### P5: Tree 空闲节点状态与链接字段压缩

范围沿用初版（替换 `frees: FxHashSet<usize>` 为更廉价的 slot/attached 表示，保持 Tree interface 与节点编号语义），r2 扩展：

- 同阶段把 `TreeNode` 的 `first_child`/`last_child`/`next`/`prev` 从 `Option<usize>`（16 B）压缩为 `Option<NonZeroUsize>`（8 B），每槽 −32 B（152→≈120 B，−21%），curated 峰值约省 400 KB 并改善局部性。前提不变量：槽 0 为根 `Document`，绝不作为任何节点的 child/sibling 出现——以 debug assert 固化。`parent` 与公开 id 保持 `usize`。
- `u32` 打包留给 v2B（需先强制节点数上限，本阶段不做）。

**验收：** Tree mutation/component normalization/list/HTML/footnote/link fixture 通过；`multiline_blockquote_dense` 或 `many_flushes_dense_inline` ≥5% 时间或 ≥10% 分配改善；完整语料 ≤2%；结果记录附 `size_of::<TreeNode<Node>>()` 前后值（可用一次性测试断言）。

### P6: Profile-gated Block dispatch（维持条件性，判据收紧）

初版触发条件"Block scan 占 parse-only ≥25%"在新采样下处于边界（不含行提取 ≈26%、含行提取 ≈32%，parse 口径），**但其中 `Span::extract`+`skip_to_eol` ≈9% 是行提取，LineHead 预筛不作用于它们**。r2 判据：

- 触发前先做一次专项归因（记录方法：`bench/benches/timing_test.rs` + `sample`/samply，方法写入 results 文件），把"matcher 快照/重试 + 容器下降"与"行提取/位置预计算"分开；仅当前者 ≥25% parse-only 时实施 LineHead。
- 范围与验收沿用初版（先独立验证 `LineHead`，再按首字节/上下文/flavor 选候选 matcher，保留优先级；heading/fence/reference/thematic break/blockquote/list/HTML/table 几何平均 ≥10%，语料 ≤2%；不达标只保留独立有收益的 `LineHead` 或回退）。

### P7: 重新决策，而非自动升级为整体重构

三选一裁决逻辑沿用初版。r2 增补必须随附的证据记录：

1. 位置模型与行提取成本（本轮 ≈10%：`Span::extract` 504、`MergedSpan` 位置族 ≈360 样本）——初版为位置模型设定的 25% 触发线在此成本形态下几乎不可能触发；P7 需按实际占比、`cjk_dense` lane 数据与跨模块改造成本重议 lazy location 是否进入 v2B，不预设结论。
2. 渲染路径每节点 `format!` 临时分配（`src/render/html.rs:33-178,617,825-863`）与 parse+HTML lane 差值——作为独立候选记录，不影响 parse-only 挑战判定。
3. WASM AST 导出深拷贝（`wasm-binding/src/lib.rs:78-108`）单独记录，不计入核心解析结论。
4. P2 记录的 delimiter/bracket/marker 计数——评估"受限 Inline commit module"（初版选项 2）时使用。

## 推荐执行顺序

```text
P0 补录 -> P0.5 lane 与 per-fixture 分配统计 -> B1 脚注顺序修复
（与 B1 并行）D1: 07 ↔ 06 联合原型 -> 05 收敛（此后 F/P 实施获得权威）
  -> F1 终态顶层 Block 事件
  -> F2 语义准备与目标遍历
  -> F3 选择性 Inline 物化
  -> P1 source-backed Text
  -> P2 delimiter workspace
  -> P3 bracket workspace
  -> P4 有序 pending Inline（移除 B1 临时排序）
  -> P5 Tree 状态 + 链接压缩
  -> P6 仅在专项归因证明 matcher 分派可省部分 ≥25% 时
  -> P7 依据证据重新决策
```

每个箭头都是停点：前一阶段没有独立可验证的结果时，不得把后续改动并入以掩盖失败；挑战目标不能成为绕过停点的理由。F1–F3 与 P1 的相对顺序属于 05 号票的裁量范围：r2 维持功能先行（在现有 Tree 上端到端交付、不被 P1 的 breaking change 阻塞），前提是 D1 已保证两组公共接口的生命周期形态相互兼容；若 05 号票决定性能先行，只需交换 F 组与 P1 的位置，其余依赖关系不变。
