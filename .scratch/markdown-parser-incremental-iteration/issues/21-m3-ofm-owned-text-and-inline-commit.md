# M3: OFM 转换文本下沉 / 受限 Inline commit（计划先行）

**Type:** task
**Status:** resolved
**Blocked by:** 20

## 交付

两阶段：先以 Proposal 形式提交范围明确的计划（P7 选项 2 的"为明确语法族新写计划"，维护者确认后实施）。目标是服务两项 v2B 终点：**OFM 附加成本 ≤5%**（OFM 通道相对本地 CommonMark 通道差值，当前 3–17% 带宽）与**分配 −50% 收尾**（`_data.md` 余 1,144 次）。

## 计划必须界定

- OFM 转换文本的 owned 下沉范围：CJK 空格校正、中文标点归一（恒等时保持 `TextRef::Source`——当前 `correct_cjk_text` 已按 Cow 判断，但输入若为 Source 已物化）、smart punctuation、entity/escape 的恒等快路径。
- Inline 侧剩余分配的构成清点（当前 `_data.md` 11,430 次的成分：Box 载荷、剩余 Text Owned、`html_stacks`、`link_refs` String 键等），据此决定是否需要受限 Inline commit module（标记不进正式 AST；P2 记账的 marker 节点数为证据），或以更小的针对性改动完成 −50%。
- 若走受限 commit module：语法族边界（例如仅 emphasis 族）、与现有 Tree 直写的共存方式、回退路径——不复活 v2 pipeline。

## 明确不做

不做渲染专项（维护者已排除）；不做并行 AST；一次只动一个明确边界。

## 验收（实施阶段）

- OFM 通道与本地 CommonMark 通道差值 ≤5%（parser_compare 同轮测量）。
- `_data.md` 分配 ≤10,285 次（−50% 达成）。
- `cjk_dense`、`_data.md`（OFM）时间改善或持平；全部 OFM/CJK fixture 与 `semantic_digest` 不变；护栏同 r2。

## Proposal（计划，2026-07-27）

### 证据 1：`_data.md` 分配清点（11,496 节点 / 10,615 次分配，harness 同款 `enabled_ofm`）

| 来源 | 数量 | 估算分配 |
| --- | ---: | ---: |
| **Code 节点**（行内为主，载荷 `InlineCode {}` 为空结构体） | 1,456 | Box 1,456 + literal String ≈1,456 = **≈2,912（27%）** |
| Text Owned（vs Source 3,311） | 2,382 | ≈2,382（97 KB 容量） |
| 其余 Box 载荷（ListItem 338 / Link 205 / List 95 / Table 23…） | 682 | ≈682 |
| Link/Image url Owned（M1 战果确认） | 2 | 2 |
| 基础设施 + 载荷内部 String + realloc（1,451 次） | — | 余量 |

−330 目标只需 −3%；**行内代码 literal 一刀即 −12% 以上**。

### 证据 2：OFM 超额归因（同轮差分采样，OFM 通道 +6.2%）

`accumulate_run` **+11.9‰**（全表最大）+ malloc/free/memmove 连带 ≈+15‰ + `append_free_node` +3.6‰；真实 OFM 语法扫描（`process_wikilink`）仅 +1.5‰。机制：OFM 特殊字节表多出 `= ^ # % $ :`，普通文本命中后**打断文本 run**→ dispatch 层 `should_try_special` 虽已剪枝，但被剪掉的字节仍走「断 run + 单字节回填」往返。

### 范围（两个独立小改，不需要受限 Inline commit）

