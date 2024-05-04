use serde::ser::SerializeMap;
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
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            Reference::Heading(str) => {
                map.serialize_entry("variant", "heading")?;
                map.serialize_entry("value", str)?;
            }
            Reference::MultiHeading(str) => {
                map.serialize_entry("variant", "multi-heading")?;
                map.serialize_entry("value", str)?;
            }
            Reference::BlockId(str) => {
                map.serialize_entry("variant", "block-id")?;
                map.serialize_entry("value", str)?;
            }
        }
        map.end()
    }
}
