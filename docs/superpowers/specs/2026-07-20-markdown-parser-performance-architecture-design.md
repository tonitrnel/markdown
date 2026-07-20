# Markdown Parser v2 性能架构设计

## 概述

本设计定义下一 major version 的 Markdown 解析器性能架构，并把交付分为 v2A 架构落地版和 v2B 深度优化版。在保留完整 CommonMark、GFM、OFM、CJK、位置、引用和脚注语义的前提下，v2A 把完整 AST 解析相对 `pulldown-cmark` 完整事件消费的性能差距稳定压缩到 2 倍以内；v2B 进一步把 1.5 倍作为硬性验收线，并以 1.2 倍作为冲刺目标。

核心方案是一条统一的紧凑解析流水线：源码由 `Document` 持有或借用；Block、待处理 Inline 和最终 AST 使用连续 typed arenas；Inline delimiter 和 bracket 使用整数索引；普通文本优先保存源码范围；Inline 在独立 workspace 中完成解析后一次性提交正式 AST。

完整解析和选择性解析共用这条流水线。选择性解析不是完整解析后的附加过滤器，而是语义准备之后决定哪些 pending Inline 需要物化的正式执行模式。本设计同时为同进程、内存中的中断与恢复定义所有权边界，但不支持 checkpoint 序列化、跨进程恢复、落盘恢复或修改源码后的恢复。

## 背景与证据

当前本地 quick benchmark 的代表性结果如下，具体数值依赖机器：

| 数据集与操作 | 本项目 | `pulldown-cmark` | 差距 |
| --- | ---: | ---: | ---: |
| default data，parse only | 约 1.51 ms | 约 0.51 ms | 约 3.0 倍 |
| 570,143-byte corpus，parse only | 约 8.82 ms | 约 2.49 ms | 约 3.5 倍 |
| default data，parse + HTML | 约 2.19 ms | 约 0.77 ms | 约 2.9 倍 |
| 570,143-byte corpus，parse + HTML | 约 11.66 ms | 约 3.07 ms | 约 3.8 倍 |

259 KB `_data.md` 的 allocation-count benchmark 每次解析约发生 20,571 次常规分配；不含 realloc 新容量时，常规分配请求约 2.89 MB。采样显示主要成本集中在：

- `inlines::process` 及其文本累计和 flush；
- delimiter 收尾和链表操作；
- `Span` 提取、合并与位置计算；
- Tree 节点添加、删除、unlink 和 reparent；
- HashMap 插入和扩容。

当前 Inline delimiter 和 bracket 通过 `Rc<RefCell<_>>` 链维护。解析过程中还会先把 marker 写成临时 Text 节点，再通过 Tree 的 `remove`、`unlink` 和 `set_parent` 解析结构。这些行为同时增加分配次数、引用计数、动态借用检查、随机内存访问和无效 AST 写入。

当前比较并非完全等价的产物比较：本项目构建并销毁完整 AST，而 `pulldown-cmark` 只完整消费事件；HTML 比较中，本项目先构建 AST 再遍历渲染，而 `pulldown-cmark` 直接把事件写成 HTML。因此 `pulldown-cmark` 是行业速度参考，不代表相同输出成本。即便如此，3 至 4 倍差距仍足以说明当前实现存在需要解决的结构性成本。

## 目标

- v2A 完整 AST 解析在主要大型 corpus 上不高于同次运行 `pulldown-cmark` 完整事件消费的 2 倍。
- v2A 在当前参考机器上，把 570,143-byte corpus 的 parse-only 中位数从约 8.82 ms 降到不高于 5.0 ms。
- v2B 完整 AST 解析在主要大型 corpus 上不高于同次运行 `pulldown-cmark` 完整事件消费的 1.5 倍，并以 1.2 倍作为冲刺目标。
- v2B 在相同 CommonMark/GFM AST 语义和选项下争取超过 `rushdown` 的完整 AST 路径。
- 把 259 KB `_data.md` 每次完整解析的分配次数至少降低 50%。
- 完整解析和选择性解析共用 Block、语义准备和 Inline 引擎。
- 普通 Text 和其他未转换内容优先使用 source range，避免碎片化字符串分配。
- delimiter、bracket 和临时 Inline token 不进行逐项堆分配。
- Inline 解析成功后批量提交 AST，不在解析过程中反复修改正式 Tree。
- Block Node ID 创建后在语义准备、选择、Inline 物化和恢复过程中保持稳定。
- 支持同进程内在稳定事件边界中断、继续或接受当前结果，不复制已构建 AST。
- 保持现有 CommonMark、GFM、OFM、CJK、HTML、位置、reference、BlockId 和 footnote 行为。
- 保留现有选择性解析设计的事件时机、目标语义、选择规则和依赖规则。

## 非目标

