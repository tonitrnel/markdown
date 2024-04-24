#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Footnote {
    pub label: String,
    pub ref_count: usize,
}
