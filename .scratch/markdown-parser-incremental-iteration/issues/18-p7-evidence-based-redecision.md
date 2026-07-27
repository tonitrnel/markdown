# P7: 依据证据重新决策（v2B 输入）

**Type:** grilling
**Status:** resolved
**Blocked by:** 16, 17

## 交付

重录全部通道（阶段耗时、alloc/realloc、drop、文本命中率、CommonMark/共享 GFM/OFM lanes、选择性路径），随附：位置模型与行提取占比（2026-07-26 基线 ≈10%）、渲染 per-node `format!` 临时分配与 parse+HTML 差值、WASM AST 导出深拷贝、P2 记录的 delimiter/bracket/marker 计数。

然后只作一种决定（不得合并）：

1. 挑战目标已达 → 停止架构迁移，只继续 profile 指向的微优化；
2. Inline 仍为最大热点 → 为一个明确语法族或受限 Inline commit module 新写计划（不复活 v2 pipeline）；
3. Tree/非 Text payload/位置模型占 parse-only ≥25% 或分配字节 ≥30% → 为该单一 module 提交独立设计评审。

结论作为 [ticket 08（v2B 入口与最终挑战）](08-define-v2b-entry-and-final-challenge.md) 的解锁证据。

## Comments

- 初版计划给位置模型设定的 25% 触发线在 ≈10% 的成本形态下几乎不可触发；本票须按实际占比、`cjk_dense` lane 数据与跨模块改造成本重议 lazy location 是否进入 v2B，不预设结论。

## 证据包（2026-07-27，P0–P6 全部完成后；决策待维护者）

**挑战目标记分**（P0 基线 → 现在，同参考机器）：

| 挑战 | 目标 | P0 | 现在 | 判定 |
| --- | --- | ---: | ---: | --- |
| CommonMark 大语料 vs pulldown 事件 | ≤2.0x | 2.80x（7.87ms） | **1.96x（5.38ms）** | **✓ 达成** |
| 共享 GFM 大语料 | ≤2.0x | 2.61x（7.32ms） | **1.99x（5.60ms）** | **✓ 达成** |
| 同上，curated 小语料 | （参考） | 2.79x / 2.75x | 2.20x / 2.13x | 接近 |
| OFM 产品大语料绝对值 | ≤5.0ms | 7.41ms | **6.28ms（−15.3%）** | ✗ 未达（差 20%） |
| `_data.md` 分配次数 | −50% | 20,571 | **11,430（−44.4%）** | ✗ 未达（差 5.6pp ≈ 1,144 次） |

**全通道重录**：parse-only CM/GFM/OFM 见上；parse+HTML：CM 大语料 10.73→7.93ms（−26%）、OFM curated 3.11→3.07ms。hotspots（criterion 中位，vs P0.5）：`many_flushes` −38.3%（727.6µs）、`many_short_paragraphs` −34.1%（134.2µs）、`cjk_dense` −17.8%、`link_dense` −16.4%、`nested_brackets` −16.0%、`reference_heavy` −14.6%、`multiline_blockquote` −12.6%、`plain_ascii` −3.7%。选择性：full 134.7µs / select-10% 105.6µs（比例 0.78 稳定）。分配终值：`_data.md` 11,430 次 / 1.90MB（−44.4% / −34.1%）。

**归因（P5 后采样）**：Inline 处理 ≈27%、内存子系统 ≈27%（自 32% 回落）、Block 扫描（不含行提取）≈23%（matcher 可省上限 ≈15%）、**行提取+位置 ≈19%**（`Span::extract` 605、`skip_to_eol` 459、`MergedSpan` 位置族 ≈490）、Tree+析构 ≈7%、pending ≈1%。

**其余记录项**：P2/P3 计数对账（delimiter run：`_data.md` ≈678、每 run 一个 Rc 已除；bracket 同理）；渲染 per-node `format!` 临时分配未动（parse+HTML 与 parse-only 差值 OFM curated ≈1.25ms）；WASM AST 导出深拷贝未动（独立于核心解析）。

**三选一的证据对位**：选项 1（挑战达成→停架构迁移）——2x 达成、其余两项未达，不完全成立；选项 2（Inline 仍最大热点 27%）——成立；选项 3（Tree/payload/位置 ≥25%）——位置 19% + Tree 7%，均未过线但位置是相对占比上升最快的类别。

