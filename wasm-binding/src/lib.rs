use js_sys::{Object, Reflect, Uint32Array, Uint8Array};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use markdown::ast::link::Link;
use markdown::{
    Document as MarkdownDocument, MarkdownNode, Node, ParseError, Parser, ParserOptions,
    ParserPhaseSnapshot,
};

mod json_tree;
mod types;

/// TypeScript type bindings for WASM exports
/// WASM 导出的 TypeScript 类型绑定
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Frontmatter")]
    pub type Frontmatter;
    #[wasm_bindgen(typescript_type = "FrontmatterOrNull")]
    pub type FrontmatterOrNull;
    // #[wasm_bindgen(typescript_type = "Location")]
    // pub type Location;
    #[wasm_bindgen(typescript_type = "Tags")]
    pub type Tags;

    #[wasm_bindgen(typescript_type = "ParserOptions")]
    pub type TParserOptions;

    #[wasm_bindgen(typescript_type = "SemanticTarget[]")]
    pub type TSemanticTargets;

    #[wasm_bindgen(typescript_type = "AstData")]
    pub type TAstData;

    #[wasm_bindgen(typescript_type = "HeadingMatch[]")]
    pub type THeadingMatches;

    #[wasm_bindgen(typescript_type = "LinkMatch[]")]
    pub type TLinkMatches;
}

/// Parsed markdown document with AST and metadata
/// 解析后的 Markdown 文档，包含 AST 和元数据
#[wasm_bindgen]
pub struct Document {
    inner: MarkdownDocument<'static>,
    snapshot: Option<ParserPhaseSnapshot>,
    ast_data: Option<NodeArrays>,
}

const NO_NODE: u32 = u32::MAX;
const NODE_KIND_NAMES: [&str; 32] = [
    "document",
    "frontmatter",
    "paragraph",
    "soft-break",
    "hard-break",
    "text",
    "embed",
    "heading",
    "strong",
    "emphasis",
    "list",
    "list-item",
    "image",
    "link",
    "tag",
    "emoji",
    "block-quote",
    "code",
    "table",
    "table-head",
    "table-head-col",
    "table-body",
    "table-row",
    "table-data-col",
    "strikethrough",
    "highlighting",
    "thematic-break",
    "footnote",
    "footnote-list",
    "math",
    "callout",
    "html",
];

struct NodeArrays {
    kind: Vec<u8>,
    first_child: Vec<u32>,
    next_sibling: Vec<u32>,
    start: Vec<u32>,
    end: Vec<u32>,
    payloads_json: String,
}

fn visit_nodes<F>(document: &MarkdownDocument, node_id: usize, visit: &mut F)
where
    F: FnMut(usize, &Node),
{
    visit(node_id, &document.tree[node_id]);
    let mut child = document.tree.get_first_child(node_id);
    while let Some(child_id) = child {
        visit_nodes(document, child_id, visit);
        child = document.tree.get_next(child_id);
    }
}

fn link_url(link: &Link, document: &MarkdownDocument) -> Option<String> {
    match link {
        Link::Default(link) => Some(document.text(&link.url).to_owned()),
        Link::Wikilink(link) => Some(link.path.clone()),
        Link::Footnote(..) | Link::FootnoteBackref(..) => None,
    }
}

impl NodeArrays {
    fn from_document(document: &MarkdownDocument) -> Self {
        let mut arrays = Self {
            kind: Vec::with_capacity(document.tree.len()),
            first_child: Vec::with_capacity(document.tree.len()),
            next_sibling: Vec::with_capacity(document.tree.len()),
            start: Vec::with_capacity(document.tree.len()),
            end: Vec::with_capacity(document.tree.len()),
            payloads_json: json_tree::node_payloads_to_json(document),
        };
        arrays.push_subtree(document, 0);
        arrays
    }

