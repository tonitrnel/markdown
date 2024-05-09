use crate::utils;
use serde::Serialize;
use std::fmt;
use std::iter::Peekable;

#[derive(Serialize, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Whitespace<'input> {
    /// 空格
    Space,
    /// 换行
    NewLine(&'input str),
    /// 制表符
    Tab,
}
impl Whitespace<'_> {
    pub fn len(&self) -> usize {
        match self {
            Whitespace::Space => 1,
            Whitespace::Tab => 1,
            Whitespace::NewLine(s) => s.len(),
        }
    }
    pub fn spaces_len(&self) -> usize {
        match self {
            Whitespace::Space => 1,
            Whitespace::Tab => 4,
            _ => 0,
        }
    }
}

impl fmt::Display for Whitespace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::NewLine(s) => f.write_str(s),
            Whitespace::Tab => f.write_str("\t"),
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Token<'input> {
    /// Text
    Text(&'input str),
    /// Digit 0-9 **Not Include decimal**
    Digit(&'input str),
    /// Whitespace
    Whitespace(Whitespace<'input>),
    /// Crosshatch `#`
    Crosshatch,
    /// Asterisk `*`
    Asterisk,
    /// Underscore `_`
    Underscore,
    /// Tilde `~`
    Tilde,
    /// Left bracket `[`
    LBracket,
    /// Double left bracket `[[`
    DoubleLBracket,
    /// Right bracket `]`
    RBracket,
    /// Double right bracket `]]`
    DoubleRBracket,
    /// Left Parenthesis `(`
    LParen,
    /// Right Parenthesis `)`
    RParen,
    /// Left brace `{`
    LBrace,
    /// Right brace `}`
    RBrace,
    /// DoublePercent `%%`
    DoublePercent,
    /// BackQuote `` ` ``
    Backtick,
    /// Equal `=`
    Eq,
    /// And `&`
    Ampersand,
    /// Caret `^`
    Caret,
    /// Pipe `|`
    Pipe,
    /// ExclamationMark `!`
    ExclamationMark,
    /// Hyphen `-`
    Hyphen,
    /// Plus `+`
    Plus,
    /// Less than `<`
    Lt,
    /// Greater than `>`
    Gt,
    /// Dollar `$`
    Dollar,
    /// Colon `:`
    Colon,
    /// DoubleQuote `'`
    SingleQuote,
    /// DoubleQuote `"`
    DoubleQuote,
    /// Question `?`
    Question,
    /// Semicolon `;`
    Semicolon,
    /// Period `.`
    Period,
    /// Slash `/`
    Slash,
    /// Backslash `\`
    Backslash,
    /// Escaped
    ///
    /// No included ascii control characters and whitespace characters
    ///
    /// `impl Display` logic:
    /// - `is_ascii_punctuation`: Display escaped `char`
    /// - other: Display `\` + escaped `char`
    Escaped(char),
    /// More ASCII Control 0x00 - 0x20
    ///
    /// No included `Token::Whitespace`
    Control(char),
    /// More ASCII Punctuation `0x21 - 0x2F`,` 0x3A - 0x40` and Unicode
    ///
    /// Exclude `Token::Text | Token::Digest | Token::Escaped | Token::Whitespace`
    Punctuation(char),
}
impl Token<'_> {
    pub fn len(&self) -> usize {
        match self {
            Token::Text(s) => s.chars().count(),
            Token::Digit(s) => s.len(),
            Token::Whitespace(ws) => ws.len(),
            Token::DoubleLBracket
            | Token::DoubleRBracket
            | Token::DoublePercent
            | Token::Escaped(..) => 2,
            _ => 1,
        }
    }
    pub fn is_space_or_tab(&self) -> bool {
        matches!(self, Token::Whitespace(Whitespace::Space | Whitespace::Tab))
    }
    pub fn is_newline(&self) -> bool {
        matches!(self, Token::Whitespace(Whitespace::NewLine(..)))
    }
    /// 是用于 Markdown Block 相关的 Token
    pub fn is_block_special_token(&self) -> bool {
        matches!(
            self,
            // ATX Heading
            Token::Crosshatch
                // Fenced code
                | Token::Backtick
                | Token::Tilde
                // Thematic breaks
                | Token::Asterisk
                | Token::Underscore
                | Token::Plus
                | Token::Eq
                // HTML Tag
                | Token::Lt
                | Token::Gt
                | Token::Digit(..)
                | Token::Hyphen
                // Table
                | Token::Pipe
                | Token::Colon
                // Footnote
                | Token::LBracket
        )
    }
    // pub fn is_special_char(ch: &char) -> bool {
    //     matches!(
    //         ch,
    //         '#' | '`'
    //             | '~'
    //             | '*'
    //             | '_'
    //             | '+'
    //             | '-'
    //             | '='
    //             | '<'
    //             | '>'
    //             | '|'
    //             | ':'
    //             | '!'
    //             | '&'
    //             | '['
    //             | ']'
    //             | '.'
    //             | '\\'
    //     )
    // }

    pub(crate) fn is_anything_space(&self) -> bool {
        let ch = match self {
            Token::Escaped(_) => return false,
            Token::Control(ch) => Some(*ch),
            Token::Text(str) => str.chars().last(),
            Token::Digit(_) => return false,
            Token::Whitespace(..) => return true,
            _ => return false,
        };
        if let Some(uc) = ch.map(|it| it as u32) {
            uc == 9
                || uc == 10
                || uc == 12
                || uc == 13
                || uc == 32
                || uc == 160
                || uc == 5760
                || (8192..=8202).contains(&uc)
                || uc == 8239
                || uc == 8287
                || uc == 12288
        } else {
            false
        }
    }
    // Checks if the value is a UTF8 punctuation character:
    pub(crate) fn is_punctuation(&self) -> bool {
        match self {
            Token::Escaped(ch) => utils::is_punctuation(*ch),
            Token::Text(_) => false,
            Token::Control(_) | Token::Digit(_) | Token::Whitespace(_) => false,
            _ => true,
        }
    }
    // Checks if the value is an ASCII control character
    //
    // Codepoint: 0x00 - 0x20
    pub(crate) fn is_control(&self) -> bool {
        matches!(self, Token::Whitespace(_) | Token::Control(_))
    }
    pub(crate) fn is_ascii_alphanumeric(&self) -> bool {
        match self {
            Token::Text(_) => self.is_ascii_alphabetic(),
            Token::Digit(..) => true,
            _ => false,
        }
    }
    pub(crate) fn is_ascii_alphabetic(&self) -> bool {
        match self {
            Token::Text(str) => str.chars().all(|it| it.is_ascii_alphabetic()),
            _ => false,
        }
    }
    /// 判断 Token 是否存在于给定的字符串
    ///
    /// 仅对可转换为 `char` 的 `Token` 生效，其他永远返回 `false`
    pub(crate) fn in_str(&self, str: &str) -> bool {
        str.chars().any(|ch| self.eq(&ch))
    }
    pub(crate) fn write<W>(&self, writer: &mut W) -> fmt::Result
        where
            W: fmt::Write,
    {
        match self {
            Token::Text(str) => write!(writer, "{str}"),
            Token::Digit(str) => write!(writer, "{str}"),
            Token::Whitespace(ws) => write!(writer, "{ws}"),
            Token::DoubleLBracket => writer.write_str("[["),
            Token::DoubleRBracket => writer.write_str("]]"),
            Token::Crosshatch => writer.write_char('#'),
            Token::Asterisk => writer.write_char('*'),
            Token::Underscore => writer.write_char('_'),
            Token::Tilde => writer.write_char('~'),
            Token::LBracket => writer.write_char('['),
            Token::RBracket => writer.write_char(']'),
            Token::LParen => writer.write_char('('),
            Token::RParen => writer.write_char(')'),
            Token::LBrace => writer.write_char('{'),
            Token::RBrace => writer.write_char('}'),
            Token::DoublePercent => writer.write_str("%%"),
            Token::Backtick => writer.write_char('`'),
            Token::Eq => writer.write_char('='),
            Token::Ampersand => writer.write_char('&'),
            Token::Caret => writer.write_char('^'),
            Token::Pipe => writer.write_char('|'),
            Token::ExclamationMark => writer.write_char('!'),
            Token::Hyphen => writer.write_char('-'),
            Token::Plus => writer.write_char('+'),
            Token::Lt => writer.write_char('<'),
            Token::Gt => writer.write_char('>'),
            Token::Dollar => writer.write_char('$'),
            Token::Colon => writer.write_char(':'),
            Token::SingleQuote => writer.write_char('\''),
            Token::DoubleQuote => writer.write_char('"'),
            Token::Question => writer.write_char('?'),
            Token::Semicolon => writer.write_char(';'),
            Token::Period => writer.write_char('.'),
            Token::Slash => writer.write_char('/'),
            Token::Backslash => writer.write_char('\\'),
            Token::Escaped(ch) => {
                if ch.is_ascii_punctuation() {
                    writer.write_char(*ch)
                } else {
                    writer.write_char('\\')?;
                    writer.write_char(*ch)
                }
            }
            Token::Control(ch) => {
                if ch == &'\u{0000}' {
                    writer.write_char('\u{FFFD}')
                } else {
                    writer.write_char(*ch)
                }
            }
            Token::Punctuation(ch) => writer.write_char(*ch),
        }
    }
}
impl<'input> AsRef<Token<'input>> for Token<'input> {
    fn as_ref(&self) -> &Token<'input> {
        self
    }
}
impl<'input> TryFrom<&Token<'input>> for char {
    type Error = ();
    fn try_from(value: &Token<'input>) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::Crosshatch => '#',
            Token::Asterisk => '*',
            Token::Underscore => '_',
            Token::Tilde => '~',
            Token::LBracket => '[',
            Token::RBracket => ']',
            Token::LParen => '(',
            Token::RParen => ')',
            Token::LBrace => '{',
            Token::RBrace => '}',
            Token::Backtick => '`',
            Token::Eq => '=',
            Token::Ampersand => '&',
            Token::Caret => '^',
            Token::Pipe => '|',
            Token::ExclamationMark => '!',
            Token::Hyphen => '-',
            Token::Plus => '+',
            Token::Lt => '<',
            Token::Gt => '>',
            Token::Dollar => '$',
            Token::Colon => ':',
            Token::SingleQuote => '\'',
            Token::DoubleQuote => '"',
            Token::Question => '?',
            Token::Semicolon => ';',
            Token::Period => '.',
            Token::Slash => '/',
            Token::Backslash => '\\',
            Token::Escaped(ch) => *ch,
            Token::Control(ch) => *ch,
            Token::Punctuation(ch) => *ch,
            Token::Text(..)
            | Token::Digit(..)
            | Token::Whitespace(..)
            | Token::DoubleRBracket
            | Token::DoubleLBracket
            | Token::DoublePercent => return Err(()),
        })
    }
}
impl<'input> PartialEq<char> for Token<'input> {
    fn eq(&self, other: &char) -> bool {
        match self {
            Token::Text(_) | Token::Digit(_) | Token::Escaped(_) => false,
            Token::Control(ch) => ch == other,
            Token::Punctuation(ch) => ch == other,
            _ => char::try_from(self).map(|ch| &ch == other).unwrap_or(false),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)
    }
}
impl<'input> From<Whitespace<'input>> for Token<'input> {
    fn from(value: Whitespace<'input>) -> Self {
        Token::Whitespace(value)
    }
}

