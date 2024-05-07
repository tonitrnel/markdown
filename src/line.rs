use crate::tokenizer::{Location, Token, TokenIterator, TokenWithLocation, Whitespace};
use std::fmt::{self, Debug, Display, Formatter, Write};
use std::iter::{Skip, Take};
use std::ops::{Index, Range};
use std::slice::Iter;

#[derive(Clone, Default)]
struct IndentationInfo {
    /// 一个 Tab 等同 4 个 spaces
    total_spaces: u16,
    /// tokens length
    len: usize,
    skipped: bool,
}

impl<'input> IndentationInfo {
    fn new<I>(iter: I, len: usize) -> Self
    where
        I: Iterator<Item = &'input TokenWithLocation<'input>>,
    {
        let mut total_tokens = 0;
        let mut total_spaces = 0;
        for (i, item) in iter.enumerate() {
            match &item.token {
                Token::Whitespace(Whitespace::Space) => total_spaces += 1,
                Token::Whitespace(Whitespace::Tab) => total_spaces += 4,
                _ => {
                    total_tokens = i;
                    break;
                }
            }
        }
        if total_tokens == 0 && total_spaces > 0 {
            total_tokens = len;
        }
        Self {
            total_spaces,
            len: total_tokens,
            skipped: false,
        }
    }
}

