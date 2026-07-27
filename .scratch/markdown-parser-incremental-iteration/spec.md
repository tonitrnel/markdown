# 渐进式解析器迭代 v2A — 收敛 Spec

**状态：** 2026-07-26 由 [决策地图](map.md) 收敛（tickets 01–07 全部 resolved）。实施顺序与门槛细节的权威执行文档是 [r2 计划](../../docs/plans/2026-07-26-markdown-parser-incremental-iteration-r2_cn.md)；行为语义来源是 [选择性解析设计](../../docs/specs/2026-07-19-selective-inline-events-design.md) 与 [ADR 0001](../../docs/adr/0001-source-backed-text-lifetime.md)。本文是两者的契约汇总，供 tracer tickets（本目录 issues/09–18）直接引用。ticket 08（v2B 入口）保持 blocked，待 P7 证据。

## 范围与不变式

- 始终只有当前 `Parser`、`Tree` 和一条正式解析路径；不创建 ParserV2/DocumentV2、并行 AST、双写 adapter。
- 每个阶段独立提交、独立可回退，结束时解析器可工作；门槛不过当场回退。
- `5 ms`（OFM 产品语料）、CommonMark/共享 GFM 相对 pulldown-cmark `2x`、分配 −50% 是**挑战目标**，不是阶段进入条件；P7 依证据裁决。
- 不做：解析器状态序列化、磁盘 checkpoint、跨进程恢复、源码变更后恢复、持久 NodeId 契约、流式 HTML、新 Block 状态机、通用 bump allocator、单文档并行。

## 已定接口契约（实施必须遵守）

### 选择性解析会话（ticket 06；F1–F3 实施）

```rust
pub enum VisitControl { Continue, Stop }        // Stop 为终态，无恢复
pub enum BlockScanStatus { Complete, Stopped }

// 阶段按值流动，编译期阻止阶段误用：
// Parser --parse_blocks_with[_checked](filter, visitor)--> BlockPhase
// BlockPhase --prepare_semantic_targets[_checked]()--> SemanticPhase
// SemanticPhase --visit_semantic_targets(filter, &mut selection, visitor)-->（原地）
// SemanticPhase --parse_selected_inlines[_checked](selection)--> SelectiveParseOutput
// F1 附加：BlockPhase::finish[_checked]() -> Document（全量物化，使 F1 独立可用）
```

- `TopLevelBlockEvent<'event>`：只读、只在回调期有效；`node_id()` 保留但无持久性契约（语义准备可能移除纯引用定义段落）。
- 只有 `Document` 直接子节点产生顶层事件；frontmatter 与 Document 自身不产生；predicate 拒绝 = Continue。
- `Stop`：停止消费源码、前缀 finalized 为结构有效树、丢弃剩余输入、不支持恢复；状态经 `BlockScanStatus` 暴露。
- 完整 `parse()` 与选择性路径共用同一物化实现（完整 = 全选）。
- checked 与非 checked（panic）变体全套提供。
- 全部语义细节（语义目标定义、Heading 预物化、选择展开、footnote 依赖、错误）沿用 2026-07-19 spec。

### Source-backed Text（ticket 07；P1 实施）

```rust
pub enum SourceText<'source> { Borrowed(&'source str), Owned(String) }
pub struct Document<'source> { /* source + tree + tags */ }
pub struct SourceSpan { start: u32, end: u32 }              // 半开、UTF-8 边界
pub enum TextRef { Source(SourceSpan), Owned(String) }
// MarkdownNode::Text(String) → MarkdownNode::Text(TextRef)
// Document::text(&TextRef) -> &str 是唯一读取缝（HTML/Serde/WASM/Rust 共用）
// owned 路径：Parser::parse_string(String, options) -> Document<'static>
//   —— 临时借用解析，scanner/pending/Span 释放后移交 String；无自引用、无 unsafe
```

- 第一切片只覆盖普通连续文本与 delimiter/bracket 标记；转换文本保持 `Owned`。
- HTML/Serde/WASM 输出逐字节不变（Serde 经 document-bound 包装）。
- Rust breaking 仅"解构 `Text(String)` → 经 `document.text`"一类，附迁移说明。

## 实施顺序与门槛（详见 r2 计划各节）

| Ticket | 阶段 | 主 lane（P0.5 已落地） | 通过门槛 |
| --- | --- | --- | --- |
| 09 | F1 顶层 Block 事件 | `phase_bench parse_ast_only` | 无 visitor 回归 ≤2%；事件/Stop 语义测试 |
| 10 | F2 语义准备与目标遍历 | 同上 | 前序遍历/BlockId/Heading 测试；完整解析共用逻辑 |
| 11 | F3 选择性 Inline 物化 | 专用 selective fixture | 全选 = `parse()` 逐字节一致；空选跳过正文 |
| 12 | P1 source-backed Text | `plain_ascii_4k`、`_data.md` | 热点分配字节 −50%；语料字节 −20% 或时间 +10% |
| 13 | P2 delimiter workspace | `many_flushes_dense_inline` | 时间 −5% 或分配 −10%；语料 ≤2% |
| 14 | P3 bracket workspace | `link_dense_flat`、`nested_brackets` | 同上 |
| 15 | P4 有序 pending | `many_short_paragraphs`、`reference_heavy` | 时间 −5% 或分配 −10%；文档顺序原生化 |
| 16 | P5 Tree 状态+链接压缩 | `multiline_blockquote_dense` 等 | 时间 −5% 或分配 −10%；TreeNode 尺寸记录 |
| 17 | P6 Block dispatch（条件） | heading/fence 等几何平均 | 先专项归因 ≥25%；实施后 ≥10% |
| 18 | P7 重新决策 | 全部 | 记录齐备后三选一，不自动扩权 |

全阶段共同护栏（r2 计划"共同护栏"节）：`cargo fmt --check`、`cargo test --workspace --all-features`、`cargo test --doc`、涉及公共类型/Serde/binding 时 `cargo check -p markdown-binding --target wasm32-unknown-unknown`；性能阶段前后各跑 `cargo bench --bench alloc_count` 并记录到 `bench/results/incremental-iteration.md`；`semantic_digest` 相对上一阶段不变（预期语义修正须注明并重录基线）。
