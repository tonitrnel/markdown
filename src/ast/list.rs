use serde::Serialize;

/// Marker type for bullet lists
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletMarker {
    Hyphen,
    Plus,
    Asterisk,
}

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
            (List::Task(_), List::Task(_)) => true,
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
    pub(crate) marker: BulletMarker,
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
    pub(crate) task: Option<char>, // - [ ] task char
    pub(crate) padding: usize,
    pub(crate) marker_offset: usize,
    pub(crate) obsidian: bool,
    pub tight: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ListItem {
    Bullet(BulletItem),
    Ordered(OrderedItem),
    Task(TaskItem),
}

impl ListItem {
    pub fn padding(&self) -> usize {
        match self {
            ListItem::Bullet(it) => it.padding,
            ListItem::Ordered(it) => it.padding,
            ListItem::Task(it) => it.padding,
        }
    }
    pub fn marker_offset(&self) -> usize {
        match self {
            ListItem::Bullet(it) => it.marker_offset,
            ListItem::Ordered(it) => it.marker_offset,
            ListItem::Task(it) => it.marker_offset,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BulletItem {
    #[serde(skip)]
    pub(crate) padding: usize,
    #[serde(skip)]
    pub(crate) marker_offset: usize,
}
impl From<BulletItem> for ListItem {
    fn from(value: BulletItem) -> Self {
        ListItem::Bullet(value)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct OrderedItem {
    pub start: u64,
    #[serde(skip)]
    pub(crate) padding: usize,
    #[serde(skip)]
    pub(crate) marker_offset: usize,
}
impl From<OrderedItem> for ListItem {
    fn from(value: OrderedItem) -> Self {
        ListItem::Ordered(value)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TaskItem {
    pub task: Option<char>,
    #[serde(skip)]
    pub(crate) padding: usize,
    #[serde(skip)]
    pub(crate) marker_offset: usize,
}
impl From<TaskItem> for ListItem {
    fn from(value: TaskItem) -> Self {
        ListItem::Task(value)
    }
}
