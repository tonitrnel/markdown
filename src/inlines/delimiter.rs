use crate::ast::MarkdownNode;
use crate::inlines::ProcessCtx;
use crate::span::MergedSpan;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub(super) struct Delimiter {
    pub(super) delimiter_byte: u8,
    pub(super) can_open: bool,
    pub(super) can_close: bool,
    pub(super) length: usize,
    pub(super) prev: Option<DelimiterChain>,
    pub(super) next: Option<DelimiterChain>,
    pub(super) position: usize,
    pub(super) node: usize,
}
impl PartialEq for Delimiter {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

#[derive(Clone, PartialEq)]
pub(super) struct DelimiterChain(Rc<RefCell<Delimiter>>);
impl DelimiterChain {
    pub(super) fn new(delimiter: Delimiter) -> Self {
        Self(Rc::new(RefCell::new(delimiter)))
    }
    pub(super) fn borrow(&self) -> Ref<'_, Delimiter> {
        self.0.borrow()
    }
    pub(super) fn borrow_mut(&self) -> RefMut<'_, Delimiter> {
        self.0.borrow_mut()
    }
}

impl Debug for DelimiterChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut count = 0;
        {
            let cur = self.borrow();
            writeln!(
                f,
                "  {count}. [{}]({},{})@{}#{}",
                cur.delimiter_byte as char, cur.can_open, cur.can_close, cur.length, cur.node
            )?;
        }
        let mut prev = self.borrow().prev.clone();
        while let Some(prev_delimiter) = prev {
            count += 1;
            {
                let prev = prev_delimiter.borrow();
                writeln!(
                    f,
                    "  {count}. [{}]({},{})@{}#{}",
                    prev.delimiter_byte as char,
                    prev.can_open,
                    prev.can_close,
                    prev.length,
                    prev.node
                )?;
            }
            let cloned = prev_delimiter.borrow().prev.clone();
            prev = cloned;
        }
        Ok(())
    }
}

/// 判断字节是否为空白
fn is_whitespace_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | 0x0B)
}

fn is_multibyte_whitespace(source: &[u8], pos: usize) -> bool {
    if pos >= source.len() || source[pos] < 0x80 {
        return false;
    }
    let s = unsafe { std::str::from_utf8_unchecked(source) };
    s.get(pos..)
        .and_then(|it| it.chars().next())
        .is_some_and(char::is_whitespace)
}

fn is_multibyte_whitespace_before(source: &[u8], pos: usize) -> bool {
    if pos == 0 || source[pos - 1] < 0x80 {
        return false;
    }
    let s = unsafe { std::str::from_utf8_unchecked(source) };
    s.get(..pos)
        .and_then(|it| it.chars().next_back())
        .is_some_and(char::is_whitespace)
}

/// 判断字节是否为 Unicode 标点（ASCII 范围内的标点）
fn is_punctuation_byte(b: u8) -> bool {
    crate::utils::is_punctuation(b as char)
        || matches!(
            b,
            b'!' | b'"'
                | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'('
                | b')'
                | b'*'
                | b'+'
                | b','
                | b'-'
                | b'.'
                | b'/'
                | b':'
                | b';'
                | b'<'
                | b'='
                | b'>'
                | b'?'
                | b'@'
                | b'['
                | b'\\'
                | b']'
                | b'^'
                | b'_'
                | b'`'
                | b'{'
                | b'|'
                | b'}'
                | b'~'
        )
}

#[inline]
fn is_hangul(ch: char) -> bool {
    matches!(
        ch,
        '\u{1100}'..='\u{11FF}'
            | '\u{3130}'..='\u{318F}'
            | '\u{A960}'..='\u{A97F}'
            | '\u{AC00}'..='\u{D7AF}'
            | '\u{D7B0}'..='\u{D7FF}'
    )
}

#[inline]
fn is_cjk_character(ch: char) -> bool {
    crate::utils::cjk::is_cjk(ch) || is_hangul(ch)
}

