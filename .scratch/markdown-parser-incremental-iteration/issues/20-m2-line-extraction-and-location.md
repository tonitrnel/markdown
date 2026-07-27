# M2: 行提取与位置模型（设计评审先行）

**Type:** task
**Status:** resolved
**Blocked by:** 19

## 交付

两阶段：**先出独立设计评审**（本票内以 Proposal 形式提交，维护者确认后实施），再落地。目标是压缩"行提取+位置"类别（门槛通道 9.3%、OFM curated 语料 19%：`Span::extract`、`skip_to_eol`、`MergedSpan::start_location`/`location_at_byte` 的行列预计算与非 ASCII 列扫描）。

## 设计评审必须回答

- lazy location 的 API 形态：`Node.start/end` 保留现有 `Location` 语义还是改 byte-span 主表示 + 按需行列（公开 API 影响、WASM/Serde 序列化位置字段的兼容策略）。
- 行列按需计算的载体（line-start 索引的构建时机与成本；与现有 `Span` 逐行预计算的取舍）。
- `LineHead`（首字节候选收缩 matcher 集合）是否并入本票——P6 结案时 matcher 可省上限 ≈15%，与行提取同处一层，合并评估但允许结论为"不做"。
- 正确性矩阵：LF/CRLF、tab、CJK、emoji、末尾空行的位置 fixture 全覆盖（`tests/location.rs` + `cjk_friendly`）。

## 明确不做

不改 Tree 表示；不引入第二套 AST；不动 Inline 引擎。

## 验收（实施阶段）

- 门槛通道 criterion ≥−5%，或 `cjk_dense` ≥−8%（当前 351.7 µs，非 ASCII 列计算密集）。
- 全部位置 fixture 与 `semantic_digest` 位置字段不变（或按设计评审通过的兼容策略变更并记录迁移说明）。
- `_data.md` 与其余 lane ≤+2%；护栏与记录纪律同 r2。

## Proposal（设计评审，2026-07-27）

**M1 后重采样**（同法门槛通道，11,149 样本）：行+位置类 **10.1%**——`skip_to_eol` 2.7% + `MergedSpan::start_location` 2.6% + `Span::extract` 2.6% + `MergedSpan::skip` 1.0% + `location_at_byte` 0.9%；另 `parse_blocks_observed` 自身 8.0% 内联着逐行位置簿记，树三件套（append_child/drop/append_free）9.8% 将受 Node 瘦身间接惠及。

### 1. API 形态：byte-span 主表示 + 按需行列

`Node { start: Location, end: Location }`（32B）→ `Node { span: SourceSpan }`（8B，起止字节偏移）。`Location` 类型保留，经新增读取缝 `Document::location_at(offset: u32) -> Location` 按需计算；语义不变（line/column 均 1 起、column 按 Unicode 标量计数、tab=1）。兼容策略沿用 P1/M1 的 TextRef 契约：Rust 侧结构化序列化（记 breaking + 迁移说明）；**WASM 导出时物化 `start`/`end` 为 Location**，JS 侧序列化输出逐字节不变；`semantic_digest` 位置字段经 `location_at` 计算，数值必须与现值逐一相等（见矩阵）。三处列算术（delimiter.rs:464/469、newline.rs:39，均 ASCII 内容）改字节算术，语义等价且更直接。

### 2. 载体：LineIndex（行起点数组）

`Vec<u32>` 行起始偏移，**在既有扫描中顺手记录**（scanner 消耗换行符处 push，`with_capacity(len/32)`），无独立扫描遍。行查找二分 + 单调游标缓存（digest/渲染按文档序读取时 O(1) 摊销）；列 = `1 + count_chars(行起点, offset)`（复用现有 SIMD 计数，仅按需支付）。与逐行预计算的取舍：预计算为**每行无条件**付出 is_ascii 全行扫描 + line/col 捕获 + scanner 逐字节列维护，读取端却只在节点创建时消费；按需模型把这些全部移到读取缝，解析热路径只剩每行一次 push。

### 3. LineHead：本票结论「不做」

块类 top-of-stack 仅 1.1%，dispatch 成本收敛在 `parse_blocks_observed` 8% 自身时间内且与位置簿记混杂；M2 落地后若剩余 profile 中块 dispatch 自身 ≥8% 再以独立小票评估，避免与位置模型互相污染归因。

### 4. 正确性矩阵