- 在 v2A 或 v2B 引入通用 bump allocator，除非满足本设计后述的重新评估条件。
- 在 v2A 或 v2B 实现不构建 AST 的直接流式 HTML 后端。
- Parser checkpoint 序列化。
- 跨进程恢复或落盘恢复。
- 修改源码后从旧状态增量恢复。
- 并行解析单个文档。
- 通过删除语法、位置或元数据能力改善 benchmark。
- 在总体架构 spec 中锁死所有公开阶段类型的名称和具体方法布局。
- 保持下一 major version 的 Rust AST 直接字段访问兼容性。

## 设计方案

### 采用方案：统一的 v2 紧凑解析流水线

```text
SourceBuffer
    ↓
Block Phase
    ├─ Compact Block Arena
    ├─ Pending Inline Store
    └─ finalized top-level block events / interruption
    ↓
Semantic Preparation
    ├─ reference definitions
    ├─ OFM BlockId extraction
    └─ all Heading Inline materialization
    ↓
Inline Selection
    ├─ Full mode: all pending Inline
    └─ Selective mode: explicitly selected subtrees
    ↓
Inline Engine
    ├─ reusable token arena
    ├─ delimiter/bracket index stacks
    ├─ dependency expansion
    └─ resolved Inline batches
    ↓
Append-only Compact AST commit
    ↓
Document
```

这条流水线使完整解析和选择性解析共享所有语法实现。完整解析等价于选择全部 pending Inline 的快捷路径，但不需要注册 visitor，也不承担事件分派成本。

### 未采用：渐进保留现有 Tree 约束

可以先单独把 `Rc<RefCell>` 改为 Vec，再逐项替换 pending map、Text 和 Tree。但如果继续允许 Inline parser 边解析边修改正式 Tree，中间实现仍需维护临时 marker 节点、free-node 集合和 reparent 逻辑，随后又会被延迟提交模型删除。这种路径适合实施时分步验证，不适合作为最终架构。

### 暂不采用：事件内核与 AST/HTML 双后端

事件内核可以让 HTML renderer 不构建 AST，更接近 `pulldown-cmark` 的端到端路径。但 OFM、reference、footnote、BlockId 和需要回看或重排的 Inline 语义会显著增加事件协议复杂度。v2A 和 v2B 优先解决完整 AST 性能；架构不得阻止未来从 resolved Inline batch 增加直接 HTML consumer。

## Source 所有权

`Document` 持有或借用唯一一份原始源码：

```rust
pub struct Document<'source> {
    source: SourceText<'source>,
    ast: AstStorage,
    line_index: LineIndex,
    metadata: DocumentMetadata,
}

enum SourceText<'source> {
    Borrowed(&'source str),
    Owned(String),
}
```

Rust API 至少支持两类构造语义：

```rust
Parser::new(&str)              // 借用 source
Parser::from_string(String)    // 获取 source 所有权
```

具体返回类型名称可以在 API 原型阶段调整，但必须满足：

- borrowed source 不复制完整输入；
- owned source 能生成不借用调用方的 Document；
- WASM binding 使用 owned source；
- paused parser 和最终 Document 使用同一 source，不复制源码；
- source 在 parser session、NodeRef 或 DocumentView 的借用仍有效时不得被替换。

## 紧凑 AST 存储

### Typed arenas

正式 AST 使用项目内部的连续 typed arenas，而不是通用 arena allocator：

```rust
struct AstStorage {
    nodes: Vec<Node>,
    links: Vec<LinkData>,
    images: Vec<ImageData>,
    code: Vec<CodeData>,
    owned_text: Vec<String>,
}
```

具体 secondary arena 可按实际 NodeKind 大小和 profile 结果调整。必须遵守以下原则：

- 节点通过整数 ID 引用；
- 同类 payload 连续存储；
- 不为 delimiter、bracket 或临时 Inline token 单独分配；
- 不为了使用 arena 而把普通 Rust Drop、Serde 或 WASM 生命周期复杂化；
- 通用 bump allocator 只有在主要结构改造完成后、profile 仍显示 allocator 为显著热点且 corpus 改善至少约 5% 时才重新评估。

### Node ID 与关系索引

内部 Node ID 和 source offset 使用紧凑整数类型：

```rust
pub struct NodeId(u32);

struct PackedNodeId(u32); // 保留一个值作为无节点 sentinel

struct Node {
    kind: NodeKind,
    source_span: SourceSpan,
    parent: PackedNodeId,
    first_child: PackedNodeId,
    last_child: PackedNodeId,
    next_sibling: PackedNodeId,
    payload: PayloadId,
    flags: NodeFlags,
}
```

内部关系字段使用保留 sentinel 的 packed ID，不使用 `Option<usize>`。公开 `NodeId` 只表示有效节点，不能构造或观察 sentinel。输入长度、节点数或 payload 数超过内部 ID 可表达范围时，checked API 返回明确错误。

Block 节点在 Block Phase 创建后不可重编号。Inline 节点在后续阶段追加，因此底层 Vec 的重分配不得影响 Node ID。

### 不再以空槽表达删除

