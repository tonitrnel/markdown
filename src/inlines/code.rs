use crate::ast::{code, MarkdownNode};
use crate::inlines::ProcessCtx;
use crate::tokenizer::Token;

impl code::InlineCode {
    pub(super) fn parse(ProcessCtx { line, parser, id, .. }: &mut ProcessCtx) -> bool {
        let start_offset = line.start_offset;
        let mut code_locations = (line.start_location(), line.end_location());
        let marker_length = line.starts_count(&Token::Backtick);
        line.skip(marker_length);
        let mut marker_count = 0;
        while let Some(next) = line.peek() {
            match next {
                Token::Backtick => {
                    marker_count += 1;
                    if marker_count == marker_length
                        && (line.is_end() || line.validate(1, |it: &Token| it != &Token::Backtick))
                    {
                        code_locations.1 = line[0].end_location();
                        line.next();
                        break;
                    }
                }
                _ => marker_count = 0,
            }
            line.next();
        }
        // 未闭合
        if marker_count != marker_length {
            return false
        }
        let parent = parser.append_block_to(
            *id,
            MarkdownNode::Code(code::Code::Inline(code::InlineCode {})),
            code_locations,
        );
        let (mut text, start, end) = {
            let part = line.slice_raw(
                start_offset + marker_length,
                line.end_offset - marker_length,
            );
            (
                part.to_string().replace('\n', " "),
                part.start_location(),
                part.last_token_end_location(),
            )
        };
        // 如果以空格开始和结束则剥离一个空格
        if text.starts_with(' ') && text.ends_with(' ') {
            // 空格仅占 1 个字节，因此不必转为 chars
            text = text[1..text.len() - 1].to_string()
        }
        parser.append_text_to(parent, text, (start, end));
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn case_328() {
        let text = r#"`foo`"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><code>foo</code></p>")
    }
    #[test]
    fn case_329() {
        let text = r#"`` foo ` bar ``"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><code>foo ` bar</code></p>")
    }
    #[test]
    fn case_3330() {
        let text = r#"` `` `"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><code>``</code></p>");
    }
    #[test]
    fn case_337(){
        let text = r#"`foo   bar 
baz`"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><code>foo   bar  baz</code></p>");
    }
}
