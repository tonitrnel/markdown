use crate::ast;
use crate::inlines::bracket::BracketChain;
use crate::inlines::delimiter::DelimiterChain;
use crate::line::Line;
use crate::parser::Parser;
use crate::tokenizer::{Token, Whitespace};
use std::collections::HashMap;

mod bracket;
mod code;
mod delimiter;
mod entity;
mod math;
mod newline;
mod link;

type RefMap = HashMap<String, (String, String)>;

struct ProcessCtx<'a, 'input> {
    id: usize,
    parser: &'a mut Parser<'input>,
    line: &'a mut Line<'input>,
    brackets: Option<BracketChain<'input>>,
    delimiters: Option<DelimiterChain<'input>>,
    ref_map: RefMap,
}

pub(super) fn process<'input>(id: usize, parser: &mut Parser<'input>, mut line: Line<'input>) {
    println!("    ({})\"{:?}\"", line.len(), line);
    let mut ctx = ProcessCtx {
        id,
        parser,
        line: &mut line,
        brackets: None,
        delimiters: None,
        ref_map: HashMap::new(),
    };
    while let Some(token) = ctx.line.peek() {
        let snapshot = ctx.line.snapshot();
        let handled = match token {
            // Hard break, Soft break
            Token::Whitespace(Whitespace::NewLine(..)) => newline::parse(&mut ctx),
            Token::Backslash => newline::parse_backslash(&mut ctx),
            // Code
            Token::Backtick => ast::code::InlineCode::parse(&mut ctx),
            // Emphasis, Strong emphasis
            Token::Asterisk | Token::Underscore => delimiter::before(&mut ctx, false, false, false),
            // Strikethrough(GFM)
            Token::Tilde => delimiter::before(&mut ctx, false, true, false),
            // Highlight(OFM)
            Token::Eq => delimiter::before(&mut ctx, false, false, true),
            // Link
            Token::LBracket => bracket::before(&mut ctx, false),
            // Wikilink(OFM)
            Token::DoubleLBracket => link::process_wikilink(&mut ctx),
            // Image, Embed(OFM)
            Token::ExclamationMark => match ctx.line.get(1) { 
                Some(Token::LBracket) => bracket::before(&mut ctx, true),
                Some(Token::DoubleLBracket) => link::process_embed(&mut ctx),
                _ => false
            },
            // Close
            Token::RBracket => bracket::process(&mut ctx),
            // Entity
            Token::Ampersand => entity::process(&mut ctx),
            // // AutoLinks, Raw HTML
            // Token::Lt => {
            //     todo!()
            // }
            // // Math
            Token::Dollar => {
                let is_block = ctx.line.validate(1, Token::Dollar);
                math::process(&mut ctx, is_block)
            }
            // Block id(OFM)
            Token::Caret => link::process_block_id(&mut ctx),
            _ => false,
        };
        if !handled {
            ctx.line.resume(snapshot);
            if let Some(it) = ctx.line.next_with_location() {
                ctx.parser.append_text_to(
                    ctx.id,
                    it.to_string(),
                    (it.start_location(), it.end_location()),
                );
            };
        }
    }
    delimiter::process(&mut ctx, 0);
}
