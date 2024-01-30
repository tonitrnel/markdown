
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeVariant{
    Inline,
    Fenced,
    Indented,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code {
    pub variant: CodeVariant,
    pub language: Option<String>,
}