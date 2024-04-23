use crate::ast::{html, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, Line, ProcessCtx};
use crate::tokenizer::{Token, Whitespace};

macro_rules! check_or_return {
    ($expr:expr) => {
        if !$expr {
            return false;
        }
    };
}

// sorted for binary search
#[rustfmt::skip]
const HTML_TAGS: [&str; 62] = [
    "address", "article", "aside",
    "base", "basefont", "blockquote", "body",
    "caption", "center", "col", "colgroup",
    "dd", "details", "dialog", "dir", "div", "dl", "dt",
    "fieldset", "figcaption", "figure", "footer", "form", "frame", "frameset", 
    "h1", "h2", "h3", "h4", "h5", "h6", "head", "header", "hr", "html",
    "iframe",
    "legend", "li", "link",
    "main", "menu", "menuitem",
    "nav", "noframes",
    "ol", "optgroup", "option",
    "p", "param",
    "search", "section", "summary",
    "table", "tbody", "td", "tfoot", "th", "thead", "title", "tr", "track",
    "ul",
];
/// example: `<pre`、`<script` ...
fn is_begin_type_1(line: &Line) -> bool {
    static BEGIN_TAGS: &[&str; 4] = &["pre", "style", "script", "textarea"];
    if let Some(Token::Text(text)) = line.get_raw(1) {
        if !BEGIN_TAGS.iter().any(|it| it.eq_ignore_ascii_case(text)) {
            return false;
        }
    } else {
        return false;
    }
    return match line.get_raw(2) {
        Some(Token::Gt) => true,
        Some(token) if token.is_space_or_tab() => true,
        None => true,
        _ => false,
    };
}
fn is_end_type_1(line: &mut Line) -> bool {
    static END_TAGS: &[&[u8]; 4] = &[b"pre", b"style", b"script", b"textarea"];
    if !(line.consume(Token::Lt) && line.consume(Token::Slash)) {
        return false;
    }
    if let Some(Token::Text(text)) = line.next() {
        if !END_TAGS
            .iter()
            .any(|it| it.eq_ignore_ascii_case(text.as_bytes()))
        {
            return false;
        }
    } else {
        return false;
    }
    if !line.consume(Token::Gt) {
        return false;
    }
    true
}
/// example: `<!--`
fn is_begin_type_2(line: &Line) -> bool {
    line.validate(3, Token::Hyphen)
}
fn is_end_type_2(line: &mut Line) -> bool {
    line.consume(Token::Hyphen) && line.consume(Token::Hyphen) && line.consume(Token::Gt)
}
fn is_end_type_3(line: &mut Line) -> bool {
    line.consume(Token::Question) && line.consume(Token::Gt)
}
/// example: `<!Document`
fn is_begin_type_4(line: &Line) -> bool {
    line.validate(2, |it: &Token| {
        if let Token::Text(text) = it {
            matches!(text.chars().next(), Some('a'..='z' | 'A'..='Z'))
        } else {
            false
        }
    })
}
fn is_end_type_4(line: &mut Line) -> bool {
    line.consume(Token::Gt)
}
/// example: `<![CDATA[`
fn is_begin_type_5(line: &Line) -> bool {
    line.validate(3, Token::Text("CDATA")) && line.validate(4, Token::LBracket)
}
fn is_end_type_5(line: &mut Line) -> bool {
    line.consume(Token::DoubleRBracket) && line.consume(Token::Gt)
}

/// example: `<address>`, `<br/>`
fn is_begin_type_6(line: &Line) -> bool {
    if let Some(Token::Text(text)) = line.get_raw(1) {
        if !is_html_tag(text.as_bytes()) {
            return false;
        }
    } else {
        return false;
    }
    return match line.get_raw(2) {
        Some(token) if token.is_space_or_tab() => true,
        Some(Token::Gt) => true,
        // self-closing tag
        Some(Token::Slash) => line.validate(3, Token::Gt),
        // line end, auto close ?
        None => true,
        _ => false,
    };
}
/// example: `</div>`
fn is_end_type_6(line: &Line) -> bool {
    line.validate(2, |token: &Token| {
        if let Token::Text(text) = token {
            is_html_tag(text.as_bytes())
        } else {
            false
        }
    }) && line.validate(3, Token::Gt)
}

