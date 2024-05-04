use crate::ast::reference::Reference;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "variant", rename_all = "kebab-case")]
pub enum Link {
    Default(DefaultLink),
    Wikilink(Wikilink),
    Footnote(FootnoteLink),
    FootnoteBackref(FootnoteBackref),
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DefaultLink {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Wikilink {
    pub path: String,
    pub text: Option<String>,
    pub reference: Option<Reference>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct FootnoteLink {
    pub footnote_label: String,
    pub index: usize,
    pub ref_count: usize,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct FootnoteBackref {
    pub footnote_label: String,
    pub index: usize,
}
impl From<Wikilink> for Link {
    fn from(value: Wikilink) -> Self {
        Self::Wikilink(value)
    }
}
impl From<DefaultLink> for Link {
    fn from(value: DefaultLink) -> Self {
        Self::Default(value)
    }
}
