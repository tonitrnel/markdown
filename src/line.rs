use crate::tokenizer::{Location, Token, TokenIterator, TokenWithLocation, Whitespace};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, Range};

pub struct Line<'input> {
    inner: Vec<TokenWithLocation<'input>>,
    pub start_offset: usize,
    pub end_offset: usize,
    pub indent: usize,
}

impl<'input> Line<'input> {
    /// 从 TokenIterator 中提取一行
    pub fn extract(iter: &mut TokenIterator<'input>) -> Option<Self> {
        let mut tokens = Vec::new();
        for it in iter {
            match &it.token {
                Token::Whitespace(Whitespace::NewLine(_)) => return Some(Line::new(tokens)),
                _ => tokens.push(it),
            }
        }
        if tokens.is_empty() {
            None
        } else {
            Some(Line::new(tokens))
        }
    }
    pub fn new(tokens: Vec<TokenWithLocation<'input>>) -> Self {
        let next_nonspace = Line::find_next_nonspace(tokens.iter()).unwrap_or(tokens.len());
        Self {
            start_offset: 0,
            end_offset: tokens.len(),
            indent: next_nonspace,
            inner: tokens,
        }
    }
    /// 当前行长度
    pub fn len(&self) -> usize {
        self.end_offset.saturating_sub(self.start_offset)
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
    /// 获取从当前位置开始和指定token相同的数量
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
            .take(self.end_offset.saturating_sub(self.start_offset))
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
    /// 确保仅空白到当前行结束
    pub fn only_spaces_to_end(&self) -> bool {
        self.inner
            .iter()
            .skip(self.start_offset)
            .take(self.end_offset.saturating_sub(self.start_offset))
            .all(|it| it.is_space_or_tab())
    }

    pub fn next_nonspace(&self) -> usize {
        Line::find_next_nonspace(self.inner.iter().skip(self.start_offset)).unwrap_or(0)
    }

    pub fn peek(&self) -> Option<&TokenWithLocation> {
        self.inner.get(self.start_offset)
    }
    pub fn next(&mut self) -> Option<&TokenWithLocation> {
        self.start_offset += 1;
        self.peek()
    }
    /// 跳过连续相同的 Tokens
    pub fn skip_consecutive_tokens(&mut self, token: &Token) {
        let count = self.starts_count(token);
        self.start_offset += count;
    }
    pub fn find_next_nonspace<'a, I>(mut tokens: I) -> Option<usize>
    where
        I: Iterator<Item = &'a TokenWithLocation<'a>>,
    {
        tokens.position(|it| !it.is_space_or_tab())
    }
    /// 跳过指定长度的 Tokens
    pub fn skip(&mut self, len: usize) -> &mut Self {
        self.start_offset += len;
        self
    }
    /// 跳过指定长度的空白 Tokens，如果长度不足则将忽略
    pub fn skip_spaces(&mut self, len: usize) -> &mut Self {
        for i in self.start_offset..self.start_offset + len {
            if self.get(i).map(|it| it.is_space_or_tab()).unwrap_or(false) {
                self.start_offset = i + 1;
            } else {
                break;
            }
        }
        self
    }
    /// 如果下一个 Token 断定为 true 则消费，否则什么也不做
    pub fn consume(&mut self, predicate: impl FnOnce(&TokenWithLocation) -> bool) -> bool {
        if let Some(next) = self.peek() {
            if predicate(next) {
                self.start_offset += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// 前进到下一个非空白的 token 字符
    pub fn advance_next_nonspace(&mut self) -> &Self {
        let next_nonspace =
            Line::find_next_nonspace(self.inner.iter().skip(self.start_offset)).unwrap_or(0);
        if next_nonspace > 0 {
            self.start_offset += next_nonspace;
        }
        self
    }
    /// 跳过缩进
    pub fn skip_indent(&mut self) -> &mut Self {
        if self.start_offset >= self.indent {
            return self;
        }
        self.start_offset = self.indent;
        self
    }
    pub fn reset(&mut self) -> &Self {
        self.start_offset = 0;
        self.end_offset = self.inner.len();
        self
    }
    /// 行已全部消费
    pub fn is_end(&self) -> bool {
        self.start_offset >= self.end_offset
    }
    /// 空白行，该行不包含任何内容或全部为红白字符
    pub fn is_blank(&self) -> bool {
        self.len() == 0 || self.indent >= self.end_offset
    }
    /// 缩进是否大于 4 个，如果是应该使用 IndentedCode 解析
    pub fn is_indented(&self) -> bool {
        self.indent >= 4
    }
    /// 根据指定的 start 和 end 创建一个切片副本，如果要忽略偏移请使用 `slice_raw`
    pub fn slice(&self, start: usize, end: usize) -> Line<'input> {
        // 有没有不用克隆直接使用 start_offset 和 end_offset 创建切片的方法？
        Line::new(
            self.inner
                .iter()
                .skip(self.start_offset + start)
                .take(end.min(self.len()).saturating_sub(start))
                .cloned()
                .collect::<Vec<_>>(),
        )
    }
    /// 忽略偏移，从原始 vector 上进行切片
    pub fn slice_raw(&self, start: usize, end: usize) -> Line<'input> {
        Line::new(
            self.inner
                .iter()
                .skip(start)
                .take(end - start)
                .cloned()
                .collect(),
        )
    }
    /// 快照当前位置
    pub fn snapshot(&self) -> LineSnapshot {
        LineSnapshot(Range {
            start: self.start_offset,
            end: self.end_offset,
        })
    }
    /// 从快照恢复到之前的位置
    pub fn resume(&mut self, snapshot: &LineSnapshot) -> &mut Self {
        self.start_offset = snapshot.0.start;
        self.end_offset = snapshot.0.end;
        self
    }
    /// 获取当前位置
    pub fn location(&self) -> Location {
        self.inner[self.start_offset].location
    }

    /// 从原始 vector 安全的获取引用，等同于 vector 的 `get`
    pub fn get(&self, index: usize) -> Option<&TokenWithLocation<'input>> {
        self.inner.get(index)
    }
}
impl Display for Line<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .inner
                .iter()
                .skip(self.start_offset)
                .take(self.end_offset.saturating_sub(self.start_offset))
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

pub struct LineSnapshot(Range<usize>);
impl Debug for LineSnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LineSnapshot {{ start: {}, end: {} }}",
            self.0.start, self.0.end
        )
    }
}

