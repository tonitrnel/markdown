use crate::ast::reference::Reference;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LinkVariant {
    Default,
    Internal,
    Invalid,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    variant: LinkVariant,
    label: Option<String>,
    url: Option<String>,
}

impl Link {
    fn new(url: String) -> Self {
        Self {
            variant: LinkVariant::Default,
            url: Some(url),
            label: None,
        }
    }
    fn new_with_label(url: String, label: String) -> Self {
        Self {
            variant: LinkVariant::Default,
            url: Some(url),
            label: Some(label),
        }
    }
    fn url(&self) -> &Option<String> {
        &self.url
    }
    fn label(&self) -> &Option<String> {
        &self.label
    }
    fn parse_reference(&self) -> Option<Reference> {
        self.url.as_ref()?;
        let url = self.url.as_ref().unwrap();
        if !url.contains('#') {
            return None;
        }
        let v = url.split('#').collect::<Vec<_>>();
        let v = v.get(1).unwrap();
        if v.starts_with('^') {
            Some(Reference::BlockId(v[1..].to_owned()))
        } else {
            Some(Reference::Heading(v.to_string()))
        }
    }
}
