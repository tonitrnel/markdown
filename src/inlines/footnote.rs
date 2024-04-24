use crate::ast::{link, MarkdownNode};
use crate::parser::Parser;

pub(crate) fn process_footnote_list(parser: &mut Parser, node_refcounts: Vec<(usize, usize)>) {
    let locations = {
        let mut start = 0;
        let mut end = 0;
        for node_refcount in node_refcounts.iter() {
            start = node_refcount.0.min(start);
            end = node_refcount.0.max(end);
        }
        let start = parser.tree[0].start;
        let end = parser.tree[end].end;
        (start, end)
    };
    let parent = parser.append_free_node(MarkdownNode::FootnoteList, locations.0);
    parser.tree[parent].end = locations.1;
    for (idx, ref_count) in node_refcounts {
        let node = &mut parser.tree[idx];
        let ref_label = if let MarkdownNode::Footnote(footnote) = &mut node.body {
            footnote.ref_count = ref_count;
            footnote.label.clone()
        } else {
            continue;
        };
        let location = node.end;
        let mut last_child = parser.tree.get_last_child(idx);
        while let Some(idx) = last_child {
            if let MarkdownNode::Paragraph = parser.tree[idx].body {
                break;
            }
            if parser.tree[idx].body.is_inline_node() {
                last_child = Some(parser.tree.get_parent(idx));
                break;
            }
            last_child = parser.tree.get_last_child(idx);
        }
        let last_child = last_child.unwrap_or(idx);
        for i in 0..ref_count {
            parser.append_block_to(
                last_child,
                MarkdownNode::Link(link::Link::FootnoteBackref(link::FootnoteBackref {
                    footnote_label: ref_label.clone(),
                    index: i + 1,
                })),
                (location, location),
            );
        }
        parser.tree.set_parent(idx, parent);
    }
    parser.tree.set_parent(parent, parser.doc);
}
