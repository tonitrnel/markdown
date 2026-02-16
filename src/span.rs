use crate::parser::Location;
use crate::scanner::Scanner;
use std::fmt::{self, Debug, Formatter, Write};

/// 基于字节偏移的行表示，替代当前基于 `Vec<TokenWithLocation>` 的 `Line`。
///
/// Span 使用字节偏移范围引用原始输入切片，不拥有独立的数据副本，
/// 从而消除 Token 收集和 Vec 分配的开销。
#[derive(Clone)]
pub struct Span<'input> {
    /// 原始输入引用（字节切片）
    source: &'input [u8],
    /// 行起始字节偏移（含）
    start: usize,
    /// 行结束字节偏移（不含）
    end: usize,
    /// 当前扫描位置（绝对字节偏移）
    cursor: usize,
    /// 缩进信息（空格等价数量，tab = 4 spaces）
    indent_spaces: u16,
    /// 缩进占用的字节数
    indent_bytes: usize,
    /// 缩进是否已跳过
    indent_skipped: bool,
    /// 行是否为纯 ASCII（字节数 == 字符数，location 计算 O(1)）
    is_ascii: bool,
    /// 行号（从 1 开始）
    line_number: u64,
    /// 行起始列号（从 1 开始）
    start_col: u64,
    /// 预计算的整行字符数（start..end），用于 O(1) 的 last_token_end_location
    total_chars: usize,
}

/// 轻量级快照，仅存储游标和偏移信息，零分配。
#[derive(Clone, Copy)]
pub struct SpanSnapshot {
    start: usize,
    end: usize,
    cursor: usize,
    indent_spaces: u16,
    indent_bytes: usize,
    indent_skipped: bool,
    is_ascii: bool,
    line_number: u64,
    start_col: u64,
    total_chars: usize,
}

impl Debug for SpanSnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SpanSnapshot {{ cursor: {}, start: {}, end: {} }}",
            self.cursor, self.start, self.end
        )
    }
}

impl<'input> Span<'input> {
    /// 创建一个空的 Span（用于占位）
    pub fn empty() -> Self {
        static EMPTY: [u8; 0] = [];
        Span {
            source: &EMPTY,
            start: 0,
            end: 0,
            cursor: 0,
            indent_spaces: 0,
            indent_bytes: 0,
            indent_skipped: true,
            is_ascii: true,
            line_number: 0,
            start_col: 0,
            total_chars: 0,
        }
    }

    /// 将 start 调整为 cursor 位置，用于合并前去掉已跳过的缩进
    pub fn normalize_start(&mut self) {
        if self.cursor > self.start {
            let delta_cols = if self.is_ascii {
                self.cursor - self.start
            } else {
                count_chars(self.source, self.start, self.cursor)
            };
            self.start_col += delta_cols as u64;
            self.start = self.cursor;
            // start 变化后，按需重新计算
            self.total_chars = 0;
        }
    }

    /// 从 Scanner 提取下一行（到换行符或输入末尾）。
    ///
    /// 消耗换行符但不包含在 Span 范围内。返回 `None` 表示输入已耗尽。
    pub fn extract(scanner: &mut Scanner<'input>) -> Option<Self> {
        if scanner.remaining() == 0 {
            return None;
        }

        let start = scanner.pos();
        let source = scanner.source();
        let line_number = scanner.line_number();
        let start_col = scanner.location().column;

        // 扫描到行尾
        let end = scanner.skip_to_eol();

        // 消耗换行符
        if let Some(b) = scanner.peek() {
            if b == b'\n' {
                scanner.advance();
            } else if b == b'\r' {
                scanner.advance();
                // \r\n
                if scanner.peek() == Some(b'\n') {
                    scanner.advance();
                }
            }
        }

        // 计算缩进
        let (indent_spaces, indent_bytes) = compute_indent(source, start, end);
        let blank = indent_bytes >= (end - start);

        // 如果是空白行，indent_bytes 设为行长度
        let indent_bytes_final = if blank { end - start } else { indent_bytes };

        // 快速检测是否纯 ASCII
        let is_ascii = source[start..end].is_ascii();

        Some(Span {
            source,
            start,
            end,
            cursor: start,
            indent_spaces,
            indent_bytes: indent_bytes_final,
            indent_skipped: false,
            is_ascii,
            line_number,
            start_col,
            // 延迟计算：0 表示未计算，需要时再算
            total_chars: 0,
        })
    }

