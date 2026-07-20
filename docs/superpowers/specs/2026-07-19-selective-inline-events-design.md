# 选择性 Inline 解析与事件设计

## 概述

新增一条仅供 Rust 调用者使用的解析路径，包含两个相互独立、支持过滤的事件阶段：

1. Block 解析过程中，访问已经完成的顶层 CommonMark Block。
2. Block 解析结束后，访问由 Heading 或 OFM BlockId 标识的可寻址 Markdown 节点。

两个事件阶段都支持 `Continue` 和 `Stop`。Inline 选择与事件控制流相互独立。语义目标遍历结束后，解析器只处理已选择节点及其全部后代的普通 Inline。为了让调用者在选择时检查完整标题，所有 Heading 的 Inline AST 都会在语义目标遍历前完成。

现有 Rust `parse()` 和 `parse_checked()` API 保持完整文档解析语义。WASM API 不变。提前停止是终态；本设计不提供 checkpoint 或恢复能力。

## 目标

- 允许 Rust 调用者在 Block 解析期间观察指定种类的已完成顶层 Block。
- 允许调用者在稳定的顶层 Block 边界停止 Block 解析。
- 在语义选择前提供所有 Heading 的完整 Inline AST。
- 在普通正文 Inline 解析前提供 OFM BlockId。
- 在树的任意深度发现 Heading 和 BlockId 目标。
- 将目标选择与 `Continue`/`Stop` 遍历控制分离。
- 解析选中节点及其全部后代的 Inline，不允许排除后代。
- 选择集为空时跳过普通正文 Inline 解析。

## 非目标

- WASM 或 JavaScript 回调。
- Parser checkpoint 序列化，或在 `Stop` 后恢复解析。
- 把每个 CommonMark Block-level 节点都视为可寻址 Markdown 目标。
- 把 Heading 隐式扩展为延续到下一个 Heading 的 section。
- 只选择节点子树的一部分。
- 返回 `Document` 后继续复用的延迟解析 session。

## 术语

本设计明确区分两种“Block”：

- **顶层 Block**：CommonMark Block 解析器生成的 `Document` 直接子节点，例如段落、标题、列表、引用、代码块和表格。
- **语义目标**：因为自身是 Heading、带有 OFM `^block-id`，或同时满足两者而可被寻址的节点。带 BlockId 的节点可以嵌套在列表、引用、callout 或其他容器中。

语义目标以节点为单位。Heading 目标只表示 Heading 节点本身，不表示它后面的 section。

## 公共 API 形态

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitControl {
    Continue,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockScanStatus {
    Complete,
    Stopped,
}

pub struct InlineSelection {
    // 私有、按 NodeId 去重的存储。
}

impl InlineSelection {
    pub fn select(&mut self, node_id: usize);
    pub fn is_empty(&self) -> bool;
}

pub struct SelectiveParser<'input> {
    // 各公开阶段之间保留的私有解析器状态。
}

pub struct SelectiveParseOutput {
    pub document: Document,
    pub block_status: BlockScanStatus,
}
```

`InlineSelection::select(node_id)` 选择该节点以及其所有能够接收 Inline 的后代。不会提供 `select_subtree` 或“排除某些后代”的 API，因为递归处理是唯一支持的选择语义。

解析流程通过一个 Rust-only 的中间状态表达，避免把多个无关 closure 塞进一个函数：

```rust
let mut parsed = Parser::new(source)
    .parse_blocks_with_checked(block_filter, block_visitor)?;

parsed.prepare_semantic_targets_checked()?;

let mut selection = InlineSelection::default();
parsed.visit_semantic_targets(
    semantic_filter,
    &mut selection,
    |target, selection| {
        if should_parse(target) {
            selection.select(target.node_id());
        }
        VisitControl::Continue
    },
)?;

let output = parsed.parse_selected_inlines_checked(selection)?;
```

非 checked 版本 `parse_blocks_with`、`prepare_semantic_targets` 和 `parse_selected_inlines` 与现有 `parse()` 一致，在解析错误时 panic。对应的 `_checked` 方法返回 `ParseError`。

### Filter 与 Visitor

每个事件阶段都包含一个 predicate 和一个 visitor：

- predicate 决定事件是否派发给 visitor；
- visitor 执行调用者逻辑并返回 `VisitControl`；
- predicate 拒绝的事件等同于 `Continue`，不会停止遍历。

顶层 Block Filter 直接检查现有 `MarkdownNode`，无需再增加一套公开 Kind 枚举：

```rust
|event| matches!(event.node().body, MarkdownNode::Heading(_) | MarkdownNode::Code(_))
```

### 顶层 Block 事件

```rust
pub struct TopLevelBlockEvent<'event> {
    // 字段私有，通过只读方法访问。
}