#[derive(Serialize, Eq, PartialEq, Clone, Copy)]
pub struct Location {
    /// Line number, starting from 1
    pub line: u64,
    /// Line column, starting from 1
    pub column: u64,
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}
impl Location {
    pub fn new(line: u64, column: u64) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TokenWithLocation<'input> {
    pub token: Token<'input>,
    pub location: Location,
}

impl TokenWithLocation<'_> {
    /// 是空白或制表符
    #[inline]
    pub fn is_space_or_tab(&self) -> bool {
        self.token.is_space_or_tab()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.token.len()
    }
    #[inline]
    pub fn start_location(&self) -> Location {
        self.location
    }
    #[inline]
    pub fn end_location(&self) -> Location {
        Location {
            line: self.location.line,
            column: self.location.column + self.token.len() as u64,
        }
    }
}

impl fmt::Display for TokenWithLocation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.token.fmt(f)
    }
}

impl<'input> PartialEq<Token<'input>> for TokenWithLocation<'input> {
    fn eq(&self, other: &Token) -> bool {
        &self.token == other
    }
}

#[derive(Clone)]
pub struct StatefulChars<'a> {
    content: &'a str,
    inner: Peekable<std::str::Chars<'a>>,
    pos: usize,
    pub line: u64,
    pub col: u64,
}

