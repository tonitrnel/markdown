//! 选择性解析会话的公共类型与入口（F1：顶层 Block 事件与终态停止）。
//!
//! 接口契约见 `.scratch/markdown-parser-incremental-iteration/spec.md` 与
//! map ticket 06；行为语义来源是 `docs/specs/2026-07-19-selective-inline-events-design.md`。

use crate::ast::MarkdownNode;
use crate::ast::heading::Heading;
use crate::document::Document;
use crate::node::Node;
use crate::parser::{ParseError, Parser};
use crate::tree::Tree;
use rustc_hash::FxHashSet;

/// 事件 visitor 的控制流返回值。`Stop` 是终态：不支持恢复扫描。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitControl {
    Continue,
    Stop,
}

/// Block 扫描的结束方式：读到 EOF（`Complete`）或被 visitor 终态停止（`Stopped`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockScanStatus {
    Complete,
    Stopped,
}

/// 已 finalized 的顶层 Block 事件。只读，仅在 visitor 回调期间有效。
///
/// `node_id()` 不承诺回调之后仍然有效：语义准备可能移除该节点
/// （例如仅含引用定义的段落）。事件在稳定的行边界派发，因此当同一行
/// 既关闭上一个顶层 Block 又开启下一个时，`tree()` 视图可能已包含下一个
/// 顶层节点的起始行；`Stop` 保证返回的前缀不含任何未接受的节点。
pub struct TopLevelBlockEvent<'event> {
    pub(crate) node_id: usize,
    pub(crate) tree: &'event Tree<Node>,
}

impl<'event> TopLevelBlockEvent<'event> {
    /// 事件节点在树中的 id。仅回调期有效，无持久性契约。
    pub fn node_id(&self) -> usize {
        self.node_id
    }
    pub fn node(&self) -> &'event Node {
        &self.tree[self.node_id]
    }
    pub fn tree(&self) -> &'event Tree<Node> {
        self.tree
    }
}

/// 已完成 Block 扫描的 Source document。
///
/// Block 结构可以直接检查，Pending Inline 尚未物化。该值拥有继续解析所需的
/// 全部状态；`Stop` 之后只能物化已接受的前缀或丢弃，不能恢复扫描。
pub struct BlockDocument<'input> {
    pub(crate) parser: Parser<'input>,
    pub(crate) status: BlockScanStatus,
}

impl<'input> BlockDocument<'input> {
    /// 原始 Source document。
    pub fn source(&self) -> &'input str {
        self.parser.scanner.source_str()
    }

    /// 已完成的 Block tree。Inline-capable 节点可能仍没有 Inline 子节点。
    pub fn tree(&self) -> &Tree<Node> {
        &self.parser.tree
    }

    pub fn block_status(&self) -> BlockScanStatus {
        self.status
    }

    /// 物化全部 Pending Inline，返回完整 [`Document`]。
    pub fn materialize_all(self) -> Result<Document<'input>, ParseError> {
        self.parser.finish_inline_phase_checked()
    }

    /// 发现 BlockId 并建立文档前序的 Semantic target 索引。
    pub fn prepare_semantics(mut self) -> Result<SemanticPhase<'input>, ParseError> {
        self.parser.discover_block_ids();
        if let Some(err) = self.parser.parse_error.take() {
            self.parser.tree.pop();
            return Err(err);
        }
        let targets = self.parser.collect_semantic_targets();
        Ok(SemanticPhase {
            parser: self.parser,
            status: self.status,
            targets,
        })
    }

    /// 对已接受的 Block 前缀执行完整 Inline 物化并返回 `Document`。
    ///
    /// 与 `Parser::parse` 的后半段共用同一实现；对 `Stopped` 前缀的结果
    /// 等价于直接解析对应的源码前缀。
    pub fn finish(self) -> Document<'input> {
        self.finish_checked()
            .expect("parse failed: input exceeds parser limits")
    }
    pub fn finish_checked(self) -> Result<Document<'input>, ParseError> {
        self.materialize_all()
    }
    /// 语义准备（F2/C3）：对已接受前缀执行 BlockId 发现（引用定义提取惰性至首次物化），
    /// 并建立文档前序的语义目标索引。Heading Inline **不在此物化**（C3 惰性化）：
    /// 目标文本经 `SemanticTarget::ref_text` 按需物化，`finish` 仍按文档序补齐。
    pub fn prepare_semantic_targets(self) -> SemanticPhase<'input> {
        self.prepare_semantic_targets_checked()
            .expect("parse failed: input exceeds parser limits")
    }
    pub fn prepare_semantic_targets_checked(self) -> Result<SemanticPhase<'input>, ParseError> {
        self.prepare_semantics()
    }
}

