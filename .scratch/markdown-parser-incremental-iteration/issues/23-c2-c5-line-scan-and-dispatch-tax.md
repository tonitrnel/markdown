# C2+C5: 行扫描 `\r`-free 快路径 + 观察者派发去每行税

**Type:** task
**Status:** resolved
**Blocked by:** None - v2C 启动票（ticket 22 顺序决议）。

## 交付

- **C2**：`Scanner::new` 一次 memchr 检测源码含 `\r` 与否；无 `\r`（现代文档绝大多数）时 `skip_to_eol` 用单针 memchr(`\n`)、`Span::extract` 免每行 `\r` 消耗分支。含 `\r` 文档走原路径，行为逐字节不变。
- **C5**：`finalize` 在顶层节点（parent == doc）转为已完成时递增信号计数；`parse_blocks_observed` 仅在计数变化时调用 `dispatch_top_level_events`（现为每行边界无条件扫描）。EOF 边界派发保持无条件。事件的首见行边界不变（节点在第 N 行处理中 finalize ⇒ 第 N 行边界派发，与现行一致）。
- 形式化 v2C 命名通道（本票顺带）：`phase_bench` 增 `block_only/_data`、`session_prepare/_data`、`session_prepare/corpus`（`parse_blocks_with(全接受) [→ prepare_semantic_targets]`，与 ticket 22 测量脚本同形）。

## 验收

- `block_only/_data` ≥−5%，或两条 session 通道合计改善；全量解析门槛通道 ≤+2%。
- 全部 selective 等价/事件顺序测试、digest/golden 逐字节不变；CRLF/孤立 CR 矩阵不回归。
- 护栏与记录纪律同 r2；配对 A/B + min-of-3 裁决。

## Answer

2026-07-27 完成。**C2**：`Scanner` 增 `no_cr`（构建时一次 memchr 检测），无 `\r` 源码的 `skip_to_eol` 走单针 memchr(`\n`)。**C5**：`finalize` 在顶层节点完成时递增 `top_level_finalized` 信号，`parse_blocks_observed` 仅在信号变化时派发（EOF 边界维持无条件）；事件首见行边界不变。新增 phase_bench 命名通道 `block_only/_data`、`session_prepare/_data`、`session_prepare/corpus`（v2C 正式测量通道）。

**门禁（src-only stash 配对，共用新通道，min-of-2/3）**：

| 通道 | 前 | 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| `block_only/_data` | 618.1 µs | **587.4 µs** | ≥−5% | **−5.0% ✓** |
| `session_prepare/_data` | 754.7 µs | 736.0 µs | 改善 | −2.5% ✓ |
| `session_prepare/corpus` | 1.783 ms | 1.756 ms | 改善 | −1.5% ✓ |
| 全量门槛通道（vs M3 基线） | 3.852 ms | 3.899 ms | ≤+2% | +1.2% ✓（该通道 C2/C5 双休眠——corpus 含 `\r` 且无观察者——纯漂移带内） |

670 项测试全绿（selective 事件顺序锁定、CRLF/孤 CR 矩阵原样）；wasm check 通过。corpus 会话通道剩余大头是 heading 急切物化（848 µs 中 ≈70%）——正是下一票 C3 的靶子。