impl<'input> StatefulChars<'input> {
    fn new(content: &'input str) -> Self {
        let location = Location::default();
        Self {
            content,
            inner: content.chars().peekable(),
            pos: 0,
            col: location.column,
            line: location.line,
        }
    }
    fn next(&mut self) -> Option<char> {
        match self.inner.next() {
            None => None,
            Some(s) => {
                self.pos += s.len_utf8();
                if s == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1
                }
                Some(s)
            }
        }
    }
    fn peek(&mut self) -> Option<&char> {
        self.inner.peek()
    }
    fn location(&self) -> Location {
        Location {
            line: self.line,
            column: self.col,
        }
    }
    fn skip(&mut self, len: usize) {
        let end = self.pos + len;
        while self.next().is_some() {
            if self.pos >= end {
                break;
            }
        }
    }
}

pub struct Tokenizer<'input> {
    text: &'input str,
}

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str) -> Self {
        Tokenizer { text }
    }
    pub fn tokenize(self) -> TokenIterator<'input> {
        let chars = StatefulChars::new(self.text);
        TokenIterator { chars }
    }
}

#[derive(Clone)]
pub struct TokenIterator<'input> {
    chars: StatefulChars<'input>,
}

fn next_token<'input>(chars: &mut StatefulChars<'input>, recursion: bool) -> Option<Token<'input>> {
    let &ch = chars.peek()?;
    match ch {
        ' ' => consume_and_return(chars, Token::Whitespace(Whitespace::Space)),
        '\t' => consume_and_return(chars, Token::Whitespace(Whitespace::Tab)),
        '\n' => consume_and_return(chars, Token::Whitespace(Whitespace::NewLine("\n"))),
        '\r' => {
            chars.next();
            if let Some('\n') = chars.peek() {
                consume_and_return(chars, Token::Whitespace(Whitespace::NewLine("\r\n")))
            } else {
                Some(Token::Whitespace(Whitespace::NewLine("\r")))
            }
        }
        '#' => consume_and_return(chars, Token::Crosshatch),
        '*' => consume_and_return(chars, Token::Asterisk),
        '_' => consume_and_return(chars, Token::Underscore),
        '~' => consume_and_return(chars, Token::Tilde),
        '/' => consume_and_return(chars, Token::Slash),
        '\\' => {
            chars.next();
            if let Some(ch) = chars
                .peek()
                .filter(|ch| !ch.is_ascii_control() && !ch.is_ascii_whitespace())
                .cloned()
            {
                chars.next();
                Some(Token::Escaped(ch))
            } else {
                Some(Token::Backslash)
            }
        }
        '[' => {
            chars.next();
            if let Some('[') = chars.peek() {
                chars.next();
                Some(Token::DoubleLBracket)
            } else {
                Some(Token::LBracket)
            }
        }
        ']' => {
            chars.next();
            if let Some(']') = chars.peek() {
                chars.next();
                Some(Token::DoubleRBracket)
            } else {
                Some(Token::RBracket)
            }
        }
        '(' => consume_and_return(chars, Token::LParen),
        ')' => consume_and_return(chars, Token::RParen),
        '{' => consume_and_return(chars, Token::LBrace),
        '}' => consume_and_return(chars, Token::RBrace),
        '`' => consume_and_return(chars, Token::Backtick),
        '=' => consume_and_return(chars, Token::Eq),
        '&' => consume_and_return(chars, Token::Ampersand),
        '^' => consume_and_return(chars, Token::Caret),
        '|' => consume_and_return(chars, Token::Pipe),
        '!' => consume_and_return(chars, Token::ExclamationMark),
        '-' => consume_and_return(chars, Token::Hyphen),
        '+' => consume_and_return(chars, Token::Plus),
        '<' => consume_and_return(chars, Token::Lt),
        '>' => consume_and_return(chars, Token::Gt),
        '$' => consume_and_return(chars, Token::Dollar),
        ':' => consume_and_return(chars, Token::Colon),
        '"' => consume_and_return(chars, Token::DoubleQuote),
        '\'' => consume_and_return(chars, Token::SingleQuote),
        '?' => consume_and_return(chars, Token::Question),
        ';' => consume_and_return(chars, Token::Semicolon),
        '.' => consume_and_return(chars, Token::Period),
        '0'..='9' => {
            let s = peeking_take_while(chars, |ch| ch.is_ascii_digit(), None);
            Some(Token::Digit(s))
        }
        '%' => {
            chars.next();
            if let Some('%') = chars.peek() {
                chars.next();
                Some(Token::DoublePercent)
            } else {
                Some(Token::Punctuation('%'))
            }
        }
        ch if ch.is_ascii_control() => consume_and_return(chars, Token::Control(ch)),
        ch if utils::is_punctuation(ch) => consume_and_return(chars, Token::Punctuation(ch)),
        ch => {
            let ch_len = ch.len_utf8();
            let start = chars.pos;
            let mut end = start + ch_len;
            chars.next();
            if recursion {
                let mut cloned_chars = chars.clone();
                while let Some(Token::Text(_)) = next_token(&mut cloned_chars, false) {
                    end = cloned_chars.pos;
                }
                let len = end - start;
                if len > ch_len {
                    chars.skip(len - ch_len)
                };
            }
            Some(Token::Text(&chars.content[start..end]))
        }
    }
}
fn consume_and_return<'input>(
    chars: &mut StatefulChars<'input>,
    token: Token<'input>,
) -> Option<Token<'input>> {
    chars.next();
    Some(token)
}
fn peeking_take_while<'input>(
    chars: &mut StatefulChars<'input>,
    mut predicate: impl FnMut(char) -> bool,
    max: Option<usize>,
) -> &'input str {
    let start = chars.pos;
    let mut end = start;
    let max = max.unwrap_or(usize::MAX);
    while let Some(&ch) = chars.peek() {
        if predicate(ch) {
            chars.next();
            end = chars.pos;
            if end - start >= max {
                break;
            }
        } else {
            break;
        }
    }
    &chars.content[start..end]
}