impl TopLevelBlockEvent<'_> {
    pub fn node_id(&self) -> usize;
    pub fn node(&self) -> &Node;
    pub fn tree(&self) -> &Tree<Node>;
}
```

只有 `Document` 的正文直接子节点会产生事件；`Document` 自身和 frontmatter 不会产生该事件。事件发生时，顶层节点及其所有后代的 Block 结构已经 finalized，但 Inline 子节点尚未生成，并且下一个顶层节点的内容还没有提交。

这一阶段的 `VisitControl::Stop`：

- 停止消费源码；
- 把已接受的前缀 finalized 为结构有效的树；
- 丢弃剩余输入；
- 对该前缀继续执行 BlockId 提取和 Heading Inline 解析；
- 不支持恢复。

解析器记录 Block 扫描是到达 EOF 还是提前停止，最终通过 `BlockScanStatus::Complete` 或 `BlockScanStatus::Stopped` 暴露结果。

### 语义目标事件

```rust
pub struct SemanticTarget<'event> {
    // 字段私有，通过只读方法访问。
}

impl SemanticTarget<'_> {
    pub fn node_id(&self) -> usize;
    pub fn node(&self) -> &Node;
    pub fn heading(&self) -> Option<&Heading>;
    pub fn block_id(&self) -> Option<&str>;
    pub fn tree(&self) -> &Tree<Node>;
}
```

解析器按文档前序遍历节点。一个节点只要是 Heading、带 BlockId，或两者兼有，就产生一次语义事件。带 BlockId 的 Heading 只产生一个同时暴露两种属性的目标，不重复派发。

语义 predicate 可以按 Heading 结构、完整 Heading Inline AST、BlockId、嵌套位置或它们的组合进行过滤。`visit_semantic_targets` 接收 `&mut InlineSelection`，并把它作为 visitor 的第二个参数显式传入。visitor 的返回值仍然只表示 `VisitControl`。

这一阶段的 `VisitControl::Stop` 只停止语义目标遍历。它不截断 Block Tree，也不会隐式选择当前目标。随后只对停止前显式加入 `InlineSelection` 的节点执行 Inline 解析。

## 处理流程

### 1. Block 阶段与顶层事件

解析器继续使用现有逐行 Block 算法。当 `Document` 的直接子节点到达稳定的 finalized 边界时，在提交下一个顶层子节点内容前派发事件。

到达 EOF 时，解析器先 finalized 所有打开的容器，再为最后一个顶层节点派发一次事件。如果 visitor 提前停止，返回树中不得残留已经部分创建的下一个兄弟节点。

### 2. BlockId 提取

把 OFM BlockId 识别从通用 Inline 解析中拆成一个可复用的小型扫描器。Block 阶段完成后，该扫描器检查 pending-inline map 中的每个 Inline-capable 节点，包括嵌套节点。

提取步骤必须：

- 遵守现有 OFM 语法和选项开关；
- 把 ID 写入 `Node.id`；
- 从 pending Inline Span 中移除 ID 标记，避免稍后变成文本；
- 不创建普通 Inline AST 节点；
- 保持独立 BlockId 行的现有行为；
- 不要求 ID 在全局唯一。

现有完整 Inline 路径也复用同一个扫描器，确保完整解析与选择性解析不会出现两套 BlockId 语法。

### 3. 全局引用定义准备

Link reference definition 可能位于选中内容之后。因此，在任何 Heading 或选中正文 Inline 开始前，现有引用定义预处理必须覆盖整个已接受的 Block 前缀。这样无需完整解析普通正文 Inline，也能维持文档级 reference link 语义。

该阶段属于元数据准备，不属于可选择的 Inline 工作。

### 4. Heading Inline 阶段

树中任意深度的所有 Heading 都按文档顺序完整执行 Inline 解析。由于完整 Heading AST 是语义过滤的前置条件，该步骤无条件执行。

该阶段结束后：

- Block 节点仍保留 Heading 类型和级别；
- emphasis、code、link、text 等 Heading Inline 子节点已经完整；
- Heading 如果带 BlockId，其 ID 已经写入节点；
- 语义 visitor 可以检查 Heading 子树；
- Heading 已从 pending-inline 集合移除，后续不会重复解析。

### 5. 语义遍历与选择

解析器按确定性的文档前序遍历已接受的树。Heading 或带 `Node.id` 的节点成为语义目标。语义 predicate 控制事件是否派发；visitor 可以独立调用 `selection.select(node_id)`，并返回 `Continue` 或 `Stop`。

Selection 内部存储私有且去重。如果祖先和后代同时被选择，后代条目属于冗余选择，不会造成重复 Inline 工作。

### 6. 选中 Inline 阶段

解析器把每个选中节点展开为其子树内所有仍处于 pending 状态的 Inline-capable 节点，再按文档顺序处理最终集合。后代展开是强制语义，调用者无法覆盖。

如果 Selection 为空，该阶段不执行任何操作。BlockId 与 Heading Inline AST 仍然存在，因为它们已在前置阶段完成。

未选择的 Inline-capable 节点保留 Block 结构，但不生成 Inline 子节点。本版本由调用者自行掌握选择过的根节点；不增加持久化 lazy session，也不允许 `Document` 返回后再执行第二轮解析。

### 7. Footnote 与其他依赖

Reference definition 元数据始终全局处理。Footnote definition 不同，因为其正文包含 Inline。当选中内容解析出 footnote reference 时，对应 definition 属于必要语义依赖；解析器会自动解析其子树，然后构建 footnote list。依赖展开由解析器控制，不属于调用者可排除的后代例外。

不在选择子树中、也未被选中内容引用的 footnote definition，不执行 Inline 解析，也不加入生成的 footnote list。

## 状态与顺序

- Block 阶段创建的 Node ID 在后续阶段保持稳定。
- 事件遍历和选中 Inline 处理都使用文档前序，即使当前 pending-inline 存储使用 HashMap。
- 顶层事件过滤不会改变已接受 Block 是否解析；只有 `Stop` 会截断已接受前缀。
- 语义过滤不会改变 Block Tree，只控制哪些目标进入 visitor。
- Selection 不影响遍历控制，遍历控制也不隐含 Selection。
- Heading 在工作流中只处理一次：Heading 阶段完成后，选中 Inline 阶段会排除它。

## 错误与限制

现有输入大小和节点数量限制在每个阶段继续生效。Checked API 返回现有 `ParseError`。如果 Block、Heading、依赖或选中 Inline 阶段超过限制，不返回成功的部分 `Document`。

用户回调不引入新的解析器错误类型；用户回调产生的 Rust panic 正常向上传播。

`InlineSelection::select` 中无效或未知的 Node ID 会在 Inline 解析前被拒绝。`ParseError` 新增 `InvalidSelectionNode { node_id: usize }`；checked API 返回该错误，非 checked API 按现有非 checked 解析方法的约定 panic。

## 兼容性

- `Parser::parse()` 和 `Parser::parse_checked()` 继续完整解析所有 Inline，行为保持不变。
- 现有 frontmatter-only 解析保持不变。
- 新工作流仅供 Rust 使用。
- `wasm-binding` 类型和导出保持不变。
- BlockId 识别重构为共享逻辑后，完整解析和选择性解析都必须通过现有 OFM BlockId 测试。

## 测试策略

### 顶层 Block 事件

- 只有 `Document` 的直接子节点产生事件。
- Kind predicate 可以抑制 visitor 调用且不影响解析。
- 嵌套 list、block quote 和 callout 不产生顶层事件。
- `Stop` 返回有效前缀，并排除刚观察到但尚未接受的下一个顶层 Block。
- EOF 处最后一个顶层事件只产生一次。

### BlockId 准备

- 根段落的 ID 在选中 Inline 解析前可用。
- 可以发现 list、block quote 和 callout 中的嵌套 ID。
- 独立 ID 保持现有 OFM 行为。
- 后续渲染文本不包含 ID 标记。
- 相同 ID 字符串仍然对应不同节点目标。

### Heading 准备

- ATX 和 Setext Heading 在语义 visitor 运行前具有完整 Inline AST。
- 嵌套 Heading 按文档顺序访问。
- 包含 emphasis、code 和 reference link 的 Heading 解析完整。
- 带 BlockId 的 Heading 只产生一个同时暴露两种属性的目标。

### Selection

- 空 Selection 不解析普通正文 Inline。
- 选择叶子节点时只处理该节点自身的 Inline。
- 选择容器时处理其所有 Inline-capable 后代。
- 同时选择祖先和后代不会产生重复节点或重复副作用。
- 语义阶段的 `Stop` 会处理此前的 Selection，但不截断 Block Tree。
- 选择包含 footnote 的内容时，会解析被引用的 definition 依赖。

### 兼容性

- 现有 CommonMark、GFM、OFM、CJK 和回归测试保持通过。
- 现有 HTML fixture 的完整解析输出保持逐字节一致。
- WASM 构建和现有两阶段测试保持不变。

## 性能预期

顶层 visitor 对每个 finalized 顶层 Block 增加一次 predicate 检查。BlockId 准备阶段对 pending Inline 容器做轻量扫描。Heading Inline 工作量与标题总内容量成正比。普通 Inline 工作量与选中子树及必要 footnote 依赖成正比。

本设计不复制 AST、不序列化 checkpoint，也不重复解析源码前缀。主要保留内存仍是现有 Block Tree，以及 Selection 完成前保存的 pending Inline Span。
