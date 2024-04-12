## Introduction

该项目是一個正在实施的 Markdown 解析器，其目的在於解析 [CommonMark](https://commonmark.org/)、[Github Flavored Markdown](https://github.github.com/gfm/)
規範和 Obsidian 編輯器的 [Obsidian Flavored Markdown](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown) 規範

該解析器主要目的是打包為 `WASM` 版后在 `NodeJS` 或 `Browser` 生產 AST 進行使用，渲染为 HTML 也在实现目标中，主要用于测试用例

## Todos

**Blocks**

- [x] Thematic-Breaks
- [x] ATX Headings
- [x] Setext headings
- [x] Indented code blocks
- [x] Fenced code blocks
- [x] HTML blocks
- [ ] Link reference definitions
- [x] Paragraphs
- [ ] Blank lines
- [x] Tables(GFM)
- [x] Block quotes
- [x] List items
- [x] Task list items(GFM)
- [ ] Callouts(OFM)
- [ ] Block defining id(OFM)

**Inlines**

- [ ] Backslash escapes
- [ ] Entity and numeric character references
- [x] Code spans
- [ ] Emphasis and strong emphasis
- [ ] Strikethrough(GFM)
- [ ] Links
- [ ] Internal links(OFM)
- [ ] Block reference(OFM)
- [ ] Images
- [ ] Embedding Files(OFM)
- [ ] Autolinks
- [ ] Autolinks(GFM)
- [ ] RawHTML
- [ ] Comments(OFM)
- [ ] DisallowedRawHTML(GFM)
- [x] Hard line breaks
- [x] Soft line breaks
- [ ] Textual content

**Other**

- [ ] Smart Punctuation
- [ ] 中文语境优化 [chinese-copywriting-guidelines](https://github.com/sparanoid/chinese-copywriting-guidelines)