# Define v2B entry and final challenge

**Type:** grilling
**Status:** resolved
**Blocked by:** Choose v2A module gates and order.

## Question

After v2A establishes measured gains and residual bottlenecks, what is the final v2B challenge, which one-module changes are eligible, and what evidence would justify widening scope?

## Resolution criteria

- v2B begins from v2A results, not the old whole-parser architecture.
- It names the remaining challenge metrics and comparison lanes that still matter.
- It permits at most one newly justified module boundary at a time.
- It explicitly rejects any unsupported return to parallel ASTs or all-at-once migration.

## Comments

- This ticket remains blocked while the relevant v2A evidence is unknown.
- 2026-07-27 unblocked by P7 (ticket 18): full v2A measurements, same-class comparisons, and the maintainer direction (Parse+AST core; rendering excluded).

## Answer

2026-07-27 解决（维护者方向：核心是 Parse + AST，渲染不做）。

**v2B 入口**：从 v2A 终盘出发——`_data.md` 11,430 allocs / 1.90 MB（−44.4% / −34.1%）、大语料 CommonMark 完整 AST 5.36 ms（与 comrak 5.32 / rushdown 5.41 打平）、OFM 5.5–6.3 ms 带宽、归因 Inline ≈27% / 内存 ≈27% / Block ≈23% / 行提取+位置 ≈19% / Tree ≈7%。

**最终挑战（Parse + AST 专注；比较通道 = parser_compare 的同类完整 AST 通道 + alloc_count + hotspots）**：

1. **同类反超**：完整 AST parse-only 在两数据集（curated + 大语料）、CM 与共享 GFM 通道上均**不劣于 comrak 与 rushdown**（当前大语料已平/略胜、curated 落后 rushdown 2.1%）——这是「实现质量」的干净度量，取代按 pulldown 倍数的旧表述（后者保留记录，工作量不等价说明不变）。
2. **OFM 附加成本 ≤5%**：OFM 产品通道相对本地 CommonMark 通道的差值（当前约 3–17% 带宽）压到 ≤5%；`5.0 ms` 绝对值降级为冲刺参考（校准存疑证据在案：同类裸 CM 即 5.3–5.4 ms）。
3. **分配 −50% 收尾**：`_data.md` ≤ 10,285 次（现 11,430，余 1,144）。

**模块边界（一次一个，各自独立设计评审 + 可回退 + 门槛）**：

- **M1 行提取+位置模型**（≈19%）：lazy location（`Node.start/end` 的行列改为按需，byte span 为主）与 `LineHead`（首字节候选收缩）在同一 module 内合并评估；跨模块面广，实施前先出独立设计评审（新 `.scratch/` 决策票）。
- **M2 受限 Inline commit / OFM 转换文本下沉**（Inline ≈27%）：明确语法族范围，不复活 v2 pipeline。

**明确拒绝**：并行 AST / 一次性迁移 / 渲染专项 / bump allocator（沿用既有护栏）。每 module 沿用 r2 共同护栏与 `bench/results/` 记录纪律。


## 证据补充：门槛通道热点归因（2026-07-27，维护者要求的分析器数据）

方法：`CARGO_PROFILE_BENCH_DEBUG=line-tables-only` 构建 parser_compare，`/usr/bin/sample 14` 采样 `parse_only/commonmark/markdown/markdown_it_corpus` 稳态（10,909 自顶栈样本）。

| 类别 | 占比 | 备注 |
| --- | ---: | --- |
| 内存子系统 | **30.7%** | malloc/free/memmove/realloc 族 |
| Inline-其它 | 19.1% | `inlines::process` 11.6%、`accumulate_run` 4.7% 等 |
| **Inline-链接/转义文本** | **14.9%** | `percent_encode` 4.4%、`backslash_unescape` 4.3%、`scan_link_url` 3.5%、`normalize_reference` 1.2%、`entities` 0.8% |
| Block 扫描 | 12.2% | `parse_blocks_observed` 6.9% 等 |
| Tree+析构 | 9.4% | `append_child` 4.2%、drop 2.5% |
| 行提取+位置 | 9.3% | 与 OFM curated 归因（19%）差异显著——语料构成不同 |

**v2B 目标账本**（spec：1.5x 硬门槛 / 1.2x 冲刺；维护者工作目标 <1.25x）：大语料 CM 5.36 ms / pulldown 2.73 ms = 1.97x；1.5x 需 ≤4.09 ms（−24%）、1.25x 需 ≤3.41 ms（−36%）、1.2x 需 ≤3.27 ms（−39%）。

**顺位修订建议（按门槛通道证据）**：M1 改为**链接/转义文本下沉**——URL/title/label 的 owned String 改 span/Cow 表示，percent_encode/unescape 延迟到读取或渲染（多数为恒等变换，P1 已验证的套路；直击 14.9% 并同步压内存子系统份额）；M2 = 行提取+位置（9.3–19% 视语料）；M3 = 受限 Inline commit / OFM 转换文本下沉。到 1.5x（−24%）以 M1+M2 现实可期；1.2–1.25x（−36~39%）需三项全兑现，逐 module 验后再评。
