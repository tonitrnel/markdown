use crate::ast::{heading, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, Line, ProcessCtx};
use crate::tokenizer::Token;
use std::ops::Range;

impl heading::ATXHeading {
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
                    if next == Token::Crosshatch {
                        line.next();
                        hash_count = 1;
                        State::HashesCounting
                    } else {
                        return None;
                    }
                }
                State::HashesCounting => match &next {
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
                    if next == Token::Crosshatch {
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
                    } else if next == Token::Crosshatch {
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
impl BlockStrategy for heading::ATXHeading {
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
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line[0].location;
        if let Some((hash_count, range)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            let idx = parser.append_block(
                MarkdownNode::Heading(heading::Heading::ATX(heading::ATXHeading {
                    level: heading::HeadingLevel::try_from(hash_count).unwrap(),
                })),
                location,
            );
            parser.append_inline(idx, line.slice_raw(range.start, range.end));
            BlockMatching::MatchedLeaf
        } else {
            BlockMatching::Unmatched
        }
    }
    fn process(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

impl BlockStrategy for heading::SetextHeading {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        if !line.is_indented()
            && parser.current_proc().body == MarkdownNode::Paragraph
            && line
                .skip_indent()
                .starts_with_matches(|it| matches!(it, Token::Eq | Token::Hyphen), 1)
        {
            let level = if line[0].token == Token::Eq {
                line.skip_consecutive_tokens(&Token::Eq);
                heading::HeadingLevel::H1
            } else {
                line.skip_consecutive_tokens(&Token::Hyphen);
                heading::HeadingLevel::H2
            };
            if !line.only_space_to_end() {
                return BlockMatching::Unmatched;
            }
            parser.replace_block(
                MarkdownNode::Heading(heading::Heading::SETEXT(heading::SetextHeading { level })),
                line.end_location(),
            );
            return BlockMatching::MatchedLeaf;
        }
        BlockMatching::Unmatched
    }
    fn process(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{heading, MarkdownNode};
    use crate::parser::Parser;
    use crate::tokenizer::Location;

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
        // 为每个标题定义预期的开始和结束位置
        let expected_locations = [
            (Location::new(1, 1), Location::new(1, 6)),
            (Location::new(2, 1), Location::new(2, 7)),
            (Location::new(3, 1), Location::new(3, 8)),
            (Location::new(4, 1), Location::new(4, 9)),
            (Location::new(5, 1), Location::new(5, 10)),
            (Location::new(6, 1), Location::new(6, 13)),
            // 注意：最后一行是段落，不是标题
            (Location::new(7, 1), Location::new(8, 9)),
        ];
        // 检查标题节点
        for (i, &(start, end)) in expected_locations.iter().enumerate().take(6) {
            match &ast[i + 1].body {
                MarkdownNode::Heading(heading::Heading::ATX(atx)) => {
                    assert_eq!(atx.level, heading::HeadingLevel::try_from(i + 1).unwrap());
                }
                _ => panic!("Expected heading, found {:?}", ast[i + 1].body),
            }
            assert_eq!(ast[i + 1].start, start);
            assert_eq!(ast[i + 1].end, end);
            assert_eq!(ast.get_next(i + 1), Some(i + 2));
        }
        // 检查最后一个段落节点
        assert_eq!(ast[7].body, MarkdownNode::Paragraph);
        let last = expected_locations.last().unwrap();
        assert_eq!(ast[7].start, last.0);
        assert_eq!(ast[7].end, last.1);
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
        println!("{ast:?}");
        let expected_locations = [
            (Location::new(1, 1), Location::new(2, 10)),
            (Location::new(3, 1), Location::new(4, 10)),
            // 注意：最后一行是段落，不是标题
            (Location::new(5, 1), Location::new(7, 5)),
        ];
        for i in 1..3 {
            let (start, end) = expected_locations[i - 1];
            assert_eq!(
                ast[i].body,
                MarkdownNode::Heading(heading::Heading::SETEXT(heading::SetextHeading {
                    level: heading::HeadingLevel::try_from(i).unwrap(),
                }))
            );
            assert_eq!(ast[i].start, start);
            assert_eq!(ast[i].end, end);
            assert_eq!(ast.get_next(i), Some(i + 1));
        }
        assert_eq!(
            ast[3].body,
            MarkdownNode::Heading(heading::Heading::SETEXT(heading::SetextHeading {
                level: heading::HeadingLevel::H1,
            }))
        );

        let last = expected_locations.last().unwrap();
        assert_eq!(ast[3].start, last.0);
        assert_eq!(ast[3].end, last.1);
    }
}
