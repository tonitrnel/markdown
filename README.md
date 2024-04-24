## Introduction

该项目是一個正在实施的 Markdown
解析器，其目的在於解析 [CommonMark](https://commonmark.org/)、[Github Flavored Markdown](https://github.github.com/gfm/)
規範和 Obsidian
編輯器的 [Obsidian Flavored Markdown](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown) 規範

該解析器主要目的是打包為 `WASM` 版后在 `NodeJS` 或 `Browser` 生產 AST 進行使用，渲染为 HTML 也在实现目标中，主要用于测试用例

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
- [ ] RawHTML
- [x] Comments(OFM)
- [ ] DisallowedRawHTML(GFM)
- [x] Hard line breaks
- [x] Soft line breaks
- [x] Emoji
- [ ] Tag
- [x] Textual content

**Other**

- [ ] Smart Punctuation
- [ ] 中文语境优化 [chinese-copywriting-guidelines](https://github.com/sparanoid/chinese-copywriting-guidelines)