use crate::ast::{MarkdownNode, html};
use crate::blocks::{BeforeCtx, BlockMatching, BlockProcessing, BlockStrategy, ProcessCtx};
use crate::span::Span;

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

fn is_begin_type_1(result: &scanners::ScanStartResult) -> bool {
    TYPE_1_TAGS
        .iter()
        .any(|it| it.eq_ignore_ascii_case(&result.0.name))
}
// fn is_end_type_1(result: &scanners::ScanEndResult) -> bool {
//     TYPE_1_TAGS
//         .iter()
//         .any(|it| it.eq_ignore_ascii_case(&result.0))
// }
fn is_begin_type_6(result: &scanners::ScanStartResult) -> bool {
    is_html_tag(result.0.name.as_bytes())
}
fn is_end_type_6(result: &scanners::ScanEndResult) -> bool {
    is_html_tag(result.0.as_bytes())
}

fn is_html_tag(tag: &[u8]) -> bool {
    HTML_TAGS
        .binary_search_by(|probe| {
            let probe_bytes_iter = probe.as_bytes().iter();
            let tag_bytes_iter = tag.iter();
            probe_bytes_iter
                .zip(tag_bytes_iter)
                .find_map(|(&a, &b)| match a.cmp(&(b | 0x20)) {
                    std::cmp::Ordering::Equal => None,
                    inequality => Some(inequality),
                })
                .unwrap_or_else(|| probe.len().cmp(&tag.len()))
        })
        .is_ok()
}