pub struct Line<'input> {
    inner: Vec<TokenWithLocation<'input>>,
    indentation: IndentationInfo,
    blank: bool,
    start_location: Location,
    pub(super) start_offset: usize,
    pub(super) end_offset: usize,
}
impl<'input> Line<'input> {
    /// 从 TokenIterator 中提取一行
    pub fn extract(iter: &mut TokenIterator<'input>) -> Option<Self> {
        let mut tokens = Vec::<TokenWithLocation<'input>>::new();
        for it in iter {
            match &it.token {
                Token::Whitespace(Whitespace::NewLine(_)) => {
                    let start_location = tokens
                        .first()
                        .map(|it| it.location)
                        .unwrap_or_else(|| it.location);
                    return Some(Line::new_with_search_next_nonspace(tokens, start_location));
                }
                _ => tokens.push(it),
            }
        }
        if tokens.is_empty() {
            None
        } else {
            let start_location = tokens[0].location;
            Some(Line::new_with_search_next_nonspace(tokens, start_location))
        }
    }
    pub fn new(inner: Vec<TokenWithLocation<'input>>) -> Self {
        Self {
            start_offset: 0,
            end_offset: inner.len(),
            inner,
            start_location: Location::default(),
            indentation: IndentationInfo::default(),
            blank: false,
        }
    }
    pub fn new_with_search_next_nonspace(
        inner: Vec<TokenWithLocation<'input>>,
        start_location: Location,
    ) -> Self {
        let indentation = IndentationInfo::new(inner.iter(), inner.len());
        let blank = indentation.len == inner.len();
        Self {
            start_offset: 0,
            end_offset: inner.len(),
            inner,
            start_location,
            indentation,
            blank,
        }
    }
    pub fn extends(lines: Vec<Line<'input>>) -> Self {
        let mut tokens = Vec::new();
        let len = lines.len();
        for (idx, line) in lines.into_iter().enumerate() {
            let is_end = idx + 1 == len;
            tokens.extend_from_slice(&line.inner[line.start_offset..line.end_offset]);
            if is_end {
                break;
            }
            if let Some(last) = tokens.last().map(|it| it.end_location()) {
                tokens.push(TokenWithLocation {
                    token: Whitespace::NewLine("\n").into(),
                    location: Location::new(last.line, last.column + 1),
                })
            };
        }
        Self::new(tokens)
    }
    /// 该函数将丢弃一定数量的Token并更新 indent
    ///
    /// 用于容器嵌套时
    pub fn re_find_indent(&mut self) {
        self.indentation = IndentationInfo::new(self.iter(), self.len());
        self.blank = self.indentation.len == self.len();
    }
    /// 当前行长度
    pub fn len(&self) -> usize {
        self.end_offset.saturating_sub(self.start_offset)
    }
    pub fn iter(&self) -> Take<Skip<Iter<'_, TokenWithLocation<'input>>>> {
        self.inner
            .iter()
            .skip(self.start_offset)
            .take(self.end_offset - self.start_offset)
    }
    pub fn starts_with(&self, token: &Token, len: usize) -> bool {
        if self.len() < len {
            return false;
        }
        self.iter().take(len).all(|it| &it.token == token)
    }
    pub fn ends_with(&self, token: &Token, len: usize) -> bool {
        self.iter()
            .skip(self.len() - len)
            .take(len)
            .all(|it| &it.token == token)
    }
    pub fn starts_count_matches<P>(&self, pat: P) -> usize
    where
        P: Fn(&Token) -> bool,
    {
        self.iter().take_while(|it| pat(&it.token)).count()
    }
    pub fn starts_with_matches<P>(&self, pat: P, len: usize) -> bool
    where
        P: Fn(&Token) -> bool,
    {
        self.iter().take(len).all(|it| pat(&it.token))
    }
    pub fn ends_with_matches<P>(&self, pat: P, len: usize) -> bool
    where
        P: Fn(&Token) -> bool,
    {
        self.iter()
            .skip(self.len() - len)
            .take(len)
            .all(|it| pat(&it.token))
    }
    /// 获取从当前位置开始和指定token相同的数量
    pub fn starts_count(&self, token: &Token) -> usize {
        self.iter().take_while(|it| &it.token == token).count()
    }
    pub fn ends_count_matches<P>(&self, pat: P) -> usize
    where
        P: Fn(&Token) -> bool,
    {
        self.iter().rev().take_while(|it| pat(&it.token)).count()
    }
    pub fn trim(&self) -> Self {
        let len = self.end_offset - self.start_offset;
        let first = self
            .iter()
            .position(|it| !it.is_space_or_tab())
            .unwrap_or(0);
        let last = self
            .iter()
            .rposition(|it| !it.is_space_or_tab())
            .unwrap_or(len);
        Self::new(
            self.iter()
                .skip(first)
                .take(last + 1 - first)
                .cloned()
                .collect::<Vec<_>>(),
        )
    }
    pub fn trim_start_matches<P>(&mut self, pat: P) -> &Self
    where
        P: Fn(&Token) -> bool,
    {
        let count = self.starts_count_matches(pat);
        self.start_offset += count;
        self
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
    pub fn only_space_to_end(&self) -> bool {
        if self.is_end() {
            return true;
        }
        for item in self.iter() {
            if item.is_newline() {
                return true;
            }
            if item.is_space_or_tab() {
                continue;
            }
            return false;
        }
        true
    }

    pub fn peek(&self) -> Option<&Token<'input>> {
        self.peek_with_location().map(|it| &it.token)
    }
    pub fn peek_with_location(&self) -> Option<&TokenWithLocation<'input>> {
        if self.start_offset >= self.end_offset {
            return None;
        }
        self.inner.get(self.start_offset)
    }
    pub fn next(&mut self) -> Option<Token<'input>> {
        let val = self.peek().cloned();
        if self.start_offset < self.end_offset {
            self.start_offset += 1;
        }
        val
    }
    pub fn next_with_location(&mut self) -> Option<TokenWithLocation<'input>> {
        let val = self.peek_with_location().cloned();
        if self.start_offset < self.end_offset {
            self.start_offset += 1;
        }
        val
    }
    /// 跳过连续相同的 Tokens
    pub fn skip_consecutive_tokens(&mut self, token: &Token) {
        let count = self.starts_count(token);
        self.start_offset += count;
    }
    /// 找到传入 Token Iter 的第一个非空白位置
    pub fn find_next_nonspace<'a, I>(mut tokens: I) -> Option<usize>
    where
        I: Iterator<Item = &'a TokenWithLocation<'a>>,
    {
        tokens.position(|it| !matches!(it.token, Token::Whitespace(..)))
    }
    pub fn position<P: ConsumePredicate<'input> + Copy>(&self, predicate: P) -> Option<usize> {
        self.iter().position(|it| predicate.evaluate(it.token))
    }
    /// 跳过指定长度的 Tokens
    pub fn skip(&mut self, len: usize) -> &mut Self {
        self.start_offset += len;
        self
    }
    /// 跳过指定的空白字符数量
    ///
    /// 这将计算空白字符数量对应的 Tokens 数量并跳过
    pub fn skip_spaces(&mut self, len: usize) -> &mut Self {
        if len == 0 {
            return self;
        }
        let mut len2 = len;
        let mut end = 0;
        for (i, item) in self.iter().enumerate() {
            match &item.token {
                Token::Whitespace(ws @ (Whitespace::Space | Whitespace::Tab)) => {
                    if ws.spaces_len() > len2 {
                        end = i;
                        break;
                    }
                    len2 -= ws.spaces_len();
                }
                _ => {
                    end = i;
                    break;
                }
            }
        }
        if end == 0 && len2 != len {
            end = self.len();
        }
        self.start_offset += end;
        self
    }
    /// 跳过缩进
    pub fn skip_indent(&mut self) -> &mut Self {
        if self.indentation.skipped {
            return self;
        }
        // 当 start_offset > self.indent 可能是父级容器重新查找 indent 的原因，因此需要添加 skipped_indent 进行判断防止重复调用
        if self.start_offset >= self.indentation.len {
            self.start_offset += self.indentation.len;
        } else {
            self.start_offset = self.indentation.len;
        }
        self.indentation.skipped = true;
        self
    }
    /// 跳至行结束，等同与标记该行已结束
    pub fn skip_to_end(&mut self) {
        self.start_offset = self.end_offset;
    }
    /// 如果下一个 Token 断定为 true 则消费，否则什么也不做
    pub fn consume<P: ConsumePredicate<'input>>(&mut self, predicate: P) -> bool {
        if let Some(next) = self.peek() {
            if predicate.evaluate(next) {
                self.start_offset += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// 基于当前位置验证指定偏移的 Token 是否与之相符合
    pub fn validate<P: ConsumePredicate<'input>>(&self, index: usize, predicate: P) -> bool {
        self.get(index)
            .map(|it| predicate.evaluate(it))
            .unwrap_or(false)
    }
    /// 前进到下一个非空白的 token 字符
    pub fn advance_next_nonspace(&mut self) -> &Self {
        let next_nonspace = self
            .iter()
            .position(|it| !it.is_space_or_tab())
            .unwrap_or(0);
        if next_nonspace > 0 {
            self.start_offset += next_nonspace;
        }
        self
    }
    /// 重置 Line，这会清除偏移、缩进等信息
    pub fn reset(&mut self) -> &Self {
        self.start_offset = 0;
        self.end_offset = self.inner.len();
        self.indentation = IndentationInfo::new(self.inner.iter(), self.inner.len());
        self
    }
    /// 行已全部消费
    pub fn is_end(&self) -> bool {
        self.start_offset >= self.end_offset
    }
    /// 空白行，该行不包含任何内容或全部为红白字符
    pub fn is_blank(&self) -> bool {
        self.blank
    }
    /// 缩进是否大于 4 个，如果是应该使用 IndentedCode 解析
    pub fn is_indented(&self) -> bool {
        self.indentation.total_spaces >= 4
    }
    /// 获取缩进的空白字符数量，一个 `\t` 将等于 `4` 个空白字符数量
    pub fn indent_spaces(&self) -> usize {
        self.indentation.total_spaces as usize
    }
    /// 获取缩进的 Token 数量，即每行开头的 `\x20` 和 `\t`
    pub fn indent_len(&self) -> usize {
        self.indentation.len
    }
    /// 根据指定的 start 和 end 创建一个切片副本，如果要忽略偏移请使用 `slice_raw`
    pub fn slice(&self, start: usize, end: usize) -> Line<'input> {
        // 有没有不用克隆直接使用 start_offset 和 end_offset 创建切片的方法？
        Line::new(
            self.iter()
                .skip(start)
                .take(end.saturating_sub(start))
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
        LineSnapshot(
            Range {
                start: self.start_offset,
                end: self.end_offset,
            },
            self.indentation.clone(),
        )
    }
    /// 从快照恢复到之前的位置
    pub fn resume(&mut self, snapshot: impl AsRef<LineSnapshot>) -> &mut Self {
        let snapshot = snapshot.as_ref();
        self.start_offset = snapshot.0.start;
        self.end_offset = snapshot.0.end;
        self.indentation.total_spaces = snapshot.1.total_spaces;
        self.indentation.len = snapshot.1.len;
        self.indentation.skipped = snapshot.1.skipped;
        self
    }
    /// 安全的获取引用
    pub fn get(&self, index: usize) -> Option<&Token<'input>> {
        let index = self.start_offset + index;
        if index >= self.end_offset {
            return None;
        }
        self.inner.get(index).map(|it| &it.token)
    }
    /// 从原始 vector 安全的获取引用，等同于 vector 的 `get`
    pub fn get_raw(&self, index: usize) -> Option<&Token<'input>> {
        self.inner.get(index).map(|it| &it.token)
    }

    /// 获取当前Token的开始位置
    pub fn start_location(&self) -> Location {
        if self.end_offset == 0 {
            return self.start_location;
        }
        if self.is_end() {
            self.inner[self.end_offset - 1].end_location()
        } else {
            self.inner[self.start_offset.min(self.end_offset - 1)].start_location()
        }
    }
    /// 获取当前Token的结束位置
    pub fn end_location(&self) -> Location {
        if self.end_offset == 0 {
            let mut end_location = self.start_location();
            end_location.column += 1;
            return end_location;
        }
        self.inner[self.start_offset.min(self.end_offset - 1)].end_location()
    }
    /// 获取当前行最后一个 Token 的结束位置
    pub fn last_token_end_location(&self) -> Location {
        if self.end_offset == 0 {
            self.end_location()
        } else {
            self.inner[self.end_offset - 1].end_location()
        }
    }
    /// Escape specified characters
    pub fn to_escaped_string(&self, escaped_chars: &[char]) -> String {
        let mut buf = String::new();
        for item in self.iter() {
            match item.token {
                Token::Escaped(ch) if escaped_chars.contains(&ch) => buf.write_char(ch),
                Token::Escaped(ch) => buf.write_char('\\').and_then(|_| buf.write_char(ch)),
                _ => item.token.write(&mut buf),
            }
            .expect("unexpected error")
        }
        buf
    }
    /// Ignore backslash escape
    pub fn to_unescape_string(&self) -> String {
        let mut buf = String::new();
        self.write_string(&mut buf, false)
            .expect("unexpected error");
        buf
    }
    pub(crate) fn write_string<W>(&self, writer: &mut W, escape: bool) -> fmt::Result
    where
        W: Write,
    {
        for item in self.iter() {
            match item.token {
                Token::Escaped(ch) if !escape => {
                    writer.write_char('\\')?;
                    writer.write_char(ch)?;
                }
                _ => item.token.write(writer)?,
            }
        }
        Ok(())
    }
}
impl Display for Line<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_string(f, true)
    }
}