#[derive(Debug)]
enum TagState {
    Start,
    TagName,
    PropName,
    PropValue,
    End,
}
fn is_begin_type_7(line: &mut Line) -> bool {
    let sn = line.snapshot();
    // skip LT Token
    line.next();
    fn consume_and_return(line: &mut Line, state: TagState) -> TagState {
        line.next();
        state
    }
    let mut state = TagState::Start;
    let mut quoted = None;
    while let Some(next) = line.peek() {
        println!("[next = {:?}] [state = {:?}]", next, state);
        state = match state {
            TagState::Start => match next {
                Token::Text(str) if str.chars().all(|char| char.is_ascii_alphabetic()) => {
                    TagState::TagName
                }
                _ => break,
            },
            TagState::TagName => match next {
                Token::Text(_) => consume_and_return(line, TagState::TagName),
                Token::Digit(_) => {
                    consume_and_return(line, TagState::TagName)
                }
                Token::Hyphen => consume_and_return(line, TagState::TagName),
                Token::Whitespace(Whitespace::Space) => {
                    consume_and_return(line, TagState::PropName)
                }
                Token::Gt => consume_and_return(line, TagState::End),
                _ => break,
            },
            TagState::PropName => match next {
                Token::Digit(_) => {
                    consume_and_return(line, TagState::PropName)
                }
                Token::Hyphen | Token::Underscore | Token::Colon | Token::Text(_) => {
                    consume_and_return(line, TagState::PropName)
                }
                Token::Eq => {
                    line.next();
                    line.skip_consecutive_tokens(&Token::Whitespace(Whitespace::Space));
                    TagState::PropValue
                }
                Token::Gt => consume_and_return(line, TagState::End),
                Token::Slash => {
                    line.next();
                    if line.consume(Token::Gt) {
                        TagState::End
                    } else {
                        break;
                    }
                }
                Token::Whitespace(Whitespace::Space) => {
                    line.next();
                    line.skip_consecutive_tokens(&Token::Whitespace(Whitespace::Space));
                    TagState::PropName
                }
                _ => break,
            },
            TagState::PropValue => match next {
                Token::DoubleQuote => {
                    line.next();
                    if quoted == Some('"') {
                        quoted = None;
                        TagState::PropName
                    } else {
                        quoted = Some('"');
                        TagState::PropValue
                    }
                }
                Token::Text("'") => {
                    line.next();
                    if quoted == Some('\'') {
                        quoted = None;
                        TagState::PropName
                    } else {
                        quoted = Some('\'');
                        TagState::PropValue
                    }
                }
                _ if quoted.is_some() => consume_and_return(line, TagState::PropValue),
                _ => break,
            },
            TagState::End => break,
        }
    }
    println!("END = {state:?}");
    line.resume(sn);
    matches!(state, TagState::End)
}
fn is_end_type_7(line: &Line) -> bool {
    check_or_return!(line.validate(2, |it: &Token| match it {
        Token::Text(str) => str.chars().all(|char| char.is_ascii_alphabetic()),
        _ => false,
    }));
    let mut i = 3;
    let mut is_end = false;
    while let Some(token) = line.get_raw(i) {
        let r = match token {
            Token::Text(str) if !is_end => str.chars().all(|char| char.is_ascii_alphabetic()),
            Token::Digit(_) if !is_end => true,
            Token::Hyphen if !is_end => true,
            Token::Whitespace(Whitespace::Space) => {
                is_end = true;
                true
            }
            Token::Gt => return true,
            _ => false,
        };
        if !r {
            return false;
        }
        i += 1;
    }
    false
}

fn is_html_tag(tag: &[u8]) -> bool {
    HTML_TAGS
        .binary_search_by(|probe| {
            let probe_bytes_iter = probe.as_bytes().iter();
            let tag_bytes_iter = tag.iter();

            probe_bytes_iter
                .zip(tag_bytes_iter)
                .find_map(|(&a, &b)| {
                    // We can compare case insensitively because the probes are
                    // all lower case alpha strings.
                    match a.cmp(&(b | 0x20)) {
                        std::cmp::Ordering::Equal => None,
                        inequality => Some(inequality),
                    }
                })
                .unwrap_or_else(|| probe.len().cmp(&tag.len()))
        })
        .is_ok()
}

impl html::Html {
    pub fn is_end(&self, line: &mut Line) -> bool {
        match self {
            html::Html::Block(html::BlockType::Type1) => is_end_type_1(line),
            html::Html::Block(html::BlockType::Type2) => is_end_type_2(line),
            html::Html::Block(html::BlockType::Type3) => is_end_type_3(line),
            html::Html::Block(html::BlockType::Type4) => is_end_type_4(line),
            html::Html::Block(html::BlockType::Type5) => is_end_type_5(line),
            _ => false,
        }
    }
}

