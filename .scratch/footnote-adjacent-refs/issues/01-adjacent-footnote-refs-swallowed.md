# 相邻脚注引用被引用链接语法吞并

**Type:** task
**Status:** resolved
**Blocked by:** None.

## 现象

`Tail[^a][^b].`（a、b 均有定义）只渲染出一个指向 b 的脚注引用，`[^a]` 被当作
引用链接的链接文本吞掉。维护者在 Obsidian 实测：应渲染**两个**脚注引用。
同理 `[^a](x)` 会被解析为内联链接而非"脚注引用 + 字面 `(x)`"。

2026-07-26 由 F2 步骤 a 的探针发现并锁定（`tests/footnotes_schedule.rs` 当时
按现状锁定，本票修复后更新期望值）。

## 原因

bracket 关闭时 `scan_link_or_image` 的完整引用形式（`[text][label]`）优先于
自身内容的 shortcut 形式：`[^a][^b]` 中 `[^b]` 被取作引用标签，`[^a]` 沦为
链接文本（`src/inlines/bracket.rs`、`src/inlines/link.rs`）。

## 修复

GFM/OFM 语义：bracket **自身内容**是已定义脚注标签（`^` + label，无换行/
方括号）且非图片时，优先解析为脚注引用，不再尝试后随 `[...]`/`(...)` 形式；
后随内容留在流中按普通语法处理。`[^undefined][^b]`（未定义标签）保持原有
回退（仍走引用链接路径），记录为已知边缘。

## 验收

- `A[^x][^y].` 渲染两个引用与两个列表项；`A[^x](t).` 渲染引用 + 字面 `(t)`。
- `tests/footnotes_schedule.rs` 的 heading_and_body 期望更新为两个引用的正确输出。
- 全部既有 fixture 与 `semantic_digest` 通过（除本票预期修正外无变化）；
  `_data.md` 护栏 ≤2%。

## Comments

- 2026-07-26 维护者确认 Obsidian 渲染两个脚注，要求修复。
