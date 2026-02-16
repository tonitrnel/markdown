use crate::ast::{MarkdownNode, embed, link, reference::Reference};
use crate::inlines::ProcessCtx;
use crate::inlines::bracket::BracketChain;
use crate::span::{MergedSpan, Span};
use crate::utils;
use rustc_hash::FxHashMap;

/// 处理反斜杠转义：将 `\X` 替换为 `X`（X 为 ASCII 标点字符）
pub(super) fn backslash_unescape(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut result = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_punctuation() {
            result.push(bytes[i + 1] as char);
            i += 2;
        } else {
            // 处理 UTF-8 多字节字符
            let Some(ch) = s[i..].chars().next() else {
                break;
            };
            result.push(ch);
            i += ch.len_utf8();
        }
    }
    result
}

enum TitleState {
    Initial,
    InString(InString),
}
enum InString {
    Double,
    Single,
    Paren,
}

pub(crate) fn scan_link_or_image(
    line: &mut Span,
    opener: &BracketChain,
    ref_map: &FxHashMap<String, (String, Option<String>)>,
    footnotes: &FxHashMap<String, usize>,
) -> Option<(String, Option<String>, bool)> {
    let snapshot = line.snapshot();
    let mut url = None;
    let mut title = None;
    let mut matched = false;
    // 尝试解析 inline link
    'scan_link_url: {
        if !line.consume(b'(') {
            break 'scan_link_url;
        }
        skip_spaces(line);
        url = match scan_link_url(line) {
            Some((size, url_span)) => {
                line.skip(size);
                Some(utils::percent_encode::encode(
                    backslash_unescape(&utils::unescape_string(url_span.to_string())),
                    true,
                ))
            }
            _ => break 'scan_link_url,
        };
        title = {
            let count = skip_spaces(line);
            if count > 0 {
                match scan_link_title(line) {
                    Some((size, title_span)) => {
                        line.skip(size);
                        Some(backslash_unescape(&utils::entities::unescape_string(
                            title_span.to_string(),
                        )))
                    }
                    _ => None,
                }
            } else {
                None
            }
        };
        skip_spaces(line);
        if !line.consume(b')') {
            break 'scan_link_url;
        };
        matched = true;
    };
    let mut is_footnote_link = false;
    // 如果上一个块未匹配，尝试解析 link label
    'scan_link_label: {
        if matched {
            break 'scan_link_label;
        };
        line.resume(&snapshot);
        let ref_label = match scan_link_label(line) {
            Some((size, label)) => {
                line.skip(size);
                // If label is empty [], use the opener content as reference
                if label == "[]" {
                    let mut opener_idx = opener.borrow().index;
                    // For images, skip the '!' character
                    if opener.is_image() {
                        opener_idx += 1;
                    }
                    let cur = line.cursor() - size; // Before the []
                    let s = line.slice_from_abs(opener_idx, cur);
                    s.to_string()
                } else {
                    label
                }
            }
            None if !opener.borrow().bracket_after => {
                // 从 opener 的 index 到当前 cursor 的内容
                let mut opener_idx = opener.borrow().index;
                // For images, skip the '!' character
                if opener.is_image() {
                    opener_idx += 1;
                }
                let cur = line.cursor();
                let s = line.slice_from_abs(opener_idx, cur);
                s.to_string()
            }
            _ => break 'scan_link_label,
        };
        if ref_label.starts_with("[^") {
            let ref_label = &ref_label[2..ref_label.len() - 1];
            if footnotes.contains_key(ref_label) {
                url = Some(ref_label.to_string());
                matched = true;
                is_footnote_link = true;
            }
        } else {
            let ref_label = normalize_reference(ref_label);
            if let Some((_link, _title)) = ref_map.get(&ref_label) {
                url = Some(_link.clone());
                title = _title.clone();
                matched = true;
            }
        }
    }
    if !matched {
        return None;
    }
    url.map(|u| (u, title, is_footnote_link))
}

