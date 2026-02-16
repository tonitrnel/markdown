use crate::ast::{heading, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::span::Span;

impl heading::ATXHeading {
    fn try_match(line: &Span) -> Option<(usize, usize, usize)> {
        if line.is_indented() {
            return None;
        }
        let len = line.len();
        if len == 0 {
            return None;
        }
        // Count leading '#'
        let hash_count = line.starts_count(b'#');
        if hash_count == 0 || hash_count > 6 {
            return None;
        }
        // After hashes: must be space/tab or end of line
        if hash_count < len {
            let next = line.get(hash_count);
            if let Some(b) = next {
                if b != b' ' && b != b'\t' {
                    return None;
                }
            }
        }
        // Find content range (skip leading space after hashes)
        let mut start = hash_count;
        if start < len {
            if let Some(b) = line.get(start) {
                if b == b' ' || b == b'\t' {
                    start += 1;
                }
            }
        }
        // Find end: trim trailing '#' and spaces
        let mut end = len;
        // Trim trailing spaces
        while end > start {
            if let Some(b) = line.get(end - 1) {
                if b == b' ' || b == b'\t' {
                    end -= 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // Trim trailing '#'
        let mut hash_end = end;
        while hash_end > start {
            if let Some(b) = line.get(hash_end - 1) {
                if b == b'#' {
                    hash_end -= 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // Trailing '#' must be preceded by space or be at start
        if hash_end < end && hash_end > start {
            if let Some(b) = line.get(hash_end - 1) {
                if b == b' ' || b == b'\t' {
                    end = hash_end;
                    // Trim trailing spaces again
                    while end > start {
                        if let Some(b) = line.get(end - 1) {
                            if b == b' ' || b == b'\t' {
                                end -= 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        } else if hash_end == start {
            // All trailing content is '#', remove it
            end = hash_end;
        }
        // Trim leading spaces from content
        while start < end {
            if let Some(b) = line.get(start) {
                if b == b' ' || b == b'\t' {
                    start += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some((hash_count, start, end))
    }
}

impl BlockStrategy for heading::ATXHeading {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        line.skip_indent();
        if let Some((hash_count, start, end)) = Self::try_match(line) {
            parser.close_unmatched_blocks();
            let Ok(level) = heading::HeadingLevel::try_from(hash_count) else {
                return BlockMatching::Unmatched;
            };
            let idx = parser.append_block(
                MarkdownNode::Heading(heading::Heading::ATX(heading::ATXHeading {
                    level,
                })),
                location,
            );
            parser.append_inline(idx, line.slice(start, end));
            line.skip_to_end();
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
    fn before(
        BeforeCtx {
            line,
            parser,
            container,
            ..
        }: BeforeCtx,
    ) -> BlockMatching {
        if !line.is_indented() && parser.tree[container].body == MarkdownNode::Paragraph {
            line.skip_indent();
            let first = line.peek();
            if first == Some(b'=') || first == Some(b'-') {
                if let Some(spans) = parser.inlines.get(&container) {
                    let all_ref_def = !spans.is_empty()
                        && spans.iter().all(crate::inlines::is_link_reference_line);
                    if all_ref_def {
                        return BlockMatching::Unmatched;
                    }
                }
                let Some(marker) = first else {
                    return BlockMatching::Unmatched;
                };
                let level = if marker == b'=' {
                    line.skip_consecutive(b'=');
                    heading::HeadingLevel::H1
                } else {
                    line.skip_consecutive(b'-');
                    heading::HeadingLevel::H2
                };
                if !line.only_space_to_end() {
                    return BlockMatching::Unmatched;
                }
                parser.replace_block(
                    MarkdownNode::Heading(heading::Heading::SETEXT(heading::SetextHeading {
                        level,
                    })),
                    line.end_location(),
                );
                return BlockMatching::MatchedLeaf;
            }
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
    use crate::parser::Location;
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
        let expected_locations = [
            (Location::new(1, 1), Location::new(1, 6)),
            (Location::new(2, 1), Location::new(2, 7)),
            (Location::new(3, 1), Location::new(3, 8)),
            (Location::new(4, 1), Location::new(4, 9)),
            (Location::new(5, 1), Location::new(5, 10)),
            (Location::new(6, 1), Location::new(6, 13)),
            (Location::new(7, 1), Location::new(8, 9)),
        ];
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
        let expected_locations = [
            (Location::new(1, 1), Location::new(2, 10)),
            (Location::new(3, 1), Location::new(4, 10)),
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
