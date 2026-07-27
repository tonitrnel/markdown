# M1: 链接/转义文本下沉

**Type:** task
**Status:** resolved
**Blocked by:** None - v2B 首个 module（ticket 08 + 门槛通道归因）。

## 交付

Link/Image 载荷的 `url`/`title`（及关联的 label/alt 路径）不再无条件分配与转换 `String`：恒等情形保存源码区间（沿用 P1 的 `TextRef` 模式），`percent_encode`/`backslash_unescape`/entity 解码延迟到读取缝或仅在真正发生改写时物化。直击门槛通道归因的 14.9%（`percent_encode` 4.4% + `backslash_unescape` 4.3% + `scan_link_url` 3.5% + `normalize_reference` 1.2%）并同步压缩 30.7% 内存子系统中的链接份额。

## 实现范围

- `ast::link::DefaultLink`/`Image` 的 `url: String`、`title: Option<String>` 改为 `TextRef` 式表示（Source 区间 | Owned 转换文本）；读取经 Document 缝解析（渲染/Serde/WASM 输出逐字节不变——WASM 导出时物化，同 P1 模式）。
- `scan_link_url`/`scan_link_title`：无转义、无实体、无需编码改写时返回源码区间，零分配；仅在实际改写时构建 Owned。
- `percent_encode`：实施前先定位其在解析路径的全部调用点（profile 显示 4.4% 在 parse-only），恒等快路径（已编码/纯 ASCII 安全字符）零分配，或整体延迟到渲染读取时（择一，以输出逐字节不变为约束）。
- `normalize_reference`：ASCII 已小写恒等时避免分配（微优化，顺带）。
- footnote label 的 encode 路径不在本票范围（量小且涉及元数据键）。

## 明确不做

不引入链接 IR/延迟提交；不改 bracket/delimiter 工作区；不动 OFM 专属语法（M3）；不改渲染器结构（只换读取端点）。

## 验收

- 门槛通道 `parse_only/commonmark/markdown/markdown_it_corpus` criterion ≥−8%，或 `link_dense_flat` + `nested_brackets` 分配次数合计 ≥−20%（当前 5,130 + 10,250）。
- `reference_heavy` 时间不回归（≤+2%）；`_data.md` parse-only ≤+2%；其余 lane ≤+2%。
- 全部 fixture、`semantic_digest`、HTML/Serde/WASM 输出逐字节不变；`cargo check -p markdown-binding --target wasm32-unknown-unknown` 通过。
- Rust breaking（载荷字段类型）记入迁移说明；结果记录 `bench/results/incremental-iteration.md`。

## Answer

2026-07-27 完成，两步提交。**phase a（恒等 Cow 快路径）**：`percent_encode::encode_cow`（全安全字符零分配借用，慢路径预留容量）、`entities::unescape_string_cow`（无 `&` 借用）、`link::backslash_unescape_cow`（无 `\`+标点借用）；inline URL/title、引用定义 URL、两处 autolink 的「to_string → unescape → backslash → encode」四连分配链改为逐级 Cow（恒等时 4→1 次）；`normalize_reference` 加纯 ASCII 小写无空白恒等快路径（零分配返回原 String）。**phase b（载荷 TextRef 化）**：`DefaultLink`/`Image` 的 `url`/`title` 改 `TextRef`；`scan_link_or_image` 返回判别枚举 `ScannedLink::Resource/Footnote`（顺带取代 bool 标志，`![^x][^y]` 边缘行为逐字节保持）；恒等 inline 链接直接保存源码区间（恒等时 1→0 次）；渲染经 `resolve(self.source)` 读取、`format_title_attr` 改收 `Option<&str>`；WASM 导出时物化 Link/Image 载荷区间（序列化输出不变）。

**门槛（同会话 P5 后对照）**：

| 指标 | 前 | M1 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| 门槛通道 `parse_only/commonmark/markdown_it_corpus` | 5.364 ms | **4.502 ms** | ≥−8% | **−16.1% ✓** |
| vs pulldown 倍数（同轮 2.750 ms） | 1.97x | **1.64x** | — | 越过 v2B 1.5x 硬门槛线内侧 |
| `link_dense_flat` + `nested_brackets` allocs | 15,380 | **5,140** | ≥−20% | **−66.6% ✓** |
| `link_dense` / `nested` / `reference_heavy` 中位 | 442.5 / 844.8 / 214.6 µs | 237.6 / 514.6 / 164.1 µs | 另 lane ≤+2% | −46.3% / −39.1% / −23.5% ✓ |
| `_data.md` allocs / 中位 | 11,430 / 1,796 µs | 10,615 / 1,758 µs | ≤+2% | −7.1% / −2.1% ✓ |

**同类对照里程碑**：4.50 ms 已**快于 comrak（5.32）15%、rushdown（5.41）17%**——v2B 终点 1（同类反超）在大语料 CM 通道达成。分配 −50% 挑战余量仅 330 次（累计 −48.4%）。668 项测试 0 失败（含全部 CommonMark/GFM spec fixture = URL/title 输出逐字节不变）；WASM check 通过。Rust breaking：`DefaultLink`/`Image` 字段类型（迁移说明待随 CHANGELOG）。