/// 语义目标事件：Heading，或带 OFM BlockId 的节点（两者兼有时只产生一个目标）。
/// 仅在 visitor 回调期间有效；目标节点 ID 在本阶段 owner 的生命周期内稳定。
/// 结构读取（层级/BlockId）零成本；`ref_text` 首次调用时惰性物化该目标的
/// Inline 子树（C3）。
pub struct SemanticTarget<'event, 'input> {
    pub(crate) node_id: usize,
    pub(crate) parser: &'event mut Parser<'input>,
}

impl<'event, 'input> SemanticTarget<'event, 'input> {
    pub fn node_id(&self) -> usize {
        self.node_id
    }
    pub fn node(&self) -> &Node {
        &self.parser.tree[self.node_id]
    }
    pub fn tree(&self) -> &Tree<Node> {
        &self.parser.tree
    }
    /// 目标为 Heading 时返回其载荷（块级结构，无需 Inline 物化）。
    pub fn heading(&self) -> Option<&Heading> {
        match &self.parser.tree[self.node_id].body {
            MarkdownNode::Heading(heading) => Some(heading),
            _ => None,
        }
    }
    /// 目标的 OFM BlockId（若有）。
    pub fn block_id(&self) -> Option<&str> {
        self.parser.tree[self.node_id]
            .id
            .as_ref()
            .map(|id| id.as_str())
    }
    /// 目标的引用文本（Obsidian 式寻址匹配用）：Inline 子树的纯文本投影，
    /// 格式标记剥除、escape/entity 解码、末尾 BlockId 不含在内——由真实
    /// Inline 引擎保证与完整解析逐字节一致。首次调用惰性物化该子树（幂等，
    /// 已物化目标直接投影）；`finish` 对未触碰目标仍按文档序补齐，输出不变。
    pub fn ref_text(&mut self) -> String {
        if self.parser.inlines.contains(self.node_id) {
            // 任何物化前确保引用定义已提取（幂等；纯结构查询会话零支付）
            self.parser.prepare_reference_definitions();
            let mut ids = vec![self.node_id];
            self.parser.materialize_pending_subset(&mut ids);
        }
        let mut out = String::new();
        let source = self.parser.scanner.source_str();
        let mut stack: Vec<usize> = Vec::new();
        let mut node = self.parser.tree.get_first_child(self.node_id);
        while let Some(id) = node {
            if let MarkdownNode::Text(text) = &self.parser.tree[id].body {
                out.push_str(text.resolve(source));
            }
            if let Some(child) = self.parser.tree.get_first_child(id) {
                stack.push(id);
                node = Some(child);
            } else {
                node = self.parser.tree.get_next(id);
                while node.is_none() {
                    match stack.pop() {
                        Some(parent) => node = self.parser.tree.get_next(parent),
                        None => break,
                    }
                }
            }
        }
        out
    }
}

/// 选择集：按 NodeId 去重。选择容器即选择其全部可接收 Inline 的后代
/// （展开发生在物化阶段，F3）。
#[derive(Default)]
pub struct InlineSelection {
    pub(crate) selected: FxHashSet<usize>,
}

impl InlineSelection {
    pub fn select(&mut self, node_id: usize) {
        self.selected.insert(node_id);
    }
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
}

/// 语义准备完成后的阶段状态：目标索引已建立；Heading Inline 保持 pending，
/// 经 `SemanticTarget::ref_text` 或 `parse_selected_inlines`/`finish` 按需物化。
pub struct SemanticPhase<'input> {
    pub(crate) parser: Parser<'input>,
    pub(crate) status: BlockScanStatus,
    pub(crate) targets: Vec<usize>,
}