impl BlockStrategy for html::Html {
    fn before(
        BeforeCtx {
            line,
            parser,
            container,
        }: BeforeCtx,
    ) -> BlockMatching {
        let location = line.start_location();
        if line.is_indented() {
            return BlockMatching::Unmatched;
        }
        if !line.skip_indent().validate(0, Token::Lt) {
            return BlockMatching::Unmatched;
        }
        let block_type = match line.get_raw(1) {
            // type 2, 4, 5
            Some(Token::ExclamationMark) => {
                let b = match line.get_raw(2) {
                    Some(Token::Hyphen) => {
                        if is_begin_type_2(line) {
                            Some(html::BlockType::Type2)
                        } else {
                            None
                        }
                    }
                    Some(Token::Text(_)) => {
                        if is_begin_type_4(line) {
                            Some(html::BlockType::Type4)
                        } else {
                            None
                        }
                    }
                    Some(Token::LBracket) => {
                        if is_begin_type_5(line) {
                            Some(html::BlockType::Type5)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(b) = b {
                    b
                } else {
                    return BlockMatching::Unmatched;
                }
            }
            // type 3
            Some(Token::Question) => html::BlockType::Type3,
            // type 1, 6, 7
            Some(Token::Text(_)) => {
                if is_begin_type_1(line) {
                    html::BlockType::Type1
                } else if is_begin_type_6(line) {
                    html::BlockType::Type6
                } else if is_begin_type_7(line) {
                    html::BlockType::Type7
                } else {
                    return BlockMatching::Unmatched;
                }
            }
            // type 6 end, type 7 end
            Some(Token::Slash) => {
                if is_end_type_6(line) {
                    html::BlockType::Type6
                } else if is_end_type_7(line) {
                    html::BlockType::Type7
                } else {
                    return BlockMatching::Unmatched;
                }
            }
            _ => {
                return BlockMatching::Unmatched;
            }
        };
        if block_type != html::BlockType::Type7
            || (parser.tree[container].body != MarkdownNode::Paragraph
                && !(!parser.all_closed
                    && !line.is_blank()
                    && parser.current_proc().body == MarkdownNode::Paragraph))
        {
            parser.close_unmatched_blocks();
            parser.append_block(MarkdownNode::Html(html::Html::Block(block_type)), location);
            BlockMatching::MatchedLeaf
        } else {
            BlockMatching::Unmatched
        }
    }

    fn process(ProcessCtx { line, parser, id }: ProcessCtx) -> BlockProcessing {
        if line.is_blank()
            && matches!(
                parser.tree[id].body,
                MarkdownNode::Html(html::Html::Block(
                    html::BlockType::Type6 | html::BlockType::Type7
                ))
            )
        {
            BlockProcessing::Unprocessed
        } else {
            BlockProcessing::Further
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{html, MarkdownNode};
    use crate::parser::Parser;
    #[test]
    fn case_1() {
        let text = r#"
<script>console.log("hello world")</script>
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("{:?}", ast);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::BlockType::Type1))
        );
        assert_eq!(ast.len(), 3);
    }
    #[test]
    fn case_2() {
        let text = r#"
<!--comments-->
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("{:?}", ast);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::BlockType::Type2))
        );
        assert_eq!(ast.len(), 3);
    }
    #[test]
    fn case_3() {
        let text = r#"
<?php
echo "php is best programming language in the universe."
?>
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("{:?}", ast);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::BlockType::Type3))
        );
        assert_eq!(ast.len(), 3);
    }
    #[test]
    fn case_4() {
        let text = r#"
<!DOCTYPE html>
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("{:?}", ast);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::BlockType::Type4))
        );
        assert_eq!(ast.len(), 3);
    }
    #[test]
    fn case_5() {
        let text = r#"
<![CDATA[
function matchwo(a,b)
{
if (a < b && a < 0) then
{
return 1;
}
else
{
return 0;
}
}
]]>
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("{:?}", ast);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::BlockType::Type5))
        );
    }
    #[test]
    fn case_6() {
        let text = r#"
<p>
  Geckos are a group of usually small, usually nocturnal lizards. They are found on every continent except Antarctica.
</p>
        "#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("{:?}", ast);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::BlockType::Type6))
        );
    }
    #[test]
    fn case_7() {
        let text = r#"
<Button>Click Me</Button>      
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::BlockType::Type7))
        );
        println!("{:?}", ast);
    }
}
