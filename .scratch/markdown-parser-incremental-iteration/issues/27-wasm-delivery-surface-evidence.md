# WASM 交付面证据：vs 主流 JS/TS 库

**Type:** investigation
**Status:** resolved
**Blocked by:** None - 新决策图定向的证据冲刺（v2C 收官后）。

## 交付

`bench/compare/wasm/`（node harness + README）：统一测「源码 → JS 侧可用结构」，本库两档（`parse` 留在 wasm 内 / `parse_tree` 全树跨边界）+ markdown-it tokens / marked lexer / remark mdast / commonmark.js AST。wasm 以 `+simd128` 构建（构建命令入 README）。

## Answer（2026-07-27 测量，node 26 / wasm-pack 0.14，SIMD 构建，中位 ms/op）

| op | `_data`(196KB) | corpus(560KB 单文档) |
| --- | ---: | ---: |
| **local_wasm/parse** | **2.62** | **6.36** |
| **local_wasm/parse_tree** | **16.25** | **47.55** |
| markdown_it/tokens | 5.79 | 24.24 |
| marked/lexer | 7.37 | 426.65 |
| remark/mdast | 149.37 | 515.80 |
| commonmark/ast（无 tables） | 3.84 | 15.06 |

三个结论：

1. **纯解析已是同类第一**：2.62 ms 赢 markdown-it 2.2x、marked 2.8x、remark 57x；corpus 上 marked/remark 出现超线性劣化（426/516 ms）。SIMD 构建让 parse 从 ~3.1-4.2 → ~2.7 ms（`parse_tree` 不动——证明其成本与解析无关）。
2. **`.tree` 边界序列化是产品链路的绝对主宰**：+13.6 ms/_data（≈ 原生解析 1.36 ms 的 **10 倍**）、+41.2 ms/corpus——serde_wasm_bindgen 逐节点构建 JS 对象（11,496 节点）。带上边界后我们反而比 markdown-it tokens 慢 2.8x。此前全部原生优化（v2A/B/C 累计 −49%）在产品全 AST 路径里只占 ~16% 份额。
3. **v2C selective 正是解药但未暴露**：ref_text/targets 查询只传小字符串，可完全绕开全树序列化——binding 目前只有全量 parse/tree。

## 新决策图（WASM 交付面）候选方向（按此证据排序）

- **W1 边界策略**：全树序列化替代方案——JSON 字符串 + `JSON.parse`（wasm-bindgen 已知常胜路径）/ 二进制扁平缓冲 + JS 解码器 / 惰性子树访问 API。目标：16.2 → ≤6 ms（与 markdown-it 打平以下）。
- **W2 selective 会话暴露**：`parse_blocks_with → targets/ref_text/select` 过 binding，heading/blockid 查询 ~1-3 ms 全链路（vs 今天被迫 16 ms 全树）。
- **W3 SIMD 构建固化**：`+simd128` 入构建脚本/文档（已验证 parse −15~35%）。
- 口味公平性注记与跨进程波动（JS 侧 ±20%）记档；原生↔wasm 同文档对照：_data 原生 1.36 ms vs wasm 2.62 ms ≈ 1.9x 运行时税（SIMD 后）。
