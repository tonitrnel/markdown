use crate::ast::{MarkdownNode, footnote};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::parser::Parser;
use crate::span::Span;
use crate::utils;

fn scan_footnote_start(line: &Span) -> Option<usize> {
    let mut pos = line.indent_len();
    if line.is_indented() || line.get(pos) != Some(b'[') || line.get(pos + 1) != Some(b'^') {
        return None;
    }

    pos += 2;
    let label_start = pos;
    while let Some(b) = line.get(pos) {
        match b {
            b']' if pos > label_start && line.get(pos + 1) == Some(b':') => {
                return Some(pos);
            }
            b'[' | b']' | b' ' | b'\t' | b'\n' | b'\r' => return None,
            _ => pos += 1,
        }
    }
    None
}

impl BlockStrategy for footnote::Footnote {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        if let Some(label_end) = scan_footnote_start(line) {
            let label = line
                .slice(line.indent_len() + 2, label_end)
                .as_str()
                .to_string();
            line.skip(label_end + 2);
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
    fn process(ProcessCtx { line, parser, .. }: ProcessCtx) -> BlockProcessing {
        if parser.options.obsidian_flavored {
            return if line.is_blank() {
                BlockProcessing::Unprocessed
            } else if scan_footnote_start(line).is_some() {
                BlockProcessing::Unprocessed
            } else {
                line.advance_next_nonspace();
                BlockProcessing::Further
            };
        }
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
