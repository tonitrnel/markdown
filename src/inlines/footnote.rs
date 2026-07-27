use crate::ast::{MarkdownNode, footnote, link};
use crate::inlines::ProcessCtx;
use crate::parser::Parser;
use crate::utils;

pub(super) fn process_inline(ctx: &mut ProcessCtx) -> bool {
    if ctx.line.get(1) != Some(b'[') {
        return false;
    }

    let span = match ctx.line.current_span() {
        Some(span) => span,
        None => return false,
    };
    let start = span.cursor();
    let content_start = start + 2;
    let mut cursor = content_start;
    let mut depth = 1usize;
    let source = span.source_slice();

    while cursor < span.end() {
        match source[cursor] {
            b'\\' => {
                cursor += 1;
                if cursor < span.end() {
                    cursor += 1;
                }
            }
            b'[' => {
                depth += 1;
                cursor += 1;
            }
            b']' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                cursor += 1;
            }
            _ => cursor += 1,
        }
    }

    if depth != 0 || content_start == cursor {
        return false;
    }

    let start_location = ctx.line.cursor_or_end() as u32;
    let content_start_location = (content_start) as u32;
    let content_end_location = (cursor) as u32;
    let end_location = (cursor + 1) as u32;
    let content_span = span.slice_from_abs(content_start, cursor);

    let mut label_index = ctx.parser.footnote_refs.len() + 1;
    let label = loop {
        let candidate = format!("inline-footnote-{label_index}");
        if !ctx.parser.footnotes.contains_key(&candidate)
            && !ctx.parser.footnote_refs.contains_key(&candidate)
        {
            break candidate;
        }
        label_index += 1;
    };
    let encoded_label = utils::percent_encode::encode(&label, true);

    let footnote_idx = ctx.parser.append_free_node(
        MarkdownNode::Footnote(Box::new(footnote::Footnote {
            label: encoded_label.clone(),
            ref_count: 0,
        })),
        start_location,
    );
    ctx.parser.tree[footnote_idx].span.end = end_location;
    let paragraph_idx = ctx.parser.append_to(
        footnote_idx,
        MarkdownNode::Paragraph,
        (content_start_location, content_end_location),
    );
    super::process(paragraph_idx, ctx.parser, smallvec::smallvec![content_span]);
    ctx.parser.footnotes.insert(label.clone(), footnote_idx);

    // index 与自动标签此处为临时值；parse_footnote_list 会按源码位置统一最终化
    let index = ctx.parser.footnote_refs.len() + 1;
    ctx.parser.footnote_refs.insert(label.clone(), (index, 1));
    ctx.parser.inline_footnote_defs.push(label.clone());
    let ref_node = ctx.parser.append_to(
        ctx.id,
        MarkdownNode::Link(Box::new(link::Link::Footnote(link::FootnoteLink {
            footnote_label: encoded_label,
            index,
            ref_count: 1,
        }))),
        (start_location, end_location),
    );
    ctx.parser.footnote_ref_nodes.push((ref_node, label));
    ctx.line.skip(cursor + 1 - start);
    true
}

pub(crate) fn process_footnote_list(
    parser: &mut Parser,
    node_refcounts: &[(usize, usize)],
    end_location: u32,
) {
    // 结束位置由调用方按文档位置计算（与 inline 处理调度无关）
    let locations = (parser.tree[0].span.start, end_location);
    let parent = parser.append_free_node(MarkdownNode::FootnoteList, locations.0);
    parser.tree[parent].span.end = locations.1;
    for &(idx, ref_count) in node_refcounts {
        let node = &mut parser.tree[idx];
        let ref_label = if let MarkdownNode::Footnote(footnote) = &mut node.body {
            footnote.ref_count = ref_count;
            footnote.label.clone()
        } else {
            continue;
        };
        let location = node.span.end;
        let mut last_child = parser.tree.get_last_child(idx);
        while let Some(idx) = last_child {
            if let MarkdownNode::Paragraph = parser.tree[idx].body {
                break;
            }
            if parser.tree[idx].body.is_inline_level() {
                last_child = Some(parser.tree.get_parent(idx));
                break;
            }
            last_child = parser.tree.get_last_child(idx);
        }
        let last_child = last_child.unwrap_or(idx);
        for i in 0..ref_count {
            parser.append_to(
                last_child,
                MarkdownNode::Link(Box::new(link::Link::FootnoteBackref(
                    link::FootnoteBackref {
                        footnote_label: ref_label.clone(),
                        index: i + 1,
                    },
                ))),
                (location, location),
            );
        }
        parser.tree.set_parent(idx, parent);
    }
    parser.tree.set_parent(parent, parser.doc);
}
