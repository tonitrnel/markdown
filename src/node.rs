use crate::ast::MarkdownNode;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

#[derive(Serialize)]
pub struct Node {
    pub body: MarkdownNode,
    /// 源码字节区间（主表示，M2）；行列 `Location` 经 `Document::location_at` 按需换算。
    pub span: crate::ast::text::SourceSpan,
    pub(crate) processing: bool,
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
