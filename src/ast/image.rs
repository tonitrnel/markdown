use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Image {
    /// 恒等 URL 保存源码区间（P1/M1 的 TextRef 模式），经 `Document::text` 解析
    pub url: crate::ast::text::TextRef,
    pub title: Option<crate::ast::text::TextRef>,
    pub size: Option<(u32, Option<u32>)>,
}
