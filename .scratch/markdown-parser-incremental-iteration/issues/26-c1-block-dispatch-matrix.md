# C1: Block 主循环矩阵（LineHead 候选收缩）

**Type:** task
**Status:** resolved
**Blocked by:** 25 - 且以 25 后重采样为门（若 `parse_blocks_observed` 份额已 <25% 则重议规模）。

## 交付

行首字节 → 候选 block matcher 集合收缩（LineHead 表，P6 结案评估上限 ≈15% matcher 池）；容器栈匹配快路径；表格 `parse_columns` 顺带复查。设计评审先行（票内 Proposal：表构建、选项组合、与容器 continuation 的交互）。

## 验收

- `block_only/_data` ≥−10% 或门槛通道 ≥−3%；全部 fixture/digest/golden 逐字节不变；护栏同 r2。

## Proposal（设计评审，2026-07-27）

**门控通过**：C4 后重采样 `parse_blocks_observed` 自身 **35.1%**（≥25%）。构成拆分：容器下降/行循环/派发（LineHead 不可省）+ **matcher 全试探**（可省）——非特殊首字节行已有预门直通，试探成本集中在特殊行（`_data` 表格行全 `|` 开头，每行满跑 11 matcher + snapshot/resume）与缩进行（陪跑 11 个才到末位 IndentedCode）。

### 设计：LineHead 位集表

`[u16; 256]`：行首（缩进后）字节 → 候选 matcher **位集**，位序 == 既有 11-matcher 优先级（`blocks::matcher` 的数组序），迭代 `trailing_zeros` 即按原优先级试探——**顺序语义零改动**。首字节读取 `line.get(line.indent_len())`：`re_find_indent` 保证容器消耗后 indent 域相对当前游标，与各 `before()` 自身 `advance_next_nonspace` 后所见字节恒等（与 incorporate_line 预门同一表达式）。

| 首字节 | 候选（按优先级） |
| --- | --- |
| `>` | Callout, BlockQuote |
| `#` | ATX |
| `` ` `` `~` | FencedCode |
| `<` | Html |
| `=` | Setext |
| `-` | Setext, ThematicBreak, ListItem, Table |
| `*` | ThematicBreak, ListItem |
| `_` | ThematicBreak |
| `+` `0-9` | ListItem |
| `\|` `:` | Table |
| `[` | Footnote |
| 其它 | ∅ |
| +缩进行 | 追加 IndentedCode（末位） |

正确性论证：省略 matcher 必须对该首字节恒 Unmatched——逐一核对各 `before()` 首字节门（callout/bq `>`、atx `#`、fenced `` ` ``/`~`、html `<`、setext `=`/`-`、thematic `-`/`_`/`*`、list `-`/`+`/`*`/数字、table `\|`/`-`/`:`、footnote `[^`、indented `is_indented`）；缩进+特殊字节行（如缩进 `> x`）候选并集与原序完全一致（各自 `is_indented` 内门照常拒绝）。选项组合不进表（Callout/OFM 等由各 before 内部自检，行为不变）。全 spec/GFM/OFM 套件 + digest/golden 为等价性兜底。

### 明确不做

容器 continuation（`blocks::process`）不动；`parse_columns` 复查仅在门禁未达时启动；不建新状态机（P6 约束继承）。

### 验收（票面既有）

`block_only/_data` ≥−10% 或门槛通道 ≥−3%；任一完整语料回归 ≤2%；fixture/digest/golden 逐字节不变；未达标仅保留独立有收益部分否则回退（P6 条款继承）。

## Answer

2026-07-27 结案：**门禁未达，按 P6 继承条款保留独立有收益的 LineHead**。

实施：`[u16; 256]` 位集表（位序 == 既有 11-matcher 优先级）+ 展开位测试分派。第一版用 `MATCHERS[i]` 动态索引在全量通道配对出 **+1.45%**（间接调用阻断内联）——护栏抓获，改展开宏保直呼后消除（这条经验记档：分派表必须保内联）。

**测量（配对 min-of-3）**：`block_only/_data` 542.4→**533.8 µs（−1.6%）**、`session_prepare/corpus` −1.6%、全量门槛通道 **−0.6%**。671 项测试与全 spec/digest/golden 逐字节不变。

**前提证伪（本票最有价值的产出）**：`parse_blocks_observed` 的 35.1% 自身时间中 matcher 试探只占极小份额——表格行/列表行等走 **continuation（`blocks::process`）**，matcher 仅在新块起点调用（每文档数百次而非每行）；非特殊行早有预门。35.1% 的主体是**容器下降 + 行循环 + 段落回退**，属"不建新 Block 状态机"（map 域外）约束下的不可省部分。P6 当年 ≈15% 上限中"含 LineHead 不可省部分"的警示条款完全应验——两票合起来把这个优化方向测到了底。

**v2C 终点结算**（对 ticket 22 目标）：corpus 会话 1,875→**1,012 µs ✓**（≤1,100）；`_data` 会话 728→**596 µs ✗**（≤500 未达，−18%；剩余为上述不可省 block 本体 + 单针 memchr 底座）；全量不回归 ✓（v2C 净 −2~3%）。未达项如实记录，进一步收益需要新状态机决策（域外，另立 map 议）。
