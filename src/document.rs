use crate::{Tree, location::Location, node::Node};
use rustc_hash::FxHashSet;
use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
};

/// Document 持有或借用唯一一份源码（ADR 0001 / map ticket 07）。
pub enum SourceText<'source> {
    Borrowed(&'source str),
    Owned(String),
}

impl SourceText<'_> {
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            SourceText::Borrowed(source) => source,
            SourceText::Owned(source) => source.as_str(),
        }
    }
}

impl Default for SourceText<'_> {
    fn default() -> Self {
        SourceText::Borrowed("")
    }
}

impl Debug for SourceText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceText::Borrowed(source) => write!(f, "Borrowed({} bytes)", source.len()),
            SourceText::Owned(source) => write!(f, "Owned({} bytes)", source.len()),
        }
    }
}

#[derive(Default)]
pub struct Document<'source> {
    pub(crate) source: SourceText<'source>,
    pub tree: Tree<Node>,
    pub tags: FxHashSet<String>,
    pub(crate) line_starts: std::sync::OnceLock<Vec<u32>>,
}
impl<'source> Document<'source> {
    /// 原始源码。`TextRef::Source` 区间对其解析。
    #[inline]
    pub fn source(&self) -> &str {
        self.source.as_str()
    }
    /// 位置读取缝：按需把字节偏移换算为 1 基 `Location`。
    ///
    /// 列按 Unicode 标量计数（tab = 1）；`offset` 超出源码长度时按末尾处理，
    /// 落在多字节字符中间时按该字符起点计列。行索引在首次调用时惰性构建
    /// （parse-only 路径零成本）。
    pub fn location_at(&self, offset: usize) -> Location {
        let src = self.source.as_str();
        let starts = self
            .line_starts
            .get_or_init(|| Location::build_line_starts(src));
        let offset = offset.min(src.len());
        let line_idx = starts
            .partition_point(|&s| (s as usize) <= offset)
            .saturating_sub(1);
        let line_start = starts.get(line_idx).copied().unwrap_or(0) as usize;
        let column = 1 + crate::span::count_chars(src.as_bytes(), line_start, offset) as u64;
        Location::new(line_idx as u64 + 1, column)
    }
    /// 唯一文本读取缝：解析 Text 载荷为显示文本。
    #[inline]
    pub fn text<'doc>(&'doc self, text: &'doc crate::ast::text::TextRef) -> &'doc str {
        text.resolve(self.source.as_str())
    }
}
impl Deref for Document<'_> {
    type Target = Tree<Node>;
    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}
impl Debug for Document<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.tree.fmt(f)
    }
}
