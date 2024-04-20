use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::tokenizer::{Token, Whitespace};

pub(super) fn parse(ProcessCtx { line, parser, id, .. }: &mut ProcessCtx) -> bool {
    if let Some((child_idx, MarkdownNode::Text(text))) = parser
        .tree
        .get_last_child(*id)
        .map(|idx| (idx, &mut parser.tree[idx].body))
    {
        if text.ends_with(' ') {
            let node = if text.ends_with("  ") {
                MarkdownNode::HardBreak
            } else {
                MarkdownNode::SoftBreak
            };
            let trimmed = text.trim_end().to_string();
            let offset = text.len() - trimmed.len();
            *text = trimmed;
            parser.tree[child_idx].end.column -= offset as u64;
            parser.append_block_to(*id, node, (line.start_location(), line.end_location()));
            line.next();
            return true;
        }
    }
    parser.append_block_to(
        *id,
        MarkdownNode::SoftBreak,
        (line.start_location(), line.end_location()),
    );
    line.next();
    true
}

pub(super) fn parse_backslash(ProcessCtx { line, parser, id, .. }: &mut ProcessCtx) ->bool{
    if line.validate(1, Token::Whitespace(Whitespace::NewLine("\n"))) {
        parser.append_block_to(
            *id,
            MarkdownNode::HardBreak,
            (line.start_location(), line[1].end_location()),
        );
        line.skip(2);
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn case_633() {
        let text = r#"foo  
baz"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
        assert_eq!(ast.to_html(), "<p>foo<br />baz</p>")
    }

    #[test]
    fn case_634() {
        let text = r#"foo\
baz"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
        assert_eq!(ast.to_html(), "<p>foo<br />baz</p>")
    }
}
