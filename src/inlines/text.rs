use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::parser::Parser;

/// 合并相邻的 Text Node
fn merge_text_node(
    parser: &mut Parser,
    text_ids: &mut Vec<usize>,
    idx: usize,
    into_idx: Option<usize>,
) {
    if matches!(parser.tree[idx].body, MarkdownNode::Text(..)) {
        let next = parser.tree.get_next(idx);
        let into_idx = if let Some(into_idx) = into_idx {
            // println!("删除文本节点 #{idx}")
            let node = parser.tree.remove(idx);
            match (&mut parser.tree[into_idx].body, node.body) {
                (MarkdownNode::Text(into_str), MarkdownNode::Text(mut str)) => {
                    into_str.extend(str.drain(..));
                }
                _ => panic!("unexpected error"),
            }
            parser.tree[into_idx].end = node.end;
            Some(into_idx)
        } else {
            text_ids.push(idx);
            Some(idx)
        };
        if let Some(next_idx) = next {
            merge_text_node(parser, text_ids, next_idx, into_idx)
        }
    } else {
        if let Some(child_idx) = parser.tree.get_first_child(idx) {
            merge_text_node(parser, text_ids, child_idx, None)
        }
        if let Some(next_idx) = parser.tree.get_next(idx) {
            merge_text_node(parser, text_ids, next_idx, None)
        }
    }
}
pub(super) fn process(ProcessCtx { id, parser, .. }: &mut ProcessCtx) {
    let next = match parser.tree.get_first_child(*id) {
        Some(idx) => idx,
        _ => return,
    };
    let mut text_ids = Vec::new();
    // let start = std::time::Instant::now();
    // println!("合并文本節點開始 #{id}")
    merge_text_node(parser, &mut text_ids, next, None);
    // println!("合并文本節點結束[{}µs]", start.elapsed().as_micros());
    let is_rich = matches!(
        &parser.tree[*id].body,
        MarkdownNode::Paragraph | MarkdownNode::Heading(..)
    );
    for text_id in text_ids {
        let text = if let MarkdownNode::Text(text) = &mut parser.tree[text_id].body {
            text
        } else {
            continue;
        };
        if parser.options.cjk_autocorrect && is_rich {
            correct_cjk_text(text);
        }
    }
}

fn correct_cjk_text(text: &mut String) {
    // println!("自動校正開始")
    // let start = std::time::Instant::now();
    let corrected = autocorrect::format(text.as_ref());
    if &corrected != text {
        *text = corrected
    }
    // println!("自動校正結束[{}µs]", start.elapsed().as_micros());
}
