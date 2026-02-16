use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::fmt;
use std::fmt::Write;

use crate::ast::{MarkdownNode, callout, html, image, link, list, table};
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
    fn render(&mut self, idx: usize) -> fmt::Result {
        let pair = match &self.tree[idx].body {
            MarkdownNode::Document => Some((Borrowed(""), Borrowed(""))),
            MarkdownNode::Paragraph => {
                let id_attr = Self::format_id_attr(&self.tree[idx].id);
                if self.tree.get_first_child(idx).is_none() && id_attr.is_empty() {
                    None
                } else if self.try_write_split_paragraph(idx, &id_attr)? {
                    None
                } else {
                    Some((Owned(format!("<p{id_attr}>")), Borrowed("</p>")))
                }
            }
            MarkdownNode::Heading(heading) => Some(match heading.level() {
                ast::heading::HeadingLevel::H1 => (
                    Owned(format!("<h1{}>", Self::format_id_attr(&self.tree[idx].id))),
                    Borrowed("</h1>"),
                ),
                ast::heading::HeadingLevel::H2 => (
                    Owned(format!("<h2{}>", Self::format_id_attr(&self.tree[idx].id))),
                    Borrowed("</h2>"),
                ),
                ast::heading::HeadingLevel::H3 => (
                    Owned(format!("<h3{}>", Self::format_id_attr(&self.tree[idx].id))),
                    Borrowed("</h3>"),
                ),
                ast::heading::HeadingLevel::H4 => (
                    Owned(format!("<h4{}>", Self::format_id_attr(&self.tree[idx].id))),
                    Borrowed("</h4>"),
                ),
                ast::heading::HeadingLevel::H5 => (
                    Owned(format!("<h5{}>", Self::format_id_attr(&self.tree[idx].id))),
                    Borrowed("</h5>"),
                ),
                ast::heading::HeadingLevel::H6 => (
                    Owned(format!("<h6{}>", Self::format_id_attr(&self.tree[idx].id))),
                    Borrowed("</h6>"),
                ),
            }),
            MarkdownNode::Code(code) => Some(match code.as_ref() {
                ast::code::Code::Inline(_) => (Borrowed("<code>"), Borrowed("</code>")),
                ast::code::Code::Indented(_) => {
                    (Borrowed("<pre><code>"), Borrowed("</code></pre>"))
                }
                ast::code::Code::Fenced(code) => (
                    if let Some(language) = &code.language {
                        Owned(format!(
                            "<pre><code class=\"language-{}\">",
                            language.split(' ').nth(0).unwrap_or("")
                        ))
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
            MarkdownNode::Link(link_box) => match link_box.as_ref() {
                link::Link::Default(link) => {
                    let title = Self::format_title_attr(&link.title);
                    Some((
                        Owned(format!(
                            "<a href=\"{}\"{title}>",
                            utils::escape_xml(&link.url)
                        )),
                        Borrowed("</a>"),
                    ))
                }
                link::Link::Wikilink(link) => {
                    use ast::reference::Reference;
                    let mut href = link.path.clone();
                    if let Some(reference) = &link.reference {
                        let suffix = match reference {
                            Reference::Heading(value) => format!("#{value}"),
                            Reference::MultiHeading(values) => format!("#{}", values.join("#")),
                            Reference::BlockId(value) => format!("#^{value}"),
                        };
                        href.push_str(&suffix);
                    }
                    let label = if let Some(text) = &link.text {
                        text.clone()
                    } else {
                        match &link.reference {
                            Some(Reference::Heading(value)) => format!("{} > {value}", link.path),
                            _ => link.path.clone(),
                        }
                    };
                    write!(
                        self.writer,
                        "<a href=\"{}\" class=\"internal-link\">{}</a>",
                        utils::escape_xml(&href),
                        utils::escape_xml(&label)
                    )?;
                    None
                }
                link::Link::Footnote(link) => {
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
                link::Link::FootnoteBackref(backref) => {
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
            },
            MarkdownNode::Footnote(footnote) => {
                let id = format!("cont-fn-{}", footnote.label);
                Some((Owned(format!("<li id={id:?}>\n")), Borrowed("\n</li>")))
            }
            MarkdownNode::FootnoteList => Some((
                Borrowed("<section>\n<h2>Footnotes</h2>\n<ol>\n"),
                Borrowed("\n</ol>\n</section>"),
            )),
            MarkdownNode::Image(img) => {
                let image::Image { url, title } = img.as_ref();
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
            MarkdownNode::Html(html_box) => {
                let _type = match html_box.as_ref() {
                    html::Html::Block(t) | html::Html::Inline(t) => t,
                };
                match _type {
                    html::HtmlType::JSComment(value) => {
                        write!(self.writer, "{{/*{value}*/}}")?;
                        None
                    }
                    html::HtmlType::JSExpression(value) => {
                        write!(self.writer, "{{{value}}}")?;
                        None
                    }
                    html::HtmlType::RawTextContainer(element, flag)
                    | html::HtmlType::CanonicalBlockTag(element, flag)
                    | html::HtmlType::GenericTag(element, flag)
                    | html::HtmlType::Component(element, flag) => {
                        let is_inline = self.tree[idx].body.is_inline_level();
                        let has_raw_opening = !is_inline && self.html_block_has_raw_opening(idx);
                        let has_raw_closing =
                            !is_inline && self.html_block_has_raw_closing(idx, &element.name);
                        let wrap_full = !is_inline && self.should_wrap_html_full_with_newline(idx);
                        let open_newline = if wrap_full && !has_raw_opening {
                            "\n"
                        } else {
                            ""
                        };
                        let close_newline = if wrap_full
                            && !self.html_block_last_child_has_newline(idx)
                            && !self.html_block_last_child_is_whitespace_text(idx)
                        {
                            "\n"
                        } else {
                            ""
                        };
                        match flag {
                            html::Flag::Full => Some((
                                if has_raw_opening {
                                    Borrowed("")
                                } else {
                                    Owned(format!(
                                        "<{}{}>{open_newline}",
                                        element.name,
                                        element.attr_str()
                                    ))
                                },
                                if has_raw_closing {
                                    Borrowed("")
                                } else {
                                    Owned(format!("{close_newline}</{}>", element.name))
                                },
                            )),
                            html::Flag::Begin => Some(if is_inline {
                                (
                                    Owned(format!("<{}{}>", element.name, element.attr_str())),
                                    Borrowed(""),
                                )
                            } else if has_raw_opening {
                                (Borrowed(""), Borrowed(""))
                            } else {
                                let trailing_newline = if self.tree.get_first_child(idx).is_some() {
                                    "\n"
                                } else {
                                    ""
                                };
                                (
                                    Owned(format!(
                                        "<{}{}>{}",
                                        element.name,
                                        element.attr_str(),
                                        trailing_newline
                                    )),
                                    Borrowed(""),
                                )
                            }),
                            html::Flag::End => Some(if is_inline {
                                (Borrowed(""), Owned(format!("</{}>", element.name)))
                            } else {
                                let end_newline = if self.tree.get_first_child(idx).is_some() {
                                    "\n"
                                } else {
                                    ""
                                };
                                (
                                    Owned(format!("</{}>{end_newline}", element.name)),
                                    Borrowed(""),
                                )
                            }),
                            html::Flag::SelfClose => {
                                write!(self.writer, "<{}{}/>", element.name, element.attr_str())?;
                                None
                            }
                        }
                    }
                    _ => Some((Borrowed(""), Borrowed(""))),
                }
            }
            MarkdownNode::BlockQuote => Some((
                Borrowed("<blockquote>\n"),
                if self.tree.get_first_child(idx).is_some() {
                    Borrowed("\n</blockquote>")
                } else {
                    Borrowed("</blockquote>")
                },
            )),
            MarkdownNode::Text(_) => {
                let parent = self.tree.get_parent(idx);
                let xml_escape = if let MarkdownNode::Html(h) = &self.tree[parent].body {
                    if matches!(h.as_ref(), html::Html::Block(_)) {
                        false
                    } else {
                        self.tree[parent].body.xml_escape()
                    }
                } else {
                    self.tree[parent].body.xml_escape()
                };
                self.write_text(idx, false, xml_escape)?;
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
            MarkdownNode::FrontMatter(..) => None,
            MarkdownNode::Math(math) => {
                self.write_math(math, idx)?;
                None
            }
            MarkdownNode::Callout(callout) => {
                self.write_callout(callout, idx)?;
                None
            }
            MarkdownNode::Embed(embed) => {
                self.write_embed(embed)?;
                None
            }
        };
        if let Some((open, close)) = pair {
            self.write_open(open, idx)?;
            if let Some(child_idx) = self.tree.get_first_child(idx) {
                if matches!(
                    self.tree[idx].body,
                    MarkdownNode::Html(ref h) if matches!(h.as_ref(), html::Html::Block(..))
                ) {
                    self.write_html_block_children(child_idx)?;
                } else {
                    self.write_html(child_idx)?;
                }
            }
            self.write_close(close, idx)?;
        }
        Ok(())
    }
    fn write_html(&mut self, idx: usize) -> fmt::Result {
        let mut next = Some(idx);
        while let Some(next_idx) = next {
            self.render(next_idx)?;
            next = self.tree.get_next(next_idx);
        }
        Ok(())
    }
    fn write_html_until(&mut self, first: usize, stop: usize) -> fmt::Result {
        let mut next = Some(first);
        while let Some(idx) = next {
            if idx == stop {
                break;
            }
            self.render(idx)?;
            next = self.tree.get_next(idx);
        }
        Ok(())
    }
    fn paragraph_split_child(&self, paragraph_idx: usize) -> Option<usize> {
        let mut prev = None;
        let mut cur = self.tree.get_first_child(paragraph_idx);
        while let Some(idx) = cur {
            let is_type6_inline = matches!(
                self.tree[idx].body,
                MarkdownNode::Html(ref h) if matches!(
                    h.as_ref(),
                    html::Html::Inline(
                        html::HtmlType::CanonicalBlockTag(..) | html::HtmlType::Component(..)
                    )
                )
            );
            let is_display_math = matches!(
                self.tree[idx].body,
                MarkdownNode::Math(ref m) if matches!(m.as_ref(), ast::math::Math::Block(..))
            );
            if is_type6_inline || is_display_math {
                if let Some(prev_idx) = prev {
                    if self.tree[prev_idx].body == MarkdownNode::SoftBreak {
                        return Some(prev_idx);
                    }
                }
                return Some(idx);
            }
            prev = Some(idx);
            cur = self.tree.get_next(idx);
        }
        None
    }
    fn try_write_split_paragraph(
        &mut self,
        paragraph_idx: usize,
        id_attr: &str,
    ) -> Result<bool, fmt::Error> {
        let Some(split) = self.paragraph_split_child(paragraph_idx) else {
            return Ok(false);
        };
        let Some(first) = self.tree.get_first_child(paragraph_idx) else {
            return Ok(false);
        };
        if split == first {
            let first_is_display_math = matches!(
                self.tree[first].body,
                MarkdownNode::Math(ref m) if matches!(m.as_ref(), ast::math::Math::Block(..))
            );
            if first_is_display_math {
                self.write_html(first)?;
                return Ok(true);
            }
            return Ok(false);
        }
        self.write_open(Owned(format!("<p{id_attr}>")), paragraph_idx)?;
        self.write_html_until(first, split)?;
        self.write_close(Borrowed("</p>"), paragraph_idx)?;
        self.write_html(split)?;
        Ok(true)
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
        let non_final_block = Some(idx) != self.tree.get_last_child(self.tree.get_parent(idx));
        if close == "\n" && is_block && !non_final_block {
            return Ok(());
        }
        if close.is_empty()
            && is_block
            && non_final_block
            && matches!(
                self.tree[idx].body,
                MarkdownNode::Html(ref h) if matches!(h.as_ref(), html::Html::Block(..))
            )
        {
            writeln!(self.writer)
        } else if !close.is_empty() && is_block && non_final_block {
            if close.ends_with('\n') {
                write!(self.writer, "{}", close)
            } else {
                writeln!(self.writer, "{}", close)
            }
        } else {
            write!(self.writer, "{}", close)
        }
    }
    fn write_html_block_children(&mut self, first_idx: usize) -> fmt::Result {
        let mut next = Some(first_idx);
        while let Some(idx) = next {
            self.render(idx)?;
            next = self.tree.get_next(idx);
            if let Some(next_idx) = next {
                let next_is_block = self.tree[next_idx].body.is_block_level();
                if !next_is_block
                    && self.tree[idx].body.is_inline_level()
                    && !Self::ends_with_newline_by_node(&self.tree[idx].body)
                {
                    writeln!(self.writer)?;
                }
            }
        }
        Ok(())
    }
    fn ends_with_newline_by_node(node: &MarkdownNode) -> bool {
        match node {
            MarkdownNode::SoftBreak | MarkdownNode::HardBreak => true,
            MarkdownNode::Html(h) => matches!(
                h.as_ref(),
                html::Html::Block(
                    html::HtmlType::RawTextContainer(_, html::Flag::Begin)
                        | html::HtmlType::RawTextContainer(_, html::Flag::End)
                        | html::HtmlType::CanonicalBlockTag(_, html::Flag::Begin)
                        | html::HtmlType::CanonicalBlockTag(_, html::Flag::End)
                        | html::HtmlType::GenericTag(_, html::Flag::Begin)
                        | html::HtmlType::GenericTag(_, html::Flag::End)
                        | html::HtmlType::Component(_, html::Flag::Begin)
                        | html::HtmlType::Component(_, html::Flag::End)
                )
            ),
            _ => false,
        }
    }
    fn should_wrap_html_full_with_newline(&self, idx: usize) -> bool {
        let Some(first) = self.tree.get_first_child(idx) else {
            return false;
        };
        let Some(last) = self.tree.get_last_child(idx) else {
            return false;
        };
        !(self.tree[first].start.line == self.tree[idx].start.line
            && self.tree[last].end.line == self.tree[idx].end.line)
    }
    fn html_block_has_raw_opening(&self, idx: usize) -> bool {
        let Some(first) = self.tree.get_first_child(idx) else {
            return false;
        };
        let MarkdownNode::Text(text) = &self.tree[first].body else {
            return false;
        };
        text.trim_start_matches(|c| c == ' ' || c == '\t')
            .starts_with('<')
    }
    fn html_block_has_raw_closing(&self, idx: usize, name: &str) -> bool {
        let mut next = self.tree.get_first_child(idx);
        let needle = format!("</{}", name.to_ascii_lowercase());
        while let Some(child) = next {
            if let MarkdownNode::Text(text) = &self.tree[child].body {
                let lower = text.to_ascii_lowercase();
                if lower.contains(&needle) {
                    return true;
                }
            }
            next = self.tree.get_next(child);
        }
        false
    }
    fn html_block_last_child_has_newline(&self, idx: usize) -> bool {
        let Some(last) = self.tree.get_last_child(idx) else {
            return false;
        };
        Self::ends_with_newline_by_node(&self.tree[last].body)
    }
    fn html_block_last_child_is_whitespace_text(&self, idx: usize) -> bool {
        let Some(last) = self.tree.get_last_child(idx) else {
            return false;
        };
        matches!(
            &self.tree[last].body,
            MarkdownNode::Text(text) if text.chars().all(|ch| matches!(ch, ' ' | '\t'))
        )
    }
    // fn block_indent_prefix(&self, idx: usize) -> String {
    //     let count = self.tree[idx].start.column.saturating_sub(1) as usize;
    //     " ".repeat(count)
    // }
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
                self.write_open(
                    if ordered.start == 1 {
                        Borrowed("<ol>\n")
                    } else {
                        Owned(format!("<ol start=\"{}\">\n", ordered.start))
                    },
                    idx,
                )?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(child_idx, ordered.tight, None)?;
                }
                self.write_close("\n</ol>".into(), idx)?;
            }
            list::List::Task(task) => {
                if task.obsidian {
                    self.write_open("<ul class=\"contains-task-list\">".into(), idx)?;
                } else {
                    self.write_open("<ul>\n".into(), idx)?;
                }
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(child_idx, task.tight, Some(task.obsidian))?;
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
        task_list: Option<bool>,
    ) -> fmt::Result {
        let newline = if !tight
            || self
                .tree
                .get_first_child(idx)
                .map(|idx| {
                    self.tree[idx].body.is_block_level()
                        && self.tree[idx].body != MarkdownNode::Paragraph
                })
                .unwrap_or(false)
        {
            "\n"
        } else {
            ""
        };
        let task_state = if task_list.is_some() {
            if let MarkdownNode::ListItem(li) = &self.tree[idx].body {
                if let list::ListItem::Task(item) = li.as_ref() {
                    item.task
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        let li_attr = if let Some(true) = task_list {
            match task_state {
                Some(ch) if ch != ' ' => " class=\"task-list-item is-checked\"",
                Some(_) => " class=\"task-list-item\"",
                None => "",
            }
        } else {
            ""
        };
        write!(self.writer, "<li{li_attr}>{newline}")?;
        if let Some(obsidian) = task_list {
            if let MarkdownNode::ListItem(li) = &self.tree[idx].body {
                if let list::ListItem::Task(item) = li.as_ref() {
                    if let Some(ch) = item.task {
                        if obsidian {
                            if ch == ' ' {
                                write!(self.writer, r#"<input type="checkbox" disabled /> "#)?;
                            } else {
                                write!(
                                    self.writer,
                                    r#"<input type="checkbox" disabled checked /> "#
                                )?;
                            }
                        } else if ch == ' ' {
                            write!(self.writer, r#"<input disabled="" type="checkbox"> "#)?;
                        } else {
                            write!(
                                self.writer,
                                r#"<input checked="" disabled="" type="checkbox"> "#
                            )?;
                        }
                    }
                }
            }
        }
        if let Some(first_child) = self.tree.get_first_child(idx) {
            if tight {
                // In tight lists, skip <p> wrappers for ALL Paragraph children
                let mut child = Some(first_child);
                while let Some(child_idx) = child {
                    if self.tree[child_idx].body == MarkdownNode::Paragraph {
                        // Output paragraph's children directly (no <p> wrapper)
                        if let Some(inner) = self.tree.get_first_child(child_idx) {
                            self.write_html(inner)?;
                        }
                    } else {
                        self.render(child_idx)?;
                    }
                    child = self.tree.get_next(child_idx);
                }
            } else {
                self.write_html(first_child)?;
            }
        }
        if let Some(next_idx) = self.tree.get_next(idx) {
            writeln!(self.writer, "{newline}</li>")?;
            self.writer_list_item(next_idx, tight, task_list)?;
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
                    table::Alignment::Center => r#" style="text-align: center""#,
                    table::Alignment::Right => r#" style="text-align: right""#,
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
                    table::Alignment::Center => r#" style="text-align: center""#,
                    table::Alignment::Right => r#" style="text-align: right""#,
                }
            };
            writeln!(self.writer, "<{tag}{align}></{tag}>")?;
        }
        writeln!(self.writer, "</tr>")?;
        Ok(())
    }
    fn format_title_attr(title: &Option<String>) -> String {
        if let Some(title) = &title {
            format!(" title=\"{}\"", utils::escape_xml(title))
        } else {
            String::new()
        }
    }
    fn format_id_attr(id: &Option<Box<String>>) -> String {
        if let Some(id) = id {
            format!(" id=\"{}\"", utils::escape_xml(id.as_str()))
        } else {
            String::new()
        }
    }
    fn callout_type_name(callout: &callout::Callout) -> String {
        match &callout._type {
            callout::CalloutType::Note => "note".to_string(),
            callout::CalloutType::Abstract => "abstract".to_string(),
            callout::CalloutType::Info => "info".to_string(),
            callout::CalloutType::Todo => "todo".to_string(),
            callout::CalloutType::Tip => "tip".to_string(),
            callout::CalloutType::Success => "success".to_string(),
            callout::CalloutType::Question => "question".to_string(),
            callout::CalloutType::Warning => "warning".to_string(),
            callout::CalloutType::Failure => "failure".to_string(),
            callout::CalloutType::Danger => "danger".to_string(),
            callout::CalloutType::Bug => "bug".to_string(),
            callout::CalloutType::Example => "example".to_string(),
            callout::CalloutType::Quote => "quote".to_string(),
            callout::CalloutType::Custom(v) => v.to_string(),
        }
    }
    fn callout_default_title(callout: &callout::Callout) -> String {
        match &callout._type {
            callout::CalloutType::Note => "Note".to_string(),
            callout::CalloutType::Abstract => "Abstract".to_string(),
            callout::CalloutType::Info => "Info".to_string(),
            callout::CalloutType::Todo => "Todo".to_string(),
            callout::CalloutType::Tip => "Tip".to_string(),
            callout::CalloutType::Success => "Success".to_string(),
            callout::CalloutType::Question => "Question".to_string(),
            callout::CalloutType::Warning => "Warning".to_string(),
            callout::CalloutType::Failure => "Failure".to_string(),
            callout::CalloutType::Danger => "Danger".to_string(),
            callout::CalloutType::Bug => "Bug".to_string(),
            callout::CalloutType::Example => "Example".to_string(),
            callout::CalloutType::Quote => "Quote".to_string(),
            callout::CalloutType::Custom(v) => {
                let mut chars = v.chars();
                if let Some(first) = chars.next() {
                    let mut title = first.to_uppercase().to_string();
                    title.push_str(chars.as_str());
                    title
                } else {
                    String::new()
                }
            }
        }
    }
    fn write_math(&mut self, math: &ast::math::Math, idx: usize) -> fmt::Result {
        use ast::math::Math;
        match math {
            Math::Inline(_) => {
                write!(self.writer, "<span class=\"math math-inline\">")?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.write_html(child_idx)?;
                }
                write!(self.writer, "</span>")?;
            }
            Math::Block(_) => {
                write!(self.writer, "<div class=\"math math-display\">")?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.write_html(child_idx)?;
                }
                write!(self.writer, "</div>")?;
            }
        }
        Ok(())
    }
    fn write_callout(&mut self, callout: &callout::Callout, idx: usize) -> fmt::Result {
        let mut class = String::from("callout");
        if let Some(foldable) = callout.foldable {
            class.push_str(" is-collapsible");
            if !foldable {
                class.push_str(" is-collapsed");
            }
        }
        let typ = Self::callout_type_name(callout);
        let title = callout
            .title
            .clone()
            .unwrap_or_else(|| Self::callout_default_title(callout));
        writeln!(
            self.writer,
            "<div class=\"{}\" data-callout=\"{}\">",
            class,
            utils::escape_xml(&typ)
        )?;
        writeln!(
            self.writer,
            "<div class=\"callout-title\">{}</div>",
            utils::escape_xml(&title)
        )?;
        writeln!(self.writer, "<div class=\"callout-content\">")?;
        if let Some(child_idx) = self.tree.get_first_child(idx) {
            self.write_html(child_idx)?;
        }
        write!(self.writer, "\n</div>\n</div>")?;
        Ok(())
    }
    fn write_embed(&mut self, embed: &ast::embed::Embed) -> fmt::Result {
        let mut src = embed.path.clone();
        if let Some(reference) = &embed.reference {
            use ast::reference::Reference;
            let suffix = match reference {
                Reference::Heading(v) => format!("#{v}"),
                Reference::MultiHeading(vs) => format!("#{}", vs.join("#")),
                Reference::BlockId(v) => format!("#^{v}"),
            };
            src.push_str(&suffix);
        }
        if let Some(attrs) = &embed.attrs {
            let attrs = attrs
                .iter()
                .map(|(k, v)| {
                    if v.is_empty() {
                        k.clone()
                    } else {
                        format!("{k}={v}")
                    }
                })
                .collect::<Vec<_>>()
                .join("&");
            if !attrs.is_empty() {
                if src.contains('#') {
                    src.push('&');
                } else {
                    src.push('#');
                }
                src.push_str(&attrs);
            }
        }
        let src_escaped = utils::escape_xml(&src);
        let path_escaped = utils::escape_xml(&embed.path);
        let ext = embed
            .path
            .rsplit('.')
            .next()
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();
        if matches!(
            ext.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp"
        ) {
            let size_attr = if let Some((w, h)) = embed.size {
                if let Some(h) = h {
                    format!(" width=\"{w}\" height=\"{h}\"")
                } else {
                    format!(" width=\"{w}\"")
                }
            } else {
                String::new()
            };
            write!(
                self.writer,
                "<img src=\"{}\" alt=\"{}\"{} />",
                src_escaped, path_escaped, size_attr
            )
        } else if matches!(ext.as_str(), "mp3" | "wav" | "ogg" | "m4a" | "flac") {
            write!(
                self.writer,
                "<audio controls src=\"{}\"></audio>",
                src_escaped
            )
        } else if ext == "pdf" {
            write!(self.writer, "<iframe src=\"{}\"></iframe>", src_escaped)
        } else {
            write!(
                self.writer,
                "<span class=\"internal-embed\" src=\"{}\"></span>",
                src_escaped
            )
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
        if self.is_empty() {
            return String::new();
        }
        // Most Markdown inputs expand modestly when rendered to HTML.
        // Reserve upfront to reduce repeated String growth during write!.
        let mut buffer = String::with_capacity(self.node_slots_len().saturating_mul(32));
        let _ = HtmlRender::new(self, &mut buffer).write_html(0);
        buffer
    }
}