    /// 查看当前字节，不前进
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.cursor >= self.end {
            None
        } else {
            Some(self.source[self.cursor])
        }
    }

    /// 前进并返回当前字节
    #[inline]
    pub fn next_byte(&mut self) -> Option<u8> {
        if self.cursor >= self.end {
            None
        } else {
            let b = self.source[self.cursor];
            self.cursor += 1;
            Some(b)
        }
    }

    /// 跳过缩进（仅首次调用生效）
    pub fn skip_indent(&mut self) -> &mut Self {
        if self.indent_skipped {
            return self;
        }
        let target = self.start + self.indent_bytes;
        if self.cursor < target {
            self.cursor = target;
        } else {
            self.cursor += self.indent_bytes;
        }
        self.indent_skipped = true;
        self
    }

    /// 获取从当前游标到行尾的字符串切片
    pub fn as_str(&self) -> &'input str {
        if self.cursor >= self.end {
            return "";
        }
        // SAFETY: source 来自 &str，start/end 在行边界上（换行符处分割），
        // cursor 只在字符边界上移动（通过 skip 等方法）
        unsafe { std::str::from_utf8_unchecked(&self.source[self.cursor..self.end]) }
    }

    /// 获取从 start 到 end 的完整行内容（忽略游标位置）
    pub fn full_str(&self) -> &'input str {
        if self.start >= self.end {
            return "";
        }
        unsafe { std::str::from_utf8_unchecked(&self.source[self.start..self.end]) }
    }

    /// 行是否为空白行（仅包含空格和制表符）
    #[inline]
    pub fn is_blank(&self) -> bool {
        self.indent_bytes >= self.end.saturating_sub(self.start)
    }

    /// 从当前游标到行尾是否为空白
    pub fn is_blank_to_end(&self) -> bool {
        for i in self.cursor..self.end {
            match self.source[i] {
                b' ' | b'\t' => continue,
                _ => return false,
            }
        }
        true
    }

    /// 是否已消费完毕
    #[inline]
    pub fn is_end(&self) -> bool {
        self.cursor >= self.end
    }

    /// 缩进是否大于等于 4 个空格（应使用 IndentedCode 解析）
    pub fn is_indented(&self) -> bool {
        self.indent_spaces >= 4
    }

    /// 获取缩进的空白字符数量（tab = 4 spaces）
    #[inline]
    pub fn indent_spaces(&self) -> usize {
        self.indent_spaces as usize
    }

    /// 获取缩进占用的字节数
    #[inline]
    pub fn indent_len(&self) -> usize {
        self.indent_bytes
    }

    /// 当前游标到行尾的剩余字节数
    #[inline]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.cursor)
    }

    /// 合并多个 Span 为一个连续的 Span。
    ///
    /// 所有 Span 必须来自同一个 source，且按顺序排列。
    /// 合并后的 Span 范围从第一个 Span 的 start 到最后一个 Span 的 end，
    /// 中间的换行符自然包含在内。
    ///
    /// 用于 link_reference 等需要跨行扫描的场景。
    pub fn merge(spans: &[Span<'input>]) -> Option<Self> {
        let first = spans.first()?;
        let last = spans.last()?;
        let start = first.start;
        let end = last.end;
        let (indent_spaces, indent_bytes) = compute_indent(first.source, start, end);
        let blank = indent_bytes >= (end - start);
        let indent_bytes_final = if blank { end - start } else { indent_bytes };
        Some(Span {
            source: first.source,
            start,
            end,
            cursor: first.cursor,
            indent_spaces,
            indent_bytes: indent_bytes_final,
            indent_skipped: first.indent_skipped,
            is_ascii: spans.iter().all(|s| s.is_ascii),
            line_number: first.line_number,
            start_col: first.start_col,
            total_chars: count_chars(first.source, start, end),
        })
    }

    /// 快照当前状态
    pub fn snapshot(&self) -> SpanSnapshot {
        SpanSnapshot {
            start: self.start,
            end: self.end,
            cursor: self.cursor,
            indent_spaces: self.indent_spaces,
            indent_bytes: self.indent_bytes,
            indent_skipped: self.indent_skipped,
            is_ascii: self.is_ascii,
            line_number: self.line_number,
            start_col: self.start_col,
            total_chars: self.total_chars,
        }
    }

    /// 从快照恢复状态
    pub fn resume(&mut self, snapshot: &SpanSnapshot) -> &mut Self {
        self.start = snapshot.start;
        self.end = snapshot.end;
        self.cursor = snapshot.cursor;
        self.indent_spaces = snapshot.indent_spaces;
        self.indent_bytes = snapshot.indent_bytes;
        self.indent_skipped = snapshot.indent_skipped;
        self.is_ascii = snapshot.is_ascii;
        self.line_number = snapshot.line_number;
        self.start_col = snapshot.start_col;
        self.total_chars = snapshot.total_chars;
        self
    }

    /// 重新计算缩进（用于容器嵌套时丢弃前缀后重新查找）
    pub fn re_find_indent(&mut self) {
        let (spaces, bytes) = compute_indent(self.source, self.cursor, self.end);
        self.indent_spaces = spaces;
        self.indent_bytes = bytes;
        self.indent_skipped = false;
    }

    // --- 与 Line 等价的操作接口 ---

    /// 检查从当前游标开始是否以指定字节开头，连续 `len` 个
    pub fn starts_with(&self, byte: u8, len: usize) -> bool {
        if self.len() < len {
            return false;
        }
        for i in 0..len {
            if self.source[self.cursor + i] != byte {
                return false;
            }
        }
        true
    }

    /// 检查从当前游标开始是否以指定字节序列开头
    pub fn starts_with_bytes(&self, needle: &[u8]) -> bool {
        if self.len() < needle.len() {
            return false;
        }
        &self.source[self.cursor..self.cursor + needle.len()] == needle
    }

    /// 获取从当前游标开始连续相同字节的数量
    pub fn starts_count(&self, byte: u8) -> usize {
        let mut count = 0;
        let mut pos = self.cursor;
        while pos < self.end && self.source[pos] == byte {
            count += 1;
            pos += 1;
        }
        count
    }

    /// 获取从当前游标开始满足条件的连续字节数量
    pub fn starts_count_matches<P>(&self, predicate: P) -> usize
    where
        P: Fn(u8) -> bool,
    {
        let mut count = 0;
        let mut pos = self.cursor;
        while pos < self.end && predicate(self.source[pos]) {
            count += 1;
            pos += 1;
        }
        count
    }

    /// 获取从行尾向前满足条件的连续字节数量
    pub fn ends_count_matches<P>(&self, predicate: P) -> usize
    where
        P: Fn(u8) -> bool,
    {
        let mut count = 0;
        let mut pos = self.end;
        while pos > self.cursor && predicate(self.source[pos - 1]) {
            count += 1;
            pos -= 1;
        }
        count
    }

    /// 跳过指定数量的字节
    pub fn skip(&mut self, len: usize) -> &mut Self {
        self.cursor = (self.cursor + len).min(self.end);
        self
    }

    /// 跳过指定数量的空白字符（按 CommonMark tab stop 规则）
    pub fn skip_spaces(&mut self, len: usize) -> &mut Self {
        if len == 0 {
            return self;
        }
        let mut remaining = len;
        // 列位置基于本次 skip 的起点，tab 前进到下一个 4 列停靠点。
        let mut cols = 0usize;
        while self.cursor < self.end && remaining > 0 {
            match self.source[self.cursor] {
                b' ' => {
                    remaining -= 1;
                    cols += 1;
                    self.cursor += 1;
                }
                b'\t' => {
                    let tab_width = 4 - (cols % 4);
                    if tab_width > remaining {
                        self.cursor += 1;
                        break;
                    }
                    remaining -= tab_width;
                    cols += tab_width;
                    self.cursor += 1;
                }
                _ => break,
            }
        }
        self
    }

    /// 如果当前字节满足条件则消费，否则不动
    pub fn consume(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    /// 如果当前字节满足谓词则消费
    pub fn consume_if<P>(&mut self, predicate: P) -> bool
    where
        P: Fn(u8) -> bool,
    {
        if let Some(b) = self.peek() {
            if predicate(b) {
                self.cursor += 1;
                return true;
            }
        }
        false
    }

    /// 前进到下一个非空白字节
    pub fn advance_next_nonspace(&mut self) -> &mut Self {
        while self.cursor < self.end {
            match self.source[self.cursor] {
                b' ' | b'\t' => self.cursor += 1,
                _ => break,
            }
        }
        self
    }

    /// 从当前游标开始去除满足条件的前缀字节
    pub fn trim_start_matches<P>(&mut self, predicate: P) -> &mut Self
    where
        P: Fn(u8) -> bool,
    {
        let count = self.starts_count_matches(&predicate);
        self.cursor += count;
        self
    }

    /// 从行尾去除满足条件的后缀字节
    pub fn trim_end_matches<P>(&mut self, predicate: P) -> &mut Self
    where
        P: Fn(u8) -> bool,
    {
        let count = self.ends_count_matches(&predicate);
        self.end -= count;
        self
    }

    /// 确保从当前游标到行尾仅有空白字符
    pub fn only_space_to_end(&self) -> bool {
        if self.is_end() {
            return true;
        }
        for i in self.cursor..self.end {
            match self.source[i] {
                b' ' | b'\t' => continue,
                b'\n' | b'\r' => return true, // 换行符视为行尾
                _ => return false,
            }
        }
        true
    }

    /// 跳至行结束
    pub fn skip_to_end(&mut self) {
        self.cursor = self.end;
    }

    /// 获取指定偏移处的字节（相对于当前游标）
    pub fn get(&self, index: usize) -> Option<u8> {
        let pos = self.cursor + index;
        if pos >= self.end {
            None
        } else {
            Some(self.source[pos])
        }
    }

    /// 获取行起始偏移
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// 获取行结束偏移
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// 获取当前游标位置
    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// 获取原始 source 字节切片
    #[inline]
    pub fn source_slice(&self) -> &'input [u8] {
        self.source
    }

    /// 获取指定绝对偏移处的字节
    #[inline]
    pub fn source_byte(&self, pos: usize) -> u8 {
        self.source[pos]
    }

    /// 手动前进游标一个字节（用于跳过 UTF-8 continuation bytes）
    #[inline]
    pub fn advance_cursor(&mut self) {
        if self.cursor < self.end {
            self.cursor += 1;
        }
    }

    // --- Location 相关方法 ---

    /// 计算指定字节偏移处的 Location
    /// 获取指定绝对字节偏移处的 Location（供 inline 文本累积器使用）
    pub fn location_at_byte(&self, byte_pos: usize) -> Location {
        self.location_at(byte_pos)
    }

    fn location_at(&self, byte_pos: usize) -> Location {
        if self.is_ascii {
            // 纯 ASCII 行：字节偏移 == 字符偏移，O(1)
            let col_offset = byte_pos.saturating_sub(self.start);
            Location::new(self.line_number, self.start_col + col_offset as u64)
        } else {
            let col_offset = count_chars(self.source, self.start, byte_pos);
            Location::new(self.line_number, self.start_col + col_offset as u64)
        }
    }

    /// 获取当前游标位置的 Location
    pub fn start_location(&self) -> Location {
        if self.is_end() {
            // 如果已消费完毕，返回行尾位置
            self.location_at(self.end.min(self.source.len()))
        } else {
            self.location_at(self.cursor)
        }
    }

    /// 获取当前游标位置的结束 Location（下一个字符位置）
    pub fn end_location(&self) -> Location {
        let pos = if self.is_end() { self.end } else { self.cursor };
        let loc = self.location_at(pos);
        Location::new(loc.line, loc.column + 1)
    }

    /// 获取行最后一个字符的结束 Location
    pub fn last_token_end_location(&self) -> Location {
        if self.start >= self.end {
            return self.end_location();
        }
        if self.is_ascii {
            Location::new(
                self.line_number,
                self.start_col + (self.end - self.start) as u64,
            )
        } else if self.total_chars > 0 {
            Location::new(self.line_number, self.start_col + self.total_chars as u64)
        } else {
            let char_count = count_chars(self.source, self.start, self.end);
            Location::new(self.line_number, self.start_col + char_count as u64)
        }
    }

    /// 确保 total_chars 已计算（用于需要多次访问的场景）
    pub fn ensure_total_chars(&mut self) {
        if self.total_chars == 0 && self.start < self.end {
            self.total_chars = count_chars(self.source, self.start, self.end);
        }
    }

    /// 创建一个子 Span（切片），用于传递给 inline parser
    pub fn slice(&self, start: usize, end: usize) -> Span<'input> {
        let abs_start = (self.cursor + start).min(self.end);
        let abs_end = (self.cursor + end).min(self.end);
        let (col_offset, total_chars) = if self.is_ascii {
            (abs_start - self.start, abs_end - abs_start)
        } else {
            (
                count_chars(self.source, self.start, abs_start),
                count_chars(self.source, abs_start, abs_end),
            )
        };
        Span {
            source: self.source,
            start: abs_start,
            end: abs_end,
            cursor: abs_start,
            indent_spaces: 0,
            indent_bytes: 0,
            indent_skipped: true,
            is_ascii: self.is_ascii,
            line_number: self.line_number,
            start_col: self.start_col + col_offset as u64,
            total_chars,
        }
    }

    /// 从绝对字节偏移创建子 Span（用于 delimiter 等需要绝对位置的场景）
    pub fn slice_from_abs(&self, abs_start: usize, abs_end: usize) -> Span<'input> {
        let mut abs_start = abs_start.clamp(self.start, self.end);
        let mut abs_end = abs_end.clamp(self.start, self.end);
        if abs_end < abs_start {
            std::mem::swap(&mut abs_start, &mut abs_end);
        }
        let (col_offset, total_chars) = if self.is_ascii {
            (abs_start - self.start, abs_end - abs_start)
        } else {
            (
                count_chars(self.source, self.start, abs_start),
                count_chars(self.source, abs_start, abs_end),
            )
        };
        Span {
            source: self.source,
            start: abs_start,
            end: abs_end,
            cursor: abs_start,
            indent_spaces: 0,
            indent_bytes: 0,
            indent_skipped: true,
            is_ascii: self.is_ascii,
            line_number: self.line_number,
            start_col: self.start_col + col_offset as u64,
            total_chars,
        }
    }

    /// 创建一个 trim 后的 Span
    pub fn trim(&self) -> Span<'input> {
        let mut s = self.cursor;
        let mut e = self.end;
        while s < e && matches!(self.source[s], b' ' | b'\t') {
            s += 1;
        }
        while e > s && matches!(self.source[e - 1], b' ' | b'\t') {
            e -= 1;
        }
        let (col_offset, total_chars) = if self.is_ascii {
            (s - self.start, e - s)
        } else {
            (
                count_chars(self.source, self.start, s),
                count_chars(self.source, s, e),
            )
        };
        Span {
            source: self.source,
            start: s,
            end: e,
            cursor: s,
            indent_spaces: 0,
            indent_bytes: 0,
            indent_skipped: true,
            is_ascii: self.is_ascii,
            line_number: self.line_number,
            start_col: self.start_col + col_offset as u64,
            total_chars,
        }
    }

    /// 将 Span 内容写入 writer（用于代码块等需要原始文本的场景）
    pub fn write_string<W>(&self, writer: &mut W) -> fmt::Result
    where
        W: Write,
    {
        writer.write_str(self.as_str())
    }

    /// 获取 Span 内容的字符串（等价于 to_unescape_string）
    pub fn to_unescape_string(&self) -> String {
        self.as_str().to_string()
    }

    /// 检查指定偏移处的字节是否匹配（相对于当前游标）
    pub fn validate(&self, index: usize, byte: u8) -> bool {
        self.get(index) == Some(byte)
    }

    /// 检查指定偏移处的字节是否满足谓词（相对于当前游标）
    pub fn validate_with<P>(&self, index: usize, predicate: P) -> bool
    where
        P: Fn(u8) -> bool,
    {
        self.get(index).map(|b| predicate(b)).unwrap_or(false)
    }

    /// 查找满足条件的第一个字节位置（相对于当前游标）
    pub fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(u8) -> bool,
    {
        for i in self.cursor..self.end {
            if predicate(self.source[i]) {
                return Some(i - self.cursor);
            }
        }
        None
    }

    /// 跳过连续相同的字节
    pub fn skip_consecutive(&mut self, byte: u8) {
        let count = self.starts_count(byte);
        self.cursor += count;
    }

    /// 检查字节是否为 "special" token（用于块级解析的特殊字符）
    pub fn is_special_byte(byte: u8) -> bool {
        matches!(
            byte,
            b'#' | b'`'
                | b'~'
                | b'-'
                | b'*'
                | b'_'
                | b'+'
                | b'='
                | b'<'
                | b'>'
                | b'|'
                | b':'
                | b'['
                | b'0'..=b'9'
        )
    }
}

