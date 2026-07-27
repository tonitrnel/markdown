# v2C: Block 相位与 selective 会话成本

**Type:** decision
**Status:** resolved
**Blocked by:** None - v2B 三终点已收官（tickets 19-21）。

## 背景与证据（2026-07-27，M3 后基线）

产品负载以 selective 会话为主：每次会话都付**全量 block 扫描 + 语义准备**（heading 物化、block-id 发现），inline 仅按需。切层测量（release，min-of-30，`parse_blocks_with(全接受) → prepare_semantic_targets`）：

| 文档 | 全量解析 | block-only | +语义准备 | 会话/全量 |
| --- | ---: | ---: | ---: | ---: |
| `_data.md`（OFM） | 1,477 µs | 598 µs（40%） | 728 µs | **49%** |
| markdown-it corpus（OFM） | 3,661 µs | 1,027 µs（28%） | 1,875 µs | **51%** |

- **语义准备在 corpus 上比 block 相位还贵**（848 µs vs 1,027 µs）；GFM 对照（无 block-id 发现）prep 仍 655 µs ⇒ **heading 物化 ≈ prep 的 70%+**（1,300 目标全部急切跑 inline 引擎，即便查询只读 block-id）。
- **block-only 相位内部热点**（`_data.md` 采样，忙线程）：`parse_blocks_observed` 自身 **34.1%**（容器匹配+matcher 分派主循环）；`skip_to_eol` **13.2%**（memchr2 双针，因 `\r` 兼容无法单针）；`Span::extract` 6.2%；**分配簇 ≈16%**（arena 预分配 memset、`PendingInlines::create_slot` 1.6%、Box 载荷）+ 会话拆除 drop ≈3%；`dispatch_top_level_events` **4.0%**（观察者存在时每行边界扫描顶层子节点，即使无新完成节点）；`finalize` 4.4%；表格 `parse_columns` 2.7%。
- OFM 使 block 相位比 GFM 贵 ≈160 µs/corpus（callout/块级 tag 探测）。

## 候选模块（收益 × 风险排序）

- **C2 行扫描 `\r`-free 快路径**：源码构建时一次 memchr 检测无 `\r`（绝大多数现代文档）→ `skip_to_eol` 用单针 memchr、`Span::extract` 免每行 `\r` 分支。低风险，预期 block 相位 −5~8%。
- **C5 观察者派发去每行税**：`incorporate_line` 返回「有无顶层节点完成」信号，`dispatch_top_level_events` 仅在信号为真时扫描（现每行边界都扫）。低风险，selective 路径 −3~4%。
- **C3 语义准备惰性化**（需产品 API 决策，见下）：heading 物化从「准备期全量」改为「目标被访问/选择时按需」，或提供不经 inline 引擎的源码级标题文本读取。heading 密集文档会话预期 −30~45%。
- **C1 Block 主循环矩阵**（34% 池，P6 结案评估 matcher 分派可省上限 ≈15%）：行首字节 → 候选 matcher 集合收缩（LineHead 表）、容器栈匹配快路径。收益最大、面最广，放最后做。
- **C4 会话分配外形**（16%+3% 池）：arena 初始容量按相位右尺寸（block-only 会话现按全量估算 `min(len/10, 8192)` 槽预分配+清零）、`PendingInlines` 槽增长策略、会话拆除成本。与 C1 解耦可独立量测。

## 待维护者决策

1. **C3 的 API 形态**：产品高频的 heading/blockid 会话实际读什么——(a) 仅标题层级+文本字符串（可源码切片直给，零 inline 物化）；(b) 标题的完整 inline AST（惰性按访问物化）；(c) 仅 block-id 寻址（heading 物化可全免）？决定惰性边界与是否新增源码级读取 API。
2. **v2C 终点草案确认**：`_data.md` 会话固定成本 728→≤500 µs（−31%）；corpus 会话 1,875→≤1,100 µs（−41%）；全量解析门槛通道不回归（≤+2%，力争 C1/C2 外溢转负）。同意/调整？
3. **模块顺序**：建议 C2+C5（合一张启动票）→ C3 → C4 → C1（证据门控：每步后重采样再决定下一步是否仍值得）。

## 护栏（沿用 r2 纪律）

selective 等价测试（prepare_then_finish == plain parse）、digest/golden 逐字节、`parse_blocks_with` 事件顺序锁定测试；新增 phase_bench 命名通道：`block_only/_data`、`session_prepare/_data`、`session_prepare/corpus`（本票测量脚本形式化）；配对 A/B + min-of-3 裁决与独立提交回滚不变。

## Answer

2026-07-27 维护者决策：

1. **C3 形态**：Heading/BlockId 语义目标服务 Obsidian 式快速寻址。heading 引用匹配按**剥离格式的纯文本**（Obsidian 规范化语义——heading 作 ref 时本就不承载多少 inline 语法），准备期不再急切物化任何 heading；`SemanticTarget` 提供按需的轻量 ref-text 读取（有限剥离 pass，非 inline 引擎）；需要完整 inline AST 时经 `selection.select` 走现有子集物化。剥离结果必须与完整解析的纯文本投影全语料等价（新增等价测试兜底）。
2. **终点**：按草案——`_data.md` 会话 728→≤500 µs、corpus 会话 1,875→≤1,100 µs、全量解析门槛通道 ≤+2%（力争转负）。
3. **顺序**：C2+C5（票 23）→ C3（票 24）→ C4（票 25）→ C1（票 26），每步重采样门控下一步。
