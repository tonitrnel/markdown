use crate::parser::Location;

/// 字节扫描器，直接在输入字节切片上操作，替代 Tokenizer 中间层。
///
/// Scanner 通过字节偏移进行模式匹配，不生成中间 Token 枚举集合，
/// 支持 UTF-8 多字节字符的正确边界检测。
pub struct Scanner<'input> {
    /// 原始输入（字节切片）
    source: &'input [u8],
    /// 原始输入字符串引用（用于 UTF-8 安全切片）
    source_str: &'input str,
    /// 当前扫描位置（字节偏移）
    pos: usize,
    /// 当前行号（从 1 开始）
    line: u64,
    /// 当前列号（从 1 开始，按字符计数）
    col: u64,
}

/// Scanner 快照，用于保存和恢复扫描位置
#[derive(Debug, Clone, Copy)]
pub struct ScannerSnapshot {
    pos: usize,
    line: u64,
    col: u64,
}

impl<'input> Scanner<'input> {
    /// 从 UTF-8 字符串创建 Scanner
    pub fn new(source: &'input str) -> Self {
        Self {
            source: source.as_bytes(),
            source_str: source,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// 查看当前字节，不前进
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        self.source.get(self.pos).copied()
    }

    /// 查看相对当前位置偏移 offset 处的字节
    #[inline]
    pub fn peek_at(&self, offset: usize) -> Option<u8> {
        self.source.get(self.pos + offset).copied()
    }

    /// 前进一个字节并返回该字节
    #[inline]
    pub fn advance(&mut self) -> Option<u8> {
        let byte = self.source.get(self.pos).copied();
        if let Some(b) = byte {
            self.pos += 1;
            if b == b'\n' {
                self.line += 1;
                self.col = 1;
            } else if !is_utf8_continuation(b) {
                self.col += 1;
            }
        }
        byte
    }

    /// 前进 n 个字节（更新行列信息）
    #[inline]
    pub fn advance_by(&mut self, n: usize) {
        let end = (self.pos + n).min(self.source.len());
        while self.pos < end {
            let b = self.source[self.pos];
            self.pos += 1;
            if b == b'\n' {
                self.line += 1;
                self.col = 1;
            } else if !is_utf8_continuation(b) {
                self.col += 1;
            }
        }
    }

    /// 当前字节偏移位置
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// 剩余未扫描的字节数
    #[inline]
    pub fn remaining(&self) -> usize {
        self.source.len().saturating_sub(self.pos)
    }

    /// 获取从 start 到 end 的字符串切片。
    ///
    /// 自动调整边界到有效的 UTF-8 字符边界，确保不在多字节字符中间截断。
    /// 如果调整后范围无效，返回空字符串。
    pub fn slice(&self, start: usize, end: usize) -> &'input str {
        let len = self.source.len();
        let start = start.min(len);
        let end = end.min(len);
        if start >= end {
            return "";
        }
        // 向前调整 start 到字符边界
        let start = self.adjust_to_char_boundary_forward(start);
        // 向后调整 end 到字符边界
        let end = self.adjust_to_char_boundary_backward(end);
        if start >= end {
            return "";
        }
        // SAFETY: source 来自 &str，且 start/end 已调整到字符边界
        unsafe { std::str::from_utf8_unchecked(&self.source[start..end]) }
    }

    /// 跳过空白字符（空格和制表符），返回跳过的空白数量（tab 计为 1）
    pub fn skip_spaces(&mut self) -> usize {
        let start = self.pos;
        while let Some(b) = self.peek() {
            match b {
                b' ' | b'\t' => {
                    self.pos += 1;
                    self.col += 1;
                }
                _ => break,
            }
        }
        self.pos - start
    }

    /// 跳过到行尾（换行符或输入末尾），返回行尾位置
    pub fn skip_to_eol(&mut self) -> usize {
        let remaining = &self.source[self.pos..];
        // 使用 memchr2 同时搜索 \n 和 \r，利用 SIMD 加速
        let pos = match memchr::memchr2(b'\n', b'\r', remaining) {
            Some(offset) => self.pos + offset,
            None => self.source.len(),
        };
        self.pos = pos;
        pos
    }

    /// 检查当前位置是否以指定字节序列开头
    #[inline]
    pub fn starts_with(&self, needle: &[u8]) -> bool {
        self.source[self.pos..].starts_with(needle)
    }

    /// 计算当前位置开始连续相同字节的数量
    pub fn count_consecutive(&self, byte: u8) -> usize {
        let mut count = 0;
        let mut pos = self.pos;
        while pos < self.source.len() && self.source[pos] == byte {
            count += 1;
            pos += 1;
        }
        count
    }

