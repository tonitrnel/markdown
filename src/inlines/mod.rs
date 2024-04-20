use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::ast;
use crate::line::Line;
use crate::parser::Parser;
use crate::tokenizer::{Token, Whitespace};

mod code;
mod emphasis;
mod math;
mod newline;

#[derive(Clone)]
pub(super) struct DelimiterChain<'input>(Rc<RefCell<Delimiter<'input>>>);
impl<'a, 'input> DelimiterChain<'input> {
    fn new(delimiter: Delimiter<'input>) -> Self {
        Self(Rc::new(RefCell::new(delimiter)))
    }
    fn borrow(&'a self) -> Ref<'a, Delimiter<'input>> {
        self.0.borrow()
    }
    fn borrow_mut(&'a self) -> RefMut<'a, Delimiter<'input>> {
        self.0.borrow_mut()
    }
}

pub(super) struct ProcessCtx<'a, 'input> {
    pub(super) id: usize,
    pub(super) parser: &'a mut Parser<'input>,
    pub(super) line: &'a mut Line<'input>,
    #[allow(unused)]
    pub(super) brackets: VecDeque<u8>,
    pub(super) delimiters: Option<DelimiterChain<'input>>,
}
#[derive(Clone)]
struct Delimiter<'input> {
    token: Token<'input>,
    can_open: bool,
    can_close: bool,
    length: usize,
    prev: Option<DelimiterChain<'input>>,
    next: Option<DelimiterChain<'input>>,
    position: usize,
    node: usize,
}

impl Debug for DelimiterChain<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut count = 0;
        {
            let cur = self.borrow();
            writeln!(
                f,
                "  {count}. [{}]({},{})@{}#{}",
                cur.token, cur.can_open, cur.can_close, cur.length, cur.node
            )?;
        }
        let mut prev = self.borrow().prev.clone();
        while let Some(prev_delimiter) = prev {
            count += 1;
            {
                let prev = prev_delimiter.borrow();
                writeln!(
                    f,
                    "  {count}. [{}]({},{})@{}#{}",
                    prev.token, prev.can_open, prev.can_close, prev.length, prev.node
                )?;
            }
            let cloned = prev_delimiter.borrow().prev.clone();
            prev = cloned;
        }
        Ok(())
    }
}

pub(super) fn process<'input>(id: usize, parser: &mut Parser<'input>, mut line: Line<'input>) {
    println!("    ({})\"{:?}\"", line.len(), line);
    let mut ctx = ProcessCtx {
        id,
        parser,
        line: &mut line,
        brackets: VecDeque::new(),
        delimiters: None,
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
            Token::Asterisk | Token::Underscore => emphasis::before(&mut ctx, false, false, false),
            // Strikethrough(GFM)
            Token::Tilde => emphasis::before(&mut ctx, false, true, false),
            // Highlight(OFM)
            Token::Eq => emphasis::before(&mut ctx, false, false, true),
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
            if let Some(it) = ctx.line.next_with_location() {
                ctx.parser.append_text_to(
                    ctx.id,
                    it.to_string(),
                    (it.start_location(), it.end_location()),
                );
            };
        }
    }
    emphasis::process(&mut ctx, 0);
}