    fn push_subtree(&mut self, document: &MarkdownDocument, node_id: usize) -> u32 {
        let node = &document.tree[node_id];
        let packed_id = self.kind.len() as u32;
        self.kind.push(node_kind_code(&node.body));
        self.first_child.push(NO_NODE);
        self.next_sibling.push(NO_NODE);
        self.start.push(node.span.start);
        self.end.push(node.span.end);

        let mut previous_child = None;
        let mut child = document.tree.get_first_child(node_id);
        while let Some(child_id) = child {
            let packed_child = self.push_subtree(document, child_id);
            if let Some(previous) = previous_child {
                self.next_sibling[previous as usize] = packed_child;
            } else {
                self.first_child[packed_id as usize] = packed_child;
            }
            previous_child = Some(packed_child);
            child = document.tree.get_next(child_id);
        }
        packed_id
    }
}

fn node_kind_code(node: &MarkdownNode) -> u8 {
    match node {
        MarkdownNode::Document => 0,
        MarkdownNode::FrontMatter(..) => 1,
        MarkdownNode::Paragraph => 2,
        MarkdownNode::SoftBreak => 3,
        MarkdownNode::HardBreak => 4,
        MarkdownNode::Text(..) => 5,
        MarkdownNode::Embed(..) => 6,
        MarkdownNode::Heading(..) => 7,
        MarkdownNode::Strong => 8,
        MarkdownNode::Emphasis => 9,
        MarkdownNode::List(..) => 10,
        MarkdownNode::ListItem(..) => 11,
        MarkdownNode::Image(..) => 12,
        MarkdownNode::Link(..) => 13,
        MarkdownNode::Tag(..) => 14,
        MarkdownNode::Emoji(..) => 15,
        MarkdownNode::BlockQuote => 16,
        MarkdownNode::Code(..) => 17,
        MarkdownNode::Table(..) => 18,
        MarkdownNode::TableHead => 19,
        MarkdownNode::TableHeadCol => 20,
        MarkdownNode::TableBody => 21,
        MarkdownNode::TableRow => 22,
        MarkdownNode::TableDataCol => 23,
        MarkdownNode::Strikethrough => 24,
        MarkdownNode::Highlighting => 25,
        MarkdownNode::ThematicBreak => 26,
        MarkdownNode::Footnote(..) => 27,
        MarkdownNode::FootnoteList => 28,
        MarkdownNode::Math(..) => 29,
        MarkdownNode::Callout(..) => 30,
        MarkdownNode::Html(..) => 31,
    }
}

/// Parse mode configuration
/// 解析模式配置
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ParseMode {
    /// Parse full document in one call.
    /// 一次性解析完整文档
    #[default]
    Full,
    /// Parse frontmatter only in phase 1, then call `continue_parse` for phase 2.
    /// 第一阶段仅解析 frontmatter，然后调用 `continue_parse` 进行第二阶段
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

impl From<MarkdownDocument<'static>> for Document {
    fn from(value: MarkdownDocument<'static>) -> Self {
        Self {
            inner: value,
            snapshot: None,
            ast_data: None,
        }
    }
}

