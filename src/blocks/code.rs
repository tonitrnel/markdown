use std::fmt::Write;

use crate::ast::{MarkdownNode, code};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::parser::Parser;
use crate::span::Span;
use crate::utils;

impl code::FencedCode {
    fn try_match(line: &Span) -> Option<(u8, usize, String)> {
        let marker = match line.peek() {
            Some(b'`') => b'`',
            Some(b'~') => b'~',
            _ => return None,
        };
        let marker_length = line.starts_count(marker);
        if marker_length < 3 {
            return None;
        }
        // For backtick fences, no backtick allowed in info string
        if marker == b'`' {
            // Check remaining content for backticks
            for i in marker_length..line.len() {
                if line.get(i) == Some(b'`') {
                    return None;
                }
            }
        }
        // Extract language (info string after marker)
        let language = if marker_length < line.len() {
            let lang_span = line.slice(marker_length, line.len());
            let trimmed = lang_span.trim();
            trimmed.as_str().to_string()
        } else {
            String::new()
        };
        Some((marker, marker_length, language))
    }
}

fn backslash_unescape(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut result = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_punctuation() {
            result.push(bytes[i + 1] as char);
            i += 2;
        } else {
            let Some(ch) = s[i..].chars().next() else {
                break;
            };
            result.push(ch);
            i += ch.len_utf8();
        }
    }
    result
}

impl BlockStrategy for code::FencedCode {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        if line.is_indented() {
            return BlockMatching::Unmatched;
        }
        line.skip_spaces(4);
        if let Some((marker, marker_length, language)) = Self::try_match(line) {
            // OFM: keep `~~~text~~~` as plain text (strikethrough-related syntax), not fenced code.
            if parser.options.obsidian_flavored && marker == b'~' && language.ends_with("~~~") {
                return BlockMatching::Unmatched;
            }
            parser.close_unmatched_blocks();
            // Convert marker byte to FenceMarker enum
            let marker_token = if marker == b'`' {
                code::FenceMarker::Backtick
            } else {
                code::FenceMarker::Tilde
            };
            parser.append_block(
                MarkdownNode::Code(Box::new(code::Code::Fenced(code::FencedCode {
                    language: if !language.is_empty() {
                        Some(backslash_unescape(&utils::entities::unescape_string(
                            language,
                        )))
                    } else {
                        None
                    },
                    length: marker_length,
                    indent: line.indent_spaces(),
                    marker: marker_token,
                }))),
                location,
            );
            line.skip_to_end();
            BlockMatching::MatchedLeaf
        } else {
            BlockMatching::Unmatched
        }
    }
    fn process(ProcessCtx { line, parser, .. }: ProcessCtx) -> BlockProcessing {
        let snapshot = line.snapshot();
        let container = if let MarkdownNode::Code(c) = &parser.tree[parser.curr_proc_node].body {
            if let code::Code::Fenced(code) = c.as_ref() {
                code
            } else {
                return BlockProcessing::Unprocessed;
            }
        } else {
            return BlockProcessing::Unprocessed;
        };
        if line.is_indented() || line.is_blank() {
            line.skip_spaces(container.indent);
            return BlockProcessing::Further;
        }
        let marker_byte = if container.marker == code::FenceMarker::Backtick {
            b'`'
        } else {
            b'~'
        };
        let length = line.skip_indent().starts_count(marker_byte);
        if length >= container.length && line.skip(length).only_space_to_end() {
            // Use end location of the closing fence line
            let end_location = line.end_location();
            parser.finalize(parser.curr_proc_node, end_location);
            return BlockProcessing::Processed;
        }
        line.resume(&snapshot).skip_spaces(container.indent);
        BlockProcessing::Further
    }
    fn after(id: usize, parser: &mut Parser) {
        if let Some(spans) = parser.inlines.remove(&id) {
            if spans.is_empty() {
                return;
            }
            let start = spans[0].start_location();
            let end = spans
                .last()
                .map(|it| it.last_token_end_location())
                .unwrap_or(start);
            let estimated = spans.iter().map(|it| it.len() + 1).sum();
            let mut literal = String::with_capacity(estimated);
            for span in spans {
                literal.push_str(span.as_str());
                literal.push('\n');
            }
            if !literal.ends_with('\n') {
                let _ = literal.write_char('\n');
            }
            parser.append_text(literal, (start, end));
        }
    }
}

