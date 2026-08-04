use crate::ast::MarkdownNode;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

#[derive(Serialize)]
/// A node stored in the document's arena-backed syntax tree.
pub struct Node {
    /// Markdown syntax represented by this node.
    pub body: MarkdownNode,
    /// Half-open byte range in [`crate::Document::source`].
    ///
    /// Convert either endpoint to line and column coordinates with
    /// [`crate::Document::location_at`].
    pub span: crate::ast::text::SourceSpan,
    pub(crate) processing: bool,
    /// Optional OFM block ID without the leading `^`.
    pub id: Option<Box<String>>,
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.body)
    }
}
impl Node {
    pub(crate) fn new(body: MarkdownNode, offset: u32) -> Self {
        Self {
            body,
            span: crate::ast::text::SourceSpan {
                start: offset,
                end: offset,
            },
            processing: true,
            id: None,
        }
    }
}
