use crate::tokenizer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeVariant {
    Inline,
    Fenced,
    Indented,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code {
    pub variant: CodeVariant,
    /// 用于 Fenced, 记录指定的语言
    pub language: Option<String>,
    /// 用于 Fenced，记录 Marker 的数量
    pub length: usize,
    /// 用于 Fenced, 记录缩进，如果开始处有缩进则内容行删除等效的缩进
    pub indent: usize,
    pub marker: Token<'static>,
}