impl Document {
    /// Build a deferred document after phase 1 parse (frontmatter only).
    fn from_frontmatter_phase(
        document: MarkdownDocument<'static>,
        snapshot: ParserPhaseSnapshot,
    ) -> Self {
        Self {
            inner: document,
            snapshot: Some(snapshot),
            ast_data: None,
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
        // 选择性解析是 Rust-only API，WASM 路径不会产生该错误
        ParseError::InvalidSelectionNode { node_id } => {
            format!("invalid selection node id={node_id}")
        }
    };
    JsValue::from_str(&msg)
}

#[wasm_bindgen]
impl Document {
    /// Private transport used by the published JS wrapper to materialize an
    /// AST. Its layout is intentionally not a public compatibility contract.
    #[wasm_bindgen(js_name = astData)]
    pub fn ast_data(&mut self) -> TAstData {
        if self.ast_data.is_none() {
            self.ast_data = Some(NodeArrays::from_document(&self.inner));
        }
        let arrays = self.ast_data.as_ref().expect("AST data cache initialized");
        let object = Object::new();
        let kind_names = serde_wasm_bindgen::to_value(&NODE_KIND_NAMES)
            .expect("node kind names are serializable");
        let set = |name: &str, property_value: &JsValue| {
            Reflect::set(&object, &JsValue::from_str(name), property_value)
                .expect("node array property is writable");
        };
        set("abi_version", &JsValue::from_f64(1.0));
        set("root", &JsValue::from_f64(0.0));
        set("node_count", &JsValue::from_f64(arrays.kind.len() as f64));
        set("kind_names", &kind_names);
        set("payloads_json", &JsValue::from_str(&arrays.payloads_json));
        // SAFETY: the cache owns these immutable vectors for the document lifetime.
        // Callers must reacquire views after WASM memory growth.
        let kind = unsafe { Uint8Array::view(&arrays.kind) };
        let first_child = unsafe { Uint32Array::view(&arrays.first_child) };
        let next_sibling = unsafe { Uint32Array::view(&arrays.next_sibling) };
        let start = unsafe { Uint32Array::view(&arrays.start) };
        let end = unsafe { Uint32Array::view(&arrays.end) };
        set("kind", &kind);
        set("first_child", &first_child);
        set("next_sibling", &next_sibling);
        set("start", &start);
        set("end", &end);
        object.unchecked_into::<TAstData>()
    }

    /// Finds headings without materializing the complete JS AST.
    #[wasm_bindgen]
    pub fn query_headings(&self) -> THeadingMatches {
        let mut headings = Vec::new();
        visit_nodes(&self.inner, 0, &mut |node_id, node| {
            if let MarkdownNode::Heading(heading) = &node.body {
                headings.push(HeadingMatch {
                    node_id: node_id as u32,
                    level: *heading.level() as u8,
                    start_offset: node.span.start,
                    end_offset: node.span.end,
                });
            }
        });
        serde_wasm_bindgen::to_value(&headings)
            .expect("heading matches are serializable")
            .unchecked_into::<THeadingMatches>()
    }

    /// Finds links without materializing the complete JS AST. URL text is
    /// copied only for matching links.
    #[wasm_bindgen]
    pub fn query_links(&self) -> TLinkMatches {
        let mut links = Vec::new();
        visit_nodes(&self.inner, 0, &mut |node_id, node| {
            if let MarkdownNode::Link(link) = &node.body {
                links.push(LinkMatch {
                    node_id: node_id as u32,
                    url: link_url(link, &self.inner),
                    start_offset: node.span.start,
                    end_offset: node.span.end,
                });
            }
        });
        serde_wasm_bindgen::to_value(&links)
            .expect("link matches are serializable")
            .unchecked_into::<TLinkMatches>()
    }
    /// Returns document tags as an unsorted array.
    /// Ordering is not guaranteed and should not be relied upon.
    /// 返回文档标签的无序数组
    /// 不保证顺序，不应依赖顺序
    #[wasm_bindgen(getter)]
    pub fn tags(&self) -> Tags {
        let tags = self.inner.tags.iter().cloned().collect::<Vec<_>>();
        serde_wasm_bindgen::to_value(&tags)
            .expect("Failed to serialize tags of document")
            .unchecked_into::<Tags>()
    }

    /// Get total number of nodes in the AST
    /// 获取 AST 中的节点总数
    #[wasm_bindgen(getter)]
    pub fn total_nodes(&self) -> u32 {
        self.inner.tree.len() as u32
    }

    /// Convert the document to HTML
    /// 将文档转换为 HTML
    #[wasm_bindgen]
    pub fn to_html(&self) -> String {
        self.inner.to_html()
    }

    /// Get the frontmatter metadata if present
    /// 获取 frontmatter 元数据（如果存在）
    #[wasm_bindgen(getter)]
    pub fn frontmatter(&self) -> FrontmatterOrNull {
        // Find frontmatter node in AST
        if let Some(first_child_idx) = self.inner.tree.get_first_child(0) {
            if let MarkdownNode::FrontMatter(fm) = &self.inner.tree[first_child_idx].body {
                return serde_wasm_bindgen::to_value(fm.as_ref())
                    .unwrap_or(JsValue::NULL)
                    .unchecked_into::<FrontmatterOrNull>();
            }
        }
        JsValue::NULL.unchecked_into::<FrontmatterOrNull>()
    }

