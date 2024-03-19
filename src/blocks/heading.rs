use crate::ast;
use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing, BlockStrategy, Line};
use crate::parser::Parser;
use crate::tokenizer::Token;
use std::ops::Range;

pub struct ATXHeading {}

impl ATXHeading {
    fn try_match(line: &mut Line) -> Option<(usize, Range<usize>)> {
        if line.is_indented() {
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
                        line.advance_next_nonspace();
                        range.start = line.start_offset;
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
impl BlockStrategy for ATXHeading {
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
    fn before<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        let location = line[0].location;
        if let Some((hash_count, range)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            let idx = parser.append_block(
                MarkdownNode::Heading(ast::heading::Heading {
                    level: ast::heading::HeadingLevel::try_from(hash_count).unwrap(),
                    variant: ast::heading::HeadingVariant::ATX,
                }),
                location,
            );
            parser.append_inline(idx, line.slice_raw(range.start, range.end));
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

impl BlockStrategy for SetextHeading {
    fn before(parser: &mut Parser, line: &mut Line) -> BlockMatching {
        if !line.is_indented()
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
            if !line.only_spaces_to_end() {
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

#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::ast::MarkdownNode;
    use crate::parser::Parser;

    #[test]
    fn test_atx_heading() {
        let text = r#"
# foo
## foo
### foo
#### foo
##### foo
###### foo #
####### foo ##
#hashtag
"#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        for i in 1..7 {
            assert_eq!(
                ast[i].body,
                MarkdownNode::Heading(ast::heading::Heading {
                    level: ast::heading::HeadingLevel::try_from(i).unwrap(),
                    variant: ast::heading::HeadingVariant::ATX
                })
            );
            assert_eq!(ast.get_next(i), Some(i + 1));
        }
        assert_eq!(ast[7].body, MarkdownNode::Paragraph);
    }
    #[test]
    fn test_setext_heading() {
        let text = r#"
Foo *bar*
=========
Foo *bar*
---------
Foo *bar
baz*
====
"#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        for i in 1..3 {
            assert_eq!(
                ast[i].body,
                MarkdownNode::Heading(ast::heading::Heading {
                    level: ast::heading::HeadingLevel::try_from(i).unwrap(),
                    variant: ast::heading::HeadingVariant::SETEXT
                })
            );
            assert_eq!(ast.get_next(i), Some(i + 1));
        }
        assert_eq!(
            ast[3].body,
            MarkdownNode::Heading(ast::heading::Heading {
                level: ast::heading::HeadingLevel::H1,
                variant: ast::heading::HeadingVariant::SETEXT
            })
        );
    }
}
