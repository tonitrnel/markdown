use crate::ast::reference::Reference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link{
    Default(DefaultLink),
    Wikilink(Wikilink)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultLink{
    pub url: String,
    pub title: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wikilink {
    pub path: String,
    pub text: Option<String>,
    pub reference: Option<Reference>
}

impl From<Wikilink> for Link {
    fn from(value: Wikilink) -> Self {
        Self::Wikilink(value)
    }
}
impl From<DefaultLink> for Link{
    fn from(value: DefaultLink) -> Self {
        Self::Default(value)
    }
}