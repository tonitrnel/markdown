use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::tokenizer::Token;

pub(super) fn process(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    if line.start_offset > 0
        && line
            .get_raw(line.start_offset - 1)
            .map(|it| !it.is_newline() && !it.is_space_or_tab())
            .unwrap_or(true)
    {
        return false;
    }
    line.next();
    let mut end = 0;
    let mut non_text = true;
    for (i, item) in line.iter().enumerate() {
        match &item.token {
            Token::Text(..) => {
                non_text = false;
                continue;
            }
            Token::Digit(..) | Token::Underscore | Token::Hyphen | Token::Slash => continue,
            _ => {
                end = i;
                break;
            }
        }
    }
    if end == 0 {
        end = line.len()
    }
    if end == 0 || non_text {
        return false;
    }
    let end_location = line[end - 1].end_location();
    let tag = line.slice(0, end).to_string();
    parser.tags.insert(tag.to_lowercase());
    parser.append_to(*id, MarkdownNode::Tag(tag), (start_location, end_location));
    line.skip(end);
    true
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn ofm_case_1() {
        let ast =
            Parser::new_with_options("#yymm1", ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(ast.to_html(), r##"<p><a href="#yymm1">#yymm1</a></p>"##)
    }
    #[test]
    fn ofm_case_2() {
        let texts = [
            "#camelCase",
            "#PascalCase",
            "#snake_case",
            "#kebab-case",
            "#inbox/to-read",
        ];
        let results = [
            r##"<p><a href="#camelCase">#camelCase</a></p>"##,
            r##"<p><a href="#PascalCase">#PascalCase</a></p>"##,
            r##"<p><a href="#snake_case">#snake_case</a></p>"##,
            r##"<p><a href="#kebab-case">#kebab-case</a></p>"##,
            r##"<p><a href="#inbox/to-read">#inbox/to-read</a></p>"##,
        ];
        for (i, text) in texts.iter().enumerate() {
            let ast =
                Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
            assert_eq!(ast.to_html(), results[i])
        }
    }
    #[test]
    fn ofm_case_3() {
        let ast = Parser::new_with_options("#泥嚎", ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast.to_html(),
            r##"<p><a href="#%E6%B3%A5%E5%9A%8E">#泥嚎</a></p>"##
        )
    }
}