/// 计算从 start 到 end 范围内的缩进信息
fn compute_indent(source: &[u8], start: usize, end: usize) -> (u16, usize) {
    let mut spaces: u16 = 0;
    let mut bytes: usize = 0;
    let mut pos = start;
    while pos < end {
        match source[pos] {
            b' ' => {
                spaces += 1;
                bytes += 1;
                pos += 1;
            }
            b'\t' => {
                let tab_width = 4 - ((spaces as usize) % 4);
                spaces += tab_width as u16;
                bytes += 1;
                pos += 1;
            }
            _ => break,
        }
    }
    (spaces, bytes)
}

/// 计算从 start 到 end 范围内的字符数（非 continuation bytes 的数量）
#[inline]
fn count_chars(source: &[u8], start: usize, end: usize) -> usize {
    let start = start.min(source.len());
    let end = end.min(source.len());
    if start >= end {
        return 0;
    }
    let slice = &source[start..end];
    let len = slice.len();
    // 快速路径：计算 continuation bytes 的数量，然后用总长度减去
    // continuation byte 的特征是 10xxxxxx (0x80..0xBF)
    // 使用 chunks 让编译器自动向量化
    let mut continuation_count = 0usize;
    for chunk in slice.chunks(64) {
        for &b in chunk {
            continuation_count += ((b & 0xC0) == 0x80) as usize;
        }
    }
    len - continuation_count
}

