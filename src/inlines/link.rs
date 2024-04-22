use crate::ast::{embed, link, reference::Reference, MarkdownNode};
use crate::inlines::bracket::BracketChain;
use crate::inlines::{ProcessCtx, RefMap};
use crate::line::Line;
use crate::tokenizer::{Token, Whitespace};
use crate::utils;

enum TitleState {
    Initial,
    InString(InString),
}
enum InString {
    Double,
    Single,
    Paren,
}

pub(crate) fn scan_link_or_image<'input>(
    line: &mut Line<'input>,
    opener: &BracketChain<'input>,
    ref_map: &RefMap,
) -> Option<(String, Option<String>)> {
    let snapshot = line.snapshot();
    let mut url = None;
    let mut title = None;
    let mut matched = false;
    println!(" -> scan_link_block");
    // 尝试解析 inline link
    'scan_link_block: {
        if !line.consume(Token::LParen) {
            break 'scan_link_block;
        }
        line.skip(line.starts_count_matches(|it| matches!(it, Token::Whitespace(..))));
        println!(" -> scan_link_block::scan_link_url");
        url = match scan_link_url(line) {
            Some((size, _line)) => {
                line.skip(size);
                println!("### {:?}", _line.to_string());
                Some(utils::percent_encode::encode(
                    utils::unescape_string(_line.to_escaped_string()),
                    true,
                ))
            }
            _ => break 'scan_link_block,
        };
        println!(" -> scan_link_block::scan_link_title => url = {url:?}");
        title = {
            let mut title = None;
            let count = line.starts_count_matches(|it| matches!(it, Token::Whitespace(..)));
            if count > 0 {
                line.skip(count);
                title = match scan_link_title(line) {
                    Some((size, _line)) => {
                        line.skip(size);
                        Some(_line.to_string())
                    }
                    _ => None,
                }
            }
            title
        };
        line.skip(line.starts_count_matches(|it| matches!(it, Token::Whitespace(..))));
        println!(" -> scan_link_block::end_matches => title = {title:?}");
        if !line.consume(Token::RParen) {
            break 'scan_link_block;
        };
        matched = true;
    };
    println!(" -> scan_link_label = {matched}");
    // 如果上一个块未匹配，尝试解析 link label
    'scan_link_label: {
        if matched {
            break 'scan_link_label;
        };
        line.resume(snapshot);
        let ref_label = match scan_link_label(line) {
            Some((size, label)) => {
                line.skip(size);
                label
            }
            None if !opener.borrow().bracket_after => line
                .slice_raw(opener.borrow().index, line.start_offset)
                .to_string(),
            _ => break 'scan_link_label,
        };
        let ref_label = normalize_reference(ref_label);
        if let Some((_link, _title)) = ref_map.get(&ref_label) {
            url = Some(_link.to_owned());
            title = Some(_title.to_owned());
            matched = true
        }
    }
    println!(" -> matched = {matched}");
    if !matched {
        return None;
    }
    Some((url.unwrap(), title))
}
fn scan_link_url<'input>(line: &Line<'input>) -> Option<(usize, Line<'input>)> {
    if line.validate(0, Token::Lt) {
        let mut i = 0;
        let mut end = false;
        let iter = line.iter().skip(1);
        for it in iter {
            i += 1;
            match it.token {
                Token::Gt => {
                    end = true;
                    break;
                }
                Token::Whitespace(Whitespace::NewLine(..)) | Token::Lt => {
                    return None;
                }
                _ => (),
            };
        }
        if !end {
            return None;
        }
        let line = line.slice(1, i);
        Some((i + 1, line))
    } else {
        let mut i = 0;
        let mut p = 0;
        let iter = line.iter();
        for it in iter {
            match it.token {
                Token::LParen => {
                    i += 1;
                    p += 1;
                    if p > 32 {
                        return None;
                    }
                }
                Token::RParen => {
                    if p == 0 {
                        break;
                    }
                    p -= 1;
                    i += 1;
                }
                Token::Whitespace(..) => {
                    break;
                }
                _ => i += 1,
            }
        }
        // 括号不对称
        if p != 0 {
            return None;
        }
        let line = line.slice(0, i);
        Some((i, line))
    }
}
fn scan_link_title<'input>(line: &Line<'input>) -> Option<(usize, Line<'input>)> {
    let mut i = 0;
    let mut state = TitleState::Initial;
    for item in line.iter() {
        i += 1;
        match (&state, item.token) {
            (TitleState::Initial, Token::DoubleQuote) => {
                state = TitleState::InString(InString::Double)
            }
            (TitleState::Initial, Token::SingleQuote) => {
                state = TitleState::InString(InString::Single)
            }
            (TitleState::Initial, Token::LParen) => state = TitleState::InString(InString::Paren),
            (TitleState::InString(InString::Double), Token::DoubleQuote)
            | (TitleState::InString(InString::Single), Token::SingleQuote)
            | (TitleState::InString(InString::Paren), Token::RParen) => {
                state = TitleState::Initial;
                break;
            }
            (TitleState::InString(_), _) => (),
            _ => return None,
        }
    }
    if !matches!(state, TitleState::Initial) {
        return None;
    }
    Some((i, line.slice(1, i - 1)))
}

