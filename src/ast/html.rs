

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HTMLAttribute {
    name: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HTMLElement {
    pub inline: bool,
    pub tag: String,
    pub attributes: Vec<HTMLAttribute>,
    pub children: Option<String>,
}