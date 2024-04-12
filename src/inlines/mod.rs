mod code;
mod math;
mod newline;

use crate::ast;
use crate::line::Line;
use crate::parser::Parser;
use crate::tokenizer::{Token, Whitespace};

pub(super) struct ProcessCtx<'a, 'input> {
    pub(super) id: usize,
    pub(super) parser: &'a mut Parser<'input>,
    pub(super) line: &'a mut Line<'input>,
}

pub(super) fn process<'input>(id: usize, parser: &mut Parser<'input>, mut line: Line<'input>) {
    println!("    ({})\"{:?}\"", line.len(), line);
    let mut ctx = ProcessCtx {
        id,
        parser,
        line: &mut line,
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
            // Token::Asterisk | Token::Underscore => {
            //     todo!()
            // }
            // // Image, Embed
            // Token::ExclamationMark => {
            //     todo!()
            // }
            // // Link
            // Token::LBracket => {
            //     todo!()
            // }
            // // AutoLinks, Raw HTML
            // Token::Lt => {
            //     todo!()
            // }
            // // Strikethrough(GFM)
            // Token::Tilde => {
            //     todo!()
            // }
            // // Highlight(OFM)
            // Token::Eq => {
            //     todo!()
            // }
            // // Math
            // Token::Dollar => {
            //     if ctx.line.validate(1, Token::Dollar) {
            //         ast::math::BlockMath::parse(&mut ctx)
            //     } else {
            //         ast::math::InlineMath::parse(&mut ctx)
            //     }
            // }
            _ => false,
        };
        if !handled {
            ctx.line.resume(snapshot);
            if let Some(it) = ctx.line.next_with_location(){
                ctx.parser.append_text_to(
                    ctx.id,
                    it.to_string(),
                    (it.start_location(), it.end_location()),
                );
            };
        }
    }
}
