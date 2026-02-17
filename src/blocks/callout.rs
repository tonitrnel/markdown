use crate::ast;
use crate::ast::{MarkdownNode, callout};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};

impl BlockStrategy for callout::Callout {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        if !line.is_indented() && line.advance_next_nonspace().starts_with(b'>', 1) {
            // skip '>' byte
            line.next_byte();
            // optional following space
            line.consume_if(|b| b == b' ' || b == b'\t');
            if !line.consume(b'[') || !line.consume(b'!') {
                return BlockMatching::Unmatched;
            }
            let end = match line.position(|b| b == b']') {
                Some(end) if end > 0 => end,
                _ => return BlockMatching::Unmatched,
            };
            let _type = line.slice(0, end).trim().as_str().to_string();
            line.skip(end + 1);
            let foldable = match line.peek() {
                Some(b'+') => {
                    line.next_byte();
                    Some(true)
                }
                Some(b'-') => {
                    line.next_byte();
                    Some(false)
                }
                _ => None,
            };
            // optional following space
            line.consume_if(|b| b == b' ' || b == b'\t');
            let title = if !line.is_end() {
                let title = line.trim().as_str().to_string();
                line.skip_to_end();
                Some(title)
            } else {
                None
            };
            parser.close_unmatched_blocks();
            parser.append_block(
                MarkdownNode::Callout(Box::new(callout::Callout {
                    _type: callout::CalloutType::from(_type.as_str()),
                    title,
                    foldable,
                })),
                location,
            );
            return BlockMatching::MatchedContainer;
        }
        BlockMatching::Unmatched
    }
    fn process(ctx: ProcessCtx) -> BlockProcessing {
        ast::block_quote::BlockQuote::process(ctx)
    }
}
