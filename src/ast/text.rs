//! Source-backed text storage.
//!
//! [`TextRef::Source`] stores a byte range into the owning document's source.
//! [`TextRef::Owned`] stores transformed or generated text. Prefer
//! [`crate::Document::text`] when reading text from an AST node.

use serde::Serialize;
use std::fmt::{Debug, Formatter};

/// A half-open byte range `[start, end)` into a document's UTF-8 source.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    /// Inclusive starting byte offset.
    pub start: u32,
    /// Exclusive ending byte offset.
    pub end: u32,
}

impl SourceSpan {
    /// Creates a source span from byte offsets.
    #[inline]
    pub fn new(start: u32, end: u32) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }
    /// Returns the span length in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
    /// Returns `true` when the span contains no bytes.
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

/// Text stored either as a source range or as an owned string.
#[derive(Serialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum TextRef {
    /// Unchanged text backed by the document source.
    Source(SourceSpan),
    /// Transformed, generated, or merged text owned by the node.
    Owned(String),
}

impl TextRef {
    /// Resolves this value against its original source string.
    ///
    /// Prefer [`crate::Document::text`] when a document is available.
    #[inline]
    pub fn resolve<'a>(&'a self, source: &'a str) -> &'a str {
        match self {
            TextRef::Source(span) => span.resolve(source),
            TextRef::Owned(text) => text.as_str(),
        }
    }
    /// Returns the resolved text length in bytes.
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
    /// Returns `true` when the resolved text is empty.
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
