use crate::tokenizer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum List {
    Bullet(BulletList),
    Ordered(OrderedList),
    Task(TaskList),
}

impl List {
    pub fn like(&self, target: &List) -> bool {
        match (self, target) {
            (List::Ordered(a), List::Ordered(b)) => a.delimiter == b.delimiter,
            (List::Bullet(a), List::Bullet(b)) => a.marker == b.marker,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulletList {
    pub marker: Token<'static>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedList {
    pub start: u64,
    pub delimiter: char,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskList {
    pub checked: bool, // - [x] task text
    pub quested: bool, // - [?] task text
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub order: Option<u64>,    // 1. xxx
    pub checked: Option<bool>, // - [x] task text
    pub quested: Option<bool>, // - [?] task text
}
