use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::fmt;
use std::fmt::Write;

use memchr::{memchr, memchr3};

use crate::ast::{MarkdownNode, callout, html, image, link, list, table};
use crate::document::Document;
use crate::node::Node;
use crate::tree::Tree;
use crate::{ast, utils};

struct HtmlRender<'input> {
    writer: &'input mut String,
    tree: &'input Tree<Node>,
    source: &'input str,
}

impl<'input> HtmlRender<'input> {
    fn new(tree: &'input Tree<Node>, source: &'input str, writer: &'input mut String) -> Self {
        Self {
            tree,
            source,
            writer,
        }
    }
    fn render(&mut self, idx: usize) -> fmt::Result {
        match &self.tree[idx].body {
            MarkdownNode::Document => self.render_wrapped(idx, "", "")?,
            MarkdownNode::Paragraph => {
                let id = self.tree[idx].id.as_deref().map(String::as_str);
                if self.tree.get_first_child(idx).is_some() || id.is_some() {
                    if !self.try_write_split_paragraph(idx, id)? {
                        self.prepare_open(idx);
                        self.writer.push_str("<p");
                        Self::push_attr(self.writer, "id", id);
                        self.writer.push('>');
                        self.write_children(idx)?;
                        self.write_close("</p>", idx);
                    }
                }
            }
            MarkdownNode::Heading(heading) => {
                let tag = match heading.level() {
                    ast::heading::HeadingLevel::H1 => "h1",
                    ast::heading::HeadingLevel::H2 => "h2",
                    ast::heading::HeadingLevel::H3 => "h3",
                    ast::heading::HeadingLevel::H4 => "h4",
                    ast::heading::HeadingLevel::H5 => "h5",
                    ast::heading::HeadingLevel::H6 => "h6",
                };
                self.prepare_open(idx);
                self.writer.push('<');
                self.writer.push_str(tag);
                Self::push_attr(
                    self.writer,
                    "id",
                    self.tree[idx].id.as_deref().map(String::as_str),
                );
                self.writer.push('>');
                self.write_children(idx)?;
                self.writer.push_str("</");
                self.writer.push_str(tag);
                self.writer.push('>');
                self.finish_close(idx, false, false);
            }
            MarkdownNode::Code(code) => match code.as_ref() {
                ast::code::Code::Inline(_) => self.render_wrapped(idx, "<code>", "</code>")?,
                ast::code::Code::Indented(_) => {
                    self.render_wrapped(idx, "<pre><code>", "</code></pre>")?
                }
                ast::code::Code::Fenced(code) => {
                    self.prepare_open(idx);
                    self.writer.push_str("<pre><code");
                    if let Some(language) = &code.language {
                        self.writer.push_str(" class=\"language-");
                        self.writer
                            .push_str(language.split(' ').next().unwrap_or(""));
                        self.writer.push('"');
                    } else {
                    }
                    self.writer.push('>');
                    self.write_children(idx)?;
                    self.write_close("</code></pre>", idx);
                }
            },
            MarkdownNode::Emphasis => self.render_wrapped(idx, "<em>", "</em>")?,
            MarkdownNode::Strong => self.render_wrapped(idx, "<strong>", "</strong>")?,
            MarkdownNode::Strikethrough => self.render_wrapped(idx, "<del>", "</del>")?,
            MarkdownNode::Highlighting => self.render_wrapped(idx, "<mark>", "</mark>")?,
            MarkdownNode::Link(link_box) => match link_box.as_ref() {
                link::Link::Default(link) => {
                    self.prepare_open(idx);
                    self.writer.push_str("<a href=\"");
                    Self::push_escaped(self.writer, link.url.resolve(self.source));
                    self.writer.push('"');
                    Self::push_attr(
                        self.writer,
                        "title",
                        link.title.as_ref().map(|title| title.resolve(self.source)),
                    );
                    self.writer.push('>');
                    self.write_children(idx)?;
                    self.write_close("</a>", idx);
                }
                link::Link::Wikilink(link) => {
                    use ast::reference::Reference;
                    self.writer.push_str("<a href=\"");
                    Self::push_escaped(self.writer, &link.path);
                    if let Some(reference) = &link.reference {
                        self.writer.push('#');
                        match reference {
                            Reference::Heading(value) => Self::push_escaped(self.writer, value),
                            Reference::MultiHeading(values) => {
                                for (index, value) in values.iter().enumerate() {
                                    if index != 0 {
                                        self.writer.push('#');
                                    }
                                    Self::push_escaped(self.writer, value);
                                }
                            }
                            Reference::BlockId(value) => {
                                self.writer.push('^');
                                Self::push_escaped(self.writer, value);
                            }
                        }
                    }
                    self.writer.push_str("\" class=\"internal-link\">");
                    if let Some(text) = &link.text {
                        Self::push_escaped(self.writer, text);
                    } else {
                        match &link.reference {
                            Some(Reference::Heading(value)) => {
                                Self::push_escaped(self.writer, &link.path);
                                self.writer.push_str(" &gt; ");
                                Self::push_escaped(self.writer, value);
                            }
                            _ => Self::push_escaped(self.writer, &link.path),
                        }
                    }
                    self.writer.push_str("</a>");
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
                }
            },
            MarkdownNode::Footnote(footnote) => {
                let id = format!("cont-fn-{}", footnote.label);
                self.prepare_open(idx);
                write!(self.writer, "<li id={id:?}>\n")?;
                self.write_children(idx)?;
                self.write_close("\n</li>", idx);
            }
            MarkdownNode::FootnoteList => self.render_wrapped(
                idx,
                "<section>\n<h2>Footnotes</h2>\n<ol>\n",
                "\n</ol>\n</section>",
            )?,
            MarkdownNode::Image(img) => {
                let image::Image { url, title, size } = img.as_ref();
                let url = url.resolve(self.source);
                write!(self.writer, "<img src={url:?} alt=\"")?;
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.write_text(child_idx, true, true)?;
                }
                self.writer.push('"');
                Self::push_attr(
                    self.writer,
                    "title",
                    title.as_ref().map(|title| title.resolve(self.source)),
                );
                if let Some((width, height)) = size {
                    write!(self.writer, " width=\"{width}\"")?;
                    if let Some(height) = height {
                        write!(self.writer, " height=\"{height}\"")?;
                    }
                }
                self.writer.push_str(" />");
            }
            MarkdownNode::Emoji(emoji) => {
                self.writer.push(':');
                self.writer.push_str(emoji);
                self.writer.push(':');
            }
            MarkdownNode::Tag(tag) => {
                write!(
                    self.writer,
                    "<a href=\"#{}\">#{tag}</a>",
                    utils::percent_encode::encode(tag, false)
                )?;
            }
            MarkdownNode::SoftBreak => {
                self.writer.push('\n');
            }
            MarkdownNode::HardBreak => {
                self.writer.push_str("<br />\n");
            }
            MarkdownNode::Html(html_box) => {
                self.render_html_node(idx, html_box)?;
            }
            MarkdownNode::BlockQuote => {
                if self.tree.get_first_child(idx).is_some() {
                    self.render_wrapped(idx, "<blockquote>\n", "\n</blockquote>")?
                } else {
                    self.render_wrapped(idx, "<blockquote>\n", "</blockquote>")?
                }
            }
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
            }
            MarkdownNode::List(list) => {
                self.write_list(list, idx)?;
            }
            MarkdownNode::ListItem(_) => unreachable!(),
            MarkdownNode::Table(table) => {
                self.write_table(table, idx)?;
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
                self.writer.push_str(before);
                self.writer.push_str("<hr />");
                self.writer.push_str(after);
            }
            MarkdownNode::FrontMatter(..) => {}
            MarkdownNode::Math(math) => {
                self.write_math(math, idx)?;
            }
            MarkdownNode::Callout(callout) => {
                self.write_callout(callout, idx)?;
            }
            MarkdownNode::Embed(embed) => {
                self.write_embed(embed)?;
            }
        }
        Ok(())
    }
    fn render_wrapped(&mut self, idx: usize, open: &str, close: &str) -> fmt::Result {
        self.write_open(open, idx);
        self.write_children(idx)?;
        self.write_close(close, idx);
        Ok(())
    }
    fn write_children(&mut self, idx: usize) -> fmt::Result {
        if let Some(child_idx) = self.tree.get_first_child(idx) {
            if matches!(
                self.tree[idx].body,
                MarkdownNode::Html(ref html) if matches!(html.as_ref(), html::Html::Block(..))
            ) {
                self.write_html_block_children(child_idx)
            } else {
                self.write_html(child_idx)
            }
        } else {
            Ok(())
        }
    }
    fn render_html_node(&mut self, idx: usize, html_node: &html::Html) -> fmt::Result {
        let html_type = match html_node {
            html::Html::Block(html_type) | html::Html::Inline(html_type) => html_type,
        };
        match html_type {
            html::HtmlType::JSComment(value) => {
                self.writer.push_str("{/*");
                self.writer.push_str(value);
                self.writer.push_str("*/}");
            }
            html::HtmlType::JSExpression(value) => {
                self.writer.push('{');
                self.writer.push_str(value);
                self.writer.push('}');
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
                let open_newline = wrap_full && !has_raw_opening;
                let close_newline = wrap_full
                    && !self.html_block_last_child_has_newline(idx)
                    && !self.html_block_last_child_is_whitespace_text(idx);

                match flag {
                    html::Flag::Full => {
                        self.prepare_open(idx);
                        if !has_raw_opening {
                            Self::push_element_open(self.writer, element);
                            if open_newline {
                                self.writer.push('\n');
                            }
                        }
                        self.write_children(idx)?;
                        if has_raw_closing {
                            self.write_close("", idx);
                        } else {
                            if close_newline {
                                self.writer.push('\n');
                            }
                            Self::push_element_close(self.writer, &element.name);
                            self.finish_close(idx, false, false);
                        }
                    }
                    html::Flag::Begin => {
                        self.prepare_open(idx);
                        if is_inline {
                            Self::push_element_open(self.writer, element);
                        } else if !has_raw_opening {
                            Self::push_element_open(self.writer, element);
                            if self.tree.get_first_child(idx).is_some() {
                                self.writer.push('\n');
                            }
                        }
                        self.write_children(idx)?;
                        self.write_close("", idx);
                    }
                    html::Flag::End => {
                        self.prepare_open(idx);
                        if is_inline {
                            self.write_children(idx)?;
                            Self::push_element_close(self.writer, &element.name);
                            self.finish_close(idx, false, false);
                        } else {
                            Self::push_element_close(self.writer, &element.name);
                            if self.tree.get_first_child(idx).is_some() {
                                self.writer.push('\n');
                            }
                            self.write_children(idx)?;
                            self.write_close("", idx);
                        }
                    }
                    html::Flag::SelfClose => {
                        self.writer.push('<');
                        self.writer.push_str(&element.name);
                        Self::push_element_attrs(self.writer, element);
                        self.writer.push_str("/>");
                    }
                }
            }
            _ => self.render_wrapped(idx, "", "")?,
        }
        Ok(())
    }
    fn push_escaped(output: &mut String, value: &str) {
        let bytes = value.as_bytes();
        let mut written = 0;
        while written < bytes.len() {
            let remaining = &bytes[written..];
            let index = match (
                memchr3(b'&', b'<', b'>', remaining),
                memchr(b'"', remaining),
            ) {
                (Some(left), Some(right)) => written + left.min(right),
                (Some(index), None) | (None, Some(index)) => written + index,
                (None, None) => break,
            };
            output.push_str(&value[written..index]);
            output.push_str(match bytes[index] {
                b'&' => "&amp;",
                b'<' => "&lt;",
                b'>' => "&gt;",
                b'\"' => "&quot;",
                _ => unreachable!(),
            });
            written = index + 1;
        }
        output.push_str(&value[written..]);
    }
    fn push_attr(output: &mut String, name: &str, value: Option<&str>) {
        if let Some(value) = value {
            output.push(' ');
            output.push_str(name);
            output.push_str("=\"");
            Self::push_escaped(output, value);
            output.push('"');
        }
    }
    fn push_element_attrs(output: &mut String, element: &html::Element) {
        if let Some(props) = &element.props {
            for (name, value) in props {
                output.push(' ');
                output.push_str(name);
                if !value.is_empty() {
                    match value {
                        html::PropValue::Literal(value) => {
                            output.push_str("=\"");
                            output.push_str(value);
                            output.push('"');
                        }
                        html::PropValue::Expr(value) => {
                            output.push_str("={");
                            output.push_str(value);
                            output.push('}');
                        }
                    }
                }
            }
        }
    }
    fn push_element_open(output: &mut String, element: &html::Element) {
        output.push('<');
        output.push_str(&element.name);
        Self::push_element_attrs(output, element);
        output.push('>');
    }
    fn push_element_close(output: &mut String, name: &str) {
        output.push_str("</");
        output.push_str(name);
        output.push('>');
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
        id: Option<&str>,
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
        self.prepare_open(paragraph_idx);
        self.writer.push_str("<p");
        Self::push_attr(self.writer, "id", id);
        self.writer.push('>');
        self.write_html_until(first, split)?;
        self.write_close("</p>", paragraph_idx);
        self.write_html(split)?;
        Ok(true)
    }
    fn prepare_open(&mut self, idx: usize) {
        if self.tree[idx].body.is_block_level()
            && self
                .tree
                .get_prev(idx)
                .map(|idx| self.tree[idx].body.is_inline_level())
                .unwrap_or(false)
        {
            self.writer.push('\n');
        }
    }
    fn write_open(&mut self, open: &str, idx: usize) {
        self.prepare_open(idx);
        self.writer.push_str(open);
    }
    fn write_close(&mut self, close: &str, idx: usize) {
        let is_block = self.tree[idx].body.is_block_level();
        let non_final_block = Some(idx) != self.tree.get_last_child(self.tree.get_parent(idx));
        if close == "\n" && is_block && !non_final_block {
            return;
        }
        self.writer.push_str(close);
        self.finish_close(idx, close.is_empty(), close.ends_with('\n'));
    }
    fn finish_close(&mut self, idx: usize, close_is_empty: bool, close_ends_with_newline: bool) {
        let is_block = self.tree[idx].body.is_block_level();
        let non_final_block = Some(idx) != self.tree.get_last_child(self.tree.get_parent(idx));
        if close_is_empty
            && is_block
            && non_final_block
            && matches!(
                self.tree[idx].body,
                MarkdownNode::Html(ref h) if matches!(h.as_ref(), html::Html::Block(..))
            )
        {
            self.writer.push('\n');
        } else if !close_is_empty && is_block && non_final_block && !close_ends_with_newline {
            self.writer.push('\n');
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
        let same_line = |a: u32, b: u32| -> bool {
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            !self.source.as_bytes()[lo as usize..hi as usize].contains(&b'\n')
        };
        !(same_line(self.tree[first].span.start, self.tree[idx].span.start)
            && same_line(self.tree[last].span.end, self.tree[idx].span.end))
    }
    fn html_block_has_raw_opening(&self, idx: usize) -> bool {
        let Some(first) = self.tree.get_first_child(idx) else {
            return false;
        };
        let MarkdownNode::Text(text) = &self.tree[first].body else {
            return false;
        };
        text.resolve(self.source)
            .trim_start_matches(|c| c == ' ' || c == '\t')
            .starts_with('<')
    }
    fn html_block_has_raw_closing(&self, idx: usize, name: &str) -> bool {
        let mut next = self.tree.get_first_child(idx);
        while let Some(child) = next {
            if let MarkdownNode::Text(text) = &self.tree[child].body {
                let text = text.resolve(self.source).as_bytes();
                if text.windows(name.len() + 2).any(|window| {
                    window.starts_with(b"</") && window[2..].eq_ignore_ascii_case(name.as_bytes())
                }) {
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
            MarkdownNode::Text(text)
                if text.resolve(self.source).chars().all(|ch| matches!(ch, ' ' | '\t'))
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
        if let MarkdownNode::Text(text) = &self.tree[idx].body {
            let str = text.resolve(self.source);
            if xml_escape {
                Self::push_escaped(self.writer, str);
            } else {
                self.writer.push_str(str);
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
                self.write_open("<ul>\n", idx);
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(child_idx, bullet.tight, None)?;
                }
                self.write_close("\n</ul>", idx);
            }
            list::List::Ordered(ordered) => {
                self.prepare_open(idx);
                if ordered.start == 1 {
                    self.writer.push_str("<ol>\n");
                } else {
                    write!(self.writer, "<ol start=\"{}\">\n", ordered.start)?;
                }
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(child_idx, ordered.tight, None)?;
                }
                self.write_close("\n</ol>", idx);
            }
            list::List::Task(task) => {
                if task.obsidian {
                    self.write_open("<ul class=\"contains-task-list\">", idx);
                } else {
                    self.write_open("<ul>\n", idx);
                }
                if let Some(child_idx) = self.tree.get_first_child(idx) {
                    self.writer_list_item(child_idx, task.tight, Some(task.obsidian))?;
                }
                self.write_close("\n</ul>", idx);
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
        let mut next = Some(idx);
        while let Some(idx) = next {
            let newline = self.write_list_item_contents(idx, tight, task_list)?;
            next = self.tree.get_next(idx);
            if next.is_some() {
                writeln!(self.writer, "{newline}</li>")?;
            } else {
                write!(self.writer, "{newline}</li>")?;
            }
        }
        Ok(())
    }
    fn write_list_item_contents(
        &mut self,
        idx: usize,
        tight: bool,
        task_list: Option<bool>,
    ) -> Result<&'static str, fmt::Error> {
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
        Ok(newline)
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
        self.write_close("</table>", idx);
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
    fn callout_type_name(callout: &callout::Callout) -> &str {
        match &callout._type {
            callout::CalloutType::Note => "note",
            callout::CalloutType::Abstract => "abstract",
            callout::CalloutType::Info => "info",
            callout::CalloutType::Todo => "todo",
            callout::CalloutType::Tip => "tip",
            callout::CalloutType::Success => "success",
            callout::CalloutType::Question => "question",
            callout::CalloutType::Warning => "warning",
            callout::CalloutType::Failure => "failure",
            callout::CalloutType::Danger => "danger",
            callout::CalloutType::Bug => "bug",
            callout::CalloutType::Example => "example",
            callout::CalloutType::Quote => "quote",
            callout::CalloutType::Custom(value) => value,
        }
    }
    fn callout_default_title(callout: &callout::Callout) -> Cow<'_, str> {
        match &callout._type {
            callout::CalloutType::Note => Borrowed("Note"),
            callout::CalloutType::Abstract => Borrowed("Abstract"),
            callout::CalloutType::Info => Borrowed("Info"),
            callout::CalloutType::Todo => Borrowed("Todo"),
            callout::CalloutType::Tip => Borrowed("Tip"),
            callout::CalloutType::Success => Borrowed("Success"),
            callout::CalloutType::Question => Borrowed("Question"),
            callout::CalloutType::Warning => Borrowed("Warning"),
            callout::CalloutType::Failure => Borrowed("Failure"),
            callout::CalloutType::Danger => Borrowed("Danger"),
            callout::CalloutType::Bug => Borrowed("Bug"),
            callout::CalloutType::Example => Borrowed("Example"),
            callout::CalloutType::Quote => Borrowed("Quote"),
            callout::CalloutType::Custom(v) => {
                let mut chars = v.chars();
                if let Some(first) = chars.next() {
                    let mut title = first.to_uppercase().to_string();
                    title.push_str(chars.as_str());
                    Owned(title)
                } else {
                    Borrowed("")
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
        let class = match callout.foldable {
            None => "callout",
            Some(true) => "callout is-collapsible",
            Some(false) => "callout is-collapsible is-collapsed",
        };
        let typ = Self::callout_type_name(callout);
        let title = match callout.title.as_deref() {
            Some(title) => Borrowed(title),
            None => Self::callout_default_title(callout),
        };
        self.writer.push_str("<div class=\"");
        self.writer.push_str(class);
        self.writer.push_str("\" data-callout=\"");
        Self::push_escaped(self.writer, typ);
        self.writer.push_str("\">\n<div class=\"callout-title\">");
        Self::push_escaped(self.writer, &title);
        self.writer
            .push_str("</div>\n<div class=\"callout-content\">\n");
        if let Some(child_idx) = self.tree.get_first_child(idx) {
            self.write_html(child_idx)?;
        }
        self.writer.push_str("\n</div>\n</div>");
        Ok(())
    }
    fn write_embed(&mut self, embed: &ast::embed::Embed) -> fmt::Result {
        let mut src = embed.path.clone();
        if let Some(reference) = &embed.reference {
            use ast::reference::Reference;
            src.push('#');
            match reference {
                Reference::Heading(value) => src.push_str(value),
                Reference::MultiHeading(values) => {
                    for (index, value) in values.iter().enumerate() {
                        if index != 0 {
                            src.push('#');
                        }
                        src.push_str(value);
                    }
                }
                Reference::BlockId(value) => {
                    src.push('^');
                    src.push_str(value);
                }
            }
        }
        if let Some(attrs) = &embed.attrs {
            if !attrs.is_empty() {
                if src.contains('#') {
                    src.push('&');
                } else {
                    src.push('#');
                }
                for (index, (key, value)) in attrs.iter().enumerate() {
                    if index != 0 {
                        src.push('&');
                    }
                    src.push_str(key);
                    if !value.is_empty() {
                        src.push('=');
                        src.push_str(value);
                    }
                }
            }
        }
        let ext = embed.path.rsplit('.').next().unwrap_or_default();
        let is_ext = |candidate: &str| ext.eq_ignore_ascii_case(candidate);
        if ["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp"]
            .iter()
            .any(|candidate| is_ext(candidate))
        {
            self.writer.push_str("<img src=\"");
            Self::push_escaped(self.writer, &src);
            self.writer.push_str("\" alt=\"");
            Self::push_escaped(self.writer, &embed.path);
            self.writer.push('"');
            if let Some((width, height)) = embed.size {
                write!(self.writer, " width=\"{width}\"")?;
                if let Some(height) = height {
                    write!(self.writer, " height=\"{height}\"")?;
                }
            }
            self.writer.push_str(" />");
            Ok(())
        } else if ["mp3", "wav", "ogg", "m4a", "flac"]
            .iter()
            .any(|candidate| is_ext(candidate))
        {
            self.writer.push_str("<audio controls src=\"");
            Self::push_escaped(self.writer, &src);
            self.writer.push_str("\"></audio>");
            Ok(())
        } else if is_ext("pdf") {
            self.writer.push_str("<iframe src=\"");
            Self::push_escaped(self.writer, &src);
            self.writer.push_str("\"></iframe>");
            Ok(())
        } else {
            self.writer
                .push_str("<span class=\"internal-embed\" src=\"");
            Self::push_escaped(self.writer, &src);
            self.writer.push_str("\"></span>");
            Ok(())
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

impl Document<'_> {
    /// 渲染 HTML。`TextRef::Source` 区间在写出时对文档源码解析，
    /// 因此渲染必须经 Document（而非裸 Tree）。
    pub fn to_html(&self) -> String {
        if self.tree.is_empty() {
            return String::new();
        }
        // Most Markdown inputs expand modestly when rendered to HTML.
        // Reserve upfront to reduce repeated String growth during write!.
        let mut buffer = String::with_capacity(self.tree.node_slots_len().saturating_mul(32));
        let _ = HtmlRender::new(&self.tree, self.source(), &mut buffer).write_html(0);
        buffer
    }
}
