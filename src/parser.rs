use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing};
use crate::exts;
use crate::scanner::Scanner;
use crate::span::Span;
use crate::tree::Tree;
use crate::{blocks, inlines};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Serialize;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// Location in the source text (line and column numbers, both starting from 1)
#[derive(Serialize, Eq, PartialEq, Clone, Copy)]
pub struct Location {
    /// Line number, starting from 1
    pub line: u64,
    /// Line column, starting from 1
    pub column: u64,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl Location {
    pub fn new(line: u64, column: u64) -> Self {
        Self { line, column }
    }
}

#[derive(Serialize)]
pub struct Node {
    pub body: MarkdownNode,
    pub start: Location,
    pub end: Location,
    pub(crate) processing: bool,
    pub id: Option<Box<String>>,
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "[{:?},{:?}]{:?}", self.start, self.end, self.body)
        write!(f, "{:?}", self.body)
    }
}
impl Node {
    pub(crate) fn new(body: MarkdownNode, location: Location) -> Self {
        Self {
            body,
            start: location,
            end: location,
            processing: true,
            id: None,
        }
    }
}

pub struct Document {
    pub tree: Tree<Node>,
    pub tags: FxHashSet<String>,
}
impl Deref for Document {
    type Target = Tree<Node>;
    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}
impl Debug for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.tree.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InputTooLarge { limit: usize, actual: usize },
    NodeLimitExceeded { limit: usize, actual: usize },
}

#[derive(Debug, Default, Clone)]
pub struct ParserOptions {
    /// 当 github_flavored 和 obsidian_flavored 未启用时为 `true`
    pub(crate) default_flavored: bool,
    pub(crate) github_flavored: bool,
    pub(crate) gfm_extended_autolink: bool,
    pub(crate) obsidian_flavored: bool,
    pub(crate) mdx_component: bool,
    pub(crate) cjk_autocorrect: bool,
    pub(crate) smart_punctuation: bool,
    pub(crate) normalize_chinese_punctuation: bool,
    pub(crate) cjk_friendly_delimiters: bool,
    pub(crate) cjk_nouns: FxHashSet<String>,
    /// 启用从 frontmatter 提取 cjk nouns 并指定字段名称
    pub(crate) cjk_nouns_from_frontmatter: Option<String>,
    pub(crate) max_input_bytes: Option<usize>,
    pub(crate) max_nodes: Option<usize>,
}

impl ParserOptions {
    pub fn enabled_gfm(self) -> Self {
        Self {
            github_flavored: true,
            default_flavored: false,
            ..self
        }
    }
    pub fn enabled_gfm_autolink(self) -> Self {
        Self {
            gfm_extended_autolink: true,
            ..self
        }
    }
    pub fn enabled_ofm(self) -> Self {
        Self {
            obsidian_flavored: true,
            default_flavored: false,
            ..self
        }
    }
    pub fn enabled_mdx_component(self) -> Self {
        Self {
            mdx_component: true,
            ..self
        }
    }
    pub fn enabled_cjk_autocorrect(self) -> Self {
        Self {
            cjk_autocorrect: true,
            ..self
        }
    }
    pub fn enabled_smart_punctuation(self) -> Self {
        Self {
            smart_punctuation: true,
            ..self
        }
    }
    pub fn enabled_normalize_chinese_punctuation(self) -> Self {
        Self {
            normalize_chinese_punctuation: true,
            ..self
        }
    }
    pub fn enabled_cjk_friendly_delimiters(self) -> Self {
        Self {
            cjk_friendly_delimiters: true,
            ..self
        }
    }
    pub fn with_max_input_bytes(self, max_input_bytes: usize) -> Self {
        Self {
            max_input_bytes: Some(max_input_bytes),
            ..self
        }
    }
    pub fn with_max_nodes(self, max_nodes: usize) -> Self {
        Self {
            max_nodes: Some(max_nodes),
            ..self
        }
    }
    pub fn with_cjk_nouns<I, S>(mut self, nouns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.cjk_nouns.clear();
        self.cjk_nouns.extend(nouns.into_iter().map(Into::into));
        self
    }
    pub fn with_cjk_nouns_from_frontmatter(self, field: impl Into<String>) -> Self {
        Self {
            cjk_nouns_from_frontmatter: Some(field.into()),
            ..self
        }
    }
}

