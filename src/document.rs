use crate::{Tree, location::Location, node::Node};
use rustc_hash::FxHashSet;
use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
};

/// Source storage retained by a parsed [`Document`].
///
/// Borrowed documents are produced by [`crate::Parser::parse`]. Owned documents
/// are produced by [`crate::Parser::parse_string`] and are convenient across
/// async, FFI, and WASM boundaries.
pub enum SourceText<'source> {
    /// Source borrowed from the parser input.
    Borrowed(&'source str),
    /// Source owned by the document.
    Owned(String),
}

impl SourceText<'_> {
    /// Returns the source as a string slice regardless of storage mode.
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

/// A parsed Markdown document.
///
/// The document owns the AST [`Tree`] and retains the source needed to resolve
/// zero-copy [`crate::ast::text::TextRef`] values. Node ID `0` is the document
/// root.
#[derive(Default)]
pub struct Document<'source> {
    pub(crate) source: SourceText<'source>,
    /// Arena-backed Markdown syntax tree.
    pub tree: Tree<Node>,
    /// Tags discovered while parsing. Iteration order is unspecified.
    pub tags: FxHashSet<String>,
    pub(crate) line_starts: std::sync::OnceLock<Vec<u32>>,
}
impl<'source> Document<'source> {
    /// Returns the original Markdown source.
    #[inline]
    pub fn source(&self) -> &str {
        self.source.as_str()
    }
    /// Converts a byte offset into a one-based [`Location`].
    ///
    /// Columns count Unicode scalar values, with a tab counting as one column.
    /// Offsets past the end are clamped. The line index is built lazily on the
    /// first call.
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
    /// Resolves a source-backed or owned text value to display text.
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
