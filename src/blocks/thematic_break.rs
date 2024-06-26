use crate::ast::{thematic_break, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::tokenizer::Token;

impl BlockStrategy for thematic_break::ThematicBreak {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        if line.is_indented() {
            return BlockMatching::Unmatched;
        }
        let location = line.start_location();
        let marker = match line.skip_indent().next() {
            Some(Token::Hyphen) => Token::Hyphen,
            Some(Token::Underscore) => Token::Underscore,
            Some(Token::Asterisk) => Token::Asterisk,
            _ => return BlockMatching::Unmatched,
        };
        let mut len = 1;
        while let Some(next) = line.next() {
            if next == marker {
                len += 1;
                continue;
            } else if next.is_space_or_tab() {
                continue;
            }
            return BlockMatching::Unmatched;
        }
        if len < 3 {
            return BlockMatching::Unmatched;
        }
        parser.close_unmatched_blocks();
        parser.append_block(MarkdownNode::ThematicBreak, location);
        BlockMatching::MatchedLeaf
    }

    fn process(_ctx: ProcessCtx) -> BlockProcessing {
        BlockProcessing::Unprocessed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

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