最终 AST 不应依赖 `Option<Node>` 与 `FxHashSet<usize>` 表示 free nodes。Inline token resolution 在 workspace 中完成，只有最终存活的节点才提交正式 AST。若 Block 算法仍存在少量结构替换需求，应使用明确的节点状态或受控 free list，并由 benchmark 证明其必要性。

## Source range 与文本

### SourceSpan

节点热数据保存 byte range：

```rust
pub struct SourceSpan {
    start: u32,
    end: u32,
}
```

范围采用半开区间 `[start, end)`，必须位于当前 Document source 内，并落在 UTF-8 合法边界。节点即使显示文本经过转换，仍保存覆盖原始 Markdown 的 source span。

### TextRef

```rust
enum TextRef {
    Source(SourceSpan),
    Owned(OwnedTextId),
}
```

以下内容优先使用 `Source`：

- 普通 Text；
- 未转换的 code 内容；
- 原始 HTML；
- 未转换的 URL、title 和 label；
- 其他连续且可直接引用输入的字符串。

只有以下情况创建 owned text：

- entity 解码；
- backslash unescape；
- smart punctuation 或 CJK 转换；
- 需要拼接不连续 source segments；
- reference normalization；
- parser 生成、输入中不存在的内容。

owned text 的具体 pool 形式可由实现和 profile 决定，但不得退化为所有 Text 节点无条件创建 String。

### 公开访问

下一 major version 不再要求保留 `MarkdownNode::Text(String)` 直接字段访问。公开读取通过绑定 Document 的只读视图完成：

```rust
let node = document.node(node_id);
node.kind();
node.source_span();
node.text();
node.parent();
node.children();
node.inline_state();
```

`NodeRef` 负责解析 source range、payload ID 和 owned text。HTML renderer、Serde 和 WASM binding 必须使用相同访问层，避免各自依赖内部 arena 布局。

## 位置模型

节点热数据不固定保存完整起止行列号。Block 扫描期间构建 line-start index，Document 按需把 byte offset 转换为行列号：

- 行号通过 line-start index 二分或缓存查询；
- ASCII 行的列号由 byte offset 直接得到；
- 非 ASCII 行只在请求位置时计算字符列；
- 同一 offset 的重复位置查询可以缓存，但缓存不得增加每个 Node 的固定热数据尺寸；
- HTML、语义遍历和不读取位置的调用者不承担完整行列转换成本。

公开位置 API 必须继续使用从 1 开始的行列语义，并通过现有位置 fixture 验证。实现可以提供 byte span 作为更廉价的主 API。

## Pending Inline Store

Block Phase 按文档顺序记录能够接收 Inline 的 Block 和 source segments。不得继续以 `FxHashMap<NodeId, Vec<Span>>` 作为主要存储。

概念结构如下：

```rust
struct PendingInline {
    node_id: NodeId,
    segments: SmallVec<[SourceSpan; 2]>,
    state: PendingState,
}

struct PendingInlineStore {
    entries: Vec<PendingInline>,
    node_to_entry: DenseIndex,
}
```

要求：

- entries 使用文档顺序；
- 单行和双 segment Inline 不进行额外 Vec 分配；
- node lookup 使用 dense index、节点旁路字段或等价的 O(1) 紧凑结构；
- BlockId 提取通过调整 segments 或记录排除范围实现，不修改 source；
- consumed Heading 不会在普通正文阶段重复处理；
- 部分物化 Document 能区分 `Pending` 和 `Materialized`。

不接受 Inline 的 Block 不需要公开虚假的 InlineState。具体可通过 NodeFlags 或 NodeRef 条件访问表达。

## Inline Engine

### Reusable workspace

Inline Engine 使用在容器之间复用 capacity 的 workspace：

```rust
struct InlineWorkspace {
    tokens: Vec<InlineToken>,
    delimiters: Vec<Delimiter>,
    brackets: Vec<Bracket>,
    resolved: Vec<ResolvedInline>,
}
```

处理下一个 pending Inline 前清空长度但保留 capacity。workspace 不属于最终 Document；暂停发生在顶层 Block 或语义目标边界时，不要求保留正在解析一半的 Inline workspace。

### Delimiter 与 bracket

delimiter 和 bracket 的 `prev`、`next` 使用 workspace index，不使用 `Rc<RefCell<_>>`：

```rust
struct Delimiter {
    prev: Option<DelimiterId>,
    next: Option<DelimiterId>,
    node: TokenId,
    // delimiter byte, length, flags, position...
}
```

Option 的实际紧凑表示可使用 sentinel 或 NonZero ID。实现不得因每个 opener/closer 触发系统堆分配。

### 延迟提交

每个 Inline 容器依次执行：

```text
source segments
  → tokenization
  → delimiter/bracket/link resolution
  → resolved batch
  → capacity/limit preflight
  → append-only AST commit
```

tokenization 和 resolution 不修改正式 AST。提交前必须计算本批次新增节点和 payload 数，先执行 node/payload limit 检查和必要 reserve，避免提交中途失败留下半棵 Inline 子树。

