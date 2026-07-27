# P4: 有序 Pending Inline module

**Type:** task
**Status:** resolved
**Blocked by:** 14

## 交付

私有 `PendingInlines` module 取代 `FxHashMap<usize, Vec<Span>>`（`src/parser.rs:216`）：文档顺序条目 + node-to-entry 稠密索引 + 单/双段 SmallVec 内联；移除 B1 在 `drain_pending_in_document_order` 中的临时排序（文档顺序原生化）。

## 实现范围

- reference definition（`src/inlines/link_reference.rs`）、table（`src/blocks/table.rs`）、code（`src/blocks/code.rs`）、heading 的按 key 存取全部改经同一 interface；`Span` 的逻辑前缀/缩进/换行/位置语义不因压缩丢失。
- B1 的脚注顺序回归测试与 F3 的文档顺序测试继续通过（守住入口语义）。

## 验收

- `many_short_paragraphs` 时间 ≥−5%（基线 203.81 µs）或分配 ≥−10%（基线 3,081/parse）。
- `reference_heavy`（基线 237.52 µs）、容器与 `_data.md` 回归 ≤2%；document-order 有显式测试；`semantic_digest` 不变。

## Answer

2026-07-27 完成。新增 `src/pending.rs`：`PendingInlines`——条目按首次注册顺序（== 文档顺序，Block 首行在创建时注册且 id 单调）保存，`node_id → 条目` 走稠密 `Vec<u32>` 索引（O(1) 无哈希），segments 以 `SmallVec<[Span; 2]>` 内联（单/双段容器零分配）；移除的槽位保留、重插入（引用定义裁剪/Setext/table 改写）回原位置，顺序不因改写漂移。`MergedSpan` 底层同步换 `SmallVec<[Span; 2]>`（与存储同构，物化时按移动接管、零中转分配）。B1 的 `drain_pending_in_document_order` 排序删除，改为 `take_next_in_document_order` 游标 drain；语义准备的两处 `keys()+sort` 改为 `live_ids_in_document_order()`。

**门槛（同会话对照）**：

| 指标 | P3 后 | P4 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| `many_short_paragraphs` allocs | 2,056 | **6** | ≥−10% | **−99.7% ✓** |
| `many_short_paragraphs` 中位 | 179.5 µs | **134.9 µs** | ≥−5% | **−24.8% ✓** |
| `reference_heavy` 中位 | 219.5 µs | 212.5 µs | ≤+2% | −3.2% ✓ |
| `_data.md` allocs | 14,942 | **11,437** | ≤+2% 时间 | **−23.5%**；中位 1,885.5 µs 带内 ✓ |
| `_data.md` alloc_bytes | 2,765,618 | **2,168,338** | — | **−21.6%**（P1 未达标的语料字节 −20% 门槛就此补齐；自 P0 累计 −26.3%） |

文档顺序显式测试 = B1 的跨块脚注编号回归（`tests/footnotes_order.rs`）与 F3 全选等价，排序移除后全部继续通过。realloc_bytes 上升为 entries/index Vec 增长搬运内联 SmallVec 所致（alloc_bytes 大降、时间中性）。667 项测试 0 失败；WASM check 通过；`semantic_digest` 不变。