impl Debug for Span<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Span {{ start: {}, end: {}, cursor: {}, content: {:?} }}",
            self.start,
            self.end,
            self.cursor,
            self.as_str()
        )
    }
}

impl fmt::Display for Span<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_single_line() {
        let mut scanner = Scanner::new("hello world");
        let span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.as_str(), "hello world");
        assert_eq!(span.full_str(), "hello world");
        assert!(!span.is_blank());
        assert!(!span.is_end());
        assert_eq!(span.indent_spaces(), 0);
        assert_eq!(span.indent_len(), 0);
    }

    #[test]
    fn test_extract_multiple_lines() {
        let mut scanner = Scanner::new("line1\nline2\nline3");
        let s1 = Span::extract(&mut scanner).unwrap();
        assert_eq!(s1.full_str(), "line1");
        let s2 = Span::extract(&mut scanner).unwrap();
        assert_eq!(s2.full_str(), "line2");
        let s3 = Span::extract(&mut scanner).unwrap();
        assert_eq!(s3.full_str(), "line3");
        assert!(Span::extract(&mut scanner).is_none());
    }

    #[test]
    fn test_extract_crlf() {
        let mut scanner = Scanner::new("line1\r\nline2");
        let s1 = Span::extract(&mut scanner).unwrap();
        assert_eq!(s1.full_str(), "line1");
        let s2 = Span::extract(&mut scanner).unwrap();
        assert_eq!(s2.full_str(), "line2");
    }

    #[test]
    fn test_blank_line() {
        let mut scanner = Scanner::new("  \t \nhello");
        let s1 = Span::extract(&mut scanner).unwrap();
        assert!(s1.is_blank());
        let s2 = Span::extract(&mut scanner).unwrap();
        assert!(!s2.is_blank());
    }

    #[test]
    fn test_indentation() {
        let mut scanner = Scanner::new("    code");
        let span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.indent_spaces(), 4);
        assert_eq!(span.indent_len(), 4);
        assert!(span.is_indented());
    }

    #[test]
    fn test_tab_indentation() {
        let mut scanner = Scanner::new("\tcode");
        let span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.indent_spaces(), 4);
        assert_eq!(span.indent_len(), 1);
        assert!(span.is_indented());
    }

    #[test]
    fn test_peek_and_next_byte() {
        let mut scanner = Scanner::new("abc");
        let mut span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.peek(), Some(b'a'));
        assert_eq!(span.next_byte(), Some(b'a'));
        assert_eq!(span.peek(), Some(b'b'));
        assert_eq!(span.next_byte(), Some(b'b'));
        assert_eq!(span.next_byte(), Some(b'c'));
        assert_eq!(span.next_byte(), None);
        assert!(span.is_end());
    }

    #[test]
    fn test_skip_indent() {
        let mut scanner = Scanner::new("  hello");
        let mut span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.indent_spaces(), 2);
        span.skip_indent();
        assert_eq!(span.as_str(), "hello");
        // Second call should be no-op
        span.skip_indent();
        assert_eq!(span.as_str(), "hello");
    }

    #[test]
    fn test_starts_with() {
        let mut scanner = Scanner::new("###heading");
        let span = Span::extract(&mut scanner).unwrap();
        assert!(span.starts_with(b'#', 3));
        assert!(!span.starts_with(b'#', 4));
    }

    #[test]
    fn test_starts_count() {
        let mut scanner = Scanner::new("---break");
        let span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.starts_count(b'-'), 3);
    }

    #[test]
    fn test_skip_and_consume() {
        let mut scanner = Scanner::new("## heading");
        let mut span = Span::extract(&mut scanner).unwrap();
        assert!(span.consume(b'#'));
        assert!(span.consume(b'#'));
        assert!(span.consume(b' '));
        assert_eq!(span.as_str(), "heading");
    }

    #[test]
    fn test_advance_next_nonspace() {
        let mut scanner = Scanner::new("   hello");
        let mut span = Span::extract(&mut scanner).unwrap();
        span.advance_next_nonspace();
        assert_eq!(span.as_str(), "hello");
    }

    #[test]
    fn test_trim_start_and_end_matches() {
        let mut scanner = Scanner::new("  hello  ");
        let mut span = Span::extract(&mut scanner).unwrap();
        span.trim_start_matches(|b| b == b' ');
        span.trim_end_matches(|b| b == b' ');
        assert_eq!(span.as_str(), "hello");
    }

    #[test]
    fn test_only_space_to_end() {
        let mut scanner = Scanner::new("   ");
        let span = Span::extract(&mut scanner).unwrap();
        assert!(span.only_space_to_end());

        let mut scanner2 = Scanner::new("  x");
        let span2 = Span::extract(&mut scanner2).unwrap();
        assert!(!span2.only_space_to_end());
    }

    #[test]
    fn test_snapshot_resume() {
        let mut scanner = Scanner::new("hello world");
        let mut span = Span::extract(&mut scanner).unwrap();
        let snap = span.snapshot();
        span.skip(5);
        assert_eq!(span.as_str(), " world");
        span.resume(&snap);
        assert_eq!(span.as_str(), "hello world");
    }

    #[test]
    fn test_re_find_indent() {
        let mut scanner = Scanner::new("> hello");
        let mut span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.indent_spaces(), 0);
        // Simulate consuming '>' and ' '
        span.skip(2);
        span.re_find_indent();
        assert_eq!(span.indent_spaces(), 0);
        assert_eq!(span.as_str(), "hello");
    }

    #[test]
    fn test_multibyte_utf8() {
        let mut scanner = Scanner::new("你好世界");
        let span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.full_str(), "你好世界");
        assert_eq!(span.len(), 12); // 4 chars × 3 bytes each
    }

    #[test]
    fn test_empty_input() {
        let mut scanner = Scanner::new("");
        assert!(Span::extract(&mut scanner).is_none());
    }

    #[test]
    fn test_empty_line() {
        let mut scanner = Scanner::new("\n");
        let span = Span::extract(&mut scanner).unwrap();
        assert!(span.is_blank());
        assert!(span.is_end());
        assert_eq!(span.len(), 0);
    }

    #[test]
    fn test_skip_spaces_method() {
        let mut scanner = Scanner::new("    code");
        let mut span = Span::extract(&mut scanner).unwrap();
        span.skip_spaces(4);
        assert_eq!(span.as_str(), "code");
    }

    #[test]
    fn test_get() {
        let mut scanner = Scanner::new("abc");
        let span = Span::extract(&mut scanner).unwrap();
        assert_eq!(span.get(0), Some(b'a'));
        assert_eq!(span.get(2), Some(b'c'));
        assert_eq!(span.get(3), None);
    }

    #[test]
    fn test_starts_with_bytes() {
        let mut scanner = Scanner::new("## heading");
        let span = Span::extract(&mut scanner).unwrap();
        assert!(span.starts_with_bytes(b"## "));
        assert!(!span.starts_with_bytes(b"### "));
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: performance-optimization, Property 2: Span 行提取正确性
    // **Validates: Requirements 2.2**
    //
    // For any UTF-8 string containing newlines, the content extracted line-by-line
    // via Span::extract SHALL be equivalent to the original input after removing
    // newline characters.
    proptest! {
        #[test]
        fn prop_span_extract_preserves_content(s in "\\PC{0,200}") {
            let mut scanner = Scanner::new(&s);
            let mut extracted_parts: Vec<&str> = Vec::new();

            while let Some(span) = Span::extract(&mut scanner) {
                extracted_parts.push(span.full_str());
            }

            // Reconstruct: join extracted lines (which exclude newlines)
            let reconstructed = extracted_parts.join("");

            // Original with all newline chars removed
            let original_no_newlines: String = s.chars()
                .filter(|c| *c != '\n' && *c != '\r')
                .collect();

            prop_assert_eq!(
                reconstructed,
                original_no_newlines,
                "Extracted lines joined should equal original without newlines"
            );
        }

        #[test]
        fn prop_span_extract_covers_all_input(s in "\\PC{0,200}") {
            let mut scanner = Scanner::new(&s);
            let mut total_bytes = 0usize;

            while let Some(span) = Span::extract(&mut scanner) {
                // Each span's full_str length contributes to total content bytes
                total_bytes += span.full_str().len();
            }

            // Total content bytes should equal input length minus newline bytes
            let newline_bytes: usize = s.bytes()
                .filter(|b| *b == b'\n' || *b == b'\r')
                .count();

            prop_assert_eq!(
                total_bytes,
                s.len() - newline_bytes,
                "Total extracted bytes should account for all non-newline input"
            );
        }

        #[test]
        fn prop_span_extract_valid_utf8(s in "\\PC{0,200}") {
            let mut scanner = Scanner::new(&s);

            while let Some(span) = Span::extract(&mut scanner) {
                // as_str() and full_str() must return valid UTF-8
                let full = span.full_str();
                prop_assert!(
                    std::str::from_utf8(full.as_bytes()).is_ok(),
                    "full_str() must be valid UTF-8"
                );
            }
        }
    }
}

