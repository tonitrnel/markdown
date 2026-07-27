# 渐进式迭代基准记录

按 [r2 计划](../../docs/plans/2026-07-26-markdown-parser-incremental-iteration-r2_cn.md) 共同护栏追加各阶段测量结果；本文件不是 changelog。参考机器与工具链见 [issue 04](../../.scratch/markdown-parser-incremental-iteration/issues/04-trustworthy-performance-baseline.md)（Apple M1 / 16 GiB / macOS 26.5.2 / rustc 1.96.0 / criterion 默认配置）。跨会话的绝对耗时受机器状态影响（本页已观察到约 5–9% 波动）；判定以同会话前后对照与 allocation count 为准。

## 2026-07-26 — P0 基线（issue 04 摘要 + 当日复现 + 自顶栈归因）

完整基线表见 issue 04。当日复现（`dev/v2-temporary` @ `d55e180`，工作树干净）：

```text
parses=500 allocs_per_parse=20571.00 reallocs_per_parse=1910.00 deallocs_per_parse=20571.00
alloc_bytes_per_parse=2885133.00 realloc_bytes_per_parse=3195447.00 dealloc_bytes_per_parse=4492367.00
median_us=2142.77
```

分配计数与 issue 04 一致；中位耗时 2142.77 µs 对记录值 2256.04 µs（−5%，机器状态波动）。

**parse-only 自顶栈归因**（方法：`CARGO_PROFILE_BENCH_DEBUG=line-tables-only CARGO_PROFILE_BENCH_STRIP=none cargo bench --bench phase_bench --no-run`，`/usr/bin/sample <pid> 12` 采样 `parse_ast_only` 稳态；线程 9,578 样本，`Parser::parse` 子树 84.7%、`Document` 析构约 8.6%）：

| 类别 | 占线程 | 主要符号 |
| --- | ---: | --- |
| 内存子系统（malloc/free/realloc/memmove/memset/Vec 增长） | ≈32% | `_xzm_free` 797、`memmove` 612、malloc 族 ≈960 |
| Inline 处理 | ≈23% | `inlines::process` 981、`accumulate_run` 629、`flush_text_acc` 200 |
| Block 扫描（不含行提取） | ≈22% | `continue_parse_checked` 自身 1,260、`skip_to_eol` 371 |
| 行提取与位置 | ≈10% | `Span::extract` 504、`MergedSpan::start_location` 216 |
| Tree 操作与析构 | ≈8% | `append_child` 196、`drop TreeNode` 189 |
| HashMap | ≈2% | insert/rehash（其中 delimiter 链自身仅 ≈1.3%） |

## 2026-07-26 — B1 脚注编号文档顺序修复

**变更：** `Parser::parse_inlines` 不再按 `FxHashMap` 遍历顺序消费 pending inline，改经 `drain_pending_in_document_order()`（collect + 按 node id `sort_unstable_by_key`；node id 扫描期单调分配，等价文档顺序）。P4 有序存储落地后移除排序。新增回归测试 `tests/footnotes_order.rs`（跨 20 个 Block 的编号与列表顺序 + 单 Block 行为不变）。

**语义影响：** 跨多 Block 的脚注编号与脚注列表由乱序修正为文档顺序（预期修正）；单 Block 场景、semantic digest 全部 fixture 不变。622 项测试全部通过；`cargo check -p markdown-binding --target wasm32-unknown-unknown` 通过。

**测量（同会话前后对照，`_data.md` OFM）：**

| 指标 | 修复前 | 修复后 | 判定 |
| --- | ---: | ---: | --- |
| allocs_per_parse | 20,571 | 20,572 | +1（entries Vec），≪2% |
| alloc_bytes_per_parse | 2,885,133 | 2,943,949 | +58,816 B（约 1,838 条 pending × 32 B） |
| reallocs_per_parse | 1,910 | 1,910 | 不变 |
| alloc_count median_us | 2,142.77 | 2,140.21 | −0.1%，时间中性 |

criterion `parse_ast_only` 本次报告 2.1347 ms（相对其存储基线 −9%）；该差异与同会话 alloc_count 中位对照不符，归因为跨会话机器状态波动，不计为本修复收益。护栏（回归 ≤2%）通过。

