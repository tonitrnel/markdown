//! 有序 Pending Inline 存储（P4，ticket 15；v2C C4 共享 arena 化）。
//!
//! 取代 `FxHashMap<usize, Vec<Span>>`：条目按**首次注册顺序**保存——Block
//! 节点 id 在扫描期单调分配且首行在创建时注册，因此条目顺序即文档顺序，
//! 消费端不再需要 B1 的排序。`node_id → 条目` 走稠密索引（O(1) 无哈希）。
//!
//! C4：段数据存**共享 span arena**，条目仅记 `(start, len)` 区间——leaf Block
//! 的行天然按序到达（块关闭后不再追加），逐行 push 即尾部追加，消灭了
//! 每条目 `SmallVec` 的溢出与倍增分配；罕见的非尾追加以 `extend_from_within`
//! 搬迁兜底。消费端（`remove`/drain）一次性拷出 `PendingSegments`，下游签名
//! 不变；重插入（引用定义裁剪、Setext、table 改写）回到**原条目位置**（文档
//! 顺序由条目序决定，与 arena 内位置无关）。

use crate::span::Span;
use smallvec::SmallVec;

/// 单个 Block 的 pending 源码段；一、两段（最常见）内联存储。
pub(crate) type PendingSegments<'input> = SmallVec<[Span<'input>; 2]>;

struct Entry {
    node_id: usize,
    /// 段区间在共享 arena 中的起点
    start: u32,
    len: u32,
    present: bool,
}

pub(crate) struct PendingInlines<'input> {
    /// 首次注册顺序 == 文档顺序
    entries: Vec<Entry>,
    /// 共享段 arena；存活条目的区间互不重叠（搬迁/替换产生的旧区间闲置）
    spans: Vec<Span<'input>>,
    /// node_id → entries 索引 + 1（0 = 无条目）
    index: Vec<u32>,
    /// 存活条目数
    live: usize,
    /// drain 游标（每次解析只 drain 一遍）
    drain_cursor: usize,
}

impl<'input> PendingInlines<'input> {
    pub(crate) fn with_capacity(entry_capacity: usize, span_capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(entry_capacity),
            spans: Vec::with_capacity(span_capacity),
            index: Vec::new(),
            live: 0,
            drain_cursor: 0,
        }
    }
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.live
    }
    #[inline]
    fn slot_of(&self, node_id: usize) -> Option<usize> {
        match self.index.get(node_id) {
            Some(&raw) if raw != 0 => Some(raw as usize - 1),
            _ => None,
        }
    }
    #[inline]
    fn range_of(entry: &Entry) -> std::ops::Range<usize> {
        entry.start as usize..(entry.start + entry.len) as usize
    }
    fn create_slot(&mut self, node_id: usize) -> usize {
        let slot = self.entries.len();
        self.entries.push(Entry {
            node_id,
            start: 0,
            len: 0,
            present: false,
        });
        if self.index.len() <= node_id {
            self.index.resize(node_id + 1, 0);
        }
        self.index[node_id] = (slot + 1) as u32;
        slot
    }
    /// 注册一行源码段（`append_inline` 路径）。
    pub(crate) fn push_line(&mut self, node_id: usize, line: Span<'input>) {
        let slot = match self.slot_of(node_id) {
            Some(slot) => slot,
            None => self.create_slot(node_id),
        };
        let entry = &mut self.entries[slot];
        if !entry.present {
            entry.present = true;
            self.live += 1;
            entry.start = self.spans.len() as u32;
            entry.len = 0;
        } else if (entry.start + entry.len) as usize != self.spans.len() {
            // 非尾追加（罕见）：把既有区间搬迁到 arena 尾部
            let range = Self::range_of(entry);
            let new_start = self.spans.len() as u32;
            self.spans.extend_from_within(range);
            self.entries[slot].start = new_start;
        }
        self.spans.push(line);
        self.entries[slot].len += 1;
    }
    /// 整体写入 segments（引用定义剩余、table 列等）。重插入回到原槽位。
    pub(crate) fn insert(&mut self, node_id: usize, segments: PendingSegments<'input>) {
        let slot = match self.slot_of(node_id) {
            Some(slot) => slot,
            None => self.create_slot(node_id),
        };
        let entry = &mut self.entries[slot];
        if !entry.present {
            entry.present = true;
            self.live += 1;
        }
        entry.start = self.spans.len() as u32;
        entry.len = segments.len() as u32;
        self.spans.extend(segments);
    }
    pub(crate) fn get(&self, node_id: usize) -> Option<&[Span<'input>]> {
        let slot = self.slot_of(node_id)?;
        let entry = &self.entries[slot];
        entry.present.then(|| &self.spans[Self::range_of(entry)])
    }
    /// 去掉末行（table 表头改写路径）。
    pub(crate) fn pop_line(&mut self, node_id: usize) {
        if let Some(slot) = self.slot_of(node_id) {
            let entry = &mut self.entries[slot];
            if entry.present && entry.len > 0 {
                entry.len -= 1;
                // 若区间在 arena 尾部则同步回收，保持后续 push 仍是尾追加
                if (entry.start + entry.len + 1) as usize == self.spans.len() {
                    self.spans.pop();
                }
            }
        }
    }
    #[inline]
    pub(crate) fn contains(&self, node_id: usize) -> bool {
        self.slot_of(node_id)
            .is_some_and(|slot| self.entries[slot].present)
    }
    pub(crate) fn remove(&mut self, node_id: usize) -> Option<PendingSegments<'input>> {
        let slot = self.slot_of(node_id)?;
        let entry = &mut self.entries[slot];
        if !entry.present {
            return None;
        }
        entry.present = false;
        self.live -= 1;
        let range = Self::range_of(entry);
        Some(SmallVec::from_slice(&self.spans[range]))
    }
    /// 按文档顺序取下一个存活条目（完整解析的 drain 循环）。
    pub(crate) fn take_next_in_document_order(
        &mut self,
    ) -> Option<(usize, PendingSegments<'input>)> {
        while self.drain_cursor < self.entries.len() {
            let slot = self.drain_cursor;
            self.drain_cursor += 1;
            let entry = &mut self.entries[slot];
            if entry.present {
                entry.present = false;
                self.live -= 1;
                let node_id = entry.node_id;
                let range = Self::range_of(entry);
                return Some((node_id, SmallVec::from_slice(&self.spans[range])));
            }
        }
        None
    }
    /// 文档顺序遍历存活条目的 node id（语义准备阶段的扫描）。
    pub(crate) fn live_ids_in_document_order(&self) -> impl Iterator<Item = usize> + '_ {
        self.entries
            .iter()
            .filter(|entry| entry.present)
            .map(|entry| entry.node_id)
    }
    /// 终态停止回滚：丢弃 `cutoff` 及之后节点的条目（冷路径；arena 区间闲置）。
    pub(crate) fn discard_from(&mut self, cutoff: usize) {
        for entry in &mut self.entries {
            if entry.node_id >= cutoff && entry.present {
                entry.present = false;
                entry.len = 0;
                self.live -= 1;
            }
        }
    }
}
