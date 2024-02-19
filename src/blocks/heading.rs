use crate::ast;
use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessStage, BlockProcessing, Line};
use crate::parser::Parser;
use crate::tokenizer::Token;
use std::ops::Range;

pub struct ATXHeading {}

impl ATXHeading {
    fn try_match(line: &mut Line) -> Option<(usize, Range<usize>)> {
        if line.indented() {
            return None;
        }
        line.skip_indent();
        enum State {
            Start,
            HashesCounting,
            Content,
            EndHashes,
            End,
        }
        let mut state = State::Start;
        let mut hash_count = 0;
        let mut range = Range { start: 0, end: 0 };
        while let Some(&next) = line.peek() {
            state = match state {
                State::Start => {
                    if next.token == Token::Crosshatch {
                        line.next();
                        hash_count = 1;
                        State::HashesCounting
                    } else {
                        return None;
                    }
                }
                State::HashesCounting => match &next.token {
                    Token::Crosshatch => {
                        range.start = line.start_offset;
                        hash_count += 1;
                        line.next();
                        State::HashesCounting
                    }
                    _ if next.is_space_or_tab() => {
                        if hash_count > 6 {
                            return None;
                        };
                        range.start = line.start_offset;
                        line.next();
                        State::Content
                    }
                    _ => return None,
                },
                State::Content => {
                    if next.is_space_or_tab() {
                        line.next();
                        State::End
                    } else {
                        line.next();
                        State::Content
                    }
                }
                State::EndHashes => {
                    range.end = line.start_offset;
                    if next.token == Token::Crosshatch {
                        line.skip_consecutive_tokens(&Token::Crosshatch);
                        State::End
                    } else if next.is_space_or_tab() {
                        State::End
                    } else {
                        State::Content
                    }
                }
                // ## heading 2 ## # ## ### #
                State::End => {
                    range.end = line.start_offset;
                    if next.is_space_or_tab() {
                        line.advance_next_nonspace();
                        State::End
                    } else if next.token == Token::Crosshatch {
                        State::EndHashes
                    } else {
                        State::Content
                    }
                }
            };
        }
        if hash_count == 0 {
            return None;
        }
        if !matches!(state, State::EndHashes | State::End) {
            range.end = line.start_offset;
        }
        Some((hash_count, range))
    }
}
impl BlockProcessStage for ATXHeading {
    /// AXT headings
    ///
    /// ```text
    ///  # foo
    ///  ## foo
    ///  ### foo
    ///  #### foo
    ///  ##### foo
    ///  ###### foo
    ///  ## foo ## ## #
    /// ```
    fn initiate<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        let location = line[0].location;
        if let Some((hash_count, range)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            let idx = parser.add_block(
                MarkdownNode::Heading(ast::heading::Heading {
                    level: ast::heading::HeadingLevel::try_from(hash_count).unwrap(),
                    variant: ast::heading::HeadingVariant::ATX,
                }),
                location,
            );
            parser.add_inline(idx, line.slice(range.start, range.end));
            BlockMatching::MatchedLeaf
        } else {
            BlockMatching::Unmatched
        }
    }
    fn process(_parser: &mut Parser, _line: &mut Line) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

pub struct SetextHeading {}

impl BlockProcessStage for SetextHeading {
    fn initiate(parser: &mut Parser, line: &mut Line) -> BlockMatching {
        if !line.indented()
            && parser.current_container().body == MarkdownNode::Paragraph
            && line
                .skip_indent()
                .starts_with_matches(|it| matches!(it, Token::Eq | Token::Hyphen), 1)
        {
            let level = if line[0].token == Token::Eq {
                line.skip_consecutive_tokens(&Token::Eq);
                ast::heading::HeadingLevel::H1
            } else {
                line.skip_consecutive_tokens(&Token::Hyphen);
                ast::heading::HeadingLevel::H2
            };
            if !line.ensure_only_spaces_to_end() {
                return BlockMatching::Unmatched;
            }
            parser.replace_block(MarkdownNode::Heading(ast::heading::Heading {
                level,
                variant: ast::heading::HeadingVariant::SETEXT,
            }));
            return BlockMatching::MatchedLeaf;
        }
        BlockMatching::Unmatched
    }
    fn process(_parser: &mut Parser, _line: &mut Line) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}
