# C4: 会话分配外形

**Type:** task
**Status:** resolved
**Blocked by:** 24

## 交付

block-only/selective 会话的分配右尺寸：tree arena 初始容量按相位估算（现 `min(len/10, 8192)` 槽全量估算+清零，block-only 浪费显著）；`PendingInlines` 槽增长策略复查；会话拆除（drop ≈3%）成本清点。先加 alloc harness 会话变体取证据，再改。

## 验收

- 会话通道分配次数/字节可量化下降；`block_only/_data` 时间改善或持平；全量解析 lane 分配与时间 ≤+2%。

## Answer

2026-07-27 完成。证据先行：alloc harness 新增 `session_block_only/_data`、`session_prepare/_data` 变体，实测 block 会话 5,260 次分配/733 次 realloc，调用树归因 **`SmallVec::try_grow` 统治分配时间**（pending 段存储逐行溢出+倍增）。两刀：

1. **Pending 共享 span arena**：条目改记 `(start,len)` 区间、段数据入共享 `Vec<Span>`（leaf 行天然按序到达，逐行 push 即尾追加；罕见非尾追加 `extend_from_within` 搬迁兜底；`remove`/drain 一次性拷出 `PendingSegments`，下游签名零改动；table 表头改写的 `get_mut` 换 `pop_line`）。`Span` 补 `Copy`（全字段 POD）。
2. **树 arena 相位感知右尺寸**：构造期上限 8192→4096 槽（选择性会话不为整棵 Inline 树买单），`parse_inlines` 入口按 `pending × 3 + 64` 一次 `reserve_nodes` 补齐（全量的增长搬迁源减半）。

**门禁（criterion 配对 min-of-3 vs C3 基线）**：

| 通道 | 前 | 后 | 判定 |
| --- | ---: | ---: | --- |
| `block_only/_data` | 595.4 µs | **547.6 µs** | **−8.0% ✓** |
| `session_prepare/_data` | 656.4 µs | **608.6 µs** | **−7.3% ✓** |
| 会话 realloc 事件 / 字节 | 733 / 1.37 MB | **28 / 193 KB** | −96% ✓ |
| 会话 alloc 字节 | 1.29 MB | 1.26 MB | 树右尺寸抵消 arena 预留 ✓ |
| 全量门槛通道 | 3.948 ms | 3.920 ms | **−0.7%**（≤+2% ✓，realloc 减半反哺） |
| 全量 `_data` allocs / reallocs | 8,206 / 1,351 | 8,207 / 647 | realloc −52% ✓ |

671 项测试全绿；wasm check 通过。会话拆除成本经右尺寸间接下降（更小 arena）；`_data` 会话 608 µs 距 ≤500 µs 终点余 −18%——C1（block 主循环矩阵）为最后一票。
