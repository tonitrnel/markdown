# F2: 语义准备与目标遍历

**Type:** task
**Status:** resolved
**Blocked by:** 09

## 交付

`BlockPhase::prepare_semantic_targets[_checked]() -> SemanticPhase`：对已接受前缀执行引用定义提取（全前缀）→ OFM BlockId 提取（共享扫描器）→ 全部 Heading Inline 物化 → 建立文档前序语义目标索引；`SemanticPhase::visit_semantic_targets(filter, &mut selection, visitor)` 按前序派发。语义与测试矩阵沿用 [2026-07-19 spec](../../../docs/specs/2026-07-19-selective-inline-events-design.md) 第 2–5 节。

## 实现范围

- BlockId 识别从通用 Inline 解析拆为可复用扫描器：写入 `Node.id`、从 pending Span 移除 ID 标记、不建普通 Inline 节点、独立 BlockId 行为不变；完整解析路径复用同一扫描器。
- Heading 物化后从 pending 集合标记 consumed，正文阶段不再重复处理。
- 目标 ID 仅 `SemanticPhase` 生命周期内稳定（map 决策 02）；`Stop` 只停遍历、不截断树、不隐式选择。

## 明确不做

不把 Heading 扩展为 section；不把每个 Block 变成目标；不引入 checkpoint 或第二套 AST。

## 验收

- ATX/Setext/嵌套 Heading 与嵌套 BlockId 按前序访问；Heading+BlockId 只产生一个目标；独立 BlockId 行与重复 ID 保持现有 OFM 语义；reference link/强调/code 在 visitor 前已完整解析。
- 完整解析使用相同 reference/BlockId/Heading 准备逻辑（`semantic_digest` 与全 fixture 不变）。
- 护栏命令同 spec；`_data.md` parse-only 回归 ≤2%。

## Comments

**2026-07-26 实施设计（代码调查结论，分三步提交）：**

1. **步骤 a——脚注最终化改为与调度无关（前置重构，输出不变）**。现状：`FootnoteLink { index, ref_count }` 在引用创建时按 `footnote_refs.len()+1` 烙进节点（`src/inlines/bracket.rs:138-157` 常规 `[^label]`、`src/inlines/footnote.rs:54-90` 内联 `^[..]`，后者的自动标签 `inline-footnote-N` 同样依赖处理顺序）。Heading 提前物化会打乱编号/标签，违背 F3 的"全选 = `parse()` 逐字节一致"。改法：创建时记 `(node_id, raw_label)` 到 parser 的 `footnote_ref_nodes`，`parse_footnote_list` 里按源码位置 `(start.line, start.col)` 排序后统一赋最终 index/occurrence，并对内联脚注按位置序重生成自动标签（补丁 `FootnoteLink`、`Footnote` def payload 与 `footnotes` map key）。当前完整解析在 B1 之后本就按文档顺序处理，位置序 == 处理序 ⇒ 全部现有输出不变（先落 lock-in 测试再重构）。已知可接受偏差：人为构造的 `inline-footnote-N` 标签碰撞场景，碰撞检查集合从"至今已见"变为"全部标签"。
2. **步骤 b——共享 reference/BlockId 准备**。`parse_reference_link` 加幂等标志（其内部会 remove 纯定义段落，不可重跑）。BlockId 共享扫描器只处理**安全子集**：纯独立 `^id` 块、末行独立 `^id`、末行尾缀 ` ^id`（尾缀后仅 ws 至 EOL，与 `src/inlines/link.rs:353` `process_block_id` 同一校验）；**块中形态保留引擎路径原样**（`src/inlines/mod.rs:186`、`src/inlines/newline.rs:53`）——理由：裸 span 盲扫会把多行 code span 内部的行尾 `^id` 误判（引擎因 `^` 不可达而不接受），安全子集则两路结果一致。后果：选择性模式下未物化块的**块中** `^id` 不可寻址（Obsidian 文档语义本就是"块末"，记为已知限制）；同块"块中+块末"双 id 的极端场景保持引擎语义（块末者由扫描器先取，引擎路径后写会覆盖——与纯引擎"最后写入者胜"在该场景不同，无 fixture 覆盖，接受并记录）。`parse()` 与 prepare 调同一扫描器。
3. **步骤 c——`SemanticPhase`**：`BlockPhase::prepare_semantic_targets[_checked]()` 依序执行 a 的引用准备、b 的 BlockId 提取、按文档顺序物化全部 pending Heading（`inlines::process` + `normalize_component_children`，处理后从 pending 移除），前序遍历建 `targets: Vec<usize>`（Heading 或 `node.id` 非空）；`visit_semantic_targets(filter, &mut InlineSelection, visitor)` 前序派发，`Stop` 只停遍历；`SemanticPhase::finish[_checked]()` 物化剩余 pending + 脚注列表（与 `parse()` 尾段共用）。`InlineSelection` 本票先以私有去重集合落地（F3 接手物化）。

