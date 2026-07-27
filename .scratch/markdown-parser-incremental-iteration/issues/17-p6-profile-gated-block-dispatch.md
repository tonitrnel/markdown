# P6: Profile-gated Block dispatch（条件性）

**Type:** task
**Status:** resolved
**Blocked by:** 16

## 进入条件（先测量后实施）

P1–P5 完成后做一次专项归因（方法：`bench/benches/timing_test.rs` + `sample`/samply，写入结果记录），把"matcher 快照/重试 + 容器下降"与"行提取/位置预计算（`Span::extract`、`skip_to_eol`，2026-07-26 归因约 9%）"分开。**仅当前者 ≥25% parse-only 时实施**；未达标则本票以"证据不足，不实施"结案并把归因数据交给 18 号票。

## 实现范围（若触发）

先实现可单独验证的 `LineHead`（首字节/上下文/启用 flavor 缩小候选 matcher 集合），保留现有 matcher 优先级（`src/blocks/mod.rs:140-173` 的 11-matcher 顺序）；不建新 Block 状态机。

## 验收（若触发）

heading/fence/reference/thematic break/blockquote/list/HTML/table 几何平均 ≥−10%；任一完整语料回归 ≤2%；未达标只保留独立有收益的 `LineHead`，否则整体回退。

## Answer

2026-07-27 结案：**证据不足，不实施**（按票面进入条件裁决）。

P5 完成后的专项归因（方法同 P0：`CARGO_PROFILE_BENCH_DEBUG=line-tables-only` 构建 + `/usr/bin/sample 12s` 采样 `parse_ast_only` 稳态，记录于 `bench/results/incremental-iteration.md`）：

| 类别 | 占线程 | 备注 |
| --- | ---: | --- |
| Block 扫描（不含行提取） | ≈23% | `parse_blocks_observed` 自身 1,376、`finalize` 211、`table::parse_columns` 119、`append_block` 71 等 |
| —— 其中 matcher 分派可省上限 | **≈15%** | `parse_blocks_observed` 自身全部计入也只有 15.4%，且其中还含容器下降/行循环等 LineHead 不可省部分 |
| 行提取与位置 | **≈19%** | `Span::extract` 605、`skip_to_eol` 459、`MergedSpan` 位置族 ≈490——P1–P5 压缩其它侧后相对占比显著上升 |
| Inline 处理 | ≈27% | |
| 内存子系统 | ≈27% | 自基线 ≈32% 回落 |
| Tree 操作与析构 | ≈7% | |

matcher 分派可省部分 ≈15% < 25% 门槛 → 不实施 LineHead。归因数据（尤其行提取+位置 ≈19% 的新格局）移交 [P7](18-p7-evidence-based-redecision.md) 作为 v2B 候选（lazy location / 行提取与 LineHead 的合并评估）的决策输入。
