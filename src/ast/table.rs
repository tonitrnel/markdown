#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub column: usize,
    pub alignments: Vec<Alignment>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}
