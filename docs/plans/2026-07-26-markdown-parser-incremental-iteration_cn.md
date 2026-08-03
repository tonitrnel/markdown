# Markdown 解析器渐进式迭代计划

**状态：** 已被 [r2 修订版](2026-07-26-markdown-parser-incremental-iteration-r2_cn.md) 取代为当前执行假设（2026-07-26：新增实测 profile 归因、脚注编号顺序 bug（B1）、测量通道补齐（P0.5）与 map ticket 06/07/05 的决策排序）。本文保留为初版记录，其中的设计来源、已定决策与可行性结论仍被 r2 引用。当前权威仍是 [Markdown Parser Incremental Iteration Map](../../.scratch/markdown-parser-incremental-iteration/map.md)；地图完成后收敛为 spec 和 blocker-ordered tracer tickets。它取代 2026-07-23 v2A 计划的实施顺序，但不删除旧设计中的问题证据和挑战指标。

## 目标与结论

计划分为两条连续的工作流：先交付选择性解析能力，再做按 module 划分的性能优化。任何时刻只有当前 `Parser`、`Tree` 和一条正式解析路径；不创建 `ParserV2`、`DocumentV2`、并行 AST 或双写 adapter。

**可行性结论：**

- 不整体重构仍可交付终态 Block 停止、语义目标遍历和选择性 Inline 物化。
- 不整体重构仍可取得实质性能收益，尤其是 Text、Inline 临时状态、pending Inline 与 Tree 空闲节点管理。
- 仅做微优化不能可靠承诺旧 v2A 目标。570,143-byte corpus 当前约为 `7.84 ms`，到 `5 ms` 仍需约 36% 的改善；`_data.md` 每次约 20,571 次分配，其中最终 `Document` 已保留 8,817 次。要达到分配降低 50%，必须几乎消除所有临时分配，而非只做局部剪枝。
- 因此 v2A 的 `5 ms`、CommonMark/共享 GFM 相对 `pulldown-cmark` `2x`、分配降低 50% 均为**挑战目标**。它们不是每个阶段的进入条件，也不预先承诺一定达到。

规划时测得的数值只是快照，正式基线必须由阶段 0 的修复后 benchmark 重新记录：

| 语料与操作 | 当前快照 |
| --- | ---: |
| 259,333-byte `_data.md`，parse only | 约 2.33 ms |
| 259,333-byte `_data.md`，parse + HTML | 约 3.28 ms |
| 570,143-byte `markdown-it-corpus`，parse only | 约 7.84 ms |
| 570,143-byte `markdown-it-corpus`，parse + HTML | 约 10.48 ms |
| `_data.md` allocations per parse | 20,571 |
| `_data.md` reallocations per parse | 1,910 |

## 设计来源与已解决的冲突

- [选择性 Inline 设计](../specs/2026-07-19-selective-inline-events-design.md) 定义功能语义：顶层 Block、语义目标、Heading 准备、BlockId、选择展开和 footnote 依赖。
- [性能架构设计](../specs/2026-07-20-markdown-parser-performance-architecture-design.md) 只保留热点、布局和目标的证据；不再绑定其 v2 管线与模块拆分。
- [旧 v2A 计划](2026-07-23-markdown-parser-v2a-performance-architecture_cn.md) 不得逐项实施。它试图一次迁移源码、AST、位置、Inline、事件和 WASM；被清理的实验也验证了这种聚合方式无法提供可信的中间结果。

以下决策已经确定：

1. 顶层 Block 回调只提供回调期间有效的只读 view，不承诺可保存的 `NodeId`。
2. 引用定义归一化之后，语义目标才可使用一次内存会话内稳定的节点 ID。
3. `Stop` 只接受当前 Block 前缀；不支持扫描恢复、序列化、磁盘 checkpoint、跨进程恢复或源码变更后的恢复。
4. 选择性结果使用 `SelectiveParseOutput` 一类的独立包装，不能被误认为完整 Inline `Document`。
5. Text 是允许改变 AST 表示的单独 module 改造。默认 Rust 路径借用源码；显式 owned-source 路径服务 WASM 或需要独立保存的调用者。详细契约见 [ADR 0001](../adr/0001-source-backed-text-lifetime.md)。

