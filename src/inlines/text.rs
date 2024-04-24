use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::parser::Parser;

// todo: 再次处理 Text Node，进行合并、中文语境优化
pub(super) fn process(
    ProcessCtx { id, parser, .. }: &mut ProcessCtx,
    enabled_cjk_autocorrect: bool,
) {
    let next = match parser.tree.get_first_child(*id) {
        Some(idx) => idx,
        _ => return,
    };
    let mut text_ids = Vec::new();
    let start = std::time::Instant::now();
    println!("合并文本節點開始 #{id}");
    merge_text(parser, &mut text_ids, next, None);
    println!("合并文本節點結束[{}µs]", start.elapsed().as_micros());
    for text_id in text_ids {
        if let MarkdownNode::Text(str) = &mut parser.tree[text_id].body {
            process_text(str, enabled_cjk_autocorrect);
        }
    }
}

fn merge_text(parser: &mut Parser, text_ids: &mut Vec<usize>, idx: usize, into_idx: Option<usize>) {
    if matches!(parser.tree[idx].body, MarkdownNode::Text(..)) {
        let next = parser.tree.get_next(idx);
        let into_idx = if let Some(into_idx) = into_idx {
            println!("删除文本节点 #{idx}");
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
            merge_text(parser, text_ids, next_idx, into_idx)
        }
    } else {
        if let Some(child_idx) = parser.tree.get_first_child(idx) {
            merge_text(parser, text_ids, child_idx, None)
        }
        if let Some(next_idx) = parser.tree.get_next(idx) {
            merge_text(parser, text_ids, next_idx, None)
        }
    }
}

fn process_text(text: &mut String, enabled_cjk_autocorrect: bool) {
    println!("自動校正開始");
    let start = std::time::Instant::now();
    if enabled_cjk_autocorrect {
        *text = autocorrect::format(text);
    }
    println!("自動校正結束[{}µs]", start.elapsed().as_micros());
}