/// 多个 Span 的合并视图，用于处理跨行的 inline 解析。
///
/// 与 `Span::merge` 不同，`MergedSpan` 保持每个 Span 的独立性，
/// 在 Span 之间自动插入换行符，避免包含中间行的前缀字符（如 blockquote 的 `>`）。
pub struct MergedSpan<'input> {
    /// 原始 Span 列表
    spans: Vec<Span<'input>>,
    /// 当前正在处理的 Span 索引
    current_span_index: usize,
    /// 是否在 Span 之间（需要插入换行符）
    at_span_boundary: bool,
}

impl<'input> MergedSpan<'input> {
    /// 从多个 Span 创建 MergedSpan
    pub fn new(spans: Vec<Span<'input>>) -> Self {
        MergedSpan {
            spans,
            current_span_index: 0,
            at_span_boundary: false,
        }
    }

    /// 从单个 Span 创建 MergedSpan
    pub fn from_single(span: Span<'input>) -> Self {
        MergedSpan {
            spans: vec![span],
            current_span_index: 0,
            at_span_boundary: false,
        }
    }

    /// 查看当前字节，不前进
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.at_span_boundary {
            return Some(b'\n');
        }
        if self.current_span_index >= self.spans.len() {
            return None;
        }
        let current = &self.spans[self.current_span_index];
        if let Some(byte) = current.peek() {
            Some(byte)
        } else if self.current_span_index + 1 < self.spans.len() {
            // 当前 Span 已结束，下一个字节是换行符
            Some(b'\n')
        } else {
            None
        }
    }

    /// 前进并返回当前字节
    #[inline]
    pub fn next_byte(&mut self) -> Option<u8> {
        if self.at_span_boundary {
            self.at_span_boundary = false;
            return Some(b'\n');
        }
        if self.current_span_index >= self.spans.len() {
            return None;
        }
        let current = &mut self.spans[self.current_span_index];
        if let Some(byte) = current.next_byte() {
            Some(byte)
        } else if self.current_span_index + 1 < self.spans.len() {
            // 当前 Span 已结束，移动到下一个 Span
            self.current_span_index += 1;
            Some(b'\n')
        } else {
            None
        }
    }

    /// 是否已消费完毕
    pub fn is_end(&self) -> bool {
        if self.at_span_boundary {
            return false;
        }
        self.current_span_index >= self.spans.len()
            || (self.current_span_index == self.spans.len() - 1
                && self.spans[self.current_span_index].is_end())
    }

    /// 获取从当前位置到末尾的字符串（用于调试）
    #[allow(unused)]
    #[cfg(debug_assertions)]
    pub fn as_str(&self) -> String {
        let mut result = String::new();
        for (i, span) in self.spans.iter().enumerate().skip(self.current_span_index) {
            if i == self.current_span_index {
                if self.at_span_boundary {
                    result.push('\n');
                }
                result.push_str(span.as_str());
            } else {
                result.push('\n');
                result.push_str(span.full_str());
            }
        }
        result
    }

    /// 快照当前状态（零分配）
    pub fn snapshot(&self) -> MergedSpanSnapshot {
        let current_span_cursor =
            if !self.at_span_boundary && self.current_span_index < self.spans.len() {
                self.spans[self.current_span_index].cursor()
            } else {
                0
            };
        MergedSpanSnapshot {
            current_span_index: self.current_span_index,
            at_span_boundary: self.at_span_boundary,
            current_span_cursor,
        }
    }

    /// 从快照恢复状态
    pub fn resume(&mut self, snapshot: &MergedSpanSnapshot) {
        let len = self.spans.len();
        if snapshot.current_span_index >= len {
            for span in &mut self.spans {
                span.cursor = span.end();
            }
            self.current_span_index = len;
            self.at_span_boundary = false;
            return;
        }

        if snapshot.at_span_boundary {
            for (idx, span) in self.spans.iter_mut().enumerate() {
                if idx < snapshot.current_span_index {
                    span.cursor = span.end();
                } else {
                    span.cursor = span.start();
                }
            }
        } else {
            for (idx, span) in self.spans.iter_mut().enumerate() {
                if idx < snapshot.current_span_index {
                    span.cursor = span.end();
                } else if idx == snapshot.current_span_index {
                    span.cursor = snapshot.current_span_cursor.min(span.end());
                } else {
                    span.cursor = span.start();
                }
            }
        }
        self.current_span_index = snapshot.current_span_index;
        self.at_span_boundary = snapshot.at_span_boundary;
    }

    /// 获取当前位置的 Location
    pub fn start_location(&self) -> Location {
        if self.current_span_index >= self.spans.len() {
            // 已结束，返回最后一个 Span 的结束位置
            if let Some(last) = self.spans.last() {
                return last.last_token_end_location();
            }
            return Location::new(1, 1);
        }
        self.spans[self.current_span_index].start_location()
    }

    /// 获取当前位置的结束 Location
    pub fn end_location(&self) -> Location {
        if self.current_span_index >= self.spans.len() {
            if let Some(last) = self.spans.last() {
                return last.last_token_end_location();
            }
            return Location::new(1, 1);
        }
        self.spans[self.current_span_index].end_location()
    }

    /// 获取当前 Span 的引用（用于需要访问底层 Span 的场景）
    pub fn current_span(&self) -> Option<&Span<'input>> {
        if self.current_span_index < self.spans.len() {
            Some(&self.spans[self.current_span_index])
        } else {
            None
        }
    }

    /// 获取当前 Span 的可变引用
    pub fn current_span_mut(&mut self) -> Option<&mut Span<'input>> {
        if self.current_span_index < self.spans.len() {
            Some(&mut self.spans[self.current_span_index])
        } else {
            None
        }
    }
}

