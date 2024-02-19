use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessStage, BlockProcessing, Line};
use crate::parser::Parser;
use crate::tokenizer::Token;

pub struct BlockQuote {}

impl BlockProcessStage for BlockQuote {
    fn initiate(parser: &mut Parser, line: &mut Line) -> BlockMatching {
        let location = line[0].location;
        if !line.indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
            if line[1].is_space_or_tab() {
                line.skip(1);
            }
            parser.close_unmatched_blocks();
            parser.add_block(MarkdownNode::BlockQuote, location);
            return BlockMatching::MatchedContainer;
        }
        BlockMatching::Unmatched
    }
    fn process(_parser: &mut Parser, line: &mut Line) -> BlockProcessing {
        if !line.indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
            if line[1].is_space_or_tab() {
                line.skip(1);
            }
            return BlockProcessing::Processed;
        }
        BlockProcessing::Unprocessed
    }
}
