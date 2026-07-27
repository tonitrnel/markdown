# Selective parsing session interface

**Type:** prototype
**Status:** resolved
**Blocked by:** None - can start immediately.

## Question

What is the smallest Rust-only phase interface that delivers finalized top-level Block events, semantic targets, and selected Inline materialization while preserving terminal Stop and avoiding durable identity or recovery semantics?

## Resolution criteria

- The prototype shows callback borrow lifetimes, phase ownership, checked-error propagation, and the explicit partial-result wrapper.
- A caller cannot retain a top-level event identity beyond its callback or resume a stopped Block scan.
- Full parsing is expressed as the same materialization path with complete selection, not a duplicate parser.
- The API can be reviewed without first rewriting Tree or pending Inline storage.

## Comments

- This is a design prototype, not production parser code.
- Its answer must provide the interface input to [Choose v2A module gates and order](05-choose-v2a-module-gates-and-order.md).

## Proposal（2026-07-26 草案，待人工评审后转为 Answer；与 07 号票联合评审）

### 类型状态分阶段——phase ownership 用移动语义表达

```rust
pub enum VisitControl { Continue, Stop }
pub enum BlockScanStatus { Complete, Stopped }

impl<'source> Parser<'source> {
    // 阶段 1：Block 扫描 + 顶层事件（filter/visitor 只在本调用内被借用）
    pub fn parse_blocks_with_checked<F, V>(
        self,
        filter: F,
        visitor: V,
    ) -> Result<BlockPhase<'source>, ParseError>
    where
        F: FnMut(&TopLevelBlockEvent<'_>) -> bool,
        V: FnMut(&TopLevelBlockEvent<'_>) -> VisitControl;
}

pub struct TopLevelBlockEvent<'event> { /* 私有：node id + &Tree */ }
impl TopLevelBlockEvent<'_> {
    pub fn node(&self) -> &Node;
    pub fn tree(&self) -> &Tree<Node>;
    // node_id() 的去留见决策点 D
}

pub struct BlockPhase<'source> { /* parser 全部状态（含 pending inline）按值持有 */ }
impl<'source> BlockPhase<'source> {
    pub fn block_status(&self) -> BlockScanStatus;
    /// 引用定义（全前缀）→ BlockId 提取 → 全部 Heading Inline 物化 → 目标前序索引
    pub fn prepare_semantic_targets_checked(self) -> Result<SemanticPhase<'source>, ParseError>;
}

pub struct SemanticPhase<'source> { /* + 语义目标前序索引 */ }
impl<'source> SemanticPhase<'source> {
    pub fn visit_semantic_targets<F, V>(
        &mut self,
        filter: F,
        selection: &mut InlineSelection,
        visitor: V,
    ) where
        F: FnMut(&SemanticTarget<'_>) -> bool,
        V: FnMut(&SemanticTarget<'_>, &mut InlineSelection) -> VisitControl;

    pub fn parse_selected_inlines_checked(
        self,
        selection: InlineSelection,
    ) -> Result<SelectiveParseOutput<'source>, ParseError>;
}

pub struct InlineSelection { /* 私有：按 NodeId 去重的集合 */ }
impl InlineSelection {
    pub fn select(&mut self, node_id: usize);
    pub fn is_empty(&self) -> bool;
}

pub struct SelectiveParseOutput<'source> {
    pub document: Document<'source>,   // 与 07 号票的 Document<'source> 同一类型
    pub block_status: BlockScanStatus,
}
```

### 关键性质（对应票面 resolution criteria）

- **回调借用**：`TopLevelBlockEvent<'event>`/`SemanticTarget<'event>` 只在 visitor 调用期间有效，`'event` 借 parser 内部、无法逃逸；`BlockPhase` 上不存在任何恢复扫描的方法——Stop 即终态（map 决策 02）。
- **phase ownership**：状态按值流动（`Parser → BlockPhase → SemanticPhase → SelectiveParseOutput`），类型系统静态禁止"语义准备前检查 Heading AST"与"物化后修改 selection"；中途 drop 即放弃，无恢复。
- **checked 传播**：每阶段 `_checked` 返回 `Result<_, ParseError>`（含 `InvalidSelectionNode`）；部分结果只经 `SelectiveParseOutput` 显式暴露，不伪装为完整解析。
- **完整解析同路径**：`parse()` 内部即 blocks（无 visitor 派发）→ prepare → 全选 → materialize 的组合，无第二套 Inline 引擎；F3 验收其语义摘要与 HTML 逐字节一致。
- **不依赖存储重写**：可全部实现在当前 Tree + pending map 上（B1 的 `drain_pending_in_document_order` 保证文档顺序）；P4 之后只替换内部存储，不动本接口。

### 与 2026-07-19 spec 的差异

- 单一 `SelectiveParser` 可变中间态 → 三个 move 型 phase 类型（编译期防阶段误用）。
- `Document` → `Document<'source>`（与 07 号票一致）。
- 其余语义（事件时机、目标定义、选择展开、footnote 依赖、Stop 终态、频率与测试矩阵）不变。

### 决策点（需人工确认）

- **D. `TopLevelBlockEvent::node_id()` 的去留**：spec 原文提供、map 决策 02 要求"无持久身份"。草案建议**保留**，文档写明"仅回调期有效；语义准备可能移除该节点（如纯引用定义段落）"——彻底不暴露会让调用者无法把顶层观察与后续语义阶段关联。
- **E. 非 checked（panic）变体**：全套提供（与现有 `parse()`/`parse_checked()` 对称，草案推荐）vs 仅提供 checked。
- **F. 阶段形态**：三段式类型状态（草案推荐）vs spec 原来的单一 `SelectiveParser` 可变状态机。

## Answer

2026-07-26 解决。维护者对 D/E/F 无倾向、委托按草案推荐执行，故上文 Proposal 即接口结论，三个决策点取推荐项：

- **D**：保留 `TopLevelBlockEvent::node_id()`，文档契约为"仅回调期有效；语义准备可能移除该节点（如纯引用定义段落）"，与 map 决策 02（无持久身份）一致。
- **E**：checked 与非 checked（panic）变体全套提供，与现有 `parse()`/`parse_checked()` 对称。
- **F**：采用三段式类型状态 `Parser → BlockPhase → SemanticPhase → SelectiveParseOutput`，phase ownership 以移动语义表达，编译期阻止阶段误用；`Stop` 终态、无恢复方法。

性质回执：事件借用无法逃逸回调；完整解析与选择性解析共用同一物化路径（`parse()` = 全选组合）；接口可全部实现在当前 Tree + pending map 上（文档顺序由 B1 入口保证），P4 只替换内部存储。F1 先落地 `Parser → BlockPhase`（含 `finish[_checked]() -> Document` 使其独立可用），F2 加 `SemanticPhase`，F3 加选择与 `SelectiveParseOutput`。实施归属 tracer tickets 09–11。
