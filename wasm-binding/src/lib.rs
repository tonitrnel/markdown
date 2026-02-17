use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use markdown::{
    Document as MarkdownDocument, Location, MarkdownNode, Node, ParseError, Parser, ParserOptions,
    ParserPhaseSnapshot, Tree,
};

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

    #[wasm_bindgen(typescript_type = "ParserOptions")]
    pub type TParserOptions;
}

fn kind(node: &MarkdownNode) -> &'static str {
    match node {
        MarkdownNode::Document => "document",
        MarkdownNode::FrontMatter(..) => "frontmatter",
        MarkdownNode::Paragraph => "paragraph",
        MarkdownNode::SoftBreak => "soft-break",
        MarkdownNode::HardBreak => "hard-break",
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
            id: value.id.as_ref().map(|b| (**b).clone()),
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
    // Internal set for fast merge in deferred phase.
    // Exposed to JS as unsorted `string[]` via getter.
    tags: FxHashSet<String>,
    source: Option<String>,
    snapshot: Option<ParserPhaseSnapshot>,
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

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ParseMode {
    /// Parse full document in one call.
    #[default]
    Full,
    /// Parse frontmatter only in phase 1, then call `continue_parse` for phase 2.
    FrontmatterOnly,
}

/// JS-facing parser options (deserialized from `ParserOptions` TS type).
///
/// Notes:
/// - Uses serde defaults so all fields are optional from JS.
/// - `parse_mode` controls one-shot vs deferred two-phase parsing.
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
struct WasmParserOptions {
    /// `"full"` or `"frontmatter_only"`.
    parse_mode: ParseMode,
    /// Enable GitHub Flavored Markdown mode.
    github_flavored: bool,
    /// Enable extended GFM autolink.
    gfm_extended_autolink: bool,
    /// Enable Obsidian Flavored Markdown mode.
    obsidian_flavored: bool,
    /// Enable MDX component parsing behavior.
    mdx_component: bool,
    /// Enable CJK autocorrect.
    cjk_autocorrect: bool,
    /// Enable smart punctuation transforms.
    smart_punctuation: bool,
    /// Normalize Chinese punctuation.
    normalize_chinese_punctuation: bool,
    /// Enable CJK-friendly delimiter rules.
    cjk_friendly_delimiters: bool,
    /// Optional input size guard (bytes).
    max_input_bytes: Option<usize>,
    /// Optional node-count guard.
    max_nodes: Option<usize>,
    /// Preconfigured CJK nouns.
    cjk_nouns: Vec<String>,
    /// Read extra CJK nouns from frontmatter field.
    cjk_nouns_from_frontmatter: Option<String>,
}

/// Converts wasm options payload into core parser options and parse mode.
fn build_parser_options(input: Option<WasmParserOptions>) -> (ParserOptions, ParseMode) {
    let input = input.unwrap_or_default();
    let parse_mode = input.parse_mode.clone();
    let mut options = ParserOptions::default();
    if input.github_flavored {
        options = options.enabled_gfm();
    }
    if input.gfm_extended_autolink {
        options = options.enabled_gfm_autolink();
    }
    if input.obsidian_flavored {
        options = options.enabled_ofm();
    }
    if input.mdx_component {
        options = options.enabled_mdx_component();
    }
    if input.cjk_autocorrect {
        options = options.enabled_cjk_autocorrect();
    }
    if input.smart_punctuation {
        options = options.enabled_smart_punctuation();
    }
    if input.normalize_chinese_punctuation {
        options = options.enabled_normalize_chinese_punctuation();
    }
    if input.cjk_friendly_delimiters {
        options = options.enabled_cjk_friendly_delimiters();
    }
    if let Some(max_input_bytes) = input.max_input_bytes {
        options = options.with_max_input_bytes(max_input_bytes);
    }
    if let Some(max_nodes) = input.max_nodes {
        options = options.with_max_nodes(max_nodes);
    }
    if !input.cjk_nouns.is_empty() {
        options = options.with_cjk_nouns(input.cjk_nouns);
    }
    if let Some(field) = input.cjk_nouns_from_frontmatter {
        options = options.with_cjk_nouns_from_frontmatter(field)
    }
    (options, parse_mode)
}

impl From<MarkdownDocument> for Document {
    fn from(value: MarkdownDocument) -> Self {
        Self {
            ast: value.tree,
            tags: value.tags,
            source: None,
            snapshot: None,
        }
    }
}

impl Document {
    /// Build a deferred document after phase 1 parse (frontmatter only).
    fn from_frontmatter_phase(
        source: String,
        document: MarkdownDocument,
        snapshot: ParserPhaseSnapshot,
    ) -> Self {
        Self {
            ast: document.tree,
            tags: document.tags,
            source: Some(source),
            snapshot: Some(snapshot),
        }
    }
}

/// Maps Rust parse errors to JS error strings.
fn parse_error_to_js(err: ParseError) -> JsValue {
    let msg = match err {
        ParseError::InputTooLarge { limit, actual } => {
            format!("input exceeds max_input_bytes limit={limit}, actual={actual}")
        }
        ParseError::NodeLimitExceeded { limit, actual } => {
            format!("node count exceeds max_nodes limit={limit}, actual={actual}")
        }
        ParseError::SnapshotInputLengthMismatch { expected, actual } => {
            format!("snapshot source length mismatch expected={expected}, actual={actual}")
        }
    };
    JsValue::from_str(&msg)
}

#[wasm_bindgen]
impl Document {
    #[wasm_bindgen(getter)]
    pub fn tree(&self) -> TAstNode {
        let mut tree = AstNode::from(&self.ast[0]);
        transform_ast(&self.ast, 0, &mut tree.children);
        serde_wasm_bindgen::to_value(&tree)
            .unwrap_or(JsValue::NULL)
            .unchecked_into::<TAstNode>()
    }
    /// Returns document tags as an unsorted array.
    /// Ordering is not guaranteed and should not be relied upon.
    #[wasm_bindgen(getter)]
    pub fn tags(&self) -> Tags {
        let tags = self.tags.iter().cloned().collect::<Vec<_>>();
        serde_wasm_bindgen::to_value(&tags)
            .expect("Failed to serialize tags of document")
            .unchecked_into::<Tags>()
    }

    #[wasm_bindgen(getter)]
    pub fn total_nodes(&self) -> u32 {
        self.ast.len() as u32
    }

    #[wasm_bindgen]
    pub fn to_html(&self) -> String {
        self.ast.to_html()
    }

    #[wasm_bindgen(getter)]
    pub fn frontmatter(&self) -> Frontmatter {
        // Find frontmatter node in AST
        if let Some(first_child_idx) = self.ast.get_first_child(0) {
            if let MarkdownNode::FrontMatter(fm) = &self.ast[first_child_idx].body {
                return serde_wasm_bindgen::to_value(fm.as_ref())
                    .unwrap_or(JsValue::NULL)
                    .unchecked_into::<Frontmatter>();
            }
        }
        JsValue::NULL.unchecked_into::<Frontmatter>()
    }

    /// Completes phase 2 parse when `parse_mode = "frontmatter_only"`.
    /// No-op if document is already fully parsed.
    #[wasm_bindgen]
    pub fn continue_parse(&mut self) -> Result<(), JsValue> {
        let Some(snapshot) = self.snapshot.take() else {
            return Ok(());
        };
        let Some(source) = self.source.as_deref() else {
            return Err(JsValue::from_str("missing source for deferred parse"));
        };
        let deferred_document = MarkdownDocument {
            tree: std::mem::take(&mut self.ast),
            tags: std::mem::take(&mut self.tags),
        };
        let parser = Parser::from_phase_snapshot(source, snapshot, deferred_document)
            .map_err(parse_error_to_js)?;
        let document = parser.continue_parse_checked().map_err(parse_error_to_js)?;
        self.ast = document.tree;
        self.tags = document.tags;
        self.source = None;
        Ok(())
    }
}

#[wasm_bindgen]
pub fn parse(text: String) -> Document {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let parser = Parser::new_with_options(
        &text,
        ParserOptions::default()
            .enabled_gfm()
            .enabled_ofm()
            .enabled_cjk_autocorrect(),
    );
    let document = parser.parse();
    Document::from(document)
}

/// Parses markdown with user options.
///
/// `parse_mode` behavior:
/// - `full` (default): parse full document immediately.
/// - `frontmatter_only`: phase 1 only (Document + FrontMatter),
///   then call `Document::continue_parse()` to run phase 2.
#[wasm_bindgen]
pub fn parse_with_options(text: String, options: TParserOptions) -> Document {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let raw = options.unchecked_into::<JsValue>();
    let parsed_options = serde_wasm_bindgen::from_value::<WasmParserOptions>(raw).ok();
    let (options, parse_mode) = build_parser_options(parsed_options);
    let parser = Parser::new_with_options(&text, options);
    match parse_mode {
        ParseMode::Full => Document::from(parser.parse()),
        ParseMode::FrontmatterOnly => {
            let (document, snapshot) = parser
                .parse_frontmatter_phase()
                .expect("parse failed: input exceeds parser limits");
            Document::from_frontmatter_phase(text, document, snapshot)
        }
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    Parser::version().to_string()
}
