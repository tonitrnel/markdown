use std::fmt::Write;
use std::ops::Range;

use crate::ast::{code, MarkdownNode};
use crate::blocks::{BlockMatching, BlockProcessing, BlockStrategy, Line};
use crate::parser::Parser;
use crate::tokenizer::Token;

impl code::FencedCode {
    fn try_match(line: &mut Line) -> Option<(Token<'static>, usize, Range<usize>)> {
        if line.is_indented() {
            return None;
        }
        line.skip_indent();
        let mark = match line.next() {
            Some(Token::Backtick) => Token::Backtick,
            Some(Token::Tilde) => Token::Tilde,
            _ => return None,
        };
        let mut count = 1;
        let mut language_range = Range { start: 0, end: 0 };
        enum State {
            Start,
            // ``` regexp: ^`{3,}(?!.*`)
            StrictLanguageStatement,
            // ~~~ regexp: ^~{3,}
            LooseLanguageStatement,
            End,
        }
        let mut state: State = State::Start;
        while let Some(&next) = line.peek() {
            state = match state {
                State::Start => {
                    if next == mark {
                        count += 1;
                        line.next();
                        State::Start
                    } else {
                        language_range.start = line.start_offset;
                        if count < 3 {
                            return None;
                        }
                        if mark == Token::Backtick {
                            State::StrictLanguageStatement
                        } else {
                            State::LooseLanguageStatement
                        }
                    }
                }
                State::LooseLanguageStatement => State::End,
                State::StrictLanguageStatement => {
                    if next == Token::Backtick {
                        return None;
                    } else {
                        line.next();
                        State::StrictLanguageStatement
                    }
                }
                State::End => break,
            }
        }
        language_range.end = line.end_offset;
        Some((mark, count, language_range))
    }
}
impl BlockStrategy for code::FencedCode {
    fn before<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        let location = line[0].location;
        if let Some((marker, length, range)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            parser.append_block(
                MarkdownNode::Code(code::Code::Fenced(code::FencedCode {
                    language: Some(line.slice_raw(range.start, range.end).to_string()),
                    length,
                    indent: line.indent,
                    marker,
                })),
                location,
            );
            BlockMatching::MatchedLeaf
        } else {
            BlockMatching::Unmatched
        }
    }
    fn process<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockProcessing {
        let snapshot = line.snapshot();
        // 尝试提取当前处理节点的代码块，如果不是代码块直接返回 Unprocessed
        let container = if let MarkdownNode::Code(code::Code::Fenced(code)) =
            &parser.tree[parser.curr_proc_node].body
        {
            code
        } else {
            return BlockProcessing::Unprocessed;
        };
        // 检查当前行是否满足结束代码块的条件
        let length = line.skip_indent().starts_count(&container.marker);
        if length >= container.length && line.skip(length).only_spaces_to_end() {
            parser.finalize(parser.curr_proc_node);
            return BlockProcessing::Processed;
        }
        // 回滚到初始状态并删除等效的缩进
        line.resume(snapshot).skip_spaces(container.indent);
        BlockProcessing::Further
    }
    fn after(id: usize, parser: &mut Parser) {
        if let Some(lines) = parser.inlines.remove(&id) {
            let location = lines[0].location();
            let literal = lines.into_iter().fold(String::new(), |mut str, it| {
                let _ = writeln!(str, "{}", it);
                str
            });
            parser.append_text(literal, location);
        }
    }
}

impl BlockStrategy for code::IndentedCode {
    fn before<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        // 没有缩进 或者 是段落（ 缩进代码块不能中止段落 ）或者该行是空的
        if !line.is_indented()
            || parser.tree[parser.curr_proc_node].body == MarkdownNode::Paragraph
            || line.is_blank()
        {
            return BlockMatching::Unmatched;
        };
        let location = line.location();
        line.skip(4);
        parser.close_unmatched_blocks();
        parser.append_block(
            MarkdownNode::Code(code::Code::Indented(code::IndentedCode {})),
            location,
        );
        BlockMatching::MatchedLeaf
    }

    fn process<'input>(_parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockProcessing {
        if line.is_indented() {
            line.skip(4);
            BlockProcessing::Further
        } else if line.is_blank() {
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
            let location = lines[0].location();
            let literal = lines.into_iter().fold(String::new(), |mut str, it| {
                let _ = writeln!(str, "{}", it);
                str
            });
            parser.append_text(literal, location);
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
        if let MarkdownNode::Text(text) = &ast[2].body {
            assert_eq!(text, "aaa\n aaa\naaa\n");
        } else {
            panic!()
        }
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
    }
}
