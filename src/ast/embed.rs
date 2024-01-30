#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Embed {
    pub(crate) attributes: Vec<(String, Option<String>)>,
}
