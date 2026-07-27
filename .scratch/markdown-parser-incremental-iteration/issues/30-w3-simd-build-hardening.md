# W3: WASM SIMD 构建固化

**Type:** task
**Status:** resolved
**Blocked by:** None

## 交付

`+simd128` 已验证 parse −15~35%（ticket 27）；固化：`wasm-binding/.cargo/config.toml` 对 `wasm32-unknown-unknown` 注入 rustflags，README 记录（含运行时兼容性注记：simd128 需 2021+ 的主流引擎，Obsidian/Electron 满足）。

## 验收

裸 `wasm-pack build` 即含 SIMD；`cargo check -p markdown-binding --target wasm32-unknown-unknown` 护栏不受影响。

## Answer

2026-07-27 完成：`wasm-binding/.cargo/config.toml` 注入 `+simd128` rustflags（裸 `wasm-pack build` 即含 SIMD，实测 parse 与手动 RUSTFLAGS 构建一致）；`Cargo.toml` 的 wasm-opt 元数据补 `--enable-simd`（维护者同步补充 bulk-memory 等 flags）修复验证失败。wasm32 check 护栏不受影响。