## 共同护栏

每个阶段必须是独立提交，且结束时保留可工作的解析器。除测量阶段外，每一阶段至少交付一项可观察结果：新调用能力，或在预先指定 benchmark lane 中有可重复的改善。

所有阶段：

```bash
cargo fmt --all -- --check
cargo test --workspace --all-features
cargo test --doc
```

影响公共类型、Serde 或 binding 时还运行：

```bash
cargo check -p markdown-binding --target wasm32-unknown-unknown
```

性能阶段额外要求：

- 固定机器、toolchain、profile、feature、fixture 和命令；记录 commit、`rustc -Vv`、CPU、操作系统和 fixture SHA。
- 用 Criterion 中位数与 allocation count 判断，不用单次 wall-clock 结论。
- 未指定更强门槛时，完整 `_data.md` parse-only 不得回归超过 2%。置信区间无法证明收益时，视为未通过。
- 保留门槛不通过的实现当场回退，不把“以后可能有用”的代码留在热路径。
- 语义摘要、HTML、CommonMark、GFM、OFM、CJK、位置、property 与 WASM 回归必须通过。

结果追加至 `bench/results/incremental-iteration.md`；该记录不是 changelog。

## 功能迭代

功能阶段先运行，因为它们能在现有 `Tree` 上端到端交付，不依赖 compact AST。它们均为 Rust-only；现有 `parse()` 和 WASM 全解析行为不变。

### F1: 顶层 Block 事件与终态停止

**交付：** Rust 调用者能够观察已 finalized 的顶层 Block，并在稳定边界停止读取后续源码。

**实现范围：** 在现有 `Parser::finalize` 与 Block 扫描之间建立私有 seam。filter 决定是否派发；visitor 只返回 `Continue` 或 `Stop`。`Stop` 产出已接受 Block 前缀的阶段结果和 `Complete`/`Stopped` 状态，然后进入后续语义准备；它不保存 visitor、不复制 Tree，也不支持恢复扫描。

**明确不做：** 不暴露持久 `NodeId`，不把 Document/frontmatter 当作事件，不提前替换 pending Inline 存储，不增加 WASM callback。

**验收：**

- 只派发 `Document` 的直接子节点，最后一个 EOF Block 只派发一次。
- predicate 拒绝事件不改变解析；嵌套 list、blockquote、callout 不产生顶层事件。
- 在每个可停止边界得到的前缀 Block 结构有效，未接受的后续节点不存在。
- 现有完整解析 HTML 与测试结果不变；不注册 visitor 的完整解析性能不回归超过 2%。

### F2: 语义准备与目标遍历

**依赖：** F1。

**交付：** 调用者可在已接受前缀上遍历 Heading 与 OFM BlockId 的语义目标，在选择前检查完整 Heading Inline AST 和 BlockId。

**实现范围：** 引入仅在进程内存中存在的阶段 owner（名称可在原型中确定）。它先处理整个已接受前缀的 reference definition，再从 pending Inline 中提取 BlockId，物化全部 Heading Inline，最后按文档前序建立目标遍历。目标 ID 从这个阶段起仅在该 owner 生命周期内有效。

**明确不做：** 不把 Heading 扩展为 section；不把每个 Block 都变成可寻址目标；不引入 checkpoint 或第二套 AST。

**验收：**

- ATX、Setext、嵌套 Heading 和嵌套 BlockId 均按前序访问。
- Heading + BlockId 只产生一个目标；独立 BlockId 行和重复 ID 保持当前 OFM 语义。
- reference link、强调、code 等 Heading 内容在 visitor 前已完整解析。
- 完整解析仍使用相同的 reference、BlockId 与 Heading 准备逻辑。

### F3: 选择性 Inline 物化

**依赖：** F2。

**交付：** 调用者可选择一个或多个语义目标，只物化所选节点、其 Inline-capable 后代和必要 footnote definition 的 Inline AST；空选择跳过普通正文 Inline。

**实现范围：** `InlineSelection` 只存会话内节点 ID；物化前按文档顺序展开，并消除祖先/后代重复选择。结果通过 `SelectiveParseOutput` 返回完整 Block Tree、选择性 Inline `Document` 和 Block scan 状态。现有 `parse()` 走同一个 Inline materializer 的“全部选择”路径。

