#![allow(unused)]
use crate::ast::{MarkdownNode, math};
use crate::inlines::ProcessCtx;

pub(super) fn process(
    ProcessCtx {
        line, parser, id, ..
    }: &mut ProcessCtx,
    is_block: bool,
) -> bool {
    let start_location = line.start_location();
    let delimiter_len = if is_block { 2 } else { 1 };
    line.skip(delimiter_len);
    // 检查开头是否允许（非 block 时不能以空白开头）
    let allow_open = if is_block {
        true
    } else {
        line.peek()
            .map(|b| !b.is_ascii_whitespace())
            .unwrap_or(false)
    };
    if !allow_open {
        return false;
    }
    let expr_start_loc = line.start_location();
    let mut expression_bytes: Vec<u8> = Vec::new();
    let expr_end_loc = loop {
        let Some(current) = line.peek() else {
            return false;
        };
        if current == b'$' && (!is_block || line.validate(1, b'$')) {
            if !is_block
                && (expression_bytes.is_empty()
                    || expression_bytes
                        .last()
                        .is_some_and(|b| b.is_ascii_whitespace()))
            {
                return false;
            }
            break line.start_location();
        }
        if let Some(next) = line.next_byte() {
            expression_bytes.push(next);
        } else {
            return false;
        }
    };
    line.skip(delimiter_len);
    let end_location = line.start_location();
    let node = if is_block {
        parser.append_to(
            *id,
            MarkdownNode::Math(Box::new(math::Math::Block(math::BlockMath {}))),
            (start_location, end_location),
        )
    } else {
        parser.append_to(
            *id,
            MarkdownNode::Math(Box::new(math::Math::Inline(math::InlineMath {}))),
            (start_location, end_location),
        )
    };
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
    fn ext_case_1() {
        let text = r#"$
\begin{vmatrix}a & b\\
c & d
\end{vmatrix}=ad-bc
$"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}")
    }
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
