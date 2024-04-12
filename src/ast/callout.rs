#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalloutType {
    Note,
    Abstract,
    Info,
    Todo,
    Tip,
    Success,
    Question,
    Warning,
    Failure,
    Danger,
    Bug,
    Example,
    Quote,
    Custom(String),
}
impl From<&str> for CalloutType{
    fn from(value: &str) -> Self {
        let lowercase_value = value.to_lowercase();
        // https://help.obsidian.md/Editing+and+formatting/Callouts
        match lowercase_value.as_str() {
            "note" => CalloutType::Note,
            "abstract" | "summary" | "tldr" => CalloutType::Abstract,
            "info" => CalloutType::Info,
            "todo" => CalloutType::Todo,
            "tip" | "hint" | "important" => CalloutType::Tip,
            "success" | "check" | "done" => CalloutType::Success,
            "question" | "help" | "faq" => CalloutType::Question,
            "warning" | "caution" | "attention" => CalloutType::Warning,
            "failure" | "fail" | "missing" => CalloutType::Failure,
            "danger" | "error" => CalloutType::Danger,
            "bug" => CalloutType::Bug,
            "example" => CalloutType::Example,
            "quote" | "cite" => CalloutType::Quote,
            _ => CalloutType::Custom(lowercase_value)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Callout {
    _type: CalloutType,
    title: Option<String>,
    foldable: Option<bool>
}
