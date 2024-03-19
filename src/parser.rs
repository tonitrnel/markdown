use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing};
use crate::line::Line;
use crate::tokenizer::{Location, TokenIterator, Tokenizer};
use crate::tree::Tree;
use crate::{blocks, exts};

pub struct Node {
    pub body: MarkdownNode,
    pub column: u64,
    pub line: u64,
    pub processing: bool,
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.body)
    }
}
impl Node {
    pub(crate) fn new(body: MarkdownNode, location: Location) -> Self {
        Self {
            body,
            column: location.column,
            line: location.line,
            processing: true,
        }
    }
}

pub struct Parser<'input> {
    pub(crate) tokens: TokenIterator<'input>,
    pub(crate) tree: Tree<Node>,
    /// 存储在解析 Block 时能接收 inlines 的 block 的 ID 和剩余未处理的 Line
    pub(crate) inlines: HashMap<usize, Vec<Line<'input>>>,
    pub(crate) doc: usize,
    /// 应等同于 tree.cur()
    pub(crate) curr_proc_node: usize,
    pub(crate) prev_proc_node: usize,
    pub(crate) last_matched_node: usize,
    pub(crate) all_closed: bool,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        let mut tree = Tree::<Node>::new();
        println!("创建 document 节点");
        let doc = tree.append(Node::new(MarkdownNode::Document, Location::default()));
        Self {
            tokens: Tokenizer::new(text).tokenize(),
            inlines: HashMap::new(),
            tree,
            doc,
            curr_proc_node: doc,
            prev_proc_node: doc,
            all_closed: true,
            last_matched_node: doc,
        }
    }
    pub fn parse(mut self) -> Tree<Node> {
        self.tree.push();
        println!("块解析");
        self.parse_blocks();
        println!("内联行解析");
        self.parse_inlines();
        self.tree.pop();
        self.tree
    }
    pub fn parse_frontmatter(&mut self) -> Option<serde_yaml::Value> {
        exts::frontmatter::parse(self)
    }
    fn parse_blocks(&mut self) {
        let mut i = 0;
        while let Some(line) = Line::extract(&mut self.tokens) {
            println!("处理第 {i} 行");
            i += 1;
            self.incorporate_line(line);
        }
        println!("开始确定块");
        while self.curr_proc_node != self.doc {
            self.finalize(self.curr_proc_node)
        }
    }
    fn parse_inlines(&mut self) {
        for (idx, lines) in self.inlines.iter() {
            println!("{:?}", self.tree[*idx].body);
            for line in lines {
                println!("    \"{}\"", line)
            }
        }
        // todo!()
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
                    break;
                }
            }
        }
        self.all_closed = container == self.prev_proc_node;
        self.last_matched_node = container;
        let mut matched_leaf = !matches!(self.tree[container].body, MarkdownNode::Paragraph)
            && self.tree[container].body.accepts_lines();
        // 查找叶子（可容纳 Inline ）节点
        if !matched_leaf {
            println!("开始匹配新的节点")
        };
        while !matched_leaf {
            if !line.is_indented()
                && !line
                    .get(line.indent + 1)
                    .map(|it| it.is_special_token())
                    .unwrap_or(false)
            {
                line.advance_next_nonspace();
                break;
            }
            match blocks::matcher(self, &mut line) {
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
            println!("创建 {:?} 节点", self.tree[container].body);
        }
        if !self.all_closed
            && !line.is_blank()
            && matches!(self.tree[self.curr_proc_node].body, MarkdownNode::Paragraph)
        {
            println!("当前行未结束，存储至之前的 Paragraph");
            self.inlines
                .entry(self.curr_proc_node)
                .or_default()
                .push(line)
        } else {
            self.close_unmatched_blocks();
            // 判断是否支持接收纯文本行，只有 Paragraph 、HTML Block、Code Block 支持，部分容器是支持存储空白行
            if self.tree[container].body.accepts_lines() && !line.is_end() {
                println!("存储当前行剩余内容");
                self.inlines.entry(container).or_default().push(line);
                // todo: process html
            }
            // 判断行是否已全部消费或者该行是空白行
            else if !line.is_end() && !line.is_blank() {
                println!("当前行未结束，创建一个新的 Paragraph 存储");
                let idx = self.append_block(MarkdownNode::Paragraph, line[0].location);
                self.append_inline(idx, line);
            } else {
                println!("当前行没有更多内容了");
            }
        }
        // non indented code and common block start token
        // if line.indent < 4 && !line.starts_with_matches(Parser::is_special_token, 1) {}
    }
    fn container(&self) -> Option<&Node> {
        self.tree.cur().map(|id| &self.tree[id])
    }
    pub fn append_block(&mut self, node: MarkdownNode, loc: Location) -> usize {
        // 如果当前处理中的节点无法容纳插入的节点则退回当上一层
        while (!self.tree[self.curr_proc_node].body.can_contain(&node)) {
            self.finalize(self.curr_proc_node)
        }
        let idx = self.tree.append(Node::new(node, loc));
        self.tree.push();
        self.curr_proc_node = idx;
        idx
    }
    pub fn replace_block(&mut self, node: MarkdownNode) -> Option<usize> {
        if let Some(idx) = self.tree.peek_up() {
            self.tree[idx].body = node;
            Some(idx)
        } else {
            None
        }
    }
    pub fn append_inline(&mut self, block_idx: usize, line: Line<'input>) {
        self.inlines.entry(block_idx).or_default().push(line)
    }
    pub fn append_text(&mut self, content: impl AsRef<str>, loc: Location) -> usize {
        // 如果当前处理中的节点无法容纳插入的节点则退回当上一层
        if !self.tree[self.curr_proc_node].body.accepts_lines() {
            panic!(
                "Failed to append text to {:?}  block, the block could not accepts lines",
                self.tree[self.curr_proc_node].body
            )
        }
        let idx = self.tree.append(Node::new(content.as_ref().into(), loc));
        idx
    }
    pub fn current_container(&self) -> &Node {
        let idx = self.tree.peek_up().unwrap_or(self.doc);
        &self.tree[idx]
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
            self.finalize(self.prev_proc_node);
            self.tree.pop();
            self.prev_proc_node = parent
        }
        self.all_closed = true;
    }
    /// 调用指定节点的 finalize 方法处理并关闭该节点，将当前节点指针移动至父节点
    pub(crate) fn finalize(&mut self, node_id: usize) {
        let parent = self.tree.get_parent(node_id);
        let node = &mut self.tree[node_id];
        node.processing = false;
        blocks::after(node_id, self);
        self.curr_proc_node = parent;
    }
}
