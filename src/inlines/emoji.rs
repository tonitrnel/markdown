use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;

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
    let mut i = line.cursor();
    while i < line.end() {
        let b = line.source_slice()[i];
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'+' | b'-' => {
                i += 1;
            }
            b':' if i > scan_start => {
                end_pos = Some(i);
                break;
            }
            _ => return false,
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
    use crate::parser::Parser;

    #[test]
    fn it_works() {
        let ast =
            Parser::new("@octocat :+1: This PR looks great - it's ready to merge! :狗头:").parse();
        println!("{ast:?}")
    }
}
