# F1: 顶层 Block 事件与终态停止

**Type:** task
**Status:** resolved
**Blocked by:** None - decisions 01-07 resolved.

## 交付

Rust 调用者能观察已 finalized 的顶层 Block，并在稳定边界终态停止；`Parser → BlockPhase` 落地并附 `finish[_checked]()` 使 F1 独立可用。接口形态以 [spec](../spec.md) 与 [06 号票 Answer](06-selective-parsing-session-interface.md) 为准。

## 实现范围

- 在 `Parser::finalize` 与 Block 扫描间建私有 seam：节点 finalize 后，若其父为 Document 根且注册了观察者，构造 `TopLevelBlockEvent` 派发（predicate 拒绝 = Continue）。通过泛型观察者（无观察者时单态化为空操作）保证 `parse()` 零派发成本。
- EOF 时最后一个顶层节点恰好派发一次；`Stop` 后停止消费源码、丢弃当前行剩余与后续输入、不创建下一个顶层节点、记录 `BlockScanStatus::Stopped`。
- `BlockPhase<'input>` 持有 parser 全部状态；`finish[_checked]()` 走既有 `parse_inlines` + `into_ast` 全量物化；无任何恢复扫描方法。
- frontmatter 与 Document 自身不产生事件；嵌套节点不产生事件。

## 明确不做

不暴露持久 NodeId 契约；不提前替换 pending 存储；不加 WASM callback；不把 Stop 做成可恢复。

## 验收

- 新测试（建议 `tests/selective_blocks.rs`）：只派发 Document 直接子节点；predicate 拒绝不改变解析；嵌套 list/blockquote/callout 无事件；每个可停止边界的前缀树结构有效且不含未接受节点；EOF 最后一个事件只派发一次；`Stop` 后 `finish()` 的 Document 与直接解析对应源码前缀一致。
- `cargo test --workspace --all-features`、`cargo test --test semantic_digest` 通过；HTML fixture 不变。
- `cargo bench --bench phase_bench -- parse_ast_only`：无 visitor 完整解析回归 ≤2%（同会话对照，记录到 `bench/results/incremental-iteration.md`）。

## Comments

- 事件时机的已知冲突（纯引用定义段落随后被移除）已由 map 决策 02 化解：事件只承诺回调期只读视图。

## Answer

2026-07-26 完成。实现：`src/selective.rs`（`VisitControl`/`BlockScanStatus`/`TopLevelBlockEvent`/`BlockPhase` 与 `Parser::parse_blocks_with[_checked]`）+ `src/parser.rs`（`parse_blocks_observed` 行边界游标派发、`dispatch_top_level_events`、`discard_unaccepted_after` 回滚、`run_block_phase_checked`/`finish_inline_phase_checked` 阶段拆分；原 `parse_blocks` 变为无观察者包装，逐行行为一致）。

**与票面 seam 描述的偏差（已记录）**：不在 `Parser::finalize` 内派发——finalize 深层调用点会把 table 吞并段落等"变换型 finalize"误当事件，且观察者存储会遇到生命周期/unsafe 问题。改为行边界游标派发：观察者以 `Option<&mut dyn FnMut>` 参数只存在于扫描函数内，`blocks::*` 零改动，无 visitor 时每行仅一个 `Option` 分支。视图弱化（同一行关闭上一个、开启下一个顶层块时，回调可能看到下一块首行）已写入 2026-07-19 spec 的 F1 实施注；`Stop` 回滚（卸链未接受子树 + 按 id 阈值清理 pending/footnotes/html_stacks/forks + `last_location` 还原）保证前缀与直接解析逐字节一致。

**验收结果**：`tests/selective_blocks.rs` 9 项全过（只派发直接子节点、predicate 拒绝不改变解析、全遍历 finish 与 `parse()` 在合成源与 curated 语料上摘要+HTML 逐字节一致、每个边界 Stop 与直接解析前缀一致、EOF 单次派发、frontmatter 无事件、checked 变体）；workspace 631 项测试 0 失败；WASM check 通过；`alloc_count` 分配数不变（20,572），criterion `parse_ast_only` −0.29%（p=0.49，判定无变化），护栏 ≤2% 通过。测量记录见 `bench/results/incremental-iteration.md`。