## 2026-07-26 — P0 收尾与 P0.5 测量通道补齐

**变更：**

- `src/utils/cjk.rs`：`correct_cjk_spacing` 加 `#[cfg_attr(not(test), allow(dead_code))]` 并注明保留理由（生产路径只用 `_with_nouns` 变体，本封装供单元测试使用）——P0 收尾，消除唯一编译警告。
- 新增 `bench/benches/hotspot_cases.rs`：8 条 lane 的合成 fixture 生成器，`hotspots.rs` 与 `alloc_count.rs` 共用；新增 lane `link_dense_flat`（P3）、`nested_brackets`（P3）、`many_short_paragraphs`（P4）、`reference_heavy`（P4 回归）、`cjk_dense`（P7 位置模型证据），既有 3 条 lane 名称与内容不变。
- `alloc_count` 参数化为多 fixture：首行保持既有无前缀格式（`_data.md`），其后每条合成 lane 输出 `fixture=<name> ...` 行；计数器读数先于任何输出格式化分配。
- `bench/compare/native/Cargo.toml`：移除未被 bench 使用的 `comrak`/`markdown_rs`/`rushdown` 依赖（注明 v2B 按对照约束重新引入）。
- `bench/compare/polyglot/run.sh` 与 `README.md`：修复陈旧路径（`bench/third_party/polyglot` → `bench/compare/polyglot`、`fixures` → `fixtures`）；`build_markdown_it_corpus` 改为"规范语料已存在则直接复用"，避免覆盖 `parser_compare` 的 include_str! 基线语料。

**热点 lane 基线**（criterion 中位数，`cargo bench --bench hotspots`）：

| lane | 中位数 | 字节数 | allocs/parse | alloc_bytes/parse |
| --- | ---: | ---: | ---: | ---: |
| plain_ascii_4k | 7.03 µs | 11,520 | 10 | 197,847 |
| many_flushes_dense_inline | 1,178.49 µs | 21,504 | 21,526 | 899,271 |
| multiline_blockquote_dense | 55.37 µs | 14,848 | 521 | 250,104 |
| link_dense_flat | 486.16 µs | 38,400 | 9,229 | 855,087 |
| nested_brackets | 930.00 µs | 53,760 | 15,884 | 1,264,767 |
| many_short_paragraphs | 203.81 µs | 34,816 | 3,081 | 1,124,880 |
| reference_heavy | 237.52 µs | 16,456 | 4,371 | 537,498 |
| cjk_dense | 427.78 µs | 39,168 | 4,617 | 869,024 |

同批 `_data.md` 行：allocs 20,572、alloc_bytes 2,943,949、median 2,158.69 µs（与 B1 后基线一致，±1%）。

值得注意的观察：`many_flushes_dense_inline` 单次解析分配 21,526 次、realloc 字节 4.77 MB——21.5 KB 输入的分配次数超过 259 KB 的 `_data.md`，验证了内联密集路径的分配放大；`plain_ascii_4k` 仅 10 次分配但 197.8 KB 分配字节（预分配容量主导），P1 的 −50% 字节门槛将主要压缩这部分与文本 String。

**polyglot 端到端验证**（路径修复后 `bash bench/compare/polyglot/run.sh`，三语言全部产出、无 NA 行；规范语料文件未被重写）：rust_markdown parse-only 1.419 ms / goldmark 1.972 ms / cmark 1.533 ms（default_data，200 KB）；markdown_it_corpus 6.864 / 9.148 / 5.346 ms。注意该 harness 的迭代计时口径与 criterion lane 不可直接互比，仅作跨语言参考。

## 2026-07-26 — F1 顶层 Block 事件与终态停止（ticket 09）

**变更**：新增 `src/selective.rs` 公共类型与入口；`parse_blocks` 改为 `parse_blocks_observed(None)` 包装，观察者存在时在每个行边界以游标派发新近 finalized 的顶层子节点事件，`Stop` 终态回滚（卸链未接受子树、按 id 阈值清理 pending/footnotes/html_stacks/forks、还原 `last_location`）。`blocks::*` 零改动。新增 `tests/selective_blocks.rs`（9 项，含 curated 语料全遍历等价与逐边界 Stop 前缀等价）。

