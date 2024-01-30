#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub alt: String,
    pub title: Option<String>,
    pub size: Option<(String, String)>,
}