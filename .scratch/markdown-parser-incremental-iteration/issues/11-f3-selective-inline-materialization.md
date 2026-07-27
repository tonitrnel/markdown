# F3: 选择性 Inline 物化

**Type:** task
**Status:** resolved
**Blocked by:** 10

## 交付

`InlineSelection`（私有 NodeId 去重）+ `SemanticPhase::parse_selected_inlines[_checked](selection) -> SelectiveParseOutput`：只物化所选节点、其 Inline-capable 后代与必要 footnote definition；空选择跳过普通正文 Inline。`SelectiveParseOutput { document, block_status }` 显式表达部分性。

## 实现范围

- 物化前按文档顺序展开选择并消除祖先/后代重复；无效 NodeId 报 `ParseError::InvalidSelectionNode`。
- pending 消费**必须**经 B1 的 `drain_pending_in_document_order` 入口（文档顺序保证；P4 之后只换内部存储）。
- 现有 `parse()` 重构为同一物化器的"全选"组合，无第二套 Inline 引擎。
- footnote 依赖：选中内容引用的 definition 自动物化并进入 FootnoteList；未引用未选择的不物化。

## 明确不做

不承诺普通 `Document` 每节点 Inline 完整；无子树排除；无二轮物化或恢复。

## 验收

- 覆盖：空/叶子/容器/祖先+后代/语义 Stop/footnote 依赖/无效 ID。
- **全选路径与现有 `parse()` 的 `semantic_digest` 与 HTML 逐字节一致**（关键回归闸）。
- 未选择节点不产生普通 Inline AST；新增"选择约 10% 正文"的专项 fixture，记录相对完整解析的耗时（只记录，不预承诺比例）。
- 护栏命令同 spec；`_data.md` parse-only 回归 ≤2%。

## Answer

2026-07-26 完成。实现：`SemanticPhase::parse_selected_inlines[_checked](InlineSelection) -> SelectiveParseOutput { document, block_status }`；共享物化器抽取为 `Parser::materialize_pending_entry`（完整 drain 循环与 F2 Heading 物化、F3 选择性子集全部走同一实现——"完整解析 = 全选路径"由此成立）；`collect_pending_in_subtree` 展开选择（祖先/后代经 sort+dedup 天然去重）；`materialize_footnote_dependencies` 工作队列递归物化被引用 definition（含嵌套引用）；无效 NodeId 在任何物化前返回新增的 `ParseError::InvalidSelectionNode`（WASM 错误映射同步补全）。

**验收结果**：`tests/selective_inlines.rs` 9 项全过——全选（选 Document 根）与 `parse()` 在合成源与 curated 语料摘要+HTML 逐字节一致；空选择跳过正文但保留 Heading 物化与无脚注列表；叶子/容器/祖先+后代去重；footnote 依赖 x→y 递归物化、未引用 z 不物化不进列表；无效 ID 拒绝；Stopped 前缀全选与直接解析前缀一致且状态贯穿。workspace 667 项测试 0 失败；WASM check 通过；`_data.md` 分配与耗时持平（完整解析路径仅做了物化器抽取，零成本）。

**选择性性能记录**（`cargo bench --bench hotspots -- selective_parse`，`many_short_paragraphs` 34.8 KB / 1024 段，选择每第 10 个顶层 Block）：full ≈ 200.65 µs，select_10pct ≈ 157.05 µs（约 −22%）。比例温和的原因：该路径包含完整 Block 扫描 + 语义准备，正文 Inline 只占该 fixture 的一部分；只记录，不承诺固定比例（记入 `bench/results/incremental-iteration.md`）。

**语义注**：全选=选 Document 根时，未被引用的 footnote definition 正文不物化（完整解析会物化但其节点悬空不可达）——两者 digest/HTML 相同，差异仅在不可达节点，符合"未引用定义不进列表"语义。
