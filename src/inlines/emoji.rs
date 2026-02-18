use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;

#[inline]
fn is_combining_mark(ch: char) -> bool {
    matches!(
        ch,
        '\u{0300}'..='\u{036F}'
            | '\u{1AB0}'..='\u{1AFF}'
            | '\u{1DC0}'..='\u{1DFF}'
            | '\u{20D0}'..='\u{20FF}'
            | '\u{FE20}'..='\u{FE2F}'
    )
}

#[inline]
fn is_emoji_name_char(ch: char) -> bool {
    ch.is_alphanumeric() || is_combining_mark(ch) || matches!(ch, '_' | '+' | '-')
}

pub(super) fn process(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    // 跳过开头的 ':'
    line.next_byte();
    // 扫描 emoji 名称：字母、数字、_、+、-，直到遇到 ':'
    let scan_start = line.cursor();
    let mut end_pos = None;
    let bytes = line.source_slice();
    let scan_slice = match std::str::from_utf8(&bytes[scan_start..line.end()]) {
        Ok(s) => s,
        Err(_) => return false,
    };
    for (offset, ch) in scan_slice.char_indices() {
        if ch == ':' {
            if offset == 0 {
                return false;
            }
            end_pos = Some(scan_start + offset);
            break;
        }
        if !is_emoji_name_char(ch) {
            return false;
        }
    }
    let end_pos = match end_pos {
        Some(p) => p,
        None => return false,
    };
    let emoji_name =
        unsafe { std::str::from_utf8_unchecked(&line.source_slice()[scan_start..end_pos]) };
    let end_location = line.location_at_byte(end_pos + 1);
    parser.append_to(
        *id,
        MarkdownNode::Emoji(emoji_name.to_string()),
        (start_location, end_location),
    );
    // 跳过 emoji 名称 + 结尾的 ':'
    // cursor 当前在 scan_start，需要跳到 end_pos + 1
    let skip_count = end_pos + 1 - line.cursor();
    line.skip(skip_count);
    true
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn supports_multilingual_emoji_names() {
        let ast = Parser::new_with_options(
            ":狗头: :привет: :مرحبا: :ñ:",
            ParserOptions::default().enabled_gfm(),
        )
        .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast.to_html(), "<p>:狗头: :привет: :مرحبا: :ñ:</p>");
    }

    #[test]
    fn rejects_symbol_inside_emoji_name() {
        let ast = Parser::new_with_options(":dog!: ok", ParserOptions::default().enabled_gfm())
            .parse();
        assert_eq!(ast.to_html(), "<p>:dog!: ok</p>");
    }

    #[test]
    fn default_mode_keeps_plain_text() {
        let ast = Parser::new(":狗头:").parse();
        assert_eq!(ast.to_html(), "<p>:狗头:</p>");
    }
}