    /// Completes phase 2 parse when `parse_mode = "frontmatter_only"`.
    /// No-op if document is already fully parsed.
    /// 当 `parse_mode = "frontmatter_only"` 时完成第二阶段解析
    /// 如果文档已完全解析则为空操作
    #[wasm_bindgen]
    pub fn continue_parse(&mut self) -> Result<(), JsValue> {
        let Some(snapshot) = self.snapshot.take() else {
            return Ok(());
        };
        // 源码保存在 inner Document（owned）内，经核心的 owned 往返完成第二阶段
        let document = std::mem::take(&mut self.inner);
        let document = Parser::continue_parse_from_snapshot_string(document, snapshot)
            .map_err(parse_error_to_js)?;
        self.inner = document;
        self.ast_data = None;
        Ok(())
    }
}

/// Parse markdown with default options (GFM + OFM + CJK autocorrect enabled)
/// 使用默认选项解析 Markdown（启用 GFM + OFM + CJK 自动纠正）
///
/// # Arguments
/// * `text` - The markdown text to parse / 要解析的 Markdown 文本
///
/// # Returns
/// A `Document` containing the parsed AST and metadata / 包含解析后的 AST 和元数据的 `Document`
#[wasm_bindgen]
pub fn parse(text: String) -> Document {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let document = Parser::parse_string(
        text,
        ParserOptions::default()
            .enabled_gfm()
            .enabled_ofm()
            .enabled_cjk_autocorrect(),
    );
    Document::from(document)
}

/// Parses markdown with user-specified options.
/// 使用用户指定的选项解析 Markdown
///
/// # Arguments
/// * `text` - The markdown text to parse / 要解析的 Markdown 文本
/// * `options` - Parser configuration options / 解析器配置选项
///
/// # Parse Mode Behavior
/// - `full` (default): parse full document immediately / 立即解析完整文档
/// - `frontmatter_only`: phase 1 only (Document + FrontMatter),
///   then call `Document::continue_parse()` to run phase 2
///   / 仅第一阶段（Document + FrontMatter），然后调用 `Document::continue_parse()` 运行第二阶段
///
/// # Returns
/// A `Document` containing the parsed AST and metadata / 包含解析后的 AST 和元数据的 `Document`
#[wasm_bindgen]
pub fn parse_with_options(text: String, options: TParserOptions) -> Document {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let raw = options.unchecked_into::<JsValue>();
    let parsed_options = serde_wasm_bindgen::from_value::<WasmParserOptions>(raw).ok();
    let (options, parse_mode) = build_parser_options(parsed_options);
    match parse_mode {
        ParseMode::Full => Document::from(Parser::parse_string(text, options)),
        ParseMode::FrontmatterOnly => {
            let (document, snapshot) = Parser::parse_frontmatter_phase_string(text, options)
                .expect("parse failed: input exceeds parser limits");
            Document::from_frontmatter_phase(document, snapshot)
        }
    }
}

/// Semantic target info returned by `query_semantic_targets`.
/// `query_semantic_targets` 返回的语义目标信息。
#[derive(Serialize)]
struct SemanticTargetInfo {
    node_id: u32,
    /// Heading level 1-6 when the target is a heading / 目标为标题时的层级
    heading_level: Option<u8>,
    /// OFM BlockId when present / 存在时的 OFM BlockId
    block_id: Option<String>,
    /// Obsidian-style reference text (formatting stripped by the real
    /// inline engine) / Obsidian 式引用文本（真实 Inline 引擎剥离格式）
    ref_text: String,
    /// Source byte offsets / 源码字节偏移
    start_offset: u32,
    end_offset: u32,
}

#[derive(Serialize)]
struct HeadingMatch {
    node_id: u32,
    level: u8,
    start_offset: u32,
    end_offset: u32,
}

#[derive(Serialize)]
struct LinkMatch {
    node_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    start_offset: u32,
    end_offset: u32,
}

fn query_targets_impl(text: &str, options: ParserOptions) -> Vec<SemanticTargetInfo> {
    use markdown::selective::{InlineSelection, VisitControl};
    let mut out = Vec::new();
    let mut phase = Parser::new_with_options(text, options)
        .parse_blocks_with(|_| true, |_| VisitControl::Continue)
        .prepare_semantic_targets();
    let mut selection = InlineSelection::default();
    phase.visit_semantic_targets(
        |_| true,
        &mut selection,
        |target, _| {
            let ref_text = target.ref_text();
            let node = target.node();
            out.push(SemanticTargetInfo {
                node_id: target.node_id() as u32,
                heading_level: target.heading().map(|h| *h.level() as u8),
                block_id: target.block_id().map(str::to_owned),
                ref_text,
                start_offset: node.span.start,
                end_offset: node.span.end,
            });
            VisitControl::Continue
        },
    );
    out
}

/// Fast target lookup for Obsidian-style addressing: block phase +
/// semantic preparation only — no full-tree boundary serialization.
/// `node_id` values are stable across calls for byte-identical text
/// with identical options.
/// Obsidian 式寻址的快速目标查询：仅块相位 + 语义准备，不做全树跨边界
/// 序列化。对逐字节相同的文本与相同选项，`node_id` 跨调用稳定。
#[wasm_bindgen]
pub fn query_semantic_targets(text: String) -> TSemanticTargets {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let targets = query_targets_impl(
        &text,
        ParserOptions::default()
            .enabled_gfm()
            .enabled_ofm()
            .enabled_cjk_autocorrect(),
    );
    serde_wasm_bindgen::to_value(&targets)
        .unwrap_or(JsValue::NULL)
        .unchecked_into::<TSemanticTargets>()
}

/// `query_semantic_targets` with user-specified options (`parse_mode` ignored).
/// 带用户选项的 `query_semantic_targets`（`parse_mode` 忽略）。
#[wasm_bindgen]
pub fn query_semantic_targets_with_options(
    text: String,
    options: TParserOptions,
) -> TSemanticTargets {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let raw = options.unchecked_into::<JsValue>();
    let parsed_options = serde_wasm_bindgen::from_value::<WasmParserOptions>(raw).ok();
    let (options, _) = build_parser_options(parsed_options);
    let targets = query_targets_impl(&text, options);
    serde_wasm_bindgen::to_value(&targets)
        .unwrap_or(JsValue::NULL)
        .unchecked_into::<TSemanticTargets>()
}

/// Selective parse: materialize inlines only for `node_ids` (descendants
/// expand automatically; referenced footnote definitions follow).
/// Unselected nodes keep block structure without inline children, so
/// `.tree` shrinks with the selection. Invalid ids throw.
/// 选择性解析：仅物化 `node_ids`（后代自动展开；被引脚注定义随附）。
/// 未选节点保留 Block 结构、无 inline 子树，`.tree` 随选择缩小。非法 id 抛错。
#[wasm_bindgen]
pub fn parse_selected(text: String, node_ids: Vec<u32>) -> Result<Document, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let ids: Vec<usize> = node_ids.into_iter().map(|id| id as usize).collect();
    let document = Parser::parse_selected_string_checked(
        text,
        ParserOptions::default()
            .enabled_gfm()
            .enabled_ofm()
            .enabled_cjk_autocorrect(),
        &ids,
    )
    .map_err(parse_error_to_js)?;
    Ok(Document::from(document))
}

