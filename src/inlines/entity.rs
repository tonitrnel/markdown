use crate::inlines::ProcessCtx;
use crate::utils;

pub(super) fn process(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    // 跳过 '&'
    line.next_byte();

    let _scan_start = line.cursor();
    let bytes = line.source_slice();

    // 检查第一个字节确定类型
    let first = match line.peek() {
        Some(b) => b,
        None => return false,
    };

    if first == b'#' {
        // Numeric entity: &#... or &#x...
        line.next_byte();
        let hex = match line.peek() {
            Some(b'x') | Some(b'X') => {
                line.next_byte();
                true
            }
            Some(b'0'..=b'9') => false,
            _ => return false,
        };

        let num_start = line.cursor();
        while let Some(b) = line.peek() {
            match (hex, b) {
                (true, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') => {
                    line.next_byte();
                }
                (false, b'0'..=b'9') => {
                    line.next_byte();
                }
                (_, b';') => {
                    let num_end = line.cursor();
                    if num_end == num_start {
                        return false;
                    }
                    let num_str =
                        unsafe { std::str::from_utf8_unchecked(&bytes[num_start..num_end]) };
                    let end_location = line.location_at_byte(line.cursor() + 1);
                    line.next_byte(); // skip ';'

                    let result = if hex {
                        u32::from_str_radix(num_str, 16)
                            .ok()
                            .and_then(char::from_u32)
                    } else {
                        num_str.parse::<u32>().ok().and_then(char::from_u32)
                    };

                    if let Some(mut ch) = result {
                        if ch == '\u{0000}' {
                            ch = '\u{FFFD}';
                        }
                        parser.append_text_char_to(*id, ch, (start_location, end_location));
                        return true;
                    }
                    return false;
                }
                _ => return false,
            }
        }
        false
    } else if first.is_ascii_alphabetic() || first.is_ascii_digit() {
        // Named entity: &name;
        let name_start = line.cursor();
        while let Some(b) = line.peek() {
            match b {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                    line.next_byte();
                }
                b';' => {
                    let name_end = line.cursor();
                    if name_end == name_start {
                        return false;
                    }
                    let name = &bytes[name_start..name_end];
                    let end_location = line.location_at_byte(line.cursor() + 1);
                    line.next_byte(); // skip ';'

                    if let Some(val) = utils::lookup_entity(name) {
                        parser.append_text_to(*id, val, (start_location, end_location));
                        return true;
                    }
                    return false;
                }
                _ => return false,
            }
        }
        false
    } else {
        false
    }
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
        assert_eq!(
            ast.to_html(),
            "<p>\u{a0} &amp; © Æ Ď\n¾ ℋ ⅆ\n∲ ≧\u{338}</p>"
        )
    }
    #[test]
    fn case_26() {
        let text = r#"&#35; &#1234; &#992; &#0;"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p># Ӓ Ϡ �</p>"#)
    }
    #[test]
    fn case_27() {
        let text = r#"&#X22; &#XD06; &#xcab;"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p>&quot; ആ ಫ</p>"#)
    }
}
