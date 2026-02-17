# Painted Markdown

一个使用 Rust 编写的 Markdown AST 解析器。

## Motivation

`Painted Markdown` 是一个 **AST-first** 的 Markdown 解析器：
- 输入 Markdown，输出结构化 AST（包含节点类型、层级与位置信息）。
- 目标兼容 CommonMark / GFM / OFM，供上层渲染系统（Web/Node）消费。
- 内置 HTML 渲染主要用于测试与调试，不是主接口。

项目重点是作为上层渲染管线的“语法前端”，尤其是通过 WebAssembly 在 JS 生态中复用 Rust 解析能力。

如果你在寻找极致吞吐和低内存占用，优先考虑事件流解析器（如 `pulldown-cmark` / `cmark`）。

## Installation

```toml
[dependencies]
markdown = { path = ".../markdown" }
```

## API Usage

### Rust API

```rust
use markdown::{Parser, ParserOptions};

let parser = Parser::new_with_options(
    "# Title\n\ncontent",
    ParserOptions::default()
        .enabled_gfm()
        .enabled_ofm()
        .enabled_cjk_autocorrect(),
);
let doc = parser.parse();
let node_count = doc.tree.len();
let html = doc.tree.to_html(); // for debug/testing
```

### 安全护栏（推荐给 WASM/NAPI）

```rust
use markdown::{ParseError, Parser, ParserOptions};

let parser = Parser::new_with_options(
    input,
    ParserOptions::default()
        .with_max_input_bytes(128 * 1024 * 1024)
        .with_max_nodes(5_000_000),
);

match parser.parse_checked() {
    Ok(doc) => { /* use doc */ }
    Err(ParseError::InputTooLarge { limit, actual }) => { /* handle */ }
    Err(ParseError::NodeLimitExceeded { limit, actual }) => { /* handle */ }
}
```

### WASM API (Browser/Bundler)

```ts
import { parse_with_options } from "@ptdgrp/markdown-wasm";

const doc = parse_with_options("# Hello", {
  github_flavored: true,
  obsidian_flavored: true,
  cjk_autocorrect: true
});

const tree = doc.tree;
const tags = doc.tags; // unsorted string[]
const frontmatter = doc.frontmatter;
const html = doc.to_html(); // debug/testing

// Two-phase parse
// phase 1: parse frontmatter only
const deferred = parse_with_options(content, {
  parse_mode: "frontmatter_only",
});
const phase1Ast = deferred.tree; // Document + FrontMatter
if (deferred.frontmatter?.draft) {
  // skip
} else {
  // phase 2: continue parsing body/inlines
  deferred.continue_parse();
  const phase2Ast = deferred.tree; // Document + FrontMatter + ...
}
```

`parse_mode` supports:
- `"full"` (default): one-shot full parse
- `"frontmatter_only"`: run phase 1 only, then call `continue_parse()` to enter phase 2

`tags` is returned as an unsorted array. Do not rely on ordering.

### WASM API (Node.js)

```ts
import { parse } from "@ptdgrp/markdown-wasm-node";

const doc = parse("This is $e^{i\\pi}+1=0$");
console.log(doc.total_nodes);
console.log(doc.to_html()); // debug/testing
```

### Frontmatter

- `frontmatter` feature 默认启用。
- 文档顶部 frontmatter 会作为 `MarkdownNode::FrontMatter(..)` 节点挂在 `Document` 下。
- HTML 渲染会忽略该节点。

## Todos

**Blocks**

- [x] Thematic-Breaks
- [x] ATX Headings
- [x] Setext headings
- [x] Indented code blocks
- [x] Fenced code blocks
- [x] HTML blocks
- [x] Link reference definitions
- [x] Paragraphs
- [x] Blank lines
- [x] Tables(GFM)
- [x] Block quotes
- [x] List items
- [x] Task list items(GFM)
- [x] Callouts(OFM)
- [x] Footnotes

**Inlines**

- [x] Backslash escapes
- [x] Entity and numeric character references
- [x] Code spans
- [x] Emphasis and strong emphasis
- [x] Strikethrough(GFM)
- [x] Links
- [x] Internal links(OFM)
- [x] Block reference(OFM)
- [x] Block defining id(OFM)
- [x] Images
- [x] Embedding Files(OFM)
- [x] Math(OFM)
- [x] Autolinks
- [x] Autolinks(GFM)
- [x] RawHTML
- [x] Comments(OFM)
- [x] DisallowedRawHTML(GFM)
- [x] Hard line breaks
- [x] Soft line breaks
- [x] Emoji
- [x] Tag
- [x] Textual content

**Other**

- [x] Smart Punctuation
- [x] 中文语境优化 [chinese-copywriting-guidelines](https://github.com/sparanoid/chinese-copywriting-guidelines)
- [x] 在 delimiter 针对 CJK 符号的特殊处理
- [ ] 颜色文字 `#fff{这是白色的文本}`、`:red[这是红色的文本]`、`:red{这是红色的背景?}`
- [x] 通过 cmark 测试

## Processing Flow

```text
Input(&str)
   |
   V
-----------------
| Parse Blocks
|  extract line
|   process line
|   add line with block_id to inlines
------------------
   |
   V
------------------
| Parse Inlines
|  process block's line
|   process delimiter
|   process text(merge, cjk autocorrect)
------------------
   |
   V
Output(AST Tree)
```

## License

MIT
