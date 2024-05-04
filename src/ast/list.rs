use crate::tokenizer::Token;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "variant", rename_all = "kebab-case")]
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
    pub fn padding(&self) -> usize {
        match self {
            List::Bullet(it) => it.padding,
            List::Ordered(it) => it.padding,
            List::Task(it) => it.padding,
        }
    }
    pub fn set_padding(&mut self, padding: usize) {
        match self {
            List::Bullet(it) => {
                it.padding = padding;
            }
            List::Ordered(it) => {
                it.padding = padding;
            }
            List::Task(it) => {
                it.padding = padding;
            }
        }
    }
    pub fn marker_offset(&self) -> usize {
        match self {
            List::Bullet(it) => it.marker_offset,
            List::Ordered(it) => it.marker_offset,
            List::Task(it) => it.marker_offset,
        }
    }
    pub fn tight(&self) -> bool {
        match self {
            List::Bullet(it) => it.tight,
            List::Ordered(it) => it.tight,
            List::Task(it) => it.tight,
        }
    }
    pub fn set_tight(&mut self, tight: bool) {
        match self {
            List::Bullet(it) => {
                it.tight = tight;
            }
            List::Ordered(it) => {
                it.tight = tight;
            }
            List::Task(it) => {
                it.tight = tight;
            }
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BulletList {
    pub(crate) marker: Token<'static>,
    pub(crate) padding: usize,
    pub(crate) marker_offset: usize,
    pub tight: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct OrderedList {
    pub start: u64,
    pub(crate) delimiter: char,
    pub(crate) padding: usize,
    pub(crate) marker_offset: usize,
    pub tight: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TaskList {
    pub(crate) checked: bool, // - [x] task text
    pub(crate) quested: bool, // - [?] task text
    pub(crate) padding: usize,
    pub(crate) marker_offset: usize,
    pub tight: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub order: Option<u64>,    // 1. xxx
    pub checked: Option<bool>, // - [x] task text
    pub quested: Option<bool>, // - [?] task text
}
