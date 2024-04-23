use crate::{ast, utils};
use crate::ast::{link, MarkdownNode};
use crate::parser::Node;
use crate::tree::Tree;
use std::borrow::Cow::{Borrowed, Owned};
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
        MarkdownNode::Link(link::Link::Default(link)) => {
            let title = format_title_attr(&link.title);
            Some((Owned(format!("<a href=\"{}\"{title}>", link.url)), "</a>"))
        }
        MarkdownNode::Image(img) => {
            let mut alt = String::new();
            if let Some(child) = tree.get_first_child(cur) {
                to_pure_text(tree, child, &mut alt, true)?;
            }
            let title = format_title_attr(&img.title);
            write!(
                writer,
                "<img src=\"{}\" alt=\"{}\"{} />",
                img.url, alt, title
            )?;
            None
        }
        MarkdownNode::SoftBreak => {
            writeln!(writer)?;
            None
        }
        MarkdownNode::HardBreak => {
            write!(writer, "<br />")?;
            None
        }
        MarkdownNode::Text(_) => {
            to_pure_text(tree, cur, writer, false)?;
            None
        }
        _ => todo!(),
    };
    if let Some((open, close)) = pair {
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
fn to_pure_text(
    tree: &Tree<Node>,
    cur: usize,
    writer: &mut impl Write,
    include_next: bool,
) -> std::fmt::Result {
    if let MarkdownNode::Text(str) = &tree[cur].body {
        if str.contains(['&', '<', '>', '"']) {
            write!(writer, "{}", utils::escape_xml(str))?;
        } else {
            write!(writer, "{}", str)?;
        }
    } else if let Some(idx) = tree.get_first_child(cur) {
        to_pure_text(tree, idx, writer, true)?;
    }
    if let Some(idx) = tree.get_next(cur).filter(|_| include_next) {
        to_pure_text(tree, idx, writer, true)?;
    }
    Ok(())
}
fn format_title_attr(title: &Option<String>) -> String {
    if let Some(title) = &title {
        format!(" title=\"{}\"", title)
    } else {
        "".to_string()
    }
}