**语义影响**：`parse()` 路径输出不变（semantic digest、全部 fixture 一致）；2026-07-19 spec 增补 F1 实施注（行边界派发的视图弱化）。

**测量（同会话对照）**：

| 指标 | F1 前 | F1 后 | 判定 |
| --- | ---: | ---: | --- |
| allocs_per_parse（`_data.md`） | 20,572 | 20,572 | 不变 |
| alloc_bytes_per_parse | 2,943,949 | 2,943,949 | 不变 |
| criterion `parse_ast_only` | 2.1347 ms | 2.1285 ms | −0.29%，p=0.49，无变化 |

护栏（无 visitor 回归 ≤2%）通过；workspace 631 项测试 0 失败；WASM check 通过。

## 2026-07-26 — F2 语义准备与目标遍历（ticket 10，三步提交）

**变更**：a）脚注编号/出现序数/自动标签在 `parse_footnote_list` 按源码位置最终化（调度无关；`FootnoteList` 结束位置一并从"max 节点 id 的 end"改为按文档位置，规则与旧值在文档序处理下逐例相等）；b）`src/semantic.rs`：幂等的引用定义准备 + BlockId 发现扫描器（只发现不改写，仅语义准备路径调用）；c）`BlockPhase::prepare_semantic_targets` → `SemanticPhase`（Heading 预物化、前序目标索引、`visit_semantic_targets`、`finish`）。

**测量（同会话对照，`_data.md` OFM）**：

| 指标 | F1 后 | F2 后 | 判定 |
| --- | ---: | ---: | --- |
| allocs_per_parse | 20,572 | 20,599 | +27（+0.13%，脚注簿记） |
| alloc_bytes_per_parse | 2,943,949 | 2,943,821 | 持平 |
| alloc_count median_us | 2,140–2,237 带 | 2,111.90 | 带内，无回归 |
| criterion `parse_ast_only` | 2.13–2.19 ms 带 | 2.22 ms（CI 宽） | p=0.41，判定无变化 |

过程记录：初版把 BlockId 发现挂在完整解析路径上，criterion 呈现约 +2.8% 嫌疑成本；改为仅语义准备路径调用后回落。今日机器噪声带较宽（criterion 单次点估计波动 ±5–9%），判定以同会话 alloc_count 中位对照 + criterion p 值为准。

**等价性**：prepare→finish 与 `parse()` 在合成源、脚注跨 Heading、内联脚注、curated 语料四场景摘要+HTML 逐字节一致；656 项测试 0 失败；WASM check 通过。新增行为锁定：`tests/footnotes_schedule.rs`（5）、`tests/block_id_probes.rs`（8）、`tests/semantic_targets.rs`（10）。

## 2026-07-26 — 相邻脚注引用修复 + F3 选择性 Inline 物化（ticket 11）

**修复**（`9d94a57`）：`[^a][^b]` 相邻引用不再被引用链接语法吞并（维护者 Obsidian 实测确认应为两个引用）；bracket 自身内容为已定义脚注标签时优先解析为脚注引用，`[^a](t)` 同理变为"引用 + 字面括号"。lock-in 期望同步更新。

**F3**：`parse_selected_inlines[_checked]` + `SelectiveParseOutput`；共享物化器 `materialize_pending_entry`（完整/Heading/选择性三路同一实现）；子树展开去重；footnote 依赖递归工作队列；`ParseError::InvalidSelectionNode`。

**测量**：`_data.md` allocs 20,597、alloc 中位 2,116 µs（与 F2 后持平，完整解析路径零改动成本）。选择性 lane（`selective_parse`，`many_short_paragraphs` 34.8 KB）：full ≈ 200.65 µs vs select_10pct ≈ 157.05 µs（约 −22%；含完整扫描+语义准备，只记录不承诺比例）。667 项测试 0 失败；WASM check 通过。

