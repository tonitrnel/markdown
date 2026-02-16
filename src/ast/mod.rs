use serde::Serialize;

pub mod block_quote;
pub mod callout;
pub mod code;
pub mod embed;
pub mod footnote;
pub mod heading;
pub mod html;
pub mod image;
pub mod link;
pub mod list;
pub mod math;
pub mod reference;
pub mod table;
pub mod thematic_break;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MarkdownNode {
    // 根节点
    Document,
    // 前言，记录 Yaml 内容，仅出现在内容顶部
    FrontMatter(Box<crate::exts::yaml::YamlMap>),
    // 段落
    Paragraph,
    // 软换行，指单个 \n
    SoftBreak,
    // 硬换行，末尾跟随空格、'\' 或多个 \n
    HardBreak,
    // 文本
    Text(String),
    // 内部嵌入
    Embed(Box<embed::Embed>),
    // 标题
    Heading(heading::Heading),
    // 重要
    Strong,
    // 强调
    Emphasis,
    // 列表
    List(Box<list::List>),
    // 列表项
    ListItem(Box<list::ListItem>),
    // 图像
    Image(Box<image::Image>),
    // 链接
    Link(Box<link::Link>),
    // 标签
    Tag(String),
    // 表情
    Emoji(String),
    // 块引用
    BlockQuote,
    // 代码
    Code(Box<code::Code>),
    // 表格
    Table(Box<table::Table>),
    TableHead,
    TableHeadCol,
    TableBody,
    TableRow,
    TableDataCol,
    // 删除线
    Strikethrough,
    // 高亮
    Highlighting,
    // 水平线
    ThematicBreak,
    // 脚注
    Footnote(Box<footnote::Footnote>),
    // 脚注列表（仅在使用某个脚注创建，不由正文生成）
    FootnoteList,
    // 数学/公式
    Math(Box<math::Math>),
    // 标注
    Callout(Box<callout::Callout>),
    // HTML
    Html(Box<html::Html>),
}
impl MarkdownNode {
    /// 是否接受目标节点
    pub fn can_contain(&self, target: &MarkdownNode) -> bool {
        match self {
            MarkdownNode::List(..) => matches!(target, MarkdownNode::ListItem(..)),
            MarkdownNode::Document
            | MarkdownNode::BlockQuote
            | MarkdownNode::Callout(..)
            | MarkdownNode::Footnote(..)
            | MarkdownNode::ListItem(..) => !matches!(target, MarkdownNode::ListItem(..)),
            MarkdownNode::Table(..) => {
                matches!(target, MarkdownNode::TableHead | MarkdownNode::TableBody)
            }
            MarkdownNode::TableHead => matches!(target, MarkdownNode::TableRow),
            MarkdownNode::TableBody => matches!(target, MarkdownNode::TableRow),
            MarkdownNode::TableRow => matches!(
                target,
                MarkdownNode::TableHeadCol | MarkdownNode::TableDataCol
            ),
            MarkdownNode::FootnoteList => matches!(target, MarkdownNode::Footnote(..)),
            _ => false,
        }
    }
    /// 是否接受纯文本行
    pub fn accepts_lines(&self) -> bool {
        matches!(
            self,
            MarkdownNode::Code(..)
                | MarkdownNode::Html(..)
                | MarkdownNode::Paragraph
                | MarkdownNode::TableHeadCol
                | MarkdownNode::TableDataCol
                | MarkdownNode::Heading(..)
        )
    }
    pub fn support_reprocess(&self) -> bool {
        matches!(self, MarkdownNode::Table(..) | MarkdownNode::TableBody)
    }
    pub fn is_inline_level(&self) -> bool {
        !self.is_block_level()
    }
    pub fn is_block_level(&self) -> bool {
        match self {
            MarkdownNode::Document
            | MarkdownNode::FrontMatter(..)
            | MarkdownNode::Paragraph
            | MarkdownNode::Heading(..)
            | MarkdownNode::List(..)
            | MarkdownNode::ListItem(..)
            | MarkdownNode::BlockQuote
            | MarkdownNode::Table(..)
            | MarkdownNode::TableHead
            | MarkdownNode::TableHeadCol
            | MarkdownNode::TableBody
            | MarkdownNode::TableRow
            | MarkdownNode::TableDataCol
            | MarkdownNode::ThematicBreak
            | MarkdownNode::Footnote(..)
            | MarkdownNode::FootnoteList
            | MarkdownNode::Callout(..) => true,
            MarkdownNode::Code(c) => matches!(
                c.as_ref(),
                code::Code::Fenced(..) | code::Code::Indented(..)
            ),
            MarkdownNode::Html(h) => matches!(h.as_ref(), html::Html::Block(..)),
            _ => false,
        }
    }
    pub fn xml_escape(&self) -> bool {
        match self {
            MarkdownNode::Html(html) => html.is_disallowed_raw_html(),
            _ => true,
        }
    }
    pub fn backslash_escape(&self) -> bool {
        !matches!(
            self,
            MarkdownNode::Code(..) | MarkdownNode::Link(..) | MarkdownNode::Html(..)
        )
    }
}
impl From<heading::HeadingLevel> for MarkdownNode {
    fn from(value: heading::HeadingLevel) -> Self {
        MarkdownNode::Heading(heading::Heading::ATX(heading::ATXHeading { level: value }))
    }
}
impl From<math::Math> for MarkdownNode {
    fn from(value: math::Math) -> Self {
        MarkdownNode::Math(Box::new(value))
    }
}
impl From<code::Code> for MarkdownNode {
    fn from(value: code::Code) -> Self {
        MarkdownNode::Code(Box::new(value))
    }
}
impl From<embed::Embed> for MarkdownNode {
    fn from(value: embed::Embed) -> Self {
        MarkdownNode::Embed(Box::new(value))
    }
}
impl From<link::Link> for MarkdownNode {
    fn from(value: link::Link) -> Self {
        MarkdownNode::Link(Box::new(value))
    }
}
impl From<image::Image> for MarkdownNode {
    fn from(value: image::Image) -> Self {
        MarkdownNode::Image(Box::new(value))
    }
}
impl From<&str> for MarkdownNode {
    fn from(value: &str) -> Self {
        MarkdownNode::Text(value.to_string())
    }
}
impl From<String> for MarkdownNode {
    fn from(value: String) -> Self {
        MarkdownNode::Text(value)
    }
}