impl BlockStrategy for code::IndentedCode {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        if !line.is_indented()
            || parser.tree[parser.curr_proc_node].body == MarkdownNode::Paragraph
            || line.is_blank()
        {
            return BlockMatching::Unmatched;
        };
        let location = line.start_location();
        line.skip_spaces(4);
        parser.close_unmatched_blocks();
        parser.append_block(
            MarkdownNode::Code(Box::new(code::Code::Indented(code::IndentedCode {}))),
            location,
        );
        BlockMatching::MatchedLeaf
    }

    fn process(ProcessCtx { line, .. }: ProcessCtx) -> BlockProcessing {
        if line.is_indented() || line.is_blank() {
            line.skip_spaces(4);
            BlockProcessing::Further
        } else {
            BlockProcessing::Unprocessed
        }
    }
    fn after(id: usize, parser: &mut Parser) {
        if let Some(mut spans) = parser.inlines.remove(&id) {
            while let Some(true) = spans.last().map(|it| it.is_blank()) {
                spans.pop();
            }
            if spans.is_empty() {
                return;
            }
            let start = spans[0].start_location();
            let end = spans
                .last()
                .map(|it| it.last_token_end_location())
                .unwrap_or(start);
            let container_prefix_cols = container_prefix_cols(id, parser);
            let mut literal = spans.into_iter().fold(String::new(), |mut str, it| {
                let _ = write_recovered_indented_line(&mut str, &it, container_prefix_cols);
                let _ = str.write_char('\n');
                str
            });
            if !literal.ends_with('\n') {
                let _ = literal.write_char('\n');
            }
            parser.append_text(literal, (start, end));
        }
    }
}

fn write_recovered_indented_line(
    out: &mut String,
    span: &crate::span::Span<'_>,
    container_prefix: usize,
) -> std::fmt::Result {
    let full = span.full_str();
    let tail = span.as_str();
    let consumed_bytes = full.len().saturating_sub(tail.len());
    let consumed = &full.as_bytes()[..consumed_bytes];
    let actual_consumed_cols = visual_cols(consumed);
    let expected_consumed_cols = container_prefix + 4;
    if actual_consumed_cols > expected_consumed_cols {
        for _ in 0..(actual_consumed_cols - expected_consumed_cols) {
            out.write_char(' ')?;
        }
    }
    out.write_str(tail)?;
    Ok(())
}

fn visual_cols(bytes: &[u8]) -> usize {
    let mut col = 0usize;
    for &b in bytes {
        if b == b'\t' {
            col += 4 - (col % 4);
        } else {
            col += 1;
        }
    }
    col
}

fn container_prefix_cols(id: usize, parser: &Parser<'_>) -> usize {
    let mut sum = 0usize;
    let mut cur = parser.tree.get_parent(id);
    while cur != 0 {
        match &parser.tree[cur].body {
            MarkdownNode::List(list) => {
                sum += list.padding() + list.marker_offset();
            }
            MarkdownNode::BlockQuote => {
                // `>` marker plus its optional following space/tab.
                sum += 2;
            }
            _ => {}
        }
        cur = parser.tree.get_parent(cur);
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::ast::{MarkdownNode, code};
    use crate::parser::Parser;

    #[test]
    fn test_fenced_code() {
        let text = r#"   ```text
   aaa
    aaa
  aaa
   ````  
"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Code(Box::new(code::Code::Fenced(code::FencedCode {
                language: Some("text".to_string()),
                length: 3,
                indent: 3,
                marker: code::FenceMarker::Backtick
            })))
        );
        assert_eq!(
            ast.to_html(),
            r##"<pre><code class="language-text">aaa
 aaa
aaa
</code></pre>"##
        )
    }

    #[test]
    fn test_indented_code() {
        let text = r#"    a simple
      indented code block
      chunk1
      
      chunk2
      
      
      chunk3
      
"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Code(Box::new(code::Code::Indented(code::IndentedCode {})))
        );
        if let MarkdownNode::Text(text) = &ast[2].body {
            assert!(text.starts_with("a simple"));
            assert!(text.ends_with("chunk3\n"));
        } else {
            panic!()
        }
        println!("{ast:?}")
    }
}