F1–F3 全部完成：选择性解析能力端到端可用。下一票 12/P1（source-backed Text，破坏性变更起点）。

## 2026-07-27 — P1 source-backed Text（ticket 12，两步提交）

**P1a（表示与读取缝，全 Owned 恒等）**：`Document<'source>`/`SourceText`/`TextRef`/`Document::text` 读取缝/`to_html` 迁 Document/copy-on-write 改写助手/`parse_string` 族与 WASM 迁移。667 项测试恒等，分配与耗时持平。

**P1b（表示翻转）**：TextAccumulator Slice 快路径与 delimiter/bracket 标记改存源码区间，相邻区间合并零拷贝。

| 指标 | P1 前 | P1 后 | 变化 |
| --- | ---: | ---: | ---: |
| `_data.md` allocs/parse | 20,595 | 16,125 | **−21.7%** |
| `_data.md` alloc 中位 | 2,143 µs | 1,981.9 µs | −7.5% |
| criterion `parse_ast_only` | 2.091 ms | 1.974 ms | −6.0% |
| `many_flushes` / `blockquote` / `cjk` / `link` / `brackets` allocs | — | — | −52% / −98% / −44% / −33% / −23% |

**门槛修订（显式记录·P1）**：两条字节门槛按字面未达标——per-fixture 分解证明 `plain_ascii_4k` 的 196 KB 分配字节约 175 KB 为 tree 竖列预分配（1152 槽 × 152 B）、`_data.md` 字节由 realloc（3.19 MB）与 Box 载荷主导，字节门槛量的是容量预留而非文本表示。以分配次数 −21.7% + 时间 −6~7.5% + 热点分配 −23~98% 接受 P1；容量证据转记 P5/v2B。详见 ticket 12 Answer。

## 2026-07-27 — P2 delimiter workspace（ticket 13）

`Rc<RefCell<Delimiter>>` 链 → `Parser::delimiter_store`（Vec + 索引链，容量复用；嵌套 process 以 base/truncate 隔离）。门槛：`many_flushes` allocs −39.9%（10,261→6,166，差值 4,095 与 fixture 的 4,096 个 delimiter run 对账吻合）、时间 −15.8%（996→839 µs）；`_data.md` allocs −4.2%（16,125→15,447）、时间配对 min-of-3 −0.5%（1,999→1,990 µs）；`cjk_dense` allocs −19.9%。criterion `many_flushes` 累计 −24.9%。今日环境漂移显著（criterion 单次 CI 宽至 ±10%），裁决一律用 stash 配对 A/B + alloc 口径 min-of-3。667 项测试通过。

## 2026-07-27 — P3 bracket workspace（ticket 14）

`Rc<RefCell<Bracket>>` 单向链 → `Parser::bracket_store`（Vec + prev 索引，base/truncate 隔离嵌套）。门槛：`link_dense_flat` allocs −16.6%（6,155→5,132，差 1,023≈2×512 bracket 对账）、`nested_brackets` −16.6%（12,300→10,253，差 2,047≈4×512）；时间 −4.4%/−2.0%；`_data.md` allocs −3.3%（15,447→14,942）。667 项测试通过。

**累计（P0 基线 → P3）**：`_data.md` allocs 20,571 → 14,942（**−27.4%**）；alloc 中位 2,142.8 → 1,875.3 µs（会话间环境漂移下的参考值；同会话配对结论 P1 −7.5%、P2/P3 中性偏正）。

## 2026-07-27 — P4 有序 Pending Inline 存储（ticket 15）

`FxHashMap<usize, Vec<Span>>` → `src/pending.rs::PendingInlines`（文档序条目 + 稠密索引 + SmallVec 单双段内联 + 原位重插入）；`MergedSpan` 同构换 SmallVec；B1 排序移除（游标 drain）。门槛：`many_short_paragraphs` allocs 2,056→**6**（−99.7%）、时间 −24.8%（179.5→134.9 µs）；`reference_heavy` −3.2%；`_data.md` allocs −23.5%（14,942→**11,437**）、alloc_bytes −21.6%（2.77→**2.17 MB**，P1 的语料字节 −20% 门槛就此达成）。667 项测试通过。

