use crate::inlines::{bracket::BracketChain, delimiter::DelimiterChain};
use crate::line::Line;
use crate::parser::Parser;
use crate::tokenizer::{Token, Whitespace};

mod bracket;
mod code;
mod comment;
mod delimiter;
mod emoji;
mod entity;
mod footnote;
mod html;
mod link;
mod link_reference;
mod math;
mod newline;
mod tag;
mod text;

pub(crate) use footnote::process_footnote_list;
pub(crate) use link_reference::process_link_reference;

struct ProcessCtx<'a, 'input> {
    id: usize,
    parser: &'a mut Parser<'input>,
    line: &'a mut Line<'input>,
    brackets: Option<BracketChain<'input>>,
    delimiters: Option<DelimiterChain<'input>>,
}

pub(super) fn process<'input>(id: usize, parser: &mut Parser<'input>, mut line: Line<'input>) {
    // println!(
    //     "    ({})↩({})@{:?}\"{:?}\"",
    //     line.len(),
    //     parser.tree.get_parent(id),
    //     parser.tree[parser.tree.get_parent(id)].body,
    //     line
    // );
    let mut ctx = ProcessCtx {
        id,
        parser,
        line: &mut line,
        brackets: None,
        delimiters: None,
    };
    while let Some(token) = ctx.line.peek() {
        let snapshot = ctx.line.snapshot();
        let handled = match token {
            // Hard break, Soft break
            Token::Whitespace(Whitespace::NewLine(..)) => newline::process(&mut ctx),
            Token::Backslash => newline::process_backslash(&mut ctx),
            // Code
            Token::Backtick => code::process(&mut ctx),
            // Emphasis, Strong emphasis
            Token::Asterisk | Token::Underscore => delimiter::before(&mut ctx, false, false),
            // Strikethrough(GFM)
            Token::Tilde if ctx.parser.options.github_flavored => {
                delimiter::before(&mut ctx, true, false)
            }
            // Highlight(OFM)
            Token::Eq if ctx.parser.options.obsidian_flavored => {
                delimiter::before(&mut ctx, false, true)
            }
            // Link
            Token::LBracket => bracket::before(&mut ctx, false),
            // Wikilink(OFM)
            Token::DoubleLBracket if ctx.parser.options.obsidian_flavored => {
                link::process_wikilink(&mut ctx)
            }
            // Image, Embed(OFM)
            Token::ExclamationMark => match ctx.line.get(1) {
                Some(Token::LBracket) => bracket::before(&mut ctx, true),
                Some(Token::DoubleLBracket) if ctx.parser.options.obsidian_flavored => {
                    link::process_embed(&mut ctx)
                }
                _ => false,
            },
            // Close
            Token::RBracket => bracket::process(&mut ctx),
            // Entity
            Token::Ampersand => entity::process(&mut ctx),
            // // AutoLinks, Raw HTML
            Token::Lt => 'multi: {
                if link::process_autolink(&mut ctx) {
                    break 'multi true;
                }
                ctx.line.resume(&snapshot);
                if html::process(&mut ctx) {
                    break 'multi true;
                }
                false
            }
            // Math
            Token::Dollar if !ctx.parser.options.default_flavored => {
                let is_block = ctx.line.validate(1, Token::Dollar);
                math::process(&mut ctx, is_block)
            }
            // Block id(OFM)
            Token::Caret if ctx.parser.options.obsidian_flavored => {
                link::process_block_id(&mut ctx)
            }
            // Emoji
            Token::Colon if !ctx.parser.options.default_flavored => emoji::process(&mut ctx),
            // Tag
            Token::Crosshatch if ctx.parser.options.obsidian_flavored => tag::process(&mut ctx),
            // Comment
            Token::DoublePercent if ctx.parser.options.obsidian_flavored => {
                comment::process(&mut ctx)
            }
            // Token::Text(protocol @ "http")
            // | Token::Text(protocol @ "https")
            // | Token::Text(protocol @ "mailto")
            // | Token::Text(protocol @ "xmpp")
            //     if ctx.parser.options.github_flavored =>
            // {
            //     link::process_autolink_with_protocol(protocol, &mut ctx)
            // }
            // Token::Text(prefix @ "www") if ctx.parser.options.github_flavored => {
            //     link::process_autolink_with_prefix(prefix, &mut ctx)
            // }
            _ => false,
        };
        if !handled {
            ctx.line.resume(&snapshot);
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
    text::process(&mut ctx);
}
