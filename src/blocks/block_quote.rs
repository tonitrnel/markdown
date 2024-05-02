use crate::ast::{block_quote, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::tokenizer::Token;

impl BlockStrategy for block_quote::BlockQuote {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        if !line.is_indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
            // skip '>' token.
            line.next();
            // optional following space.
            line.consume(|it: &Token| it.is_space_or_tab());
            line.re_find_indent();
            parser.close_unmatched_blocks();
            parser.append_block(
                MarkdownNode::BlockQuote(block_quote::BlockQuote {}),
                location,
            );
            return BlockMatching::MatchedContainer;
        }
        BlockMatching::Unmatched
    }
    fn process(ctx: ProcessCtx) -> BlockProcessing {
        if !ctx.line.is_indented() && ctx.line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
            // skip '>' token.
            ctx.line.next();
            // optional following space.
            ctx.line.consume(|it: &Token| it.is_space_or_tab());
            ctx.line.re_find_indent();
            return BlockProcessing::Further;
        }
        BlockProcessing::Unprocessed
    }
}

#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::parser::Parser;

    use super::*;

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
        assert_eq!(
            ast[1].body,
            MarkdownNode::BlockQuote(block_quote::BlockQuote {})
        );
        assert_eq!(
            ast[2].body,
            MarkdownNode::Heading(ast::heading::Heading::ATX(ast::heading::ATXHeading {
                level: ast::heading::HeadingLevel::H1
            }))
        );
        assert_eq!(ast[3].body, MarkdownNode::Paragraph);
        assert_eq!(ast.get_first_child(1), Some(2));
        assert_eq!(ast.get_last_child(1), Some(3));
        println!("{ast:?}")
    }
}
