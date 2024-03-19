use crate::blocks::{BlockMatching, BlockProcessing, BlockStrategy, Line};
use crate::parser::Parser;

pub struct Item {}

impl BlockStrategy for Item {
    fn before<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        BlockMatching::Unmatched
    }

    fn process<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}