#[inline]
fn is_non_emoji_general_use_variation_selector(ch: char) -> bool {
    ('\u{FE00}'..='\u{FE0E}').contains(&ch)
}

#[inline]
fn is_ideographic_variation_selector(ch: char) -> bool {
    ('\u{E0100}'..='\u{E01EF}').contains(&ch)
}

#[inline]
fn is_non_cjk_punctuation_char(ch: char) -> bool {
    crate::utils::is_punctuation(ch) && !is_cjk_character(ch)
}

fn next_char_at(source: &[u8], pos: usize) -> Option<(char, usize)> {
    if pos >= source.len() {
        return None;
    }
    let s = unsafe { std::str::from_utf8_unchecked(source) };
    s.get(pos..)
        .and_then(|tail| tail.chars().next().map(|ch| (ch, ch.len_utf8())))
}

fn prev_char_before(source: &[u8], pos: usize) -> Option<(char, usize)> {
    if pos == 0 || pos > source.len() {
        return None;
    }
    let s = unsafe { std::str::from_utf8_unchecked(source) };
    s.get(..pos)
        .and_then(|head| head.char_indices().next_back().map(|(i, ch)| (ch, i)))
}

fn is_unicode_punctuation_sequence_next(source: &[u8], pos: usize) -> bool {
    let Some((ch, len)) = next_char_at(source, pos) else {
        return false;
    };
    if !crate::utils::is_punctuation(ch) {
        return false;
    }
    if let Some((vs, _)) = next_char_at(source, pos + len) {
        if is_non_emoji_general_use_variation_selector(vs) {
            return true;
        }
    }
    true
}

fn is_unicode_punctuation_sequence_before(source: &[u8], pos: usize) -> bool {
    let Some((ch, start)) = prev_char_before(source, pos) else {
        return false;
    };
    if is_non_emoji_general_use_variation_selector(ch) {
        let Some((prev, _)) = prev_char_before(source, start) else {
            return false;
        };
        return crate::utils::is_punctuation(prev);
    }
    crate::utils::is_punctuation(ch)
}

fn is_non_cjk_punctuation_sequence_before(source: &[u8], pos: usize) -> bool {
    let Some((ch, start)) = prev_char_before(source, pos) else {
        return false;
    };
    if is_non_emoji_general_use_variation_selector(ch) {
        let Some((prev, _)) = prev_char_before(source, start) else {
            return false;
        };
        return is_non_cjk_punctuation_char(prev);
    }
    is_non_cjk_punctuation_char(ch)
}

fn is_cjk_sequence_before(source: &[u8], pos: usize) -> bool {
    let Some((ch, start)) = prev_char_before(source, pos) else {
        return false;
    };
    if is_non_emoji_general_use_variation_selector(ch) {
        let Some((prev, _)) = prev_char_before(source, start) else {
            return false;
        };
        return is_cjk_character(prev);
    }
    is_cjk_character(ch)
}

