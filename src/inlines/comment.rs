use crate::inlines::ProcessCtx;
use crate::tokenizer::Token;

pub(super) fn process(ProcessCtx { line, .. }: &mut ProcessCtx) -> bool {
    let end = line
        .iter()
        .skip(1)
        .position(|it| it.token == Token::DoublePercent)
        .unwrap_or(0);
    if end == 0 {
        return false;
    }
    line.skip(end + 2);
    true
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn ofm_case_1() {
        let ast = Parser::new_with_options(
            "This is an %%inline%% comment.",
            ParserOptions::default().enabled_ofm(),
        )
        .parse();
        assert_eq!(ast.to_html(), r##"<p>This is an  comment.</p>"##)
    }
    #[test]
    fn ofm_case_2() {
        let ast = Parser::new_with_options(
            "%%
This is a block comment.
Block comments can span multiple lines.
%%",
            ParserOptions::default().enabled_ofm(),
        )
        .parse();
        assert_eq!(ast.to_html(), r##"<p></p>"##)
    }
}
