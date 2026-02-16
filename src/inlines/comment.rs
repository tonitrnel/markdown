use crate::inlines::ProcessCtx;

pub(super) fn process(ProcessCtx { line, .. }: &mut ProcessCtx) -> bool {
    // 从当前位置（第一个 %）之后开始搜索闭合 %%
    let mut i = 2;
    while let Some(b) = line.get(i) {
        if b == b'%' && line.get(i + 1) == Some(b'%') {
            line.skip(i + 2);
            return true;
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{Parser, ParserOptions},
        MarkdownNode,
    };

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
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Paragraph);
        assert_eq!(ast.to_html(), r##""##)
    }
}
