# Painted Markdown

一个使用 Rust 编写的 Markdown AST 解析器。

## Motivation

目标是解析 CommonMark、GFM（GitHub Flavored Markdown）和 OFM（Obsidian Flavored Markdown），输出 AST 供上层渲染系统使用。
HTML 渲染仅用于测试与调试，不是核心目标。

需要注意的是，渲染 HTML 并不是该库的实现目标，这一功能主要用于测试目的。该库其核心需求是将库编译为 WebAssembly（WASM），然后在
NodeJS 的 Astro 项目中使用，根据 AST（抽象语法树）渲染相应的组件。之所以使用 Rust 语言而不是 TypeScript 编写是为了方便学习和实践
Rust 语言。

如果你在寻找极致吞吐和低内存占用，优先考虑事件流解析器（如 `pulldown-cmark` / `cmark`）。

## Installation

```toml
[dependencies]
markdown = { path = ".../markdown" }
```

## Usage

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
