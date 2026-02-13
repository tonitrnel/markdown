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
    let mut stack = vec![(idx, into_idx)];
    while let Some((idx, into_idx)) = stack.pop() {
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
                stack.push((next_idx, into_idx))
            }
        } else {
            if let Some(child_idx) = parser.tree.get_first_child(idx) {
                stack.push((child_idx, None))
            }
            if let Some(next_idx) = parser.tree.get_next(idx) {
                stack.push((next_idx, None))
            }
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
    if !parser.options.cjk_autocorrect {
        return;
    }
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
        if is_rich {
            correct_cjk_text(text);
        }
    }
}

fn correct_cjk_text(text: &mut String) {
    let corrected = crate::utils::cjk::correct_cjk_spacing(text.as_ref());
    if let std::borrow::Cow::Owned(new_text) = corrected {
        *text = new_text;
    }
}