    /// 检查指定位置是否在 UTF-8 字符边界上
    #[inline]
    pub fn is_char_boundary(&self, pos: usize) -> bool {
        if pos >= self.source.len() {
            return pos == self.source.len();
        }
        // UTF-8 continuation bytes have the pattern 10xxxxxx (0x80..0xBF)
        // A position is a char boundary if the byte is NOT a continuation byte
        !is_utf8_continuation(self.source[pos])
    }

    /// 获取原始字节切片引用
    #[inline]
    pub fn source(&self) -> &'input [u8] {
        self.source
    }

    /// 获取原始字符串引用
    #[inline]
    pub fn source_str(&self) -> &'input str {
        self.source_str
    }

    /// 获取当前 Location（行号、列号）
    #[inline]
    pub fn location(&self) -> Location {
        Location::new(self.line, self.col)
    }

    /// 获取当前行号
    #[inline]
    pub fn line_number(&self) -> u64 {
        self.line
    }

    /// 创建当前位置的快照
    #[inline]
    pub fn snapshot(&self) -> ScannerSnapshot {
        ScannerSnapshot {
            pos: self.pos,
            line: self.line,
            col: self.col,
        }
    }

    /// 恢复到之前保存的快照位置
    #[inline]
    pub fn resume(&mut self, snapshot: &ScannerSnapshot) {
        self.pos = snapshot.pos;
        self.line = snapshot.line;
        self.col = snapshot.col;
    }

    /// 直接设置位置（不更新行列信息，仅用于特殊场景）
    #[inline]
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos.min(self.source.len());
    }

    /// 向前查找最近的字符边界（>= pos）
    fn adjust_to_char_boundary_forward(&self, pos: usize) -> usize {
        let mut p = pos;
        while p < self.source.len() && is_utf8_continuation(self.source[p]) {
            p += 1;
        }
        p
    }

    /// 向后查找最近的字符边界（<= pos）
    fn adjust_to_char_boundary_backward(&self, pos: usize) -> usize {
        let mut p = pos;
        // pos at or past end is always a valid boundary
        if p >= self.source.len() {
            return self.source.len();
        }
        while p > 0 && is_utf8_continuation(self.source[p]) {
            p -= 1;
        }
        p
    }
}

