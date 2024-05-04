use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Footnote {
    pub label: String,
    pub ref_count: usize,
}