正式提交按最终父子顺序追加节点并建立关系。marker、失败的 delimiter 候选和临时 bracket 不进入 AST，因此不需要事后 remove、unlink 或 reparent。

## Block Phase

Block Phase 继续使用逐行 CommonMark Block 算法，但输出 Compact Block Arena 和 Pending Inline Store。

优化实现应评估：

- 按首个非空白字节缩小 Block matcher 候选集合；
- 减少完整 Span 复制和重复 snapshot；
- line-start index 与行提取共用换行扫描；
- source offset 作为热路径位置表示；
- 保持现有 lazy continuation、HTML、table、callout 和 footnote 行为。

这些是受 profile 驱动的实现选择，不得改变 matcher 优先级和语法结果。

## 选择性解析整合

### 与现有设计的关系

本设计依赖并纳入：

`docs/superpowers/specs/2026-07-19-selective-inline-events-design.md`

现有设计中的以下语义继续有效：

- 只为 finalized 的 Document 直接子节点产生顶层 Block 事件；
- frontmatter 和 Document 自身不产生顶层 Block 事件；
- Heading 和带 OFM BlockId 的节点是语义目标；
- Heading 与 BlockId 同时存在时只产生一个目标；
- 所有 Heading Inline 在语义目标遍历前完整物化；
- reference definitions 覆盖整个已接受 Block 前缀；
- selection 选择节点及其所有可接收 Inline 的后代；
- selection 与事件遍历控制相互独立；
- 选中内容引用的 footnote definition 自动成为依赖；
- 空 selection 跳过普通正文 Inline；
- 语义目标按文档前序遍历。

本设计覆盖现有设计中的以下实现假设：

- pending Inline 不再以 HashMap 为主要存储；
- Tree 替换为紧凑 typed arenas；
- Text 优先使用 source range；
- Inline 通过 workspace 解析后批量提交；
- 终态 `Stop` 改为可继续或可终结的中断 outcome；
- Node 和 Tree 的直接字段 API 改为 Document-bound views；
- 未选中的 Inline 节点显式暴露 pending 状态。

现有 selective-inline spec 后续需要单独修订公共 API 示例和测试名称，但其语义规则无需另起一套实现。

### 统一阶段

选择性解析内部包含四个阶段：

1. Block scan 与顶层事件；
2. Semantic preparation；
3. Semantic target traversal 与 selection；
4. Selected Inline materialization。

总体架构要求这些状态边界存在，但不在本设计中锁死所有公开 Rust 类型名称。API 原型可以选择少量 phase-specific types 或一个受控 facade；无论采用哪种形式，都必须阻止调用者在语义准备前访问完整 Heading AST，或在 materialization 后继续修改 selection。

### Semantic preparation

Block Phase 完成或接受当前前缀后，固定执行：

1. 扫描已接受前缀中的 reference definitions；
2. 从全部 pending Inline segments 提取 OFM BlockId；
3. 使用共享 Inline Engine 物化所有 Heading；
4. 建立按文档前序排列的 semantic target index。

Heading 完成后从 pending 集合中标记为 consumed，不能在选择普通正文时重复物化。

### Selection

`InlineSelection` 使用 NodeId-indexed bitset 或等价的紧凑去重结构，不使用按字符串或 NodeId 哈希的集合。选择容器时，在物化前展开为其所有 pending Inline 后代。选择祖先和后代不得重复解析。

未选中的 Inline-capable Block 保留 Block 结构并标记为 `Pending`。已物化的空内容标记为 `Materialized`，调用者可以明确区分两种状态。

### Footnote 依赖

选中内容生成 footnote reference 时，把对应 definition 加入 NodeId 去重工作队列。依赖按文档顺序处理并使用相同 Inline Engine。未选择且未被引用的 definition 不物化 Inline，也不进入生成的 FootnoteList。

## 中断与同进程恢复

### 控制语义

visitor 只回答当前调用是否继续：

```rust
pub enum VisitControl {
    Continue,
    Break,
}
```

`Break` 返回拥有当前 parser 状态的 phase-specific interrupted outcome。调用者之后可以：

- 使用新的 filter/visitor 从下一个事件继续；
- 消费 interrupted state，接受当前 Block 前缀或当前 selection；
- 直接 drop interrupted state，放弃结果。

不提供语义重复的 `Pause` 和 `Stop` visitor 返回值。具体 outcome 类型名称由 API 原型决定。

### 无复制恢复

interrupted state 移动并继续拥有：

- 同一个 SourceText；
- 已构建的 Block/AST arenas；
- Pending Inline Store；
- scanner byte offset；
- line index；
- reference、footnote、tag 等文档元数据；
- 当前内部阶段和事件 cursor；
- Node ID 与 payload ID 分配位置。

恢复直接继续使用这些 Vec 和索引，不 clone Tree，不复制 source，不重新扫描已接受前缀。visitor closure 本身不存入 parser state；恢复时调用者提供新的 visitor。

### 稳定中断边界

