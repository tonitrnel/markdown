use crate::ast::MarkdownNode;
use crate::blocks::{BlockMatching, BlockProcessing, BlockStrategy, Line};
use crate::parser::Parser;
use crate::tokenizer::Token;

/// BlockQuote example:
///
/// ```markdown
/// > #Foo
/// > bar
/// > baz
/// ```
pub struct BlockQuote {}

impl BlockStrategy for BlockQuote {
    fn before(parser: &mut Parser, line: &mut Line) -> BlockMatching {
        let location = line.location();
        if !line.is_indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
            // skip '>' token.
            line.next();
            // optional following space.
            line.consume(|it| it.is_space_or_tab());
            parser.close_unmatched_blocks();
            parser.append_block(MarkdownNode::BlockQuote, location);
            return BlockMatching::MatchedContainer;
        }
        BlockMatching::Unmatched
    }
    fn process(_parser: &mut Parser, line: &mut Line) -> BlockProcessing {
        if !line.is_indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
            // skip '>' token.
            line.next();
            // optional following space.
            line.consume(|it| it.is_space_or_tab());
            return BlockProcessing::Further;
        }
        BlockProcessing::Unprocessed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    #[test]
    fn it_works() {
        let text = r#"
> # Foo
> bar
> baz        
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::BlockQuote);
        assert_eq!(
            ast[2].body,
            MarkdownNode::Heading(ast::heading::Heading {
                variant: ast::heading::HeadingVariant::ATX,
                level: ast::heading::HeadingLevel::H1
            })
        );
        assert_eq!(ast[3].body, MarkdownNode::Paragraph);
        assert_eq!(ast.get_first_child(1), Some(2));
        assert_eq!(ast.get_last_child(1), Some(3));
    }
}
