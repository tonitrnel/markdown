use crate::ast;
use crate::ast::MarkdownNode;
use crate::parser::Node;
use crate::tree::Tree;
use std::borrow::Cow::{Owned, Borrowed};
use std::fmt::Write;

fn to_html(tree: &Tree<Node>, cur: usize, writer: &mut impl Write) -> std::fmt::Result {
    let pair = match &tree[cur].body {
        MarkdownNode::Paragraph => Some((Borrowed("<p>"), "</p>")),
        MarkdownNode::Heading(heading) => Some(match heading.level() {
            ast::heading::HeadingLevel::H1 => (Borrowed("<h1>"), "</h1>"),
            ast::heading::HeadingLevel::H2 => (Borrowed("<h2>"), "</h2>"),
            ast::heading::HeadingLevel::H3 => (Borrowed("<h3>"), "</h3>"),
            ast::heading::HeadingLevel::H4 => (Borrowed("<h4>"), "</h4>"),
            ast::heading::HeadingLevel::H5 => (Borrowed("<h5>"), "</h5>"),
            ast::heading::HeadingLevel::H6 => (Borrowed("<h6>"), "</h6>"),
        }),
        MarkdownNode::Code(code) => Some(match code {
            ast::code::Code::Inline(_) => (Borrowed("<code>"), "</code>"),
            ast::code::Code::Indented(_) => (Borrowed("<pre><code>"), "</code></pre>"),
            ast::code::Code::Fenced(code) => (
                if let Some(language) = &code.language {
                    Owned(format!("<pre><code language=\"{}\">", language))
                } else {
                    Borrowed("<pre><code>")
                },
                "</code></pre>",
            ),
        }),
        MarkdownNode::Emphasis => Some((Borrowed("<em>"), "</em>")),
        MarkdownNode::Strong => Some((Borrowed("<strong>"), "</strong>")),
        MarkdownNode::Strikethrough => Some((Borrowed("<del>"), "</del>")),
        MarkdownNode::Highlighting => Some((Borrowed("<mark>"), "</mark>")),
        MarkdownNode::SoftBreak => {
            write!(writer, " ")?;
            None
        },
        MarkdownNode::HardBreak => {
            write!(writer, "<br />")?;
            None
        } ,
        MarkdownNode::Text(str) => {
            write!(writer, "{}", str)?;
            None
        },
        _ => todo!(),
    };
    if let Some((open, close)) = pair{
        write!(writer, "{}", open)?;
        if let Some(idx) = tree.get_first_child(cur) {
            to_html(tree, idx, writer)?;
        }
        write!(writer, "{}", close)?;
    }
    if let Some(idx) = tree.get_next(cur) {
        to_html(tree, idx, writer)?;
    }
    Ok(())
}
impl Tree<Node> {
    pub fn to_html(self) -> String {
        let mut string = String::new();
        if self.is_empty() {
            return string;
        }
        to_html(&self, 1, &mut string).unwrap();
        string
    }
}
