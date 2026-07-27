# Benchmarks / 基准测试

`bench/compare/` contains comparison benchmark crates.
`bench/compare/` 存放跨实现比较基准 crate。

## Directory Rules / 目录规则

- `bench/benches/`: internal performance-regression benchmarks for this repository's `markdown` crate.\
  `bench/benches/`：本仓库 `markdown` crate 的内部性能回归基准。
- `bench/compare/`: cross-implementation benchmarks (for example, `markdown` vs `pulldown-cmark`).\
  `bench/compare/`：跨实现比较基准（例如 `markdown` 与 `pulldown-cmark`）。

Benchmark crates/folders:
基准 crate/目录：

- `parser-compare`: Rust parser comparison for CommonMark, shared GFM, and local OFM product lanes.\
  `parser-compare`：CommonMark、共享 GFM 和本地 OFM 产品通道的 Rust 解析器比较。
- `polyglot`: cross-language scaffold (`markdown` / `goldmark` / `cmark`).\
  `polyglot`：跨语言脚手架（`markdown` / `goldmark` / `cmark`）。

Shared third-party corpus:
共享第三方语料：

- `bench/fixtures/corpora/markdown-it` (source fixtures).\
  `bench/fixtures/corpora/markdown-it`（源 fixture）。
- `bench/fixtures/corpora/markdown-it-corpus.md` (merged corpus used by benchmarks).\
  `bench/fixtures/corpora/markdown-it-corpus.md`（基准所用的合并语料）。

## Run / 运行

`parser-compare` measures:
`parser-compare` 测量：

- local `markdown` AST parsing and HTML rendering in all three lanes.\
  三个通道中的本地 `markdown` AST 解析和 HTML 渲染。
- `pulldown-cmark` event consumption and HTML rendering only for the CommonMark and shared-GFM lanes.\
  仅在 CommonMark 和共享 GFM 通道中测量 `pulldown-cmark` 事件消费和 HTML 渲染。
- `comrak` full-AST parsing and HTML rendering in the CommonMark and shared-GFM lanes; `rushdown` uses the same two lanes.\
  在 CommonMark 和共享 GFM 通道中测量 `comrak` 完整 AST 与 HTML；`rushdown` 使用相同的两条通道。
- CommonMark uses the default local options and empty `pulldown-cmark` options. Shared GFM is limited to tables, strikethrough, and task lists.\
  CommonMark 使用各实现的默认 CommonMark 选项；共享 GFM 对所有实现都只启用表格、删除线和任务列表。
- OFM is a local product lane and is never presented as semantically equivalent to `pulldown-cmark`.\
  OFM 是本地产品通道，绝不表述为与 `pulldown-cmark` 语义等价。

Run the internal phase lanes (parse-only, render-only, and parse-plus-render):
运行内部阶段通道（仅解析、仅渲染、解析加渲染）：

```bash
cargo bench --bench phase_bench
```

Run allocation accounting on the existing curated fixture. Its output is per parse after one warm-up, over 500 samples, and includes the true median elapsed time:
在现有 curated fixture 上运行分配统计。输出是一次预热后的每次解析值，取 500 个样本，并包含真实中位耗时：

```bash
cargo bench --bench alloc_count
```

Run the cross-implementation comparison crate:
运行跨实现比较 crate：

```bash
cargo bench -p parser-compare-bench --bench parser_compare
```

Compile benchmark targets only:
仅编译基准目标：

```bash
cargo bench -p parser-compare-bench --no-run
```

## Recording Results / 记录结果

Record the full command, commit, operating system, CPU, memory, Rust toolchain, lane options, medians, and allocation output with every baseline. Results from different machines are not regression comparisons.
每次记录基线时，都要记录完整命令、提交、操作系统、CPU、内存、Rust 工具链、通道选项、中位数和分配输出。不同机器的结果不能用于回归比较。
