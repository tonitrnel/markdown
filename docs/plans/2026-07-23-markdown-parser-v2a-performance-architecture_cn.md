# Markdown 解析器 v2A 性能架构实施计划

> **执行状态（2026-07-26）：** 本文保留为 v2A 架构提案、风险分析与挑战指标的历史记录，不再作为实施顺序。实施前先完成 [Wayfinder 决策地图](../../.scratch/markdown-parser-incremental-iteration/map.md)；[渐进式迭代计划](2026-07-26-markdown-parser-incremental-iteration_cn.md) 在此期间只是工作假设。完成地图后一次只改造一个 module，始终保留一条正式解析路径。`5 ms`、相对 `2x` 与分配降低 `50%` 是挑战目标，不是本轮的预先承诺。

> 注：superpowers 已移除，不要使用

> **面向代理式工作器：** 必需子技能：使用 `superpowers:subagent-driven-development`（推荐）或 `superpowers:executing-plans`，逐项实施本计划。各步骤使用复选框（`- [ ]`）语法进行跟踪。

**目标：** 在满足 v2A 正确性与性能门槛的前提下，交付具备源码支撑文本、紧凑稳定 AST 存储、索引化 Inline 工作区、事务式 Inline 提交、选择性物化，以及同进程中断/恢复能力的 v2A 解析器架构。

**架构：** 在现有解析器旁并行构建 v2 引擎，使每项结构性变更都能独立测试和基准评估；在证明语义等价后，再切换公共外观接口。源码偏移量是热路径中的规范表示；`Document` 持有或借用源码，文本由源码切片或逻辑片段表示，只有完成解析的 Inline 节点才提交到紧凑 AST 存储中。

**技术栈：** Rust 2024、基于 `Vec` 的类型化存储、`smallvec`、`memchr`、`rustc-hash`、Criterion、proptest、Serde、wasm-bindgen。

## 全局约束

- 在等价的 CommonMark 和共享 GFM 测试通道中，v2A 完整 AST 的纯解析速度不得慢于 `pulldown-cmark` 完整事件消费的 2 倍。
- 在参考机器上，570,143 字节的 OFM 产品语料库纯解析中位数不得高于 5.0 ms；结果旁必须记录参考机器与工具链信息。
- `_data.md` 的每次解析分配次数必须较约 20,571 次的基线至少降低 50%。
- 保持 CommonMark、GFM、OFM、CJK、HTML、位置、引用、BlockId、脚注、标注块、表格、frontmatter 及现有 HTML 输出语义不变。
- 完整解析与选择性解析必须共享同一套 Block、语义准备、Inline 解析和 AST 提交实现。
- 不得为每个分隔符、每个括号或每个临时 token 单独进行堆分配。
- 除非所有结构性工作均已完成，且独立基准证明可带来至少约 5% 的语料库性能提升，否则不得引入通用 bump allocator。
- v2A 不实现直接流式 HTML 后端。
- 不实现解析器状态序列化、跨进程恢复、磁盘检查点、源码变更后的恢复，或单文档并行解析。
- 稳定 `NodeId` 意味着节点永不重新编号；Block 关系边仍可进行受控修改。
- 只有完成局部结构规范化后，顶层事件才可暴露 `NodeId`，其中包括移除仅含引用定义的 Paragraph。
- `PendingInline` 使用比单一 `SourceSpan` 更丰富的逻辑片段；容器前缀和合成连接必须仍可表示。
- WASM 持有自己的源码，不暴露 Rust visitor 或可恢复会话。
- 现有 `CHANGELOG.md` 属于用户维护内容，本计划不得覆盖或还原该文件。

---

## 文件与模块映射

最终实现应收敛为以下职责分工：

- `src/source.rs`：`SourceSpan`、`SourceText`、偏移量校验与源码切片。
- `src/location.rs`：`LineIndex` 以及从字节偏移量到 `Location` 的惰性转换。
- `src/storage/mod.rs`：紧凑 `AstStorage` 所有者，以及容量/限制预检。
- `src/storage/id.rs`：`NodeId`、打包的可选 ID、payload ID 与受检转换。
- `src/storage/node.rs`：紧凑节点头、`NodeKind`、标志位与关系修改。
- `src/storage/text.rs`：`TextRef`、片段 arena、自有字节池、`TextView` 与命中率指标。
- `src/storage/view.rs`：`NodeRef`、子节点/兄弟节点迭代器，以及绑定到文档生命周期的只读 API。
- `src/pending.rs`：有序 `PendingInlineStore`、`PendingSegment`、状态与稠密查找。
- `src/semantic/mod.rs`：语义准备流程编排。
- `src/semantic/reference_definition.rs`：在稳定事件之前提取引用定义。
- `src/semantic/block_id.rs`：共享的 OFM BlockId 提取。
- `src/inlines/workspace.rs`：可复用 token、索引化分隔符/括号栈，以及 ID 类型。
- `src/inlines/ir.rs`：已解析的 Inline 节点、文本草稿、元数据效果与依赖项。
- `src/inlines/commit.rs`：批次预检与仅追加式 Inline 提交。
- `src/session.rs`：Block/语义/选择阶段的所有权与中断结果。
- `src/document.rs`：包含源码、AST 与元数据的最终文档。
- `src/parser.rs`：公共解析器外观接口与完整解析快速路径。
- `src/render/html.rs`：基于 `NodeRef`/`TextView` 的渲染器，不进行中间文本拼接。
- `wasm-binding/src/lib.rs`：自有源码构造，以及通过 view 完成 AST 转换。
- `tests/v2_semantic_equivalence.rs`：迁移期间新旧语义与 HTML 对比。
- `tests/v2_text_storage.rs`：源码/复合/自有文本行为。
- `tests/v2_selective.rs`：事件、选择、依赖、中断与恢复语义。
- `bench/benches/v2_stages.rs`：Block、准备、Inline 解析、提交、渲染与释放耗时。
- `bench/benches/alloc_count.rs`：每次解析的分配/重新分配/字节数报告。
- `bench/compare/native/benches/parser_compare.rs`：等价 CommonMark/共享 GFM 通道与 OFM 产品通道。

用于临时比较 legacy 与 v2 的接口是 `#[doc(hidden)] pub struct ParserV2`。任务 11 将移除 `ParserV2`，把 v2 外观接口改名为 `Parser`，并删除旧的 Tree 驱动解析器热路径。

任务 3 提供将文本附加到稳定节点所需的紧凑 ID 与存储骨架；任务 4 是首个覆盖整个解析器的性能特性，必须在开始索引化 Inline 或 Block 优化之前完成基准评估。

---

### 任务 1：建立可复现的正确性与性能基线

**文件：**
- 修改：`Cargo.toml`
- 修改：`bench/benches/benchmark.rs`
- 修改：`bench/benches/phase_bench.rs`
- 修改：`bench/benches/alloc_count.rs`
- 修改：`bench/compare/native/benches/parser_compare.rs`
- 新建：`bench/benches/v2_stages.rs`
- 新建：`tests/support/mod.rs`
- 新建：`tests/support/semantic.rs`
- 新建：`tests/v2_semantic_equivalence.rs`

**接口：**
- 输入：现有 `Parser`、`ParserOptions`、`Document`、`Tree<Node>` 和 `to_html()`。
- 输出：legacy 与 v2 的语义摘要、命名基准通道，以及供后续所有任务使用的每次解析分配计数。

- [ ] **步骤 1：添加一个引用尚未创建的 v2 解析器的语义摘要测试**

```rust
mod support;

use markdown::{Parser, ParserOptions, ParserV2};
use support::semantic::{legacy_semantic_digest, v2_semantic_digest};

#[test]
fn v2_matches_legacy_semantics_for_curated_fixture() {
    let source = include_str!("../bench/fixtures/curated/_data.md");
    let options = ParserOptions::default().enabled_ofm();
    let legacy = Parser::new_with_options(source, options.clone()).parse();
    let v2 = ParserV2::new_with_options(source, options).parse();

    assert_eq!(legacy_semantic_digest(&legacy), v2_semantic_digest(&v2));
    assert_eq!(legacy.to_html(), v2.to_html());
}
```

- [ ] **步骤 2：运行测试并确认其因 `ParserV2` 不存在而失败**

执行： `cargo test --test v2_semantic_equivalence v2_matches_legacy_semantics_for_curated_fixture`

预期：由于迁移相关导入无法解析而失败；其中 `markdown::ParserV2` 不存在是必须观察到的失败，摘要辅助函数在步骤 3 前也可能无法解析。

- [ ] **步骤 3：添加 legacy 语义摘要辅助函数，并将 v2 对比测试标记为忽略，直至任务 5**