pub(super) fn scan_delimiters(
    line: &mut MergedSpan,
    cjk_friendly: bool,
) -> (u8, usize, bool, bool) {
    let start = line.cursor();
    let Some(initial_byte) = line.peek() else {
        return (b'*', 1, false, false);
    };
    let length = if matches!(initial_byte, b'\'' | b'"') {
        1
    } else {
        line.starts_count(initial_byte)
    };
    let before_byte = if start == line.start() {
        b'\n' // 行首视为换行
    } else {
        line.source_slice()[start - 1]
    };
    // peek at byte after the delimiter run
    let after_byte = line.get(length).unwrap_or(b'\n');
    let after_cursor = start + length;
    // 对于多字节 UTF-8 字符，需要检查实际字符
    let after_is_whitespace = is_whitespace_byte(after_byte)
        || is_multibyte_whitespace(line.source_slice(), after_cursor);
    let after_is_punctuation = !after_is_whitespace
        && (is_punctuation_byte(after_byte)
            || (after_cursor < line.source_slice().len()
                && is_multibyte_punctuation(line.source_slice(), after_cursor)));
    let before_is_whitespace = is_whitespace_byte(before_byte)
        || is_multibyte_whitespace_before(line.source_slice(), start);
    let before_is_punctuation = !before_is_whitespace
        && (is_punctuation_byte(before_byte)
            || (start > 0 && is_multibyte_punctuation_before(line.source_slice(), start)));
    let before_is_unicode_punctuation_sequence =
        !before_is_whitespace && is_unicode_punctuation_sequence_before(line.source_slice(), start);
    let after_is_unicode_punctuation_sequence = !after_is_whitespace
        && is_unicode_punctuation_sequence_next(line.source_slice(), after_cursor);
    let (left_flanking, right_flanking) = if cjk_friendly {
        let after_is_non_cjk_punctuation_char = !after_is_whitespace
            && next_char_at(line.source_slice(), after_cursor)
                .map(|(ch, _)| is_non_cjk_punctuation_char(ch))
                .unwrap_or(false);
        let before_is_non_cjk_punctuation_sequence = !before_is_whitespace
            && is_non_cjk_punctuation_sequence_before(line.source_slice(), start);
        let before_is_cjk_sequence = is_cjk_sequence_before(line.source_slice(), start);
        let before_is_ivs = prev_char_before(line.source_slice(), start)
            .map(|(ch, _)| is_ideographic_variation_selector(ch))
            .unwrap_or(false);
        let after_is_cjk_character = next_char_at(line.source_slice(), after_cursor)
            .map(|(ch, _)| is_cjk_character(ch))
            .unwrap_or(false);
        (
            !after_is_whitespace
                && (!after_is_non_cjk_punctuation_char
                    || before_is_whitespace
                    || before_is_non_cjk_punctuation_sequence
                    || before_is_cjk_sequence
                    || before_is_ivs),
            length > 0
                && !before_is_whitespace
                && (!before_is_non_cjk_punctuation_sequence
                    || after_is_whitespace
                    || after_is_non_cjk_punctuation_char
                    || after_is_cjk_character),
        )
    } else {
        (
            !after_is_whitespace
                && (!after_is_punctuation || before_is_whitespace || before_is_punctuation),
            length > 0
                && !before_is_whitespace
                && (!before_is_punctuation || after_is_whitespace || after_is_punctuation),
        )
    };
    let (left, right) = match initial_byte {
        b'_' | b'~' | b'=' => (
            left_flanking && (!right_flanking || before_is_unicode_punctuation_sequence),
            right_flanking && (!left_flanking || after_is_unicode_punctuation_sequence),
        ),
        b'\'' | b'"' => (left_flanking && !right_flanking, right_flanking),
        _ => (left_flanking, right_flanking),
    };
    // 不消费任何字节，调用者负责 skip
    (initial_byte, length, left, right)
}

/// 检查当前位置是否为多字节 Unicode 标点
fn is_multibyte_punctuation(source: &[u8], pos: usize) -> bool {
    if pos >= source.len() || source[pos] < 0x80 {
        return false;
    }
    let s = unsafe { std::str::from_utf8_unchecked(source) };
    if let Some(ch) = s.get(pos..).and_then(|s| s.chars().next()) {
        crate::utils::is_punctuation(ch)
    } else {
        false
    }
}

/// 检查 pos 之前的字符是否为多字节 Unicode 标点
fn is_multibyte_punctuation_before(source: &[u8], pos: usize) -> bool {
    if pos == 0 || source[pos - 1] < 0x80 {
        return false;
    }
    let s = unsafe { std::str::from_utf8_unchecked(source) };
    if let Some(ch) = s.get(..pos).and_then(|s| s.chars().next_back()) {
        crate::utils::is_punctuation(ch)
    } else {
        false
    }
}