impl<'input> SemanticPhase<'input> {
    pub fn block_status(&self) -> BlockScanStatus {
        self.status
    }
    /// 语义目标数量（文档前序）。
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }
    /// 按文档前序派发语义目标。`filter` 拒绝等同于 `Continue`；
    /// `Stop` 只停止本次遍历，不截断树、不隐式修改 `selection`。
    pub fn visit_semantic_targets<F, V>(
        &mut self,
        mut filter: F,
        selection: &mut InlineSelection,
        mut visitor: V,
    ) where
        F: FnMut(&SemanticTarget<'_, 'input>) -> bool,
        V: FnMut(&mut SemanticTarget<'_, 'input>, &mut InlineSelection) -> VisitControl,
    {
        let targets = std::mem::take(&mut self.targets);
        for &node_id in &targets {
            let mut target = SemanticTarget {
                node_id,
                parser: &mut self.parser,
            };
            if !filter(&target) {
                continue;
            }
            match visitor(&mut target, selection) {
                VisitControl::Continue => continue,
                VisitControl::Stop => break,
            }
        }
        self.targets = targets;
    }
    /// 物化剩余全部 pending Inline 并返回完整 `Document`
    /// （与 `Parser::parse` 尾段共用实现）。
    pub fn finish(self) -> Document<'input> {
        self.finish_checked()
            .expect("parse failed: input exceeds parser limits")
    }
    pub fn finish_checked(self) -> Result<Document<'input>, ParseError> {
        self.parser.finish_inline_phase_checked()
    }
    /// 选择性物化（F3）：只物化所选节点及其全部可接收 Inline 的后代，
    /// 以及被引用 footnote definition 的必要内容；空选择跳过普通正文 Inline。
    /// 未选择的节点保留 Block 结构，不生成 Inline 子节点。
    pub fn parse_selected_inlines(
        self,
        selection: InlineSelection,
    ) -> SelectiveParseOutput<'input> {
        self.parse_selected_inlines_checked(selection)
            .expect("parse failed: input exceeds parser limits or invalid selection")
    }
    pub fn parse_selected_inlines_checked(
        mut self,
        selection: InlineSelection,
    ) -> Result<SelectiveParseOutput<'input>, ParseError> {
        // 惰性引用定义（C3）：任何物化开始前确保已提取（幂等）
        self.parser.prepare_reference_definitions();
        // 无效/未知节点 ID 在任何物化开始前拒绝
        for &node_id in &selection.selected {
            if !self.parser.tree.node_exists(node_id) {
                self.parser.tree.pop();
                return Err(ParseError::InvalidSelectionNode { node_id });
            }
        }
        // 展开：所选节点 + 子树内全部 pending 后代；祖先/后代重复选择天然去重
        let mut expanded: Vec<usize> = Vec::new();
        for &node_id in &selection.selected {
            self.parser
                .collect_pending_in_subtree(node_id, &mut expanded);
        }
        expanded.sort_unstable();
        expanded.dedup();
        self.parser.materialize_pending_subset(&mut expanded);
        // 选中内容（含语义准备阶段的 Heading）引用的 footnote definition 是必要依赖
        self.parser.materialize_footnote_dependencies();
        if let Some(err) = self.parser.parse_error.take() {
            self.parser.tree.pop();
            return Err(err);
        }
        self.parser.parse_footnote_list();
        if let Some(err) = self.parser.parse_error.take() {
            self.parser.tree.pop();
            return Err(err);
        }
        self.parser.tree.pop();
        let document = self.parser.into_ast();
        Ok(SelectiveParseOutput {
            document,
            block_status: self.status,
        })
    }
}

/// 选择性解析的显式部分结果：完整 Block 结构 + 仅所选内容的 Inline AST。
/// 不可与完整解析的 `Document` 混同（未选择节点没有 Inline 子节点）。
pub struct SelectiveParseOutput<'source> {
    pub document: Document<'source>,
    pub block_status: BlockScanStatus,
}

impl<'input> Parser<'input> {
    /// Parse the complete Block structure without materializing Inline nodes.
    pub fn parse_blocks(self) -> Result<BlockDocument<'input>, ParseError> {
        self.run_block_phase_checked(None)
    }

    /// Block 扫描阶段：对每个已 finalized 的 `Document` 直接子节点派发事件。
    ///
    /// `filter` 拒绝的事件等同于 `Continue`，不会停止遍历；`visitor` 返回
    /// [`VisitControl::Stop`] 时终态停止：停止消费源码、丢弃剩余输入，
    /// 返回的前缀树中不含任何未接受的节点。frontmatter 与 `Document`
    /// 自身不产生事件。
    pub fn parse_blocks_with<F, V>(self, filter: F, visitor: V) -> BlockDocument<'input>
    where
        F: FnMut(&TopLevelBlockEvent<'_>) -> bool,
        V: FnMut(&TopLevelBlockEvent<'_>) -> VisitControl,
    {
        self.parse_blocks_with_checked(filter, visitor)
            .expect("parse failed: input exceeds parser limits")
    }
    pub fn parse_blocks_with_checked<F, V>(
        self,
        mut filter: F,
        mut visitor: V,
    ) -> Result<BlockDocument<'input>, ParseError>
    where
        F: FnMut(&TopLevelBlockEvent<'_>) -> bool,
        V: FnMut(&TopLevelBlockEvent<'_>) -> VisitControl,
    {
        let mut combined = |event: &TopLevelBlockEvent<'_>| -> VisitControl {
            if filter(event) {
                visitor(event)
            } else {
                VisitControl::Continue
            }
        };
        self.run_block_phase_checked(Some(&mut combined))
    }
}
