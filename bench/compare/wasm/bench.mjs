// WASM 交付面 vs 主流 JS/TS 库：统一测「源码 -> JS 侧可用结构」的中位耗时。
// 输出 CSV：name,dataset,ms_per_op。
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
const wasm = require(join(root, "wasm-binding/pkg"));

const datasets = {
  _data: readFileSync(join(root, "bench/fixtures/curated/_data.md"), "utf8"),
  corpus: readFileSync(join(root, "bench/fixtures/corpora/markdown-it-corpus.md"), "utf8"),
};

const md = new MarkdownIt(); // default preset: CommonMark + table 等
const remark = unified().use(remarkParse).use(remarkGfm);
const cmark = new CommonmarkParser();

const args = new Set(process.argv.slice(2));
if ([...args].some((arg) => arg !== "--quick")) {
  throw new Error("usage: node bench.mjs [--quick]");
}
const quick = args.has("--quick");
const sampleCount = quick ? 5 : undefined;
const warmupCount = quick ? 1 : 3;

const ops = {
  // 本库：解析（Document 留在 wasm 内，不跨边界）
  "local_wasm/parse": (text) => {
    const doc = wasm.parse(text);
    doc.dispose();
  },
  // 本库：公开 JS API 的完整 AST 路径（惰性合成普通 JS Object）。
  "local_wasm/parse_tree": (text) => {
    const doc = wasm.parse(text);
    const tree = doc.tree;
    doc.dispose();
    return tree;
  },
  // 本库：目标寻址查询（W2——块相位 + 语义准备 + ref_text，无全树序列化）
  "local_wasm/query_targets": (text) => wasm.querySemanticTargets(text),
  "local_wasm/query_headings": (text) => {
    const doc = wasm.parse(text);
    const headings = doc.queryHeadings();
    doc.dispose();
    return headings;
  },
  "local_wasm/query_links": (text) => {
    const doc = wasm.parse(text);
    const links = doc.queryLinks();
    doc.dispose();
    return links;
  },
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

let observedResult = 0;

function observe(result) {
  if (Array.isArray(result)) {
    observedResult += result.length;
    return;
  }

  if (result && typeof result === "object") {
    if (Array.isArray(result.children)) {
      observedResult += result.children.length;
      return;
    }
    if ("firstChild" in result) {
      observedResult += result.firstChild === null ? 0 : 1;
      return;
    }
  }

  observedResult += 1;
}

function measure(op, text, samples) {
  for (let i = 0; i < warmupCount; i++) observe(op(text));

  const elapsed = [];
  for (let i = 0; i < samples; i++) {
    const start = process.hrtime.bigint();
    observe(op(text));
    elapsed.push(Number(process.hrtime.bigint() - start) / 1e6);
  }
  return median(elapsed);
}

console.log("name,dataset,ms_per_op");
for (const [dsName, text] of Object.entries(datasets)) {
  const samples = sampleCount ?? (dsName === "corpus" ? 20 : 30);
  for (const [opName, op] of Object.entries(ops)) {
    console.log(`${opName},${dsName},${measure(op, text, samples).toFixed(3)}`);
  }
}

// Keep the root result observation alive without altering the CSV stream.
if (observedResult < 0) throw new Error("unreachable benchmark result");