/// Get the parser version string
/// 获取解析器版本字符串
///
/// # Returns
/// Version string in semver format / semver 格式的版本字符串
#[wasm_bindgen]
pub fn version() -> String {
    Parser::version().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Private AST payloads are valid JSON and align with packed topology.
    #[test]
    fn ast_payloads_are_valid_and_complete() {
        let src = "# T ^h1\n\npara [a](https://e.com/x \"t\") ![i](img) `c`\n\n- item ^b1\n";
        let doc = Parser::parse_string(
            src.to_string(),
            ParserOptions::default().enabled_gfm().enabled_ofm(),
        );
        let wrapped = Document {
            inner: doc,
            snapshot: None,
            ast_data: None,
        };
        let json = json_tree::node_payloads_to_json(&wrapped.inner);
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        let payloads = value.as_array().expect("payload array");
        assert_eq!(payloads.len(), wrapped.inner.tree.len());
        let h = &payloads[1];
        assert_eq!(h["id"], "h1");
        assert!(json.contains("\"content\":{\"variant\":\"default\",\"url\":\"https://e.com/x\""));
        assert!(!json.contains("\"id\":\"\""));
    }

    #[test]
    fn node_arrays_pack_reachable_tree_topology() {
        let doc = Parser::parse_string(
            "# Title\n\nA [link](https://example.com).\n".to_string(),
            ParserOptions::default().enabled_gfm(),
        );
        let arrays = NodeArrays::from_document(&doc);

        assert_eq!(arrays.kind.len(), doc.tree.len());
        assert_eq!(arrays.kind[0], 0, "root is the document kind");
        assert_eq!(
            arrays.kind[arrays.first_child[0] as usize], 7,
            "first child is heading"
        );
        assert_eq!(arrays.first_child.len(), arrays.kind.len());
        assert_eq!(arrays.next_sibling.len(), arrays.kind.len());
        assert_eq!(arrays.start.len(), arrays.kind.len());
        assert_eq!(arrays.end.len(), arrays.kind.len());
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&arrays.payloads_json)
                .expect("valid payload JSON")
                .as_array()
                .expect("payload array")
                .len(),
            arrays.kind.len()
        );
        assert!(
            arrays.kind.iter().any(|&kind| kind == 13),
            "link is included"
        );
        assert!(arrays.next_sibling.iter().any(|&next| next != NO_NODE));
    }

    /// Test two-phase parsing produces same result as one-phase parsing
    /// 测试两阶段解析与一阶段解析产生相同结果
    #[test]
    fn test_two_phase_equals_one_phase() {
        let markdown = r#"---
title: Test
tags: [rust, markdown]
---

# Hello World

This is **bold** and *italic*.

#tag1 #tag2
"#;

        // One-phase: parse_with_options with default (full mode)
        let options_full = WasmParserOptions {
            parse_mode: ParseMode::Full,
            ..Default::default()
        };
        let (opts_full, _) = build_parser_options(Some(options_full));
        let parser_full = Parser::new_with_options(markdown, opts_full);
        let doc_full = Document::from(parser_full.parse());
        println!("full ast:\n{:?}", doc_full.inner.tree);
        assert_eq!(doc_full.inner.tree.len(), 14);

        // Two-phase: parse_with_options with frontmatter_only -> continue_parse
        let options_two = WasmParserOptions {
            parse_mode: ParseMode::FrontmatterOnly,
            ..Default::default()
        };
        let (opts_two, _) = build_parser_options(Some(options_two));
        let parser_two = Parser::new_with_options(markdown, opts_two);
        let (doc_phase1, snapshot) = parser_two
            .parse_frontmatter_phase()
            .expect("phase 1 failed");

        println!("ast_phase1:\n{:?}", doc_phase1);

        let mut doc_phase2 = Document::from_frontmatter_phase(doc_phase1, snapshot);
        doc_phase2.continue_parse().expect("phase 2 failed");

        println!("ast_phase2:\n{:?}", doc_phase2.inner.tree);
        // Compare: same nodes, same tags, same HTML
        assert_eq!(
            doc_full.inner.tree.len(),
            doc_phase2.inner.tree.len(),
            "node count mismatch"
        );
        assert_eq!(doc_full.inner.tags, doc_phase2.inner.tags, "tags mismatch");
        assert_eq!(
            doc_full.inner.to_html(),
            doc_phase2.inner.to_html(),
            "HTML mismatch"
        );
    }
}