pub(super) fn scan_link_url<'input>(line: &Span<'input>) -> Option<(usize, Span<'input>)> {
    if line.validate(0, b'<') {
        // <url> 格式 - 尖括号内不允许反斜杠转义
        // 反斜杠被视为字面字符，不能用于转义 >
        let mut i = 1;
        loop {
            match line.get(i) {
                Some(b'>') => {
                    break;
                }
                Some(b'\\') => {
                    // 在尖括号URL中，反斜杠后必须跟着非 > 的字符
                    // 如果反斜杠后是 >，这是无效的（反斜杠不能转义 >）
                    if let Some(b'>') = line.get(i + 1) {
                        return None; // 无效：\> 不被允许
                    }
                    i += 1;
                }
                Some(b'\n') | Some(b'\r') | Some(b'<') | None => return None,
                Some(_) => i += 1,
            }
        }
        let url_span = line.slice(1, i);
        Some((i + 1, url_span))
    } else {
        // 普通 url 格式
        let mut i = 0;
        let mut p = 0; // 括号深度
        loop {
            match line.get(i) {
                Some(b'\\') => {
                    // 反斜杠转义：跳过下一个 ASCII 标点字符
                    if let Some(next) = line.get(i + 1) {
                        if next.is_ascii_punctuation() {
                            i += 2;
                        } else {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                Some(b'(') => {
                    i += 1;
                    p += 1;
                    if p > 32 {
                        return None;
                    }
                }
                Some(b')') => {
                    if p == 0 {
                        break;
                    }
                    p -= 1;
                    i += 1;
                }
                Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => break,
                Some(b) if b < 0x20 => break, // control chars
                None => break,
                Some(_) => i += 1,
            }
        }
        if p != 0 {
            return None;
        }
        let url_span = line.slice(0, i);
        Some((i, url_span))
    }
}

pub(super) fn scan_link_title<'input>(line: &Span<'input>) -> Option<(usize, Span<'input>)> {
    let mut i = 0;
    let mut state = TitleState::Initial;
    loop {
        let b = match line.get(i) {
            Some(b) => b,
            None => break,
        };
        i += 1;
        match (&state, b) {
            (TitleState::Initial, b'"') => state = TitleState::InString(InString::Double),
            (TitleState::Initial, b'\'') => state = TitleState::InString(InString::Single),
            (TitleState::Initial, b'(') => state = TitleState::InString(InString::Paren),
            (TitleState::InString(_), b'\\') => {
                if line.get(i).is_some() {
                    i += 1;
                }
            }
            (TitleState::InString(InString::Double), b'"')
            | (TitleState::InString(InString::Single), b'\'')
            | (TitleState::InString(InString::Paren), b')') => {
                state = TitleState::Initial;
                break;
            }
            (TitleState::InString(_), _) => (),
            _ => return None,
        }
    }
    if !matches!(state, TitleState::Initial) {
        return None;
    }
    Some((i, line.slice(1, i.saturating_sub(1))))
}

/// 扫描链接的 Label，用于 Reference
pub(super) fn scan_link_label(line: &Span) -> Option<(usize, String)> {
    if line.get(0) != Some(b'[') {
        return None;
    }
    let mut end = 0;
    let mut i = 1;
    loop {
        match line.get(i) {
            Some(b']') => {
                end = i;
                break;
            }
            Some(b'[') => return None,
            Some(b'\\') => {
                // 反斜杠转义：跳过下一个字符
                i += 1;
                if let Some(next) = line.get(i) {
                    let char_len = if next < 0x80 {
                        1
                    } else if next < 0xE0 {
                        2
                    } else if next < 0xF0 {
                        3
                    } else {
                        4
                    };
                    i += char_len;
                }
                if i > 1001 {
                    return None;
                }
            }
            None => break,
            Some(b) => {
                // 计算字符长度
                let char_len = if b < 0x80 {
                    1
                } else if b < 0xE0 {
                    2
                } else if b < 0xF0 {
                    3
                } else {
                    4
                };
                i += char_len;
                if i > 1001 {
                    return None;
                }
            }
        }
    }
    if end == 0 {
        return None;
    }
    // Allow empty labels like [] for collapsed reference links
    Some((end + 1, line.slice(0, end + 1).to_string()))
}

/// 跳过空白字符（空格、制表符、换行），返回跳过的数量
pub(super) fn skip_spaces(line: &mut Span) -> usize {
    let count = line.starts_count_matches(|b: u8| matches!(b, b' ' | b'\t' | b'\n' | b'\r'));
    if count > 0 {
        line.skip(count);
    }
    count
}

pub(super) fn normalize_reference(str: String) -> String {
    // CommonMark requires Unicode case-folding
    // The German ẞ (U+1E9E) should fold to "ss", not "ß"
    // We also normalize all whitespace runs to a single space.
    let trimmed = str.trim();
    let content = if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };
    let content = content.trim();

    let mut normalized = String::with_capacity(content.len());
    let mut pending_space = false;
    for ch in content.chars() {
        if ch.is_whitespace() {
            pending_space = !normalized.is_empty();
            continue;
        }
        if pending_space {
            normalized.push(' ');
            pending_space = false;
        }
        if ch == 'ẞ' {
            normalized.push('s');
            normalized.push('s');
        } else {
            normalized.extend(ch.to_lowercase());
        }
    }
    normalized
}

/// Block Id (OFM)
pub(super) fn process_block_id(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    // 跳过 '^'
    line.next_byte();
    let start = line.cursor();
    let mut len = 0usize;
    while let Some(b) = line.get(len) {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' => len += 1,
            _ => break,
        }
    }
    if len == 0 {
        return false;
    }

    // 允许行尾空格/制表符；若后续出现其它字符则不是合法 block-id 行
    let mut skip_count = len;
    while let Some(b) = line.get(skip_count) {
        match b {
            b' ' | b'\t' => skip_count += 1,
            b'\n' | b'\r' => break,
            _ => return false,
        }
    }

    let end = start + len;
    let block_id = unsafe { std::str::from_utf8_unchecked(&line.source_slice()[start..end]) };
    parser.tree[*id].id = Some(Box::new(block_id.to_string()));
    line.skip(skip_count);
    true
}