pub(super) fn before(
    ProcessCtx {
        line,
        parser,
        id,
        delimiters,
        ..
    }: &mut ProcessCtx,
    enabled_gfm_strikethrough: bool,
    enabled_ofm_highlight: bool,
) -> bool {
    if line.peek().is_none() {
        return false;
    }
    let scan_result = scan_delimiters(line, parser.options.cjk_friendly_delimiters);
    let start = line.cursor();
    let (text, locations) = {
        // For quotes, we need to skip the byte and get the location
        if matches!(scan_result.0, b'\'' | b'"') {
            line.skip(1);
            let end = line.cursor();
            let text = if scan_result.0 == b'\'' {
                "'".to_string()
            } else {
                "\"".to_string()
            };
            let loc_start = line.location_at_byte(start);
            let loc_end = line.location_at_byte(end);
            (text, (loc_start, loc_end))
        } else {
            line.skip(scan_result.1);
            let end = line.cursor();
            // 对常见 delimiter 使用静态字符串避免分配
            let text = match (scan_result.0, scan_result.1) {
                (b'*', 1) => "*".to_string(),
                (b'*', 2) => "**".to_string(),
                (b'*', 3) => "***".to_string(),
                (b'_', 1) => "_".to_string(),
                (b'_', 2) => "__".to_string(),
                (b'~', 1) => "~".to_string(),
                (b'~', 2) => "~~".to_string(),
                (b'=', 2) => "==".to_string(),
                _ => {
                    let slice = line.slice_from_abs(start, end);
                    slice.to_string()
                }
            };
            let loc_start = line.location_at_byte(start);
            let loc_end = line.location_at_byte(end);
            (text, (loc_start, loc_end))
        }
    };
    let node = parser.append_to(*id, MarkdownNode::Text(text), locations);
    parser.mark_as_processed(node);
    if (scan_result.2 || scan_result.3)
        && (parser.options.smart_punctuation || !matches!(scan_result.0, b'\'' | b'"'))
        && (!enabled_gfm_strikethrough || scan_result.1 == 1 || scan_result.1 == 2)
        && (!enabled_ofm_highlight || scan_result.1 == 2)
    {
        *delimiters = Some(DelimiterChain::new(Delimiter {
            delimiter_byte: scan_result.0,
            can_open: scan_result.2,
            can_close: scan_result.3,
            length: scan_result.1,
            prev: delimiters.clone(),
            next: None,
            position: start,
            node,
        }));
        if let Some(delimiters) = delimiters {
            let cloned = delimiters.clone();
            if let Some(previous) = &delimiters.borrow().prev {
                previous.borrow_mut().next = Some(cloned);
            }
        }
    }
    true
}

