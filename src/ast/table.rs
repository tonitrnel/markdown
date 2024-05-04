use serde::{Serialize, Serializer};

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
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

impl Serialize for Alignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
        };
        serializer.serialize_str(value)
    }
}