/// MergedSpan 的快照（零分配）
#[derive(Clone)]
pub struct MergedSpanSnapshot {
    current_span_index: usize,
    at_span_boundary: bool,
    /// 当前 Span 的 cursor（仅在 !at_span_boundary 时有效）
    current_span_cursor: usize,
}

impl Debug for MergedSpanSnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MergedSpanSnapshot {{ index: {}, at_boundary: {} }}",
            self.current_span_index, self.at_span_boundary
        )
    }
}

impl<'input> MergedSpan<'input> {
    /// 获取当前绝对字节偏移（与 Span API 对齐）
    pub fn cursor(&self) -> usize {
        if self.at_span_boundary {
            if self.current_span_index > 0 {
                return self.spans[self.current_span_index - 1].end();
            }
            return 0;
        }
        self.current_span().map(|s| s.cursor()).unwrap_or(0)
    }

    /// 获取当前 Span 的起始偏移
    pub fn start(&self) -> usize {
        self.current_span().map(|s| s.start()).unwrap_or(0)
    }

    /// 获取当前 Span 的结束偏移
    pub fn end(&self) -> usize {
        self.current_span().map(|s| s.end()).unwrap_or(0)
    }

    /// 获取原始 source 字节切片
    pub fn source_slice(&self) -> &'input [u8] {
        static EMPTY: [u8; 0] = [];
        self.current_span()
            .map(|s| s.source_slice())
            .unwrap_or(&EMPTY)
    }

    /// 跳过指定数量的字节（跨 Span 自动处理）
    pub fn skip(&mut self, len: usize) -> &mut Self {
        if len == 0 {
            return self;
        }
        // 快速路径：如果在当前 Span 内且不在边界上，直接调整 cursor
        if !self.at_span_boundary && self.current_span_index < self.spans.len() {
            let span = &mut self.spans[self.current_span_index];
            let remaining_in_span = span.end().saturating_sub(span.cursor());
            if len <= remaining_in_span {
                span.skip(len);
                return self;
            }
        }
        // 慢速路径：逐字节跳过（处理跨 Span 边界的情况）
        for _ in 0..len {
            if self.next_byte().is_none() {
                break;
            }
        }
        self
    }

    /// 获取从当前游标开始连续相同字节的数量（支持跨 Span 边界）
    pub fn starts_count(&self, byte: u8) -> usize {
        // 快速路径：在当前 Span 内直接扫描
        if !self.at_span_boundary && self.current_span_index < self.spans.len() {
            let span = &self.spans[self.current_span_index];
            let count = span.starts_count(byte);
            // 如果没有扫到 Span 末尾，说明结果就在当前 Span 内
            if span.cursor() + count < span.end() {
                return count;
            }
            // 扫到了 Span 末尾，需要继续检查后续 Span
            let mut total = count;
            for span_idx in (self.current_span_index + 1)..self.spans.len() {
                // Span 之间有换行符
                if byte != b'\n' {
                    return total;
                }
                total += 1; // 换行符
                let next_span = &self.spans[span_idx];
                let source = next_span.source_slice();
                let mut pos = next_span.start();
                let end = next_span.end();
                while pos < end && source[pos] == byte {
                    total += 1;
                    pos += 1;
                }
                if pos < end {
                    return total;
                }
            }
            return total;
        }
        // 慢速路径
        let mut count = 0;
        loop {
            match self.get(count) {
                Some(b) if b == byte => count += 1,
                _ => break,
            }
        }
        count
    }

    /// 获取指定偏移处的字节（相对于当前游标）
    pub fn get(&self, index: usize) -> Option<u8> {
        if self.current_span_index >= self.spans.len() {
            return None;
        }
        // 快速路径：在当前 Span 内
        if !self.at_span_boundary {
            let span = &self.spans[self.current_span_index];
            let abs_pos = span.cursor() + index;
            if abs_pos < span.end() {
                return Some(span.source_slice()[abs_pos]);
            }
        }
        // 慢速路径：跨 Span
        let mut remaining = index;

        if self.at_span_boundary {
            if remaining == 0 {
                return Some(b'\n');
            }
            remaining -= 1;
        }

        for span_idx in self.current_span_index..self.spans.len() {
            let span = &self.spans[span_idx];
            let local_start = if span_idx == self.current_span_index {
                span.cursor()
            } else {
                span.start()
            };
            let local_len = span.end().saturating_sub(local_start);
            if remaining < local_len {
                return Some(span.source_slice()[local_start + remaining]);
            }
            remaining = remaining.saturating_sub(local_len);
            if span_idx + 1 < self.spans.len() {
                if remaining == 0 {
                    return Some(b'\n');
                }
                remaining -= 1;
            }
        }
        None
    }

    /// 检查指定偏移处的字节是否匹配（相对于当前游标）
    pub fn validate(&self, index: usize, byte: u8) -> bool {
        self.get(index) == Some(byte)
    }

    /// 检查指定偏移处的字节是否满足谓词（相对于当前游标）
    pub fn validate_with<P>(&self, index: usize, predicate: P) -> bool
    where
        P: Fn(u8) -> bool,
    {
        self.get(index).map(predicate).unwrap_or(false)
    }

    // /// 获取行最后一个字符的结束 Location
    // pub fn last_token_end_location(&self) -> Location {
    //     if let Some(span) = self.current_span() {
    //         span.last_token_end_location()
    //     } else if let Some(last) = self.spans.last() {
    //         last.last_token_end_location()
    //     } else {
    //         Location::new(1, 1)
    //     }
    // }

    /// 从绝对字节偏移创建子 Span（用于 delimiter 等场景）
    pub fn slice_from_abs(&self, abs_start: usize, abs_end: usize) -> Span<'input> {
        self.current_span()
            .map(|s| s.slice_from_abs(abs_start, abs_end))
            .unwrap_or_else(Span::empty)
    }

    // /// 如果当前字节匹配则消费
    // pub fn consume(&mut self, byte: u8) -> bool {
    //     if self.peek() == Some(byte) {
    //         self.next_byte();
    //         true
    //     } else {
    //         false
    //     }
    // }

    // /// 手动前进游标一个字节（用于跳过 UTF-8 continuation bytes）
    // pub fn advance_cursor(&mut self) {
    //     if let Some(span) = self.current_span_mut() {
    //         span.advance_cursor();
    //     }
    // }

    /// 获取指定绝对字节偏移处的 Location
    pub fn location_at_byte(&self, byte_pos: usize) -> Location {
        // 找到包含该字节偏移的 Span
        for span in &self.spans {
            if byte_pos >= span.start() && byte_pos <= span.end() {
                return span.location_at_byte(byte_pos);
            }
        }
        // 如果找不到，返回最后一个 Span 的结束位置
        if let Some(last) = self.spans.last() {
            last.last_token_end_location()
        } else {
            Location::new(1, 1)
        }
    }
}

#[cfg(test)]
mod merged_span_tests {
    use super::{MergedSpan, Scanner, Span};

    #[test]
    fn test_snapshot_resume_across_spans() {
        let mut scanner = Scanner::new("a\nb\nc");
        let s1 = Span::extract(&mut scanner).unwrap();
        let s2 = Span::extract(&mut scanner).unwrap();
        let s3 = Span::extract(&mut scanner).unwrap();
        let mut merged = MergedSpan::new(vec![s1, s2, s3]);

        let snapshot = merged.snapshot();
        while merged.next_byte().is_some() {}
        merged.resume(&snapshot);

        assert_eq!(merged.peek(), Some(b'a'));
        assert_eq!(merged.next_byte(), Some(b'a'));
        assert_eq!(merged.next_byte(), Some(b'\n'));
        assert_eq!(merged.next_byte(), Some(b'b'));
    }
}
