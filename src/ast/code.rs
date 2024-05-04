use crate::tokenizer::Token;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "variant", rename_all = "kebab-case")]
pub enum Code {
    Inline(InlineCode),
    Fenced(FencedCode),
    Indented(IndentedCode),
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct FencedCode {
    /// 记录指定的语言
    pub language: Option<String>,
    pub(crate) length: usize,
    pub(crate) indent: usize,
    pub(crate) marker: Token<'static>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode {}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct InlineCode {}
