use std::fmt::Write;

use crate::ast::{code, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, Line, ProcessCtx};
use crate::parser::Parser;
use crate::tokenizer::Token;
use crate::utils;

#[derive(Debug)]
enum State {
    // ``` regexp: ^`{3,}(?!.*`)
    MarkerBackticks,
    // ~~~ regexp: ^~{3,}
    MarkerTilde,
    InLanguage(bool), // Loose = false, Strict = true
}

impl code::FencedCode {
    fn try_match(line: &Line) -> Option<(Token<'static>, usize, String)> {
        let (marker, mut state) = match line.peek() {
            Some(Token::Backtick) => (Token::Backtick, State::MarkerBackticks),
            Some(Token::Tilde) => (Token::Tilde, State::MarkerTilde),
            _ => return None,
        };
        let mut marker_length = 0;
        let mut language_range = (0, line.len());
        for (i, item) in line.iter().enumerate() {
            match (&state, &item.token) {
                (State::MarkerBackticks, Token::Backtick) => {}
                (State::MarkerBackticks, _) if i >= 3 => {
                    marker_length = i;
                    language_range.0 = i;
                    state = State::InLanguage(true);
                }
                (State::MarkerTilde, Token::Tilde) => {}
                (State::MarkerTilde, _) if i >= 3 => {
                    marker_length = i;
                    language_range.0 = i;
                    state = State::InLanguage(false);
                }
                (State::InLanguage(true), Token::Backtick) => return None,
                (State::InLanguage(_), _) => {}
                _ => return None,
            }
        }
        match state {
            State::MarkerBackticks | State::MarkerTilde => {
                marker_length = line.len();
                language_range.0 = line.len();
            }
            _ => (),
        }
        if marker_length < 3 {
            return None;
        }
        // println!(
        //     "marker = {marker:?} count={count} {:?} state={state:?}",
        //     language_range
        // );
        Some((
            marker,
            marker_length,
            if language_range.0 == language_range.1 {
                String::new()
            } else {
                line.slice(language_range.0, language_range.1)
                    .trim()
                    .to_string()
            },
        ))
    }
}
impl BlockStrategy for code::FencedCode {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line[0].location;
        if line.is_indented() {
            return BlockMatching::Unmatched;
        }
        line.skip_indent();
        if let Some((marker, marker_length, language)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            parser.append_block(
                MarkdownNode::Code(code::Code::Fenced(code::FencedCode {
                    language: if !language.is_empty() {
                        Some(utils::entities::unescape_string(language))
                    } else {
                        None
                    },
                    length: marker_length,
                    indent: line.indent_spaces(),
                    marker,
                })),
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
        // 尝试提取当前处理节点的代码块，如果不是代码块直接返回 Unprocessed
        let container = if let MarkdownNode::Code(code::Code::Fenced(code)) =
            &parser.tree[parser.curr_proc_node].body
        {
            code
        } else {
            return BlockProcessing::Unprocessed;
        };
        // println!("{line:?}");
        if line.is_indented() || line.is_blank() {
            line.skip_spaces(container.indent);
            return BlockProcessing::Further;
        }
        // 检查当前行是否满足结束代码块的条件
        let location = line.start_location();
        let length = line.skip_indent().starts_count(&container.marker);
        if length >= container.length && line.skip(length).only_space_to_end() {
            parser.finalize(parser.curr_proc_node, location);
            return BlockProcessing::Processed;
        }
        // 回滚到初始状态并删除等效的缩进
        line.resume(snapshot).skip_spaces(container.indent);
        BlockProcessing::Further
    }
    fn after(id: usize, parser: &mut Parser) {
        if let Some(lines) = parser.inlines.remove(&id) {
            let start = lines[0].start_location();
            let end = lines.last().map(|it| it.last_token_end_location()).unwrap();
            let mut literal = lines.into_iter().fold(String::new(), |mut str, it| {
                let _ = it.write_string(&mut str, false);
                let _ = str.write_char('\n');
                str
            });
            if !literal.ends_with('\n') {
                literal.write_char('\n').unwrap();
            }
            parser.append_text(literal, (start, end));
        }
    }
}

impl BlockStrategy for code::IndentedCode {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        // 没有缩进 或者 是段落（ 缩进代码块不能中止段落 ）或者该行是空的
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
            MarkdownNode::Code(code::Code::Indented(code::IndentedCode {})),
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
        if let Some(mut lines) = parser.inlines.remove(&id) {
            while let Some(true) = lines.last().map(|it| it.is_blank()) {
                lines.pop();
            }
            let start = lines[0].start_location();
            let end = lines.last().map(|it| it.last_token_end_location()).unwrap();
            let mut literal = lines.into_iter().fold(String::new(), |mut str, it| {
                let _ = it.write_string(&mut str, false);
                let _ = str.write_char('\n');
                str
            });
            if !literal.ends_with('\n') {
                literal.write_char('\n').unwrap();
            }
            parser.append_text(literal, (start, end));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{code, MarkdownNode};
    use crate::parser::Parser;
    use crate::tokenizer::Token;

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
            MarkdownNode::Code(code::Code::Fenced(code::FencedCode {
                language: Some("text".to_string()),
                length: 3,
                indent: 3,
                marker: Token::Backtick
            }))
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
            MarkdownNode::Code(code::Code::Indented(code::IndentedCode {}))
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
