#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalloutType {
    Success,
    Info,
    Warning,
    Error,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Callout {
    _type: CalloutType,
}