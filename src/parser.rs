use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing};
use crate::line::Line;
use crate::tokenizer::{Location, TokenIterator, Tokenizer};
use crate::tree::Tree;
use crate::{blocks, exts, inlines};

pub struct Node {
    pub body: MarkdownNode,
    pub start: Location,
    pub end: Location,
    pub processing: bool,
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]{:?}", self.start, self.end, self.body)
        // write!(f, "{:?}", self.body)
    }
}
impl Node {
    pub(crate) fn new(body: MarkdownNode, location: Location) -> Self {
        Self {
            body,
            start: location,
            end: location,
            processing: true,
        }
    }
}

pub struct Parser<'input> {
    pub(crate) tokens: TokenIterator<'input>,
    pub(crate) tree: Tree<Node>,
    /// 存储在解析 Block 时能接收 inlines 的 block 的 ID 和剩余未处理的 Line
    pub(crate) inlines: BTreeMap<usize, Vec<Line<'input>>>,
    pub(crate) doc: usize,
    /// 应等同于 tree.cur()
    pub(crate) curr_proc_node: usize,
    pub(crate) prev_proc_node: usize,
    pub(crate) last_matched_node: usize,
    pub(crate) last_location: Location,
    pub(crate) all_closed: bool,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        let mut tree = Tree::<Node>::new();
        println!("创建 Document 节点");
        let doc = tree.append(Node::new(MarkdownNode::Document, Location::default()));
        Self {
            tokens: Tokenizer::new(text).tokenize(),
            inlines: BTreeMap::new(),
            tree,
            doc,
            curr_proc_node: doc,
            prev_proc_node: doc,
            all_closed: true,
            last_matched_node: doc,
            last_location: Location::default(),
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
            let last_location = if line.is_blank() {
                self.last_location
            } else {
                line.last_token_end_location()
            };
            self.incorporate_line(line);
            self.last_location = last_location;
        }
        println!("开始确定块");
        while self.curr_proc_node != self.doc {
            self.finalize(self.curr_proc_node, self.last_location)
        }
        self.tree[self.doc].end = self.last_location;
        self.tree.reset();
    }
    fn parse_inlines(&mut self) {
        let keys = self.inlines.keys().copied().collect::<Vec<_>>();
        for idx in keys {
            let lines = self.inlines.remove(&idx);
            let node = &self.tree[idx].body;
            if lines.is_none()
                || !node.accepts_lines()
            {
                eprintln!("WARNING: Invalid node {node:?} exists inlines");
                continue;
            }
            let lines = lines.unwrap();
            println!("#{idx} {:?}", self.tree[idx].body);
            inlines::process(idx, self, Line::extends(lines));
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
            println!("开始匹配新的节点")
        };
        while !matched_leaf {
            if !line.is_indented()
                && !line
                    .get(line.indent)
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
            println!("当前行未结束，存储至之前的 Paragraph");
            self.append_inline(self.curr_proc_node, line);
        } else {
            self.close_unmatched_blocks();
            // 判断是否支持接收纯文本行，只有 Paragraph 、HTML Block、Code Block 支持，部分容器是支持存储空白行
            let cur_container = &self.tree[container].body;
            if cur_container.accepts_lines() && !line.is_end() {
                println!("存储当前行剩余内容");
                if let MarkdownNode::Html(html) = cur_container {
                    let location = line.start_location();
                    let sn = line.snapshot();
                    let is_end = html.is_end(&mut line);
                    line.resume(sn);
                    self.append_inline(container, line);
                    if is_end {
                        self.finalize(container, location)
                    }
                } else {
                    self.append_inline(container, line);
                }
            }
            // 判断行是否已全部消费或者该行是空白行
            else if !line.is_end() && !line.is_blank() {
                println!("当前行未结束，创建一个新的 Paragraph 存储");
                let idx = self.append_block(MarkdownNode::Paragraph, line.start_location());
                self.append_inline(idx, line);
            } else {
                println!("当前行没有更多内容了");
            }
        }
        // non indented code and common block start token
        // if line.indent < 4 && !line.starts_with_matches(Parser::is_special_token, 1) {}
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
        println!("创建节点 #{idx} {:?}", self.tree[idx].body);
        idx
    }
    pub fn append_free_node(&mut self, node: MarkdownNode, loc: Location) -> usize{
        let idx = self.tree.create_node(Node::new(node, loc));
        println!("创建游离节点 #{idx} {:?}", self.tree[idx].body);
        idx
    }
    pub fn append_block_to(
        &mut self,
        id: usize,
        node: MarkdownNode,
        location: (Location, Location),
    ) -> usize {
        let idx = self.tree.append_child(id, Node::new(node, location.0));
        self.tree[idx].end = location.1;
        println!("创建节点 #{idx} {:?}", self.tree[idx].body);
        idx
    }
    pub fn replace_block(&mut self, node: MarkdownNode, loc: Location) -> Option<usize> {
        self.last_location = loc;
        if let Some(idx) = self.tree.peek_up() {
            println!("替换节点 {:?} => {:?}", self.tree[idx].body, node);
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
        println!("创建节点 #{idx} {:?}", self.tree[idx].body);
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
            .filter(|id|self.tree[*id].processing)
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
            println!("创建节点 #{idx} {:?}", self.tree[idx].body);
            idx
        }
    }
    pub fn mark_as_processed(&mut self, idx: usize){
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
        blocks::after(node_id, self, location);
        let node = &mut self.tree[node_id];
        node.processing = false;
        println!("退出节点 #{node_id} {:?}", node.body);
        if Some(node_id) == self.tree.peek_up() {
            println!("树退出 #{node_id:?}");
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