`SemanticDigest` 必须包含节点类型、父子顺序、起止位置、块 ID、显示文本或 payload 的调试表示、按字典序排序的标签，以及渲染后的 HTML。通过对 `Tree<Node>` 进行显式递归遍历实现 `legacy_semantic_digest(&Document) -> SemanticDigest`，并将 `v2_semantic_digest(&DocumentV2) -> SemanticDigest` 留到任务 5。不得序列化指针地址或 HashMap 的迭代顺序。

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct SemanticDigest {
    pub nodes: Vec<SemanticNode>,
    pub tags: Vec<String>,
    pub html: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticNode {
    pub kind: String,
    pub parent_preorder: Option<usize>,
    pub start: (u64, u64),
    pub end: (u64, u64),
    pub block_id: Option<String>,
    pub payload: String,
}
```

观察到失败后，用以下基线测试替换无法解析的对比测试：

```rust
mod support;

use markdown::{Parser, ParserOptions};
use support::semantic::legacy_semantic_digest;

#[test]
fn legacy_semantic_digest_is_deterministic() {
    let source = include_str!("../bench/fixtures/curated/_data.md");
    let options = ParserOptions::default().enabled_ofm();
    let first = Parser::new_with_options(source, options.clone()).parse();
    let second = Parser::new_with_options(source, options).parse();

    assert_eq!(
        legacy_semantic_digest(&first),
        legacy_semantic_digest(&second),
    );
}
```

待 `ParserV2` 和 `v2_semantic_digest` 存在后，任务 5 会重新加入新旧对比。

- [ ] **步骤 4：将基准矩阵拆分为显式通道**

在 `parser_compare.rs` 中实现以下精确通道：

```rust
enum Lane {
    CommonMark,
    SharedGfm,
    OfmProduct,
}

fn local_options(lane: Lane) -> ParserOptions {
    match lane {
        Lane::CommonMark => ParserOptions::default(),
        Lane::SharedGfm => ParserOptions::default().enabled_gfm(),
        Lane::OfmProduct => ParserOptions::default().enabled_ofm(),
    }
}

fn pulldown_options(lane: Lane) -> Option<pulldown_cmark::Options> {
    match lane {
        Lane::CommonMark => Some(pulldown_cmark::Options::empty()),
        Lane::SharedGfm => Some(
            pulldown_cmark::Options::ENABLE_TABLES
                | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
                | pulldown_cmark::Options::ENABLE_TASKLISTS,
        ),
        Lane::OfmProduct => None,
    }
}
```

对每个受支持的比较器，分别对纯解析和解析加 HTML 进行基准测试。通过 `criterion::black_box` 消费结果；不要把 OFM 产品模式与所谓等价的 pulldown 模式进行比较。

- [ ] **步骤 5：将分配报告修正为每次解析的数值**

使用固定样例路径 `bench/fixtures/curated/_data.md`，在预热后精确测量 500 次解析，并在打印前将 alloc、realloc、dealloc 和字节数分别除以 500。`REALLOC_BYTES` 必须累加 `new_size`，而不是旧布局大小。

预期输出格式：

```text
parses=500 allocs_per_parse=... reallocs_per_parse=... alloc_bytes_per_parse=... median_us=...
```

- [ ] **步骤 6：在阶段钩子尚未存在前添加阶段基准名称**

在 `Cargo.toml` 中注册 `v2_stages`，并创建名为 `block_scan`、`semantic_prepare`、`inline_resolve`、`ast_commit`、`render` 和 `drop` 的基准用例。起初在该文件中加入编译期错误：

```rust
compile_error!("remove this line when ParserV2 stage hooks are added in Task 8");
```

在任务 8 移除该编译错误之前，不得在 CI 中运行此基准。

- [ ] **步骤 7：运行基线验证**

执行：

```bash
cargo test --all-features
cargo bench --bench benchmark --no-run
cargo bench --bench phase_bench --no-run
cargo bench -p parser-compare-bench --bench parser_compare --no-run
cargo bench --bench alloc_count
```

预期：所有测试与编译均通过；分配输出按每次解析报告数值。`v2_stages` 被有意排除。

- [ ] **步骤 8：提交基线**

```bash
git add Cargo.toml bench tests
git commit -m "bench: establish v2 performance and semantic baselines"
```

在提交信息正文中，或在 `bench/results/` 下附加的基准日志中，记录机器、Rust 版本、提交、通道选项、中位数、分配次数和字节数。

---

### 任务 2：添加源码所有权、紧凑 Span 与惰性位置基础类型

**文件：**
- 新建：`src/source.rs`
- 新建：`src/location.rs`
- 修改：`src/lib.rs`
- 新建：`tests/v2_source.rs`
- 修改：`tests/location.rs`

**接口：**
- 输入：UTF-8 `&str` 或自有 `String`。
- 输出：`SourceText<'source>`、`SourceSpan`、`LineIndex`、`LineId`、`Location`，以及供存储、Block、待处理 Inline 和 WASM 使用的受检切片 API。

- [ ] **步骤 1：编写会失败的源码所有权与位置测试**

```rust
use markdown::{LineIndex, SourceSpan, SourceText};

#[test]
fn borrowed_and_owned_sources_slice_identically() {
    let borrowed = SourceText::borrowed("a中\nb");
    let owned = SourceText::owned("a中\nb".to_owned());
    let span = SourceSpan::new(1, 4).unwrap();

    assert_eq!(borrowed.slice(span).unwrap(), "中");
    assert_eq!(owned.slice(span).unwrap(), "中");
}

#[test]
fn line_index_maps_utf8_offsets_lazily() {
    let source = "a中\r\n🙂b";
    let index = LineIndex::new(source).unwrap();

    assert_eq!(index.location(source, 0).unwrap(), markdown::Location::new(1, 1));
    assert_eq!(index.location(source, 1).unwrap(), markdown::Location::new(1, 2));
    assert_eq!(index.location(source, 6).unwrap(), markdown::Location::new(2, 1));
    assert_eq!(index.location(source, 10).unwrap(), markdown::Location::new(2, 2));
}
```

- [ ] **步骤 2：运行测试并确认导入无法解析**

执行： `cargo test --test v2_source`

预期：失败，因为未导出 `SourceText`、`SourceSpan` 和 `LineIndex`。

- [ ] **步骤 3：按精确定义实现源码类型**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    start: u32,
    end: u32,
}

impl SourceSpan {
    pub fn new(start: u32, end: u32) -> Result<Self, SourceError>;
    pub const fn start(self) -> u32;
    pub const fn end(self) -> u32;
    pub const fn len(self) -> u32;
    pub const fn is_empty(self) -> bool;
}

pub enum SourceText<'source> {
    Borrowed(&'source str),
    Owned(String),
}

