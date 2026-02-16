## Markdown Parser Playground

An interactive playground for testing and exploring the markdown parser with real-time AST visualization, HTML rendering, and live preview.

### Features

- **Real-time Parsing**: See AST, HTML, and rendered preview as you type
- **Parser Options**: Toggle various parser features (GFM, OFM, CJK support, etc.)
- **Multiple Views**:
  - AST: JSON tree structure of parsed markdown
  - HTML: Generated HTML output
  - Preview: Rendered markdown with GitHub-style formatting
- **Performance Metrics**: Parse time displayed for each update

### Usage

Prerequisites:

- `wasm-pack` installed (`cargo install wasm-pack`)
- Node.js 18+

Install and run:

```bash
# Build WASM binding first
cd ..
make build-wasm

# Install and run playground
cd playground
npm install
npm run dev
```

The playground will open at `http://localhost:5173`

### Development

```bash
# Development mode with hot reload
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

### Parser Options

- **GitHub Flavored**: Enable GFM extensions (tables, strikethrough, task lists)
- **GFM Autolink**: Extended autolink support
- **Obsidian Flavored**: Enable OFM extensions (wikilinks, callouts, embeds)
- **MDX Component**: Support MDX components
- **CJK Autocorrect**: Auto-correct CJK spacing
- **Smart Punctuation**: Convert quotes and dashes to typographic variants
- **Normalize Chinese Punctuation**: Normalize Chinese punctuation marks
- **CJK Friendly Delimiters**: CJK-friendly emphasis delimiters
