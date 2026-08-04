use serde::Serialize;
use std::fmt::{Debug, Formatter};

/// A location in source text, with line and column numbers starting at one.
#[derive(Serialize, Eq, PartialEq, Clone, Copy)]
pub struct Location {
    /// Line number, starting from 1
    pub line: u64,
    /// Line column, starting from 1
    pub column: u64,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl Location {
    /// Creates a one-based source location.
    pub fn new(line: u64, column: u64) -> Self {
        Self { line, column }
    }

    /// 构建行起点索引：`line_starts[i]` = 第 `i+1` 行首字节偏移（首行恒为 0）。
    ///
    /// 与 Scanner 行号语义构造性一致：行号仅在 `\n` 之后递增（孤立 `\r` 不增行）。
    pub(crate) fn build_line_starts(source: &str) -> Vec<u32> {
        let bytes = source.as_bytes();
        let mut starts = Vec::with_capacity(bytes.len() / 32 + 2);
        starts.push(0u32);
        for i in memchr::memchr_iter(b'\n', bytes) {
            starts.push((i + 1) as u32);
        }
        starts
    }
}
