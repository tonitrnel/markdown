use markdown::{Document, MarkdownNode};

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticDigest {
    pub nodes: Vec<SemanticNode>,
    pub tags: Vec<String>,
    pub html: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticNode {
    pub kind: &'static str,
    pub parent_preorder: Option<usize>,
    pub start: (u64, u64),
    pub end: (u64, u64),
    pub block_id: Option<String>,
    pub payload: String,
}

pub fn semantic_digest(document: &Document) -> SemanticDigest {
    let mut nodes = Vec::new();
    visit(document, 0, None, &mut nodes);

    let mut tags: Vec<_> = document.tags.iter().cloned().collect();
    tags.sort();

    SemanticDigest {
        nodes,
        tags,
        html: document.to_html(),
    }
}

fn visit(
    document: &Document,
    node_id: usize,
    parent_preorder: Option<usize>,
    output: &mut Vec<SemanticNode>,
) {
    let source = document.source();
    let preorder = output.len();
    let node = &document.tree[node_id];
    let start = document.location_at(node.span.start as usize);
    let end = document.location_at(node.span.end as usize);
    output.push(SemanticNode {
        kind: node_kind(&node.body),
        parent_preorder,
        start: (start.line, start.column),
        end: (end.line, end.column),
        block_id: node.id.as_ref().map(|id| id.as_str().to_owned()),
        payload: canonical_payload(&node.body, source),
    });

    let mut child = document.tree.get_first_child(node_id);
    while let Some(child_id) = child {
        visit(document, child_id, Some(preorder), output);
        child = document.tree.get_next(child_id);
    }
}

fn node_kind(node: &MarkdownNode) -> &'static str {
    match node {
        MarkdownNode::Document => "Document",
        MarkdownNode::FrontMatter(..) => "FrontMatter",
        MarkdownNode::Paragraph => "Paragraph",
        MarkdownNode::SoftBreak => "SoftBreak",
        MarkdownNode::HardBreak => "HardBreak",
        MarkdownNode::Text(..) => "Text",
        MarkdownNode::Embed(..) => "Embed",
        MarkdownNode::Heading(..) => "Heading",
        MarkdownNode::Strong => "Strong",
        MarkdownNode::Emphasis => "Emphasis",
        MarkdownNode::List(..) => "List",
        MarkdownNode::ListItem(..) => "ListItem",
        MarkdownNode::Image(..) => "Image",
        MarkdownNode::Link(..) => "Link",
        MarkdownNode::Tag(..) => "Tag",
        MarkdownNode::Emoji(..) => "Emoji",
        MarkdownNode::BlockQuote => "BlockQuote",
        MarkdownNode::Code(..) => "Code",
        MarkdownNode::Table(..) => "Table",
        MarkdownNode::TableHead => "TableHead",
        MarkdownNode::TableHeadCol => "TableHeadCol",
        MarkdownNode::TableBody => "TableBody",
        MarkdownNode::TableRow => "TableRow",
        MarkdownNode::TableDataCol => "TableDataCol",
        MarkdownNode::Strikethrough => "Strikethrough",
        MarkdownNode::Highlighting => "Highlighting",
        MarkdownNode::ThematicBreak => "ThematicBreak",
        MarkdownNode::Footnote(..) => "Footnote",
        MarkdownNode::FootnoteList => "FootnoteList",
        MarkdownNode::Math(..) => "Math",
        MarkdownNode::Callout(..) => "Callout",
        MarkdownNode::Html(..) => "Html",
    }
}

fn canonical_payload(node: &MarkdownNode, source: &str) -> String {
    match node {
        MarkdownNode::FrontMatter(values) => {
            let mut entries: Vec<_> = values
                .iter()
                .map(|(key, value)| format!("{key}={value:?}"))
                .collect();
            entries.sort();
            entries.join(",")
        }
        // Text 按显示文本呈现：Source 与 Owned 表示、区间平移不影响摘要
        MarkdownNode::Text(text) => format!("Text({:?})", text.resolve(source)),
        _ => format!("{node:?}"),
    }
}