#[derive(Debug)]
enum WikilinkState {
    Initial,
    InPath,
    InText,
    InRef(InRef),
}
#[derive(Debug)]
enum InRef {
    Ref,
    RefHeading(usize),
    RefBlock,
}

pub(super) fn process_wikilink(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    // 跳过 '[['
    line.next_byte(); // 第一个 '['
    line.next_byte(); // 第二个 '['
    // 现在 cursor 指向内容开始
    let content_start = line.cursor();
    let bytes = line.source_slice();
    // 找到 ']]' 的位置
    let mut _parts = Vec::<(usize, usize)>::new(); // 收集各部分的字节范围
    let mut state = WikilinkState::Initial;
    let mut pr = (0usize, 0usize); // path range (relative to content_start)
    let mut rrs: (bool, Vec<(usize, usize)>) = (false, Vec::new());
    let mut tr = (0usize, 0usize); // text range

    let mut i = content_start;
    let mut rel = 0; // relative position
    while i + 1 < line.end() {
        let b = bytes[i];
        let b_next = bytes[i + 1];
        let is_double_rbracket = b == b']' && b_next == b']';
        match (&state, b, is_double_rbracket) {
            (WikilinkState::Initial, _, true) => return false,
            (WikilinkState::Initial, _, _) => {
                state = WikilinkState::InPath;
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (WikilinkState::InPath, b'|', _) => {
                pr.1 = rel;
                tr.0 = rel + 1;
                state = WikilinkState::InText;
            }
            (WikilinkState::InPath, b'#', _) => {
                pr.1 = rel;
                state = WikilinkState::InRef(InRef::Ref);
            }
            (WikilinkState::InPath, _, true) => {
                pr.1 = rel;
                state = WikilinkState::Initial;
                i += 2;
                break;
            }
            (WikilinkState::InPath, _, _) => {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (WikilinkState::InText, _, true) => {
                tr.1 = rel;
                state = WikilinkState::Initial;
                i += 2;
                break;
            }
            (WikilinkState::InText, _, _) => {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (WikilinkState::InRef(InRef::Ref), b'^', _) => {
                rrs.0 = true;
                rrs.1.push((rel + 1, rel + 1));
                state = WikilinkState::InRef(InRef::RefBlock);
            }
            (WikilinkState::InRef(InRef::Ref), _, _) if b.is_ascii_alphanumeric() || b >= 0x80 => {
                rrs.0 = false;
                rrs.1.push((rel, rel));
                state = WikilinkState::InRef(InRef::RefHeading(0));
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (WikilinkState::InRef(InRef::RefBlock), _, true) => {
                rrs.1[0].1 = rel;
                state = WikilinkState::Initial;
                i += 2;
                break;
            }
            (WikilinkState::InRef(InRef::RefBlock), b'|', _) => {
                rrs.1[0].1 = rel;
                tr.0 = rel + 1;
                state = WikilinkState::InText;
            }
            (WikilinkState::InRef(InRef::RefBlock), _, _)
                if b.is_ascii_alphanumeric() || b == b'-' || b >= 0x80 =>
            {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (WikilinkState::InRef(InRef::RefHeading(index)), _, true) => {
                rrs.1[*index].1 = rel;
                state = WikilinkState::Initial;
                i += 2;
                break;
            }
            (WikilinkState::InRef(InRef::RefHeading(index)), b'|', _) => {
                rrs.1[*index].1 = rel;
                tr.0 = rel + 1;
                state = WikilinkState::InText;
            }
            (WikilinkState::InRef(InRef::RefHeading(index)), b'#', _) => {
                rrs.1[*index].1 = rel;
                rrs.1.push((rel + 1, rel + 1));
                state = WikilinkState::InRef(InRef::RefHeading(index + 1));
            }
            (WikilinkState::InRef(InRef::RefHeading(_)), _, _) => {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            _ => return false,
        }
        i += utf8_char_len(b);
        rel += 1;
    }
    // 处理最后一个字节是 ']]' 的情况
    if !matches!(state, WikilinkState::Initial) {
        // 检查最后是否是 ]]
        if i + 1 == line.end() && bytes[i] == b']' {
            // 只有一个 ']'，不匹配
            return false;
        }
        return false;
    }
    if pr.0 == pr.1 {
        return false;
    }
    // 使用 content_start 为基准，通过 slice 提取各部分
    // 注意：这里的 rel 索引是基于字符的，但 Span.slice 是基于字节的
    // 我们需要用字节偏移。重新用字节方式提取。
    let _content_bytes = &bytes[content_start..i];
    let path = extract_range_str(bytes, content_start, pr);
    let reference = extract_ref_from_bytes(bytes, content_start, &rrs);
    let text = if tr.0 != tr.1 {
        Some(extract_range_str(bytes, content_start, tr))
    } else {
        None
    };
    let end_location = line.location_at_byte(i);
    let skip_count = i - line.cursor();
    line.skip(skip_count);
    parser.append_to(
        *id,
        MarkdownNode::Link(Box::new(
            link::Wikilink {
                path,
                reference,
                text,
            }
            .into(),
        )),
        (start_location, end_location),
    );
    true
}

/// 从字节范围提取字符串（范围是基于"token 索引"的，这里我们用字节扫描重新实现）
/// 注意：wikilink 中的 rel 索引实际上是按字符计数的，需要转换为字节偏移
fn extract_range_str(source: &[u8], base: usize, range: (usize, usize)) -> String {
    // range 是字符索引，需要转换为字节偏移
    let mut byte_start = base;
    let mut char_count = 0;
    while char_count < range.0 && byte_start < source.len() {
        byte_start += utf8_char_len(source[byte_start]);
        char_count += 1;
    }
    let mut byte_end = byte_start;
    while char_count < range.1 && byte_end < source.len() {
        byte_end += utf8_char_len(source[byte_end]);
        char_count += 1;
    }
    unsafe { std::str::from_utf8_unchecked(&source[byte_start..byte_end]) }.to_string()
}

fn extract_ref_from_bytes(
    source: &[u8],
    base: usize,
    range: &(bool, Vec<(usize, usize)>),
) -> Option<Reference> {
    if !range.1.is_empty() {
        Some(if range.0 {
            let value = extract_range_str(source, base, range.1[0]);
            Reference::BlockId(value)
        } else if range.1.len() == 1 {
            Reference::Heading(extract_range_str(source, base, range.1[0]))
        } else {
            Reference::MultiHeading(
                range
                    .1
                    .iter()
                    .map(|r| {
                        if r.0 == r.1 {
                            String::new()
                        } else {
                            extract_range_str(source, base, *r)
                        }
                    })
                    .collect(),
            )
        })
    } else {
        None
    }
}

fn utf8_char_len(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b < 0xE0 {
        2
    } else if b < 0xF0 {
        3
    } else {
        4
    }
}

#[derive(Debug)]
enum EmbedState {
    Initial,
    InPath,
    InRef(InRef),
    InAttr(usize),
    InSize(InSize),
}
#[derive(Debug)]
enum InSize {
    Width,
    Height,
}

pub(super) fn process_embed(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    // 跳过 '![[' 三个字节
    line.skip(3);
    let content_start = line.cursor();
    let bytes = line.source_slice();
    let mut state = EmbedState::Initial;
    let mut pr = (0usize, 0usize);
    let mut sr = (0usize, 0usize);
    let mut rrs: (bool, Vec<(usize, usize)>) = (false, Vec::new());
    let mut ars: Vec<(usize, usize)> = Vec::new();

    let mut i = content_start;
    let mut rel = 0;
    while i + 1 < line.end() {
        let b = bytes[i];
        let b_next = bytes[i + 1];
        let is_double_rbracket = b == b']' && b_next == b']';
        match (&state, b, is_double_rbracket) {
            (EmbedState::Initial, _, true) => return false,
            (EmbedState::Initial, _, _) => {
                state = EmbedState::InPath;
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (EmbedState::InPath, b'#', _) => {
                pr.1 = rel;
                state = EmbedState::InRef(InRef::Ref);
            }
            (EmbedState::InPath, b'|', _) => {
                pr.1 = rel;
                sr.0 = rel + 1;
                state = EmbedState::InSize(InSize::Width);
            }
            (EmbedState::InPath, _, true) => {
                pr.1 = rel;
                state = EmbedState::Initial;
                i += 2;
                break;
            }
            (EmbedState::InPath, _, _) => {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (EmbedState::InSize(InSize::Width), b'x', _) => {
                state = EmbedState::InSize(InSize::Height);
            }
            (EmbedState::InSize(_), _, true) => {
                sr.1 = rel;
                state = EmbedState::Initial;
                i += 2;
                break;
            }
            (EmbedState::InSize(InSize::Width), b'0'..=b'9', _) => {
                i += 1;
                rel += 1;
                continue;
            }
            (EmbedState::InSize(InSize::Height), b'0'..=b'9', _) => {
                i += 1;
                rel += 1;
                continue;
            }
            (EmbedState::InRef(InRef::Ref), b'^', _) => {
                rrs.0 = true;
                rrs.1.push((rel + 1, rel + 1));
                state = EmbedState::InRef(InRef::RefBlock);
            }
            (EmbedState::InRef(InRef::Ref), _, _) if b.is_ascii_alphanumeric() || b >= 0x80 => {
                rrs.0 = false;
                rrs.1.push((rel, rel));
                state = EmbedState::InRef(InRef::RefHeading(0));
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (EmbedState::InRef(InRef::RefBlock), _, true) => {
                rrs.1[0].1 = rel;
                state = EmbedState::Initial;
                i += 2;
                break;
            }
            (EmbedState::InRef(InRef::RefBlock), _, _)
                if b.is_ascii_alphanumeric() || b == b'-' || b >= 0x80 =>
            {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (EmbedState::InRef(InRef::RefHeading(index)), b'#', _) => {
                rrs.1[*index].1 = rel;
                rrs.1.push((rel + 1, rel + 1));
                state = EmbedState::InRef(InRef::RefHeading(index + 1));
            }
            (EmbedState::InRef(InRef::RefHeading(index)), b'=', _) => {
                rrs.1[*index].1 = rel;
                ars.push(rrs.1[*index]);
                rrs.1.pop();
                state = EmbedState::InAttr(0);
            }
            (EmbedState::InRef(InRef::RefHeading(index)), _, true) => {
                rrs.1[*index].1 = rel;
                state = EmbedState::Initial;
                i += 2;
                break;
            }
            (EmbedState::InRef(InRef::RefHeading(_)), _, _) => {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            (EmbedState::InAttr(index), b'&', _) => {
                ars[*index].1 = rel;
                ars.push((rel + 1, rel + 1));
                state = EmbedState::InAttr(index + 1);
            }
            (EmbedState::InAttr(index), _, true) => {
                ars[*index].1 = rel;
                state = EmbedState::Initial;
                i += 2;
                break;
            }
            (EmbedState::InAttr(_), _, _) => {
                i += utf8_char_len(b);
                rel += 1;
                continue;
            }
            _ => return false,
        }
        i += utf8_char_len(b);
        rel += 1;
    }
    if !matches!(state, EmbedState::Initial) || pr.0 == pr.1 {
        return false;
    }
    let path = extract_range_str(bytes, content_start, pr);
    let reference = extract_ref_from_bytes(bytes, content_start, &rrs);
    let attrs = if !ars.is_empty() {
        Some(
            ars.iter()
                .map(|r| {
                    if r.0 == r.1 {
                        String::new()
                    } else {
                        extract_range_str(bytes, content_start, *r)
                    }
                })
                .filter_map(|it| {
                    let mut parts = it.split('=');
                    match (parts.next(), parts.next()) {
                        (Some(a), Some(b)) => Some((a.to_string(), b.to_string())),
                        (Some(a), None) => Some((a.to_string(), String::new())),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>(),
        )
    } else {
        None
    };
    let size = if sr.0 != sr.1 {
        let size_str = extract_range_str(bytes, content_start, sr);
        let mut parts = size_str.split('x');
        let first = parts.next().and_then(|it| it.parse::<u32>().ok());
        let second = parts.next().and_then(|it| it.parse::<u32>().ok());
        match (first, second) {
            (Some(a), Some(b)) => Some((a, Some(b))),
            (Some(a), None) => Some((a, None)),
            _ => None,
        }
    } else {
        None
    };
    let end_location = line.location_at_byte(i);
    let skip_count = i - line.cursor();
    line.skip(skip_count);
    parser.append_to(
        *id,
        MarkdownNode::Embed(Box::new(embed::Embed {
            path,
            size,
            reference,
            attrs,
        })),
        (start_location, end_location),
    );
    true
}

pub(super) fn process_autolink(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let current_span = match line.current_span_mut() {
        Some(span) => span,
        None => return false,
    };
    let start_location = current_span.start_location();
    current_span.next_byte(); // skip '<'
    if let Some(end) = scan_email(current_span) {
        let link_span = current_span.slice(0, end);
        let end_location = current_span.location_at_byte(current_span.cursor() + end + 1);
        current_span.skip(end + 1); // skip content + '>'
        let link_str = link_span.to_string();
        let node = parser.append_to(
            *id,
            MarkdownNode::Link(Box::new(
                link::DefaultLink {
                    url: format!("mailto:{}", link_str),
                    title: None,
                }
                .into(),
            )),
            (start_location, end_location),
        );
        let locations = (
            link_span.start_location(),
            link_span.last_token_end_location(),
        );
        parser.append_text_to_owned(node, link_str, locations);
        true
    } else if let Some((end, escaped_esc)) = scan_url(current_span) {
        let link_span = current_span.slice(0, end);
        let skip_amount = if escaped_esc { end + 2 } else { end + 1 }; // escaped_esc: skip '\' + '>'
        let end_location = current_span.location_at_byte(current_span.cursor() + skip_amount);
        let mut unescaped_string = link_span.to_unescape_string();
        if escaped_esc {
            unescaped_string.push('\\')
        }
        current_span.skip(skip_amount); // skip content + '>' (or '\>')
        let node = parser.append_to(
            *id,
            MarkdownNode::Link(Box::new(
                link::DefaultLink {
                    url: utils::percent_encode::encode(&unescaped_string, true),
                    title: None,
                }
                .into(),
            )),
            (start_location, end_location),
        );
        let mut locations = (
            link_span.start_location(),
            link_span.last_token_end_location(),
        );
        if escaped_esc {
            locations.1.column += 1;
        }
        parser.append_text_to_owned(node, unescaped_string, locations);
        true
    } else {
        false
    }
}

pub(super) fn process_gfm_autolink(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    let Some((end, needs_http_prefix)) = scan_gfm_url(line) else {
        return false;
    };
    let end_location = line.location_at_byte(line.cursor() + end);
    let text = extract_bytes_from_merged(line, end);
    line.skip(end);
    let mut url = String::new();
    if needs_http_prefix {
        url.push_str("http://");
    }
    url.push_str(&text);
    let node = parser.append_to(
        *id,
        MarkdownNode::Link(Box::new(
            link::DefaultLink {
                url: utils::percent_encode::encode(&url, true),
                title: None,
            }
            .into(),
        )),
        (start_location, end_location),
    );
    parser.append_text_to_owned(node, text, (start_location, end_location));
    true
}

fn scan_gfm_url(line: &MergedSpan) -> Option<(usize, bool)> {
    let (prefix_len, needs_http_prefix) = if starts_with_ci(line, b"https://") {
        (8usize, false)
    } else if starts_with_ci(line, b"http://") {
        (7usize, false)
    } else if starts_with_ci(line, b"www.") {
        (4usize, true)
    } else {
        return None;
    };
    let next = line.get(prefix_len)?;
    if next.is_ascii_control() || next.is_ascii_whitespace() {
        return None;
    }

    let mut i = prefix_len;
    while let Some(b) = line.get(i) {
        if b.is_ascii_control() || b.is_ascii_whitespace() || b == b'<' {
            break;
        }
        i += utf8_char_len(b);
    }
    if i <= prefix_len {
        return None;
    }
    let trimmed = trim_gfm_url_end(line, prefix_len, i);
    if trimmed <= prefix_len {
        return None;
    }
    Some((trimmed, needs_http_prefix))
}

fn starts_with_ci(line: &MergedSpan, prefix: &[u8]) -> bool {
    for (i, &want) in prefix.iter().enumerate() {
        let Some(got) = line.get(i) else {
            return false;
        };
        if got.to_ascii_lowercase() != want {
            return false;
        }
    }
    true
}

fn trim_gfm_url_end(line: &MergedSpan, start: usize, mut end: usize) -> usize {
    while end > start {
        let Some(last) = line.get(end - 1) else {
            break;
        };
        let trim = matches!(
            last,
            b'.' | b',' | b'?' | b'!' | b':' | b'*' | b'_' | b'~' | b'\'' | b'"'
        ) || (last == b')' && has_extra_closing_paren(line, start, end))
            || (last == b';' && ends_with_html_entity(line, start, end));
        if !trim {
            break;
        }
        end -= 1;
    }
    end
}

fn has_extra_closing_paren(line: &MergedSpan, start: usize, end: usize) -> bool {
    let mut open = 0usize;
    let mut close = 0usize;
    for i in start..end {
        match line.get(i) {
            Some(b'(') => open += 1,
            Some(b')') => close += 1,
            _ => {}
        }
    }
    close > open
}

fn ends_with_html_entity(line: &MergedSpan, start: usize, end: usize) -> bool {
    if end <= start + 1 {
        return false;
    }
    let mut i = end - 2; // skip ';'
    while i > start {
        let Some(b) = line.get(i) else {
            return false;
        };
        if b == b'&' {
            let entity_start = i + 1;
            if entity_start >= end - 1 {
                return false;
            }
            for j in entity_start..(end - 1) {
                let Some(ch) = line.get(j) else {
                    return false;
                };
                if !(ch.is_ascii_alphanumeric() || ch == b'#') {
                    return false;
                }
            }
            return true;
        }
        if !(b.is_ascii_alphanumeric() || b == b'#') {
            return false;
        }
        i -= 1;
    }
    false
}

fn extract_bytes_from_merged(line: &MergedSpan, len: usize) -> String {
    if let Some(span) = line.current_span() {
        let start = span.cursor();
        let end = start.saturating_add(len);
        if end <= span.end() {
            let source = span.source_slice();
            let slice = unsafe { std::str::from_utf8_unchecked(&source[start..end]) };
            return slice.to_owned();
        }
    }

    let mut bytes = Vec::with_capacity(len);
    for i in 0..len {
        let Some(b) = line.get(i) else {
            break;
        };
        bytes.push(b);
    }
    String::from_utf8(bytes).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned())
}

fn scan_url(line: &Span) -> Option<(usize, bool)> {
    let mut i = 0;
    // let mut escaped_esc = false;
    let mut len = 0;
    // 第一阶段：扫描 scheme（字母开头，后跟字母/数字/+/./-)
    loop {
        match line.get(i) {
            Some(b) if b.is_ascii_alphabetic() && len == 0 => {
                len += 1;
                i += 1;
            }
            Some(b)
                if (b.is_ascii_alphanumeric() || b == b'+' || b == b'.' || b == b'-')
                    && len > 0 =>
            {
                len += 1;
                i += 1;
            }
            Some(b':') if (2..32).contains(&len) => {
                i += 1;
                break;
            }
            _ => return None,
        }
    }
    // 第二阶段：扫描 URL 内容直到 '>'
    loop {
        match line.get(i) {
            Some(b'>') => {
                return Some((i, false));
            }
            Some(b'\\') => {
                // 检查下一个字节
                if line.get(i + 1) == Some(b'>') {
                    // escaped_esc = true;
                    return Some((i, true));
                }
                if line.get(i + 1) == Some(b'<') {
                    return None;
                }
                i += 1;
            }
            Some(b'<') => return None,
            Some(b' ') => return None, // 空格不允许出现在 autolink URL 中
            Some(b) if b < 0x20 => return None, // control chars
            None => return None,
            Some(_) => i += 1,
        }
    }
}

#[derive(Debug)]
enum EmailState {
    Initial,
    Username,
    At,
    Domain(usize),
    Tld(usize),
}

fn scan_email(line: &Span) -> Option<usize> {
    let mut state = EmailState::Initial;
    let mut len = 0;
    let mut i = 0;
    loop {
        let b = match line.get(i) {
            Some(b) => b,
            None => break,
        };
        match (&state, b) {
            (EmailState::Initial, b) if b.is_ascii_alphanumeric() => {
                state = EmailState::Username;
            }
            (EmailState::Username, b'@') if len > 2 => state = EmailState::At,
            (EmailState::Username, b) if b.is_ascii_alphanumeric() => (),
            (EmailState::Username, b) if b".!#$%&'*+/=?^_`{|}~-".contains(&b) => (),
            (EmailState::At, b) if b.is_ascii_alphanumeric() => state = EmailState::Domain(len),
            (EmailState::Domain(_), b) if b.is_ascii_alphanumeric() || b == b'-' => (),
            (EmailState::Domain(s), b'.') if (2..62).contains(&(len - *s)) => {
                state = EmailState::Tld(len)
            }
            (EmailState::Tld(_), b) if b.is_ascii_alphanumeric() || b == b'-' => (),
            (EmailState::Tld(s), b'.') if (2..62).contains(&(len - *s)) => {
                state = EmailState::Tld(len)
            }
            (EmailState::Tld(s), b'>') if (2..62).contains(&(len - *s)) => {
                return Some(i);
            }
            _ => return None,
        }
        // 计算字符长度
        let char_len = utf8_char_len(b);
        len += char_len;
        i += char_len;
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::ast::reference::Reference;
    use crate::ast::{MarkdownNode, embed, link};
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn ofm_case_block_id() {
        let text = r#""You do not rise to the level of your goals. You fall to the level of your systems." by James Clear ^quote-of-the-day"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(ast[1].id, Some(Box::new("quote-of-the-day".to_string())))
    }

    #[test]
    fn ofm_case_wikilink_1() {
        let text = r#"[[Three laws of motion]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(Box::new(
                link::Wikilink {
                    path: "Three laws of motion".to_string(),
                    reference: None,
                    text: None
                }
                .into()
            ))
        )
    }
    #[test]
    fn ofm_case_wikilink_2() {
        let text = r#"[[Three laws of motion#Second law]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(Box::new(
                link::Wikilink {
                    path: "Three laws of motion".to_string(),
                    reference: Some(Reference::Heading("Second law".to_string())),
                    text: None
                }
                .into()
            ))
        )
    }
    #[test]
    fn ofm_case_wikilink_3() {
        let text = r#"[[My note#Heading 1#Heading 2]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(Box::new(
                link::Wikilink {
                    path: "My note".to_string(),
                    reference: Some(Reference::MultiHeading(vec![
                        "Heading 1".to_string(),
                        "Heading 2".to_string()
                    ])),
                    text: None
                }
                .into()
            ))
        )
    }
    #[test]
    fn ofm_case_wikilink_4() {
        let text = r#"[[2023-01-01#^quote-of-the-day]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(Box::new(
                link::Wikilink {
                    path: "2023-01-01".to_string(),
                    reference: Some(Reference::BlockId("quote-of-the-day".to_string())),
                    text: None
                }
                .into()
            ))
        )
    }
    #[test]
    fn ofm_case_wikilink_5() {
        let text = r#"[[Internal links|custom display text]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(Box::new(
                link::Wikilink {
                    path: "Internal links".to_string(),
                    reference: None,
                    text: Some("custom display text".to_string())
                }
                .into()
            ))
        )
    }

    #[test]
    fn ofm_case_embed_1() {
        let text = r#"![[Internal links]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(Box::new(embed::Embed {
                path: "Internal links".to_string(),
                size: None,
                reference: None,
                attrs: None,
            }))
        )
    }
    #[test]
    fn ofm_case_embed_2() {
        let text = r#"![[Internal links#^b15695]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(Box::new(embed::Embed {
                path: "Internal links".to_string(),
                size: None,
                reference: Some(Reference::BlockId("b15695".to_string())),
                attrs: None,
            }))
        )
    }
    #[test]
    fn ofm_case_embed_3() {
        let text = r#"![[Engelbart.jpg|100x145]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(Box::new(embed::Embed {
                path: "Engelbart.jpg".to_string(),
                size: Some((100, Some(145))),
                reference: None,
                attrs: None,
            }))
        )
    }
    #[test]
    fn ofm_case_embed_4() {
        let text = r#"![[Engelbart.jpg|100]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(Box::new(embed::Embed {
                path: "Engelbart.jpg".to_string(),
                size: Some((100, None)),
                reference: None,
                attrs: None,
            }))
        )
    }
    #[test]
    fn ofm_case_embed_5() {
        let text = r#"![[Document.pdf#page=3]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(Box::new(embed::Embed {
                path: "Document.pdf".to_string(),
                size: None,
                reference: None,
                attrs: Some(vec![("page".to_string(), "3".to_string()),]),
            }))
        )
    }
    #[test]
    fn ofm_case_embed_6() {
        let text = r#"![[Document.pdf#page=3&theme=dark]]"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(Box::new(embed::Embed {
                path: "Document.pdf".to_string(),
                size: None,
                reference: None,
                attrs: Some(vec![
                    ("page".to_string(), "3".to_string()),
                    ("theme".to_string(), "dark".to_string()),
                ]),
            }))
        )
    }

    #[test]
    fn case_594() {
        let text = r#"<http://foo.bar.baz>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="http://foo.bar.baz">http://foo.bar.baz</a></p>"#
        )
    }
    #[test]
    fn case_595() {
        let text = r#"<https://foo.bar.baz/test?q=hello&id=22&boolean>"#;
        let ast = Parser::new(text).parse();
        println!("AST:\n{ast:?}");
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean">https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>"#
        )
    }
    #[test]
    fn case_596() {
        let text = r#"<irc://foo.bar:2233/baz>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="irc://foo.bar:2233/baz">irc://foo.bar:2233/baz</a></p>"#
        )
    }
    #[test]
    fn case_597() {
        let text = r#"<MAILTO:FOO@BAR.BAZ>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="MAILTO:FOO@BAR.BAZ">MAILTO:FOO@BAR.BAZ</a></p>"#
        )
    }
    #[test]
    fn case_602() {
        let text = r#"<https://foo.bar/baz bim>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p>&lt;https://foo.bar/baz bim&gt;</p>"#)
    }
    #[test]
    fn case_603() {
        let text = r#"<https://example.com/\[\>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="https://example.com/%5C%5B%5C">https://example.com/\[\</a></p>"#
        )
    }
    #[test]
    fn case_604() {
        let text = r#"<foo@bar.example.com>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="mailto:foo@bar.example.com">foo@bar.example.com</a></p>"#
        )
    }
    #[test]
    fn case_605() {
        let text = r#"<foo+special@Bar.baz-bar0.com>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="mailto:foo+special@Bar.baz-bar0.com">foo+special@Bar.baz-bar0.com</a></p>"#
        )
    }
    #[test]
    fn case_606() {
        let text = r#"<foo\+@bar.example.com>"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), r#"<p>&lt;foo+@bar.example.com&gt;</p>"#)
    }
    #[test]
    fn gfm_extended_autolink_http() {
        let text = "see https://example.com/path?q=1";
        let ast = Parser::new_with_options(
            text,
            ParserOptions::default()
                .enabled_gfm()
                .enabled_gfm_autolink(),
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<p>see <a href="https://example.com/path?q=1">https://example.com/path?q=1</a></p>"#
        );
    }
    #[test]
    fn gfm_extended_autolink_www() {
        let text = "www.example.com/docs";
        let ast = Parser::new_with_options(
            text,
            ParserOptions::default()
                .enabled_gfm()
                .enabled_gfm_autolink(),
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<p><a href="http://www.example.com/docs">www.example.com/docs</a></p>"#
        );
    }
    #[test]
    fn gfm_extended_autolink_trim_punct() {
        let text = "visit https://example.com/test).";
        let ast = Parser::new_with_options(
            text,
            ParserOptions::default()
                .enabled_gfm()
                .enabled_gfm_autolink(),
        )
        .parse();
        assert_eq!(
            ast.to_html(),
            r#"<p>visit <a href="https://example.com/test">https://example.com/test</a>).</p>"#
        );
    }
    #[test]
    fn test() {
        let text = r#"
![[wallhaven-5wk153.jpg]]

[孤独な巡礼](https://music.163.com/#/song?id=448065)

> 时境变迁，思绪万千"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
    }
}
