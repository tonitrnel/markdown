# Choose v2A module gates and order

**Type:** grilling
**Status:** resolved
**Blocked by:** Trustworthy performance baseline, Selective parsing session interface, Source-backed Text interface.

## Question

Given corrected measurements and concrete public interfaces, which modules form v2A, in what order, and which measurable retention gate applies to each without expanding into a whole-parser rewrite?

## Resolution criteria

- The v2A list contains only end-to-end, independently reversible module slices.
- Each slice has a named benchmark lane, a no-regression rule, and a semantic test seam.
- The first selected performance module is justified by current evidence rather than the discarded v2 architecture.
- The result names the conditions under which a later module is deferred to v2B.

## Comments

- This is blocked until the two interface prototypes and baseline evidence exist.
- 2026-07-26: Baseline evidence exists (issue 04 resolved and reproduced: 20,571 allocs/parse, median 2142.77 µs on the reference machine). New candidate answer with fresh evidence: [r2 revised plan](../../../docs/plans/2026-07-26-markdown-parser-incremental-iteration-r2_cn.md) — adds a self-time profile attribution (≈1/3 of parse-only inside the memory subsystem; delimiter chains only ≈1.3% at corpus level; block side ≈26-32% but ≈9% of it is line extraction), a confirmed document-order footnote numbering bug ([B1](../../footnote-numbering-order/issues/01-footnote-numbering-follows-hashmap-order.md)), and a gap list (hotspot lanes cited by phase gates do not exist yet; `bench/results/` missing). Remaining blockers: prototypes 06 and 07, which must be resolved jointly so the selective-session types and `Document<'source>` lifetimes stay compatible.

## Answer

2026-07-26 解决。前置条件全部满足：issue 04 基线 + 当日复现与 profile 归因（记录于 `bench/results/incremental-iteration.md`）、[06 号接口结论](06-selective-parsing-session-interface.md)、[07 号接口结论](07-source-backed-text-interface.md)、P0.5 已把各门槛引用的 benchmark lane 落地、B1 已修复文档顺序 bug。维护者委托按 r2 推荐执行。

**v2A module 列表与顺序**（每个箭头是停点，均为端到端、独立可回退的切片）：

```text
F1 顶层 Block 事件与终态停止
  -> F2 语义准备与目标遍历
  -> F3 选择性 Inline 物化
  -> P1 source-backed Text
  -> P2 delimiter workspace
  -> P3 bracket workspace
  -> P4 有序 pending Inline（移除 B1 临时排序）
  -> P5 Tree 状态 + 链接字段压缩
  -> P6 仅在专项归因证明 matcher 分派可省部分 ≥25% parse-only 时实施
  -> P7 依据证据重新决策（v2B 输入）
```

**顺序理由**：功能先行——F1–F3 在现有 Tree 上端到端交付、不被 P1 的 breaking change 阻塞；06/07 接口已共用 `Document<'source>` 形态，选择性 API 不需要二次破坏。首个性能 module 是 P1，依据当前证据（parse-only 约 1/3 时间在内存子系统、Document 析构 8.6%、保留分配 8,817 次的大头是 Text String），而非旧 v2 架构。

**每个切片的 lane、无回归规则与语义测试缝**：详见 [r2 计划](../../../docs/plans/2026-07-26-markdown-parser-incremental-iteration-r2_cn.md) 各阶段（P0.5 落地的 lane：`plain_ascii_4k`/P1、`many_flushes_dense_inline`/P2、`link_dense_flat`+`nested_brackets`/P3、`many_short_paragraphs`+`reference_heavy`/P4、`multiline_blockquote_dense`/P5、`cjk_dense`/P7 位置证据）；全阶段共同护栏为 `_data.md` parse-only 回归 ≤2% + `semantic_digest` 一致 + 全 fixture 通过；门槛不过当场回退。

**延后到 v2B 的条件**（本票明确命名）：P6 在专项归因（区分 matcher 分派与行提取）未达 25% 时不实施；Tree 链接 `u32` 打包在未强制节点上限前不做；lazy location、受限 Inline commit module、其余 payload/位置迁移一律经 P7 依据 P1–P5 后的新测量单独立项；不复活并行 AST 或一次性迁移。

收敛产物：spec 见 [spec.md](../spec.md)，实施 tracer tickets 为本目录 09–18 号，blockers-first 领取。