pub struct TokenIteratorGuard<'a, 'input> {
    committed: bool,
    pub original: &'a mut TokenIterator<'input>,
    snapshot: TokenIterator<'input>,
}

impl<'a, 'input> TokenIteratorGuard<'a, 'input> {
    pub fn new(original: &'a mut TokenIterator<'input>) -> Self {
        TokenIteratorGuard {
            committed: false,
            snapshot: original.clone(),
            original,
        }
    }
    pub fn commit(&mut self) {
        self.committed = true;
    }
    pub fn line(&mut self) -> Option<Line<'input>> {
        Line::extract(self.original)
    }
}

impl<'a, 'input> Drop for TokenIteratorGuard<'a, 'input> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        *self.original = self.snapshot.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_token_iterator_guard() {
        let mut tokens = Tokenizer::new("abcdefgh\n1256648483541\n#5rr32@334\nsadfrasg").tokenize();
        {
            let mut guard = TokenIteratorGuard::new(&mut tokens);
            let mut i = 0;
            while let Some(line) = guard.line() {
                i += 1;
                match i {
                    1 => assert!(matches!(
                        line.peek(),
                        Some(TokenWithLocation {
                            token: Token::Text("abcdefgh"),
                            ..
                        })
                    )),
                    2 => assert!(matches!(
                        line.peek(),
                        Some(TokenWithLocation {
                            token: Token::Number("1256648483541"),
                            ..
                        })
                    )),
                    3 => assert!(matches!(
                        line.peek(),
                        Some(TokenWithLocation {
                            token: Token::Crosshatch,
                            ..
                        })
                    )),
                    4 => assert!(matches!(
                        line.peek(),
                        Some(TokenWithLocation {
                            token: Token::Text("sadfrasg"),
                            ..
                        })
                    )),
                    _ => panic!("unexpected line"),
                }
            }
        }
        assert!(matches!(
            &tokens.next(),
            Some(TokenWithLocation {
                token: Token::Text("abcdefgh"),
                ..
            })
        ));
        {
            let mut guard = TokenIteratorGuard::new(&mut tokens);
            let mut i = 0;
            while let Some(line) = guard.line() {
                i += 1;
                match i {
                    1 => assert!(line.is_blank()),
                    2 => assert!(matches!(
                        line.peek(),
                        Some(TokenWithLocation {
                            token: Token::Number("1256648483541"),
                            ..
                        })
                    )),
                    3 => assert!(matches!(
                        line.peek(),
                        Some(TokenWithLocation {
                            token: Token::Crosshatch,
                            ..
                        })
                    )),
                    4 => assert!(matches!(
                        line.peek(),
                        Some(TokenWithLocation {
                            token: Token::Text("sadfrasg"),
                            ..
                        })
                    )),
                    _ => panic!("unexpected line"),
                }
            }
            guard.commit();
        }
        assert!(tokens.next().is_none());
    }

    #[test]
    fn test_slice() {
        let mut tokens = Tokenizer::new("r12你5%#").tokenize();
        let line = Line::extract(&mut tokens).unwrap();
        assert_eq!(line.len(), 6);
        assert_eq!(line[0].token, Token::Text("r"));
        assert_eq!(line[5].token, Token::Crosshatch);
        let cp1 = line.slice(0, 3);
        assert_eq!(cp1.len(), 3);
        assert_eq!(cp1[0].token, Token::Text("r"));
        assert_eq!(cp1[2].token, Token::Text("你"));
        let cp2 = line.slice(3, 6);
        assert_eq!(cp2.len(), 3);
        assert_eq!(cp2[0].token, Token::Number("5"));
        assert_eq!(cp2[2].token, Token::Crosshatch);
    }
}
