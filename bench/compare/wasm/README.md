# WASM Compare Bench

WASM 交付面 vs 主流 JS/TS Markdown 库：统一测「源码 → JS 侧可用结构」中位耗时。

```bash
# 构建（SIMD）：
RUSTFLAGS='-C target-feature=+simd128' wasm-pack build wasm-binding --target nodejs --release
cd bench/compare/wasm && npm install && node bench.mjs
```

输出 CSV：`name,dataset,ms_per_op`。

- `local_wasm/parse`：解析，Document 留在 wasm 内（不跨边界）
- `local_wasm/parse_tree`：解析 + `.tree` 全树 serde_wasm_bindgen 序列化（产品全 AST 路径）
- 对照：markdown-it（default preset，含 tables）、marked lexer（gfm）、remark-parse+gfm（mdast）、commonmark.js（纯 CM，无 tables）

口味差异如实记录：本库 parse 为 GFM+OFM 全开，对照库各自最接近配置；commonmark.js 无表格（`_data` 表格密集，其耗时偏乐观）。
