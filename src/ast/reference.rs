#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reference {
    Heading(String),
    MultiHeading(Vec<String>),
    BlockId(String),
}