pub struct Parser<'input> {
    pub(crate) scanner: Scanner<'input>,
    pub(crate) tree: Tree<Node>,
    pub(crate) options: ParserOptions,
    /// 存储在解析 Block 时能接收 inlines 的 block 的 ID 和剩余未处理的 Span
    pub(crate) inlines: FxHashMap<usize, Vec<Span<'input>>>,
    pub(crate) link_refs: FxHashMap<String, (String, Option<String>)>, // HRefLabel, (Url, Option<Title>)
    pub(crate) footnotes: FxHashMap<String, usize>,                    // label, node_id
    pub(crate) footnote_refs: FxHashMap<String, (usize, usize)>,       // label, index, ref count
    pub(crate) doc: usize,
    /// 应等同于 tree.cur()
    pub(crate) curr_proc_node: usize,
    pub(crate) prev_proc_node: usize,
    pub(crate) last_matched_node: usize,
    pub(crate) last_location: Location,
    pub(crate) all_closed: bool,
    pub(crate) tags: FxHashSet<String>,
    pub(crate) html_stacks: VecDeque<(String, usize)>, // tag name, node idx
    pub(crate) parse_error: Option<ParseError>,
}

impl<'input> Parser<'input> {
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    pub fn new(text: &'input str) -> Self {
        Self::new_with_options(text, ParserOptions::default())
    }
    pub fn new_with_options(text: &'input str, options: ParserOptions) -> Self {
        // 预估节点数量：大约每 10 字节一个节点
        let estimated_nodes = text.len() / 10;
        let mut tree = Tree::<Node>::with_capacity(estimated_nodes.min(8192));
        let doc = tree.append(Node::new(MarkdownNode::Document, Location::default()));
        let scanner = Scanner::new(text);
        // 预估 inline 容器数量
        let estimated_inlines = text.len() / 80;
        Self {
            scanner,
            inlines: FxHashMap::with_capacity_and_hasher(
                estimated_inlines.min(1024),
                Default::default(),
            ),
            options,
            link_refs: FxHashMap::default(),
            footnotes: FxHashMap::default(),
            footnote_refs: FxHashMap::default(),
            tags: FxHashSet::default(),
            tree,
            doc,
            curr_proc_node: doc,
            prev_proc_node: doc,
            all_closed: true,
            last_matched_node: doc,
            last_location: Location::default(),
            html_stacks: VecDeque::new(),
            parse_error: None,
        }
    }
    pub fn parse(self) -> Document {
        self.parse_checked()
            .expect("parse failed: input exceeds parser limits")
    }
    pub fn parse_checked(mut self) -> Result<Document, ParseError> {
        self.ensure_limits()?;
        self.parse_frontmatter()?;
        self.tree.push();
        self.parse_blocks();
        if let Some(err) = self.parse_error.take() {
            self.tree.pop();
            return Err(err);
        }
        self.parse_inlines();
        if let Some(err) = self.parse_error.take() {
            self.tree.pop();
            return Err(err);
        }
        self.tree.pop();
        Ok(Document {
            tree: self.tree,
            tags: self.tags,
        })
    }
    fn merge_cjk_nouns_from_frontmatter(&mut self, frontmatter: &crate::exts::yaml::YamlMap) {
        use crate::exts::yaml::YamlValue;

        let Some(value) = self
            .options
            .cjk_nouns_from_frontmatter
            .as_deref()
            .and_then(|field| frontmatter.get(field))
        else {
            return;
        };

        let mut merged = self.options.cjk_nouns.clone();
        let mut push_unique = |s: &str| {
            if !s.is_empty() && !merged.iter().any(|it| it == s) {
                merged.insert(s.to_string());
            }
        };

        match value {
            YamlValue::String(s) => push_unique(s.trim()),
            YamlValue::List(items) => {
                for item in items {
                    if let YamlValue::String(s) = item {
                        push_unique(s.trim());
                    }
                }
            }
            _ => {}
        }
        self.options.cjk_nouns = merged;
    }
    pub fn parse_frontmatter(&mut self) -> Result<(), ParseError> {
        if let Some(frontmatter) = exts::frontmatter::parse(self) {
            self.merge_cjk_nouns_from_frontmatter(&frontmatter);
            let idx = self.tree.append_child(
                self.doc,
                Node::new(
                    MarkdownNode::FrontMatter(Box::new(frontmatter)),
                    Location::default(),
                ),
            );
            self.tree[idx].processing = false;
            self.tree[idx].end = self.scanner.location();
            if self.reach_node_limit() {
                if let Some(err) = self.parse_error.take() {
                    return Err(err);
                }
                return Err(ParseError::NodeLimitExceeded {
                    limit: self.options.max_nodes.unwrap_or(0),
                    actual: self.tree.node_slots_len(),
                });
            }
        }
        Ok(())
    }

