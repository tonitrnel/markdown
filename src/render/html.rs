use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::fmt;
use std::fmt::Write;

use crate::ast::{html, image, link, list, table, MarkdownNode};
use crate::parser::Node;
use crate::tree::Tree;
use crate::{ast, utils};

struct HtmlRender<'input, W> {
    writer: &'input mut W,
    tree: &'input Tree<Node>,
}

impl<'input, W> HtmlRender<'input, W>
where
    W: Write,
{
    fn new(tree: &'input Tree<Node>, writer: &'input mut W) -> Self {
        Self { tree, writer }
    }
    fn write_html(&mut self, idx: usize) -> fmt::Result {
        let pair = match &self.tree[idx].body {
            MarkdownNode::Document => Some((Borrowed(""), Borrowed(""))),
            MarkdownNode::Paragraph => Some((Borrowed("<p>"), Borrowed("</p>"))),
            MarkdownNode::Heading(heading) => Some(match heading.level() {
                ast::heading::HeadingLevel::H1 => (Borrowed("<h1>"), Borrowed("</h1>")),
                ast::heading::HeadingLevel::H2 => (Borrowed("<h2>"), Borrowed("</h2>")),
                ast::heading::HeadingLevel::H3 => (Borrowed("<h3>"), Borrowed("</h3>")),
                ast::heading::HeadingLevel::H4 => (Borrowed("<h4>"), Borrowed("</h4>")),
                ast::heading::HeadingLevel::H5 => (Borrowed("<h5>"), Borrowed("</h5>")),
                ast::heading::HeadingLevel::H6 => (Borrowed("<h6>"), Borrowed("</h6>")),
            }),
            MarkdownNode::Code(code) => Some(match code {
                ast::code::Code::Inline(_) => (Borrowed("<code>"), Borrowed("</code>")),
                ast::code::Code::Indented(_) => {
                    (Borrowed("<pre><code>"), Borrowed("</code></pre>"))
                }
                ast::code::Code::Fenced(code) => (
                    if let Some(language) = &code.language {
                        Owned(format!("<pre><code class=\"language-{}\">", language))
                    } else {
                        Borrowed("<pre><code>")
                    },
                    Borrowed("</code></pre>"),
                ),
            }),
            MarkdownNode::Emphasis => Some((Borrowed("<em>"), Borrowed("</em>"))),
            MarkdownNode::Strong => Some((Borrowed("<strong>"), Borrowed("</strong>"))),
            MarkdownNode::Strikethrough => Some((Borrowed("<del>"), Borrowed("</del>"))),
            MarkdownNode::Highlighting => Some((Borrowed("<mark>"), Borrowed("</mark>"))),
            MarkdownNode::Link(link::Link::Default(link)) => {
                let title = Self::format_title_attr(&link.title);
                Some((
                    Owned(format!("<a href=\"{}\"{title}>", link.url)),
                    Borrowed("</a>"),
                ))
            }
            MarkdownNode::Link(link::Link::Footnote(link)) => {
                let ref_count = if link.ref_count == 1 {
                    Borrowed("")
                } else {
                    Owned(format!("-{}", link.ref_count))
                };
                let id = format!("cont-fn-ref-{}{ref_count}", link.footnote_label);
                let href = format!("#cont-fn-{}", link.footnote_label);
                write!(
                    self.writer,
                    "<a href={href:?} id={id:?}>[{}]</a>",
                    link.index
                )?;
                None
            }
            MarkdownNode::Link(link::Link::FootnoteBackref(backref)) => {
                let index = if backref.index == 1 {
                    Borrowed("")
                } else {
                    Owned(format!("-{}", backref.index))
                };
                let sup = if backref.index == 1 {
                    Borrowed("")
                } else {
                    Owned(format!("<sup>{}</sup>", backref.index))
                };
                let href = format!("#cont-fn-ref-{}{index}", backref.footnote_label);
                write!(self.writer, "<a href={href:?}>↩{sup}</a>")?;
                None
            }
            MarkdownNode::Footnote(footnote) => {
                let id = format!("cont-fn-{}", footnote.label);
                Some((Owned(format!("<li id={id:?}>\n")), Borrowed("</li>")))
            }
            MarkdownNode::FootnoteList => Some((
                Borrowed("<section>\n<h2>Footnotes</h2>\n<ol>\n"),
                Borrowed("</ol>\n</section>"),
            )),
            MarkdownNode::Image(image::Image { url, title }) => {
                write!(self.writer, "<img src={url:?} alt=\"")?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.write_text(child_idx, true, true)?;
                }
                write!(self.writer, "\"{} />", Self::format_title_attr(title))?;
                None
            }
            MarkdownNode::Emoji(emoji) => {
                write!(self.writer, ":{}:", emoji)?;
                None
            }
            MarkdownNode::Tag(tag) => {
                write!(
                    self.writer,
                    "<a href=\"#{}\">#{tag}</a>",
                    utils::percent_encode::encode(tag, false)
                )?;
                None
            }
            MarkdownNode::SoftBreak => {
                writeln!(self.writer)?;
                None
            }
            MarkdownNode::HardBreak => {
                writeln!(self.writer, "<br />")?;
                None
            }
            MarkdownNode::Html(html::Html::Block(_type))
            | MarkdownNode::Html(html::Html::Inline(_type)) => match _type {
                html::HtmlType::Type1(element, flag)
                | html::HtmlType::Type6(element, flag)
                | html::HtmlType::Type7(element, flag) => {
                    let is_inline = self.tree[idx].body.is_inline_level();
                    let newline = if !is_inline
                        && (!self.is_first_layer(idx) || matches!(flag, html::Flag::Full))
                    {
                        "\n"
                    } else {
                        ""
                    };
                    match flag {
                        html::Flag::Full => Some((
                            Owned(format!("<{}{}>{newline}", element.name, element.attr_str())),
                            Owned(format!("{newline}</{}>", element.name)),
                        )),
                        html::Flag::Begin => Some((
                            Owned(format!("<{}{}>{newline}", element.name, element.attr_str())),
                            Borrowed(newline),
                        )),
                        html::Flag::End => {
                            Some((Borrowed(newline), Owned(format!("</{}>", element.name))))
                        }
                        html::Flag::SelfClose => {
                            write!(
                                self.writer,
                                "<{}{} />{newline}",
                                element.name,
                                element.attr_str()
                            )?;
                            None
                        }
                    }
                }
                _ => Some((Borrowed(""), Borrowed(""))),
            },
            MarkdownNode::BlockQuote(_) => {
                Some((Borrowed("<blockquote>\n"), Borrowed("</blockquote>")))
            }
            MarkdownNode::Text(_) => {
                self.write_text(
                    idx,
                    false,
                    self.tree[self.tree.get_parent(idx)].body.xml_escape(),
                )?;
                None
            }
            MarkdownNode::List(list) => {
                self.write_list(list, idx)?;
                None
            }
            MarkdownNode::ListItem(_) => unreachable!(),
            MarkdownNode::Table(table) => {
                self.write_table(table, idx)?;
                None
            }
            MarkdownNode::TableHead => unreachable!(),
            MarkdownNode::TableHeadCol => unreachable!(),
            MarkdownNode::TableBody => unreachable!(),
            MarkdownNode::TableRow => unreachable!(),
            MarkdownNode::TableDataCol => unreachable!(),
            MarkdownNode::ThematicBreak => {
                let before = if self.is_first_child(idx) && !self.is_first_layer(idx) {
                    "\n"
                } else {
                    ""
                };
                let after = if !(self.is_last_child(idx) && self.is_first_layer(idx)) {
                    "\n"
                } else {
                    ""
                };
                write!(self.writer, "{before}<hr />{after}")?;
                None
            }
            MarkdownNode::FrontMatter => None,
            // ignore obsidian flavored
            MarkdownNode::Link(link::Link::Wikilink(..)) => None,
            MarkdownNode::Math(_) => None,
            MarkdownNode::Callout(_) => None,
            MarkdownNode::Embed(_) => None,
        };
        if let Some((open, close)) = pair {
            self.write_open(open, idx)?;
            if let Some(child_idx) = self.tree.get_first_child(idx) {
                self.write_html(child_idx)?;
            }
            self.write_close(close, idx)?;
        }
        if let Some(next_idx) = self.tree.get_next(idx) {
            self.write_html(next_idx)?;
        }
        Ok(())
    }
    fn write_open(&mut self, open: Cow<str>, idx: usize) -> fmt::Result {
        if self.tree[idx].body.is_block_level()
            && self
                .tree
                .get_prev(idx)
                .map(|idx| self.tree[idx].body.is_inline_level())
                .unwrap_or(false)
        {
            writeln!(self.writer)?;
        }
        write!(self.writer, "{}", open)
    }
    fn write_close(&mut self, close: Cow<str>, idx: usize) -> fmt::Result {
        let is_block = self.tree[idx].body.is_block_level();
        let non_final_block = Some(idx) != self.tree.get_last_child(0);
        if !close.is_empty() && is_block && non_final_block {
            writeln!(self.writer, "{}", close)
        } else {
            write!(self.writer, "{}", close)
        }
    }
    fn write_text(
        &mut self,
        idx: usize,
        include_next_sibling: bool,
        xml_escape: bool,
    ) -> fmt::Result {
        if let MarkdownNode::Text(str) = &self.tree[idx].body {
            if xml_escape && str.contains(['&', '<', '>', '"']) {
                write!(self.writer, "{}", utils::escape_xml(str))?;
            } else {
                write!(self.writer, "{}", str)?;
            }
        } else if let Some(child_idx) = self.tree.get_first_child(idx) {
            self.write_text(child_idx, true, self.tree[idx].body.xml_escape())?;
        }
        if let Some(next_idx) = self.tree.get_next(idx).filter(|_| include_next_sibling) {
            self.write_text(next_idx, true, xml_escape)?;
        }
        Ok(())
    }
    fn write_list(&mut self, list: &list::List, idx: usize) -> fmt::Result {
        match list {
            list::List::Bullet(bullet) => {
                self.write_open("<ul>\n".into(), idx)?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(child_idx, bullet.tight, None)?;
                }
                self.write_close("\n</ul>".into(), idx)?;
            }
            list::List::Ordered(ordered) => {
                self.write_open("<ol>\n".into(), idx)?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(child_idx, ordered.tight, None)?;
                }
                self.write_close("\n</ol>".into(), idx)?;
            }
            list::List::Task(task) => {
                self.write_open("<ul>".into(), idx)?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(
                        child_idx,
                        task.tight,
                        Some((task.checked, task.quested)),
                    )?;
                }
                self.write_close("\n</ul>".into(), idx)?;
            }
        }
        Ok(())
    }
    fn writer_list_item(
        &mut self,
        idx: usize,
        tight: bool,
        task: Option<(bool, bool)>,
    ) -> fmt::Result {
        let newline = if tight { "" } else { "\n" };
        write!(self.writer, "<li>{newline}")?;
        if !tight {
            writeln!(self.writer, "<p>")?;
        }
        if let Some((checked, _)) = task {
            if checked {
                writeln!(self.writer, r#"<input type="checkbox" disabled checked />"#)?;
            } else {
                writeln!(self.writer, r#"<input type="checkbox" disabled />"#)?;
            }
        }
        // 如果是 Paragraph 节点则跳过，因为 Paragraph 由 tight 控制，已在上面输出了
        if let Some(child_idx) = self.tree.get_first_child(idx).and_then(|idx| {
            if self.tree[idx].body == MarkdownNode::Paragraph {
                self.tree.get_first_child(idx)
            } else {
                Some(idx)
            }
        }) {
            self.write_html(child_idx)?;
        }
        if !tight {
            writeln!(self.writer, "</p>")?;
        }
        if let Some(next_idx) = self.tree.get_next(idx) {
            writeln!(self.writer, "{newline}</li>")?;
            self.writer_list_item(next_idx, tight, task)?;
        } else {
            write!(self.writer, "{newline}</li>")?;
        }
        Ok(())
    }
    fn write_table(&mut self, table: &table::Table, idx: usize) -> fmt::Result {
        writeln!(self.writer, "<table>")?;
        if let Some(head_idx) = self
            .tree
            .get_first_child(idx)
            .filter(|head_idx| self.tree[*head_idx].body == MarkdownNode::TableHead)
        {
            writeln!(self.writer, "<thead>")?;
            let mut next = self.tree.get_first_child(head_idx);
            while let Some(next_idx) = next {
                next = self.tree.get_next(next_idx);
                self.write_table_row(next_idx, &table.alignments, true, table.column)?;
            }
            writeln!(self.writer, "</thead>")?;
        };
        if let Some(body_idx) = self
            .tree
            .get_last_child(idx)
            .filter(|body_idx| self.tree[*body_idx].body == MarkdownNode::TableBody)
        {
            writeln!(self.writer, "<tbody>")?;
            let mut next = self.tree.get_first_child(body_idx);
            while let Some(next_idx) = next {
                next = self.tree.get_next(next_idx);
                self.write_table_row(next_idx, &table.alignments, false, table.column)?;
            }
            writeln!(self.writer, "</tbody>")?;
        };
        self.write_close("</table>".into(), idx)?;
        Ok(())
    }
    fn write_table_row(
        &mut self,
        idx: usize,
        alignments: &[table::Alignment],
        in_head: bool,
        max_column: usize,
    ) -> fmt::Result {
        assert_eq!(self.tree[idx].body, MarkdownNode::TableRow);
        let tag = if in_head { "th" } else { "td" };
        let mut i = 0;
        let mut next = self.tree.get_first_child(idx);
        writeln!(self.writer, "<tr>")?;
        while let Some(next_id) = next {
            next = self.tree.get_next(next_id);
            let align = {
                let align = alignments.get(i).unwrap_or(&table::Alignment::Left);
                match align {
                    table::Alignment::Left => "",
                    table::Alignment::Center => r#" align="center""#,
                    table::Alignment::Right => r#" align="right""#,
                }
            };
            write!(self.writer, "<{tag}{align}>")?;
            if let Some(child_idx) = self.tree.get_first_child(next_id) {
                self.write_html(child_idx)?;
            }
            writeln!(self.writer, "</{tag}>")?;
            i += 1;
        }
        for i in i..max_column {
            let align = {
                let align = alignments.get(i).unwrap_or(&table::Alignment::Left);
                match align {
                    table::Alignment::Left => "",
                    table::Alignment::Center => r#" align="center""#,
                    table::Alignment::Right => r#" align="right""#,
                }
            };
            writeln!(self.writer, "<{tag}{align}></{tag}>")?;
        }
        writeln!(self.writer, "</tr>")?;
        Ok(())
    }
    fn format_title_attr(title: &Option<String>) -> String {
        if let Some(title) = &title {
            format!(" title=\"{}\"", title)
        } else {
            String::new()
        }
    }
    fn is_first_layer(&self, idx: usize) -> bool {
        self.tree.get_parent(idx) == 0
    }
    fn is_first_child(&self, idx: usize) -> bool {
        self.tree.get_prev(idx).is_none()
    }
    fn is_last_child(&self, idx: usize) -> bool {
        self.tree.get_next(idx).is_none()
    }
}

impl Tree<Node> {
    pub fn to_html(&self) -> String {
        let mut buffer = String::new();
        if self.is_empty() {
            return buffer;
        }
        HtmlRender::new(self, &mut buffer).write_html(0).unwrap();
        buffer
    }
}