/// 扫描 HTML 标签的类型（基于 Span 字节操作）
///
/// 返回：
/// - `usize`: 起始位置，一般为 0，如果不为 0 则意味着反转（仅 Type6/Type7 End）
/// - `usize`: 标签长度（需要跳过）
/// - `html::HtmlType`: HTML 类型
pub(crate) fn scan_html_type(
    span: &mut Span,
    is_inline: bool,
    mdx_components: bool,
) -> Option<(usize, usize, html::HtmlType)> {
    let offset = 0;
    let get = |i: usize| -> Option<u8> { span.get(offset + i) };
    // span[offset] should be '<'
    match get(1)? {
        // type 2: <!--, type 4: <!LETTER, type 5: <![CDATA[
        b'!' => match get(2)? {
            b'-' => {
                // <!-- (type 2)
                if get(3) == Some(b'-') {
                    Some((0, 4, html::HtmlType::HtmlComment))
                } else {
                    None
                }
            }
            b if b.is_ascii_alphabetic() => {
                // <!LETTER (type 4)
                let mut end = 3;
                while let Some(b) = get(end) {
                    if b == b'>' {
                        end += 1;
                        break;
                    }
                    end += 1;
                }
                Some((0, end, html::HtmlType::Declaration))
            }
            b'[' => {
                // <![CDATA[ (type 5)
                let bytes = b"CDATA[";
                let mut ok = true;
                for (j, &expected) in bytes.iter().enumerate() {
                    if get(3 + j) != Some(expected) {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    Some((0, 9, html::HtmlType::CDataSection))
                } else {
                    None
                }
            }
            _ => None,
        },
        // type 3: <?
        b'?' => Some((0, 2, html::HtmlType::ProcessingInstruction)),
        // type 1, 6, 7: <tagname...
        b if b.is_ascii_alphabetic() => {
            if !is_inline {
                let (name, name_end) = scan_tag_name_prefix(span, 1)?;
                let delimiter = span.get(name_end);
                if !is_block_tag_start_delimiter(delimiter, span.get(name_end + 1)) {
                    return None;
                }

                if TYPE_1_TAGS.iter().any(|it| it.eq_ignore_ascii_case(&name)) {
                    if let Some((element, end, self_close)) = scanners::scan_html_start(span) {
                        return Some((
                            0,
                            end,
                            html::HtmlType::RawTextContainer(
                                element,
                                if self_close {
                                    html::Flag::SelfClose
                                } else {
                                    html::Flag::Begin
                                },
                            ),
                        ));
                    }
                    return Some((
                        0,
                        0,
                        html::HtmlType::RawTextContainer(
                            html::Element::new(name),
                            html::Flag::Begin,
                        ),
                    ));
                }
                if is_html_tag(name.as_bytes()) {
                    if let Some((element, end, self_close)) = scanners::scan_html_start(span) {
                        return Some((
                            0,
                            end,
                            html::HtmlType::CanonicalBlockTag(
                                element,
                                if self_close {
                                    html::Flag::SelfClose
                                } else {
                                    html::Flag::Begin
                                },
                            ),
                        ));
                    }
                    return Some((
                        0,
                        0,
                        html::HtmlType::CanonicalBlockTag(
                            html::Element::new(name),
                            html::Flag::Begin,
                        ),
                    ));
                }

                if let Some((element, end, self_close)) = scanners::scan_html_start(span) {
                    if element.name.contains('.')
                        && !(mdx_components && is_component_name(&element.name))
                    {
                        return None;
                    }
                    if !line_tail_is_space_or_tab(span, end) {
                        return None;
                    }
                    let is_component = is_component_name(&element.name);
                    if mdx_components && !is_component {
                        return None;
                    }
                    return Some((
                        0,
                        end,
                        if mdx_components && is_component {
                            html::HtmlType::Component(
                                element,
                                if self_close {
                                    html::Flag::SelfClose
                                } else {
                                    html::Flag::Begin
                                },
                            )
                        } else {
                            html::HtmlType::GenericTag(
                                element,
                                if self_close {
                                    html::Flag::SelfClose
                                } else {
                                    html::Flag::Begin
                                },
                            )
                        },
                    ));
                }
                return None;
            }
            let result = scanners::scan_html_start(span);
            Some(
                if let Some((element, end, self_close)) =
                    result.as_ref().filter(|it| is_begin_type_1(it)).cloned()
                {
                    (
                        0,
                        end,
                        html::HtmlType::RawTextContainer(
                            element,
                            if self_close {
                                html::Flag::SelfClose
                            } else {
                                html::Flag::Begin
                            },
                        ),
                    )
                } else if let Some((element, end, self_close)) =
                    result.as_ref().filter(|it| is_begin_type_6(it)).cloned()
                {
                    (
                        0,
                        end,
                        html::HtmlType::CanonicalBlockTag(
                            element,
                            if self_close {
                                html::Flag::SelfClose
                            } else {
                                html::Flag::Begin
                            },
                        ),
                    )
                } else if let Some((element, end, self_close)) = result {
                    if element.name.contains('.')
                        && !(mdx_components && is_component_name(&element.name))
                    {
                        return None;
                    }
                    let is_component = is_component_name(&element.name);
                    if mdx_components && !is_component {
                        return None;
                    }
                    (
                        0,
                        end,
                        if mdx_components && is_component {
                            html::HtmlType::Component(
                                element,
                                if self_close {
                                    html::Flag::SelfClose
                                } else {
                                    html::Flag::Begin
                                },
                            )
                        } else {
                            html::HtmlType::GenericTag(
                                element,
                                if self_close {
                                    html::Flag::SelfClose
                                } else {
                                    html::Flag::Begin
                                },
                            )
                        },
                    )
                } else {
                    return None;
                },
            )
        }
        // type 6 end, type 7 end: </tagname>
        b'/' => {
            if !is_inline {
                let result = scanners::scan_html_end(span)?;
                if !line_tail_is_space_or_tab(span, result.1) {
                    return None;
                }
                return if is_end_type_6(&result) {
                    Some((
                        0,
                        result.1,
                        html::HtmlType::CanonicalBlockTag(
                            html::Element::new(result.0),
                            html::Flag::End,
                        ),
                    ))
                } else {
                    if result.0.contains('.') && !(mdx_components && is_component_name(&result.0)) {
                        return None;
                    }
                    let is_component = is_component_name(&result.0);
                    if mdx_components && !is_component {
                        return None;
                    }
                    Some((
                        0,
                        result.1,
                        if mdx_components && is_component {
                            html::HtmlType::Component(html::Element::new(result.0), html::Flag::End)
                        } else {
                            html::HtmlType::GenericTag(
                                html::Element::new(result.0),
                                html::Flag::End,
                            )
                        },
                    ))
                };
            }
            // 如果是 Block 就直接扫描当前行最后一个 `</`
            let last = if is_inline {
                0
            } else {
                let bytes = span.as_str().as_bytes();
                let mut found = 0;
                for i in 0..bytes.len().saturating_sub(1) {
                    if bytes[i] == b'<' && bytes[i + 1] == b'/' {
                        found = i;
                    }
                }
                found
            };
            let snapshot = span.snapshot();
            if last > 0 {
                span.skip(last);
            }
            let result = scanners::scan_html_end(span);
            let r = if let Some((name, end)) =
                result.as_ref().filter(|it| is_end_type_6(it)).cloned()
            {
                Some((
                    last,
                    end,
                    html::HtmlType::CanonicalBlockTag(html::Element::new(name), html::Flag::End),
                ))
            } else if let Some((name, end)) = result {
                if name.contains('.') && !(mdx_components && is_component_name(&name)) {
                    return None;
                }
                let is_component = is_component_name(&name);
                if mdx_components && !is_component {
                    return None;
                }
                Some((
                    last,
                    end,
                    if mdx_components && is_component {
                        html::HtmlType::Component(html::Element::new(name), html::Flag::End)
                    } else {
                        html::HtmlType::GenericTag(html::Element::new(name), html::Flag::End)
                    },
                ))
            } else {
                None
            };
            if last > 0 {
                span.resume(&snapshot);
            }
            r
        }
        _ => None,
    }
}

fn scan_tag_name_prefix(span: &Span, start: usize) -> Option<(String, usize)> {
    let first = span.get(start)?;
    if !first.is_ascii_alphabetic() {
        return None;
    }
    let mut i = start + 1;
    while let Some(b) = span.get(i) {
        if b.is_ascii_alphanumeric() || b == b'-' {
            i += 1;
        } else {
            break;
        }
    }
    Some((span.slice(start, i).as_str().to_string(), i))
}
/// 行尾是否为空白
fn is_block_tag_start_delimiter(current: Option<u8>, next: Option<u8>) -> bool {
    match current {
        None => true,
        Some(b' ' | b'\t' | b'>') => true,
        Some(b'/') => next == Some(b'>'),
        _ => false,
    }
}
fn line_tail_is_space_or_tab(span: &Span, from: usize) -> bool {
    let mut i = from;
    while let Some(b) = span.get(i) {
        if b != b' ' && b != b'\t' {
            return false;
        }
        i += 1;
    }
    true
}

fn is_component_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(ch) if ch.is_ascii_uppercase() => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$' | '.'))
}