fn scan_link_label(line: &Line) -> Option<(usize, String)> {
    if !line.validate(0, Token::LBracket) {
        return None;
    }
    let iter = line.iter().skip(1);
    let mut end = false;
    let mut i = 0;
    for item in iter {
        i += 1;
        if i > 1000 {
            return None;
        }
        match item.token {
            Token::RBracket => {
                end = true;
                break;
            }
            Token::LBracket => return None,
            _ => (),
        }
    }
    if !end || i <= 2 {
        return None;
    }
    Some((i, line.slice(0, i).to_string()))
}
fn normalize_reference(str: String) -> String {
    str[1..str.len()]
        .trim()
        .to_uppercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Block Id
///
/// https://help.obsidian.md/Linking+notes+and+files/Internal+links#Link+to+a+block+in+a+note
pub(super) fn process_block_id(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    line.next();
    for item in line.iter() {
        match &item.token {
            Token::Text(str) if str.chars().all(|ch| ch.is_ascii_alphabetic()) => continue,
            Token::Number(str) if !str.contains('.') => continue,
            Token::Hyphen => continue,
            _ => return false,
        }
    }
    let block_id = line.slice(0, line.len()).to_string();
    parser.tree[*id].id = Some(block_id);
    line.skip_to_end();
    true
}

#[derive(Debug)]
enum WikilinkState {
    Initial,
    InPath,
    InText,
    InRef(InRef),
}
#[derive(Debug)]
enum InRef {
    Ref,
    RefHeading(usize),
    RefBlock,
}

pub(super) fn process_wikilink(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    line.next();
    let mut state = WikilinkState::Initial;
    let mut pr = (0, 0); // path range: start, end
    let mut rrs = (false, Vec::new()); // ref range: is_block, start, end
    let mut tr = (0, 0); // text range: start, end
    for (end, item) in line.iter().enumerate() {
        match (&state, &item.token) {
            (WikilinkState::Initial, Token::DoubleRBracket) => return false,
            (WikilinkState::Initial, _) => state = WikilinkState::InPath,
            (WikilinkState::InPath, Token::Pipe) => {
                pr.1 = end;
                tr.0 = end + 1;
                state = WikilinkState::InText
            }
            (WikilinkState::InPath, Token::Crosshatch) => {
                pr.1 = end;
                state = WikilinkState::InRef(InRef::Ref)
            }
            (WikilinkState::InPath, Token::DoubleRBracket) => {
                pr.1 = end;
                state = WikilinkState::Initial;
                break;
            }
            (WikilinkState::InPath, _) => continue,
            (WikilinkState::InText, Token::DoubleRBracket) => {
                tr.1 = end;
                state = WikilinkState::Initial;
                break;
            }
            (WikilinkState::InText, _) => continue,
            (WikilinkState::InRef(InRef::Ref), Token::Caret) => {
                rrs.0 = true;
                rrs.1.push((end + 1, end + 1));
                state = WikilinkState::InRef(InRef::RefBlock)
            }
            (WikilinkState::InRef(InRef::Ref), Token::Text(_) | Token::Number(_)) => {
                rrs.0 = false;
                rrs.1.push((end, end));
                state = WikilinkState::InRef(InRef::RefHeading(0))
            }
            (WikilinkState::InRef(InRef::RefBlock), Token::DoubleRBracket) => {
                rrs.1[0].1 = end;
                state = WikilinkState::Initial;
                break;
            }
            (WikilinkState::InRef(InRef::RefBlock), Token::Pipe) => {
                rrs.1[0].1 = end;
                tr.0 = end + 1;
                state = WikilinkState::InText
            }
            (
                WikilinkState::InRef(InRef::RefBlock),
                Token::Text(_) | Token::Number(_) | Token::Hyphen,
            ) => continue,
            (WikilinkState::InRef(InRef::RefHeading(index)), Token::DoubleRBracket) => {
                rrs.1[*index].1 = end;
                state = WikilinkState::Initial;
                break;
            }
            (WikilinkState::InRef(InRef::RefHeading(index)), Token::Pipe) => {
                rrs.1[*index].1 = end;
                tr.0 = end + 1;
                state = WikilinkState::InText
            }
            (WikilinkState::InRef(InRef::RefHeading(index)), Token::Crosshatch) => {
                rrs.1[*index].1 = end;
                rrs.1.push((end + 1, end + 1));
                state = WikilinkState::InRef(InRef::RefHeading(index + 1))
            }
            (WikilinkState::InRef(InRef::RefHeading(_)), _) => continue,
            _ => return false,
        }
    }
    if !matches!(state, WikilinkState::Initial) || pr.0 == pr.1 {
        return false;
    }
    let path = line.slice(pr.0, pr.1).to_string();
    let reference = extract_ref(line, &rrs);
    let text = if tr.0 != tr.1 {
        Some(line.slice(tr.0, tr.1).to_string())
    } else {
        None
    };
    let end = pr.1.max(rrs.1.last().map(|it| it.1).unwrap_or(0)).max(tr.1);
    let end_location = line[end].end_location();
    line.skip(end + 1);
    parser.append_block_to(
        *id,
        MarkdownNode::Link(
            link::Wikilink {
                path,
                reference,
                text,
            }
            .into(),
        ),
        (start_location, end_location),
    );
    true
}

fn extract_ref(line: &Line, range: &(bool, Vec<(usize, usize)>)) -> Option<Reference> {
    if !range.1.is_empty() {
        Some(if range.0 {
            let value = line.slice(range.1[0].0, range.1[0].1).to_string();
            Reference::BlockId(value)
        } else if range.1.len() == 1 {
            Reference::Heading(line.slice(range.1[0].0, range.1[0].1).to_string())
        } else {
            Reference::MultiHeading(
                range
                    .1
                    .iter()
                    .map(|(start, end)| {
                        if start == end {
                            "".to_string()
                        } else {
                            line.slice(*start, *end).to_string()
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
    } else {
        None
    }
}

#[derive(Debug)]
enum EmbedState {
    Initial,
    InPath,
    InRef(InRef),
    InAttr(usize),
    InSize(InSize),
}
#[derive(Debug)]
enum InSize {
    Width,
    Height,
}
pub(super) fn process_embed(
    ProcessCtx {
        id, line, parser, ..
    }: &mut ProcessCtx,
) -> bool {
    let start_location = line.start_location();
    line.skip(2);
    let mut state = EmbedState::Initial;
    let mut pr = (0, 0); // path range: start, end
    let mut sr = (0, 0); // size range: start, end
    let mut rrs = (false, Vec::new()); // ref range: is_block, start, end
    let mut ars = Vec::new(); // attrs range: is_block, start, end
    for (end, item) in line.iter().enumerate() {
        match (&state, &item.token) {
            (EmbedState::Initial, Token::DoubleRBracket) => return false,
            (EmbedState::Initial, _) => state = EmbedState::InPath,
            (EmbedState::InPath, Token::Crosshatch) => {
                pr.1 = end;
                state = EmbedState::InRef(InRef::Ref);
            }
            (EmbedState::InPath, Token::Pipe) => {
                pr.1 = end;
                sr.0 = end + 1;
                state = EmbedState::InSize(InSize::Width);
            }
            (EmbedState::InPath, Token::DoubleRBracket) => {
                pr.1 = end;
                state = EmbedState::Initial;
                break;
            }
            (EmbedState::InPath, _) => continue,

            (EmbedState::InSize(InSize::Width), Token::Text(str)) if *str == "x" => {
                state = EmbedState::InSize(InSize::Height)
            }
            (EmbedState::InSize(_), Token::DoubleRBracket) => {
                sr.1 = end;
                state = EmbedState::Initial;
                break;
            }
            (EmbedState::InSize(InSize::Width), Token::Number(_)) => continue,
            (EmbedState::InSize(InSize::Height), Token::Number(_)) => continue,

            (EmbedState::InRef(InRef::Ref), Token::Caret) => {
                rrs.0 = true;
                rrs.1.push((end + 1, end + 1));
                state = EmbedState::InRef(InRef::RefBlock)
            }
            (EmbedState::InRef(InRef::Ref), Token::Text(_) | Token::Number(_)) => {
                rrs.0 = false;
                rrs.1.push((end, end));
                state = EmbedState::InRef(InRef::RefHeading(0))
            }
            (EmbedState::InRef(InRef::RefBlock), Token::DoubleRBracket) => {
                rrs.1[0].1 = end;
                state = EmbedState::Initial;
                break;
            }
            (
                EmbedState::InRef(InRef::RefBlock),
                Token::Text(_) | Token::Number(_) | Token::Hyphen,
            ) => continue,

            (EmbedState::InRef(InRef::RefHeading(index)), Token::Crosshatch) => {
                rrs.1[*index].1 = end;
                rrs.1.push((end + 1, end + 1));
                state = EmbedState::InRef(InRef::RefHeading(index + 1))
            }
            (EmbedState::InRef(InRef::RefHeading(index)), Token::Eq) => {
                rrs.1[*index].1 = end;
                ars.push(rrs.1[*index]);
                rrs.1.pop();
                state = EmbedState::InAttr(0)
            }
            (EmbedState::InRef(InRef::RefHeading(index)), Token::DoubleRBracket) => {
                rrs.1[*index].1 = end;
                state = EmbedState::Initial;
                break;
            }
            (EmbedState::InRef(InRef::RefHeading(_)), _) => continue,

            (EmbedState::InAttr(index), Token::Ampersand) => {
                ars[*index].1 = end;
                ars.push((end + 1, end + 1));
                state = EmbedState::InAttr(index + 1)
            }
            (EmbedState::InAttr(index), Token::DoubleRBracket) => {
                ars[*index].1 = end;
                state = EmbedState::Initial;
                break;
            }
            (EmbedState::InAttr(_), _) => continue,

            _ => return false,
        }
    }

    if !matches!(state, EmbedState::Initial) || pr.0 == pr.1 {
        return false;
    }
    let path = line.slice(pr.0, pr.1).to_string();
    let reference = extract_ref(line, &rrs);
    let attrs = if !ars.is_empty() {
        Some(
            ars.iter()
                .map(|(start, end)| {
                    if start == end {
                        "".to_string()
                    } else {
                        line.slice(*start, *end).to_string()
                    }
                })
                .filter_map(|it|{
                    let mut parts = it.split('=');
                    match (parts.next(), parts.next()) {
                        (Some(a), Some(b)) => Some((a.to_string(), b.to_string())),
                        (Some(a), None) => Some((a.to_string(), "".to_string())),
                        _ => None
                    }
                })
                .collect::<Vec<_>>(),
        )
    } else {
        None
    };
    let size = if sr.0 != sr.1 {
        let size = line.slice(sr.0, sr.1).to_string();
        let mut parts = size.split('x');
        let first = parts.next().and_then(|it|it.parse::<u32>().ok());
        let second = parts.next().and_then(|it|it.parse::<u32>().ok());
        match (first, second) {
            (Some(a), Some(b)) => Some((a, Some(b))),
            (Some(a), None) => Some((a, None)),
            _ => None
        }
    } else { 
        None
    };
    let end =
        pr.1.max(rrs.1.last().map(|it| it.1).unwrap_or(0))
            .max(ars.last().map(|it| it.1).unwrap_or(0))
            .max(sr.1);
    let end_location = line[end].end_location();
    line.skip(end + 1);
    parser.append_block_to(*id, MarkdownNode::Embed(embed::Embed {
        path,
        size,
        reference,
        attrs,
    }), (start_location, end_location));
    true
}
#[cfg(test)]
mod tests {
    use crate::ast::reference::Reference;
    use crate::ast::{embed, link, MarkdownNode};
    use crate::parser::Parser;

    #[test]
    fn ofm_case_block_id() {
        let text = r#""You do not rise to the level of your goals. You fall to the level of your systems." by James Clear ^quote-of-the-day"#;
        let ast = Parser::new(text).parse();
        println!("{ast:?}");
        assert_eq!(ast[1].id, Some("quote-of-the-day".to_string()))
    }

    #[test]
    fn ofm_case_wikilink_1() {
        let text = r#"[[Three laws of motion]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(
                link::Wikilink {
                    path: "Three laws of motion".to_string(),
                    reference: None,
                    text: None
                }
                .into()
            )
        )
    }
    #[test]
    fn ofm_case_wikilink_2() {
        let text = r#"[[Three laws of motion#Second law]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(
                link::Wikilink {
                    path: "Three laws of motion".to_string(),
                    reference: Some(Reference::Heading("Second law".to_string())),
                    text: None
                }
                .into()
            )
        )
    }
    #[test]
    fn ofm_case_wikilink_3() {
        let text = r#"[[My note#Heading 1#Heading 2]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(
                link::Wikilink {
                    path: "My note".to_string(),
                    reference: Some(Reference::MultiHeading(vec![
                        "Heading 1".to_string(),
                        "Heading 2".to_string()
                    ])),
                    text: None
                }
                .into()
            )
        )
    }
    #[test]
    fn ofm_case_wikilink_4() {
        let text = r#"[[2023-01-01#^quote-of-the-day]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(
                link::Wikilink {
                    path: "2023-01-01".to_string(),
                    reference: Some(Reference::BlockId("quote-of-the-day".to_string())),
                    text: None
                }
                .into()
            )
        )
    }
    #[test]
    fn ofm_case_wikilink_5() {
        let text = r#"[[Internal links|custom display text]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Link(
                link::Wikilink {
                    path: "Internal links".to_string(),
                    reference: None,
                    text: Some("custom display text".to_string())
                }
                .into()
            )
        )
    }
    
    #[test]
    fn ofm_case_embed_1(){
        let text = r#"![[Internal links]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(
                embed::Embed{
                    path: "Internal links".to_string(),
                    size: None,
                    reference: None,
                    attrs: None,
                }
            )
        )
    }
    #[test]
    fn ofm_case_embed_2(){
        let text = r#"![[Internal links#^b15695]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(
                embed::Embed{
                    path: "Internal links".to_string(),
                    size: None,
                    reference: Some(Reference::BlockId("b15695".to_string())),
                    attrs: None,
                }
            )
        )
    }
    #[test]
    fn ofm_case_embed_3(){
        let text = r#"![[Engelbart.jpg|100x145]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(
                embed::Embed{
                    path: "Engelbart.jpg".to_string(),
                    size: Some((100, Some(145))),
                    reference: None,
                    attrs: None,
                }
            )
        )
    }
    #[test]
    fn ofm_case_embed_4(){
        let text = r#"![[Engelbart.jpg|100]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(
                embed::Embed{
                    path: "Engelbart.jpg".to_string(),
                    size: Some((100, None)),
                    reference: None,
                    attrs: None,
                }
            )
        )
    }
    #[test]
    fn ofm_case_embed_5(){
        let text = r#"![[Document.pdf#page=3]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(
                embed::Embed{
                    path: "Document.pdf".to_string(),
                    size: None,
                    reference: None,
                    attrs: Some(vec![
                        ("page".to_string(), "3".to_string()),
                    ]),
                }
            )
        )
    }
    #[test]
    fn ofm_case_embed_6(){
        let text = r#"![[Document.pdf#page=3&theme=dark]]"#;
        let ast = Parser::new(text).parse();
        assert_eq!(
            ast[2].body,
            MarkdownNode::Embed(
                embed::Embed{
                    path: "Document.pdf".to_string(),
                    size: None,
                    reference: None,
                    attrs: Some(vec![
                        ("page".to_string(), "3".to_string()),
                        ("theme".to_string(), "dark".to_string()),
                    ]),
                }
            )
        )
    }
}