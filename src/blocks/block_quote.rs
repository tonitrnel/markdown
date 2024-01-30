use crate::ast::MarkdownNode;
use crate::blocks::{Line, BlockMatching, BlockProcessing};
use crate::parser::{Node, Parser};
use crate::tokenizer::Token;

pub fn initiate(parser: &mut Parser, line: &mut Line) -> BlockMatching {
    let location = line[0].location;
    if !line.indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
        if line[1].is_space_or_tab() {
            line.skip(1);
        }
        parser.interrupt_block();
        parser.add_block(MarkdownNode::BlockQuote, location);
        return BlockMatching::MatchedContainer;
    }
    BlockMatching::Unmatched
}

pub fn process(_parser: &mut Parser, line: &mut Line) -> BlockProcessing{
    if !line.indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
        if line[1].is_space_or_tab() {
            line.skip(1);
        }
        return BlockProcessing::Processed
    }
    BlockProcessing::Unprocessed
}
pub fn finalize(_parser: &mut Parser, _block: Node){}