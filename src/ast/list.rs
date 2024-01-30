
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListVariant{
    Default,
    Ordered,
    Task
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    variant: ListVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub order: Option<u64>, // 1. xxx
    pub checked: Option<bool>, // - [x] task text
    pub every: Option<bool> // - [?] task text
}