顶层 Block 事件必须发生在：

- 当前 Document 直接子节点及其 Block 后代已经 finalized；
- 下一个顶层节点尚未提交；
- 不存在正在提交一半的 Inline batch。

语义目标中断发生在一次 target visitor 返回后，paused state 保存下一个 target cursor。中断不隐式选择当前目标。

### 只读视图

interrupted state 可以提供短生命周期 `DocumentView<'_>`。该视图借用 parser state，不能跨越下一次 resume 或 finish。最终 owned/borrowed `Document` 只有在解析完成或 interrupted state 被消费后返回。

### 不支持的恢复模式

- 不序列化 parser state；
- 不生成磁盘 checkpoint；
- 不跨进程传递 interrupted state；
- 不在 source 内容、长度或身份变化后恢复；
- 不承诺 visitor panic 后可以恢复。

## 错误与限制

checked API 延续明确错误返回，至少覆盖：

- 输入超过配置或内部 source offset 上限；
- 节点、payload 或 owned-text 数量超过配置或内部 ID 上限；
- 非法或未知 selection Node ID；
- source range 越界或不在 UTF-8 字符边界；
- parser phase 使用错误，如果最终公开 API 不能在类型层消除该错误。

`Interrupted` 是正常控制流，不是 ParseError。消费 interrupted state 接受当前结果后不能再恢复。

Inline batch 在正式提交前执行完整 preflight。checked API 在任何阶段失败都不返回一个声称完整成功的 Document。visitor panic 正常传播，panic 后 parser state 不保证可用。

## 性能验证

### v2A 架构落地版

以当前 benchmark corpus 和同一进程内交错运行的基准为准：

- 570,143-byte `markdown-it-corpus` parse-only 不高于同次 `pulldown-cmark` 完整事件消费的 2 倍；
- 当前参考机器绝对中位数不高于 5.0 ms；
- 259 KB `_data.md` 分配次数相对约 20,571 次基线至少降低 50%；
- 不通过泄漏、`mem::forget` 或跳过 AST 析构改善结果；
- 所有正确性门槛通过。

若硬件、编译器或 corpus 更新，绝对数值可以重新记录，但 2 倍相对门槛和 50% 分配降低目标不变，除非有新的 profile 和设计评审明确修订本 spec。

v2A 必须完成本设计的主要结构能力：typed arenas、source-range Text、Vec-index Inline workspace、延迟 AST 提交、紧凑关系索引、lazy location、选择性解析整合和同进程内存恢复。v2A 以正确性、结构收敛和 2 倍性能门槛为优先，不要求通过高风险微优化追赶 1.5 倍。

### v2B 深度优化版

- 完整 AST parse-only 不高于 `pulldown-cmark` 完整事件消费的 1.5 倍；
- 以不高于 `pulldown-cmark` 的 1.2 倍作为冲刺目标；
- 在相同 CommonMark/GFM AST 语义、扩展开关和结果消费方式下，争取超过 `rushdown`；
- 继续由 profile 驱动 Block matcher 分派、flavor specialization、payload/string pool 布局和 cache locality 优化；
- 可以在完整架构改造后使用 PGO 或 target-specific 编译获得额外改善；
- PGO 和 target-specific 结果单独报告，不能替代默认 release 配置下的 1.5 倍硬门槛。

完整 AST 低于 `pulldown-cmark` 事件消费的 1.0 倍不属于 v2B 验收要求。选择性解析和未来直接 HTML 后端可以各自建立低于 1.0 倍的专项目标，但不得用不同工作量的结果代替完整 AST 指标。

### 基准矩阵

至少维护：

- CommonMark 对 CommonMark；
- 双方共同支持的 GFM；
- 本项目完整 OFM 产品模式；
- parse only；
- parse + HTML；
- Block scan；
- semantic preparation；
- Inline tokenization/resolution；
- AST commit；
- drop；
- allocation count、reallocation count 和累计字节；
- plain ASCII、CJK、delimiter dense、link dense、blockquote/list、table 和大型 corpus。

所有 benchmark 结果必须通过 `black_box`、计数或等价方式被观察，避免优化器删除工作。`pulldown-cmark` parse-only 仍需完整消费 iterator，但报告中必须注明它不构建等价 AST。

直接 HTML renderer 不属于 v2A 或 v2B，因此 parse + HTML 不使用对应的相对硬性门槛，但必须持续记录以防 AST traversal 回归。

### 选择性解析性能

- 未注册 visitor 的完整解析不执行事件 callback 分派；
- 注册且始终 `Continue` 的顶层 visitor，相对相同 Block 模式回归不超过 5%；
- `Break → resume → complete` 不重复扫描已接受 source，不复制 arenas；
- 空 selection 不进入普通正文 Inline Engine；
- 祖先与后代重复选择不重复解析；
- 在不含大量 Heading、选择约 1% 正文的专项 corpus 上，总时间低于完整解析的 50%。

## 正确性验证

### 现有行为

