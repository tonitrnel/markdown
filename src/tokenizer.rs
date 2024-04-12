use std::fmt::{self, Write};
use std::iter::Peekable;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Whitespace<'input> {
    /// 空格
    Space,
    /// 换行
    NewLine(&'input str),
    /// 制表符
    Tab,
    /// 注释, use `%%` symbol (inline, comment)
    Comment(&'input str),
}
impl Whitespace<'_> {
    pub fn len(&self) -> usize {
        match self {
            Whitespace::Space => 1,
            Whitespace::Tab => 1,
            Whitespace::NewLine(s) => s.len(),
            Whitespace::Comment(c) => c.len() + 4,
        }
    }
}

impl fmt::Display for Whitespace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::NewLine(s) => f.write_str(s),
            Whitespace::Tab => f.write_str("\t"),
            Whitespace::Comment(_str) => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Token<'input> {
    /// Text
    Text(&'input str),
    /// Number 0-9 Include decimal
    Number(&'input str),
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
    /// BlockReference `#^`
    BlockReference,
    /// Ordered, like: `1. `, `2. `
    Ordered(u64, char),
    /// Slash `/`
    Slash,
    /// Backslash `\`
    Backslash,
    /// Escaped
    Escaped(char),
    Invalid(char),
}

impl Token<'_> {
    pub fn len(&self) -> usize {
        match self {
            Token::Text(s) => s.chars().count(),
            Token::Number(s) => s.len(),
            Token::Whitespace(ws) => ws.len(),
            Token::DoubleLBracket
            | Token::DoubleRBracket
            | Token::BlockReference
            | Token::Escaped(_) => 2,
            Token::Ordered(n, _) => get_digit_count(*n) + 1,
            _ => 1,
        }
    }
    pub fn is_space_or_tab(&self) -> bool {
        matches!(self, Token::Whitespace(Whitespace::Space | Whitespace::Tab))
    }
    /// 是用于 Markdown Block 相关的 Token
    pub fn is_special_token(&self) -> bool {
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
                // Ordered Task or Task
                | Token::Ordered(..)
                | Token::Hyphen
                // Table
                | Token::Pipe
                | Token::Colon
        )
    }
}

impl<'input> AsRef<Token<'input>> for Token<'input> {
    fn as_ref(&self) -> &Token<'input> {
        self
    }
}

fn get_digit_count(mut num: u64) -> usize {
    if num == 0 {
        return 1;
    }

    let mut count = 0usize;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Text(str) => write!(f, "{str}"),
            Token::Number(str) => write!(f, "{str}"),
            Token::Whitespace(ws) => write!(f, "{ws}"),
            Token::Crosshatch => f.write_str("#"),
            Token::Asterisk => f.write_str("*"),
            Token::Underscore => f.write_str("_"),
            Token::Tilde => f.write_str("~"),
            Token::LBracket => f.write_str("["),
            Token::DoubleLBracket => f.write_str("[["),
            Token::RBracket => f.write_str("]"),
            Token::DoubleRBracket => f.write_str("]]"),
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
            Token::Backtick => f.write_str("`"),
            Token::Eq => f.write_str("="),
            Token::Ampersand => f.write_str("&"),
            Token::Caret => f.write_str("^"),
            Token::Pipe => f.write_str("|"),
            Token::ExclamationMark => f.write_str("!"),
            Token::Hyphen => f.write_str("-"),
            Token::Plus => f.write_str("+"),
            Token::Lt => f.write_str("<"),
            Token::Gt => f.write_str(">"),
            Token::Dollar => f.write_str("$"),
            Token::Colon => f.write_str(":"),
            Token::SingleQuote => f.write_str("'"),
            Token::DoubleQuote => f.write_str("\""),
            Token::Question => f.write_str("?"),
            Token::Semicolon => f.write_str(";"),
            Token::BlockReference => f.write_str("#^"),
            Token::Ordered(u, d) => write!(f, "{u}{d}"),
            Token::Slash => write!(f, "/"),
            Token::Backslash => write!(f, "\\"),
            Token::Escaped(ch) => write!(f, "\\{ch}"),
            Token::Invalid(_) => f.write_char('\u{FFFD}'),
        }
    }
}
impl<'input> From<Whitespace<'input>> for Token<'input> {
    fn from(value: Whitespace<'input>) -> Self {
        Token::Whitespace(value)
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
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
    pub fn is_column_start(&self) -> bool {
        self.column == 1
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TokenWithLocation<'input> {
    pub token: Token<'input>,
    pub location: Location,
}

impl TokenWithLocation<'_> {
    /// 是当前列开头
    pub fn is_column_start(&self) -> bool {
        self.location.is_column_start()
    }
    /// 是空白或制表符
    pub fn is_space_or_tab(&self) -> bool {
        self.token.is_space_or_tab()
    }
    pub fn is_special_token(&self) -> bool {
        self.token.is_special_token()
    }
    pub fn len(&self) -> usize {
        self.token.len()
    }
    pub fn start_location(&self) -> Location {
        self.location
    }
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
        '#' => {
            chars.next();
            if let Some('^') = chars.peek() {
                chars.next();
                Some(Token::BlockReference)
            } else {
                Some(Token::Crosshatch)
            }
        }
        '*' => consume_and_return(chars, Token::Asterisk),
        '_' => consume_and_return(chars, Token::Underscore),
        '~' => consume_and_return(chars, Token::Tilde),
        '/' => consume_and_return(chars, Token::Slash),
        '\\' => {
            chars.next();
            if let Some(ch) = chars.peek().filter(|ch| !ch.is_ascii_control()).cloned() {
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
        '0'..='9' => {
            let start = chars.pos;
            let s = peeking_take_while(chars, |ch| ch.is_ascii_digit(), None);
            let mut end = chars.pos;
            match chars.peek() {
                Some(d @ '.' | d @ ')') => {
                    let d = *d;
                    chars.next();
                    end += 1;
                    match chars.peek() {
                        Some(ch) if d == '.' && ch.is_ascii_digit() => {
                            chars.next();
                            peeking_take_while(chars, |ch| ch.is_ascii_digit(), None);
                            Some(Token::Number(&chars.content[start..chars.pos]))
                        }
                        Some(' ' | '\n') if s.len() < 10 => {
                            Some(Token::Ordered(s.parse::<u64>().unwrap(), d))
                        }
                        _ => Some(Token::Text(&chars.content[start..end])),
                    }
                }
                _ => Some(Token::Number(s)),
            }
        }
        '%' => {
            chars.next();
            if let Some('%') = chars.peek() {
                chars.next();
                let start = chars.pos;
                let mut end = start;
                while let Some(ch) = chars.next() {
                    if ch == '%' && chars.peek() == Some(&'%') {
                        chars.next();
                        break;
                    }
                    end = chars.pos;
                }
                Some(Token::Whitespace(Whitespace::Comment(
                    &chars.content[start..end],
                )))
            } else {
                Some(Token::Text("%"))
            }
        }
        '\u{0000}' => {
            chars.next();
            Some(Token::Invalid(ch))
        }
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
            16
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
        assert_eq!(tokens[0].token, Token::Ordered(2, ')'));
        let tokens = Tokenizer::new("1234567890) hello world")
            .tokenize()
            .collect::<Vec<_>>();
        assert_eq!(tokens[0].token, Token::Text("1234567890)"));
    }
}
