# Source-backed Text interface

**Type:** prototype
**Status:** resolved
**Blocked by:** None - can start immediately.

## Question

What exact public Text and Document interface realizes borrowed source-backed text in the current Tree, including an owned-source path, without a self-referential parser or a change to resolved HTML, Serde, and WASM output?

## Resolution criteria

- The prototype states the borrowed and owned construction signatures and `Document` lifetime behaviour.
- It identifies the document-bound text view/read seam that replaces direct `String` access.
- It explains how the parser drops scanner and pending state before moving an owned source into the completed document.
- It gives a migration shape for Rust callers and concrete invariants for HTML, Serde, and WASM compatibility.

## Comments

- This ticket resolves the interface, not the implementation or benchmark result.
- Its answer must provide the interface input to [Choose v2A module gates and order](05-choose-v2a-module-gates-and-order.md).

## Proposal（2026-07-26 草案，待人工评审后转为 Answer）

### 1. 构造签名与 Document 生命周期

```rust
pub enum SourceText<'source> {
    Borrowed(&'source str),
    Owned(String),
}

pub struct Document<'source> {
    source: SourceText<'source>,
    tree: Tree<Node>,            // 现有 Tree 不变
    tags: FxHashSet<String>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self;                                  // 既有
    pub fn new_with_options(source: &'source str, options: ParserOptions) -> Self;
    pub fn parse(self) -> Document<'source>;                                   // 返回类型带生命周期
    pub fn parse_checked(self) -> Result<Document<'source>, ParseError>;
}

// owned-source 路径（WASM 与需要文档独立于输入缓冲的调用者）
impl Parser<'_> {
    pub fn parse_string(source: String, options: ParserOptions) -> Document<'static>;
    pub fn parse_string_checked(
        source: String,
        options: ParserOptions,
    ) -> Result<Document<'static>, ParseError>;
}
```

`Document<'static>` 之所以成立：Text 载荷存 **byte range 而非 `&str`**，range 在读取时经
document-bound view 对 source 解析；移动 `String` 不影响 range 有效性，无自引用、无 unsafe。

### 2. Text 载荷与唯一读取缝

```rust
// MarkdownNode::Text(String) → MarkdownNode::Text(TextRef)
pub struct SourceSpan { start: u32, end: u32 }    // 半开区间，UTF-8 合法边界

pub enum TextRef {
    Source(SourceSpan),   // 普通连续文本、delimiter/bracket 标记等未转换内容
    Owned(String),        // entity/escape/smart punct/CJK 转换、跨段合并、生成内容
}

impl<'source> Document<'source> {
    pub fn source(&self) -> &str;
    /// 唯一读取缝：HTML、Serde、WASM、Rust 调用者一律经此取显示文本。
    pub fn text<'doc>(&'doc self, text_ref: &'doc TextRef) -> &'doc str;
}
```

第一切片只把普通连续文本与标记文本转为 `Source`；Tag/Emoji 等其它 String 载荷不动。

### 3. owned-source 无自引用移交

```rust
pub fn parse_string(source: String, options: ParserOptions) -> Document<'static> {
    let (tree, tags) = {
        let parser = Parser::new_with_options(&source, options);  // 临时借用
        parser.parse_into_parts()                                 // Scanner/pending/Span 在此内部全部释放
    };                                                            // ← 对 source 的借用在此结束
    Document { source: SourceText::Owned(source), tree, tags }    // 之后才移动 String
}
```

`parse_into_parts` 为私有出口，返回 `(Tree<Node>, FxHashSet<String>)`：Tree 内只有 range 与
owned String，不携带 `'input` 借用，借用检查静态成立。

### 4. 迁移形态与兼容不变量

- **Rust breaking（唯一一类）**：解构 `MarkdownNode::Text(s)` 的调用点改为
  `Text(r) => document.text(r)`；CHANGELOG 提供迁移段。
- **HTML**：`to_html` 从 `Tree` 方法改为 `Document` 方法（renderer 需要 source）；输出逐字节不变。
- **Serde**：`TextRef::Source` 无法脱离 source 独立序列化；改经 `Document` 侧包装序列化，
  输出与现在相同的已解析文本 JSON，WASM/TS 结构不变。
- **WASM**：binding 已为 two-phase 持有 owned source；改走 `parse_string`，导出行为不变。
- 位置、BlockId、tags、reference、footnote 元数据不受影响。

### 决策点（需人工确认）

- **A. owned 构造入口命名**：`Parser::parse_string(String, options)`（草案推荐，最小实现、调用体验贴近现状）vs `Document::from_string` vs 让 `Parser` 自身持有 `SourceText`（后者要求 scanner 改为纯偏移量、每次方法调用重新借用，实现成本显著更高，建议留给 v2B 评估）。
- **B. 变体形态**：保留变体名 `MarkdownNode::Text(TextRef)`（草案推荐）vs 拆分 `Text(SourceSpan)`/`OwnedText(String)` 两个变体。
- **C. Serde 兼容策略**：document-bound 包装序列化（草案推荐）vs 序列化前把全部 `Source` 物化为 owned（简单但破坏 P1 的分配收益于序列化场景）。

## Answer

2026-07-26 解决。维护者对 A/B/C 无倾向、委托按草案推荐执行，故上文 Proposal 全文即接口结论，三个决策点取推荐项：

- **A**：owned 构造入口为 `Parser::parse_string(String, ParserOptions) -> Document<'static>`（含 `parse_string_checked`）。内部经临时借用解析、scanner/pending/Span 释放后再移交 `String`；`Parser` 持有 `SourceText` 的统一形态留给 v2B 依证据评估。
- **B**：保留变体名，`MarkdownNode::Text(TextRef)`，`TextRef = Source(SourceSpan) | Owned(String)`；`Document::text(&TextRef) -> &str` 是唯一读取缝。
- **C**：Serde 走 document-bound 包装序列化，输出 JSON 与现有已解析文本完全一致；不采用序列化前整体物化。

约束回执：borrowed 默认不复制源码；owned 路径无自引用、无 unsafe；HTML/Serde/WASM 输出逐字节不变；Rust breaking 仅"解构 `Text(String)` → 经 `document.text`"一类，附迁移说明。实施归属渐进计划 P1（tracer ticket 12）。
