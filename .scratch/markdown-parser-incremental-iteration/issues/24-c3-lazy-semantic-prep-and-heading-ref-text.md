# C3: 语义准备惰性化 + Heading ref-text 轻量读取

**Type:** task
**Status:** resolved
**Blocked by:** 23

## 交付（形态已由 ticket 22 Answer 决议）

- `prepare_semantic_targets` 不再调用 `materialize_headings`（heading 保持 pending；`finish()` 按文档序物化，输出不变——恰为 plain-parse 顺序，脚注编号天然正确）。
- `SemanticTarget` 新增按需 ref-text 读取：对 heading 的 pending spans 做**有限剥离 pass**（非 inline 引擎、不建树、不物化）：强调/删除/高亮标记剥除、code span 保内容去反引号、link/wikilink 取显示文本、escape 反转义、entity 解码、末尾 BlockId 剥除——规则集与 Obsidian heading 引用规范化对齐，实现为独立纯函数。
- 等价测试：全 spec/curated/obsidian 语料每个 heading，`ref_text` == 完整解析后该 heading inline 子树的纯文本投影（Text 节点按序拼接）。允许记录明确豁免清单（若 Obsidian 语义与投影存在有意差异）。
- `discover_block_ids` 维持现状（130–190 µs 量级，已够快）。

## 验收

- `session_prepare/corpus` ≥−35%（heading 密集主落点，现 prep 增量 848 µs 中 heading 物化 ≈70%）；`session_prepare/_data` 改善。
- `prepare_then_finish == plain parse` 等价测试全数不变；ref-text 等价测试全绿；全量解析 ≤+2%。

## Answer

2026-07-27 完成，两处按证据修订（均较原案更优，记档）：

1. **ref-text 实现从「有限剥离 pass」改为「按需真引擎物化 + 纯文本投影」**：剥离规则要与引擎等价需复刻 flanking/配对/code span 匹配（第三套文本语义，长期同步税）；改为 `SemanticTarget::ref_text(&mut self)` 首次调用时对单目标 `materialize_pending_subset` 再投影 Text 节点——保真度构造性精确（`^id` 剥除、escape/entity、链接显示文本全部免费），等价测试近乎恒真但仍锁管线。`SemanticTarget` 改持 `&mut Parser`（visitor 签名 breaking）。
2. **范围扩展**：摘除急切 heading 物化后 corpus 会话仅 −11%——重采样揭示先前 GFM 对照归因高估了标题份额（实际 ≈195 µs），准备成本大头是 `collect_semantic_targets` **全树遍历**（自身 ≈15.6% 会话）与 **引用定义提取** ≈15%。遂一并处理：目标改增量收集（`append_block`/`replace_block` 记录 Heading、`discover_block_ids` 记录 id 节点、discard 处 cutoff retain、collect 仅合并 +(span.start,id) 排序去重——等价文档前序，锁定测试原样通过）；`prepare_reference_definitions` 移出 prep，由 `ref_text`/`parse_selected_inlines`/`finish`（原有幂等调用）首次物化前确保——纯结构查询会话零支付。悬空目标不可能：尾部 `^id` 行不构成合法定义语法 ⇒ 携带 id 的段落必在提取后存活。`discover` 顺带免克隆（`as_str` 本就基于 cursor）。空选择契约更新：Heading 同样保持 pending（要 AST 经 select/ref_text）。

**门禁（配对 min-of-2，vs ticket 23 基线）**：

| 通道 | 前 | 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| `session_prepare/corpus` | 1.767 ms | **1.092 ms** | ≥−35% | **−38.2% ✓（v2C 终点 ≤1,100 µs 命中）** |
| `session_prepare/_data` | 731.6 µs | **648.5 µs** | 改善 | −11.4% ✓ |
| `block_only/_data` | 590.7 µs | 591.5 µs | — | 持平（未触碰）✓ |
| 全量门槛通道 | 3.915 ms | 3.923 ms | ≤+2% | +0.2% ✓ |
| `_data.md` allocs | 8,205 | 8,206 | — | +1（heading 记录 Vec） |

671 项测试全绿（含新 ref_text 全语料等价、惰性物化锁定、prepare_then_finish 等价原样）；wasm check 通过。`_data` 会话 648 µs 距 ≤500 µs 终点余 −23%，剩余大头即 block_only 本体（591 µs）——C4/C1 战场。
