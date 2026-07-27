# Polyglot Bench

Cross-language benchmark scaffold for:

- Rust local parser (`markdown`)
- Go `goldmark`
- C `cmark`

## Run

```bash
./bench/compare/polyglot/run.sh
```

Output format:

```text
name,ms_per_op
```

Default datasets:

- `default_data`: `bench/compare/polyglot/fixures/data.md`
- `markdown_it_corpus`: `bench/fixtures/corpora/markdown-it-corpus.md`（已存在时直接复用；缺失时由 `bench/fixtures/corpora/markdown-it/*` 合并生成）

## Requirements

- Rust toolchain (`cargo`)
- Go toolchain (`go`), default binary path: `/usr/local/go/bin/go`
- C compiler (`cc`) and `cmake`
- `curl` (or `wget`) to fetch `cmark` source on first run

Notes:

- If Go dependencies cannot be downloaded, goldmark rows are emitted as `NA`.
- `cmark` is built locally under `bench/compare/polyglot/.third_party` (no system install required).
- If local cmark build fails, cmark rows are emitted as `NA`.
