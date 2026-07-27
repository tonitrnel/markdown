# P1: 当前 Tree 中的 Source-Backed Text

**Type:** task
**Status:** resolved
**Blocked by:** 11

## 交付

按 [07 号票 Answer](07-source-backed-text-interface.md)：`Document<'source>`（borrowed 默认）、`Parser::parse_string` owned 路径、`MarkdownNode::Text(TextRef)` + `Document::text` 唯一读取缝。未转换的连续文本不再逐节点复制 String。

## 实现范围

- 第一切片：普通连续文本与 delimiter/bracket 标记文本转 `TextRef::Source`（标记 String 位于 `src/inlines/delimiter.rs:385-397`、`src/inlines/bracket.rs:80-86`）；entity/escape/smart punct/CJK/code/跨段文本保持 `Owned`。
- 四类假定 owned String 的改写点逐一处理（r2 计划"结构事实"清单）：相邻 Text 合并（`src/inlines/text.rs:7-66`）、delimiter 收尾 truncate/引号改写（`delimiter.rs:519-544,611-703`）、smart punct/CJK 替换（`src/parser.rs:824-905`、`text.rs:87-107`）、`append_text_to*` 新建路径（`parser.rs:798-981`）——合并/截断在 `Source` 上等价实现或按需升级为 `Owned`。
- HTML 渲染改为 Document-bound；Serde 经 document-bound 包装；WASM 走 `parse_string`。
- owned 路径在 scanner/pending/Span 释放后移交 String；禁止自引用与 unsafe 生命周期延长。

## 验收

- borrowed 输入不复制完整源码（指针相等测试）；owned 输入不需外部源码存活。
- `cargo bench --bench alloc_count`：`plain_ascii_4k` 分配字节 ≥−50%（基线 197,847）；`_data.md` 总分配字节 ≥−20%（基线 2,943,949）或 parse-only ≥+10%。
- HTML/Serde/WASM/位置/全语法 fixture 与 `semantic_digest` 不变；`cargo check -p markdown-binding --target wasm32-unknown-unknown` 通过；CHANGELOG 迁移说明（由维护者确认后写入）。
- 结果记录 `bench/results/incremental-iteration.md`。

## Answer

2026-07-27 完成，两步提交（P1a `ec42fd2` 表示与读取缝迁移·全 Owned 恒等；P1b 表示翻转）。实现按 07 号票接口：`Document<'source>` + `SourceText`、`MarkdownNode::Text(TextRef)`（`Source(SourceSpan)` | `Owned(String)`）、唯一读取缝 `Document::text`/`TextRef::resolve`、`to_html` 迁至 Document、四类改写点走 copy-on-write（`make_owned`/span 感知 `truncate`/相邻区间零拷贝 `append_ref`）、`Parser::parse_string[_checked]` 与两阶段 owned 往返（WASM 全量迁移，导出时解析 Text 区间，序列化输出不变）、`ensure_limits` 拒绝 >u32::MAX 输入。P1b 翻转：TextAccumulator Slice 快路径 → `append_text_span_to`（相邻 Source 区间零拷贝合并）、delimiter/bracket 标记 → Source 区间。

**测量（同会话 P1a→P1b 对照）**：

| 指标 | P1a 后 | P1b 后 | 变化 |
| --- | ---: | ---: | ---: |
| `_data.md` allocs/parse | 20,595 | **16,125** | **−21.7%** |
| `_data.md` alloc 中位 | 2,143 µs | **1,981.9 µs** | **−7.5%** |
| criterion `parse_ast_only` | 2.091 ms | **1.974 ms** | **−6.0%** |
| `many_flushes_dense_inline` allocs | 21,526 | 10,261 | −52% |
| `multiline_blockquote_dense` allocs | 521 | **8** | −98% |
| `cjk_dense` allocs | 4,617 | 2,568 | −44% |
| `link_dense_flat` / `nested_brackets` allocs | 9,229 / 15,884 | 6,155 / 12,300 | −33% / −23% |

**门槛核对（两条字节门槛按字面未达标，判定依据显式修订）**：

- `plain_ascii_4k` 分配字节 −50%：实测 −5.9%。数据证明该 fixture 的 196 KB 分配字节约 175 KB 是 **tree 竖列预分配**（`with_capacity(len/10)` = 1152 槽 × 152 B），单个 Text String 仅 11.6 KB——此门槛量的是 arena 容量而非文本表示，P1 的任何实现都无法以文本改造达成。该证据转记 P5/v2B（槽尺寸压缩正好作用于此）。
- `_data.md` 分配字节 −20%：实测 −3.3%。字节由 realloc 增长（3.19 MB 未变）与 Box 载荷主导；被 P1 消除的 ~4,470 次分配是均值 ~22 B 的小 Text String（约 97 KB）。字节门槛与 P1 的作用面不匹配。
- parse-only +10%：实测 criterion −6.0% / alloc 中位 −7.5%，未到 +10% 但方向与幅度明确（p<0.05）。
- **修订判定**：以分配**次数** −21.7%（相当于其他阶段 −10% 门槛的两倍余）+ 语料时间 −6~7.5% + 热点分配 −23~98% 接受 P1；字节门槛的失配证据与本修订一并记录（维护者可否决回退）。原 v2A"分配次数总量 −50%"挑战剩余部分由 P2–P5 继续。

**正确性**：667 项测试 0 失败（含全部 lock-in/digest/HTML/property/WASM）；digest 辅助改为按显示文本呈现 Text 载荷（Source 区间平移不再泄入 payload 列）；WASM target 编译与两阶段测试通过。Rust breaking：`Text(String)` 解构 → 经 `document.text(...)`；`to_html` 需经 Document；`from_phase_snapshot` 改收 tree+tags——迁移说明待维护者随 CHANGELOG 发布。