impl Debug for Line<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            &self.inner[self.start_offset..self.end_offset]
                .iter()
                .map(|it| it.token)
                .collect::<Vec<_>>()
        )
    }
}

pub trait ConsumePredicate<'input> {
    fn evaluate(self, token: impl AsRef<Token<'input>>) -> bool;
}

impl<'input> ConsumePredicate<'input> for Token<'input> {
    fn evaluate(self, token: impl AsRef<Token<'input>>) -> bool {
        &self == token.as_ref()
    }
}
impl<'input> ConsumePredicate<'input> for &Token<'input> {
    fn evaluate(self, token: impl AsRef<Token<'input>>) -> bool {
        self == token.as_ref()
    }
}

impl<'input, F> ConsumePredicate<'input> for F
where
    F: Fn(&Token<'input>) -> bool,
{
    fn evaluate(self, token: impl AsRef<Token<'input>>) -> bool {
        let token = token.as_ref();
        self(token)
    }
}

impl AsRef<LineSnapshot> for LineSnapshot {
    fn as_ref(&self) -> &LineSnapshot {
        self
    }
}
impl<'input> Index<usize> for Line<'input> {
    type Output = TokenWithLocation<'input>;
    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(self.start_offset + index)
    }
}
#[derive(Clone)]
pub struct LineSnapshot(Range<usize>, IndentationInfo);
impl Debug for LineSnapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LineSnapshot {{ start: {}, end: {} }}",
            self.0.start, self.0.end
        )
    }
}

