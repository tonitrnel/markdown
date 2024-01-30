## Introduction

`@painted/markdown` 是一個 Markdown 解析器，其目的在於解析 [CommonMark](https://commonmark.org/)、[Github Flavored Markdown](https://github.github.com/gfm/) 規範和 Obsidian 編輯器的 [Obsidian Flavored Markdown](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown) 規範

該解析器主要目的是生產 AST 然後打包為 `WASM` 后在 `NodeJS` 或 `Browser` 進行使用

## Features

Leaf blocks

- [ ] Thematic-Breaks
- [ ] ATX Headings
- [ ] Setext headings
- [ ] Indented code blocks
- [ ] Fenced code blocks
- [ ] HTML blocks
- [ ] Link reference definitions
- [ ] Paragraphs
- [ ] Blank lines
- [ ] Tables(GFM)

Container blocks

- [ ] Block quotes
- [ ] List items
- [ ] Task list items(GFM)
- [ ] Callouts(OFM)
- [ ] Backslash escapes
- [ ] Entity and numeric character references
- [ ] Code spans
- [ ] Emphasis and strong emphasis
- [ ] Strikethrough(GFM)
- [ ] Links
- [ ] Internal links(OFM)
- [ ] Block reference(OFM)
- [ ] Block defining id(OFM)
- [ ] Images
- [ ] Embedding Files(OFM)
- [ ] Autolinks
- [ ] Autolinks(GFM)
- [ ] RawHTML
- [ ] Comments(OFM)
- [ ] DisallowedRawHTML(GFM)
- [ ] Hard line breaks
- [ ] Soft line breaks
- [ ] Textual content
- [ ] Smart Punctuation

## API 设计

```javascript
const makrdown = require("..");

async function send(stream){
    const parser = makrdown.createStreamParser();
    for await (const chunk of stream){
        await parser.send(new Uint8Array(chunk));
        await parser.next();
    }
}
```