use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, Line};
use crate::tokenizer::{Location, Token, TokenIterator, Tokenizer};
use crate::tree::Tree;
use std::collections::HashMap;

pub struct Node {
    pub body: MarkdownNode,
    pub column: u64,
    pub line: u64,
    pub open: bool,
}
impl Node {
    fn new(body: MarkdownNode, location: Location) -> Self {
        Self {
            body,
            column: location.column,
            line: location.line,
            open: true,
        }
    }
}

pub struct Parser<'input> {
    tokens: TokenIterator<'input>,
    tree: Tree<Node>,
    inlines: HashMap<usize, Vec<Line<'input>>>,
    doc: usize,
    prev_node: usize,
    node: usize,
    all_closed: bool,
    last_matched_node: usize,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        let mut tree = Tree::<Node>::new();
        let doc = tree.append(Node::new(MarkdownNode::Document, Location::default()));
        Self {
            tokens: Tokenizer::new(text).tokenize(),
            inlines: HashMap::new(),
            tree,
            doc,
            node: doc,
            prev_node: doc,
            all_closed: true,
            last_matched_node: doc,
        }
    }
    pub fn parse(mut self) -> Tree<Node> {
        self.tree.push();
        self.parse_front_matter();
        self.parse_blocks();
        self.parse_inlines();
        self.tree.pop();
        self.tree
    }
    fn parse_front_matter(&mut self) {
        let mut guard = TokenIteratorGuard::new(&mut self.tokens);
        let mut line = match guard.line() {
            Some(line) => line,
            None => return,
        };
        let marker = match line[0].token {
            Token::Hyphen => Token::Hyphen,
            Token::Plus => Token::Plus,
            _ => return,
        };
        if !line.starts_with(&marker, 3) || !line.skip(3).ensure_only_spaces_to_end() {
            return;
        }
        let mut lines = Vec::<Line>::new();
        while let Some(mut line) = guard.line() {
            if line.starts_with(&marker, 3) && line.skip(3).ensure_only_spaces_to_end() {
                let location = lines[0][0].location;
                let text = lines.iter().map(|it| it.to_string()).collect::<String>();
                self.tree.create_scope(
                    Node::new(MarkdownNode::FrontMatter, Location::default()),
                    |tree| {
                        tree.append(Node::new(text.into(), location));
                    },
                );
                guard.commit();
                return;
            }
            line.reset();
            lines.push(line)
        }
    }
    fn parse_blocks(&mut self) {
        while let Some(line) = Line::extract(&mut self.tokens) {
            self.incorporate_line(line)
        }
    }
    fn parse_inlines(&mut self) {
        todo!()
    }
    fn incorporate_line(&mut self, line: Line) {
        let all_closed = self.prev_node == self.doc;
        let last_matched_node = self.doc;
        // non indented code and common block start token
        // if line.indent < 4 && !line.starts_with_matches(Parser::is_special_token, 1) {}
    }
    fn is_special_token(token: &Token) -> bool {
        matches!(
            token,
            // ATX Heading
            Token::Crosshatch
                // Fenced code
                | Token::Backtick
                | Token::Tilde
                // Thematic breaks
                | Token::Asterisk
                | Token::Underscore
                | Token::Plus
                | Token::Eq
                // HTML Tag
                | Token::Lt
                | Token::Gt
                // Ordered Task or Task
                | Token::Ordered(..)
                | Token::Hyphen
        )
    }
    pub fn add_block(&mut self, node: MarkdownNode, loc: Location) -> usize {
        while let Some(parent) = self.tree.peek_up() {
            if self.tree[parent].body.can_contain(&node) {
                break;
            }
            self.finalize(parent)
        }
        let idx = self.tree.append(Node::new(node, loc));
        self.tree.push();
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
    pub fn add_inline(&mut self, block_idx: usize, line: Line<'input>) {
        self.inlines.entry(block_idx).or_default().push(line)
    }
    pub fn current_container(&self) -> &Node {
        let idx = self.tree.peek_up().unwrap_or(self.doc);
        &self.tree[idx]
    }
    pub fn interrupt_block(&mut self) {
        if !self.all_closed {
            loop {
                if self.prev_node == self.last_matched_node {
                    break;
                }
                let parent = self.tree.get_parent(self.prev_node);
                self.finalize(self.prev_node);
                self.prev_node = parent
            }
            self.all_closed = true;
        }
    }
    fn finalize(&mut self, node_id: usize) {
        let node = &mut self.tree[node_id];
        node.open = false;
        // todo: call node finalize
        self.tree.pop();
    }
}

struct TokenIteratorGuard<'a, 'input> {
    committed: bool,
    pub original: &'a mut TokenIterator<'input>,
    snapshot: TokenIterator<'input>,
}

impl<'a, 'input> TokenIteratorGuard<'a, 'input> {
    fn new(original: &'a mut TokenIterator<'input>) -> Self {
        TokenIteratorGuard {
            committed: false,
            snapshot: original.clone(),
            original,
        }
    }
    fn commit(&mut self) {
        self.committed = true;
    }
    fn line(&mut self) -> Option<Line<'input>> {
        Line::extract(self.original)
    }
}

impl<'a, 'input> Drop for TokenIteratorGuard<'a, 'input> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        *self.original = self.snapshot.clone();
    }
}
