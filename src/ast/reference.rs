#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reference {
    Heading(String),
    BlockId(String),
}