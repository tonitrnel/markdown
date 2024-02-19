use crate::parser::{Node, Parser};
use crate::tokenizer::{Token, TokenIterator, TokenWithLocation, Whitespace};
use std::fmt::{Display, Formatter};
use std::ops::{Index, Range};

mod block_quote;
mod code;
mod heading;

pub use block_quote::*;
pub use code::*;
pub use heading::*;

pub enum BlockMatching {
    Unmatched = 0,
    MatchedContainer,
    MatchedLeaf,
}
pub enum BlockProcessing {
    Unprocessed = 0,
    Processed,
    Further,
}
pub struct Line<'input> {
    inner: Vec<TokenWithLocation<'input>>,
    start_offset: usize,
    end_offset: usize,
    pub indent: usize,
}

impl<'input> Line<'input> {
    pub fn extract(iter: &mut TokenIterator<'input>) -> Option<Self> {
        let mut tokens = Vec::new();
        for it in iter {
            match &it.token {
                Token::Whitespace(Whitespace::NewLine(_)) => return Some(Line::new(tokens)),
                _ => tokens.push(it),
            }
        }
        if !tokens.is_empty() {
            Some(Line::new(tokens))
        } else {
            None
        }
    }
    pub fn new(tokens: Vec<TokenWithLocation<'input>>) -> Self {
        let next_nonspace = Line::find_next_nonspace(tokens.iter());
        Self {
            start_offset: 0,
            end_offset: tokens.len(),
            indent: next_nonspace,
            inner: tokens,
        }
    }
    pub fn len(&self) -> usize {
        self.end_offset - self.start_offset
    }
    pub fn starts_with(&self, token: &Token, len: usize) -> bool {
        self.inner
            .iter()
            .skip(self.start_offset)
            .take(len)
            .all(|it| &it.token == token)
    }
    pub fn ends_with(&self, token: &Token, len: usize) -> bool {
        self.inner
            .iter()
            .skip(self.end_offset - len)
            .take(len)
            .all(|it| &it.token == token)
    }
    pub fn starts_with_matches<P>(&self, pat: P, len: usize) -> bool
    where
        P: Fn(&Token) -> bool,
    {
        self.inner
            .iter()
            .skip(self.start_offset)
            .take(len)
            .all(|it| pat(&it.token))
    }
    pub fn ends_with_matches<P>(&self, pat: P, len: usize) -> bool
    where
        P: Fn(&Token) -> bool,
    {
        self.inner
            .iter()
            .skip(self.end_offset - len)
            .take(len)
            .all(|it| pat(&it.token))
    }
    pub fn starts_count(&self, token: &Token) -> usize {
        self.inner
            .iter()
            .skip(self.start_offset)
            .take_while(|it| &it.token == token)
            .count()
    }
    pub fn ends_count_matches<P>(&self, pat: P) -> usize
    where
        P: Fn(&Token) -> bool,
    {
        self.inner
            .iter()
            .skip(self.start_offset)
            .take(self.end_offset - self.start_offset)
            .rev()
            .take_while(|it| pat(&it.token))
            .count()
    }
    pub fn trim_end_matches<P>(&mut self, pat: P) -> &Self
    where
        P: Fn(&Token) -> bool,
    {
        let count = self.ends_count_matches(pat);
        self.end_offset -= count;
        self
    }
    pub fn ensure_only_spaces_to_end(&self) -> bool {
        self.inner
            .iter()
            .skip(self.start_offset)
            .take(self.end_offset - self.start_offset)
            .all(|it| it.is_space_or_tab())
    }

    pub fn next_nonspace(&self) -> usize {
        Line::find_next_nonspace(self.inner.iter().skip(self.start_offset))
    }

    pub fn peek(&self) -> Option<&TokenWithLocation> {
        self.inner.get(self.start_offset)
    }
    pub fn next(&mut self) -> Option<&TokenWithLocation> {
        self.start_offset += 1;
        self.peek()
    }
    pub fn skip_consecutive_tokens(&mut self, token: &Token) {
        let count = self.starts_count(token);
        self.start_offset += count;
    }
    pub fn find_next_nonspace<'a, I>(mut tokens: I) -> usize
    where
        I: Iterator<Item = &'a TokenWithLocation<'a>>,
    {
        tokens
            .position(|it| {
                !matches!(
                    it.token,
                    Token::Whitespace(Whitespace::Space | Whitespace::Tab)
                )
            })
            .unwrap_or(0)
    }
    pub fn skip(&mut self, len: usize) -> &Self {
        self.start_offset += len;
        self
    }
    pub fn advance_next_nonspace(&mut self) -> &Self {
        let next_nonspace = Line::find_next_nonspace(self.inner.iter().skip(self.start_offset));
        if next_nonspace > 0 {
            self.start_offset += next_nonspace;
        }
        self
    }
    pub fn skip_indent(&mut self) -> &Self {
        if self.start_offset >= self.indent {
            return self;
        }
        self.start_offset = self.indent;
        return self;
    }
    pub fn reset(&mut self) -> &Self {
        self.start_offset = 0;
        self.end_offset = self.inner.len();
        self
    }
    pub fn is_end(&self) -> bool {
        self.start_offset >= self.end_offset
    }
    pub fn indented(&self) -> bool {
        self.indent >= 4
    }
    pub fn slice(&self, start: usize, end: usize) -> Line<'input> {
        self.slice_range(Range { start, end })
    }
    pub fn slice_range(&self, range: Range<usize>) -> Line<'input> {
        Line::new(
            self.inner
                .iter()
                .skip(range.start)
                .take(range.end - range.start)
                .cloned()
                .collect::<Vec<_>>(),
        )
    }
}
impl Display for Line<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .inner
                .iter()
                .skip(self.start_offset)
                .take(self.end_offset - self.start_offset)
                .map(|it| it.to_string())
                .collect::<String>(),
        )
    }
}

impl<'input> Index<usize> for Line<'input> {
    type Output = TokenWithLocation<'input>;
    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(self.start_offset + index)
    }
}

pub trait BlockProcessStage {
    /// 初始化容器
    ///
    /// 该函数将检查 Line 是否符合当前 Block 定义，如果符合则向 Parser Tree 上创建当前 Block
    ///
    /// 返回值:
    /// - `BlockMatching::Unmatched` 不匹配该 Block 定义
    /// - `BlockMatching::MatchedLeaf` 已匹配并且创建了 Block，该 Block 不支持嵌套其他 Block
    /// - `BlockMatching::MatchedContainer` 已匹配并且创建了 Block，该 Block 支持嵌套其他 Block，需要进一步拆分
    fn initiate<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockMatching;

    /// 继续处理
    ///
    /// 该函数将为未关闭的 Block 进行处理
    ///
    /// 返回值：
    /// - `BlockProcessing::Unprocessed` 未处理，后续步骤应该退出当前容器
    /// - `BlockProcessing::Processed` 已处理，后续步骤也应该退出当前容器
    /// - `BlockProcessing::Further` 可以继续处理
    fn process<'input>(parser: &mut Parser<'input>, line: &mut Line<'input>) -> BlockProcessing;
    #[allow(unused_variables)]
    fn finalize(parser: &mut Parser, node: Node) {}
}
