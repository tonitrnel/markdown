use crate::ast::{MarkdownNode, math};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};

fn closing_delimiter_offset(line: &crate::span::Span<'_>) -> Option<usize> {
    let remaining = line.as_str();
    let offset = remaining.rfind("$$")?;
    remaining[offset + 2..]
        .bytes()
        .all(|byte| matches!(byte, b' ' | b'\t'))
        .then_some(offset)
}

impl BlockStrategy for math::BlockMath {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        if parser.options.default_flavored || line.is_indented() {
            return BlockMatching::Unmatched;
        }

        let location = line.cursor_or_end() as u32;
        line.skip_indent();
        if !line.starts_with(b'$', 2) {
            return BlockMatching::Unmatched;
        }

        parser.close_unmatched_blocks();
        let id = parser.append_block(
            MarkdownNode::Math(Box::new(math::Math::Block(math::BlockMath {}))),
            location,
        );
        line.skip(2);

        if let Some(offset) = closing_delimiter_offset(line) {
            parser.append_inline(id, line.slice(0, offset));
            line.skip(offset + 2);
            parser.finalize(id, line.cursor_or_end() as u32);
            line.skip_to_end();
        } else {
            parser.append_inline(id, line.slice(0, line.len()));
            line.skip_to_end();
        }

        BlockMatching::MatchedLeaf
    }

    fn process(ProcessCtx { id, line, parser }: ProcessCtx) -> BlockProcessing {
        if let Some(offset) = closing_delimiter_offset(line) {
            parser.append_inline(id, line.slice(0, offset));
            line.skip(offset + 2);
            parser.finalize(id, line.cursor_or_end() as u32);
            line.skip_to_end();
            BlockProcessing::Processed
        } else {
            BlockProcessing::Further
        }
    }

    fn after(id: usize, parser: &mut crate::parser::Parser) {
        let Some(spans) = parser.inlines.remove(id) else {
            return;
        };
        let Some(first) = spans.first() else {
            return;
        };

        let start = first.cursor_or_end() as u32;
        let end = spans.last().map(|span| span.end() as u32).unwrap_or(start);
        let capacity =
            spans.iter().map(|span| span.len()).sum::<usize>() + spans.len().saturating_sub(1);
        let mut expression = String::with_capacity(capacity);
        for (index, span) in spans.iter().enumerate() {
            if index > 0 {
                expression.push('\n');
            }
            expression.push_str(span.as_str());
        }
        parser.append_text_to_owned_no_smart(id, expression, (start, end));
    }
}
