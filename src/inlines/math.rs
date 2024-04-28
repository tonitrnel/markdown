#![allow(unused)]
use crate::ast::{math, MarkdownNode};
use crate::inlines::ProcessCtx;
use crate::tokenizer::{Token, Whitespace};

pub(super) fn process(
    ProcessCtx {
        line, parser, id, ..
    }: &mut ProcessCtx,
    is_block: bool,
) -> bool {
    let start_location = line.start_location();
    line.skip(if is_block { 2 } else { 1 });
    let allow_open = if is_block {
        true
    } else {
        line.validate(0, |it: &Token| !matches!(it, Token::Whitespace(..)))
    };
    if !allow_open {
        return false;
    };
    let mut iter = line.iter().enumerate();
    let (end, expression) = loop {
        if let Some((end, item)) = iter.next() {
            if Token::Dollar == item.token {
                if is_block && line.validate(end + 1, Token::Dollar) {
                    let mut _line = line.slice(0, end);
                    break (end + 2, _line);
                } else if !is_block {
                    let _line = line.slice(0, end);
                    break (end + 1, _line);
                }
            }
            continue;
        }
        return false;
    };
    let allow_close = if is_block {
        true
    } else {
        line.validate(end - 2, |it: &Token| !matches!(it, Token::Whitespace(..)))
    };
    if !allow_close {
        return false;
    };
    let text = line.slice(0, end);
    let end_location = line[end - 1].end_location();
    let node = if is_block {
        parser.append_to(
            *id,
            MarkdownNode::Math(math::Math::Block(math::BlockMath {})),
            (start_location, end_location),
        )
    } else {
        parser.append_to(
            *id,
            MarkdownNode::Math(math::Math::Inline(math::InlineMath {})),
            (start_location, end_location),
        )
    };
    line.skip(end);
    parser.append_text_to(
        node,
        expression.to_string(),
        (
            expression.start_location(),
            expression.last_token_end_location(),
        ),
    );
    true
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn ext_case_1() {
        let text = r#"$$
\begin{vmatrix}a & b\\
c & d
\end{vmatrix}=ad-bc
$$"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}")
    }
    #[test]
    fn ext_case_2() {
        let text = r#"This is an inline math expression $e^{2i\pi} = 1$."#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}")
    }
}
