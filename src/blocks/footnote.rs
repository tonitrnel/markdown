use crate::ast::{MarkdownNode, footnote};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::parser::Parser;
use crate::utils;

impl BlockStrategy for footnote::Footnote {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        if !line.is_indented() && line.advance_next_nonspace().starts_with(b'[', 1) {
            line.next_byte(); // consume '['
            if !line.consume(b'^') {
                return BlockMatching::Unmatched;
            }
            let mut end = 0;
            let mut found = false;
            let remaining = line.len();
            for i in 0..remaining {
                let b = match line.get(i) {
                    Some(b) => b,
                    None => break,
                };
                match b {
                    b']' => {
                        if line.get(i + 1) == Some(b':') {
                            end = i;
                            found = true;
                            break;
                        }
                        return BlockMatching::Unmatched;
                    }
                    b'[' | b' ' | b'\t' | b'\n' | b'\r' => return BlockMatching::Unmatched,
                    _ => continue,
                }
            }
            if !found {
                return BlockMatching::Unmatched;
            }
            let label = line.slice(0, end).as_str().to_string();
            line.skip(end + 2);
            parser.close_unmatched_blocks();
            let idx = parser.append_block(
                MarkdownNode::Footnote(Box::new(footnote::Footnote {
                    label: utils::percent_encode::encode(&label, true),
                    ref_count: 0,
                })),
                location,
            );
            parser.footnotes.entry(label).or_insert(idx);
            return BlockMatching::MatchedContainer;
        }
        BlockMatching::Unmatched
    }
    fn process(ProcessCtx { line, .. }: ProcessCtx) -> BlockProcessing {
        if line.is_indented() {
            line.skip(line.indent_len());
            line.re_find_indent();
            BlockProcessing::Further
        } else if line.is_blank() {
            BlockProcessing::Further
        } else {
            BlockProcessing::Unprocessed
        }
    }
    fn after(id: usize, parser: &mut Parser) {
        parser.tree.unlink(id);
    }
}