## Answer

2026-07-26 完成，三步三提交：

- **步骤 a**（`2baab65`）：脚注最终化与调度无关。`footnote_ref_nodes` 记录 `(节点, 原始 label)`，`parse_footnote_list` 按源码位置排序统一赋最终 index/occurrence，内联脚注自动标签按位置序重生成（交换式重命名两阶段应用，避免 1↔2 互相覆盖）。`tests/footnotes_schedule.rs` 锁定 5 组行为不变，并记录既有怪癖：相邻 `[^a][^b]` 会被引用链接语法吞并（待单独决策）。
- **步骤 b**（`a980c14`）：`src/semantic.rs` 落地 `prepare_reference_definitions`（幂等标志，内部会消费纯定义段落）与 `discover_block_ids`（只发现、不改写 spans；最右 `^` + 字符集 + 空白至条目末尾校验，天然拒绝多行 code span 陷阱）。`tests/block_id_probes.rs` 锁定 8 组形态（含同行尾缀空格保留、code span 陷阱、last-id-wins）。
- **步骤 c**（本提交）：`prepare_semantic_targets[_checked]` → `SemanticPhase`（`SemanticTarget`/`InlineSelection`/前序 `targets`/`visit_semantic_targets`/`finish[_checked]`）。Heading 物化与 drain 循环逐项一致；发现阶段实测会给完整解析加约 2% 冗余成本后，改为**仅语义准备路径调用**（完整解析中引擎本就是 id 权威写入者；语义单一来源由共享校验逻辑+探针测试保证）。修复 `process_footnote_list` 的 `FootnoteList` 结束位置：原实现取 max 节点 id 的 end（调度依赖的任意值），改为按文档位置计算（内联定义存在时取其位置最大者，否则取块级定义位置最大者），该规则在文档序处理下与旧值逐例相等。

**验收结果**：`tests/semantic_targets.rs` 10 项全过——前序目标（嵌套 Heading/引用内 id/列表项 id/Heading+id 单目标/独立 id）、Heading Inline 完整性（reference link 已解析）、**prepare→finish 与 `parse()` 在合成源、脚注跨 Heading、内联脚注、curated 语料四种场景摘要+HTML 逐字节一致**、Stop 只停遍历、filter 拒绝继续、重复 id 独立目标、Stopped 状态贯穿。workspace 656 项测试 0 失败；WASM check 通过；`_data.md` allocs 20,599（相对 F1 后 +27，+0.13%）、alloc 中位 2111.9 µs（当日噪声带低端）、criterion `parse_ast_only` 判定无变化（p=0.41）。

**已知限制（选择性模式）**：块中（非末行）`^id` 只在物化时由引擎识别，未物化 Block 的此类 id 不可作为语义目标寻址（Obsidian 文档语义为"块末"；发现子集刻意保守以保证与引擎零分歧）。
