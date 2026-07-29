# WASM Compare Bench

WASM binding 交付面与主流 JS/TS Markdown 库的对比基准。它测量从 Markdown 源码到 JS 可用结构的中位耗时，而不是仅测 WASM 内部的 Rust 解析。

```bash
# 根目录：构建 Node WASM 产物（SIMD）
RUSTFLAGS='-C target-feature=+simd128' wasm-pack build wasm-binding --target nodejs --release

# 安装锁定的对照库并运行完整基准
cd bench/compare/wasm
npm ci
npm run bench
```

也可以从仓库根目录执行 `make wasm-bench`。日常验证可执行 `npm run bench:quick`；它只取 5 个样本，不应用于记录或比较性能结果。

输出为带表头的 CSV：`name,dataset,ms_per_op`。完整运行在 `_data`（curated）上取 30 个样本，在 `corpus`（markdown-it 合并语料）上取 20 个样本；每个操作先预热 3 次。每次结果都读取根结构的计数，保证解析产物在计时范围内被观察。

- `local_wasm/parse`：解析，Document 留在 wasm 内（不跨边界）
- `local_wasm/parse_tree`：解析 + `.tree` 全树 serde_wasm_bindgen 序列化（产品全 AST 路径）
- `local_wasm/parse_tree_json`：解析 + JSON 字符串跨边界 + `JSON.parse`（全 AST 产品替代路径）
- `local_wasm/query_targets`：本库的目标寻址查询，不传输完整 AST；它是产品专项通道，不与第三方通用 Markdown 解析器作一对一比较
- `local_wasm/node_arrays`：解析 + 首次列式 AST 索引构建。返回的 `Uint8Array`/`Uint32Array` 是 WASM 内存视图，适合 JS 全量遍历，但在释放 Document 或 WASM 内存增长后必须重新获取
- `local_wasm/query_headings`、`local_wasm/query_links`：在 WASM 内遍历后只返回命中的小结果集
- 对照：`markdown-it` token 流（默认 preset，含表格）、`marked` lexer（GFM）、`remark-parse` + `remark-gfm`（mdast）、`commonmark.js` AST（纯 CommonMark）

口味差异必须随结果记录：本库默认解析启用 GFM 与 OFM，对照库使用各自最接近的配置；`commonmark.js` 不支持表格，故在表格密集的 `_data` 上结果偏乐观。记录基线时还要附上提交、Node 版本、wasm-pack 版本、CPU、操作系统与完整命令；不同机器的数值不能直接作为回归比较。