- **M3a 代码文本下沉**：行内代码恒等情形（非表格或无 `\`；无换行替换；剥空格边缘为子切片）改 `append_text_span_to` 存 `Text::Source`，仅真改写时 Owned；Fenced/Indented 块 literal 在「相邻行间隙恰为 1 字节 `\n` 且各行 `cursor==start` 且含结尾换行」时整体存源码区间（缩进剥离/容器前缀/CRLF 自动落回 Owned 路径）。**不拆 `Box<Code>`**：拆箱使 `MarkdownNode` 体积膨胀（FencedCode ≈48B > Text 32B），每槽 +8B × 全部节点的 arena 代价高于 1,456 次小分配收益——记为评估后放弃。
- **M3b 扫描环门控**：把 `should_try_special` 的源码局部规则（`==`、`%%`、`^` 后继类、`#` 前边界+后继类、`&` 实体类、`!` 后继 `[`、`<` 后继非空白、emoji `:` 后继类、`$` 数学、h/w 前缀首字节近似）下沉进 `accumulate_run` 扫描环：命中特殊字节时先做 1 字节前瞻门控，未通过则**不打断 run**；`pos+1 == span.end` 的跨 Span 边界一律放行给 dispatch（保守，跨行为不变）。单一规则实现，dispatch 层保留兜底。顺带医治 GFM extended-autolink 表中 h/w 全字母断 run 的同类病灶。

### 明确不做

受限 Inline commit module（清点显示不需要——目标由上述两改达成）；Text 合并物化专项（次级，留待证据再议）；`Box<Code>` 拆箱（见上）。

### 验收（票面既有 + 本计划）

OFM−CM 差值 ≤5%（两数据集）；`_data.md` 分配 ≤10,285；`cjk_dense`/`_data.md` 时间不回归；digest/HTML/WASM 逐字节不变（代码内容 Source 化经读取缝恒等）；护栏 ≤+2%。

## Answer

2026-07-27 完成，按 Proposal 两改落地（无受限 Inline commit，清点证明不需要）：

- **M3a 代码文本下沉**：行内代码恒等情形（无换行替换、非表格 `\|` 改写；剥空格边缘取子切片、GFM 单空格清空保持零分配 `String::new`）改存 `Text::Source`；Fenced 块 literal 经 `contiguous_lf_range`（各行 `cursor==start`、行间隙恰为 1 字节 `\n`、含结尾换行）整体存源码区间，缩进/容器前缀/CRLF 自动落回 Owned。Indented 走缩进恢复重写，维持不变。
- **M3b 扫描环门控**：`scan_gate`（与 `should_try_special` 同规则的源码局部 1 字节前瞻：`==`/`%%`/`^`/`#` 边界+类/`&`/`!`/`<`/h·w 首字节/emoji `:`/`$`）下沉进 `accumulate_run` 热环，未通过的特殊字节不再打断文本 run；`pos+1 == span.end` 跨 Span 边界保守放行给 dispatch 层，跨行行为不变。

**验收（全部超额）**：

| 指标 | M2 后 | M3 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| OFM−CM 差值（大语料 / curated，同轮） | +6.2% / +4.5% | **+0.28% / +2.6%** | ≤5% | **✓✓** |
| `_data.md` allocs | 10,615 | **8,205** | ≤10,285 | **✓（对 P0 −60.1%）** |
| `_data.md` 中位 | 1,491 µs | **1,354 µs** | 不回归 | −9.2% ✓ |
| `cjk_dense` | 203.8 µs | 204.8 µs | 不回归 | 持平 ✓ |
| 门槛通道（配对 min-of-3） | 3.884 ms | **3.801 ms** | ≤+2% 护栏 | **−2.1%**（`&`/`<`/`!` 门控外溢红利） |

**同轮终榜**：大语料 local 3.897 / pulldown 2.745 = **1.42x**，快 comrak 26.5%（rushdown 本轮环境漂移 6.61ms，按其常态 5.27 保守亦快 26%）；curated 1.34 ms = 1.68x，快 comrak 37.6%、rushdown 23.1%。671 项测试与全 spec/digest/golden 逐字节不变；WASM check 通过。**ticket 08 的 v2B 三项终点（双数据集同类反超、OFM ≤5%、分配 −50%）全部达成**；1.25x 冲刺目标余 −12%（3.90→3.43 ms），后续另立证据再议。
