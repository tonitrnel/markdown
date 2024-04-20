use crate::ast;
use crate::ast::{callout, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::tokenizer::Token;

impl BlockStrategy for callout::Callout {
    fn before(BeforeCtx { line, parser, .. }: BeforeCtx) -> BlockMatching {
        let location = line.start_location();
        if !line.is_indented() && line.advance_next_nonspace().starts_with(&Token::Gt, 1) {
            // skip '>' token.
            line.next();
            // optional following space.
            line.consume(|it: &Token| it.is_space_or_tab());
            if !line.consume(Token::LBracket) || !line.consume(Token::ExclamationMark) {
                return BlockMatching::Unmatched;
            }
            let end = match line.position(|it: &Token| it == &Token::RBracket) {
                Some(end) if end > 0 => end,
                _ => return BlockMatching::Unmatched,
            };
            let _type = line.slice(0, end).trim().to_string();
            line.skip(end + 1);
            let foldable =
                if line.validate(0, |it: &Token| matches!(it, Token::Plus | Token::Hyphen)) {
                    let next = line.next().unwrap();
                    Some(next == Token::Plus)
                } else {
                    None
                };
            // optional following space.
            line.consume(|it: &Token| it.is_space_or_tab());
            let title = if !line.is_end() {
                let title = line.trim().to_string();
                line.skip_to_end();
                Some(title)
            } else {
                None
            };
            parser.close_unmatched_blocks();
            parser.append_block(
                MarkdownNode::Callout(callout::Callout {
                    _type: callout::CalloutType::from(_type.as_str()),
                    title,
                    foldable,
                }),
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

#[cfg(test)]
mod tests{
    use crate::ast::{callout, MarkdownNode};
    use crate::parser::Parser;

    #[test]
    fn case_1(){
        let text = r#"> [!info]
> Here's a callout block.
> It supports **Markdown**, [[Internal link|Wikilinks]], and [[Embed files|embeds]]!
> ![[Engelbart.jpg]]"#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Info,
            title: None,
            foldable: None
        }));
        println!("{ast:?}")
    }
    #[test]
    fn case_2(){

        let text = r#"> [!tip] Callouts can have custom titles
> Like this one."#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Tip,
            title: Some("Callouts can have custom titles".to_string()),
            foldable: None
        }));
        println!("{ast:?}")
    }
    #[test]
    fn case_3(){

        let text = r#"> [!tip] Title-only callout
"#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Tip,
            title: Some("Title-only callout".to_string()),
            foldable: None
        }));
        assert_eq!(ast.len(), 2)
    }
    #[test]
    fn case_4(){

        let text = r#"> [!faq]- Are callouts foldable?
> Yes! In a foldable callout, the contents are hidden when the callout is collapsed."#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Question,
            title: Some("Are callouts foldable?".to_string()),
            foldable: Some(false)
        }));
        println!("{ast:?}")
    }
    #[test]
    fn case_5(){
        let text = r#"> [!question] Can callouts be nested?
> > [!todo] Yes!, they can.
> > > [!example]  You can even use multiple layers of nesting."#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Question,
            title: Some("Can callouts be nested?".to_string()),
            foldable: None
        }));
        assert_eq!(ast[2].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Todo,
            title: Some("Yes!, they can.".to_string()),
            foldable: None
        }));
        assert_eq!(ast[3].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Example,
            title: Some("You can even use multiple layers of nesting.".to_string()),
            foldable: None
        }));
        println!("{ast:?}")
    }
    #[test]
    fn case_6(){
        let text = r#"> [!custom-question-type]
> hello world!"#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Callout(callout::Callout{
            _type: callout::CalloutType::Custom("custom-question-type".to_string()),
            title: None,
            foldable: None
        }));
        println!("{ast:?}")
    }
}