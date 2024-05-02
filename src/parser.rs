use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing};
use crate::line::Line;
use crate::tokenizer::{Location, Token, TokenIterator, Tokenizer};
use crate::tree::Tree;
use crate::{blocks, exts, inlines};

pub struct Node {
    pub body: MarkdownNode,
    pub start: Location,
    pub end: Location,
    pub(crate) processing: bool,
    pub id: Option<String>,
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

#[derive(Debug, Default)]
pub struct ParserOptions {
    /// 当 github_flavored 和 obsidian_flavored 未启用时为 `true`
    pub(crate) default_flavored: bool,
    pub(crate) github_flavored: bool,
    pub(crate) obsidian_flavored: bool,
    pub(crate) cjk_autocorrect: bool,
    pub(crate) smart_punctuation: bool,
}

impl ParserOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn enabled_gfm(self) -> Self {
        Self {
            github_flavored: true,
            default_flavored: false,
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
}

pub struct Parser<'input> {
    pub(crate) tokens: TokenIterator<'input>,
    pub(crate) tree: Tree<Node>,
    pub(crate) options: ParserOptions,
    /// 存储在解析 Block 时能接收 inlines 的 block 的 ID 和剩余未处理的 Line
    pub(crate) inlines: BTreeMap<usize, Vec<Line<'input>>>,
    pub(crate) link_refs: HashMap<String, (String, Option<String>)>, // HRefLabel, (Url, Option<Title>)
    pub(crate) footnotes: HashMap<String, usize>,                    // label, node_id
    pub(crate) footnote_refs: HashMap<String, (usize, usize)>,       // label, index, ref count
    pub(crate) doc: usize,
    /// 应等同于 tree.cur()
    pub(crate) curr_proc_node: usize,
    pub(crate) prev_proc_node: usize,
    pub(crate) last_matched_node: usize,
    pub(crate) last_location: Location,
    pub(crate) all_closed: bool,
    pub(crate) tags: HashSet<String>,
    pub(crate) html_stacks: VecDeque<(String, usize)>, // tag name, node idx
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        Self::new_with_options(text, ParserOptions::default())
    }
    pub fn new_with_options(text: &'input str, options: ParserOptions) -> Self {
        let mut tree = Tree::<Node>::new();
        // println!("创建 Document 节点")
        let doc = tree.append(Node::new(MarkdownNode::Document, Location::default()));
        Self {
            tokens: Tokenizer::new(text).tokenize(),
            inlines: BTreeMap::new(),
            options,
            link_refs: HashMap::new(),
            footnotes: HashMap::new(),
            footnote_refs: HashMap::new(),
            tags: HashSet::new(),
            tree,
            doc,
            curr_proc_node: doc,
            prev_proc_node: doc,
            all_closed: true,
            last_matched_node: doc,
            last_location: Location::default(),
            html_stacks: VecDeque::new(),
        }
    }
    pub fn parse(mut self) -> Tree<Node> {
        self.tree.push();
        // let start = std::time::Instant::now();
        // println!("块解析开始")
        self.parse_blocks();
        // println!("块解析结束[{}ms]", start.elapsed().as_millis());
        // let start = std::time::Instant::now();
        // println!("行解析开始")
        self.parse_inlines();
        // println!("行解析结束[{}ms]", start.elapsed().as_millis());
        self.tree.pop();
        self.tree
    }
    pub fn parse_frontmatter(&mut self) -> Option<serde_yaml::Value> {
        exts::frontmatter::parse(self)
    }
    fn parse_blocks(&mut self) {
        // let mut i = 0;
        while let Some(line) = Line::extract(&mut self.tokens) {
            // println!("处理第 {i} 行")
            // i += 1;
            let last_location = if line.is_blank() {
                self.last_location
            } else {
                line.last_token_end_location()
            };
            self.incorporate_line(line);
            self.last_location = last_location;
        }
        // println!("开始确定块")
        while self.curr_proc_node != self.doc {
            self.finalize(self.curr_proc_node, self.last_location)
        }
        self.tree[self.doc].end = self.last_location;
        // 重置树，后面的不在使用树的状态控制层级而是直接操作层级
        self.tree.reset();
    }
    fn parse_inlines(&mut self) {
        self.parse_reference_link();
        let keys = self.inlines.keys().copied().collect::<Vec<_>>();
        for idx in keys {
            let lines = self.inlines.remove(&idx);
            let node = &self.tree[idx].body;
            if lines.is_none() || !node.accepts_lines() {
                eprintln!("WARNING: Invalid node {node:?} exists inlines");
                continue;
            }
            let mut line = Line::extends(lines.unwrap());
            line.trim_end_matches(|it: &Token| matches!(it, Token::Whitespace(..)));
            // println!("#{idx} {:?} {:?}", self.tree[idx].body, line);
            inlines::process(idx, self, line);
        }
        self.parse_footnote_list();
    }
    fn parse_reference_link(&mut self) {
        let mut next = self.tree.get_first_child(self.doc);
        while let Some(idx) = next {
            next = self.tree.get_next(idx);
            inlines::process_link_reference(self, idx);
        }
    }
    fn parse_footnote_list(&mut self) {
        if self.footnote_refs.is_empty() {
            return;
        }
        let mut values = self.footnote_refs.drain().collect::<Vec<_>>();
        values.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
        let values = values
            .into_iter()
            .map(|(label, (_, ref_count))| (self.footnotes.remove(&label).unwrap(), ref_count))
            .collect::<Vec<_>>();
        inlines::process_footnote_list(self, values);
        self.footnotes.clear();
    }
    fn incorporate_line(&mut self, mut line: Line<'input>) {
        let mut container = self.doc;
        self.prev_proc_node = self.curr_proc_node;
        println!("检查是否存在正在处理的节点");
        while let Some(last_child) = &self.tree.get_last_child(container).and_then(|idx| {
            if self.tree[idx].processing {
                Some(idx)
            } else {
                None
            }
        }) {
            container = *last_child;
            println!("继续处理 {:?}", self.tree[container].body);
            match blocks::process(container, self, &mut line) {
                BlockProcessing::Processed => return,
                BlockProcessing::Further => continue,
                BlockProcessing::Unprocessed => {
                    container = self.tree.get_parent(container);
                    println!("无法处理，执行返回上一层容器");
                    break;
                }
            }
        }
        self.all_closed = container == self.prev_proc_node;
        println!("当前容器 #{container}  {:?}", self.tree[container].body);
        self.last_matched_node = container;
        let mut matched_leaf = !matches!(self.tree[container].body, MarkdownNode::Paragraph)
            && self.tree[container].body.accepts_lines();
        // 查找叶子（可容纳 Inline ）节点
        if !matched_leaf {
            // println!("开始匹配新的节点");
        };
        while !matched_leaf {
            if !line.is_indented()
                && !line
                    .get(line.indent_len())
                    .map(|it| it.is_block_special_token())
                    .unwrap_or(false)
            {
                line.advance_next_nonspace();
                // println!(
                //     "非特殊 indent = {}, {:?}",
                //     line.indent,
                //     line.get(line.indent)
                // );
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

        if !self.all_closed
            && !line.is_blank()
            && matches!(self.tree[self.curr_proc_node].body, MarkdownNode::Paragraph)
        {
            // println!("当前行未结束，存储至之前的 Paragraph")
            self.append_inline(self.curr_proc_node, line);
        } else {
            // 未匹配新的容器节点，尝试追加之前的节点
            if self.tree[self.prev_proc_node].body.support_reprocess() && !line.is_end() {
                blocks::reprocess(self.prev_proc_node, self, &mut line);
            }
            self.close_unmatched_blocks();
            // 判断是否支持接收纯文本行，只有 Paragraph 、HTML Block、Code Block 支持，部分容器是支持存储空白行
            let cur_container = &mut self.tree[container].body;
            if cur_container.accepts_lines() && (!line.is_end() || line.is_blank()) {
                // println!("存储当前行剩余内容")
                if let MarkdownNode::Html(html) = cur_container {
                    let snapshot = line.snapshot();
                    if let Some((before, after)) = html.scan_end(&mut line) {
                        // println!("HTML Block 结束 ..{before}..{after}..")
                        line.resume(snapshot);
                        // 将 Before 前的内容插入到 HTML
                        if before > 0 {
                            let line = line.slice(0, before);
                            // println!("HTML Block 内容 [0..{before}]{line:?}",)
                            self.append_text(
                                line.to_unescape_string(),
                                (line.start_location(), line.last_token_end_location()),
                            );
                        }
                        // 将 After 后的内容插入到新的 Paragraph
                        line.skip(after);
                        self.finalize(container, line.start_location());
                        if !line.is_end() {
                            // println!("HTML 剩余 after={after} len={} {:?}", line.len(), line);
                            let idx =
                                self.append_block(MarkdownNode::Paragraph, line.start_location());
                            self.append_inline(idx, line);
                        }
                    } else {
                        line.resume(snapshot);
                        self.append_text(
                            line.to_unescape_string(),
                            (line.start_location(), line.last_token_end_location()),
                        );
                    }
                } else if !line.is_end() || line.is_blank() {
                    // println!(
                    //     "add line #{container} processing={} {line:?}",
                    //     self.tree[container].processing
                    // );
                    self.append_inline(container, line);
                }
            }
            // 判断行是否已全部消费或者该行是空白行
            else if !line.is_end() && !line.is_blank() {
                // println!("当前行未结束，创建一个新的 Paragraph 存储")
                container = self.append_block(MarkdownNode::Paragraph, line.start_location());
                self.append_inline(container, line);
            } else {
                // println!("当前行没有更多内容了")
            }
        }
    }
    pub fn append_block(&mut self, node: MarkdownNode, loc: Location) -> usize {
        // 如果当前处理中的节点无法容纳插入的节点则退回当上一层
        while !self.tree[self.curr_proc_node].body.can_contain(&node) {
            self.finalize(self.curr_proc_node, loc)
        }
        let idx = self.tree.append(Node::new(node, loc));
        self.tree.push();
        self.curr_proc_node = idx;
        self.last_location = loc;
        // println!("创建节点 #{idx} {:?}", self.tree[idx].body)
        idx
    }
    pub fn append_free_node(&mut self, node: MarkdownNode, loc: Location) -> usize {
        let idx = self.tree.create_node(Node::new(node, loc));
        // println!("创建游离节点 #{idx} {:?}", self.tree[idx].body)
        idx
    }
    pub fn append_to(
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
    pub fn replace_block(&mut self, node: MarkdownNode, loc: Location) -> Option<usize> {
        self.last_location = loc;
        if let Some(idx) = self.tree.peek_up() {
            // println!("替换节点 {:?} => {:?}", self.tree[idx].body, node)
            self.tree[idx].body = node;
            Some(idx)
        } else {
            None
        }
    }
    pub fn append_inline(&mut self, block_idx: usize, line: Line<'input>) {
        self.inlines.entry(block_idx).or_default().push(line)
    }
    pub fn append_text(
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
    pub fn append_text_to(
        &mut self,
        parent: usize,
        content: impl AsRef<str>,
        location: (Location, Location),
    ) -> usize {
        if let Some((idx, MarkdownNode::Text(text))) = self
            .tree
            .get_last_child(parent)
            .filter(|id| self.tree[*id].processing)
            .map(|id| (id, &mut self.tree[id].body))
        {
            text.push_str(content.as_ref());
            self.tree[idx].end = location.1;
            // println!("追加文本到节点 #{idx} {:?}", self.tree[idx].body);
            idx
        } else {
            let idx = self
                .tree
                .append_child(parent, Node::new(content.as_ref().into(), location.0));
            self.tree[idx].end = location.1;
            // println!("创建节点 #{idx} {:?}", self.tree[idx].body)
            idx
        }
    }
    pub fn mark_as_processed(&mut self, idx: usize) {
        self.tree[idx].processing = false;
    }
    pub fn current_proc(&self) -> &Node {
        &self.tree[self.curr_proc_node]
    }
    pub fn close_unmatched_blocks(&mut self) {
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
        #[cfg(debug_assertions)]
        if parent == self.doc {
            println!("块 #{node_id} {:?} 解析完成", node.body)
        } else {
            println!("退出节点 #{node_id} {:?}", node.body)
        }
        if Some(node_id) == self.tree.peek_up() {
            // println!("树退出 #{node_id:?}")
            self.tree.pop();
        }
        self.curr_proc_node = parent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = r#"# Block formatting

You can also use Markdown to create various text blocks, such as:

- Block quotes - Start a line with `﹥` followed by a space.

- Headings:

    1. Heading 1 - Start a line with `#` followed by a space.

    2. Heading 2 - Start a line with `##` followed by a space.

    3. Heading 3 - Start a line with `###` followed by a space.

- Lists, including nested ones:

    - Numbered lists - Start a line with `1.` or `1)` followed by a space.

    - Bulleted lists - Start a line with `*` or `-` followed by a space.

    - To-do lists - Start a line with `[ ]` or `[x]` followed by a space to insert an unchecked or checked list item.

- Code blocks - Start a line with ` ˋˋˋ `.

- Horizontal lines - Start a line with `---`"#;
        let ast = Parser::new(text).parse();
        println!("AST\n---------------------------------------\n{ast:?}---------------------------------------")
    }
}
