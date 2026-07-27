# P2: Delimiter workspace

**Type:** task
**Status:** resolved
**Blocked by:** 12

## 交付

emphasis/strong/strikethrough/highlight/smart-quote 的临时 delimiter 链不再使用 `Rc<RefCell<Delimiter>>`（`src/inlines/delimiter.rs:8-37`），改为私有 `InlineWorkspace` 内 `Vec<Delimiter>` + 索引链，容器间复用 capacity。

## 实现范围

- 只替换内部临时状态，继续直接写当前 Tree；不引入 resolved IR、延迟提交或第二种 AST 写入。
- `reset()` 清长度保 capacity；ID 用索引（哨兵或 NonZero）。
- 阶段结束时记录 delimiter/bracket run 计数与 marker 节点数（一次性统计即可），作为 P7 评估受限 Inline commit module 的证据。

## 验收

- `many_flushes_dense_inline` 时间 ≥−5%（基线 1,178.49 µs）或分配次数 ≥−10%（基线 21,526/parse）。
- `_data.md` parse-only 回归 ≤2%（语料级 delimiter 链自身仅 ≈1.3%，期望收益主要在分配次数——不以语料时间为过关条件）。
- CJK delimiter、未匹配 delimiter 与全部现有 fixture 通过；`semantic_digest` 不变。

## Answer

2026-07-27 完成。`Delimiter` 改为 `Parser::delimiter_store: Vec<Delimiter>` 内的条目，`prev`/`next` 为 `Option<usize>` 索引；`ProcessCtx.delimiters` 为链尾索引。emphasis 算法（openers_bottom、odd-match、配对收尾、引号转换、残链清理）逐语句移植，`remove_delimiter` 保留原有"有 next 时不动 slot"语义。嵌套 `inlines::process`（内联脚注定义体 `footnote.rs:79`、HTML 文本 `html.rs:475`）以 **base/truncate 协议**隔离：入口记 `store.len()`，出口截回，外层索引恒有效；容量跨容器/跨块复用。

**门槛（同会话对照，环境漂移用 stash 配对 A/B + 三次取最小中位裁决）**：

| 指标 | P1b 后 | P2 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| `many_flushes_dense_inline` allocs | 10,261 | **6,166** | ≥−10% | **−39.9% ✓** |
| `many_flushes` 中位（alloc 口径） | 996.5 µs | **839.4 µs** | ≥−5% | **−15.8% ✓** |
| `_data.md` allocs | 16,125 | 15,447 | — | −4.2% |
| `_data.md` 中位（min-of-3 配对） | 1,999.4 µs | 1,990.3 µs | ≤+2% | −0.5% ✓ |
| `cjk_dense` allocs | 2,568 | 2,057 | — | −19.9%（CJK delimiter 同受益） |

**计数证据（验收要求）**：`many_flushes` 分配差 4,095/解析 ≈ fixture 的 4×1024=4,096 个 delimiter run——每 run 一个 `Rc<RefCell<_>>` 被消除的直接对账；`_data.md` 差 678 ≈ 语料 delimiter run 数。criterion `many_flushes` 累计（P1+P2）−24.9%（885.5 µs vs P0.5 基线 1,178.5 µs）。667 项测试 0 失败；WASM check 通过。marker Text 节点仍写入 Tree（P1 已为 Source 区间、零 String），"标记不进正式 AST"的提交模型留给 P7 依证据决策。