#[cfg(feature = "serde_yaml")]
pub struct TokenIteratorGuard<'a, 'input> {
    committed: bool,
    pub original: &'a mut TokenIterator<'input>,
    snapshot: TokenIterator<'input>,
}

#[cfg(feature = "serde_yaml")]
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

#[cfg(feature = "serde_yaml")]
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
                    1 => assert!(matches!(line.peek(), Some(Token::Text("abcdefgh")))),
                    2 => assert!(matches!(line.peek(), Some(Token::Digit("1256648483541")))),
                    3 => assert!(matches!(line.peek(), Some(Token::Crosshatch))),
                    4 => assert!(matches!(line.peek(), Some(Token::Text("sadfrasg")))),
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
                    2 => assert!(matches!(line.peek(), Some(Token::Digit("1256648483541")))),
                    3 => assert!(matches!(line.peek(), Some(Token::Crosshatch))),
                    4 => assert!(matches!(line.peek(), Some(Token::Text("sadfrasg")))),
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
        assert_eq!(cp2[0].token, Token::Digit("5"));
        assert_eq!(cp2[2].token, Token::Crosshatch);
    }

    #[test]
    fn test_peek_and_next() {
        let mut tokens = Tokenizer::new("r12你5%#").tokenize();
        let mut line = Line::extract(&mut tokens).unwrap();
        assert_eq!(line.peek(), Some(&Token::Text("r")));
        assert_eq!(line.next(), Some(Token::Text("r")));
        assert_eq!(line.peek(), Some(&Token::Digit("12")));
        assert_eq!(line.peek(), Some(&Token::Digit("12")));
        assert_eq!(line.next(), Some(Token::Digit("12")));
        assert_eq!(line.peek(), Some(&Token::Text("你")));
    }
}
