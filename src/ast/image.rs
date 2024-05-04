use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub url: String,
    pub title: Option<String>,
}
