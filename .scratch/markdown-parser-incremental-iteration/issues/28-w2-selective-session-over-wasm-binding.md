# W2: selective 会话暴露过 WASM binding

**Type:** task
**Status:** resolved
**Blocked by:** None - ticket 27 证据定向；维护者拍板 WASM 优先（"号称给 JS 用，路没修好"）。

## 设计（无状态查询，尊重 spec 非目标"不做 WASM/JS 回调"）

型态机（`BlockPhase`/`SemanticPhase`）带生命周期不过边界；改为**无状态调用**——同文本+同选项的解析确定性 ⇒ `node_id` 跨调用稳定（契约：仅对逐字节相同的源码有效）：

- `query_semantic_targets(text, options?) -> TSemanticTarget[]`：块相位 + 语义准备 + 逐目标 `ref_text`（按需引擎物化，C3），返回 `{node_id, heading_level?, block_id?, ref_text, start, end}` 小数组——目标寻址主路径完全绕开全树序列化。
- `parse_selected(text, node_ids, options?) -> Document`：核心新增 `Parser::parse_selected_string`（镜像 `parse_string` 的 owned-source 重建），选择展开/footnote 依赖/InvalidSelectionNode 走既有 F3 通道；未选节点无 inline 子树，`.tree` 序列化量随选择缩小。

## 验收

- JS 侧目标查询全链路（`query_semantic_targets`，`_data.md`）≤3 ms（今天被迫 `.tree` ≈16 ms）；bench/compare/wasm 增 op 记录。
- Rust 侧等价测试：`parse_selected_string(全选)` digest == `parse_string`；node_id 稳定性测试；无效 id 报 `InvalidSelectionNode`。
- 现有 wasm 导出/序列化不变；668+ 测试全绿；TS 类型（`SemanticTarget`）入 types 模块。

## Answer

2026-07-27 完成。核心新增 `Parser::parse_selected_string_checked`（镜像 `parse_string` 的 owned-source 重建，选择展开/footnote 依赖/`InvalidSelectionNode` 走 F3 通道）；binding 新增 `query_semantic_targets[_with_options]`（块相位 + 语义准备 + 逐目标 `ref_text`，返回 `{node_id, heading_level?, block_id?, ref_text, start_offset, end_offset}`——字节偏移直配编辑器坐标）与 `parse_selected(text, node_ids)`；TS `SemanticTarget` 入 types。`HeadingLevel` 补 `Copy`。

**测量（bench/compare/wasm，优化产物，中位 ms）**：

| op | `_data` | corpus | 对照 |
| --- | ---: | ---: | --- |
| `query_targets` | **0.98** | **3.07** | 被迫全树 16.4 / 47.6 → **16.7x / 15.5x** |
| （门 ≤3 ms） | ✓ | ≈门（3 倍大文档） | markdown-it tokens 5.6 / 24.9 的 **5.7x / 8.1x** |

等价/契约测试：全选 digest == `parse_string`、非法 id 报 `InvalidSelectionNode`、node_id 跨调用稳定（674 项全绿）。产品側「快速搜寻目标」全链路自此 <1 ms/196KB——v2C 的原生成果第一次真正抵达 JS。
