use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing, BlockStrategy, Line};
use crate::parser::Parser;
use crate::tokenizer::Token;

pub struct ThematicBreak {}

impl BlockStrategy for ThematicBreak {
    fn before<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching {
        if line.is_indented() {
            return BlockMatching::Unmatched;
        }
        let location = line.location();
        line.skip_indent();
        let marker = match line.next().map(|it| it.token) {
            Some(Token::Hyphen) => Token::Hyphen,
            Some(Token::Underscore) => Token::Underscore,
            Some(Token::Asterisk) => Token::Asterisk,
            _ => return BlockMatching::Unmatched,
        };
        while let Some(next) = line.next() {
            if next.token == marker || next.is_space_or_tab() {
                continue;
            }
            return BlockMatching::Unmatched;
        }
        parser.close_unmatched_blocks();
        parser.append_block(MarkdownNode::ThematicBreak, location);
        BlockMatching::MatchedLeaf
    }

    fn process<'input>(_parser: &mut Parser<'input>, _line: &mut Line<'input>) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = r#"
***
---
___
 **  * ** * ** * **
"#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast[2].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast[3].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast[4].body, MarkdownNode::ThematicBreak);
        assert_eq!(ast.get_next(1), Some(2));
        assert_eq!(ast.get_next(2), Some(3));
        assert_eq!(ast.get_next(3), Some(4));
        assert_eq!(ast.get_next(4), None);
    }
}