**累计（P0 → P4）**：`_data.md` allocs 20,571 → **11,437（−44.4%）**；alloc_bytes 2.89 → 2.17 MB（−24.8%）。v2A"分配 −50%"挑战完成 89%。

## 2026-07-27 — P5 Tree 状态与链接压缩（ticket 16）

`frees: FxHashSet` → `free_flags: Vec<bool>`；4 个链接字段 `Option<usize>` → `Option<NonZeroUsize>`（`TreeNode<Node>` 152→≤120 B/槽，尺寸测试断言；槽 0 不变量 debug_assert 固化）。门槛：`many_flushes` 时间 **−18.8%**（898.7→729.8 µs，allocs 持平=纯布局收益）；`_data.md` 时间 −4.5%（1,796.4 µs）、alloc_bytes −12.3%（1.90 MB，预分配槽 ×32 B 对账）；criterion `parse_ast_only` **1.8327 ms**（CI 紧，会话最佳）。668 项测试通过。

**累计（P0 → P5）**：allocs −44.4%（11,430）；alloc_bytes **−34.1%**（1.90 MB）；criterion parse-only 约 −14%（1.833 ms）。剩余：P6（条件性）、P7（重新决策）。

## 2026-07-27 — P6 结案 + v2A 收官测量（tickets 17–18 证据）

**P6**：证据不足，不实施——P5 后归因显示 matcher 分派可省上限 ≈15% < 25% 触发线；行提取+位置升至 ≈19%（相对占比），归因移交 P7。

**v2A 记分（P0 → P5，同参考机器）**：

| 项 | P0 | 现在 | 变化 |
| --- | ---: | ---: | ---: |
| CommonMark 大语料 vs pulldown | 2.80x（7.87ms） | **1.96x（5.38ms）** | **2x 门槛达成** |
| 共享 GFM 大语料 | 2.61x（7.32ms） | **1.99x（5.60ms）** | **2x 门槛达成** |
| OFM 产品大语料 | 7.41ms | 6.28ms | −15.3%（5ms 未达） |
| `_data.md` 分配 | 20,571 / 2.89MB | **11,430 / 1.90MB** | −44.4% / −34.1%（−50% 未达） |
| parse+HTML CM 大语料 | 10.73ms | 7.93ms | −26% |
| hotspots 全 lane | — | — | −3.7% ～ −38.3% |

P7 证据包与三选一对位见 ticket 18；建议 v2B 候选顺位：行提取+位置（≈19%）、受限 Inline commit（≈27%）。

## 2026-07-27 — comrak/rushdown 完整 AST 对照（维护者要求；同轮横向）

parse-only：大语料 CM 本地 5.364 vs comrak 5.319 vs rushdown 5.411 ms（±1% 打平，本地略胜 rushdown）；curated CM 本地 1.743 vs comrak 2.171（+19.7%）vs rushdown 1.707；GFM 大语料本地 5.467 vs comrak 5.287。parse+HTML：大语料 CM 本地 7.18 vs rushdown 6.64 / comrak 6.52——渲染侧落后 8–10%。结论重校准与 v2B 顺位修订见 ticket 18 证据补充。lane 约束：rushdown 核心为 CommonMark 仅进 CM 通道；comrak 在共享 GFM 开 table/strikethrough/tasklist；构造均移出计时循环。

## 2026-07-27 — v2B 门槛通道热点归因（维护者要求；sample 14s，CM 大语料 parse-only）

方法：`CARGO_PROFILE_BENCH_DEBUG=line-tables-only` 构建 parser_compare，`/usr/bin/sample 14` 采样 `parse_only/commonmark/markdown/markdown_it_corpus` 稳态（10,909 自顶栈样本）。