impl html::Html {
    /// 基于 Span 的 HTML Block 结束标志扫描
    ///
    /// 返回 `Option<(usize, usize)>`：
    /// - 第一个 `usize` 为结束标志起始位置（相对于 span 游标）
    /// - 第二个 `usize` 为结束标志结束位置（相对于 span 游标）
    pub fn scan_end_span(&mut self, span: &mut Span) -> Option<(usize, usize)> {
        let bytes = span.as_str().as_bytes();
        let len = bytes.len();
        match self {
            html::Html::Block(html::HtmlType::RawTextContainer(
                element,
                flag @ html::Flag::Begin,
            )) => {
                if let Some((_before, after)) = find_closing_tag(bytes, &element.name) {
                    *flag = html::Flag::Full;
                    return Some((len, if after <= len { len } else { after }));
                }
            }
            html::Html::Block(html::HtmlType::HtmlComment) => {
                if let Some(pos) = find_bytes(bytes, b"-->") {
                    let end = pos + 3;
                    return Some((len, if end <= len { len } else { end }));
                }
            }
            html::Html::Block(html::HtmlType::ProcessingInstruction) => {
                if let Some(pos) = find_bytes(bytes, b"?>") {
                    let end = pos + 2;
                    return Some((len, if end <= len { len } else { end }));
                }
            }
            html::Html::Block(html::HtmlType::Declaration) => {
                for i in 0..len {
                    if bytes[i] == b'>' {
                        let end = i + 1;
                        return Some((len, if end <= len { len } else { end }));
                    }
                }
            }
            html::Html::Block(html::HtmlType::CDataSection) => {
                if let Some(pos) = find_bytes(bytes, b"]]>") {
                    let end = pos + 3;
                    return Some((len, if end <= len { len } else { end }));
                }
            }
            html::Html::Block(html::HtmlType::CanonicalBlockTag(
                element,
                flag @ html::Flag::Begin,
            )) => {
                if let Some((_before, _after)) = find_closing_tag(bytes, &element.name) {
                    *flag = html::Flag::Full;
                    return None;
                }
            }
            html::Html::Block(
                html::HtmlType::GenericTag(element, flag @ html::Flag::Begin)
                | html::HtmlType::Component(element, flag @ html::Flag::Begin),
            )
            | html::Html::Inline(html::HtmlType::RawTextContainer(
                element,
                flag @ html::Flag::Begin,
            ))
            | html::Html::Inline(html::HtmlType::CanonicalBlockTag(
                element,
                flag @ html::Flag::Begin,
            ))
            | html::Html::Inline(html::HtmlType::GenericTag(element, flag @ html::Flag::Begin))
            | html::Html::Inline(html::HtmlType::Component(element, flag @ html::Flag::Begin)) => {
                if let Some((before, after)) = find_closing_tag(bytes, &element.name) {
                    *flag = html::Flag::Full;
                    return Some((before, after));
                }
            }
            _ => return None,
        }
        None
    }
}

/// 在字节切片中搜索子串
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    for i in 0..=(haystack.len() - needle.len()) {
        if &haystack[i..i + needle.len()] == needle {
            return Some(i);
        }
    }
    None
}

