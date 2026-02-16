use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;

pub(super) fn process(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    // 检查 '#' 前面的字符：必须是行首、换行或空白
    let cur = line.cursor();
    if cur > line.start() {
        let prev_byte = line.source_slice()[cur - 1];
        if prev_byte != b'\n' && prev_byte != b'\r' && prev_byte != b' ' && prev_byte != b'\t' {
            return false;
        }
    }
    // 跳过 '#'
    line.next_byte();
    let tag_start = line.cursor();
    let mut has_text = false;
    let mut i = line.cursor();
    while i < line.end() {
        let b = line.source_slice()[i];
        match b {
            b'a'..=b'z' | b'A'..=b'Z' => {
                has_text = true;
                i += 1;
            }
            b'0'..=b'9' | b'_' | b'-' | b'/' => {
                i += 1;
            }
            // 多字节 UTF-8 字符（非 ASCII 文本，如中文）
            0xC0..=0xFF => {
                has_text = true;
                // 确定 UTF-8 字符长度
                let char_len = if b < 0xE0 {
                    2
                } else if b < 0xF0 {
                    3
                } else {
                    4
                };
                i += char_len;
            }
            // continuation byte（不应该在这里出现，但安全处理）
            0x80..=0xBF => {
                i += 1;
            }
            _ => break,
        }
    }
    let tag_end = i.min(line.end());
    if tag_end == tag_start || !has_text {
        return false;
    }
    let tag = unsafe { std::str::from_utf8_unchecked(&line.source_slice()[tag_start..tag_end]) };
    let end_location = line.location_at_byte(tag_end);
    parser.tags.insert(tag.to_lowercase());
    parser.append_to(
        *id,
        MarkdownNode::Tag(tag.to_string()),
        (start_location, end_location),
    );
    // 移动 cursor 到 tag_end
    let skip_count = tag_end - line.cursor();
    line.skip(skip_count);
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
