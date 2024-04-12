#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table{
    pub(crate) column: usize
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}
