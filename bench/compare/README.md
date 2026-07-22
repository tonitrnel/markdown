# Benchmarks

`bench/third_party/` contains comparison benchmark crates.

## Directory Rules

- `bench/benches/`: internal performance regression benches for this repository's `markdown` crate.
- `bench/third_party/`: cross-implementation benchmarks (for example, `markdown` vs `pulldown-cmark`).

Benchmark crates/folders:

- `parser-compare`: Rust parser comparison (`markdown` / `pulldown-cmark` / `markdown-rs`)
- `polyglot`: cross-language scaffold (`markdown` / `goldmark` / `cmark`)

Shared third-party corpus:

- `bench/data/markdown-it` (source fixtures)
- `bench/data/markdown-it-corpus.md` (merged corpus used by benchmarks)

## Run

`parser-compare` currently compares:

- local `markdown` crate
- `pulldown-cmark`
- `wooorm/markdown-rs` (crate: `markdown`)

Run the comparison crate:

```bash
cargo bench -p parser-compare-bench
```

Compile bench targets only:

```bash
cargo bench -p parser-compare-bench --no-run
```

## Latest Sample Result

Command:

```bash
cargo bench -p parser-compare-bench --bench parser_compare
```

Sample medians from one local run (machine-dependent):

Dataset: `default_data`

| Benchmark | local `markdown` | `pulldown-cmark` | `markdown-rs` |
| --- | ---: | ---: | ---: |
| parse only | 1.724 ms | 0.545 ms | 45.555 ms |
| parse + html | 2.471 ms | 0.799 ms | 44.389 ms |

Dataset: `markdown_it_corpus`

| Benchmark | local `markdown` | `pulldown-cmark` | `markdown-rs` |
| --- | ---: | ---: | ---: |
| parse only | 12.064 ms | 2.774 ms | 134.030 ms |
| parse + html | 12.522 ms | 3.485 ms | 140.400 ms |

## Polyglot Sample Result

Command:

```bash
./bench/third_party/polyglot/run.sh
```

Sample output from one local run (machine-dependent):

Dataset: `default_data`

| Benchmark | local `markdown` (Rust) | `goldmark` (Go) | `cmark` (C) |
| --- | ---: | ---: | ---: |
| parse only | 1.629 ms | 1.807 ms | 1.610 ms |
| parse + html | 2.354 ms | 2.447 ms | 2.125 ms |

Dataset: `markdown_it_corpus`

| Benchmark | local `markdown` (Rust) | `goldmark` (Go) | `cmark` (C) |
| --- | ---: | ---: | ---: |
| parse only | 7.769 ms | 7.887 ms | 5.863 ms |
| parse + html | 10.020 ms | 11.280 ms | 7.053 ms |
