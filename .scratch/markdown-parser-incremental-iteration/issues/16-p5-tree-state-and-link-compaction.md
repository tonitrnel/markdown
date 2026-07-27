# P5: Tree 空闲节点状态与链接字段压缩

**Type:** task
**Status:** resolved
**Blocked by:** 15

## 交付

1. `Tree.frees: FxHashSet<usize>`（`src/tree.rs:47`）换成更廉价的 slot/attached 表示（位图、attached flag 或受控 free list），保持 Tree interface 与节点编号语义。
2. `TreeNode` 的 `first_child`/`last_child`/`next`/`prev` 由 `Option<usize>`（16 B）压缩为 `Option<NonZeroUsize>`（8 B）：每槽 −32 B（152→≈120 B），前提不变量"槽 0 为 Document 根、绝不作为 child/sibling"以 debug assert 固化；`parent` 与公开 id 保持 `usize`。

## 明确不做

`u32` 打包（需先强制节点上限，v2B 评估）；append-only Block AST。

## 验收

- Tree mutation、component normalization、list、HTML、footnote、link fixture 通过；`semantic_digest` 不变。
- `multiline_blockquote_dense`（基线 55.37 µs）或 `many_flushes_dense_inline` 时间 ≥−5% 或分配 ≥−10%；`_data.md` ≤2%。
- 一次性测试断言并在结果记录附 `size_of::<TreeNode<Node>>()` 前后值。

## Answer

2026-07-27 完成。1）`frees: FxHashSet<usize>` → `free_flags: Vec<bool>`（按 id 索引，O(1) 无哈希；`set_next`/`set_prev` 保持原实现"不清除标记"的既有语义，纯表示替换）。2）`first_child`/`last_child`/`next`/`prev` 由 `Option<usize>`（16 B）压缩为 `Option<NonZeroUsize>`（8 B）：`TreeNode<Node>` **152 → ≤120 B/槽（−21%）**；不变量"槽 0 恒为根、绝不作为 child/sibling"由 `link_to` 内的 `debug_assert` 固化；公开 API 与 `parent` 保持 `usize`。尺寸以单元测试断言（`tree::tests::tree_node_slot_is_compact`）。`u32` 打包按票面留给 v2B。

**门槛（同会话对照）**：

| 指标 | P4 后 | P5 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| `many_flushes_dense_inline` 中位 | 898.7 µs | **729.8 µs** | ≥−5% 时间 | **−18.8% ✓**（allocs 持平——收益纯来自布局/缓存） |
| `cjk_dense` / `multiline_blockquote` 中位 | 364.5 / 49.6 µs | 344.4 / 48.1 µs | — | −5.5% / −3.0% |
| `_data.md` 中位 | 1,880.2 µs | **1,796.4 µs** | ≤+2% | **−4.5% ✓** |
| `_data.md` alloc_bytes | 2,168,338 | **1,900,893** | — | **−12.3%**（8,192 预分配槽 × 32 B ≈ −262 KB 对账吻合） |
| criterion `parse_ast_only` | — | **1.8327 ms**（CI 紧） | — | 本会话绝对最佳 |

668 项测试 0 失败（含新尺寸断言）；WASM check 通过；`semantic_digest` 与全部 fixture 不变。

**累计（P0 → P5）**：`_data.md` allocs 20,571 → 11,430（−44.4%）；alloc_bytes 2.89 → **1.90 MB（−34.1%）**；criterion parse-only ~2.13 → **1.833 ms（约 −14%）**。