    // +9.1691ms
    //     while            +4.6833ms
    //     incorporate_line +4.4858ms
    //     ...              +1ms
    fn parse_blocks(&mut self) {
        while let Some(line) = Span::extract(&mut self.scanner) {
            if self.reach_node_limit() {
                break;
            }
            let last_location = if line.is_blank() {
                self.last_location
            } else {
                line.last_token_end_location()
            };
            self.incorporate_line(line);
            self.last_location = last_location;
            if self.reach_node_limit() {
                break;
            }
        }
        if self.parse_error.is_some() {
            return;
        }
        while self.curr_proc_node != self.doc {
            self.finalize(self.curr_proc_node, self.last_location)
        }
        self.tree[self.doc].end = self.last_location;
        self.tree.reset();
    }
    // +9.5869ms
    //     inlines::process +8.729ms
    fn parse_inlines(&mut self) {
        if self.reach_node_limit() {
            return;
        }
        self.parse_reference_link();
        // drain inlines map 避免额外的 keys Vec 分配
        let inlines = std::mem::take(&mut self.inlines);
        for (idx, mut spans) in inlines {
            if self.reach_node_limit() {
                return;
            }
            let node = &self.tree[idx].body;
            if !node.accepts_lines() {
                eprintln!("WARNING: Invalid node {node:?} exists inlines");
                continue;
            }
            // 去除最后一个 Span 末尾的空白
            if let Some(last) = spans.last_mut() {
                last.trim_end_matches(|b: u8| b == b' ' || b == b'\t');
            }
            inlines::process(idx, self, spans);
            self.normalize_component_children(idx);
            if self.reach_node_limit() {
                return;
            }
        }
        if self.reach_node_limit() {
            return;
        }
        self.parse_footnote_list();
    }
    fn ensure_limits(&self) -> Result<(), ParseError> {
        if let Some(limit) = self.options.max_input_bytes {
            let actual = self.scanner.source().len();
            if actual > limit {
                return Err(ParseError::InputTooLarge { limit, actual });
            }
        }
        if let Some(limit) = self.options.max_nodes {
            let actual = self.tree.node_slots_len();
            if actual > limit {
                return Err(ParseError::NodeLimitExceeded { limit, actual });
            }
        }
        Ok(())
    }
    fn reach_node_limit(&mut self) -> bool {
        let Some(limit) = self.options.max_nodes else {
            return false;
        };
        let actual = self.tree.node_slots_len();
        if actual <= limit {
            return false;
        }
        if self.parse_error.is_none() {
            self.parse_error = Some(ParseError::NodeLimitExceeded { limit, actual });
        }
        true
    }
    fn parse_reference_link(&mut self) {
        let mut nodes = Vec::new();
        self.collect_ref_link_candidates(self.doc, &mut nodes);
        for idx in nodes {
            match self.tree[idx].body {
                MarkdownNode::Paragraph => inlines::process_link_reference(self, idx),
                MarkdownNode::Heading(crate::ast::heading::Heading::SETEXT(_)) => {
                    inlines::process_setext_heading_link_reference(self, idx)
                }
                _ => {}
            }
        }
    }
    /// 只收集 Paragraph 和 SETEXT Heading 节点（用于 reference link 解析）
    fn collect_ref_link_candidates(&self, parent: usize, out: &mut Vec<usize>) {
        let mut next = self.tree.get_first_child(parent);
        while let Some(idx) = next {
            match &self.tree[idx].body {
                MarkdownNode::Paragraph
                | MarkdownNode::Heading(crate::ast::heading::Heading::SETEXT(_)) => {
                    out.push(idx);
                }
                _ => {
                    self.collect_ref_link_candidates(idx, out);
                }
            }
            next = self.tree.get_next(idx);
        }
    }
    fn parse_footnote_list(&mut self) {
        if self.footnote_refs.is_empty() {
            return;
        }
        let mut values = self.footnote_refs.drain().collect::<Vec<_>>();
        values.sort_by(|a, b| a.1.0.cmp(&b.1.0));
        let values = values
            .into_iter()
            .filter_map(|(label, (_, ref_count))| {
                self.footnotes
                    .remove(&label)
                    .map(|node_id| (node_id, ref_count))
            })
            .collect::<Vec<_>>();
        inlines::process_footnote_list(self, values);
        self.footnotes.clear();
    }
    fn incorporate_line(&mut self, mut line: Span<'input>) {
        let mut container = self.doc;
        self.prev_proc_node = self.curr_proc_node;
        while let Some(last_child) = &self
            .tree
            .get_last_child(container)
            .filter(|idx| self.tree[*idx].processing)
        {
            container = *last_child;
            match blocks::process(container, self, &mut line) {
                BlockProcessing::Processed => {
                    return;
                }
                BlockProcessing::Further => {
                    continue;
                }
                BlockProcessing::Unprocessed => {
                    container = self.tree.get_parent(container);
                    break;
                }
            }
        }
        self.all_closed = container == self.prev_proc_node;
        self.last_matched_node = container;
        let mut matched_leaf = !matches!(self.tree[container].body, MarkdownNode::Paragraph)
            && self.tree[container].body.accepts_lines();
        while !matched_leaf {
            if !line.is_indented()
                && !line
                    .get(line.indent_len())
                    .map(|b| Span::is_special_byte(b))
                    .unwrap_or(false)
            {
                line.advance_next_nonspace();
                break;
            }
            match blocks::matcher(container, self, &mut line) {
                BlockMatching::MatchedLeaf => {
                    container = self.curr_proc_node;
                    matched_leaf = true;
                }
                BlockMatching::MatchedContainer => {
                    container = self.curr_proc_node;
                }
                BlockMatching::Unmatched => {
                    line.advance_next_nonspace();
                    break;
                }
            }
        }

        let break_html_paragraph = if !self.all_closed
            && !line.is_blank_to_end()
            && matches!(self.tree[self.curr_proc_node].body, MarkdownNode::Paragraph)
        {
            let indent_len = line.indent_len();
            let has_multi_end_tag_chain = line.get(indent_len) == Some(b'<')
                && count_html_end_tag_chain(line.slice(indent_len, line.len()).as_str()) >= 2;
            if has_multi_end_tag_chain {
                true
            } else {
                let parent = self.tree.get_parent(self.curr_proc_node);
                if matches!(
                    self.tree[parent].body,
                    MarkdownNode::Html(ref h) if matches!(h.as_ref(),
                        crate::ast::html::Html::Block(
                            crate::ast::html::HtmlType::CanonicalBlockTag(..)
                                | crate::ast::html::HtmlType::GenericTag(..)
                                | crate::ast::html::HtmlType::Component(..)
                        )
                    )
                ) {
                    if line.get(indent_len) == Some(b'<') {
                        let mut scan_line = line.slice(indent_len, line.len());
                        let is_end_tag = matches!(
                            crate::blocks::html::scan_html_type(
                                &mut scan_line,
                                false,
                                self.options.mdx_component
                            ),
                            Some((
                                _,
                                _,
                                crate::ast::html::HtmlType::CanonicalBlockTag(
                                    _,
                                    crate::ast::html::Flag::End
                                ) | crate::ast::html::HtmlType::GenericTag(
                                    _,
                                    crate::ast::html::Flag::End
                                ) | crate::ast::html::HtmlType::Component(
                                    _,
                                    crate::ast::html::Flag::End
                                )
                            ))
                        );
                        is_end_tag
                            || count_html_end_tag_chain(line.slice(indent_len, line.len()).as_str())
                                >= 2
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        } else {
            false
        };

        if !self.all_closed
            && !line.is_blank_to_end()
            && matches!(self.tree[self.curr_proc_node].body, MarkdownNode::Paragraph)
            && !break_html_paragraph
        {
            // 段落延续行：跳过前导空白
            line.advance_next_nonspace();
            self.append_inline(self.curr_proc_node, line);
        } else {
            if self.tree[self.prev_proc_node].body.support_reprocess() && !line.is_end() {
                blocks::reprocess(self.prev_proc_node, self, &mut line);
            }
            self.close_unmatched_blocks();
            let cur_container = &mut self.tree[container].body;
            if cur_container.accepts_lines() && (!line.is_end() || line.is_blank()) {
                if let MarkdownNode::Html(html) = cur_container {
                    let component_block_open = matches!(
                        html.as_ref(),
                        crate::ast::html::Html::Block(crate::ast::html::HtmlType::Component(
                            _,
                            crate::ast::html::Flag::Begin
                        ))
                    );
                    let snapshot = line.snapshot();
                    if let Some((before, after)) = html.scan_end_span(&mut line) {
                        line.resume(&snapshot);
                        if before > 0 {
                            let sub = line.slice(0, before);
                            if component_block_open {
                                self.append_inline(container, sub);
                            } else {
                                self.append_html_raw_text_line(
                                    container,
                                    sub.to_unescape_string(),
                                    (sub.start_location(), sub.last_token_end_location()),
                                );
                            }
                        }
                        line.skip(after);
                        self.finalize(container, line.start_location());
                        if !line.is_end() {
                            let idx =
                                self.append_block(MarkdownNode::Paragraph, line.start_location());
                            self.append_inline(idx, line);
                        }
                    } else {
                        line.resume(&snapshot);
                        if component_block_open {
                            self.append_inline(container, line.slice(0, line.len()));
                        } else {
                            self.append_html_raw_text_line(
                                container,
                                line.to_unescape_string(),
                                (line.start_location(), line.last_token_end_location()),
                            );
                        }
                    }
                } else if !line.is_end() || line.is_blank() {
                    if matches!(self.tree[container].body, MarkdownNode::Paragraph) {
                        line.advance_next_nonspace();
                    }
                    self.append_inline(container, line);
                }
            } else if !line.is_end() && !line.is_blank() {
                container = self.append_block(MarkdownNode::Paragraph, line.start_location());
                self.append_inline(container, line);
            }
        }
    }
    pub(crate) fn append_block(&mut self, node: MarkdownNode, loc: Location) -> usize {
        // 如果当前处理中的节点无法容纳插入的节点则退回当上一层
        while !self.tree[self.curr_proc_node].body.can_contain(&node) {
            self.finalize(self.curr_proc_node, loc)
        }
        let idx = self.tree.append(Node::new(node, loc));
        self.tree.push();
        self.curr_proc_node = idx;
        self.last_location = loc;
        // println!(
        //     "创建节点 #{idx} {:?} ↑ {:?} ← {:?} 🤣 {:?}",
        //     self.tree[idx].body,
        //     self.tree.get_parent(idx),
        //     self.tree.get_prev(idx),
        //     self.tree
        //         .get_prev(idx)
        //         .and_then(|idx| self.tree.get_next(idx))
        // );
        idx
    }
    pub(crate) fn append_free_node(&mut self, node: MarkdownNode, loc: Location) -> usize {
        let idx = self.tree.create_node(Node::new(node, loc));
        // #[cfg(debug_assertions)]
        // println!("创建游离节点 #{idx} {:?}", self.tree[idx].body);
        idx
    }
    pub(crate) fn append_to(
        &mut self,
        id: usize,
        node: MarkdownNode,
        location: (Location, Location),
    ) -> usize {
        let idx = self.tree.append_child(id, Node::new(node, location.0));
        self.tree[idx].end = location.1;
        // println!("创建节点 #{idx} {:?}", self.tree[idx].body)
        idx
    }
    pub(crate) fn replace_block(&mut self, node: MarkdownNode, loc: Location) -> Option<usize> {
        self.last_location = loc;
        if let Some(idx) = self.tree.peek_up() {
            // println!("替换节点 {:?} => {:?}", self.tree[idx].body, node)
            self.tree[idx].body = node;
            Some(idx)
        } else {
            None
        }
    }
    pub(crate) fn append_inline(&mut self, block_idx: usize, line: Span<'input>) {
        self.inlines.entry(block_idx).or_default().push(line)
    }
    pub(crate) fn append_text(
        &mut self,
        content: impl AsRef<str>,
        location: (Location, Location),
    ) -> usize {
        // 如果当前处理中的节点无法容纳插入的节点则退回当上一层
        if !self.tree[self.curr_proc_node].body.accepts_lines() {
            panic!(
                "Failed to append text to {:?}  block, the block could not accepts lines",
                self.tree[self.curr_proc_node].body
            )
        }
        let idx = self
            .tree
            .append(Node::new(content.as_ref().into(), location.0));
        self.tree[idx].end = location.1;
        // println!("创建节点 #{idx} {:?}", self.tree[idx].body)
        idx
    }
    /// 插入文本当目标节点，这会自动合并相邻 *仍在处理* 的 Text 节点
    pub(crate) fn append_text_to(
        &mut self,
        parent: usize,
        content: &str,
        location: (Location, Location),
    ) -> usize {
        let transformed = if self.options.smart_punctuation
            && crate::utils::smart_punctuation::needs_smart_punctuation(content)
        {
            Some(crate::utils::smart_punctuation::smart_punctuation(content))
        } else {
            None
        };

        if let Some((idx, MarkdownNode::Text(text))) = self
            .tree
            .get_last_child(parent)
            .filter(|id| self.tree[*id].processing)
            .map(|id| (id, &mut self.tree[id].body))
        {
            if let Some(cow) = &transformed {
                text.push_str(cow.as_ref());
            } else {
                text.push_str(content);
            }
            self.tree[idx].end = location.1;
            return idx;
        }

        let text = match transformed {
            Some(std::borrow::Cow::Owned(s)) => s,
            Some(std::borrow::Cow::Borrowed(s)) => s.to_string(),
            None => content.to_owned(),
        };
        let idx = self
            .tree
            .append_child(parent, Node::new(MarkdownNode::Text(text), location.0));
        self.tree[idx].end = location.1;
        idx
    }
    /// 与 append_text_to 相同，但直接接受 String 避免重复分配
    pub(crate) fn append_text_to_owned(
        &mut self,
        parent: usize,
        mut content: String,
        location: (Location, Location),
    ) -> usize {
        // 应用 smart punctuation 转换（dash 和 ellipsis）
        if self.options.smart_punctuation
            && crate::utils::smart_punctuation::needs_smart_punctuation(&content)
        {
            let transformed = crate::utils::smart_punctuation::smart_punctuation(&content);
            if let std::borrow::Cow::Owned(new_content) = transformed {
                content = new_content;
            }
        }

        if let Some((idx, MarkdownNode::Text(text))) = self
            .tree
            .get_last_child(parent)
            .filter(|id| self.tree[*id].processing)
            .map(|id| (id, &mut self.tree[id].body))
        {
            text.push_str(&content);
            self.tree[idx].end = location.1;
            idx
        } else {
            let idx = self
                .tree
                .append_child(parent, Node::new(MarkdownNode::Text(content), location.0));
            self.tree[idx].end = location.1;
            idx
        }
    }
    #[inline]
    pub(crate) fn append_text_char_to(
        &mut self,
        parent: usize,
        ch: char,
        location: (Location, Location),
    ) -> usize {
        if let Some((idx, MarkdownNode::Text(text))) = self
            .tree
            .get_last_child(parent)
            .filter(|id| self.tree[*id].processing)
            .map(|id| (id, &mut self.tree[id].body))
        {
            text.push(ch);
            self.tree[idx].end = location.1;
            return idx;
        }
        let mut text = String::new();
        text.push(ch);
        let idx = self
            .tree
            .append_child(parent, Node::new(MarkdownNode::Text(text), location.0));
        self.tree[idx].end = location.1;
        idx
    }
    pub(crate) fn mark_as_processed(&mut self, idx: usize) {
        self.tree[idx].processing = false;
    }
    pub(crate) fn current_proc(&self) -> &Node {
        &self.tree[self.curr_proc_node]
    }
    pub(crate) fn close_unmatched_blocks(&mut self) {
        if self.all_closed {
            return;
        }
        loop {
            if self.prev_proc_node == self.last_matched_node {
                break;
            }
            let parent = self.tree.get_parent(self.prev_proc_node);
            self.finalize(self.prev_proc_node, self.last_location);
            self.prev_proc_node = parent
        }
        self.all_closed = true;
    }
    /// 调用指定节点的 finalize 方法处理并关闭该节点，将当前节点指针移动至父节点
    pub(crate) fn finalize(&mut self, node_id: usize, location: Location) {
        let parent = self.tree.get_parent(node_id);
        assert_ne!(
            node_id, self.doc,
            "Unable call finalize to process the Document Node"
        );
        if !self.tree[node_id].processing {
            self.curr_proc_node = parent;
            return;
        }
        blocks::after(node_id, self, location);
        let node = &mut self.tree[node_id];
        node.processing = false;
        if Some(node_id) == self.tree.peek_up() {
            self.tree.pop();
        }
        self.curr_proc_node = parent;
    }

    fn append_html_raw_text_line(
        &mut self,
        parent: usize,
        content: String,
        location: (Location, Location),
    ) -> usize {
        let mut value = content;
        if self.tree.get_last_child(parent).is_some() {
            value.insert(0, '\n');
        }
        self.append_text_to_owned(parent, value, location)
    }

    fn is_component_node(&self, idx: usize) -> bool {
        matches!(
            &self.tree[idx].body,
            MarkdownNode::Html(h)
                if matches!(
                    h.as_ref(),
                    crate::ast::html::Html::Block(crate::ast::html::HtmlType::Component(..))
                        | crate::ast::html::Html::Inline(crate::ast::html::HtmlType::Component(..))
                )
        )
    }

    fn component_name_and_flag(&self, idx: usize) -> Option<(String, crate::ast::html::Flag)> {
        let MarkdownNode::Html(h) = &self.tree[idx].body else {
            return None;
        };
        match h.as_ref() {
            crate::ast::html::Html::Block(crate::ast::html::HtmlType::Component(element, flag))
            | crate::ast::html::Html::Inline(crate::ast::html::HtmlType::Component(
                element,
                flag,
            )) => Some((element.name.clone(), flag.clone())),
            _ => None,
        }
    }

    fn set_component_flag_full(&mut self, idx: usize) {
        let MarkdownNode::Html(h) = &mut self.tree[idx].body else {
            return;
        };
        match h.as_mut() {
            crate::ast::html::Html::Block(crate::ast::html::HtmlType::Component(_, flag))
            | crate::ast::html::Html::Inline(crate::ast::html::HtmlType::Component(_, flag)) => {
                *flag = crate::ast::html::Flag::Full
            }
            _ => {}
        }
    }

    fn remove_whitespace_text_children(&mut self, parent: usize) {
        let mut current = self.tree.get_first_child(parent);
        while let Some(idx) = current {
            current = self.tree.get_next(idx);
            if let MarkdownNode::Text(text) = &self.tree[idx].body {
                if text.chars().all(|ch| matches!(ch, ' ' | '\t')) {
                    self.tree.remove(idx);
                }
            }
        }
    }

    fn normalize_component_children(&mut self, parent: usize) {
        if !self.is_component_node(parent) {
            return;
        }
        self.remove_whitespace_text_children(parent);

        let mut stack: Vec<(String, usize)> = Vec::new();
        let mut current = self.tree.get_first_child(parent);
        while let Some(idx) = current {
            let next = self.tree.get_next(idx);
            if let Some((name, flag)) = self.component_name_and_flag(idx) {
                match flag {
                    crate::ast::html::Flag::Begin => stack.push((name, idx)),
                    crate::ast::html::Flag::End => {
                        if let Some(pos) = stack.iter().rposition(|(n, _)| *n == name) {
                            let (_, begin_idx) = stack.remove(pos);
                            let mut walker = self.tree.get_next(begin_idx);
                            while let Some(child) = walker {
                                if child == idx {
                                    break;
                                }
                                walker = self.tree.get_next(child);
                                self.tree.unlink(child);
                                self.tree.set_parent(child, begin_idx);
                            }
                            self.set_component_flag_full(begin_idx);
                            self.tree.remove(idx);
                            current = self.tree.get_next(begin_idx);
                            continue;
                        }
                    }
                    _ => {}
                }
            }
            current = next;
        }

        let mut child = self.tree.get_first_child(parent);
        while let Some(idx) = child {
            child = self.tree.get_next(idx);
            self.normalize_component_children(idx);
        }
    }
}

fn count_html_end_tag_chain(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut count = 0usize;
    while i < bytes.len() {
        while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
        if i + 3 > bytes.len() || bytes[i] != b'<' || bytes[i + 1] != b'/' {
            break;
        }
        i += 2;
        let name_start = i;
        if i >= bytes.len() || !bytes[i].is_ascii_alphabetic() {
            return 0;
        }
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-') {
            i += 1;
        }
        if i == name_start {
            return 0;
        }
        while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'>' {
            return 0;
        }
        i += 1;
        count += 1;
    }
    if bytes[i..].iter().all(|b| *b == b' ' || *b == b'\t') {
        count
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseError, Parser, ParserOptions};

    #[test]
    fn parse_checked_rejects_oversized_input() {
        let text = "abcd";
        let parser =
            Parser::new_with_options(text, ParserOptions::default().with_max_input_bytes(3));
        let result = parser.parse_checked();
        assert!(matches!(
            result,
            Err(ParseError::InputTooLarge {
                limit: 3,
                actual: 4
            })
        ));
    }

    #[test]
    fn parse_checked_rejects_node_overflow() {
        let text = "# hi";
        let parser = Parser::new_with_options(text, ParserOptions::default().with_max_nodes(1));
        let result = parser.parse_checked();
        assert!(matches!(
            result,
            Err(ParseError::NodeLimitExceeded { limit: 1, .. })
        ));
    }
}
