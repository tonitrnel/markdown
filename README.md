# Painted Markdown

> 一个使用 Rust 编写的 Markdown AST 解析器

## Motivation

我需要一个能解析 Markdown AST 的解析器，用于解析 CommonMark、GFM（GitHub Flavored Markdown）和 OFM（Obsidian Flavored
Markdown）。除了解析 Markdown 格式外，还能解析 HTML 顶层标签（Tag）的 `name` 和 `attributes`。

需要注意的是，渲染 HTML 并不是该库的实现目标，这一功能主要用于测试目的。该库其核心需求是将库编译为 WebAssembly（WASM），然后在
NodeJS 的 Astro 项目中使用，根据 AST（抽象语法树）渲染相应的组件。之所以使用 Rust 语言而不是 TypeScript 编写是为了方便学习和实践
Rust 语言。

如果你在寻找一个高性能的 Markdown Rust 库，我推荐使用 [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)
，其性能是本库的 20 倍以上。

> 注意：该库尚未通过所有测试，使用 `\t` 制表符会导致某些例如列表出现解析错误，建议使用 4 个空格替代 `\t`。

## Installation

```shell
npm i @painted/markdown-binding
```

## Usage

```ts
import { parse, type DocumentNode, type AstNode } from "@painted/markdown-binding";

const markdown = parse("## hello world");
const node = markdown.tree as DocumentNode; // Document Node，入口

export type DetermineElement =
  | readonly []
  | readonly [tag: string]
  | readonly [tag: string, props: Record<string, unknown>];

export const determineElement = (
  node: AstNode | undefined,
): DetermineElement => {
    if (!node) return [];
    switch (node.kind) {
        case "document": {
            return []
        }
        case 'paragraph': {
          return ['p', {id: node.id ? `block-${node.id}` : undefined}];
        }
        case "text": {
            const text = node.content;
        }
        // ...
    }
}

const render = (nodes: ReadonlyArray<AstNode>) => {
  for (const node of nodes){
    const [tag, props] = determineElement(node);
    if (!tag) continue
    // do something...
  }
}
render(node.children) // render document children
```

默认启用 `autocorrect` 用于优化排版

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
- [ ] Autolinks(GFM)
- [x] RawHTML
- [x] Comments(OFM)
- [x] DisallowedRawHTML(GFM)
- [x] Hard line breaks
- [x] Soft line breaks
- [x] Emoji
- [x] Tag
- [x] Textual content

**Other**

- [ ] Smart Punctuation
- [x] 中文语境优化 [chinese-copywriting-guidelines](https://github.com/sparanoid/chinese-copywriting-guidelines)
- [ ] 在 delimiter 针对 CJK 符号的特殊处理
- [ ] 颜色文字 `#fff{这是白色的文本}`、`:red[这是红色的文本]`、`:red{这是红色的背景?}`

## Processing flow

```text
Input(&str)
   |
   V
Tokenizer(TokenIterator)
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