pub(super) fn process(
    ProcessCtx {
        parser, delimiters, ..
    }: &mut ProcessCtx,
    stack_bottom: usize,
) {
    let mut openers_bottom = [stack_bottom; 21];
    let mut candidate = delimiters.clone();
    let mut closer = None;
    while let Some(candidate_delimiter) = candidate
        .as_ref()
        .filter(|it| it.borrow().position >= stack_bottom)
    {
        closer = Some(candidate_delimiter.clone());
        let cloned_previous = candidate_delimiter.borrow().prev.clone();
        candidate = cloned_previous;
    }
    while let Some(closer_delimiter) = closer.as_ref() {
        if closer_delimiter.borrow().can_close {
            let (mut opener, openers_bottom_index) = {
                let closer_delimiter = closer_delimiter.borrow();
                let openers_bottom_index = match closer_delimiter.delimiter_byte {
                    b'"' => 0,
                    b'\'' => 1,
                    b'_' => {
                        2 + (if closer_delimiter.can_open { 3 } else { 0 })
                            + (closer_delimiter.length % 3)
                    }
                    b'*' => {
                        8 + (if closer_delimiter.can_open { 3 } else { 0 })
                            + (closer_delimiter.length % 3)
                    }
                    b'~' => {
                        14 + if closer_delimiter.can_open { 2 } else { 0 } + closer_delimiter.length
                    }
                    b'=' => 19 + if closer_delimiter.can_open { 1 } else { 0 },
                    _ => panic!(
                        "Invalid delimiter byte {}",
                        closer_delimiter.delimiter_byte as char
                    ),
                };
                (closer_delimiter.prev.clone(), openers_bottom_index)
            };
            let mut opener_found = false;
            {
                let closer_delimiter = closer_delimiter.borrow();
                while let Some(opener_delimiter) = opener
                    .as_ref()
                    .filter(|it| it.borrow().position >= openers_bottom[openers_bottom_index])
                {
                    {
                        let opener_delimiter = opener_delimiter.borrow();
                        let odd_match = (closer_delimiter.can_open || opener_delimiter.can_close)
                            && closer_delimiter.length % 3 != 0
                            && (opener_delimiter.length + closer_delimiter.length) % 3 == 0;
                        if opener_delimiter.can_open
                            && opener_delimiter.delimiter_byte == closer_delimiter.delimiter_byte
                            && !odd_match
                        {
                            opener_found = true;
                            break;
                        }
                    }
                    {
                        let cloned_previous = opener_delimiter.borrow().prev.clone();
                        opener = cloned_previous;
                    }
                }
            }
            let mut old_closer = closer.clone();
            let closer_byte = closer_delimiter.borrow().delimiter_byte;
            match closer_byte {
                b'*' | b'_' | b'~' | b'=' => {
                    if let Some(opener_delimiter) = opener.as_ref().filter(|_| opener_found) {
                        let opener_inl = opener_delimiter.borrow().node;
                        let closer_inl = closer_delimiter.borrow().node;

                        // Safety check: if opener and closer don't have the same parent,
                        // skip this delimiter pair to avoid tree corruption
                        if parser.tree.get_parent(opener_inl) != parser.tree.get_parent(closer_inl)
                        {
                            // Don't process this pair, just move to next closer
                            let cloned_next = closer_delimiter.borrow().next.clone();
                            closer = cloned_next;
                            continue;
                        }

                        let mut opener_char_nums =
                            if let MarkdownNode::Text(t) = &parser.tree[opener_inl].body {
                                t.len()
                            } else {
                                0
                            };
                        let mut closer_char_nums =
                            if let MarkdownNode::Text(t) = &parser.tree[closer_inl].body {
                                t.len()
                            } else {
                                0
                            };
                        let used_delimiter_nums = if closer_char_nums >= 2 && opener_char_nums >= 2
                        {
                            2
                        } else {
                            1
                        };
                        if let MarkdownNode::Text(text) = &mut parser.tree[opener_inl].body {
                            text.truncate(text.len() - used_delimiter_nums);
                            opener_char_nums = text.len();
                            parser.tree[opener_inl].end.column -= used_delimiter_nums as u64;
                        }
                        if let MarkdownNode::Text(text) = &mut parser.tree[closer_inl].body {
                            text.truncate(text.len() - used_delimiter_nums);
                            closer_char_nums = text.len();
                            parser.tree[closer_inl].end.column -= used_delimiter_nums as u64;
                        }
                        let start_location = parser.tree[opener_inl].end;
                        let node = match closer_byte {
                            b'*' | b'_' => {
                                if used_delimiter_nums == 1 {
                                    parser.append_free_node(MarkdownNode::Emphasis, start_location)
                                } else {
                                    parser.append_free_node(MarkdownNode::Strong, start_location)
                                }
                            }
                            b'~' => {
                                parser.append_free_node(MarkdownNode::Strikethrough, start_location)
                            }
                            b'=' => {
                                parser.append_free_node(MarkdownNode::Highlighting, start_location)
                            }
                            _ => panic!("Invalid delimiter byte {}", closer_byte as char),
                        };
                        parser.tree[node].end = {
                            let mut loc = parser.tree[closer_inl].end;
                            loc.column += used_delimiter_nums as u64;
                            loc
                        };
                        let mut temp = parser.tree.get_next(opener_inl);
                        while let Some(item) = temp.filter(|it| it != &closer_inl) {
                            let next = parser.tree.get_next(item);
                            parser.tree.unlink(item);
                            parser.tree.set_parent(item, node);
                            temp = next;
                        }
                        parser
                            .tree
                            .set_parent(node, parser.tree.get_parent(opener_inl));
                        // Safety check: if opener and closer don't have the same parent,
                        // skip this delimiter pair to avoid tree corruption
                        if parser.tree.get_parent(opener_inl) != parser.tree.get_parent(closer_inl)
                        {
                            // Don't process this pair, just move to next closer
                            let cloned_next = closer_delimiter.borrow().next.clone();
                            closer = cloned_next;
                            continue;
                        }
                        parser.tree.set_next(opener_inl, node);
                        parser.tree.set_prev(closer_inl, node);
                        if opener_delimiter.borrow().next.as_ref() != Some(closer_delimiter) {
                            opener_delimiter.borrow_mut().next = closer.clone();
                            closer_delimiter.borrow_mut().prev = opener.clone();
                        }
                        if opener_char_nums == 0 {
                            parser.tree.remove(opener_inl);
                            remove_delimiter(&mut opener)
                        }
                        if closer_char_nums == 0 {
                            parser.tree.remove(closer_inl);
                            let cloned_next = closer_delimiter.borrow().next.clone();
                            remove_delimiter(&mut closer);
                            closer = cloned_next
                        }
                    } else {
                        let cloned_next = closer_delimiter.borrow().next.clone();
                        closer = cloned_next;
                    }
                }
                b'\'' => {
                    if opener_found {
                        // Found matching opener, convert opener to left quote and closer to right quote
                        if let Some(opener_delimiter) = opener.as_ref() {
                            let opener_node = &mut parser.tree[opener_delimiter.borrow().node];
                            opener_node.body = MarkdownNode::Text("\u{2018}".into());
                        }
                        let closer_node = &mut parser.tree[closer_delimiter.borrow().node];
                        closer_node.body = MarkdownNode::Text("\u{2019}".into());
                        // Remove both opener and closer from delimiter chain
                        remove_delimiter(&mut opener);
                    } else {
                        // No matching opener, treat as apostrophe (right single quote)
                        let node = &mut parser.tree[closer_delimiter.borrow().node];
                        node.body = MarkdownNode::Text("\u{2019}".into());
                    }
                    let cloned_next = closer_delimiter.borrow().next.clone();
                    closer = cloned_next;
                }
                b'"' => {
                    if opener_found {
                        // Found matching opener, convert opener to left quote and closer to right quote
                        if let Some(opener_delimiter) = opener.as_ref() {
                            let opener_node = &mut parser.tree[opener_delimiter.borrow().node];
                            opener_node.body = MarkdownNode::Text("\u{201C}".into());
                        }
                        let closer_node = &mut parser.tree[closer_delimiter.borrow().node];
                        closer_node.body = MarkdownNode::Text("\u{201D}".into());
                        // Remove both opener and closer from delimiter chain
                        remove_delimiter(&mut opener);
                    } else {
                        // No matching opener, should not happen for double quotes in normal text
                        // but treat as right double quote
                        let node = &mut parser.tree[closer_delimiter.borrow().node];
                        node.body = MarkdownNode::Text("\u{201D}".into());
                    }
                    let cloned_next = closer_delimiter.borrow().next.clone();
                    closer = cloned_next;
                }
                _ => (),
            };
            if let Some(old_closer_delimiter) = old_closer.as_ref().filter(|_| !opener_found) {
                openers_bottom[openers_bottom_index] = old_closer_delimiter.borrow().position;
                if !old_closer_delimiter.borrow().can_open {
                    remove_delimiter(&mut old_closer)
                }
            }
        } else {
            let cloned_next = closer_delimiter.borrow().next.clone();
            closer = cloned_next;
        }
    }
    while delimiters
        .as_ref()
        .filter(|it| it.borrow().position >= stack_bottom)
        .is_some()
    {
        // Before removing, check if it's a quote delimiter and convert it
        if let Some(delimiter) = delimiters.as_ref() {
            let delimiter_byte = delimiter.borrow().delimiter_byte;
            if matches!(delimiter_byte, b'\'' | b'"') {
                let node_id = delimiter.borrow().node;
                let can_open = delimiter.borrow().can_open;
                let can_close = delimiter.borrow().can_close;
                let next_is_lower_alpha = parser
                    .tree
                    .get_next(node_id)
                    .and_then(|next_id| match &parser.tree[next_id].body {
                        MarkdownNode::Text(text) => text.chars().next(),
                        _ => None,
                    })
                    .is_some_and(|ch| ch.is_ascii_lowercase());

                // Convert unmatched quotes
                let node = &mut parser.tree[node_id];
                if delimiter_byte == b'\'' {
                    // For unmatched single quotes:
                    // - leading contractions like `'tis` should become apostrophes (right quote)
                    // - other openers stay as left quotes.
                    node.body = if can_open && !can_close {
                        if next_is_lower_alpha {
                            MarkdownNode::Text("\u{2019}".into())
                        } else {
                            MarkdownNode::Text("\u{2018}".into())
                        }
                    } else {
                        MarkdownNode::Text("\u{2019}".into())
                    };
                } else if delimiter_byte == b'"' {
                    // For double quotes: if can_open, use left quote; otherwise use right quote
                    node.body = if can_open && !can_close {
                        MarkdownNode::Text("\u{201C}".into())
                    } else {
                        MarkdownNode::Text("\u{201D}".into())
                    };
                }
            }
        }
        remove_delimiter(delimiters)
    }
}

