# W1: 全树边界序列化策略

**Type:** task
**Status:** resolved
**Blocked by:** 28 - 且以 W2 落地后的产品路径占比为门。

## 交付

`.tree` 的 serde_wasm_bindgen 逐节点 JS 对象构建（+13.6 ms/_data，原生解析 10 倍）替代方案对比实验：(a) JSON 字符串 + JS `JSON.parse`（wasm-bindgen 已知常胜路径）；(b) 二进制扁平缓冲 + JS 解码器；(c) 惰性子树访问 API。先做 (a) 原型量测（预期数小时），若 ≥2x 即采纳并评估 (b) 是否还值得。

## 验收

- `local_wasm/parse_tree`（_data）16.2 → ≤8 ms（与 markdown-it tokens 5.8 打平以下为冲刺）；序列化输出与现 TS 类型兼容或记迁移说明。

## Answer

2026-07-27 完成，两轮：

1. **(a) 原型证伪了"JSON 往返即胜"**：保持 AstNode 克隆图、仅换 serde_json 成串 + `JSON.parse` → 只 −3~4%。拆分定位：瓶颈是三段合谋——AstNode 克隆图构建（逐节点 `body.clone()`）+ 1.68 MB 的 JSON 本体（`"id":null`/`"content":null` ≈300 KB、`{line,column}` 位置对象 ≈690 KB）+ V8 `JSON.parse` 8.1 ms。
2. **直写序列化器（`json_tree.rs`）**：直接遍历内部树写单个缓冲（Text 借 resolved 字符串零克隆、Link/Image 仅小额克隆物化、itoa 整数直写），**紧凑 v2 形状**：`start`/`end` 为字节偏移（与 `SemanticTarget` 一致）、无值 `id`/`content`/空 `children` 省略、`content` 载荷与 `.tree` 同形（untagged serde）。JSON 1.68 MB → **978 KB（−42%）**。

**端到端（`parse_tree_json` = parse + 成串 + 过边界 + `JSON.parse`，min-of-3）**：

| | `_data` | corpus |
| --- | ---: | ---: |
| `.tree`（旧） | 16.5 ms | 48.2 ms |
| **`tree_json` + `JSON.parse`** | **8.30 ms（2.0x）** | **21.97 ms（2.2x）** |
| markdown-it tokens | 5.5-5.9 | 24.4-26.7 |

corpus 上**全 AST 路径首次反超 markdown-it**；`_data` 门 ≤8 ms 差 3.7%（**近未达**，如实记档）：剩余构成 parse 2.7 + 直写 2.0 + V8 `JSON.parse` ≈4.0——后者是 978 KB 下的引擎地板，再进一步只剩激进格式（短 kind 码/扁平数组，牺牲消费端可读性），留待产品实测需要再议。`.tree` 原样保留供迁移期。675 测试全绿（含直写完备性/形状测试）。
