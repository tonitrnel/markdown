use crate::ast::{MarkdownNode, math};
use crate::inlines::ProcessCtx;

pub(super) fn process(
    ProcessCtx {
        line, parser, id, ..
    }: &mut ProcessCtx,
) -> bool {
    if line.get(1) == Some(b'$') {
        return false;
    }

    let start_location = line.cursor_or_end() as u32;
    line.skip(1);
    // Inline math cannot open before whitespace.
    let allow_open = line
        .peek()
        .map(|b| !b.is_ascii_whitespace())
        .unwrap_or(false);
    if !allow_open {
        return false;
    }
    let expr_start_loc = line.cursor_or_end() as u32;
    let mut expression_bytes: Vec<u8> = Vec::new();
    let expr_end_loc = loop {
        let Some(current) = line.peek() else {
            return false;
        };
        if current == b'$' {
            if expression_bytes.is_empty()
                || expression_bytes
                    .last()
                    .is_some_and(|b| b.is_ascii_whitespace())
            {
                return false;
            }
            break line.cursor_or_end() as u32;
        }
        if let Some(next) = line.next_byte() {
            expression_bytes.push(next);
        } else {
            return false;
        }
    };
    line.skip(1);
    let end_location = line.cursor_or_end() as u32;
    let node = parser.append_to(
        *id,
        MarkdownNode::Math(Box::new(math::Math::Inline(math::InlineMath {}))),
        (start_location, end_location),
    );
    let expression_str = match std::str::from_utf8(&expression_bytes) {
        Ok(v) => v,
        Err(_) => return false,
    };
    parser.append_text_to(node, expression_str, (expr_start_loc, expr_end_loc));
    true
}

#[cfg(test)]
mod tests {
    use crate::ParserOptions;
    use crate::parser::Parser;

    #[test]
    fn ext_case_2() {
        let text = r#"This is an inline math expression $e^{2i\pi} = 1$."#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}")
    }
    #[test]
    fn ext_case_3() {
        let text = r#"$\begin{cases} a = 1\\ b = 2 \end{cases}$"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        println!("{:?}", ast)
    }
}
