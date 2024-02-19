use crate::ast::code::{Code, CodeVariant};
use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessStage, BlockProcessing, Line};
use crate::parser::Parser;
use crate::tokenizer::Token;
use std::ops::Range;

pub struct FencedCode {}

impl FencedCode {
    fn try_match(line: &mut Line) -> Option<(Token<'static>, usize, Range<usize>)> {
        if line.indented() {
            return None;
        }
        line.skip_indent();
        let mark = match line.next().map(|it| it.token) {
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
                    if next.token == mark {
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
                    if next.token == Token::Backtick {
                        return None;
                    } else {
                        line.next();
                        State::StrictLanguageStatement
                    }
                }
                State::End => break,
            }
        }
        language_range.end = line.len();
        Some((mark, count, language_range))
    }
}
impl BlockProcessStage for FencedCode {
    fn initiate<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        let location = line[0].location;
        if let Some((mark, length, range)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            parser.add_block(
                MarkdownNode::Code(Code {
                    variant: CodeVariant::Fenced,
                    language: Some(line.slice_range(range).to_string()),
                    length,
                    mark,
                }),
                location,
            );
            BlockMatching::MatchedLeaf
        } else {
            BlockMatching::Unmatched
        }
    }
    fn process<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockProcessing {
        todo!()
    }
}

pub struct IndentedCode {}
