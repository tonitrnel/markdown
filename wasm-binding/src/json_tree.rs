//! W1（ticket 29）：全树 JSON 直写序列化器。
//!
//! 绕开 `.tree` 路径的 AstNode 克隆图与逐节点反射：直接遍历内部树写入
//! 单个 JSON 缓冲。与 `.tree` 的形状差异（v2 紧凑约定，消费端按此解码）：
//! - `start`/`end` 为**源码字节偏移**数字（与 `SemanticTarget` 一致），非 `{line,column}`；
//! - 无值字段（`id`、结构性节点的 `content`）**省略**而非 `null`；
//! - 叶节点省略空 `children`；
//! - `content` 载荷形状与 `.tree` 相同（untagged serde；Text/Link/Image 的
//!   `TextRef` 已物化为字符串）。
use markdown::ast::link::Link;
use markdown::ast::text::TextRef;
use markdown::Document;
use markdown::MarkdownNode;

pub(crate) fn tree_to_json(doc: &Document) -> String {
    let mut out: Vec<u8> = Vec::with_capacity(doc.source().len() * 4 + 1024);
    write_node(doc, 0, &mut out);
    // SAFETY: 推入内容均为 serde_json 输出或 ASCII 结构字符，恒为合法 UTF-8
    unsafe { String::from_utf8_unchecked(out) }
}

fn write_node(doc: &Document, id: usize, out: &mut Vec<u8>) {
    let node = &doc.tree[id];
    out.extend_from_slice(b"{\"kind\":\"");
    out.extend_from_slice(super::kind(&node.body).as_bytes());
    out.push(b'"');
    if let Some(block_id) = &node.id {
        out.extend_from_slice(b",\"id\":");
        let _ = serde_json::to_writer(&mut *out, block_id.as_str());
    }
    write_content(doc, &node.body, out);
    let mut ibuf = itoa::Buffer::new();
    out.extend_from_slice(b",\"start\":");
    out.extend_from_slice(ibuf.format(node.span.start).as_bytes());
    out.extend_from_slice(b",\"end\":");
    out.extend_from_slice(ibuf.format(node.span.end).as_bytes());
    if let Some(first) = doc.tree.get_first_child(id) {
        out.extend_from_slice(b",\"children\":[");
        let mut child = Some(first);
        while let Some(c) = child {
            write_node(doc, c, out);
            child = doc.tree.get_next(c);
            if child.is_some() {
                out.push(b',');
            }
        }
        out.push(b']');
    }
    out.push(b'}');
}

fn materialize(text: &mut TextRef, source: &str) {
    if matches!(text, TextRef::Source(_)) {
        *text = TextRef::Owned(text.resolve(source).to_string());
    }
}

fn write_content(doc: &Document, body: &MarkdownNode, out: &mut Vec<u8>) {
    match body {
        // TextRef 载荷：物化后写出（Text 直接借 resolved 字符串，零克隆）
        MarkdownNode::Text(t) => {
            out.extend_from_slice(b",\"content\":");
            let _ = serde_json::to_writer(&mut *out, doc.text(t));
        }
        MarkdownNode::Link(link) => {
            let mut link = link.clone();
            if let Link::Default(d) = link.as_mut() {
                materialize(&mut d.url, doc.source());
                if let Some(title) = &mut d.title {
                    materialize(title, doc.source());
                }
            }
            out.extend_from_slice(b",\"content\":");
            let _ = serde_json::to_writer(&mut *out, &*link);
        }
        MarkdownNode::Image(image) => {
            let mut image = image.clone();
            materialize(&mut image.url, doc.source());
            if let Some(title) = &mut image.title {
                materialize(title, doc.source());
            }
            out.extend_from_slice(b",\"content\":");
            let _ = serde_json::to_writer(&mut *out, &*image);
        }
        // 无载荷的结构性节点：省略 content
        MarkdownNode::Document
        | MarkdownNode::Paragraph
        | MarkdownNode::SoftBreak
        | MarkdownNode::HardBreak
        | MarkdownNode::Strong
        | MarkdownNode::Emphasis
        | MarkdownNode::BlockQuote
        | MarkdownNode::ThematicBreak
        | MarkdownNode::Strikethrough
        | MarkdownNode::Highlighting
        | MarkdownNode::FootnoteList
        | MarkdownNode::TableHead
        | MarkdownNode::TableBody
        | MarkdownNode::TableRow
        | MarkdownNode::TableHeadCol
        | MarkdownNode::TableDataCol => {}
        // 其余载荷：untagged serde 与 `.tree` 同形
        other => {
            out.extend_from_slice(b",\"content\":");
            let _ = serde_json::to_writer(&mut *out, other);
        }
    }
}