- CommonMark、GFM、OFM、CJK、property 和 regression tests 全部通过；
- 现有 HTML fixture 输出逐字节一致；
- WASM owned-source 路径输出和异常行为保持一致；
- BlockId、reference、footnote、callout、table 和 HTML 行为不回归。

### AST 迁移

- 使用测试专用语义适配器比较旧 AST 与新 Document view；
- 比较 Node kind、父子顺序、source span、显示文本、payload、ID、tags 和渲染结果；
- borrowed 与 owned source 构造产生相同语义 AST 和 HTML；
- Source TextRef 与 Owned TextRef 对调用者返回相同预期文本；
- 转换文本保留覆盖原始 Markdown 的 source span。

### 位置

- byte span 覆盖正确输入范围；
- ASCII 与多字节 UTF-8 行列位置与现有 fixture 一致；
- CRLF、空行、tab、CJK 和 emoji 边界正确；
- 按需计算位置不能改变 AST 或 source state。

### Inline Engine

- delimiter、bracket、link、image、entity、code、math、emoji、tag、HTML 和 footnote fixture 保持一致；
- resolved batch 提交前后节点顺序正确；
- 失败解析不会提交部分 batch；
- workspace capacity 复用不泄漏上一个容器状态；
- delimiter/link 密集恶意输入保持线性或现有复杂度保证。

### 中断与恢复

- uninterrupted 结果与在每个合法顶层 Block 边界中断再恢复的最终结果一致；
- 多次中断/恢复与一次性解析一致；
- 接受 Block 前缀的结果与直接解析对应 source 前缀一致；
- semantic target 恢复从下一个未访问目标继续，不重复或跳过目标；
- 中断不隐式修改 selection；
- `DocumentView` 不能在类型安全 API 中跨 resume 保持有效。

### 选择性解析

继续覆盖现有 selective-inline spec 中的全部事件、BlockId、Heading、selection、footnote 和兼容性场景，并增加：

- 未物化节点公开为 Pending；
- 已物化空节点公开为 Materialized；
- source-range Text 在选择性与完整模式下返回相同内容；
- 中断恢复不改变选择展开和依赖去重结果。

## Rust API 与 WASM 兼容策略

这是下一 major version，允许以下 Rust breaking changes：

- `Document` 可以携带 source lifetime；
- 文本和 payload 通过 NodeRef/Document 访问，不再要求直接持有 String；
- Node ID 使用新类型；
- Tree 内部布局不再公开；
- 选择性解析的终态 Stop API 调整为 Break outcome。

应尽量保留：

- `Parser::new(&str).parse()` 的基本调用体验；
- checked 与非 checked API 的一致关系；
- Document 遍历、HTML、Serde 和 metadata 能力；
- 清晰的 owned-source 构造；
- WASM 当前以 owned String 输入和拥有 Document 的模型。

WASM 不暴露 Rust visitor callback 或 paused session。它继续使用完整解析和现有 frontmatter-only 行为；内部可以使用新 arenas 和 source ranges。除非另有独立设计，TypeScript-facing AST/HTML/metadata 行为保持兼容。

## 实施边界与顺序约束

本设计不是逐文件实施计划，但后续计划必须遵守以下依赖关系：

1. 先修正并扩展 benchmark、阶段计时和 allocation 观测；
2. 先证明 Vec-index delimiter/bracket workspace 的正确性和收益；
3. 再引入 resolved batch 与延迟 AST 提交；
4. 然后迁移 compact Node relations、source ownership 和 TextRef；
5. 再把位置切换为 byte span + lazy line/column；
6. 最后把选择性解析和内存恢复接到统一 pipeline；
7. 每个结构性步骤单独 benchmark 和回归，不把多个性能假设合并成一次无法归因的提交。

实施计划可以为了保持测试持续通过而增加短期 adapter，但 adapter 必须有明确删除步骤，不能成为 v2 最终热路径。

v2A 计划负责完成上述结构步骤并达到 2 倍门槛。v2B 必须建立在已经通过 v2A 正确性与性能验收的代码上，不得以推翻统一 pipeline、删除功能或恢复临时 Tree 写入作为提速手段。

## 决策摘要

- 采用统一的 v2 紧凑解析流水线。
- 完整解析与选择性解析共享实现。
- Document 借用或拥有 source。
- 文本优先保存 source range。
- 使用 Vec-based typed arenas，不先引入通用 arena allocator。
- Inline delimiter/bracket 使用整数索引。
- Inline 解析完成后批量提交正式 AST。
- Node ID 在 Block 创建后保持稳定。
- 位置以 byte span 为主，行列按需计算。
- visitor 使用 `Continue/Break`；Break outcome 可恢复或终结。
- 同进程恢复不复制 Tree 或 source。
- 不支持序列化、跨进程、落盘或修改 source 后恢复。
- 下一 major version 允许 Rust AST API breaking changes。
- v2A 以 2 倍、当前参考机器 5.0 ms 和分配次数降低 50% 为硬门槛。
- v2B 以 1.5 倍为硬门槛、1.2 倍为冲刺目标，并争取超过 `rushdown` 的等价 AST 路径。
- 完整 AST 低于 `pulldown-cmark` 事件消费的 1.0 倍不属于 v2B 验收要求。

