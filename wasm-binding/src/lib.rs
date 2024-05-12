use serde::Serialize;
use wasm_bindgen::prelude::*;

use markdown::{Location, MarkdownNode, Node, Parser, ParserOptions, Tree};

mod types;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Frontmatter")]
    pub type Frontmatter;
    // #[wasm_bindgen(typescript_type = "Location")]
    // pub type Location;
    #[wasm_bindgen(typescript_type = "Tags")]
    pub type Tags;

    #[wasm_bindgen(typescript_type = "AstNode")]
    pub type TAstNode;
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

#[derive(Serialize)]
pub struct AstNode {
    id: Option<String>,
    kind: String,
    content: MarkdownNode,
    start: Location,
    end: Location,
    children: Vec<AstNode>,
}

impl From<&Node> for AstNode {
    fn from(value: &Node) -> Self {
        Self {
            id: value.id.to_owned(),
            kind: kind(&value.body).to_string(),
            start: value.start,
            end: value.end,
            content: value.body.clone(),
            children: Vec::new(),
        }
    }
}

#[wasm_bindgen]
pub struct Document {
    ast: Tree<Node>,
    tags: Vec<String>,
}

fn transform_ast(ast: &Tree<Node>, index: usize, children: &mut Vec<AstNode>) {
    let mut next = ast.get_first_child(index);
    while let Some(next_idx) = next {
        let mut tree = AstNode::from(&ast[next_idx]);
        transform_ast(ast, next_idx, &mut tree.children);
        children.push(tree);
        next = ast.get_next(next_idx)
    }
}

#[wasm_bindgen]
impl Document {
    #[wasm_bindgen(getter)]
    pub fn tree(self) -> TAstNode {
        let mut tree = AstNode::from(&self.ast[0]);
        transform_ast(&self.ast, 0, &mut tree.children);
        serde_wasm_bindgen::to_value(&tree)
            .unwrap()
            .unchecked_into::<TAstNode>()
    }
    #[wasm_bindgen(getter)]
    pub fn tags(&self) -> Tags {
        serde_wasm_bindgen::to_value(&self.tags)
            .expect("Failed to serialize tags of document")
            .unchecked_into::<Tags>()
    }

    #[wasm_bindgen(getter)]
    pub fn total_nodes(&self) -> u32 {
        self.ast.len() as u32
    }
}

#[wasm_bindgen]
pub fn parse(text: String) -> Document {
    // console_error_panic_hook::set_once();
    let parser = Parser::new_with_options(
        &text,
        ParserOptions::default()
            .enabled_gfm()
            .enabled_ofm()
            .enabled_cjk_autocorrect(),
    );
    let (ast, tags) = parser.parse_with_tags();
    Document {
        ast,
        tags: tags.into_iter().collect(),
    }
}