**明确不做：** 不在普通 `Document` 中声称每个节点都已 Inline 完整；不提供选择子树的排除规则；不支持之后追加一轮物化或恢复扫描。

**验收：**

- 覆盖空、叶子、容器、祖先+后代、语义 visitor Stop、footnote 依赖和无效 ID。
- 完整选择与现有 `parse()` 的语义摘要和 HTML 逐字节一致。
- 未选择节点不产生普通 Inline AST；选择约 10% 正文的专用产品 fixture 必须比完整解析更快，结果记录但不预先承诺固定比例。

### F4: 有意延后恢复

**依赖：** 无，属于明确的非工作项。

终态停止已经满足“只解析文档前缀”和选择性解析的前提。只有出现具体上层需求、且 F1--F3 的状态 owner 已通过长期使用后，才可以提出新的恢复设计；它必须单独说明内存所有权、事件游标和测试矩阵，不能从本计划自动继承。

## 性能迭代

性能阶段每次只加深一个 module。任何阶段都不得借机迁移无关 AST payload、位置模型、Block 状态机或 WASM 表示。

### P0: 修复测量和语义护栏

**交付：** 可复现的 benchmark 与等价性判断，作为后续每个性能结论的依据。

**范围：**

- 修复 `alloc_count` fixture 路径，预热后按每次解析报告 alloc/realloc/dealloc、字节数和真实中位数；`REALLOC_BYTES` 累加 `new_size`。
- `phase_bench` 明确 parse-only、render-only、parse+render，并使用 `black_box`。
- 比较器拆为 CommonMark、共享 GFM 与 OFM 产品 lanes；只把前两者称为跨实现等价比较。
- 增加稳定语义摘要，覆盖树形、位置、payload、BlockId、排序 tags 和 HTML。

**验收：** 所有 benchmark 命令成功运行；结果写入记录；本阶段不改 `src/` 热路径。

### P1: 当前 Tree 中的 Source-Backed Text

**依赖：** P0。此阶段可在 F1--F3 之后执行；阶段 owner 只需机械地适配 `Document<'source>`。

**交付：** 未转换的连续文本不再逐节点复制 `String`，Rust 默认路径不复制整份源码。

**实现范围：**

- `Document<'source>` 借用或拥有 Source document；borrowed 与 owned-source 构造语义遵循 ADR 0001。
- Text payload 仅改为 source range 或 owned transformed text；当前 `Tree`、Block 算法和非 Text payload 保持原状。
- 用 document-bound text view 统一 HTML、Serde、WASM 和 Rust 读取；WASM 输出仍是已解析的字符串。
- 第一切片只覆盖普通连续 Text。entity、escape、smart punctuation、CJK、code、HTML 与跨段文本先保持 owned，逐类由 profile 决定是否扩展。
- owned-source 适配必须在解析结束、scanner 和 pending state 释放后转移 String，禁止自引用结构和不安全生命周期延长。

**验收：**

- borrowed 输入不复制完整 source；owned 输入不再需要外部 source 存活。
- plain-text hotspot 分配字节降低至少 50%，且 `_data.md` 总分配字节降低至少 20% 或 parse-only 提升至少 10%。
- HTML、Serde、WASM、位置和全部语法 fixture 一致；Rust breaking change 有迁移说明。

### P2: Delimiter workspace

**依赖：** P0；可与 P1 独立排序，但不与 bracket 改造混合。

**交付：** emphasis、strong、strikethrough、highlight 和 smart punctuation 的临时 delimiter 不再使用 `Rc<RefCell<_>>`。

**实现范围：** 在 `inlines::process` 后建立私有 `InlineWorkspace`，以 `Vec<Delimiter>` 和索引链表达 delimiter。它只替换内部临时状态，继续直接写当前 Tree，不引入 resolved IR 或第二种 AST 写入。

**验收：** delimiter-dense hotspot 时间至少改善 5% 或分配次数至少降低 10%；`_data.md` 不回归超过 2%；CJK delimiter、未匹配 delimiter 与全部现有 fixture 通过。