fn remove_delimiter(delimiter_chain: &mut Option<DelimiterChain>) {
    let delimiter = match delimiter_chain.as_ref() {
        Some(d) => d,
        None => return,
    };
    if let Some(previous) = delimiter.borrow().prev.as_ref() {
        previous.borrow_mut().next = delimiter.borrow().next.clone();
    }
    if let Some(next) = delimiter.borrow().next.as_ref() {
        next.borrow_mut().prev = delimiter.borrow().prev.clone();
        return;
    }
    let cloned_previous = delimiter.borrow().prev.clone();
    *delimiter_chain = cloned_previous
}

/// 最终处理 delimiter chain（在所有 Span 处理完毕后调用）
pub(super) fn process_final<'input>(
    id: usize,
    parser: &mut crate::parser::Parser<'input>,
    brackets: &mut Option<super::BracketChain>,
    delimiters: &mut Option<DelimiterChain>,
) {
    let mut empty_line = crate::span::MergedSpan::from_single(crate::span::Span::empty());
    let mut ctx = ProcessCtx {
        id,
        parser,
        line: &mut empty_line,
        brackets: brackets.take(),
        delimiters: delimiters.take(),
    };
    process(&mut ctx, 0);
    *brackets = ctx.brackets;
    *delimiters = ctx.delimiters;
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn case_350() {
        let text = r#"*foo bar*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><em>foo bar</em></p>")
    }
    #[test]
    fn case_351() {
        let text = r#"a * foo bar*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>a * foo bar*</p>")
    }
    #[test]
    fn case_357() {
        let text = r#"_foo bar_"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><em>foo bar</em></p>")
    }

    #[test]
    fn case_354_nbsp_not_emphasis() {
        let text = "*\u{a0}a\u{a0}*";
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>*\u{a0}a\u{a0}*</p>");
    }
    #[test]
    fn case_378() {
        let text = r#"**foo bar**"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><strong>foo bar</strong></p>")
    }
    #[test]
    fn case_409() {
        let text = r#"*foo *bar**"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><em>foo <em>bar</em></em></p>")
    }
    #[test]
    fn case_411() {
        let text = r#"*foo**bar**baz*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><em>foo<strong>bar</strong>baz</em></p>")
    }
    #[test]
    fn case_412() {
        let text = r#"*foo**bar*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><em>foo**bar</em></p>")
    }
    #[test]
    fn case_413() {
        let text = r#"***foo** bar*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><em><strong>foo</strong> bar</em></p>")
    }
    #[test]
    fn case_416() {
        let text = r#"foo***bar***baz"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>foo<em><strong>bar</strong></em>baz</p>")
    }
    #[test]
    fn case_417() {
        let text = r#"foo******bar*********baz"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            "<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>"
        )
    }
    #[test]
    fn case_418() {
        let text = r#"*foo **bar *baz* bim** bop*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            "<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>"
        )
    }
    #[test]
    fn case_420() {
        let text = r#"** is not an empty emphasis"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>** is not an empty emphasis</p>")
    }
    #[test]
    fn case_425() {
        let text = r#"__foo __bar__ baz__"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast.to_html(),
            "<p><strong>foo <strong>bar</strong> baz</strong></p>"
        )
    }
    #[test]
    fn case_442() {
        let text = r#"**foo*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>*<em>foo</em></p>")
    }
    #[test]
    fn case_443() {
        let text = r#"*foo**"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p><em>foo</em>*</p>")
    }
    #[test]
    fn case_444() {
        let text = r#"***foo**"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>*<strong>foo</strong></p>")
    }
    #[test]
    fn case_445() {
        let text = r#"****foo*"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>***<em>foo</em></p>")
    }
    #[test]
    fn case_449() {
        let text = r#"foo _\__"#;
        let ast = Parser::new(text).parse();
        assert_eq!(ast.to_html(), "<p>foo <em>_</em></p>")
    }

    #[test]
    fn gfm_case_491() {
        let text = r#"~~Hi~~ Hello, ~there~ world!"#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        assert_eq!(
            ast.to_html(),
            "<p><del>Hi</del> Hello, <del>there</del> world!</p>"
        )
    }
    #[test]
    fn gfm_case_492() {
        let text = r#"This ~~has a

new paragraph~~."#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        assert_eq!(
            ast.to_html(),
            r#"<p>This ~~has a</p>
<p>new paragraph~~.</p>"#
        )
    }
    #[test]
    fn gfm_case_493() {
        let text = r#"This will ~~~not~~~ strike."#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_gfm()).parse();
        assert_eq!(ast.to_html(), "<p>This will ~~~not~~~ strike.</p>")
    }

    #[test]
    fn ofm_case_1() {
        let text = r#"==Highlighted text=="#;
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_ofm()).parse();
        assert_eq!(ast.to_html(), "<p><mark>Highlighted text</mark></p>")
    }

    #[test]
    fn test_crisscross() {
        let text = r#"**bold _italic** ending_"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
        assert_eq!(
            ast.to_html(),
            "<p><strong>bold _italic</strong> ending_</p>"
        );
    }

    #[test]
    fn cjk_friendly_star_open_with_cjk_punct_after() {
        let text = "A*。B*";
        let ast_default = Parser::new(text).parse();
        assert_eq!(ast_default.to_html(), "<p>A*。B*</p>");

        let ast_cjk = Parser::new_with_options(
            text,
            ParserOptions::default().enabled_cjk_friendly_delimiters(),
        )
        .parse();
        assert_eq!(ast_cjk.to_html(), "<p>A<em>。B</em></p>");
    }

    #[test]
    fn cjk_friendly_star_close_with_cjk_punct_before() {
        let text = "*A。*B";
        let ast_default = Parser::new(text).parse();
        assert_eq!(ast_default.to_html(), "<p>*A。*B</p>");

        let ast_cjk = Parser::new_with_options(
            text,
            ParserOptions::default().enabled_cjk_friendly_delimiters(),
        )
        .parse();
        assert_eq!(ast_cjk.to_html(), "<p><em>A。</em>B</p>");
    }
}
