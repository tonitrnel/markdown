use crate::ast::{MarkdownNode, code};
use crate::inlines::ProcessCtx;
use memchr::memchr;

pub(super) fn process(
    ProcessCtx {
        line, parser, id, ..
    }: &mut ProcessCtx,
) -> bool {
    let code_start_location = line.cursor_or_end() as u32;
    let marker_length = line.starts_count(b'`');
    line.skip(marker_length);
    let after_marker_snapshot = line.snapshot();
    let content_start = line.cursor();
    let mut marker_count = 0;
    while let Some(byte) = line.peek() {
        match byte {
            b'`' => {
                marker_count += 1;
                line.next_byte();
                if marker_count == marker_length && (line.is_end() || line.peek() != Some(b'`')) {
                    break;
                }
            }
            _ => {
                marker_count = 0;
                // 快速路径：在当前 Span 内直接跳到下一个 backtick，避免逐字符推进
                if let Some(span) = line.current_span() {
                    let start = span.cursor();
                    let end = span.end();
                    if start < end {
                        let source = span.source_slice();
                        let skipped = if let Some(off) = memchr(b'`', &source[start..end]) {
                            off
                        } else {
                            end - start
                        };
                        if skipped > 0 {
                            line.skip(skipped);
                            continue;
                        }
                    }
                }
                // 慢速路径：处理跨 Span 虚拟换行等边界字节
                line.next_byte();
            }
        }
    }
    if marker_count != marker_length {
        let marker_end_location = (content_start) as u32;
        parser.append_text_to_owned(
            *id,
            "`".repeat(marker_length),
            (code_start_location, marker_end_location),
        );
        line.resume(&after_marker_snapshot);
        return true;
    }
    let code_end_location = (line.cursor()) as u32;
    let content_end = line.cursor() - marker_length;
    let parent = parser.append_to(
        *id,
        MarkdownNode::Code(Box::new(code::Code::Inline(code::InlineCode {}))),
        (code_start_location, code_end_location),
    );
    let in_table = matches!(
        parser.tree[*id].body,
        MarkdownNode::TableHeadCol | MarkdownNode::TableDataCol
    );
    let content_bytes = &line.source_slice()[content_start..content_end];
    let content_str = unsafe { std::str::from_utf8_unchecked(content_bytes) };
    let had_line_endings = content_str
        .as_bytes()
        .iter()
        .any(|&b| b == b'\n' || b == b'\r');
    let has_newline = content_str.as_bytes().iter().any(|&b| b == b'\n');
    let start_loc = (content_start) as u32;
    let end_loc = (content_end) as u32;
    // 恒等快路径（M3）：无换行替换、无表格 `\|` 改写时内容即源码切片；
    // 剥一对空格的边缘仍是子切片。真正改写时才走 Owned 路径。
    let identity = !has_newline && !(in_table && content_str.as_bytes().contains(&b'\\'));
    if identity {
        let (mut s, mut e) = (content_start, content_end);
        if content_str.len() >= 2
            && content_str.starts_with(' ')
            && content_str.ends_with(' ')
            && (content_str.contains('`') || had_line_endings || marker_length >= 2)
        {
            s += 1;
            e -= 1;
        }
        let gfm_single_space = parser.options.github_flavored
            && marker_length >= 3
            && e - s == 1
            && content_str.as_bytes()[s - content_start] == b' ';
        if gfm_single_space {
            parser.append_text_to_owned(parent, String::new(), (start_loc, end_loc));
        } else {
            parser.append_text_span_to(parent, s as u32, e as u32, (start_loc, end_loc));
        }
        return true;
    }
    let mut text = if in_table {
        escaped_string(content_str, &['|'])
    } else if has_newline {
        content_str.replace('\n', " ")
    } else {
        content_str.to_string()
    };
    // 只有在包含换行且是 table 模式时才需要再次 replace
    if in_table && text.as_bytes().iter().any(|&b| b == b'\n') {
        text = text.replace('\n', " ");
    }
    // Align with current spec fixtures:
    // - single-space content in code span should become empty (` ``` ``` ` case)
    // - keep surrounding spaces for plain text (` ` b ` ` case)
    // - still strip one pair of spaces when protecting literal backticks.
    if text.len() >= 2
        && text.starts_with(' ')
        && text.ends_with(' ')
        && (text.contains('`') || had_line_endings || marker_length >= 2)
    {
        text.pop();
        text.drain(..1);
    }
    if parser.options.github_flavored && marker_length >= 3 && text == " " {
        text.clear();
    }
    parser.append_text_to_owned(parent, text, (start_loc, end_loc));
    true
}

fn escaped_string(s: &str, escaped_chars: &[char]) -> String {
    let bytes = s.as_bytes();
    let mut buf = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            let Some(next_char) = s[i + 1..].chars().next() else {
                break;
            };
            if escaped_chars.contains(&next_char) {
                buf.push(next_char);
            } else {
                buf.push('\\');
                buf.push(next_char);
            }
            i += 1 + next_char.len_utf8();
        } else {
            let Some(ch) = s[i..].chars().next() else {
                break;
            };
            buf.push(ch);
            i += ch.len_utf8();
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn case_328() {
        let text = r#"`foo`"#;
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(ast.to_html(), "<p><code>foo</code></p>")
    }
    #[test]
    fn case_329() {
        let text = r#"`` foo ` bar ``"#;
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(ast.to_html(), "<p><code>foo ` bar</code></p>")
    }
    #[test]
    fn case_3330() {
        let text = r#"` `` `"#;
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(ast.to_html(), "<p><code>``</code></p>");
    }
    #[test]
    fn case_337() {
        let text = "`foo   bar \nbaz`";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(ast.to_html(), "<p><code>foo   bar  baz</code></p>");
    }

    #[test]
    fn case_137_space_only() {
        let text = "`` ``";
        let ast = Parser::new(text).parse().unwrap();
        assert_eq!(ast.to_html(), "<p><code> </code></p>");
    }

    #[test]
    fn gfm_case_106_space_only() {
        let text = "``` ```\naaa";
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm())
            .parse()
            .unwrap();
        assert_eq!(ast.to_html(), "<p><code></code>\naaa</p>");
    }
}
