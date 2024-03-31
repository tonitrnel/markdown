use crate::tokenizer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Code {
    Inline,
    Fenced(FencedCode),
    Indented(IndentedCode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FencedCode {
    /// 记录指定的语言
    pub language: Option<String>,
    pub length: usize,
    pub indent: usize,
    pub marker: Token<'static>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode {}
