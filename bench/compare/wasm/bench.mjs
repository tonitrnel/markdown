// WASM 交付面 vs 主流 JS 库：统一测「源码 → JS 侧可用结构」的中位耗时。
// 输出 CSV：name,dataset,ms_per_op（与 polyglot 约定一致）。
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";

import MarkdownIt from "markdown-it";
import { Lexer } from "marked";
import { Parser as CommonmarkParser } from "commonmark";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkGfm from "remark-gfm";

const require = createRequire(import.meta.url);
const root = join(dirname(fileURLToPath(import.meta.url)), "../../..");
const wasm = require(join(root, "wasm-binding/pkg/markdown_binding.js"));

const datasets = {
  _data: readFileSync(join(root, "bench/fixtures/curated/_data.md"), "utf8"),
  corpus: readFileSync(join(root, "bench/fixtures/corpora/markdown-it-corpus.md"), "utf8"),
};

const md = new MarkdownIt(); // default preset: CommonMark + table 等
const remark = unified().use(remarkParse).use(remarkGfm);
const cmark = new CommonmarkParser();

const ops = {
  // 本库：解析（Document 留在 wasm 内，不跨边界）
  "local_wasm/parse": (text) => {
    const doc = wasm.parse(text);
    doc.free();
  },
  // 本库：解析 + 全树跨边界（产品路径：JS 侧拿到完整 AST）
  "local_wasm/parse_tree": (text) => {
    const doc = wasm.parse(text);
    const tree = doc.tree;
    doc.free();
    return tree;
  },
  // 本库：解析 + JSON 串过边界 + V8 原生 JSON.parse（W1 边界策略）
  "local_wasm/parse_tree_json": (text) => {
    const doc = wasm.parse(text);
    const tree = JSON.parse(doc.tree_json());
    doc.free();
    return tree;
  },
  // 本库：目标寻址查询（W2——块相位 + 语义准备 + ref_text，无全树序列化）
  "local_wasm/query_targets": (text) => wasm.query_semantic_targets(text),
  "markdown_it/tokens": (text) => md.parse(text, {}),
  "marked/lexer": (text) => new Lexer({ gfm: true }).lex(text),
  "remark/mdast": (text) => remark.parse(text),
  "commonmark/ast": (text) => cmark.parse(text),
};

function median(samples) {
  samples.sort((a, b) => a - b);
  const hi = samples.length >> 1;
  return samples.length % 2 ? samples[hi] : (samples[hi - 1] + samples[hi]) / 2;
}

for (const [dsName, text] of Object.entries(datasets)) {
  const iters = dsName === "corpus" ? 20 : 30;
  for (const [opName, op] of Object.entries(ops)) {
    for (let i = 0; i < 3; i++) op(text); // warmup
    const samples = [];
    for (let i = 0; i < iters; i++) {
      const t = process.hrtime.bigint();
      op(text);
      samples.push(Number(process.hrtime.bigint() - t) / 1e6);
    }
    console.log(`${opName},${dsName},${median(samples).toFixed(3)}`);
  }
}
