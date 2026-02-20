# Polyglot Bench

Cross-language benchmark scaffold for:

- Rust local parser (`markdown`)
- Go `goldmark`
- C `cmark`

## Run

```bash
./bench/third_party/polyglot/run.sh
```

Output format:

```text
name,ms_per_op
```

Default datasets:

- `default_data`: `bench/third_party/polyglot/data/data.md`
- `markdown_it_corpus`: merged from `bench/data/markdown-it/*`

## Requirements

- Rust toolchain (`cargo`)
- Go toolchain (`go`), default binary path: `/usr/local/go/bin/go`
- C compiler (`cc`) and `cmake`
- `curl` (or `wget`) to fetch `cmark` source on first run

Notes:

- If Go dependencies cannot be downloaded, goldmark rows are emitted as `NA`.
- `cmark` is built locally under `bench/third_party/polyglot/.third_party` (no system install required).
- If local cmark build fails, cmark rows are emitted as `NA`.
