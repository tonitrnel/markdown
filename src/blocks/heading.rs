use crate::ast;
use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing, Line};
use crate::parser::{Node, Parser};
use crate::tokenizer::Token;

enum ATXState {
    Start,
    HashesCounting,
    Content,
    EndHashes,
    End,
}

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
pub fn atx_initiate<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
    let location = line[0].location;
    if line.indented() {
        return BlockMatching::Unmatched;
    }
    line.skip_indent();

    let mut state = ATXState::Start;
    let mut count = 0;
    let mut start = 0;
    let mut end = 0;
    while let Some(&next) = line.peek() {
        state = match state {
            ATXState::Start => {
                if next.token == Token::Crosshatch {
                    line.next();
                    count = 1;
                    ATXState::HashesCounting
                } else {
                    return BlockMatching::Unmatched;
                }
            }
            ATXState::HashesCounting => match &next.token {
                Token::Crosshatch => {
                    start = line.start_offset;
                    count += 1;
                    line.next();
                    ATXState::HashesCounting
                }
                _ if next.is_space_or_tab() => {
                    if count > 6 {
                        return BlockMatching::Unmatched;
                    };
                    start = line.start_offset;
                    line.next();
                    ATXState::Content
                }
                _ => return BlockMatching::Unmatched,
            },
            ATXState::Content => {
                if next.is_space_or_tab() {
                    line.next();
                    ATXState::End
                } else {
                    line.next();
                    ATXState::Content
                }
            }
            ATXState::EndHashes => {
                end = line.start_offset;
                if next.token == Token::Crosshatch {
                    line.skip_consecutive_tokens(&Token::Crosshatch);
                    ATXState::End
                } else if next.is_space_or_tab() {
                    ATXState::End
                } else {
                    ATXState::Content
                }
            }
            // ## heading 2 ## # ## ### #
            ATXState::End => {
                end = line.start_offset;
                if next.is_space_or_tab() {
                    line.advance_next_nonspace();
                    ATXState::End
                } else if next.token == Token::Crosshatch {
                    ATXState::EndHashes
                } else {
                    ATXState::Content
                }
            }
        };
    }
    if count == 0 {
        return BlockMatching::Unmatched;
    }
    if !matches!(state, ATXState::EndHashes | ATXState::End) {
        end = line.start_offset;
    }
    let idx = parser.add_block(
        MarkdownNode::Heading(ast::heading::Heading {
            level: ast::heading::HeadingLevel::try_from(count).unwrap(),
            variant: ast::heading::HeadingVariant::ATX,
        }),
        location,
    );
    parser.add_inline(idx, line.slice(start, end));
    BlockMatching::MatchedLeaf
}

pub fn setext_initiate(parser: &mut Parser, line: &mut Line) -> BlockMatching {
    let location = line[0].location;
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

pub fn process(_parser: &mut Parser, _line: &mut Line) -> BlockProcessing {
    BlockProcessing::Unprocessed
}
pub fn finalize(_parser: &mut Parser, _block: Node) {}