impl<'source> SourceText<'source> {
    pub fn borrowed(source: &'source str) -> Self;
    pub fn owned(source: String) -> SourceText<'static>;
    pub fn as_str(&self) -> &str;
    pub fn slice(&self, span: SourceSpan) -> Result<&str, SourceError>;
    pub fn len_u32(&self) -> Result<u32, SourceError>;
}
```

对反向范围、超过 `u32::MAX` 的偏移量、超出源码的范围以及非 UTF-8 边界，分别返回不同的 `SourceError` 变体。

- [ ] **步骤 4：使用解析所需的同一套换行扫描实现 `LineIndex`**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineId(u32);

pub struct LineIndex {
    starts: Vec<u32>,
}

impl LineIndex {
    pub fn new(source: &str) -> Result<Self, SourceError>;
    pub fn push_line_start(&mut self, offset: u32) -> Result<LineId, SourceError>;
    pub fn location(
        &self,
        source: &str,
        offset: u32,
    ) -> Result<Location, SourceError>;
}
```

将 CRLF 视为一个行终止符；位置从 1 开始；ASCII 前缀使用字节列号，只有非 ASCII 前缀才计算 Unicode 标量值数量。

添加 `LineId::new(u32) -> Result<LineId, SourceError>` 和 `LineId::get(self) -> u32`；保留 `0` 作为无效值。

- [ ] **步骤 5：为安全 Span 添加属性测试**

对于任意有效 Rust 字符串和任意字节偏移量：

- 位于有效字符边界的 Span 可通过 `slice` 往返还原；
- 无效边界返回错误，且绝不 panic；
- 对于 LF、CRLF、制表符、CJK、emoji 和末尾空行，`location` 与现有扫描器测试样例一致。

- [ ] **步骤 6：运行源码与位置测试**

执行：

```bash
cargo test --test v2_source
cargo test --test location
cargo test --test z_property_tests
```

预期：通过。

- [ ] **步骤 7：提交**

```bash
git add src/source.rs src/location.rs src/lib.rs tests/v2_source.rs tests/location.rs tests/z_property_tests.rs
git commit -m "feat: add source-backed spans and lazy line index"
```

---

### 任务 3：构建紧凑稳定的 AST 存储并执行 Payload 布局门槛评估

**文件：**
- 新建：`src/storage/mod.rs`
- 新建：`src/storage/id.rs`
- 新建：`src/storage/node.rs`
- 新建：`src/storage/view.rs`
- 修改：`src/lib.rs`
- 新建：`tests/v2_storage.rs`
- 新建：`bench/benches/storage_layout.rs`
- 修改：`Cargo.toml`

**接口：**
- 输入：`SourceSpan`、现有 AST payload 结构体，以及已配置的节点/payload 限制。
- 输出：稳定的 `NodeId`、`AstStorage`、关系操作、`NodeRef`，以及选定的 payload 布局。

- [ ] **步骤 1：编写会失败的 ID 稳定性与关系测试**

```rust
use markdown::{AstStorage, NodeKind, SourceSpan};

#[test]
fn node_ids_survive_vec_growth_and_edge_mutation() {
    let mut ast = AstStorage::new();
    let root = ast.push_root(SourceSpan::new(0, 0).unwrap()).unwrap();
    let paragraph = ast
        .append_child(root, NodeKind::Paragraph, SourceSpan::new(0, 3).unwrap())
        .unwrap();

    for _ in 0..10_000 {
        ast.append_child(root, NodeKind::Paragraph, SourceSpan::new(0, 0).unwrap())
            .unwrap();
    }

    assert_eq!(ast.node(paragraph).unwrap().kind(), NodeKind::Paragraph);
    assert_eq!(ast.node(paragraph).unwrap().parent().unwrap().id(), root);
}

#[test]
fn removing_unexposed_block_does_not_renumber_later_nodes() {
    let mut ast = AstStorage::new();
    let root = ast.push_root(SourceSpan::new(0, 0).unwrap()).unwrap();
    let removed = ast.append_child(root, NodeKind::Paragraph, SourceSpan::new(0, 1).unwrap()).unwrap();
    let kept = ast.append_child(root, NodeKind::Paragraph, SourceSpan::new(2, 3).unwrap()).unwrap();

    ast.remove_unexposed_leaf(removed).unwrap();
    assert_eq!(ast.node(kept).unwrap().source_span(), SourceSpan::new(2, 3).unwrap());
}
```

- [ ] **步骤 2：运行存储测试并确认导入无法解析**

执行： `cargo test --test v2_storage`

预期：失败，因为紧凑存储类型尚不存在。

- [ ] **步骤 3：实现以零为哨兵值的打包 ID**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(NonZeroU32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PackedNodeId(u32);

impl PackedNodeId {
    pub const NONE: Self = Self(0);
    pub fn from_node_id(id: NodeId) -> Self;
    pub fn into_option(self) -> Option<NodeId>;
}
```

`Vec<Node>` 中索引 `0` 对应公共 `NodeId(1)`。受检构造函数应拒绝无法装入 `u32` 的节点数或 payload 数量。

为 `ParseError` 添加后续任务会使用的精确受检失败类型：

```rust
SourceOffsetOverflow { actual: usize },
PayloadLimitExceeded { limit: u32, actual: u64 },
OwnedTextLimitExceeded { limit: u32, actual: u64 },
InvalidSelectionNode { node_id: u32 },
InvalidSourceSpan { start: u32, end: u32 },
WrongParserPhase,
```

- [ ] **步骤 4：实现紧凑关系头部**

```rust
pub(crate) struct Node {
    pub kind: NodeKind,
    pub flags: NodeFlags,
    pub span: SourceSpan,
    pub parent: PackedNodeId,
    pub first_child: PackedNodeId,
    pub last_child: PackedNodeId,
    pub next_sibling: PackedNodeId,
    pub payload: PayloadId,
}
```

提供追加操作、供 Block 算法使用的受控解除链接/重新挂载，以及 `remove_unexposed_leaf`。不要实现通用的空闲节点 HashSet。事件暴露前被移除的 Block 节点应成为显式 tombstone，且 `node()` 不得返回它们。

- [ ] **步骤 5：仅在基准中实现两种布局候选方案**

基准测试内容：

1. 紧凑节点头 + 紧凑 payload 枚举；
2. 紧凑节点头 + 按类型划分的次级 payload `Vec`。

使用从 `_data.md` 和 `markdown-it-corpus` 测得的同一组合成节点类型分布，然后对构建、前序遍历、类似 HTML 的 payload 读取以及释放进行基准测试。只有满足以下条件时才保留次级 arena：

- 解析/遍历时间的几何平均值慢幅不超过 3%；
- 总保留字节数至少降低 10%，或几何平均速度至少提升 5%；
- Serde/WASM 查找不需要在每次 payload 访问时进行堆分配。

否则使用紧凑 payload 枚举。将选定布局及测量结果记录在 `storage/mod.rs` 的模块文档中。

- [ ] **步骤 6：实现 `NodeRef` 与迭代器**

```rust
#[derive(Clone, Copy)]
pub struct NodeRef<'doc> {
    storage: &'doc AstStorage,
    id: NodeId,
}

impl<'doc> NodeRef<'doc> {
    pub fn id(self) -> NodeId;
    pub fn kind(self) -> NodeKind;
    pub fn source_span(self) -> SourceSpan;
    pub fn parent(self) -> Option<NodeRef<'doc>>;
    pub fn children(self) -> Children<'doc>;
    pub fn next_sibling(self) -> Option<NodeRef<'doc>>;
}
```

迭代器持有 ID 和存储引用；不得分配临时 `Vec`。

- [ ] **步骤 7：断言热路径类型大小**

添加带显式上限的 64 位测试：

```rust
assert!(size_of::<NodeId>() <= 4);
assert!(size_of::<PackedNodeId>() <= 4);
assert!(size_of::<storage::Node>() <= 40);
```

若选定的 payload 布局使节点头超过 40 字节，则本任务应失败并重新审视字段打包，而不是提高上限。

- [ ] **步骤 8：运行测试与存储基准**

执行：

```bash
cargo test --test v2_storage
cargo bench --bench storage_layout
```

预期：测试通过，且基准记录明确指出唯一一个被接受的布局。

- [ ] **步骤 9：提交**

```bash
git add Cargo.toml src/storage src/lib.rs tests/v2_storage.rs bench/benches/storage_layout.rs
git commit -m "feat: add compact stable AST storage"
```

---

### 任务 4：在不为每段文本单独创建字符串的前提下实现源码、复合与自有文本

**文件：**
- 新建：`src/storage/text.rs`
- 修改：`src/storage/mod.rs`
- 修改：`src/storage/node.rs`
- 修改：`src/storage/view.rs`
- 新建：`tests/v2_text_storage.rs`
- 新建：`bench/benches/text_storage.rs`
- 修改：`Cargo.toml`

**接口：**
- 输入：`SourceSpan`、生成字符、转换后的片段，以及文档源码。
- 输出：`TextRef`、`TextPiece`、`TextStorage`、`TextView`、无中间 `String` 的渲染，以及命中率计数器。

- [ ] **步骤 1：为三种表示形式编写会失败的测试**

```rust
use markdown::{SourceSpan, TextBuilder, TextView};

#[test]
fn contiguous_text_borrows_source() {
    let source = "hello";
    let mut storage = markdown::TextStorage::new();
    let mut builder = TextBuilder::new(source, &mut storage);
    builder.push_source(SourceSpan::new(0, 5).unwrap());
    let text_ref = builder.finish();
    let view = TextView::new(source, text_ref, &storage);

    assert_eq!(view.as_str(), Some("hello"));
    assert_eq!(view.to_cow(), "hello");
}

#[test]
fn composite_text_writes_without_materializing_during_parse() {
    let source = "> first\n> second";
    let mut storage = markdown::TextStorage::new();
    let mut builder = TextBuilder::new(source, &mut storage);
    builder.push_source(SourceSpan::new(2, 7).unwrap());
    builder.push_char('\n');
    builder.push_source(SourceSpan::new(10, 16).unwrap());
    let text_ref = builder.finish();
    let view = TextView::new(source, text_ref, &storage);
    let mut output = String::new();

    view.write_to(&mut output).unwrap();
    assert_eq!(output, "first\nsecond");
    assert_eq!(view.as_str(), None);
}
```

- [ ] **步骤 2：运行测试并确认类型无法解析**

执行： `cargo test --test v2_text_storage`

预期：失败，因为文本存储类型尚不存在。

- [ ] **步骤 3：实现紧凑文本引用与共享自有字节池**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextRef {
    Source(SourceSpan),
    Composite(PieceRange),
    Owned(OwnedSpan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextPiece {
    Source(SourceSpan),
    Char(char),
    Owned(OwnedSpan),
}

pub struct TextStorage {
    pieces: Vec<TextPiece>,
    owned: Vec<u8>,
}

pub struct TextBuilder<'source, 'storage> {
    source: &'source str,
    storage: &'storage mut TextStorage,
    draft_pieces: SmallVec<[TextPiece; 4]>,
}

impl<'source, 'storage> TextBuilder<'source, 'storage> {
    pub fn new(source: &'source str, storage: &'storage mut TextStorage) -> Self;
    pub fn push_source(&mut self, span: SourceSpan);
    pub fn push_char(&mut self, value: char);
    pub fn push_owned(&mut self, value: &str);
    pub fn finish(self) -> TextRef;
}
```

`OwnedSpan` 为共享 `owned` 字节缓冲区建立索引。追加时只校验一次 UTF-8。不要在每个 AST 节点内部存储 `Vec<String>` 或 `SmallVec`。

- [ ] **步骤 4：实现合并与碎片化回退策略**

`TextBuilder` 必须：

- 当 `previous.end == next.start` 时合并相邻源码 Span；
- 在共享字节池中合并相邻的自有片段；
- 为不连续且未经转换的文本保留逻辑片段列表；
- 当片段数超过配置阈值时，回退为一个自有 Span。

在纯文本、块引用、实体、转义和 CJK 测试样例上，对 4、8、16 个片段的阈值进行基准测试。选择解析加渲染几何平均值最佳的阈值，但其分配次数不得比最佳候选方案高出 2% 以上。

- [ ] **步骤 5：实现 `TextView`**

```rust
impl<'doc> TextView<'doc> {
    pub fn as_str(self) -> Option<&'doc str>;
    pub fn pieces(self) -> TextPieces<'doc>;
    pub fn write_to<W: std::fmt::Write>(self, out: &mut W) -> std::fmt::Result;
    pub fn to_cow(self) -> std::borrow::Cow<'doc, str>;
}
```

`to_cow` 对 Source 和 Owned 值使用借用，仅对 Composite 值分配内存。`write_to` 绝不构建中间拼接 `String`。

- [ ] **步骤 6：添加表示形式指标**

暴露仅供基准使用的计数器：

```rust
pub struct TextStorageMetrics {
    pub source_nodes: u64,
    pub composite_nodes: u64,
    pub owned_nodes: u64,
    pub source_output_bytes: u64,
    pub composite_output_bytes: u64,
    pub owned_output_bytes: u64,
}
```

统计逻辑输出字节数，而不是 Markdown 标记字节数。

- [ ] **步骤 7：运行测试与阈值基准**

执行：

```bash
cargo test --test v2_text_storage
cargo bench --bench text_storage
```

预期：通过；基准输出记录选定的碎片化阈值和源码支撑命中率。

- [ ] **步骤 8：提交**

```bash
git add Cargo.toml src/storage tests/v2_text_storage.rs bench/benches/text_storage.rs
git commit -m "feat: add source-backed composite text storage"
```

---

### 任务 5：引入 v2 Document、解析器骨架与基于 View 的渲染器

**文件：**
- 新建：`src/document.rs`
- 新建：`src/session.rs`
- 修改：`src/parser.rs`
- 修改：`src/lib.rs`
- 修改：`src/render/html.rs`
- 修改：`tests/v2_semantic_equivalence.rs`
- 新建：`tests/v2_document.rs`

**接口：**
- 输入：源码基础类型、紧凑 AST、文本存储，以及现有解析器选项。
- 输出：临时 `ParserV2`、借用型与自有型 `DocumentV2`、`NodeRef::text()`，以及通过 view 完成的 HTML 渲染。在任务 6 移植 Block 解析之前，`ParserV2::parse()` 可以使用明确标注为过渡方案的 legacy 到紧凑存储转换；基准必须将该路径标注为过渡路径，任务 11 必须删除它。

- [ ] **步骤 1：编写会失败的借用型/自有型解析器测试**

```rust
use markdown::{ParserOptions, ParserV2};

#[test]
fn owned_parser_document_outlives_input_variable() {
    let document = {
        let source = String::from("# title");
        ParserV2::from_string_with_options(source, ParserOptions::default()).parse()
    };

    assert_eq!(document.to_html(), "<h1>title</h1>");
}

#[test]
fn borrowed_parser_does_not_copy_source() {
    let source = String::from("text");
    let document = ParserV2::new(&source).parse();

    assert_eq!(document.source().as_ptr(), source.as_ptr());
}
```

- [ ] **步骤 2：运行测试并确认 `ParserV2` 无法解析**

执行： `cargo test --test v2_document`

预期：失败，因为未导出 `ParserV2`。

- [ ] **步骤 3：添加不包含自引用的解析器骨架**

```rust
#[doc(hidden)]
pub struct ParserV2<'source> {
    source: SourceText<'source>,
    options: ParserOptions,
    state: ParserState,
}

impl<'source> ParserV2<'source> {
    pub fn new(source: &'source str) -> Self;
    pub fn new_with_options(source: &'source str, options: ParserOptions) -> Self;
}

impl ParserV2<'static> {
    pub fn from_string(source: String) -> Self;
    pub fn from_string_with_options(source: String, options: ParserOptions) -> Self;
}
```

`ParserState` 存储扫描器字节偏移量，绝不存储 `&str`。所有扫描仅在一次方法调用期间借用 `self.source.as_str()`。

- [ ] **步骤 4：实现 `DocumentV2` 与绑定到文档生命周期的 View**

```rust
pub struct DocumentV2<'source> {
    source: SourceText<'source>,
    ast: AstStorage,
    line_index: LineIndex,
    metadata: DocumentMetadata,
}

impl<'source> DocumentV2<'source> {
    pub fn source(&self) -> &str;
    pub fn root(&self) -> NodeRef<'_>;
    pub fn node(&self, id: NodeId) -> Option<NodeRef<'_>>;
    pub fn to_html(&self) -> String;
}
```

添加 `NodeRef::text() -> Option<TextView<'doc>>` 和类型化 payload 访问器。View 的生命周期不得超过对 Document 的借用。

- [ ] **步骤 5：使 HTML 渲染适配通用文档 View**

在新渲染器路径中，用 `NodeRef`、`children()`、payload 访问器和 `TextView::write_to` 替换对 `Tree<Node>` 的直接索引。保留 legacy 渲染器至任务 11。

对于普通 Text 节点，渲染器不得调用 `TextView::to_cow()`；应将源码片段和复合片段直接写入目标。

- [ ] **步骤 6：为迁移实现临时 legacy-to-v2 适配器**

该适配器把 legacy Tree 转换为紧凑存储。只有在任务 6 引入原生 v2 Block 路径之前，`ParserV2::parse()` 才可调用它。对于 legacy Text 字符串，将其字节放入共享自有字节池。将适配器置于名称明确的 `legacy_adapter` 模块中，不得把该路径计入性能声明，并在任务 11 删除它。

显式添加以下临时方法：

```rust
impl<'source> ParserV2<'source> {
    pub fn parse(self) -> DocumentV2<'source> {
        legacy_adapter::parse_and_convert(self.source, self.options)
    }
}
```

- [ ] **步骤 7：启用语义等价性测试**

添加可执行的新旧对比：

```rust
use markdown::{Parser, ParserOptions, ParserV2};
use support::semantic::{legacy_semantic_digest, v2_semantic_digest};

#[test]
fn v2_matches_legacy_semantics_for_curated_fixture() {
    let source = include_str!("../bench/fixtures/curated/_data.md");
    let options = ParserOptions::default().enabled_ofm();
    let legacy = Parser::new_with_options(source, options.clone()).parse();
    let v2 = ParserV2::new_with_options(source, options).parse();

    assert_eq!(legacy_semantic_digest(&legacy), v2_semantic_digest(&v2));
    assert_eq!(legacy.to_html(), v2.to_html());
}
```

添加以下表驱动配套测试：

```rust
#[test]
fn v2_matches_legacy_across_flavors_and_structures() {
    let cases = [
        ("commonmark", "# h\n\n- a\n- b", ParserOptions::default()),
        ("gfm", "| a |\n| - |\n| b |\n", ParserOptions::default().enabled_gfm()),
        ("ofm", "> [!note]\n> text ^id\n", ParserOptions::default().enabled_ofm()),
        ("cjk", "中文 **强调**。", ParserOptions::default().enabled_cjk_autocorrect()),
        ("html", "<div>\ntext\n</div>", ParserOptions::default()),
        ("footnote", "a[^x]\n\n[^x]: note", ParserOptions::default().enabled_ofm()),
    ];

    for (name, source, options) in cases {
        let legacy = Parser::new_with_options(source, options.clone()).parse();
        let v2 = ParserV2::new_with_options(source, options).parse();
        assert_eq!(
            legacy_semantic_digest(&legacy),
            v2_semantic_digest(&v2),
            "{name}",
        );
        assert_eq!(legacy.to_html(), v2.to_html(), "{name}");
    }
}
```

将精选语料与 markdown-it 语料对比分成独立测试，使失败能够直接定位大型测试样例的回归。

- [ ] **步骤 8：运行 View、渲染器与等价性测试**

执行：

```bash
cargo test --test v2_document
cargo test --test v2_semantic_equivalence
cargo test --test html
```

预期：通过，且 HTML 字节完全一致。

- [ ] **步骤 9：提交**

```bash
git add src/document.rs src/session.rs src/parser.rs src/lib.rs src/render/html.rs tests
git commit -m "feat: add v2 document views and source-owned parser shell"
```

---

### 任务 6：替换基于 Hash 的待处理 Inline 存储并稳定 Block 事件

**文件：**
- 新建：`src/pending.rs`
- 新建：`src/semantic/mod.rs`
- 新建：`src/semantic/reference_definition.rs`
- 新建：`src/semantic/block_id.rs`
- 修改：`src/blocks/mod.rs`
- 修改：`src/blocks/*.rs`
- 修改：`src/parser.rs`
- 修改：`src/session.rs`
- 新建：`tests/v2_pending.rs`
- 修改：`tests/link_reference_definitions.rs`
- 修改：`tests/obsidian_setup.rs`

**接口：**
- 输入：行偏移量、紧凑 `NodeId`、紧凑 AST 的 Block 操作。
- 输出：有序 `PendingInlineStore`、事件暴露前的局部结构规范化，以及共享的引用定义和 BlockId 扫描器。

- [ ] **步骤 1：编写会失败的逻辑片段测试**

```rust
use markdown::{PendingInlineStore, SegmentJoin, SourceSpan};

#[test]
fn pending_segments_preserve_removed_container_prefixes() {
    let mut ast = markdown::AstStorage::new();
    let root = ast.push_root(SourceSpan::new(0, 0).unwrap()).unwrap();
    let node = ast
        .append_child(
            root,
            markdown::NodeKind::Paragraph,
            SourceSpan::new(0, 16).unwrap(),
        )
        .unwrap();
    let mut store = PendingInlineStore::new();
    store.push_segment(
        node,
        SourceSpan::new(2, 7).unwrap(),
        SegmentJoin::None,
        markdown::LineId::new(1).unwrap(),
        3,
    ).unwrap();
    store.push_segment(
        node,
        SourceSpan::new(10, 16).unwrap(),
        SegmentJoin::SoftBreak,
        markdown::LineId::new(2).unwrap(),
        3,
    ).unwrap();

    let entry = store.get(node).unwrap();
    assert_eq!(entry.segments()[0].span(), SourceSpan::new(2, 7).unwrap());
    assert_eq!(entry.segments()[1].join_before(), SegmentJoin::SoftBreak);
}
```

- [ ] **步骤 2：运行测试并确认待处理类型无法解析**

执行： `cargo test --test v2_pending`

预期：失败。

- [ ] **步骤 3：实现有序稠密存储**

```rust
pub enum SegmentJoin {
    None,
    SoftBreak,
}

pub struct PendingSegment {
    span: SourceSpan,
    line_id: LineId,
    logical_start_column: u16,
    join_before: SegmentJoin,
    flags: SegmentFlags,
}

pub enum PendingState {
    Pending,
    Materialized,
}

pub struct PendingInline {
    node_id: NodeId,
    segments: SmallVec<[PendingSegment; 2]>,
    state: PendingState,
}

pub struct PendingInlineStore {
    entries: Vec<PendingInline>,
    node_to_entry: Vec<PackedPendingId>,
}
```

条目保持文档顺序。节点存储增长时扩展 `node_to_entry`；无需哈希即可实现 O(1) 查找。

- [ ] **步骤 4：将现有 Block 算法移植到偏移量与紧凑 AST**

保留当前基于行的 CommonMark continuation 逻辑和 matcher 优先级。用存储/view 方法替换 Tree 字段读取，并用 `PendingSegment` 替换待处理存储中保留的 `Span<'input>`。对于列表、HTML、表格、Setext 和脚注行为，继续提供受控的 Block 解除链接/重新挂载操作。

不要实现完整的 Block 下行状态机。

- [ ] **步骤 5：在稳定顶层事件之前提取引用定义**

对于每个已完成的顶层 Block：

1. 检查其局部待处理后代中是否存在完整引用定义；
2. 将定义记录到文档元数据；
3. 在事件可能暴露之前移除仅含引用的 Paragraph；
4. 保留非定义的剩余片段；
5. 然后发出顶层事件。

添加一个回归测试：visitor 记录所有已暴露 ID，并断言语义准备后每个已记录 ID 仍可访问。

- [ ] **步骤 6：将 BlockId 扫描提取到共享语义代码中**

扫描器接收待处理逻辑片段，写入 ID payload，调整剩余片段，且不创建普通 Inline 节点。完整解析和选择性解析调用同一函数。

- [ ] **步骤 7：运行 Block、引用与 OFM 测试**

执行：

```bash
cargo test --test v2_pending
cargo test --test link_reference_definitions
cargo test --test block_quotes
cargo test --test lists
cargo test --test headings
cargo test --test html
cargo test --test obsidian_setup
```

预期：通过。

- [ ] **步骤 8：对待处理存储变更进行基准测试**

在纯文本、块引用/列表、引用密集型和完整语料库上运行 legacy 与 v2 Block/待处理路径。验收标准：

- 任一基准回归不得超过 3%；
- 待处理存储的分配次数下降；
- 构建含一个或两个片段的逻辑条目时不进行分配。

- [ ] **步骤 9：提交**

```bash
git add src/pending.rs src/semantic src/blocks src/parser.rs src/session.rs tests
git commit -m "feat: add dense pending inline storage and stable block events"
```

---

### 任务 7：用索引化工作区存储替换 Rc/RefCell 分隔符链与括号链

**文件：**
- 新建：`src/inlines/workspace.rs`
- 修改：`src/inlines/mod.rs`
- 修改：`src/inlines/delimiter.rs`
- 修改：`src/inlines/bracket.rs`
- 修改：`src/inlines/tests/mod.rs`
- 修改：`bench/benches/hotspots.rs`

**接口：**
- 输入：待处理逻辑片段与解析器选项。
- 输出：可复用的 `InlineWorkspace`、`TokenId`、`DelimiterId`、`BracketId`，以及无分配的候选链。

- [ ] **步骤 1：编写会失败的工作区复用与索引测试**

```rust
#[test]
fn workspace_reuses_capacity_without_retaining_state() {
    let mut workspace = InlineWorkspace::new();
    workspace.tokenize_for_test("*a*");
    let capacities = workspace.capacities();
    workspace.reset();
    workspace.tokenize_for_test("[b](c)");

    assert!(workspace.capacities().tokens >= capacities.tokens);
    assert_eq!(workspace.delimiter_head(), None);
}
```

添加强调/链接高度嵌套的测试样例，对比 legacy 与索引化输出。

- [ ] **步骤 2：运行测试并确认工作区缺失**

执行： `cargo test inlines::tests::workspace_reuses_capacity_without_retaining_state`

预期：失败。

- [ ] **步骤 3：实现紧凑工作区 ID 与存储**

```rust
pub struct InlineWorkspace {
    tokens: Vec<InlineToken>,
    delimiters: Vec<Delimiter>,
    brackets: Vec<Bracket>,
}

pub struct Delimiter {
    prev: PackedDelimiterId,
    next: PackedDelimiterId,
    token: TokenId,
    marker: u8,
    length: u16,
    flags: DelimiterFlags,
}

pub struct Bracket {
    prev: PackedBracketId,
    token: TokenId,
    flags: BracketFlags,
    image_size: Option<(u32, Option<u32>)>,
}
```

ID 在外部使用非零 `u32`，在内部使用零哨兵值。`reset()` 清空长度和头指针，但保留容量。

- [ ] **步骤 4：在不创建 AST 标记节点的情况下移植分隔符解析**

起始标记在工作区中成为 token。成功配对会改变已解析 token 的关系；失败候选项解析为由源码支撑的字面文本。不要向正式 AST 追加标记 Text 节点。

- [ ] **步骤 5：在不进行 AST 重新挂载/移除的情况下移植括号解析**

链接和图片在解析完成前引用 token 范围。停用嵌套链接起始符时只修改工作区标志。分隔符/括号模块中不得残留 `Rc`、`RefCell`、`tree.unlink`、`tree.set_parent` 或 `tree.remove`。

- [ ] **步骤 6：运行 Inline 测试样例与复杂度测试**

执行：

```bash
cargo test inlines::tests
cargo test --test commonmark_setup
cargo test --test github_setup
cargo test --test obsidian_setup
cargo test --test z_property_tests
```

预期：通过。

- [ ] **步骤 7：对索引化链进行基准测试**

执行： `cargo bench --bench hotspots`

验收标准：

- 分隔符/链接密集型用例获得提升，或回归控制在 2% 以内；
- 纯 ASCII 回归不超过 2%；
- 每个分隔符和每个括号的分配次数为零；
- 最坏情况测试样例保持现有的线性/受保护复杂度行为。

- [ ] **步骤 8：提交**

```bash
git add src/inlines bench/benches/hotspots.rs
git commit -m "perf: replace inline rc chains with indexed workspace"
```

---

### 任务 8：添加已解析 Inline IR 与事务式 AST 提交

**文件：**
- 新建：`src/inlines/ir.rs`
- 新建：`src/inlines/commit.rs`
- 修改：`src/inlines/mod.rs`
- 修改：`src/inlines/workspace.rs`
- 修改：`src/storage/mod.rs`
- 修改：`src/session.rs`
- 修改：`bench/benches/v2_stages.rs`
- 新建：`tests/v2_inline_commit.rs`

**接口：**
- 输入：工作区 token、文本草稿、全局元数据快照，以及已配置的限制。
- 输出：`ResolvedBatch`、`MetadataEffect`、`InlineDependency`、预检，以及全有或全无提交。

- [ ] **步骤 1：编写会失败的原子性测试**

```rust
#[test]
fn node_limit_failure_commits_neither_nodes_nor_metadata() {
    let mut session = fixture_session_with_node_limit(2);
    let before_nodes = session.ast_len();
    let before_tags = session.tags().to_vec();
    let batch = fixture_batch_with_three_nodes_and_one_tag();

    let error = session.commit_inline_batch(batch).unwrap_err();

    assert!(matches!(error, ParseError::NodeLimitExceeded { .. }));
    assert_eq!(session.ast_len(), before_nodes);
    assert_eq!(session.tags(), before_tags);
}
```

- [ ] **步骤 2：运行测试并确认提交 API 缺失**

执行： `cargo test --test v2_inline_commit`

预期：失败。

- [ ] **步骤 3：实现已解析 IR**

```rust
pub struct ResolvedBatch {
    pub nodes: Vec<ResolvedNode>,
    pub effects: Vec<MetadataEffect>,
    pub dependencies: Vec<InlineDependency>,
    pub text_pieces: Vec<TextPieceDraft>,
    pub owned_text: Vec<u8>,
}

pub struct ResolvedNode {
    pub kind: NodeKind,
    pub span: SourceSpan,
    pub parent: PackedResolvedId,
    pub text: Option<TextDraft>,
    pub payload: ResolvedPayload,
}

pub enum TextDraft {
    Source(SourceSpan),
    Composite(DraftPieceRange),
    Owned(DraftOwnedSpan),
}

pub enum TextPieceDraft {
    Source(SourceSpan),
    Char(char),
    Owned(DraftOwnedSpan),
}

pub enum MetadataEffect {
    InsertTag(TextDraft),
    FootnoteReference {
        label: TextDraft,
        index: u32,
        ref_count: u32,
    },
}

pub enum InlineDependency {
    FootnoteDefinition(NodeId),
}
```

组件规范化在提交前于已解析 IR 内完成，而不是提交后对 Tree 进行修改。

- [ ] **步骤 4：实现完整预检**

在修改 Document 之前：

- 对节点数和 payload 数量执行受检加法；
- 对自有文本字节数和片段数执行受检加法；
- 校验每个父 ID/已解析 ID；
- 解析元数据容量与脚注依赖 ID；
- 为所有目标 `Vec` 预留容量。

任何失败都必须在第一次写入 AST 或元数据之前返回 `ParseError`。

- [ ] **步骤 5：实现仅追加式提交**

按父节点先于子节点的顺序提交节点，将 `ResolvedId` 映射为新的 `NodeId`，一次性建立兄弟关系，把文本草稿提交到 `TextStorage`，然后应用元数据效果。临时标记绝不进入存储。

- [ ] **步骤 6：启用阶段基准**

从 `v2_stages.rs` 移除 `compile_error!`。暴露仅供基准使用的阶段方法，并用 `black_box` 消费其结果：

```rust
ParserV2::bench_block_scan();
BlockDocument::bench_semantic_prepare();
PreparedDocument::bench_inline_resolve();
ResolvedDocument::bench_ast_commit();
DocumentV2::to_html();
```

这些钩子标记为 `#[doc(hidden)]`，并受 `bench-internals` feature gate 控制。

- [ ] **步骤 7：运行原子性、等价性与阶段编译测试**

执行：

```bash
cargo test --test v2_inline_commit
cargo test --test v2_semantic_equivalence
cargo bench --features bench-internals --bench v2_stages --no-run
```

预期：通过。

- [ ] **步骤 8：提交**

```bash
git add src/inlines src/storage src/session.rs tests/v2_inline_commit.rs bench/benches/v2_stages.rs Cargo.toml
git commit -m "feat: add transactional resolved inline commit"
```

---

### 任务 9：将 Inline 语法族迁移到文本草稿与已解析 IR

**文件：**
- 修改：`src/inlines/text.rs`
- 修改：`src/inlines/entity.rs`
- 修改：`src/inlines/newline.rs`
- 修改：`src/inlines/code.rs`
- 修改：`src/inlines/math.rs`
- 修改：`src/inlines/emoji.rs`
- 修改：`src/inlines/tag.rs`
- 修改：`src/inlines/comment.rs`
- 修改：`src/inlines/link.rs`
- 修改：`src/inlines/footnote.rs`
- 修改：`src/inlines/html.rs`
- 修改：`src/utils/cjk.rs`
- 修改：`src/utils/chinese_punctuation.rs`
- 修改：`src/utils/smart_punctuation.rs`
- 修改：`tests/v2_semantic_equivalence.rs`
- 修改：`tests/v2_text_storage.rs`

**接口：**
- 输入：`InlineWorkspace`、`TextDraft`、文档元数据快照与解析器选项。
- 输出：为每种现有语法生成完整的已解析批次，且解析期间不修改正式 AST。

- [ ] **步骤 1：迁移纯文本、换行、转义与实体**

实现以下表示规则：

- 连续且未变化的文本段 → `TextDraft::Source`；
- 源码片段之间的逻辑软换行 → 复合 `Char('\n')`；
- 对现有字符的反斜杠转义 → 被转义字符的源码 Span；
- 解码后的实体 → 生成的 `Char` 或自有片段；
- 硬换行/软换行语法 → 已解析的换行节点，而非文本。

执行：

```bash
cargo test --test paragraphs
cargo test --test blank_lines
cargo test --test escapes
cargo test --test entity
```

预期：通过。

- [ ] **步骤 2：提交基础文本语法族**

```bash
git add src/inlines/text.rs src/inlines/entity.rs src/inlines/newline.rs tests
git commit -m "feat: resolve basic inline text into source-backed drafts"
```

- [ ] **步骤 3：迁移代码、数学、emoji、标签与注释**

代码规范化和生成内容根据任务 4 的碎片化阈值使用复合草稿或自有草稿。标签元数据以 `MetadataEffect::InsertTag` 发出。不产生 AST 输出的注释只消费 token。

执行：

```bash
cargo test --test code
cargo test --test math
cargo test --test obsidian_setup
```

预期：通过。

- [ ] **步骤 4：提交自包含语法族**

```bash
git add src/inlines/code.rs src/inlines/math.rs src/inlines/emoji.rs src/inlines/tag.rs src/inlines/comment.rs
git commit -m "feat: migrate standalone inline syntax to resolved batches"
```

- [ ] **步骤 5：迁移链接、图片、自动链接、BlockId 复用与脚注**

链接/图片的子内容表示为已解析 token 范围。引用查找读取语义准备元数据。脚注引用发出元数据效果与 `InlineDependency::FootnoteDefinition`；不会立即构建最终脚注列表。

执行：

```bash
cargo test --test link_reference_definitions
cargo test --test github_setup
cargo test --test obsidian_setup
cargo test inlines::tests
```

预期：通过。

- [ ] **步骤 6：提交链接与依赖项**

```bash
git add src/inlines/link.rs src/inlines/bracket.rs src/inlines/footnote.rs tests
git commit -m "feat: resolve links and footnote dependencies transactionally"
```

- [ ] **步骤 7：迁移 Inline HTML 与组件规范化**

在已解析 IR 中构建组件嵌套。仅含空白的子节点移除以及起止配对均在提交前完成。只要输出字节不变，就保留原始 HTML 的源码范围。

执行：

```bash
cargo test --test html
cargo test --test commonmark_setup
cargo test --test obsidian_setup
```

预期：通过。

- [ ] **步骤 8：迁移智能标点与 CJK 转换**

转换逻辑消费源码 Span，并发出最小的逻辑片段序列。只有在超过选定的碎片化阈值后才回退到自有文本。即使显示文本发生变化，也要保留原始 Markdown `source_span`。

执行：

```bash
cargo test --test cjk_friendly
cargo test --test smart_punctuation_test
cargo test --test z_property_tests
```

预期：通过。

- [ ] **步骤 9：运行完整语义等价性测试与分配测量**

执行：

```bash
cargo test --all-features
cargo test --test v2_semantic_equivalence
cargo bench --bench alloc_count
cargo bench --features bench-internals --bench v2_stages
```

验收标准：

- 语义与 HTML 输出等价；
- 打印源码/复合/自有文本指标；
- 分配次数已实质性接近降低 50% 的目标；
- `src/inlines/` 中没有任何语法族残留 Tree 修改调用。

- [ ] **步骤 10：提交 HTML/CJK 完成状态**

```bash
git add src/inlines src/utils tests
git commit -m "feat: complete resolved inline syntax migration"
```

---

### 任务 10：添加选择性物化与同进程中断/恢复

**文件：**
- 修改：`src/session.rs`
- 修改：`src/parser.rs`
- 修改：`src/document.rs`
- 修改：`src/semantic/mod.rs`
- 修改：`src/inlines/mod.rs`
- 新建：`tests/v2_selective.rs`
- 新建：`tests/v2_resume.rs`
- 修改：`tests/headings.rs`
- 修改：`tests/obsidian_setup.rs`

**接口：**
- 输入：已完成的 Block 会话、语义元数据、待处理条目，以及共享 Inline 引擎。
- 输出：`VisitControl`、`BlockScanOutcome`、`BlockBreak`、`BlockDocument`、`PreparedDocument`、`InlineSelection`，以及语义遍历/恢复能力。

- [ ] **步骤 1：编写会失败的公共工作流测试**

```rust
use markdown::{BlockScanOutcome, InlineSelection, ParserV2, VisitControl};

#[test]
fn break_resume_matches_uninterrupted_parse() {
    let source = "# a\n\nbody\n\n# b";
    let expected = ParserV2::new(source).parse().to_html();
    let outcome = ParserV2::new(source)
        .parse_blocks_with_checked(
            |_| true,
            |_| VisitControl::Break,
        )
        .unwrap();

    let interrupted = match outcome {
        BlockScanOutcome::Break(value) => value,
        BlockScanOutcome::Complete(_) => panic!("expected interruption"),
    };
    let completed = interrupted.resume_with_checked(|_| true, |_| VisitControl::Continue).unwrap();
    let document = completed.prepare_semantics_checked().unwrap().materialize_all_checked().unwrap();

    assert_eq!(document.to_html(), expected);
}
```

- [ ] **步骤 2：运行测试并确认工作流类型缺失**

执行： `cargo test --test v2_resume`

预期：失败。

- [ ] **步骤 3：实现 Block 中断所有权**

```rust
pub enum VisitControl {
    Continue,
    Break,
}

pub enum BlockScanOutcome<'source> {
    Complete(BlockDocument<'source>),
    Break(BlockBreak<'source>),
}

pub struct BlockBreak<'source> {
    core: ParserCore<'source>,
    next_event_cursor: u32,
}

impl<'source> BlockBreak<'source> {
    pub fn resume_with_checked<P, V>(
        self,
        predicate: P,
        visitor: V,
    ) -> Result<BlockScanOutcome<'source>, ParseError>;

    pub fn finish_prefix_checked(self) -> Result<BlockDocument<'source>, ParseError>;
}
```

`BlockBreak` 持有同一份源码、AST `Vec`、待处理存储、扫描器偏移量、行索引和元数据。它不实现 `Clone`；恢复操作会消费它。

- [ ] **步骤 4：实现语义准备与选择阶段类型**

```rust
pub struct BlockDocument<'source> { /* owns ParserCore */ }
pub struct PreparedDocument<'source> { /* owns ParserCore + target index */ }

pub struct InlineSelection {
    selected: Vec<bool>,
}

impl InlineSelection {
    pub fn select(&mut self, node_id: NodeId) -> Result<(), ParseError>;
    pub fn is_empty(&self) -> bool;
}
```

准备阶段扫描已接受前缀中的引用定义，提取每个 BlockId，精确物化所有 Heading Inline 一次，并建立前序目标索引。

- [ ] **步骤 5：实现语义遍历中断**

语义中断保存下一个目标游标和当前选择。它可以使用新的 visitor 恢复，也可以使用当前选择结束。中断不会隐式选择当前事件。

- [ ] **步骤 6：实现已选子树与脚注依赖扩展**

使用以 NodeId 为索引的位集。按文档顺序把已选容器扩展到待处理 Inline 后代。通过去重的 NodeId 队列处理脚注定义。空选择会跳过普通正文 Inline 解析。

- [ ] **步骤 7：添加中断矩阵测试**

覆盖以下情况：

- 在每个合法顶层边界中断并恢复；
- EOF 前发生多次中断；
- 完成前缀的结果等价于解析已接受源码前缀；
- 语义中断从下一个目标恢复；
- 在目标 visitor 运行前物化 Heading Inline；
- 同时选择祖先与后代时不执行重复工作；
- 空选择使普通正文节点保持 `Pending`；
- 被引用的脚注定义成为已物化依赖；
- 源码与 AST 均不被克隆。

- [ ] **步骤 8：运行选择性解析与恢复测试**

执行：

```bash
cargo test --test v2_selective
cargo test --test v2_resume
cargo test --test headings
cargo test --test obsidian_setup
cargo test --test link_reference_definitions
```

预期：通过。

- [ ] **步骤 9：对回调与选择性开销进行基准测试**

验收标准：

- 不使用 visitor 的完整解析不执行回调分派；
- 始终返回 Continue 的顶层 visitor 相较同一 Block 模式增加的开销不超过 5%；
- 中断/恢复不会重新扫描已接受源码；
- 当标题不占主导时，在专用语料库中选择约 1% 正文的耗时低于完整解析的 50%。

- [ ] **步骤 10：提交**

```bash
git add src/session.rs src/parser.rs src/document.rs src/semantic src/inlines tests
git commit -m "feat: add selective inline materialization and resumable breaks"
```

---

### 任务 11：切换公共 Rust、渲染器、Serde 与 WASM API

**文件：**
- 修改：`src/lib.rs`
- 修改：`src/parser.rs`
- 修改：`src/document.rs`
- 修改：`src/storage/view.rs`
- 修改：`src/render/html.rs`
- 删除：`src/tree.rs`
- 修改：`wasm-binding/src/lib.rs`
- 修改：`wasm-binding/src/types.rs`
- 修改：`tests/*.rs`
- 修改：`examples/data/src/main.rs`
- 修改：`README.md`

**接口：**
- 输入：已经验证的 v2 引擎与临时 `ParserV2` 外观接口。
- 输出：最终的 `Parser`、`Document`、`NodeId`、`NodeRef`、持有源码的 WASM Document、兼容的 TypeScript AST 输出，并彻底移除 legacy 热路径。

- [ ] **步骤 1：为 View 生命周期添加编译失败文档**

记录并测试以下代码：

```rust,compile_fail
let node = document.node(id).unwrap();
let resumed = interrupted.resume_with_checked(predicate, visitor)?;
use_node(node);
```

无法编译，因为对 View 的借用跨越了会消费会话的恢复操作。

- [ ] **步骤 2：重命名 v2 外观接口，仅在语义仍有效时保留已弃用构造函数**

- `ParserV2` 更名为 `Parser`。
- `DocumentV2` 更名为 `Document`。
- 保留 `Parser::new(&str)`。
- 添加 `Parser::from_string(String)`。
- 移除对 `Tree<Node>` 字段的直接访问。
- 文本和 payload 访问统一通过 `NodeRef`。

不要保留 `MarkdownNode::Text(String)` 的直接字段兼容性。

- [ ] **步骤 3：将所有集成测试移植到 View**

将直接索引和对 `.body` 字段的匹配替换为：

```rust
let node = document.node(id).unwrap();
assert_eq!(node.kind(), NodeKind::Heading);
assert_eq!(node.text().unwrap().to_cow(), "title");
```

保持语义预期不变。

- [ ] **步骤 4：将 WASM 移植到自有源码与 View 遍历**

`parse_with_options` 将 JavaScript 字符串移动到 `Parser::from_string_with_options`。AST 转换递归访问 `NodeRef`、解析惰性位置，并且只在绑定边界物化 JavaScript 字符串。保持 TypeScript AST、HTML、标签、元数据以及仅 frontmatter 行为兼容。

WASM 不导出选择性 visitor、`BlockBreak` 或可恢复解析器会话。

- [ ] **步骤 5：通过同一组访问器移植 Serde 与 HTML**

两条路径都不得直接检查内部 arena `Vec`。文本序列化可以在序列化器边界分配连续字符串；HTML 必须使用 `TextView::write_to`。

- [ ] **步骤 6：移除临时路径与 legacy 路径**

删除：

- legacy Tree 驱动解析器实现；
- 最终等价性快照通过后，删除仅供测试使用的 legacy 适配器；
- `Tree<Node>` 渲染器；
- `ParserV2`/`DocumentV2` 名称；
- 空闲槽位 HashSet 热路径；
- 直接操作 AST 的测试辅助函数。

保留语义测试样例读取器，不保留可执行的 legacy 解析器代码。

- [ ] **步骤 7：运行完整 Rust 与 WASM 测试矩阵**

执行：

```bash
cargo test --workspace --all-features
cargo check -p markdown-binding --target wasm32-unknown-unknown
cargo test --doc
cargo test --test v2_semantic_equivalence
```

预期：通过。

- [ ] **步骤 8：提交**

```bash
git add src wasm-binding tests examples README.md
git rm src/tree.rs
git commit -m "feat!: switch public parser and bindings to compact v2 engine"
```

---

### 任务 12：应用由性能剖析门槛控制的 Block 分派优化

**文件：**
- 修改：`src/span.rs` or its v2 replacement
- 修改：`src/blocks/mod.rs`
- 修改：`src/blocks/*.rs`
- 修改：`bench/benches/hotspots.rs`
- 修改：`bench/benches/v2_stages.rs`
- 新建：`tests/v2_block_dispatch.rs`

**接口：**
- 输入：当前基于行的 CommonMark Block 算法。
- 输出：一次性 `LineHead` 计算，以及基于上下文/风格/首字节的候选分派。不会生成新的 Block 状态机。

- [ ] **步骤 1：添加分派等价性测试**

对于每个特殊首字节（`#`、反引号、`~`、`-`、`*`、`_`、`+`、`=`、`<`、`>`、`|`、`:`、`[`、数字），在默认、GFM、OFM 和 JSXLike 选项，以及 Paragraph/非 Paragraph 上下文中，将优化后的分派器与仅供测试使用的穷举分派器进行比较。

- [ ] **步骤 2：实现 `LineHead`**

```rust
pub struct LineHead {
    pub line_start: u32,
    pub line_end: u32,
    pub first_nonspace: u32,
    pub indent_bytes: u16,
    pub indent_columns: u16,
    pub first_byte: Option<u8>,
    pub is_blank: bool,
    pub is_ascii: bool,
}
```

在提取行时仅计算一次，并将其复用于 matcher 分派和行索引构建。

- [ ] **步骤 3：实现候选掩码**

根据首字节、段落上下文、容器类型和已启用风格进行分派。在存在歧义的掩码中，保持现有精确优先级：

- `-`/`*`：在当前语义要求的位置，Setext 或 thematic break 优先于列表；
- `~`：围栏代码与 OFM 特定行为；
- `>`：标注块与块引用；
- `<`：HTML 变体；
- `|`：仅当前方存在可转换 Paragraph 时匹配表格；
- `[` 与 `:`：仅考虑已启用的扩展候选项；
- 至少四列缩进：缩进代码块。

- [ ] **步骤 4：运行正确性测试**

执行：

```bash
cargo test --test v2_block_dispatch
cargo test --test commonmark_setup
cargo test --test github_setup
cargo test --test obsidian_setup
cargo test --test html
cargo test --test lists
cargo test --test headings
```

预期：通过。

- [ ] **步骤 5：运行保留门槛评估**

执行：

```bash
cargo bench --bench hotspots
cargo bench --features bench-internals --bench v2_stages
```

只有满足以下条件时才保留该优化：

- block-heading、fence、reference、thematic-break、blockquote、list、HTML 与 table 语料库的几何平均性能至少提升 10%；
- 任一完整语料库纯解析结果的回归不超过 2%；
- 正确性测试样例无变化。

若门槛未通过，仅保留能够独立满足同一无回归规则的 `LineHead` 变更，并在最终提交中省略候选掩码。

- [ ] **步骤 6：提交通过验收的部分**

```bash
git add src/blocks src/span.rs bench tests/v2_block_dispatch.rs
git commit -m "perf: predispatch block candidates from shared line heads"
```

v2A 不实现完整的 Block 下行状态机。只有当 v2A 完成后的性能剖析显示 Block 占纯解析时间的 25–30% 或更高，或 Block 密集型语料阻碍达到 1.5 倍目标时，才在 v2B 中重新评估。

---

### 任务 13：执行 v2A 正确性、内存与性能发布门槛

**文件：**
- 修改：`bench/compare/README.md`
- 修改：`README.md`
- 修改：`docs/superpowers/specs/2026-07-20-markdown-parser-performance-architecture-design.md`
- 新建：`bench/results/v2a-reference.md`

**接口：**
- 输入：最终 v2A 解析器与所有基准通道。
- 输出：可审计的验收记录，以及达到发布就绪状态的 v2A 分支。

- [ ] **步骤 1：运行格式化、lint、测试、文档与 WASM 检查**

执行：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --doc
cargo check -p markdown-binding --target wasm32-unknown-unknown
```

预期：每条命令的退出码均为 0。

- [ ] **步骤 2：分别运行语义测试样例与属性测试套件**

执行：

```bash
cargo test --test commonmark_setup
cargo test --test github_setup
cargo test --test obsidian_setup
cargo test --test cjk_friendly
cargo test --test z_property_tests
cargo test --test v2_semantic_equivalence
cargo test --test v2_selective
cargo test --test v2_resume
```

预期：通过；预期 HTML 字节完全一致，且没有被忽略的 v2 等价性测试。

- [ ] **步骤 3：运行分配与阶段测量**

执行：

```bash
cargo bench --bench alloc_count
cargo bench --features bench-internals --bench v2_stages
cargo bench --bench text_storage
```

要求：

- `_data.md` 每次解析的分配次数至少比记录的基线低 50%；
- 不使用内存泄漏、`mem::forget`、跳过 drop 或未观察结果等手段；
- 记录源码/复合/自有节点及输出字节比例；
- 分别报告解析、提交、渲染与释放。

- [ ] **步骤 4：运行等价比较器通道**

执行： `cargo bench -p parser-compare-bench --bench parser_compare`

要求：

- CommonMark 与共享 GFM 的完整 AST 纯解析速度不得慢于 pulldown 完整事件消费的 2 倍；
- 在记录的参考机器上，OFM 产品语料库中位数不高于 5.0 ms；
- 分别报告纯解析与解析加 HTML；
- 不得把 OFM 表述为与 pulldown 等价的比较。

- [ ] **步骤 5：编写验收记录**

应包含：

- 提交 SHA；
- CPU 与操作系统；
- `rustc -Vv`；
- 构建 profile 与 feature；
- 语料库 SHA/字节长度；
- 中位数与置信区间；
- 分配/重新分配/释放的次数与字节数；
- 文本表示形式比例；
- Node/头部大小；
- 每项 v2A 门槛的通过/失败状态；
- 所有 v2B 后续性能剖析证据。

- [ ] **步骤 6：更新文档，但不重写历史设计文本**

在性能规范中追加实施结果作者注。说明胜出的 payload 布局和文本碎片化阈值、Block 候选掩码是否通过门槛，以及实测 v2A 结果。不要静默修改之前的设计或可行性作者注。

- [ ] **步骤 7：提交验收记录**

```bash
git add bench/compare/README.md bench/results README.md docs/superpowers/specs/2026-07-20-markdown-parser-performance-architecture-design.md
git commit -m "docs: record v2a performance architecture acceptance"
```

- [ ] **步骤 8：请求最终代码审查**

使用 `superpowers:requesting-code-review`，并要求审查以下内容：

- 语义等价性；
- 不安全 UTF-8 边界假设；
- 稳定 ID 与 tombstone 行为；
- 提交原子性；
- 选择/依赖去重；
- 中断所有权；
- WASM 边界分配；
- 基准公平性与观测结果。

在审查意见全部解决且步骤 1–4 的每条命令均已重新成功运行之前，不得宣布 v2A 完成。

---

## 规范覆盖矩阵

| 规范要求 | 实施任务 |
| --- | --- |
| 等价 CommonMark/共享 GFM 与 OFM 产品基准 | 任务 1 和 13 |
| 借用型/自有型源码，且不复制完整输入 | 任务 2、5 和 11 |
| 稳定紧凑 ID 与关系存储 | 任务 3 |
| 源码/复合/自有文本与直接片段渲染 | 任务 4、5 和 9 |
| 惰性行/列位置 | 任务 2、5 和 11 |
| 有序的待处理 Inline 逻辑片段 | 任务 6 |
| 引用定义与 BlockId 语义准备 | 任务 6 和 10 |
| 索引化分隔符/括号工作区 | 任务 7 |
| 已解析 Inline IR、元数据效果、依赖与原子提交 | 任务 8 和 9 |
| 完整/选择性共享流水线与 Heading 准备 | 任务 10 |
| 同进程 Break/恢复，且不克隆源码或 AST | 任务 10 |
| Rust API、Serde、渲染器与自有源码 WASM 迁移 | 任务 11 |
| 由性能剖析门槛控制的 Block 候选分派，无需重写状态机 | 任务 12 |
| 分配、正确性、内存、释放与最终性能门槛 | 任务 13 |
| 不使用通用 arena、直接 HTML 后端、跨进程恢复或磁盘检查点 | 全局约束与任务 13 审查 |

没有任何 v2A 规范要求被有意推迟。v2B 的 1.5 倍硬目标、1.2 倍进阶目标、完整 Block 状态机、PGO、目标平台特定调优，以及可能重新评估通用 arena，均不在本实施计划范围内。

---

## 执行检查点

在任务 1、4、6、8、10、11 和 13 后暂停并进行审查。每个检查点必须包含：

- `git show --stat --oneline HEAD`;
- 实际运行的精确测试及其退出状态；
- 受影响阶段的前后基准中位数；
- 分配变化；
- 语义等价性状态；
- 对本计划的任何已接受偏差及其支持证据。

关键路径如下：

```text
Baseline
  → Source/location primitives
  → Compact storage
  → Text storage
  → Document/views
  → Pending/Block semantics
  → Indexed Inline workspace
  → Resolved IR/commit
  → Syntax migration
  → Selective/resume
  → Public/WASM cutover
  → Profile-gated Block dispatch
  → v2A acceptance
```

不得绕过失败的正确性或原子性门槛来追求性能数字。未通过既定保留门槛的性能假设应被移除，而不是隐藏在更大的提交中。