**建议（待维护者裁决）**：v2A 就此收官（2x 达成宣告，其余两项记部分完成）；v2B（ticket 08）按证据顺位立项两个单 module 候选：① 行提取+位置模型（≈19%，lazy location 与 LineHead 在同一 module 内合并评估——两者都动行扫描）；② 受限 Inline commit module（Inline 侧 ≈27% + P2 记账的 marker 节点数）。分配 −50% 与 5ms 的尾巴预计随这两项自然覆盖，不单独立项。

## 证据补充（2026-07-27，维护者加入 comrak/rushdown 完整 AST 对照后）

维护者反馈"未达 v2A 预期"并加回 comrak/rushdown 参照；按对照约束接线（comrak 进 CM+共享 GFM 通道、rushdown 核心为 CommonMark 仅进 CM 通道、双方均含 parse-only 完整 AST 通道、构造移出计时循环）。同轮横向数据：

**parse-only 完整 AST 三方**（同轮，工作量等价）：

| dataset | 本地 | comrak | rushdown |
| --- | ---: | ---: | ---: |
| 大语料 CM | 5.364 ms | 5.319 ms（+0.8% 快） | 5.411 ms（**本地快 0.9%**） |
| curated CM | 1.743 ms | 2.171 ms（本地快 19.7%） | 1.707 ms（rushdown 快 2.1%） |
| 大语料 GFM | 5.467 ms | 5.287 ms（comrak 快 3.4%） | — |

**parse+HTML**：本地大语料 CM 7.18 ms vs rushdown 6.64 / comrak 6.52——**渲染侧落后同类 8–10%**（对应已记录的渲染 per-node `format!` 临时分配项）。

**重新校准的结论**：

1. 与工作量等价的同类相比，本地 parse-only 已处第一梯队（大语料打平/略胜 rushdown，胜 comrak curated 20%）。对 pulldown 的 ~2x 差距主要是"事件流 vs 建树"的工作量差——这正是 P0 起就记录的工作量说明的定量确认。**v2B 原定"超过 rushdown 完整 AST"的冲刺目标在 parse-only 上已达成边缘**。
2. **5.0 ms OFM 目标存在校准问题的证据**：同类 CommonMark 完整 AST（无 OFM 附加语法）本身就在 5.3–5.4 ms；OFM 附加语法再 +0.2–0.9 ms。5 ms 要求本地 OFM 快于同类裸 CommonMark——超出"消除实现浪费"所能达到的范围。OFM 绝对值两轮测量为 5.54–6.28 ms（环境漂移带宽），距 5 ms 差 10–25%。
3. 明确落后且可追的杠杆浮现：**渲染路径**（−8~10% 对同类）。
4. 分配 −50% 剩余（−44.4%）的构成不变：inline 侧 + 转换文本 owned + Box 载荷。

**修订建议（结合维护者"未达预期"信号，不选收官选项 1）**：v2A 以"2x 达成、同类打平、5ms/−50% 未达且 5ms 门槛校准存疑"记录收束；v2B（ticket 08）立项顺位修订为：**① 渲染路径**（对同类落后 8–10%，目标：parse+HTML 追平 comrak/rushdown）；**② 行提取+位置模型**（≈19%）；**③ OFM 转换文本 owned 路径下沉 + 受限 Inline commit**（服务 5ms 与 −50% 尾巴）。v2B 终点建议以同类对照定义（完整 AST parse-only 与 parse+HTML 均不劣于 comrak/rushdown），替代按 pulldown 倍数定义的旧表述。

## Answer

2026-07-27 由维护者裁决：**HTML 渲染不是本项目首选，不进入 v2B；核心是 Parse + AST**。据此在三选一框架内落为「继续单 module 性能工作（解析路径）」：

- 渲染路径候选**排除**（尽管对同类落后 8–10%，维护者明确不做）。
- **M1 = 行提取+位置模型**（归因 ≈19%，解析侧最大的非 Inline 可寻址块；lazy location 与 LineHead 在同一 module 内合并评估——虽未过原 25% 触发线，由维护者方向 + 相对占比第一的证据立项，先做独立设计评审再实施）。
- **M2 = 受限 Inline commit / OFM 转换文本 owned 下沉**（Inline 侧 ≈27%；服务 OFM 绝对值与分配 −50% 尾巴）。
- 一次只开一个 module；不复活并行 AST 与一次性迁移。

v2A 终盘表述：2x 达成、同类完整 AST 打平（大语料略胜 rushdown）、5ms/−50% 未达（5ms 门槛校准存疑已记录）。结论作为 ticket 08 的解锁证据。
