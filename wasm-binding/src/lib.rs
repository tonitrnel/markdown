mod types;

use markdown::{MarkdownNode, Node, Parser, ParserOptions, Tree};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Frontmatter")]
    pub type Frontmatter;
    #[wasm_bindgen(typescript_type = "Location")]
    pub type Location;
    #[wasm_bindgen(typescript_type = "Tags")]
    pub type Tags;
}

fn kind(node: &MarkdownNode) -> &'static str {
    match node {
        MarkdownNode::Document => "document",
        MarkdownNode::FrontMatter => "frontmatter",
        MarkdownNode::Paragraph => "paragraph",
        MarkdownNode::SoftBreak => "soft-break",
        MarkdownNode::HardBreak => "hardbreak",
        MarkdownNode::Text(..) => "text",
        MarkdownNode::Embed(..) => "embed",
        MarkdownNode::Heading(..) => "heading",
        MarkdownNode::Strong => "strong",
        MarkdownNode::Emphasis => "emphasis",
        MarkdownNode::List(..) => "list",
        MarkdownNode::ListItem(..) => "list-item",
        MarkdownNode::Image(..) => "image",
        MarkdownNode::Link(..) => "link",
        MarkdownNode::Tag(..) => "tag",
        MarkdownNode::Emoji(..) => "emoji",
        MarkdownNode::BlockQuote => "block-quote",
        MarkdownNode::Code(..) => "code",
        MarkdownNode::Table(..) => "table",
        MarkdownNode::TableHead => "table-head",
        MarkdownNode::TableHeadCol => "table-head-col",
        MarkdownNode::TableBody => "table-body",
        MarkdownNode::TableRow => "table-row",
        MarkdownNode::TableDataCol => "table-data-col",
        MarkdownNode::Strikethrough => "strikethrough",
        MarkdownNode::Highlighting => "highlighting",
        MarkdownNode::ThematicBreak => "thematic-break",
        MarkdownNode::Footnote(..) => "footnote",
        MarkdownNode::FootnoteList => "footnote-list",
        MarkdownNode::Math(..) => "math",
        MarkdownNode::Callout(..) => "callout",
        MarkdownNode::Html(..) => "html",
    }
}

#[wasm_bindgen(skip_typescript)]
pub struct AstNode {
    tree_idx: usize,
    inner: Rc<Tree<Node>>,
    kind: &'static str,
}

#[wasm_bindgen]
impl AstNode {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Option<String> {
        self.inner[self.tree_idx].id.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn content(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner[self.tree_idx].body).unwrap()
    }
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.kind.to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Location {
        serde_wasm_bindgen::to_value(&self.inner[self.tree_idx].start)
            .unwrap()
            .unchecked_into::<Location>()
    }
    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Location {
        serde_wasm_bindgen::to_value(&self.inner[self.tree_idx].end)
            .unwrap()
            .unchecked_into::<Location>()
    }
    #[wasm_bindgen(getter)]
    pub fn next(&self) -> Option<AstNode> {
        self.inner.get_next(self.tree_idx).map(|next| {
            let node = &self.inner[next];
            AstNode {
                tree_idx: next,
                kind: kind(&node.body),
                inner: self.inner.clone(),
            }
        })
    }
    #[wasm_bindgen(getter)]
    pub fn child(&self) -> Option<AstNode> {
        self.inner.get_first_child(self.tree_idx).map(|child| {
            let node = &self.inner[child];
            AstNode {
                tree_idx: child,
                kind: kind(&node.body),
                inner: self.inner.clone(),
            }
        })
    }
}
#[wasm_bindgen]
pub struct Document {
    ast: Rc<Tree<Node>>,
    tags: Vec<String>,
}

#[wasm_bindgen]
impl Document {
    #[wasm_bindgen(getter)]
    pub fn document(&self) -> AstNode {
        let node = &self.ast[0];
        AstNode {
            tree_idx: 0,
            kind: kind(&node.body),
            inner: self.ast.clone(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn tags(&self) -> Tags {
        serde_wasm_bindgen::to_value(&self.tags)
            .unwrap()
            .unchecked_into::<Tags>()
    }
}

#[wasm_bindgen]
pub struct Markdown {
    inner: Parser<'static>,
    leaked_ptr: &'static str,
}

#[wasm_bindgen]
impl Markdown {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String) -> Markdown {
        let text = Box::leak(text.into_boxed_str());
        let inner = Parser::<'static>::new_with_options(
            text,
            ParserOptions::new()
                .enabled_gfm()
                .enabled_ofm()
                .enabled_cjk_autocorrect(),
        );
        Self {
            inner,
            leaked_ptr: text,
        }
    }
    pub fn parse(mut self) -> Document {
        let (ast, tags) = self.inner.parse_with_tags();
        unsafe {
            // 销毁 text
            let _ = Box::from_raw(&mut self.leaked_ptr);
        }
        Document {
            ast: Rc::new(ast),
            tags: tags.into_iter().collect(),
        }
    }
}
