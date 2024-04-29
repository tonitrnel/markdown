use crate::ast::{html, MarkdownNode};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, Line, ProcessCtx};
use crate::tokenizer::Token;

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
const TYPE_1_TAGS: &[&str; 4] = &["pre", "style", "script", "textarea"];
/// example: `<pre`、`<script` ...
fn is_begin_type_1(scan_start_result: &scanners::ScanStartResult) -> bool {
    if TYPE_1_TAGS
        .iter()
        .any(|it| it.eq(&scan_start_result.0.name) && scan_start_result.0.props.is_none())
    {
        return true;
    }
    false
}
fn is_end_type_1(scan_end_result: &scanners::ScanEndResult) -> bool {
    for tag in TYPE_1_TAGS {
        if !tag.eq_ignore_ascii_case(&scan_end_result.0) {
            continue;
        }
        return true;
    }
    false
}
/// example: `<!--`
fn is_begin_type_2(line: &Line) -> bool {
    line.validate(3, Token::Hyphen)
}
/// example: `-->`
fn is_end_type_2(line: &Line) -> bool {
    line.validate(0, Token::Hyphen)
        && line.validate(1, Token::Hyphen)
        && line.validate(2, Token::Gt)
}
/// example: `?>`
fn is_end_type_3(line: &Line) -> bool {
    line.validate(0, Token::Question) && line.validate(1, Token::Gt)
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
/// example: `>`
fn is_end_type_4(line: &Line) -> bool {
    line.validate(0, Token::Gt)
}
/// example: `<![CDATA[`
fn is_begin_type_5(line: &Line) -> bool {
    line.validate(3, Token::Text("CDATA")) && line.validate(4, Token::LBracket)
}
/// example: `]]>`
fn is_end_type_5(line: &Line) -> bool {
    line.validate(0, Token::DoubleRBracket) && line.validate(1, Token::Gt)
}

/// example: `<address>`, `<br/>`
fn is_begin_type_6(scan_start_result: &scanners::ScanStartResult) -> bool {
    is_html_tag(scan_start_result.0.name.as_bytes())
}
/// example: `</div>`
fn is_end_type_6(scan_end_result: &scanners::ScanEndResult) -> bool {
    is_html_tag(scan_end_result.0.as_bytes())
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

/// 扫描 HTML 标签的类型
///
/// 返回：
/// - `usize`: 起始位置，一般为 0，如果不为 0 则意味着反转，需要将 0 至该值范围内的 Token 插入到后面创建的容器，
///    只有 Type6 和 Type7 结束才会可能大于 0
/// - `usize`: 标签长度（需要跳过）
/// - `html:HtmlType`: HTML 类型
pub(crate) fn scan_html_type(
    line: &mut Line,
    is_inline: bool,
) -> Option<(usize, usize, html::HtmlType)> {
    let offset = if is_inline { line.start_offset } else { 0 };
    match line.get_raw(offset + 1) {
        // type 2, 4, 5
        Some(Token::ExclamationMark) => match line.get_raw(offset + 2) {
            Some(Token::Hyphen) => {
                if is_begin_type_2(line) {
                    Some((0, 0, html::HtmlType::Type2))
                } else {
                    None
                }
            }
            Some(Token::Text(_)) => {
                if is_begin_type_4(line) {
                    Some((0, 0, html::HtmlType::Type4))
                } else {
                    None
                }
            }
            Some(Token::LBracket) => {
                if is_begin_type_5(line) {
                    Some((0, 0, html::HtmlType::Type5))
                } else {
                    None
                }
            }
            _ => None,
        },
        // type 3
        Some(Token::Question) => Some((0, 0, html::HtmlType::Type3)),
        // type 1, 6, 7
        Some(Token::Text(_)) => {
            let scan_start_result = scanners::scan_html_start(line);
            Some(
                if scan_start_result
                    .as_ref()
                    .filter(|it| is_begin_type_1(it))
                    .is_some()
                {
                    let (element, end, self_close) = scan_start_result.unwrap();
                    (
                        0,
                        end,
                        html::HtmlType::Type1(
                            element,
                            if self_close {
                                html::Flag::SelfClose
                            } else {
                                html::Flag::Begin
                            },
                        ),
                    )
                } else if scan_start_result
                    .as_ref()
                    .filter(|it| is_begin_type_6(it))
                    .is_some()
                {
                    let (element, end, self_close) = scan_start_result.unwrap();
                    (
                        0,
                        end,
                        html::HtmlType::Type6(
                            element,
                            if self_close {
                                html::Flag::SelfClose
                            } else {
                                html::Flag::Begin
                            },
                        ),
                    )
                } else if let Some((element, end, self_close)) = scan_start_result {
                    (
                        0,
                        end,
                        html::HtmlType::Type7(
                            element,
                            if self_close {
                                html::Flag::SelfClose
                            } else {
                                html::Flag::Begin
                            },
                        ),
                    )
                } else {
                    return None;
                },
            )
        }
        // type 6 end, type 7 end
        Some(Token::Slash) => {
            // 如果是 Block 就直接扫描当前行最后一个
            let last = if is_inline {
                0
            } else {
                line.iter()
                    .enumerate()
                    .rposition(|(i, item)| {
                        item.token == Token::Lt && line.get(i + 1) == Some(&Token::Slash)
                    })
                    .unwrap_or(0)
            };
            let snapshot = line.snapshot();
            if last > 0 {
                line.skip(last);
            }
            let scan_end_result = scanners::scan_html_end(line);
            let r = if scan_end_result
                .as_ref()
                .filter(|it| is_end_type_6(it))
                .is_some()
            {
                let (name, end) = scan_end_result.unwrap();
                Some((
                    last,
                    end,
                    html::HtmlType::Type6(html::Element::new(name), html::Flag::End),
                ))
            } else if let Some((name, end)) = scan_end_result {
                Some((
                    last,
                    end,
                    html::HtmlType::Type7(html::Element::new(name), html::Flag::End),
                ))
            } else {
                None
            };
            if last > 0 {
                line.resume(snapshot);
            }
            r
        }
        _ => None,
    }
}

impl html::Html {
    /// 扫描当前行是否存在 HTML Block 结束标志
    ///
    /// 返回：
    /// `Option<(usize, usize)>`
    /// - 第一个 `usize` 为结束标志起始位置，在这之前的 Token 需要写入 HTML
    /// - 第二个 `usize` 为结束标志结束位置，在这之后的 Token 需要新建 Paragraph 存储
    pub fn scan_end(&mut self, line: &mut Line) -> Option<(usize, usize)> {
        let mut prev_maybe_end = 0;
        match self {
            html::Html::Block(html::HtmlType::Type1(element, flag @ html::Flag::Begin)) => {
                for maybe_end in scanners::scan_identifier(line, &[Token::Lt, Token::Slash]) {
                    line.skip(maybe_end - prev_maybe_end);
                    let scan_end_result = scanners::scan_html_end(line);
                    if let Some((_, end)) = scan_end_result
                        .filter(is_end_type_1)
                        .filter(|it| it.0.eq(&element.name))
                    {
                        *flag = html::Flag::Full;
                        return Some((maybe_end, maybe_end + end));
                    }
                    prev_maybe_end = maybe_end;
                }
            }
            html::Html::Block(html::HtmlType::Type2) => {
                for maybe_end in
                    scanners::scan_identifier(line, &[Token::Hyphen, Token::Hyphen, Token::Gt])
                {
                    line.skip(maybe_end - prev_maybe_end);
                    if is_end_type_2(line) {
                        return Some((maybe_end + 3, maybe_end + 3));
                    }
                    prev_maybe_end = maybe_end;
                }
            }
            html::Html::Block(html::HtmlType::Type3) => {
                for maybe_end in scanners::scan_identifier(line, &[Token::Question, Token::Gt]) {
                    line.skip(maybe_end - prev_maybe_end);
                    if is_end_type_3(line) {
                        return Some((maybe_end + 2, maybe_end + 2));
                    }
                    prev_maybe_end = maybe_end;
                }
            }
            html::Html::Block(html::HtmlType::Type4) => {
                for maybe_end in scanners::scan_identifier(line, &[Token::Gt]) {
                    line.skip(maybe_end - prev_maybe_end);
                    if is_end_type_4(line) {
                        return Some((maybe_end + 1, maybe_end + 1));
                    }
                    prev_maybe_end = maybe_end;
                }
            }
            html::Html::Block(html::HtmlType::Type5) => {
                for maybe_end in
                    scanners::scan_identifier(line, &[Token::DoubleRBracket, Token::Gt])
                {
                    line.skip(maybe_end - prev_maybe_end);
                    if is_end_type_5(line) {
                        return Some((maybe_end + 2, maybe_end + 2));
                    }
                    prev_maybe_end = maybe_end;
                }
            }
            html::Html::Block(html::HtmlType::Type6(element, flag @ html::Flag::Begin)) => {
                for maybe_end in scanners::scan_identifier(line, &[Token::Lt, Token::Slash]) {
                    line.skip(maybe_end - prev_maybe_end);
                    let scan_end_result = scanners::scan_html_end(line);
                    if let Some((_, end)) = scan_end_result
                        .filter(is_end_type_6)
                        .filter(|it| it.0.eq(&element.name))
                    {
                        *flag = html::Flag::Full;
                        return Some((maybe_end, maybe_end + end));
                    }
                    prev_maybe_end = maybe_end;
                }
            }
            html::Html::Block(html::HtmlType::Type7(element, flag @ html::Flag::Begin)) => {
                // println!("扫描 HTML Block 是否结束 {:?}", line)
                for maybe_end in scanners::scan_identifier(line, &[Token::Lt, Token::Slash]) {
                    line.skip(maybe_end - prev_maybe_end);
                    // println!(
                    //     "maybe_end = {maybe_end} result={:?}",
                    //     scanners::scan_html_end(line)
                    // );
                    if let Some((_, end)) =
                        scanners::scan_html_end(line).filter(|it| it.0.eq(&element.name))
                    {
                        *flag = html::Flag::Full;
                        return Some((maybe_end, maybe_end + end));
                    }
                    prev_maybe_end = maybe_end;
                }
            }
            _ => return None,
        }
        None
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
        let (start, len, block_type) = if let Some(block_type) = scan_html_type(line, false) {
            block_type
        } else {
            return BlockMatching::Unmatched;
        };
        // println!("{:?}", block_type);
        match &block_type {
            html::HtmlType::Type1(..)
            | html::HtmlType::Type2
            | html::HtmlType::Type3
            | html::HtmlType::Type4
            | html::HtmlType::Type5
            | html::HtmlType::Type6(_, html::Flag::SelfClose) => {
                parser.close_unmatched_blocks();
                line.skip(len);
                parser.append_block(MarkdownNode::Html(html::Html::Block(block_type)), location);
                BlockMatching::MatchedLeaf
            }
            html::HtmlType::Type7(el, html::Flag::SelfClose) => {
                if !(parser.tree[container].body != MarkdownNode::Paragraph
                    && !(!parser.all_closed
                        && !line.is_blank()
                        && parser.current_proc().body == MarkdownNode::Paragraph))
                {
                    BlockMatching::Unmatched
                } else {
                    parser.close_unmatched_blocks();
                    line.skip(len);
                    let name = el.name.clone();
                    let idx = parser
                        .append_block(MarkdownNode::Html(html::Html::Block(block_type)), location);
                    parser.html_stacks.push_front((name, idx));
                    BlockMatching::MatchedLeaf
                }
            }
            html::HtmlType::Type6(el, flag) | html::HtmlType::Type7(el, flag) => {
                // Type 7 可能会和现有的 Link Reference Definition 冲突，因此需要判断是否为 Paragraph，
                // Link Reference Definition 是基于 Paragraph 解析，不过是否应该约束 Type 7 以大写字符开始，
                // 就像 JSX 组件那样，这也是编写该库的目的之一
                if matches!(block_type, html::HtmlType::Type7(..))
                    && (parser.tree[container].body == MarkdownNode::Paragraph
                        || (!parser.all_closed
                            && !line.is_blank()
                            && parser.current_proc().body == MarkdownNode::Paragraph))
                {
                    // println!(
                    //     "AAA => body = {:?}, all_closed = {}, is_blank = {}, current_proc = {:?}",
                    //     parser.tree[container].body,
                    //     parser.all_closed,
                    //     line.is_blank(),
                    //     parser.current_proc().body
                    // );
                    return BlockMatching::Unmatched;
                }
                parser.close_unmatched_blocks();
                match flag {
                    html::Flag::Begin => {
                        line.skip(len);
                        let key = if let Some((name, _)) = parser.html_stacks.back() {
                            format!("{name}{}/", el.name)
                        } else {
                            format!("{}/", el.name)
                        };
                        let idx = parser.append_block(
                            MarkdownNode::Html(html::Html::Block(block_type)),
                            location,
                        );
                        parser.html_stacks.push_front((key, idx));
                        BlockMatching::MatchedLeaf
                    }
                    html::Flag::End => {
                        // 找到相匹配的节点位置
                        let index = if let Some((last_key, _)) = parser.html_stacks.back() {
                            let key = format!("{}/", el.name);
                            let index = 'loop_match_tag: {
                                if last_key.starts_with(&key) {
                                    while let Some((last_key, idx)) = parser.html_stacks.pop_back()
                                    {
                                        if last_key == key {
                                            break 'loop_match_tag Some(idx);
                                        }
                                        if !last_key.starts_with(&key) {
                                            break;
                                        }
                                    }
                                }
                                None
                            };
                            index.filter(|idx| {
                                matches!(
                                    parser.tree[*idx].body,
                                    MarkdownNode::Html(html::Html::Block(
                                        html::HtmlType::Type6(_, html::Flag::Begin)
                                            | html::HtmlType::Type7(_, html::Flag::Begin)
                                    ))
                                )
                            })
                        } else {
                            None
                        };
                        if let Some(parent) = index {
                            let mut next = parser.tree.get_next(parent);
                            // 将后面的节点全部变为目标的子节点
                            while let Some(idx) = next {
                                next = parser.tree.get_next(idx);
                                parser.tree.unlink(idx);
                                parser.tree.set_parent(idx, parent);
                            }
                            if start > 0 {
                                let previous = line.slice(0, start);
                                parser.append_text_to(
                                    parent,
                                    previous.to_string(),
                                    (
                                        previous.start_location(),
                                        previous.last_token_end_location(),
                                    ),
                                );
                            }
                            if let MarkdownNode::Html(html::Html::Block(
                                html::HtmlType::Type6(_, flag) | html::HtmlType::Type7(_, flag),
                            )) = &mut parser.tree[parent].body
                            {
                                *flag = html::Flag::Full
                            }
                            line.skip(start + len);
                            if !line.is_end() {
                                let idx = parser
                                    .append_block(MarkdownNode::Paragraph, line.start_location());
                                parser.append_inline(idx, line.slice(0, line.len()));
                                line.skip_to_end()
                            }
                        } else {
                            if start > 0 {
                                let previous = line.slice(0, start);
                                parser.append_text(
                                    previous.to_string(),
                                    (
                                        previous.start_location(),
                                        previous.last_token_end_location(),
                                    ),
                                );
                            }
                            let idx = parser.append_block(
                                MarkdownNode::Html(html::Html::Block(block_type)),
                                location,
                            );
                            line.skip(start + len);
                            if !line.is_end() {
                                parser.finalize(idx, line.start_location());
                                let idx = parser
                                    .append_block(MarkdownNode::Paragraph, line.start_location());
                                parser.append_inline(idx, line.slice(0, line.len()));
                                line.skip_to_end()
                            }
                        }
                        BlockMatching::MatchedLeaf
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn process(ProcessCtx { line, parser, id }: ProcessCtx) -> BlockProcessing {
        // html type 6 and html 7 可以在空行中断
        if line.is_blank()
            && matches!(
                parser.tree[id].body,
                MarkdownNode::Html(html::Html::Block(
                    html::HtmlType::Type6(..) | html::HtmlType::Type7(..)
                ))
            )
        {
            return BlockProcessing::Unprocessed;
        }
        BlockProcessing::Further
    }
}

mod scanners {
    use std::borrow::Cow;

    use crate::ast::html;
    use crate::line::Line;
    use crate::tokenizer::{Token, Whitespace};

    #[derive(Debug)]
    enum State {
        Initial,
        InTag(usize), // start
        InAttr(InAttr),
    }

    #[derive(Debug)]
    enum InAttr {
        Attr(Option<usize>),                      // previous attr index
        InName(usize),                            // attr name: start
        InValue(AttrQuote, Option<usize>, usize), // attr value: quote, attr name index, start
    }

    #[derive(Debug)]
    enum AttrQuote {
        SingleQuote,
        DoubleQuote,
        Empty,
        None,
    }
    impl<'input> PartialEq<Token<'input>> for AttrQuote {
        fn eq(&self, other: &Token<'input>) -> bool {
            matches!(
                (self, other),
                (AttrQuote::DoubleQuote, Token::DoubleQuote)
                    | (AttrQuote::SingleQuote, Token::SingleQuote)
            )
        }
    }
    pub(super) type ScanStartResult = (html::Element, usize, bool);
    pub(super) type ScanEndResult = (String, usize);
    /// 扫描 HTML 标签
    ///
    /// 返回：
    /// - html::Element
    /// - usize: 结束位置
    /// - bool: 是否为 self close tag
    pub(super) fn scan_html_start(line: &Line) -> Option<ScanStartResult> {
        let iter = line.iter().skip(1).enumerate();
        let mut name = String::new();
        let mut attrs = Vec::<(String, Cow<str>)>::new();
        let mut state = State::Initial;
        let mut end = 0;
        let mut self_close = false;
        let is_close_tag = |pos: usize| {
            line.validate(pos, Token::Gt)
                || (line.validate(pos, Token::Slash) && line.validate(pos + 1, Token::Gt))
        };
        // println!("000000000000000000000000000000000000000000000000000000");
        for (i, item) in iter {
            // 因为 skip 导致 pos 实际上小了
            let i = i + 1;
            // println!(
            //     "scan_html_start #{i} state = {state:?}, token = {:?}",
            //     item.token
            // );
            match (&state, &item.token) {
                (State::Initial, Token::Text(_)) if item.token.is_ascii_alphabetic() => {
                    state = State::InTag(i)
                }
                // enter <in name>
                (State::InTag(_), Token::Text(_) | Token::Digit(_) | Token::Hyphen) => (),
                // advance to <in attr>
                (State::InTag(start), Token::Whitespace(Whitespace::Space)) => {
                    name = line.slice(*start, i).to_string();
                    state = State::InAttr(InAttr::Attr(None))
                }
                // exit <in name>
                (State::InTag(start), Token::Gt | Token::Slash) if is_close_tag(i) => {
                    name = line.slice(*start, i).to_string();
                    state = State::Initial;
                    if item.token == Token::Gt {
                        end = i + 1
                    } else {
                        end = i + 2;
                        self_close = true;
                    };
                    break;
                }
                // enter <in attr>(未进入任何属性状态)
                (State::InAttr(InAttr::Attr(_)), Token::Text(str)) if str.is_ascii() => {
                    state = State::InAttr(InAttr::InName(i))
                }
                // advance to <in attr name>
                (State::InAttr(InAttr::Attr(None)), Token::Underscore | Token::Colon) => {
                    state = State::InAttr(InAttr::InName(i))
                }
                // advance to <in attr value>
                (State::InAttr(InAttr::Attr(Some(attr_index))), Token::Eq) => {
                    state =
                        State::InAttr(InAttr::InValue(AttrQuote::None, Some(*attr_index), i + 1))
                }
                (State::InAttr(InAttr::Attr(_)), Token::Whitespace(..)) => {}
                // exit <in attr>
                (State::InAttr(InAttr::Attr(_)), Token::Gt | Token::Slash) if is_close_tag(i) => {
                    state = State::Initial;
                    if item.token == Token::Gt {
                        end = i + 1
                    } else {
                        end = i + 2;
                        self_close = true;
                    };
                    break;
                }
                // <in attr name>
                (State::InAttr(InAttr::InName(_)), Token::Text(_) | Token::Digit(..)) => (),
                (
                    State::InAttr(InAttr::InName(_)),
                    Token::Hyphen | Token::Underscore | Token::Colon,
                ) => (),
                // advance to <in attr value>
                (State::InAttr(InAttr::InName(start)), Token::Eq) => {
                    let attr = line.slice(*start, i).to_string();
                    let index = attrs.len();
                    attrs.push((attr, Cow::Borrowed("")));
                    state = State::InAttr(InAttr::InValue(AttrQuote::None, Some(index), i + 1))
                }
                // advance to <in attr>
                (State::InAttr(InAttr::InName(start)), Token::Whitespace(Whitespace::Space)) => {
                    let attr = line.slice(*start, i).to_string();
                    let index = attrs.len();
                    attrs.push((attr, Cow::Borrowed("")));
                    // 无法判断是否结束还是只是等号前后存在空格，因此将当前 attr name 所在位置存储带下一次处理
                    state = State::InAttr(InAttr::Attr(Some(index)))
                }
                // exit <in attr name>
                (State::InAttr(InAttr::InName(start)), Token::Slash | Token::Gt)
                    if is_close_tag(i) =>
                {
                    let attr = line.slice(*start, i).to_string();
                    attrs.push((attr, Cow::Borrowed("")));
                    state = State::Initial;
                    if item.token == Token::Gt {
                        end = i + 1
                    } else {
                        end = i + 2;
                        self_close = true;
                    };
                    break;
                }
                // enter <in attr value> or exit <in attr value>, 如果当前 Token 为引号，用于匹配带引号的属性值
                (
                    State::InAttr(InAttr::InValue(AttrQuote::None, None, _)),
                    Token::Whitespace(Whitespace::Space | Whitespace::Tab),
                ) => (),
                (
                    State::InAttr(InAttr::InValue(
                        quote @ (AttrQuote::None | AttrQuote::DoubleQuote | AttrQuote::SingleQuote),
                        Some(index),
                        start,
                    )),
                    Token::DoubleQuote | Token::SingleQuote,
                ) => {
                    state = if quote.eq(&item.token) {
                        // 结束 attr value
                        let value = line.slice(*start, i).to_unescape_string();
                        attrs[*index].1 = Cow::Owned(value);
                        State::InAttr(InAttr::Attr(None))
                    } else {
                        // 开始 attr value
                        State::InAttr(InAttr::InValue(
                            if item.token == Token::SingleQuote {
                                AttrQuote::SingleQuote
                            } else {
                                AttrQuote::DoubleQuote
                            },
                            Some(*index),
                            i + 1, // 跳过引号
                        ))
                    }
                }
                // <in attr value>
                (
                    State::InAttr(InAttr::InValue(
                        AttrQuote::DoubleQuote | AttrQuote::SingleQuote,
                        ..,
                    )),
                    _,
                ) => (),
                // enter <in attr value> 用于匹配无引号的属性值
                (
                    State::InAttr(InAttr::InValue(
                        quote @ (AttrQuote::None | AttrQuote::Empty),
                        Some(index),
                        _,
                    )),
                    t,
                ) if !t.is_control() && !t.in_str("=<>`") => {
                    if matches!(quote, AttrQuote::None) {
                        // 无引号的 attr value 开始
                        state = State::InAttr(InAttr::InValue(AttrQuote::Empty, Some(*index), i))
                    }
                }
                // exit <in attr value>
                (
                    State::InAttr(InAttr::InValue(AttrQuote::Empty, Some(index), start)),
                    Token::Whitespace(Whitespace::Space | Whitespace::Tab),
                ) => {
                    // 无引号的 attr value 结束
                    let value = line.slice(*start, i).to_unescape_string();
                    attrs[*index].1 = Cow::Owned(value);
                    state = State::InAttr(InAttr::Attr(None));
                }
                (
                    State::InAttr(InAttr::InValue(AttrQuote::Empty, Some(index), start)),
                    Token::Gt | Token::Slash,
                ) if is_close_tag(i) => {
                    // 无引号的 attr value 结束
                    let value = line.slice(*start, i).to_unescape_string();
                    attrs[*index].1 = Cow::Owned(value);
                    state = State::Initial;
                    if item.token == Token::Gt {
                        end = i + 1
                    } else {
                        end = i + 2;
                        self_close = true;
                    };
                    break;
                }
                _ => return None,
            }
        }
        // println!(
        //     "is_begin_type_7 finish state={state:?} name = {name:?} attrs = {attrs:?} end = {end}"
        // );
        if matches!(state, State::Initial) && end > 0 {
            Some((
                html::Element::new_with_props(name, Some(attrs)),
                end,
                self_close,
            ))
        } else {
            None
        }
    }
    pub(super) fn scan_html_end(line: &Line) -> Option<ScanEndResult> {
        // println!("scan_html_end line = {line:?}")
        if !line.validate(0, Token::Lt) || !line.validate(1, Token::Slash) {
            return None;
        }
        let mut state = State::Initial;
        let mut end = 0;
        let mut name = String::new();
        let iter = line.iter().skip(2).enumerate();
        for (i, item) in iter {
            let i = i + 2;
            // println!(
            //     "scan_html_end #{i} state = {state:?}, token = {:?}",
            //     item.token
            // );
            match (&state, &item.token) {
                (State::Initial, Token::Text(_)) if item.token.is_ascii_alphabetic() => {
                    state = State::InTag(i)
                }
                (State::InTag(_), Token::Text(_) | Token::Digit(_) | Token::Hyphen) => (),
                (State::InTag(start), Token::Whitespace(Whitespace::Space)) => {
                    name = line.slice(*start, i).to_string();
                    state = State::InAttr(InAttr::Attr(None))
                }
                (State::InTag(start), Token::Gt) => {
                    name = line.slice(*start, i).to_string();
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                (
                    State::InAttr(InAttr::Attr(None)),
                    Token::Whitespace(Whitespace::Space | Whitespace::Tab),
                ) => (),
                (State::InAttr(InAttr::Attr(None)), Token::Gt) => {
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                _ => return None,
            }
        }
        if matches!(state, State::Initial) && end > 0 {
            Some((name, end))
        } else {
            None
        }
    }
    pub(super) fn scan_identifier(line: &Line, tokens: &[Token]) -> Vec<usize> {
        assert!(!tokens.is_empty());
        line.iter()
            .enumerate()
            .filter_map(|(i, item)| {
                if item.token == tokens[0]
                    && tokens
                        .iter()
                        .enumerate()
                        .skip(1)
                        .all(|(n, it)| line.validate(i + n, it))
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
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
        // println!("{:?}", ast)
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::HtmlType::Type1(
                html::Element {
                    name: "script".to_string(),
                    props: None
                },
                html::Flag::Full
            )))
        );
        assert_eq!(ast.len(), 3);
    }
    #[test]
    fn case_2() {
        let text = r#"
<!--comme
h
nts-->h"#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        // println!("{:?}", ast)
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::HtmlType::Type2))
        );
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
        // println!("{:?}", ast)
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::HtmlType::Type3))
        );
    }
    #[test]
    fn case_4() {
        let text = r#"
<!DOCTYPE html>
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        // println!("{:?}", ast)
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::HtmlType::Type4))
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
        // println!("{:?}", ast)
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::HtmlType::Type5))
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
        // println!("{:?}", ast)
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::HtmlType::Type6(
                html::Element {
                    name: "p".to_string(),
                    props: None
                },
                html::Flag::Full
            )))
        );
    }
    #[test]
    fn case_7() {
        let text = r#"<Button>Click Me 1</Button>"#.trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(html::Html::Block(html::HtmlType::Type7(
                html::Element {
                    name: "Button".to_string(),
                    props: None
                },
                html::Flag::Full
            )))
        );
        // println!("{:?}", ast)
    }
    #[test]
    fn case_148() {
        let text = r#"<table><tr><td>
<pre>
**Hello**,

_world_.
</pre>
</td></tr></table>"#
            .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        println!("AST:\n{:?}", ast);
        assert_eq!(
            ast.to_html(),
            r#"<table>
<tr><td><pre>**Hello**,
<p><em>world</em>.
</pre></p>
</td></tr>
</table>"#
        )
    }
}
