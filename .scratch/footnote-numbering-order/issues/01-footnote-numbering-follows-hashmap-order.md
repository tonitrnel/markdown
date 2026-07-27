# 脚注编号跟随 HashMap 遍历顺序而非文档顺序

**Type:** task
**Status:** resolved
**Blocked by:** None - can start immediately.

## 现象

当脚注引用分布在多个 Block 中时，渲染出的脚注编号不按文档顺序递增，底部脚注列表同样乱序。单个 Block 内的引用不受影响（现有 fixture 因此未捕获）。

复现（2026-07-26，`dev/v2-temporary` @ `d55e180`）：20 个段落各自引用 `[^n1]`…`[^n20]`，OFM 选项解析后 HTML 中前三个段落的编号为 `[14]`、`[7]`、`[1]`（应为 `[1]`、`[2]`、`[3]`）。

```rust
let mut src = String::new();
for i in 1..=20 { src.push_str(&format!("Paragraph {i} refs[^n{i}].\n\n")); }
for i in 1..=20 { src.push_str(&format!("[^n{i}]: note {i}\n")); }
let doc = Parser::new_with_options(&src, ParserOptions::default().enabled_ofm()).parse();
// doc.to_html() 中编号乱序
```

## 原因

- `parse_inlines` 直接 drain `inlines: FxHashMap<usize, Vec<Span>>` 并按 map 遍历顺序处理各 Block（`src/parser.rs:444-458`）。
- 脚注引用编号在 inline 处理时按 `footnote_refs.len() + 1` 赋值（`src/inlines/footnote.rs:82-83`），因此编号顺序 = map 遍历顺序。
- `parse_footnote_list` 按已赋 index 排序（`src/parser.rs:544`），只能保持这个乱序结果自洽。

FxHashMap 无随机种子，同一构建下结果可复现，所以这是确定性的错误顺序，而非 flaky。

## 修复方向

短期（本票）：drain 后按 key（node id）排序再处理。Block 节点 id 在扫描期单调分配，等价于文档顺序。把"遍历 pending inline"收敛为一个私有辅助函数（如 `drain_pending_in_document_order`），后续有序 Pending 存储（渐进计划 P4）与选择性解析（F3）都必须复用该入口，届时移除排序。

排序成本约为一次 `Vec<(usize, Vec<Span>)>` 收集加 `sort_unstable_by_key`，条目数为 Block 级数量（`_data.md` 量级为千级），相对约 2.3 ms 的解析时间可忽略；仍需按护栏跑 `alloc_count` 与 `parse-only` 确认无 >2% 回归。

## 验收

- 新增回归测试：跨 ≥3 个 Block 的脚注引用，断言引用标记编号与脚注列表均按文档顺序（建议放入 `tests/z_regression_cases.rs` 或独立 `tests/footnotes_order.rs`）。
- `cargo test --workspace --all-features` 全部通过；`semantic_digest` 测试如受影响需说明差异原因（编号修正属预期变化）。
- `cargo bench --bench alloc_count` 与 phase_bench parse-only 无 >2% 回归。

## Comments

- 由 2026-07-26 性能计划调研发现并实测确认；证据与上下文见 `docs/plans/2026-07-26-markdown-parser-incremental-iteration-r2_cn.md`。

## Answer

2026-07-26 已修复。`Parser::parse_inlines` 改经新增的私有辅助 `drain_pending_in_document_order()` 消费 pending inline（`src/parser.rs`）：drain 后按 node id `sort_unstable_by_key`，node id 在扫描期单调分配，等价文档顺序；函数文档注明 P4 有序存储落地后移除排序，F3/P4 复用此入口。

- 回归测试：`tests/footnotes_order.rs`——跨 20 个 Block 的引用编号 1..=20 与脚注列表位置严格递增，另含单 Block 顺序不变用例。修复前红（首三个编号为 [14]/[7]/[1]），修复后绿。
- 验收：622 项 workspace 测试全部通过（semantic digest 不变，现有 fixture 无跨块脚注）；WASM `cargo check` 通过；`alloc_count` 同会话对照 +1 次分配/+58,816 B（entries Vec，约 1,838 条 pending × 32 B），中位耗时 2142.77 → 2140.21 µs，时间中性，护栏 ≤2% 通过。
- 测量记录：`bench/results/incremental-iteration.md`。
