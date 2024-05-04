use crate::ast::reference::Reference;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Embed {
    pub path: String,
    pub size: Option<(u32, Option<u32>)>,
    pub reference: Option<Reference>,
    pub attrs: Option<Vec<(String, String)>>,
}