| 类别 | 占比 | 备注 |
| --- | ---: | --- |
| 内存子系统 | **30.7%** | malloc/free/memmove/realloc 族 |
| Inline-其它 | 19.1% | `inlines::process` 11.6%、`accumulate_run` 4.7% 等 |
| **Inline-链接/转义文本** | **14.9%** | `percent_encode` 4.4%、`backslash_unescape` 4.3%、`scan_link_url` 3.5%、`normalize_reference` 1.2%、`entities` 0.8% |
| Block 扫描 | 12.2% | `parse_blocks_observed` 6.9% 等 |
| Tree+析构 | 9.4% | `append_child` 4.2%、drop 2.5% |
| 行提取+位置 | 9.3% | 与 OFM curated 归因（19%）差异显著——语料构成不同 |

**v2B 目标账本**（spec：1.5x 硬门槛 / 1.2x 冲刺；维护者工作目标 <1.25x）：大语料 CM 5.36 ms / pulldown 2.73 ms = 1.97x；1.5x 需 ≤4.09 ms（−24%）、1.25x 需 ≤3.41 ms（−36%）、1.2x 需 ≤3.27 ms（−39%）。

**顺位修订建议（按门槛通道证据）**：M1 改为**链接/转义文本下沉**——URL/title/label 的 owned String 改 span/Cow 表示，percent_encode/unescape 延迟到读取或渲染（多数为恒等变换，P1 已验证的套路；直击 14.9% 并同步压内存子系统份额）；M2 = 行提取+位置（9.3–19% 视语料）；M3 = 受限 Inline commit / OFM 转换文本下沉。到 1.5x（−24%）以 M1+M2 现实可期；1.2–1.25x（−36~39%）需三项全兑现，逐 module 验后再评。

## 2026-07-27 — v2B M1 链接/转义文本下沉（ticket 19，两步提交）

phase a：encode/unescape/backslash 三重 Cow 恒等快路径 + 四连分配链重排 + normalize_reference 快路径。phase b：`DefaultLink`/`Image` 载荷 TextRef 化（`ScannedLink` 判别枚举），恒等 inline URL 存源码区间。

门槛通道 **5.364→4.502 ms（−16.1%）**，vs pulldown **1.97x→1.64x**；**同类反超达成**（快 comrak 15%、rushdown 17%）。链接 lane 分配合计 −66.6%（15,380→5,140）；`_data.md` allocs 10,615（累计 −48.4%，距 −50% 余 330）。668 项测试通过，输出逐字节不变。

## 2026-07-27 — v2B M2 行提取与位置模型（ticket 20，三片提交）

a：OnceLock 惰性行索引 + `Document::location_at` 读取缝 + 位置矩阵/golden（急切版 +4.4% 被配对 A/B 否决，修订为惰性 +0.8%）。b：Node 位置换 byte-span 主表示（32B→8B），全部创建点改传偏移，两类 `\r`/行界哨兵偏差经评审接受（SoftBreak 幻影列、孤立 CR 列）。c：Span/Scanner 急切行列簿记全删。

配对复合 **−11.6%**：门槛通道 4.502→**≈3.92 ms**；同轮 vs pulldown **1.46x**（1.5x 硬门槛内）；**双数据集同类反超达成**（快 comrak 24.5%/33.3%、rushdown 25.6%/16.0%）。`cjk_dense` −35.4%；`_data.md` −15.2% 时间、分配零变化、字节 1.88→1.45 MB（−24%）。OFM 附加 +6.2%/+4.5%。

## 2026-07-27 — v2B M3 代码文本下沉 + 扫描环门控（ticket 21）

清点定案：`_data.md` 10,615 次分配中行内代码 Box+String ≈27%；OFM +6.2% 超额归因 `accumulate_run` 断 run 往返（真实 wikilink 扫描仅 +1.5‰）。落地：行内代码/Fenced literal 恒等存 `Text::Source`；`scan_gate` 一字节前瞻下沉扫描环（与 `should_try_special` 同规则，跨 Span 保守放行）。

**v2B 三项终点全部达成**：OFM 差值 **+0.28%/+2.6%**（门 ≤5%）；`_data.md` 分配 **8,205（对 P0 −60.1%**，门 −50%）；同类反超保持（同轮快 comrak 26.5%/37.6%、rushdown ≥26%/23.1%）。门槛通道配对再 **−2.1%** 至 3.80 ms（vs pulldown 1.42x）；`_data.md` −9.2% 至 1,354 µs；cjk 持平。671 测试逐字节不变。

