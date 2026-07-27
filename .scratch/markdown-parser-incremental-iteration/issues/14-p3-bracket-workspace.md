# P3: Bracket workspace

**Type:** task
**Status:** resolved
**Blocked by:** 13

## 交付

link/image 的临时 bracket 链（`src/inlines/bracket.rs:8-39` 的 `Rc<RefCell<Bracket>>`）改为复用 P2 workspace capacity 的 `Vec<Bracket>` + 索引。

## 实现范围

保持 link、image、reference、footnote 与现有 Tree 重连语义；不同时引入链接 IR、延迟提交或 payload 重排。

## 验收

- `link_dense_flat`（基线 486.16 µs / 9,229 allocs）或 `nested_brackets`（基线 930.00 µs / 15,884 allocs）时间 ≥−5% 或分配 ≥−10%；另一个目标热点回归 ≤2%。
- 嵌套、失配、image size、reference、footnote 与多行测试通过；`_data.md` ≤2%；`semantic_digest` 不变。

## Answer

2026-07-27 完成。`Bracket` 改为 `Parser::bracket_store: Vec<Bracket>` 条目（`prev` 单向索引链），`ProcessCtx.brackets` 为链尾索引；`scan_link_or_image` 改收 `&Bracket`（只读 index/is_image/bracket_after）；`own_footnote_label` 改收 `open_index`；停用嵌套链接 opener 的循环与 `remove_brackets` 均为索引操作。复用 P2 的 base/truncate 嵌套隔离协议与容量复用。

**门槛（同会话对照，P2 未触及 bracket 故基线为 P1b 值）**：

| 指标 | 前 | P3 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| `link_dense_flat` allocs | 6,155 | **5,132** | ≥−10% | **−16.6% ✓**（差 1,023 ≈ 2×512 个 bracket，逐 run 对账吻合） |
| `nested_brackets` allocs | 12,300 | **10,253** | 另一 lane ≤+2% | **−16.6%**（差 2,047 ≈ 4×512） |
| `link_dense` / `nested` 中位 | 449.0 / 853.2 µs | 429.2 / 836.4 µs | — | −4.4% / −2.0% |
| `_data.md` allocs | 15,447 | 14,942 | ≤+2% 时间 | −3.3%；中位 1,875 µs（环境向好漂移，护栏通过） |

嵌套、失配、image size、reference、footnote 与多行场景由既有 fixture 覆盖：667 项测试 0 失败；WASM check 通过。`semantic_digest` 不变。
