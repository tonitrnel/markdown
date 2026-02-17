# Changelog

All notable changes to this project are documented in this file.

## [1.0.1] - 2026-02-17

### Added
- Exposed package version from wasm binding API via `version()` for runtime verification.
- Added two-phase frontmatter parsing flow to improve parser behavior around metadata extraction.
- Improved and expanded TypeScript type annotations/comments in wasm binding output.

### Changed
- Updated release/build docs and examples for current Rust/WASM usage.
- Improved Makefile flow for wasm packaging and profiling commands.

### Fixed
- Cleaned repository metadata artifacts and unrelated files from release history.

## [1.0.0] - 2026-02-16

### Breaking / Major
- Squashed the long feature/fix train into a single release commit for a clean `1.0.0` baseline.
- Bumped crate versions to `1.0.0`:
  - `markdown` (`Cargo.toml`)
  - `markdown-binding` (`wasm-binding/Cargo.toml`)
  - Workspace lockfile metadata (`Cargo.lock`)

### Added
- Full spec-runner setup and grouped suites for CommonMark, GitHub-flavored, and Obsidian-flavored tests.
- Property and regression test suites:
  - `tests/z_property_tests.rs`
  - `tests/z_regression_cases.rs`
- CJK-friendly delimiter and spacing behavior tests (`tests/cjk_friendly/*`).
- Frontmatter YAML parsing support (`src/exts/yaml.rs`) and related parser wiring.
- Additional performance benchmark entries (phase/timing benches).

### Changed
- Large parser/internal refactor:
  - moved from legacy line/tokenizer flow to `scanner` + `span` architecture
  - improved inline/block processing pipeline and reduced allocations in hot paths
- HTML parsing/rendering behavior improved across block/inline/component paths.
- OFM/GFM behavior alignment advanced across lists, footnotes, links, tables, math, and punctuation handling.
- Playground/web-demo structure updated and renamed to `playground`.

### Fixed
- Math parsing and display-math rendering behavior (including multiline `$$...$$` handling and paragraph splitting rules).
- Node location accuracy fixes in inline/span processing paths.
- Multiple spec compliance fixes across headings, lists, breaks, entities, and HTML edge cases.

### Notes
- WebAssembly artifacts remain target-architecture independent at the `.wasm` level (`wasm32-*`), while JS glue target selection still depends on runtime target (`web`/`bundler`/`nodejs`).