`tests/location.rs` 新建：LF/CRLF 混合、行内 tab、CJK、emoji（含 ZWJ 序列）、末尾空行、无结尾换行、空文档，每样本全树遍历断言位置逐字段相等；`cjk_friendly` 全 fixture + CommonMark/GFM spec 套件经 digest 位置字段兜底。回滚一致性：`ScannerSnapshot` 记录 `line_starts.len()`，restore 时截断（F1 discard 路径）。

### 实施切片（每片独立提交+门禁）

- **a（增量，无 breaking）**：scanner 记录 line_starts；`Document::location_at` + 单调缓存；矩阵测试以「急切值 == 按需值」全语料断言。门禁：全 lane ≤+2%。
- **b（表示切换，breaking）**：Node 位置字段换 SourceSpan；全部创建/调整点改传字节偏移（MergedSpan 增 offset 读取器，删 `start_location`/`location_at_byte` 调用）；digest/渲染/WASM/selective 经 `location_at` 读取。门禁：digest 位置不变；门槛通道预期 −3~5%。
- **c（删簿记）**：`Span::extract` 删 is_ascii 扫描与 line/col 捕获及字段；Scanner 删逐字节 line/col 维护（快照仅 pos+索引长度）；`skip_to_eol` 归纯 memchr2。门禁：`cjk_dense` ≥−8% 预期主落点；`_data.md` ≤+2%。

**验收对照票面**：门槛通道 ≥−5% 或 `cjk_dense` ≥−8%；预期合计门槛通道 −6~10%（10.1% 直接类 + 簿记份额 + 24B/节点树侧红利）。风险登记：空文档 (1,1) 哨兵、`end_location` 的 +1 语义按调用点逐一映射为「独占端偏移」、pending 跨行 span 绝对偏移无每行状态依赖。

## Answer

2026-07-27 完成，三片提交（97b7bd4 / a486bce / affa360），Proposal 全案落地，两处按纪律修订：

- **a（增量）**：行索引从 Proposal 的「扫描中顺手记录」修订为 **`OnceLock` 首读惰性构建**——急切版配对 A/B 实测 +4.4%（多小文件语料的每解析 Vec 分配），惰性版 +0.8%（≤+2% 门禁），parse-only 分配计数与 M1 逐字节相同。矩阵测试（LF/CRLF/孤立 CR/tab/CJK/emoji/末尾空行/无结尾换行/空文档/混合行尾）朴素对照一次通过；golden 钉住急切全树位置。
- **b（表示切换，breaking）**：`Node{start,end:Location}`(32B)→`span:SourceSpan`(8B)；110 处调用点改传既有偏移，`cursor_or_end`/`char_end_offset` 为旧读取缝的偏移对应物；list 紧凑判定与渲染器同行判定改「偏移间换行计数」（与行号算术精确等价）；digest/render/WASM/标题单测经 `location_at` 按需读取。**接受两类偏差**（golden 注记，其余逐字节不变）：SoftBreak/HardBreak 的 end 幻影列（指进行终止符，纯偏移不可表示）→ 行内容终点；孤立 `\r` 后续列（急切簿记只计终止符，数值不自洽）→ 距行起点字符数。配对 **−9.1%**。
- **c（删簿记）**：Span 四元数据字段与急切位置函数全删；Scanner 行列维护全删（快照仅 pos）。配对 **−3.5%**。

**门槛（vs M1 后基线）**：

| 指标 | M1 后 | M2 后 | 门槛 | 判定 |
| --- | ---: | ---: | --- | --- |
| 门槛通道（配对复合） | 4.502 ms | **≈3.92 ms（−11.6%）** | ≥−5% | **✓✓** |
| `cjk_dense` | 315.5 µs | **203.8 µs** | ≥−8% | **−35.4% ✓** |
| `_data.md` 中位 / allocs | 1,758 µs / 10,615 | **1,491 µs** / 10,615 | ≤+2% | **−15.2% ✓**（分配零变化） |
| `_data.md` 分配字节 | 1.88 MB | **1.45 MB** | — | −24%（24B/节点树红利） |

**同轮跨解析器（大语料 / curated）**：local 3.923/1.421 ms；pulldown 2.679/0.791（**1.46x / 1.80x**，大语料入 1.5x 硬门槛内）；comrak 5.195/2.131（快 24.5%/33.3%）；rushdown 5.270/1.692（快 25.6%/**16.0%**，curated 由落后 2% 反超）——**v2B 终点 1（双数据集同类反超）达成**。OFM 附加成本 +6.2%/+4.5%（M3 目标 ≤5%）。670 项测试通过；wasm check 通过。Rust breaking：Node 字段（迁移说明随 CHANGELOG）。LineHead 维持「不做」，见 Proposal §3。