## 2026-07-27 — v2C C2+C5 行扫描快路径与派发信号门（ticket 23）

Scanner `no_cr` 单针 memchr 行扫描；顶层完成计数门控观察者派发；新增 v2C 命名通道（block_only/_data、session_prepare/_data、session_prepare/corpus）。配对：block_only −5.0%（618→587 µs）、session −2.5%/−1.5%；全量护栏 +1.2%（通道双休眠，漂移带内）。670 测试不变。

## 2026-07-27 — v2C C3 惰性语义准备 + ref_text（ticket 24）

prep 摘除急切 heading 物化；ref_text 按需真引擎物化+投影（修订自剥离 pass，保真构造性精确）；重采样纠正归因后扩展范围：目标增量收集（杀全树遍历）、引用定义提取惰性至首次物化。配对：session_prepare/corpus **−38.2%**（1.767→1.092 ms，v2C 终点 ≤1,100 µs 命中）、session_prepare/_data −11.4%；全量护栏 +0.2%；671 测试含新等价锁定全绿。

## 2026-07-27 — v2C C4 会话分配外形（ticket 25）

harness 会话变体取证：SmallVec 段存储溢出统治分配时间。Pending 共享 span arena（条目记区间、尾追加、拷出保签名）+ 树 arena 相位右尺寸（构造 4096 槽上限、inline 相位按 pending 一次 reserve）。配对：block_only **−8.0%**（595→548 µs）、session_prepare/_data −7.3%、会话 realloc −96%（733→28）；全量 −0.7% 反哺、realloc −52%。671 测试全绿。

## 2026-07-27 — v2C C1 LineHead 位集分派（ticket 26，门禁未达保留局部）

LineHead [u16;256] 候选位集（保优先级序；动态索引版 +1.45% 被护栏抓获，改展开宏保内联后消除）。配对：block_only −1.6%、全量 −0.6%。前提证伪：35.1% 主体是容器下降/行循环（continuation 路径，matcher 仅新块起点跑）——P6 警示条款应验，方向测到底。

**v2C 结算**：corpus 会话 1,875→1,012 µs ✓；_data 会话 728→596 µs（≤500 未达，余为域外状态机议题）；全量净改善。tickets 22–26 全部 resolved。

## 2026-07-27 — WASM 交付面证据冲刺（ticket 27，新图定向）

bench/compare/wasm 落地（vs markdown-it/marked/remark/commonmark.js）。纯解析同类第一（2.62 ms/_data，赢 markdown-it 2.2x）；**`.tree` 边界序列化 +13.6 ms = 产品链路主宰**（原生解析的 10 倍）；SIMD 构建 parse −15~35%。v2C selective 是解药但未暴露到 binding。新决策图候选：W1 边界策略、W2 selective 暴露、W3 SIMD 固化。

## 2026-07-27 — W2+W3：selective 过 wasm binding + SIMD 固化（tickets 28/30）

`query_semantic_targets`（块相位+语义准备+ref_text，无全树序列化）+ `parse_selected`（owned-source F3 通道）+ TS 类型。**目标寻址全链路 0.98 ms/_data、3.07 ms/corpus**——vs 被迫全树 16.4/47.6（16.7x/15.5x）、vs markdown-it tokens 5.7x/8.1x。SIMD 经 .cargo/config.toml 固化，wasm-opt 补 --enable-simd。674 测试全绿。W1（全树边界策略，ticket 29）留待产品占比证据。

## 2026-07-27 — W1 全树边界直写序列化（ticket 29）

JSON 往返原型证伪（仅 −4%，瓶颈在克隆图构建+JSON 体积+V8 parse 三段）；改直写序列化器（零克隆遍历、紧凑 v2 形状：字节偏移、空值省略）→ JSON −42%（978 KB）。端到端 **16.5→8.30 ms（2.0x）/_data、48.2→21.97 ms（2.2x）/corpus——corpus 全 AST 路径反超 markdown-it tokens**。≤8 ms 门差 3.7% 近未达（V8 parse 地板），激进格式留待产品证据。675 测试全绿。
