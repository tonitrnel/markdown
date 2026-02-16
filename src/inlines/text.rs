use crate::ast::MarkdownNode;
use crate::parser::Parser;

/// 合并相邻的 Text Node，同时对合并后的文本执行 CJK 校正（如果启用）。
/// 单次遍历完成合并 + 校正，不需要额外的 text_ids 收集步骤。
fn merge_and_correct_text(
    parser: &mut Parser,
    idx: usize,
    cjk_rich: bool,
    normalize_chi_punct: bool,
) {
    let mut stack = vec![(idx, None::<usize>)];
    while let Some((idx, into_idx)) = stack.pop() {
        if matches!(parser.tree[idx].body, MarkdownNode::Text(..)) {
            let next = parser.tree.get_next(idx);
            let merged_id = if let Some(into_idx) = into_idx {
                let node = parser.tree.remove(idx);
                match (&mut parser.tree[into_idx].body, &node.body) {
                    (MarkdownNode::Text(into_str), MarkdownNode::Text(str)) => {
                        into_str.push_str(str);
                    }
                    _ => panic!("unexpected error"),
                }
                parser.tree[into_idx].end = node.end;
                into_idx
            } else {
                idx
            };
            // 如果下一个兄弟也是 Text，继续合并
            if let Some(next_idx) = next {
                if matches!(parser.tree[next_idx].body, MarkdownNode::Text(..)) {
                    stack.push((next_idx, Some(merged_id)));
                    continue;
                }
                // 下一个兄弟不是 Text，合并链结束，执行 CJK 校正
                if cjk_rich {
                    if let MarkdownNode::Text(text) = &mut parser.tree[merged_id].body {
                        let cjk_nouns = parser.options.cjk_nouns.iter();
                        correct_cjk_text(text, normalize_chi_punct, cjk_nouns);
                    }
                }
                stack.push((next_idx, None));
            } else {
                // 没有后续兄弟，合并链结束，执行 CJK 校正
                if cjk_rich {
                    if let MarkdownNode::Text(text) = &mut parser.tree[merged_id].body {
                        let cjk_nouns = parser.options.cjk_nouns.iter();
                        correct_cjk_text(text, normalize_chi_punct, cjk_nouns);
                    }
                }
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

/// 最终处理文本节点（合并相邻文本、CJK 校正），在所有 Span 处理完毕后调用
pub(super) fn process_final(id: usize, parser: &mut Parser) {
    let next = match parser.tree.get_first_child(id) {
        Some(idx) => idx,
        _ => return,
    };
    let cjk_rich = parser.options.cjk_autocorrect
        && matches!(
            &parser.tree[id].body,
            MarkdownNode::Paragraph | MarkdownNode::Heading(..)
        );
    let normalize_chi_punct = parser.options.normalize_chinese_punctuation
        && matches!(
            &parser.tree[id].body,
            MarkdownNode::Paragraph | MarkdownNode::Heading(..)
        );
    merge_and_correct_text(parser, next, cjk_rich, normalize_chi_punct);
}

fn correct_cjk_text<I, S>(text: &mut String, normalize_chi_punct: bool, cjk_nouns: I)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    // 1. CJK 空格插入
    let corrected =
        crate::utils::cjk::correct_cjk_spacing_with_nouns::<I, S>(text.as_ref(), cjk_nouns);
    if let std::borrow::Cow::Owned(new_text) = corrected {
        *text = new_text;
    }

    // 2. 中文标点规范化（可选）
    if normalize_chi_punct {
        let normalized =
            crate::utils::chinese_punctuation::normalize_chi_punctuation(text.as_ref());
        if let std::borrow::Cow::Owned(new_text) = normalized {
            *text = new_text;
        }
    }
}