## 作者注：基于现有代码的可行性复核

> 复核日期：2026-07-20
>
> 本栏保留前述设计原文，不对既有架构决策作无痕改写。它记录从当前实现、数据布局和基准结果中发现的实施约束、待验证假设及已知冲突。后续实施计划若采用与本栏不同的处理方式，必须给出代码证据、benchmark 或单独的设计评审结论。

### 总体判断

统一流水线的主方向可行，v2A 把差距压缩到 `pulldown-cmark` 完整事件消费的 2 倍以内也具有现实可能，但尚不能视为已经由原型证明。当前证据更强地支持以下结构改造：

- 使用 Vec 索引替换 delimiter/bracket 的 `Rc<RefCell<_>>`；
- 使用紧凑整数表示 Node ID 和关系索引；
- 使用文档顺序的 dense pending Inline store；
- 让 Document 持有或借用 source；
- 通过移动同一个 parser session 实现同进程无复制恢复；
- 把临时 Inline marker 和候选结构留在 workspace，只提交最终存活节点。

以下设计虽然可行，但需要原型和独立 benchmark 后才能锁定最终布局：

- 每种 payload 都使用独立 secondary typed arena；
- source-range Text 在 OFM、CJK 和多段 Inline 输入中的实际命中率；
- lazy location 对运行时间的贡献；
- resolved Inline batch 的具体结构；
- v2B 的 1.5 倍目标以及 1.2 倍冲刺目标。

### 代码与内存证据

当前 64-bit release 构建下的布局抽样为：

- `Location`：16 bytes；
- `MarkdownNode`：32 bytes；
- `Node`：80 bytes；
- `TreeNode<Node>`：152 bytes；
- `Option<usize>`：16 bytes；
- `Option<NonZeroU32>`：4 bytes。

570,143-byte corpus 解析后约有 33,393 个活跃节点和 38,509 个 Tree slots。仅 `TreeNode<Node>` slots 约占 5.85 MB，其中约 13.3% 是已经失效的槽位。259 KB `_data.md` 约有 11,496 个活跃节点和 12,942 个 slots，约 11.2% 为失效槽位。

这些数据说明紧凑关系索引、减少临时 AST 节点和取消 Inline 解析期的反复 remove/unlink/reparent 具有直接的 cache locality 与内存收益。当前 `_data.md` 每次约 20,571 次分配，因此分配次数降低 50% 的目标可信度较高。

570,143-byte corpus 从约 8.82 ms 降到不高于约 4.98 ms，需要减少约 43.5% 的运行时间。现有 profile 中 Inline、delimiter、Span、Tree mutation 和分配均为显著成本，所以 2 倍目标可以保留为硬性发布门槛；但在 indexed Inline workspace、compact relations 和 TextRef 原型完成前，不应把它描述为确定可达。

### 已知设计冲突

#### 稳定 Node ID 与顶层 Block 事件

当前 reference-definition 处理会在 Block scan 后取走 Paragraph 的 pending spans；当整个 Paragraph 都由 reference definitions 构成时，会直接执行 `tree.remove(node_id)`。因此，以下三件事不能按前文当前描述同时发生：

1. Paragraph 在 Block finalization 后立即产生顶层事件；
2. 事件向调用者暴露永久稳定的 Node ID；
3. Semantic preparation 随后允许把该 Paragraph 整体删除。

后续 API 原型必须显式选择并验证事件时机。优先考虑的解决方式是在顶层事件前完成该 Block 的局部结构语义归一化，至少包括 reference-definition extraction；事件发出后，已经暴露的 Node ID 不得删除或重编号。全局 reference resolution 仍可在接受的 Block 前缀上稍后完成。

#### Pending Inline 不能只保存裸 SourceSpan

当前 `MergedSpan` 不只是拼接 source ranges。它会去掉 blockquote/list 等容器前缀和已经消耗的缩进，并在不同 Span 之间提供逻辑换行。当前 `Span` 还携带 indent、line、column 和 ASCII 等逻辑信息。

因此，前文的 `SmallVec<[SourceSpan; 2]>` 只能作为简化示意，不能直接视为足够的最终表示。实现原型至少需要表达：

- 每段的 source range；
- 段前的逻辑连接方式，例如无连接或 soft break；
- 被去除的前缀或等价的逻辑起始信息；
- lazy location 所需的 line identity 或可恢复信息；
- tab、缩进和其他影响逻辑 Inline 输入的 flags。

可以使用紧凑的 `PendingSegment`，并继续以内联容量优化常见的一段和两段输入，但不能因为压缩结构而重新把容器前缀混入 Inline 内容。

#### Append-only 仅能严格约束 Inline commit

当前 Block 算法本身存在低频但有语义必要性的关系修改：

