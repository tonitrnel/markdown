use crate::inlines::ProcessCtx;
use crate::tokenizer::Token;
use crate::utils;
use crate::utils::entities::EscapeState;

pub(super) fn process(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    line.next();
    let mut state = EscapeState::Entity;
    let mut start = 0;
    for (end, item) in line.iter().enumerate() {
        match (&state, &item.token) {
            (EscapeState::Entity, Token::Crosshatch) => {
                state = EscapeState::Numeric;
            }
            (EscapeState::Entity, Token::Text(..) | Token::Digit(..)) => {
                start = end;
                state = EscapeState::Named
            }
            (EscapeState::Numeric, Token::Text(str)) if str.starts_with(['x', 'X']) => {
                start = end;
                state = EscapeState::Hex;
            }
            (EscapeState::Numeric, Token::Digit(_)) => {
                start = end;
                state = EscapeState::Dec;
            }
            (EscapeState::Named, Token::Text(_) | Token::Digit(..)) => continue,
            (EscapeState::Named, Token::Semicolon) => {
                let buf = line.slice(start, end).to_string();
                if let Some(val) = utils::lookup_entity(buf.as_bytes()) {
                    parser.append_text_to(*id, val, (start_location, item.end_location()));
                    line.skip(end + 1);
                    return true;
                };
                return false;
            }
            (EscapeState::Hex, Token::Text(_) | Token::Digit(..)) => continue,
            (EscapeState::Hex, Token::Semicolon) => {
                let buf = line.slice(start, end).to_string();
                if let Ok(Some(mut ch)) = u32::from_str_radix(&buf[1..], 16).map(char::from_u32) {
                    ch = if ch == '\u{0000}' { '\u{FFFD}' } else { ch };
                    parser.append_text_to(
                        *id,
                        ch.to_string(),
                        (start_location, item.end_location()),
                    );
                    line.skip(end + 1);
                    return true;
                }
                return false;
            }
            (EscapeState::Dec, Token::Digit(..)) => continue,
            (EscapeState::Dec, Token::Semicolon) => {
                let buf = line.slice(start, end).to_string();
                if let Ok(Some(mut ch)) = buf.parse::<u32>().map(char::from_u32) {
                    ch = if ch == '\u{0000}' { '\u{FFFD}' } else { ch };
                    parser.append_text_to(
                        *id,
                        ch.to_string(),
                        (start_location, item.end_location()),
                    );
                    line.skip(end + 1);
                    return true;
                }
                return false;
            }
            _ => return false,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn case_25() {
        let text = r#"&nbsp; &amp; &copy; &AElig; &Dcaron;
&frac34; &HilbertSpace; &DifferentialD;
&ClockwiseContourIntegral; &ngE;"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(
            ast.to_html(),
            r#"<p>  &amp; © Æ Ď
¾ ℋ ⅆ
∲ ≧̸</p>"#
        )
    }
    #[test]
    fn case_26() {
        let text = r#"&#35; &#1234; &#992; &#0;"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p># Ӓ Ϡ �</p>"#)
    }
    #[test]
    fn case_27() {
        let text = r#"&#X22; &#XD06; &#xcab;"#;
        let ast = Parser::new(text).parse();
        // println!("{ast:?}")
        assert_eq!(ast.to_html(), r#"<p>&quot; ആ ಫ</p>"#)
    }
}
