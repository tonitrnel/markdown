use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reference {
    Heading(String),
    MultiHeading(Vec<String>),
    BlockId(String),
}
impl Serialize for Reference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_struct("Reference", 2)?;
        match self {
            Reference::Heading(str) => {
                map.serialize_field("variant", "heading")?;
                map.serialize_field("value", str)?;
            }
            Reference::MultiHeading(str) => {
                map.serialize_field("variant", "multi-heading")?;
                map.serialize_field("value", str)?;
            }
            Reference::BlockId(str) => {
                map.serialize_field("variant", "block-id")?;
                map.serialize_field("value", str)?;
            }
        }
        map.end()
    }
}