impl<'input> Iterator for TokenIterator<'input> {
    type Item = TokenWithLocation<'input>;
    fn next(&mut self) -> Option<Self::Item> {
        let location = self.chars.location();
        next_token(&mut self.chars, true).map(|token| TokenWithLocation { token, location })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let markdown =
            "---\ntitle: Markdown Syntax\nstatus: draft\n---\n## Heading 2\n\nexample text";
        let tokens = Tokenizer::new(markdown).tokenize().collect::<Vec<_>>();
        assert_eq!(tokens.len(), 31);
        assert_eq!(tokens.first().unwrap().token, Token::Hyphen);
        assert_eq!(
            tokens.get(tokens.len() - 2).unwrap().token,
            Token::Whitespace(Whitespace::Space)
        );
        assert_eq!(tokens.last().unwrap().token, Token::Text("text"));
        assert_eq!(
            Tokenizer::new("#width=200&height=300&fit=cover")
                .tokenize()
                .count(),
            12
        );
        assert_eq!(Tokenizer::new("`````").tokenize().count(), 5);
        assert_eq!(
            Tokenizer::new("1. Markdown 语法\n2. test2.2\n3. 12453\n21.214")
                .tokenize()
                .count(),
            20
        )
    }

    #[test]
    fn other() {
        let t1 = "**(Test)**WithoutSpace";
        let t2 = "**（测试）**不加空格";
        println!(
            "t1:\n{:#?}",
            Tokenizer::new(t1).tokenize().collect::<Vec<_>>()
        );
        println!(
            "t2:\n{:#?}",
            Tokenizer::new(t2).tokenize().collect::<Vec<_>>()
        )
    }

    #[test]
    fn case_3() {
        let tokens = Tokenizer::new("- hello world")
            .tokenize()
            .collect::<Vec<_>>();
        assert_eq!(tokens[1].token, Token::Whitespace(Whitespace::Space))
    }
    #[test]
    fn case_4() {
        let tokens = Tokenizer::new("2) hello world")
            .tokenize()
            .collect::<Vec<_>>();
        assert_eq!(tokens[0].token, Token::Digit("2"));
        let tokens = Tokenizer::new("1234567890) hello world")
            .tokenize()
            .collect::<Vec<_>>();
        assert_eq!(tokens[0].token, Token::Digit("1234567890"));
    }

    #[test]
    fn case_5() {
        let tokens = Tokenizer::new("(/url2)").tokenize().collect::<Vec<_>>();
        assert_eq!(tokens.last().map(|it| it.token), Some(Token::RParen))
    }
}