### P3: Bracket workspace

**依赖：** P2。

**交付：** link/image 的临时 bracket 链改为 `Vec<Bracket>` 和索引，不再使用 `Rc<RefCell<_>>`。

**实现范围：** 复用 P2 的 workspace capacity；保持 link、image、reference、footnote 和现有 Tree 重连语义。不得同时引入链接 IR、延迟提交或 payload 重排。

**验收：** link-dense 或 nested-bracket hotspot 至少改善 5% 或分配次数至少降低 10%，另一个目标 hotspot 不回归超过 2%，并通过嵌套、失配、image size、reference、footnote 与多行测试。

### P4: 有序 Pending Inline module

**依赖：** P0；F3 可以先使用当前存储，随后接入此 module。

**交付：** Block 注册和 Inline 消费保留文档顺序，不再为常见单段/双段容器分配独立 `Vec`，同时为选择性物化提供确定性顺序。

**实现范围：** 以私有 `PendingInlines` 隐藏 `Vec` 条目、node-to-entry 稠密索引和小容量逻辑片段。reference definition、table 与 BlockId 调整必须通过同一 interface 修改 pending 内容；`Span` 的逻辑前缀、缩进、换行和位置语义不得因压缩而丢失。

**验收：** many-short-paragraphs 的时间至少改善 5% 或分配次数至少降低 10%；document-order 有显式测试；reference-heavy、容器和完整语料不回归超过 2%。

### P5: Tree 空闲节点状态

**依赖：** P0；可独立于 P2--P4。

**交付：** 当前 Tree 的临时节点、unlink 与重连不再依赖 `FxHashSet` 追踪 free node。

**实现范围：** 只替换 `Tree` 内部的 slot/attached 状态表示，保持现有 Tree interface 与节点编号语义。Block 侧的必要关系修改仍被允许；此阶段不尝试 append-only Block AST。

**验收：** Tree mutation、component normalization、list、HTML、footnote 和 link fixture 通过；至少一个 Tree/Inline 密集 hotspot 获得 5% 时间改善或 10% 分配改善；完整语料不回归超过 2%。

### P6: Profile-gated Block dispatch

**依赖：** P0，且 profile 证明 Block scan 占 parse-only 时间至少 25%。

**交付：** 若证据成立，共享的行首信息减少无关 Block matcher 调用。

**实现范围：** 先实现可单独验证的 `LineHead`，再以首字节、上下文和启用 flavor 选择候选 matcher；保留现有优先级。不会创建新的 Block 状态机。

**验收：** heading、fence、reference、thematic break、blockquote、list、HTML、table 的几何平均改善至少 10%，任一完整语料回归不超过 2%。未达到时只保留独立有收益的 `LineHead`，否则回退。

### P7: 重新决策，而非自动升级为整体重构

**依赖：** P1、P2、P3、P4、P5，以及在有证据时的 P6。

重新记录阶段耗时、allocation/reallocation、drop、文本命中率、CommonMark/共享 GFM/OFM lanes 和选择性路径。然后只作以下一种决定：

1. 挑战目标已达到：停止架构迁移，只继续 profile 指向的微优化。
2. Inline 仍为最大热点：为一个明确语法族或一个受限的 Inline commit module 新写计划；不得直接复活 v2 pipeline。
3. Tree/非 Text payload/位置模型占 parse-only 时间至少 25%，或占分配字节至少 30%：为那个单一 module 提交独立设计评审。

不得因为挑战目标未达成而同时迁移 compact AST、payload arena、lazy location、恢复会话和 WASM。

## 推荐执行顺序

```text
P0 measurement and semantic guardrails
  -> F1 terminal top-level Block events
  -> F2 semantic preparation and targets
  -> F3 selective Inline materialization
  -> P1 source-backed Text
  -> P2 delimiter workspace
  -> P3 bracket workspace
  -> P4 ordered pending Inline
  -> P5 Tree free-node state
  -> P6 only when profile justifies it
  -> P7 evidence-based next decision
```

每个箭头都是停点。前一阶段没有独立可验证的结果时，不得把后续改动并入以掩盖失败；旧 v2A 挑战目标也不能成为绕过这些停点的理由。