- 嵌套 List 可能在创建后调整父节点；
- HTML begin/end 节点可能吸收后续 sibling；
- footnote definition 在 Block finalization 后从正文关系中 unlink；
- table、Setext Heading 和部分容器会替换或调整已有 Block 结构。

因此，应在实施中区分：

- Node allocation 保持稳定 ID，原则上只追加且不重编号；
- Block relations 允许受控 mutation；
- Inline workspace 不修改正式 AST；
- resolved Inline commit 只追加最终存活节点。

稳定 Node ID 不等同于关系字段完全不可变，也不要求为了追求全局 append-only 而重写成熟的 Block 算法。

### 需要保留弹性的结构假设

#### Secondary typed arenas

紧凑 Node/关系存储是高置信度方向，但“同类 payload 必须连续存储”仍可能因为额外间接访问而损害遍历性能。实施计划应比较至少两种布局：

- 紧凑 `NodeKind`/payload union 或 compact enum；
- Node header 加 per-kind secondary arenas。

最终选择以完整解析、HTML traversal、Serde/WASM 转换和 drop 的综合 benchmark 为准。typed arena 是候选最终布局，不应成为在原型前不可撤销的约束。

#### Source-range Text

普通、连续且未经转换的文本适合保存 source range；但以下情况会产生 owned text 或更复杂的 segment 表示：

- soft break 或跨多个不连续 source segment 的文本；
- entity 解码与 backslash unescape；
- smart punctuation 与 CJK 文本修正；
- parser 生成内容；
- 需要合并的相邻文本；
- 部分 code、HTML、URL 或 label 的规范化结果。

性能验证应同时记录按 Text 节点数和按输出文本字节数计算的 source-backed hit rate，不能只检查功能上是否存在 `TextRef::Source`。

#### Lazy location

当前每个 Node 固定保存 start/end 两个 `Location`，合计 32 bytes，因此 byte span 加 lazy line/column 对节点尺寸和 cache locality 的价值明确。另一方面，当前位置计算被 Block 和 Inline matcher 广泛使用，迁移是跨模块重写，直接 CPU 收益尚未单独证明。

实施时应把它视为节点最终布局目标，并分别测量：

- parse-only 时间；
- 不访问位置时的遍历时间；
- 密集访问位置时的时间；
- Node 尺寸和峰值内存；
- ASCII、CJK、emoji、tab 和 CRLF 的正确性。

#### Resolved Inline batch 的副作用

resolved batch 不能只保存最终 Inline 节点。当前 Inline 解析还会产生或修改 footnote reference 编号、dependency、tag、reference/link metadata 和 component normalization 状态。

原型应把这些副作用建模为 batch 的显式组成部分，例如 metadata effects 和 dependencies，并在 capacity/limit preflight 之后与 AST 节点一起提交。否则“失败不留下半棵子树”的保证不能覆盖 Document metadata。

### v2A 实施风险注记

前文把 typed arenas、source-range Text、Vec-index workspace、延迟提交、紧凑关系、lazy location、选择性解析和同进程恢复都列为 v2A 必须完成的能力。作为最终 v2A 发布范围可以保留，但不宜作为一个不可拆分的实施步骤。

后续实施计划宜设置内部验证门：

1. indexed delimiter/bracket 与 resolved Inline IR，必要时保留旧 AST adapter；
2. delayed Inline commit 与 metadata effects；
3. compact Node relations、source ownership 和 TextRef，并验证分配次数降低 50%；
4. lazy location 与 payload layout 原型，以 benchmark 决定最终表示；
5. selective parsing、事件边界和 Break/resume 接入统一流水线；
6. 完整 v2A 正确性、2 倍相对门槛和 5.0 ms 产品模式门槛验收。

每一门都应保留独立的 before/after benchmark，避免多个结构假设一次落地后无法归因。选择性解析仍属于总体设计，但不应阻止核心 Inline/AST 性能路径先完成可行性验证。

### Benchmark 解释约束

本项目的完整 OFM 配置与 `pulldown-cmark` 的扩展选项不构成完全等价的语义集合。相对性能门槛应至少分为：

- CommonMark lane：双方只启用等价的 CommonMark 能力；
- shared GFM lane：只启用双方共同支持且输出可比较的扩展；
- OFM product lane：记录本项目完整产品模式的绝对时间和版本回归。

`pulldown-cmark` 的 parse-only 仍是事件消费而非等价 AST 构建，所以报告必须同时保留工作量说明。v2A 的 2 倍相对门槛应由等价 CommonMark/shared GFM lane 验收；当前参考机器的 5.0 ms 指标可继续作为 OFM product lane 的独立硬门槛。

与 `rushdown` 的比较只能在共同语法、相同输出语义、相同结果消费方式和独立 benchmark harness 下成立。v2B 的 1.5 倍可保留为硬目标，1.2 倍继续作为研究性冲刺目标；两者都需要在 v2A 数据出来后重新确认实现路径，而不是预先锁定高风险微优化。
