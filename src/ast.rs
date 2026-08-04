//! Markdown AST node kinds and their syntax-specific payload types.

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
pub mod text;
pub mod thematic_break;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
/// The syntax kind and payload stored in a [`crate::Node`].
///
/// Use [`MarkdownNode::is_block_level`] and [`MarkdownNode::is_inline_level`]
/// when generic tree processing needs to distinguish structural and inline
/// nodes.
pub enum MarkdownNode {
    /// Document root. This is always node ID `0`.
    Document,
    /// Parsed frontmatter at the start of the document.
    FrontMatter(Box<crate::exts::yaml::YamlMap>),
    /// Paragraph container.
    Paragraph,
    /// Soft line break.
    SoftBreak,
    /// Hard line break.
    HardBreak,
    /// Source-backed or owned text.
    Text(text::TextRef),
    /// OFM embedded file or note.
    Embed(Box<embed::Embed>),
    /// ATX or Setext heading.
    Heading(heading::Heading),
    /// Strong emphasis.
    Strong,
    /// Emphasis.
    Emphasis,
    /// Ordered, bullet, or task list.
    List(Box<list::List>),
    /// List item.
    ListItem(Box<list::ListItem>),
    /// Image.
    Image(Box<image::Image>),
    /// Link, wikilink, or footnote link.
    Link(Box<link::Link>),
    /// OFM tag.
    Tag(String),
    /// Emoji shortcode or value.
    Emoji(String),
    /// Block quote container.
    BlockQuote,
    /// Inline, fenced, or indented code.
    Code(Box<code::Code>),
    /// GFM table container.
    Table(Box<table::Table>),
    /// Table head section.
    TableHead,
    /// Table header cell.
    TableHeadCol,
    /// Table body section.
    TableBody,
    /// Table row.
    TableRow,
    /// Table body cell.
    TableDataCol,
    /// GFM strikethrough.
    Strikethrough,
    /// OFM highlighting.
    Highlighting,
    /// Thematic break.
    ThematicBreak,
    /// Footnote definition or inline footnote.
    Footnote(Box<footnote::Footnote>),
    /// Generated footnote list.
    FootnoteList,
    /// Inline or block math.
    Math(Box<math::Math>),
    /// OFM callout.
    Callout(Box<callout::Callout>),
    /// Raw HTML block, inline HTML, comment, or JSX-like element.
    Html(Box<html::Html>),
}
impl MarkdownNode {
    /// Returns whether this node kind may directly contain `target`.
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
    /// Returns whether this node kind accepts source lines during block parsing.
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
    /// Returns whether this node kind may be reprocessed by the block parser.
    pub fn support_reprocess(&self) -> bool {
        matches!(self, MarkdownNode::Table(..) | MarkdownNode::TableBody)
    }
    /// Returns `true` for inline-level syntax nodes.
    pub fn is_inline_level(&self) -> bool {
        !self.is_block_level()
    }
    /// Returns `true` for block-level syntax nodes.
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
    /// Returns whether text under this node should be XML-escaped when rendered.
    pub fn xml_escape(&self) -> bool {
        match self {
            MarkdownNode::Html(html) => html.is_disallowed_raw_html(),
            _ => true,
        }
    }
    /// Returns whether backslash escapes apply under this node.
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
        MarkdownNode::Text(text::TextRef::from(value))
    }
}
impl From<String> for MarkdownNode {
    fn from(value: String) -> Self {
        MarkdownNode::Text(text::TextRef::Owned(value))
    }
}
impl From<text::TextRef> for MarkdownNode {
    fn from(value: text::TextRef) -> Self {
        MarkdownNode::Text(value)
    }
}