/// 判断字节是否为 UTF-8 continuation byte (10xxxxxx)
#[inline]
fn is_utf8_continuation(byte: u8) -> bool {
    (byte & 0xC0) == 0x80
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_basic_ops() {
        let s = Scanner::new("hello");
        assert_eq!(s.pos(), 0);
        assert_eq!(s.remaining(), 5);
        assert_eq!(s.peek(), Some(b'h'));
    }

    #[test]
    fn test_advance() {
        let mut s = Scanner::new("abc");
        assert_eq!(s.advance(), Some(b'a'));
        assert_eq!(s.advance(), Some(b'b'));
        assert_eq!(s.advance(), Some(b'c'));
        assert_eq!(s.advance(), None);
        assert_eq!(s.pos(), 3);
        assert_eq!(s.remaining(), 0);
    }

    #[test]
    fn test_peek_at() {
        let s = Scanner::new("abcd");
        assert_eq!(s.peek_at(0), Some(b'a'));
        assert_eq!(s.peek_at(2), Some(b'c'));
        assert_eq!(s.peek_at(10), None);
    }

    #[test]
    fn test_advance_by() {
        let mut s = Scanner::new("hello world");
        s.advance_by(5);
        assert_eq!(s.pos(), 5);
        assert_eq!(s.peek(), Some(b' '));
        // advance_by past end clamps to len
        s.advance_by(100);
        assert_eq!(s.pos(), 11);
        assert_eq!(s.remaining(), 0);
    }

    #[test]
    fn test_slice_ascii() {
        let s = Scanner::new("hello world");
        assert_eq!(s.slice(0, 5), "hello");
        assert_eq!(s.slice(6, 11), "world");
        assert_eq!(s.slice(5, 5), "");
        assert_eq!(s.slice(10, 5), "");
    }

    #[test]
    fn test_slice_multibyte() {
        // "你好" is 6 bytes: \xe4\xbd\xa0\xe5\xa5\xbd
        let s = Scanner::new("你好world");
        assert_eq!(s.slice(0, 6), "你好");
        assert_eq!(s.slice(6, 11), "world");
        // Slicing in the middle of a multibyte char adjusts boundaries
        assert_eq!(s.slice(1, 6), "好"); // adjusts start forward to byte 3
        assert_eq!(s.slice(0, 4), "你"); // adjusts end backward to byte 3
    }

    #[test]
    fn test_skip_spaces() {
        let mut s = Scanner::new("   hello");
        assert_eq!(s.skip_spaces(), 3);
        assert_eq!(s.peek(), Some(b'h'));

        let mut s2 = Scanner::new("\t\thello");
        assert_eq!(s2.skip_spaces(), 2);
        assert_eq!(s2.peek(), Some(b'h'));

        let mut s3 = Scanner::new("hello");
        assert_eq!(s3.skip_spaces(), 0);
    }

    #[test]
    fn test_skip_to_eol() {
        let mut s = Scanner::new("hello\nworld");
        let eol = s.skip_to_eol();
        assert_eq!(eol, 5);
        assert_eq!(s.peek(), Some(b'\n'));

        let mut s2 = Scanner::new("no newline");
        let eol2 = s2.skip_to_eol();
        assert_eq!(eol2, 10);
        assert_eq!(s2.peek(), None);
    }

    #[test]
    fn test_starts_with() {
        let s = Scanner::new("## Heading");
        assert!(s.starts_with(b"##"));
        assert!(s.starts_with(b"## "));
        assert!(!s.starts_with(b"###"));
    }

    #[test]
    fn test_count_consecutive() {
        let s = Scanner::new("###heading");
        assert_eq!(s.count_consecutive(b'#'), 3);

        let s2 = Scanner::new("---");
        assert_eq!(s2.count_consecutive(b'-'), 3);

        let s3 = Scanner::new("hello");
        assert_eq!(s3.count_consecutive(b'#'), 0);
    }

    #[test]
    fn test_is_char_boundary() {
        let s = Scanner::new("a你b");
        // 'a' at 0
        assert!(s.is_char_boundary(0));
        // '你' starts at 1
        assert!(s.is_char_boundary(1));
        // continuation bytes at 2, 3
        assert!(!s.is_char_boundary(2));
        assert!(!s.is_char_boundary(3));
        // 'b' at 4
        assert!(s.is_char_boundary(4));
        // past end
        assert!(s.is_char_boundary(5));
        assert!(!s.is_char_boundary(6));
    }

    #[test]
    fn test_empty_input() {
        let mut s = Scanner::new("");
        assert_eq!(s.peek(), None);
        assert_eq!(s.advance(), None);
        assert_eq!(s.remaining(), 0);
        assert_eq!(s.skip_spaces(), 0);
        assert_eq!(s.count_consecutive(b' '), 0);
        assert_eq!(s.slice(0, 0), "");
    }

    #[test]
    fn test_crlf() {
        let mut s = Scanner::new("line1\r\nline2");
        let eol = s.skip_to_eol();
        assert_eq!(eol, 5);
        assert_eq!(s.peek(), Some(b'\r'));
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: performance-optimization, Property 1: Scanner UTF-8 边界正确性
    // **Validates: Requirements 1.4**
    //
    // For any valid UTF-8 string input, all byte slices produced by Scanner::slice
    // SHALL lie on valid UTF-8 character boundaries, i.e. std::str::from_utf8(slice)
    // returns Ok for all slices.
    proptest! {
        #[test]
        fn prop_scanner_slice_utf8_boundary(s in "\\PC{0,200}") {
            let scanner = Scanner::new(&s);
            let len = s.len();

            // Test all possible (start, end) pairs at a coarser granularity
            // to keep runtime reasonable
            let step = if len > 20 { len / 10 } else { 1 };
            let mut start = 0;
            while start <= len {
                let mut end = start;
                while end <= len {
                    let slice = scanner.slice(start, end);
                    // The slice must be valid UTF-8 (it's &str so this is guaranteed
                    // by construction, but let's verify the bytes match)
                    prop_assert!(
                        std::str::from_utf8(slice.as_bytes()).is_ok(),
                        "slice({}, {}) produced invalid UTF-8",
                        start,
                        end
                    );
                    end += step.max(1);
                }
                start += step.max(1);
            }
        }

        #[test]
        fn prop_scanner_is_char_boundary_consistent(s in "\\PC{0,200}") {
            let scanner = Scanner::new(&s);
            // Scanner::is_char_boundary must agree with str::is_char_boundary
            // for all positions within the string length
            for pos in 0..=s.len() {
                prop_assert_eq!(
                    scanner.is_char_boundary(pos),
                    s.is_char_boundary(pos),
                    "is_char_boundary({}) disagrees with str::is_char_boundary",
                    pos
                );
            }
        }

        #[test]
        fn prop_scanner_skip_to_eol_preserves_boundary(s in "\\PC{0,100}") {
            let mut scanner = Scanner::new(&s);
            let eol_pos = scanner.skip_to_eol();
            // eol_pos must be a valid char boundary
            prop_assert!(
                s.is_char_boundary(eol_pos),
                "skip_to_eol returned {} which is not a char boundary",
                eol_pos
            );
        }
    }
}
