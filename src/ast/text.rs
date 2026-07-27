//! Source-backed Text 表示（P1，接口契约见 map ticket 07）。
//!
//! `TextRef::Source` 保存指向 Document 源码的 byte range，`TextRef::Owned`
//! 保存转换后/生成的文本。读取一律经 `Document::text`（或在解析器内部经
//! `TextRef::resolve`）完成；需要原地改写时经 `make_owned` 按需物化
//! （copy-on-write）。

use serde::Serialize;
use std::fmt::{Debug, Formatter};

/// 指向 Document 源码的半开 byte 区间 `[start, end)`，必须落在 UTF-8 边界。
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub start: u32,
    pub end: u32,
}

impl SourceSpan {
    #[inline]
    pub fn new(start: u32, end: u32) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }
    #[inline]
    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    #[inline]
    pub(crate) fn resolve<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start as usize..self.end as usize]
    }
}

impl Debug for SourceSpan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Text 载荷：源码区间或转换后的自有文本。
#[derive(Serialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum TextRef {
    /// 未经转换的连续源码切片
    Source(SourceSpan),
    /// 转换/生成/合并后的自有文本
    Owned(String),
}

impl TextRef {
    /// 解析为显示文本。`source` 必须是创建该 `Source` 区间的同一份源码。
    #[inline]
    pub fn resolve<'a>(&'a self, source: &'a str) -> &'a str {
        match self {
            TextRef::Source(span) => span.resolve(source),
            TextRef::Owned(text) => text.as_str(),
        }
    }
    /// 显示文本的字节长度。
    #[inline]
    pub fn len(&self, source: &str) -> usize {
        match self {
            TextRef::Source(span) => {
                debug_assert!(span.end as usize <= source.len());
                span.len()
            }
            TextRef::Owned(text) => text.len(),
        }
    }
    #[inline]
    pub fn is_empty(&self, source: &str) -> bool {
        self.len(source) == 0
    }
    /// 截断到 `new_len` 字节。`Source` 直接收缩区间（保持零拷贝），
    /// `Owned` 走 `String::truncate`；`new_len` 必须落在 UTF-8 边界。
    #[inline]
    pub(crate) fn truncate(&mut self, new_len: usize, source: &str) {
        match self {
            TextRef::Owned(text) => text.truncate(new_len),
            TextRef::Source(span) => {
                debug_assert!(new_len <= span.len());
                debug_assert!(source.is_char_boundary(span.start as usize + new_len));
                let _ = source;
                span.end = span.start + new_len as u32;
            }
        }
    }
    /// 追加另一段文本。两段均为 `Source` 且区间相邻时直接扩展区间
    /// （零拷贝），否则按需物化后拼接。
    #[inline]
    pub(crate) fn append_ref(&mut self, other: &TextRef, source: &str) {
        match (&mut *self, other) {
            (TextRef::Source(a), TextRef::Source(b)) if a.end == b.start => {
                a.end = b.end;
            }
            (this, other) => {
                this.make_owned(source).push_str(other.resolve(source));
            }
        }
    }
    /// copy-on-write：需要原地改写时把 `Source` 物化为 `Owned` 并返回可变引用。
    #[inline]
    pub(crate) fn make_owned(&mut self, source: &str) -> &mut String {
        if let TextRef::Source(span) = self {
            *self = TextRef::Owned(span.resolve(source).to_string());
        }
        match self {
            TextRef::Owned(text) => text,
            TextRef::Source(_) => unreachable!(),
        }
    }
}

impl Debug for TextRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // 与旧 `Text(String)` 的 Debug 形态保持一致，Source 显式标注区间
            TextRef::Owned(text) => write!(f, "{text:?}"),
            TextRef::Source(span) => write!(f, "src[{span:?}]"),
        }
    }
}

impl From<String> for TextRef {
    fn from(value: String) -> Self {
        TextRef::Owned(value)
    }
}

impl From<&str> for TextRef {
    fn from(value: &str) -> Self {
        TextRef::Owned(value.to_string())
    }
}

impl From<char> for TextRef {
    fn from(value: char) -> Self {
        TextRef::Owned(value.to_string())
    }
}