/// 搜索 `</tagname>` 或 `</tagname >` 形式的闭合标签（不区分大小写）
fn find_closing_tag(bytes: &[u8], tag_name: &str) -> Option<(usize, usize)> {
    let tag_bytes = tag_name.as_bytes();
    let tag_len = tag_bytes.len();
    let min_len = 3 + tag_len; // </tag>
    if bytes.len() < min_len {
        return None;
    }
    for i in 0..bytes.len() {
        if bytes[i] == b'<' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            let name_start = i + 2;
            if name_start + tag_len > bytes.len() {
                continue;
            }
            let name_slice = &bytes[name_start..name_start + tag_len];
            let matched = name_slice
                .iter()
                .zip(tag_bytes.iter())
                .all(|(a, b)| a.to_ascii_lowercase() == b.to_ascii_lowercase());
            if !matched {
                continue;
            }
            let mut pos = name_start + tag_len;
            while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
                pos += 1;
            }
            if pos < bytes.len() && bytes[pos] == b'>' {
                return Some((i, pos + 1));
            }
        }
    }
    None
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
        let indent_len = line.indent_len();
        if line.is_indented() {
            return BlockMatching::Unmatched;
        }
        if line.get(indent_len) != Some(b'<') {
            return BlockMatching::Unmatched;
        }
        let mut scan_line = line.slice(indent_len, line.len());
        let (start, len, block_type) = if let Some(block_type) =
            scan_html_type(&mut scan_line, false, parser.options.mdx_component)
        {
            block_type
        } else {
            return BlockMatching::Unmatched;
        };
        match &block_type {
            html::HtmlType::RawTextContainer(..)
            | html::HtmlType::HtmlComment
            | html::HtmlType::ProcessingInstruction
            | html::HtmlType::Declaration
            | html::HtmlType::CDataSection => {
                parser.close_unmatched_blocks();
                parser.append_block(
                    MarkdownNode::Html(Box::new(html::Html::Block(block_type))),
                    location,
                );
                BlockMatching::MatchedLeaf
            }
            html::HtmlType::CanonicalBlockTag(_, html::Flag::SelfClose) => {
                parser.close_unmatched_blocks();
                line.skip(indent_len + len);
                parser.append_block(
                    MarkdownNode::Html(Box::new(html::Html::Block(block_type))),
                    location,
                );
                BlockMatching::MatchedLeaf
            }
            html::HtmlType::GenericTag(el, html::Flag::SelfClose)
            | html::HtmlType::Component(el, html::Flag::SelfClose) => {
                if !(parser.tree[container].body != MarkdownNode::Paragraph
                    && !(!parser.all_closed
                        && !line.is_blank()
                        && parser.current_proc().body == MarkdownNode::Paragraph))
                {
                    BlockMatching::Unmatched
                } else {
                    parser.close_unmatched_blocks();
                    line.skip(indent_len + len);
                    let name = el.name.clone();
                    let idx = parser.append_block(
                        MarkdownNode::Html(Box::new(html::Html::Block(block_type))),
                        location,
                    );
                    parser.html_stacks.push_front((name, idx));
                    BlockMatching::MatchedLeaf
                }
            }
            html::HtmlType::CanonicalBlockTag(el, flag)
            | html::HtmlType::GenericTag(el, flag)
            | html::HtmlType::Component(el, flag) => {
                if matches!(
                    block_type,
                    html::HtmlType::GenericTag(..) | html::HtmlType::Component(..)
                ) && (parser.tree[container].body == MarkdownNode::Paragraph
                    || (!parser.all_closed
                        && !line.is_blank()
                        && parser.current_proc().body == MarkdownNode::Paragraph))
                {
                    return BlockMatching::Unmatched;
                }
                parser.close_unmatched_blocks();
                match flag {
                    html::Flag::Begin => {
                        if matches!(
                            block_type,
                            html::HtmlType::GenericTag(..) | html::HtmlType::Component(..)
                        ) {
                            line.skip(indent_len + len);
                        }
                        let key = if let Some((name, _)) = parser.html_stacks.front() {
                            format!("{name}{}/", el.name)
                        } else {
                            format!("{}/", el.name)
                        };
                        let idx = parser.append_block(
                            MarkdownNode::Html(Box::new(html::Html::Block(block_type))),
                            location,
                        );
                        parser.html_stacks.push_front((key, idx));
                        BlockMatching::MatchedLeaf
                    }
                    html::Flag::End => {
                        let index = if let Some((last_key, _)) = parser.html_stacks.front() {
                            let key = format!("{}/", el.name);
                            let index = 'loop_match_tag: {
                                if last_key.ends_with(&key) {
                                    while let Some((last_key, idx)) = parser.html_stacks.pop_front()
                                    {
                                        if last_key.ends_with(&key) {
                                            break 'loop_match_tag Some(idx);
                                        }
                                        if !last_key.ends_with(&key) {
                                            break;
                                        }
                                    }
                                }
                                None
                            };
                            index.filter(|idx| {
                                if let MarkdownNode::Html(h) = &parser.tree[*idx].body {
                                    matches!(
                                        h.as_ref(),
                                        html::Html::Block(
                                            html::HtmlType::CanonicalBlockTag(_, html::Flag::Begin)
                                                | html::HtmlType::GenericTag(_, html::Flag::Begin)
                                                | html::HtmlType::Component(_, html::Flag::Begin)
                                        )
                                    )
                                } else {
                                    false
                                }
                            })
                        } else {
                            None
                        };
                        if let Some(parent) = index {
                            let mut next = parser.tree.get_next(parent);
                            while let Some(idx) = next {
                                next = parser.tree.get_next(idx);
                                parser.tree.unlink(idx);
                                parser.tree.set_parent(idx, parent);
                            }
                            if indent_len + start > 0 {
                                let previous = line.slice(0, indent_len + start);
                                parser.append_text_to_owned(
                                    parent,
                                    previous.to_string(),
                                    (
                                        previous.start_location(),
                                        previous.last_token_end_location(),
                                    ),
                                );
                            }
                            if let MarkdownNode::Html(h) = &mut parser.tree[parent].body {
                                if let html::Html::Block(
                                    html::HtmlType::CanonicalBlockTag(_, flag)
                                    | html::HtmlType::GenericTag(_, flag)
                                    | html::HtmlType::Component(_, flag),
                                ) = h.as_mut()
                                {
                                    *flag = html::Flag::Full
                                }
                            }
                            line.skip(indent_len + start + len);
                            if !line.is_end() {
                                let idx = parser
                                    .append_block(MarkdownNode::Paragraph, line.start_location());
                                parser.append_inline(idx, line.slice(0, line.len()));
                                line.skip_to_end()
                            }
                        } else {
                            if indent_len + start > 0 {
                                let previous = line.slice(0, indent_len + start);
                                if parser.current_proc().body.accepts_lines() {
                                    parser.append_text(
                                        previous.to_string(),
                                        (
                                            previous.start_location(),
                                            previous.last_token_end_location(),
                                        ),
                                    );
                                } else {
                                    let idx = parser.append_block(
                                        MarkdownNode::Paragraph,
                                        previous.start_location(),
                                    );
                                    parser.append_inline(idx, previous);
                                }
                            }
                            let idx = parser.append_block(
                                MarkdownNode::Html(Box::new(html::Html::Block(block_type))),
                                location,
                            );
                            line.skip(indent_len + start + len);
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
            html::HtmlType::JSComment(..) | html::HtmlType::JSExpression(..) => {
                BlockMatching::Unmatched
            }
        }
    }

    fn process(ProcessCtx { line, parser, id }: ProcessCtx) -> BlockProcessing {
        if line.is_blank() {
            if let MarkdownNode::Html(h) = &parser.tree[id].body {
                if matches!(
                    h.as_ref(),
                    html::Html::Block(
                        html::HtmlType::CanonicalBlockTag(..)
                            | html::HtmlType::GenericTag(..)
                            | html::HtmlType::Component(..)
                    )
                ) {
                    return BlockProcessing::Unprocessed;
                }
            }
        }
        BlockProcessing::Further
    }
}

mod scanners {
    use crate::ast::html;
    use crate::span::Span;

    pub(super) type ScanStartResult = (html::Element, usize, bool);
    pub(super) type ScanEndResult = (String, usize);

    #[derive(Debug)]
    enum State {
        Initial,
        InTag(usize),
        InAttr(InAttr),
    }

    #[derive(Debug)]
    enum InAttr {
        Attr(Option<usize>),
        InName(usize),
        InValue(AttrQuote, Option<usize>, usize),
    }

    #[derive(Debug, PartialEq)]
    enum AttrQuote {
        SingleQuote,
        DoubleQuote,
        Empty,
        None,
    }

    impl AttrQuote {
        fn matches_byte(&self, b: u8) -> bool {
            matches!(
                (self, b),
                (AttrQuote::DoubleQuote, b'"') | (AttrQuote::SingleQuote, b'\'')
            )
        }
    }

    // fn is_close_tag(span: &Span, pos: usize) -> bool {
    //     span.get(pos) == Some(b'>')
    //         || (span.get(pos) == Some(b'/') && span.get(pos + 1) == Some(b'>'))
    // }

    /// 判断字节是否为 HTML 属性值中不允许的字符
    fn is_unquoted_attr_disallowed(b: u8) -> bool {
        matches!(b, b'=' | b'<' | b'>' | b'`' | b'"' | b'\'' | b' ' | b'\t')
    }

    #[derive(Clone, Copy)]
    enum JsScanState {
        Normal,
        SingleQuote,
        DoubleQuote,
        TemplateQuote,
        LineComment,
        BlockComment,
    }

    fn scan_jsx_attr_expr_end(span: &Span, open_brace_index: usize) -> Option<usize> {
        if span.get(open_brace_index) != Some(b'{') {
            return None;
        }
        let mut i = open_brace_index + 1;
        let mut depth = 1usize;
        let mut state = JsScanState::Normal;
        let mut escaped = false;
        while let Some(b) = span.get(i) {
            match state {
                JsScanState::Normal => match b {
                    b'\'' => state = JsScanState::SingleQuote,
                    b'"' => state = JsScanState::DoubleQuote,
                    b'`' => state = JsScanState::TemplateQuote,
                    b'/' if span.get(i + 1) == Some(b'/') => {
                        state = JsScanState::LineComment;
                        i += 1;
                    }
                    b'/' if span.get(i + 1) == Some(b'*') => {
                        state = JsScanState::BlockComment;
                        i += 1;
                    }
                    b'{' => depth += 1,
                    b'}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            return Some(i);
                        }
                    }
                    _ => {}
                },
                JsScanState::SingleQuote => {
                    if escaped {
                        escaped = false;
                    } else if b == b'\\' {
                        escaped = true;
                    } else if b == b'\'' {
                        state = JsScanState::Normal;
                    }
                }
                JsScanState::DoubleQuote => {
                    if escaped {
                        escaped = false;
                    } else if b == b'\\' {
                        escaped = true;
                    } else if b == b'"' {
                        state = JsScanState::Normal;
                    }
                }
                JsScanState::TemplateQuote => {
                    if escaped {
                        escaped = false;
                    } else if b == b'\\' {
                        escaped = true;
                    } else if b == b'`' {
                        state = JsScanState::Normal;
                    }
                }
                JsScanState::LineComment => {
                    if matches!(b, b'\n' | b'\r') {
                        state = JsScanState::Normal;
                    }
                }
                JsScanState::BlockComment => {
                    if b == b'*' && span.get(i + 1) == Some(b'/') {
                        state = JsScanState::Normal;
                        i += 1;
                    }
                }
            }
            i += 1;
        }
        None
    }

    /// 扫描 HTML 开始标签
    ///
    /// span 当前 cursor 应指向 '<'
    /// 返回 (Element, end_offset, is_self_close)
    pub(super) fn scan_html_start(span: &Span) -> Option<ScanStartResult> {
        let mut name = String::new();
        let mut attrs = Vec::<(String, html::PropValue)>::new();
        let mut state = State::Initial;
        let mut end = 0;
        let mut self_close = false;

        // 从 '<' 之后开始扫描（index 1）
        let mut i = 1;
        let len = span.len();
        while i < len {
            let b = span.get(i)?;
            match (&state, b) {
                (State::Initial, b) if b.is_ascii_alphabetic() => {
                    state = State::InTag(i);
                }
                (State::InTag(_), b) if b.is_ascii_alphanumeric() || b == b'-' || b == b'.' => {}
                (State::InTag(start), b' ') => {
                    name = span.slice(*start, i).as_str().to_string();
                    state = State::InAttr(InAttr::Attr(None));
                }
                (State::InTag(start), b'>') => {
                    name = span.slice(*start, i).as_str().to_string();
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                (State::InTag(start), b'/') if span.get(i + 1) == Some(b'>') => {
                    name = span.slice(*start, i).as_str().to_string();
                    end = i + 2;
                    self_close = true;
                    state = State::Initial;
                    break;
                }
                // InAttr: 等待属性名开始
                (State::InAttr(InAttr::Attr(_)), b)
                    if b.is_ascii_alphabetic() || b == b'_' || b == b':' =>
                {
                    state = State::InAttr(InAttr::InName(i));
                }
                (State::InAttr(InAttr::Attr(Some(attr_index))), b'=') => {
                    state =
                        State::InAttr(InAttr::InValue(AttrQuote::None, Some(*attr_index), i + 1));
                }
                (State::InAttr(InAttr::Attr(_)), b' ' | b'\t') => {}
                (State::InAttr(InAttr::Attr(_)), b'>') => {
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                (State::InAttr(InAttr::Attr(_)), b'/') if span.get(i + 1) == Some(b'>') => {
                    end = i + 2;
                    self_close = true;
                    state = State::Initial;
                    break;
                }
                // InName: 属性名
                (State::InAttr(InAttr::InName(_)), b)
                    if b.is_ascii_alphanumeric()
                        || b == b'-'
                        || b == b'_'
                        || b == b':'
                        || b == b'.' => {}
                (State::InAttr(InAttr::InName(start)), b'=') => {
                    let attr = span.slice(*start, i).as_str().to_string();
                    let index = attrs.len();
                    attrs.push((attr, html::PropValue::Literal(String::new())));
                    state = State::InAttr(InAttr::InValue(AttrQuote::None, Some(index), i + 1));
                }
                (State::InAttr(InAttr::InName(start)), b' ') => {
                    let attr = span.slice(*start, i).as_str().to_string();
                    let index = attrs.len();
                    attrs.push((attr, html::PropValue::Literal(String::new())));
                    state = State::InAttr(InAttr::Attr(Some(index)));
                }
                (State::InAttr(InAttr::InName(start)), b'>') => {
                    let attr = span.slice(*start, i).as_str().to_string();
                    attrs.push((attr, html::PropValue::Literal(String::new())));
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                (State::InAttr(InAttr::InName(start)), b'/') if span.get(i + 1) == Some(b'>') => {
                    let attr = span.slice(*start, i).as_str().to_string();
                    attrs.push((attr, html::PropValue::Literal(String::new())));
                    end = i + 2;
                    self_close = true;
                    state = State::Initial;
                    break;
                }
                // InValue: 等待引号或无引号值
                (State::InAttr(InAttr::InValue(AttrQuote::None, None, _)), b' ' | b'\t') => {}
                (State::InAttr(InAttr::InValue(AttrQuote::None, Some(_), _)), b' ' | b'\t') => {}
                (
                    State::InAttr(InAttr::InValue(
                        quote @ (AttrQuote::None | AttrQuote::SingleQuote | AttrQuote::DoubleQuote),
                        Some(index),
                        start,
                    )),
                    b'"' | b'\'',
                ) => {
                    if quote.matches_byte(b) {
                        // 结束引号值
                        let value = span.slice(*start, i).to_unescape_string();
                        attrs[*index].1 = html::PropValue::Literal(value);
                        // After a quoted value, the next byte must be whitespace,
                        // tag end (`>`) or self-close (`/>`).
                        match span.get(i + 1) {
                            Some(b' ' | b'\t' | b'>') => {}
                            Some(b'/') if span.get(i + 2) == Some(b'>') => {}
                            _ => return None,
                        }
                        state = State::InAttr(InAttr::Attr(None));
                    } else if *quote == AttrQuote::None {
                        // 开始引号值
                        let new_quote = if b == b'\'' {
                            AttrQuote::SingleQuote
                        } else {
                            AttrQuote::DoubleQuote
                        };
                        state = State::InAttr(InAttr::InValue(new_quote, Some(*index), i + 1));
                    }
                    // 在引号值内部遇到另一种引号，继续
                }
                // 在引号值内部，任何字符都接受
                (
                    State::InAttr(InAttr::InValue(
                        AttrQuote::DoubleQuote | AttrQuote::SingleQuote,
                        ..,
                    )),
                    _,
                ) => {}
                // 无引号值开始或继续
                (
                    State::InAttr(InAttr::InValue(
                        quote @ (AttrQuote::None | AttrQuote::Empty),
                        Some(index),
                        _,
                    )),
                    b,
                ) if !is_unquoted_attr_disallowed(b) && b > 0x1f => {
                    if *quote == AttrQuote::None {
                        if b == b'{' {
                            if let Some(expr_end) = scan_jsx_attr_expr_end(span, i) {
                                let value = span.slice(i + 1, expr_end).as_str().to_string();
                                attrs[*index].1 = html::PropValue::Expr(value);
                                state = State::InAttr(InAttr::Attr(None));
                                i = expr_end + 1;
                                continue;
                            }
                            return None;
                        }
                        state = State::InAttr(InAttr::InValue(AttrQuote::Empty, Some(*index), i));
                    }
                }
                // 无引号值结束（空格）
                (
                    State::InAttr(InAttr::InValue(AttrQuote::Empty, Some(index), start)),
                    b' ' | b'\t',
                ) => {
                    let value = span.slice(*start, i).to_unescape_string();
                    attrs[*index].1 = html::PropValue::Literal(value);
                    state = State::InAttr(InAttr::Attr(None));
                }
                // 无引号值结束（>）
                (State::InAttr(InAttr::InValue(AttrQuote::Empty, Some(index), start)), b'>') => {
                    let value = span.slice(*start, i).to_unescape_string();
                    attrs[*index].1 = html::PropValue::Literal(value);
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                (State::InAttr(InAttr::InValue(AttrQuote::Empty, Some(index), start)), b'/')
                    if span.get(i + 1) == Some(b'>') =>
                {
                    let value = span.slice(*start, i).to_unescape_string();
                    attrs[*index].1 = html::PropValue::Literal(value);
                    end = i + 2;
                    self_close = true;
                    state = State::Initial;
                    break;
                }
                _ => return None,
            }
            i += 1;
        }
        if matches!(state, State::Initial) && end > 0 {
            Some((
                html::Element {
                    name,
                    props: if attrs.is_empty() { None } else { Some(attrs) },
                },
                end,
                self_close,
            ))
        } else {
            None
        }
    }

    /// 扫描 HTML 结束标签 `</tagname>`
    ///
    /// span 当前 cursor 应指向 '<'
    pub(super) fn scan_html_end(span: &Span) -> Option<ScanEndResult> {
        if span.get(0) != Some(b'<') || span.get(1) != Some(b'/') {
            return None;
        }
        let mut state = State::Initial;
        let mut end = 0;
        let mut name = String::new();
        let len = span.len();
        let mut i = 2;
        while i < len {
            let b = match span.get(i) {
                Some(b) => b,
                None => break,
            };
            match (&state, b) {
                (State::Initial, b) if b.is_ascii_alphabetic() => {
                    state = State::InTag(i);
                }
                (State::InTag(_), b) if b.is_ascii_alphanumeric() || b == b'-' || b == b'.' => {}
                (State::InTag(start), b' ') => {
                    name = span.slice(*start, i).as_str().to_string();
                    state = State::InAttr(InAttr::Attr(None));
                }
                (State::InTag(start), b'>') => {
                    name = span.slice(*start, i).as_str().to_string();
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                (State::InAttr(InAttr::Attr(None)), b' ' | b'\t') => {}
                (State::InAttr(InAttr::Attr(None)), b'>') => {
                    end = i + 1;
                    state = State::Initial;
                    break;
                }
                _ => return None,
            }
            i += 1;
        }
        if matches!(state, State::Initial) && end > 0 {
            Some((name, end))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{MarkdownNode, html};
    use crate::html::{Element, PropValue};
    use crate::parser::{Parser, ParserOptions};

    #[test]
    fn case_1() {
        let text = r#"
<script>console.log("hello world")</script>
        "#
        .trim();
        let ast = Parser::new(text).parse();
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(Box::new(html::Html::Block(
                html::HtmlType::RawTextContainer(
                    html::Element {
                        name: "script".to_string(),
                        props: None
                    },
                    html::Flag::Full
                )
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
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(Box::new(html::Html::Block(html::HtmlType::HtmlComment)))
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
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(Box::new(html::Html::Block(
                html::HtmlType::ProcessingInstruction
            )))
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
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(Box::new(html::Html::Block(html::HtmlType::Declaration)))
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
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(Box::new(html::Html::Block(html::HtmlType::CDataSection)))
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
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(Box::new(html::Html::Block(
                html::HtmlType::CanonicalBlockTag(
                    html::Element {
                        name: "p".to_string(),
                        props: None
                    },
                    html::Flag::Full
                )
            )))
        );
        assert_eq!(
            ast[2].body,
            MarkdownNode::Text(
                "<p>\n  Geckos are a group of usually small, usually nocturnal lizards. They are found on every continent except Antarctica.\n</p>".to_string()
            )
        );
        assert_eq!(ast.len(), 3);
    }
    #[test]
    fn case_7() {
        let text = r#"<Button>Click Me 1</Button>"#.trim();
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_mdx_component())
            .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Paragraph);
        assert_eq!(
            ast[2].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::Component(
                html::Element {
                    name: "Button".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );
        assert_eq!(ast[3].body, MarkdownNode::Text("Click Me 1".to_string()));
        assert_eq!(ast.len(), 4);
    }

    #[test]
    fn mdx_components_mode_does_not_change_default_type7_behavior() {
        let text = "<foo-bar>\ntext\n</foo-bar>";
        let ast = Parser::new(text).parse();
        assert!(if let MarkdownNode::Html(h) = &ast[1].body {
            matches!(
                h.as_ref(),
                html::Html::Block(html::HtmlType::GenericTag(..))
            )
        } else {
            false
        });
    }

    #[test]
    fn mdx_components_mode_limits_block_type7_to_component_names() {
        let text = "<foo-bar>\ntext\n</foo-bar>";
        let ast = Parser::new_with_options(text, ParserOptions::default().enabled_mdx_component())
            .parse();
        assert_eq!(ast[1].body, MarkdownNode::Paragraph);
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
        assert_eq!(
            ast.to_html(),
            r#"<table><tr><td>
<pre>
**Hello**,
<p><em>world</em>.
</pre></p>
</td></tr></table>"#
        )
    }

    #[test]
    fn generic_and_component_classification() {
        let generic = Parser::new("<foo-bar>Click</foo-bar>").parse();
        assert_eq!(generic[0].body, MarkdownNode::Document);
        assert_eq!(generic[1].body, MarkdownNode::Paragraph);
        assert_eq!(
            generic[2].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::GenericTag(
                html::Element {
                    name: "foo-bar".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );

        let component = Parser::new("<Button>Click</Button>").parse();
        assert_eq!(component[0].body, MarkdownNode::Document);
        assert_eq!(component[1].body, MarkdownNode::Paragraph);
        assert_eq!(
            component[2].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::GenericTag(
                html::Element {
                    name: "Button".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );

        let component_mdx = Parser::new_with_options(
            "<Button>Click</Button>",
            ParserOptions::default().enabled_mdx_component(),
        )
        .parse();
        assert_eq!(component_mdx[0].body, MarkdownNode::Document);
        assert_eq!(component_mdx[1].body, MarkdownNode::Paragraph);
        assert_eq!(
            component_mdx[2].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::Component(
                html::Element {
                    name: "Button".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );
    }

    #[test]
    fn nested_components() {
        let ast = Parser::new_with_options(
            r#"<Comments>
  <CommentsHeader/>        <!-- 计数 / 排序 / 筛选 -->
  <CommentsList>
    <CommentItem>
      <CommentMeta/>       {/* 头像 作者 时间 */}
      <CommentContent/>    <!-- 正文 -->
      <CommentToolbar/>    <!-- 回复/点赞 -->
      <CommentReplies/>    <!-- 子评论 -->
    </CommentItem>
  </CommentsList>
  <CommentsComposer/>      <!-- 输入框 -->
</Comments>
"#,
            ParserOptions::default().enabled_mdx_component(),
        )
        .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        let root = ast.get_first_child(0).unwrap();
        assert_eq!(
            ast[root].body,
            MarkdownNode::Html(Box::new(html::Html::Block(html::HtmlType::Component(
                Element {
                    name: "Comments".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );

        let mut has_comments_header = false;
        let mut has_comments_list = false;
        let mut has_comment_item_nested = false;
        let mut has_html_comment_whole = false;
        let mut has_whitespace_only_text = false;

        let mut stack = vec![root];
        while let Some(parent) = stack.pop() {
            let mut child = ast.get_first_child(parent);
            while let Some(idx) = child {
                if let MarkdownNode::Text(text) = &ast[idx].body {
                    if text.chars().all(|ch| matches!(ch, ' ' | '\t')) {
                        has_whitespace_only_text = true;
                    }
                }
                if let MarkdownNode::Html(h) = &ast[idx].body {
                    match h.as_ref() {
                        html::Html::Inline(html::HtmlType::Component(element, flag))
                            if element.name == "CommentsHeader"
                                && *flag == html::Flag::SelfClose =>
                        {
                            has_comments_header = true;
                        }
                        html::Html::Inline(html::HtmlType::Component(element, flag))
                            if element.name == "CommentsList" && *flag == html::Flag::Full =>
                        {
                            has_comments_list = true;
                            let mut cc = ast.get_first_child(idx);
                            while let Some(inner) = cc {
                                if let MarkdownNode::Html(h2) = &ast[inner].body {
                                    if matches!(
                                        h2.as_ref(),
                                        html::Html::Inline(html::HtmlType::Component(e, html::Flag::Full))
                                            if e.name == "CommentItem"
                                    ) {
                                        has_comment_item_nested = true;
                                    }
                                }
                                cc = ast.get_next(inner);
                            }
                        }
                        html::Html::Inline(html::HtmlType::HtmlComment) => {
                            if let Some(text_idx) = ast.get_first_child(idx) {
                                if ast.get_next(text_idx).is_none()
                                    && matches!(&ast[text_idx].body, MarkdownNode::Text(t) if t.starts_with("<!--") && t.ends_with("-->"))
                                {
                                    has_html_comment_whole = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                stack.push(idx);
                child = ast.get_next(idx);
            }
        }

        assert!(has_comments_header);
        assert!(has_comments_list);
        assert!(has_comment_item_nested);
        assert!(has_html_comment_whole);
        assert!(!has_whitespace_only_text);
    }

    #[test]
    fn mdx_js_expression_in_component() {
        let ast = Parser::new_with_options(
            "<Button>{(x: Foo) => ({ value: x })}</Button>",
            ParserOptions::default().enabled_mdx_component(),
        )
        .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Paragraph);
        assert_eq!(
            ast[2].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::Component(
                Element {
                    name: "Button".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );
        assert_eq!(
            ast[3].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::JSExpression(
                "(x: Foo) => ({ value: x })".to_string()
            ))))
        );
    }

    #[test]
    fn mdx_js_comment_in_component() {
        let ast = Parser::new_with_options(
            "<Button>{/* comment */}</Button>",
            ParserOptions::default().enabled_mdx_component(),
        )
        .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Paragraph);
        assert_eq!(
            ast[2].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::Component(
                Element {
                    name: "Button".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );
        assert_eq!(
            ast[3].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::JSComment(
                " comment ".to_string()
            ))))
        );
    }

    #[test]
    fn attrs_js_expression() {
        let ast = Parser::new_with_options(
            "<NestedList items={['A', ['B', 'C'], 'D']} />",
            ParserOptions::default().enabled_mdx_component(),
        )
        .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(
            ast[1].body,
            MarkdownNode::Html(Box::new(html::Html::Block(html::HtmlType::Component(
                Element {
                    name: "NestedList".to_string(),
                    props: Some(vec![(
                        "items".to_string(),
                        PropValue::Expr("['A', ['B', 'C'], 'D']".to_string())
                    )])
                },
                html::Flag::SelfClose
            ))))
        );
    }

    #[test]
    fn component_name_with_dot() {
        let ast = Parser::new_with_options(
            "<UI.Button>Click</UI.Button>",
            ParserOptions::default().enabled_mdx_component(),
        )
        .parse();
        println!("AST:\n{ast:?}");
        assert_eq!(ast[0].body, MarkdownNode::Document);
        assert_eq!(ast[1].body, MarkdownNode::Paragraph);
        assert_eq!(
            ast[2].body,
            MarkdownNode::Html(Box::new(html::Html::Inline(html::HtmlType::Component(
                Element {
                    name: "UI.Button".to_string(),
                    props: None
                },
                html::Flag::Full
            ))))
        );
    }

    #[test]
    fn scan_probe_html_595() {
        let raw = "<a foo=\"bar\" bam = 'baz <em>\"</em>' _boolean zoop:33=zoop:33 />";
        let mut scanner = crate::scanner::Scanner::new(raw);
        let mut span = crate::span::Span::extract(&mut scanner).unwrap();
        assert!(super::scan_html_type(&mut span, true, false).is_some());
    }
}
